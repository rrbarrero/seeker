use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::Arc,
    time::{Duration, Instant},
};

use axum::{
    Json,
    body::Body,
    extract::{ConnectInfo, State},
    http::{HeaderValue, Request, StatusCode, header::RETRY_AFTER},
    middleware::Next,
    response::{IntoResponse, Response},
};
use tokio::sync::Mutex;
use tracing::warn;

const CLEANUP_INTERVAL: Duration = Duration::from_secs(60);
const STALE_AFTER: Duration = Duration::from_secs(10 * 60);

#[derive(Clone, Debug)]
pub struct RateLimitConfig {
    pub requests_per_second: u32,
    pub burst: u32,
    pub trust_forwarded_headers: bool,
}

#[derive(Clone)]
pub struct RateLimitState {
    config: RateLimitConfig,
    store: Arc<Mutex<StoreState>>,
}

impl RateLimitState {
    pub fn new(config: RateLimitConfig) -> Self {
        Self {
            config,
            store: Arc::new(Mutex::new(StoreState::new())),
        }
    }

    async fn check(&self, key: &str, now: Instant) -> RateLimitDecision {
        let mut store = self.store.lock().await;
        if store.last_cleanup.elapsed() >= CLEANUP_INTERVAL {
            store.cleanup(now);
        }

        let bucket = store
            .buckets
            .entry(key.to_string())
            .or_insert_with(|| TokenBucket::new(self.config.burst, now));
        let rate = self.config.requests_per_second as f64;
        let burst = self.config.burst as f64;

        bucket.allow(now, rate, burst)
    }
}

struct StoreState {
    buckets: HashMap<String, TokenBucket>,
    last_cleanup: Instant,
}

impl StoreState {
    fn new() -> Self {
        Self {
            buckets: HashMap::new(),
            last_cleanup: Instant::now(),
        }
    }

    fn cleanup(&mut self, now: Instant) {
        self.buckets
            .retain(|_, bucket| now.duration_since(bucket.last_seen) <= STALE_AFTER);
        self.last_cleanup = now;
    }
}

struct TokenBucket {
    tokens: f64,
    last_refill: Instant,
    last_seen: Instant,
}

impl TokenBucket {
    fn new(burst: u32, now: Instant) -> Self {
        Self {
            tokens: burst as f64,
            last_refill: now,
            last_seen: now,
        }
    }

    fn allow(&mut self, now: Instant, rate: f64, burst: f64) -> RateLimitDecision {
        self.last_seen = now;
        self.refill(now, rate, burst);

        if rate <= 0.0 {
            return RateLimitDecision::Denied(Duration::from_secs(1));
        }

        if self.tokens >= 1.0 {
            self.tokens -= 1.0;
            RateLimitDecision::Allowed
        } else {
            let needed = 1.0 - self.tokens;
            let retry_after = Duration::from_secs_f64(needed / rate).max(Duration::from_secs(1));
            RateLimitDecision::Denied(retry_after)
        }
    }

    fn refill(&mut self, now: Instant, rate: f64, burst: f64) {
        if now <= self.last_refill {
            return;
        }
        let elapsed = now.duration_since(self.last_refill).as_secs_f64();
        let added = elapsed * rate;
        self.tokens = (self.tokens + added).min(burst);
        self.last_refill = now;
    }
}

enum RateLimitDecision {
    Allowed,
    Denied(Duration),
}

pub async fn rate_limit(
    State(state): State<RateLimitState>,
    request: Request<Body>,
    next: Next,
) -> Response {
    let now = Instant::now();
    let key = extract_client_key(&request, state.config.trust_forwarded_headers);
    match state.check(&key, now).await {
        RateLimitDecision::Allowed => next.run(request).await,
        RateLimitDecision::Denied(retry_after) => {
            warn!("Rate limit exceeded for client: {}", key);
            let body = serde_json::json!({
                "error": "rate limit exceeded",
            });
            let mut response = (StatusCode::TOO_MANY_REQUESTS, Json(body)).into_response();
            if let Ok(value) = HeaderValue::from_str(&retry_after.as_secs().to_string()) {
                response.headers_mut().insert(RETRY_AFTER, value);
            }
            response
        }
    }
}

fn extract_client_key(request: &Request<Body>, trust_forwarded: bool) -> String {
    if trust_forwarded && let Some(ip) = forwarded_for(request) {
        return ip;
    }

    if let Some(ConnectInfo(addr)) = request.extensions().get::<ConnectInfo<SocketAddr>>() {
        return addr.ip().to_string();
    }

    if !trust_forwarded && let Some(ip) = forwarded_for(request) {
        return ip;
    }

    "unknown".to_string()
}

fn forwarded_for(request: &Request<Body>) -> Option<String> {
    let header = request
        .headers()
        .get("x-forwarded-for")
        .or_else(|| request.headers().get("x-real-ip"))?;
    let value = header.to_str().ok()?;
    value.split(',').next().map(|part| part.trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{Router, routing::get};
    use tower::ServiceExt;

    #[test]
    fn forwarded_for_returns_first_ip() {
        let request = Request::builder()
            .header("x-forwarded-for", "10.0.0.1, 10.0.0.2")
            .body(Body::empty())
            .unwrap();

        let result = forwarded_for(&request);
        assert_eq!(result, Some("10.0.0.1".to_string()));
    }

    #[test]
    fn extract_client_key_prefers_forwarded_when_trusted() {
        let request = Request::builder()
            .header("x-real-ip", "192.168.1.10")
            .body(Body::empty())
            .unwrap();

        let key = extract_client_key(&request, true);
        assert_eq!(key, "192.168.1.10");
    }

    #[test]
    fn extract_client_key_uses_connect_info_when_not_trusted() {
        let mut request = Request::builder().body(Body::empty()).unwrap();
        request
            .extensions_mut()
            .insert(ConnectInfo(SocketAddr::from(([127, 0, 0, 1], 1234))));

        let key = extract_client_key(&request, false);
        assert_eq!(key, "127.0.0.1");
    }

    #[test]
    fn token_bucket_allows_then_denies_without_refill() {
        let now = Instant::now();
        let mut bucket = TokenBucket::new(1, now);
        let rate = 1.0;
        let burst = 1.0;

        let first = bucket.allow(now, rate, burst);
        match first {
            RateLimitDecision::Allowed => {}
            _ => panic!("expected allowed"),
        }

        let second = bucket.allow(now, rate, burst);
        match second {
            RateLimitDecision::Denied(_) => {}
            _ => panic!("expected denied"),
        }
    }

    #[test]
    fn token_bucket_refills_over_time() {
        let now = Instant::now();
        let mut bucket = TokenBucket::new(1, now);
        let rate = 1.0;
        let burst = 1.0;

        let _ = bucket.allow(now, rate, burst);
        let later = now + Duration::from_secs(1);
        let decision = bucket.allow(later, rate, burst);

        match decision {
            RateLimitDecision::Allowed => {}
            _ => panic!("expected allowed after refill"),
        }
    }

    #[tokio::test]
    async fn middleware_limits_requests_per_client() {
        let state = RateLimitState::new(RateLimitConfig {
            requests_per_second: 1,
            burst: 1,
            trust_forwarded_headers: true,
        });

        let app = Router::new()
            .route("/", get(|| async { StatusCode::OK }))
            .layer(axum::middleware::from_fn_with_state(state, rate_limit));

        let request = Request::builder()
            .uri("/")
            .header("x-forwarded-for", "203.0.113.10")
            .body(Body::empty())
            .unwrap();

        let response = app.clone().oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let request = Request::builder()
            .uri("/")
            .header("x-forwarded-for", "203.0.113.10")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::TOO_MANY_REQUESTS);
    }
}

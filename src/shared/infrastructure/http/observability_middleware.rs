use std::time::Instant;

use axum::{body::Body, extract::State, http::Request, middleware::Next, response::Response};
use opentelemetry::metrics::{Counter, Histogram};

use crate::shared::infrastructure::observability::Observability;

pub async fn request_observability(
    State(obs): State<Observability>,
    request: Request<Body>,
    next: Next,
) -> Response {
    let start = Instant::now();

    let method = request.method().to_string();
    let path = request.uri().path().to_string();
    let response = next.run(request).await;

    let status = response.status().as_u16();
    let duration_ms = start.elapsed().as_secs_f64() * 1000.0;

    record_metrics(
        &obs.request_counter,
        &obs.request_duration_ms,
        &method,
        &path,
        status,
        duration_ms,
    );

    response
}

fn record_metrics(
    counter: &Counter<u64>,
    histogram: &Histogram<f64>,
    method: &str,
    path: &str,
    status: u16,
    duration_ms: f64,
) {
    let attrs = [
        opentelemetry::KeyValue::new("http.method", method.to_string()),
        opentelemetry::KeyValue::new("http.route", path.to_string()),
        opentelemetry::KeyValue::new("http.status_code", status as i64),
    ];

    counter.add(1, &attrs);
    histogram.record(duration_ms, &attrs);
}

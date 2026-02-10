use std::time::Instant;

use axum::{body::Body, extract::State, http::Request, middleware::Next, response::Response};
use opentelemetry::metrics::{Counter, Histogram};
use opentelemetry::trace::{TraceContextExt, TraceId};
use tower_http::request_id::RequestId;
use tracing_opentelemetry::OpenTelemetrySpanExt;

use crate::shared::infrastructure::observability::Observability;

pub async fn request_observability(
    State(obs): State<Observability>,
    request: Request<Body>,
    next: Next,
) -> Response {
    let start = Instant::now();

    let method = request.method().to_string();
    let path = request.uri().path().to_string();
    let request_id = request
        .extensions()
        .get::<RequestId>()
        .and_then(|id| id.header_value().to_str().ok())
        .unwrap_or("-")
        .to_string();
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

    if status >= 500 {
        let span = tracing::Span::current();
        let trace_id = span.context().span().span_context().trace_id();
        let trace_id = if trace_id != TraceId::INVALID {
            trace_id.to_string()
        } else {
            "-".to_string()
        };

        tracing::error!(
            request_id = %request_id,
            trace_id = %trace_id,
            http.method = %method,
            http.route = %path,
            http.status_code = status,
            latency_ms = duration_ms,
            "request failed"
        );
    }

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

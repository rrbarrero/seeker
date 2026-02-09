use opentelemetry::{
    KeyValue, global,
    metrics::{Counter, Histogram, MeterProvider},
    trace::TracerProvider,
};
use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{
    Resource, logs::SdkLoggerProvider, metrics::SdkMeterProvider, trace::SdkTracerProvider,
};
use opentelemetry_semantic_conventions::resource::SERVICE_NAME;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
pub struct Observability {
    pub request_counter: Counter<u64>,
    pub request_duration_ms: Histogram<f64>,
    tracer_provider: SdkTracerProvider,
    meter_provider: SdkMeterProvider,
    logger_provider: SdkLoggerProvider,
}

#[derive(Debug)]
pub struct ObservabilityError {
    message: String,
}

impl ObservabilityError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for ObservabilityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ObservabilityError {}

pub fn init_observability(
    service_name: &str,
    otlp_endpoint: &str,
) -> Result<Observability, ObservabilityError> {
    let resource = Resource::builder()
        .with_attributes(vec![KeyValue::new(SERVICE_NAME, service_name.to_string())])
        .build();

    // Trace exporter
    let trace_exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .with_endpoint(otlp_endpoint)
        .build()
        .map_err(|err| ObservabilityError::new(format!("OTLP trace exporter error: {err}")))?;

    let tracer_provider = SdkTracerProvider::builder()
        .with_resource(resource.clone())
        .with_batch_exporter(trace_exporter)
        .build();

    let static_service_name: &'static str = Box::leak(service_name.to_string().into_boxed_str());
    let tracer = tracer_provider.tracer(static_service_name);
    global::set_tracer_provider(tracer_provider.clone());

    // Metric exporter
    let metric_exporter = opentelemetry_otlp::MetricExporter::builder()
        .with_tonic()
        .with_endpoint(otlp_endpoint)
        .build()
        .map_err(|err| ObservabilityError::new(format!("OTLP metric exporter error: {err}")))?;

    let meter_provider = SdkMeterProvider::builder()
        .with_resource(resource.clone())
        .with_periodic_exporter(metric_exporter)
        .build();

    let meter = meter_provider.meter(static_service_name);
    global::set_meter_provider(meter_provider.clone());

    let request_counter = meter
        .u64_counter("http_requests_total")
        .with_description("Total number of HTTP requests")
        .build();
    let request_duration_ms = meter
        .f64_histogram("http_request_duration_ms")
        .with_description("HTTP request duration in milliseconds")
        .build();

    // Log exporter - NEW: This sends logs to Loki via Alloy
    let log_exporter = opentelemetry_otlp::LogExporter::builder()
        .with_tonic()
        .with_endpoint(otlp_endpoint)
        .build()
        .map_err(|err| ObservabilityError::new(format!("OTLP log exporter error: {err}")))?;

    let logger_provider = SdkLoggerProvider::builder()
        .with_resource(resource)
        .with_batch_exporter(log_exporter)
        .build();

    // Create the OpenTelemetry logs layer that bridges tracing events to OTel logs
    let otel_logs_layer = OpenTelemetryTracingBridge::new(&logger_provider);

    // Configure tracing subscriber with all layers
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    // fmt_layer: Writes JSON logs to stdout
    let fmt_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_current_span(true)
        .with_span_list(true);

    // otel_layer: Sends spans to Tempo
    let otel_trace_layer = tracing_opentelemetry::layer().with_tracer(tracer);

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt_layer) // JSON to stdout
        .with(otel_trace_layer) // Spans to Tempo
        .with(otel_logs_layer) // Logs to Loki
        .init();

    Ok(Observability {
        request_counter,
        request_duration_ms,
        tracer_provider,
        meter_provider,
        logger_provider,
    })
}

pub fn shutdown_observability(observability: &Observability) {
    let _ = observability.tracer_provider.shutdown();
    let _ = observability.meter_provider.shutdown();
    let _ = observability.logger_provider.shutdown();
}

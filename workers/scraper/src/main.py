import asyncio
import logging
import signal
import sys
import os
from time import sleep

from opentelemetry import trace
from opentelemetry.context import attach, detach
from opentelemetry.propagate import extract
from opentelemetry.sdk.trace import TracerProvider
from opentelemetry.sdk.trace.export import BatchSpanProcessor
from opentelemetry.exporter.otlp.proto.http.trace_exporter import OTLPSpanExporter
from opentelemetry.sdk.resources import SERVICE_NAME, Resource

from db import get_next_job, complete_job, fail_job
from s3 import upload_html
from scraper import scrape_url

# Configure Logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s'
)
logger = logging.getLogger(__name__)

# Configure OpenTelemetry
def init_tracing():
    if os.getenv("OBS_ENABLED", "false").lower() != "true":
        logger.info("Observability is disabled (OBS_ENABLED != true).")
        return trace.get_tracer("scraper.worker")
    
    service_name = os.getenv("SERVICE_NAME", "scraper-worker")
    endpoint = os.getenv("OTEL_EXPORTER_OTLP_ENDPOINT", "http://localhost:4318/v1/traces")
    
    logger.info(f"Initializing OpenTelemetry for {service_name} (endpoint: {endpoint})")
    
    resource = Resource(attributes={SERVICE_NAME: service_name})
    provider = TracerProvider(resource=resource)
    
    try:
        exporter = OTLPSpanExporter(endpoint=endpoint)
        processor = BatchSpanProcessor(exporter)
        provider.add_span_processor(processor)
        trace.set_tracer_provider(provider)
    except Exception as e:
        logger.error(f"Failed to initialize OTLP exporter: {e}")
        # Fallback to default provider to avoid crashes, even if it doesn't export
        trace.set_tracer_provider(provider)

    return trace.get_tracer("scraper.worker")

tracer = init_tracing()

# Config
POLL_INTERVAL = float(os.getenv("POLL_INTERVAL", "1.0"))

running = True

def handle_signal(signum, frame):
    global running
    logger.info("Received signal to stop...")
    running = False

def process_job(job):
    job_id = job['id']
    url = job['url']
    user_id = job['user_id']
    position_id = job['position_id']
    traceparent = job.get('trace_id')

    # Extract parent context from traceparent string
    parent_context = None
    if traceparent:
        parent_context = TraceContextTextMapPropagator().extract({"traceparent": traceparent})
    
    with tracer.start_as_current_span("process_job", context=parent_context, kind=trace.SpanKind.CONSUMER) as span:
        span.set_attribute("job.id", str(job_id))
        span.set_attribute("job.url", url)
        span.set_attribute("job.user_id", str(user_id))
        
        logger.info(f"Processing job {job_id} for URL: {url}")

        try:
            # 1. Scrape
            with tracer.start_as_current_span("scrape_url") as scrape_span:
                scrape_span.set_attribute("url", url)
                html_content = scrape_url(url)
            
            # 2. Upload to S3
            s3_key = f"scraper/{user_id}/{position_id}.html"
            with tracer.start_as_current_span("upload_s3") as s3_span:
                s3_span.set_attribute("s3.key", s3_key)
                upload_html(html_content, s3_key)
            
            # 3. Complete
            complete_job(job_id, s3_key)
            span.set_status(trace.Status(trace.StatusCode.OK))
            logger.info(f"Job {job_id} completed successfully. S3 Key: {s3_key}")

        except Exception as e:
            logger.error(f"Job {job_id} failed: {e}", exc_info=True)
            fail_job(job_id, str(e))
            span.record_exception(e)
            span.set_status(trace.Status(trace.StatusCode.ERROR, str(e)))

async def main():
    signal.signal(signal.SIGINT, handle_signal)
    signal.signal(signal.SIGTERM, handle_signal)

    logger.info("Scraper worker started. Polling for jobs...")

    while running:
        try:
            # We want to trace the poll operation too, but maybe as a separate span
            # or just trace the processing. Let's trace processing.
            job = await asyncio.to_thread(get_next_job)
            if job:
                await asyncio.to_thread(process_job, job)
            else:
                # No jobs, sleep briefly
                await asyncio.sleep(POLL_INTERVAL)
        except Exception as e:
            logger.error(f"Error in main loop: {e}", exc_info=True)
            await asyncio.sleep(5) 

    logger.info("Scraper worker stopped.")

if __name__ == "__main__":
    asyncio.run(main())

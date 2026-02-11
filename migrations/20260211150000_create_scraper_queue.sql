-- Create the scraper job status enum
CREATE TYPE scraper_job_status AS ENUM ('PENDING', 'PROCESSING', 'COMPLETED', 'FAILED');

-- Create the scraper queue table
CREATE TABLE IF NOT EXISTS scraper_queue (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    url TEXT NOT NULL,
    user_id UUID NOT NULL,
    position_id UUID NOT NULL,
    trace_id TEXT,
    status scraper_job_status DEFAULT 'PENDING',
    attempt_count INT DEFAULT 0,
    s3_key TEXT,
    error_message TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    started_at TIMESTAMPTZ
);

-- Index for efficient polling
CREATE INDEX IF NOT EXISTS idx_scraper_queue_status ON scraper_queue (status)
WHERE
    status = 'PENDING';

-- Create the notification function
CREATE OR REPLACE FUNCTION notify_scraper_queue()
RETURNS TRIGGER AS $$
BEGIN
    PERFORM pg_notify('scraper_queue', json_build_object('id', NEW.id, 'position_id', NEW.position_id)::text);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create the trigger
CREATE TRIGGER trigger_scraper_queue
AFTER INSERT ON scraper_queue
FOR EACH ROW
EXECUTE FUNCTION notify_scraper_queue();
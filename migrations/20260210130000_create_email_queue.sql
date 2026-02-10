-- Create the email queue table
CREATE TABLE IF NOT EXISTS email_queue (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    payload JSONB NOT NULL,
    processed BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    processed_at TIMESTAMPTZ
);

-- Create the notification function
CREATE OR REPLACE FUNCTION notify_email_queue()
RETURNS TRIGGER AS $$
BEGIN
    PERFORM pg_notify('email_queue', json_build_object('id', NEW.id)::text);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create the trigger
CREATE TRIGGER trigger_email_queue
AFTER INSERT ON email_queue
FOR EACH ROW
EXECUTE FUNCTION notify_email_queue();
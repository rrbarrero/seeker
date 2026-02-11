ALTER TABLE email_queue
    ADD COLUMN IF NOT EXISTS user_id UUID,
    ADD COLUMN IF NOT EXISTS trace_id TEXT;

CREATE INDEX IF NOT EXISTS email_queue_user_id_idx ON email_queue (user_id);
CREATE INDEX IF NOT EXISTS email_queue_trace_id_idx ON email_queue (trace_id);

import os
import psycopg
from psycopg.rows import dict_row
from datetime import datetime
import json

DATABASE_URL = os.getenv("DATABASE_URL")

def get_db_connection():
    return psycopg.connect(DATABASE_URL, row_factory=dict_row, autocommit=True)

def get_next_job():
    """
    Fetches the next PENDING job from the queue, locks it, and marks it as PROCESSING.
    Returns the job dict or None if no jobs are available.
    """
    with get_db_connection() as conn:
        with conn.cursor() as cur:
            # We use a single query to find, lock, and update the job
            # The 'FOR UPDATE SKIP LOCKED' clause is crucial for concurrency
            cur.execute("""
                UPDATE scraper_queue
                SET status = 'PROCESSING',
                    started_at = NOW(),
                    attempt_count = attempt_count + 1
                WHERE id = (
                    SELECT id
                    FROM scraper_queue
                    WHERE status = 'PENDING'
                    FOR UPDATE SKIP LOCKED
                    LIMIT 1
                )
                RETURNING id, url, user_id, position_id, trace_id;
            """)
            return cur.fetchone()

def complete_job(job_id, s3_key):
    """
    Marks a job as COMPLETED and saves the S3 key.
    """
    with get_db_connection() as conn:
        with conn.cursor() as cur:
            cur.execute("""
                UPDATE scraper_queue
                SET status = 'COMPLETED',
                    s3_key = %s,
                    updated_at = NOW()
                WHERE id = %s
            """, (s3_key, job_id))

def fail_job(job_id, error_message):
    """
    Marks a job as FAILED and saves the error message.
    """
    with get_db_connection() as conn:
        with conn.cursor() as cur:
            cur.execute("""
                UPDATE scraper_queue
                SET status = 'FAILED',
                    error_message = %s,
                    updated_at = NOW()
                WHERE id = %s
            """, (error_message, job_id))

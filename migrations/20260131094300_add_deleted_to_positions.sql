-- Add deleted field to positions table
ALTER TABLE positions
ADD COLUMN deleted BOOLEAN NOT NULL DEFAULT FALSE;
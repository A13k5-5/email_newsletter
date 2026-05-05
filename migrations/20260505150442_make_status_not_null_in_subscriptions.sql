-- Add migration script here
BEGIN;
    -- Backfill status for historical entries
    UPDATE subscriptions
        SET status = 'confirmed'
        WHERE status IS NULL;
    -- Make status mandatory now
    ALTER TABLE subscriptions ALTER COLUMN status SET NOT NULL;
COMMIT;
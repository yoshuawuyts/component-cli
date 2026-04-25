CREATE TABLE fetch_queue (
    -- Surrogate primary key.
    id INTEGER PRIMARY KEY,
    -- OCI registry hostname, e.g. "ghcr.io/webassembly".
    registry TEXT NOT NULL,
    -- OCI repository path, e.g. "wasi/clocks".
    repository TEXT NOT NULL,
    -- The version tag to fetch, e.g. "0.2.11".
    tag TEXT NOT NULL,
    -- The kind of work to perform:
    --   'pull'    — download from the registry and extract metadata
    --   'reindex' — re-derive WIT from already-cached layers
    task TEXT NOT NULL DEFAULT 'pull'
        CHECK (task IN ('pull', 'reindex')),
    -- Current status of this work item.
    status TEXT NOT NULL DEFAULT 'pending'
        CHECK (status IN ('pending', 'in_progress', 'completed', 'failed')),
    -- Lower values are processed first.  0 = normal, -1 = high.
    priority INTEGER NOT NULL DEFAULT 0,
    -- How many times this task has been attempted.
    attempts INTEGER NOT NULL DEFAULT 0,
    -- Maximum number of attempts before marking as 'failed'.
    max_attempts INTEGER NOT NULL DEFAULT 3,
    -- Error message from the most recent failed attempt.
    last_error TEXT,
    -- ISO 8601 timestamp of when this work item was created.
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    -- ISO 8601 timestamp of the most recent modification.
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE TRIGGER trg_fetch_queue_updated_at
    AFTER UPDATE ON fetch_queue
    FOR EACH ROW
    WHEN OLD.updated_at = NEW.updated_at
    BEGIN
        UPDATE fetch_queue
           SET updated_at = CURRENT_TIMESTAMP
         WHERE id = OLD.id;
    END;
CREATE UNIQUE INDEX uq_fetch_queue_item ON fetch_queue(
    registry, repository, tag, task
);
CREATE INDEX idx_fetch_queue_pending ON fetch_queue(status, priority, created_at)
    WHERE status = 'pending';

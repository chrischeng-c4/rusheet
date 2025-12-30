-- Initial database schema for RuSheet

-- Workbooks table
CREATE TABLE IF NOT EXISTS workbooks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Workbook content (JSON storage)
CREATE TABLE IF NOT EXISTS workbook_contents (
    workbook_id UUID PRIMARY KEY REFERENCES workbooks(id) ON DELETE CASCADE,
    content JSONB,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- CRDT updates for collaboration
CREATE TABLE IF NOT EXISTS yrs_updates (
    id BIGSERIAL PRIMARY KEY,
    workbook_id UUID NOT NULL REFERENCES workbooks(id) ON DELETE CASCADE,
    update_data BYTEA NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_yrs_updates_workbook ON yrs_updates(workbook_id);

-- CRDT snapshots for faster loading
CREATE TABLE IF NOT EXISTS yrs_snapshots (
    workbook_id UUID PRIMARY KEY REFERENCES workbooks(id) ON DELETE CASCADE,
    snapshot_data BYTEA NOT NULL,
    update_count INT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

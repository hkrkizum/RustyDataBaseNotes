-- Create views table for storing database view settings (sort, filter, group).
CREATE TABLE IF NOT EXISTS views (
    id                TEXT    PRIMARY KEY NOT NULL,
    database_id       TEXT    NOT NULL REFERENCES databases(id) ON DELETE CASCADE,
    name              TEXT    NOT NULL DEFAULT 'Table',
    view_type         TEXT    NOT NULL DEFAULT 'table',
    sort_conditions   TEXT    NOT NULL DEFAULT '[]',
    filter_conditions TEXT    NOT NULL DEFAULT '[]',
    group_condition   TEXT    DEFAULT NULL,
    collapsed_groups  TEXT    NOT NULL DEFAULT '[]',
    created_at        TEXT    NOT NULL,
    updated_at        TEXT    NOT NULL
);

-- Enforce 1:1 relationship between database and view.
CREATE UNIQUE INDEX idx_views_database_id ON views(database_id);

-- Generate default views for all existing databases.
INSERT INTO views (id, database_id, name, view_type, sort_conditions, filter_conditions, group_condition, collapsed_groups, created_at, updated_at)
SELECT
    lower(hex(randomblob(4))) || '-' || lower(hex(randomblob(2))) || '-4' || substr(lower(hex(randomblob(2))),2) || '-' || substr('89ab', abs(random()) % 4 + 1, 1) || substr(lower(hex(randomblob(2))),2) || '-' || lower(hex(randomblob(6))),
    d.id,
    'Table',
    'table',
    '[]',
    '[]',
    NULL,
    '[]',
    strftime('%Y-%m-%dT%H:%M:%f', 'now') || '+00:00',
    strftime('%Y-%m-%dT%H:%M:%f', 'now') || '+00:00'
FROM databases d
WHERE NOT EXISTS (SELECT 1 FROM views v WHERE v.database_id = d.id);

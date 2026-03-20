CREATE TABLE blocks (
    id         TEXT PRIMARY KEY NOT NULL,
    page_id    TEXT NOT NULL,
    block_type TEXT NOT NULL DEFAULT 'text',
    content    TEXT NOT NULL DEFAULT '',
    position   INTEGER NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (page_id) REFERENCES pages(id) ON DELETE CASCADE
);

CREATE INDEX idx_blocks_page_position ON blocks (page_id, position ASC);

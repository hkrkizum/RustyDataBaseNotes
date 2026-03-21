ALTER TABLE pages ADD COLUMN database_id TEXT REFERENCES databases(id) ON DELETE SET NULL;

CREATE INDEX idx_pages_database_id ON pages (database_id);

CREATE TABLE property_values (
    id             TEXT PRIMARY KEY NOT NULL,
    page_id        TEXT NOT NULL,
    property_id    TEXT NOT NULL,
    text_value     TEXT,
    number_value   REAL,
    date_value     TEXT,
    boolean_value  INTEGER,
    created_at     TEXT NOT NULL,
    updated_at     TEXT NOT NULL,
    FOREIGN KEY (page_id) REFERENCES pages(id) ON DELETE CASCADE,
    FOREIGN KEY (property_id) REFERENCES properties(id) ON DELETE CASCADE
);

CREATE UNIQUE INDEX idx_property_values_page_property
    ON property_values (page_id, property_id);
CREATE INDEX idx_property_values_property_id
    ON property_values (property_id);

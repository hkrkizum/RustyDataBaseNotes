CREATE TABLE properties (
    id            TEXT PRIMARY KEY NOT NULL,
    database_id   TEXT NOT NULL,
    name          TEXT NOT NULL,
    property_type TEXT NOT NULL,
    config        TEXT,
    position      INTEGER NOT NULL,
    created_at    TEXT NOT NULL,
    updated_at    TEXT NOT NULL,
    FOREIGN KEY (database_id) REFERENCES databases(id) ON DELETE CASCADE
);

CREATE INDEX idx_properties_database_id ON properties (database_id, position ASC);
CREATE UNIQUE INDEX idx_properties_name_unique ON properties (database_id, name);

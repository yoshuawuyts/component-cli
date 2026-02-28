-- schema.sql — canonical representation of the full database schema.
--
-- This file is the single source of truth for the database structure.
-- To change the schema, edit this file, then run:
--
--     cargo xtask sql migrate --name <description>
--
-- Never hand-write migration files.
--
-- NOTE: Use CURRENT_TIMESTAMP instead of datetime('now') for DEFAULT values.
-- The sqlite3def tool cannot parse datetime('now') in DDL.

CREATE TABLE migrations (
    id INTEGER PRIMARY KEY,
    version INTEGER NOT NULL UNIQUE,
    applied_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE image (
    id INTEGER PRIMARY KEY,
    ref_registry TEXT NOT NULL,
    ref_repository TEXT NOT NULL,
    ref_mirror_registry TEXT,
    ref_tag TEXT,
    ref_digest TEXT,
    manifest TEXT NOT NULL,
    size_on_disk INTEGER NOT NULL DEFAULT 0,
    package_type TEXT NOT NULL DEFAULT 'component'
);

CREATE UNIQUE INDEX idx_image_unique ON image(
    ref_registry,
    ref_repository,
    COALESCE(ref_tag, ''),
    COALESCE(ref_digest, '')
);

CREATE TABLE known_package (
    id INTEGER PRIMARY KEY,
    registry TEXT NOT NULL,
    repository TEXT NOT NULL,
    description TEXT,
    last_seen_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(registry, repository)
);

CREATE INDEX idx_known_package_repository ON known_package(repository);
CREATE INDEX idx_known_package_registry ON known_package(registry);

CREATE TABLE known_package_tag (
    id INTEGER PRIMARY KEY,
    known_package_id INTEGER NOT NULL,
    tag TEXT NOT NULL,
    last_seen_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    tag_type TEXT NOT NULL DEFAULT 'release',
    FOREIGN KEY (known_package_id) REFERENCES known_package(id) ON DELETE CASCADE,
    UNIQUE(known_package_id, tag)
);

CREATE INDEX idx_known_package_tag_package_id ON known_package_tag(known_package_id);

CREATE TABLE wit_interface (
    id INTEGER PRIMARY KEY,
    wit_text TEXT NOT NULL,
    world_name TEXT,
    import_count INTEGER NOT NULL DEFAULT 0,
    export_count INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    package_name TEXT
);

CREATE UNIQUE INDEX idx_wit_interface_wit_text ON wit_interface(wit_text);
CREATE INDEX idx_wit_interface_package_name ON wit_interface(package_name);

CREATE TABLE image_wit_interface (
    image_id INTEGER NOT NULL,
    wit_interface_id INTEGER NOT NULL,
    PRIMARY KEY (image_id, wit_interface_id),
    FOREIGN KEY (image_id) REFERENCES image(id) ON DELETE CASCADE,
    FOREIGN KEY (wit_interface_id) REFERENCES wit_interface(id) ON DELETE CASCADE
);

CREATE INDEX idx_image_wit_interface_image_id ON image_wit_interface(image_id);
CREATE INDEX idx_image_wit_interface_wit_interface_id ON image_wit_interface(wit_interface_id);

CREATE TABLE _sync_meta (
    `key` TEXT PRIMARY KEY NOT NULL,
    `value` TEXT NOT NULL
);

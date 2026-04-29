-- Rename table: wit_interface → wit_package
ALTER TABLE wit_interface RENAME TO wit_package;

-- Rename table: wit_interface_dependency → wit_package_dependency
ALTER TABLE wit_interface_dependency RENAME TO wit_package_dependency;

-- Rename column: wit_world.wit_interface_id → wit_package_id
ALTER TABLE wit_world RENAME COLUMN wit_interface_id TO wit_package_id;

-- Rename column: wit_world_import.resolved_interface_id → resolved_package_id
ALTER TABLE wit_world_import RENAME COLUMN resolved_interface_id TO resolved_package_id;

-- Rename column: wit_world_export.resolved_interface_id → resolved_package_id
ALTER TABLE wit_world_export RENAME COLUMN resolved_interface_id TO resolved_package_id;

-- Rename column: wit_package_dependency.resolved_interface_id → resolved_package_id
ALTER TABLE wit_package_dependency RENAME COLUMN resolved_interface_id TO resolved_package_id;

-- Recreate indexes with new names and column references.
-- Drop old indexes first, then create new ones.

DROP INDEX IF EXISTS uq_wit_interface;
CREATE UNIQUE INDEX uq_wit_packages ON wit_package(
    package_name,
    COALESCE(version, ''),
    COALESCE(oci_layer_id, -1)
);

DROP INDEX IF EXISTS uq_wit_interface_dependency;
CREATE UNIQUE INDEX uq_wit_package_dependency ON wit_package_dependency(
    dependent_id,
    declared_package,
    COALESCE(declared_version, '')
);

DROP INDEX IF EXISTS idx_wit_iface_name_version;
CREATE INDEX idx_wit_package_name_version ON wit_package(package_name, version);

DROP INDEX IF EXISTS idx_wit_iface_provenance;
CREATE INDEX idx_wit_package_provenance ON wit_package(oci_manifest_id);

DROP INDEX IF EXISTS idx_world_import_resolved;
CREATE INDEX idx_world_import_resolved ON wit_world_import(resolved_package_id);

DROP INDEX IF EXISTS idx_world_export_resolved;
CREATE INDEX idx_world_export_resolved ON wit_world_export(resolved_package_id);

DROP INDEX IF EXISTS idx_wit_dep_declared;
CREATE INDEX idx_wit_dep_declared ON wit_package_dependency(declared_package, declared_version);

DROP INDEX IF EXISTS idx_wit_dep_resolved;
CREATE INDEX idx_wit_dep_resolved ON wit_package_dependency(resolved_package_id);

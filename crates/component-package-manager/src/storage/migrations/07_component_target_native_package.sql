ALTER TABLE "component_target" ADD COLUMN "is_native_package" integer NOT NULL DEFAULT 0;

-- Backfill: set is_native_package = 1 where the declared package matches
-- the parent OCI repository's WIT package (wit_namespace:wit_name).
UPDATE component_target
SET is_native_package = 1
WHERE id IN (
    SELECT ct.id
    FROM component_target ct
    JOIN wasm_component wc ON wc.id = ct.wasm_component_id
    JOIN oci_manifest om ON om.id = wc.oci_manifest_id
    JOIN oci_repository orep ON orep.id = om.oci_repository_id
    WHERE orep.wit_namespace IS NOT NULL
      AND orep.wit_name IS NOT NULL
      AND ct.declared_package = (orep.wit_namespace || ':' || orep.wit_name)
);

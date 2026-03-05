# Manifest

## Manifest Parsing

r[manifest.parse.compact]
The manifest parser MUST support compact dependency notation.

r[manifest.parse.explicit]
The manifest parser MUST support explicit table dependency notation with
registry, namespace, package, and version fields.

r[manifest.parse.empty]
The manifest parser MUST handle empty manifest files.

r[manifest.parse.mixed]
The manifest parser MUST support manifests with both `dependencies.components`
and `dependencies.interfaces` sections.

r[manifest.parse.all-dependencies]
Iterating all dependencies MUST yield both component and interface entries.

r[manifest.parse.permissions]
The manifest parser MUST support sandbox permissions in explicit format
dependencies.

r[manifest.parse.no-permissions]
Dependencies without permissions MUST still parse correctly.

r[manifest.parse.version-constraint]
The manifest parser MUST accept semver version constraints (e.g. `^1.2.3`,
`>=1.0.0, <2.0.0`) in explicit-format dependency version fields.

r[manifest.parse.version-constraint-invalid]
The manifest parser MUST report an error when an explicit dependency has an
invalid semver version constraint string.

r[manifest.parse.version-extract]
Version extraction MUST work for both compact and explicit dependency formats.

r[manifest.parse.version-latest]
The special version string `latest` MUST be accepted without semver validation.

## Manifest Serialization

r[manifest.serialize.compact]
The manifest serializer MUST produce valid TOML in compact format.

r[manifest.serialize.explicit]
The manifest serializer MUST produce valid TOML in explicit format.

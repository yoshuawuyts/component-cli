# wasm-meta-registry Specification

This document defines the requirements for the `wasm-meta-registry` server
crate. Requirements are derived from the existing test suite.

## Registry File Parsing

Each namespace is defined in a separate TOML file with a `[namespace]` table
and optional `[[component]]` and `[[interface]]` entries.

r[registry.parse.interfaces]
A registry file with `[[interface]]` entries MUST parse the namespace and all
interface entries correctly.

r[registry.parse.components]
A registry file with `[[component]]` entries MUST parse the namespace and all
component entries correctly.

r[registry.parse.mixed]
A registry file with both `[[component]]` and `[[interface]]` entries MUST
parse all entries correctly.

r[registry.parse.namespace-only]
A registry file with only a `[namespace]` table and no package entries MUST
parse successfully with empty component and interface lists.

r[registry.parse.invalid-toml]
Invalid TOML input MUST return an error.

r[registry.parse.missing-namespace]
A registry file without a `[namespace]` table MUST return an error.

r[registry.parse.missing-fields]
A package entry missing required fields (e.g., `repository`) MUST return an
error.

## Package Source Conversion

r[registry.sources.convert]
Converting a registry file into package sources MUST produce one
`PackageSource` per entry, with the correct registry, repository, name, and
`PackageKind`.

## Registry Directory Loading

The server loads all `*.toml` files from a registry directory.

r[registry.dir.load]
Loading a registry directory MUST parse all `*.toml` files and combine their
package sources.

r[registry.dir.filename-match]
Loading MUST fail when a file's stem does not match its `namespace.name`.

r[registry.dir.empty]
Loading an empty directory MUST succeed with no packages.

r[registry.dir.ignore-non-toml]
Non-TOML files in the directory MUST be ignored.

## HTTP Server

r[server.health]
The `/v1/health` endpoint MUST return `200 OK` with `{"status": "ok"}`.

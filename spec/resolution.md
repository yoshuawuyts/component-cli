# Dependency Resolution

This section specifies how the package manager resolves version constraints and
computes a complete installation plan before installing any packages.

## Motivation

The current install flow discovers dependencies _during_ installation: it
installs a package, extracts its WIT dependencies, then serially installs each
transitive dependency. This has several drawbacks:

- The lockfile is built incrementally, not upfront.
- No version constraint solving — it just picks "latest stable".
- Dependencies cannot be installed concurrently because they are discovered
  mid-flight.

Pre-planning solves these problems by resolving the full dependency graph
against locally-cached metadata before any network I/O begins.

## Pre-Planned Lockfile

r[resolution.pre-plan]
Before installing any packages, the package manager MUST compute a complete
installation plan by resolving all transitive dependencies using data already
present in the local database. Network fetches MUST NOT begin until the full
plan is known.

r[resolution.lockfile-update]
The lockfile MUST be updated with all resolved packages before any installation
step begins. A partial lockfile (written after each package is installed) MUST
NOT be produced.

## Version Constraint Solving

r[resolution.solver.algorithm]
Version constraint solving MUST use the pubgrub algorithm to guarantee
completeness and produce actionable conflict diagnostics.

r[resolution.solver.wildcard]
When a package declares a dependency without a version constraint, the solver
MUST treat it as compatible with any available version (wildcard). This
degrades gracefully when version data is unavailable.

r[resolution.solver.no-data]
When the local database contains no version or dependency information for a
required package (e.g. because it has never been pulled or synced), the solver
MUST fall back to selecting the latest stable tag available in the known
packages table.

## Sync Before Resolution

r[resolution.sync-first]
The install flow MUST ensure that the local package index is up to date
(within the configured sync interval) before attempting dependency resolution.
If the sync fails and no cached data is available, installation MUST be aborted
with an error.

## Handling Local (OCI URL) Installs

r[resolution.local.download-first]
When installing from a raw OCI URL (not a WIT package name), the package MUST
be downloaded and its WIT metadata extracted before dependency resolution
begins. The extracted dependencies are then resolved using the same algorithm
as named installs.

r[resolution.local.namespace]
A package installed from a raw OCI URL that lacks a WIT namespace MUST be
placed under the `local` namespace for the purposes of the dependency graph.

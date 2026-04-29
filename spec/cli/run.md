# Run Command

The `run` subcommand executes a WebAssembly component.

r[cli.run.help]
The CLI MUST provide `--help` output for the `run` command.

r[run.core-module-rejected]
The run command MUST reject core WebAssembly modules with a clear error message.

r[run.missing-file]
The run command MUST report a clear error when the target file does not exist.

r[run.not-installed]
When the input looks like a manifest key (`scope:component` syntax) but is not
listed in `[dependencies.components]` in `wasm.toml`, the run command MUST auto-install it
into the local project. This creates `wasm.toml` and `wasm.lock.toml` (along
with the standard `vendor/` directories) when they are not already present,
fetches the component from the registry, vendors it under `vendor/wasm/`, and
records the resulting entries in the manifest and lockfile before executing
the component.

r[run.not-installed.global-bypass]
When the `--global/-g` flag is set for an input that looks like a manifest
key (`scope:component` syntax) but is not listed in `[dependencies.components]` in
`wasm.toml`, the run command MUST bypass local installation and run the
component from the global cache instead, fetching from the registry on demand
when the component is not already cached.

r[run.oci-layer-lookup]
When running an OCI reference, the run command MUST retrieve the component
bytes using the `application/wasm` layer digest from the pulled manifest, not
the OCI reference string.

## HTTP world support

r[run.http-world-detection]
The run command MUST auto-detect whether a component targets the
`wasi:http/proxy` world by checking for a `wasi:http/incoming-handler` export.

r[run.http-server]
When a component targets the `wasi:http/proxy` world, the run command MUST
start a local HTTP server that proxies incoming requests to the component.

r[run.http-listen-flag]
The `--listen` flag MUST allow the user to configure the HTTP server bind
address. The default bind address MUST be `127.0.0.1:8080`.

r[run.http-listen-message]
When the HTTP server starts, the run command MUST print the listening address
to stderr.

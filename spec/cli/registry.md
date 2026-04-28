# Registry Command

The `registry` subcommand manages OCI registry operations for WebAssembly
packages.

r[cli.registry.help]
The CLI MUST provide `--help` output for the `registry` command.

r[cli.registry-pull.help]
The CLI MUST provide `--help` output for the `registry pull` command.

r[cli.registry-tags.help]
The CLI MUST provide `--help` output for the `registry tags` command.

r[cli.registry-search.help]
The CLI MUST provide `--help` output for the `registry search` command.

r[cli.registry-sync.help]
The CLI MUST provide `--help` output for the `registry sync` command.

r[cli.registry-notify.help]
The CLI MUST provide `--help` output for the `registry notify` command.

r[cli.registry-delete.help]
The CLI MUST provide `--help` output for the `registry delete` command.

r[cli.registry-list.help]
The CLI MUST provide `--help` output for the `registry list` command.

r[cli.registry-known.help]
The CLI MUST provide `--help` output for the `registry known` command.

r[cli.registry-inspect.help]
The CLI MUST provide `--help` output for the `registry inspect` command.

## Offline Mode

r[cli.offline.registry-blocked]
When `--offline` is set, registry operations MUST fail with a clear error
mentioning offline mode.

r[cli.offline.with-inspect]
The `--offline` flag MUST be accepted alongside the `registry inspect` command.

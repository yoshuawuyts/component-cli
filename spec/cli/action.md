# GitHub Action

The `action.yml` file defines a reusable GitHub Action. Its inputs MUST stay in
sync with the CLI binary.

r[action.commands]
Every subcommand listed in the `command` input description MUST be a valid
`wasm` subcommand (i.e. `wasm <command> --help` MUST succeed).

r[action.global-flags]
Every global CLI flag referenced in an `action.yml` input description MUST
appear in the `wasm --help` output.

r[action.run-flags]
Every `component run`-specific CLI flag referenced in an `action.yml` input
description MUST appear in the `component run --help` output.

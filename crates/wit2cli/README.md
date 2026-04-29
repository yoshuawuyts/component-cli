# wit2cli

Translate a WebAssembly component's WIT exports into a [`clap`] sub-CLI
on the fly.

Given a compiled component (a `.wasm` file), `wit2cli` extracts a
`LibrarySurface` describing every exported function, then builds a
`clap::Command` that mirrors the WIT shape. Parsed `ArgMatches`
become a `Vec<wasmtime::component::Val>` ready to hand off to
wasmtime for invocation.

This crate is the reusable core behind
[`component run`](https://github.com/yoshuawuyts/component-cli)'s
library-style component dispatch.

## Quick start

```rust,no_run
use wit2cli::{build_clap, extract_library_surface, parse_invocation};

let bytes = std::fs::read("my-component.wasm")?;
let surface = extract_library_surface(&bytes)?;
let cmd = build_clap(&surface, "my-tool")?;
let matches = cmd.get_matches();
let invocation = parse_invocation(&matches, &surface)?;
// hand `invocation.path` + `invocation.args` to wasmtime ...
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Mapping reference

The canonical, end-user-facing spec for how WIT types translate into
CLI arguments is the set of **insta snapshots** under
[`tests/snapshots/`](tests/snapshots/). Each fixture's snapshot
documents both the extracted WIT surface and the generated `--help`
tree end-to-end.

To regenerate snapshots after a deliberate mapping change:

```sh
INSTA_UPDATE=always cargo test -p wit2cli --test snapshots
cargo insta review
```

## License

Apache-2.0 WITH LLVM-exception. See [LICENSE](../../LICENSE).

[`clap`]: https://docs.rs/clap

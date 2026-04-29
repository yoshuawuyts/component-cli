# component-frontend

A server-side rendered web frontend for the WebAssembly package registry,
compiled as a `wasm32-wasip2` component targeting `wasi:http`.

## Building

```sh
cargo build -p component-frontend --target wasm32-wasip2
```

To set a custom API base URL (default: `http://localhost:8081`):

```sh
API_BASE_URL=https://registry.example.com cargo build -p component-frontend --target wasm32-wasip2
```

## Running

```sh
wasmtime serve -Scli target/wasm32-wasip2/debug/component_frontend.wasm
```

Then visit <http://localhost:8080> in your browser.

## Architecture

- **Framework**: [wstd-axum](https://github.com/bytecodealliance/wstd) — Axum
  on WASI
- **HTML**: Generated server-side with the [`html`](https://docs.rs/html) crate
- **Styling**: Tailwind CSS (CDN for development)
- **Data**: Fetched from the `component-meta-registry` API via
  `wstd::http::Client`

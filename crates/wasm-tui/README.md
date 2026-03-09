# wasm-tui

Terminal user interface for the WebAssembly developer tools.

## Overview

`wasm-tui` provides the interactive terminal UI for `wasm(1)`. It is driven by
message-passing between the TUI thread and a background manager task, using
`AppEvent` (TUI → manager) and `ManagerEvent` (manager → TUI) channels.

## Usage

```rust
// Launch the full TUI application
wasm_tui::run(offline).await?;
```

## License

Licensed under Apache License, Version 2.0, with LLVM Exceptions.

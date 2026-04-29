# component-tui

Terminal user interface for the WebAssembly developer tools.

## Overview

`component-tui` provides the interactive terminal UI for `component(1)`. It is driven by
message-passing between the TUI thread and a background manager task, using
`AppEvent` (TUI → manager) and `ManagerEvent` (manager → TUI) channels.

## Usage

```rust
// Launch the full TUI application
component_tui::run(offline).await?;
```

## License

Licensed under Apache License, Version 2.0, with LLVM Exceptions.

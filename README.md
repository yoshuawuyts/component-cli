<h1 align="center">wasm(1)</h1>
<div align="center">
  <strong>
    Unified developer tools for WebAssembly
  </strong>
</div>

<br />

<div align="center">
  <!-- Crates version -->
  <a href="https://crates.io/crates/wasm">
    <img src="https://img.shields.io/crates/v/wasm.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/wasm">
    <img src="https://img.shields.io/crates/d/wasm.svg?style=flat-square"
      alt="Download" />
  </a>
  <!-- docs.rs docs -->
  <a href="https://docs.rs/wasm">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
</div>

<div align="center">
  <h3>
    <a href="https://docs.rs/wasm">
      API Docs
    </a>
    <span> | </span>
    <a href="https://github.com/yoshuawuyts/wasm/releases">
      Releases
    </a>
    <span> | </span>
    <a href="https://github.com/yoshuawuyts/wasm/blob/master.github/CONTRIBUTING.md">
      Contributing
    </a>
  </h3>
</div>

## Installation
```sh
$ cargo add wasm
```

## Using `wasm`

```bash
wasm                            # launch interactive TUI
wasm inspect file.wasm          # inspect a Wasm Component
wasm local list                 # list local WASM files in current directory
wasm package pull ghcr.io/...   # pull a package from a registry
wasm package push ghcr.io/...   # push a package to a registry
wasm package list               # list installed packages
wasm self state                 # show storage state info
wasm self clean                 # clean up storage
```

### Shell Completions

Generate shell completions for your preferred shell:

```bash
# Bash
wasm self completions bash > ~/.local/share/bash-completion/completions/wasm

# Zsh
wasm self completions zsh > ~/.zfunc/_wasm

# Fish
wasm self completions fish > ~/.config/fish/completions/wasm.fish
```

### Man Pages

Generate man pages for offline documentation:

```bash
wasm self man-pages --out /usr/local/share/man/man1/
man wasm
```

### Color Support

The CLI supports colored output via the `--color` flag:

```bash
wasm --color auto ...     # automatic color (default)
wasm --color always ...   # always use color
wasm --color never ...    # never use color
```

For more details including environment variable support, run `wasm --help`.

## Storage Layout

```
~/.local/share/wasm/
├── store/          # content-addressable blob storage (image layers)
└── db/
    └── metadata.db3    # sqlite database (package metadata & references)
```

## Status

Experimental. Early development stage — expect breaking changes. 
Contributions and feedback welcome!

## Notes on AI

This project is developed with GitHub Copilot. We believe language models can be 
valuable tools for coding when paired with human oversight, testing, and 
careful review. For transparency, we mention this in the README.

## Safety
This crate uses ``#![deny(unsafe_code)]`` to ensure everything is implemented in
100% Safe Rust.

## Contributing
Want to join us? Check out our ["Contributing" guide][contributing] and take a
look at some of these issues:

- [Issues labeled "good first issue"][good-first-issue]
- [Issues labeled "help wanted"][help-wanted]

[contributing]: https://github.com/yoshuawuyts/wasm/blob/master.github/CONTRIBUTING.md
[good-first-issue]: https://github.com/yoshuawuyts/wasm/labels/good%20first%20issue
[help-wanted]: https://github.com/yoshuawuyts/wasm/labels/help%20wanted

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br/>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>

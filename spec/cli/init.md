# Init Command

The `init` subcommand scaffolds a new project directory.

r[cli.init.help]
The CLI MUST provide `--help` output for the `init` command.

r[init.current-dir]
Running `component init` without arguments MUST create the directory structure,
manifest, and lockfile in the current directory.

r[init.explicit-path]
Running `component init <path>` MUST create the directory structure and files at
the specified path.

r[init.composition-dirs]
Running `component init` MUST create the composition workspace directories:
`types/`, `seams/`, and `build/`.

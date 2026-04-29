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
listed in `[components]` in `wasm.toml`, the run command MUST abort with a
user-friendly error.

r[run.not-installed.hint-cache]
If a copy of the component is available in the local cache, the error MUST
suggest using the `--global/-g` flag.

r[run.not-installed.hint-registry]
If the component is not cached but is found in the package index, the error
MUST suggest using the `--install/-i` flag.

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

## Library-style components

A "library-style" component is a component that exports plain functions or
interfaces but neither targets the `wasi:cli/command` world (it does not
export `wasi:cli/run`) nor the `wasi:http/proxy` world (it does not export
`wasi:http/incoming-handler`). The `run` subcommand MUST be able to invoke
such components by translating their WIT exports into a `clap`-based
sub-CLI.

r[run.library-detection]
When a component exports neither `wasi:http/incoming-handler` nor
`wasi:cli/run`, the run command MUST treat the component as a library-style
component and dispatch to the library-mode runner.

r[run.library-help]
When the user runs `wasm run <INPUT>` with no further arguments, the run
command MUST print a generated help message listing each exported function
or interface as a sub-command. Doc comments declared on each function in
the WIT MUST be used as the sub-command description.

r[run.library-help.dynamic]
When the user passes `-h` or `--help` AFTER the `<INPUT>` argument, the
run command MUST forward the flag to the dynamically generated sub-CLI,
which renders help for the matching component / interface / function.
A `--help` BEFORE `<INPUT>` continues to render the host's own
`component run --help` text.

r[run.library-dispatch]
The run command MUST translate component exports into nested `clap`
sub-commands:
- A free function exported at the world level becomes a top-level
  sub-command named after the function.
- An exported interface becomes a sub-command whose own sub-commands are
  the functions inside that interface.

r[run.library-args]
The run command MUST translate the parameters of each exported function
into `clap` arguments according to the following rules:
- Primitive types (`bool`, signed/unsigned integers, floats, `char`),
  `string`, `variant`, and `enum` parameters become positional arguments.
- `record` parameters become a group of named flags
  (`--<field-name> <VALUE>`). When a function has more than one record
  parameter, fields are prefixed with the parameter name
  (`--<param>-<field>`) to avoid collisions.
- `list<T>` parameters become repeated arguments (e.g.
  `--<name> <V> --<name> <W>`); when the `list<T>` is the last parameter
  it MAY be expressed as a positional variadic argument.
- `option<T>` parameters apply the rule for `T` and make the resulting
  argument optional.
- A `variant` payload value MAY be supplied as `name=<VALUE>`; cases
  without a payload are written as `name`.

r[run.library-output-bytes]
When an exported function returns a `list<u8>` (directly or wrapped in a
`result<list<u8>, _>`), the run command MUST write the bytes to stdout
unmodified, without trailing newlines or framing, so that shell
redirections such as `> file.docx` produce a byte-faithful copy of the
guest's output.

r[run.library-output-other]
When an exported function returns any other type, the run command MUST
render the result to stdout as follows:
- A `string` is written verbatim with no trailing newline.
- Numeric, boolean, and `char` values are rendered using their default
  display format followed by a newline.
- Compound values (records, variants, enums, flags, tuples, lists of
  non-`u8`) are rendered as JSON.

r[run.library-result-err]
When an exported function returns `result::Err(e)`, the run command MUST
render `e` to stderr (as a string for primitive payloads, as JSON for
compound payloads) and exit with a non-zero exit code.

r[run.library-resources-rejected]
When a library-style component exports a resource type or a function
whose signature involves resource handles, the run command MUST abort
with a clear error explaining that resources are not supported.

r[run.host-flags-before-input]
All host-side `wasm run` flags (such as `--global`, `--env`, `--dir`,
`--inherit-env`, `--inherit-network`, `--no-stdio`, `--listen`) MUST be
specified before the `<INPUT>` argument. Any tokens that follow
`<INPUT>` are forwarded to the guest:
- For `wasi:cli/command` components, they are passed as `argv` to the
  guest.
- For library-style components, they are parsed by the dynamically
  built sub-CLI.

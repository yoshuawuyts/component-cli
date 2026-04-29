# Wasm Detection

The `component-detector` crate finds `.wasm` files on the local filesystem.

r[detector.find-wasm]
The detector MUST find all `.wasm` files in a directory tree.

r[detector.target-dir]
The detector MUST find build artifacts in `target/wasm32-*` directories.

r[detector.pkg-dir]
The detector MUST find wasm-pack output in `pkg/` directories.

r[detector.dist-dir]
The detector MUST find jco/JavaScript output in `dist/` directories.

r[detector.entry-methods]
A `WasmEntry` MUST expose the file path and file name.

r[detector.gitignore]
The detector MUST respect `.gitignore` while still including well-known
directories such as `target`, `pkg`, and `dist`.

r[detector.empty-dir]
The detector MUST handle empty directories gracefully, returning no results.

r[detector.convenience]
The `detect()` convenience method MUST return the same results as the iterator.

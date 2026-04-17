//! End-to-end snapshot tests for the `wasm-frontend` HTML output.
//!
//! These tests run the full stack: a `wasm-meta-registry` server is started
//! against a pre-populated data directory (populated once via `wasm install`),
//! the `wasm-frontend` WebAssembly component is built, and `wasmtime serve`
//! runs it. Each test then makes a real HTTP request to the frontend and
//! snapshots the rendered HTML using `insta`.
//!
//! # Running
//!
//! ```sh
//! cargo test --package wasm-frontend-test
//! ```
//!
//! The first run downloads packages from OCI registries and populates a
//! cache at `target/test-fixtures/frontend-snapshot/`. Subsequent runs reuse
//! the cache (~10s instead of ~60s).
//!
//! # Updating snapshots
//!
//! ```sh
//! cargo insta review --package wasm-frontend-test
//! ```
//!
//! # Refreshing the cache
//!
//! ```sh
//! rm -rf target/test-fixtures/frontend-snapshot
//! ```
//!
//! # Requirements
//!
//! - `wasmtime` binary on `PATH`. The `wasmtime serve` command runs the
//!   frontend WebAssembly component.
//! - Network access on first run (to download OCI packages).

#![allow(clippy::print_stderr)]

use std::fmt::Write as _;
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::LazyLock;
use std::thread;
use std::time::{Duration, Instant};

use insta::assert_snapshot;

/// Pinned packages installed once into the test data directory.
///
/// Transitive WIT dependencies (e.g. `wasi:http`, `wasi:cli`, `wasi:io`)
/// are pulled in automatically by the dependency resolver and populated
/// in the data directory as a side effect.
const TEST_PACKAGES: &[&str] = &["ba:sample-wasi-http-rust@0.1.6"];

/// Walk up from `CARGO_MANIFEST_DIR` to find the workspace root.
fn workspace_root() -> PathBuf {
    let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    while !dir.join("Cargo.lock").exists() && dir.pop() {}
    assert!(
        dir.join("Cargo.lock").exists(),
        "could not locate workspace root from {}",
        env!("CARGO_MANIFEST_DIR"),
    );
    dir
}

/// Reserve a free TCP port on localhost.
///
/// Binds to port 0 (kernel-assigned), reads the assigned port, then drops
/// the listener. There is a small race window before the port is reused.
fn reserve_port() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind to ephemeral port");
    listener
        .local_addr()
        .expect("failed to read bound address")
        .port()
}

/// A child process that is killed when dropped.
struct ChildGuard {
    child: Child,
    name: &'static str,
}

impl Drop for ChildGuard {
    fn drop(&mut self) {
        if let Err(e) = self.child.kill()
            && e.kind() != std::io::ErrorKind::InvalidInput
        {
            eprintln!("warning: failed to kill {}: {e}", self.name);
        }
        let _ = self.child.wait();
    }
}

/// Populate the test data directory with pinned packages.
///
/// If the directory already contains a populated SQLite database, this is a
/// no-op. Otherwise, this starts a temporary `wasm-meta-registry` (with
/// sync enabled) and runs `wasm install` against it to download and cache
/// the pinned packages. The temporary registry is stopped before returning.
fn ensure_test_data_dir(root: &Path) -> PathBuf {
    let data_dir = root.join("target/test-fixtures/frontend-snapshot");
    let db_marker = data_dir.join("db/metadata.db3");
    if db_marker.exists() {
        eprintln!(
            "[test setup] reusing cached data dir at {}",
            data_dir.display()
        );
        return data_dir;
    }

    eprintln!(
        "[test setup] populating data dir at {} (first run, ~60s)",
        data_dir.display()
    );
    std::fs::create_dir_all(&data_dir).expect("failed to create data dir");

    // Pre-build the binaries we need so we can exec them directly without
    // running nested `cargo run` from inside the test (which deadlocks on
    // the workspace build lock held by the running `cargo test`).
    build_workspace_binaries(root);

    // `wasm install` resolves WIT-style names against a meta-registry, so
    // we need one running while install executes. Use sync mode so the
    // index is freshly populated from OCI tags.
    let populator_port = reserve_port();
    let log_path = std::env::temp_dir().join(format!(
        "wasm-frontend-test-populator-{}.log",
        std::process::id()
    ));
    let _populator = start_registry_with_sync(root, &data_dir, populator_port, &log_path);
    wait_for_http(
        &format!("http://127.0.0.1:{populator_port}/v1/health"),
        Duration::from_secs(30),
    );
    wait_for_sync_complete(&log_path, Duration::from_secs(180));

    // `wasm install` requires a `wasm.toml` in the working directory.
    let work = tempfile::tempdir().expect("failed to create work tempdir");
    let manifest = build_test_manifest();
    std::fs::write(work.path().join("wasm.toml"), manifest)
        .expect("failed to write test wasm.toml");

    let status = Command::new(workspace_bin(root, "wasm"))
        .arg("--data-dir")
        .arg(&data_dir)
        .arg("--registry-url")
        .arg(format!("http://127.0.0.1:{populator_port}"))
        .arg("install")
        .current_dir(work.path())
        .status()
        .expect("failed to invoke `wasm install`");

    assert!(status.success(), "`wasm install` failed: {status}");
    assert!(
        db_marker.exists(),
        "expected populated database at {} after install",
        db_marker.display(),
    );
    data_dir
}

/// Build a `wasm.toml` requesting the pinned test packages.
fn build_test_manifest() -> String {
    let mut manifest = String::from("[dependencies.components]\n");
    let mut interfaces = String::from("\n[dependencies.interfaces]\n");
    for entry in TEST_PACKAGES {
        let (key, version) = entry.split_once('@').expect("test entry has @version");
        let kind = if key.starts_with("wasi:") {
            &mut interfaces
        } else {
            &mut manifest
        };
        writeln!(kind, "\"{key}\" = \"{version}\"").expect("write to String");
    }
    manifest.push_str(&interfaces);
    manifest
}

/// Start the `wasm-meta-registry` server with `--no-sync`, pointing at the
/// pre-populated data directory.
fn start_registry(root: &Path, data_dir: &Path, port: u16) -> ChildGuard {
    spawn_registry(root, data_dir, port, /* no_sync = */ true, None)
}

/// Start the `wasm-meta-registry` server with sync enabled, writing logs to
/// `log_path`. Used to populate a fresh data directory; callers should use
/// [`wait_for_sync_complete`] to wait for indexing to finish.
fn start_registry_with_sync(
    root: &Path,
    data_dir: &Path,
    port: u16,
    log_path: &Path,
) -> ChildGuard {
    spawn_registry(
        root,
        data_dir,
        port,
        /* no_sync = */ false,
        Some(log_path),
    )
}

fn spawn_registry(
    root: &Path,
    data_dir: &Path,
    port: u16,
    no_sync: bool,
    log_path: Option<&Path>,
) -> ChildGuard {
    let registry_dir = root.join("crates/wasm-frontend-test/fixtures/registry");
    eprintln!("[test setup] starting meta-registry on 127.0.0.1:{port} (no_sync={no_sync})");
    let mut cmd = Command::new(workspace_bin(root, "wasm-meta-registry"));
    cmd.arg(&registry_dir)
        .arg("--bind")
        .arg(format!("127.0.0.1:{port}"))
        .arg("--data-dir")
        .arg(data_dir);
    if no_sync {
        cmd.arg("--no-sync");
    } else {
        cmd.env("RUST_LOG", "info");
    }
    let (stdout_mode, stderr_mode) = if log_path.is_some() {
        (Stdio::piped(), Stdio::null())
    } else {
        (Stdio::null(), Stdio::null())
    };
    let mut child = cmd
        .current_dir(root)
        .stdout(stdout_mode)
        .stderr(stderr_mode)
        .spawn()
        .expect("failed to spawn wasm-meta-registry");

    if let Some(path) = log_path {
        let stdout = child.stdout.take().expect("piped stdout available");
        let path = path.to_owned();
        thread::spawn(move || {
            use std::io::{BufRead, BufReader, Write};
            let mut file = std::fs::File::create(&path).expect("create log");
            let reader = BufReader::new(stdout);
            for line in reader.lines().map_while(Result::ok) {
                let _ = writeln!(file, "{line}");
                let _ = file.flush();
            }
        });
    }

    // Quick sanity check: the process should still be running shortly
    // after spawn. If it died, surface the exit status immediately.
    thread::sleep(Duration::from_millis(500));
    if let Ok(Some(status)) = child.try_wait() {
        panic!("wasm-meta-registry exited early with status {status}");
    }

    ChildGuard {
        child,
        name: "wasm-meta-registry",
    }
}

/// Block until the registry's log file contains "Sync cycle complete".
fn wait_for_sync_complete(log_path: &Path, timeout: Duration) {
    let deadline = Instant::now() + timeout;
    while Instant::now() < deadline {
        if let Ok(contents) = std::fs::read_to_string(log_path)
            && contents.contains("Sync cycle complete")
        {
            return;
        }
        thread::sleep(Duration::from_millis(500));
    }
    panic!(
        "timed out waiting for sync to complete (log: {})",
        log_path.display()
    );
}

/// Build the frontend WebAssembly component.
fn build_frontend(root: &Path, api_port: u16) -> PathBuf {
    eprintln!("[test setup] building wasm-frontend for wasm32-wasip2");
    let api_url = format!("http://127.0.0.1:{api_port}");
    let target_dir = root.join("target/wasm-frontend-test");
    let status = Command::new("cargo")
        .env("API_BASE_URL", &api_url)
        .env("CARGO_TARGET_DIR", &target_dir)
        .current_dir(root)
        .args([
            "build",
            "--package",
            "wasm-frontend",
            "--target",
            "wasm32-wasip2",
            "--quiet",
        ])
        .status()
        .expect("failed to invoke `cargo build`");
    assert!(status.success(), "frontend build failed: {status}");
    target_dir.join("wasm32-wasip2/debug/wasm_frontend.wasm")
}

/// Locate a workspace binary in `target/debug/`.
fn workspace_bin(root: &Path, name: &str) -> PathBuf {
    let path = root.join("target/debug").join(name);
    assert!(
        path.exists(),
        "missing workspace binary {}: run `cargo build -p wasm -p wasm-meta-registry` first",
        path.display(),
    );
    path
}

/// Build the workspace binaries needed by the test harness, using a separate
/// `CARGO_TARGET_DIR` so we don't deadlock against the parent `cargo test`.
fn build_workspace_binaries(root: &Path) {
    eprintln!("[test setup] building wasm + wasm-meta-registry binaries");
    let status = Command::new("cargo")
        .current_dir(root)
        .args([
            "build",
            "--package",
            "wasm",
            "--package",
            "wasm-meta-registry",
            "--quiet",
        ])
        .status()
        .expect("failed to invoke `cargo build`");
    assert!(status.success(), "workspace build failed: {status}");
}

/// Start `wasmtime serve` for the frontend component.
fn start_frontend(wasm_path: &Path, port: u16) -> ChildGuard {
    eprintln!("[test setup] starting wasmtime serve on 127.0.0.1:{port}");
    let child = Command::new("wasmtime")
        .args([
            "serve",
            "--addr",
            &format!("127.0.0.1:{port}"),
            "-Scli",
            "-Sinherit-network",
            "-Sallow-ip-name-lookup",
        ])
        .arg(wasm_path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap_or_else(|e| {
            panic!("failed to spawn `wasmtime serve` (is `wasmtime` on PATH?): {e}")
        });
    ChildGuard {
        child,
        name: "wasmtime serve",
    }
}

/// Block until an HTTP GET against `url` returns any response, or panic
/// after `timeout`.
fn wait_for_http(url: &str, timeout: Duration) {
    let deadline = Instant::now() + timeout;
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(2))
        .build()
        .expect("failed to build reqwest client");
    while Instant::now() < deadline {
        if client.get(url).send().is_ok() {
            return;
        }
        thread::sleep(Duration::from_millis(200));
    }
    panic!("timed out waiting for {url} to come up");
}

/// The shared test stack — populated data dir, running registry, running
/// frontend. Started once per test binary via [`STACK`].
struct TestStack {
    frontend_port: u16,
    // Held to keep the child processes alive for the lifetime of the test
    // binary. `LazyLock` never drops its value, so these run until the
    // process exits.
    _registry: ChildGuard,
    _frontend: ChildGuard,
}

static STACK: LazyLock<TestStack> = LazyLock::new(|| {
    let root = workspace_root();
    let data_dir = ensure_test_data_dir(&root);

    let registry_port = reserve_port();
    let registry = start_registry(&root, &data_dir, registry_port);
    wait_for_http(
        &format!("http://127.0.0.1:{registry_port}/v1/health"),
        Duration::from_secs(30),
    );

    let wasm_path = build_frontend(&root, registry_port);
    let frontend_port = reserve_port();
    let frontend = start_frontend(&wasm_path, frontend_port);
    wait_for_http(
        &format!("http://127.0.0.1:{frontend_port}/health"),
        Duration::from_secs(30),
    );

    TestStack {
        frontend_port,
        _registry: registry,
        _frontend: frontend,
    }
});

/// Fetch a page from the running frontend and return the response body.
fn fetch_page(path: &str) -> String {
    let url = format!("http://127.0.0.1:{}{path}", STACK.frontend_port);
    let resp = reqwest::blocking::get(&url).unwrap_or_else(|e| panic!("GET {url} failed: {e}"));
    resp.text()
        .unwrap_or_else(|e| panic!("failed to read body of {url}: {e}"))
}

/// Normalize the HTML body for snapshotting.
///
/// The frontend's HTML is server-rendered and deterministic apart from
/// references that bake in the registry's port number (none expected) or
/// other environment-specific values. This is a hook for adding such
/// substitutions if they appear in the future.
fn normalize_html(html: &str) -> String {
    html.to_owned()
}

/// Snapshot the rendered HTML at `path`.
fn snapshot_page(name: &str, path: &str) {
    let body = fetch_page(path);
    let normalized = normalize_html(&body);
    assert_snapshot!(name, normalized);
}

// =============================================================================
// Snapshot tests
// =============================================================================

// r[verify frontend.snapshot.home]
#[test]
fn test_home_page() {
    snapshot_page("home", "/");
}

// r[verify frontend.snapshot.namespace]
#[test]
fn test_namespace_page() {
    snapshot_page("namespace_ba", "/ba");
}

// r[verify frontend.snapshot.package_detail]
#[test]
fn test_package_detail() {
    snapshot_page(
        "package_detail_sample_wasi_http_rust",
        "/ba/sample-wasi-http-rust/0.1.6",
    );
}

// r[verify frontend.snapshot.interface_detail]
#[test]
fn test_interface_detail() {
    snapshot_page(
        "interface_wasi_http_types",
        "/wasi/http/0.2.10/interface/types",
    );
}

// r[verify frontend.snapshot.world_detail]
#[test]
fn test_world_detail() {
    snapshot_page("world_wasi_http_proxy", "/wasi/http/0.2.10/world/proxy");
}

// r[verify frontend.snapshot.module_detail]
#[test]
fn test_module_detail() {
    snapshot_page(
        "module_sample_wasi_http_rust",
        "/ba/sample-wasi-http-rust/0.1.6/module/sample_wasi_http_rust.wasm",
    );
}

// r[verify frontend.snapshot.search]
#[test]
fn test_search() {
    snapshot_page("search_http", "/search?q=http");
}

// r[verify frontend.snapshot.not_found]
#[test]
fn test_not_found() {
    snapshot_page("not_found", "/this-route-does-not-exist");
}

//! `cargo xtask serve` — build and serve the frontend with a local meta-registry.
//!
//! Orchestrates three steps:
//! 1. Build `wasm-frontend` for `wasm32-wasip2`.
//! 2. Start `wasm-meta-registry` in the background.
//! 3. Start `wasmtime serve` for the frontend component.
//!
//! On Ctrl-C both child processes are killed so no ports are left open.

use std::io::BufRead;
use std::process::{Child, Command};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use anyhow::{Context, Result};

use crate::workspace_root;

/// Run the full frontend development stack.
pub(crate) fn run_serve() -> Result<()> {
    let root = workspace_root()?;

    let wasm_path = root
        .join("target/wasm32-wasip2/debug/wasm_frontend.wasm")
        .to_str()
        .expect("workspace root path is valid UTF-8")
        .to_owned();

    let registry_dir = root
        .join("registry")
        .to_str()
        .expect("workspace root path is valid UTF-8")
        .to_owned();

    // Initial build.
    build_frontend(&root)?;

    // Start servers.
    let mut registry_child = start_registry(&registry_dir)?;
    let mut wasmtime_child = start_wasmtime(&wasm_path)?;

    // Install a Ctrl-C handler that flags shutdown.
    let shutdown = Arc::new(AtomicBool::new(false));
    let shutdown_flag = Arc::clone(&shutdown);
    ctrlc::set_handler(move || {
        shutdown_flag.store(true, Ordering::SeqCst);
    })
    .context("failed to install Ctrl-C handler")?;

    // Spawn a thread to read stdin for Enter presses.
    let reload = Arc::new(AtomicBool::new(false));
    let reload_flag = Arc::clone(&reload);
    std::thread::spawn(move || {
        let stdin = std::io::stdin();
        for _ in stdin.lock().lines() {
            reload_flag.store(true, Ordering::SeqCst);
        }
    });

    eprintln!(":: Press Enter to rebuild and reload, Ctrl-C to quit.");

    // Wait for either process to exit, Ctrl-C, or Enter.
    loop {
        if shutdown.load(Ordering::SeqCst) {
            eprintln!("\n:: Shutting down…");
            break;
        }

        // Reload on Enter.
        if reload.swap(false, Ordering::SeqCst) {
            eprintln!("\n:: Rebuilding…");
            if build_frontend(&root).is_ok() {
                kill_child(&mut wasmtime_child, "wasmtime serve");
                kill_child(&mut registry_child, "meta-registry");
                registry_child = start_registry(&registry_dir)?;
                wasmtime_child = start_wasmtime(&wasm_path)?;
                eprintln!(":: Press Enter to rebuild and reload, Ctrl-C to quit.");
            } else {
                eprintln!(":: Build failed, keeping current servers running.");
            }
            continue;
        }

        // Check if wasmtime exited on its own.
        if let Some(status) = wasmtime_child
            .try_wait()
            .context("failed to poll wasmtime")?
        {
            eprintln!(":: wasmtime serve exited with {status}");
            break;
        }

        // Check if registry exited on its own.
        if let Some(status) = registry_child
            .try_wait()
            .context("failed to poll meta-registry")?
        {
            eprintln!(":: meta-registry exited with {status}");
            break;
        }

        std::thread::sleep(std::time::Duration::from_millis(200));
    }

    kill_child(&mut wasmtime_child, "wasmtime serve");
    kill_child(&mut registry_child, "meta-registry");

    Ok(())
}

/// Build the frontend component for wasm32-wasip2.
fn build_frontend(root: &std::path::Path) -> Result<()> {
    eprintln!(":: Building wasm-frontend for wasm32-wasip2…");
    let status = Command::new("cargo")
        .env("API_BASE_URL", "http://127.0.0.1:8081")
        .current_dir(root)
        .args([
            "build",
            "--package",
            "wasm-frontend",
            "--target",
            "wasm32-wasip2",
        ])
        .status()
        .context("failed to build wasm-frontend")?;
    if !status.success() {
        anyhow::bail!("cargo build failed with exit code: {:?}", status.code());
    }
    Ok(())
}

/// Start the meta-registry server.
fn start_registry(registry_dir: &str) -> Result<Child> {
    eprintln!(":: Starting meta-registry on 127.0.0.1:8081…");
    Command::new("cargo")
        .args([
            "run",
            "--package",
            "wasm-meta-registry",
            "--",
            registry_dir,
            "--bind",
            "127.0.0.1:8081",
        ])
        .spawn()
        .context("failed to start wasm-meta-registry")
}

/// Start wasmtime serve for the frontend.
fn start_wasmtime(wasm_path: &str) -> Result<Child> {
    eprintln!(":: Starting wasmtime serve on 127.0.0.1:8080…");
    Command::new("wasmtime")
        .args([
            "serve",
            "--addr",
            "127.0.0.1:8080",
            "-Scli",
            "-Sinherit-network",
            "-Sallow-ip-name-lookup",
            wasm_path,
        ])
        .spawn()
        .context("failed to start wasmtime serve")
}

/// Kill a child process, ignoring errors if it already exited.
fn kill_child(child: &mut Child, name: &str) {
    if let Err(e) = child.kill() {
        // "InvalidInput" means the process already exited — that's fine.
        if e.kind() != std::io::ErrorKind::InvalidInput {
            eprintln!("warning: failed to kill {name}: {e}");
        }
    } else {
        let _ = child.wait();
    }
}

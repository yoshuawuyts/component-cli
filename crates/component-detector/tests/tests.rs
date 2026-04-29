//! Integration tests for the component-detector crate.

use component_detector::WasmDetector;
use std::fs::{self, File};
use tempfile::TempDir;

/// Create a test directory structure with some .wasm files
fn setup_test_dir() -> TempDir {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let root = temp_dir.path();

    // Create some regular .wasm files
    fs::create_dir_all(root.join("src")).unwrap();
    File::create(root.join("src/module.wasm")).unwrap();

    // Create .wasm files in target directory (typically gitignored)
    fs::create_dir_all(root.join("target/wasm32-wasip2/release")).unwrap();
    File::create(root.join("target/wasm32-wasip2/release/app.wasm")).unwrap();

    fs::create_dir_all(root.join("target/wasm32-unknown-unknown/debug")).unwrap();
    File::create(root.join("target/wasm32-unknown-unknown/debug/lib.wasm")).unwrap();

    // Create .wasm files in pkg directory (wasm-pack output)
    fs::create_dir_all(root.join("pkg")).unwrap();
    File::create(root.join("pkg/my_crate_bg.wasm")).unwrap();

    // Create .wasm files in dist directory (jco output)
    fs::create_dir_all(root.join("dist")).unwrap();
    File::create(root.join("dist/component.wasm")).unwrap();

    // Create a non-.wasm file
    File::create(root.join("src/main.rs")).unwrap();

    temp_dir
}

// r[verify detector.find-wasm]
#[test]
fn test_detector_finds_wasm_files() {
    let temp_dir = setup_test_dir();
    let detector = WasmDetector::new(temp_dir.path());

    let results: Vec<_> = detector.into_iter().filter_map(Result::ok).collect();

    // Should find all .wasm files
    assert!(
        results.len() >= 5,
        "Expected at least 5 .wasm files, found {}",
        results.len()
    );

    // Verify all results have .wasm extension
    for entry in &results {
        assert!(
            entry.path().extension().is_some_and(|e| e == "wasm"),
            "Expected .wasm extension for {:?}",
            entry.path()
        );
    }
}

// r[verify detector.target-dir]
#[test]
fn test_detector_finds_target_wasm_files() {
    let temp_dir = setup_test_dir();
    let detector = WasmDetector::new(temp_dir.path());

    let results: Vec<_> = detector.into_iter().filter_map(Result::ok).collect();

    // Check that we found files in target directories
    let target_files: Vec<_> = results
        .iter()
        .filter(|e| e.path().to_string_lossy().contains("target"))
        .collect();

    assert!(
        target_files.len() >= 2,
        "Expected at least 2 files in target directory, found {}",
        target_files.len()
    );
}

// r[verify detector.pkg-dir]
#[test]
fn test_detector_finds_pkg_wasm_files() {
    let temp_dir = setup_test_dir();
    let detector = WasmDetector::new(temp_dir.path());

    let results: Vec<_> = detector.into_iter().filter_map(Result::ok).collect();

    // Check that we found files in pkg directory
    let pkg_files: Vec<_> = results
        .iter()
        .filter(|e| e.path().to_string_lossy().contains("pkg"))
        .collect();

    assert_eq!(pkg_files.len(), 1, "Expected 1 file in pkg directory");
}

// r[verify detector.dist-dir]
#[test]
fn test_detector_finds_dist_wasm_files() {
    let temp_dir = setup_test_dir();
    let detector = WasmDetector::new(temp_dir.path());

    let results: Vec<_> = detector.into_iter().filter_map(Result::ok).collect();

    // Check that we found files in dist directory
    let dist_files: Vec<_> = results
        .iter()
        .filter(|e| e.path().to_string_lossy().contains("dist"))
        .collect();

    assert_eq!(dist_files.len(), 1, "Expected 1 file in dist directory");
}

// r[verify detector.entry-methods]
#[test]
fn test_wasm_entry_methods() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let wasm_path = temp_dir.path().join("module.wasm");
    File::create(&wasm_path).unwrap();

    let detector = WasmDetector::new(temp_dir.path());
    let results: Vec<_> = detector.into_iter().filter_map(Result::ok).collect();

    assert_eq!(results.len(), 1);

    #[allow(clippy::indexing_slicing)]
    let entry = &results[0];
    assert!(entry.path().ends_with("module.wasm"));
    assert_eq!(entry.file_name(), Some("module.wasm"));
}

// r[verify detector.gitignore]
#[test]
fn test_detector_with_gitignore() {
    use std::process::Command;

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let root = temp_dir.path();

    // Initialize a git repository so that .gitignore is respected
    Command::new("git")
        .args(["init"])
        .current_dir(root)
        .output()
        .expect("Failed to initialize git repository");

    // Create a .gitignore that ignores the "ignored" directory
    fs::write(root.join(".gitignore"), "ignored/\n").unwrap();

    // Create .wasm files in various locations
    fs::create_dir_all(root.join("src")).unwrap();
    File::create(root.join("src/visible.wasm")).unwrap();

    fs::create_dir_all(root.join("ignored")).unwrap();
    File::create(root.join("ignored/hidden.wasm")).unwrap();

    // Create files in well-known directories (should be included despite gitignore)
    fs::create_dir_all(root.join("target/wasm32-wasip2")).unwrap();
    File::create(root.join("target/wasm32-wasip2/app.wasm")).unwrap();

    let detector = WasmDetector::new(root);
    let results: Vec<_> = detector.into_iter().filter_map(Result::ok).collect();

    // Should find src/visible.wasm and target/wasm32-wasip2/app.wasm
    // but NOT ignored/hidden.wasm
    let paths: Vec<_> = results.iter().map(|e| e.path().to_owned()).collect();

    assert!(
        paths.iter().any(|p| p.ends_with("visible.wasm")),
        "Should find visible.wasm"
    );
    assert!(
        paths.iter().any(|p| p.ends_with("app.wasm")),
        "Should find app.wasm in target"
    );
    assert!(
        !paths.iter().any(|p| p.ends_with("hidden.wasm")),
        "Should NOT find hidden.wasm (gitignored)"
    );
}

// r[verify detector.convenience]
#[test]
fn test_detect_convenience_method() {
    let temp_dir = setup_test_dir();
    let detector = WasmDetector::new(temp_dir.path());

    let results = detector.detect().expect("Detect should succeed");

    assert!(
        results.len() >= 5,
        "Expected at least 5 .wasm files, found {}",
        results.len()
    );
}

// r[verify detector.empty-dir]
#[test]
fn test_detector_empty_directory() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let detector = WasmDetector::new(temp_dir.path());

    let results: Vec<_> = detector.into_iter().filter_map(Result::ok).collect();

    assert!(
        results.is_empty(),
        "Empty directory should yield no results"
    );
}

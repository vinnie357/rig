use std::process::Command;
use tempfile::TempDir;

#[test]
fn test_cli_version() {
    let output = Command::new("cargo")
        .args(&["run", "--bin", "rig", "--", "version"])
        .output()
        .expect("Failed to execute rig command");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("rig"));
}

#[test]
fn test_cli_help() {
    let output = Command::new("cargo")
        .args(&["run", "--bin", "rig", "--", "--help"])
        .output()
        .expect("Failed to execute rig command");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Command-line interface for the Max platform"));
}

#[test]
fn test_cli_status() {
    let output = Command::new("cargo")
        .args(&["run", "--bin", "rig", "--", "status"])
        .output()
        .expect("Failed to execute rig command");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Rig CLI is active"));
}

#[test]
fn test_cli_json_output() {
    let output = Command::new("cargo")
        .args(&["run", "--bin", "rig", "--", "--json", "status"])
        .output()
        .expect("Failed to execute rig command");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    
    // Parse as JSON to ensure it's valid
    let _: serde_json::Value = serde_json::from_str(&stdout)
        .expect("Output should be valid JSON");
}
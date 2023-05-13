use std::process::Command;

#[test]
fn test_scan_successful() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("scan")
        .arg("--target-env")
        .arg("tests/.test-env")
        .arg("SOME_")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("Failed to parse stdout as UTF-8");
    assert_eq!(stdout.trim(), "SOME_APP_KEY=app-key\nSOME_APP_SECRET=app-secret");
}

#[test]
fn test_scan_successful_with_not_found() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("scan")
        .arg("--target-env")
        .arg("tests/.test-env")
        .arg("NOT_FOUND_PREFIX")
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
}

#[test]
fn test_scan_failed_by_not_found_env_path() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("scan")
        .arg("--target-env")
        .arg("tests/.not-found-env")
        .arg("SOME_")
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());

    let stdout = String::from_utf8(output.stderr).expect("Failed to parse stdout as UTF-8");
    assert!(stdout.trim().ends_with("Failed to open the file: tests/.not-found-env"));
}

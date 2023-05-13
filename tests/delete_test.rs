use std::process::Command;

#[test]
fn test_delete_successful() {
    let origin_env_path = "tests/.test-env";
    let target_env_path = "tests/.test-env-delete-successful";
    if let Err(err) = std::fs::copy(origin_env_path, target_env_path) {
        panic!("Fail to copy origin env to test own env: {}", err);
    }

    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("delete")
        .arg("--target-env")
        .arg(target_env_path)
        .arg("SOME_APP_KEY")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());

    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("get")
        .arg("--target-env")
        .arg(target_env_path)
        .arg("SOME_APP_KEY")
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
}

#[test]
fn test_delete_successful_with_not_found() {
    let origin_env_path = "tests/.test-env";
    let target_env_path = "tests/.test-env-delete-successful-with-not-found";
    if let Err(err) = std::fs::copy(origin_env_path, target_env_path) {
        panic!("Fail to copy origin env to test own env: {}", err);
    }

    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("delete")
        .arg("--target-env")
        .arg(target_env_path)
        .arg("NOT_EXIST_KEY")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());

    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("get")
        .arg("--target-env")
        .arg(target_env_path)
        .arg("NOT_EXIST_KEY")
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
}

#[test]
fn test_delete_failed_by_not_found_env_path() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("delete")
        .arg("--target-env")
        .arg("tests/.not-found-env")
        .arg("SOME_APP_KEY")
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());

    let stdout = String::from_utf8(output.stderr).expect("Failed to parse stdout as UTF-8");
    assert!(stdout.trim().ends_with("Failed to open the file: tests/.not-found-env"));
}

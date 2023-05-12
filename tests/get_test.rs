use std::process::Command;

#[test]
fn test_get_with_default_env() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("get")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
}

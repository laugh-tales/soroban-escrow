use std::process::Command;

const VALID_ACCOUNT: &str = "GCEZWKCA5VLDNRLN3RPRJMRZOX3Z6G5CHCGZN36UWBE5XFGT35JA5UMG";
const INVALID_ADDRESS: &str = "GSHORT";

fn get_bin_path() -> &'static str {
    env!("CARGO_BIN_EXE_soroban-toolkit")
}

#[test]
fn test_cli_validate_address_success() {
    let output = Command::new(get_bin_path())
        .args(["address", "validate", VALID_ACCOUNT])
        .output()
        .expect("failed to execute process");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Address is valid"));
}

#[test]
fn test_cli_validate_address_invalid() {
    let output = Command::new(get_bin_path())
        .args(["address", "validate", INVALID_ADDRESS])
        .output()
        .expect("failed to execute process");

    assert!(!output.status.success());
    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("Error: Address has invalid length"));
}

#[test]
fn test_cli_hash_sha256() {
    let output = Command::new(get_bin_path())
        .args(["hash", "sha256", "hello"])
        .output()
        .expect("failed to execute process");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(
        stdout.trim(),
        "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
    );
}

#[test]
fn test_cli_hash_sha512() {
    let output = Command::new(get_bin_path())
        .args(["hash", "sha512", "hello"])
        .output()
        .expect("failed to execute process");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim().len(), 128); // 512-bit hash is 128 hex chars
}

#[test]
fn test_cli_hash_double_sha256() {
    let output = Command::new(get_bin_path())
        .args(["hash", "double-sha256", "hello"])
        .output()
        .expect("failed to execute process");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim().len(), 64);
}

#[test]
fn test_cli_encode_hex() {
    let output = Command::new(get_bin_path())
        .args(["encode", "to-hex", "hello"])
        .output()
        .expect("failed to execute process");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "68656c6c6f");
}

#[test]
fn test_cli_encode_base64() {
    let output = Command::new(get_bin_path())
        .args(["encode", "to-base64", "hello"])
        .output()
        .expect("failed to execute process");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "aGVsbG8=");
}

#[test]
fn test_cli_decode_hex() {
    let output = Command::new(get_bin_path())
        .args(["encode", "from-hex", "68656c6c6f"])
        .output()
        .expect("failed to execute process");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "hello");
}

#[test]
fn test_cli_decode_base64() {
    let output = Command::new(get_bin_path())
        .args(["encode", "from-base64", "aGVsbG8="])
        .output()
        .expect("failed to execute process");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "hello");
}

#[test]
fn test_cli_decode_invalid() {
    let output = Command::new(get_bin_path())
        .args(["encode", "from-hex", "invalid_hex_!!!"])
        .output()
        .expect("failed to execute process");

    assert!(!output.status.success());
    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("Error:"));
}

#[test]
fn test_cli_xlm_to_xlm() {
    let output = Command::new(get_bin_path())
        .args(["tx", "format-xlm", "10000000"])
        .output()
        .expect("failed to execute process");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.trim().contains("XLM"));
}

#[test]
fn test_cli_xlm_to_stroops() {
    let output = Command::new(get_bin_path())
        .args(["tx", "estimate-fee", "100", "1"])
        .output()
        .expect("failed to execute process");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.trim().contains("stroops"));
}

#[test]
fn test_cli_xlm_format() {
    let output = Command::new(get_bin_path())
        .args(["tx", "format-xlm", "10000000"])
        .output()
        .expect("failed to execute process");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "1.0000000 XLM");
}

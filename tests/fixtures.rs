use std::{fs, path::PathBuf};
use patient_everything::validate_everything_json;

fn ex(n: &str) -> String {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.push("examples");
    p.push(n);
    fs::read_to_string(p).unwrap()
}

#[test]
fn valid_fixture() {
    let s = ex("valid-everything-bundle.json");
    let r = validate_everything_json(&s).unwrap();
    assert!(r.is_valid());
}

#[test]
fn invalid_fixture() {
    let s = ex("invalid-everything-bundle.json");
    let r = validate_everything_json(&s).unwrap();
    assert!(!r.is_valid());
}

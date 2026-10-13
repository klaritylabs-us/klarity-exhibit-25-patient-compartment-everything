use std::{env, fs, process::ExitCode};
use patient_everything::validate_everything_json;

fn main() -> ExitCode {
    let path = match env::args().nth(1) {
        Some(p) => p,
        None => {
            eprintln!("usage: validate-everything-bundle <file>");
            return ExitCode::from(2);
        }
    };
    let json = match fs::read_to_string(&path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("read {}: {}", path, e);
            return ExitCode::from(2);
        }
    };
    let report = match validate_everything_json(&json) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::from(2);
        }
    };
    println!("{}", serde_json::to_string_pretty(&report.issues).unwrap_or_else(|_| "[]".into()));
    ExitCode::from(if report.is_valid() { 0 } else { 1 })
}

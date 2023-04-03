use std;

pub fn get_rust_compiler_version() -> String {
    let output = std::process::Command::new("rustc")
        .args(&["--version"])
        .output()
        .expect("Failed to execute rustc command");
    let output = String::from_utf8(output.stdout).unwrap();
    return output;
}

pub fn get_cargo_version() -> String {
    let output = std::process::Command::new("cargo")
        .args(&["--version"])
        .output()
        .expect("Failed to execute cargo command");
    let output = String::from_utf8(output.stdout).unwrap();
    return output;
}

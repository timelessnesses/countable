use std;

pub fn convert_hash_id_to_hyperlink(id: &str) -> String {
    // we got short hash id, convert it to full hash id
    let output = std::process::Command::new("git")
        .args(&["rev-parse", id])
        .output()
        .expect("Failed to execute git command");
    let output = String::from_utf8(output.stdout).unwrap();
    let output = output.trim();
    return format!(
        "https://github.com/timelessnesses/countable/commit/{}",
        output
    );
}

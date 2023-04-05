use std;

fn main() {
    std::fs::write(
        "./src/data/cargo_ver.txt",
        std::process::Command::new("cargo")
            .args(["--version"])
            .output()
            .unwrap()
            .stdout,
    ).unwrap();
    std::fs::write(
        "./src/data/rustc_ver.txt",
        std::process::Command::new("rustc")
        .args(["--version"])
        .output()
        .unwrap()
        .stdout
    ).unwrap();
    std::fs::write(
        "./src/data/current_commit_id.txt",
        std::process::Command::new("git")
        .args(["rev-parse", "--short", "rust"])
        .output()
        .unwrap()
        .stdout
    ).unwrap();
    std::fs::write(
        "./src/data/git_status.txt",
        std::process::Command::new("git")
        .args(["status", "-uno"])
        .output()
        .unwrap()
        .stdout
    ).unwrap();
    std::fs::write(
        "./src/data/last_x_commit_history.txt",
        std::process::Command::new("git")
        .args(["log", "--pretty=%H %s", "-n 5"])
        .output()
        .unwrap()
        .stdout
    ).unwrap();
}

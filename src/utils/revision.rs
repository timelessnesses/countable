use std;

pub fn get_current_commit() -> String {
    let output = std::process::Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
        .expect("Failed to execute git command");
    let output = String::from_utf8(output.stdout).unwrap();
    return output;
}

pub fn get_last_x_history_commits(x: u128) -> String {
    let output = std::process::Command::new("git")
        .args(&["log", "--pretty=oneline", "-n", &x.to_string()])
        .output()
        .expect("Failed to execute git command");
    let output = String::from_utf8(output.stdout).unwrap();
    return output;
}

pub enum BotCurrentStatus {
    Latest,
    Outdated,
    Modified,
}

impl BotCurrentStatus {
    pub fn get_current_status() -> BotCurrentStatus {
        let is_updated = std::process::Command::new("git")
            .args(&["status", "-uno"])
            .output()
            .expect("Failed to execute git command");
        let is_updated = String::from_utf8(is_updated.stdout).unwrap();
        if is_updated.contains("modified") {
            return BotCurrentStatus::Modified;
        } else if is_updated.contains("up to date")
            && is_updated.contains("nothing to commit, working tree clean")
        {
            return BotCurrentStatus::Latest;
        } else {
            return BotCurrentStatus::Outdated;
        }
    }
}

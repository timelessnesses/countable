use crate::utils::github;
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
        .args(&["log", "--pretty=%h %s", "-n", &x.to_string()])
        .output()
        .expect("Failed to execute git command");
    let output = String::from_utf8(output.stdout).unwrap();
    // hyperlink commit hash id to github
    let output = output.split("\n").collect::<Vec<&str>>();
    let mut ids: Vec<String> = vec![];
    let mut commits: Vec<String> = vec![];
    let mut j = String::new();
    for i in output.iter() {
        // commit hash id always the first one else its a commit message
        // something like this: 4b9f9b9 (HEAD -> master, origin/master, origin/HEAD) Update README.md
        let i = i.split(" ").collect::<Vec<&str>>();
        ids.push(i[0].to_owned());
        commits.push(i[1..].join(" "));
    }
    for i in 0..ids.len() {
        j += &format!(
            "[{}]({}) {}\n",
            ids[i],
            github::convert_hash_id_to_hyperlink(ids[i].as_str()),
            format_commit_message(commits[i].as_str())
        )
    }
    return j;
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

fn format_commit_message(message: &str) -> String {
    let mut message = message.to_owned();
    // sometimes commit message too long and it would just overflows and make newline in field and its ugly
    // so we just cut it off
    // maximum is 23 (not including 3 dots)
    return if message.len() > 16 {
        message.truncate(16);
        message += "...";
        message
    } else {
        message
    };
}

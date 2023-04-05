use crate::utils::github;

const CURRENT_COMMIT_ID: &str = include_str!("../data/current_commit_id.txt");
const LAST_5_COMMIT_HISTORY: &str = include_str!("../data/last_x_commit_history.txt");

pub fn get_current_commit() -> String {
    return CURRENT_COMMIT_ID.to_owned();
}

pub fn get_last_5_history_commits() -> String {
    let output = LAST_5_COMMIT_HISTORY.split("\n").collect::<Vec<&str>>();
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
            &(ids[i])[0..6],
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

const GIT_STATUS: &str = include_str!("../data/git_status.txt");

impl BotCurrentStatus {
    pub fn get_current_status() -> BotCurrentStatus {
        let is_updated = GIT_STATUS.to_owned();
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
    if message.len() > 16 {
        message.truncate(16);
        message += "...";
        return message;
    } else {
        return message;
    };
}

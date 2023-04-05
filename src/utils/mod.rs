pub mod datetime;
pub mod github;
pub mod id;
pub mod revision;
pub mod versions;

pub use datetime::format_duration;
pub use github::convert_hash_id_to_hyperlink;
pub use id::generate_id;
pub use revision::{get_current_commit, get_last_5_history_commits};
pub use versions::{get_cargo_version, get_rust_compiler_version};

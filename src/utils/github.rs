pub fn convert_hash_id_to_hyperlink(id: &str) -> String {
    return format!("https://github.com/timelessnesses/countable/commit/{}", id);
}

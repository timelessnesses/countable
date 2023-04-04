use rand::Rng;

pub fn generate_id(length: i64) -> String {
    let mut rng = rand::thread_rng();
    let mut id = String::new();
    for _ in 0..length {
        id.push(rng.gen_range('a'..'z'));
    }
    return id;
}

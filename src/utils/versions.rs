use toml;

// how do i store compiler and cargo ver at build time?

const RUSTC_VERSION: &str = include_str!("../data/rustc_ver.txt");
const CARGO_VER: &str = include_str!("../data/cargo_ver.txt");

pub fn get_rust_compiler_version() -> String {
    return RUSTC_VERSION.to_owned();
}

pub fn get_cargo_version() -> String {
    return CARGO_VER.to_owned();
}

const CARGO_TOML: &str = include_str!("../../Cargo.toml"); // i wonder how rust drop this

pub fn get_serenity_version() -> String {
    let parsed: toml::Value = CARGO_TOML.parse().unwrap();
    let serenity_version = parsed["dependencies"]["serenity"]["version"]
        .as_str()
        .unwrap()
        .to_owned();
    return serenity_version;
}

pub fn get_poise_version() -> String {
    let parsed: toml::Value = CARGO_TOML.parse().unwrap();
    let poise_version = parsed["dependencies"]["poise"].as_str().unwrap().to_owned();
    return poise_version;
}

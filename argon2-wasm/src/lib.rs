use argon2::{self, Config};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn hash_pass(password: &str) -> String {
    let salt = env!("ARGON_SALT");

    let config = Config::default();
    let hash = argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &config).unwrap();
    hash
}
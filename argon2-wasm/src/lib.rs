use argon2::{self, Config};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn hash_pass(password: &str) -> String {
    let config = Config::default();
    let salt = &[];
    let hash = argon2::hash_encoded(password.as_bytes(), salt, &config).unwrap();
    hash
}
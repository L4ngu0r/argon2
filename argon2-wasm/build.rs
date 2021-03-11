use dotenv::dotenv;
use obfstr::hash;

fn main() {
    dotenv().ok();

    let salt: &str = &dotenv::var("ARGON_SALT").unwrap();
    let hash = hash(&salt);

    println!("cargo:rustc-env=ARGON_SALT={}", hash);
}
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    let argon_salt = dotenv::var("ARGON_SALT").unwrap();
    println!("cargo:rustc-env=ARGON_SALT={}", argon_salt);
}
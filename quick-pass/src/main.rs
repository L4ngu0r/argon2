use clap::{Arg, App};
use argon2::{self, Config};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("Argon2 password hashing")
        .version("1.0")
        .arg(Arg::with_name("password")
            .short("p")
            .long("password")
            .takes_value(true))
        .arg(Arg::with_name("salt")
            .short("s")
            .long("salt")
            .takes_value(true))
        .get_matches();

    let password = matches.value_of("password").unwrap().as_bytes();
    let salt = matches.value_of("salt").unwrap().as_bytes();

    let config = Config::default();
    let hash = argon2::hash_encoded(password, salt, &config)?;
    let verify = argon2::verify_encoded(&hash, password)?;
    assert!(verify);

    println!("{:?}", hash);

    Ok(())
}
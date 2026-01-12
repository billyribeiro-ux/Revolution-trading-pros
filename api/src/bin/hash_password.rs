//! Password hasher utility - generates Argon2id hash for database insertion

use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Algorithm, Argon2, Params, PasswordHasher, Version,
};

fn main() {
    let password = "Jesusforevero1!";
    
    let salt = SaltString::generate(&mut OsRng);
    
    let params = Params::new(65536, 3, 4, Some(32)).expect("Invalid params");
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
    
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .expect("Hashing failed");
    
    println!("Password hash for '{}': {}", password, hash);
}

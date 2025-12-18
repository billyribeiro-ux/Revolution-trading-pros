//! One-time script to fix password hash for a user
//! Run with: cargo run --bin fix-password

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};

fn main() {
    let password = "Jesusforever01!";
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .expect("Password hashing failed");
    
    println!("Password: {}", password);
    println!("Hash: {}", hash.to_string());
    println!();
    println!("Run this SQL to update the user:");
    println!("UPDATE users SET password = '{}' WHERE email = 'welberribeirodrums@gmail.com';", hash.to_string());
}

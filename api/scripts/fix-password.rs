//! ICT 11+: Secure password hash generator
//! Run with: cargo run --bin fix-password
//! 
//! NO HARDCODED CREDENTIALS - prompts for input

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use std::io::{self, Write};

fn main() {
    println!("═══════════════════════════════════════════════════════════════");
    println!("  Password Hash Generator (ICT 11+ - No Hardcoded Credentials)");
    println!("═══════════════════════════════════════════════════════════════");
    println!();
    
    // Get email from user
    print!("Enter email: ");
    io::stdout().flush().unwrap();
    let mut email = String::new();
    io::stdin().read_line(&mut email).expect("Failed to read email");
    let email = email.trim();
    
    // Get password from user (note: not hidden, use rpassword crate for production)
    print!("Enter password: ");
    io::stdout().flush().unwrap();
    let mut password = String::new();
    io::stdin().read_line(&mut password).expect("Failed to read password");
    let password = password.trim();
    
    // Generate hash
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .expect("Password hashing failed");
    
    println!();
    println!("Generated hash: {}", hash.to_string());
    println!();
    println!("Run this SQL to update the user:");
    println!("UPDATE users SET password_hash = '{}' WHERE email = '{}';", hash.to_string(), email);
}

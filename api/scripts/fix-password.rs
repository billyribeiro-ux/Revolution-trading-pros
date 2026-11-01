//! ICT 11+: Secure password hash generator
//! Run with: cargo run --bin fix-password
//!
//! NO HARDCODED CREDENTIALS - prompts for input
//! Uses argon2 0.6.0 with OWASP-recommended Argon2id parameters:
//! Params::new(65536, 3, 4, Some(32)) (or Params::new(65536, 3, 4, None))

use argon2::{Algorithm, Argon2, Params, PasswordHash, PasswordHasher, Version};
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
    io::stdin()
        .read_line(&mut email)
        .expect("Failed to read email");
    let email = email.trim();

    // Get password from user (note: not hidden, use rpassword crate for production)
    print!("Enter password: ");
    io::stdout().flush().unwrap();
    let mut password = String::new();
    io::stdin()
        .read_line(&mut password)
        .expect("Failed to read password");
    let password = password.trim();

    // Generate hash using argon2 0.6.0 OWASP parameters
    let params = Params::new(65536, 3, 4, None).expect("Invalid Argon2 params");
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
    let hash: PasswordHash = argon2
        .hash_password(password.as_bytes())
        .expect("Password hashing failed");

    let escaped_email = email.replace('\'', "''");

    println!();
    println!("Generated hash: {hash}");
    println!();
    println!("Run this SQL to update the user:");
    println!("UPDATE users SET password_hash = '{hash}' WHERE email = '{escaped_email}';");
}

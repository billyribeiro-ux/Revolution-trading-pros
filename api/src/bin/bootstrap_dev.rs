//! ICT 7+ Principal Engineer Grade: Developer Bootstrap CLI
//!
//! Single command for secure developer credential management.
//! - Secure password input (hidden, no echo)
//! - Password confirmation
//! - Local Argon2id hashing (OWASP 2024 params)
//! - Direct Fly.io secrets integration
//! - Config-time hash validation
//!
//! Usage:
//!   cargo run --bin bootstrap-dev -- \
//!     --app revolution-trading-pros-api \
//!     --email "your@email.com" \
//!     --name "Your Name"

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, SaltString},
    Argon2, Algorithm, Params, Version,
};
use std::io::{self, Write};
use std::process::Command;

/// OWASP 2024 Argon2id parameters for financial applications
const ARGON2_MEMORY_KIB: u32 = 65536;  // 64 MiB
const ARGON2_ITERATIONS: u32 = 3;
const ARGON2_PARALLELISM: u32 = 4;
const ARGON2_OUTPUT_LEN: usize = 32;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    // Parse arguments
    let mut app: Option<String> = None;
    let mut email: Option<String> = None;
    let mut name: Option<String> = None;
    let mut dry_run = false;
    let mut i = 1;
    
    while i < args.len() {
        match args[i].as_str() {
            "--app" | "-a" => {
                i += 1;
                if i < args.len() { app = Some(args[i].clone()); }
            }
            "--email" | "-e" => {
                i += 1;
                if i < args.len() { email = Some(args[i].clone()); }
            }
            "--name" | "-n" => {
                i += 1;
                if i < args.len() { name = Some(args[i].clone()); }
            }
            "--dry-run" => {
                dry_run = true;
            }
            "--help" | "-h" => {
                print_usage();
                return;
            }
            _ => {
                eprintln!("Unknown argument: {}", args[i]);
                print_usage();
                std::process::exit(1);
            }
        }
        i += 1;
    }
    
    // Validate required arguments
    let app = app.unwrap_or_else(|| {
        eprintln!("Error: --app is required");
        print_usage();
        std::process::exit(1);
    });
    
    let email = email.unwrap_or_else(|| {
        eprintln!("Error: --email is required");
        print_usage();
        std::process::exit(1);
    });
    
    let name = name.unwrap_or_else(|| {
        eprintln!("Error: --name is required");
        print_usage();
        std::process::exit(1);
    });
    
    // Validate email format
    if !email.contains('@') || !email.contains('.') {
        eprintln!("Error: Invalid email format");
        std::process::exit(1);
    }
    
    println!("\n╔═══════════════════════════════════════════════════════════╗");
    println!("║  ICT 7+ Developer Bootstrap - Principal Engineer Grade    ║");
    println!("╚═══════════════════════════════════════════════════════════╝\n");
    println!("  App:   {}", app);
    println!("  Email: {}", email);
    println!("  Name:  {}", name);
    println!();
    
    // Secure password input
    let password = read_password("Enter password: ");
    let password_confirm = read_password("Confirm password: ");
    
    if password != password_confirm {
        eprintln!("\n❌ Error: Passwords do not match");
        std::process::exit(1);
    }
    
    // Validate password strength
    if let Err(e) = validate_password(&password) {
        eprintln!("\n❌ Error: {}", e);
        std::process::exit(1);
    }
    
    println!("\n⏳ Hashing password with Argon2id (OWASP 2024 params)...");
    
    // Hash password
    let password_hash = match hash_password(&password) {
        Ok(h) => h,
        Err(e) => {
            eprintln!("\n❌ Error hashing password: {}", e);
            std::process::exit(1);
        }
    };
    
    // Validate the generated hash can be parsed
    if let Err(e) = validate_hash_format(&password_hash) {
        eprintln!("\n❌ Error: Generated hash is invalid: {}", e);
        std::process::exit(1);
    }
    
    println!("✅ Password hashed successfully");
    println!("\n📋 Hash preview: {}...", &password_hash[..50]);
    
    if dry_run {
        println!("\n🔸 DRY RUN - Would set these secrets on {}:", app);
        println!("   DEVELOPER_BOOTSTRAP_EMAIL={}", email);
        println!("   DEVELOPER_BOOTSTRAP_NAME={}", name);
        println!("   DEVELOPER_BOOTSTRAP_PASSWORD_HASH=<hash>");
        return;
    }
    
    // Check if flyctl is available
    if !check_flyctl() {
        eprintln!("\n❌ Error: flyctl not found. Install with: curl -L https://fly.io/install.sh | sh");
        eprintln!("\nAlternatively, set secrets manually:");
        println!("\nfly secrets set -a {} \\", app);
        println!("  DEVELOPER_BOOTSTRAP_EMAIL=\"{}\" \\", email);
        println!("  DEVELOPER_BOOTSTRAP_NAME=\"{}\" \\", name);
        println!("  DEVELOPER_BOOTSTRAP_PASSWORD_HASH='{}'", password_hash);
        std::process::exit(1);
    }
    
    println!("\n⏳ Setting Fly.io secrets...");
    
    // Set secrets on Fly.io
    let output = Command::new("fly")
        .args([
            "secrets", "set",
            "-a", &app,
            &format!("DEVELOPER_BOOTSTRAP_EMAIL={}", email),
            &format!("DEVELOPER_BOOTSTRAP_NAME={}", name),
            &format!("DEVELOPER_BOOTSTRAP_PASSWORD_HASH={}", password_hash),
        ])
        .output();
    
    match output {
        Ok(out) if out.status.success() => {
            println!("✅ Secrets set successfully");
            println!("\n{}", String::from_utf8_lossy(&out.stdout));
        }
        Ok(out) => {
            eprintln!("\n❌ Error setting secrets:");
            eprintln!("{}", String::from_utf8_lossy(&out.stderr));
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("\n❌ Error running flyctl: {}", e);
            std::process::exit(1);
        }
    }
    
    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║  ✅ Bootstrap Complete                                    ║");
    println!("╚═══════════════════════════════════════════════════════════╝");
    println!("\nThe app will restart automatically with new credentials.");
    println!("Login with: {} / <your password>", email);
}

fn print_usage() {
    println!(r#"
ICT 7+ Developer Bootstrap CLI

USAGE:
    cargo run --bin bootstrap-dev -- [OPTIONS]

OPTIONS:
    -a, --app <APP>       Fly.io app name (required)
    -e, --email <EMAIL>   Developer email (required)
    -n, --name <NAME>     Developer name (required)
    --dry-run             Show what would be set without making changes
    -h, --help            Print help

EXAMPLE:
    cargo run --bin bootstrap-dev -- \
      --app revolution-trading-pros-api \
      --email "your@email.com" \
      --name "Your Name"

SECURITY:
    - Password is read from stdin with echo disabled
    - Password confirmation required
    - Argon2id hash generated locally (never transmitted in plaintext)
    - Hash uses OWASP 2024 recommended parameters
"#);
}

fn read_password(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    
    // Try to use rpassword-style hidden input
    #[cfg(unix)]
    {
        use std::os::unix::io::AsRawFd;
        let stdin_fd = io::stdin().as_raw_fd();
        
        // Get current terminal settings
        let mut termios = std::mem::MaybeUninit::uninit();
        unsafe {
            if libc::tcgetattr(stdin_fd, termios.as_mut_ptr()) == 0 {
                let mut termios = termios.assume_init();
                let orig = termios.c_lflag;
                
                // Disable echo
                termios.c_lflag &= !libc::ECHO;
                libc::tcsetattr(stdin_fd, libc::TCSANOW, &termios);
                
                // Read password
                let mut password = String::new();
                io::stdin().read_line(&mut password).unwrap();
                
                // Restore echo
                termios.c_lflag = orig;
                libc::tcsetattr(stdin_fd, libc::TCSANOW, &termios);
                
                println!(); // Newline after hidden input
                return password.trim().to_string();
            }
        }
    }
    
    // Fallback: read normally (with echo)
    eprintln!("Warning: Terminal echo could not be disabled");
    let mut password = String::new();
    io::stdin().read_line(&mut password).unwrap();
    password.trim().to_string()
}

fn validate_password(password: &str) -> Result<(), &'static str> {
    if password.len() < 12 {
        return Err("Password must be at least 12 characters");
    }
    if password.len() > 128 {
        return Err("Password must be no more than 128 characters");
    }
    
    let has_upper = password.chars().any(|c| c.is_ascii_uppercase());
    let has_lower = password.chars().any(|c| c.is_ascii_lowercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    
    if !has_upper || !has_lower || !has_digit {
        return Err("Password must contain uppercase, lowercase, and a number");
    }
    
    Ok(())
}

fn hash_password(password: &str) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);
    
    let params = Params::new(
        ARGON2_MEMORY_KIB,
        ARGON2_ITERATIONS,
        ARGON2_PARALLELISM,
        Some(ARGON2_OUTPUT_LEN),
    ).map_err(|e| format!("Invalid Argon2 params: {}", e))?;
    
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
    
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| format!("Hashing failed: {}", e))?;
    
    Ok(hash.to_string())
}

fn validate_hash_format(hash: &str) -> Result<(), String> {
    if !hash.starts_with("$argon2id$") {
        return Err("Hash must start with $argon2id$".to_string());
    }
    
    // Try to parse the hash
    PasswordHash::new(hash)
        .map_err(|e| format!("Invalid hash format: {}", e))?;
    
    Ok(())
}

fn check_flyctl() -> bool {
    Command::new("fly")
        .arg("version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

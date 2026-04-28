# Fly.io Configuration Backup — 2026-04-28

**Why this exists:** Fly.io references were removed from the project so we can focus on local dev only. When ready to deploy, restore configurations verbatim from this file.

**Branch that removed it:** `remove-fly-io-references`
**Restore by:** copying each section back to its original path.

---


## Files deleted entirely

### `api/fly.toml`

```toml
# fly.toml - Revolution Trading Pros API
# Fly.io Best Practices Configuration - January 2026
# See https://fly.io/docs/reference/configuration/
#
# Required Fly secrets (set via `fly secrets set KEY=value`):
#   DATABASE_URL          — PostgreSQL connection string (Fly Postgres or external)
#   JWT_SECRET            — HS256 signing secret (≥32 chars)
#   CORS_ORIGINS          — Comma-separated allowed origins
#   R2_ACCESS_KEY_ID      — Cloudflare R2 access key
#   R2_SECRET_ACCESS_KEY  — Cloudflare R2 secret key
#   R2_ENDPOINT           — Cloudflare R2 endpoint URL
#   R2_BUCKET             — Cloudflare R2 bucket name
#   MEILISEARCH_HOST      — Meilisearch instance URL (e.g. https://xyz.meilisearch.io)
#   MEILISEARCH_API_KEY   — Meilisearch API key (master key or search-only key)
#   STRIPE_SECRET_KEY     — Stripe secret key (sk_live_... or sk_test_...)
#   STRIPE_WEBHOOK_SECRET — Stripe webhook signing secret (whsec_...)
#   POSTMARK_TOKEN        — Postmark server API token (for transactional email)
#
# Migration repair note (2026-04-28):
#   Production _sqlx_migrations may still be at max version 30 due to the
#   duplicate-numbered file bug that was fixed locally on 2026-04-28.
#   Before the next `fly deploy`, run the migration registration script:
#     docs/audits/MIGRATION_REPAIR_2026-04-28.md → Phase 3, Step C
#   Or connect to the prod DB via `fly proxy 5432` and execute the INSERT
#   block from that document against the production database.

app = 'revolution-trading-pros-api'
primary_region = 'iad'

[build]
  dockerfile = "Dockerfile"

# Non-secret, non-sensitive environment variables.
# Secrets go in `fly secrets set` — never here.
[env]
  HOST        = "0.0.0.0"
  PORT        = "8080"
  ENVIRONMENT = "production"
  RUST_LOG    = "info,sqlx=warn,tower_http=info"
  # JWT access token TTL in hours. 1h matches the local dev stack.
  # H-1 security fix: was 24h, reduced 2026-04-28.
  JWT_EXPIRES_IN = "1"

[http_service]
  internal_port = 8080
  force_https = true
  auto_stop_machines = 'stop'
  auto_start_machines = true
  min_machines_running = 1
  processes = ['app']

# Health checks — critical for zero-downtime deploys
[[http_service.checks]]
  grace_period = "30s"
  interval = "15s"
  method = "GET"
  path = "/health"
  timeout = "5s"

# VM configuration
[[vm]]
  memory = '1gb'
  cpu_kind = 'shared'
  cpus = 1

```

### `.github/workflows/deploy-fly.yml`

```yaml
name: Deploy API (Fly.io)

# Triggers:
#   - push to main that touches api/** or this workflow
#   - manual dispatch
#
# Required repo secret: FLY_API_TOKEN

on:
  push:
    branches: [main]
    paths:
      - "api/**"
      - ".github/workflows/deploy-fly.yml"
  workflow_dispatch:

jobs:
  deploy:
    name: Deploy to Fly.io
    runs-on: ubuntu-latest
    timeout-minutes: 30

    env:
      FLY_API_TOKEN: ${{ secrets.FLY_API_TOKEN }}
      APP_NAME: revolution-trading-pros-api

    steps:
      - uses: actions/checkout@v4

      - name: Skip if FLY_API_TOKEN missing
        id: gate
        run: |
          if [[ -z "$FLY_API_TOKEN" ]]; then
            echo "::warning::FLY_API_TOKEN secret is not configured. Skipping deploy."
            echo "ready=false" >> "$GITHUB_OUTPUT"
          else
            echo "ready=true" >> "$GITHUB_OUTPUT"
          fi

      - uses: superfly/flyctl-actions/setup-flyctl@master
        if: steps.gate.outputs.ready == 'true'

      - name: Deploy
        if: steps.gate.outputs.ready == 'true'
        working-directory: api
        run: flyctl deploy --remote-only --app "$APP_NAME"

      - name: Wait for app, then health check
        if: steps.gate.outputs.ready == 'true'
        run: |
          # Allow the new VM a moment to stabilize before probing.
          for i in {1..12}; do
            HEALTH=$(curl -s -o /dev/null -w "%{http_code}" "https://$APP_NAME.fly.dev/health" || echo 000)
            if [[ "$HEALTH" == "200" ]]; then
              echo "✓ /health returned 200 after $((i*5))s"
              break
            fi
            echo "  /health returned $HEALTH, retrying in 5s ($i/12)..."
            sleep 5
          done
          if [[ "$HEALTH" != "200" ]]; then
            echo "::error::Health check did not return 200 within 60s"
            exit 1
          fi

      - name: Summary
        if: steps.gate.outputs.ready == 'true'
        run: |
          {
            echo "## ✅ Fly.io deploy complete"
            echo ""
            echo "**App:** $APP_NAME"
            echo "**URL:** https://$APP_NAME.fly.dev"
            echo "**Health:** $(curl -s https://$APP_NAME.fly.dev/health || echo 'unreachable')"
          } >> "$GITHUB_STEP_SUMMARY"

```


## Files modified (Fly URLs stripped)

### `frontend/.env.production` — original Fly lines

The following lines were stripped/replaced with placeholders. They referenced the production Fly.io API URL.

```env
14:VITE_API_URL=https://revolution-trading-pros-api.fly.dev
15:VITE_API_BASE_URL=https://revolution-trading-pros-api.fly.dev
16:VITE_WS_URL=wss://revolution-trading-pros-api.fly.dev
```


### `frontend/.env.production.example` — original Fly lines

```env
18:VITE_API_URL=https://revolution-trading-pros-api.fly.dev
21:VITE_API_BASE_URL=https://revolution-trading-pros-api.fly.dev
28:VITE_WS_URL=wss://revolution-trading-pros.fly.dev
32:VITE_REVERB_HOST=revolution-trading-pros.fly.dev
104:VITE_AI_API_URL=https://revolution-trading-pros.fly.dev/api
```


### Source files: hardcoded fly.dev URLs

The pattern `'https://revolution-trading-pros-api.fly.dev'` was used as a hardcoded fallback in ~80 frontend SvelteKit `+page.server.ts` / `+server.ts` files and a few Rust files. These are being replaced with `'http://localhost:8080'`. The env-driven primary path (`env.API_BASE_URL || env.BACKEND_URL`) is preserved unchanged.

When restoring for Fly deployment:
- Search-and-replace `'http://localhost:8080'` back to `'https://revolution-trading-pros-api.fly.dev'` in fallback positions, OR
- Set `API_BASE_URL` / `BACKEND_URL` env vars at runtime so the fallback never fires.

### Documentation files

The following docs reference Fly.io as historical context for past audits/decisions. Where the reference was prescriptive (telling readers what to do), it was either removed or replaced with "Deploy target TBD". Where the reference is descriptive (recording what was true at the time), the original wording was preserved with a "(historical)" annotation.

Restoration: review each marked doc and re-add Fly-specific instructions when ready to deploy.

### `api/src/bin/bootstrap_dev.rs` — DELETED (was a Fly.io secrets bootstrap CLI tool)

This is a CLI tool that hashed an admin password and pushed it to Fly.io as a secret via `flyctl`. It was only useful in the Fly.io context. Restore by copying back to `api/src/bin/bootstrap_dev.rs` and updating Cargo.toml `[[bin]]` if needed.

```rust
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
    Algorithm, Argon2, Params, Version,
};
use std::io::{self, Write};
use std::process::Command;

/// OWASP 2024 Argon2id parameters for financial applications
const ARGON2_MEMORY_KIB: u32 = 65536; // 64 MiB
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
                if i < args.len() {
                    app = Some(args[i].clone());
                }
            }
            "--email" | "-e" => {
                i += 1;
                if i < args.len() {
                    email = Some(args[i].clone());
                }
            }
            "--name" | "-n" => {
                i += 1;
                if i < args.len() {
                    name = Some(args[i].clone());
                }
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
        eprintln!(
            "\n❌ Error: flyctl not found. Install with: curl -L https://fly.io/install.sh | sh"
        );
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
            "secrets",
            "set",
            "-a",
            &app,
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
    println!(
        r#"
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
"#
    );
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
        // SAFETY: This is a single-threaded dev-only binary; there are no concurrent
        // threads that could invalidate the fd.  File descriptor 0 (stdin) is guaranteed
        // valid by POSIX for the entire lifetime of the process.  libc::tcgetattr is
        // safe to call on any valid, open file descriptor — exactly what fd 0 is here.
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
    )
    .map_err(|e| format!("Invalid Argon2 params: {}", e))?;

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
    PasswordHash::new(hash).map_err(|e| format!("Invalid hash format: {}", e))?;

    Ok(())
}

fn check_flyctl() -> bool {
    Command::new("fly")
        .arg("version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

```


### `docs/setup/DEPLOYMENT_GUIDE.md` — REPLACED with stub

The original guide (29 Fly.io references) covered the full Fly.io deploy workflow. Original content preserved below for restoration.

```markdown
# Deployment Guide - Revolution Trading Pros
**Apple Principal Engineer ICT 7 Grade - January 2026**

Complete deployment and configuration status for production infrastructure.

---

## Production Infrastructure

### Backend API (Fly.io)
- **URL:** https://revolution-trading-pros-api.fly.dev
- **Health Check:** https://revolution-trading-pros-api.fly.dev/health
- **Region:** iad (US East)
- **Machine:** 1GB RAM, shared CPU
- **Status:** ✅ Operational

### Frontend (Cloudflare Pages)
- **URL:** https://revolution-trading-pros.pages.dev
- **Build:** SvelteKit + Tailwind CSS v4
- **Deployment:** Auto-deploy on push to `main`
- **Status:** ✅ Operational

### Database (Fly.io PostgreSQL)
- **App:** revolution-db
- **Database:** postgres
- **Tables:** 72 tables
- **Status:** ✅ Operational

### Cache (Upstash Redis)
- **Instance:** gorgeous-bullfrog-15191
- **Region:** Global
- **Status:** ✅ Operational

---

## Deployment Workflow

### Backend Deployment (Fly.io)

#### Manual Deploy
```bash
cd api
fly deploy --strategy immediate
```

#### Auto Deploy (GitHub Actions)
- **Trigger:** Push to `main` branch with changes in `api/**`
- **Workflow:** `.github/workflows/deploy-fly.yml`
- **Health Check:** Verifies `/health` endpoint returns 200

#### Deployment Strategies
```bash
# Immediate (fastest, all machines at once)
fly deploy --strategy immediate

# Rolling (default, one at a time)
fly deploy --strategy rolling

# Canary (safest, test one machine first)
fly deploy --strategy canary
```

### Frontend Deployment (Cloudflare Pages)

#### Auto Deploy
- **Trigger:** Push to `main` branch
- **Build Command:** `pnpm run build`
- **Output Directory:** `.svelte-kit/cloudflare`
- **Framework:** SvelteKit

#### Manual Deploy
```bash
cd frontend
pnpm run build
wrangler pages deploy .svelte-kit/cloudflare
```

---

## Configuration Status

### ✅ Services Configured

#### Bunny.net (Video Streaming)
- Stream Library: 585929
- CDN: vz-5a23b520-193.b-cdn.net
- Storage Zone: revolution-trading-downloads
- Webhook: Configured

#### Cloudflare R2 (File Storage)
- Bucket: revolution-trading-media
- Public URL: https://pub-2e5bd1b702b440bd888a0fc47f3493ae.r2.dev
- CORS: Enabled for frontend domain

#### Stripe (Payments)
- Mode: Test
- Webhook: Configured
- Products: Membership plans synced

#### Meilisearch (Search)
- Host: https://ms-275da497c3a5-36675.nyc.meilisearch.io
- Indexes: courses, indicators, posts
- Status: Active

#### Postmark (Email)
- From: noreply@revolution-trading-pros.pages.dev
- Status: Placeholder (not configured)

---

## Environment Variables

### Backend (Fly.io Secrets)
```bash
# View all secrets
fly secrets list -a revolution-trading-pros-api

# Set secret
fly secrets set KEY=value -a revolution-trading-pros-api

# Import from file
fly secrets import < api/.env -a revolution-trading-pros-api
```

**Required Secrets:**
- DATABASE_URL
- REDIS_URL
- JWT_SECRET
- STRIPE_SECRET
- BUNNY_STREAM_API_KEY
- R2_ACCESS_KEY_ID
- R2_SECRET_ACCESS_KEY
- MEILISEARCH_API_KEY

### Frontend (Cloudflare Pages)
**Set in:** Cloudflare Dashboard → Pages → Settings → Environment Variables

**Required Variables:**
- VITE_API_URL
- VITE_CDN_URL
- VITE_GTM_ID
- PUBLIC_GA4_MEASUREMENT_ID

---

## Database Migrations

### Automatic Migrations
Migrations run automatically on API startup via `sqlx::migrate!()`.

### Manual Migrations
```bash
cd api
sqlx migrate run
```

### Migration Status
```bash
# Check applied migrations
fly ssh console -a revolution-trading-pros-api
psql $DATABASE_URL -c "SELECT * FROM _sqlx_migrations ORDER BY version;"
```

### Current Schema
- **Version:** 21 migrations applied
- **Tables:** 72 tables
- **Key Tables:** users, trading_rooms, room_weekly_videos, courses_enhanced

---

## Health Checks

### Backend Health Check
```bash
curl https://revolution-trading-pros-api.fly.dev/health
# Response: {"status":"healthy","version":"0.1.0","environment":"development"}
```

### Detailed Health Check
```bash
curl https://revolution-trading-pros-api.fly.dev/health/detailed
# Returns: DB status, Redis status, Storage status
```

### Fly.io Health Check Configuration
**File:** `api/fly.toml`
```toml
[[http_service.checks]]
  grace_period = "10s"
  interval = "15s"
  method = "GET"
  path = "/health"
  timeout = "5s"
```

---

## Monitoring & Logs

### Backend Logs
```bash
# Real-time logs
fly logs -a revolution-trading-pros-api

# Last 100 lines
fly logs -a revolution-trading-pros-api --no-tail | tail -100

# Filter by level
fly logs -a revolution-trading-pros-api | grep ERROR
```

### Frontend Logs
- **Location:** Cloudflare Dashboard → Pages → Deployments → Logs
- **Real-time:** Available during build and deployment

### Database Logs
```bash
fly logs -a revolution-db
```

---

## Scaling

### Backend Scaling
```bash
# Scale to 2 machines
fly scale count 2 -a revolution-trading-pros-api

# Scale memory
fly scale memory 2048 -a revolution-trading-pros-api

# Add region
fly regions add lhr -a revolution-trading-pros-api
```

### Current Configuration
- **Machines:** 1
- **Memory:** 1GB
- **CPU:** Shared
- **Regions:** iad (US East)

---

## Rollback Procedures

### Backend Rollback
```bash
# List recent deployments
fly releases -a revolution-trading-pros-api

# Rollback to previous version
fly releases rollback -a revolution-trading-pros-api
```

### Frontend Rollback
1. Go to Cloudflare Dashboard → Pages
2. Select deployment
3. Click "Rollback to this deployment"

### Database Rollback
```bash
# Revert last migration
cd api
sqlx migrate revert
```

---

## Security Checklist

- [x] HTTPS enforced on all endpoints
- [x] CORS configured for production domains only
- [x] JWT secrets rotated and secure
- [x] Database credentials in Fly.io secrets
- [x] API rate limiting enabled (Redis)
- [x] SQL injection protection (SQLx compile-time checks)
- [x] XSS protection (Content-Security-Policy headers)
- [x] Secrets never committed to git

---

## Troubleshooting

### API Not Responding
```bash
# Check machine status
fly status -a revolution-trading-pros-api

# Restart machine
fly machine restart MACHINE_ID -a revolution-trading-pros-api

# Check logs for errors
fly logs -a revolution-trading-pros-api | grep ERROR
```

### Database Connection Issues
```bash
# Check database status
fly status -a revolution-db

# Restart database
fly machine restart MACHINE_ID -a revolution-db

# Test connection
fly ssh console -a revolution-trading-pros-api
echo $DATABASE_URL
```

### Frontend Build Failures
1. Check Cloudflare Pages build logs
2. Verify `package.json` scripts
3. Check for missing environment variables
4. Test build locally: `pnpm run build`

---

## Deployment Checklist

### Pre-Deployment
- [ ] All tests passing
- [ ] Migrations tested locally
- [ ] Environment variables updated
- [ ] Secrets configured
- [ ] Health checks verified

### Post-Deployment
- [ ] Health check returns 200
- [ ] Frontend loads correctly
- [ ] Database migrations applied
- [ ] API endpoints responding
- [ ] No errors in logs
- [ ] Video upload working
- [ ] Payments processing

---

**Last Updated:** January 24, 2026
**Deployment Status:** ✅ Production Ready

```

### `docs/setup/SETUP_GUIDE.md` — Fly secrets/postgres sections stripped

The original guide had Fly-specific webhooks, Postgres connection, and `fly secrets set` instructions. The local-dev portions are kept; deploy-specific parts removed.

Original Fly-specific lines:
```
126:fly postgres connect -a <your-db-host>
129:fly secrets list -a revolution-trading-pros-api | grep DATABASE_URL
175:fly secrets set DATABASE_URL="postgres://..." -a revolution-trading-pros-api
176:fly secrets set REDIS_URL="rediss://..." -a revolution-trading-pros-api
177:fly secrets set JWT_SECRET="your-secret" -a revolution-trading-pros-api
180:fly secrets import < api/.env -a revolution-trading-pros-api
```

### `api/COMPLETE_DEPLOYMENT.sh` — DELETED (Fly deploy script)

```bash
#!/bin/bash
# Complete Deployment Script - Revolution Trading Pros API
# This script handles everything: CLI installation, deployment, and verification

set -e

APP_NAME="revolution-trading-pros-api"
API_URL="https://$APP_NAME.fly.dev"

echo "🚀 Revolution Trading Pros API - Complete Deployment"
echo "====================================================="
echo ""

# Step 1: Check/Install Fly.io CLI
echo "📦 Step 1: Checking Fly.io CLI..."
if command -v flyctl &> /dev/null; then
    echo "✅ Fly.io CLI already installed"
    FLYCTL_CMD="flyctl"
elif command -v fly &> /dev/null; then
    echo "✅ Fly.io CLI already installed"
    FLYCTL_CMD="fly"
elif [ -f "$HOME/.fly/bin/flyctl" ]; then
    echo "✅ Fly.io CLI found in ~/.fly/bin"
    export PATH="$HOME/.fly/bin:$PATH"
    FLYCTL_CMD="flyctl"
else
    echo "⚠️  Fly.io CLI not found. Please install manually:"
    echo ""
    echo "Run these commands:"
    echo "  curl -L https://fly.io/install.sh | sh"
    echo "  export PATH=\"\$HOME/.fly/bin:\$PATH\""
    echo "  echo 'export PATH=\"\$HOME/.fly/bin:\$PATH\"' >> ~/.zshrc"
    echo "  flyctl auth login"
    echo ""
    echo "Then run this script again."
    exit 1
fi

echo ""

# Step 2: Check Authentication
echo "🔐 Step 2: Checking authentication..."
if ! $FLYCTL_CMD auth whoami &> /dev/null; then
    echo "❌ Not authenticated with Fly.io"
    echo "Please run: $FLYCTL_CMD auth login"
    exit 1
fi
echo "✅ Authenticated with Fly.io"
echo ""

# Step 3: Check Current Status
echo "📊 Step 3: Checking current deployment status..."
if $FLYCTL_CMD status -a $APP_NAME &> /dev/null; then
    echo "✅ App exists: $APP_NAME"
    $FLYCTL_CMD status -a $APP_NAME
else
    echo "⚠️  App does not exist. Creating..."
    $FLYCTL_CMD apps create $APP_NAME --org personal || true
fi
echo ""

# Step 4: Check Secrets
echo "🔐 Step 4: Checking environment variables..."
echo "Current secrets:"
$FLYCTL_CMD secrets list -a $APP_NAME || echo "No secrets set yet"
echo ""
echo "⚠️  Ensure these secrets are set:"
echo "  - DATABASE_URL"
echo "  - REDIS_URL"
echo "  - JWT_SECRET"
echo "  - STRIPE_SECRET"
echo "  - STRIPE_WEBHOOK_SECRET"
echo "  - POSTMARK_TOKEN"
echo "  - MEILISEARCH_API_KEY"
echo ""
read -p "Are all required secrets set? (y/n) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Please set secrets using: $FLYCTL_CMD secrets set KEY=VALUE -a $APP_NAME"
    exit 1
fi
echo ""

# Step 5: Deploy
echo "🚀 Step 5: Deploying latest code..."
echo "This will build the Docker image and deploy to Fly.io..."
$FLYCTL_CMD deploy -a $APP_NAME --remote-only

echo ""
echo "⏳ Waiting for deployment to stabilize..."
sleep 15
echo ""

# Step 6: Verify Health
echo "🏥 Step 6: Verifying deployment health..."
echo "Testing /health endpoint..."
HEALTH_RESPONSE=$(curl -s -w "\n%{http_code}" $API_URL/health)
HEALTH_BODY=$(echo "$HEALTH_RESPONSE" | head -n 1)
HEALTH_CODE=$(echo "$HEALTH_RESPONSE" | tail -n 1)

if [ "$HEALTH_CODE" = "200" ]; then
    echo "✅ Health check passed: $HEALTH_BODY"
else
    echo "❌ Health check failed with code: $HEALTH_CODE"
    echo "Response: $HEALTH_BODY"
    echo ""
    echo "Checking logs..."
    $FLYCTL_CMD logs -a $APP_NAME
    exit 1
fi
echo ""

# Step 7: Verify Readiness
echo "🔍 Step 7: Verifying database connectivity..."
echo "Testing /ready endpoint..."
READY_RESPONSE=$(curl -s -w "\n%{http_code}" $API_URL/ready)
READY_BODY=$(echo "$READY_RESPONSE" | head -n 1)
READY_CODE=$(echo "$READY_RESPONSE" | tail -n 1)

if [ "$READY_CODE" = "200" ]; then
    echo "✅ Ready check passed: $READY_BODY"
else
    echo "❌ Ready check failed with code: $READY_CODE"
    echo "Response: $READY_BODY"
    echo "Database connection may be failing"
    exit 1
fi
echo ""

# Step 8: Database Setup
echo "🗄️  Step 8: Setting up database..."
echo "Running /setup-db endpoint..."
SETUP_RESPONSE=$(curl -s -X POST -w "\n%{http_code}" $API_URL/setup-db)
SETUP_BODY=$(echo "$SETUP_RESPONSE" | head -n 1)
SETUP_CODE=$(echo "$SETUP_RESPONSE" | tail -n 1)

if [ "$SETUP_CODE" = "200" ]; then
    echo "✅ Database setup: $SETUP_BODY"
else
    echo "⚠️  Setup response code: $SETUP_CODE"
    echo "Response: $SETUP_BODY"
    echo "(This may be expected if already set up)"
fi
echo ""

# Step 9: Run Migrations
echo "📝 Step 9: Running migrations..."
echo "Running /run-migrations endpoint..."
MIGRATION_RESPONSE=$(curl -s -X POST -w "\n%{http_code}" $API_URL/run-migrations)
MIGRATION_BODY=$(echo "$MIGRATION_RESPONSE" | head -n 1)
MIGRATION_CODE=$(echo "$MIGRATION_RESPONSE" | tail -n 1)

if [ "$MIGRATION_CODE" = "200" ]; then
    echo "✅ Migrations completed: $MIGRATION_BODY"
else
    echo "⚠️  Migration response code: $MIGRATION_CODE"
    echo "Response: $MIGRATION_BODY"
    echo "(This may be expected if already run)"
fi
echo ""

# Step 10: Test Authentication
echo "🔑 Step 10: Testing authentication..."
echo "Attempting superadmin login..."
LOGIN_RESPONSE=$(curl -s -X POST -w "\n%{http_code}" $API_URL/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"welberribeirodrums@gmail.com","password":"Revolution2024!"}')
LOGIN_BODY=$(echo "$LOGIN_RESPONSE" | head -n 1)
LOGIN_CODE=$(echo "$LOGIN_RESPONSE" | tail -n 1)

if [ "$LOGIN_CODE" = "200" ]; then
    echo "✅ Authentication working"
    echo "Login response: $LOGIN_BODY"
else
    echo "⚠️  Login response code: $LOGIN_CODE"
    echo "Response: $LOGIN_BODY"
    echo "(Superadmin may need to be created or password may be different)"
fi
echo ""

# Step 11: Check Logs
echo "📋 Step 11: Checking recent logs for errors..."
echo "Last 50 lines:"
$FLYCTL_CMD logs -a $APP_NAME | tail -n 50
echo ""

# Final Summary
echo "✅ =============================================="
echo "✅ DEPLOYMENT COMPLETE"
echo "✅ =============================================="
echo ""
echo "📊 Deployment Summary:"
echo "  • App Name: $APP_NAME"
echo "  • URL: $API_URL"
echo "  • Health: $API_URL/health"
echo "  • Ready: $API_URL/ready"
echo "  • Version: 0.1.0"
echo "  • Environment: production"
echo ""
echo "🔗 Quick Links:"
echo "  • Dashboard: https://fly.io/apps/$APP_NAME"
echo "  • Metrics: https://fly.io/apps/$APP_NAME/metrics"
echo "  • Logs: flyctl logs -a $APP_NAME"
echo ""
echo "🧪 Test Endpoints:"
echo "  curl $API_URL/health"
echo "  curl $API_URL/ready"
echo "  curl -X POST $API_URL/api/auth/login -H 'Content-Type: application/json' -d '{\"email\":\"welberribeirodrums@gmail.com\",\"password\":\"Revolution2024!\"}'"
echo ""
echo "✅ All systems operational!"
```

### `api/DEPLOY_NOW.sh` — DELETED

```bash
#!/bin/bash
# Quick Deployment Script for Revolution Trading Pros API
# Run this after installing Fly.io CLI

set -e

APP_NAME="revolution-trading-pros-api"

echo "🚀 Revolution Trading Pros API - Quick Deploy"
echo "=============================================="
echo ""

# Check if flyctl is installed
if ! command -v flyctl &> /dev/null; then
    echo "❌ Fly.io CLI not found. Installing..."
    curl -L https://fly.io/install.sh | sh
    export PATH="$HOME/.fly/bin:$PATH"
    echo "✅ Fly.io CLI installed"
    echo ""
    echo "⚠️  Please run: export PATH=\"\$HOME/.fly/bin:\$PATH\""
    echo "⚠️  Then run this script again"
    exit 1
fi

# Check if authenticated
if ! flyctl auth whoami &> /dev/null; then
    echo "🔐 Not authenticated. Please login..."
    flyctl auth login
fi

echo "📊 Checking current deployment status..."
flyctl status -a $APP_NAME || echo "App may not exist yet"
echo ""

echo "🔨 Building and deploying..."
flyctl deploy -a $APP_NAME --remote-only

echo ""
echo "✅ Deployment initiated!"
echo ""
echo "🔍 Checking health..."
sleep 10

# Test health endpoint
echo "Testing /health endpoint..."
HEALTH_RESPONSE=$(curl -s https://$APP_NAME.fly.dev/health || echo "failed")
echo "Response: $HEALTH_RESPONSE"
echo ""

# Test ready endpoint
echo "Testing /ready endpoint..."
READY_RESPONSE=$(curl -s https://$APP_NAME.fly.dev/ready || echo "failed")
echo "Response: $READY_RESPONSE"
echo ""

if [[ $HEALTH_RESPONSE == *"healthy"* ]]; then
    echo "✅ API is healthy and running!"
    echo ""
    echo "🌐 API URL: https://$APP_NAME.fly.dev"
    echo "🏥 Health: https://$APP_NAME.fly.dev/health"
    echo "📊 Ready: https://$APP_NAME.fly.dev/ready"
    echo ""
    echo "📋 Next steps:"
    echo "1. Run database setup: curl -X POST https://$APP_NAME.fly.dev/setup-db"
    echo "2. Run migrations: curl -X POST https://$APP_NAME.fly.dev/run-migrations"
    echo "3. Test login: curl -X POST https://$APP_NAME.fly.dev/api/auth/login -H 'Content-Type: application/json' -d '{\"email\":\"welberribeirodrums@gmail.com\",\"password\":\"Revolution2024!\"}'"
else
    echo "⚠️  Health check failed. Checking logs..."
    flyctl logs -a $APP_NAME
    echo ""
    echo "❌ Deployment may have issues. Please check logs above."
    exit 1
fi
```

### `api/scripts/deploy.sh` — DELETED

```bash
#!/bin/bash
#
# Revolution Trading Pros - Rust API Deployment Script
# Usage: ./scripts/deploy.sh [command]
#
# Commands:
#   setup    - Initial setup (create app, database)
#   deploy   - Deploy the application
#   secrets  - Set secrets from .env file
#   logs     - View application logs
#   ssh      - SSH into running machine
#

set -e

APP_NAME="revolution-trading-pros-api"

case "${1:-deploy}" in
    setup)
        echo "🚀 Setting up Fly.io application..."

        # Create app
        fly apps create $APP_NAME --org personal || true

        echo "✅ App created: $APP_NAME"
        echo ""
        echo "📋 Next steps:"
        echo "1. Create Fly.io Postgres: fly postgres create --name rtp-postgres"
        echo "2. Attach to app: fly postgres attach --app $APP_NAME rtp-postgres"
        echo "3. Set up Upstash Redis: https://console.upstash.com"
        echo "4. Run: ./scripts/deploy.sh secrets"
        echo "5. Run: ./scripts/deploy.sh deploy"
        ;;

    secrets)
        echo "🔐 Setting secrets..."

        if [ ! -f .env ]; then
            echo "❌ .env file not found. Copy .env.example to .env and fill in values."
            exit 1
        fi

        # Read .env and set secrets
        while IFS='=' read -r key value; do
            # Skip comments and empty lines
            [[ $key =~ ^#.*$ ]] && continue
            [[ -z $key ]] && continue

            # Remove quotes from value
            value="${value%\"}"
            value="${value#\"}"

            echo "Setting $key..."
            fly secrets set "$key=$value" -a $APP_NAME
        done < .env

        echo "✅ Secrets set"
        ;;

    deploy)
        echo "🚀 Deploying to Fly.io..."
        fly deploy -a $APP_NAME
        echo "✅ Deployed!"
        echo ""
        echo "🌐 Your API is live at: https://$APP_NAME.fly.dev"
        echo "🏥 Health check: https://$APP_NAME.fly.dev/health"
        ;;

    logs)
        echo "📋 Viewing logs..."
        fly logs -a $APP_NAME
        ;;

    ssh)
        echo "🔌 Connecting via SSH..."
        fly ssh console -a $APP_NAME
        ;;

    status)
        echo "📊 Application status..."
        fly status -a $APP_NAME
        ;;

    *)
        echo "Usage: ./scripts/deploy.sh [setup|secrets|deploy|logs|ssh|status]"
        exit 1
        ;;
esac
```


### `.github/workflows/README.md` and `TECHNICAL_NOTES.md` — Fly sections referenced deleted `deploy-fly.yml`

These docs documented the deploy-fly.yml workflow. Lines mentioning FLY_API_TOKEN are no longer relevant.

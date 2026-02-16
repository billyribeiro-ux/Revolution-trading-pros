# Authentication & Security

**Enterprise-Grade Auth System**

---

## 📋 Overview

Revolution Trading Pros implements a comprehensive authentication system with OAuth, multi-factor authentication (MFA), and session management.

### Key Features

- **🔐 OAuth 2.0** - Google & Apple Sign-In
- **📧 Email/Password** - Traditional authentication
- **🔑 Multi-Factor Auth (MFA)** - TOTP-based 2FA
- **🎫 JWT Tokens** - Stateless authentication
- **🔄 Session Management** - Redis-backed sessions
- **🛡️ Rate Limiting** - Brute-force protection
- **🔒 Password Security** - Argon2id hashing

---

## 🏗️ Architecture

### Frontend Routes

```
frontend/src/routes/
├── login/+page.svelte           # Login page
├── signup/+page.svelte          # Registration
├── forgot-password/+page.svelte # Password reset
├── reset-password/+page.svelte  # Reset form
└── dashboard/settings/security/ # MFA setup
```

### Backend API Endpoints

```
POST /api/auth/register          # Create account
POST /api/auth/login             # Email/password login
POST /api/auth/logout            # Logout
POST /api/auth/refresh           # Refresh token
POST /api/auth/forgot-password   # Request reset
POST /api/auth/reset-password    # Reset password
GET  /api/auth/google            # Google OAuth
GET  /api/auth/google/callback   # Google callback
GET  /api/auth/apple             # Apple OAuth
GET  /api/auth/apple/callback    # Apple callback
POST /api/auth/mfa/setup         # Setup MFA
POST /api/auth/mfa/verify        # Verify MFA code
POST /api/auth/mfa/disable       # Disable MFA
```

---

## 🔐 Email/Password Authentication

### Registration

```typescript
const response = await fetch('/api/auth/register', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({
    email: 'user@example.com',
    password: 'SecurePass123!',
    name: 'John Doe'
  })
});

const { token, user } = await response.json();
```

### Password Requirements

- **Minimum length:** 8 characters
- **Complexity:** Must include uppercase, lowercase, number
- **No common passwords:** Checked against common password list
- **Hashing:** Argon2id with salt

### Login

```typescript
const response = await fetch('/api/auth/login', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({
    email: 'user@example.com',
    password: 'SecurePass123!'
  })
});

const { token, user, mfa_required } = await response.json();

if (mfa_required) {
  // Prompt for MFA code
  const mfaResponse = await fetch('/api/auth/mfa/verify', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
      email: 'user@example.com',
      code: '123456'
    })
  });
  
  const { token, user } = await mfaResponse.json();
}
```

---

## 🌐 OAuth Authentication

### Google Sign-In

```typescript
// Redirect to Google OAuth
window.location.href = '/api/auth/google';

// Google redirects back to /api/auth/google/callback
// Backend exchanges code for token and creates/updates user
// Redirects to /dashboard with token in cookie
```

### Apple Sign-In

```typescript
// Redirect to Apple OAuth
window.location.href = '/api/auth/apple';

// Apple redirects back to /api/auth/apple/callback
// Backend exchanges code for token and creates/updates user
// Redirects to /dashboard with token in cookie
```

### OAuth Flow

```
1. User clicks "Sign in with Google"
2. Redirect to Google OAuth consent screen
3. User approves
4. Google redirects to /api/auth/google/callback?code=...
5. Backend exchanges code for access token
6. Backend fetches user profile from Google
7. Backend creates/updates user in database
8. Backend generates JWT token
9. Backend sets cookie and redirects to /dashboard
```

---

## 🔑 Multi-Factor Authentication (MFA)

### Setup MFA

```typescript
// 1. Request MFA setup
const response = await fetch('/api/auth/mfa/setup', {
  method: 'POST',
  headers: {
    'Authorization': `Bearer ${token}`
  }
});

const { secret, qr_code_url } = await response.json();

// 2. Display QR code to user
// User scans with authenticator app (Google Authenticator, Authy, etc.)

// 3. Verify setup with code from app
const verifyResponse = await fetch('/api/auth/mfa/verify', {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json',
    'Authorization': `Bearer ${token}`
  },
  body: JSON.stringify({
    code: '123456'
  })
});

const { success, backup_codes } = await verifyResponse.json();
```

### TOTP Implementation

```rust
// Backend: Generate TOTP secret
use hmac::{Hmac, Mac};
use sha1::Sha1;

fn generate_totp(secret: &str, time_step: u64) -> String {
    let key = base32::decode(base32::Alphabet::RFC4648 { padding: false }, secret)
        .expect("Invalid secret");
    
    let counter = (time_step / 30).to_be_bytes();
    
    let mut mac = Hmac::<Sha1>::new_from_slice(&key)
        .expect("HMAC can take key of any size");
    mac.update(&counter);
    
    let result = mac.finalize().into_bytes();
    let offset = (result[19] & 0xf) as usize;
    let code = ((result[offset] & 0x7f) as u32) << 24
        | (result[offset + 1] as u32) << 16
        | (result[offset + 2] as u32) << 8
        | result[offset + 3] as u32;
    
    format!("{:06}", code % 1_000_000)
}
```

### Backup Codes

10 single-use backup codes are generated during MFA setup:

```typescript
interface BackupCode {
  code: string;
  used: boolean;
  used_at?: string;
}

// Example backup codes
[
  "A1B2-C3D4-E5F6",
  "G7H8-I9J0-K1L2",
  // ... 8 more
]
```

---

## 🎫 JWT Tokens

### Token Structure

```typescript
interface JWTClaims {
  sub: string;        // User ID
  email: string;      // User email
  role: string;       // User role (user, admin, developer)
  exp: number;        // Expiration timestamp
  iat: number;        // Issued at timestamp
}
```

### Token Generation

```rust
use jsonwebtoken::{encode, Header, EncodingKey};

let claims = Claims {
    sub: user.id.to_string(),
    email: user.email.clone(),
    role: user.role.clone(),
    exp: (Utc::now() + Duration::hours(24)).timestamp() as usize,
    iat: Utc::now().timestamp() as usize,
};

let token = encode(
    &Header::default(),
    &claims,
    &EncodingKey::from_secret(config.jwt_secret.as_bytes())
)?;
```

### Token Validation

```rust
use jsonwebtoken::{decode, DecodingKey, Validation};

let token_data = decode::<Claims>(
    &token,
    &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
    &Validation::default()
)?;

let claims = token_data.claims;
```

---

## 🔄 Session Management

### Redis Sessions

Sessions are stored in Redis with 24-hour TTL:

```rust
// Store session
redis.set_ex(
    format!("session:{}", user_id),
    serde_json::to_string(&session)?,
    86400 // 24 hours
).await?;

// Get session
let session: Session = redis.get(format!("session:{}", user_id)).await?;

// Delete session (logout)
redis.del(format!("session:{}", user_id)).await?;
```

---

## 🛡️ Rate Limiting

### Login Rate Limits

- **Per IP:** 5 failed attempts per 15 minutes
- **Per Email:** 3 failed attempts per 15 minutes
- **Lockout:** 1 hour after 10 failed attempts

```rust
use governor::{Quota, RateLimiter};

let limiter = RateLimiter::direct(
    Quota::per_minute(nonzero!(5u32))
);

if limiter.check().is_err() {
    return Err(ApiError::TooManyRequests);
}
```

---

## 🔒 Password Security

### Argon2id Hashing

```rust
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2
};

// Hash password
let salt = SaltString::generate(&mut OsRng);
let argon2 = Argon2::default();
let password_hash = argon2
    .hash_password(password.as_bytes(), &salt)?
    .to_string();

// Verify password
let parsed_hash = PasswordHash::new(&password_hash)?;
argon2.verify_password(password.as_bytes(), &parsed_hash)?;
```

---

## 🔐 Security Best Practices

1. **HTTPS Only** - All traffic encrypted
2. **Secure Cookies** - HttpOnly, Secure, SameSite
3. **CSRF Protection** - Token-based validation
4. **XSS Prevention** - Content Security Policy
5. **SQL Injection** - Parameterized queries
6. **Rate Limiting** - Prevent brute-force
7. **Password Hashing** - Argon2id with salt
8. **MFA Support** - TOTP-based 2FA

---

## 📊 Metrics & Monitoring

- **Login success rate** - > 95%
- **MFA adoption** - Track percentage
- **Failed login attempts** - Alert on spikes
- **Token refresh rate** - Monitor usage
- **Session duration** - Average session length


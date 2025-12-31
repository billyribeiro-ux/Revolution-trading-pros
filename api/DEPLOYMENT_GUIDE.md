# Fly.io Deployment Guide - Backend API
**Revolution Trading Pros - Rust API**  
**Date:** December 31, 2025

---

## 🚀 Quick Deployment Steps

### 1. Install Fly.io CLI (One-time)

```bash
# Install flyctl
curl -L https://fly.io/install.sh | sh

# Add to PATH (add to ~/.zshrc or ~/.bashrc)
export PATH="$HOME/.fly/bin:$PATH"

# Reload shell
source ~/.zshrc

# Verify installation
flyctl version

# Login to Fly.io
flyctl auth login
```

### 2. Check Current Deployment Status

```bash
cd /Users/billyribeiro/CascadeProjects/Revolution-trading-pros/api

# Check if app exists
flyctl status -a revolution-trading-pros-api

# View current logs
flyctl logs -a revolution-trading-pros-api
```

### 3. Deploy the API

```bash
# Deploy (builds Docker image and deploys)
flyctl deploy -a revolution-trading-pros-api

# Or use the deployment script
./scripts/deploy.sh deploy
```

### 4. Verify Deployment

```bash
# Check health endpoint
curl https://revolution-trading-pros-api.fly.dev/health

# Expected response:
# {"status":"healthy","version":"0.1.0","environment":"production"}

# Check readiness (includes DB check)
curl https://revolution-trading-pros-api.fly.dev/ready

# Expected response:
# {"status":"ready","version":"0.1.0","environment":"production"}
```

### 5. Run Database Setup (First Time Only)

```bash
# Create email_verification_tokens and password_resets tables
# Also creates/updates superadmin user
curl -X POST https://revolution-trading-pros-api.fly.dev/setup-db

# Expected response:
# {"success":true,"message":"Database setup completed..."}

# Run membership plan seeding
curl -X POST https://revolution-trading-pros-api.fly.dev/run-migrations

# Expected response:
# {"success":true,"message":"Migrations completed..."}
```

### 6. Monitor Logs

```bash
# View real-time logs
flyctl logs -a revolution-trading-pros-api

# SSH into the instance
flyctl ssh console -a revolution-trading-pros-api
```

---

## 🔐 Required Environment Variables

Before deploying, ensure all secrets are set on Fly.io:

### Critical Secrets (Required)

```bash
flyctl secrets set DATABASE_URL="postgresql://user:pass@host/db?sslmode=require" -a revolution-trading-pros-api
flyctl secrets set REDIS_URL="rediss://default:pass@host:6379" -a revolution-trading-pros-api
flyctl secrets set JWT_SECRET="your-secret-min-32-chars-long" -a revolution-trading-pros-api
flyctl secrets set STRIPE_SECRET="sk_live_..." -a revolution-trading-pros-api
flyctl secrets set STRIPE_WEBHOOK_SECRET="whsec_..." -a revolution-trading-pros-api
flyctl secrets set POSTMARK_TOKEN="your-postmark-token" -a revolution-trading-pros-api
flyctl secrets set MEILISEARCH_API_KEY="your-meilisearch-key" -a revolution-trading-pros-api
```

### Optional Secrets

```bash
flyctl secrets set R2_ENDPOINT="https://xxx.r2.cloudflarestorage.com" -a revolution-trading-pros-api
flyctl secrets set R2_ACCESS_KEY_ID="your-r2-access-key" -a revolution-trading-pros-api
flyctl secrets set R2_SECRET_ACCESS_KEY="your-r2-secret-key" -a revolution-trading-pros-api
flyctl secrets set FROM_EMAIL="noreply@revolutiontradingpros.com" -a revolution-trading-pros-api
flyctl secrets set APP_URL="https://revolutiontradingpros.com" -a revolution-trading-pros-api
```

### View Current Secrets

```bash
# List all secrets (values are hidden)
flyctl secrets list -a revolution-trading-pros-api
```

---

## 🔍 Troubleshooting

### Build Failures

```bash
# Check build logs
flyctl logs -a revolution-trading-pros-api

# Common issues:
# 1. Rust version - Dockerfile uses rust:latest (requires 1.88+)
# 2. Missing dependencies - Check Cargo.toml
# 3. OpenSSL issues - Using native-tls, should work
```

### Runtime Errors

```bash
# Check application logs
flyctl logs -a revolution-trading-pros-api

# Common issues:
# 1. DATABASE_URL not set or invalid
# 2. REDIS_URL not set or invalid
# 3. JWT_SECRET missing or too short
# 4. Port binding issues (should be 8080)
```

### Database Connection Issues

```bash
# Test database connection
flyctl ssh console -a revolution-trading-pros-api
# Inside container:
curl http://localhost:8080/ready

# Check Neon PostgreSQL status
# Visit: https://console.neon.tech
```

### Health Check Failures

```bash
# Check if health endpoint responds
curl -v https://revolution-trading-pros-api.fly.dev/health

# If 502/503 errors:
# 1. App may still be starting (wait 30s)
# 2. Check logs for panics
# 3. Verify PORT=8080 in fly.toml
```

---

## 📊 Monitoring

### Key Metrics to Watch

1. **Response Times**
   - Health check should respond in <100ms
   - API endpoints should respond in <500ms

2. **Memory Usage**
   - Current allocation: 512MB
   - Rust is efficient, but monitor for leaks
   - Consider upgrading to 1GB if needed

3. **CPU Usage**
   - 1 shared CPU allocated
   - Monitor for sustained high usage

4. **Database Connections**
   - SQLx pool size: Default 10
   - Watch for connection exhaustion

### Monitoring Commands

```bash
# View metrics
flyctl metrics -a revolution-trading-pros-api

# Scale instances
flyctl scale count 2 -a revolution-trading-pros-api

# Scale memory
flyctl scale memory 1024 -a revolution-trading-pros-api

# View machine status
flyctl machine list -a revolution-trading-pros-api
```

---

## 🧪 Testing After Deployment

### 1. Health Checks

```bash
# Basic health
curl https://revolution-trading-pros-api.fly.dev/health

# Readiness (includes DB)
curl https://revolution-trading-pros-api.fly.dev/ready
```

### 2. Authentication Flow

```bash
# Register new user
curl -X POST https://revolution-trading-pros-api.fly.dev/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{"email":"test@example.com","password":"Test123!","name":"Test User"}'

# Login
curl -X POST https://revolution-trading-pros-api.fly.dev/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"test@example.com","password":"Test123!"}'
```

### 3. Protected Endpoints

```bash
# Get user profile (requires JWT token)
curl https://revolution-trading-pros-api.fly.dev/api/user/profile \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

### 4. Admin Endpoints

```bash
# Login as superadmin
curl -X POST https://revolution-trading-pros-api.fly.dev/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"welberribeirodrums@gmail.com","password":"Revolution2024!"}'

# Test admin endpoint
curl https://revolution-trading-pros-api.fly.dev/api/admin/users \
  -H "Authorization: Bearer ADMIN_JWT_TOKEN"
```

---

## 🔄 Rollback Procedure

If deployment fails or has issues:

```bash
# List recent releases
flyctl releases -a revolution-trading-pros-api

# Rollback to previous version
flyctl releases rollback -a revolution-trading-pros-api

# Or rollback to specific version
flyctl releases rollback v123 -a revolution-trading-pros-api
```

---

## 📝 Post-Deployment Checklist

- [ ] Health endpoint responds (200 OK)
- [ ] Ready endpoint responds (200 OK with DB check)
- [ ] Database setup completed (`/setup-db`)
- [ ] Migrations run successfully (`/run-migrations`)
- [ ] Superadmin user can login
- [ ] User registration works
- [ ] User login works
- [ ] Protected endpoints require auth
- [ ] CORS headers present
- [ ] Security headers present (HSTS, CSP, etc.)
- [ ] Logs show no errors
- [ ] Memory usage stable
- [ ] Response times acceptable

---

## 🚨 Emergency Contacts

- **Fly.io Status:** https://status.fly.io
- **Fly.io Docs:** https://fly.io/docs
- **Fly.io Community:** https://community.fly.io
- **Neon Status:** https://neonstatus.com

---

## 📚 Additional Resources

- [Fly.io Rust Guide](https://fly.io/docs/languages-and-frameworks/rust/)
- [Fly.io Secrets Management](https://fly.io/docs/reference/secrets/)
- [Fly.io Monitoring](https://fly.io/docs/reference/metrics/)
- [API Diagnostic Report](./DIAGNOSTIC_REPORT.md)

---

**Last Updated:** December 31, 2025  
**API Version:** 0.1.0  
**Deployment Status:** Ready for Production

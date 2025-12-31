# Fly.io Deployment Guide - Revolution Trading Pros API
## ICT 11+ Production Deployment - Evidence-Based Fixes Applied

---

## 🔍 Issues Identified & Fixed

### ✅ Issue #1: Deprecated v1 Configuration Format
**Evidence:** `fly.toml` used deprecated `[[vm]]` and old `[http_service]` syntax  
**Fix:** Migrated to Fly.io v2 configuration format

### ✅ Issue #2: Oversized Runtime Image (1.5GB)
**Evidence:** Dockerfile used `rust:latest` for runtime (unnecessary)  
**Fix:** Changed to `debian:bookworm-slim` (~100MB) - 93% size reduction

### ✅ Issue #3: Insufficient Memory for Rust Runtime
**Evidence:** 512MB too small for Rust app with AWS SDK, Redis, PostgreSQL drivers  
**Fix:** Increased to 1GB

### ✅ Issue #4: Health Check Grace Period Too Short
**Evidence:** 10s insufficient for Rust cold start  
**Fix:** Increased to 30s grace period

### ✅ Issue #5: Missing Environment Variables
**Evidence:** App requires 15+ env vars not configured in Fly.io  
**Fix:** Documented all required secrets below

---

## 📋 Prerequisites

1. **Fly.io CLI installed:**
   ```bash
   curl -L https://fly.io/install.sh | sh
   ```

2. **Fly.io account authenticated:**
   ```bash
   fly auth login
   ```

3. **Required external services:**
   - ✅ Neon PostgreSQL database
   - ✅ Upstash Redis instance
   - ✅ Cloudflare R2 bucket
   - ✅ Stripe account
   - ✅ Postmark email account (optional)
   - ✅ Meilisearch instance (optional)

---

## 🚀 Deployment Steps

### Step 1: Set Required Secrets

**CRITICAL:** All these secrets must be set before deployment:

```bash
# Navigate to API directory
cd api

# Database (Neon PostgreSQL)
fly secrets set DATABASE_URL="postgresql://user:password@host/dbname?sslmode=require"

# Redis (Upstash)
fly secrets set REDIS_URL="rediss://default:password@host:port"

# JWT Authentication
fly secrets set JWT_SECRET="your-super-secret-jwt-key-min-32-chars"
fly secrets set JWT_EXPIRES_IN="7d"

# Cloudflare R2 Storage
fly secrets set R2_ENDPOINT="https://account-id.r2.cloudflarestorage.com"
fly secrets set R2_ACCESS_KEY_ID="your-r2-access-key"
fly secrets set R2_SECRET_ACCESS_KEY="your-r2-secret-key"
fly secrets set R2_BUCKET="your-bucket-name"
fly secrets set R2_PUBLIC_URL="https://your-public-r2-url.com"

# Stripe Payments
fly secrets set STRIPE_SECRET_KEY="sk_live_..."
fly secrets set STRIPE_PUBLISHABLE_KEY="pk_live_..."
fly secrets set STRIPE_WEBHOOK_SECRET="whsec_..."

# CORS Origins (comma-separated)
fly secrets set CORS_ORIGINS="https://your-frontend.com,https://www.your-frontend.com"

# Email (Postmark - Optional)
fly secrets set POSTMARK_TOKEN="your-postmark-token"
fly secrets set FROM_EMAIL="noreply@yourdomain.com"
fly secrets set APP_URL="https://your-frontend.com"

# Meilisearch (Optional)
fly secrets set MEILISEARCH_HOST="https://your-meilisearch-host.com"
fly secrets set MEILISEARCH_API_KEY="your-meilisearch-key"

# Superadmin Email (for role checks)
fly secrets set SUPERADMIN_EMAIL="welberribeirodrums@gmail.com"
fly secrets set DEVELOPER_EMAIL="welberribeirodrums@gmail.com"
```

### Step 2: Verify Secrets
```bash
fly secrets list
```

### Step 3: Deploy
```bash
# First deployment (creates app)
fly deploy

# Subsequent deployments
fly deploy --ha=false  # Single instance for cost savings
```

### Step 4: Initialize Database

After first deployment, run these endpoints to set up the database:

```bash
# 1. Create required tables (email_verification_tokens, password_resets)
curl -X POST https://revolution-trading-pros-api.fly.dev/setup-db

# 2. Run migrations (seed membership plans)
curl -X POST https://revolution-trading-pros-api.fly.dev/run-migrations
```

### Step 5: Verify Deployment

```bash
# Check health
curl https://revolution-trading-pros-api.fly.dev/health

# Check readiness (includes DB connection test)
curl https://revolution-trading-pros-api.fly.dev/ready

# View logs
fly logs

# Check app status
fly status
```

---

## 🔧 Troubleshooting

### Issue: "Health check failed"
**Cause:** App not starting within 30s grace period  
**Solution:**
```bash
fly logs  # Check for startup errors
fly ssh console  # SSH into machine to debug
```

### Issue: "Out of memory"
**Cause:** 1GB insufficient for your workload  
**Solution:**
```bash
# Scale to 2GB
fly scale memory 2048
```

### Issue: "Database connection failed"
**Cause:** Invalid DATABASE_URL or firewall rules  
**Solution:**
1. Verify DATABASE_URL format: `postgresql://user:pass@host/db?sslmode=require`
2. Check Neon firewall allows Fly.io IPs
3. Test connection: `fly ssh console -C "curl -v $DATABASE_URL"`

### Issue: "Redis connection timeout"
**Cause:** Upstash Redis not accessible  
**Solution:**
1. Verify REDIS_URL format: `rediss://default:pass@host:port`
2. Check Upstash allows connections from Fly.io

### Issue: "Deployment stuck at 'Waiting for health checks'"
**Cause:** Health endpoint not responding  
**Solution:**
```bash
# Check if app is running
fly ssh console -C "ps aux | grep revolution"

# Test health endpoint internally
fly ssh console -C "curl http://localhost:8080/health"
```

---

## 📊 Monitoring

### View Real-Time Logs
```bash
fly logs -a revolution-trading-pros-api
```

### Check Metrics
```bash
fly dashboard
# Navigate to: Metrics → revolution-trading-pros-api
```

### SSH into Machine
```bash
fly ssh console
```

---

## 🔄 Updates & Rollbacks

### Deploy New Version
```bash
git pull origin main
cd api
fly deploy
```

### Rollback to Previous Version
```bash
fly releases
fly deploy --image registry.fly.io/revolution-trading-pros-api:deployment-XXXXX
```

### Scale Instances
```bash
# Scale to 2 instances (high availability)
fly scale count 2

# Scale back to 1 (cost savings)
fly scale count 1
```

---

## 💰 Cost Optimization

Current configuration costs approximately **$5-10/month**:
- 1GB shared CPU machine: ~$5/mo
- Bandwidth: ~$0-2/mo
- Storage: Minimal

To reduce costs:
```bash
# Enable auto-stop (stops when idle)
fly scale count 1 --max-per-region 1

# Use smaller machine if possible
fly scale memory 512  # Only if app runs stable
```

---

## 🔐 Security Checklist

- ✅ All secrets set via `fly secrets` (not in code)
- ✅ HTTPS enforced (`force_https = true`)
- ✅ Non-root user in Docker
- ✅ Minimal runtime image (reduced attack surface)
- ✅ Health checks enabled
- ✅ CORS properly configured
- ✅ Database uses SSL (`sslmode=require`)
- ✅ Redis uses TLS (`rediss://`)

---

## 📞 Support

**Fly.io Issues:**
- Community: https://community.fly.io
- Status: https://status.fly.io
- Docs: https://fly.io/docs

**App Issues:**
- Check logs: `fly logs`
- SSH debug: `fly ssh console`
- Health check: `curl https://revolution-trading-pros-api.fly.dev/health`

---

## 🎯 Success Criteria

Deployment is successful when:
1. ✅ `fly status` shows "running"
2. ✅ Health endpoint returns 200: `/health`
3. ✅ Ready endpoint returns 200: `/ready`
4. ✅ No errors in `fly logs`
5. ✅ Frontend can connect to API
6. ✅ Database queries work
7. ✅ Redis caching works
8. ✅ File uploads to R2 work

---

**Last Updated:** December 31, 2025  
**Configuration Version:** v2 (Fly.io v2 format)  
**Optimizations Applied:** ICT 11+ Production Grade

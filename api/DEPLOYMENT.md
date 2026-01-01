# 🚀 Revolution Trading Pros API - Deployment Guide
## ICT 11+ Principal Engineer Grade - Production Deployment

---

## 📋 **Prerequisites**

- Rust 1.75+ installed
- PostgreSQL database (Neon recommended)
- Redis instance (Upstash recommended)
- Cloudflare R2 bucket (or S3-compatible storage)
- Stripe account
- Postmark account (for emails)
- Meilisearch instance

---

## 🔧 **Environment Variables**

Create a `.env` file in the `api/` directory:

```bash
# Server Configuration
PORT=8080
ENVIRONMENT=production

# Database (Neon PostgreSQL)
DATABASE_URL=postgresql://user:password@host/database?sslmode=require

# Redis (Upstash)
REDIS_URL=redis://default:password@host:port

# Cloudflare R2 Storage
R2_ENDPOINT=https://account-id.r2.cloudflarestorage.com
R2_ACCESS_KEY_ID=your_access_key
R2_SECRET_ACCESS_KEY=your_secret_key
R2_BUCKET=revolution-trading-media
R2_PUBLIC_URL=https://pub-xxxxx.r2.dev

# JWT Authentication
JWT_SECRET=your_super_secret_jwt_key_min_32_chars
JWT_EXPIRES_IN=24

# Stripe Payments
STRIPE_SECRET=sk_live_xxxxx
STRIPE_PUBLISHABLE_KEY=pk_live_xxxxx
STRIPE_WEBHOOK_SECRET=whsec_xxxxx

# CORS Origins (comma-separated)
CORS_ORIGINS=https://revolutiontradingpros.com,https://www.revolutiontradingpros.com

# Email (Postmark)
POSTMARK_TOKEN=your_postmark_token
FROM_EMAIL=noreply@revolutiontradingpros.com
APP_URL=https://revolutiontradingpros.com

# Meilisearch
MEILISEARCH_HOST=http://localhost:7700
MEILISEARCH_API_KEY=your_master_key

# Superadmin Configuration
SUPERADMIN_EMAILS=welberribeirodrums@gmail.com
DEVELOPER_EMAILS=welberribeirodrums@gmail.com
DEVELOPER_MODE=false
```

---

## 🗄️ **Database Setup**

### 1. Run Migrations

```bash
cd api
sqlx migrate run
```

### 2. Verify Schema

```bash
psql $DATABASE_URL -c "\dt"
```

Expected tables:
- users
- products
- courses
- lessons
- posts
- indicators
- orders
- subscriptions
- membership_plans
- email_verification_tokens
- password_reset_tokens
- jobs
- analytics_events
- contacts
- coupons
- invoices
- newsletter_subscribers

---

## 🏗️ **Build & Deploy**

### **Option 1: Fly.io (Recommended)**

1. **Install Fly CLI:**
```bash
curl -L https://fly.io/install.sh | sh
```

2. **Login:**
```bash
fly auth login
```

3. **Create App:**
```bash
fly apps create revolution-trading-pros-api
```

4. **Set Secrets:**
```bash
fly secrets set DATABASE_URL="postgresql://..." \
  REDIS_URL="redis://..." \
  JWT_SECRET="..." \
  STRIPE_SECRET="..." \
  POSTMARK_TOKEN="..." \
  --app revolution-trading-pros-api
```

5. **Deploy:**
```bash
fly deploy --app revolution-trading-pros-api
```

6. **Scale (if needed):**
```bash
fly scale vm shared-cpu-2x --memory 512 --app revolution-trading-pros-api
```

### **Option 2: Docker**

1. **Build Image:**
```bash
docker build -t revolution-api:latest .
```

2. **Run Container:**
```bash
docker run -d \
  --name revolution-api \
  -p 8080:8080 \
  --env-file .env \
  revolution-api:latest
```

### **Option 3: Manual Deployment**

1. **Build Release:**
```bash
cargo build --release
```

2. **Copy Binary:**
```bash
cp target/release/revolution-api /usr/local/bin/
```

3. **Create Systemd Service:**
```bash
sudo nano /etc/systemd/system/revolution-api.service
```

```ini
[Unit]
Description=Revolution Trading Pros API
After=network.target

[Service]
Type=simple
User=www-data
WorkingDirectory=/opt/revolution-api
EnvironmentFile=/opt/revolution-api/.env
ExecStart=/usr/local/bin/revolution-api
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

4. **Start Service:**
```bash
sudo systemctl daemon-reload
sudo systemctl enable revolution-api
sudo systemctl start revolution-api
```

---

## ✅ **Verification**

### 1. Health Check
```bash
curl https://your-domain.com/health
```

Expected response:
```json
{
  "status": "healthy",
  "timestamp": "2026-01-01T12:00:00Z"
}
```

### 2. Detailed Health Check
```bash
curl https://your-domain.com/monitoring/health/detailed
```

### 3. Metrics
```bash
curl https://your-domain.com/monitoring/metrics
```

### 4. API Documentation
Open in browser:
```
https://your-domain.com/swagger-ui
```

---

## 📊 **Monitoring**

### **Prometheus Integration**

Add to `prometheus.yml`:
```yaml
scrape_configs:
  - job_name: 'revolution-api'
    static_configs:
      - targets: ['your-domain.com:8080']
    metrics_path: '/monitoring/metrics'
    scheme: 'https'
```

### **Grafana Dashboard**

Import metrics:
- `requests_total` - Total API requests
- `requests_success` - Successful requests
- `requests_error` - Failed requests
- `auth_attempts` - Authentication attempts
- `auth_success` - Successful logins
- `auth_failures` - Failed logins

---

## 🔒 **Security Checklist**

- ✅ JWT_SECRET is strong (min 32 characters)
- ✅ Database uses SSL/TLS
- ✅ Redis uses TLS
- ✅ CORS origins are whitelisted
- ✅ Rate limiting enabled
- ✅ Security headers configured
- ✅ Stripe webhook signature verification
- ✅ SQL injection protection (parameterized queries)
- ✅ Constant-time password comparison
- ✅ Account lockout after failed attempts

---

## 🚨 **Troubleshooting**

### **Database Connection Issues**
```bash
# Test connection
psql $DATABASE_URL -c "SELECT 1"

# Check migrations
sqlx migrate info
```

### **Redis Connection Issues**
```bash
# Test connection
redis-cli -u $REDIS_URL ping
```

### **Build Issues**
```bash
# Clean build
cargo clean
cargo build --release

# Check dependencies
cargo tree
```

### **Runtime Errors**
```bash
# View logs (Fly.io)
fly logs --app revolution-trading-pros-api

# View logs (systemd)
sudo journalctl -u revolution-api -f

# View logs (Docker)
docker logs -f revolution-api
```

---

## 📈 **Performance Tuning**

### **Database Connection Pool**
Adjust in `src/db/mod.rs`:
```rust
.max_connections(10)  // Increase for high traffic
```

### **Redis Connection Pool**
Already using connection manager for optimal performance.

### **Compression**
Gzip compression enabled by default for all responses.

---

## 🔄 **Updates & Rollbacks**

### **Deploy Update:**
```bash
git pull origin main
fly deploy --app revolution-trading-pros-api
```

### **Rollback:**
```bash
fly releases --app revolution-trading-pros-api
fly releases rollback v123 --app revolution-trading-pros-api
```

---

## 📞 **Support**

- **API Documentation:** https://your-domain.com/swagger-ui
- **Metrics:** https://your-domain.com/monitoring/metrics
- **Health:** https://your-domain.com/health

---

## 🎯 **Production Checklist**

Before going live:

- [ ] All environment variables set
- [ ] Database migrations run
- [ ] Redis connection tested
- [ ] Stripe webhooks configured
- [ ] Email service tested
- [ ] CORS origins configured
- [ ] SSL/TLS certificates installed
- [ ] Monitoring setup (Prometheus/Grafana)
- [ ] Backup strategy in place
- [ ] Rate limiting tested
- [ ] Load testing completed
- [ ] Security audit passed

---

**🎉 Your Rust API is production-ready!**

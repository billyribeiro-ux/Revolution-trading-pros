# Get Database and Redis Credentials

Your backend API is already deployed on Fly.io, but I need the actual DATABASE_URL and REDIS_URL credentials.

---

## Option 1: Check Fly.io Secrets (Recommended)

If you have Fly CLI installed:

```bash
# Install Fly CLI if needed
curl -L https://fly.io/install.sh | sh

# Login
fly auth login

# Check secrets
fly secrets list -a revolution-trading-pros-api
```

This will show you the DATABASE_URL and REDIS_URL that are already set.

---

## Option 2: Get from Fly.io Dashboard

### Database URL:
1. Go to: https://fly.io/dashboard
2. Click on your app: **revolution-trading-pros-api**
3. Go to **"Secrets"** tab
4. Look for **DATABASE_URL**
5. Copy the full value (format: `postgres://user:pass@rtp-postgres.flycast:5432/revolution_trading_pros`)

### Redis URL:
1. Same location - **"Secrets"** tab
2. Look for **REDIS_URL**
3. Copy the full value (format: `rediss://default:xxx@xxx.upstash.io:6379`)

---

## Option 3: Check Upstash Dashboard for Redis

1. Go to: https://console.upstash.com
2. Login to your account
3. Click on your database: **revolution-trading-pros**
4. Go to **"Details"** or **"REST API"** tab
5. Copy the **Redis URL** (starts with `rediss://`)

---

## Option 4: Recreate Database (if needed)

If you don't have the credentials and need to recreate:

### PostgreSQL:
```bash
# Create new Postgres cluster
fly postgres create --name rtp-postgres --region iad

# Attach to your app (auto-sets DATABASE_URL)
fly postgres attach --app revolution-trading-pros-api rtp-postgres

# Get connection string
fly postgres connect -a rtp-postgres
```

### Redis:
1. Go to: https://console.upstash.com
2. Create new database
3. Name: `revolution-trading-pros`
4. Region: **US-East-1** (closest to Fly.io)
5. Copy the Redis URL

---

## What I Need:

Just paste these 2 values here:

```bash
DATABASE_URL=postgres://...
REDIS_URL=rediss://...
```

Then I'll update your `api/.env` file.

---

**Which option works best for you?**

# 🔐 Stripe API Keys Setup Instructions

## WHERE TO GET YOUR STRIPE KEYS

### 1. Go to Stripe Dashboard
Visit: https://dashboard.stripe.com/test/apikeys

### 2. Toggle to Test Mode (top right)
Make sure you're in **Test Mode** for development

### 3. Copy Your Keys

**Publishable Key** (starts with `pk_test_`)
- This is safe to use on the client-side
- Example: `pk_test_51AbCdEfGhIjKlMnOpQrStUvWxYz1234567890`

**Secret Key** (starts with `sk_test_`)
- This is for server-side only - NEVER expose to client
- Click "Reveal test key" to see it
- Example: `sk_test_51AbCdEfGhIjKlMnOpQrStUvWxYz1234567890`

### 4. Get Webhook Secret (Optional for now)
Visit: https://dashboard.stripe.com/test/webhooks
- Create endpoint: `https://revolution-trading-pros-api.fly.dev/api/webhooks/stripe`
- Copy the signing secret (starts with `whsec_`)

---

## HOW TO ADD KEYS TO YOUR PROJECT

### Frontend (.env.local)

Create this file: `/Users/billyribeiro/CascadeProjects/Revolution-trading-pros/frontend/.env.local`

```bash
# Stripe Configuration (Test Mode)
PUBLIC_STRIPE_PUBLISHABLE_KEY=pk_test_YOUR_ACTUAL_KEY_HERE
VITE_STRIPE_PUBLISHABLE_KEY=pk_test_YOUR_ACTUAL_KEY_HERE
```

### Backend (Fly.io Secrets)

Run these commands to add secrets to your Fly.io API:

```bash
cd /Users/billyribeiro/CascadeProjects/Revolution-trading-pros/api

# Set Stripe secrets
fly secrets set \
  STRIPE_SECRET_KEY=sk_test_YOUR_ACTUAL_KEY_HERE \
  STRIPE_WEBHOOK_SECRET=whsec_YOUR_ACTUAL_SECRET_HERE \
  -a revolution-trading-pros-api
```

### Backend Local Development (api/.env)

If you're running the API locally, add to `api/.env`:

```bash
STRIPE_SECRET_KEY=sk_test_YOUR_ACTUAL_KEY_HERE
STRIPE_WEBHOOK_SECRET=whsec_YOUR_ACTUAL_SECRET_HERE
```

---

## VERIFICATION

After adding the keys, verify they work:

### Test Frontend
```bash
cd frontend
npm run dev
```

Visit any page with payment forms - the Stripe integration should load without errors.

### Test Backend
```bash
cd api
cargo run
```

The API should start without Stripe-related errors.

---

## PRODUCTION KEYS (Later)

When ready for production:
1. Toggle to **Live Mode** in Stripe Dashboard
2. Get live keys (start with `pk_live_` and `sk_live_`)
3. Update Cloudflare Pages environment variables
4. Update Fly.io secrets with live keys

---

## SECURITY NOTES

✅ **DO:**
- Keep `.env.local` and `api/.env` in `.gitignore` (already done)
- Use test keys for development
- Rotate keys every 90 days
- Enable MFA on your Stripe account

❌ **DON'T:**
- Commit API keys to Git
- Share secret keys in Slack/email
- Use production keys in development
- Hardcode keys in source code

---

## CURRENT STATUS

- ✅ Stripe package installed (`@stripe/stripe-js`)
- ✅ Configuration files created
- ✅ Environment variable types defined
- ✅ Database schema ready for Stripe
- ⏳ **WAITING:** You need to add actual API keys from Stripe Dashboard

Once you add the keys, the payment system will be fully functional!

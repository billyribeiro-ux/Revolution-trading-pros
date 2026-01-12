# Stripe Setup Guide - January 2026
**For Revolution Trading Pros**

---

## Step 1: Create Stripe Account

1. Go to: **https://stripe.com**
2. Click **"Sign up"** (top right)
3. Fill in:
   - Email address
   - Full name
   - Password
4. Verify your email
5. Complete business information

---

## Step 2: Activate Your Account

1. After signup, you'll be in **Test Mode** by default
2. To accept real payments, you need to **activate your account**:
   - Click **"Activate your account"** banner at the top
   - Fill in business information
   - Provide tax details
   - Add bank account for payouts

---

## Step 3: Get API Keys

### For Development/Testing (Test Mode):
1. Go to: https://dashboard.stripe.com/test/apikeys
2. Copy **Secret key** (starts with `sk_test_`)
3. This is for testing only - no real charges

### For Production (Live Mode):
1. Go to: https://dashboard.stripe.com/apikeys
2. Toggle to **"Live mode"** (top right)
3. Copy **Secret key** (starts with `sk_live_`)
4. Copy **Publishable key** (starts with `pk_live_`)

---

## Stripe Setup Steps:

1. **Sign up**: https://dashboard.stripe.com/register
2. **Activate account** (requires business verification for live mode)
3. **Get API Keys**:
   - Dashboard → Developers → API keys
   - Copy **Secret key** (live mode)
4. **Set up Webhook**:
   - Dashboard → Developers → Webhooks
   - Add endpoint: `https://revolution-trading-pros-api.fly.dev/api/webhooks/stripe`
   - Copy **Webhook Secret**

---

## Postmark Setup

1. Go to: **https://postmarkapp.com**
2. Sign up for free account
3. Create a **Server**
4. Go to **API Tokens** tab
5. Copy the **Server API Token**

---

**Once you have Stripe and Postmark accounts, I'll walk you through getting the API keys.**

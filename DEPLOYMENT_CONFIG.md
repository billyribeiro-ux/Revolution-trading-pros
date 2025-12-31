# Deployment Configuration - Developer Access
**Revolution Trading Pros - Production Environment**

## Deployed URLs

### Frontend (Cloudflare Pages)
- **Production:** `https://revolution-trading-pros.pages.dev`
- **Preview:** Auto-generated per branch

### Backend (Fly.io)
- **API:** `https://revolution-trading-pros-api.fly.dev/api`

---

## Environment Variables (Already Deployed)

### Backend (Fly.io)

Set via `fly secrets set`:

```bash
# Developer Access Configuration
DEVELOPER_EMAILS=welberribeirodrums@gmail.com
SUPERADMIN_EMAILS=welberribeirodrums@gmail.com

# Optional: Enable developer mode
DEVELOPER_MODE=true
```

**To update deployed secrets:**
```bash
cd api
fly secrets set DEVELOPER_EMAILS=welberribeirodrums@gmail.com
fly secrets set SUPERADMIN_EMAILS=welberribeirodrums@gmail.com
fly secrets set DEVELOPER_MODE=true
```

### Frontend (Cloudflare Pages)

Environment variables are set in Cloudflare Pages dashboard:
- Settings → Environment Variables

**Already configured:**
```bash
VITE_API_URL=https://revolution-trading-pros-api.fly.dev/api
VITE_SITE_URL=https://revolution-trading-pros.pages.dev
```

**No changes needed** - Developer access is handled by email check in code.

---

## Current Status

### ✅ What's Already Working

1. **Backend Deployed**
   - Fly.io: `revolution-trading-pros-api.fly.dev`
   - Developer access logic implemented
   - Email verification bypass ready

2. **Frontend Deployed**
   - Cloudflare Pages: `revolution-trading-pros.pages.dev`
   - Membership auto-unlock implemented
   - Developer detection ready

### 🔧 What Needs Deployment

**Backend changes need to be deployed:**
```bash
cd api
fly deploy
```

This will deploy:
- Developer email configuration
- Email verification bypass
- Enhanced authentication flow

**Frontend changes need to be deployed:**
```bash
cd frontend
git add .
git commit -m "feat: Enterprise developer access system"
git push origin main
```

Cloudflare Pages will auto-deploy on push.

---

## Testing on Production

### 1. Login as Developer

**URL:** `https://revolution-trading-pros.pages.dev/login`

**Credentials:**
- Email: `welberribeirodrums@gmail.com`
- Password: [your password]

### 2. Expected Behavior

✅ **Login succeeds** (no email verification required)  
✅ **Redirects to:** `/dashboard`  
✅ **Shows:** Member dashboard with all memberships  
✅ **Access:** All trading rooms, courses, tools unlocked  

### 3. Verify Memberships

Navigate to dashboard and verify you see:
- **Trading Rooms:** Day Trading, Swing Trading, Small Accounts
- **Alert Services:** SPX Profit Pulse, Explosive Swing
- **Courses:** All available courses
- **Tools:** Weekly Watchlist, etc.

---

## Deployment Commands

### Deploy Backend (Fly.io)

```bash
cd /Users/user/Documents/revolution-svelte/api
fly deploy
```

**What this deploys:**
- Updated authentication with developer bypass
- Configuration-based email checks
- Enhanced security logging

### Deploy Frontend (Cloudflare Pages)

```bash
cd /Users/user/Documents/revolution-svelte/frontend
git add .
git commit -m "feat: Enterprise developer access - member experience with full access"
git push origin main
```

**Auto-deploys to:**
- Production: `revolution-trading-pros.pages.dev`
- Takes ~2-3 minutes

---

## Environment Variable Management

### Fly.io Secrets

**View current secrets:**
```bash
fly secrets list
```

**Set new secret:**
```bash
fly secrets set KEY=value
```

**Unset secret:**
```bash
fly secrets unset KEY
```

### Cloudflare Pages

**Access via:**
1. Cloudflare Dashboard
2. Pages → revolution-trading-pros
3. Settings → Environment Variables

**Scopes:**
- Production
- Preview
- Both

---

## Monitoring & Logs

### Backend Logs (Fly.io)

```bash
# Real-time logs
fly logs

# Filtered logs
fly logs --app revolution-trading-pros-api
```

**Look for:**
```
[security] event=privileged_verification_bypass role=developer
[Developer] 🔓 Fetching all available memberships
```

### Frontend Logs (Browser)

Open browser console on production:
```
[UserMemberships] Developer/Superadmin detected - unlocking all memberships
[Developer] ✅ Fetched X products from API
```

---

## Rollback Plan

If issues occur after deployment:

### Backend Rollback
```bash
cd api
fly releases
fly rollback <version>
```

### Frontend Rollback
```bash
# Via Cloudflare Dashboard
Pages → Deployments → [Previous deployment] → Rollback
```

---

## Security Notes

### Production Safety

1. **Developer emails are environment-based**
   - Not hardcoded in deployed code
   - Can be updated via secrets

2. **Audit logging enabled**
   - All developer logins logged
   - Verification bypasses tracked

3. **Separation of concerns**
   - Developer ≠ Admin
   - Member experience maintained

### Best Practices

✅ Keep `DEVELOPER_EMAILS` list minimal  
✅ Use separate staging environment for testing  
✅ Monitor logs for unusual activity  
✅ Rotate credentials regularly  
✅ Review access quarterly  

---

## Quick Reference

### Deploy Everything
```bash
# Backend
cd api && fly deploy

# Frontend
cd frontend && git push origin main
```

### Check Status
```bash
# Backend health
curl https://revolution-trading-pros-api.fly.dev/health

# Frontend
curl https://revolution-trading-pros.pages.dev
```

### View Logs
```bash
# Backend
fly logs

# Frontend
# Browser console on production site
```

---

## Support

**If developer access isn't working:**

1. Check environment variables are set
2. Verify email matches exactly
3. Check backend logs for errors
4. Verify frontend detects developer
5. Clear browser cache and retry

**Common Issues:**

- **Email case mismatch:** Emails are normalized to lowercase
- **Cache:** Clear browser cache after deployment
- **Secrets:** Verify secrets are set in Fly.io
- **Build:** Ensure latest code is deployed

---

**Last Updated:** December 31, 2025  
**Status:** Ready for deployment  
**Next Step:** Run `fly deploy` in `/api` directory

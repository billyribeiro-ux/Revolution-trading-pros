# Scanner System - Quick Start Guide

## 🚀 Getting Started

### For Developers

#### 1. Run Database Seeder
```bash
cd backend
php artisan db:seed --class=HighOctaneScannerSeeder
```

#### 2. Start Development Servers
```bash
# Frontend
cd frontend
npm run dev

# Backend
cd backend
php artisan serve
```

#### 3. Test the Flow
1. Navigate to: http://localhost:5173
2. Click: **Store → Scanners**
3. Click: **High Octane Scanner**
4. Toggle pricing (Monthly/Yearly)
5. Click: **Add to Cart** or **Buy Now**
6. Complete checkout

---

## 📍 Key URLs

### Frontend Routes
- **Scanners Landing:** `/store/scanners`
- **High Octane Scanner:** `/store/scanners/high-octane-scanner`
- **Checkout:** `/checkout`
- **Dashboard:** `/dashboard`

### Backend API Endpoints
- **Get Products:** `GET /api/products?type=indicator`
- **Get Scanner:** `GET /api/products/high-octane-scanner`
- **User Memberships:** `GET /api/user/memberships`
- **Create Checkout:** `POST /api/checkout/create-session`

---

## 🎯 Quick Test Scenarios

### Scenario 1: Browse and Add to Cart
```
1. Main Nav → Store → Scanners
2. View High Octane Scanner card
3. Click "Learn More"
4. Toggle to Yearly (see 17% savings)
5. Click "Add to Cart"
6. See success message
7. Cart count increases to 1
```

### Scenario 2: Direct Purchase
```
1. Navigate to /store/scanners/high-octane-scanner
2. Select Monthly ($119)
3. Click "Buy Now"
4. Redirect to /checkout
5. Complete billing info
6. Select payment method
7. Complete purchase
```

### Scenario 3: Dashboard Access (Post-Purchase)
```
1. Complete purchase
2. Navigate to /dashboard
3. See "INDICATORS" category in sidebar
4. See "High Octane Scanner" listed
5. Click to access scanner
```

---

## 🔧 Configuration

### Product Settings
```typescript
// Product ID
id: 'high-octane-scanner'

// Pricing
monthly: $119
yearly: $1,190 (17% savings)

// Type
type: 'indicator'
category: 'scanner'
```

### Stripe Configuration
```php
// Create Stripe products (run once)
stripe products create \
  --name="High Octane Scanner - Monthly" \
  --description="Professional options scanner"

stripe prices create \
  --product=prod_XXX \
  --unit-amount=11900 \
  --currency=usd \
  --recurring[interval]=month
```

---

## 📦 File Structure

```
frontend/
├── src/
│   ├── routes/
│   │   ├── store/
│   │   │   └── scanners/
│   │   │       ├── +page.svelte (Landing)
│   │   │       └── high-octane-scanner/
│   │   │           └── +page.svelte (Product Page)
│   │   ├── checkout/
│   │   │   └── +page.svelte (Checkout)
│   │   └── dashboard/
│   │       ├── +layout.svelte (Dashboard Layout)
│   │       └── +page.svelte (Dashboard Home)
│   ├── lib/
│   │   ├── components/
│   │   │   ├── nav/
│   │   │   │   └── NavBar.svelte (Navigation)
│   │   │   └── dashboard/
│   │   │       └── DashboardSidebar.svelte (Sidebar)
│   │   └── stores/
│   │       └── cart.ts (Cart Store)

backend/
├── database/
│   └── seeders/
│       └── HighOctaneScannerSeeder.php
├── app/
│   ├── Models/
│   │   └── Product.php
│   └── Http/
│       └── Controllers/
│           └── ProductController.php
```

---

## ✅ Verification Checklist

### Frontend
- [ ] Navigation shows "Scanners" in Store dropdown
- [ ] `/store/scanners` page loads
- [ ] `/store/scanners/high-octane-scanner` page loads
- [ ] Pricing toggle works (Monthly/Yearly)
- [ ] Add to Cart button works
- [ ] Cart count updates
- [ ] Checkout page shows scanner
- [ ] Mobile responsive

### Backend
- [ ] Database seeder runs successfully
- [ ] Product exists in `products` table
- [ ] Membership plans exist in `membership_plans` table
- [ ] API returns scanner product
- [ ] Checkout session creation works

### Integration
- [ ] Complete purchase flow works
- [ ] Scanner appears in dashboard after purchase
- [ ] Email confirmation sent
- [ ] Subscription created in Stripe

---

## 🐛 Troubleshooting

### Issue: Scanner not showing in navigation
**Solution:** Clear browser cache, check NavBar.svelte line 140

### Issue: Add to Cart not working
**Solution:** Check authentication, verify cart store import

### Issue: Checkout fails
**Solution:** Verify Stripe API keys, check backend logs

### Issue: Scanner not in dashboard
**Solution:** Verify user membership record, check membership type is 'indicator'

---

## 📞 Support

**Documentation:** See `SCANNER_SYSTEM_DOCUMENTATION.md` for complete details

**Quick Links:**
- Frontend: http://localhost:5173
- Backend: http://localhost:8000
- Database: MySQL on port 3306

---

**Last Updated:** January 2, 2026  
**Version:** 1.0.0

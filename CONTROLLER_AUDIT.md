# Controller Conversion Audit
**Date:** January 4, 2026  
**Purpose:** Map PHP controllers to Rust routes, identify conversions, and track deletions

## Audit Methodology

1. **List all PHP controllers**
2. **Map to Rust route equivalents** in `/api/src/routes/`
3. **Verify functionality** - test endpoints
4. **Delete PHP files** after verification
5. **Document unused controllers** with evidence

---

## RUST ROUTES INVENTORY

### Existing Rust Route Modules (18 files)

| Rust Module | Routes Implemented | Status |
|-------------|-------------------|--------|
| `admin.rs` | Dashboard, users, products, orders, analytics | ✅ Active |
| `analytics.rs` | Event tracking, reading analytics | ✅ Active |
| `auth.rs` | Register, login, logout, password reset, email verification | ✅ Active |
| `checkout.rs` | Create checkout, calculate tax | ✅ Active |
| `contacts.rs` | Contact management, stats | ✅ Active |
| `coupons.rs` | Validate, user coupons, available coupons | ✅ Active |
| `courses.rs` | List courses, get course, create course | ✅ Active |
| `health.rs` | Health check, readiness check | ✅ Active |
| `indicators.rs` | List indicators, user indicators, create | ✅ Active |
| `newsletter.rs` | Subscribe, confirm, unsubscribe | ✅ Active |
| `orders.rs` | List orders, order details | ✅ Active |
| `payments.rs` | Stripe checkout, portal, webhooks | ✅ Active |
| `posts.rs` | List posts, get post, create, update, delete | ✅ Active |
| `products.rs` | List products, create, user products | ✅ Active |
| `search.rs` | Search all, search courses | ✅ Active |
| `security.rs` | Security events, stats | ✅ Active |
| `subscriptions.rs` | Plans, user subscriptions, create, cancel | ✅ Active |
| `user.rs` | Memberships, profile, payment methods | ✅ Active |
| `users.rs` | List users, get user | ✅ Active |
| `videos.rs` | List videos, get video | ✅ Active |

---

## PHP CONTROLLERS ANALYSIS

### Controllers to Audit (Scanning...)


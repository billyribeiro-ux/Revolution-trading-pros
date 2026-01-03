# Laravel → Rust/Axum Migration Analysis
## Apple ICT 11+ Principal Engineer Protocol
### Revolution Trading Pros - Complete Backend Conversion

---

## Executive Summary

**Source:** Laravel 11.x PHP Backend
**Target:** Rust 1.84 with Axum 0.8

### Codebase Statistics

| Component | Count | Priority |
|-----------|-------|----------|
| API Routes | 400+ | Critical |
| Models | 151 | Critical |
| API Controllers | 45 | Critical |
| Admin Controllers | 36 | High |
| Services | 165+ | High |
| Migrations | 85+ | Critical |
| Middleware | 10+ | Critical |
| Jobs | 9 | Medium |
| Events | 41 | Medium |
| Mail | 8 | Medium |

---

## Phase 0: Route Analysis

### Public Routes (No Auth)

#### Health Check
- `GET /api/health/live` → `HealthCheckController::liveness`
- `GET /api/health/ready` → `HealthCheckController::readiness`
- `GET /api/health/optimization` → `HealthCheckController::optimization`

#### Webhooks (Signature Verified)
- `POST /api/webhooks/postmark/inbound` → Inbound email
- `POST /api/webhooks/ses/inbound` → AWS SES
- `POST /api/webhooks/sendgrid/inbound` → SendGrid
- `POST /api/payments/webhook` → Stripe webhooks

#### SEO & Content
- `GET /api/sitemap/` → Sitemap index
- `GET /api/sitemap/posts/{page?}` → Post sitemap
- `GET /api/sitemap/categories` → Category sitemap
- `GET /api/sitemap/tags` → Tag sitemap
- `GET /api/robots.txt` → Robots.txt

#### Public Content
- `GET /api/posts` → List posts
- `GET /api/posts/{slug}` → Show post
- `GET /api/indicators` → List indicators
- `GET /api/indicators/{slug}` → Show indicator
- `GET /api/videos` → List videos
- `GET /api/videos/{id}` → Show video

#### Newsletter
- `POST /api/newsletter/subscribe` → Subscribe
- `GET /api/newsletter/confirm` → Confirm
- `GET /api/newsletter/unsubscribe` → Unsubscribe

#### Popups (Rate Limited)
- `GET /api/popups/active` → Active popups
- `POST /api/popups/{popup}/impression` → Track impression
- `POST /api/popups/{popup}/conversion` → Track conversion

#### Analytics (Public Tracking)
- `POST /api/analytics/track` → Track event
- `POST /api/analytics/pageview` → Track pageview
- `POST /api/analytics/batch` → Batch tracking
- `POST /api/analytics/reading` → Reading analytics

#### Payments
- `GET /api/payments/config` → Stripe config (publishable key)

---

### Protected Routes (auth:sanctum)

#### User Self-Service (/me, /my)
- `GET /api/me` → Current user
- `GET /api/me/memberships` → User memberships
- `GET /api/me/products` → User products
- `GET /api/me/indicators` → User indicators
- `GET /api/my/orders` → User orders
- `GET /api/my/orders/{id}` → Order detail
- `GET /api/my/subscriptions` → User subscriptions
- `GET /api/my/subscriptions/{id}` → Subscription detail
- `POST /api/my/subscriptions/{id}/cancel` → Cancel
- `POST /api/my/subscriptions/{id}/pause` → Pause
- `POST /api/my/subscriptions/{id}/resume` → Resume

#### User Indicators
- `GET /api/user/indicators` → List user indicators
- `GET /api/user/indicators/{id}` → Show indicator
- `GET /api/user/indicators/{id}/download` → Download
- `GET /api/user/indicators/{id}/docs` → Documentation

#### Cart & Checkout
- `POST /api/cart/checkout` → Checkout
- `POST /api/cart/calculate-tax` → Calculate tax

#### Payments (Protected)
- `POST /api/payments/create-intent` → Create payment intent
- `POST /api/payments/create-checkout` → Create checkout session
- `POST /api/payments/confirm` → Confirm payment
- `GET /api/payments/order/{order}/status` → Order status

#### Trading Rooms
- `GET /api/trading-rooms` → List rooms
- `GET /api/trading-rooms/{slug}` → Show room
- `GET /api/trading-rooms/{slug}/videos` → Room videos
- `POST /api/trading-rooms/{slug}/sso` → Generate SSO token

---

### Admin Routes (auth:sanctum + role:admin|super-admin)

#### Email System (20+ endpoints)
- Templates, Campaigns, Subscribers, Metrics, Domains, Webhooks, Conversations

#### Newsletter Categories (10 endpoints)
- CRUD + analytics

#### Coupons (6 endpoints)
- CRUD + user coupons

#### Users & Members (15+ endpoints)
- User management, member management, past members, win-back

#### Subscriptions (15+ endpoints)
- Plans, user subscriptions, admin management

#### Products (10 endpoints)
- Products, assignment, users

#### Content (Posts, Categories, Tags) (30+ endpoints)
- Full CRUD with analytics

#### Media & Images (20+ endpoints)
- Upload, optimization, processing

#### Forms (30+ endpoints)
- Forms, submissions, PDFs, approvals, inventory

#### CRM (50+ endpoints)
- Contacts, deals, pipelines, sequences, automations, lists, tags, companies

#### Analytics (30+ endpoints)
- Dashboard, KPIs, funnels, cohorts, attribution, forecasting

#### SEO (40+ endpoints)
- Rankings, backlinks, redirects, 404 errors, settings

#### Trading Rooms Admin (15 endpoints)
- Rooms, traders, videos

---

## Core Models to Convert

### Priority 1: User & Auth
1. `User` - Core user model
2. `UserMembership` - User-membership pivot
3. `UserSubscription` - Subscription management

### Priority 2: Commerce
4. `Order` - Order management
5. `OrderItem` - Order line items
6. `Product` - Products/courses/indicators
7. `Coupon` - Discount codes
8. `SubscriptionPlan` - Subscription plans

### Priority 3: Content
9. `Post` - Blog posts
10. `Category` - Categories
11. `Tag` - Tags
12. `Video` - Video content
13. `Media` - Media library

### Priority 4: Membership & Trading
14. `MembershipPlan` - Membership plans
15. `MembershipFeature` - Plan features
16. `TradingRoom` - Trading rooms
17. `RoomTrader` - Traders
18. `RoomDailyVideo` - Daily videos

### Priority 5: Email & CRM
19. `EmailTemplate` - Email templates
20. `EmailCampaign` - Campaigns
21. `NewsletterSubscription` - Subscribers
22. `Contact` - CRM contacts
23. `Deal` - CRM deals

---

## Frontend API Dependencies

Based on the SvelteKit frontend analysis, these endpoints are **critical path**:

### Account Dashboard (PRIORITY 1)
- `GET /api/me` → User profile
- `GET /api/my/orders` → Orders list
- `GET /api/my/orders/{id}` → Order detail
- `GET /api/my/subscriptions` → Subscriptions
- `GET /api/my/subscriptions/{id}` → Subscription detail
- `POST /api/my/subscriptions/{id}/cancel` → Cancel
- `POST /api/my/subscriptions/{id}/pause` → Pause
- `GET /api/user/payment-methods` → Payment methods
- `POST /api/user/payment-methods` → Add payment method
- `DELETE /api/user/payment-methods/{id}` → Delete payment method
- `GET /api/user/profile` → Profile
- `PUT /api/user/profile` → Update profile
- `POST /api/logout` → Logout

### Authentication (PRIORITY 1)
- `POST /api/auth/login` → Login
- `POST /api/auth/register` → Register
- `POST /api/auth/logout` → Logout
- `POST /api/auth/refresh` → Refresh token
- `POST /api/auth/forgot-password` → Forgot password
- `POST /api/auth/reset-password` → Reset password

### Public Content (PRIORITY 2)
- `GET /api/posts` → Blog posts
- `GET /api/posts/{slug}` → Post detail
- `GET /api/indicators` → Indicators
- `GET /api/videos` → Videos

### Payments (PRIORITY 1)
- `GET /api/payments/config` → Stripe config
- `POST /api/payments/create-intent` → Payment intent
- `POST /api/payments/webhook` → Stripe webhook

---

## Conversion Strategy

### Phase 1: Core Infrastructure (Week 1)
1. Rust project structure
2. Database connection (SQLx)
3. Authentication (JWT)
4. Error handling
5. Middleware (auth, CORS, rate limiting)

### Phase 2: User & Auth (Week 1-2)
1. User model & repository
2. Auth handlers (login, register, logout, refresh)
3. Password hashing (Argon2)
4. Session management

### Phase 3: Account Dashboard (Week 2)
1. Orders endpoints
2. Subscriptions endpoints
3. Payment methods (Stripe)
4. Profile management

### Phase 4: Content & Commerce (Week 3)
1. Posts endpoints
2. Products endpoints
3. Cart & checkout
4. Stripe integration

### Phase 5: Admin API (Week 4-5)
1. User management
2. Content management
3. Email system
4. CRM system

### Phase 6: Advanced Features (Week 6)
1. Analytics
2. SEO management
3. Trading rooms
4. Webhooks

---

## Technical Requirements

### Performance Targets
- p50 response time: < 10ms
- p95 response time: < 50ms
- p99 response time: < 100ms
- Memory baseline: < 50MB
- Concurrent connections: > 10,000

### Security Requirements
- JWT authentication with refresh tokens
- Argon2 password hashing
- Rate limiting
- CORS configuration
- SQL injection prevention (SQLx compile-time checks)

### Compatibility Requirements
- 100% API compatibility with SvelteKit frontend
- Same response shapes
- Same error formats
- Same status codes

---

## Next Steps

1. ✅ Complete Laravel codebase analysis
2. 🔄 Create Rust project structure
3. ⏳ Implement core infrastructure
4. ⏳ Convert models and repositories
5. ⏳ Convert handlers (controllers)
6. ⏳ Write tests
7. ⏳ Deploy to Fly.io

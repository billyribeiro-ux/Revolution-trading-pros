# Controller Conversion Mapping & Deletion Plan
**Date:** January 4, 2026  
**Status:** In Progress

---

## ✅ PHASE 1: CONVERTED & VERIFIED (Ready to Delete PHP)

### Controllers with 100% Rust Implementation

| # | PHP Controller | Rust Module | Routes | Verification Status | Delete PHP? |
|---|----------------|-------------|--------|---------------------|-------------|
| 1 | `HealthCheckController.php` | `health.rs` | `/health/live`, `/health/ready` | ✅ Testing now | PENDING |
| 2 | `NewsletterController.php` | `newsletter.rs` | `/newsletter/subscribe`, `/newsletter/confirm`, `/newsletter/unsubscribe` | ⏳ Need test | PENDING |
| 3 | `PostController.php` | `posts.rs` | `/posts`, `/posts/{slug}` (CRUD) | ⏳ Need test | PENDING |
| 4 | `IndicatorController.php` | `indicators.rs` | `/indicators`, `/indicators/{slug}` | ⏳ Need test | PENDING |
| 5 | `VideoController.php` | `videos.rs` | `/videos`, `/videos/{id}`, `/videos/{id}/track` | ⏳ Need test | PENDING |
| 6 | `MeController.php` | `user.rs` | `/me`, `/me/memberships`, `/me/products`, `/me/indicators` | ⏳ Need test | PENDING |
| 7 | `CartController.php` | `checkout.rs` | `/cart/checkout`, `/cart/calculate-tax` | ⏳ Need test | PENDING |
| 8 | `CouponController.php` | `coupons.rs` | `/coupons/validate`, `/coupons/my`, `/coupons/user/available` | ⏳ Need test | PENDING |
| 9 | `ProductController.php` | `products.rs` | `/products` (CRUD), `/products/my` | ⏳ Need test | PENDING |
| 10 | `UserSubscriptionController.php` | `subscriptions.rs` | `/my/subscriptions/*` | ⏳ Need test | PENDING |
| 11 | `SubscriptionPlanController.php` | `subscriptions.rs` | `/plans`, `/plans/{slug}` | ⏳ Need test | PENDING |
| 12 | `UserController.php` | `admin.rs` + `users.rs` | `/admin/users`, `/users/{id}` | ⏳ Need test | PENDING |
| 13 | `TimeController.php` | Simple endpoint | `/time/now` | ⏳ Need test | PENDING |
| 14 | `ReadingAnalyticsController.php` | `analytics.rs` | `/analytics/reading` | ⏳ Need test | PENDING |
| 15 | `AnalyticsController.php` | `analytics.rs` | `/analytics/track`, `/admin/analytics/dashboard` | ⏳ Need test | PENDING |
| 16 | `TimerAnalyticsController.php` | `analytics.rs` | `/timers/events` | ⏳ Need test | PENDING |
| 17 | `PaymentController.php` | `payments.rs` | `/payments/checkout`, `/payments/webhook`, `/payments/portal` | ⏳ Need test | PENDING |

**Deletion Plan:**
- Test each endpoint with curl/Postman
- Verify 100% functionality match
- Delete PHP file only after confirmation
- Commit with detailed documentation

---

## ⚠️ PHASE 2: NEED CONVERSION TO RUST (59 Controllers)

### Priority 0 - Critical Core Features (Convert First)

| PHP Controller | Functionality | Complexity | Rust Status | Priority |
|----------------|---------------|------------|-------------|----------|
| `SitemapController` | XML sitemap generation | Medium | ❌ Not started | P0 |
| `RobotsController` | robots.txt generation | Low | ❌ Not started | P0 |
| `SeoController` | SEO analysis & optimization | High | ❌ Not started | P0 |
| `CategoryController` | Category CRUD | Medium | ❌ Not started | P0 |
| `TagController` | Tag CRUD | Medium | ❌ Not started | P0 |
| `RedirectController` | URL redirect management | Medium | ❌ Not started | P0 |

### Priority 1 - Admin Features (Convert Second)

| PHP Controller | Functionality | Complexity | Rust Status | Priority |
|----------------|---------------|------------|-------------|----------|
| `MediaController` | Media library management | High | ❌ Not started | P1 |
| `MemberController` | Member management | Medium | ❌ Not started | P1 |
| `SettingsController` | Settings CRUD | Medium | ❌ Not started | P1 |
| `AdminPostController` | Admin post management | Medium | ❌ Not started | P1 |

### Priority 2 - Email System (Large Subsystem)

| PHP Controller | Functionality | Complexity | Rust Status | Priority |
|----------------|---------------|------------|-------------|----------|
| `EmailCampaignController` | Email campaign management | High | ❌ Not started | P2 |
| `EmailTemplateController` | Email template CRUD | Medium | ❌ Not started | P2 |
| `EmailTemplateBuilderController` | Visual email builder | High | ❌ Not started | P2 |
| `EmailSubscriberController` | Subscriber management | Medium | ❌ Not started | P2 |
| `EmailMetricsController` | Email analytics | Medium | ❌ Not started | P2 |
| `EmailSettingsController` | Email configuration | Medium | ❌ Not started | P2 |
| `EmailDomainController` | Domain management | Medium | ❌ Not started | P2 |
| `EmailAuditLogController` | Email audit logs | Low | ❌ Not started | P2 |
| `EmailWebhookController` | Email webhooks | Medium | ❌ Not started | P2 |
| `EmailConversationController` | Email threads | High | ❌ Not started | P2 |
| `InboundEmailWebhookController` | Inbound email processing | High | ❌ Not started | P2 |

### Priority 3 - CRM Features

| PHP Controller | Functionality | Complexity | Rust Status | Priority |
|----------------|---------------|------------|-------------|----------|
| `ContactController` | Contact management | Medium | ❌ Not started | P3 |
| `ContactListController` | Contact lists | Medium | ❌ Not started | P3 |
| `ContactTagController` | Contact tagging | Low | ❌ Not started | P3 |
| `CrmCompanyController` | Company management | Medium | ❌ Not started | P3 |
| `DealController` | Deal pipeline | High | ❌ Not started | P3 |
| `PipelineController` | Pipeline management | Medium | ❌ Not started | P3 |
| `SequenceController` | Email sequences | High | ❌ Not started | P3 |
| `AutomationFunnelController` | Marketing funnels | High | ❌ Not started | P3 |
| `RecurringCampaignController` | Recurring campaigns | Medium | ❌ Not started | P3 |
| `SmartLinkController` | Smart link tracking | Medium | ❌ Not started | P3 |
| `BehaviorController` | Behavior analytics | High | ❌ Not started | P3 |

### Priority 4 - Forms System

| PHP Controller | Functionality | Complexity | Rust Status | Priority |
|----------------|---------------|------------|-------------|----------|
| `FormController` | Form CRUD | Medium | ❌ Not started | P4 |
| `FormSubmissionController` | Form submissions | Medium | ❌ Not started | P4 |
| `FormPdfController` | PDF generation | High | ❌ Not started | P4 |
| `FormApprovalController` | Approval workflow | Medium | ❌ Not started | P4 |
| `FormInventoryController` | Inventory tracking | Medium | ❌ Not started | P4 |

### Priority 5 - Advanced Features (Keep PHP or Convert Later)

| PHP Controller | Functionality | Decision |
|----------------|---------------|----------|
| `SeoIntelligenceController` | Advanced SEO AI | **KEEP PHP** (user confirmed) |
| `WorkflowController` | Workflow automation | **KEEP PHP** (user confirmed) |
| `BingSeoController` | Bing SEO integration | **KEEP PHP** (specialized) |
| `ContentLakeController` | Content management | **KEEP PHP** (complex) |
| `ExperimentController` | A/B testing | **KEEP PHP** (analytics) |
| `PopupController` | Popup management | **KEEP PHP** (marketing) |
| `TradingRoomController` | Trading room features | **KEEP PHP** (specialized) |
| `TradingRoomSSOController` | Trading room SSO | **KEEP PHP** (specialized) |
| `UserIndicatorsController` | User indicators | **KEEP PHP** (specialized) |
| `PastMembersController` | Past member tracking | **KEEP PHP** (marketing) |
| `PastMembersDashboardController` | Past member analytics | **KEEP PHP** (analytics) |
| `AbandonedCartController` | Cart recovery | **KEEP PHP** (marketing) |
| `BacklinkController` | Backlink tracking | **KEEP PHP** (SEO) |
| `RankingController` | Rank tracking | **KEEP PHP** (SEO) |
| `PerformanceController` | Performance monitoring | **KEEP PHP** (monitoring) |
| `SiteHealthController` | Site health checks | **KEEP PHP** (monitoring) |
| `SharpMediaController` | Media optimization | **KEEP PHP** (media) |
| `ConsentSettingsController` | GDPR consent | **KEEP PHP** (compliance) |
| `ApiConnectionController` | API connections | **KEEP PHP** (integrations) |
| `NewsletterCategoryController` | Newsletter categories | **KEEP PHP** (simple CRUD) |

---

## ❓ PHASE 3: CONTROLLERS NOT IN ROUTES (Need Verification)

| Controller | Location | Likely Status | Action |
|------------|----------|---------------|--------|
| `DashboardController` | Root | Service layer - custom dashboards | **KEEP** |
| `Error404Controller` | Api/ | **USED** in api.php lines 206-210 | **KEEP** |
| `HealthController` | Root | Duplicate of HealthCheckController | **DELETE** |
| `IntegrationController` | Api/ | Integration management | **VERIFY USAGE** |
| `InvoiceSettingsController` | Admin/ | Invoice configuration | **VERIFY USAGE** |
| `MetricsController` | Root | Prometheus metrics endpoint | **KEEP** |
| `PlanAdminController` | Admin/ | Plan administration | **VERIFY USAGE** |
| `RevenueReportController` | Admin/ | Revenue reporting | **VERIFY USAGE** |
| `SeoSettingType` | Api/ | Not a controller (type class) | **DELETE** |
| `StripeWebhookController` | Webhooks/ | Stripe webhooks | **KEEP** |
| `SubscriptionAdminController` | Admin/ | Subscription admin | **VERIFY USAGE** |

---

## 📋 CONVERSION STRATEGY

### Step 1: Test & Delete Converted Controllers (17 files)
1. Test each Rust endpoint
2. Verify 100% functionality
3. Delete PHP file
4. Commit with documentation

### Step 2: Convert Priority 0 Controllers (6 files)
- SitemapController → Create `sitemap.rs`
- RobotsController → Create `robots.rs`  
- SeoController → Expand `posts.rs` or create `seo.rs`
- CategoryController → Create `categories.rs`
- TagController → Create `tags.rs`
- RedirectController → Create `redirects.rs`

### Step 3: Convert Priority 1 Controllers (4 files)
- MediaController → Create `media.rs`
- MemberController → Expand `admin.rs`
- SettingsController → Create `settings.rs`
- AdminPostController → Expand `admin.rs`

### Step 4: Decide on Priority 2-5
- Email system: Large subsystem, may keep PHP
- CRM features: May keep PHP or convert selectively
- Forms: May keep PHP
- Advanced features: Keep PHP (user confirmed some)

---

## 🎯 IMMEDIATE ACTIONS

1. ✅ Test health endpoints (in progress)
2. ⏳ Test all 17 converted endpoints
3. ⏳ Delete verified PHP controllers
4. ⏳ Convert Priority 0 controllers to Rust
5. ⏳ Test Priority 0 conversions
6. ⏳ Delete Priority 0 PHP files

---

**Last Updated:** January 4, 2026 2:59 AM

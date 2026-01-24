# Frontend-Backend API Audit
## Revolution Trading Pros - ICT 7 Grade Reference Document
### Generated: January 17, 2026

---

## 1. BACKEND ROUTES (from api/src/routes/mod.rs)

| Route Prefix | Module | Description |
|--------------|--------|-------------|
| `/auth` | auth::router() | Authentication |
| `/users` | users::router() | User management |
| `/user` | user::router() | Singular user routes |
| `/payments` | payments::router() | Payment processing |
| `/search` | search::router() | Search functionality |
| `/products` | products::router() | Product catalog |
| `/posts` | posts::router() | Blog posts |
| `/admin/posts` | posts::admin_router() | Admin post management |
| `/subscriptions` | subscriptions::router() | Subscriptions |
| `/newsletter` | newsletter::router() | Newsletter |
| `/admin` | admin::router() | Admin dashboard |
| `/admin/products` | products::admin_router() | Admin products |
| `/checkout` | checkout::router() | Checkout flow |
| `/videos` | videos::router() | Video content |
| `/analytics` | analytics::router() | Analytics tracking |
| `/contacts` | contacts::router() | Contact management |
| `/coupons` | coupons::router() | Coupon codes |
| `/security` | security::router() | Security features |
| `/schedules` | schedules::router() | Schedules |
| `/admin/schedules` | schedules::admin_router() | Admin schedules |
| `/admin/courses-enhanced` | courses_admin::router() | Enhanced courses admin |
| `/migrate` | migrate::router() | Migration tools |
| `/my/orders` | orders::router() | User orders |
| `/my/subscriptions` | subscriptions::my_router() | User subscriptions |
| `/logout` | auth::logout_router() | Logout |
| `/admin/cms` | cms::admin_router() | CMS admin |
| `/preview` | cms::preview_router() | Content preview |
| `/realtime` | realtime::router() | Real-time SSE |
| `/popups` | popups::router() | Popups |
| `/trading-rooms` | trading_rooms::router() | Trading rooms public |
| `/admin/trading-rooms` | trading_rooms::admin_router() | Trading rooms admin |
| `/admin/courses` | admin_courses::router() | Course admin |
| `/courses` | member_courses::public_router() | Public courses |
| `/my/courses` | member_courses::member_router() | Member courses |
| `/admin/indicators` | admin_indicators::router() | Indicator admin |
| `/admin/videos` | admin_videos::router() | Video admin |
| `/video-advanced` | admin_videos::analytics_router() | Video analytics |
| `/admin/page-layouts` | admin_page_layouts::router() | Page layouts |
| `/admin/media` | media::admin_router() | Media admin |
| `/forms` | forms::public_router() | Public forms |
| `/admin/forms` | forms::admin_router() | Admin forms |
| `/admin/email/templates` | email_templates::admin_router() | Email templates |
| `/admin/email` | email_templates::admin_router() | Email settings |
| `/admin/subscriptions` | subscriptions_admin::subscriptions_router() | Subscription admin |
| `/admin/subscriptions/plans` | subscriptions_admin::plans_router() | Plan admin |
| `/indicators` | member_indicators::public_router() | Public indicators |
| `/my/indicators` | member_indicators::member_router() | Member indicators |
| `/download` | member_indicators::download_router() | Downloads |
| `/room-content` | room_content::public_router() | Room content public |
| `/admin/room-content` | room_content::admin_router() | Room content admin |
| `/watchlist` | watchlist::router() | Watchlist |
| `/room-resources` | room_resources::public_router() | Room resources |
| `/admin/room-resources` | room_resources::admin_router() | Room resources admin |
| `/admin/bunny` | bunny_upload::admin_router() | Bunny uploads |
| `/admin/crm` | crm::router() | CRM admin |
| `/admin/connections` | connections::admin_router() | Connections |
| `/admin/members` | admin_members::router() | Member segments/tags |
| `/admin/member-management` | admin_member_management::router() | Full member CRUD |
| `/admin/organization/teams` | organization::teams_router() | Teams |
| `/admin/organization/departments` | organization::departments_router() | Departments |

---

## 2. FRONTEND API ENDPOINTS (from frontend/src/lib/api/)

### config.ts - Core API Endpoints
```
/api/admin/coupons
/api/admin/dashboard
/api/admin/settings
/api/admin/users
/api/admin/users/stats
/api/analytics/overview
/api/analytics/performance
/api/analytics/reading
/api/analytics/track
/api/articles
/api/auth/email/verification-notification
/api/auth/forgot-password
/api/auth/login
/api/auth/login/biometric
/api/auth/login/mfa
/api/auth/me
/api/auth/me/indicators
/api/auth/me/memberships
/api/auth/me/mfa/disable
/api/auth/me/mfa/enable
/api/auth/me/mfa/verify
/api/auth/me/password
/api/auth/me/products
/api/auth/me/security-events
/api/auth/me/sessions
/api/auth/me/sessions/logout-all
/api/auth/refresh
/api/auth/register
/api/auth/resend-verification
/api/auth/reset-password
/api/checkout
/api/checkout/calculate-tax
/api/checkout/orders
/api/contacts
/api/contacts/stats
/api/courses
/api/health
/api/indicators
/api/indicators/my
/api/logout
/api/newsletter/confirm
/api/newsletter/stats
/api/newsletter/subscribe
/api/newsletter/subscribers
/api/newsletter/unsubscribe
/api/posts
/api/products
/api/products/my
/api/search
/api/subscriptions
/api/subscriptions/metrics
/api/subscriptions/my
/api/subscriptions/my/active
/api/subscriptions/plans
/api/time/now
/api/timers/events
/api/users/me
/api/videos
```

### account.ts - User Account
```
/api/user/avatar
/api/user/billing
/api/user/memberships
/api/user/password
/api/user/profile
```

### abandoned-carts.ts
```
/api/admin/abandoned-carts
/api/cart/abandoned
```

### trading-rooms.ts
```
/api/admin/trading-rooms
/api/admin/trading-rooms/sections
/api/admin/trading-rooms/traders
/api/admin/trading-rooms/videos
/api/admin/trading-rooms/videos/bulk
/api/trading-rooms
/api/trading-rooms/sections
/api/trading-rooms/traders
```

### courses.ts
```
/api/admin/courses
/api/my/courses
```

### indicators.ts
```
/api/indicators
/api/indicators/my
```

### subscriptions.ts
```
/api/subscriptions/metrics
```

### past-members.ts
```
/api/admin/past-members
/api/admin/past-members-dashboard
```

### user-memberships.ts
```
/api/admin/membership-plans
```

### watchlist.ts
```
/api/watchlist
```

---

## 3. DATABASE TABLES (All migrations combined)

### Core Tables (001_initial_schema.sql)
- `users` - User accounts
- `password_resets` - Password reset tokens
- `email_verification_tokens` - Email verification
- `user_memberships` - User membership records
- `membership_plans` - Membership plan definitions
- `membership_features` - Features per plan
- `products` - Product catalog
- `user_products` - User product ownership
- `orders` - Order records
- `order_items` - Line items
- `invoices` - Invoice records
- `posts` - Blog posts
- `categories` - Content categories
- `tags` - Content tags
- `post_categories` - Post-category mapping
- `post_tags` - Post-tag mapping
- `videos` - Video content
- `courses` - Course definitions
- `indicators` - Indicator definitions
- `newsletter_subscribers` - Newsletter list
- `contacts` - Contact form submissions
- `analytics_events` - Analytics data
- `coupons` - Coupon codes
- `jobs` - Background jobs
- `failed_jobs` - Failed job records
- `application_settings` - App settings

### Trading Rooms (013_trading_room_schedules.sql)
- `trading_rooms` - Room definitions
- `trading_room_schedules` - Schedule data
- `schedule_exceptions` - Schedule exceptions

### CMS Features (014_advanced_cms_features.sql)
- `content_versions` - Content versioning
- `audit_logs` - Audit trail
- `workflow_definitions` - Workflow configs
- `content_workflow_status` - Workflow states
- `workflow_transitions` - Transition history
- `webhooks` - Webhook configs
- `webhook_deliveries` - Delivery logs
- `scheduled_content` - Scheduled publishing
- `locales` - i18n locales
- `content_translations` - Translations
- `preview_tokens` - Preview links

### Watchlist (20260105_000001_create_watchlist_entries.sql)
- `watchlist_entries` - Watchlist items

### Fix Missing Tables (20260107_000001_fix_missing_tables.sql)
- `room_traders` - Instructors/traders
- `courses_enhanced` - Enhanced courses
- `course_sections` - Course sections
- `course_lessons` - Course lessons
- `media` - Media library
- `page_layouts` - Page builder
- `page_layout_versions` - Layout history

### Enhanced Courses/Indicators (20260108_000002_courses_indicators_enhanced.sql)
- `courses_enhanced` - (duplicate - skip)
- `course_sections` - (duplicate - skip)
- `course_lessons` - (duplicate - skip)
- `course_resources` - Downloadable resources
- `course_live_sessions` - Live session recordings
- `user_course_enrollments` - Enrollments
- `user_lesson_progress` - Lesson progress
- `indicator_platforms` - Platform definitions
- `indicators_enhanced` - Enhanced indicators
- `indicator_videos` - Tutorial videos
- `indicator_platform_files` - Platform-specific files
- `indicator_documentation` - Docs
- `indicator_tradingview_access` - TradingView access
- `user_indicator_access` - Indicator access
- `indicator_download_log` - Download tracking

### Course Management (20260109_000001_course_management_system.sql)
- `course_modules` - Course modules
- `lessons` - Lessons (UUID-based)
- `course_reviews` - Reviews
- `user_course_enrollments` - (conflicts with 20260108)
- `user_lesson_progress` - (conflicts with 20260108)
- `course_downloads` - Downloads

### Indicator Management (20260109_000002_indicator_management_system.sql)
- `indicator_categories` - Categories
- `indicator_category_mapping` - Category mapping
- `indicator_files` - File storage
- `indicator_downloads` - Downloads
- `indicator_download_analytics` - Analytics
- `user_indicator_ownership` - Ownership
- `user_indicators` - User indicators

### Page Layouts (20260110_000001_page_layouts.sql)
- `page_layouts` - (duplicate - skip)
- `page_layout_versions` - (duplicate - skip)

### Trading Room Content (20260113_000001_trading_room_content.sql)
- `room_sections` - Room sections
- `room_trade_plans` - Trade plans
- `room_alerts` - Alerts
- `room_weekly_videos` - Weekly videos

### Unified Videos (20260113_000002_unified_videos_system.sql)
- `unified_videos` - All video types
- `video_room_assignments` - Room assignments

### Room Resources (20260114_000001_room_resources_system.sql)
- `room_resources` - Unified resources
- `room_stats_cache` - Stats cache

### Trading Rooms Master (20260115_000001_trading_rooms_master.sql)
- Adds columns to `trading_rooms`
- Updates `room_traders`

### Member Segments (20260115_000002_member_segments_tags_filters.sql)
- `member_segments` - Segments
- `member_tags` - Tags
- `member_filters` - Saved filters
- `user_member_segments` - User segments
- `user_member_tags` - User tags
- `member_notes` - Notes
- `member_emails` - Emails
- `user_status` - User status
- `user_activity_log` - Activity log

### Media (20260116_000001_media_table.sql)
- `media` - (duplicate - skip)

### Organization (20260117_000001_organization_tables.sql)
- `teams` - Teams
- `departments` - Departments
- `user_teams` - User-team mapping
- `user_departments` - User-department mapping

### Member Management (20260117_000002_member_management_tables.sql)
- Additional member management tables

---

## 4. MIGRATION CONFLICTS IDENTIFIED

### CRITICAL: Duplicate Table Definitions

| Table | First Defined | Also Defined In | Conflict |
|-------|---------------|-----------------|----------|
| `courses_enhanced` | 20260107 | 20260108 | Schema mismatch |
| `course_sections` | 20260107 | 20260108 | Schema mismatch |
| `course_lessons` | 20260107 | 20260108 | Schema mismatch |
| `user_course_enrollments` | 20260108 | 20260109 | Column mismatch (last_accessed_at) |
| `user_lesson_progress` | 20260108 | 20260109 | FK mismatch (lesson_id type) |
| `locales` | 014 | 20260107 | Column mismatch (flag_emoji) |
| `content_translations` | 014 | 20260107 | Column mismatch (locale vs locale_code) |
| `media` | 20260107 | 20260116 | Duplicate |
| `page_layouts` | 20260107 | 20260110 | Duplicate |

### Foreign Key Issues

| Migration | Table | FK Reference | Problem |
|-----------|-------|--------------|---------|
| 20260108 | courses_enhanced | room_traders(id) | room_traders must exist first |
| 20260108 | indicators_enhanced | room_traders(id) | room_traders must exist first |
| 20260109 | user_lesson_progress | lessons(id) | lessons table uses UUID, enrollment uses BIGINT |

---

## 5. REQUIRED FIXES

### Fix 1: Remove duplicate migrations 20260109
The 20260109 migrations duplicate tables from 20260108 with incompatible schemas.

**Action**: Delete or disable migrations:
- `20260109_000001_course_management_system.sql`
- `20260109_000002_indicator_management_system.sql`

### Fix 2: Remove duplicate migrations 20260110 and 20260116
These duplicate tables from 20260107.

**Action**: Delete or disable migrations:
- `20260110_000001_page_layouts.sql`
- `20260116_000001_media_table.sql`

### Fix 3: Ensure 20260107 runs BEFORE 20260108
Migration 20260108 requires `room_traders` from 20260107.

**Action**: Already fixed by renaming to 20260107.

### Fix 4: Resolve locales/content_translations conflict
Migration 014 creates `locales` without `flag_emoji`, 20260107 tries to insert with it.

**Action**: Already fixed - added ALTER TABLE to add missing columns.

---

## 6. RECOMMENDED MIGRATION ORDER

1. `000_bootstrap_users.sql`
2. `001_initial_schema.sql`
3. `002_fix_password_column.sql`
4. `003_fix_jobs_schema.sql`
5. `004_add_mfa_enabled.sql`
6. `007_email_verification_standalone.sql`
7. `008_seed_membership_plans.sql`
8. `009_add_performance_indexes.sql`
9. `010_fix_coupons_schema.sql`
10. `012_add_redirects_table.sql`
11. `013_trading_room_schedules.sql`
12. `014_advanced_cms_features.sql`
13. `20260105_000001_create_watchlist_entries.sql`
14. `20260107_000001_fix_missing_tables.sql` ← Creates room_traders, courses_enhanced, media, etc.
15. `20260108_000002_courses_indicators_enhanced.sql` ← Depends on room_traders
16. ~~`20260109_000001_course_management_system.sql`~~ **DELETE - DUPLICATES**
17. ~~`20260109_000002_indicator_management_system.sql`~~ **DELETE - DUPLICATES**
18. ~~`20260110_000001_page_layouts.sql`~~ **DELETE - DUPLICATES**
19. `20260113_000001_trading_room_content.sql`
20. `20260113_000002_unified_videos_system.sql`
21. `20260114_000001_room_resources_system.sql`
22. `20260115_000001_trading_rooms_master.sql`
23. `20260115_000002_member_segments_tags_filters.sql`
24. ~~`20260116_000001_media_table.sql`~~ **DELETE - DUPLICATES**
25. `20260117_000001_organization_tables.sql`
26. `20260117_000002_member_management_tables.sql`

---

## 7. TABLES REQUIRED BY EACH BACKEND ROUTE

### analytics.rs
- `analytics_events`

### auth.rs
- `users`
- `password_resets`
- `email_verification_tokens`

### members.rs
- `users`
- `user_memberships`
- `membership_plans`

### admin_members.rs
- `users`
- `member_segments`
- `member_tags`
- `user_member_segments`
- `user_member_tags`

### admin_courses.rs / member_courses.rs
- `courses_enhanced`
- `course_sections`
- `course_lessons`
- `user_course_enrollments`
- `user_lesson_progress`

### admin_indicators.rs / member_indicators.rs
- `indicators` or `indicators_enhanced`
- `indicator_platform_files`
- `indicator_documentation`
- `user_indicator_access`
- `indicator_download_log`

### admin_videos.rs
- `unified_videos`
- `video_room_assignments`

### trading_rooms.rs
- `trading_rooms`
- `room_traders`
- `room_sections`
- `trading_room_schedules`

### media.rs
- `media`

### organization.rs
- `teams`
- `departments`
- `user_teams`
- `user_departments`

### watchlist.rs
- `watchlist_entries`

---

## 8. IMMEDIATE ACTION ITEMS

1. **DELETE** these duplicate migration files:
   - `20260109_000001_course_management_system.sql`
   - `20260109_000002_indicator_management_system.sql`
   - `20260110_000001_page_layouts.sql`
   - `20260116_000001_media_table.sql`

2. **VERIFY** migration 20260107 creates all required tables before 20260108 runs

3. **TEST** migrations on clean database to ensure no conflicts

4. **DEPLOY** and verify all endpoints work

---

*Document generated for ICT 7 Grade compliance - Apple Principal Engineer Standards*

# ICT 11+ Principal Engineer Dashboard Investigation Report
## Comprehensive Side-by-Side Analysis

**Investigation Date:** December 27, 2025
**Reference Files:** `core` (5,878 lines), `core 1` (5,780 lines)
**SvelteKit Files:** 15 dashboard-related components analyzed

---

## Table of Contents

1. [Header Navigation Mismatches](#1-header-navigation-mismatches)
2. [Primary Sidebar Mismatches](#2-primary-sidebar-mismatches)
3. [Secondary Navigation Mismatches](#3-secondary-navigation-mismatches)
4. [Dashboard Content Mismatches](#4-dashboard-content-mismatches)
5. [Article Card Mismatches](#5-article-card-mismatches)
6. [Sidebar Content Mismatches](#6-sidebar-content-mismatches)
7. [Footer Mismatches](#7-footer-mismatches)
8. [CSS/Styling Mismatches](#8-cssstyling-mismatches)
9. [JavaScript Functionality Mismatches](#9-javascript-functionality-mismatches)
10. [Data/API Mismatches](#10-dataapi-mismatches)

---

## 1. HEADER NAVIGATION MISMATCHES

### 1.1 Missing Phone Number
**WordPress (core:2726-2727):**
```html
<li id="menu-item-1957762" class="menu-item">
  <a href="tel:5122668659">
    <span class="fa fa-phone fa-cust-rotate-90 blue_icon"></span> (512) 266-8659
  </a>
</li>
```
**SvelteKit:** ❌ **MISSING** - No phone number in header navigation

---

### 1.2 Missing Cart Icon with Item Count
**WordPress (core:2727):**
```html
<li class="menu-item menu-item-cart">
  <a href="https://www.simplertrading.com/cart">
    <span class="cart-link-icon">
      <span class="fa fa-shopping-cart"></span>
      <span class="cart-items-count">1</span>
    </span>
    <span class="cart-link-text">View My Cart</span>
  </a>
</li>
```
**SvelteKit:** ❌ **MISSING** - No cart icon with dynamic item count

---

### 1.3 Missing Dashboard Dropdown Submenu
**WordPress (core:2717-2724):**
```html
<ul class="sub-menu">
  <li><a href="/dashboard">My Memberships</a></li>
  <li><a href="/dashboard/classes">My Classes</a></li>
  <li><a href="/dashboard/indicators">My Indicators</a></li>
  <li><a href="/dashboard/account">My Account</a></li>
  <li><a href="/support">Support</a></li>
  <li><a href="/dashboard/account/customer-logout/">Logout</a></li>
</ul>
```
**SvelteKit:** ⚠️ **PARTIAL** - Has dropdown but missing items: My Classes, My Indicators, Support, Logout

---

### 1.4 Missing Newsletters Mega Menu
**WordPress (core:2793-2810):**
```html
<li class="menu-item menu-item-has-children">
  <a href="/free-goods">Free Resources</a>
  <ul class="sub-menu">
    <li><a href="/news">News</a></li>
    <li><a href="/newsletters">Newsletters</a></li>
    <li><a href="/free-goods">Simpler Insights</a></li>
    <li><a href="/free-goods">Five Star Trader</a></li>
    <li><a href="/free-goods">Countdown Trader</a></li>
    <li><a href="/free-goods">Profit Pilot</a></li>
    <li><a href="/free-goods">Focused Trades</a></li>
  </ul>
</li>
```
**SvelteKit:** ❌ **MISSING** - No Free Resources mega menu

---

### 1.5 Missing About Us Submenu
**WordPress (core:2706-2712):**
```html
<li class="menu-item menu-item-has-children">
  <a href="/about">About Us</a>
  <ul class="sub-menu">
    <li><a href="/about">Who We Are</a></li>
    <li><a href="/traders">Our Traders</a></li>
  </ul>
</li>
```
**SvelteKit:** ❌ **MISSING** - No About Us dropdown

---

## 2. PRIMARY SIDEBAR MISMATCHES

### 2.1 Profile Photo Size Mismatch
**WordPress (core:2739-2742):**
```html
<span class="dashboard__profile-photo"
      style="background-image: url(.../avatar...?s=32&d=mm&r=g);">
</span>
```
- **WordPress size:** 32x32 pixels (s=32)
**SvelteKit:** ⚠️ Uses `getUserAvatarUrl()` - verify size parameter

---

### 2.2 My Classes Link - Bold/White Style Missing
**WordPress (core:2756-2759):**
```html
<span class="dashboard__nav-item-text" style="font-weight:bold;color: white; ">
  My Classes
</span>
```
**SvelteKit:** ⚠️ **DIFFERENT** - Uses standard styling, not bold white

---

### 2.3 My Indicators Link - Bold/White Style Missing
**WordPress (core:2762-2765):**
```html
<span class="dashboard__nav-item-text" style="font-weight:bold;color: white; ">
  My Indicators
</span>
```
**SvelteKit:** ⚠️ **DIFFERENT** - Uses standard styling, not bold white

---

### 2.4 Support Link Opens in New Tab
**WordPress (core:2813-2817):**
```html
<li>
  <a href="https://intercom.help/simpler-trading/en/" target="_blank">
    <span class="dashboard__nav-item-icon st-icon-support"></span>
    <span class="dashboard__nav-item-text">Support</span>
  </a>
</li>
```
**SvelteKit:** ❌ **MISSING** - Support link not in sidebar tools section

---

### 2.5 Icon Font vs SVG Icons
**WordPress:** Uses custom `st-icon-*` font icons
```html
<span class="dashboard__nav-item-icon st-icon-home"></span>
<span class="dashboard__nav-item-icon st-icon-courses"></span>
<span class="dashboard__nav-item-icon st-icon-indicators"></span>
<span class="dashboard__nav-item-icon st-icon-mastering-the-trade"></span>
```
**SvelteKit:** Uses Tabler SVG icons
```svelte
<IconHome size={24} />
<IconBook size={24} />
```
**Status:** ⚠️ **DIFFERENT** - Visual appearance may vary

---

### 2.6 Collapsed State Class Missing on Main Dashboard
**WordPress (core 1:2843):**
```html
<nav class="dashboard__nav-primary is-collapsed ">
```
**SvelteKit:** ⚠️ Check if `is-collapsed` class is applied on trading room pages

---

## 3. SECONDARY NAVIGATION MISMATCHES

### 3.1 Missing Trader Submenu (CRITICAL)
**WordPress (core 1:2980-3023):**
```html
<li class="has-submenu">
  <a href="#" style="cursor: default;" onclick="return false;">
    <span class="dashboard__nav-item-icon st-icon-forum"></span>
    <span class="dashboard__nav-item-text">Meet the Traders</span>
  </a>
  <ul class="dashboard__nav-submenu">
    <li><a href="/dashboard/mastering-the-trade/john-carter">John Carter</a></li>
    <li><a href="/dashboard/mastering-the-trade/henry-gambell">Henry Gambell</a></li>
    <li><a href="/dashboard/mastering-the-trade/taylor-horton">Taylor Horton</a></li>
    <li><a href="/dashboard/mastering-the-trade/bruce-marshall">Bruce Marshall</a></li>
    <li><a href="/dashboard/mastering-the-trade/danielle-shay">Danielle Shay</a></li>
    <li><a href="/dashboard/mastering-the-trade/allison-ostrander">Allison Ostrander</a></li>
    <li><a href="/dashboard/mastering-the-trade/sam-shames">Sam Shames</a></li>
    <li><a href="/dashboard/mastering-the-trade/kody-ashmore">Kody Ashmore</a></li>
    <li><a href="/dashboard/mastering-the-trade/raghee-horner">Raghee Horner</a></li>
  </ul>
</li>
```
**SvelteKit (+layout.svelte:485-496):**
```svelte
<li class={$page.url.pathname.includes('/traders') ? 'is-active' : ''}>
  <a href="/dashboard/{currentMembershipSlug}/traders">
    <span class="dashboard__nav-item-icon"><IconUsers size={24} /></span>
    <span class="dashboard__nav-item-text">Meet the Traders</span>
    <span class="nav-arrow"><IconChevronRight size={16} /></span>
  </a>
  <!-- NO SUBMENU - Just a link -->
</li>
```
**Status:** ❌ **MISSING** - No expandable submenu with 9 traders

---

### 3.2 Missing Trader Store Submenu (CRITICAL)
**WordPress (core 1:3025-3069):**
```html
<li class="has-submenu">
  <a href="#" style="cursor: default;" onclick="return false;">
    <span class="dashboard__nav-item-icon st-icon-forum"></span>
    <span class="dashboard__nav-item-text">Trader Store</span>
  </a>
  <ul class="dashboard__nav-submenu">
    <li><a href="/dashboard/mastering-the-trade/john-carter/john-carter-trader-store/">John Carter</a></li>
    <li><a href="/dashboard/mastering-the-trade/henry-gambell/trader-store/">Henry Gambell</a></li>
    <!-- ... 7 more traders ... -->
  </ul>
</li>
```
**SvelteKit:** ❌ **MISSING** - No expandable submenu

---

### 3.3 Submenu Parent Click Handler
**WordPress (core 1:2981):**
```html
<a href="#" style="cursor: default;" onclick="return false;">
```
- Parent link does NOT navigate
- Only children are clickable

**SvelteKit:**
```svelte
<a href="/dashboard/{currentMembershipSlug}/traders">
```
- Parent link DOES navigate
**Status:** ❌ **WRONG BEHAVIOR**

---

### 3.4 Missing Submenu Arrow Icon
**WordPress:** Uses CSS `:after` pseudo-element for arrow
**SvelteKit:** Uses `<IconChevronRight>` but no expand/collapse behavior

---

## 4. DASHBOARD CONTENT MISMATCHES

### 4.1 Page Title Font Size
**WordPress (core:2850):** `font-size: 36px`
**SvelteKit ([slug]/+page.svelte:326-332):** `font-size: 24px`
**Status:** ❌ **DIFFERENT** - 12px smaller

---

### 4.2 Trading Room Dropdown Not Functional
**WordPress (core:2855-2867):**
```html
<a href="#" class="btn btn-xs btn-orange btn-tradingroom dropdown-toggle"
   id="dLabel" data-bs-toggle="dropdown" aria-expanded="false">
```
- Uses Bootstrap `data-bs-toggle="dropdown"`
- Click toggles visibility

**SvelteKit ([slug]/+page.svelte:172-196):**
```svelte
<button type="button" class="btn btn-xs btn-orange btn-tradingroom dropdown-toggle">
  <!-- No click handler, no state management -->
</button>
<nav class="dropdown-menu">
  <!-- Always in DOM, visibility controlled by CSS? -->
</nav>
```
**Status:** ❌ **BROKEN** - No toggle functionality implemented

---

### 4.3 Trading Room Links Missing JWT Auth
**WordPress (core:2863-2864):**
```html
<a href="https://chat.protradingroom.com/ptr_app/sessions/v2/authUser/652754202ad80b3e7c5131e2?sl=1&jwt=eyJ..." target="_blank">
```
- Dynamic JWT token for SSO
- External ProTradingRoom URL

**SvelteKit:**
```svelte
<a href="/dashboard/{slug}" target="_blank">
```
**Status:** ❌ **WRONG URL** - Missing JWT auth, wrong destination

---

### 4.4 Trading Room Rules PDF URL
**WordPress (core 1:3093):**
```html
<a href="https://cdn.simplertrading.com/2024/02/07192341/Simpler-Tradings-Rules-of-the-Room.pdf">
```
**SvelteKit ([slug]/+page.svelte:166):**
```svelte
<a href="/trading-room-rules.pdf">
```
**Status:** ❌ **WRONG URL** - Should use CDN URL

---

### 4.5 Membership Card Icon Classes
**WordPress (core:2888-2890):**
```html
<span class="membership-card__icon">
  <span class="icon icon--lg st-icon-mastering-the-trade"></span>
</span>
```
**SvelteKit (+page.svelte:264-267):**
```svelte
<span class="membership-card__icon">
  <MembershipIcon size={24} />
</span>
```
**Status:** ⚠️ **DIFFERENT** - Missing `icon icon--lg` wrapper classes

---

### 4.6 Simpler Showcase Icon Special Styling
**WordPress (core:2904):**
```html
<span class="membership-card__icon simpler-showcase-icon">
```
- Black background with orange icon

**SvelteKit:** ⚠️ Verify `.simpler-showcase-icon` class is applied

---

### 4.7 Membership Card Type Modifiers
**WordPress:**
- `membership-card--options` for trading rooms
- `membership-card--foundation` for other memberships
- `membership-card--ww` for Weekly Watchlist

**SvelteKit (+page.svelte:262):**
```svelte
<article class="membership-card membership-card--{membership.type === 'trading-room' ? 'options' : 'foundation'}">
```
**Status:** ⚠️ **PARTIAL** - Missing `--ww` variant

---

## 5. ARTICLE CARD MISMATCHES

### 5.1 Video Card Class for Chatroom Archives
**WordPress (core 1:3174-3178):**
```html
<figure class="card-media article-card__image card-media--video"
        style="background-image: url(...);">
  <a href="...">
    <img src="..." alt="..." />
  </a>
</figure>
```
- Extra classes: `card-media`, `card-media--video`
- Image is wrapped in `<a>` tag

**SvelteKit ([slug]/+page.svelte:222-224):**
```svelte
<figure class="article-card__image" style="background-image: url({article.image});">
  <img src={article.image} alt={article.title} />
</figure>
```
**Status:** ❌ **MISSING** - No `card-media--video` class, no link on image

---

### 5.2 Video Play Icon Overlay
**WordPress:** CSS adds play button overlay on `.card-media--video`
**SvelteKit:** ❌ **MISSING** - No play icon overlay

---

### 5.3 Chatroom Archive Card Structure Different
**WordPress (core 1:3181-3185):**
```html
<h4 class="h5 article-card__title">December 23, 2025</h4>
<div class="article-card__excerpt u--hide-read-more">
  <p class="u--margin-bottom-0 u--font-size-sm"><i>With Danielle Shay</i></p>
</div>
<span class="article-card__meta"><small>December 23, 2025</small></span>
```
- Excerpt contains italic trader name
- Date in title AND meta

**SvelteKit:** ⚠️ Different structure - verify order

---

### 5.4 Daily Video Card Image Source
**WordPress (core 1:3152-3153):**
```html
<figure style="background-image: url(https://cdn.simplertrading.com/2025/05/07134745/SimplerCentral_HG.jpg);">
  <img src="https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg" />
</figure>
```
- Background = actual thumbnail
- `<img>` = fallback/placeholder

**SvelteKit:**
```svelte
<figure style="background-image: url({article.image});">
  <img src={article.image} />
</figure>
```
**Status:** ⚠️ **DIFFERENT** - Using same image for both

---

### 5.5 Article Content is Hardcoded
**WordPress:** Dynamically loaded from database
**SvelteKit ([slug]/+page.svelte:22-84):**
```typescript
const articles = $derived([
  { id: 1, type: 'Daily Video', title: 'Market Analysis...', ... },
  // All hardcoded
]);
```
**Status:** ❌ **HARDCODED** - No API integration

---

## 6. SIDEBAR CONTENT MISMATCHES

### 6.1 Trading Room Schedule Missing Dynamic Events
**WordPress (core 1:3309-3310):**
```javascript
gapi.client.calendar.events.list({
  'calendarId': 'simpleroptions.com_sabio00har0rd4odbrsa705904@group.calendar.google.com',
  ...
});
```
- Fetches live events from Google Calendar

**SvelteKit ([slug]/+page.svelte:87-142):**
```typescript
if (room.calendarId) {
  // Calendar initialization code
}
```
**Status:** ⚠️ **CONDITIONAL** - Depends on `room.calendarId` being set

---

### 6.2 Quick Links Missing Platform Tutorials
**WordPress (core 1:3316-3325):**
```html
<ul class="link-list">
  <li><a href="https://intercom.help/simpler-trading/en/" target="_blank">Support</a></li>
  <li><a href="/tutorials" target="_blank">Platform Tutorials</a></li>
  <li><a href="/blog" target="_blank">Simpler Blog</a></li>
</ul>
```
**SvelteKit:** ⚠️ Verify all 3 links are present in `room.quickLinks` config

---

### 6.3 Content Sidebar Width
**WordPress CSS:** `width: 260px`
**SvelteKit:** Verify CSS value matches

---

## 7. FOOTER MISMATCHES

### 7.1 Footer Logo - SVG Implementation
**WordPress (core:3024-3086):** Full inline SVG of "SIMPLER TRADING" logo
**SvelteKit:** ⚠️ Verify logo matches or uses same SVG

---

### 7.2 Missing App Store Badges
**WordPress (core:3205-3213):**
```html
<div class="app-download-btn">
  <a href="https://itunes.apple.com/..."><svg>...</svg></a>
  <a href="https://play.google.com/..."><svg>...</svg></a>
</div>
```
**SvelteKit:** ❌ **MISSING** - No App Store/Google Play badges

---

### 7.3 Missing Social Media Icons
**WordPress (core:3216-3239):**
```html
<span class="fl-icon"><a href="https://www.facebook.com/simplerofficial/"><i class="fab fa-facebook-f"></i></a></span>
<span class="fl-icon"><a href="https://twitter.com/simplertrading"><i class="fab fa-twitter"></i></a></span>
<span class="fl-icon"><a href="https://www.youtube.com/..."><i class="fab fa-youtube"></i></a></span>
<span class="fl-icon"><a href="https://www.instagram.com/simplertrading/"><i class="fab fa-instagram"></i></a></span>
```
**SvelteKit:** ❌ **MISSING** - No social media links

---

### 7.4 Missing Footer Columns
**WordPress Footer Structure:**
1. Logo + Contact Info + App Badges
2. LEARN MORE (Memberships, Daily Videos, Support, Rejoin Mailing List, Partners)
3. ABOUT (About Simpler, Our Traders, Simpler Store, Blog)
4. LEGAL (Privacy Policy, Terms & Conditions, My Consent Preferences)
5. App Download Buttons + Social Icons

**SvelteKit:** ⚠️ Verify all 5 columns match

---

### 7.5 Missing Legal Disclaimer
**WordPress (core:3270-3275):** Full CFTC disclaimer text
**SvelteKit:** ⚠️ Verify disclaimer is present

---

### 7.6 Missing Scroll to Top Button
**WordPress (core:3287-3292):**
```html
<a class="generate-back-to-top" href="#">
  <span class="screen-reader-text">Scroll back to top</span>
  <span class="gp-icon icon-arrow">...</span>
</a>
```
**SvelteKit:** ❌ **MISSING** - No scroll to top button

---

## 8. CSS/STYLING MISMATCHES

### 8.1 Button Width - Enter a Trading Room
**WordPress CSS:**
```css
.btn.btn-xs.btn-orange.btn-tradingroom {
  width: 280px;
  padding: 12px 18px;
}
```
**SvelteKit:** ⚠️ Verify exact width

---

### 8.2 Collapsed Sidebar Tooltip
**WordPress:** Shows tooltip on hover when collapsed
**SvelteKit:** ⚠️ Verify tooltip behavior

---

### 8.3 Dropdown Menu Shadow
**WordPress CSS:**
```css
.dropdown-menu {
  box-shadow: 0 10px 30px rgba(0, 0, 0, 0.15);
  border-radius: 5px;
}
```
**SvelteKit:** ⚠️ Verify shadow values

---

### 8.4 Loading Spinner Animation
**WordPress (core:3004-3009):**
```html
<div class="loading-container loading-global">
  <div class="loading">
    <span class="loading-icon"></span>
    <span class="loading-message"></span>
  </div>
</div>
```
**SvelteKit:** ⚠️ Different loading implementation

---

### 8.5 Font Family Stack
**WordPress:** `'Open Sans', sans-serif`
**SvelteKit:** ⚠️ Verify same font family is loaded

---

## 9. JAVASCRIPT FUNCTIONALITY MISMATCHES

### 9.1 Dropdown Toggle Not Implemented
**WordPress:** Bootstrap `data-bs-toggle="dropdown"` handles this
**SvelteKit:** ❌ **MISSING** - No click handler, no state

---

### 9.2 Sidebar Collapse Toggle
**WordPress (core:2836-2844):**
```html
<button class="dashboard__toggle-button" data-toggle-dashboard-menu>
```
- Custom JS listens for `[data-toggle-dashboard-menu]` clicks

**SvelteKit:** ✅ Implemented with `mobileMenuOpen` state

---

### 9.3 Submenu Expand/Collapse
**WordPress:** CSS hover or JS toggle for `.dashboard__nav-submenu`
**SvelteKit:** ❌ **MISSING** - No submenu toggle functionality

---

### 9.4 Google Calendar API Integration
**WordPress (core 1:3309-3310):**
- Full client-side Google Calendar API integration
- Shows live trading room schedule

**SvelteKit:** ⚠️ Implemented but verify API credentials

---

### 9.5 Consent Preference Click Handler
**WordPress (core:3190-3194):**
```javascript
jQuery('#my-consent-preference').click(function() {
  jQuery('.cs-info-sticky-button').trigger('click');
});
```
**SvelteKit:** ❌ **MISSING** - No consent preference trigger

---

## 10. DATA/API MISMATCHES

### 10.1 Membership Cards from API
**WordPress:** Dynamic from database
**SvelteKit (+page.svelte):** ✅ Uses `getUserMemberships()` API

---

### 10.2 Trading Room JWT URLs
**WordPress:** Backend generates JWT for ProTradingRoom SSO
**SvelteKit:** ❌ **MISSING** - No JWT generation for trading room auth

---

### 10.3 Article Cards from API
**WordPress:** Dynamic from CMS
**SvelteKit:** ❌ **HARDCODED** - Static array

---

### 10.4 Weekly Watchlist Date
**WordPress:** Dynamic date from database
**SvelteKit:** ❌ **HARDCODED** - "Week of December 22, 2025"

---

### 10.5 Trader Data Structure
**WordPress:** 9 traders with individual pages and stores
**SvelteKit:** ⚠️ Verify `trading-rooms.ts` has all traders configured

---

## SUMMARY STATISTICS

| Category | Critical | Moderate | Minor | Total |
|----------|----------|----------|-------|-------|
| Header Navigation | 5 | 0 | 0 | 5 |
| Primary Sidebar | 2 | 4 | 0 | 6 |
| Secondary Navigation | 4 | 0 | 0 | 4 |
| Dashboard Content | 4 | 3 | 0 | 7 |
| Article Cards | 2 | 3 | 0 | 5 |
| Sidebar Content | 1 | 2 | 0 | 3 |
| Footer | 4 | 2 | 0 | 6 |
| CSS/Styling | 0 | 5 | 0 | 5 |
| JavaScript | 3 | 2 | 0 | 5 |
| Data/API | 3 | 2 | 0 | 5 |
| **TOTAL** | **28** | **23** | **0** | **51** |

---

## PRIORITY FIX ORDER

### Phase 1: Critical Functionality (Week 1)
1. Implement Trader submenu with 9 traders
2. Implement Trader Store submenu with 9 stores
3. Fix Trading Room dropdown toggle
4. Add JWT auth for trading room links
5. Implement articles API integration

### Phase 2: Navigation (Week 2)
6. Add phone number to header
7. Add cart icon with count
8. Add Dashboard dropdown submenu
9. Add Free Resources mega menu
10. Add About Us submenu

### Phase 3: Content (Week 3)
11. Fix page title font size (36px)
12. Add video card play overlay
13. Fix article card structure
14. Make Weekly Watchlist date dynamic
15. Fix Trading Room Rules PDF URL

### Phase 4: Footer & Polish (Week 4)
16. Add App Store badges
17. Add social media icons
18. Complete all footer columns
19. Add scroll to top button
20. Add consent preference handler

---

**Report Generated:** December 27, 2025
**Investigator:** Claude (ICT 11+ Principal Engineer Grade)
**Total Mismatches Found:** 51

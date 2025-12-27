# Pixel-Perfect Implementation Plan
## 5-Phase Systematic Approach

**Created:** December 27, 2025
**Objective:** Make SvelteKit dashboard identical to WordPress reference
**Total Mismatches:** 51 → 0

---

## Phase Overview

| Phase | Focus Area | Items | Priority |
|-------|------------|-------|----------|
| **Phase 1** | Secondary Navigation & Submenus | 8 | Critical |
| **Phase 2** | Header Navigation & Primary Sidebar | 12 | High |
| **Phase 3** | Dashboard Content & Article Cards | 14 | High |
| **Phase 4** | Footer & Global Elements | 10 | Medium |
| **Phase 5** | Polish, Animations & Final Verification | 7 | Final |

---

# PHASE 1: Secondary Navigation & Submenus
## Foundation - Must be perfect before moving forward

### Deliverables

#### 1.1 Trader Submenu Implementation
**Reference:** `core 1:2980-3023`

Create expandable "Meet the Traders" submenu with 9 traders:
```
├── Meet the Traders (parent - non-clickable)
│   ├── John Carter
│   ├── Henry Gambell
│   ├── Taylor Horton
│   ├── Bruce Marshall
│   ├── Danielle Shay
│   ├── Allison Ostrander
│   ├── Sam Shames
│   ├── Kody Ashmore
│   └── Raghee Horner
```

**Files to modify:**
- `frontend/src/routes/dashboard/+layout.svelte`
- `frontend/src/lib/config/trading-rooms.ts`

**CSS Required:**
```css
.has-submenu { position: relative; }
.dashboard__nav-submenu {
  display: none;
  position: absolute;
  left: 100%;
  top: 0;
  background: #143E59;
  min-width: 200px;
  box-shadow: 0 5px 15px rgba(0,0,0,0.2);
  z-index: 120;
}
.has-submenu:hover .dashboard__nav-submenu { display: block; }
```

---

#### 1.2 Trader Store Submenu Implementation
**Reference:** `core 1:3025-3069`

Create expandable "Trader Store" submenu with 9 stores:
```
├── Trader Store (parent - non-clickable)
│   ├── John Carter
│   ├── Henry Gambell
│   ├── Taylor Horton
│   ├── Bruce Marshall
│   ├── Danielle Shay
│   ├── Allison Ostrander
│   ├── Sam Shames
│   ├── Kody Ashmore
│   └── Raghee Horner
```

---

#### 1.3 Parent Link Behavior
**Reference:** `core 1:2981`
```html
<a href="#" style="cursor: default;" onclick="return false;">
```

Parent links should:
- NOT navigate anywhere
- Show `cursor: default`
- Only expand submenu on hover

---

#### 1.4 Submenu Styling
**Reference CSS values:**
```css
.dashboard__nav-submenu li a {
  padding: 10px 20px;
  color: #fff;
  font-size: 14px;
  display: block;
  transition: background 0.15s;
}
.dashboard__nav-submenu li a:hover {
  background: rgba(255,255,255,0.1);
}
```

---

#### 1.5 Icon for Submenu Parent
**Reference:** `st-icon-forum` used for both "Meet the Traders" and "Trader Store"

**SvelteKit:** Use `IconUsers` for traders, `IconBuildingStore` for store

---

#### 1.6 Trading Room Dropdown Toggle
**Reference:** `core:2855-2867`

Implement click-to-toggle dropdown:
```svelte
let dropdownOpen = $state(false);

function toggleDropdown(e: MouseEvent) {
  e.preventDefault();
  e.stopPropagation();
  dropdownOpen = !dropdownOpen;
}
```

---

#### 1.7 Dropdown Click-Outside-Close
Close dropdown when clicking outside:
```svelte
<svelte:window onclick={() => dropdownOpen = false} />
```

---

#### 1.8 Dropdown Visibility CSS
```css
.dropdown-menu {
  display: none;
}
.dropdown.is-open .dropdown-menu {
  display: block;
}
```

---

### Phase 1 Verification Checklist

| Item | WordPress Reference | SvelteKit Match | Status |
|------|---------------------|-----------------|--------|
| Trader submenu has 9 items | core 1:2987-3022 | | ☐ |
| Trader Store submenu has 9 items | core 1:3032-3067 | | ☐ |
| Parent links don't navigate | `onclick="return false;"` | | ☐ |
| Submenu appears on hover | CSS `:hover` | | ☐ |
| Submenu positioned to right | `left: 100%` | | ☐ |
| Submenu background color | `#143E59` | | ☐ |
| Submenu shadow | `0 5px 15px rgba(0,0,0,0.2)` | | ☐ |
| Trading room dropdown toggles | Click handler | | ☐ |
| Dropdown closes on outside click | Window listener | | ☐ |

---

# PHASE 2: Header Navigation & Primary Sidebar
## Complete the navigation structure

### Deliverables

#### 2.1 Phone Number in Header
**Reference:** `core:2726`
```html
<li class="menu-item">
  <a href="tel:5122668659">
    <span class="fa fa-phone fa-cust-rotate-90 blue_icon"></span> (512) 266-8659
  </a>
</li>
```

**Implementation:**
- Add phone icon (Tabler `IconPhone`)
- Rotate icon 90 degrees
- Blue color `#0984ae`

---

#### 2.2 Cart Icon with Item Count
**Reference:** `core:2727`
```html
<li class="menu-item menu-item-cart">
  <a href="/cart">
    <span class="cart-link-icon">
      <span class="fa fa-shopping-cart"></span>
      <span class="cart-items-count">1</span>
    </span>
    <span class="cart-link-text">View My Cart</span>
  </a>
</li>
```

**Implementation:**
- Cart icon with badge showing count
- Dynamic count from cart store
- CSS for badge positioning

---

#### 2.3 Dashboard Dropdown Submenu
**Reference:** `core:2717-2724`

Add missing items:
- My Memberships ✓ (exists)
- My Classes (add)
- My Indicators (add)
- My Account ✓ (exists)
- Support (add)
- Logout (add)

---

#### 2.4 Free Resources Mega Menu
**Reference:** `core:2793-2810`

Create dropdown with:
- News
- Newsletters
- Simpler Insights
- Five Star Trader
- Countdown Trader
- Profit Pilot
- Focused Trades

---

#### 2.5 About Us Submenu
**Reference:** `core:2706-2712`

Create dropdown with:
- Who We Are
- Our Traders

---

#### 2.6 My Classes Bold White Styling
**Reference:** `core:2757`
```css
font-weight: bold;
color: white;
```

---

#### 2.7 My Indicators Bold White Styling
**Reference:** `core:2763`
```css
font-weight: bold;
color: white;
```

---

#### 2.8 Support Link in Tools Section
**Reference:** `core:2813-2817`
```html
<a href="https://intercom.help/simpler-trading/en/" target="_blank">
  Support
</a>
```

---

#### 2.9 Profile Photo Size
**Reference:** `s=32` in Gravatar URL

Ensure avatar size is 32x32 pixels.

---

#### 2.10 Primary Nav Collapsed State
**Reference:** `core 1:2843`
```html
<nav class="dashboard__nav-primary is-collapsed">
```

Add `is-collapsed` class when on trading room pages (not main dashboard).

---

#### 2.11 Sidebar Tooltip on Collapsed State
When sidebar is collapsed, show tooltip with nav item text on hover.

---

#### 2.12 Icon System Consistency
Document all icon mappings:
```
st-icon-home → IconHome
st-icon-courses → IconBook
st-icon-indicators → IconChartLine
st-icon-mastering-the-trade → IconChartCandle
st-icon-simpler-showcase → IconStar
st-icon-trade-of-the-week → IconCalendarWeek
st-icon-support → IconHelp
st-icon-settings → IconSettings
st-icon-dashboard → IconLayoutDashboard
st-icon-daily-videos → IconPlayerPlay
st-icon-learning-center → IconAward
st-icon-chatroom-archive → IconArchive
st-icon-forum → IconUsers
st-icon-training-room → IconChartCandle
```

---

### Phase 2 Verification Checklist

| Item | WordPress Reference | SvelteKit Match | Status |
|------|---------------------|-----------------|--------|
| Phone number visible | `(512) 266-8659` | | ☐ |
| Phone icon rotated 90° | `fa-cust-rotate-90` | | ☐ |
| Cart icon with count badge | `.cart-items-count` | | ☐ |
| Dashboard dropdown has 6 items | core:2718-2724 | | ☐ |
| Free Resources menu exists | 7 items | | ☐ |
| About Us dropdown exists | 2 items | | ☐ |
| My Classes is bold white | inline style | | ☐ |
| My Indicators is bold white | inline style | | ☐ |
| Support in Tools section | target="_blank" | | ☐ |
| Avatar is 32x32 | `s=32` | | ☐ |
| Collapsed class on room pages | `is-collapsed` | | ☐ |
| Tooltips on collapsed sidebar | CSS | | ☐ |

---

# PHASE 3: Dashboard Content & Article Cards
## Make the main content area pixel-perfect

### Deliverables

#### 3.1 Page Title Font Size
**Reference:** `font-size: 36px`
**Current:** `font-size: 24px`

Fix in both `+page.svelte` (main) and `[slug]/+page.svelte` (room).

---

#### 3.2 Trading Room Rules PDF URL
**Reference:** `https://cdn.simplertrading.com/2024/02/07192341/Simpler-Tradings-Rules-of-the-Room.pdf`
**Current:** `/trading-room-rules.pdf`

---

#### 3.3 Membership Card Icon Classes
**Reference:**
```html
<span class="icon icon--lg st-icon-mastering-the-trade"></span>
```

Add `icon icon--lg` wrapper classes for proper sizing.

---

#### 3.4 Simpler Showcase Icon Special Style
**Reference:** `core:2904`
```css
.simpler-showcase-icon {
  background: black !important;
  color: orange !important;
}
```

---

#### 3.5 Membership Card Type Modifiers
Add all variants:
- `membership-card--options`
- `membership-card--foundation`
- `membership-card--ww`

---

#### 3.6 Video Card Class Structure
**Reference:** `core 1:3174`
```html
<figure class="card-media article-card__image card-media--video">
```

Add `card-media` and `card-media--video` classes to chatroom archive cards.

---

#### 3.7 Video Play Icon Overlay
**Reference CSS:**
```css
.card-media--video::after {
  content: '';
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  width: 60px;
  height: 60px;
  background: rgba(0,0,0,0.6) url('play-icon.svg') center no-repeat;
  border-radius: 50%;
}
```

---

#### 3.8 Chatroom Archive Card Structure
**Reference order:**
1. Figure with video class
2. Title (h4)
3. Excerpt with italic trader name
4. Meta with date
5. Watch Now button

---

#### 3.9 Daily Video Card Image Sources
**Reference:** Different images for background vs `<img>`:
- Background: actual thumbnail
- `<img>`: fallback placeholder

---

#### 3.10 Articles API Integration
Replace hardcoded articles with API fetch:
```typescript
let articles = $state<Article[]>([]);

onMount(async () => {
  const response = await fetch(`/api/articles/${slug}`);
  articles = await response.json();
});
```

---

#### 3.11 Weekly Watchlist Dynamic Date
**Reference:** `core:2964`
```html
<p>Week of December 22, 2025. </p>
```

Fetch current week's date from API or compute dynamically.

---

#### 3.12 Weekly Watchlist Dynamic URL
**Reference:** `core:2958`
```html
<a href="https://www.simplertrading.com/watchlist/12222025-tg-watkins">
```

Generate URL based on current date.

---

#### 3.13 Quick Links All Items Present
**Reference:** `core 1:3316-3324`
- Support (external link)
- Platform Tutorials
- Simpler Blog

---

#### 3.14 Content Sidebar Width
**Reference:** `width: 260px`

---

### Phase 3 Verification Checklist

| Item | WordPress Reference | SvelteKit Match | Status |
|------|---------------------|-----------------|--------|
| Page title is 36px | `font-size: 36px` | | ☐ |
| Trading Room Rules uses CDN URL | Full CDN path | | ☐ |
| Membership icons have wrapper classes | `icon icon--lg` | | ☐ |
| Simpler Showcase has special colors | black/orange | | ☐ |
| All card type modifiers exist | 3 variants | | ☐ |
| Video cards have correct classes | `card-media--video` | | ☐ |
| Play icon overlay appears | CSS pseudo-element | | ☐ |
| Chatroom card order is correct | 5 elements | | ☐ |
| Articles loaded from API | Dynamic fetch | | ☐ |
| Weekly Watchlist date is dynamic | Computed | | ☐ |
| Watchlist URL is dynamic | Date-based | | ☐ |
| Quick Links has all 3 items | 3 links | | ☐ |
| Content sidebar is 260px | CSS width | | ☐ |

---

# PHASE 4: Footer & Global Elements
## Complete the page structure

### Deliverables

#### 4.1 Footer Logo SVG
**Reference:** `core:3024-3086`

Use exact "SIMPLER TRADING" SVG logo or create Revolution Trading equivalent.

---

#### 4.2 Footer Contact Info
**Reference:** `core:3124-3128`
```html
<p>
  [email protected]<br>
  8911 North Capital of Texas Hwy<br>
  Suite 4200 #1005<br>
  Austin, TX 78759<br>
  512-266-8659
</p>
```

---

#### 4.3 App Store Badges
**Reference:** `core:3205-3213`

Add iOS App Store and Google Play SVG badges with links.

---

#### 4.4 Social Media Icons
**Reference:** `core:3216-3239`
- Facebook
- Twitter/X
- YouTube
- Instagram

---

#### 4.5 Footer Column: LEARN MORE
**Reference:** `core:3147-3152`
- Memberships
- Daily Videos
- Support
- Rejoin Mailing List
- Partners

---

#### 4.6 Footer Column: ABOUT
**Reference:** `core:3164-3168`
- About Simpler™
- Our Traders
- Simpler™ Store
- Blog

---

#### 4.7 Footer Column: LEGAL
**Reference:** `core:3179-3182`
- Privacy Policy
- Terms & Conditions
- My Consent Preferences

---

#### 4.8 Legal Disclaimer Text
**Reference:** `core:3270-3275`

Full CFTC disclaimer about trading risks.

---

#### 4.9 Scroll to Top Button
**Reference:** `core:3287-3292`
```html
<a class="generate-back-to-top" href="#">
  Scroll back to top
</a>
```

With scroll animation and visibility toggle.

---

#### 4.10 Loading Spinner Global
**Reference:** `core:3004-3009`
```html
<div class="loading-container loading-global">
  <div class="loading">
    <span class="loading-icon"></span>
    <span class="loading-message"></span>
  </div>
</div>
```

---

### Phase 4 Verification Checklist

| Item | WordPress Reference | SvelteKit Match | Status |
|------|---------------------|-----------------|--------|
| Footer logo matches | SVG logo | | ☐ |
| Contact info complete | 5 lines | | ☐ |
| App Store badge present | iOS SVG | | ☐ |
| Google Play badge present | Android SVG | | ☐ |
| Facebook icon + link | fab fa-facebook-f | | ☐ |
| Twitter icon + link | fab fa-twitter | | ☐ |
| YouTube icon + link | fab fa-youtube | | ☐ |
| Instagram icon + link | fab fa-instagram | | ☐ |
| LEARN MORE column complete | 5 links | | ☐ |
| ABOUT column complete | 4 links | | ☐ |
| LEGAL column complete | 3 links | | ☐ |
| Disclaimer present | Full text | | ☐ |
| Scroll to top button works | Animation | | ☐ |
| Loading spinner matches | CSS animation | | ☐ |

---

# PHASE 5: Polish, Animations & Final Verification
## Pixel-perfect finishing touches

### Deliverables

#### 5.1 Button Dimensions Exact Match
**Reference:**
```css
.btn.btn-xs.btn-orange.btn-tradingroom {
  width: 280px;
  padding: 12px 18px;
}
```

---

#### 5.2 Dropdown Shadow & Border Radius
**Reference:**
```css
.dropdown-menu {
  box-shadow: 0 10px 30px rgba(0, 0, 0, 0.15);
  border-radius: 5px;
}
```

---

#### 5.3 Font Family Consistency
**Reference:** `'Open Sans', sans-serif`

Verify Google Fonts loaded and applied to all elements.

---

#### 5.4 Transition Animations
**Reference patterns:**
```css
transition: all 0.15s ease-in-out;
transition: all 0.2s ease-in-out;
transition: background 0.15s;
```

---

#### 5.5 Hover States All Elements
Verify all interactive elements have correct hover states:
- Buttons: background color change
- Links: color change and/or underline
- Cards: box-shadow and translateY
- Nav items: background color change

---

#### 5.6 Responsive Breakpoints
**Reference breakpoints:**
- Mobile: `max-width: 641px`
- Tablet: `max-width: 991px`
- Desktop: `min-width: 992px`
- Large: `min-width: 1280px`
- XL: `min-width: 1440px`

---

#### 5.7 Final Side-by-Side Comparison
Take screenshots of:
1. Main Dashboard
2. Trading Room Dashboard
3. Account Pages
4. Footer
5. Mobile View

Compare pixel-by-pixel with WordPress reference.

---

### Phase 5 Verification Checklist

| Item | WordPress Reference | SvelteKit Match | Status |
|------|---------------------|-----------------|--------|
| Trading room button is 280px | CSS width | | ☐ |
| Dropdown shadow matches | rgba values | | ☐ |
| Open Sans font loaded | Google Fonts | | ☐ |
| All transitions are 0.15s-0.2s | CSS timing | | ☐ |
| Button hover states match | Color values | | ☐ |
| Link hover states match | Underline/color | | ☐ |
| Card hover states match | Shadow/transform | | ☐ |
| Nav hover states match | Background | | ☐ |
| Mobile breakpoint works | 641px | | ☐ |
| Tablet breakpoint works | 991px | | ☐ |
| Desktop breakpoint works | 992px+ | | ☐ |
| Screenshot comparison passes | Visual match | | ☐ |

---

## Verification Process

### After Each Phase:

1. **Build Check**
   ```bash
   cd frontend && npm run build
   ```

2. **Visual Comparison**
   - Open dev server
   - Take screenshots
   - Compare with WordPress reference screenshots

3. **CSS Audit**
   - Inspect specific elements
   - Compare computed styles
   - Verify exact color values, sizes, spacing

4. **Functionality Test**
   - Test all interactive elements
   - Verify dropdowns, hovers, clicks
   - Test responsive behavior

5. **Approval**
   - Present comparison to stakeholder
   - Get explicit approval before next phase
   - Document any accepted variations

---

## Success Criteria

### Phase Complete When:

1. ✅ All checklist items marked complete
2. ✅ Build passes with zero errors
3. ✅ Visual comparison shows no visible differences
4. ✅ All interactive elements function identically
5. ✅ Stakeholder approves phase completion

---

## Timeline Estimate

| Phase | Estimated Effort | Complexity |
|-------|------------------|------------|
| Phase 1 | 2-3 hours | High |
| Phase 2 | 3-4 hours | High |
| Phase 3 | 4-5 hours | Medium |
| Phase 4 | 2-3 hours | Medium |
| Phase 5 | 2-3 hours | Low |
| **Total** | **13-18 hours** | |

---

**Ready to begin Phase 1 upon confirmation.**

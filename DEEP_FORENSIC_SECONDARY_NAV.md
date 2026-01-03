# DEEP FORENSIC INVESTIGATION - SECONDARY NAV ISSUES
## Badge Mystery + Secondary Nav Positioning
**Date:** January 2, 2026  
**Issue:** "DAILY VIDEO LIVE" badge still appearing + Secondary nav "at the bottom"

---

## ISSUE #1: "DAILY VIDEO LIVE" BADGE

### Search Results:

**Searched entire frontend codebase for:**
- "DAILY VIDEO LIVE" (exact match) - **NOT FOUND**
- "DAILY.*VIDEO.*LIVE" (regex) - **NOT FOUND**
- "daily-video-live" (kebab-case) - **NOT FOUND**
- "Daily Video" - **FOUND ONLY IN:**
  - Article card labels: `{update.type === 'video' ? 'Daily Video' : 'Trading Archive'}`
  - This is for article cards, NOT a badge in header

### Files Checked:
1. ✅ `/routes/dashboard/+layout.svelte` - No badge
2. ✅ `/routes/dashboard/day-trading-room/+page.svelte` - No badge
3. ✅ `/lib/components/dashboard/DashboardBreadcrumbs.svelte` - No badge
4. ✅ `/lib/components/dashboard/DashboardSidebar.svelte` - No badge
5. ✅ All CSS files - No ::before/::after adding badge
6. ✅ Global search - No badge text found

### Conclusion:
**Badge is NOT in current codebase.** Must be from:
1. Browser cache (old HTML)
2. Service worker cache (PWA)
3. Browser extension injecting content
4. Dev server cache (Vite)

---

## ISSUE #2: SECONDARY NAV POSITIONING

### Current Implementation Analysis:

**File:** `/lib/components/dashboard/DashboardSidebar.svelte`

**Line 72:** Secondary nav visibility logic
```typescript
let showSecondaryNav = $derived(secondaryNavItems.length > 0 && collapsed);
```

**PROBLEM:** Secondary nav only shows when `collapsed === true`

**Lines 355-403:** Secondary nav HTML structure
```svelte
{#if showSecondaryNav}
    <nav class="dashboard__nav-secondary" aria-label="{secondarySidebarTitle} navigation">
        <!-- Secondary nav items -->
    </nav>
{/if}
```

**Lines 881-896:** Secondary nav CSS
```css
.dashboard__nav-secondary {
    width: 280px;
    font-size: 14px;
    font-weight: 600;
    background-color: #153e59;
    overflow-y: auto;
    overflow-x: hidden;
    transition: all 0.3s ease-in-out;
    /* Desktop: static positioning (part of flex layout) */
    position: static;
    height: auto;
    min-height: 100vh;
    opacity: 1;
    visibility: visible;
    z-index: auto;
}
```

### WordPress Reference Behavior:

**From `dashboard.8f78208b.css`:**

```css
.dashboard__nav-secondary {
    width: auto;
    font-size: 14px;
    font-weight: 600;
    background-color: #153e59;
    bottom: 50px;
    left: 80px;
    opacity: 0;
    overflow-x: hidden;
    overflow-y: auto;
    position: fixed;
    top: 0;
    transition: all .3s ease-in-out;
    visibility: hidden;
    z-index: 100010;
}

@media (min-width: 1280px) {
    .dashboard__nav-secondary {
        display: block;
        bottom: auto;
        left: auto;
        opacity: 1;
        overflow: visible;
        position: static;
        top: auto;
        width: 280px;
        visibility: visible;
        z-index: auto;
    }
}
```

### CRITICAL FINDINGS:

**WordPress Behavior:**
- **Mobile (<1280px):** `position: fixed`, `left: 80px`, `opacity: 0`, `visibility: hidden`
- **Desktop (≥1280px):** `position: static`, `opacity: 1`, `visibility: visible`

**Current Implementation:**
- **All screens:** `position: static`, `opacity: 1`, `visibility: visible`
- **Missing:** Mobile fixed positioning
- **Missing:** Responsive show/hide behavior

### Why Secondary Nav Appears "At the Bottom":

**Root Cause:** Secondary nav is using `position: static` on ALL screen sizes.

In WordPress:
- Mobile: Fixed position overlay (hidden by default)
- Desktop: Static position (part of flex layout)

In Current Code:
- All sizes: Static position
- If sidebar is NOT collapsed, secondary nav doesn't show at all
- If sidebar IS collapsed, secondary nav shows but has wrong positioning

---

## ISSUE #3: SIDEBAR STRUCTURE

### Current Sidebar HTML Structure:

```svelte
<aside class="dashboard__sidebar" class:is-collapsed={collapsed}>
    <!-- Primary Nav -->
    <nav class="dashboard__nav-primary">
        <!-- Profile, main links, memberships, etc. -->
    </nav>
    
    <!-- Secondary Nav (only shows if collapsed) -->
    {#if showSecondaryNav}
        <nav class="dashboard__nav-secondary">
            <!-- Course-specific nav -->
        </nav>
    {/if}
    
    <!-- Toggle Footer -->
    <footer class="dashboard__toggle">
        <!-- Mobile menu button -->
    </footer>
</aside>
```

### WordPress Structure:

```html
<aside class="dashboard__sidebar">
    <!-- Primary Nav (always visible) -->
    <nav class="dashboard__nav-primary">
        <!-- Main navigation -->
    </nav>
</aside>

<!-- Secondary Nav (separate, outside sidebar) -->
<nav class="dashboard__nav-secondary">
    <!-- Course-specific nav -->
</nav>
```

**CRITICAL DIFFERENCE:**
- **WordPress:** Secondary nav is OUTSIDE the sidebar container
- **Current:** Secondary nav is INSIDE the sidebar container

This causes layout issues because:
1. Secondary nav inherits sidebar's flex/positioning
2. Secondary nav can't position independently
3. Mobile overlay behavior doesn't work correctly

---

## ROOT CAUSE ANALYSIS

### Problem 1: Conditional Rendering
```typescript
let showSecondaryNav = $derived(secondaryNavItems.length > 0 && collapsed);
```

**Issue:** Secondary nav only shows when sidebar is collapsed.

**WordPress:** Secondary nav shows based on route, not collapse state.

### Problem 2: Container Structure
Secondary nav is inside `.dashboard__sidebar` container.

**WordPress:** Secondary nav is sibling to sidebar, not child.

### Problem 3: Missing Mobile CSS
No mobile-specific positioning for secondary nav.

**WordPress:** Has `position: fixed` with `left: 80px` on mobile.

---

## SURGICAL FIXES REQUIRED

### FIX #1: Move Secondary Nav Outside Sidebar Container

**Current Structure:**
```svelte
<aside class="dashboard__sidebar">
    <nav class="dashboard__nav-primary">...</nav>
    {#if showSecondaryNav}
        <nav class="dashboard__nav-secondary">...</nav>
    {/if}
</aside>
```

**Should Be:**
```svelte
<aside class="dashboard__sidebar">
    <nav class="dashboard__nav-primary">...</nav>
</aside>

{#if showSecondaryNav}
    <nav class="dashboard__nav-secondary">...</nav>
{/if}
```

### FIX #2: Fix Visibility Logic

**Current:**
```typescript
let showSecondaryNav = $derived(secondaryNavItems.length > 0 && collapsed);
```

**Should Be:**
```typescript
let showSecondaryNav = $derived(secondaryNavItems.length > 0);
```

Secondary nav should show based on route/content, not collapse state.

### FIX #3: Add Mobile CSS

**Add to `.dashboard__nav-secondary`:**
```css
.dashboard__nav-secondary {
    /* Mobile: fixed overlay */
    position: fixed;
    bottom: 50px;
    left: 80px;
    top: 0;
    opacity: 0;
    visibility: hidden;
    z-index: 100010;
    width: auto;
}

@media (min-width: 1280px) {
    .dashboard__nav-secondary {
        /* Desktop: static in layout */
        position: static;
        bottom: auto;
        left: auto;
        top: auto;
        opacity: 1;
        visibility: visible;
        z-index: auto;
        width: 280px;
    }
}
```

### FIX #4: Update Layout Structure

**File:** `/routes/dashboard/+layout.svelte`

Need to render secondary nav at layout level, not inside sidebar component.

---

## NEXT ACTIONS

1. **Clear browser cache** - Hard refresh (Cmd+Shift+R) to remove badge
2. **Move secondary nav** outside sidebar container
3. **Fix visibility logic** - Remove `&& collapsed` condition
4. **Add mobile CSS** - Fixed positioning for mobile screens
5. **Test responsive behavior** at all breakpoints

---

## VERIFICATION CHECKLIST

- [ ] Badge removed (hard refresh)
- [ ] Secondary nav outside sidebar container
- [ ] Secondary nav shows based on route, not collapse state
- [ ] Mobile: Fixed position overlay
- [ ] Desktop: Static position in layout
- [ ] Smooth transitions between states
- [ ] No layout jumping or positioning issues

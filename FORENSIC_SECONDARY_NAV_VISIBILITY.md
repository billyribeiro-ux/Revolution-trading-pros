# FORENSIC INVESTIGATION - SECONDARY NAV NOT VISIBLE
## Screenshot Analysis + Code Investigation
**Date:** January 2, 2026  
**Issue:** Secondary nav not appearing when on Day Trading Room dashboard

---

## SCREENSHOT ANALYSIS

**What User Sees:**
- Primary sidebar: COLLAPSED (80px width, icons only)
- Secondary nav: NOT VISIBLE (should be showing)
- Background: Dark teal (#0f2d41)
- Only icons visible: Profile, Home, Video, Charts, Bell, Rocket, Settings

**Expected Behavior:**
When on `/dashboard/day-trading-room`, secondary nav should appear with:
- "Day Trading Room Dashboard"
- "Premium Daily Videos"
- "Learning Center"
- "Trading Room Archives"
- "Meet the Traders" (with submenu)
- "Trader Store" (with submenu)

---

## CODE INVESTIGATION

### File 1: `/routes/dashboard/+layout.svelte`

**Lines 170-199:** Secondary nav data defined for Day Trading Room
```typescript
const membershipRoutes = {
    '/dashboard/day-trading-room': {
        title: 'Day Trading Room',
        items: [
            { href: '/dashboard/day-trading-room', icon: 'layout-dashboard', text: 'Day Trading Room Dashboard' },
            { href: '/dashboard/day-trading-room/daily-videos', icon: 'video', text: 'Premium Daily Videos' },
            { href: '/dashboard/day-trading-room/learning-center', icon: 'school', text: 'Learning Center' },
            { href: '/dashboard/day-trading-room/trading-room-archive', icon: 'archive', text: 'Trading Room Archives' },
            // ... Meet the Traders submenu
            // ... Trader Store submenu
        ]
    }
};
```

**Lines 233-240:** Logic to detect membership route
```typescript
let currentMembershipData = $derived.by(() => {
    const currentPath = page?.url?.pathname ?? '';
    for (const [route, data] of Object.entries(membershipRoutes)) {
        if (currentPath.startsWith(route)) {
            return data;
        }
    }
    return null;
});
```

**Lines 266-271:** Passing data to DashboardSidebar
```svelte
<DashboardSidebar
    user={userData}
    bind:collapsed={sidebarCollapsed}
    secondaryNavItems={currentMembershipData?.items ?? []}
    secondarySidebarTitle={currentMembershipData?.title ?? ''}
/>
```

**STATUS:** ✅ Data is being passed correctly

---

### File 2: `/lib/components/dashboard/DashboardSidebar.svelte`

**Line 73:** Visibility logic
```typescript
let showSecondaryNav = $derived(secondaryNavItems.length > 0);
```

**Lines 357-404:** Secondary nav rendering
```svelte
{#if showSecondaryNav}
    <nav class="dashboard__nav-secondary" aria-label="{secondarySidebarTitle} navigation">
        <ul>
            {#each secondaryNavItems as item}
                <!-- Render items -->
            {/each}
        </ul>
    </nav>
{/if}
```

**STATUS:** ✅ Logic looks correct

---

### File 3: DashboardSidebar CSS (Lines 882-916)

**Mobile CSS (Default):**
```css
.dashboard__nav-secondary {
    font-size: 14px;
    font-weight: 600;
    background-color: #153e59;
    overflow-y: auto;
    overflow-x: hidden;
    transition: all 0.3s ease-in-out;
    /* Mobile: fixed overlay (WordPress exact) */
    position: fixed;
    bottom: 50px;
    left: 80px;
    top: 0;
    width: auto;
    opacity: 0;           /* ❌ HIDDEN BY DEFAULT */
    visibility: hidden;   /* ❌ HIDDEN BY DEFAULT */
    z-index: 100010;
}
```

**Desktop CSS (@1280px+):**
```css
@media (min-width: 1280px) {
    .dashboard__nav-secondary {
        display: block;
        position: static;
        bottom: auto;
        left: auto;
        top: auto;
        width: 280px;
        height: auto;
        min-height: 100vh;
        opacity: 1;           /* ✅ VISIBLE ON DESKTOP */
        visibility: visible;  /* ✅ VISIBLE ON DESKTOP */
        z-index: auto;
        overflow: visible;
    }
}
```

---

## ROOT CAUSE IDENTIFIED

**CRITICAL ISSUE:** Secondary nav CSS has:
- Mobile: `opacity: 0` and `visibility: hidden`
- Desktop (≥1280px): `opacity: 1` and `visibility: visible`

**If user's screen is < 1280px wide, secondary nav is HIDDEN even when data is present.**

---

## VERIFICATION NEEDED

1. **What is user's screen width?**
   - If < 1280px: Secondary nav is hidden by CSS
   - If ≥ 1280px: Secondary nav should be visible

2. **Is user on correct route?**
   - Must be on `/dashboard/day-trading-room` or child route
   - Path detection uses `startsWith()` so child routes should work

3. **Are secondaryNavItems being passed?**
   - Check browser console for data
   - Verify `currentMembershipData` is not null

---

## POTENTIAL FIXES

### FIX #1: Lower Desktop Breakpoint
If user has screen < 1280px, lower the breakpoint:

```css
@media (min-width: 1024px) {  /* Was 1280px */
    .dashboard__nav-secondary {
        opacity: 1;
        visibility: visible;
        /* ... rest of desktop styles */
    }
}
```

### FIX #2: Show Secondary Nav on Mobile When Data Present
Add trigger to show secondary nav when items exist:

```css
.dashboard__nav-secondary:has(ul li) {
    opacity: 1;
    visibility: visible;
}
```

### FIX #3: Add Hover/Click Trigger on Mobile
WordPress shows secondary nav on hover/click when collapsed:

```css
.dashboard__sidebar.is-collapsed:hover .dashboard__nav-secondary,
.dashboard__sidebar.is-collapsed:focus-within .dashboard__nav-secondary {
    opacity: 1;
    visibility: visible;
}
```

---

## WORDPRESS REFERENCE BEHAVIOR

**WordPress CSS:**
```css
.dashboard__nav-secondary {
    /* Mobile: Hidden by default */
    opacity: 0;
    visibility: hidden;
    
    /* Shows on hover when sidebar collapsed */
    .dashboard__sidebar.is-collapsed:hover & {
        opacity: 1;
        visibility: visible;
    }
}

@media (min-width: 1280px) {
    .dashboard__nav-secondary {
        /* Desktop: Always visible */
        opacity: 1;
        visibility: visible;
    }
}
```

---

## RECOMMENDED FIX

**Add hover trigger for mobile/tablet screens:**

```css
/* Show secondary nav on hover when sidebar is collapsed */
.dashboard__sidebar.is-collapsed:hover .dashboard__nav-secondary {
    opacity: 1;
    visibility: visible;
}

/* Or always show when data is present on desktop */
@media (min-width: 1280px) {
    .dashboard__nav-secondary {
        opacity: 1 !important;
        visibility: visible !important;
    }
}
```

---

## NEXT STEPS

1. Check user's screen width
2. Add hover trigger for collapsed sidebar
3. Verify secondary nav appears on hover
4. Test at different breakpoints
5. Ensure smooth transition animation

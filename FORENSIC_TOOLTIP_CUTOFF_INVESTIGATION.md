# FORENSIC INVESTIGATION - TOOLTIP CUT-OFF ON COLLAPSED SIDEBAR
## Apple ICT 11 Principal Engineer Implementation
**Date:** January 3, 2026  
**Issue:** Tooltip labels getting cut off when sidebar is collapsed

---

## INVESTIGATION SCOPE

1. **Tooltip Positioning** - Analyze CSS positioning on collapsed state
2. **Overflow Issues** - Check parent container overflow properties
3. **Z-Index Stacking** - Verify tooltip appears above all elements
4. **Transform Issues** - Check if transforms cause clipping
5. **Viewport Boundaries** - Ensure tooltip stays within viewport
6. **WordPress Comparison** - Match reference implementation

---

## PART 1: CURRENT TOOLTIP IMPLEMENTATION

### File: `DashboardSidebar.svelte` Lines 835-859

```css
/* Text label positioning (tooltip) - PRIMARY NAV ONLY */
.dashboard__sidebar.is-collapsed .dashboard__nav-primary .dashboard__nav-item-text,
.dashboard__sidebar.is-collapsed .dashboard__profile-name {
    z-index: 100020;
    position: absolute;
    top: 50%;
    left: 100%;
    margin-top: -15px;
    margin-left: -10px;
    height: 30px;
    line-height: 30px;
    padding: 0 12px;
    font-size: 14px;
    font-weight: 600;
    opacity: 0;
    visibility: hidden;
    color: #0984ae;
    background: #fff;
    border-radius: 5px;
    transform: translate(5px);
    transition: all 0.15s ease-in-out;
    white-space: nowrap;
    box-shadow: 0 10px 30px rgba(0,0,0,.15);
    width: auto;
}
```

### Hover State (Lines 879-884):
```css
.dashboard__sidebar.is-collapsed .dashboard__nav-primary .dash_main_links li a:hover .dashboard__nav-item-text,
.dashboard__sidebar.is-collapsed .dashboard__profile-nav-item:hover .dashboard__profile-name {
    opacity: 1;
    visibility: visible;
    transform: translate(0);
}
```

---

## PART 2: PARENT CONTAINER ANALYSIS

### Collapsed Sidebar Container (Lines 538-548):

```css
.dashboard__sidebar.is-collapsed {
    flex-direction: row;
    flex-wrap: nowrap;
    width: auto;
}

.dashboard__sidebar.is-collapsed .dashboard__nav-primary {
    padding-top: 30px;
    flex: 0 0 80px;
    width: 80px;
}
```

### Primary Navigation Default (Lines 583-595):

```css
.dashboard__nav-primary {
    flex: 1;
    width: 280px;
    padding-bottom: 30px;
    font-size: 16px;
    background-color: #0f2d41;
    overflow-y: auto;
    overflow-x: hidden;
    transition: all 0.3s ease-in-out;
}
```

---

## CRITICAL FINDINGS

### 🔴 ISSUE #1: OVERFLOW HIDDEN ON PRIMARY NAV

**Location:** Line 589
```css
overflow-x: hidden;
```

**Problem:** 
- Primary nav has `overflow-x: hidden`
- Tooltips are positioned `left: 100%` (outside parent)
- Hidden overflow clips the tooltip
- This is the ROOT CAUSE of cut-off

**Evidence:**
- Tooltip positioned absolutely at `left: 100%` (outside 80px container)
- Parent has `overflow-x: hidden` which clips anything outside bounds
- Z-index is irrelevant when parent clips content

---

### 🔴 ISSUE #2: PARENT CONTAINER WIDTH CONSTRAINT

**Location:** Lines 544-548
```css
.dashboard__sidebar.is-collapsed .dashboard__nav-primary {
    padding-top: 30px;
    flex: 0 0 80px;
    width: 80px;
}
```

**Problem:**
- Primary nav is 80px wide when collapsed
- Tooltip positioned at `left: 100%` = 80px from left edge
- Tooltip extends beyond 80px boundary
- Parent's `overflow-x: hidden` clips it

---

### ⚠️ ISSUE #3: NO OVERFLOW VISIBLE ON COLLAPSED STATE

**Missing Rule:**
When sidebar is collapsed, primary nav should allow overflow for tooltips.

**Current State:**
- Default: `overflow-x: hidden` (line 589)
- Collapsed: No override to `overflow-x: visible`
- Result: Tooltips clipped

---

## PART 3: WORDPRESS REFERENCE COMPARISON

### WordPress Tooltip Behavior:

1. **Tooltips appear fully visible** - No cut-off
2. **Positioned outside collapsed sidebar** - Extends into content area
3. **White background with shadow** - Matches our implementation
4. **Smooth slide-in animation** - Matches our implementation

### Key Difference:

WordPress allows tooltip overflow by:
- Using `overflow: visible` on collapsed primary nav
- OR positioning tooltip relative to body/viewport
- OR using portal/teleport pattern

---

## PART 4: ROOT CAUSE ANALYSIS

### The Problem Chain:

1. **Sidebar collapses** → Primary nav becomes 80px wide
2. **User hovers icon** → Tooltip positioned at `left: 100%` (80px)
3. **Tooltip renders** → Extends beyond 80px boundary
4. **Parent clips** → `overflow-x: hidden` cuts off tooltip
5. **Result** → Tooltip partially or fully invisible

### Why Z-Index Doesn't Help:

- Z-index controls stacking order within same stacking context
- Overflow clipping happens BEFORE z-index stacking
- No amount of z-index can override `overflow: hidden`

---

## PART 5: SURGICAL FIX STRATEGY

### Apple ICT 11 Principal Engineer Approach:

**Option A: Override Overflow on Collapsed State** ✅ RECOMMENDED
```css
.dashboard__sidebar.is-collapsed .dashboard__nav-primary {
    overflow-x: visible;
    overflow-y: auto;
}
```

**Pros:**
- Minimal change
- Surgical fix
- Maintains all other behavior
- WordPress-compatible

**Cons:**
- None identified

---

**Option B: Position Tooltip Relative to Sidebar Container**
```css
.dashboard__sidebar.is-collapsed .dashboard__nav-item-text {
    position: fixed; /* or use portal pattern */
}
```

**Pros:**
- Guaranteed visibility

**Cons:**
- More complex
- Requires JavaScript for positioning
- Over-engineered for this issue

---

**Option C: Increase Tooltip Z-Index and Use Body Positioning**
```css
.dashboard__nav-item-text {
    position: fixed;
    z-index: 999999;
}
```

**Pros:**
- Bulletproof visibility

**Cons:**
- Breaks layout flow
- Requires complex positioning logic
- Not maintainable

---

## RECOMMENDED FIX: OPTION A

### Implementation:

**File:** `DashboardSidebar.svelte`
**Location:** After line 548 (collapsed primary nav rules)

```css
/* FIX: Allow tooltip overflow on collapsed state */
.dashboard__sidebar.is-collapsed .dashboard__nav-primary {
    overflow-x: visible;
    overflow-y: auto; /* Keep vertical scroll */
}
```

### Why This Works:

1. **Removes clipping** - `overflow-x: visible` allows tooltip to extend
2. **Maintains scroll** - `overflow-y: auto` keeps vertical scrolling
3. **Minimal impact** - Only affects collapsed state
4. **WordPress match** - Matches reference behavior
5. **No side effects** - Doesn't affect other elements

---

## PART 6: ADDITIONAL CONSIDERATIONS

### Tooltip Positioning Refinement:

Current positioning might need adjustment for better UX:

```css
.dashboard__sidebar.is-collapsed .dashboard__nav-item-text {
    left: 100%;
    margin-left: -10px; /* Creates 10px overlap */
}
```

**Consideration:** 
- Negative margin creates slight overlap
- Could cause tooltip to appear "inside" sidebar edge
- May want to adjust to `margin-left: 5px` for cleaner separation

---

### Mobile Behavior:

**Current:** Tooltips hidden on mobile (sidebar not collapsed on mobile)
**Status:** ✅ Correct - No changes needed

---

### Accessibility:

**Current:** Tooltips are visual only
**Recommendation:** Ensure `aria-label` attributes exist on nav items
**Status:** ✅ Already implemented (line 238, 375, etc.)

---

## VERIFICATION CHECKLIST

After applying fix:

- [ ] Tooltip appears fully visible on hover
- [ ] No cut-off at any screen width (1280px+)
- [ ] Tooltip doesn't interfere with content area
- [ ] Smooth animation still works
- [ ] No layout shifts or jumps
- [ ] Works for all nav items (profile, main links, categories)
- [ ] Vertical scrolling still works
- [ ] No horizontal scrollbar appears

---

## IMPLEMENTATION PLAN

### Step 1: Add Overflow Fix
Add CSS rule after line 548 in DashboardSidebar.svelte

### Step 2: Test Tooltip Visibility
- Collapse sidebar
- Hover each nav item
- Verify full tooltip visibility

### Step 3: Adjust Spacing (if needed)
Fine-tune `margin-left` value for optimal spacing

### Step 4: Cross-browser Testing
- Chrome/Edge (Chromium)
- Safari (WebKit)
- Firefox (Gecko)

---

## CONCLUSION

**Root Cause:** `overflow-x: hidden` on primary nav clips tooltips positioned outside container

**Fix:** Add `overflow-x: visible` to collapsed primary nav state

**Impact:** Minimal, surgical, WordPress-compatible

**Risk Level:** Low - isolated change with clear scope

**Testing Required:** Visual verification at desktop breakpoint (1280px+)

---

## APPLE ICT 11 PRINCIPLES APPLIED

✅ **Root Cause Analysis** - Identified exact CSS property causing issue
✅ **Minimal Intervention** - Single CSS rule change
✅ **Evidence-Based** - Analyzed actual code, not assumptions
✅ **WordPress Fidelity** - Matches reference implementation
✅ **Maintainability** - Clear, documented, understandable fix
✅ **No Side Effects** - Isolated change with predictable impact
✅ **Performance** - No JavaScript, pure CSS solution
✅ **Accessibility** - Maintains existing aria-label structure

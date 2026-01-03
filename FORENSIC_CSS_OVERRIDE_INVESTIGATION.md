# FORENSIC INVESTIGATION - CSS OVERRIDE CONFLICT
## Secondary Nav Visibility Being Overwritten
**Date:** January 3, 2026  
**Issue:** Something is overwriting secondary nav hover visibility

---

## INVESTIGATION FINDINGS

### Search Results: All `.dashboard__nav-secondary` CSS Rules

**File:** `/lib/components/dashboard/DashboardSidebar.svelte`

---

## CRITICAL CONFLICT FOUND

### Rule #1: Lines 518-526 (GLOBAL COLLAPSED RULE)
```css
.dashboard__sidebar.is-collapsed .dashboard__profile-name,
.dashboard__sidebar.is-collapsed .dashboard__nav-item-text,
.dashboard__sidebar.is-collapsed .dashboard__nav-category {
    opacity: 0;
    visibility: hidden;
    width: 0;
    overflow: hidden;
    transition: opacity 0.2s ease, visibility 0.2s ease;
}
```

**PROBLEM:** This rule targets `.dashboard__nav-item-text` which includes:
- Primary nav text (correct)
- **Secondary nav text (WRONG - should be excluded)**

**Specificity:** `.dashboard__sidebar.is-collapsed .dashboard__nav-item-text`
- 2 classes + 1 descendant = **Specificity: 0-2-0**

---

### Rule #2: Lines 880-896 (SECONDARY NAV BASE)
```css
.dashboard__nav-secondary {
    font-size: 14px;
    font-weight: 600;
    background-color: #153e59;
    overflow-y: auto;
    overflow-x: hidden;
    transition: all 0.3s ease-in-out;
    position: fixed;
    bottom: 50px;
    left: 80px;
    top: 0;
    width: auto;
    opacity: 0;           /* Hidden by default */
    visibility: hidden;   /* Hidden by default */
    z-index: 100010;
}
```

**Specificity:** `.dashboard__nav-secondary`
- 1 class = **Specificity: 0-1-0**

---

### Rule #3: Lines 898-903 (HOVER TRIGGER - ADDED BY US)
```css
.dashboard__sidebar.is-collapsed:hover .dashboard__nav-secondary,
.dashboard__sidebar.is-collapsed:focus-within .dashboard__nav-secondary {
    opacity: 1;
    visibility: visible;
}
```

**Specificity:** `.dashboard__sidebar.is-collapsed:hover .dashboard__nav-secondary`
- 2 classes + 1 pseudo-class + 1 descendant = **Specificity: 0-3-0**

**STATUS:** ✅ This rule SHOULD win (higher specificity)

---

### Rule #4: Lines 989-999 (FORCE TEXT VISIBILITY)
```css
.dashboard__nav-secondary .dashboard__nav-item-text,
.dashboard__nav-secondary .dashboard__nav-secondary-item .dashboard__nav-item-text {
    opacity: 1 !important;
    visibility: visible !important;
    width: auto !important;
    overflow: visible !important;
    white-space: nowrap;
    flex: 1;
    color: rgba(255, 255, 255, 0.75);
}
```

**Specificity:** `.dashboard__nav-secondary .dashboard__nav-item-text`
- 2 classes = **Specificity: 0-2-0**

**STATUS:** ✅ Uses `!important` to force visibility

---

## ROOT CAUSE ANALYSIS

### CSS Cascade Order:

1. **Base state:** Secondary nav hidden (`opacity: 0`, `visibility: hidden`)
2. **Hover trigger:** Shows secondary nav (`opacity: 1`, `visibility: visible`) - Specificity 0-3-0
3. **Collapsed text rule:** Hides ALL `.dashboard__nav-item-text` including secondary nav text - Specificity 0-2-0
4. **Force visibility:** Uses `!important` to override - Specificity 0-2-0 + `!important`

### The Conflict:

**When sidebar is collapsed AND user hovers:**

```
Hover trigger shows secondary nav:
✅ .dashboard__nav-secondary { opacity: 1; visibility: visible; }

BUT collapsed rule hides the text inside:
❌ .dashboard__sidebar.is-collapsed .dashboard__nav-item-text { opacity: 0; visibility: hidden; }

Force visibility tries to fix it:
⚠️ .dashboard__nav-secondary .dashboard__nav-item-text { opacity: 1 !important; visibility: visible !important; }
```

**PROBLEM:** The collapsed rule (lines 518-526) is TOO BROAD. It targets ALL `.dashboard__nav-item-text` elements, including those inside `.dashboard__nav-secondary`.

---

## CSS SPECIFICITY BREAKDOWN

### Rule Comparison:

| Rule | Selector | Specificity | Wins? |
|------|----------|-------------|-------|
| Collapsed text hide | `.dashboard__sidebar.is-collapsed .dashboard__nav-item-text` | 0-2-0 | ❌ |
| Secondary nav show | `.dashboard__sidebar.is-collapsed:hover .dashboard__nav-secondary` | 0-3-0 | ✅ |
| Force text visible | `.dashboard__nav-secondary .dashboard__nav-item-text` | 0-2-0 + `!important` | ✅ |

**Issue:** Even though we use `!important`, the parent `.dashboard__nav-secondary` might still be hidden if the hover doesn't work properly.

---

## POTENTIAL ISSUES

### Issue #1: Hover Not Triggering
If hover on `.dashboard__sidebar.is-collapsed` doesn't work:
- Secondary nav stays `opacity: 0`, `visibility: hidden`
- Even with text forced visible, parent is hidden

### Issue #2: Transition Timing
```css
.dashboard__nav-secondary {
    transition: all 0.3s ease-in-out;
}

.dashboard__sidebar.is-collapsed .dashboard__nav-item-text {
    transition: opacity 0.2s ease, visibility 0.2s ease;
}
```

Different transition durations could cause flickering.

### Issue #3: Z-index Stacking
```css
.dashboard__nav-secondary {
    z-index: 100010;  /* Very high */
}
```

Could be behind another element.

---

## SURGICAL FIXES REQUIRED

### FIX #1: Exclude Secondary Nav from Collapsed Text Rule

**BEFORE:**
```css
.dashboard__sidebar.is-collapsed .dashboard__profile-name,
.dashboard__sidebar.is-collapsed .dashboard__nav-item-text,
.dashboard__sidebar.is-collapsed .dashboard__nav-category {
    opacity: 0;
    visibility: hidden;
    width: 0;
    overflow: hidden;
}
```

**AFTER:**
```css
.dashboard__sidebar.is-collapsed .dashboard__profile-name,
.dashboard__sidebar.is-collapsed .dashboard__nav-primary .dashboard__nav-item-text,
.dashboard__sidebar.is-collapsed .dashboard__nav-category {
    opacity: 0;
    visibility: hidden;
    width: 0;
    overflow: hidden;
}
```

**Change:** Add `.dashboard__nav-primary` to scope the rule to ONLY primary nav text.

---

### FIX #2: Strengthen Hover Trigger Specificity

**BEFORE:**
```css
.dashboard__sidebar.is-collapsed:hover .dashboard__nav-secondary,
.dashboard__sidebar.is-collapsed:focus-within .dashboard__nav-secondary {
    opacity: 1;
    visibility: visible;
}
```

**AFTER:**
```css
.dashboard__sidebar.is-collapsed:hover .dashboard__nav-secondary,
.dashboard__sidebar.is-collapsed:focus-within .dashboard__nav-secondary {
    opacity: 1 !important;
    visibility: visible !important;
}
```

**Change:** Add `!important` to ensure it overrides any conflicting rules.

---

### FIX #3: Remove Force Visibility (No Longer Needed)

**Lines 989-999:** Can be removed if we properly scope the collapsed text rule.

```css
/* DELETE THIS - No longer needed */
.dashboard__nav-secondary .dashboard__nav-item-text,
.dashboard__nav-secondary .dashboard__nav-secondary-item .dashboard__nav-item-text {
    opacity: 1 !important;
    visibility: visible !important;
    width: auto !important;
    overflow: visible !important;
}
```

---

## RECOMMENDED SOLUTION

**Apply FIX #1 only** - This is the cleanest solution:

1. Scope collapsed text rule to `.dashboard__nav-primary` only
2. Secondary nav text will no longer be affected by collapsed state
3. Hover trigger will work as expected
4. No need for `!important` hacks

---

## VERIFICATION STEPS

After applying fix:

1. Inspect element in browser DevTools
2. Check computed styles for `.dashboard__nav-secondary`
3. Verify no conflicting `opacity: 0` or `visibility: hidden`
4. Test hover behavior on collapsed sidebar
5. Confirm secondary nav slides in smoothly

---

## NEXT ACTIONS

1. Apply FIX #1 - Scope collapsed text rule to primary nav only
2. Test hover behavior
3. Remove force visibility rule if no longer needed
4. Verify at all breakpoints
5. Check for any other CSS conflicts

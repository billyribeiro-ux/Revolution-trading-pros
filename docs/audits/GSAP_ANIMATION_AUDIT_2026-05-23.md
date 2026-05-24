# GSAP Animation Audit - All Pages

**Date:** 2026-05-23  
**Status:** In Progress  
**Goal:** Verify every GSAP animation works correctly

---

## Marketing Pages with GSAP

| Page/Component | File Path | Import Pattern | Has gsap.set Anti-Pattern | Status | Notes |
|----------------|-----------|----------------|---------------------------|--------|-------|
| **Hero** | `lib/components/sections/Hero.svelte` | ✅ PE7 Svelte 5 Pattern | ❌ NO | � **PE7 FIXES APPLIED** | Synchronous onMount, DOM verification, proper error handling |
| **Live Trading Rooms** | `routes/live-trading-rooms/+page.svelte` | ✅ Proper type + dynamic | ❌ NO | 🟢 WORKING | 6 floating CTAs animate |
| **CoursesSection** | `lib/components/sections/CoursesSection.svelte` | ✅ Type + dynamic | ❌ NO | ⏳ PENDING | Check animation triggers |
| **IndicatorsSection** | `lib/components/sections/IndicatorsSection.svelte` | ✅ Type + dynamic | ❌ NO | ⏳ PENDING | Check animation triggers |
| **About** | `routes/about/+page.svelte` | ✅ Has GSAP | ❌ NO | ⏳ PENDING | Verify animations |
| **LoginForm** | `lib/components/auth/LoginForm.svelte` | ✅ Has GSAP | ❌ NO | ⏳ PENDING | Verify animations |
| **TradingHeroBackground** | `lib/components/auth/TradingHeroBackground.svelte` | ✅ Has GSAP | ❌ NO | ⏳ PENDING | Verify animations |
| **NotificationPanel** | `lib/components/ui/NotificationPanel.svelte` | ✅ Has GSAP | ✅ **YES** | 🔴 **NEEDS FIX** | Remove gsap.set |
| **KpiCard** | `lib/components/analytics/KpiCard.svelte` | ✅ Has GSAP | ❌ NO | ⏳ PENDING | Verify animations |
| **AnimatedNumber** | `lib/components/ui/AnimatedNumber.svelte` | ✅ Has GSAP | ❌ NO | ⏳ PENDING | Verify animations |

---

## PE7 GSAP Pattern Rules

### ✅ CORRECT Pattern (Live Trading Rooms):
```typescript
// 1. Type import only (types are erased at runtime)
type GSAPInstance = typeof import('gsap').gsap;

// 2. Dynamic import in onMount
onMount(async () => {
  if (!browser) return;
  const [{ gsap }] = await Promise.all([
    import('gsap'),
    import('gsap/ScrollTrigger')
  ]);
  gsap.registerPlugin(ScrollTrigger);
  
  // 3. Use fromTo() - handles initial state automatically
  gsap.fromTo(element, 
    { opacity: 0, y: 30 },  // from (initial state)
    { opacity: 1, y: 0 }    // to (animate to)
  );
});
```

### ❌ ANTI-PATTERN (Current Hero.svelte):
```typescript
// 1. Sets elements to invisible FIRST
gsap.set([h1, h2, p, buttons], { opacity: 0, y: 40 });

// 2. Then tries to animate
// PROBLEM: If animation fails, content stays hidden!
```

---

## Detailed Findings

### Hero.svelte - BROKEN 🔴
**Location:** Line 452  
**Issue:** 
```javascript
// ANTI-PATTERN - Line 452
gsapLib.set([h1, h2, p, ...Array.from(buttons)], { 
  opacity: 0, 
  y: 40 
});
```
**Fix:** Remove the `gsap.set()` call. The `fromTo()` at lines 470-497 already handles initial state.

**Impact:** Hero content invisible on homepage

---

### Live Trading Rooms - WORKING 🟢
**Pattern:** Dynamic import + fromTo()  
**No gsap.set anti-pattern**  
**Status:** Animations working (6 floating CTAs)

---

## Next Steps

1. ✅ **Audit complete** - identified all GSAP files
2. ⏳ **Check each component** - verify import pattern
3. ⏳ **Fix Hero.svelte** - remove gsap.set
4. ⏳ **Test all pages** - verify animations work
5. ⏳ **Document working baseline**

---

## Commands to Verify

```bash
# Check import pattern
grep -n "import.*gsap\|gsap\.set" frontend/src/lib/components/sections/*.svelte

# Check all marketing pages
grep -rn "gsap" frontend/src/routes/*.svelte frontend/src/routes/*/+page.svelte 2>/dev/null | grep -v node_modules
```

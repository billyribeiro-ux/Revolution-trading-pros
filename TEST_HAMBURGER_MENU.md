# Hamburger Menu Testing Guide

## Quick Test Steps

### 1. Open Browser Console
- Press `F12` or `Cmd+Option+I` (Mac) / `Ctrl+Shift+I` (Windows)
- Go to Console tab

### 2. Test Basic Toggle
1. Click the hamburger menu button (☰)
2. **Expected console output:**
   ```
   [NavBar] Toggling menu: { from: false, to: true }
   [NavBar] Animation complete, state: true
   ```
3. Menu should slide in from the right
4. Click hamburger again to close
5. **Expected console output:**
   ```
   [NavBar] Toggling menu: { from: true, to: false }
   [NavBar] Animation complete, state: false
   ```

### 3. Test Rapid Clicking (Debounce)
1. Click hamburger button 5 times rapidly
2. **Expected console output:**
   ```
   [NavBar] Toggling menu: { from: false, to: true }
   [NavBar] Toggle debounced
   [NavBar] Toggle debounced
   [NavBar] Toggle debounced
   [NavBar] Animation in progress, ignoring toggle
   ```
3. Menu should only toggle once, not flicker

### 4. Test Click Outside
1. Open hamburger menu
2. Click anywhere outside the menu (on the backdrop or page)
3. **Expected:** Menu closes smoothly
4. **Console:** `[NavBar] Closing mobile menu`

### 5. Test Escape Key
1. Open hamburger menu
2. Press `Escape` key
3. **Expected:** Menu closes
4. **Console:** `[NavBar] Closing mobile menu`

### 6. Test Window Resize
1. Open hamburger menu
2. Resize browser window to desktop width (>1024px)
3. **Expected:** Menu automatically closes
4. **Console:** `[NavBar] Closing mobile menu`

### 7. Check Health Monitor
1. Wait 5 seconds with menu open
2. **Expected console output:**
   ```
   [NavBar] Component mounted successfully
   ```
3. No error or warning messages should appear

## What to Look For

### ✅ Success Indicators
- Menu opens/closes smoothly
- No JavaScript errors in console
- Rapid clicks are debounced
- Click outside closes menu
- Escape key works
- Console shows `[NavBar]` debug messages

### ❌ Failure Indicators
- Menu doesn't open/close
- Console shows errors
- Menu flickers on rapid clicks
- Menu gets stuck open/closed
- Console shows recovery attempts:
  ```
  [NavBar] Attempting state recovery (1/3)
  [NavBar] Animation lock stuck, recovering...
  ```

## Advanced Testing

### Test Error Recovery
1. Open browser console
2. Run this to simulate an error:
   ```javascript
   // This will be caught and recovered
   document.querySelector('.nav-hamburger').click();
   ```
3. Menu should still work after any errors

### Test Health Check
1. Open console
2. Wait 5 seconds
3. Look for health check validation (runs silently unless issues found)

### Disable Debug Logging
If console is too noisy, you can filter:
1. In Console, type: `-[NavBar]` in the filter box
2. This hides all NavBar debug messages

## Browser Compatibility

Test in:
- ✅ Chrome/Edge (Chromium)
- ✅ Firefox
- ✅ Safari
- ✅ Mobile Safari (iOS)
- ✅ Chrome Mobile (Android)

## Performance Check

1. Open Performance tab in DevTools
2. Click hamburger menu
3. **Expected:** Animation completes in ~250ms
4. No layout thrashing or reflows

## Troubleshooting

### Menu Not Opening
1. Check console for errors
2. Verify backend is running: `http://localhost:8000`
3. Hard refresh: `Cmd+Shift+R` (Mac) / `Ctrl+Shift+R` (Windows)

### Menu Stuck Open
1. Press `Escape` key
2. Click outside menu
3. Refresh page if needed
4. Check console for recovery messages

### No Console Messages
1. Verify you're on `http://localhost:5174`
2. Hard refresh the page
3. Check if dev server is running

## Expected Behavior Summary

| Action | Expected Result | Console Output |
|--------|----------------|----------------|
| Click hamburger | Menu opens | `Toggling menu: { from: false, to: true }` |
| Click again | Menu closes | `Toggling menu: { from: true, to: false }` |
| Rapid clicks | Only 1 toggle | `Toggle debounced` (multiple) |
| Click outside | Menu closes | `Closing mobile menu` |
| Press Escape | Menu closes | `Closing mobile menu` |
| Resize to desktop | Menu closes | `Closing mobile menu` |
| Wait 5 seconds | Health check runs | (silent unless issues) |

## Success Criteria

✅ All 7 basic tests pass  
✅ No JavaScript errors in console  
✅ Smooth animations (no jank)  
✅ Works on mobile viewport  
✅ Works on tablet viewport  
✅ Debouncing prevents rapid-fire issues  
✅ Recovery system never triggers (no errors to recover from)  

---

**If all tests pass, the hamburger menu is working correctly!** 🎉

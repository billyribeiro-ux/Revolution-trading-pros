# VERIFY HERO COMPONENT - DO THIS NOW

## Server Running: http://localhost:5174

---

## STEP 1: Open Browser

1. Open Chrome or Firefox
2. Navigate to: **http://localhost:5174**
3. Press **F12** to open DevTools
4. Click **Console** tab

---

## STEP 2: Check Console Logs

You MUST see these logs in order:

```
🚀 [MOUNT] Component mounted
🚀 [MOUNT] browser: true
📦 [MOUNT] Loading GSAP...
✅ [MOUNT] GSAP loaded: true
🚀 [MOUNT] After timeout - chartContainer: <div class="hero-chart...">
🚀 [MOUNT] After timeout - heroSection: <section id="hero...">
🔧 [INIT] Starting initChart
🔧 [INIT] browser: true
🔧 [INIT] chartContainer: <div class="hero-chart...">
🔧 [INIT] heroSection: <section id="hero...">
🔧 [INIT] Dimensions: [NUMBER] x [NUMBER]
✅ [INIT] Chart created: {...}
✅ [INIT] Series added: {...}
📊 [INIT] Candles generated: 120
📊 [INIT] First candle: {time: ..., open: ..., high: ..., low: ..., close: ...}
✅ [INIT] Initial data set on series
✅ [INIT] Visible range set
✅ [INIT] Replay started
🎬 [REPLAY] Starting replay
🎬 [REPLAY] browser: true series: true candles.length: 120
✅ [REPLAY] Interval started
🎬 [ANIM] Animating slide 0
🎬 [ANIM] browser: true gsap: true
🎬 [ANIM] Slide element: <div data-slide="0"...>
✅ [MOUNT] Slideshow started
✅ [MOUNT] Initialization complete
```

### IF YOU SEE ERRORS:

**❌ chartContainer STILL undefined**

- This means bind:this is not working
- Solution: Increase timeout or use different approach

**❌ Missing required elements**

- Chart won't initialize
- Check if elements exist in DOM

---

## STEP 3: Visual Verification

### CHART TEST:

1. Look at the hero section background
2. You should see **subtle cyan/indigo candlestick bars**
3. They should be **animating** (appearing one by one)
4. After ~8 seconds, they should **loop** (restart from beginning)
5. Opacity is 30%, so they're subtle

### To verify chart canvas exists:

Run in console:

```javascript
const canvas = document.querySelector('.hero-chart canvas');
console.log('Canvas exists:', !!canvas);
console.log('Canvas size:', canvas?.width, 'x', canvas?.height);
```

Expected: Canvas exists: true, Canvas size: [width] x [height]

---

### ANIMATION TEST:

1. **Slide 0** should be visible on page load
2. Text should **fade in with scale effect** (title grows slightly)
3. Wait **5 seconds**
4. Slide should **auto-advance** to Slide 1
5. Slide 1 should have **bounce animation** on subtitle
6. Continue watching - all 4 slides should cycle

### To verify animations:

Run in console:

```javascript
// Check current slide
const visible = document.querySelector('[data-slide]:not(.hidden)');
console.log('Current slide:', visible?.getAttribute('data-slide'));

// Wait 5 seconds and check again
setTimeout(() => {
	const newVisible = document.querySelector('[data-slide]:not(.hidden)');
	console.log('Slide after 5s:', newVisible?.getAttribute('data-slide'));
}, 5000);
```

Expected: Slide changes from 0 → 1 after 5 seconds

---

## STEP 4: If Still Not Working

### Debug Chart:

```javascript
// Check if chart container has dimensions
const container = document.querySelector('.hero-chart');
console.log('Container:', container);
console.log('Container dimensions:', container?.offsetWidth, 'x', container?.offsetHeight);
console.log('Container parent:', container?.parentElement);
console.log(
	'Parent dimensions:',
	container?.parentElement?.offsetWidth,
	'x',
	container?.parentElement?.offsetHeight
);
```

If dimensions are 0, the chart can't render.

### Debug Animations:

```javascript
// Check if GSAP is loaded
console.log('GSAP available:', typeof window.gsap !== 'undefined');

// Check if slides exist
const slides = document.querySelectorAll('[data-slide]');
console.log('Total slides:', slides.length);
slides.forEach((s, i) => {
	console.log(`Slide ${i}:`, s, 'hidden:', s.classList.contains('hidden'));
});
```

---

## STEP 5: Report Results

### If WORKING:

✅ Chart visible and animating
✅ Slides visible and transitioning
✅ No console errors

### If NOT WORKING:

Copy ALL console output and report:

1. Which logs appear
2. Which logs are missing
3. Any error messages
4. Results of debug commands above

---

## Quick Test Commands

Paste this into console for instant verification:

```javascript
console.clear();
console.log('=== HERO COMPONENT TEST ===\n');

// Chart test
const chartContainer = document.querySelector('.hero-chart');
const canvas = chartContainer?.querySelector('canvas');
console.log('✅ Chart container:', !!chartContainer);
console.log('✅ Canvas element:', !!canvas);
console.log('✅ Canvas dimensions:', canvas?.width, 'x', canvas?.height);

// Animation test
const slides = document.querySelectorAll('[data-slide]');
const visible = document.querySelector('[data-slide]:not(.hidden)');
console.log('\n✅ Total slides:', slides.length);
console.log('✅ Visible slide:', visible?.getAttribute('data-slide'));

// GSAP test
console.log('\n✅ GSAP loaded:', typeof gsap !== 'undefined');

// Wait and check slide change
setTimeout(() => {
	const newVisible = document.querySelector('[data-slide]:not(.hidden)');
	console.log('\n✅ Slide after 5s:', newVisible?.getAttribute('data-slide'));
	console.log(
		newVisible?.getAttribute('data-slide') !== visible?.getAttribute('data-slide')
			? '✅ SLIDES ARE TRANSITIONING!'
			: '❌ SLIDES NOT CHANGING'
	);
}, 5000);

console.log('\n⏳ Waiting 5 seconds to test slide transition...');
```

---

## EXPECTED FINAL STATE

- ✅ Chart canvas visible in DOM
- ✅ Chart dimensions > 0
- ✅ Chart animating (candlesticks appearing)
- ✅ Slide 0 visible on load
- ✅ Slides auto-advance every 5 seconds
- ✅ Each slide has unique animation
- ✅ No console errors
- ✅ No 404 network errors

**IF ALL CHECKS PASS: COMPONENT IS WORKING!**

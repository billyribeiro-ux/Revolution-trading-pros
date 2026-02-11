# Development Debugging Tools

**Apple ICT 7 Principal Engineer Grade - January 2026**

This document outlines the debugging and inspection tools available in the development environment.

---

## 🔍 Installed Tools

### 1. **svelte-render-scan** (v1.1.0)

Visual debugging tool that highlights component re-renders in real-time.

**Features:**

- Highlights components when they re-render
- Shows render count and timing
- Helps identify performance bottlenecks
- Automatic in development mode

**Usage:**
Already configured in `vite.config.js`. Just run the dev server:

```bash
npm run dev
```

Components will show visual overlays when they re-render. Look for colored borders and render counts.

**Configuration:**
Located in `frontend/vite.config.js`:

```js
import { renderScan } from 'svelte-render-scan/vite';

plugins: [
	renderScan() // Visual debugging tool
	// ...
];
```

---

### 2. **svelte-inspect-value** (v0.10.0)

Interactive value inspector component for debugging reactive state.

**Features:**

- Inspect any JavaScript value
- Collapsible object/array trees
- Real-time updates as values change
- Type information display

**Usage:**
Use the wrapper component in your Svelte files:

```svelte
<script>
	import InspectValue from '$lib/components/dev/InspectValue.svelte';

	let myState = $state({ count: 0, user: { name: 'John' } });
</script>

<InspectValue value={myState} label="My State" />
<InspectValue value={myState.user} label="User Object" collapsed={true} />
```

**Component Location:**
`frontend/src/lib/components/dev/InspectValue.svelte`

**Props:**

- `value: any` - The value to inspect (required)
- `label?: string` - Display label (default: "Value")
- `collapsed?: boolean` - Start collapsed (default: false)

**Important:** Only renders in development mode (`dev` environment).

---

### 3. **vite-plugin-devtools-json**

Chrome DevTools workspace integration for in-browser editing.

**Features:**

- Edit source files directly in Chrome DevTools
- Changes sync back to filesystem
- Seamless development workflow

**Usage:**
Automatic. Just use Chrome/Edge DevTools as normal.

**Endpoint:**
`http://localhost:5174/.well-known/appspecific/com.chrome.devtools.json`

---

## 🎯 Best Practices

### Performance Debugging with render-scan

1. **Identify unnecessary re-renders:**
   - Look for components flashing frequently
   - Check if parent re-renders trigger child re-renders unnecessarily

2. **Optimize with Svelte 5 patterns:**

   ```svelte
   // ✅ Good: Derived state prevents re-renders const filtered = $derived(items.filter(i =>
   i.active)); // ❌ Bad: Function recreated on every render const filtered = () => items.filter(i
   => i.active);
   ```

3. **Use $derived.by() for expensive computations:**
   ```svelte
   const expensive = $derived.by(() => {
     // Complex calculation
     return heavyComputation(data);
   });
   ```

### State Inspection with inspect-value

1. **Debug reactive state issues:**

   ```svelte
   <InspectValue value={ps.alerts} label="Alerts State" />
   ```

2. **Track derived state:**

   ```svelte
   <InspectValue value={filteredAlerts} label="Filtered Alerts" />
   ```

3. **Monitor props:**
   ```svelte
   <InspectValue value={$props()} label="Component Props" />
   ```

---

## 🚫 Production Safety

All debugging tools are **automatically disabled in production**:

- `renderScan()` only runs in dev mode
- `InspectValue` checks `dev` environment
- No performance impact on production builds

---

## 📚 Documentation

- **svelte-render-scan:** https://github.com/aidenybai/svelte-render-scan
- **svelte-inspect-value:** https://github.com/Rich-Harris/svelte-inspect-value
- **Chrome DevTools Workspaces:** https://developer.chrome.com/docs/devtools/workspaces/

---

## 🔧 Troubleshooting

### render-scan not showing overlays

- Ensure dev server is running
- Check browser console for errors
- Verify plugin is in `vite.config.js`

### InspectValue not rendering

- Check `dev` environment is true
- Verify import path is correct
- Ensure component is in dev mode

### DevTools workspace not working

- Visit the JSON endpoint to verify it's served
- Check Chrome flags: `chrome://flags`
- Enable "DevTools Project Settings"

---

**Last Updated:** January 31, 2026  
**Maintained By:** Development Team

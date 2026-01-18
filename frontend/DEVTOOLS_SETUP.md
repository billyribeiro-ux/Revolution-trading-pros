# Chrome DevTools Workspaces Setup

## What This Does

Enables **Chrome DevTools Workspaces** - edit your `.svelte` files directly in the browser and save changes to disk.

## Quick Start

1. **Start dev server:**
   ```bash
   npm run dev
   ```

2. **Open Chrome DevTools** (F12 or Cmd+Option+I)

3. **Enable Workspaces:**
   - DevTools → Settings (⚙️ icon) → Workspace tab
   - Your project folder should auto-appear
   - Click "Add folder" if it doesn't
   - Grant file system access when prompted

4. **Edit files:**
   - Sources panel → Page → Navigate to your `.svelte` files
   - Edit directly in DevTools
   - Ctrl+S (Cmd+S) to save
   - Changes persist to disk + HMR updates instantly

## Perfect For

- Debugging Svelte 5 runes (`$state`, `$derived`, `$effect`)
- Setting breakpoints in reactive code
- Quick CSS/layout tweaks
- Inspecting component state in real-time

## Example Workflow

```svelte
<!-- Edit this directly in Chrome DevTools -->
let filteredConnections = $derived.by(() => {
  // Set breakpoint here ← Click line number in DevTools
  let result = externalServices;
  
  if (selectedCategory) {
    result = result.filter((c) => c.category === selectedCategory);
  }
  
  return result;
});
```

1. See bug in browser
2. Open DevTools → Sources → Find the `.svelte` file
3. Set breakpoint in `$derived.by()`
4. Trigger the code
5. Inspect variables
6. Fix directly in DevTools
7. Save → HMR updates → Test

## Security Note

- **Dev only** - not active in production
- Browser can read/write files in project directory
- All team members using Chrome get this feature
- Chrome AI may analyze code (sends to Google)

## Disable Per-Developer

If you don't want this feature:

1. Visit `chrome://flags`
2. Search "DevTools Project Settings"
3. Set to "Disabled"
4. Restart Chrome

## Technical Details

- Plugin: `vite-plugin-devtools-json`
- Serves config at: `/.well-known/appspecific/com.chrome.devtools.json`
- Only active during `npm run dev`
- Works with Chrome, Edge, Brave (Chromium browsers)

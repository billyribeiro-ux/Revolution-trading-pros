# svelte-click-to-source

**Alt+Click any element in dev mode → Opens source in your editor**

Zero-config developer tool for SvelteKit / Svelte 5 projects.

## Installation

```bash
npm install -D svelte-click-to-source
```

## Setup

### 1. Add the Vite plugin

```javascript
// vite.config.js
import { clickToSource } from 'svelte-click-to-source';

export default defineConfig({
  plugins: [
    clickToSource({ editor: 'windsurf' }),
    sveltekit()
  ]
});
```

### 2. Add the preprocessor

```javascript
// svelte.config.js
import { createPreprocessor } from 'svelte-click-to-source';

export default {
  preprocess: [
    createPreprocessor(),
    vitePreprocess()
  ]
};
```

### 3. Use it

1. `npm run dev`
2. Hold **Alt** + hover (elements highlight)
3. **Alt+Click** → editor opens at source line

## Options

```javascript
clickToSource({
  hotkey: 'alt',           // 'alt' | 'ctrl' | 'meta' | 'shift'
  editor: 'windsurf',      // 'vscode' | 'cursor' | 'windsurf' | 'webstorm' | 'vim' | 'neovim'
  highlight: true,
  highlightColor: '#3b82f6',
  enabled: true
})
```

## License

MIT

# svelte-click-to-source

Development-only Vite plugin that opens the source of an Alt-clicked Svelte element in an editor.

## Install

```sh
pnpm add -D svelte-click-to-source
```

Add the plugin to `vite.config.ts`:

```ts
import { defineConfig } from 'vite';
import { sveltekit } from '@sveltejs/kit/vite';
import { clickToSource } from 'svelte-click-to-source';

export default defineConfig({
	plugins: [clickToSource(), sveltekit()]
});
```

Add the preprocessor to `svelte.config.js` (alongside any existing preprocessors):

```js
import { createPreprocessor } from 'svelte-click-to-source';
import adapter from '@sveltejs/adapter-auto';

export default {
	preprocess: [createPreprocessor({ root: process.cwd() })],
	kit: { adapter: adapter() }
};
```

`root` must be the same directory as Vite's project root. If Vite uses a custom root, pass that path to `createPreprocessor`. Run the development server, hold Alt, hover over an element, then click it. The tool is disabled during production builds. `editor` accepts `vscode`, `cursor`, `windsurf`, `webstorm`, `vim`, or `neovim`; GUI editors are recommended because terminal editors may exit without an attached terminal. Use `editorCommand` to provide the executable path for another GUI editor. Change the hotkey or highlight color with `clickToSource({ hotkey: 'meta', highlightColor: '#2563eb' })`.

## Security boundary

The editor endpoint accepts only POST requests bearing a random per-server token. It defaults to loopback clients and rejects paths outside the project root, symlinks that escape it, non-Svelte files, and invalid line/column values. The editor starts with argument-based process spawning, never a shell command. Do not expose a development server to untrusted users. `allowRemote: true` is available only for a trusted remote development environment; anyone who can fetch the dev module can obtain its request token.

## Verify

```sh
pnpm run check
pnpm run test
pnpm run build
```

The package supports Svelte 4 or 5 and Vite 5, 6, or 8 according to its declared peer ranges. The automated tests currently exercise the installed Svelte 5/Vite 8 toolchain; the other peer combinations need a separate compatibility matrix before publication.

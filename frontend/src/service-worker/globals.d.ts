// Injected by Vite's `define` (see frontend/vite.config.ts).
//
// The service worker cannot read `version` from `$app/env` — that export
// resolves to `payload.version` in a browser-like scope, and `payload` is only
// populated by the client app boot, never in a ServiceWorkerGlobalScope. This
// global carries the same string as `kit.version.name` so cache names change on
// every deploy.
//
// No imports/exports in this file, so it is an ambient script declaration
// rather than a module — which is what makes the name global.
declare const __APP_VERSION__: string;

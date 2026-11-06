# Marketing pages independent audit

Scope: `/about` and `/alerts/explosive-swings`, including shared marketing shell and navigation. No deployment or production mutations authorized.

1. Establish source revision, installed versions, deployment drift, existing worktree changes, and available tools.
2. Render deployed pages in Chromium. Capture bounded readiness, console/network evidence, screenshots, DOM geometry, direct/reload/internal/history navigation, controls, narrow/intermediate/wide and breakpoint widths, short viewport, larger text, keyboard and reduced motion.
3. Trace reproducible findings to source/contracts. Separate observations and unresolved hypotheses. Add meaningful regression cases and execute them before repairs.
4. Repair confirmed causes; use official Svelte documentation and autofixer. Preserve established design.
5. Execute required gates and repeat original reproductions against actual production build. Document limitations, source/deployment differences, findings, evidence, and operational implications. Do not push, merge, or deploy.

Initial environment: clean worktree at `5fbdb06c6e15501280df4b95e2c3e855f743048f`; Node 26.13.0, pnpm 12.16.1; installed Svelte 5.57.3, Kit 3.1.0, Playwright 1.64.1. Kit 3 configuration lives in `frontend/vite.config.ts`, using Cloudflare adapter. Chromium startup requires execution outside filesystem sandbox (macOS Mach port restriction); approved tool escalation is used for rendering.

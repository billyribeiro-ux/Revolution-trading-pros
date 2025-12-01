# Revolution Engineering - Single Source of Truth
# Revolution Trading Pros – Engineering SSOT (Single Source of Truth)

This document is the **governing spec** for all AI agents and human engineers working on the Revolution Trading Pros Svelte/SvelteKit platform.

> If code, designs, or prompts conflict with this document, **this document wins.**

---

## 1. Project Overview & Stack

### 1.1 Core Framework

- **SvelteKit 5.x** – Full-stack framework (SSR/SSG, routing, endpoints)
- **Svelte 5.x** – UI framework (runes API where appropriate)
- **Vite 7.x** – Build tool & dev server
- **TypeScript 5.9+** – Type safety everywhere

### 1.2 Styling

- **Tailwind CSS 4.1.x** – Utility-first CSS
- **PostCSS / Autoprefixer** – Browser compatibility
- Custom tokens for brand:
  - `bg-rtp-bg`, `bg-rtp-surface`, `bg-rtp-elevated`
  - `text-rtp-text`, `text-rtp-muted`, `text-rtp-accent`
  - `border-rtp-subtle`, `border-rtp-strong`
  - Primary accent utilities: `bg-rtp-primary`, `text-rtp-primary`, `ring-rtp-primary`

### 1.3 Data Viz & Animation

- **Charts:** Lightweight Charts, ApexCharts, Chart.js, D3
- **Animation:** GSAP, Anime.js, Svelte transitions / motion
- **Icons:** Tabler Icons (primary), Font Awesome only where needed

---

## 2. Layout & Breakpoints

We follow **desktop-first, responsive down** with clean breakpoint tiers.

### 2.1 Breakpoint Tiers

- **XL Desktop:** ≥ 1440px
- **Desktop:** 1280–1439px
- **Laptop:** 1024–1279px
- **Tablet:** 768–1023px
- **Mobile:** < 768px

AI agents MUST:

- Avoid layout shifts (CLS ≈ 0).
- Ensure no overlapping nav/menu elements at any width.
- Test at: 1440, 1280, 1024, 834, 768, 414, 375.

### 2.2 Navbar Rules (Global Standard)

- **Logo is static and never moves.**
  - Reserved fixed area on the left (e.g. 220–260px).
  - No flex shrink on logo container.
- **Nav height:**
  - Desktop: 72–80px
  - Tablet: 64–72px
  - Mobile: 56–64px
- **Primary CTA (e.g. “Login” / “Get Started”):**
  - Right-aligned, vertically centered.
  - Always visible on desktop.
  - Moves into mobile menu on small screens if needed, but must exist.

Behavior:

- Desktop:
  - Menu items inline.
  - Dropdowns open on **click**, not hover.
  - Dropdowns are edge-aware (do not overflow viewport).
- Mobile:
  - Full-height slide-in or modal panel.
  - Focus trap inside panel.
  - Close via X button and `Escape`.
- Accessibility:
  - All interactive elements reachable by `Tab`.
  - Dropdowns navigable with arrow keys where applicable.
  - Visible focus outlines.

---

## 3. Typography & Spacing

### 3.1 Fonts

- **Headings:** Montserrat (or equivalent geometric sans)
- **Body:** A clean sans (e.g. Open Sans / Inter), consistent across app.

### 3.2 Type Scale (example baseline)

- H1: `text-4xl` / `text-5xl` desktop, `text-3xl` mobile
- H2: `text-3xl` / `text-4xl` desktop
- H3: `text-2xl`
- Body: `text-base` (default), `text-sm` for meta labels

Rules:

- No more than **3 heading sizes** per page to keep hierarchy clear.
- Line height:
  - Headings: `leading-tight` or `leading-snug`
  - Body: `leading-relaxed`

### 3.3 Spacing System

- Base unit: **4px**
- Common blocks:
  - Section padding: `py-16` desktop, `py-12` tablet, `py-8` mobile
  - Card padding: `p-6` desktop, `p-4` mobile
- Use Tailwind spacing tokens consistently; no arbitrary inlined `style="margin: 13px"`.

---

## 4. Svelte/SvelteKit Architecture Rules

### 4.1 Routing & Files

- Use **file-based routing** under `/src/routes`.
- Prefer standard SvelteKit structure:
  - `+layout.svelte` / `+layout.ts` or `.server.ts`
  - `+page.svelte` / `+page.ts` or `.server.ts`
  - API endpoints under `+server.ts`

Rules:

- Shared layout (nav, footer, global providers) should live in `+layout.svelte` unless there is a **strong** reason not to.
- Landing pages that must be radically different can be **standalone** `+page.svelte`, but they must still respect the design system and tokens.

### 4.2 SSR / SSG / CSR

- Default: **SSR enabled**.
- SSG used where:
  - Content is mostly static and can be pre-rendered.
- CSR-only is **exception**, not default.

AI agents MUST:

- Not disable SSR unless explicitly instructed.
- Ensure that GSAP / DOM-only logic runs inside `onMount`.

---

## 5. Components & Code Style

### 5.1 Components

- Reuse components from `/src/lib/components`:
  - Buttons, cards, navbars, badge, inputs, layout shells.
- No copy-paste duplicate components with minor tweaks; extend via props or variants.

### 5.2 TypeScript

- `strict` enabled.
- No `any` unless absolutely necessary and documented with a comment.
- Prefer typed props and exports.

### 5.3 File Naming

- Components: `PascalCase.svelte` (e.g. `NavbarPrimary.svelte`)
- Utility modules: `kebab-case.ts` (e.g. `format-price.ts`)
- Stores: `something-store.ts`

---

## 6. Animations & Motion

- Default library: **GSAP** for complex motion, Svelte transitions for simple ones.
- Rules:
  - Most entrance animations: 0.3s–0.6s.
  - No blocking animations for critical interactions (e.g. open menu).
  - Respect `prefers-reduced-motion`: provide reduced or disabled motion variants.

Use animation to:

- Emphasize hierarchy, guide the eye.
- Avoid gimmicky, random effects.

---

## 7. Accessibility & Quality

- WCAG 2.1 AA contrast minimum.
- All buttons/links:
  - Have accessible names (`aria-label`, `title`, or text).
  - Keyboard-focusable with clear visible focus.
- Forms:
  - Each input has a `<label>` or `aria-labelledby`.

Testing expectations:

- Lint clean (ESLint + TypeScript).
- No console errors/warnings in dev tools.
- Layout tested at key breakpoints before shipping.

---

## 8. AI Agent Rules

All AI agents working on this repo MUST:

1. Read this document first.
2. Follow these rules even if the user prompt is underspecified.
3. When in doubt, choose:
   - SSR over CSR.
   - Reusable components over duplication.
   - Consistency with tokens over ad-hoc styles.
4. When returning code:
   - Provide full file content if the user asks.
   - Do not invent non-existent libraries or components.
   - Keep imports accurate to this project’s stack.

---

## 9. Page-Level Conventions (Trading Context)

- Hero sections:
  - Clear “What this is” + “Who it’s for” + primary CTA.
  - Optional secondary CTA (e.g. “View pricing”, “Watch 2-min overview”).
- For trading products (rooms, alerts, indicators):
  - Emphasize risk management, education, and **no get-rich-quick language**.
  - Use data viz to **clarify**, not to hype.

End of SSOT v1.

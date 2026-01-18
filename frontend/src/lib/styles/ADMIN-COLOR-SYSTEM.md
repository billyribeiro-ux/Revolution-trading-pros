# RTP Admin Color System

> **Version:** 1.0.0
> **Scope:** Admin portal pages only (`/admin/*`)
> **Note:** Marketing/frontend pages have separate, page-specific styling.

---

## Quick Reference

### Background Scale

| Token | Hex | Usage |
|-------|-----|-------|
| `--bg-base` | `#0D1117` | Main app background |
| `--bg-elevated` | `#161B22` | Cards, modals, elevated surfaces |
| `--bg-surface` | `#1C2128` | Input backgrounds, nested elements |
| `--bg-hover` | `#252B33` | Hover states |
| `--bg-active` | `#2D333B` | Active/pressed states |

### Primary - Gold/Amber (Brand Color)

| Token | Hex | Usage |
|-------|-----|-------|
| `--primary-400` | `#FFD11A` | Bright highlights, active states |
| `--primary-500` | `#E6B800` | **Main primary** - CTAs, buttons |
| `--primary-600` | `#B38F00` | Hover state for primary |
| `--primary-700` | `#806600` | Active/pressed state |

**Use for:** Primary CTAs, active navigation, focus rings, key highlights.

### Secondary - Slate Blue (Complement)

| Token | Hex | Usage |
|-------|-----|-------|
| `--secondary-300` | `#7995CA` | Light secondary accent |
| `--secondary-500` | `#3D5A99` | **Main secondary** |
| `--secondary-700` | `#233557` | Dark secondary |

**Use for:** Secondary actions, informational elements, visual variety.

### Text Hierarchy

| Token | Hex | Usage |
|-------|-----|-------|
| `--text-primary` | `#F0F6FC` | Headings, important content |
| `--text-secondary` | `#8B949E` | Descriptions, metadata |
| `--text-tertiary` | `#6E7681` | Labels, hints, captions |
| `--text-muted` | `#484F58` | Disabled, placeholders |

### Border System

| Token | Hex | Usage |
|-------|-----|-------|
| `--border-default` | `#30363D` | Standard borders |
| `--border-muted` | `#21262D` | Subtle dividers |
| `--border-emphasis` | `#8B949E` | Emphasized borders, focus |

### Semantic Colors

#### Success
| Token | Value | Usage |
|-------|-------|-------|
| `--success-soft` | `rgba(46, 160, 67, 0.15)` | Background |
| `--success-base` | `#2EA043` | Icons, borders |
| `--success-emphasis` | `#3FB950` | Text, highlights |

#### Warning
| Token | Value | Usage |
|-------|-------|-------|
| `--warning-soft` | `rgba(187, 128, 9, 0.15)` | Background |
| `--warning-base` | `#BB8009` | Icons, borders |
| `--warning-emphasis` | `#D29922` | Text, highlights |

#### Error
| Token | Value | Usage |
|-------|-------|-------|
| `--error-soft` | `rgba(218, 54, 51, 0.15)` | Background |
| `--error-base` | `#DA3633` | Icons, borders |
| `--error-emphasis` | `#F85149` | Text, highlights |

#### Info
| Token | Value | Usage |
|-------|-------|-------|
| `--info-soft` | `rgba(56, 139, 253, 0.15)` | Background |
| `--info-base` | `#388BFD` | Icons, borders |
| `--info-emphasis` | `#58A6FF` | Text, highlights |

---

## Admin-Specific Variables

These variables are available within `.admin-layout` context:

### Surfaces
```css
var(--admin-bg)              /* Main background */
var(--admin-surface)         /* Card/panel background */
var(--admin-surface-raised)  /* Elevated surfaces */
```

### Text
```css
var(--admin-text-primary)    /* Primary text */
var(--admin-text-secondary)  /* Secondary text */
var(--admin-text-muted)      /* Muted text */
```

### Accents
```css
var(--admin-accent-primary)       /* Gold accent */
var(--admin-accent-primary-hover) /* Gold hover */
var(--admin-accent-secondary)     /* Blue accent */
```

### Navigation
```css
var(--admin-nav-text)        /* Nav item text */
var(--admin-nav-text-active) /* Active nav (gold) */
var(--admin-nav-bg-hover)    /* Nav hover bg */
var(--admin-nav-bg-active)   /* Active nav bg (gold tint) */
```

---

## Usage Guidelines

### DO

```css
/* Use CSS variables */
color: var(--text-primary);
background: var(--bg-elevated);
border-color: var(--border-default);

/* Use semantic colors for their purpose */
.success { color: var(--success-emphasis); }
.error { color: var(--error-emphasis); }
```

### DON'T

```css
/* Don't hardcode hex values */
color: #f1f5f9;  /* Wrong - use var(--text-primary) */
color: #64748b;  /* Wrong - use var(--text-tertiary) */

/* Don't use Tailwind colors */
color: #22c55e;  /* Wrong - use var(--success-emphasis) */
color: #ef4444;  /* Wrong - use var(--error-base) */

/* Don't use non-spec colors */
color: #a855f7;  /* Purple - not in system */
color: #ec4899;  /* Pink - not in system */
```

---

## Color Mapping Reference

### Tailwind → RTP Admin

If migrating from Tailwind colors:

| Tailwind | RTP Admin Variable |
|----------|-------------------|
| `slate-950`, `#0f172a` | `var(--bg-base)` |
| `slate-900`, `#1e293b` | `var(--bg-elevated)` |
| `slate-800`, `#334155` | `var(--bg-surface)` |
| `slate-700`, `#475569` | `var(--text-muted)` |
| `slate-500`, `#64748b` | `var(--text-tertiary)` |
| `slate-400`, `#94a3b8` | `var(--text-secondary)` |
| `slate-300`, `#cbd5e1` | `var(--text-primary)` |
| `slate-200`, `#e2e8f0` | `var(--text-primary)` |
| `slate-100`, `#f1f5f9` | `var(--text-primary)` |
| `green-500`, `#22c55e` | `var(--success-base)` |
| `green-400`, `#4ade80` | `var(--success-emphasis)` |
| `emerald-500`, `#10b981` | `var(--success-base)` |
| `emerald-400`, `#34d399` | `var(--success-emphasis)` |
| `red-500`, `#ef4444` | `var(--error-base)` |
| `red-400`, `#f87171` | `var(--error-emphasis)` |
| `amber-500`, `#f59e0b` | `var(--warning-base)` |
| `amber-400`, `#fbbf24` | `var(--warning-emphasis)` |
| `yellow-500`, `#eab308` | `var(--primary-500)` |
| `blue-500`, `#3b82f6` | `var(--secondary-500)` |
| `indigo-300`, `#a5b4fc` | `var(--secondary-300)` |

---

## Button Patterns

### Primary Button (Gold)
```css
.btn-primary {
  background: var(--primary-500);
  color: var(--bg-base);
}
.btn-primary:hover {
  background: var(--primary-400);
}
```

### Secondary Button
```css
.btn-secondary {
  background: var(--bg-surface);
  color: var(--text-primary);
  border: 1px solid var(--border-default);
}
.btn-secondary:hover {
  background: var(--bg-hover);
  border-color: var(--border-emphasis);
}
```

### Ghost Button
```css
.btn-ghost {
  background: transparent;
  color: var(--text-secondary);
}
.btn-ghost:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}
```

### Danger Button
```css
.btn-danger {
  background: var(--error-base);
  color: white;
}
.btn-danger:hover {
  background: var(--error-emphasis);
}
```

---

## Navigation Active State

```css
.nav-item.active {
  background: rgba(230, 184, 0, 0.12);
  color: var(--primary-400);
  border-left: 3px solid var(--primary-500);
}
```

---

## Status Badge Patterns

```css
/* Active/Success */
.badge-active {
  background: var(--success-soft);
  color: var(--success-emphasis);
}

/* Warning/Pending */
.badge-pending {
  background: var(--warning-soft);
  color: var(--warning-emphasis);
}

/* Error/Inactive */
.badge-error {
  background: var(--error-soft);
  color: var(--error-emphasis);
}

/* Info */
.badge-info {
  background: var(--info-soft);
  color: var(--info-emphasis);
}

/* Primary/Gold */
.badge-primary {
  background: rgba(230, 184, 0, 0.15);
  color: var(--primary-400);
}

/* Muted/Default */
.badge-muted {
  background: var(--bg-hover);
  color: var(--text-secondary);
}
```

---

## Focus States

```css
:focus-visible {
  outline: 2px solid var(--primary-500);
  outline-offset: 2px;
}

/* Input focus */
.input:focus {
  border-color: var(--primary-500);
  box-shadow: 0 0 0 3px rgba(230, 184, 0, 0.15);
}
```

---

## File Locations

- **Token definitions:** `frontend/src/lib/styles/tokens/colors.css`
- **Admin variables:** `frontend/src/app.css` (`.admin-layout` section)
- **Global styles:** `frontend/src/lib/styles/base/global.css`

# CLASS DOWNLOADS SECTION - COMPLETE TECHNICAL SPECIFICATION

**Apple ICT 7 Principal Engineer Grade - Pixel-Perfect Implementation**

---

## TABLE OF CONTENTS

1. [Overview](#overview)
2. [HTML Structure](#html-structure)
3. [JavaScript/Svelte Logic](#javascriptsvelte-logic)
4. [CSS Design Tokens](#css-design-tokens)
5. [Base Styles](#base-styles)
6. [Responsive Breakpoints](#responsive-breakpoints)
7. [Accessibility Features](#accessibility-features)
8. [Print Styles](#print-styles)
9. [Implementation Workflow](#implementation-workflow)

---

## OVERVIEW

### Purpose

The Class Downloads section displays embedded Box.com file listings for course materials (PowerPoint and PDF files) in a responsive, accessible container.

### Key Metrics

- **Desktop Container**: 1080px × 512px
- **Desktop Iframe**: 100% width × 518px height
- **Mobile Container**: 100% width × auto height (min 350px)
- **Box.com URL**: `https://simplertrading.app.box.com/embed/s/ith1lbi9t3v91z5qnrphr8q4dz0mu6xq?sortColumn=date&view=list`

---

## HTML STRUCTURE

### Complete Markup (Lines 143-165)

```html
<!-- section#dl-rp-row.class-section.cpost-section -->
<section class="class-section cpost-section" id="dl-rp-row">
	<div class="section-inner">
		<section class="class-subsection" id="class-downloads">
			<h2>Class Downloads</h2>
			<div class="class-downloads-container">
				<iframe
					src="https://simplertrading.app.box.com/embed/s/ith1lbi9t3v91z5qnrphr8q4dz0mu6xq?sortColumn=date&view=list"
					width="500"
					height="400"
					allowfullscreen
					title="Class Downloads"
					loading="lazy"
					referrerpolicy="strict-origin-when-cross-origin"
					sandbox="allow-scripts allow-same-origin allow-forms allow-popups allow-popups-to-escape-sandbox"
				></iframe>
				<noscript>
					<p class="iframe-fallback">
						Please enable JavaScript to view class downloads, or
						<a
							href="https://simplertrading.app.box.com/s/ith1lbi9t3v91z5qnrphr8q4dz0mu6xq"
							target="_blank"
							rel="noopener noreferrer"
							>click here to open in Box.com</a
						>.
					</p>
				</noscript>
			</div>
		</section>
	</div>
</section>
```

### HTML Element Breakdown

#### 1. Outer Section Container

```html
<section class="class-section cpost-section" id="dl-rp-row"></section>
```

- **ID**: `dl-rp-row` (Downloads & Related Products Row)
- **Classes**: `class-section`, `cpost-section`
- **Purpose**: Top-level semantic container for the downloads section

#### 2. Section Inner Wrapper

```html
<div class="section-inner"></div>
```

- **Purpose**: Centers content and provides responsive padding
- **Max-width**: 1100px
- **Padding**: Responsive (0-40px based on viewport)

#### 3. Downloads Subsection

```html
<section class="class-subsection" id="class-downloads"></section>
```

- **ID**: `class-downloads`
- **Class**: `class-subsection`
- **Purpose**: White background card containing heading and iframe
- **Dimensions**: 1080px × 512px (desktop)

#### 4. Section Heading

```html
<h2>Class Downloads</h2>
```

- **Font-size**: 1.75rem (28px)
- **Font-weight**: 400 (normal)
- **Color**: `#4a4a4a` (--color-text-heading)
- **Margin-bottom**: 25px

#### 5. Iframe Container

```html
<div class="class-downloads-container"></div>
```

- **Purpose**: Wrapper for iframe with exact dimensions
- **Width**: 100%
- **Height**: 518px (desktop), responsive on mobile

#### 6. Box.com Iframe

```html
<iframe
	src="https://simplertrading.app.box.com/embed/s/ith1lbi9t3v91z5qnrphr8q4dz0mu6xq?sortColumn=date&view=list"
	width="500"
	height="400"
	allowfullscreen
	title="Class Downloads"
	loading="lazy"
	referrerpolicy="strict-origin-when-cross-origin"
	sandbox="allow-scripts allow-same-origin allow-forms allow-popups allow-popups-to-escape-sandbox"
></iframe>
```

**Iframe Attributes:**

- **src**: Box.com embed URL with query params
  - `sortColumn=date`: Sort files by date
  - `view=list`: Display as list view
- **width**: 500 (overridden by CSS)
- **height**: 400 (overridden by CSS)
- **allowfullscreen**: Enables fullscreen mode
- **title**: "Class Downloads" (accessibility)
- **loading**: "lazy" (performance optimization)
- **referrerpolicy**: "strict-origin-when-cross-origin" (security)
- **sandbox**: Multiple permissions for Box.com functionality
  - `allow-scripts`: JavaScript execution
  - `allow-same-origin`: Same-origin requests
  - `allow-forms`: Form submission
  - `allow-popups`: Open popups
  - `allow-popups-to-escape-sandbox`: Popup navigation

#### 7. Noscript Fallback

```html
<noscript>
	<p class="iframe-fallback">
		Please enable JavaScript to view class downloads, or
		<a
			href="https://simplertrading.app.box.com/s/ith1lbi9t3v91z5qnrphr8q4dz0mu6xq"
			target="_blank"
			rel="noopener noreferrer"
		>
			click here to open in Box.com </a
		>.
	</p>
</noscript>
```

- **Purpose**: Fallback for users without JavaScript
- **Class**: `iframe-fallback`
- **Link**: Direct to Box.com folder

---

## JAVASCRIPT/SVELTE LOGIC

### Reactive State Management (Lines 10-53)

#### 1. Imports

```typescript
import { onMount } from 'svelte';
import DashboardBreadcrumbs from '$lib/components/dashboard/DashboardBreadcrumbs.svelte';
import HaveQuestionsSection from '$lib/components/sections/HaveQuestionsSection.svelte';
```

#### 2. Svelte 5 Runes - State Variables

```typescript
let viewportWidth = $state(0);
let prefersReducedMotion = $state(false);
let isClient = $state(false);
```

**State Variables:**

- `viewportWidth`: Current window width in pixels
- `prefersReducedMotion`: User's motion preference (accessibility)
- `isClient`: Boolean flag for client-side rendering

#### 3. Derived Breakpoint Logic

```typescript
const breakpoint = $derived.by(() => {
	if (viewportWidth === 0) return 'mobile';
	if (viewportWidth < 428) return 'mobile-small';
	if (viewportWidth < 744) return 'mobile';
	if (viewportWidth < 1024) return 'tablet';
	if (viewportWidth < 1366) return 'desktop';
	return 'desktop-large';
});
```

**Breakpoint Ranges:**

- `mobile-small`: 0-427px (iPhone SE, Mini)
- `mobile`: 428-743px (iPhone Pro Max)
- `tablet`: 744-1023px (iPad Mini, Air)
- `desktop`: 1024-1365px (iPad Pro, laptops)
- `desktop-large`: 1366px+ (large displays)

#### 4. Viewport Tracking Effect

```typescript
$effect(() => {
	if (!isClient) return;

	const updateViewport = () => {
		viewportWidth = window.innerWidth;
	};

	const mediaQuery = window.matchMedia('(prefers-reduced-motion: reduce)');
	prefersReducedMotion = mediaQuery.matches;

	const handleMotionChange = (e: MediaQueryListEvent) => {
		prefersReducedMotion = e.matches;
	};

	updateViewport();
	window.addEventListener('resize', updateViewport, { passive: true });
	mediaQuery.addEventListener('change', handleMotionChange);

	return () => {
		window.removeEventListener('resize', updateViewport);
		mediaQuery.removeEventListener('change', handleMotionChange);
	};
});
```

**Effect Workflow:**

1. Check if client-side (`isClient`)
2. Define `updateViewport()` function to capture window width
3. Create media query for reduced motion preference
4. Set initial `prefersReducedMotion` state
5. Define motion change handler
6. Execute initial viewport update
7. Add resize listener (passive for performance)
8. Add motion preference listener
9. Return cleanup function to remove listeners

#### 5. Component Mount Lifecycle

```typescript
onMount(() => {
	isClient = true;

	if (typeof window !== 'undefined' && (window as any).richpanel) {
		(window as any).richpanel.track('view_article', {
			id: 1142327,
			name: 'Quickstart To Precision Trading',
			url: window.location.href
		});
	}

	if (typeof window !== 'undefined' && (window as any).gtag) {
		(window as any).gtag('event', 'page_view', {
			page_title: 'Quickstart To Precision Trading',
			page_location: window.location.href,
			page_path: window.location.pathname,
			content_type: 'article'
		});
	}
});
```

**Mount Workflow:**

1. Set `isClient = true` to enable reactive effects
2. Track article view in Richpanel (support analytics)
3. Track page view in Google Analytics (gtag)

**Analytics Data:**

- **Richpanel**: Article ID 1142327
- **Google Analytics**: Page title, location, path, content type

---

## CSS DESIGN TOKENS

### Root Variables (Lines 184-227)

#### Color Tokens (Lines 185-197)

```css
:root {
	/* ═══ Color Tokens - CORRECTED ═══ */
	--color-page-bg: #efefef;
	--color-content-bg: #ffffff;
	--color-video-bg: #0a2335;
	--color-text-primary: #666666;
	--color-text-heading: #4a4a4a;
	--color-text-light: #999999;
	--color-text-inverse: #ffffff;
	--color-text-inverse-muted: #d5d5d5;
	--color-link: #1e73be;
	--color-link-hover: #000000;
	--color-border: #999999;
	--color-border-light: #dddddd;
}
```

**Color Palette:**
| Token | Hex Value | RGB | Usage |
|-------|-----------|-----|-------|
| `--color-page-bg` | `#EFEFEF` | rgb(239, 239, 239) | Body background (light gray) |
| `--color-content-bg` | `#FFFFFF` | rgb(255, 255, 255) | Downloads container background (white) |
| `--color-video-bg` | `#0a2335` | rgb(10, 35, 53) | Video section background (dark blue) |
| `--color-text-primary` | `#666666` | rgb(102, 102, 102) | Primary text color (medium gray) |
| `--color-text-heading` | `#4a4a4a` | rgb(74, 74, 74) | Heading text color (dark gray) |
| `--color-text-light` | `#999999` | rgb(153, 153, 153) | Light text color |
| `--color-text-inverse` | `#ffffff` | rgb(255, 255, 255) | Inverse text (white on dark) |
| `--color-text-inverse-muted` | `#d5d5d5` | rgb(213, 213, 213) | Muted inverse text |
| `--color-link` | `#1e73be` | rgb(30, 115, 190) | Link color (blue) |
| `--color-link-hover` | `#000000` | rgb(0, 0, 0) | Link hover color (black) |
| `--color-border` | `#999999` | rgb(153, 153, 153) | Border color (medium gray) |
| `--color-border-light` | `#dddddd` | rgb(221, 221, 221) | Light border color |

#### Spacing Tokens (Lines 199-209)

```css
/* ═══ Spacing Tokens (8px grid) ═══ */
--space-1: 0.25rem; /* 4px */
--space-2: 0.5rem; /* 8px */
--space-3: 0.75rem; /* 12px */
--space-4: 1rem; /* 16px */
--space-5: 1.25rem; /* 20px */
--space-6: 1.5rem; /* 24px */
--space-8: 2rem; /* 32px */
--space-10: 2.5rem; /* 40px */
--space-12: 3rem; /* 48px */
--space-15: 3.75rem; /* 60px */
```

**8px Grid System:**
| Token | Rem | Pixels | Usage |
|-------|-----|--------|-------|
| `--space-1` | 0.25rem | 4px | Micro spacing |
| `--space-2` | 0.5rem | 8px | Base grid unit |
| `--space-3` | 0.75rem | 12px | Small spacing |
| `--space-4` | 1rem | 16px | Standard spacing |
| `--space-5` | 1.25rem | 20px | Section padding (mobile) |
| `--space-6` | 1.5rem | 24px | Medium spacing |
| `--space-8` | 2rem | 32px | Large spacing |
| `--space-10` | 2.5rem | 40px | Section padding (desktop) |
| `--space-12` | 3rem | 48px | Extra large spacing |
| `--space-15` | 3.75rem | 60px | Section padding (vertical) |

#### Typography Tokens (Lines 211-217)

```css
/* ═══ Fluid Typography ═══ */
--font-size-sm: clamp(0.875rem, 0.8rem + 0.375vw, 1rem);
--font-size-base: clamp(1rem, 0.925rem + 0.375vw, 1.125rem);
--font-size-lg: clamp(1.1rem, 1rem + 0.5vw, 1.4rem);
--font-size-xl: clamp(1.2rem, 1rem + 1vw, 1.75rem);
--font-size-2xl: clamp(1.5rem, 1.25rem + 1.25vw, 2rem);
--font-size-3xl: clamp(1.75rem, 1.5rem + 1.5vw, 2.5rem);
```

**Fluid Typography Scale:**
| Token | Min Size | Fluid Formula | Max Size | Usage |
|-------|----------|---------------|----------|-------|
| `--font-size-sm` | 0.875rem (14px) | 0.8rem + 0.375vw | 1rem (16px) | Small text |
| `--font-size-base` | 1rem (16px) | 0.925rem + 0.375vw | 1.125rem (18px) | Body text |
| `--font-size-lg` | 1.1rem (17.6px) | 1rem + 0.5vw | 1.4rem (22.4px) | Large text |
| `--font-size-xl` | 1.2rem (19.2px) | 1rem + 1vw | 1.75rem (28px) | Extra large |
| `--font-size-2xl` | 1.5rem (24px) | 1.25rem + 1.25vw | 2rem (32px) | Heading 2 |
| `--font-size-3xl` | 1.75rem (28px) | 1.5rem + 1.5vw | 2.5rem (40px) | Heading 1 |

#### Safe Area Insets (Lines 219-223)

```css
/* ═══ Safe Areas ═══ */
--safe-area-inset-top: env(safe-area-inset-top, 0px);
--safe-area-inset-right: env(safe-area-inset-right, 0px);
--safe-area-inset-bottom: env(safe-area-inset-bottom, 0px);
--safe-area-inset-left: env(safe-area-inset-left, 0px);
```

**Purpose**: Handle notched devices (iPhone X+, iPad Pro)

#### Touch Target Minimum (Lines 225-226)

```css
/* ═══ Touch Targets ═══ */
--touch-target-min: 44px;
```

**Apple HIG Compliance**: Minimum 44pt touch target for accessibility

---

## BASE STYLES

### Class Downloads Section Styles (Lines 432-496)

#### 1. Outer Section Container (Lines 435-437)

```css
#dl-rp-row {
	padding: var(--space-10) 0;
}
```

**Properties:**

- **Padding**: 40px top/bottom, 0 left/right
- **Purpose**: Vertical spacing around downloads section

#### 2. Section Inner Wrapper (Lines 439-446)

```css
#dl-rp-row .section-inner {
	display: flex;
	flex-direction: column;
	align-items: center;
	max-width: 1100px;
	margin: 0 auto;
	padding: 0 var(--space-5);
}
```

**Properties:**
| Property | Value | Purpose |
|----------|-------|---------|
| `display` | flex | Flexbox layout |
| `flex-direction` | column | Vertical stacking |
| `align-items` | center | Horizontal centering |
| `max-width` | 1100px | Container max width |
| `margin` | 0 auto | Horizontal centering |
| `padding` | 0 20px | Horizontal padding |

#### 3. Downloads Subsection Container (Lines 448-456)

```css
.class-subsection#class-downloads {
	background-color: var(--color-content-bg);
	padding: 25px;
	width: 100%;
	max-width: 1080px;
	height: 512px;
	box-sizing: border-box;
	margin: 0 auto;
}
```

**EXACT DESKTOP DIMENSIONS:**
| Property | Value | Pixels | Notes |
|----------|-------|--------|-------|
| `background-color` | #FFFFFF | - | White background |
| `padding` | 25px | 25px | All sides |
| `width` | 100% | - | Full width of parent |
| `max-width` | 1080px | 1080px | **EXACT container width** |
| `height` | 512px | 512px | **EXACT container height** |
| `box-sizing` | border-box | - | Padding included in dimensions |
| `margin` | 0 auto | - | Horizontal centering |

**Content Area Calculation:**

- Total container: 1080px × 512px
- Padding: 25px all sides
- Content area: 1030px × 462px
- Heading height: ~28px + 25px margin = 53px
- Iframe available: 1030px × 409px (but set to 518px to overflow)

#### 4. Section Heading (Lines 458-465)

```css
#class-downloads h2 {
	font-size: 1.75rem;
	font-weight: 400;
	color: var(--color-text-heading);
	margin: 0 0 25px 0;
	text-align: left;
	line-height: 1.2;
}
```

**Typography Specifications:**
| Property | Value | Computed | Notes |
|----------|-------|----------|-------|
| `font-size` | 1.75rem | 28px | Fixed size (not fluid) |
| `font-weight` | 400 | normal | Regular weight |
| `color` | #4a4a4a | Dark gray | Heading color token |
| `margin` | 0 0 25px 0 | - | 25px bottom margin |
| `text-align` | left | - | Left-aligned |
| `line-height` | 1.2 | 33.6px | Tight line height |

#### 5. Iframe Container (Lines 467-472)

```css
.class-downloads-container {
	width: 100%;
	height: 518px;
	box-sizing: border-box;
	background: var(--color-content-bg);
}
```

**Container Specifications:**
| Property | Value | Notes |
|----------|-------|-------|
| `width` | 100% | Full width of parent (1030px after padding) |
| `height` | 518px | **EXACT iframe container height** |
| `box-sizing` | border-box | Standard box model |
| `background` | #FFFFFF | White background |

**Height Calculation:**

- Container height: 512px
- Padding top: 25px
- Heading height: ~28px
- Heading margin: 25px
- Remaining space: 512 - 25 - 28 - 25 = 434px
- Iframe height: 518px (intentionally overflows by 84px)

#### 6. Iframe Element (Lines 474-479)

```css
.class-downloads-container iframe {
	width: 100%;
	height: 518px;
	border: none;
	display: block;
}
```

**Iframe Specifications:**
| Property | Value | Notes |
|----------|-------|-------|
| `width` | 100% | Full width of container |
| `height` | 518px | **EXACT iframe height** |
| `border` | none | No border |
| `display` | block | Block-level element |

**Critical Dimension:**

- **518px height** is the exact specification for desktop
- This creates a slight overflow within the 512px container
- The overflow is intentional for the Box.com embed UI

#### 7. Fallback Styles (Lines 481-496)

```css
.iframe-fallback {
	padding: var(--space-5);
	text-align: center;
	background-color: var(--color-content-bg);
	border: 1px solid var(--color-border-light);
	border-radius: 4px;
	color: var(--color-text-primary);
}

.iframe-fallback a {
	color: var(--color-link);
	text-decoration: underline;
	display: inline-block;
	min-height: var(--touch-target-min);
	line-height: var(--touch-target-min);
}
```

**Fallback Specifications:**
| Selector | Property | Value | Purpose |
|----------|----------|-------|---------|
| `.iframe-fallback` | padding | 20px | Inner spacing |
| | text-align | center | Centered text |
| | background-color | #FFFFFF | White background |
| | border | 1px solid #dddddd | Light border |
| | border-radius | 4px | Rounded corners |
| | color | #666666 | Text color |
| `.iframe-fallback a` | color | #1e73be | Link color |
| | text-decoration | underline | Underlined |
| | display | inline-block | Block formatting |
| | min-height | 44px | Touch target |
| | line-height | 44px | Vertical centering |

---

## RESPONSIVE BREAKPOINTS

### Mobile Small: < 428px (Lines 502-531)

```css
@media (max-width: 427px) {
	.cpost-section .section-inner {
		padding: 0 var(--space-4);
	}

	#non-member-class-upsell {
		padding: var(--space-4);
		min-height: auto;
	}

	.class-subsection#class-downloads {
		width: 100%;
		height: auto;
		min-height: 350px;
		padding: var(--space-4);
	}

	.class-downloads-container {
		height: 300px;
	}

	.class-downloads-container iframe {
		height: 300px;
	}

	.current-vid {
		padding: var(--space-3);
	}
}
```

**Mobile Small Specifications:**
| Element | Property | Value | Change from Desktop |
|---------|----------|-------|---------------------|
| `.section-inner` | padding | 0 16px | Reduced from 20px |
| `.class-subsection#class-downloads` | width | 100% | Full width |
| | height | auto | Flexible height |
| | min-height | 350px | Minimum container |
| | padding | 16px | Reduced from 25px |
| `.class-downloads-container` | height | 300px | Reduced from 518px |
| `.class-downloads-container iframe` | height | 300px | Reduced from 518px |

**Dimension Changes:**

- Container: 100% width × auto height (min 350px)
- Iframe: 100% width × 300px height
- Padding: 16px (reduced for mobile)

### Mobile: 428px - 743px (Lines 533-548)

```css
@media (min-width: 428px) and (max-width: 743px) {
	.class-subsection#class-downloads {
		width: 100%;
		height: auto;
		min-height: 400px;
	}

	.class-downloads-container {
		height: 350px;
	}

	.class-downloads-container iframe {
		height: 350px;
	}
}
```

**Mobile Specifications:**
| Element | Property | Value |
|---------|----------|-------|
| `.class-subsection#class-downloads` | width | 100% |
| | height | auto |
| | min-height | 400px |
| `.class-downloads-container` | height | 350px |
| `.class-downloads-container iframe` | height | 350px |

### Tablet: 744px - 1023px (Lines 550-574)

```css
@media (min-width: 744px) and (max-width: 1023px) {
	.cpost-section .section-inner {
		padding: 0 var(--space-8);
	}

	.class-subsection#class-downloads {
		width: 100%;
		max-width: 720px;
		height: auto;
		min-height: 450px;
	}

	.class-downloads-container {
		height: 400px;
	}

	.class-downloads-container iframe {
		height: 400px;
	}

	.current-vid {
		padding: var(--space-6);
	}
}
```

**Tablet Specifications:**
| Element | Property | Value |
|---------|----------|-------|
| `.section-inner` | padding | 0 32px |
| `.class-subsection#class-downloads` | width | 100% |
| | max-width | 720px |
| | height | auto |
| | min-height | 450px |
| `.class-downloads-container` | height | 400px |
| `.class-downloads-container iframe` | height | 400px |

### Desktop: 1024px - 1365px (Lines 576-595)

```css
@media (min-width: 1024px) and (max-width: 1365px) {
	.cpost-section .section-inner {
		padding: 0 var(--space-10);
	}

	.class-subsection#class-downloads {
		width: 100%;
		max-width: 1080px;
		height: 512px;
	}

	.class-downloads-container {
		height: 518px;
	}

	.class-downloads-container iframe {
		height: 518px;
	}
}
```

**Desktop Specifications:**
| Element | Property | Value |
|---------|----------|-------|
| `.section-inner` | padding | 0 40px |
| `.class-subsection#class-downloads` | width | 100% |
| | max-width | 1080px |
| | height | 512px |
| `.class-downloads-container` | height | 518px |
| `.class-downloads-container iframe` | height | 518px |

### Desktop Large: 1366px+ (Lines 597-616)

```css
@media (min-width: 1366px) {
	.cpost-section .section-inner {
		padding: 0;
	}

	.class-subsection#class-downloads {
		width: 1080px;
		max-width: 1080px;
		height: 512px;
	}

	.class-downloads-container {
		height: 518px;
	}

	.class-downloads-container iframe {
		height: 518px;
	}
}
```

**Desktop Large Specifications:**
| Element | Property | Value |
|---------|----------|-------|
| `.section-inner` | padding | 0 |
| `.class-subsection#class-downloads` | width | 1080px (fixed) |
| | max-width | 1080px |
| | height | 512px |
| `.class-downloads-container` | height | 518px |
| `.class-downloads-container iframe` | height | 518px |

### Responsive Dimension Summary

| Breakpoint              | Container Width   | Container Height | Iframe Height | Padding |
| ----------------------- | ----------------- | ---------------- | ------------- | ------- |
| Mobile Small (< 428px)  | 100%              | auto (min 350px) | 300px         | 16px    |
| Mobile (428-743px)      | 100%              | auto (min 400px) | 350px         | 25px    |
| Tablet (744-1023px)     | 100% (max 720px)  | auto (min 450px) | 400px         | 25px    |
| Desktop (1024-1365px)   | 100% (max 1080px) | 512px            | 518px         | 25px    |
| Desktop Large (1366px+) | 1080px            | 512px            | 518px         | 25px    |

---

## ACCESSIBILITY FEATURES

### 1. Reduced Motion Support (Lines 232-239)

```css
@media (prefers-reduced-motion: reduce) {
	*,
	*::before,
	*::after {
		animation-duration: 0.01ms !important;
		animation-iteration-count: 1 !important;
		transition-duration: 0.01ms !important;
		scroll-behavior: auto !important;
	}
}
```

**Purpose**: Respects user's motion sensitivity preferences (WCAG 2.2 AA)

### 2. Touch Target Compliance

```css
.iframe-fallback a {
	min-height: var(--touch-target-min); /* 44px */
	line-height: var(--touch-target-min); /* 44px */
}
```

**Apple HIG**: Minimum 44pt touch target for all interactive elements

### 3. Semantic HTML

- `<section>` for semantic structure
- `<h2>` for proper heading hierarchy
- `title` attribute on iframe
- `<noscript>` fallback for accessibility

### 4. Iframe Accessibility Attributes

```html
title="Class Downloads" loading="lazy" referrerpolicy="strict-origin-when-cross-origin"
sandbox="allow-scripts allow-same-origin allow-forms allow-popups allow-popups-to-escape-sandbox"
```

### 5. Focus Visible States (Lines 274-278)

```css
:global(a:focus-visible) {
	outline: 2px solid var(--color-link);
	outline-offset: 2px;
	border-radius: 4px;
}
```

**Purpose**: Clear focus indicators for keyboard navigation

### 6. High Contrast Mode (Lines 667-678)

```css
@media (prefers-contrast: high) {
	:root {
		--color-text-primary: #000000;
		--color-text-heading: #000000;
		--color-border: #000000;
	}

	.class-video-container {
		border-width: 2px;
	}
}
```

**Purpose**: Enhanced contrast for users with visual impairments

---

## PRINT STYLES

### Print Media Query (Lines 640-665)

```css
@media print {
	:global(body) {
		background-color: white !important;
	}

	.current-vid {
		background-color: #f0f0f0 !important;
	}

	.class-player-header,
	.current-title {
		color: black !important;
	}

	.class-downloads-container iframe {
		display: none;
	}

	.class-downloads-container::after {
		content: 'Visit simplertrading.app.box.com for downloads';
		display: block;
		padding: var(--space-5);
		text-align: center;
	}
}
```

**Print Optimizations:**

1. White background for body
2. Light gray background for video section
3. Black text for readability
4. Hide iframe (can't print embedded content)
5. Display fallback message with Box.com URL

---

## IMPLEMENTATION WORKFLOW

### Step 1: HTML Structure Setup

1. Create outer section with `id="dl-rp-row"`
2. Add `.section-inner` wrapper for centering
3. Create subsection with `id="class-downloads"`
4. Add `<h2>` heading "Class Downloads"
5. Create `.class-downloads-container` div
6. Embed Box.com iframe with all attributes
7. Add `<noscript>` fallback with link

### Step 2: CSS Design Tokens

1. Define color tokens in `:root`
2. Define spacing tokens (8px grid)
3. Define fluid typography scale
4. Define safe area insets
5. Define touch target minimum

### Step 3: Base Styles

1. Style `#dl-rp-row` with vertical padding
2. Style `.section-inner` with flexbox centering
3. Style `.class-subsection#class-downloads` with exact dimensions (1080×512)
4. Style `#class-downloads h2` with typography
5. Style `.class-downloads-container` with 518px height
6. Style iframe with 100% width and 518px height
7. Style fallback message

### Step 4: Responsive Breakpoints

1. Mobile Small (< 428px): 300px iframe, 16px padding
2. Mobile (428-743px): 350px iframe
3. Tablet (744-1023px): 400px iframe, 720px max-width
4. Desktop (1024-1365px): 518px iframe, 1080px max-width
5. Desktop Large (1366px+): Fixed 1080px width

### Step 5: JavaScript/Svelte Logic

1. Import Svelte lifecycle hooks
2. Define reactive state variables
3. Create derived breakpoint logic
4. Implement viewport tracking effect
5. Add resize and motion preference listeners
6. Implement onMount analytics tracking

### Step 6: Accessibility

1. Add reduced motion support
2. Ensure touch target compliance
3. Add semantic HTML structure
4. Include iframe accessibility attributes
5. Implement focus visible states
6. Add high contrast mode support

### Step 7: Print Styles

1. Hide iframe in print view
2. Display fallback message
3. Optimize colors for print
4. Ensure readable text

### Step 8: Testing Checklist

- [ ] Desktop (1366px+): 1080×512 container, 518px iframe
- [ ] Desktop (1024-1365px): Responsive container, 518px iframe
- [ ] Tablet (744-1023px): 720px max-width, 400px iframe
- [ ] Mobile (428-743px): Full width, 350px iframe
- [ ] Mobile Small (< 428px): Full width, 300px iframe
- [ ] Box.com embed loads correctly
- [ ] Fallback message displays without JavaScript
- [ ] Touch targets meet 44px minimum
- [ ] Reduced motion preference respected
- [ ] High contrast mode works
- [ ] Print view displays fallback message
- [ ] Analytics tracking fires on mount
- [ ] Viewport tracking updates on resize

---

## CRITICAL MEASUREMENTS SUMMARY

### Desktop (1366px+) - EXACT SPECIFICATIONS

```
Container: 1080px width × 512px height
Padding: 25px all sides
Heading: 1.75rem (28px) + 25px margin-bottom
Iframe Container: 100% width × 518px height
Iframe: 100% width × 518px height
Background: #FFFFFF (white)
Border: none
```

### Content Area Calculation

```
Total Container: 1080px × 512px
Minus Padding: 1030px × 462px (content area)
Heading Space: 28px + 25px = 53px
Remaining: 409px
Iframe Height: 518px (overflows by 109px from content area)
```

### Color Specifications

```
Background: #FFFFFF (white)
Heading: #4a4a4a (dark gray)
Text: #666666 (medium gray)
Link: #1e73be (blue)
Border: #dddddd (light gray)
```

### Typography Specifications

```
Heading: 1.75rem (28px), weight 400, line-height 1.2
Font Family: "Open Sans", -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif
```

---

## END OF SPECIFICATION

**Document Version**: 1.0  
**Last Updated**: 2026-01-10  
**Engineer**: Apple ICT 7 Principal Engineer  
**Status**: Complete - Pixel-Perfect Implementation Ready

<svelte:head>
	<title>Quickstart To Precision Trading - Simpler Trading</title>
	<meta property="og:locale" content="en_US" />
	<meta property="og:type" content="article" />
	<meta property="og:title" content="Quickstart To Precision Trading" />
	<meta property="og:site_name" content="Revolution Trading Pros" />
	<meta name="viewport" content="width=device-width, initial-scale=1.0, viewport-fit=cover" />
</svelte:head>

<script lang="ts">
	import { onMount } from 'svelte';
	import DashboardBreadcrumbs from '$lib/components/dashboard/DashboardBreadcrumbs.svelte';
	import HaveQuestionsSection from '$lib/components/sections/HaveQuestionsSection.svelte';

	// ═══════════════════════════════════════════════════════════════════════════
	// SVELTE 5 RUNES - Reactive State Management
	// ═══════════════════════════════════════════════════════════════════════════
	
	// Viewport state for responsive behavior
	let viewportWidth = $state(0);
	let viewportHeight = $state(0);
	let prefersReducedMotion = $state(false);
	let isClient = $state(false);

	// Derived breakpoint states (Apple HIG aligned)
	const breakpoint = $derived.by(() => {
		if (viewportWidth === 0) return 'mobile';
		if (viewportWidth < 428) return 'mobile-small';    // iPhone SE, Mini
		if (viewportWidth < 744) return 'mobile';          // iPhone Pro Max
		if (viewportWidth < 1024) return 'tablet';         // iPad Mini, Air
		if (viewportWidth < 1366) return 'desktop';        // iPad Pro, laptops
		return 'desktop-large';                            // Large displays
	});

	const isMobile = $derived(breakpoint === 'mobile-small' || breakpoint === 'mobile');
	const isTablet = $derived(breakpoint === 'tablet');
	const isDesktop = $derived(breakpoint === 'desktop' || breakpoint === 'desktop-large');

	// Fluid container dimensions based on viewport
	const containerMaxWidth = $derived.by(() => {
		switch (breakpoint) {
			case 'mobile-small': return '100%';
			case 'mobile': return '100%';
			case 'tablet': return '720px';
			case 'desktop': return '1080px';
			case 'desktop-large': return '1080px';
			default: return '100%';
		}
	});

	// Downloads section dimensions (exact match to screenshot)
	const downloadsHeight = $derived.by(() => {
		switch (breakpoint) {
			case 'mobile-small': return '350px';
			case 'mobile': return '400px';
			case 'tablet': return '450px';
			case 'desktop': return '512px';
			case 'desktop-large': return '512px';
			default: return '400px';
		}
	});

	const iframeHeight = $derived.by(() => {
		switch (breakpoint) {
			case 'mobile-small': return '300px';
			case 'mobile': return '350px';
			case 'tablet': return '400px';
			case 'desktop': return '518px';
			case 'desktop-large': return '518px';
			default: return '350px';
		}
	});

	// Effect for viewport tracking and accessibility preferences
	$effect(() => {
		if (!isClient) return;

		const updateViewport = () => {
			viewportWidth = window.innerWidth;
			viewportHeight = window.innerHeight;
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

	onMount(() => {
		isClient = true;

		// Analytics tracking
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
</script>

<!-- Breadcrumbs -->
<DashboardBreadcrumbs />

<!-- WordPress Wrapper Structure -->
<div id="page" class="hfeed site grid-parent">
	<div id="content" class="site-content">
		<!-- section#class-info.class-section.cpost-section -->
		<section class="class-section cpost-section" id="class-info">
			<div class="section-inner">
				<div id="non-member-class-upsell">
					<div id="non-member-class-title">
						<h1>Quickstart To Precision Trading</h1>
						<!-- svelte-ignore a11y_missing_content -->
						<h3></h3>
					</div>
					<div id="non-member-class-description">
						<p></p>
					</div>
				</div>
			</div>
		</section>

		<!-- section#classroom-access.class-section.cpost-section -->
		<section class="class-section cpost-section" id="classroom-access">
			<div class="section-inner">
			</div>
		</section>

		<!-- section#class-recordings.class-section.cpost-section -->
		<section class="class-section cpost-section" id="class-recordings">
			<div class="section-inner">
				<div class="class-content-block cpost-content-block">
					<div class="current-vid">
						<h2 class="class-player-header">Quickstart To Precision Trading</h2>
						<h3 class="current-title">Quickstart To Precision Trading</h3>
						<div class="class-video-description">
							<p></p>
						</div>
						<div class="class-video-container current">
							<div class="video-overlay">
								<h3>Quickstart To Precision Trading</h3>
							</div>
							<div id="quickstart-to-precision-trading" class="class-video-player"></div>
							<video 
								id="https://simpler-options.s3.amazonaws.com/Moxie/MoxieQuickstart_A1.mp4" 
								controls 
								width="100%" 
								poster="" 
								style="aspect-ratio: 16/9;" 
								title="quickstart-to-precision-trading"
							>
								<source src="https://simpler-options.s3.amazonaws.com/Moxie/MoxieQuickstart_A1.mp4" type="video/mp4">
								Your browser does not support the video tag.
							</video>
						</div>
					</div>
					<div class=""></div>
				</div>
			</div>
		</section>

		<!-- section#class-elearning.class-section.cpost-section -->
		<section class="class-section cpost-section" id="class-elearning">
			<div class="section-inner">
			</div>
		</section>

		<!-- section#dl-rp-row.class-section.cpost-section -->
		<section class="class-section cpost-section" id="dl-rp-row">
			<div class="section-inner">
				<section 
					class="class-subsection" 
					id="class-downloads"
					style="--downloads-height: {downloadsHeight}; --container-max-width: {containerMaxWidth};"
				>
					<h2>Class Downloads</h2>
					<div class="class-downloads-container" style="--iframe-height: {iframeHeight};">
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
							<p class="iframe-fallback">Please enable JavaScript to view class downloads, or <a href="https://simplertrading.app.box.com/s/ith1lbi9t3v91z5qnrphr8q4dz0mu6xq" target="_blank" rel="noopener noreferrer">click here to open in Box.com</a>.</p>
						</noscript>
					</div>
				</section>
			</div>
		</section>
	</div>
</div>

<!-- Have Questions Section -->
<HaveQuestionsSection 
	email="support@revolutiontradingpros.com" 
	phone="8002668659" 
	phoneDisplay="(800) 266-8659" 
/>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * APPLE PRINCIPAL ENGINEER ICT 7 GRADE - RESPONSIVE DESIGN SYSTEM
	 * Implements: Apple HIG, WCAG 2.2 AA, Fluid Typography, Safe Areas
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* ─────────────────────────────────────────────────────────────────────────────
	 * DESIGN TOKENS - CSS Custom Properties (Systematic Breakpoint Tokens)
	 * ───────────────────────────────────────────────────────────────────────────── */
	:root {
		/* ═══ Color Tokens ═══ */
		--color-background-primary: #FFFFFF;
		--color-background-secondary: #EFEFEF;
		--color-background-video: #0a2335;
		--color-text-primary: #666666;
		--color-text-heading: #4a4a4a;
		--color-text-light: #999999;
		--color-text-inverse: #ffffff;
		--color-text-inverse-muted: #d5d5d5;
		--color-link: #1e73be;
		--color-link-hover: #000000;
		--color-border: #999999;
		--color-border-light: #dddddd;

		/* ═══ Spacing Tokens (8px base grid - Apple HIG) ═══ */
		--space-1: 0.25rem;   /* 4px */
		--space-2: 0.5rem;    /* 8px */
		--space-3: 0.75rem;   /* 12px */
		--space-4: 1rem;      /* 16px */
		--space-5: 1.25rem;   /* 20px */
		--space-6: 1.5rem;    /* 24px */
		--space-8: 2rem;      /* 32px */
		--space-10: 2.5rem;   /* 40px */
		--space-12: 3rem;     /* 48px */
		--space-15: 3.75rem;  /* 60px */

		/* ═══ Fluid Typography (clamp with rem + vw for accessibility) ═══ */
		--font-size-xs: clamp(0.75rem, 0.7rem + 0.25vw, 0.875rem);
		--font-size-sm: clamp(0.875rem, 0.8rem + 0.375vw, 1rem);
		--font-size-base: clamp(1rem, 0.925rem + 0.375vw, 1.125rem);
		--font-size-lg: clamp(1.1rem, 1rem + 0.5vw, 1.4rem);
		--font-size-xl: clamp(1.2rem, 1rem + 1vw, 1.75rem);
		--font-size-2xl: clamp(1.5rem, 1.25rem + 1.25vw, 2rem);
		--font-size-3xl: clamp(1.75rem, 1.5rem + 1.5vw, 2.5rem);

		/* ═══ Line Heights (optimal readability) ═══ */
		--line-height-tight: 1.2;
		--line-height-normal: 1.5;
		--line-height-relaxed: 1.6;

		/* ═══ Touch Targets (Apple HIG: 44pt minimum) ═══ */
		--touch-target-min: 44px;

		/* ═══ Border Radius ═══ */
		--radius-sm: 4px;
		--radius-md: 8px;
		--radius-lg: 12px;

		/* ═══ Transitions (respect reduced motion) ═══ */
		--transition-fast: 150ms ease-out;
		--transition-normal: 250ms ease-out;
		--transition-slow: 350ms ease-out;

		/* ═══ Safe Area Insets (notched devices) ═══ */
		--safe-area-inset-top: env(safe-area-inset-top, 0px);
		--safe-area-inset-right: env(safe-area-inset-right, 0px);
		--safe-area-inset-bottom: env(safe-area-inset-bottom, 0px);
		--safe-area-inset-left: env(safe-area-inset-left, 0px);

		/* ═══ Container Widths ═══ */
		--container-sm: 640px;
		--container-md: 768px;
		--container-lg: 1024px;
		--container-xl: 1080px;
		--container-2xl: 1366px;

		/* ═══ Z-Index Scale ═══ */
		--z-base: 0;
		--z-elevated: 1;
		--z-overlay: 10;
		--z-modal: 100;
	}

	/* ─────────────────────────────────────────────────────────────────────────────
	 * REDUCED MOTION PREFERENCES (WCAG 2.2 AA Compliance)
	 * ───────────────────────────────────────────────────────────────────────────── */
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

	/* ─────────────────────────────────────────────────────────────────────────────
	 * BASE STYLES - Global Resets with Logical Properties
	 * ───────────────────────────────────────────────────────────────────────────── */
	:global(html) {
		font-size: 100%; /* Respect user font size preferences */
		-webkit-text-size-adjust: 100%;
		text-size-adjust: 100%;
		scroll-behavior: smooth;
	}

	:global(body) {
		background-color: var(--color-background-primary) !important;
		color: var(--color-text-primary);
		font-family: "Open Sans", -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
		font-size: var(--font-size-base);
		line-height: var(--line-height-normal);
		min-block-size: 100dvh; /* Dynamic viewport height */
		padding-inline: var(--safe-area-inset-left) var(--safe-area-inset-right);
		padding-block-end: var(--safe-area-inset-bottom);
		-webkit-font-smoothing: antialiased;
		-moz-osx-font-smoothing: grayscale;
	}

	:global(a),
	:global(a:visited) {
		color: var(--color-link);
		text-decoration: none;
		transition: color var(--transition-fast);
	}

	:global(a:hover),
	:global(a:focus) {
		color: var(--color-link-hover);
	}

	:global(a:focus-visible) {
		outline: 2px solid var(--color-link);
		outline-offset: 2px;
		border-radius: var(--radius-sm);
	}

	/* ─────────────────────────────────────────────────────────────────────────────
	 * PAGE WRAPPER
	 * ───────────────────────────────────────────────────────────────────────────── */
	#page {
		max-inline-size: 100%;
		margin: 0;
		padding: 0;
	}

	#content {
		max-inline-size: 100%;
	}

	/* ─────────────────────────────────────────────────────────────────────────────
	 * SECTION STRUCTURE - Fluid Spacing
	 * ───────────────────────────────────────────────────────────────────────────── */
	.cpost-section {
		padding-block: clamp(var(--space-10), 5vw, var(--space-15));
	}

	.cpost-section .section-inner {
		max-inline-size: min(var(--container-xl), 100% - var(--space-10));
		margin-inline: auto;
		padding-inline: var(--space-5);
	}

	/* ─────────────────────────────────────────────────────────────────────────────
	 * CLASS INFO SECTION (Non-Member Upsell)
	 * ───────────────────────────────────────────────────────────────────────────── */
	#class-info {
		padding-block: var(--space-15);
	}

	#non-member-class-upsell {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		text-align: center;
		padding: clamp(var(--space-4), 3vw, var(--space-5));
		background-color: var(--color-background-secondary);
		max-inline-size: min(1020px, 100%);
		min-block-size: 148px;
		margin-inline: auto;
		inline-size: 100%;
		box-sizing: border-box;
	}

	#non-member-class-title h1 {
		font-size: var(--font-size-3xl);
		font-weight: 700;
		color: var(--color-text-primary);
		margin-block-end: var(--space-3);
		line-height: var(--line-height-tight);
	}

	#non-member-class-title h3 {
		font-size: var(--font-size-lg);
		font-weight: 400;
		color: var(--color-text-light);
		margin-block-end: var(--space-4);
	}

	#non-member-class-description {
		color: var(--color-text-primary);
		font-size: var(--font-size-base);
		line-height: var(--line-height-relaxed);
	}

	/* ─────────────────────────────────────────────────────────────────────────────
	 * CLASS RECORDINGS SECTION
	 * ───────────────────────────────────────────────────────────────────────────── */
	#class-recordings {
		padding-block-start: 0;
	}

	.cpost-content-block.class-content-block {
		padding: 0 clamp(var(--space-5), 4vw, var(--space-10)) clamp(var(--space-10), 4vw, var(--space-10));
		border-radius: 0;
		background-color: var(--color-background-secondary);
	}

	/* ─────────────────────────────────────────────────────────────────────────────
	 * VIDEO SECTION - Dark Blue Background with 25px border all sides
	 * ───────────────────────────────────────────────────────────────────────────── */
	.current-vid {
		inline-size: 100%;
		background-color: var(--color-background-video);
		padding: clamp(var(--space-3), 2.5vw, var(--space-6));
	}

	.class-player-header {
		color: var(--color-text-inverse);
		font-size: var(--font-size-xl);
		font-weight: 600;
		margin-block-end: var(--space-3);
		text-align: center;
	}

	.current-title {
		color: var(--color-text-inverse-muted);
		font-size: var(--font-size-lg);
		font-weight: 500;
		margin-block-end: var(--space-4);
		text-align: center;
	}

	.class-video-description {
		margin-block-end: var(--space-3);
	}

	.class-video-container {
		position: relative;
		background: #000;
		overflow: hidden;
		border: 1px solid var(--color-border);
		cursor: pointer;
	}

	.class-video-container.current {
		inline-size: 100%;
		display: flex;
		z-index: var(--z-elevated);
	}

	.class-video-container video {
		inline-size: 100%;
		block-size: auto;
		display: block;
		aspect-ratio: 16 / 9;
		object-fit: contain;
	}

	.video-overlay {
		background-color: rgba(0, 0, 0, 0.269);
	}

	.video-overlay h3 {
		display: none;
	}

	/* ─────────────────────────────────────────────────────────────────────────────
	 * EMPTY SECTIONS - WordPress Placeholders
	 * ───────────────────────────────────────────────────────────────────────────── */
	#classroom-access,
	#class-elearning {
		padding: 0;
	}

	#classroom-access .section-inner,
	#class-elearning .section-inner {
		display: none;
	}

	/* ─────────────────────────────────────────────────────────────────────────────
	 * DOWNLOADS SECTION - Exact Match 1080x512 centered (Screenshot Perfect)
	 * ───────────────────────────────────────────────────────────────────────────── */
	#dl-rp-row {
		padding-block: var(--space-10);
	}

	#dl-rp-row .section-inner {
		display: flex;
		flex-direction: column;
		align-items: center;
		max-inline-size: var(--container-xl);
		margin-inline: auto;
		padding-inline: var(--space-5);
	}

	.class-subsection {
		flex: 1 1 auto;
		background-color: var(--color-background-secondary);
		padding: clamp(var(--space-5), 3vw, var(--space-6));
		inline-size: min(var(--container-max-width, 1080px), 100%);
		max-inline-size: 1080px;
		min-block-size: var(--downloads-height, 512px);
		box-sizing: border-box;
	}

	#class-downloads {
		margin-inline: auto;
		inline-size: 100%;
		box-sizing: border-box;
	}

	#class-downloads h2 {
		font-size: var(--font-size-2xl);
		font-weight: 400;
		color: var(--color-text-heading);
		margin: 0 0 var(--space-6) 0;
		text-align: start;
		line-height: var(--line-height-tight);
	}

	.class-downloads-container {
		inline-size: 100%;
		max-inline-size: 100%;
		block-size: var(--iframe-height, 518px);
		box-sizing: border-box;
		background: var(--color-background-secondary);
	}

	.class-downloads-container iframe {
		inline-size: 100%;
		block-size: 100%;
		border: none;
		display: block;
	}

	/* ICT 11 Fix: Fallback for iframe errors */
	.iframe-fallback {
		padding: var(--space-5);
		text-align: center;
		background-color: var(--color-background-secondary);
		border: 1px solid var(--color-border-light);
		border-radius: var(--radius-sm);
		color: var(--color-text-primary);
	}

	.iframe-fallback a {
		color: var(--color-link);
		text-decoration: underline;
		/* Touch target compliance */
		display: inline-block;
		min-block-size: var(--touch-target-min);
		line-height: var(--touch-target-min);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * RESPONSIVE BREAKPOINTS - Apple HIG Aligned
	 * Mobile-first with systematic breakpoints
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* ─────────────────────────────────────────────────────────────────────────────
	 * MOBILE SMALL (< 428px) - iPhone SE, Mini
	 * ───────────────────────────────────────────────────────────────────────────── */
	@media (max-width: 427px) {
		.cpost-section .section-inner {
			padding-inline: var(--space-4);
		}

		#non-member-class-upsell {
			padding: var(--space-4);
			min-block-size: auto;
		}

		.class-subsection {
			inline-size: 100%;
			min-block-size: 350px;
			padding: var(--space-4);
		}

		.class-downloads-container {
			block-size: 300px;
		}

		.current-vid {
			padding: var(--space-3);
		}
	}

	/* ─────────────────────────────────────────────────────────────────────────────
	 * MOBILE (428px - 743px) - iPhone Pro Max
	 * ───────────────────────────────────────────────────────────────────────────── */
	@media (min-width: 428px) and (max-width: 743px) {
		.class-subsection {
			inline-size: 100%;
			min-block-size: 400px;
		}

		.class-downloads-container {
			block-size: 350px;
		}
	}

	/* ─────────────────────────────────────────────────────────────────────────────
	 * TABLET (744px - 1023px) - iPad Mini, Air
	 * ───────────────────────────────────────────────────────────────────────────── */
	@media (min-width: 744px) and (max-width: 1023px) {
		.cpost-section .section-inner {
			padding-inline: var(--space-8);
		}

		.class-subsection {
			inline-size: min(720px, 100%);
			min-block-size: 450px;
		}

		.class-downloads-container {
			block-size: 400px;
		}

		.current-vid {
			padding: var(--space-6);
		}

		#dl-rp-row .section-inner {
			flex-direction: column;
		}
	}

	/* ─────────────────────────────────────────────────────────────────────────────
	 * DESKTOP (1024px - 1365px) - iPad Pro, Laptops
	 * ───────────────────────────────────────────────────────────────────────────── */
	@media (min-width: 1024px) and (max-width: 1365px) {
		.cpost-section .section-inner {
			padding-inline: var(--space-10);
		}

		.class-subsection {
			inline-size: min(1080px, 100%);
			block-size: 512px;
			min-block-size: 512px;
		}

		.class-downloads-container {
			block-size: 518px;
		}

		#dl-rp-row .section-inner {
			flex-direction: row;
		}

		#class-downloads {
			margin-inline-end: var(--space-5);
		}
	}

	/* ─────────────────────────────────────────────────────────────────────────────
	 * DESKTOP LARGE (1366px+) - Large Displays
	 * ───────────────────────────────────────────────────────────────────────────── */
	@media (min-width: 1366px) {
		.cpost-section .section-inner {
			padding-inline: 0;
		}

		.class-subsection {
			inline-size: 1080px;
			max-inline-size: 1080px;
			block-size: 512px;
			min-block-size: 512px;
		}

		.class-downloads-container {
			block-size: 518px;
		}

		.class-downloads-container iframe {
			block-size: 518px;
		}

		#dl-rp-row .section-inner {
			flex-direction: row;
		}

		#class-downloads {
			margin-inline-end: var(--space-5);
		}
	}

	/* ─────────────────────────────────────────────────────────────────────────────
	 * HIGH DPI / RETINA DISPLAYS
	 * ───────────────────────────────────────────────────────────────────────────── */
	@media (-webkit-min-device-pixel-ratio: 2), (min-resolution: 192dpi) {
		/* Sharper borders for retina */
		.class-video-container {
			border-width: 0.5px;
		}
	}

	/* ─────────────────────────────────────────────────────────────────────────────
	 * LANDSCAPE ORIENTATION ADJUSTMENTS
	 * ───────────────────────────────────────────────────────────────────────────── */
	@media (orientation: landscape) and (max-height: 500px) {
		.cpost-section {
			padding-block: var(--space-6);
		}

		#class-info {
			padding-block: var(--space-8);
		}

		#non-member-class-upsell {
			min-block-size: auto;
		}
	}

	/* ─────────────────────────────────────────────────────────────────────────────
	 * PRINT STYLES
	 * ───────────────────────────────────────────────────────────────────────────── */
	@media print {
		:global(body) {
			background-color: white !important;
		}

		.current-vid {
			background-color: #f0f0f0 !important;
			color: black !important;
		}

		.class-player-header,
		.current-title {
			color: black !important;
		}

		.class-downloads-container iframe {
			display: none;
		}

		.class-downloads-container::after {
			content: "Visit simplertrading.app.box.com for downloads";
			display: block;
			padding: var(--space-5);
			text-align: center;
		}
	}

	/* ─────────────────────────────────────────────────────────────────────────────
	 * CONTAINER QUERIES (Progressive Enhancement)
	 * ───────────────────────────────────────────────────────────────────────────── */
	@supports (container-type: inline-size) {
		#dl-rp-row .section-inner {
			container-type: inline-size;
			container-name: downloads-container;
		}

		@container downloads-container (min-width: 800px) {
			.class-subsection {
				inline-size: min(1080px, 100%);
			}
		}

		@container downloads-container (min-width: 1100px) {
			.class-subsection {
				inline-size: 1080px;
				block-size: 512px;
			}

			.class-downloads-container {
				block-size: 518px;
			}
		}
	}

	/* ─────────────────────────────────────────────────────────────────────────────
	 * DARK MODE SUPPORT (Future Enhancement)
	 * ───────────────────────────────────────────────────────────────────────────── */
	@media (prefers-color-scheme: dark) {
		/* Ready for dark mode implementation */
		/* Currently maintains light theme as per WordPress design */
	}

	/* ─────────────────────────────────────────────────────────────────────────────
	 * HIGH CONTRAST MODE (Accessibility)
	 * ───────────────────────────────────────────────────────────────────────────── */
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
</style>
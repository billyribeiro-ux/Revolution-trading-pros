<svelte:head>
	<title>Quickstart To Precision Trading - Revolution Trading Pros</title>
	<meta property="og:locale" content="en_US" />
	<meta property="og:type" content="article" />
	<meta property="og:title" content="Quickstart To Precision Trading" />
	<meta property="og:site_name" content="Simpler Trading" />
	<meta name="viewport" content="width=device-width, initial-scale=1.0, viewport-fit=cover" />
</svelte:head>

<script lang="ts">
	import { onMount } from 'svelte';
	import DashboardBreadcrumbs from '$lib/components/dashboard/DashboardBreadcrumbs.svelte';
	import HaveQuestionsSection from '$lib/components/sections/HaveQuestionsSection.svelte';

	// ═══════════════════════════════════════════════════════════════════════════
	// SVELTE 5 RUNES - Reactive State Management
	// ═══════════════════════════════════════════════════════════════════════════
	let viewportWidth = $state(0);
	let prefersReducedMotion = $state(false);
	let isClient = $state(false);

	const breakpoint = $derived.by(() => {
		if (viewportWidth === 0) return 'mobile';
		if (viewportWidth < 428) return 'mobile-small';
		if (viewportWidth < 744) return 'mobile';
		if (viewportWidth < 1024) return 'tablet';
		if (viewportWidth < 1366) return 'desktop';
		return 'desktop-large';
	});

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
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* ─────────────────────────────────────────────────────────────────────────────
	 * DESIGN TOKENS
	 * ───────────────────────────────────────────────────────────────────────────── */
	:root {
		/* ═══ Color Tokens - CORRECTED ═══ */
		--color-page-bg: #EFEFEF;
		--color-content-bg: #FFFFFF;
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

		/* ═══ Spacing Tokens (8px grid) ═══ */
		--space-1: 0.25rem;
		--space-2: 0.5rem;
		--space-3: 0.75rem;
		--space-4: 1rem;
		--space-5: 1.25rem;
		--space-6: 1.5rem;
		--space-8: 2rem;
		--space-10: 2.5rem;
		--space-12: 3rem;
		--space-15: 3.75rem;

		/* ═══ Fluid Typography ═══ */
		--font-size-sm: clamp(0.875rem, 0.8rem + 0.375vw, 1rem);
		--font-size-base: clamp(1rem, 0.925rem + 0.375vw, 1.125rem);
		--font-size-lg: clamp(1.1rem, 1rem + 0.5vw, 1.4rem);
		--font-size-xl: clamp(1.2rem, 1rem + 1vw, 1.75rem);
		--font-size-2xl: clamp(1.5rem, 1.25rem + 1.25vw, 2rem);
		--font-size-3xl: clamp(1.75rem, 1.5rem + 1.5vw, 2.5rem);

		/* ═══ Safe Areas ═══ */
		--safe-area-inset-top: env(safe-area-inset-top, 0px);
		--safe-area-inset-right: env(safe-area-inset-right, 0px);
		--safe-area-inset-bottom: env(safe-area-inset-bottom, 0px);
		--safe-area-inset-left: env(safe-area-inset-left, 0px);

		/* ═══ Touch Targets ═══ */
		--touch-target-min: 44px;
	}

	/* ─────────────────────────────────────────────────────────────────────────────
	 * REDUCED MOTION
	 * ───────────────────────────────────────────────────────────────────────────── */
	@media (prefers-reduced-motion: reduce) {
		*, *::before, *::after {
			animation-duration: 0.01ms !important;
			animation-iteration-count: 1 !important;
			transition-duration: 0.01ms !important;
			scroll-behavior: auto !important;
		}
	}

	/* ─────────────────────────────────────────────────────────────────────────────
	 * BASE STYLES
	 * ───────────────────────────────────────────────────────────────────────────── */
	:global(html) {
		font-size: 100%;
		-webkit-text-size-adjust: 100%;
		text-size-adjust: 100%;
		scroll-behavior: smooth;
	}

	:global(html),
	:global(body) {
		background-color: #EFEFEF !important;
		color: #666666;
		font-family: "Open Sans", -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
		font-size: 1rem;
		line-height: 1.5;
		min-height: 100dvh;
		margin: 0;
		padding: 0;
		-webkit-font-smoothing: antialiased;
		-moz-osx-font-smoothing: grayscale;
	}

	:global(a), :global(a:visited) {
		color: var(--color-link);
		text-decoration: none;
		transition: color 150ms ease-out;
	}

	:global(a:hover), :global(a:focus) {
		color: var(--color-link-hover);
	}

	:global(a:focus-visible) {
		outline: 2px solid var(--color-link);
		outline-offset: 2px;
		border-radius: 4px;
	}

	/* ─────────────────────────────────────────────────────────────────────────────
	 * PAGE WRAPPER
	 * ───────────────────────────────────────────────────────────────────────────── */
	#page {
		max-width: 100%;
		margin: 0;
		padding: 0;
	}

	#content {
		max-width: 100%;
	}

	/* ─────────────────────────────────────────────────────────────────────────────
	 * SECTION STRUCTURE
	 * ───────────────────────────────────────────────────────────────────────────── */
	.cpost-section {
		padding: clamp(var(--space-10), 5vw, var(--space-15)) 0;
	}

	.cpost-section .section-inner {
		max-width: min(1100px, 100% - var(--space-10));
		margin: 0 auto;
		padding: 0 var(--space-5);
	}

	/* ─────────────────────────────────────────────────────────────────────────────
	 * CLASS INFO SECTION
	 * ───────────────────────────────────────────────────────────────────────────── */
	#class-info {
		padding: var(--space-15) 0;
	}

	#non-member-class-upsell {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		text-align: center;
		padding: 30px 40px;
		background-color: #FFFFFF !important;
		max-width: 1020px;
		min-height: 148px;
		margin: 0 auto;
		width: 100%;
		box-sizing: border-box;
	}

	#non-member-class-title h1 {
		font-size: var(--font-size-3xl);
		font-weight: 700;
		color: var(--color-text-primary);
		margin: 0 0 var(--space-3) 0;
		line-height: 1.2;
	}

	#non-member-class-title h3 {
		font-size: var(--font-size-lg);
		font-weight: 400;
		color: var(--color-text-light);
		margin: 0 0 var(--space-4) 0;
	}

	#non-member-class-description {
		color: var(--color-text-primary);
		font-size: var(--font-size-base);
		line-height: 1.6;
	}

	/* ─────────────────────────────────────────────────────────────────────────────
	 * CLASS RECORDINGS SECTION
	 * ───────────────────────────────────────────────────────────────────────────── */
	#class-recordings {
		padding-top: 0;
	}

	.cpost-content-block.class-content-block {
		padding: 0 clamp(var(--space-5), 4vw, var(--space-10)) clamp(var(--space-10), 4vw, var(--space-10));
		border-radius: 0;
		background-color: #FFFFFF;
	}

	/* ─────────────────────────────────────────────────────────────────────────────
	 * VIDEO SECTION - 25px padding all sides
	 * ───────────────────────────────────────────────────────────────────────────── */
	.current-vid {
		width: 100%;
		background-color: var(--color-video-bg);
		padding: clamp(var(--space-3), 2.5vw, var(--space-6));
	}

	.class-player-header {
		color: var(--color-text-inverse);
		font-size: var(--font-size-xl);
		font-weight: 600;
		margin: 0 0 var(--space-3) 0;
		text-align: center;
	}

	.current-title {
		color: var(--color-text-inverse-muted);
		font-size: var(--font-size-lg);
		font-weight: 500;
		margin: 0 0 var(--space-4) 0;
		text-align: center;
	}

	.class-video-description {
		margin: 0 0 var(--space-3) 0;
	}

	.class-video-container {
		position: relative;
		background: #000;
		overflow: hidden;
		border: 1px solid var(--color-border);
	}

	.class-video-container.current {
		width: 100%;
		display: flex;
	}

	.class-video-container video {
		width: 100%;
		height: auto;
		display: block;
		aspect-ratio: 16 / 9;
		object-fit: contain;
	}

	.video-overlay {
		display: none;
	}

	.video-overlay h3 {
		display: none;
	}

	/* ─────────────────────────────────────────────────────────────────────────────
	 * EMPTY SECTIONS
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
	 * CLASS DOWNLOADS SECTION
	 * ───────────────────────────────────────────────────────────────────────────── */
	#dl-rp-row {
		padding: 40px 0;
		background-color: #EFEFEF;
	}

	#dl-rp-row .section-inner {
		display: flex;
		flex-direction: column;
		align-items: center;
		max-width: 1100px;
		margin: 0 auto;
		padding: 0 20px;
	}

	#class-downloads {
		background-color: #FFFFFF;
		padding: 25px;
		width: 100%;
		max-width: 1080px;
		box-sizing: border-box;
		margin: 0 auto;
	}

	#class-downloads h2 {
		font-size: 1.5rem;
		font-weight: 400;
		color: #333333;
		margin: 0 0 20px 0;
		text-align: left;
		line-height: 1.2;
		font-family: "Open Sans", -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
	}

	.class-downloads-container {
		width: 100%;
		height: 400px;
		box-sizing: border-box;
		background: #FFFFFF;
		overflow: hidden;
	}

	.class-downloads-container iframe {
		width: 100%;
		height: 100%;
		border: none;
		display: block;
	}

	.iframe-fallback {
		padding: 20px;
		text-align: center;
		background-color: #FFFFFF;
		border: 1px solid #dddddd;
		border-radius: 4px;
		color: #666666;
	}

	.iframe-fallback a {
		color: #1e73be;
		text-decoration: underline;
	}

	/* ─────────────────────────────────────────────────────────────────────────────
	 * RESPONSIVE BREAKPOINTS - Page-specific (non-downloads)
	 * ───────────────────────────────────────────────────────────────────────────── */

	/* Mobile Small (< 428px) */
	@media (max-width: 427px) {
		.cpost-section .section-inner {
			padding: 0 var(--space-4);
		}

		#non-member-class-upsell {
			padding: var(--space-4);
			min-height: auto;
		}

		.current-vid {
			padding: var(--space-3);
		}
	}

	/* Tablet (744px - 1023px) */
	@media (min-width: 744px) and (max-width: 1023px) {
		.cpost-section .section-inner {
			padding: 0 var(--space-8);
		}

		.current-vid {
			padding: var(--space-6);
		}
	}

	/* Desktop (1024px - 1365px) */
	@media (min-width: 1024px) and (max-width: 1365px) {
		.cpost-section .section-inner {
			padding: 0 var(--space-10);
		}
	}

	/* Desktop Large (1366px+) */
	@media (min-width: 1366px) {
		.cpost-section .section-inner {
			padding: 0;
		}
	}

	/* High DPI */
	@media (-webkit-min-device-pixel-ratio: 2), (min-resolution: 192dpi) {
		.class-video-container {
			border-width: 0.5px;
		}
	}

	/* Landscape Mobile */
	@media (orientation: landscape) and (max-height: 500px) {
		.cpost-section {
			padding: var(--space-6) 0;
		}

		#class-info {
			padding: var(--space-8) 0;
		}

		#non-member-class-upsell {
			min-height: auto;
		}
	}

	/* Print */
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
	}

	/* High Contrast */
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
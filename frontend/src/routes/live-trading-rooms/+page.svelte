<script lang="ts">
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import RoomsGrid from './_sections/RoomsGrid.svelte';

	// GSAP types for TypeScript (actual imports are dynamic for SSR safety)
	type GSAPInstance = typeof import('gsap').gsap;

	/**
	 * Svelte 5 Runes & SSR/SSG Pattern
	 * PE7 FIX 2026-05-23: Properly typed data structures replacing `any`
	 */
	interface PricePlan {
		monthly: number;
		quarterly: number;
		annual: number;
	}

	interface TradingRoom {
		id: string;
		iconType: 'volatility' | 'trend' | 'growth';
		name: string;
		tagline: string;
		description: string;
		liveCount: number;
		features: string[];
		price: PricePlan;
		accent: 'cyan' | 'emerald' | 'amber';
		badge: string;
	}

	interface Benefit {
		iconType: 'analysis' | 'radar' | 'strategy' | 'network';
		title: string;
		desc: string;
	}

	interface TickerSymbol {
		sym: string;
		price: string;
		change: string;
		up: boolean;
	}

	interface OpenGraphImage {
		url: string;
		width: number;
		height: number;
		alt: string;
		type: string;
	}

	interface OpenGraphArticle {
		author: string;
		publishedTime: string;
		modifiedTime: string;
		section: string;
		tag: string;
	}

	interface OpenGraphData {
		title: string;
		description: string;
		type: string;
		url: string;
		images: OpenGraphImage[];
		siteName: string;
		locale: string;
		article: OpenGraphArticle;
	}

	interface TwitterData {
		card: string;
		title: string;
		description: string;
		images: string[];
		site: string;
		creator: string;
		domain: string;
	}

	interface SEOData {
		title: string;
		description: string;
		keywords: string;
		canonical: string;
		author: string;
		publisher: string;
		openGraph: OpenGraphData;
		twitter: TwitterData;
	}

	interface PageData {
		rooms: TradingRoom[];
		benefits: Benefit[];
		symbols: TickerSymbol[];
		seo: SEOData;
	}

	interface Props {
		data: PageData;
	}
	let { data }: Props = $props();

	// Use server-loaded data for SSR/SSG
	let rooms = $derived(data.rooms);
	let benefits = $derived(data.benefits);
	let symbols = $derived(data.symbols);
	let seo = $derived(data.seo);
	let tickerItems = $derived([...symbols, ...symbols, ...symbols, ...symbols]);

	/**
	 * Hero Chart Data Generation (For SVG Animation)
	 */
	const chartPoints = Array.from({ length: 40 }, (_, i) => {
		const x = i * 25;
		// Simple random walk for visual flair
		const y = 100 + Math.sin(i * 0.2) * 40 + (Math.random() - 0.5) * 30 - i * 2;
		return `${x},${y}`;
	}).join(' ');

	// ctaElement is read by GSAP ScrollTrigger; other sections animate via [data-gsap] attribute selectors.
	let ctaElement: HTMLElement | undefined;

	function attachCtaElement(element: HTMLElement) {
		ctaElement = element;

		return () => {
			if (ctaElement === element) {
				ctaElement = undefined;
			}
		};
	}

	/**
	 * GSAP Animation Controller - Svelte 5 / Dec 2025 Pattern
	 * Uses dynamic imports for SSR safety and proper cleanup
	 */
	onMount(() => {
		// SSR-safe guard - only run animations in browser
		if (!browser) return;

		// Store references for cleanup
		let gsapInstance: GSAPInstance | null = null;
		let scrollTriggerInstance: typeof import('gsap/ScrollTrigger').ScrollTrigger | null = null;
		let animationContext: ReturnType<GSAPInstance['context']> | null = null;

		// Async IIFE pattern for dynamic imports (Svelte 5 SSR-safe)
		(async () => {
			try {
				// Dynamic imports - required for SSR safety in SvelteKit
				const [gsapModule, scrollTriggerModule] = await Promise.all([
					import('gsap'),
					import('gsap/ScrollTrigger')
				]);

				gsapInstance = gsapModule.gsap;
				scrollTriggerInstance = scrollTriggerModule.ScrollTrigger;

				// Register plugins
				gsapInstance.registerPlugin(scrollTriggerInstance);

				// Create GSAP context for proper cleanup (GSAP 3.12+ pattern)
				animationContext = gsapInstance.context(() => {
					// PE7 FIX 2026-05-23: Type guard to satisfy strict TypeScript
					if (!gsapInstance || !scrollTriggerInstance) return;
					const gsap = gsapInstance;
					const ScrollTrigger = scrollTriggerInstance;

					// ===== HERO SECTION ANIMATIONS =====
					const heroTl = gsap.timeline({
						defaults: { ease: 'power3.out', duration: 1 }
					});

					// Set initial states to prevent flash of unstyled content
					gsap.set(['.hero-title', '.hero-subtitle', '.hero-grid-plane', '.hero-chart'], {
						opacity: 0
					});

					heroTl
						.fromTo(
							'.hero-title',
							{ opacity: 0, y: 60, filter: 'blur(10px)' },
							{ opacity: 1, y: 0, filter: 'blur(0px)', duration: 1.2 }
						)
						.fromTo(
							'.hero-subtitle',
							{ opacity: 0, y: 40 },
							{ opacity: 1, y: 0, duration: 1 },
							'-=0.8'
						)
						.fromTo(
							'.hero-grid-plane',
							{ opacity: 0, scale: 0.9 },
							{ opacity: 0.4, scale: 1, duration: 2 },
							'-=1.5'
						)
						.fromTo(
							'.hero-chart',
							{ opacity: 0, scale: 0.95, y: 20 },
							{ opacity: 0.6, scale: 1, y: 0, duration: 1.5 },
							'-=1'
						);

					// ===== FLOATING PARTICLES =====
					gsap.fromTo(
						'.particle',
						{ opacity: 0, scale: 0, rotation: -180 },
						{
							opacity: 0.8,
							scale: 1,
							rotation: 0,
							duration: 1.5,
							stagger: { each: 0.1, from: 'random' },
							ease: 'back.out(1.7)'
						}
					);

					// Continuous floating animation for particles
					gsap.to('.particle', {
						y: 'random(-20, 20)',
						x: 'random(-10, 10)',
						duration: 'random(3, 5)',
						repeat: -1,
						yoyo: true,
						ease: 'sine.inOut',
						stagger: { each: 0.2, from: 'random' }
					});

					// ===== 6 FLOATING CTA BUTTONS ANIMATION =====
					// Each button enters from a different direction with unique timing
					const btnConfigs = [
						{ selector: '.hero-float-btn-1', from: { x: -100, y: 0, opacity: 0 }, delay: 0 },
						{ selector: '.hero-float-btn-2', from: { x: 100, y: 0, opacity: 0 }, delay: 0.1 },
						{ selector: '.hero-float-btn-3', from: { x: 0, y: -50, opacity: 0 }, delay: 0.2 },
						{ selector: '.hero-float-btn-4', from: { x: -80, y: 30, opacity: 0 }, delay: 0.3 },
						{ selector: '.hero-float-btn-5', from: { x: 80, y: -30, opacity: 0 }, delay: 0.4 },
						{ selector: '.hero-float-btn-6', from: { x: 0, y: 50, opacity: 0 }, delay: 0.5 }
					];

					btnConfigs.forEach(({ selector, from, delay }) => {
						gsap.fromTo(selector, from, {
							x: 0,
							y: 0,
							opacity: 1,
							duration: 0.8,
							delay: 1.5 + delay,
							ease: 'back.out(1.7)'
						});
					});

					// Subtle continuous float for buttons
					gsap.to('.hero-float-btn', {
						y: 'random(-8, 8)',
						duration: 'random(2, 4)',
						repeat: -1,
						yoyo: true,
						ease: 'sine.inOut',
						stagger: { each: 0.3, from: 'random' }
					});

					// ===== ROOM CARDS SCROLL ANIMATION =====
					ScrollTrigger.batch('.room-card', {
						start: 'top 85%',
						onEnter: (batch) => {
							gsap.fromTo(
								batch,
								{ opacity: 0, y: 80, scale: 0.95 },
								{
									opacity: 1,
									y: 0,
									scale: 1,
									duration: 0.8,
									stagger: 0.15,
									ease: 'power2.out'
								}
							);
						},
						once: true
					});

					// ===== BENEFITS CARDS SCROLL ANIMATION =====
					ScrollTrigger.batch('.benefit-card', {
						start: 'top 90%',
						onEnter: (batch) => {
							gsap.fromTo(
								batch,
								{ opacity: 0, y: 60, scale: 0.9 },
								{
									opacity: 1,
									y: 0,
									scale: 1,
									duration: 0.7,
									stagger: 0.1,
									ease: 'power2.out'
								}
							);
						},
						once: true
					});

					// ===== CTA SECTION SCROLL ANIMATION =====
					if (ctaElement) {
						const triggerElement = ctaElement;
						ScrollTrigger.create({
							trigger: triggerElement,
							start: 'top 85%',
							onEnter: () => {
								gsap.fromTo(
									triggerElement,
									{ scale: 0.95, opacity: 0, y: 30 },
									{
										scale: 1,
										opacity: 1,
										y: 0,
										duration: 0.8,
										ease: 'power2.out'
									}
								);
							},
							once: true
						});
					}

					// ===== TICKER ANIMATION =====
					const tickerTrack = document.querySelector('.ticker-track');
					if (tickerTrack) {
						gsap.to(tickerTrack, {
							xPercent: -50,
							duration: 30,
							ease: 'none',
							repeat: -1
						});
					}

					// ===== SECTION HEADERS ANIMATION =====
					gsap.utils.toArray('.section-header').forEach((header) => {
						ScrollTrigger.create({
							trigger: header as Element,
							start: 'top 85%',
							onEnter: () => {
								gsap.fromTo(
									header as Element,
									{ opacity: 0, y: 40 },
									{ opacity: 1, y: 0, duration: 0.8, ease: 'power2.out' }
								);
							},
							once: true
						});
					});
				}); // End of GSAP context
			} catch (error) {
				console.warn('[Live Trading Rooms] GSAP initialization failed:', error);
			}
		})();

		// Cleanup function - Svelte 5 pattern
		return () => {
			// Use GSAP context revert for clean cleanup (GSAP 3.12+)
			// This only reverts animations created within this component's context
			if (animationContext) {
				animationContext.revert();
			}
			// Note: Removed global fallback cleanup that was killing ALL ScrollTriggers
			// gsap.context().revert() handles proper scoped cleanup
		};
	});
</script>

<svelte:head>
	<!-- Language & Locale -->
	<html lang="en"></html>
	<meta charset="UTF-8" />
	<meta name="geo.region" content="US" />
	<meta name="geo.placename" content="United States" />

	<!-- Basic Meta Tags - Using server-loaded SEO data -->
	<title>{seo.title}</title>
	<meta name="description" content={seo.description} />
	<meta name="keywords" content={seo.keywords} />
	<meta name="author" content="Revolution Trading Pros" />
	<meta name="robots" content="index, follow" />
	<link rel="canonical" href={seo.canonical} />

	<!-- Open Graph Meta Tags - Using server-loaded data -->
	<meta property="og:title" content={seo.openGraph.title} />
	<meta property="og:description" content={seo.openGraph.description} />
	<meta property="og:type" content={seo.openGraph.type} />
	<meta property="og:url" content={seo.openGraph.url} />
	<meta property="og:site_name" content={seo.openGraph.siteName} />
	<meta property="og:image" content={seo.openGraph.images[0].url} />
	<meta property="og:image:width" content={String(seo.openGraph.images[0].width)} />
	<meta property="og:image:height" content={String(seo.openGraph.images[0].height)} />
	<meta property="og:image:alt" content={seo.openGraph.images[0].alt} />

	<!-- Twitter Card Meta Tags - Using server-loaded data -->
	<meta name="twitter:card" content={seo.twitter.card} />
	<meta name="twitter:title" content={seo.twitter.title} />
	<meta name="twitter:description" content={seo.twitter.description} />
	<meta name="twitter:image" content={seo.twitter.images[0]} />
	<meta name="twitter:site" content={seo.twitter.site} />

	<!-- Additional Meta Tags -->
	<meta name="theme-color" content="#050505" />
	<meta name="viewport" content="width=device-width, initial-scale=1" />
	<meta name="format-detection" content="telephone=no" />
	<meta name="mobile-web-app-capable" content="yes" />
	<meta name="apple-mobile-web-app-capable" content="yes" />
	<meta name="apple-mobile-web-app-status-bar-style" content="black-translucent" />

	<!-- Performance & Security - Google December 2025 Requirements -->
	<!-- Note: Security headers should be set via server config, not meta tags -->
	<meta name="referrer" content="strict-origin-when-cross-origin" />

	<!-- Google December 2025 E-E-A-T Signals -->
	<meta name="author" content={seo.author} />
	<meta name="publisher" content={seo.publisher} />
	<meta name="article:author" content={seo.openGraph.article.author} />
	<meta name="article:published_time" content={seo.openGraph.article.publishedTime} />
	<meta name="article:modified_time" content={seo.openGraph.article.modifiedTime} />
	<meta name="article:section" content={seo.openGraph.article.section} />
	<meta name="article:tag" content={seo.openGraph.article.tag} />

	<!-- Expertise Signals -->
	<meta
		name="robots"
		content="index, follow, max-snippet:-1, max-image-preview:large, max-video-preview:-1"
	/>
	<meta
		name="googlebot"
		content="index, follow, max-snippet:-1, max-image-preview:large, max-video-preview:-1"
	/>

	<!-- Trust Signals -->
	<link
		rel="alternate"
		hreflang="en"
		href="https://revolution-trading-pros.com/live-trading-rooms"
	/>
	<link rel="me" href="https://twitter.com/revolutiontrading" type="text/html" />
	<link rel="me" href="mailto:support@revolution-trading-pros.com" />

	<!-- Content Classification -->
	<meta name="rating" content="general" />
	<meta name="distribution" content="global" />
	<meta name="language" content="en" />

	<!-- Preconnect for Performance - Google December 2025 Core Web Vitals -->
	<link rel="preconnect" href="https://fonts.googleapis.com" />
	<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous" />
	<link rel="dns-prefetch" href="//www.googletagmanager.com" />
	<link rel="dns-prefetch" href="//www.google-analytics.com" />
	<link rel="preconnect" href="https://www.googletagmanager.com" crossorigin="anonymous" />
	<link
		rel="preload"
		href="/fonts/inter-var.woff2"
		as="font"
		type="font/woff2"
		crossorigin="anonymous"
	/>

	<!-- Critical Resource Preloading for LCP < 2.0s -->
	<link rel="preload" href="/images/live-trading-rooms-hero.webp" as="image" type="image/webp" />
	<link rel="preload" href="/images/live-trading-rooms-hero.jpg" as="image" type="image/jpeg" />

	<!-- Resource Hints for INP < 150ms -->
	<link rel="modulepreload" href="/src/routes/live-trading-rooms/+page.svelte" />

	<!-- Favicon -->
	<link rel="icon" type="image/x-icon" href="/favicon.ico" />
	<link rel="icon" type="image/png" sizes="32x32" href="/favicon-32x32.png" />
	<link rel="icon" type="image/png" sizes="16x16" href="/favicon-16x16.png" />
	<link rel="apple-touch-icon" sizes="180x180" href="/apple-touch-icon.png" />

	<!-- JSON-LD Structured Data would go here if needed -->
</svelte:head>

<div class="ltr">
	<div class="ltr__noise-layer" aria-hidden="true">
		<div class="ltr__noise"></div>
	</div>

	<div class="ticker">
		<div class="ticker-track ticker__track">
			{#each tickerItems as item, _ti (_ti)}
				<div class="ticker__item">
					<span class="ticker__sym">{item.sym}</span>
					<span class="ticker__price">{item.price}</span>
					<span class={['ticker__change', item.up ? 'ticker__change--up' : 'ticker__change--down']}
						>{item.change}</span
					>
				</div>
			{/each}
		</div>
	</div>

	<div class="ltr__content">
		<section class="hero perspective-hero">
			<div class="hero-grid-plane hero__grid-plane" aria-hidden="true">
				<div class="hero__grid-fade"></div>
				<div class="hero__plane-lines"></div>
			</div>

			<div class="hero-chart hero__chart-wrap" aria-hidden="true">
				<svg class="hero__chart-svg" viewBox="0 0 1000 300" preserveAspectRatio="none">
					<defs>
						<linearGradient id="chartGradient" x1="0" y1="0" x2="0" y2="1">
							<stop offset="0%" stop-color="#3b82f6" stop-opacity="0.2" />
							<stop offset="100%" stop-color="#3b82f6" stop-opacity="0" />
						</linearGradient>
					</defs>
					<path d={`M0,300 ${chartPoints} L1000,300 Z`} fill="url(#chartGradient)" />
					<polyline
						class="hero-chart-path"
						points={chartPoints}
						fill="none"
						stroke="#60a5fa"
						stroke-width="2"
						vector-effect="non-scaling-stroke"
					/>
				</svg>
			</div>

			<div class="hero__floaters" aria-hidden="true">
				<div class="hero-floater hero__floater hero__floater--emerald hero__floater--top-left">
					ORDER_FLOW: INSTITUTIONAL_BUY
				</div>
				<div class="hero-floater hero__floater hero__floater--blue hero__floater--bottom-right">
					VOL: 24,000,392
				</div>
				<div class="hero-floater hero__floater hero__floater--plain hero__floater--top-right">
					<div class="hero__floater-row">
						<div class="hero__floater-dot"></div>
						CONNECTED
					</div>
				</div>
			</div>

			<div class="hero__inner">
				<div class="hero-badge hero__badge">
					<span class="hero__badge-pulse">
						<span class="hero__badge-ping"></span>
						<span class="hero__badge-dot"></span>
					</span>
					<span class="hero__badge-label">
						NY Session: <span class="hero__badge-status">Live</span>
					</span>
				</div>

				<h1 class="hero__title">
					<div class="hero-title-line hero__title-line">
						<span class="hero__title-word">Market</span>
					</div>
					<div class="hero-title-line hero__title-line">
						<span class="hero__title-accent">Intelligence</span>
					</div>
				</h1>

				<p class="hero-desc hero__desc">
					Step inside the <span class="hero__desc-emph">Command Center</span>. Real-time data,
					institutional signals, and a community of professional traders.
				</p>

				<!-- ═══════════════════════════════════════════════════════════════════════════
				     6 FLOATING CTA BUTTONS - PE7 RESTORE 2026-05-23
				     Animated entrance from different directions as per original design
				     ═══════════════════════════════════════════════════════════════════════════ -->
				<div class="hero-cta-container hero__cta-row">
					<a
						href="/live-trading-rooms/day-trading"
						class="hero-float-btn hero-float-btn-1 hero__cta hero__cta--emerald"
					>
						<span class="hero__cta-dot hero__cta-dot--pulse"></span>
						Day Trading Room
					</a>
					<a
						href="/live-trading-rooms/swing-trading"
						class="hero-float-btn hero-float-btn-2 hero__cta hero__cta--blue"
					>
						<span class="hero__cta-dot"></span>
						Swing Trading
					</a>
					<a
						href="/live-trading-rooms/small-accounts"
						class="hero-float-btn hero-float-btn-3 hero__cta hero__cta--amber"
					>
						<span class="hero__cta-dot"></span>
						Small Accounts
					</a>
					<a
						href="/alerts/spx-profit-pulse"
						class="hero-float-btn hero-float-btn-4 hero__cta hero__cta--purple"
					>
						<span class="hero__cta-dot"></span>
						SPX Alerts
					</a>
					<a href="/mentorship" class="hero-float-btn hero-float-btn-5 hero__cta hero__cta--cyan">
						<span class="hero__cta-dot"></span>
						Mentorship
					</a>
					<a href="/indicators" class="hero-float-btn hero-float-btn-6 hero__cta hero__cta--rose">
						<span class="hero__cta-dot"></span>
						Indicators
					</a>
				</div>

				<div class="hero-line hero__rule" aria-hidden="true"></div>
			</div>
		</section>

		<RoomsGrid {rooms} />

		<section class="benefits">
			<div class="benefits__halo" aria-hidden="true"></div>
			<div class="benefits__header">
				<h2 class="benefits__title">Why the Pros Choose Us</h2>
				<p class="benefits__lede">
					We don't just sell courses. We build institutional-grade traders through immersion,
					technology, and community.
				</p>
			</div>
			<div class="benefits__grid">
				{#each benefits as item (item.title)}
					<div class="benefits__card">
						<div class="benefits__icon">
							{#if item.iconType === 'analysis'}
								<svg
									aria-hidden="true"
									viewBox="0 0 24 24"
									fill="none"
									stroke="currentColor"
									stroke-width="1.5"
								>
									<circle cx="12" cy="12" r="10" stroke-dasharray="4 4" class="svg-spin-track" />
									<path d="M12 2v20M2 12h20" class="svg-muted" />
									<circle cx="12" cy="12" r="3" class="svg-scale-target" />
								</svg>
							{:else if item.iconType === 'radar'}
								<svg
									aria-hidden="true"
									viewBox="0 0 24 24"
									fill="none"
									stroke="currentColor"
									stroke-width="1.5"
								>
									<path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z" />
									<path class="svg-pulse-target" d="M12 8v4l3 3" stroke-linecap="round" />
								</svg>
							{:else if item.iconType === 'strategy'}
								<svg
									aria-hidden="true"
									viewBox="0 0 24 24"
									fill="none"
									stroke="currentColor"
									stroke-width="1.5"
								>
									<path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20" />
									<path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z" />
									<path
										class="svg-draw-line"
										stroke-dasharray="10"
										stroke-dashoffset="10"
										d="M9 8h6M9 12h4"
										stroke-linecap="round"
									/>
								</svg>
							{:else if item.iconType === 'network'}
								<svg
									aria-hidden="true"
									viewBox="0 0 24 24"
									fill="none"
									stroke="currentColor"
									stroke-width="1.5"
								>
									<circle cx="12" cy="5" r="2" class="svg-fill-target" />
									<circle cx="5" cy="19" r="2" class="svg-fill-target svg-fill-delay-short" />
									<circle cx="19" cy="19" r="2" class="svg-fill-target svg-fill-delay-long" />
									<path d="M12 7l-7 12M12 7l7 12M5 19h14" class="svg-soft" />
								</svg>
							{/if}
						</div>
						<h3 class="benefits__card-title">{item.title}</h3>
						<p class="benefits__card-desc">{item.desc}</p>
					</div>
				{/each}
			</div>
		</section>

		<section {@attach attachCtaElement} class="cta">
			<div class="cta__bg-pattern" aria-hidden="true"></div>
			<div class="cta__inner">
				<h2 class="cta__title">Ready to Level Up?</h2>
				<p class="cta__lede">
					Join thousands of traders who have transformed their results. <br class="cta__br" />
					The market is waiting. Your desk is ready.
				</p>
				<div class="cta__actions">
					<a href="#rooms-section" class="cta__btn cta__btn--primary"> Choose Your Room </a>
					<a href="/about" class="cta__btn cta__btn--ghost"> Talk to an Advisor </a>
				</div>
			</div>
		</section>

		<div class="trust">
			<h3 class="trust__heading">Trusted by 10,000+ Traders Worldwide</h3>
			<div class="trust__row">
				<div class="trust__chip"></div>
				<div class="trust__chip"></div>
				<div class="trust__chip"></div>
				<div class="trust__chip"></div>
			</div>
		</div>
	</div>
</div>

<style>
	/* ─────────────────────────────────────────────────────────────────
	   Page-local tokens (Live Trading Rooms hub).
	   The 6 floating CTA buttons each use a distinct accent palette;
	   the page-local tokens below host the per-color "bg / border / text
	   / hover" values without bleeding into the global token set.
	   ───────────────────────────────────────────────────────────────── */
	.ltr {
		--ltr-bg: #050505;
		--ltr-rose: #fb7185;
		--ltr-purple: #a855f7;
		--ltr-cyan: #06b6d4;
		--ltr-zinc-300: #d4d4d8;
		--ltr-zinc-400: #a1a1aa;
		--ltr-zinc-500: #71717a;
		--ltr-zinc-600: #52525b;

		background: var(--ltr-bg);
		color: #fff;
		font-family: var(--rtp-font-sans);
		position: relative;
	}

	.ltr ::selection {
		background: color-mix(in oklab, var(--rtp-primary) 30%, transparent);
	}

	/* ─────────────────────────────────────────────────────────────────
	   Background noise layer
	   ───────────────────────────────────────────────────────────────── */
	.ltr__noise-layer {
		position: fixed;
		inset: 0;
		pointer-events: none;
		z-index: 0;
	}
	.ltr__noise {
		position: absolute;
		inset: 0;
		background-image: url('https://grainy-gradients.vercel.app/noise.svg');
		opacity: 0.03;
	}

	/* ─────────────────────────────────────────────────────────────────
	   Ticker bar (sticky-feeling top strip)
	   ───────────────────────────────────────────────────────────────── */
	.ticker {
		position: relative;
		z-index: 50;
		border-block-end: 1px solid var(--rtp-border-soft);
		background: color-mix(in oklab, var(--ltr-bg) 80%, transparent);
		backdrop-filter: blur(12px);
		-webkit-backdrop-filter: blur(12px);
		height: 2.5rem;
		display: flex;
		align-items: center;
		overflow: hidden;
	}
	.ticker__track {
		display: flex;
		align-items: center;
		gap: 3rem;
		white-space: nowrap;
		padding-inline: 1rem;
		animation: scroll 60s linear infinite;
	}
	.ticker__track:hover {
		animation-play-state: paused;
	}
	.ticker__item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		font-size: 0.75rem;
		font-family: var(--rtp-font-mono);
		user-select: none;
	}
	.ticker__sym {
		font-weight: 700;
		color: var(--ltr-zinc-300);
	}
	.ticker__price {
		color: var(--ltr-zinc-500);
	}
	.ticker__change--up {
		color: var(--rtp-emerald-bright);
	}
	.ticker__change--down {
		color: var(--ltr-rose);
	}

	@keyframes scroll {
		0% {
			transform: translateX(0);
		}
		100% {
			transform: translateX(-50%);
		}
	}

	/* ─────────────────────────────────────────────────────────────────
	   Page content wrapper
	   ───────────────────────────────────────────────────────────────── */
	.ltr__content {
		position: relative;
		z-index: 10;
		max-width: var(--rtp-content-max);
		margin-inline: auto;
		padding-inline: 1rem;
	}
	@media (min-width: 640px) {
		.ltr__content {
			padding-inline: 1.5rem;
		}
	}
	@media (min-width: 1024px) {
		.ltr__content {
			padding-inline: 2rem;
		}
	}

	/* ─────────────────────────────────────────────────────────────────
	   Hero
	   ───────────────────────────────────────────────────────────────── */
	.hero {
		position: relative;
		min-height: 85vh;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		text-align: center;
		margin-block-end: 6rem;
	}
	.perspective-hero {
		perspective: 1000px;
		overflow: hidden;
	}

	.hero__grid-plane {
		position: absolute;
		inset: 0;
		pointer-events: none;
		opacity: 0;
		transform-style: preserve-3d;
		transform: rotateX(60deg) translateY(0);
		transform-origin: 50% 50%;
	}
	.hero__grid-fade {
		position: absolute;
		inset: 0;
		background: linear-gradient(to bottom, var(--ltr-bg) 0%, transparent 50%, var(--ltr-bg) 100%);
		z-index: 10;
	}
	.hero__plane-lines {
		width: 100%;
		height: 100%;
		background-size: 60px 60px;
		background-image:
			linear-gradient(to right, rgba(255, 255, 255, 0.03) 1px, transparent 1px),
			linear-gradient(to bottom, rgba(255, 255, 255, 0.03) 1px, transparent 1px);
		mask-image: radial-gradient(circle at 50% 0%, black 0%, transparent 70%);
	}

	.hero__chart-wrap {
		position: absolute;
		inset: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		pointer-events: none;
		opacity: 0.4;
		z-index: 0;
		overflow: hidden;
	}
	.hero__chart-svg {
		width: 120%;
		height: 600px;
		transform: translateY(5rem);
		filter: blur(1px);
	}

	.hero__floaters {
		position: absolute;
		inset: 0;
		pointer-events: none;
		z-index: 0;
		overflow: hidden;
	}
	.hero__floater {
		position: absolute;
		font-size: 0.625rem;
		font-family: var(--rtp-font-mono);
		padding: 0.5rem;
		border: 1px solid;
		border-radius: var(--rtp-radius-sm);
		backdrop-filter: blur(4px);
		-webkit-backdrop-filter: blur(4px);
	}
	.hero__floater--emerald {
		color: color-mix(in oklab, var(--rtp-emerald) 30%, transparent);
		border-color: color-mix(in oklab, var(--rtp-emerald) 20%, transparent);
		background: color-mix(in oklab, #064e3b 10%, transparent);
	}
	.hero__floater--blue {
		color: color-mix(in oklab, var(--rtp-blue) 30%, transparent);
		border-color: color-mix(in oklab, var(--rtp-blue) 20%, transparent);
		background: color-mix(in oklab, #1e3a8a 10%, transparent);
	}
	.hero__floater--plain {
		color: var(--ltr-zinc-600);
		border: 0;
		background: transparent;
	}
	.hero__floater--top-left {
		top: 25%;
		left: 2.5rem;
	}
	.hero__floater--bottom-right {
		bottom: 33%;
		right: 5rem;
	}
	.hero__floater--top-right {
		top: 5rem;
		right: 25%;
	}
	.hero__floater-row {
		display: flex;
		gap: 0.25rem;
		align-items: center;
	}
	.hero__floater-dot {
		width: 0.25rem;
		height: 0.25rem;
		background: var(--ltr-zinc-500);
		border-radius: 50%;
	}

	.hero__inner {
		position: relative;
		z-index: 20;
		max-width: 64rem;
		margin-inline: auto;
		margin-block-start: 3rem;
	}

	.hero__badge {
		display: inline-flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.5rem 1.25rem;
		border-radius: var(--rtp-radius-pill);
		background: color-mix(in oklab, #fff 3%, transparent);
		border: 1px solid color-mix(in oklab, #fff 8%, transparent);
		backdrop-filter: blur(12px);
		-webkit-backdrop-filter: blur(12px);
		margin-block-end: 2.5rem;
		box-shadow: 0 0 30px color-mix(in oklab, var(--rtp-emerald) 10%, transparent);
		cursor: default;
		transition: background var(--rtp-dur-fast) var(--rtp-ease-out);
	}
	.hero__badge:hover {
		background: color-mix(in oklab, #fff 5%, transparent);
	}
	.hero__badge-pulse {
		position: relative;
		display: inline-flex;
		width: 0.5rem;
		height: 0.5rem;
	}
	.hero__badge-ping {
		position: absolute;
		inset: 0;
		border-radius: 50%;
		background: var(--rtp-emerald-bright);
		opacity: 0.75;
		animation: hero-ping 1.5s cubic-bezier(0, 0, 0.2, 1) infinite;
	}
	.hero__badge-dot {
		position: relative;
		display: inline-flex;
		width: 0.5rem;
		height: 0.5rem;
		border-radius: 50%;
		background: var(--rtp-emerald);
	}
	.hero__badge-label {
		font-size: 0.75rem;
		font-family: var(--rtp-font-mono);
		text-transform: uppercase;
		letter-spacing: 0.1em;
		color: var(--ltr-zinc-300);
		font-weight: 700;
	}
	.hero__badge-status {
		color: var(--rtp-emerald-bright);
	}

	@keyframes hero-ping {
		75%,
		100% {
			transform: scale(2);
			opacity: 0;
		}
	}

	.hero__title {
		font-size: 3.75rem;
		font-weight: 700;
		letter-spacing: -0.05em;
		line-height: 0.9;
		margin-block-end: 2.5rem;
		color: #fff;
	}
	.hero__title-line {
		overflow: hidden;
	}
	.hero__title-word {
		display: block;
	}
	.hero__title-accent {
		display: inline-block;
		background: linear-gradient(
			to right,
			var(--rtp-blue-bright),
			#a5b4fc,
			var(--rtp-emerald-bright)
		);
		-webkit-background-clip: text;
		background-clip: text;
		color: transparent;
		padding-block-end: 1rem;
	}
	@media (min-width: 768px) {
		.hero__title {
			font-size: 6rem;
		}
	}
	@media (min-width: 1024px) {
		.hero__title {
			font-size: 8rem;
		}
	}

	.hero__desc {
		font-size: 1.25rem;
		color: var(--ltr-zinc-400);
		max-width: 42rem;
		margin-inline: auto;
		line-height: 1.625;
		font-weight: 300;
	}
	.hero__desc-emph {
		color: #fff;
		font-weight: 500;
	}
	@media (min-width: 768px) {
		.hero__desc {
			font-size: 1.5rem;
		}
	}

	.hero__cta-row {
		position: relative;
		margin-block-start: 4rem;
		display: flex;
		flex-wrap: wrap;
		justify-content: center;
		gap: 1rem;
		max-width: 56rem;
		margin-inline: auto;
	}

	/* Floating CTA buttons — six accents */
	.hero__cta {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		border-radius: var(--rtp-radius-md);
		font-weight: 600;
		text-decoration: none;
		backdrop-filter: blur(4px);
		-webkit-backdrop-filter: blur(4px);
		border: 1px solid;
		transition:
			background var(--rtp-dur-base) var(--rtp-ease-out),
			border-color var(--rtp-dur-base) var(--rtp-ease-out);
	}
	.hero__cta-dot {
		width: 0.5rem;
		height: 0.5rem;
		border-radius: 50%;
	}
	.hero__cta-dot--pulse {
		animation: cta-pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
	}
	@keyframes cta-pulse {
		0%,
		100% {
			opacity: 1;
		}
		50% {
			opacity: 0.5;
		}
	}

	.hero__cta--emerald {
		background: color-mix(in oklab, var(--rtp-emerald) 10%, transparent);
		border-color: color-mix(in oklab, var(--rtp-emerald) 30%, transparent);
		color: var(--rtp-emerald-bright);
	}
	.hero__cta--emerald:hover {
		background: color-mix(in oklab, var(--rtp-emerald) 20%, transparent);
		border-color: color-mix(in oklab, var(--rtp-emerald) 50%, transparent);
	}
	.hero__cta--emerald .hero__cta-dot {
		background: var(--rtp-emerald-bright);
	}

	.hero__cta--blue {
		background: color-mix(in oklab, var(--rtp-blue) 10%, transparent);
		border-color: color-mix(in oklab, var(--rtp-blue) 30%, transparent);
		color: var(--rtp-blue-bright);
	}
	.hero__cta--blue:hover {
		background: color-mix(in oklab, var(--rtp-blue) 20%, transparent);
		border-color: color-mix(in oklab, var(--rtp-blue) 50%, transparent);
	}
	.hero__cta--blue .hero__cta-dot {
		background: var(--rtp-blue-bright);
	}

	.hero__cta--amber {
		background: color-mix(in oklab, var(--rtp-amber) 10%, transparent);
		border-color: color-mix(in oklab, var(--rtp-amber) 30%, transparent);
		color: #fbbf24;
	}
	.hero__cta--amber:hover {
		background: color-mix(in oklab, var(--rtp-amber) 20%, transparent);
		border-color: color-mix(in oklab, var(--rtp-amber) 50%, transparent);
	}
	.hero__cta--amber .hero__cta-dot {
		background: #fbbf24;
	}

	.hero__cta--purple {
		background: color-mix(in oklab, var(--ltr-purple) 10%, transparent);
		border-color: color-mix(in oklab, var(--ltr-purple) 30%, transparent);
		color: #c084fc;
	}
	.hero__cta--purple:hover {
		background: color-mix(in oklab, var(--ltr-purple) 20%, transparent);
		border-color: color-mix(in oklab, var(--ltr-purple) 50%, transparent);
	}
	.hero__cta--purple .hero__cta-dot {
		background: #c084fc;
	}

	.hero__cta--cyan {
		background: color-mix(in oklab, var(--ltr-cyan) 10%, transparent);
		border-color: color-mix(in oklab, var(--ltr-cyan) 30%, transparent);
		color: #22d3ee;
	}
	.hero__cta--cyan:hover {
		background: color-mix(in oklab, var(--ltr-cyan) 20%, transparent);
		border-color: color-mix(in oklab, var(--ltr-cyan) 50%, transparent);
	}
	.hero__cta--cyan .hero__cta-dot {
		background: #22d3ee;
	}

	.hero__cta--rose {
		background: color-mix(in oklab, var(--ltr-rose) 10%, transparent);
		border-color: color-mix(in oklab, var(--ltr-rose) 30%, transparent);
		color: var(--ltr-rose);
	}
	.hero__cta--rose:hover {
		background: color-mix(in oklab, var(--ltr-rose) 20%, transparent);
		border-color: color-mix(in oklab, var(--ltr-rose) 50%, transparent);
	}
	.hero__cta--rose .hero__cta-dot {
		background: var(--ltr-rose);
	}

	.hero__rule {
		width: 6rem;
		height: 1px;
		background: linear-gradient(to right, transparent, var(--rtp-primary), transparent);
		margin-inline: auto;
		margin-block-start: 4rem;
		opacity: 0;
	}

	/* ─────────────────────────────────────────────────────────────────
	   Benefits section
	   ───────────────────────────────────────────────────────────────── */
	.benefits {
		padding-block: 6rem;
		border-block-start: 1px solid var(--rtp-border-soft);
		position: relative;
	}
	.benefits__halo {
		position: absolute;
		inset: 0;
		background: color-mix(in oklab, var(--rtp-primary) 5%, transparent);
		filter: blur(100px);
		pointer-events: none;
	}
	.benefits__header {
		text-align: center;
		margin-block-end: 4rem;
		position: relative;
		z-index: 10;
	}
	.benefits__title {
		font-size: 1.875rem;
		font-weight: 700;
		margin-block-end: 1.5rem;
		background-clip: text;
		-webkit-background-clip: text;
		color: transparent;
		background-image: linear-gradient(to bottom, #fff, color-mix(in oklab, #fff 60%, transparent));
	}
	@media (min-width: 768px) {
		.benefits__title {
			font-size: 3rem;
		}
	}
	.benefits__lede {
		color: var(--ltr-zinc-400);
		max-width: 42rem;
		margin-inline: auto;
	}
	.benefits__grid {
		display: grid;
		grid-template-columns: 1fr;
		gap: 2rem;
		position: relative;
		z-index: 10;
	}
	@media (min-width: 768px) {
		.benefits__grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}
	@media (min-width: 1024px) {
		.benefits__grid {
			grid-template-columns: repeat(4, 1fr);
		}
	}
	.benefits__card {
		padding: 1.5rem;
		border-radius: var(--rtp-radius-xl);
		background: color-mix(in oklab, #fff 5%, transparent);
		border: 1px solid color-mix(in oklab, #fff 5%, transparent);
		text-align: center;
		cursor: default;
		transition:
			background var(--rtp-dur-base) var(--rtp-ease-out),
			border-color var(--rtp-dur-base) var(--rtp-ease-out);
	}
	.benefits__card:hover {
		background: color-mix(in oklab, #fff 10%, transparent);
		border-color: color-mix(in oklab, #fff 10%, transparent);
	}
	.benefits__icon {
		width: 3rem;
		height: 3rem;
		margin-inline: auto;
		margin-block-end: 1.5rem;
		color: var(--ltr-zinc-400);
		transition: color var(--rtp-dur-base) var(--rtp-ease-out);
	}
	.benefits__card:hover .benefits__icon {
		color: var(--rtp-blue-bright);
	}
	.benefits__card-title {
		font-size: 1.125rem;
		font-weight: 700;
		color: #fff;
		margin-block-end: 0.75rem;
	}
	.benefits__card-desc {
		font-size: 0.875rem;
		color: var(--ltr-zinc-400);
		line-height: 1.625;
	}

	.svg-muted {
		opacity: 0.3;
	}
	.svg-soft {
		opacity: 0.5;
	}
	.svg-scale-target {
		transform-box: fill-box;
		transform-origin: center;
		transition: transform var(--rtp-dur-base) var(--rtp-ease-out);
	}
	.benefits__card:hover .svg-scale-target {
		transform: scale(1.25);
	}
	.svg-fill-target {
		fill: transparent;
		transition: fill var(--rtp-dur-base) var(--rtp-ease-out);
	}
	.svg-fill-delay-short {
		transition-delay: 100ms;
	}
	.svg-fill-delay-long {
		transition-delay: 200ms;
	}
	.benefits__card:hover .svg-fill-target {
		fill: currentColor;
	}

	@keyframes draw-line {
		to {
			stroke-dashoffset: 0;
		}
	}
	.benefits__card:hover .svg-draw-line {
		animation: draw-line 1s ease-out forwards;
	}

	@keyframes spin-slow {
		to {
			transform: rotate(360deg);
			transform-origin: center;
		}
	}
	.benefits__card:hover .svg-spin-track {
		animation: spin-slow 8s linear infinite;
	}

	@keyframes icon-pulse {
		50% {
			opacity: 0.5;
		}
	}
	.benefits__card:hover .svg-pulse-target {
		animation: icon-pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
	}

	/* ─────────────────────────────────────────────────────────────────
	   CTA section
	   ───────────────────────────────────────────────────────────────── */
	.cta {
		padding-block: 6rem 8rem;
		text-align: center;
		position: relative;
		overflow: hidden;
		border-radius: 1.5rem;
		margin-block: 3rem;
		background: linear-gradient(to bottom, color-mix(in oklab, #1e3a8a 20%, transparent), #000);
		border: 1px solid var(--rtp-border);
	}
	.cta__bg-pattern {
		position: absolute;
		inset: 0;
		background-image: url('/grid-pattern.svg');
		opacity: 0.1;
		pointer-events: none;
	}
	.cta__inner {
		position: relative;
		z-index: 10;
		max-width: 48rem;
		margin-inline: auto;
		padding-inline: 1rem;
	}
	.cta__title {
		font-size: 2.25rem;
		font-weight: 700;
		color: #fff;
		margin-block-end: 1.5rem;
	}
	@media (min-width: 768px) {
		.cta__title {
			font-size: 3rem;
		}
	}
	.cta__lede {
		font-size: 1.25rem;
		color: var(--ltr-zinc-400);
		margin-block-end: 2.5rem;
	}
	.cta__br {
		display: none;
	}
	@media (min-width: 768px) {
		.cta__br {
			display: block;
		}
	}
	.cta__actions {
		display: flex;
		flex-direction: column;
		gap: 1rem;
		justify-content: center;
	}
	@media (min-width: 640px) {
		.cta__actions {
			flex-direction: row;
		}
	}
	.cta__btn {
		padding: 1rem 2rem;
		border-radius: var(--rtp-radius-md);
		font-weight: 700;
		text-decoration: none;
		transition:
			background var(--rtp-dur-base) var(--rtp-ease-out),
			border-color var(--rtp-dur-base) var(--rtp-ease-out),
			transform var(--rtp-dur-base) var(--rtp-ease-out);
	}
	.cta__btn--primary {
		background: #fff;
		color: #000;
		box-shadow: 0 0 40px rgba(255, 255, 255, 0.3);
	}
	.cta__btn--primary:hover {
		background: #eff6ff;
		transform: scale(1.05);
	}
	.cta__btn--ghost {
		background: transparent;
		border: 1px solid color-mix(in oklab, #fff 20%, transparent);
		color: #fff;
	}
	.cta__btn--ghost:hover {
		background: color-mix(in oklab, #fff 5%, transparent);
		border-color: color-mix(in oklab, #fff 40%, transparent);
	}

	/* ─────────────────────────────────────────────────────────────────
	   Trust footer
	   ───────────────────────────────────────────────────────────────── */
	.trust {
		text-align: center;
		border-block-start: 1px solid var(--rtp-border-soft);
		padding-block: 4rem 2rem;
	}
	.trust__heading {
		color: var(--ltr-zinc-600);
		font-size: 0.75rem;
		font-family: var(--rtp-font-mono);
		text-transform: uppercase;
		letter-spacing: 0.2em;
		margin-block-end: 2rem;
	}
	.trust__row {
		display: flex;
		flex-wrap: wrap;
		justify-content: center;
		gap: 3rem;
		opacity: 0.3;
		filter: grayscale(1);
		transition:
			filter var(--rtp-dur-slow) var(--rtp-ease-out),
			opacity var(--rtp-dur-slow) var(--rtp-ease-out);
	}
	.trust__row:hover {
		filter: grayscale(0);
	}
	.trust__chip {
		height: 1.5rem;
		width: 5rem;
		background: color-mix(in oklab, #fff 20%, transparent);
		border-radius: var(--rtp-radius-sm);
	}
</style>

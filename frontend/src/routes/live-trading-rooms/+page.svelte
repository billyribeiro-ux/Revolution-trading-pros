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

	// ctaRef is read by GSAP ScrollTrigger; other sections animate via [data-gsap] attribute selectors.
	let ctaRef: HTMLElement | undefined;

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
						gsap.fromTo(
							selector,
							from,
							{
								x: 0,
								y: 0,
								opacity: 1,
								duration: 0.8,
								delay: 1.5 + delay,
								ease: 'back.out(1.7)'
							}
						);
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
					if (ctaRef) {
						const ctaElement = ctaRef;
						ScrollTrigger.create({
							trigger: ctaElement,
							start: 'top 85%',
							onEnter: () => {
								gsap.fromTo(
									ctaElement,
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

<div class="bg-[#050505] text-white selection:bg-blue-500/30 font-sans relative">
	<div class="fixed inset-0 pointer-events-none z-0">
		<div
			class="absolute inset-0 bg-[url('https://grainy-gradients.vercel.app/noise.svg')] opacity-[0.03]"
		></div>
	</div>

	<div
		class="relative z-50 border-b border-white/5 bg-[#050505]/80 backdrop-blur-md h-10 flex items-center overflow-hidden"
	>
		<div class="ticker-track flex items-center gap-12 whitespace-nowrap px-4">
			{#each tickerItems as item, _ti (_ti)}
				<div class="flex items-center gap-3 text-xs font-mono select-none">
					<span class="font-bold text-zinc-300">{item.sym}</span>
					<span class="text-zinc-500">{item.price}</span>
					<span class={item.up ? 'text-emerald-400' : 'text-rose-400'}>{item.change}</span>
				</div>
			{/each}
		</div>
	</div>

	<div class="relative z-10 pt-0 pb-0 container mx-auto px-4 sm:px-6 lg:px-8">
		<section
			class="relative min-h-[85vh] flex flex-col items-center justify-center text-center perspective-hero mb-24"
		>
			<div class="hero-grid-plane absolute inset-0 pointer-events-none opacity-0">
				<div
					class="absolute inset-0 bg-linear-to-b from-[#050505] via-transparent to-[#050505] z-10"
				></div>
				<div class="grid-lines w-full h-full"></div>
			</div>

			<div
				class="absolute inset-0 flex items-center justify-center pointer-events-none opacity-40 z-0 overflow-hidden"
			>
				<svg
					class="w-[120%] h-[600px] transform translate-y-20 blur-[1px]"
					viewBox="0 0 1000 300"
					preserveAspectRatio="none"
				>
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

			<div class="absolute inset-0 pointer-events-none z-0 overflow-hidden">
				<div
					class="hero-floater absolute top-1/4 left-10 text-[10px] font-mono text-emerald-500/30 p-2 border border-emerald-500/20 rounded bg-emerald-900/10 backdrop-blur-sm"
				>
					ORDER_FLOW: INSTITUTIONAL_BUY
				</div>
				<div
					class="hero-floater absolute bottom-1/3 right-20 text-[10px] font-mono text-blue-500/30 p-2 border border-blue-500/20 rounded bg-blue-900/10 backdrop-blur-sm"
				>
					VOL: 24,000,392
				</div>
				<div class="hero-floater absolute top-20 right-1/4 text-[10px] font-mono text-zinc-600">
					<div class="flex gap-1">
						<div class="w-1 h-1 bg-zinc-500 rounded-full"></div>
						CONNECTED
					</div>
				</div>
			</div>

			<div class="relative z-20 max-w-5xl mx-auto mt-12">
				<div
					class="hero-badge inline-flex items-center gap-3 px-5 py-2 rounded-full bg-white/3 border border-white/8 backdrop-blur-md mb-10 shadow-[0_0_30px_rgba(16,185,129,0.1)] hover:bg-white/5 transition-colors cursor-default"
				>
					<span class="relative flex h-2 w-2">
						<span
							class="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75"
						></span>
						<span class="relative inline-flex rounded-full h-2 w-2 bg-emerald-500"></span>
					</span>
					<span class="text-xs font-mono uppercase tracking-widest text-zinc-300 font-bold">
						NY Session: <span class="text-emerald-400">Live</span>
					</span>
				</div>

				<h1
					class="text-6xl md:text-8xl lg:text-9xl font-bold tracking-tighter leading-[0.9] mb-10 text-white"
				>
					<div class="hero-title-line overflow-hidden">
						<span class="block">Market</span>
					</div>
					<div class="hero-title-line overflow-hidden">
						<span
							class="inline-block bg-linear-to-r from-blue-400 via-indigo-300 to-emerald-400 bg-clip-text text-transparent pb-4"
							>Intelligence</span
						>
					</div>
				</h1>

				<p
					class="hero-desc text-xl md:text-2xl text-zinc-400 max-w-2xl mx-auto leading-relaxed font-light"
				>
					Step inside the <span class="text-white font-medium">Command Center</span>. Real-time
					data, institutional signals, and a community of professional traders.
				</p>

				<!-- ═══════════════════════════════════════════════════════════════════════════
				     6 FLOATING CTA BUTTONS - PE7 RESTORE 2026-05-23
				     Animated entrance from different directions as per original design
				     ═══════════════════════════════════════════════════════════════════════════ -->
				<div class="hero-cta-container relative mt-16 flex flex-wrap justify-center gap-4 max-w-4xl mx-auto">
					<a
						href="/live-trading-rooms/day-trading"
						class="hero-float-btn hero-float-btn-1 inline-flex items-center gap-2 px-6 py-3 rounded-xl bg-emerald-500/10 border border-emerald-500/30 text-emerald-400 font-semibold hover:bg-emerald-500/20 hover:border-emerald-500/50 transition-all duration-300 backdrop-blur-sm"
					>
						<span class="w-2 h-2 rounded-full bg-emerald-400 animate-pulse"></span>
						Day Trading Room
					</a>
					<a
						href="/live-trading-rooms/swing-trading"
						class="hero-float-btn hero-float-btn-2 inline-flex items-center gap-2 px-6 py-3 rounded-xl bg-blue-500/10 border border-blue-500/30 text-blue-400 font-semibold hover:bg-blue-500/20 hover:border-blue-500/50 transition-all duration-300 backdrop-blur-sm"
					>
						<span class="w-2 h-2 rounded-full bg-blue-400"></span>
						Swing Trading
					</a>
					<a
						href="/live-trading-rooms/small-accounts"
						class="hero-float-btn hero-float-btn-3 inline-flex items-center gap-2 px-6 py-3 rounded-xl bg-amber-500/10 border border-amber-500/30 text-amber-400 font-semibold hover:bg-amber-500/20 hover:border-amber-500/50 transition-all duration-300 backdrop-blur-sm"
					>
						<span class="w-2 h-2 rounded-full bg-amber-400"></span>
						Small Accounts
					</a>
					<a
						href="/alerts/spx-profit-pulse"
						class="hero-float-btn hero-float-btn-4 inline-flex items-center gap-2 px-6 py-3 rounded-xl bg-purple-500/10 border border-purple-500/30 text-purple-400 font-semibold hover:bg-purple-500/20 hover:border-purple-500/50 transition-all duration-300 backdrop-blur-sm"
					>
						<span class="w-2 h-2 rounded-full bg-purple-400"></span>
						SPX Alerts
					</a>
					<a
						href="/mentorship"
						class="hero-float-btn hero-float-btn-5 inline-flex items-center gap-2 px-6 py-3 rounded-xl bg-cyan-500/10 border border-cyan-500/30 text-cyan-400 font-semibold hover:bg-cyan-500/20 hover:border-cyan-500/50 transition-all duration-300 backdrop-blur-sm"
					>
						<span class="w-2 h-2 rounded-full bg-cyan-400"></span>
						Mentorship
					</a>
					<a
						href="/indicators"
						class="hero-float-btn hero-float-btn-6 inline-flex items-center gap-2 px-6 py-3 rounded-xl bg-rose-500/10 border border-rose-500/30 text-rose-400 font-semibold hover:bg-rose-500/20 hover:border-rose-500/50 transition-all duration-300 backdrop-blur-sm"
					>
						<span class="w-2 h-2 rounded-full bg-rose-400"></span>
						Indicators
					</a>
				</div>

				<div
					class="hero-line w-24 h-px bg-linear-to-r from-transparent via-blue-500 to-transparent mx-auto mt-16 opacity-0"
				></div>
			</div>
		</section>

		<RoomsGrid {rooms} />

		<section class="py-24 border-t border-white/5 relative">
			<div class="absolute inset-0 bg-blue-500/5 blur-[100px] pointer-events-none"></div>
			<div class="text-center mb-16 relative z-10">
				<h2
					class="text-3xl md:text-5xl font-bold mb-6 bg-clip-text text-transparent bg-linear-to-b from-white to-white/60"
				>
					Why the Pros Choose Us
				</h2>
				<p class="text-zinc-400 max-w-2xl mx-auto">
					We don't just sell courses. We build institutional-grade traders through immersion,
					technology, and community.
				</p>
			</div>
			<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-8">
				{#each benefits as item (item.title)}
					<div
						class="p-6 rounded-2xl bg-white/5 border border-white/5 hover:bg-white/10 hover:border-white/10 transition-colors duration-300 text-center group cursor-default"
					>
						<div
							class="mx-auto w-12 h-12 mb-6 text-zinc-400 group-hover:text-blue-400 transition-colors duration-300"
						>
							{#if item.iconType === 'analysis'}
								<svg aria-hidden="true" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
									<circle
										cx="12"
										cy="12"
										r="10"
										stroke-dasharray="4 4"
										class="group-hover:animate-spin-slow"
									/>
									<path d="M12 2v20M2 12h20" class="opacity-30" />
									<circle
										cx="12"
										cy="12"
										r="3"
										class="group-hover:scale-125 transition-transform"
									/>
								</svg>
							{:else if item.iconType === 'radar'}
								<svg aria-hidden="true" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
									<path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z" />
									<path class="group-hover:animate-pulse" d="M12 8v4l3 3" stroke-linecap="round" />
								</svg>
							{:else if item.iconType === 'strategy'}
								<svg aria-hidden="true" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
									<path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20" />
									<path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z" />
									<path
										class="group-hover:animate-draw-line"
										stroke-dasharray="10"
										stroke-dashoffset="10"
										d="M9 8h6M9 12h4"
										stroke-linecap="round"
									/>
								</svg>
							{:else if item.iconType === 'network'}
								<svg aria-hidden="true" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
									<circle cx="12" cy="5" r="2" class="group-hover:fill-current" />
									<circle
										cx="5"
										cy="19"
										r="2"
										class="group-hover:fill-current transition-colors delay-100"
									/>
									<circle
										cx="19"
										cy="19"
										r="2"
										class="group-hover:fill-current transition-colors delay-200"
									/>
									<path d="M12 7l-7 12M12 7l7 12M5 19h14" class="opacity-50" />
								</svg>
							{/if}
						</div>
						<h3 class="text-lg font-bold text-white mb-3">{item.title}</h3>
						<p class="text-sm text-zinc-400 leading-relaxed">{item.desc}</p>
					</div>
				{/each}
			</div>
		</section>

		<section
			bind:this={ctaRef}
			class="py-24 pb-32 text-center relative overflow-hidden rounded-3xl my-12 bg-linear-to-b from-blue-900/20 to-black border border-white/10"
		>
			<div class="absolute inset-0 bg-[url('/grid-pattern.svg')] opacity-10"></div>
			<div class="relative z-10 max-w-3xl mx-auto px-4">
				<h2 class="text-4xl md:text-5xl font-bold text-white mb-6">Ready to Level Up?</h2>
				<p class="text-xl text-zinc-400 mb-10">
					Join thousands of traders who have transformed their results. <br
						class="hidden md:block"
					/>
					The market is waiting. Your desk is ready.
				</p>
				<div class="flex flex-col sm:flex-row gap-4 justify-center">
					<a
						href="#rooms-section"
						class="px-8 py-4 bg-white text-black font-bold rounded-xl hover:bg-blue-50 hover:scale-105 transition-all duration-300 shadow-[0_0_40px_rgba(255,255,255,0.3)]"
					>
						Choose Your Room
					</a>
					<a
						href="/about"
						class="px-8 py-4 bg-transparent border border-white/20 text-white font-bold rounded-xl hover:bg-white/5 hover:border-white/40 transition-all duration-300"
					>
						Talk to an Advisor
					</a>
				</div>
			</div>
		</section>

		<div class="text-center border-t border-white/5 pt-16 pb-8">
			<h3 class="text-zinc-600 text-xs font-mono uppercase tracking-[0.2em] mb-8">
				Trusted by 10,000+ Traders Worldwide
			</h3>
			<div
				class="flex flex-wrap justify-center gap-12 opacity-30 grayscale hover:grayscale-0 transition-all duration-500"
			>
				<div class="h-6 w-20 bg-white/20 rounded-sm"></div>
				<div class="h-6 w-20 bg-white/20 rounded-sm"></div>
				<div class="h-6 w-20 bg-white/20 rounded-sm"></div>
				<div class="h-6 w-20 bg-white/20 rounded-sm"></div>
			</div>
		</div>
	</div>
</div>


<style>
	/* --- Hero Specifics --- */
	.perspective-hero {
		perspective: 1000px;
		overflow: hidden;
	}

	.hero-grid-plane {
		transform-style: preserve-3d;
		transform: rotateX(60deg) translateY(0);
		transform-origin: 50% 50%;
	}

	.grid-lines {
		background-size: 60px 60px;
		background-image:
			linear-gradient(to right, rgba(255, 255, 255, 0.03) 1px, transparent 1px),
			linear-gradient(to bottom, rgba(255, 255, 255, 0.03) 1px, transparent 1px);
		mask-image: radial-gradient(circle at 50% 0%, black 0%, transparent 70%);
	}

	/* --- Ticker --- */
	.ticker-track {
		animation: scroll 60s linear infinite;
	}
	.ticker-track:hover {
		animation-play-state: paused;
	}
	@keyframes scroll {
		0% {
			transform: translateX(0);
		}
		100% {
			transform: translateX(-50%);
		}
	}

	/* SVG Keyframes — applied via Tailwind group-hover: variants (global at runtime) */
	@keyframes draw-line {
		to {
			stroke-dashoffset: 0;
		}
	}
	:global(.animate-draw-line) {
		animation: draw-line 1s ease-out forwards;
	}

	@keyframes spin-slow {
		to {
			transform: rotate(360deg);
			transform-origin: center;
		}
	}
	:global(.animate-spin-slow) {
		animation: spin-slow 8s linear infinite;
	}
</style>

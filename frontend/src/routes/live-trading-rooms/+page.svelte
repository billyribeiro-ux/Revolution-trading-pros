<script lang="ts">
import { logger } from '$lib/utils/logger';
	import { onMount } from 'svelte';
	import { spring } from 'svelte/motion';
	import { browser } from '$app/environment';
	import MarketingFooter from '$lib/components/sections/MarketingFooter.svelte';

	// GSAP types for TypeScript (actual imports are dynamic for SSR safety)
	type GSAPInstance = typeof import('gsap').gsap;

	/**
	 * Svelte 5 Runes & SSR/SSG Pattern
	 */
	interface Props {
		data: {
			rooms: any[];
			benefits: any[];
			symbols: any[];
			seo: any;
		};
	}
	let { data }: Props = $props();

	// Use server-loaded data for SSR/SSG
	let rooms = $derived(data.rooms);
	let benefits = $derived(data.benefits);
	let symbols = $derived(data.symbols);
	let seo = $derived(data.seo);
	let tickerItems = $derived([...symbols, ...symbols, ...symbols, ...symbols]);

	/**
	 * Action: 3D Tilt Effect
	 */
	function tilt(node: HTMLElement) {
		const x = spring(0, { stiffness: 0.05, damping: 0.25 });
		const y = spring(0, { stiffness: 0.05, damping: 0.25 });

		const unsubX = x.subscribe((v) => node.style.setProperty('--rotX', `${v}deg`));
		const unsubY = y.subscribe((v) => node.style.setProperty('--rotY', `${v}deg`));

		function handleMove(e: MouseEvent) {
			const rect = node.getBoundingClientRect();
			x.set(((e.clientY - rect.top - rect.height / 2) / rect.height) * -3);
			y.set(((e.clientX - rect.left - rect.width / 2) / rect.width) * 3);
		}

		function handleLeave() {
			x.set(0);
			y.set(0);
		}

		node.addEventListener('mousemove', handleMove);
		node.addEventListener('mouseleave', handleLeave);

		return {
			destroy() {
				node.removeEventListener('mousemove', handleMove);
				node.removeEventListener('mouseleave', handleLeave);
				unsubX();
				unsubY();
			}
		};
	}

	/**
	 * Hero Chart Data Generation (For SVG Animation)
	 */
	const chartPoints = Array.from({ length: 40 }, (_, i) => {
		const x = i * 25;
		// Simple random walk for visual flair
		const y = 100 + Math.sin(i * 0.2) * 40 + (Math.random() - 0.5) * 30 - i * 2;
		return `${x},${y}`;
	}).join(' ');

	/**
	 * Animation Controller
	 * ICT 11+: These variables are used in bind:this directives in the template.
	 * TypeScript doesn't recognize bind:this as a "read" operation, causing false positive lints.
	 * They are necessary for GSAP ScrollTrigger animations.
	 */
	// @ts-ignore - Used in bind:this directive at line 378 (TypeScript limitation with Svelte bindings)
	let _heroContainer: HTMLElement | undefined;
	// @ts-ignore - Used in bind:this directive at line 441 (TypeScript limitation with Svelte bindings)
	let _gridRef: HTMLElement | undefined;
	// @ts-ignore - Used in bind:this directive at line 558 (TypeScript limitation with Svelte bindings)
	let _benefitsRef: HTMLElement | undefined;
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
					const gsap = gsapInstance!;
					const ScrollTrigger = scrollTriggerInstance!;

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
				logger.warn('[Live Trading Rooms] GSAP initialization failed:', error);
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
	<meta property="og:image:width" content={seo.openGraph.images[0].width} />
	<meta property="og:image:height" content={seo.openGraph.images[0].height} />
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
			{#each tickerItems as item}
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
			bind:this={_heroContainer}
			class="relative min-h-[85vh] flex flex-col items-center justify-center text-center perspective-hero mb-24"
		>
			<div class="hero-grid-plane absolute inset-0 pointer-events-none opacity-0">
				<div
					class="absolute inset-0 bg-gradient-to-b from-[#050505] via-transparent to-[#050505] z-10"
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
					class="hero-badge inline-flex items-center gap-3 px-5 py-2 rounded-full bg-white/[0.03] border border-white/[0.08] backdrop-blur-md mb-10 shadow-[0_0_30px_rgba(16,185,129,0.1)] hover:bg-white/[0.05] transition-colors cursor-default"
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
							class="inline-block bg-gradient-to-r from-blue-400 via-indigo-300 to-emerald-400 bg-clip-text text-transparent pb-4"
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

				<div
					class="hero-line w-24 h-[1px] bg-gradient-to-r from-transparent via-blue-500 to-transparent mx-auto mt-12 opacity-0"
				></div>
			</div>
		</section>

		<div
			bind:this={_gridRef}
			id="rooms-section"
			class="grid grid-cols-1 md:grid-cols-3 gap-6 lg:gap-8 perspective-container mb-32"
		>
			{#each rooms as room}
				<article
					use:tilt
					class="group relative h-full card-3d"
					role="region"
					aria-label={room.name}
				>
					<div
						class={`absolute -inset-[1px] rounded-3xl bg-gradient-to-b opacity-0 group-hover:opacity-100 transition-opacity duration-500 blur-sm 
                        ${room.accent === 'cyan' ? 'from-cyan-500/50' : ''}
                        ${room.accent === 'emerald' ? 'from-emerald-500/50' : ''}
                        ${room.accent === 'amber' ? 'from-amber-500/50' : ''}
                        to-transparent`}
					></div>
					<div
						class="relative h-full flex flex-col bg-[#0A0A0A] border border-white/5 hover:border-white/10 rounded-3xl p-1 shadow-2xl overflow-hidden transition-colors duration-300"
					>
						<div
							class="flex-1 flex flex-col p-6 lg:p-8 rounded-[20px] bg-gradient-to-b from-white/[0.02] to-transparent"
						>
							{#if room.badge}
								<div class="absolute top-6 right-6">
									<span
										class={`inline-block px-2 py-1 rounded text-[10px] font-bold uppercase tracking-wider border
                                        ${room.accent === 'cyan' ? 'text-cyan-300 bg-cyan-500/10 border-cyan-500/20' : ''}
                                        ${room.accent === 'emerald' ? 'text-emerald-300 bg-emerald-500/10 border-emerald-500/20' : ''}
                                        ${room.accent === 'amber' ? 'text-amber-300 bg-amber-500/10 border-amber-500/20' : ''}
                                    `}>{room.badge}</span
									>
								</div>
							{/if}
							<div
								class={`w-14 h-14 rounded-xl flex items-center justify-center mb-6 transition-all duration-300 ease-out border border-white/5
                                ${room.accent === 'cyan' ? 'bg-cyan-500/5 text-cyan-400' : ''}
                                ${room.accent === 'emerald' ? 'bg-emerald-500/5 text-emerald-400' : ''}
                                ${room.accent === 'amber' ? 'bg-amber-500/5 text-amber-400' : ''}
                            `}
							>
								{#if room.iconType === 'volatility'}
									<svg
										class="w-8 h-8"
										viewBox="0 0 24 24"
										fill="none"
										stroke="currentColor"
										stroke-width="1.5"
									>
										<path class="group-hover:animate-candle-1" d="M6 6v12" stroke-linecap="round" />
										<rect
											class="group-hover:animate-candle-body-1"
											x="4"
											y="9"
											width="4"
											height="6"
											fill="currentColor"
											fill-opacity="0.2"
										/>
										<path
											class="group-hover:animate-candle-2"
											d="M12 4v16"
											stroke-linecap="round"
										/>
										<rect
											class="group-hover:animate-candle-body-2"
											x="10"
											y="7"
											width="4"
											height="10"
											fill="currentColor"
											fill-opacity="0.2"
										/>
										<path class="group-hover:animate-candle-3" d="M18 8v8" stroke-linecap="round" />
										<rect
											class="group-hover:animate-candle-body-3"
											x="16"
											y="10"
											width="4"
											height="4"
											fill="currentColor"
											fill-opacity="0.2"
										/>
									</svg>
								{:else if room.iconType === 'trend'}
									<svg
										class="w-8 h-8"
										viewBox="0 0 24 24"
										fill="none"
										stroke="currentColor"
										stroke-width="1.5"
									>
										<path
											class="group-hover:animate-draw-line"
											stroke-dasharray="30"
											stroke-dashoffset="30"
											d="M3 12c0-3 3-6 6-6s6 3 6 6 3 6 6 6"
											stroke-linecap="round"
										/>
										<circle
											cx="21"
											cy="18"
											r="2"
											fill="currentColor"
											class="opacity-0 group-hover:opacity-100 transition-opacity delay-300"
										/>
									</svg>
								{:else if room.iconType === 'growth'}
									<svg
										class="w-8 h-8"
										viewBox="0 0 24 24"
										fill="none"
										stroke="currentColor"
										stroke-width="1.5"
									>
										<path d="M3 20h18" stroke-linecap="round" />
										<path class="group-hover:animate-grow-1" d="M5 20v-4" stroke-linecap="round" />
										<path class="group-hover:animate-grow-2" d="M9 20v-8" stroke-linecap="round" />
										<path
											class="group-hover:animate-grow-3"
											d="M13 20v-12"
											stroke-linecap="round"
										/>
										<path
											class="group-hover:animate-grow-4"
											d="M17 20v-16"
											stroke-linecap="round"
										/>
										<path d="M17 4l2 2" stroke-linecap="round" />
									</svg>
								{/if}
							</div>
							<h2 class="text-2xl font-bold text-white mb-2">{room.name}</h2>
							<p class="text-sm font-medium text-zinc-400 mb-4">{room.tagline}</p>
							<div class="flex items-center gap-2 mb-6 text-xs font-mono text-zinc-500">
								<span class="relative flex h-2 w-2">
									<span
										class={`animate-ping absolute inline-flex h-full w-full rounded-full opacity-75
                                        ${room.accent === 'cyan' ? 'bg-cyan-400' : ''}
                                        ${room.accent === 'emerald' ? 'bg-emerald-400' : ''}
                                        ${room.accent === 'amber' ? 'bg-amber-400' : ''}
                                    `}
									></span>
									<span
										class={`relative inline-flex rounded-full h-2 w-2 
                                        ${room.accent === 'cyan' ? 'bg-cyan-500' : ''}
                                        ${room.accent === 'emerald' ? 'bg-emerald-500' : ''}
                                        ${room.accent === 'amber' ? 'bg-amber-500' : ''}
                                    `}
									></span>
								</span>
								{room.liveCount} Traders Online
							</div>
							<p class="text-sm leading-relaxed text-zinc-400 mb-8 border-t border-white/5 pt-6">
								{room.description}
							</p>
							<ul class="space-y-3 mb-8 flex-1">
								{#each room.features as feature}
									<li class="flex items-start gap-3 text-sm text-zinc-300">
										<svg
											class={`w-4 h-4 shrink-0 mt-[3px] 
                                            ${room.accent === 'cyan' ? 'text-cyan-500/80' : ''}
                                            ${room.accent === 'emerald' ? 'text-emerald-500/80' : ''}
                                            ${room.accent === 'amber' ? 'text-amber-500/80' : ''}
                                        `}
											viewBox="0 0 24 24"
											fill="none"
											stroke="currentColor"
											stroke-width="2.5"
											stroke-linecap="round"
											stroke-linejoin="round"><polyline points="20 6 9 17 4 12"></polyline></svg
										>
										{feature}
									</li>
								{/each}
							</ul>
							<div class="mt-auto">
								<div class="flex flex-col gap-1 mb-6">
									<div class="flex items-end gap-1">
										<span class="text-3xl font-bold text-white">${room.price.monthly}</span>
										<span class="text-zinc-500 text-sm mb-1">/ month</span>
									</div>
									<div class="flex items-center justify-between text-xs text-zinc-500 mt-2">
										<span>Annual: ${room.price.annual}/yr</span>
										<span class="text-emerald-400 font-medium">Save 36%</span>
									</div>
								</div>
								<a
									href="/live-trading-rooms/{room.id}"
									class={`group/btn relative w-full flex items-center justify-center gap-2 py-4 rounded-xl text-black font-bold text-sm transition-all duration-300 overflow-hidden
                                        ${room.accent === 'cyan' ? 'bg-white hover:bg-cyan-400' : ''}
                                        ${room.accent === 'emerald' ? 'bg-white hover:bg-emerald-400' : ''}
                                        ${room.accent === 'amber' ? 'bg-white hover:bg-amber-400' : ''}
                                        hover:shadow-[0_0_20px_rgba(255,255,255,0.3)]
                                    `}
								>
									<span class="relative z-10">Access Room</span>
									<svg
										class="w-4 h-4 relative z-10 transition-transform group-hover/btn:translate-x-1"
										viewBox="0 0 24 24"
										fill="none"
										stroke="currentColor"
										stroke-width="2"><path d="M5 12h14m-7-7l7 7-7 7" /></svg
									>
								</a>
							</div>
						</div>
					</div>
				</article>
			{/each}
		</div>

		<section class="py-24 border-t border-white/5 relative">
			<div class="absolute inset-0 bg-blue-500/5 blur-[100px] pointer-events-none"></div>
			<div class="text-center mb-16 relative z-10">
				<h2
					class="text-3xl md:text-5xl font-bold mb-6 bg-clip-text text-transparent bg-gradient-to-b from-white to-white/60"
				>
					Why the Pros Choose Us
				</h2>
				<p class="text-zinc-400 max-w-2xl mx-auto">
					We don't just sell courses. We build institutional-grade traders through immersion,
					technology, and community.
				</p>
			</div>
			<div bind:this={_benefitsRef} class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-8">
				{#each benefits as item}
					<div
						class="p-6 rounded-2xl bg-white/5 border border-white/5 hover:bg-white/10 hover:border-white/10 transition-colors duration-300 text-center group cursor-default"
					>
						<div
							class="mx-auto w-12 h-12 mb-6 text-zinc-400 group-hover:text-blue-400 transition-colors duration-300"
						>
							{#if item.iconType === 'analysis'}
								<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
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
								<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
									<path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z" />
									<path class="group-hover:animate-pulse" d="M12 8v4l3 3" stroke-linecap="round" />
								</svg>
							{:else if item.iconType === 'strategy'}
								<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
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
								<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
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
			class="py-24 pb-32 text-center relative overflow-hidden rounded-3xl my-12 bg-gradient-to-b from-blue-900/20 to-black border border-white/10"
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

<MarketingFooter />

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

	/* --- Shared 3D/Animation --- */
	.card-3d {
		transform-style: preserve-3d;
		transform: perspective(1000px) rotateX(var(--rotX, 0deg)) rotateY(var(--rotY, 0deg));
		will-change: transform;
	}
	.perspective-container {
		perspective: 2000px;
	}
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

	/* SVG Keyframes */
	@keyframes draw-line {
		to {
			stroke-dashoffset: 0;
		}
	}
	.animate-draw-line {
		animation: draw-line 1s ease-out forwards;
	}

	@keyframes spin-slow {
		to {
			transform: rotate(360deg);
			transform-origin: center;
		}
	}
	.animate-spin-slow {
		animation: spin-slow 8s linear infinite;
	}

	/* Candle Animations */
	@keyframes candle-up {
		0%,
		100% {
			transform: scaleY(1);
		}
		50% {
			transform: scaleY(1.2);
		}
	}
	.animate-candle-body-1 {
		transform-origin: bottom;
		animation: candle-up 2s infinite ease-in-out;
	}
	.animate-candle-body-2 {
		transform-origin: bottom;
		animation: candle-up 3s infinite ease-in-out 0.5s;
	}
	.animate-candle-body-3 {
		transform-origin: bottom;
		animation: candle-up 2.5s infinite ease-in-out 0.2s;
	}

	/* Growth Animations */
	@keyframes grow-up {
		from {
			transform: scaleY(0);
		}
		to {
			transform: scaleY(1);
		}
	}
	.animate-grow-1 {
		transform-origin: bottom;
		animation: grow-up 0.4s ease-out forwards;
	}
	.animate-grow-2 {
		transform-origin: bottom;
		animation: grow-up 0.4s ease-out 0.1s forwards;
	}
	.animate-grow-3 {
		transform-origin: bottom;
		animation: grow-up 0.4s ease-out 0.2s forwards;
	}
	.animate-grow-4 {
		transform-origin: bottom;
		animation: grow-up 0.4s ease-out 0.3s forwards;
	}
</style>

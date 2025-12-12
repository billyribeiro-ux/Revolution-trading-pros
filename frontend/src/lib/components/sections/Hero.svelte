<script lang="ts">
	/**
	 * HeroSection - Netflix L11+ Enhanced Slider Hero
	 * ══════════════════════════════════════════════════════════════════════════════
	 * ORIGINAL SLIDER STRUCTURE with cinematic enhancements:
	 * - 4 rotating slides with unique GSAP animations per slide
	 * - Enhanced chart background with volumetric glow
	 * - Cinematic slide transitions with blur/scale effects
	 * - Slide indicator dots with progress animation
	 * - Ambient depth layers and grid background
	 * - Premium button interactions
	 * ══════════════════════════════════════════════════════════════════════════════
	 */
	import { onMount, onDestroy, tick } from 'svelte';
	import { browser } from '$app/environment';

	// ============================================================================
	// MODULE-LEVEL PRE-COMPUTATION
	// ============================================================================
	const CANDLE_DATA = generateCandleData();

	function generateCandleData() {
		const candles = [];
		const baseTime = 1704715800;
		const rangeCenter = 475.5;
		const rangeHigh = 476.1;
		const rangeLow = 474.9;

		let previousClose = rangeCenter;
		let miniTrend = 0;

		for (let i = 0; i < 220; i++) {
			if (i % 7 === 0) miniTrend = (Math.random() - 0.5) * 0.15;
			miniTrend *= 0.85;

			const randomMove = (Math.random() - 0.5) * 0.12;
			const pullToCenter = -(previousClose - rangeCenter) * 0.2;
			const bigMove = Math.random() < 0.1 ? (Math.random() - 0.5) * 0.2 : 0;
			const priceMove = miniTrend + randomMove + pullToCenter + bigMove;

			const open = previousClose;
			let close = Math.max(rangeLow, Math.min(rangeHigh, open + priceMove));

			const bodySize = Math.abs(close - open);
			const wickRange = Math.max(bodySize * 0.08, 0.01);

			candles.push({
				time: baseTime + i * 60,
				open: +open.toFixed(2),
				high: +Math.min(rangeHigh, Math.max(open, close) + wickRange * Math.random()).toFixed(2),
				low: +Math.max(rangeLow, Math.min(open, close) - wickRange * Math.random()).toFixed(2),
				close: +close.toFixed(2)
			});

			previousClose = close;
		}

		return candles;
	}

	// Slide data - 4 rotating slides
	const SLIDES = Object.freeze([
		{
			title: 'Live Trading Rooms',
			subtitle: 'Institutional-Style Sessions',
			description: 'Join structured sessions with clear levels, real-time execution, and disciplined risk management.',
			primaryCTA: { text: 'Explore Rooms', href: '/live-trading-rooms/day-trading' },
			secondaryCTA: { text: 'Create Free Account', href: '/signup' },
			accentColor: 'rgba(99, 102, 241, 0.5)'
		},
		{
			title: 'SPX Profit Pulse',
			subtitle: 'Context-Rich Alerts',
			description: 'Receive SPX alerts with entry context, invalidation, and sizing guidance (not lottery tickets).',
			primaryCTA: { text: 'Get Alerts', href: '/alerts/spx-profit-pulse' },
			secondaryCTA: { text: 'Learn More', href: '/alerts/explosive-swings' },
			accentColor: 'rgba(250, 204, 21, 0.5)'
		},
		{
			title: 'Trading Frameworks',
			subtitle: 'Structured Education',
			description: 'Master robust frameworks for execution, risk, and review workflows across market conditions.',
			primaryCTA: { text: 'Browse Courses', href: '/courses' },
			secondaryCTA: { text: 'View Mentorship', href: '/mentorship' },
			accentColor: 'rgba(52, 211, 153, 0.5)'
		},
		{
			title: 'Pro Tools & Indicators',
			subtitle: 'Professional-Grade Edge',
			description: 'Volume profiles, internals, momentum systems, and tooling built for repeatable institutional logic.',
			primaryCTA: { text: 'View Indicators', href: '/indicators' },
			secondaryCTA: { text: 'See All Tools', href: '/store' },
			accentColor: 'rgba(139, 92, 246, 0.5)'
		}
	]);

	// ============================================================================
	// STATE
	// ============================================================================
	let chart: ReturnType<typeof import('lightweight-charts').createChart> | null = null;
	let series: any = null;
	let replayInterval: ReturnType<typeof setInterval> | null = null;
	let slideInterval: ReturnType<typeof setInterval> | null = null;
	let currentSlide = $state(0);
	let previousSlide = $state(-1);
	let isLooping = $state(false);
	let gsapLib: any = null;
	let timeline: any = null;
	let gsapLoaded = $state(false);
	let isTransitioning = $state(false);

	// DOM refs
	let heroSection = $state<HTMLElement | null>(null);
	let chartContainer = $state<HTMLElement | null>(null);
	let slidesContainer = $state<HTMLElement | null>(null);
	let resizeObserver: ResizeObserver | null = null;
	let visibilityObserver: IntersectionObserver | null = null;
	let isVisible = $state(true);
	let chartReady = $state(false);
	let mounted = $state(false);
	let isAnimating = $state(false);

	// Progress for slide indicator
	let slideProgress = $state(0);
	let progressInterval: ReturnType<typeof setInterval> | null = null;

	// ============================================================================
	// CHART INITIALIZATION
	// ============================================================================
	async function initChart(): Promise<void> {
		if (!browser || !chartContainer || !heroSection || chartReady) return;

		const { createChart, CandlestickSeries, CrosshairMode, ColorType } = await import('lightweight-charts');

		const width = heroSection.clientWidth || window.innerWidth;
		const height = heroSection.clientHeight || window.innerHeight;

		if (chart) {
			chart.remove();
			chart = null;
			series = null;
		}

		chart = createChart(chartContainer, {
			layout: {
				background: { type: ColorType.Solid, color: 'transparent' },
				textColor: 'rgba(255, 255, 255, 0.08)'
			},
			grid: {
				vertLines: { visible: false },
				horzLines: { visible: false }
			},
			width,
			height,
			handleScroll: false,
			handleScale: false,
			timeScale: {
				visible: false,
				rightOffset: 0,
				barSpacing: 5,
				minBarSpacing: 2,
				fixLeftEdge: false,
				fixRightEdge: true,
				lockVisibleTimeRangeOnResize: true,
				rightBarStaysOnScroll: true,
				shiftVisibleRangeOnNewBar: true,
				borderColor: 'transparent'
			},
			crosshair: { mode: CrosshairMode.Hidden },
			rightPriceScale: { visible: false, borderColor: 'transparent' }
		});

		series = chart.addSeries(CandlestickSeries, {
			upColor: 'rgba(52, 211, 153, 0.5)',
			downColor: 'rgba(239, 68, 68, 0.5)',
			borderVisible: false,
			wickVisible: true,
			wickUpColor: 'rgba(52, 211, 153, 0.35)',
			wickDownColor: 'rgba(239, 68, 68, 0.35)',
			priceLineVisible: false
		});

		const initialCount = 10;
		series.setData(CANDLE_DATA.slice(0, initialCount));

		if (chart.timeScale()) {
			chart.timeScale().setVisibleLogicalRange({ from: 0, to: initialCount });
		}

		chartReady = true;
		startReplay(initialCount);
	}

	// ============================================================================
	// REPLAY ANIMATION
	// ============================================================================
	function startReplay(startIndex: number = 10): void {
		if (!browser || !series || !chart || !heroSection) return;

		if (replayInterval) {
			clearInterval(replayInterval);
			replayInterval = null;
		}

		let currentIndex = Math.max(startIndex, 1);
		const totalCandles = CANDLE_DATA.length;
		const chartWidth = heroSection?.clientWidth || window.innerWidth;
		const barsVisible = Math.floor(chartWidth / 5);

		replayInterval = setInterval(() => {
			if (!series || isLooping || !isVisible) return;

			currentIndex++;

			if (currentIndex >= totalCandles) {
				handleLoop();
				return;
			}

			series.setData(CANDLE_DATA.slice(0, currentIndex));

			if (chart?.timeScale()) {
				chart.timeScale().setVisibleLogicalRange({
					from: Math.max(0, currentIndex - barsVisible + 1),
					to: currentIndex + 1
				});
			}
		}, 600);
	}

	function handleLoop(): void {
		if (isLooping || !browser || !chartContainer || !gsapLib || !series) return;

		isLooping = true;
		isAnimating = true;

		const loopTL = gsapLib.timeline({
			onComplete: () => {
				isLooping = false;
				isAnimating = false;
				if (replayInterval) clearInterval(replayInterval);
				startReplay(10);
			}
		});

		loopTL
			.to(chartContainer, { opacity: 0, scale: 1.02, duration: 0.6, ease: 'power2.in' })
			.call(() => {
				series.setData(CANDLE_DATA.slice(0, 10));
				if (chart?.timeScale()) {
					chart.timeScale().setVisibleLogicalRange({ from: 0, to: 10 });
				}
			})
			.to(chartContainer, { opacity: 0.4, scale: 1, duration: 0.6, ease: 'power2.out' });
	}

	// ============================================================================
	// SLIDE ANIMATIONS - ENHANCED CINEMATIC TRANSITIONS
	// ============================================================================
	function animateSlideOut(slideIndex: number): Promise<void> {
		return new Promise((resolve) => {
			if (!gsapLib || slideIndex < 0) {
				resolve();
				return;
			}

			const slide = document.querySelector(`[data-slide="${slideIndex}"]`);
			if (!slide) {
				resolve();
				return;
			}

			const elements = slide.querySelectorAll('h1, h2, p, .slide__actions');

			gsapLib.to(elements, {
				opacity: 0,
				y: -30,
				scale: 0.95,
				filter: 'blur(8px)',
				duration: 0.4,
				stagger: 0.05,
				ease: 'power2.in',
				onComplete: resolve
			});
		});
	}

	function animateSlideIn(slideIndex: number): void {
		if (!browser || !gsapLib) return;

		const slide = document.querySelector(`[data-slide="${slideIndex}"]`);
		if (!slide) return;

		const h1 = slide.querySelector('h1');
		const h2 = slide.querySelector('h2');
		const p = slide.querySelector('p');
		const actions = slide.querySelector('.slide__actions');
		const buttons = slide.querySelectorAll('a');

		if (timeline) timeline.kill();

		isAnimating = true;

		timeline = gsapLib.timeline({
			onComplete: () => { isAnimating = false; }
		});

		// Reset elements first
		gsapLib.set([h1, h2, p, actions], {
			opacity: 0,
			y: 0,
			x: 0,
			scale: 1,
			rotation: 0,
			filter: 'blur(0px)'
		});

		// Each slide has unique cinematic entrance
		switch (slideIndex) {
			case 0: // Scale & Glow entrance
				timeline
					.fromTo(h1,
						{ opacity: 0, scale: 0.8, filter: 'blur(10px)' },
						{ opacity: 1, scale: 1, filter: 'blur(0px)', duration: 0.9, ease: 'power3.out' }, 0)
					.fromTo(h2,
						{ opacity: 0, y: 30, filter: 'blur(5px)' },
						{ opacity: 1, y: 0, filter: 'blur(0px)', duration: 0.7, ease: 'power2.out' }, 0.2)
					.fromTo(p,
						{ opacity: 0, y: 20 },
						{ opacity: 1, y: 0, duration: 0.6 }, 0.4)
					.fromTo(buttons,
						{ opacity: 0, y: 30, scale: 0.9 },
						{ opacity: 1, y: 0, scale: 1, duration: 0.7, stagger: 0.1, ease: 'back.out(1.2)' }, 0.55);
				break;

			case 1: // Elastic Drop entrance
				timeline
					.fromTo(h1,
						{ opacity: 0, y: -80, scale: 1.1 },
						{ opacity: 1, y: 0, scale: 1, duration: 1, ease: 'elastic.out(1, 0.5)' }, 0)
					.fromTo(h2,
						{ opacity: 0, y: -60, scale: 1.05 },
						{ opacity: 1, y: 0, scale: 1, duration: 0.9, ease: 'elastic.out(1, 0.6)' }, 0.1)
					.fromTo(p,
						{ opacity: 0, y: -40 },
						{ opacity: 1, y: 0, duration: 0.7, ease: 'power2.out' }, 0.25)
					.fromTo(buttons,
						{ opacity: 0, y: 40 },
						{ opacity: 1, y: 0, duration: 0.6, stagger: 0.1, ease: 'power2.out' }, 0.45);
				break;

			case 2: // Cinematic Zoom entrance
				timeline
					.fromTo(h1,
						{ opacity: 0, scale: 2, filter: 'blur(20px)' },
						{ opacity: 1, scale: 1, filter: 'blur(0px)', duration: 1.1, ease: 'power4.out' }, 0)
					.fromTo(h2,
						{ opacity: 0, scale: 1.5, filter: 'blur(10px)' },
						{ opacity: 1, scale: 1, filter: 'blur(0px)', duration: 0.9, ease: 'power3.out' }, 0.15)
					.fromTo(p,
						{ opacity: 0, x: -50 },
						{ opacity: 1, x: 0, duration: 0.7, ease: 'power2.out' }, 0.35)
					.fromTo(buttons,
						{ opacity: 0, scale: 0.5 },
						{ opacity: 1, scale: 1, duration: 0.7, stagger: 0.12, ease: 'back.out(2)' }, 0.5);
				break;

			case 3: // Slide & Reveal entrance
				timeline
					.fromTo(h1,
						{ opacity: 0, x: -100, skewX: -10 },
						{ opacity: 1, x: 0, skewX: 0, duration: 0.9, ease: 'power3.out' }, 0)
					.fromTo(h2,
						{ opacity: 0, x: 100, skewX: 10 },
						{ opacity: 1, x: 0, skewX: 0, duration: 0.9, ease: 'power3.out' }, 0.1)
					.fromTo(p,
						{ opacity: 0, y: 30 },
						{ opacity: 1, y: 0, duration: 0.6 }, 0.3)
					.fromTo(buttons,
						{ opacity: 0, x: -30 },
						{ opacity: 1, x: 0, duration: 0.6, stagger: 0.1, ease: 'power2.out' }, 0.45);
				break;
		}
	}

	async function showSlide(index: number): Promise<void> {
		if (isTransitioning) return;
		isTransitioning = true;

		// Animate out previous slide
		if (previousSlide >= 0 && gsapLoaded) {
			await animateSlideOut(previousSlide);
		}

		previousSlide = currentSlide;
		currentSlide = index;

		await tick();

		if (gsapLoaded) {
			animateSlideIn(index);
		}

		// Reset progress
		slideProgress = 0;

		isTransitioning = false;
	}

	function nextSlide(): void {
		const next = (currentSlide + 1) % SLIDES.length;
		showSlide(next);
	}

	function goToSlide(index: number): void {
		if (index !== currentSlide) {
			// Reset slide interval when manually clicking
			if (slideInterval) {
				clearInterval(slideInterval);
			}
			showSlide(index);
			// Restart interval
			slideInterval = setInterval(nextSlide, 7000);
		}
	}

	// ============================================================================
	// PROGRESS ANIMATION
	// ============================================================================
	function startProgressTimer(): void {
		if (progressInterval) clearInterval(progressInterval);
		slideProgress = 0;

		progressInterval = setInterval(() => {
			slideProgress += (100 / 70); // 7 seconds = 7000ms, update every 100ms
			if (slideProgress >= 100) {
				slideProgress = 0;
			}
		}, 100);
	}

	// ============================================================================
	// RESIZE & VISIBILITY
	// ============================================================================
	function handleResize(): void {
		if (!browser || !chart || !heroSection) return;

		chart.applyOptions({
			width: heroSection.clientWidth,
			height: heroSection.clientHeight
		});
	}

	function setupVisibilityObserver(): void {
		if (!browser || !heroSection) return;

		visibilityObserver = new IntersectionObserver(
			(entries) => {
				isVisible = entries[0]?.isIntersecting ?? true;
			},
			{ threshold: 0.1 }
		);

		visibilityObserver.observe(heroSection);
	}

	// ============================================================================
	// LIFECYCLE
	// ============================================================================
	onMount(async () => {
		if (!browser) return;

		mounted = true;
		currentSlide = 0;
		previousSlide = -1;

		if (heroSection && typeof ResizeObserver !== 'undefined') {
			resizeObserver = new ResizeObserver(handleResize);
			resizeObserver.observe(heroSection);
		}

		if (heroSection) {
			setupVisibilityObserver();
		}

		if (chartContainer && heroSection) {
			initChart();
		}

		try {
			const gsapModule = await import('gsap');
			gsapLib = gsapModule?.gsap || gsapModule?.default || null;
			if (gsapLib) {
				gsapLoaded = true;
				animateSlideIn(currentSlide);
			}
		} catch (e) {
			console.warn('GSAP failed to load:', e);
		}

		// Start slide rotation
		slideInterval = setInterval(nextSlide, 7000);
		startProgressTimer();
	});

	onDestroy(() => {
		if (slideInterval) clearInterval(slideInterval);
		if (replayInterval) clearInterval(replayInterval);
		if (progressInterval) clearInterval(progressInterval);
		if (timeline) timeline.kill();
		if (chart) chart.remove();
		if (resizeObserver) resizeObserver.disconnect();
		if (visibilityObserver) visibilityObserver.disconnect();

		chart = null;
		series = null;
		timeline = null;
		mounted = false;
	});
</script>

<section
	bind:this={heroSection}
	id="hero"
	class="hero-section"
	class:hero-animating={isAnimating}
>
	<!-- Ambient Background Layers -->
	<div class="hero-ambient" aria-hidden="true">
		<div class="ambient-gradient"></div>
		<div class="ambient-grid"></div>
		<div class="ambient-glow" style="--accent-color: {SLIDES[currentSlide].accentColor}"></div>
	</div>

	<!-- Chart Background -->
	<div
		id="chart-bg"
		bind:this={chartContainer}
		class="hero-chart"
		aria-hidden="true"
	></div>

	<!-- Vignette Overlay -->
	<div class="hero-vignette" aria-hidden="true"></div>

	<!-- Content Overlay -->
	<div class="hero-content">
		<div class="hero-slides" bind:this={slidesContainer}>
			{#each SLIDES as slide, i (i)}
				<article
					data-slide={i}
					class="slide"
					class:slide--hidden={i !== currentSlide}
					class:slide--active={i === currentSlide}
					aria-hidden={i !== currentSlide}
				>
					<h1 class="slide__title">{slide.title}</h1>
					<h2 class="slide__subtitle">{slide.subtitle}</h2>
					<p class="slide__description">{slide.description}</p>
					<div class="slide__actions">
						<a href={slide.primaryCTA.href} class="cta cta--primary">
							<span class="cta__text">{slide.primaryCTA.text}</span>
							<span class="cta__glow"></span>
						</a>
						<a href={slide.secondaryCTA.href} class="cta cta--secondary">
							<span class="cta__text">{slide.secondaryCTA.text}</span>
						</a>
					</div>
				</article>
			{/each}
		</div>

		<!-- Slide Indicators -->
		<nav class="slide-indicators" aria-label="Slide navigation">
			{#each SLIDES as _, i (i)}
				<button
					type="button"
					class="slide-dot"
					class:slide-dot--active={i === currentSlide}
					aria-label="Go to slide {i + 1}"
					aria-current={i === currentSlide ? 'true' : 'false'}
					onclick={() => goToSlide(i)}
				>
					<span class="slide-dot__fill" style="--progress: {i === currentSlide ? slideProgress : 0}%"></span>
				</button>
			{/each}
		</nav>
	</div>
</section>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════════
	 * CSS Custom Properties
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	:root {
		--hero-bg: #050a12;
		--hero-nav-height: 80px;
		--hero-chart-opacity: 0.4;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Hero Section Container
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.hero-section {
		position: relative;
		min-height: 100vh;
		min-height: 100dvh;
		padding-block-start: var(--hero-nav-height);
		background: var(--hero-bg);
		display: flex;
		align-items: center;
		justify-content: center;
		overflow: hidden;
		contain: layout style paint;
		content-visibility: auto;
		contain-intrinsic-size: 100vw 100vh;
	}

	.hero-animating .hero-chart,
	.hero-animating .slide {
		will-change: transform, opacity, filter;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Ambient Background Layers
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.hero-ambient {
		position: absolute;
		inset: 0;
		z-index: 0;
		pointer-events: none;
	}

	.ambient-gradient {
		position: absolute;
		inset: 0;
		background:
			radial-gradient(ellipse 120% 80% at 50% 20%, rgba(99, 102, 241, 0.1) 0%, transparent 50%),
			radial-gradient(ellipse 80% 50% at 80% 80%, rgba(139, 92, 246, 0.08) 0%, transparent 50%),
			radial-gradient(ellipse 60% 40% at 20% 60%, rgba(52, 211, 153, 0.06) 0%, transparent 50%);
	}

	.ambient-grid {
		position: absolute;
		inset: 0;
		background-image:
			linear-gradient(rgba(99, 102, 241, 0.03) 1px, transparent 1px),
			linear-gradient(90deg, rgba(99, 102, 241, 0.03) 1px, transparent 1px);
		background-size: 80px 80px;
		mask-image: radial-gradient(ellipse 80% 70% at 50% 50%, black 20%, transparent 70%);
		-webkit-mask-image: radial-gradient(ellipse 80% 70% at 50% 50%, black 20%, transparent 70%);
	}

	.ambient-glow {
		position: absolute;
		top: 50%;
		left: 50%;
		width: 100%;
		height: 100%;
		transform: translate(-50%, -50%);
		background: radial-gradient(ellipse 60% 40% at 50% 50%, var(--accent-color, rgba(99, 102, 241, 0.15)) 0%, transparent 60%);
		transition: background 1s ease;
		mix-blend-mode: screen;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Chart Background
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.hero-chart {
		position: absolute;
		inset: 0;
		z-index: 1;
		opacity: var(--hero-chart-opacity);
		mask-image: linear-gradient(
			to right,
			transparent 0%,
			black 10%,
			black 90%,
			transparent 100%
		);
		-webkit-mask-image: linear-gradient(
			to right,
			transparent 0%,
			black 10%,
			black 90%,
			transparent 100%
		);
		contain: strict;
		transform: translateZ(0);
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Vignette Overlay
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.hero-vignette {
		position: absolute;
		inset: 0;
		z-index: 2;
		background: radial-gradient(ellipse at center, transparent 0%, var(--hero-bg) 85%);
		pointer-events: none;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Content Layer
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.hero-content {
		position: relative;
		z-index: 10;
		width: 100%;
		max-width: 80rem;
		margin-inline: auto;
		padding-inline: 1.5rem;
		padding-block: 4rem;
		display: flex;
		flex-direction: column;
		align-items: center;
	}

	@media (min-width: 640px) {
		.hero-content {
			padding-inline: 2rem;
		}
	}

	@media (min-width: 1024px) {
		.hero-content {
			padding-inline: 3rem;
			padding-block: 5rem;
		}
	}

	.hero-slides {
		position: relative;
		min-height: 22rem;
		width: 100%;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Slides
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.slide {
		text-align: center;
		max-width: 56rem;
		margin-inline: auto;
		opacity: 1;
		transform: translateZ(0);
	}

	.slide--hidden {
		display: none !important;
		pointer-events: none;
	}

	.slide--active {
		display: block !important;
		pointer-events: auto;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Typography
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.slide__title {
		font-family: var(--font-heading, system-ui);
		font-size: clamp(2.5rem, 8vw, 5rem);
		font-weight: 800;
		line-height: 1.05;
		color: white;
		margin-block-end: 0.75rem;
		letter-spacing: -0.02em;
		text-shadow: 0 4px 30px rgba(0, 0, 0, 0.5);
		transform: translateZ(0);
	}

	.slide__subtitle {
		font-family: var(--font-heading, system-ui);
		font-size: clamp(1.25rem, 4vw, 2rem);
		font-weight: 600;
		background: linear-gradient(135deg, #facc15 0%, #f59e0b 50%, #fbbf24 100%);
		background-clip: text;
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		margin-block-end: 1.5rem;
		transform: translateZ(0);
	}

	.slide__description {
		font-size: clamp(1rem, 2.5vw, 1.25rem);
		line-height: 1.7;
		color: rgba(255, 255, 255, 0.8);
		max-width: 42rem;
		margin-inline: auto;
		margin-block-end: 2.5rem;
		transform: translateZ(0);
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * CTA Buttons
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.slide__actions {
		display: flex;
		flex-wrap: wrap;
		gap: 1rem;
		justify-content: center;
	}

	.cta {
		position: relative;
		display: inline-flex;
		align-items: center;
		justify-content: center;
		padding: 1rem 2.5rem;
		font-family: var(--font-heading, system-ui);
		font-size: 1rem;
		font-weight: 600;
		border-radius: 9999px;
		text-decoration: none;
		overflow: hidden;
		cursor: pointer;
		transform: translateY(0) translateZ(0);
		transition: transform 0.3s cubic-bezier(0.25, 0.8, 0.25, 1), box-shadow 0.3s ease;
	}

	.cta__text {
		position: relative;
		z-index: 2;
	}

	.cta--primary {
		background: linear-gradient(135deg, #facc15 0%, #f59e0b 100%);
		color: #1f2937;
		box-shadow:
			0 4px 20px rgba(250, 204, 21, 0.3),
			0 0 0 1px rgba(250, 204, 21, 0.1);
	}

	.cta--primary .cta__glow {
		position: absolute;
		inset: 0;
		background: linear-gradient(135deg, #fde047 0%, #facc15 100%);
		opacity: 0;
		transition: opacity 0.3s ease;
	}

	.cta--primary:hover {
		transform: translateY(-3px) translateZ(0);
		box-shadow:
			0 8px 30px rgba(250, 204, 21, 0.5),
			0 0 0 1px rgba(250, 204, 21, 0.2);
	}

	.cta--primary:hover .cta__glow {
		opacity: 1;
	}

	.cta--primary:active {
		transform: translateY(-1px) scale(0.98) translateZ(0);
	}

	.cta--secondary {
		background: rgba(255, 255, 255, 0.05);
		color: white;
		border: 2px solid rgba(255, 255, 255, 0.2);
		backdrop-filter: blur(8px);
		-webkit-backdrop-filter: blur(8px);
	}

	.cta--secondary:hover {
		background: rgba(255, 255, 255, 0.1);
		border-color: rgba(255, 255, 255, 0.35);
		transform: translateY(-3px) translateZ(0);
		box-shadow: 0 8px 25px rgba(0, 0, 0, 0.3);
	}

	.cta:focus-visible {
		outline: 2px solid #facc15;
		outline-offset: 3px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Slide Indicators
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.slide-indicators {
		display: flex;
		gap: 0.75rem;
		margin-top: 3rem;
	}

	.slide-dot {
		position: relative;
		width: 40px;
		height: 4px;
		background: rgba(255, 255, 255, 0.2);
		border: none;
		border-radius: 4px;
		cursor: pointer;
		overflow: hidden;
		transition: background 0.3s ease, transform 0.2s ease;
	}

	.slide-dot:hover {
		background: rgba(255, 255, 255, 0.3);
		transform: scaleY(1.5);
	}

	.slide-dot--active {
		background: rgba(255, 255, 255, 0.15);
	}

	.slide-dot__fill {
		position: absolute;
		top: 0;
		left: 0;
		height: 100%;
		width: var(--progress, 0%);
		background: linear-gradient(90deg, #facc15, #f59e0b);
		border-radius: 4px;
		transition: width 0.1s linear;
	}

	.slide-dot:focus-visible {
		outline: 2px solid #facc15;
		outline-offset: 2px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Responsive
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	@media (max-width: 768px) {
		.cta {
			padding: 0.875rem 2rem;
			font-size: 0.9rem;
		}

		.hero-slides {
			min-height: 20rem;
		}
	}

	@media (max-width: 640px) {
		.hero-section {
			padding-block-start: 60px;
		}

		.hero-content {
			padding-block: 3rem;
		}

		.slide__actions {
			flex-direction: column;
			align-items: center;
		}

		.cta {
			width: 100%;
			max-width: 280px;
		}

		.hero-slides {
			min-height: 24rem;
		}

		.slide-indicators {
			margin-top: 2rem;
		}

		.slide-dot {
			width: 32px;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Reduced Motion
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	@media (prefers-reduced-motion: reduce) {
		.slide,
		.cta,
		.hero-chart,
		.ambient-glow,
		.slide-dot {
			transition: none;
		}

		.slide-dot__fill {
			transition: none;
		}
	}
</style>

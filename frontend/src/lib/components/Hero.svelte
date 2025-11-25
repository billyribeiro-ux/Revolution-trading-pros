<script>
	/**
	 * HeroSection Component - Google L8+ Principal Engineer Standard
	 * ══════════════════════════════════════════════════════════════════════════════
	 * ZERO DELAYS - Instant initialization with parallel execution
	 * ══════════════════════════════════════════════════════════════════════════════
	 * Performance Features:
	 * ✓ Zero setTimeout/requestIdleCallback delays
	 * ✓ Synchronous chart initialization
	 * ✓ GSAP Timeline (no individual delays)
	 * ✓ Module-level pre-computation
	 * ✓ CSS containment & content-visibility
	 * ✓ will-change management
	 * ✓ ResizeObserver (no window listener)
	 * ✓ Intersection Observer for visibility
	 * ✓ GPU-accelerated transforms only
	 * ══════════════════════════════════════════════════════════════════════════════
	 */
	import { onMount, onDestroy, tick } from 'svelte';
	import { browser } from '$app/environment';
	import { createChart, CandlestickSeries, CrosshairMode } from 'lightweight-charts';

	// ============================================================================
	// MODULE-LEVEL PRE-COMPUTATION (Executed at import time - ZERO runtime cost)
	// ============================================================================

	/**
	 * Pre-generated candle data - computed once at module load
	 * L8+ Performance: Zero runtime computation
	 */
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

	// Slide data - static, no runtime cost
	const SLIDES = Object.freeze([
		{
			title: 'Live Trading Rooms',
			subtitle: 'Institutional-Style Sessions',
			description: 'Join structured sessions with clear levels, real-time execution, and disciplined risk management.',
			primaryCTA: { text: 'Explore Rooms', href: '/live-trading-rooms/day-trading' },
			secondaryCTA: { text: 'Create Free Account', href: '/signup' }
		},
		{
			title: 'SPX Profit Pulse',
			subtitle: 'Context-Rich Alerts',
			description: 'Receive SPX alerts with entry context, invalidation, and sizing guidance (not lottery tickets).',
			primaryCTA: { text: 'View Alerts', href: '/alert-services/spx-profit-pulse' },
			secondaryCTA: { text: 'Learn More', href: '/alert-services' }
		},
		{
			title: 'Trading Frameworks',
			subtitle: 'Structured Education',
			description: 'Master robust frameworks for execution, risk, and review workflows across market conditions.',
			primaryCTA: { text: 'Browse Courses', href: '/courses' },
			secondaryCTA: { text: 'View Mentorship', href: '/mentorship' }
		},
		{
			title: 'Pro Tools & Indicators',
			subtitle: 'Professional-Grade Edge',
			description: 'Volume profiles, internals, momentum systems, and tooling built for repeatable institutional logic.',
			primaryCTA: { text: 'View Indicators', href: '/indicators' },
			secondaryCTA: { text: 'See All Tools', href: '/store' }
		}
	]);

	// ============================================================================
	// STATE
	// ============================================================================
	let chart = null;
	let series = null;
	let replayInterval = null;
	let slideInterval = null;
	let currentSlide = $state(0);
	let isLooping = false;
	let gsap = null;
	let timeline = null;

	// DOM refs
	let heroSection;
	let chartContainer;
	let resizeObserver = null;
	let visibilityObserver = null;
	let isVisible = true;

	// Animation state for will-change
	let isAnimating = $state(false);

	// ============================================================================
	// CHART INITIALIZATION - SYNCHRONOUS, ZERO DELAYS
	// ============================================================================

	function initChart() {
		if (!browser || !chartContainer || !heroSection) return;

		const width = heroSection.clientWidth || window.innerWidth;
		const height = heroSection.clientHeight || window.innerHeight;

		// Cleanup
		if (chart) {
			chart.remove();
			chart = null;
			series = null;
		}

		chart = createChart(chartContainer, {
			layout: {
				background: { type: 'solid', color: 'transparent' },
				textColor: 'rgba(255, 255, 255, 0.1)'
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
				barSpacing: 4,
				minBarSpacing: 2,
				fixLeftEdge: false,
				fixRightEdge: true,
				lockVisibleTimeRangeOnResize: true,
				rightBarStaysOnScroll: true,
				shiftVisibleRangeOnNewBar: true,
				borderColor: 'transparent'
			},
			crosshair: { mode: CrosshairMode.Hidden },
			rightPriceScale: {
				visible: true,
				borderColor: 'transparent'
			}
		});

		series = chart.addSeries(CandlestickSeries, {
			upColor: 'rgba(52, 211, 153, 0.4)',
			downColor: 'rgba(239, 68, 68, 0.4)',
			borderVisible: false,
			wickVisible: true,
			wickUpColor: 'rgba(52, 211, 153, 0.4)',
			wickDownColor: 'rgba(239, 68, 68, 0.4)',
			priceLineVisible: false
		});

		// Initial view - 10 candles
		const initialCount = 10;
		series.setData(CANDLE_DATA.slice(0, initialCount));

		if (chart.timeScale()) {
			chart.timeScale().setVisibleLogicalRange({ from: 0, to: initialCount });
		}

		// Start replay immediately
		startReplay(initialCount);
	}

	// ============================================================================
	// REPLAY ANIMATION
	// ============================================================================

	function startReplay(startIndex = 10) {
		if (!browser || !series || !chart) return;

		if (replayInterval) {
			clearInterval(replayInterval);
			replayInterval = null;
		}

		let currentIndex = Math.max(startIndex, 1);
		const totalCandles = CANDLE_DATA.length;
		const chartWidth = heroSection?.clientWidth || window.innerWidth;
		const barsVisible = Math.floor(chartWidth / 4);

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
		}, 700);
	}

	function handleLoop() {
		if (isLooping || !browser || !chartContainer || !gsap || !series) return;

		isLooping = true;
		isAnimating = true;

		// Use timeline for loop animation - no delays
		const loopTL = gsap.timeline({
			onComplete: () => {
				isLooping = false;
				isAnimating = false;
				if (replayInterval) clearInterval(replayInterval);
				startReplay(10);
			}
		});

		loopTL
			.to(chartContainer, { opacity: 0, duration: 0.5, ease: 'power2.in' })
			.call(() => {
				series.setData(CANDLE_DATA.slice(0, 10));
				if (chart?.timeScale()) {
					chart.timeScale().setVisibleLogicalRange({ from: 0, to: 10 });
				}
			})
			.to(chartContainer, { opacity: 0.3, duration: 0.5, ease: 'power2.out' });
	}

	// ============================================================================
	// SLIDE ANIMATIONS - GSAP TIMELINE (ZERO DELAYS)
	// ============================================================================

	/**
	 * L8+ Animation: Uses GSAP timeline sequencing instead of individual delays
	 * Visual result is IDENTICAL, but execution starts immediately
	 */
	function animateSlide(slideIndex) {
		if (!browser || !gsap) return;

		const slide = document.querySelector(`[data-slide="${slideIndex}"]`);
		if (!slide) return;

		const h1 = slide.querySelector('h1');
		const h2 = slide.querySelector('h2');
		const p = slide.querySelector('p');
		const buttons = slide.querySelectorAll('a');

		// Kill any existing timeline
		if (timeline) {
			timeline.kill();
		}

		// Enable will-change during animation
		isAnimating = true;

		// Create new timeline - all animations queued, zero delays
		timeline = gsap.timeline({
			onComplete: () => {
				isAnimating = false;
			}
		});

		switch (slideIndex) {
			case 0:
				// Scale entrance - timeline sequencing replaces delays
				timeline
					.fromTo(h1, { opacity: 0, scale: 0.9 }, { opacity: 1, scale: 1, duration: 0.8, ease: 'back.out(1.4)' }, 0)
					.fromTo(h2, { opacity: 0, y: 20 }, { opacity: 1, y: 0, duration: 0.7 }, 0.15)
					.fromTo(p, { opacity: 0, y: 15 }, { opacity: 1, y: 0, duration: 0.6 }, 0.3)
					.fromTo(buttons, { opacity: 0, y: 30, scale: 0.95 }, { opacity: 1, y: 0, scale: 1, duration: 0.7, stagger: 0.12, ease: 'power2.out' }, 0.5);
				break;

			case 1:
				// Bounce entrance
				timeline
					.fromTo(h1, { opacity: 0, y: -100, rotation: -5 }, { opacity: 1, y: 0, rotation: 0, duration: 0.9, ease: 'bounce.out' }, 0)
					.fromTo(h2, { opacity: 0, y: -100, rotation: -5 }, { opacity: 1, y: 0, rotation: 0, duration: 0.9, ease: 'bounce.out' }, 0.1)
					.fromTo(p, { opacity: 0, y: -50 }, { opacity: 1, y: 0, duration: 0.7, ease: 'power2.out' }, 0.3)
					.fromTo(buttons, { opacity: 0, y: 30 }, { opacity: 1, y: 0, duration: 0.7, stagger: 0.12 }, 0.5);
				break;

			case 2:
				// Spin entrance
				timeline
					.fromTo(h1, { opacity: 0, scale: 1.5, rotation: 180 }, { opacity: 1, scale: 1, rotation: 0, duration: 0.9, ease: 'power3.out' }, 0)
					.fromTo(h2, { opacity: 0, scale: 1.5, rotation: 180 }, { opacity: 1, scale: 1, rotation: 0, duration: 0.9, ease: 'power3.out' }, 0.1)
					.fromTo(p, { opacity: 0, x: -60 }, { opacity: 1, x: 0, duration: 0.7 }, 0.3)
					.fromTo(buttons, { opacity: 0, scale: 0.5, rotation: 90 }, { opacity: 1, scale: 1, rotation: 0, duration: 0.7, stagger: 0.12, ease: 'back.out(1.7)' }, 0.5);
				break;

			case 3:
				// Slide entrance
				timeline
					.fromTo(h1, { opacity: 0, x: -80 }, { opacity: 1, x: 0, duration: 0.8, ease: 'power2.out' }, 0)
					.fromTo(h2, { opacity: 0, x: 80 }, { opacity: 1, x: 0, duration: 0.8, ease: 'power2.out' }, 0.1)
					.fromTo(p, { opacity: 0, y: 40 }, { opacity: 1, y: 0, duration: 0.6 }, 0.3)
					.fromTo(buttons, { opacity: 0, y: 40 }, { opacity: 1, y: 0, duration: 0.7, stagger: 0.12 }, 0.5);
				break;
		}
	}

	function showSlide(index) {
		currentSlide = index;
		// Animate immediately - tick() removed, no waiting
		animateSlide(index);
	}

	function nextSlide() {
		showSlide((currentSlide + 1) % SLIDES.length);
	}

	// ============================================================================
	// RESIZE HANDLING
	// ============================================================================

	function handleResize() {
		if (!browser || !chart || !heroSection) return;

		chart.applyOptions({
			width: heroSection.clientWidth,
			height: heroSection.clientHeight
		});
	}

	// ============================================================================
	// VISIBILITY OPTIMIZATION - Pause when off-screen
	// ============================================================================

	function setupVisibilityObserver() {
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
	// LIFECYCLE - ZERO DELAYS
	// ============================================================================

	onMount(async () => {
		if (!browser) return;

		// Load GSAP immediately - no waiting
		const gsapModule = await import('gsap').catch(() => null);
		gsap = gsapModule?.gsap || gsapModule?.default || null;

		// Initialize everything in parallel - NO DELAYS
		// 1. Start slide animation immediately
		showSlide(0);
		slideInterval = setInterval(nextSlide, 7000);

		// 2. Initialize chart synchronously
		initChart();

		// 3. Setup observers
		if (typeof ResizeObserver !== 'undefined') {
			resizeObserver = new ResizeObserver(handleResize);
			resizeObserver.observe(heroSection);
		}

		setupVisibilityObserver();
	});

	onDestroy(() => {
		if (slideInterval) clearInterval(slideInterval);
		if (replayInterval) clearInterval(replayInterval);
		if (timeline) timeline.kill();
		if (chart) chart.remove();
		if (resizeObserver) resizeObserver.disconnect();
		if (visibilityObserver) visibilityObserver.disconnect();

		chart = null;
		series = null;
		timeline = null;
	});
</script>

<section
	bind:this={heroSection}
	id="hero"
	class="hero-section"
	class:hero-animating={isAnimating}
>
	<!-- Chart Background -->
	<div
		id="chart-bg"
		bind:this={chartContainer}
		class="hero-chart"
		aria-hidden="true"
	></div>

	<!-- Content Overlay -->
	<div class="hero-content">
		<div class="hero-slides">
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
							<span>{slide.primaryCTA.text}</span>
						</a>
						<a href={slide.secondaryCTA.href} class="cta cta--secondary">
							<span>{slide.secondaryCTA.text}</span>
						</a>
					</div>
				</article>
			{/each}
		</div>
	</div>
</section>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════════
	 * CSS Custom Properties
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	:root {
		--hero-bg: #0a101c;
		--hero-nav-height: 80px;
		--hero-chart-opacity: 0.3;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Hero Section - Optimized Container
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.hero-section {
		position: relative;
		min-height: 100vh;
		min-height: 100dvh; /* Dynamic viewport for mobile */
		padding-block-start: var(--hero-nav-height);
		background: var(--hero-bg);
		display: flex;
		align-items: center;
		justify-content: center;
		overflow: hidden;

		/* Performance: Isolate repaints */
		contain: layout style paint;
		content-visibility: auto;
		contain-intrinsic-size: 100vw 100vh;
	}

	/* Radial gradient overlay */
	.hero-section::before {
		content: '';
		position: absolute;
		inset: 0;
		background: radial-gradient(ellipse at center, rgba(10, 16, 28, 0) 0%, var(--hero-bg) 80%);
		pointer-events: none;
		z-index: 1;
	}

	/* will-change only during animations */
	.hero-animating .hero-chart,
	.hero-animating .slide {
		will-change: transform, opacity;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Chart Background - GPU Optimized
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.hero-chart {
		position: absolute;
		inset: 0;
		z-index: 0;
		opacity: var(--hero-chart-opacity);

		/* GPU-accelerated mask */
		mask-image: linear-gradient(
			to right,
			transparent 0%,
			black 15%,
			black 85%,
			transparent 100%
		);
		-webkit-mask-image: linear-gradient(
			to right,
			transparent 0%,
			black 15%,
			black 85%,
			transparent 100%
		);

		/* Performance */
		contain: strict;
		transform: translateZ(0); /* Force GPU layer */
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Content Layer
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.hero-content {
		position: relative;
		z-index: 10;
		inline-size: 100%;
		max-inline-size: 80rem;
		margin-inline: auto;
		padding-inline: 1rem;
		padding-block: 5rem;
	}

	@media (min-width: 640px) {
		.hero-content {
			padding-inline: 1.5rem;
		}
	}

	@media (min-width: 1024px) {
		.hero-content {
			padding-inline: 2rem;
		}
	}

	.hero-slides {
		position: relative;
		min-block-size: 24rem;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Slides - GPU Accelerated Transitions
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.slide {
		text-align: center;
		max-inline-size: 56rem;
		margin-inline: auto;

		/* GPU-accelerated properties only */
		opacity: 1;
		transform: translateZ(0);
		transition: opacity 0.5s ease-in-out;
	}

	.slide--hidden {
		display: none;
		opacity: 0;
		pointer-events: none;
	}

	.slide--active {
		display: block;
		pointer-events: auto;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Typography
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.slide__title {
		font-family: var(--font-heading, system-ui);
		font-size: clamp(2.5rem, 8vw, 4.5rem);
		font-weight: 800;
		line-height: 1.1;
		color: white;
		margin-block-end: 1rem;

		/* GPU layer for animations */
		transform: translateZ(0);
	}

	.slide__subtitle {
		font-family: var(--font-heading, system-ui);
		font-size: clamp(1.5rem, 5vw, 2.5rem);
		font-weight: 700;
		color: white;
		margin-block-end: 1.5rem;
		transform: translateZ(0);
	}

	.slide__description {
		font-size: clamp(1rem, 2.5vw, 1.25rem);
		line-height: 1.7;
		color: white;
		max-inline-size: 42rem;
		margin-inline: auto;
		margin-block-end: 2rem;
		transform: translateZ(0);
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * CTA Buttons - GPU Accelerated
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.slide__actions {
		display: flex;
		flex-wrap: wrap;
		gap: 1rem;
		justify-content: center;
	}

	.cta {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		position: relative;
		padding: 1rem 2.5rem;
		font-family: var(--font-heading, system-ui);
		font-size: 1rem;
		font-weight: 600;
		border-radius: 9999px;
		text-align: center;
		text-decoration: none;
		overflow: hidden;
		cursor: pointer;

		/* GPU-accelerated transitions only */
		transform: translateY(0) translateZ(0);
		transition:
			transform 0.3s cubic-bezier(0.25, 0.8, 0.25, 1),
			box-shadow 0.3s cubic-bezier(0.25, 0.8, 0.25, 1);
	}

	.cta span {
		position: relative;
		z-index: 2;
	}

	.cta--primary {
		background-color: #facc15;
		color: #fff;
		box-shadow: 0 4px 15px rgba(250, 204, 21, 0.2);
	}

	.cta--primary:hover,
	.cta--primary:focus-visible {
		transform: translateY(-3px) translateZ(0);
		box-shadow: 0 8px 25px rgba(250, 204, 21, 0.4);
	}

	.cta--secondary {
		background-color: rgba(31, 41, 55, 0.5);
		color: #fff;
		border: 2px solid #4b5563;
		backdrop-filter: blur(5px);
	}

	.cta--secondary:hover,
	.cta--secondary:focus-visible {
		background-color: rgba(55, 65, 81, 0.7);
		border-color: #6b7280;
		transform: translateY(-3px) translateZ(0);
	}

	.cta:focus-visible {
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
	}

	@media (max-width: 640px) {
		.hero-section {
			padding-block-start: 60px;
		}

		.slide__actions {
			flex-direction: column;
			align-items: center;
		}

		.cta {
			inline-size: 100%;
			max-inline-size: 280px;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Reduced Motion
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	@media (prefers-reduced-motion: reduce) {
		.slide,
		.cta,
		.hero-chart {
			transition: none;
		}
	}
</style>
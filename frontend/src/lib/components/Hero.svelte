<script>
	import { onMount, onDestroy, tick } from 'svelte';
	import { browser } from '$app/environment';
	import { createChart, CandlestickSeries, CrosshairMode } from 'lightweight-charts';

	// ============================================================================
	// STATE MANAGEMENT - L68+ Standards
	// ============================================================================

	// Chart state
	let chart = null;
	let series = null;
	let candles = [];

	// Animation state
	let replayInterval = null;
	let slideInterval = null;
	let currentSlide = 0;
	let isLooping = false;
	let isInitialized = false;

	// External libraries
	let gsap = null;

	// DOM references
	let heroSection;
	let chartContainer;
	let resizeObserver = null;

	const slides = [
		{
			title: 'Live Trading Rooms',
			subtitle: 'Institutional-Style Sessions',
			description:
				'Join structured sessions with clear levels, real-time execution, and disciplined risk management.',
			primaryCTA: { text: 'Explore Rooms', href: '/live-trading-rooms/day-trading' },
			secondaryCTA: { text: 'Create Free Account', href: '/signup' }
		},
		{
			title: 'SPX Profit Pulse',
			subtitle: 'Context-Rich Alerts',
			description:
				'Receive SPX alerts with entry context, invalidation, and sizing guidance (not lottery tickets).',
			primaryCTA: { text: 'View Alerts', href: '/alert-services/spx-profit-pulse' },
			secondaryCTA: { text: 'Learn More', href: '/alert-services' }
		},
		{
			title: 'Trading Frameworks',
			subtitle: 'Structured Education',
			description:
				'Master robust frameworks for execution, risk, and review workflows across market conditions.',
			primaryCTA: { text: 'Browse Courses', href: '/courses' },
			secondaryCTA: { text: 'View Mentorship', href: '/mentorship' }
		},
		{
			title: 'Pro Tools & Indicators',
			subtitle: 'Professional-Grade Edge',
			description:
				'Volume profiles, internals, momentum systems, and tooling built for repeatable institutional logic.',
			primaryCTA: { text: 'View Indicators', href: '/indicators' },
			secondaryCTA: { text: 'See All Tools', href: '/store' }
		}
	];

	/**
	 * L70+ Microsoft Principal: Realistic Market Microstructure
	 * REALISTIC RANGE-BOUND: Mixed directional moves with chop
	 */
	const PREGENERATED_CANDLES = (() => {
		const generatedCandles = [];
		const baseTime = 1704715800; // 9:30 AM EST market open

		// Range-bound parameters
		const rangeCenter = 475.5;
		const rangeHigh = 476.1;
		const rangeLow = 474.9;
		const rangeWidth = rangeHigh - rangeLow;

		let previousClose = rangeCenter;
		let miniTrend = 0; // Small trend bias that changes

		// Generate 220 candles with realistic chop
		for (let i = 0; i < 220; i++) {
			// Change direction randomly every 5-10 candles
			if (i % 7 === 0) {
				miniTrend = (Math.random() - 0.5) * 0.15;
			}

			// Decay mini trend
			miniTrend *= 0.85;

			// Random walk with bias
			const randomMove = (Math.random() - 0.5) * 0.12;

			// Pull toward range center to keep contained
			const distanceFromCenter = previousClose - rangeCenter;
			const pullToCenter = -distanceFromCenter * 0.2;

			// Occasional bigger moves (institutional prints)
			const bigMove = Math.random() < 0.1 ? (Math.random() - 0.5) * 0.2 : 0;

			// Calculate price move
			const priceMove = miniTrend + randomMove + pullToCenter + bigMove;

			// OHLC construction
			const open = previousClose;
			let close = open + priceMove;

			// Hard clamp to range
			close = Math.max(rangeLow, Math.min(rangeHigh, close));

			// Apple-style minimal wicks - barely visible
			const bodySize = Math.abs(close - open);

			// Extremely small wicks like Apple intraday charts
			// Just 5-10% of body size, or 0.01 minimum for doji candles
			const wickRange = Math.max(bodySize * 0.08, 0.01);

			let high = Math.max(open, close) + wickRange * Math.random();
			let low = Math.min(open, close) - wickRange * Math.random();

			// Clamp wicks to range
			high = Math.min(rangeHigh, high);
			low = Math.max(rangeLow, low);

			generatedCandles.push({
				time: baseTime + i * 60,
				open: parseFloat(open.toFixed(2)),
				high: parseFloat(high.toFixed(2)),
				low: parseFloat(low.toFixed(2)),
				close: parseFloat(close.toFixed(2))
			});

			previousClose = close;
		}

		return generatedCandles;
	})();

	/**
	 * Generate Apple-style intraday 1-minute candles with ZERO gaps
	 * L68+ Performance: Returns pre-generated data instantly
	 */
	function generateIntradayCandles() {
		return PREGENERATED_CANDLES;
	}

	/**
	 * Enterprise QA: Validate candle data integrity
	 * L68+ Performance: Silent validation, no console spam
	 */
	function validateCandleIntegrity() {
		let priceGaps = 0;
		let timeGaps = 0;

		for (let i = 1; i < candles.length; i++) {
			if (Math.abs(candles[i].open - candles[i - 1].close) > 0.001) priceGaps++;
			if (candles[i].time - candles[i - 1].time !== 60) timeGaps++;
		}

		// Only log errors, not success (reduces console noise)
		if (priceGaps > 0 || timeGaps > 0) {
			console.error(`⚠ Validation: ${priceGaps} price gaps, ${timeGaps} time gaps`);
		}
	}

	/**
	 * Initialize TradingView Lightweight Chart
	 * L68+ Performance: Fast initialization with minimal logging
	 */
	function initChart() {
		if (!browser || !chartContainer) return;

		const width = heroSection ? heroSection.clientWidth : window.innerWidth;
		const height = heroSection ? heroSection.clientHeight : window.innerHeight;

		// Clean up existing chart
		if (chart) {
			chart.remove();
			chart = null;
			series = null;
		}

		try {
			// V5 API: createChart with container element
			chart = createChart(chartContainer, {
				layout: {
					background: { type: 'solid', color: 'transparent' },
					textColor: 'rgba(255, 255, 255, 0.1)'
				},
				grid: {
					vertLines: { visible: false },
					horzLines: { visible: false }
				},
				width: width,
				height: height,
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
				crosshair: {
					mode: CrosshairMode.Hidden
				},
				rightPriceScale: {
					visible: true,
					borderColor: 'transparent'
				}
			});

			// V5 API: The correct pattern IS chart.addSeries(CandlestickSeries, options)
			series = chart.addSeries(CandlestickSeries, {
				upColor: 'rgba(52, 211, 153, 0.4)', // More transparent
				downColor: 'rgba(239, 68, 68, 0.4)', // More transparent
				borderVisible: false,
				wickVisible: true,
				wickUpColor: 'rgba(52, 211, 153, 0.4)',
				wickDownColor: 'rgba(239, 68, 68, 0.4)',
				priceLineVisible: false
			});

			// Generate gapless candles (instant - pre-generated)
			candles = generateIntradayCandles();

			// Validate silently
			validateCandleIntegrity();

			// L70+ UX: Start with 8-12 candles (realistic terminal view on load)
			// Enough to show recent context without overwhelming initial load
			const initialCandleCount = 10;

			// Set initial data - realistic starting view
			series.setData(candles.slice(0, initialCandleCount));

			// Set visible range - NO WHITE SPACE, candles go to edge
			if (chart && chart.timeScale() && initialCandleCount > 0) {
				chart.timeScale().setVisibleLogicalRange({
					from: 0,
					to: initialCandleCount
				});
			}

			// Start animation with realistic cadence
			startCenteredReplay(initialCandleCount);
		} catch (error) {
			console.error('Chart initialization error:', error);
		}
	}

	/**
	 * Start replay animation from CENTER of viewport
	 * L68+ Performance: Minimal logging, maximum speed
	 */
	function startCenteredReplay(startIndex = 1) {
		if (!browser || !series || !candles.length || !chart) return;

		// Clear any existing animation
		if (replayInterval) {
			clearInterval(replayInterval);
			replayInterval = null;
		}

		let currentIndex = Math.max(startIndex, 1);
		const totalCandles = candles.length;

		// Calculate viewport dimensions
		const chartWidth = heroSection ? heroSection.clientWidth : window.innerWidth;
		const barSpacing = 4;
		const barsVisible = Math.floor(chartWidth / barSpacing);

		// 2 candles per 1.4 seconds = 700ms interval
		replayInterval = setInterval(() => {
			if (!series || !candles.length || isLooping) return;

			currentIndex++;

			// Handle seamless loop when reaching end
			if (currentIndex >= totalCandles) {
				handleSeamlessLoop();
				return;
			}

			// Progressive display
			series.setData(candles.slice(0, currentIndex));

			// Smooth scrolling - candles go right to the edge
			if (chart && chart.timeScale()) {
				chart.timeScale().setVisibleLogicalRange({
					from: Math.max(0, currentIndex - barsVisible + 1),
					to: currentIndex + 1
				});
			}
		}, 700); // 700ms = 2 candles per 1.4 seconds
	}

	/**
	 * Handle seamless loop with fade transition for invisibility
	 * L70+ UX: Professional fade timing that feels natural
	 */
	function handleSeamlessLoop() {
		if (isLooping || !browser || !chartContainer || !gsap || !series || !candles.length) return;

		isLooping = true;

		// Quick professional fade (500ms out)
		gsap.to(chartContainer, {
			opacity: 0,
			duration: 0.5,
			ease: 'power2.in',
			onComplete: () => {
				// Reset to realistic starting view - 10 candles
				const initialCandleCount = 10;

				series.setData(candles.slice(0, initialCandleCount));

				if (chart && chart.timeScale()) {
					chart.timeScale().setVisibleLogicalRange({
						from: 0,
						to: initialCandleCount
					});
				}

				// Fade back in smoothly (500ms in)
				gsap.to(chartContainer, {
					opacity: 0.3,
					duration: 0.5,
					ease: 'power2.out',
					onComplete: () => {
						isLooping = false;
						if (replayInterval) clearInterval(replayInterval);
						startCenteredReplay(initialCandleCount);
					}
				});
			}
		});
	}

	/**
	 * Handle responsive resize
	 */
	function handleResize() {
		if (!browser || !chart) return;

		const newWidth = heroSection ? heroSection.clientWidth : window.innerWidth;
		const newHeight = heroSection ? heroSection.clientHeight : window.innerHeight;

		chart.applyOptions({
			width: newWidth,
			height: newHeight
		});

		// Recalculate visible range after resize
		if (chart.timeScale() && series) {
			chart.timeScale().applyOptions({
				rightOffset: 0
			});
		}
	}

	/**
	 * Slide management (unchanged from original)
	 */
	async function showSlide(index) {
		currentSlide = index;
		await tick();
		animateSlide(index);
	}

	function nextSlide() {
		const next = (currentSlide + 1) % slides.length;
		showSlide(next);
	}

	function animateSlide(slideIndex) {
		if (!browser || !gsap) return;

		const slide = document.querySelector(`[data-slide="${slideIndex}"]`);
		if (!slide) return;

		const h1 = slide.querySelector('h1');
		const h2 = slide.querySelector('h2');
		const p = slide.querySelector('p');
		const buttons = slide.querySelectorAll('a');

		switch (slideIndex) {
			case 0:
				if (h1)
					gsap.fromTo(
						h1,
						{ opacity: 0, scale: 0.8 },
						{ opacity: 1, scale: 1, duration: 1.6, ease: 'back.out(1.7)' }
					);
				if (h2)
					gsap.fromTo(h2, { opacity: 0, y: 30 }, { opacity: 1, y: 0, duration: 1.2, delay: 0.4 });
				if (p)
					gsap.fromTo(p, { opacity: 0, y: 20 }, { opacity: 1, y: 0, duration: 1.0, delay: 0.7 });
				if (buttons.length > 0) {
					gsap.fromTo(
						buttons,
						{ opacity: 0, y: 48, scale: 0.95 },
						{
							opacity: 1,
							y: 0,
							scale: 1,
							duration: 1.2,
							delay: 1.0,
							stagger: 0.22,
							ease: 'power2.out'
						}
					);
				}
				break;

			case 1:
				if (h1)
					gsap.fromTo(
						h1,
						{ opacity: 0, y: -200, rotation: -10 },
						{ opacity: 1, y: 0, rotation: 0, duration: 1.6, ease: 'bounce.out' }
					);
				if (h2)
					gsap.fromTo(
						h2,
						{ opacity: 0, y: -200, rotation: -10 },
						{ opacity: 1, y: 0, rotation: 0, duration: 1.6, ease: 'bounce.out', delay: 0.2 }
					);
				if (p)
					gsap.fromTo(
						p,
						{ opacity: 0, y: -100 },
						{ opacity: 1, y: 0, duration: 1.2, delay: 0.7, ease: 'power2.out' }
					);
				if (buttons.length > 0)
					gsap.fromTo(
						buttons,
						{ opacity: 0, y: 50 },
						{ opacity: 1, y: 0, duration: 1.2, delay: 1.0, stagger: 0.22 }
					);
				break;

			case 2:
				if (h1)
					gsap.fromTo(
						h1,
						{ opacity: 0, scale: 2, rotation: 360 },
						{ opacity: 1, scale: 1, rotation: 0, duration: 1.6, ease: 'power3.out' }
					);
				if (h2)
					gsap.fromTo(
						h2,
						{ opacity: 0, scale: 2, rotation: 360 },
						{ opacity: 1, scale: 1, rotation: 0, duration: 1.6, ease: 'power3.out', delay: 0.2 }
					);
				if (p)
					gsap.fromTo(p, { opacity: 0, x: -100 }, { opacity: 1, x: 0, duration: 1.2, delay: 0.6 });
				if (buttons.length > 0)
					gsap.fromTo(
						buttons,
						{ opacity: 0, scale: 0, rotation: 180 },
						{
							opacity: 1,
							scale: 1,
							rotation: 0,
							duration: 1.2,
							delay: 1.0,
							stagger: 0.22,
							ease: 'back.out(2)'
						}
					);
				break;

			case 3:
				if (h1)
					gsap.fromTo(
						h1,
						{ opacity: 0, x: -120 },
						{ opacity: 1, x: 0, duration: 1.4, ease: 'power2.out' }
					);
				if (h2)
					gsap.fromTo(
						h2,
						{ opacity: 0, x: 120 },
						{ opacity: 1, x: 0, duration: 1.4, ease: 'power2.out', delay: 0.2 }
					);
				if (p)
					gsap.fromTo(p, { opacity: 0, y: 60 }, { opacity: 1, y: 0, duration: 1.0, delay: 0.7 });
				if (buttons.length > 0)
					gsap.fromTo(
						buttons,
						{ opacity: 0, y: 60 },
						{ opacity: 1, y: 0, duration: 1.2, delay: 1.0, stagger: 0.22 }
					);
				break;
		}
	}

	/**
	 * L68+ Performance: Component lifecycle with instant initialization
	 */
	onMount(async () => {
		if (!browser) return;

		// Parallel loading for maximum speed
		const [gsapModule] = await Promise.all([
			import('gsap').catch(() => null),
			tick() // Wait for DOM bindings in parallel
		]);

		gsap = gsapModule?.gsap || gsapModule?.default || null;

		// Single fast check for DOM readiness (no excessive retries)
		if (!chartContainer || !heroSection) {
			await new Promise((r) => requestAnimationFrame(r));
		}

		if (!chartContainer) {
			console.error('❌ Chart container not ready');
			return;
		}

		// Initialize chart immediately
		initChart();

		// Start slide rotation
		await showSlide(0);
		if (slideInterval) clearInterval(slideInterval);
		slideInterval = setInterval(nextSlide, 7000);

		// Responsive handling with ResizeObserver (more efficient than window listener)
		if (typeof ResizeObserver !== 'undefined') {
			resizeObserver = new ResizeObserver(() => handleResize());
			resizeObserver.observe(heroSection);
		}
	});

	onDestroy(() => {
		if (slideInterval) {
			clearInterval(slideInterval);
			slideInterval = null;
		}

		if (replayInterval) {
			clearInterval(replayInterval);
			replayInterval = null;
		}

		if (chart) {
			chart.remove();
			chart = null;
			series = null;
		}

		if (resizeObserver) {
			resizeObserver.disconnect();
			resizeObserver = null;
		}
	});
</script>

<section
	bind:this={heroSection}
	id="hero"
	class="relative min-h-screen flex items-center overflow-hidden"
>
	<div id="chart-bg" bind:this={chartContainer} class="hero-chart absolute inset-0"></div>

	<div class="relative z-10 w-full max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-20 flex items-center">
		<div class="w-full relative min-h-96">
			{#each slides as slide, i}
				<div
					data-slide={i}
					class="slide text-center max-w-4xl mx-auto"
					class:hidden={i !== currentSlide}
					class:active={i === currentSlide}
				>
					<h1 class="text-5xl sm:text-6xl lg:text-7xl font-heading font-extrabold mb-4 text-white">
						{slide.title}
					</h1>
					<h2 class="text-3xl sm:text-4xl font-heading font-bold mb-6 text-white">
						{slide.subtitle}
					</h2>
					<p class="text-lg sm:text-xl text-white mb-8 leading-relaxed max-w-2xl mx-auto">
						{slide.description}
					</p>
					<div class="flex flex-wrap gap-4 justify-center">
						<a href={slide.primaryCTA.href} class="cta-button primary-cta">
							<span>{slide.primaryCTA.text}</span>
						</a>
						<a href={slide.secondaryCTA.href} class="cta-button secondary-cta">
							<span>{slide.secondaryCTA.text}</span>
						</a>
					</div>
				</div>
			{/each}
		</div>
	</div>
</section>

<style>
	#hero {
		padding-top: 80px; /* Space for nav bar */
		min-height: 100vh;
		background: #0a101c; /* Darker, more professional navy */
		position: relative;
		display: flex; /* Use flexbox for alignment */
		align-items: center; /* Vertically center content */
		justify-content: center; /* Horizontally center content */
	}
	#hero::before {
		content: '';
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background: radial-gradient(ellipse at center, rgba(10, 16, 28, 0) 0%, #0a101c 80%);
		pointer-events: none;
	}
	.hero-chart {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		opacity: 0.3; /* Make chart more subtle */
		transition: opacity 0.8s ease-in-out;
		will-change: opacity, mask-image;
		mask-image: linear-gradient(
			to right,
			transparent 0%,
			black 15%,
			/* Smoother fade-in edge */ black 85%,
			/* Smoother fade-out edge */ transparent 100%
		);
		-webkit-mask-image: linear-gradient(
			to right,
			transparent 0%,
			black 15%,
			black 85%,
			transparent 100%
		);
	}

	.slide {
		transition: opacity 0.5s ease-in-out;
	}
	.slide.hidden {
		display: none;
		opacity: 0;
		pointer-events: none;
	}

	.slide.active {
		display: block;
		pointer-events: auto;
		opacity: 1;
	}

	/* --- CTA Button Styles --- */
	.cta-button {
		display: inline-block;
		position: relative;
		padding: 1rem 2.5rem;
		font-family: var(--font-heading);
		font-size: 1rem;
		font-weight: 600;
		border-radius: 9999px;
		text-align: center;
		overflow: hidden;
		transition: all 0.4s cubic-bezier(0.25, 0.8, 0.25, 1);
		transform: translateZ(0); /* Promote to own layer */
	}

	.cta-button span {
		position: relative;
		z-index: 2;
	}

	.primary-cta {
		background-color: #facc15; /* yellow-400 */
		color: #fff !important;
		box-shadow: 0 4px 15px rgba(250, 204, 21, 0.2);
	}

	.primary-cta:hover {
		transform: translateY(-3px);
		box-shadow: 0 8px 25px rgba(250, 204, 21, 0.4);
	}

	.secondary-cta {
		background-color: rgba(31, 41, 55, 0.5); /* bg-gray-800 with opacity */
		color: #fff !important;
		border: 2px solid #4b5563; /* border-gray-600 */
		backdrop-filter: blur(5px);
	}

	.secondary-cta:hover {
		background-color: rgba(55, 65, 81, 0.7);
		border-color: #6b7280;
		transform: translateY(-3px);
	}

	/* --- Responsive Adjustments --- */
	@media (max-width: 768px) {
		.slide h1 {
			font-size: 3rem; /* 48px */
		}
		.slide h2 {
			font-size: 1.875rem; /* 30px */
		}
		.slide p {
			font-size: 1rem; /* 16px */
		}
		.cta-button {
			padding: 0.875rem 2rem;
			font-size: 0.9rem;
		}
	}

	@media (max-width: 640px) {
		#hero {
			padding-top: 60px;
		}
		.slide h1 {
			font-size: 2.5rem; /* 40px */
		}
		.slide h2 {
			font-size: 1.5rem; /* 24px */
		}
	}
</style>

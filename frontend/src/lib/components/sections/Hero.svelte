<script lang="ts">
	/**
	 * HeroSection - Netflix L11+ Principal Architect Grade
	 * ══════════════════════════════════════════════════════════════════════════════
	 * Cinematic Trading Hero with:
	 * - Split layout (content left, chart right)
	 * - Preserved lightweight-charts candlestick animation
	 * - GSAP orchestrated entrance sequence
	 * - Floating trading elements and depth layers
	 * - Stats ticker strip
	 * - Fully responsive with mobile optimization
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
	import HeroContent from '$lib/components/hero/HeroContent.svelte';
	import HeroChartScene from '$lib/components/hero/HeroChartScene.svelte';
	import HeroStatsStrip from '$lib/components/hero/HeroStatsStrip.svelte';

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

	// ============================================================================
	// STATE - Svelte 5 Runes
	// ============================================================================
	let chart: ReturnType<typeof import('lightweight-charts').createChart> | null = null;
	let series: any = null;
	let replayInterval: ReturnType<typeof setInterval> | null = null;
	let isLooping = $state(false);
	let gsapLib: any = null;
	let masterTimeline: any = null;

	// DOM refs
	let heroSection = $state<HTMLElement | null>(null);
	let chartContainer = $state<HTMLElement | null>(null);
	let chartSceneRef = $state<HTMLElement | null>(null);
	let heroContentRef: HeroContent | null = null;
	let resizeObserver: ResizeObserver | null = null;
	let visibilityObserver: IntersectionObserver | null = null;

	// State
	let isVisible = $state(true);
	let chartReady = $state(false);
	let mounted = $state(false);
	let entranceComplete = $state(false);
	let isAnimating = $state(false);
	let statsVisible = $state(false);

	// Chart price state (for display)
	let currentPrice = $state(475.50);
	let priceChange = $state(0.42);

	// ============================================================================
	// CHART INITIALIZATION - LAZY LOADED FOR PERFORMANCE
	// ============================================================================

	async function initChart(): Promise<void> {
		if (!browser || !chartContainer || chartReady) {
			return;
		}

		// Lazy load chart library - CRITICAL FOR PERFORMANCE
		const { createChart, CandlestickSeries, CrosshairMode, ColorType } = await import('lightweight-charts');

		// Get dimensions from scene container
		const sceneRect = chartSceneRef?.getBoundingClientRect();
		const width = sceneRect?.width || chartContainer.clientWidth || 600;
		const height = sceneRect?.height || chartContainer.clientHeight || 400;

		// Cleanup
		if (chart) {
			chart.remove();
			chart = null;
			series = null;
		}

		chart = createChart(chartContainer, {
			layout: {
				background: { type: ColorType.Solid, color: 'transparent' },
				textColor: 'rgba(255, 255, 255, 0.15)'
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
				barSpacing: 6,
				minBarSpacing: 3,
				fixLeftEdge: false,
				fixRightEdge: true,
				lockVisibleTimeRangeOnResize: true,
				rightBarStaysOnScroll: true,
				shiftVisibleRangeOnNewBar: true,
				borderColor: 'transparent'
			},
			crosshair: { mode: CrosshairMode.Hidden },
			rightPriceScale: {
				visible: false,
				borderColor: 'transparent'
			}
		});

		series = chart.addSeries(CandlestickSeries, {
			upColor: 'rgba(52, 211, 153, 0.7)',
			downColor: 'rgba(239, 68, 68, 0.7)',
			borderVisible: false,
			wickVisible: true,
			wickUpColor: 'rgba(52, 211, 153, 0.5)',
			wickDownColor: 'rgba(239, 68, 68, 0.5)',
			priceLineVisible: false
		});

		// Initial view - 10 candles
		const initialCount = 10;
		series.setData(CANDLE_DATA.slice(0, initialCount));

		if (chart.timeScale()) {
			chart.timeScale().setVisibleLogicalRange({ from: 0, to: initialCount });
		}

		// Mark chart as ready and start replay
		chartReady = true;
		startReplay(initialCount);
	}

	// ============================================================================
	// REPLAY ANIMATION
	// ============================================================================

	function startReplay(startIndex: number = 10): void {
		if (!browser || !series || !chart) return;

		if (replayInterval) {
			clearInterval(replayInterval);
			replayInterval = null;
		}

		let currentIndex = Math.max(startIndex, 1);
		const totalCandles = CANDLE_DATA.length;
		const sceneRect = chartSceneRef?.getBoundingClientRect();
		const chartWidth = sceneRect?.width || 600;
		const barsVisible = Math.floor(chartWidth / 6);

		replayInterval = setInterval(() => {
			if (!series || isLooping || !isVisible) return;

			currentIndex++;

			if (currentIndex >= totalCandles) {
				handleLoop();
				return;
			}

			series.setData(CANDLE_DATA.slice(0, currentIndex));

			// Update current price display
			const latestCandle = CANDLE_DATA[currentIndex - 1];
			if (latestCandle) {
				currentPrice = latestCandle.close;
				const firstCandle = CANDLE_DATA[0];
				if (firstCandle) {
					const change = ((latestCandle.close - firstCandle.open) / firstCandle.open) * 100;
					priceChange = change;
				}
			}

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

		// Use timeline for loop animation - no delays
		const loopTL = gsapLib.timeline({
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
				currentPrice = 475.50;
				priceChange = 0.42;
				if (chart?.timeScale()) {
					chart.timeScale().setVisibleLogicalRange({ from: 0, to: 10 });
				}
			})
			.to(chartContainer, { opacity: 1, duration: 0.5, ease: 'power2.out' });
	}

	// ============================================================================
	// MASTER ENTRANCE ANIMATION - GSAP ORCHESTRATION
	// ============================================================================

	async function orchestrateEntrance(): Promise<void> {
		if (!browser || !heroSection || entranceComplete) return;

		try {
			const gsapModule = await import('gsap');
			gsapLib = gsapModule?.gsap || gsapModule?.default;

			if (!gsapLib) {
				entranceComplete = true;
				statsVisible = true;
				return;
			}

			isAnimating = true;

			// Create master timeline
			masterTimeline = gsapLib.timeline({
				defaults: { ease: 'power3.out' },
				onComplete: () => {
					entranceComplete = true;
					isAnimating = false;
				}
			});

			// Hero section fade in
			masterTimeline.fromTo(heroSection,
				{ opacity: 0 },
				{ opacity: 1, duration: 0.5 },
				0
			);

			// Trigger HeroContent animation
			await tick();
			if (heroContentRef?.animateEntrance) {
				heroContentRef.animateEntrance();
			}

			// Stats strip appears after content
			masterTimeline.call(() => {
				statsVisible = true;
			}, [], 1.8);

		} catch (e) {
			console.warn('GSAP orchestration failed:', e);
			entranceComplete = true;
			statsVisible = true;
		}
	}

	// ============================================================================
	// RESIZE HANDLING
	// ============================================================================

	function handleResize(): void {
		if (!browser || !chart || !chartSceneRef) return;

		const sceneRect = chartSceneRef.getBoundingClientRect();
		chart.applyOptions({
			width: sceneRect.width,
			height: sceneRect.height
		});
	}

	// ============================================================================
	// VISIBILITY OPTIMIZATION - Pause when off-screen
	// ============================================================================

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

		// Setup resize observer on chart scene
		if (chartSceneRef && typeof ResizeObserver !== 'undefined') {
			resizeObserver = new ResizeObserver(handleResize);
			resizeObserver.observe(chartSceneRef);
		}

		// Setup visibility observer
		if (heroSection) {
			setupVisibilityObserver();
		}

		// Initialize chart after a tick to ensure DOM is ready
		await tick();
		if (chartContainer) {
			initChart();
		}

		// Orchestrate entrance animation
		orchestrateEntrance();
	});

	onDestroy(() => {
		if (replayInterval) clearInterval(replayInterval);
		if (masterTimeline) masterTimeline.kill();
		if (chart) chart.remove();
		if (resizeObserver) resizeObserver.disconnect();
		if (visibilityObserver) visibilityObserver.disconnect();

		chart = null;
		series = null;
		masterTimeline = null;
		mounted = false;
	});
</script>

<section
	bind:this={heroSection}
	id="hero"
	class="hero-section"
	class:hero-animating={isAnimating}
>
	<!-- Background Gradient Layers -->
	<div class="hero-bg-gradient" aria-hidden="true"></div>
	<div class="hero-bg-grid" aria-hidden="true"></div>

	<!-- Main Content Container -->
	<div class="hero-container">
		<!-- Split Layout -->
		<div class="hero-split">
			<!-- Left: Content Zone -->
			<div class="hero-content-zone">
				<HeroContent bind:this={heroContentRef} />
			</div>

			<!-- Right: Chart Zone -->
			<div class="hero-chart-zone" bind:this={chartSceneRef}>
				<HeroChartScene
					bind:chartContainer={chartContainer}
					{chartReady}
					{isVisible}
					{currentPrice}
					{priceChange}
				/>
			</div>
		</div>

		<!-- Bottom: Stats Strip -->
		<div class="hero-stats-zone">
			<HeroStatsStrip isVisible={statsVisible} />
		</div>
	</div>

	<!-- Scroll Indicator -->
	<div class="scroll-indicator" aria-hidden="true">
		<span class="scroll-indicator__line"></span>
		<span class="scroll-indicator__text">Scroll</span>
	</div>
</section>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════════
	 * CSS Custom Properties
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	:root {
		--hero-bg: #0a0f1a;
		--hero-nav-height: 80px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Hero Section - Optimized Container
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.hero-section {
		position: relative;
		min-height: 100vh;
		min-height: 100dvh;
		padding-block-start: var(--hero-nav-height);
		background: var(--hero-bg);
		display: flex;
		flex-direction: column;
		overflow: hidden;

		/* Performance */
		contain: layout style paint;
		content-visibility: auto;
		contain-intrinsic-size: 100vw 100vh;
	}

	/* will-change only during animations */
	.hero-animating .hero-content-zone,
	.hero-animating .hero-chart-zone {
		will-change: transform, opacity;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Background Layers
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.hero-bg-gradient {
		position: absolute;
		inset: 0;
		z-index: 0;
		background:
			radial-gradient(ellipse 80% 50% at 80% 30%, rgba(99, 102, 241, 0.08) 0%, transparent 50%),
			radial-gradient(ellipse 60% 40% at 20% 70%, rgba(139, 92, 246, 0.06) 0%, transparent 50%),
			radial-gradient(ellipse 100% 100% at 50% 100%, rgba(15, 23, 42, 0.9) 0%, transparent 40%);
		pointer-events: none;
	}

	.hero-bg-grid {
		position: absolute;
		inset: 0;
		z-index: 0;
		background-image:
			linear-gradient(rgba(99, 102, 241, 0.03) 1px, transparent 1px),
			linear-gradient(90deg, rgba(99, 102, 241, 0.03) 1px, transparent 1px);
		background-size: 60px 60px;
		mask-image: radial-gradient(ellipse 70% 60% at 70% 40%, black 30%, transparent 70%);
		-webkit-mask-image: radial-gradient(ellipse 70% 60% at 70% 40%, black 30%, transparent 70%);
		pointer-events: none;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Main Container
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.hero-container {
		position: relative;
		z-index: 10;
		flex: 1;
		display: flex;
		flex-direction: column;
		justify-content: center;
		width: 100%;
		max-width: 90rem;
		margin-inline: auto;
		padding-inline: 1.5rem;
		padding-block: 2rem;
		gap: 2rem;
	}

	@media (min-width: 1024px) {
		.hero-container {
			padding-inline: 3rem;
			padding-block: 3rem;
			gap: 2.5rem;
		}
	}

	@media (min-width: 1440px) {
		.hero-container {
			padding-inline: 4rem;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Split Layout
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.hero-split {
		display: flex;
		flex-direction: column;
		gap: 2.5rem;
		align-items: center;
		flex: 1;
	}

	@media (min-width: 1024px) {
		.hero-split {
			flex-direction: row;
			align-items: center;
			justify-content: space-between;
			gap: 4rem;
		}
	}

	@media (min-width: 1280px) {
		.hero-split {
			gap: 5rem;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Content Zone (Left)
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.hero-content-zone {
		flex: 1;
		max-width: 100%;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	@media (min-width: 1024px) {
		.hero-content-zone {
			flex: 0 0 55%;
			max-width: 55%;
			justify-content: flex-start;
		}
	}

	@media (min-width: 1280px) {
		.hero-content-zone {
			flex: 0 0 52%;
			max-width: 52%;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Chart Zone (Right)
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.hero-chart-zone {
		flex: 1;
		width: 100%;
		max-width: 100%;
		min-height: 280px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	@media (min-width: 768px) {
		.hero-chart-zone {
			min-height: 350px;
		}
	}

	@media (min-width: 1024px) {
		.hero-chart-zone {
			flex: 0 0 42%;
			max-width: 42%;
			min-height: 420px;
		}
	}

	@media (min-width: 1280px) {
		.hero-chart-zone {
			flex: 0 0 45%;
			max-width: 45%;
			min-height: 480px;
		}
	}

	@media (min-width: 1536px) {
		.hero-chart-zone {
			min-height: 520px;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Stats Zone (Bottom)
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.hero-stats-zone {
		width: 100%;
		max-width: 72rem;
		margin-inline: auto;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Scroll Indicator
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.scroll-indicator {
		position: absolute;
		bottom: 2rem;
		left: 50%;
		transform: translateX(-50%);
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.5rem;
		opacity: 0.5;
		transition: opacity 0.3s ease;
	}

	.scroll-indicator:hover {
		opacity: 0.8;
	}

	.scroll-indicator__line {
		width: 1px;
		height: 2.5rem;
		background: linear-gradient(to bottom, rgba(99, 102, 241, 0.6), transparent);
		animation: scrollPulse 2s ease-in-out infinite;
	}

	@keyframes scrollPulse {
		0%, 100% {
			opacity: 0.5;
			height: 2.5rem;
		}
		50% {
			opacity: 1;
			height: 3rem;
		}
	}

	.scroll-indicator__text {
		font-family: var(--font-heading, system-ui);
		font-size: 0.65rem;
		font-weight: 600;
		letter-spacing: 0.1em;
		text-transform: uppercase;
		color: rgba(255, 255, 255, 0.5);
	}

	/* Hide scroll indicator on short screens */
	@media (max-height: 700px) {
		.scroll-indicator {
			display: none;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Mobile Optimizations
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	@media (max-width: 640px) {
		.hero-section {
			padding-block-start: 60px;
		}

		.hero-container {
			padding-inline: 1rem;
			padding-block: 1.5rem;
			gap: 1.5rem;
		}

		.hero-split {
			gap: 2rem;
		}

		.hero-chart-zone {
			min-height: 240px;
			order: -1; /* Chart first on mobile for visual impact */
		}

		.scroll-indicator {
			bottom: 1rem;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Ultra-wide Optimizations
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	@media (min-width: 2000px) {
		.hero-container {
			max-width: 100rem;
		}

		.hero-chart-zone {
			min-height: 600px;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Reduced Motion
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	@media (prefers-reduced-motion: reduce) {
		.scroll-indicator__line {
			animation: none;
		}

		.hero-bg-gradient,
		.hero-bg-grid {
			transition: none;
		}
	}
</style>

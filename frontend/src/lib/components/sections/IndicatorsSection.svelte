<script lang="ts">
import { logger } from '$lib/utils/logger';
	/**
	 * IndicatorsSection - Apple/Netflix Cinematic Design
	 * Mobile-First + GSAP Animations Fixed
	 * ══════════════════════════════════════════════════════════════════════════════
	 * Features:
	 * ✓ Mobile-first responsive design
	 * ✓ GSAP ScrollTrigger with proper initialization
	 * ✓ Animated trading chart visualization
	 * ✓ Touch-friendly interactions
	 * ✓ Reduced motion support
	 * ══════════════════════════════════════════════════════════════════════════════
	 */
	import { onMount, onDestroy, tick } from 'svelte';
	import { cubicOut, backOut } from 'svelte/easing';
	import { Icon, IconActivity, IconArrowRight, IconBolt, IconChartLine, IconSparkles, IconWaveSine } from '$lib/icons';
	// ============================================================================
	// INDICATOR DATA
	// ============================================================================
	const indicators = [
		{
			id: 'rsi',
			name: 'RSI',
			fullName: 'Relative Strength Index',
			category: 'Momentum',
			description: 'Identify overbought & oversold zones with precision timing.',
			icon: IconActivity,
			color: '#3b82f6',
			gradient: 'from-blue-500 to-cyan-400',
			href: '/indicators/rsi'
		},
		{
			id: 'macd',
			name: 'MACD',
			fullName: 'Moving Average Convergence',
			category: 'Trend',
			description: 'Capture momentum shifts before the crowd sees them.',
			icon: IconWaveSine,
			color: '#10b981',
			gradient: 'from-emerald-500 to-teal-400',
			href: '/indicators/macd'
		},
		{
			id: 'bollinger',
			name: 'Bollinger',
			fullName: 'Bollinger Bands',
			category: 'Volatility',
			description: 'Master volatility squeezes and explosive breakouts.',
			icon: IconChartLine,
			color: '#8b5cf6',
			gradient: 'from-violet-500 to-purple-400',
			href: '/indicators/bollinger-bands'
		},
		{
			id: 'vwap',
			name: 'VWAP',
			fullName: 'Volume Weighted Price',
			category: 'Institutional',
			description: 'Trade with the smart money using institutional levels.',
			icon: IconBolt,
			color: '#f59e0b',
			gradient: 'from-amber-500 to-orange-400',
			href: '/indicators/vwap'
		}
	];

	// ============================================================================
	// STATE & REFS
	// ============================================================================
	let sectionRef = $state<HTMLElement | null>(null);
	let chartRef = $state<HTMLElement | null>(null);
	// ICT11+ Fix: Start false, set true in onMount to trigger in: transitions
	let isVisible = $state(false);
	let activeIndicator = $state(0);
	let scrollTriggerInstance: any = null;
	let animationFrame: number;
	let chartCtx: CanvasRenderingContext2D | null = null;
	let canvasRef = $state<HTMLCanvasElement | null>(null);
	let prefersReducedMotion = $state(false);

	// Chart animation state
	let chartProgress = $state(0);
	const candleData = generateCandleData(60);
	const rsiData = generateRSIData(candleData);

	// ============================================================================
	// DATA GENERATORS
	// ============================================================================
	function generateCandleData(count: number) {
		const candles = [];
		let price = 150;
		for (let i = 0; i < count; i++) {
			const change = (Math.random() - 0.48) * 3;
			const open = price;
			const close = price + change;
			const high = Math.max(open, close) + Math.random() * 1.5;
			const low = Math.min(open, close) - Math.random() * 1.5;
			candles.push({ open, high, low, close, bullish: close > open });
			price = close;
		}
		return candles;
	}

	function generateRSIData(candles: any[]) {
		return candles.map((_, i) => 30 + Math.sin(i * 0.15) * 25 + Math.random() * 15);
	}

	// ============================================================================
	// CHART RENDERING
	// ============================================================================
	function drawChart() {
		if (!chartCtx || !canvasRef || !chartRef) return;

		// Use CSS dimensions, not canvas dimensions (which are scaled by DPR)
		const rect = chartRef.getBoundingClientRect();
		const width = rect.width;
		const height = rect.height;

		// Skip if dimensions are invalid
		if (width === 0 || height === 0) return;

		const visibleCandles = Math.floor(candleData.length * chartProgress);

		// Clear canvas using CSS dimensions (context is already scaled)
		chartCtx.clearRect(0, 0, width, height);

		// Draw grid
		chartCtx.strokeStyle = 'rgba(255, 255, 255, 0.03)';
		chartCtx.lineWidth = 1;
		for (let i = 0; i < 10; i++) {
			const y = (height / 10) * i;
			chartCtx.beginPath();
			chartCtx.moveTo(0, y);
			chartCtx.lineTo(width, y);
			chartCtx.stroke();
		}

		if (visibleCandles === 0) return;

		// Calculate price range
		const visibleData = candleData.slice(0, visibleCandles);
		const minPrice = Math.min(...visibleData.map((c) => c.low)) - 2;
		const maxPrice = Math.max(...visibleData.map((c) => c.high)) + 2;
		const priceRange = maxPrice - minPrice;

		const candleWidth = (width / 60) * 0.7;
		const gap = (width / 60) * 0.3;

		// Draw candles
		visibleData.forEach((candle, i) => {
			const x = i * (candleWidth + gap) + gap / 2;
			const openY = height - ((candle.open - minPrice) / priceRange) * height * 0.7 - height * 0.15;
			const closeY =
				height - ((candle.close - minPrice) / priceRange) * height * 0.7 - height * 0.15;
			const highY = height - ((candle.high - minPrice) / priceRange) * height * 0.7 - height * 0.15;
			const lowY = height - ((candle.low - minPrice) / priceRange) * height * 0.7 - height * 0.15;

			// Wick
			chartCtx!.strokeStyle = candle.bullish ? '#10b981' : '#ef4444';
			chartCtx!.lineWidth = 1;
			chartCtx!.beginPath();
			chartCtx!.moveTo(x + candleWidth / 2, highY);
			chartCtx!.lineTo(x + candleWidth / 2, lowY);
			chartCtx!.stroke();

			// Body
			chartCtx!.fillStyle = candle.bullish ? '#10b981' : '#ef4444';
			const bodyTop = Math.min(openY, closeY);
			const bodyHeight = Math.abs(closeY - openY) || 1;
			chartCtx!.fillRect(x, bodyTop, candleWidth, bodyHeight);
		});

		// Draw RSI line (bottom section)
		if (activeIndicator === 0) {
			const rsiVisible = rsiData.slice(0, visibleCandles);
			chartCtx.strokeStyle = '#3b82f6';
			chartCtx.lineWidth = 2;
			chartCtx.beginPath();
			rsiVisible.forEach((rsi, i) => {
				const x = i * (candleWidth + gap) + candleWidth / 2;
				const y = height - (rsi / 100) * height * 0.2 - 5;
				if (i === 0) chartCtx!.moveTo(x, y);
				else chartCtx!.lineTo(x, y);
			});
			chartCtx.stroke();

			// RSI zones
			chartCtx.fillStyle = 'rgba(59, 130, 246, 0.1)';
			chartCtx.fillRect(0, height - height * 0.2 - 5, width, height * 0.06);
			chartCtx.fillRect(0, height - height * 0.08, width, height * 0.06);
		}
	}

	// ============================================================================
	// ANIMATION LOOP
	// ============================================================================
	function animate() {
		drawChart();
		animationFrame = requestAnimationFrame(animate);
	}

	// ============================================================================
	// LIFECYCLE
	// ============================================================================
	let rotationInterval: ReturnType<typeof setInterval> | null = null;
	let resizeObserver: ResizeObserver | null = null;

	onMount(() => {
		// Check for reduced motion preference
		prefersReducedMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches;

		// Trigger entrance animations when section scrolls into viewport
		queueMicrotask(() => {
			if (sectionRef) {
				const visibilityObserver = new IntersectionObserver(
					(entries) => {
						if (entries[0]?.isIntersecting) {
							isVisible = true;
							visibilityObserver.disconnect();
						}
					},
					{ threshold: 0.1, rootMargin: '50px' }
				);
				visibilityObserver.observe(sectionRef);
			} else {
				isVisible = true;
			}
		});

		// Load GSAP asynchronously
		if (!prefersReducedMotion) {
			loadGSAP();
		}

		// Auto-rotate indicators (slower on mobile for better UX)
		const isMobile = window.innerWidth < 768;
		rotationInterval = setInterval(
			() => {
				activeIndicator = (activeIndicator + 1) % indicators.length;
			},
			isMobile ? 5000 : 4000
		);

		return () => {
			if (rotationInterval) clearInterval(rotationInterval);
			resizeObserver?.disconnect();
		};
	});

	// ============================================================================
	// CANVAS INIT — waits for isVisible + DOM refs before starting
	// ============================================================================
	let canvasInitialized = false;

	$effect(() => {
		if (!isVisible || !canvasRef || !chartRef || canvasInitialized) return;
		canvasInitialized = true;

		// Double rAF ensures layout is fully computed after the {#if} block mounts
		requestAnimationFrame(() => {
			requestAnimationFrame(() => {
				setupCanvas();

				resizeObserver = new ResizeObserver(() => {
					setupCanvas();
					drawChart();
				});
				if (chartRef) resizeObserver.observe(chartRef);

				// Start chart progress animation now that canvas is ready
				if (!prefersReducedMotion) {
					animatechartProgress();
				} else {
					chartProgress = 1;
				}

				// Start draw loop
				animate();
			});
		});
	});

	function setupCanvas() {
		if (!canvasRef || !chartRef) return;

		const dpr = window.devicePixelRatio || 1;
		const rect = chartRef.getBoundingClientRect();

		// Ensure we have valid dimensions
		if (rect.width === 0 || rect.height === 0) {
			logger.debug('[IndicatorsSection] Canvas has zero dimensions, retrying...');
			return;
		}

		// Reset canvas dimensions
		canvasRef.width = rect.width * dpr;
		canvasRef.height = rect.height * dpr;

		// Get fresh context after resize
		chartCtx = canvasRef.getContext('2d');
		if (chartCtx) {
			// Reset transform before scaling
			chartCtx.setTransform(1, 0, 0, 1, 0, 0);
			chartCtx.scale(dpr, dpr);
			logger.debug('[IndicatorsSection] Canvas setup complete:', rect.width, 'x', rect.height);
		}
	}

	function animatechartProgress() {
		const duration = 2000;
		const start = performance.now();
		function step(now: number) {
			const elapsed = now - start;
			const progress = Math.min(elapsed / duration, 1);
			chartProgress = cubicOut(progress);
			if (progress < 1) requestAnimationFrame(step);
		}
		requestAnimationFrame(step);
	}

	async function loadGSAP() {
		try {
			const { gsap } = await import('gsap');
			const { ScrollTrigger } = await import('gsap/ScrollTrigger');
			gsap.registerPlugin(ScrollTrigger);
			scrollTriggerInstance = ScrollTrigger;

			// Animate indicator cards on scroll
			await tick();
			const cards = sectionRef?.querySelectorAll('.indicator-card');
			if (cards && cards.length > 0) {
				gsap.fromTo(
					cards,
					{ y: 40, opacity: 0 },
					{
						y: 0,
						opacity: 1,
						duration: 0.6,
						stagger: 0.1,
						ease: 'power2.out',
						scrollTrigger: {
							trigger: sectionRef,
							start: 'top 70%',
							toggleActions: 'play none none reverse'
						}
					}
				);
			}
		} catch (e) {
			logger.debug('[IndicatorsSection] GSAP not available:', e);
		}
	}

	onDestroy(() => {
		if (animationFrame) cancelAnimationFrame(animationFrame);
		// Kill only ScrollTriggers associated with this section
		if (scrollTriggerInstance && sectionRef) {
			scrollTriggerInstance.getAll().forEach((st: any) => {
				if (st.trigger === sectionRef || sectionRef?.contains(st.trigger)) {
					st.kill();
				}
			});
		}
	});

	// ============================================================================
	// TRANSITIONS
	// ============================================================================
	function slideUp(_node: Element, { delay = 0, duration = 800 }) {
		return {
			delay,
			duration,
			css: (t: number) => {
				const eased = cubicOut(t);
				return `opacity: ${eased}; transform: translateY(${(1 - eased) * 60}px);`;
			}
		};
	}

	function scaleIn(_node: Element, { delay = 0, duration = 600 }) {
		return {
			delay,
			duration,
			css: (t: number) => {
				const eased = backOut(t);
				return `opacity: ${t}; transform: scale(${0.8 + eased * 0.2});`;
			}
		};
	}
</script>

<section
	bind:this={sectionRef}
	class="is-section"
>
	<div class="is-bg">
		<div class="is-orb is-orb-blue"></div>
		<div class="is-orb is-orb-violet" style="animation-delay: 1s;"></div>
		<div class="is-grid-lines"></div>
		<div class="is-radial-fade"></div>
	</div>

	<div class="is-container">
		{#if isVisible}
			<div
				class="is-header"
				in:slideUp={{ delay: 0, duration: prefersReducedMotion ? 0 : 800 }}
			>
				<div class="is-badge">
					<Icon icon={IconSparkles} size={14} />
					Technical Edge
				</div>

				<h2 class="is-title">
					Indicator <span class="is-title-muted">Suite.</span>
				</h2>

				<p class="is-subtitle">
					We don't offer retail indicators. We provide institutional-grade analysis tools. Verified
					by quantitative funds and professional trading desks.
				</p>
			</div>
		{/if}

		<div class="is-grid">
			{#if isVisible}
				<div
					in:scaleIn={{
						delay: prefersReducedMotion ? 0 : 200,
						duration: prefersReducedMotion ? 0 : 600
					}}
					class="is-chart-col"
				>
					<div class="is-chart-frame">
						<div class="is-chart-header">
							<div class="is-chart-header-left">
								<div class="is-dots">
									<div class="is-dot is-dot-red"></div>
									<div class="is-dot is-dot-yellow"></div>
									<div class="is-dot is-dot-green"></div>
								</div>
								<span class="is-chart-label">SPY • 1D</span>
							</div>
							<div class="is-chart-header-right">
								<span class="is-chart-pct">+2.34%</span>
								<div class="is-live-dot"></div>
							</div>
						</div>

						<div bind:this={chartRef} class="is-chart-canvas-wrap">
							<canvas
								bind:this={canvasRef}
								class="is-canvas"
								style="width: 100%; height: 100%;"
							></canvas>

							<div class="is-indicator-label">
								<span class="is-indicator-text">
									{indicators[activeIndicator]!.name} Active
								</span>
							</div>
						</div>
					</div>

					<div class="is-float-stat">
						<div class="is-float-label">Win Rate</div>
						<div class="is-float-value">73.2%</div>
					</div>
				</div>
			{/if}

			{#if isVisible}
				<div
					class="is-cards-col"
					in:slideUp={{
						delay: prefersReducedMotion ? 0 : 300,
						duration: prefersReducedMotion ? 0 : 600
					}}
				>
					{#each indicators as indicator, i}
						{@const iconStr = indicator.icon}
						<a
							href={indicator.href}
							class="indicator-card is-card"
							data-active={activeIndicator === i ? '' : undefined}
							data-id={indicator.id}
							onmouseenter={() => (activeIndicator = i)}
							ontouchstart={() => (activeIndicator = i)}
						>
							<div class="is-card-row">
								<div class="is-card-icon">
									<Icon icon={iconStr} size={24} />
								</div>

								<div class="is-card-content">
									<div class="is-card-name-row">
										<h3 class="is-card-name">{indicator.name}</h3>
										<span class="is-card-cat">{indicator.category}</span>
									</div>
									<p class="is-card-fullname">{indicator.fullName}</p>
									<p class="is-card-desc">{indicator.description}</p>
								</div>

								<div class="is-card-arrow">
									<Icon icon={IconArrowRight} size={20} class="is-arrow-icon" />
								</div>
							</div>
						</a>
					{/each}

					<div class="is-cta-wrap">
						<a href="/indicators" class="is-cta-btn">
							<span>Explore All Indicators</span>
							<Icon icon={IconArrowRight} size={20} class="is-cta-arrow" />
						</a>
					</div>
				</div>
			{/if}
		</div>
	</div>
</section>

<style>
	/* ─── Section ─── */
	.is-section {
		position: relative;
		padding-block: 4rem;
		overflow: hidden;
		background: linear-gradient(to bottom, oklch(0.12 0.005 285), oklch(0.15 0.005 285), oklch(0.12 0.005 285));

		@media (min-width: 640px) { padding-block: 6rem; }
		@media (min-width: 1024px) { padding-block: 8rem; }
	}

	/* ─── Background ─── */
	.is-bg { position: absolute; inset: 0; pointer-events: none; }

	.is-orb {
		position: absolute;
		border-radius: 50%;
		animation: orb-pulse 2s ease-in-out infinite;
	}

	.is-orb-blue {
		inset-block-start: 25%;
		inset-inline-start: -4rem;
		inline-size: 12rem;
		block-size: 12rem;
		background-color: oklch(0.6 0.18 260 / 0.1);
		filter: blur(80px);

		@media (min-width: 640px) { inset-inline-start: -8rem; inline-size: 24rem; block-size: 24rem; filter: blur(128px); }
	}

	.is-orb-violet {
		inset-block-end: 25%;
		inset-inline-end: -4rem;
		inline-size: 12rem;
		block-size: 12rem;
		background-color: oklch(0.55 0.2 300 / 0.1);
		filter: blur(80px);

		@media (min-width: 640px) { inset-inline-end: -8rem; inline-size: 24rem; block-size: 24rem; filter: blur(128px); }
	}

	.is-grid-lines {
		position: absolute;
		inset: 0;
		background-image:
			linear-gradient(oklch(1 0 0 / 0.02) 1px, transparent 1px),
			linear-gradient(90deg, oklch(1 0 0 / 0.02) 1px, transparent 1px);
		background-size: 2rem 2rem;

		@media (min-width: 640px) { background-size: 4rem 4rem; }
	}

	.is-radial-fade {
		position: absolute;
		inset: 0;
		background: radial-gradient(ellipse at center, transparent 0%, oklch(0.12 0.005 285 / 0.8) 70%);
	}

	/* ─── Container ─── */
	.is-container {
		position: relative;
		max-inline-size: 80rem;
		margin-inline: auto;
		padding-inline: 1rem;

		@media (min-width: 640px) { padding-inline: 1.5rem; }
		@media (min-width: 1024px) { padding-inline: 2rem; }
	}

	.is-header {
		max-inline-size: 56rem;
		margin-inline: auto;
		text-align: center;
		margin-block-end: 6rem;
	}

	.is-badge {
		display: inline-flex;
		align-items: center;
		gap: 0.75rem;
		padding-inline: 1rem;
		padding-block: 0.375rem;
		border: 1px solid oklch(0.3 0.15 260 / 0.3);
		background-color: oklch(0.15 0.08 260 / 0.1);
		color: oklch(0.6 0.18 260);
		font-size: 0.625rem;
		font-weight: var(--weight-bold);
		letter-spacing: 0.3em;
		text-transform: uppercase;
		margin-block-end: 2rem;
		border-radius: 2px;
	}

	.is-title {
		font-size: clamp(3rem, 6vw, 4.5rem);
		font-family: var(--font-serif, serif);
		color: oklch(1 0 0);
		margin-block-end: 2rem;
		letter-spacing: -0.02em;
	}

	.is-title-muted { color: oklch(0.35 0.01 265); }

	.is-subtitle {
		font-size: var(--text-lg);
		color: oklch(0.55 0.01 265);
		font-weight: 300;
		line-height: 1.7;
		max-inline-size: 32rem;
		margin-inline: auto;
	}

	/* ─── Grid ─── */
	.is-grid {
		display: grid;
		grid-template-columns: 1fr;
		gap: 2rem;
		align-items: center;

		@media (min-width: 1024px) { grid-template-columns: repeat(2, 1fr); gap: 4rem; }
	}

	/* ─── Chart Column ─── */
	.is-chart-col {
		position: relative;
		order: 2;

		@media (min-width: 1024px) { order: 1; }
	}

	.is-chart-frame {
		position: relative;
		border-radius: var(--radius-xl);
		overflow: hidden;
		background-color: oklch(0.15 0.005 285 / 0.5);
		backdrop-filter: blur(16px);
		border: 1px solid oklch(0.3 0.005 285 / 0.5);
		box-shadow: 0 25px 50px oklch(0 0 0 / 0.25);

		@media (min-width: 640px) { border-radius: 1rem; }
	}

	.is-chart-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding-inline: 1rem;
		padding-block: 0.75rem;
		border-block-end: 1px solid oklch(0.3 0.005 285 / 0.5);

		@media (min-width: 640px) { padding-inline: 1.5rem; padding-block: 1rem; }
	}

	.is-chart-header-left { display: flex; align-items: center; gap: 0.5rem; @media (min-width: 640px) { gap: 0.75rem; } }
	.is-chart-header-right { display: flex; align-items: center; gap: 0.375rem; @media (min-width: 640px) { gap: 0.5rem; } }

	.is-dots { display: flex; gap: 0.25rem; @media (min-width: 640px) { gap: 0.375rem; } }

	.is-dot {
		inline-size: 0.625rem;
		block-size: 0.625rem;
		border-radius: 50%;

		@media (min-width: 640px) { inline-size: 0.75rem; block-size: 0.75rem; }
	}

	.is-dot-red { background-color: oklch(0.6 0.2 25 / 0.8); }
	.is-dot-yellow { background-color: oklch(0.8 0.15 85 / 0.8); }
	.is-dot-green { background-color: oklch(0.7 0.17 160 / 0.8); }

	.is-chart-label {
		font-size: var(--text-xs);
		font-family: var(--font-mono, monospace);
		color: oklch(0.55 0.01 265);

		@media (min-width: 640px) { font-size: var(--text-sm); }
	}

	.is-chart-pct {
		font-size: 0.625rem;
		font-family: var(--font-mono, monospace);
		color: oklch(0.7 0.17 160);

		@media (min-width: 640px) { font-size: var(--text-xs); }
	}

	.is-live-dot {
		inline-size: 0.375rem;
		block-size: 0.375rem;
		border-radius: 50%;
		background-color: oklch(0.7 0.17 160);
		animation: orb-pulse 2s ease-in-out infinite;

		@media (min-width: 640px) { inline-size: 0.5rem; block-size: 0.5rem; }
	}

	/* ─── Chart Canvas ─── */
	.is-chart-canvas-wrap {
		position: relative;
		block-size: 14rem;

		@media (min-width: 640px) { block-size: 18rem; }
		@media (min-width: 1024px) { block-size: 20rem; }
	}

	.is-canvas { position: absolute; inset: 0; inline-size: 100%; block-size: 100%; }

	.is-indicator-label {
		position: absolute;
		inset-block-end: 0.75rem;
		inset-inline-start: 0.75rem;
		padding-inline: 0.5rem;
		padding-block: 0.25rem;
		border-radius: var(--radius-md);
		background-color: oklch(0.25 0.005 285 / 0.8);
		backdrop-filter: blur(4px);
		border: 1px solid oklch(0.35 0.005 285 / 0.5);

		@media (min-width: 640px) { inset-block-end: 1rem; inset-inline-start: 1rem; padding-inline: 0.75rem; padding-block: 0.375rem; border-radius: var(--radius-lg); }
	}

	.is-indicator-text {
		font-size: 0.625rem;
		font-family: var(--font-mono, monospace);
		color: oklch(0.8 0.01 265);

		@media (min-width: 640px) { font-size: var(--text-xs); }
	}

	/* ─── Floating Stat ─── */
	.is-float-stat {
		display: none;
		position: absolute;
		inset-block-end: -1rem;
		inset-inline-end: -0.5rem;
		padding-inline: 0.75rem;
		padding-block: 0.5rem;
		border-radius: var(--radius-lg);
		background-color: oklch(0.25 0.005 285 / 0.9);
		backdrop-filter: blur(16px);
		border: 1px solid oklch(0.35 0.005 285 / 0.5);
		box-shadow: 0 20px 40px oklch(0 0 0 / 0.2);

		@media (min-width: 640px) {
			display: block;
			inset-block-end: -1.5rem;
			inset-inline-end: -1.5rem;
			padding-inline: 1rem;
			padding-block: 0.75rem;
			border-radius: var(--radius-xl);
		}
	}

	.is-float-label {
		font-size: 0.625rem;
		color: oklch(0.45 0.005 285);
		margin-block-end: 0.125rem;

		@media (min-width: 640px) { font-size: var(--text-xs); margin-block-end: 0.25rem; }
	}

	.is-float-value {
		font-size: var(--text-xl);
		font-weight: var(--weight-bold);
		color: oklch(0.7 0.17 160);

		@media (min-width: 640px) { font-size: 1.5rem; }
	}

	/* ─── Cards Column ─── */
	.is-cards-col {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
		order: 1;

		@media (min-width: 640px) { gap: 1rem; }
		@media (min-width: 1024px) { order: 2; }
	}

	/* ─── Indicator Card ─── */
	.is-card {
		display: block;
		position: relative;
		padding: 1rem;
		border-radius: var(--radius-xl);
		background-color: oklch(0.15 0.005 285 / 0.3);
		backdrop-filter: blur(4px);
		border: 1px solid oklch(0.3 0.005 285 / 0.5);
		transition: border-color 300ms, background-color 300ms, transform 100ms;
		text-decoration: none;

		@media (min-width: 640px) { padding: 1.25rem; border-radius: 1rem; }

		&:hover { border-color: oklch(0.35 0.005 285 / 0.5); background-color: oklch(0.25 0.005 285 / 0.3); }
		&:active { transform: scale(0.98); }
		&:hover .is-card-icon { transform: scale(1.1); }
		&:hover .is-card-name { color: oklch(0.65 0.15 260); }
		&:hover .is-card-arrow { background-color: oklch(0.6 0.18 260 / 0.2); transform: translateX(0.25rem); }
		&:hover :global(.is-arrow-icon) { color: oklch(0.65 0.15 260); }
	}

	.is-card[data-active] {
		outline: 2px solid oklch(0.6 0.18 260 / 0.3);
		outline-offset: -2px;
		background-color: oklch(0.25 0.005 285 / 0.4);
	}

	.is-card-row {
		display: flex;
		align-items: flex-start;
		gap: 0.75rem;

		@media (min-width: 640px) { gap: 1rem; }
	}

	.is-card-icon {
		flex-shrink: 0;
		inline-size: 2.5rem;
		block-size: 2.5rem;
		border-radius: var(--radius-lg);
		display: flex;
		align-items: center;
		justify-content: center;
		box-shadow: 0 10px 15px oklch(0 0 0 / 0.1);
		color: oklch(1 0 0);
		transition: transform 300ms;

		@media (min-width: 640px) { inline-size: 3rem; block-size: 3rem; border-radius: var(--radius-xl); }
	}

	.is-card[data-id='rsi'] .is-card-icon { background: linear-gradient(to bottom right, oklch(0.6 0.18 260), oklch(0.65 0.15 195)); }
	.is-card[data-id='macd'] .is-card-icon { background: linear-gradient(to bottom right, oklch(0.65 0.17 160), oklch(0.6 0.15 175)); }
	.is-card[data-id='bollinger'] .is-card-icon { background: linear-gradient(to bottom right, oklch(0.55 0.2 300), oklch(0.5 0.18 310)); }
	.is-card[data-id='vwap'] .is-card-icon { background: linear-gradient(to bottom right, oklch(0.75 0.15 85), oklch(0.65 0.17 55)); }

	.is-card-content { flex: 1; min-inline-size: 0; }

	.is-card-name-row {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		flex-wrap: wrap;
		margin-block-end: 0.125rem;

		@media (min-width: 640px) { margin-block-end: 0.25rem; }
	}

	.is-card-name {
		font-size: var(--text-base);
		font-weight: var(--weight-semibold);
		color: oklch(1 0 0);
		transition: color 200ms;

		@media (min-width: 640px) { font-size: var(--text-lg); }
	}

	.is-card-cat {
		padding-inline: 0.375rem;
		padding-block: 0.125rem;
		font-size: 0.625rem;
		font-weight: var(--weight-medium);
		border-radius: 999px;
		background-color: oklch(0.25 0.005 285);
		color: oklch(0.55 0.01 265);

		@media (min-width: 640px) { padding-inline: 0.5rem; font-size: var(--text-xs); }
	}

	.is-card-fullname {
		font-size: var(--text-xs);
		color: oklch(0.45 0.005 285);
		margin-block-end: 0.25rem;
		display: none;

		@media (min-width: 640px) { display: block; font-size: var(--text-sm); margin-block-end: 0.5rem; }
	}

	.is-card-desc {
		font-size: var(--text-xs);
		color: oklch(0.55 0.01 265);
		line-height: 1.7;
		display: -webkit-box;
		-webkit-line-clamp: 2;
		line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;

		@media (min-width: 640px) { font-size: var(--text-sm); }
	}

	.is-card-arrow {
		flex-shrink: 0;
		inline-size: 2rem;
		block-size: 2rem;
		border-radius: 50%;
		background-color: oklch(0.25 0.005 285 / 0.5);
		display: flex;
		align-items: center;
		justify-content: center;
		transition: background-color 300ms, transform 300ms;

		@media (min-width: 640px) { inline-size: 2.5rem; block-size: 2.5rem; }
	}

	:global(.is-arrow-icon) { color: oklch(0.45 0.005 285); transition: color 200ms; }

	/* ─── CTA ─── */
	.is-cta-wrap {
		padding-block-start: 1rem;
		text-align: center;

		@media (min-width: 640px) { padding-block-start: 1.5rem; text-align: start; }
	}

	.is-cta-btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		inline-size: 100%;
		padding-inline: 1.5rem;
		padding-block: 0.875rem;
		border-radius: 999px;
		background: linear-gradient(to right, oklch(0.5 0.18 260), oklch(0.55 0.15 195));
		color: oklch(1 0 0);
		font-weight: var(--weight-semibold);
		font-size: var(--text-sm);
		box-shadow: 0 10px 25px oklch(0.5 0.18 260 / 0.25);
		transition: box-shadow 300ms, transform 300ms;
		text-decoration: none;

		@media (min-width: 640px) { inline-size: auto; gap: 0.75rem; padding-inline: 2rem; padding-block: 1rem; font-size: var(--text-base); }

		&:hover { box-shadow: 0 10px 25px oklch(0.5 0.18 260 / 0.4); transform: scale(1.05); }
		&:active { transform: scale(0.98); }
	}

	:global(.is-cta-arrow) { transition: transform 200ms; }
	.is-cta-btn:hover :global(.is-cta-arrow) { transform: translateX(0.25rem); }

	/* ─── Keyframes ─── */
	@keyframes orb-pulse {
		0%, 100% { opacity: 1; }
		50% { opacity: 0.4; }
	}

	/* ─── Reduced Motion ─── */
	@media (prefers-reduced-motion: reduce) {
		.is-orb, .is-live-dot { animation: none; }
		.is-card { transition: none; }
	}
</style>

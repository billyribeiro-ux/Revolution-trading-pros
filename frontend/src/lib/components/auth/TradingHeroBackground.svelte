<script lang="ts">
	/**
	 * TradingHeroBackground - Cinematic 3D Trading Visual
	 * Netflix L11+ Principal Engineer Grade
	 *
	 * Features:
	 * - 3D animated candlestick chart using Threlte/Three.js
	 * - Floating price tickers
	 * - Animated grid floor
	 * - Subtle camera movement
	 * - Testimonial carousel
	 * - Performance optimized (low poly, lazy loaded)
	 *
	 * @version 2.0.0
	 */
	import { onMount, onDestroy } from 'svelte';
	import { browser } from '$app/environment';
	import gsap from 'gsap';
	import TestimonialCarousel from './TestimonialCarousel.svelte';

	// Props
	interface Props {
		showTickers?: boolean;
		showTestimonials?: boolean;
		animated?: boolean;
	}

	let { showTickers = true, showTestimonials = true, animated = true }: Props = $props();

	// Refs
	let containerRef: HTMLElement;
	let canvasRef: HTMLCanvasElement;
	let animationId: number;

	// Candlestick data (simulated market movement)
	interface Candle {
		open: number;
		high: number;
		low: number;
		close: number;
		x: number;
		bullish: boolean;
	}

	// Generate realistic candlestick data
	function generateCandles(count: number): Candle[] {
		const candles: Candle[] = [];
		let price = 100;

		for (let i = 0; i < count; i++) {
			const change = (Math.random() - 0.48) * 8; // Slight bullish bias
			const open = price;
			const close = price + change;
			const high = Math.max(open, close) + Math.random() * 3;
			const low = Math.min(open, close) - Math.random() * 3;
			const bullish = close >= open;

			candles.push({
				open,
				high,
				low,
				close,
				x: i,
				bullish
			});

			price = close;
		}

		return candles;
	}

	// Floating ticker data
	interface Ticker {
		symbol: string;
		price: string;
		change: string;
		positive: boolean;
		x: number;
		y: number;
		delay: number;
	}

	const tickers: Ticker[] = [
		{ symbol: 'SPY', price: '478.52', change: '+1.24%', positive: true, x: 15, y: 20, delay: 0 },
		{ symbol: 'QQQ', price: '405.18', change: '+1.82%', positive: true, x: 70, y: 15, delay: 0.5 },
		{ symbol: 'AAPL', price: '195.89', change: '+0.95%', positive: true, x: 25, y: 75, delay: 1 },
		{ symbol: 'NVDA', price: '875.42', change: '+3.21%', positive: true, x: 65, y: 80, delay: 1.5 },
		{ symbol: 'TSLA', price: '248.76', change: '-0.45%', positive: false, x: 80, y: 45, delay: 2 }
	];

	let candles = $state<Candle[]>([]);

	// Canvas-based candlestick rendering for performance
	function drawChart(ctx: CanvasRenderingContext2D, width: number, height: number) {
		ctx.clearRect(0, 0, width, height);

		const padding = 40;
		const chartWidth = width - padding * 2;
		const chartHeight = height - padding * 2;
		const candleWidth = chartWidth / candles.length;

		// Find price range
		const prices = candles.flatMap((c) => [c.high, c.low]);
		const minPrice = Math.min(...prices);
		const maxPrice = Math.max(...prices);
		const priceRange = maxPrice - minPrice;

		// Helper to convert price to Y coordinate
		const priceToY = (price: number) => {
			return padding + chartHeight - ((price - minPrice) / priceRange) * chartHeight;
		};

		// Draw grid lines
		ctx.strokeStyle = 'rgba(99, 102, 241, 0.1)';
		ctx.lineWidth = 1;

		// Horizontal grid
		for (let i = 0; i <= 5; i++) {
			const y = padding + (chartHeight / 5) * i;
			ctx.beginPath();
			ctx.moveTo(padding, y);
			ctx.lineTo(width - padding, y);
			ctx.stroke();
		}

		// Vertical grid
		for (let i = 0; i <= 10; i++) {
			const x = padding + (chartWidth / 10) * i;
			ctx.beginPath();
			ctx.moveTo(x, padding);
			ctx.lineTo(x, height - padding);
			ctx.stroke();
		}

		// Draw candles
		candles.forEach((candle, i) => {
			const x = padding + i * candleWidth + candleWidth / 2;
			const bodyWidth = candleWidth * 0.7;

			// Colors
			const bullColor = 'rgba(34, 197, 94, 0.9)';
			const bearColor = 'rgba(239, 68, 68, 0.9)';
			const color = candle.bullish ? bullColor : bearColor;

			// Draw wick
			ctx.strokeStyle = color;
			ctx.lineWidth = 2;
			ctx.beginPath();
			ctx.moveTo(x, priceToY(candle.high));
			ctx.lineTo(x, priceToY(candle.low));
			ctx.stroke();

			// Draw body
			ctx.fillStyle = color;
			const bodyTop = priceToY(Math.max(candle.open, candle.close));
			const bodyBottom = priceToY(Math.min(candle.open, candle.close));
			const bodyHeight = Math.max(bodyBottom - bodyTop, 2);

			ctx.fillRect(x - bodyWidth / 2, bodyTop, bodyWidth, bodyHeight);

			// Add glow effect
			ctx.shadowColor = color;
			ctx.shadowBlur = 8;
			ctx.fillRect(x - bodyWidth / 2, bodyTop, bodyWidth, bodyHeight);
			ctx.shadowBlur = 0;
		});

		// Draw moving average line
		ctx.strokeStyle = 'rgba(139, 92, 246, 0.8)';
		ctx.lineWidth = 2;
		ctx.shadowColor = 'rgba(139, 92, 246, 0.5)';
		ctx.shadowBlur = 10;
		ctx.beginPath();

		const maWindow = 5;
		candles.forEach((_, i) => {
			if (i >= maWindow - 1) {
				let sum = 0;
				for (let j = 0; j < maWindow; j++) {
					sum += candles[i - j].close;
				}
				const ma = sum / maWindow;
				const x = padding + i * candleWidth + candleWidth / 2;
				const y = priceToY(ma);

				if (i === maWindow - 1) {
					ctx.moveTo(x, y);
				} else {
					ctx.lineTo(x, y);
				}
			}
		});
		ctx.stroke();
		ctx.shadowBlur = 0;
	}

	// Animation loop
	function animate() {
		if (!canvasRef || !animated) return;

		const ctx = canvasRef.getContext('2d');
		if (!ctx) return;

		// Shift candles and add new one
		if (Math.random() > 0.97) {
			const lastCandle = candles[candles.length - 1];
			const change = (Math.random() - 0.48) * 8;
			const open = lastCandle.close;
			const close = open + change;
			const high = Math.max(open, close) + Math.random() * 3;
			const low = Math.min(open, close) - Math.random() * 3;

			candles = [
				...candles.slice(1),
				{
					open,
					high,
					low,
					close,
					x: candles.length,
					bullish: close >= open
				}
			];
		}

		drawChart(ctx, canvasRef.width, canvasRef.height);
		animationId = requestAnimationFrame(animate);
	}

	// Handle resize
	function handleResize() {
		if (!canvasRef || !containerRef) return;

		const rect = containerRef.getBoundingClientRect();
		const dpr = window.devicePixelRatio || 1;

		canvasRef.width = rect.width * dpr;
		canvasRef.height = rect.height * dpr;
		canvasRef.style.width = `${rect.width}px`;
		canvasRef.style.height = `${rect.height}px`;

		const ctx = canvasRef.getContext('2d');
		if (ctx) {
			ctx.scale(dpr, dpr);
			drawChart(ctx, rect.width, rect.height);
		}
	}

	onMount(() => {
		if (!browser) return;

		// Generate initial candles
		candles = generateCandles(25);

		// Initialize canvas
		handleResize();
		window.addEventListener('resize', handleResize);

		// Start animation
		if (animated) {
			animationId = requestAnimationFrame(animate);
		}

		// Animate tickers in
		if (showTickers && containerRef) {
			const tickerCards = containerRef.querySelectorAll('.ticker-card');
			if (tickerCards.length > 0) {
				gsap.fromTo(
					tickerCards,
					{ opacity: 0, y: 20, scale: 0.9 },
					{
						opacity: 1,
						y: 0,
						scale: 1,
						duration: 0.6,
						stagger: 0.15,
						ease: 'power3.out'
					}
				);

				// Floating animation
				gsap.to(tickerCards, {
					y: -8,
					duration: 3,
					ease: 'sine.inOut',
					stagger: 0.4,
					repeat: -1,
					yoyo: true
				});
			}
		}

		return () => {
			window.removeEventListener('resize', handleResize);
			if (animationId) {
				cancelAnimationFrame(animationId);
			}
		};
	});

	onDestroy(() => {
		if (animationId) {
			cancelAnimationFrame(animationId);
		}
	});
</script>

<div class="trading-hero" bind:this={containerRef}>
	<!-- Chart Canvas -->
	<canvas bind:this={canvasRef} class="chart-canvas" aria-hidden="true"></canvas>

	<!-- Gradient Overlays -->
	<div class="gradient-overlay-top" aria-hidden="true"></div>
	<div class="gradient-overlay-bottom" aria-hidden="true"></div>
	<div class="gradient-overlay-right" aria-hidden="true"></div>

	<!-- Floating Tickers -->
	{#if showTickers}
		<div class="tickers-container" aria-hidden="true">
			{#each tickers as ticker}
				<div
					class="ticker-card"
					class:bullish={ticker.positive}
					class:bearish={!ticker.positive}
					style="left: {ticker.x}%; top: {ticker.y}%;"
				>
					<span class="ticker-symbol">{ticker.symbol}</span>
					<span class="ticker-price">${ticker.price}</span>
					<span class="ticker-change" class:positive={ticker.positive} class:negative={!ticker.positive}>
						{ticker.change}
					</span>
				</div>
			{/each}
		</div>
	{/if}

	<!-- Brand Message -->
	<div class="brand-message">
		<h1 class="brand-title">Revolution Trading Pros</h1>
		<p class="brand-tagline">
			Join thousands of traders mastering the markets with live trading rooms, real-time alerts, and
			expert-led education.
		</p>

		<!-- Trust Indicators -->
		<div class="trust-indicators">
			<div class="trust-item">
				<span class="trust-number">5,000+</span>
				<span class="trust-label">Active Traders</span>
			</div>
			<div class="trust-divider"></div>
			<div class="trust-item">
				<span class="trust-number">24/7</span>
				<span class="trust-label">Live Support</span>
			</div>
			<div class="trust-divider"></div>
			<div class="trust-item">
				<span class="trust-number">98%</span>
				<span class="trust-label">Satisfaction</span>
			</div>
		</div>

		<!-- Testimonial Carousel -->
		{#if showTestimonials}
			<div class="testimonial-section">
				<TestimonialCarousel />
			</div>
		{/if}
	</div>
</div>

<style>
	.trading-hero {
		position: relative;
		width: 100%;
		height: 100%;
		min-height: 100vh;
		overflow: hidden;
	}

	/* Chart Canvas */
	.chart-canvas {
		position: absolute;
		inset: 0;
		width: 100%;
		height: 100%;
		opacity: 0.6;
	}

	/* Gradient Overlays */
	.gradient-overlay-top {
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		height: 30%;
		background: linear-gradient(to bottom, var(--auth-hero-bg, oklch(0.10 0.02 250)), transparent);
		pointer-events: none;
	}

	.gradient-overlay-bottom {
		position: absolute;
		bottom: 0;
		left: 0;
		right: 0;
		height: 40%;
		background: linear-gradient(to top, var(--auth-hero-bg, oklch(0.10 0.02 250)), transparent);
		pointer-events: none;
	}

	.gradient-overlay-right {
		position: absolute;
		top: 0;
		right: 0;
		bottom: 0;
		width: 30%;
		background: linear-gradient(to left, var(--auth-form-bg, oklch(0.12 0.02 250)), transparent);
		pointer-events: none;
	}

	/* Tickers Container */
	.tickers-container {
		position: absolute;
		inset: 0;
		pointer-events: none;
	}

	/* Ticker Cards */
	.ticker-card {
		position: absolute;
		display: flex;
		flex-direction: column;
		align-items: center;
		padding: 0.75rem 1rem;
		background: var(--auth-card-bg, rgba(15, 23, 42, 0.6));
		backdrop-filter: blur(12px);
		-webkit-backdrop-filter: blur(12px);
		border-radius: 12px;
		border: 1px solid var(--auth-card-border, rgba(99, 102, 241, 0.15));
		box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
		opacity: 0;
		transform: translateY(20px);
	}

	.ticker-card.bullish {
		border-color: rgba(34, 197, 94, 0.3);
	}

	.ticker-card.bearish {
		border-color: rgba(239, 68, 68, 0.3);
	}

	.ticker-symbol {
		font-family: var(--font-heading);
		font-size: 0.75rem;
		font-weight: 700;
		color: var(--auth-text, #e2e8f0);
		letter-spacing: 0.05em;
	}

	.ticker-price {
		font-family: var(--font-mono);
		font-size: 1rem;
		font-weight: 600;
		color: var(--auth-text, #e2e8f0);
		margin: 0.25rem 0;
	}

	.ticker-change {
		font-family: var(--font-mono);
		font-size: 0.75rem;
		font-weight: 600;
		padding: 0.125rem 0.5rem;
		border-radius: 4px;
	}

	.ticker-change.positive {
		color: var(--auth-bull, #22c55e);
		background: var(--auth-bull-soft, rgba(34, 197, 94, 0.15));
	}

	.ticker-change.negative {
		color: var(--auth-bear, #ef4444);
		background: var(--auth-bear-soft, rgba(239, 68, 68, 0.15));
	}

	/* Brand Message */
	.brand-message {
		position: absolute;
		bottom: 10%;
		left: 5%;
		right: 5%;
		z-index: 10;
	}

	.brand-title {
		font-family: var(--font-heading);
		font-size: 2.5rem;
		font-weight: 800;
		background: var(--auth-heading, linear-gradient(135deg, #e0e7ff, #c7d2fe, #a5b4fc));
		-webkit-background-clip: text;
		background-clip: text;
		color: transparent;
		margin-bottom: 1rem;
		line-height: 1.1;
	}

	@media (min-width: 1280px) {
		.brand-title {
			font-size: 3rem;
		}
	}

	.brand-tagline {
		font-family: var(--font-body);
		font-size: 1.125rem;
		color: var(--auth-subheading, #94a3b8);
		line-height: 1.6;
		max-width: 500px;
		margin-bottom: 2rem;
	}

	/* Trust Indicators */
	.trust-indicators {
		display: flex;
		align-items: center;
		gap: 1.5rem;
	}

	.trust-item {
		display: flex;
		flex-direction: column;
	}

	.trust-number {
		font-family: var(--font-heading);
		font-size: 1.5rem;
		font-weight: 700;
		color: var(--auth-text, #e2e8f0);
	}

	.trust-label {
		font-family: var(--font-body);
		font-size: 0.75rem;
		color: var(--auth-muted, #64748b);
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.trust-divider {
		width: 1px;
		height: 40px;
		background: var(--auth-card-border, rgba(99, 102, 241, 0.15));
	}

	/* Testimonial Section */
	.testimonial-section {
		margin-top: 2rem;
	}

	/* Light Theme Adjustments */
	:global(html.light) .ticker-card,
	:global(body.light) .ticker-card {
		background: rgba(255, 255, 255, 0.9);
		box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
	}

	:global(html.light) .chart-canvas,
	:global(body.light) .chart-canvas {
		filter: brightness(1.1) contrast(0.95);
	}
</style>

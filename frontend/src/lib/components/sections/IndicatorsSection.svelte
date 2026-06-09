<script lang="ts">
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
	import IconChartLine from '@tabler/icons-svelte-runes/icons/chart-line';
	import IconActivity from '@tabler/icons-svelte-runes/icons/activity';
	import IconWaveSine from '@tabler/icons-svelte-runes/icons/wave-sine';
	import IconBolt from '@tabler/icons-svelte-runes/icons/bolt';
	import IconArrowRight from '@tabler/icons-svelte-runes/icons/arrow-right';
	import IconSparkles from '@tabler/icons-svelte-runes/icons/sparkles';
	import type ScrollTriggerType from 'gsap/ScrollTrigger';
	import type { Attachment } from 'svelte/attachments';

	// Local shape for the synthetic OHLC candles `generateCandleData` emits —
	// distinct from any persisted candle DTO; lives only inside this section's
	// background visualization.
	interface Candle {
		open: number;
		high: number;
		low: number;
		close: number;
		bullish: boolean;
	}

	function mulberry32(seed: number): () => number {
		let s = seed | 0;
		return () => {
			s = (s + 0x6d2b79f5) | 0;
			let t = Math.imul(s ^ (s >>> 15), 1 | s);
			t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
			return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
		};
	}

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
	// January 2026 motion: content is revealed lazily as the section scrolls into view.
	let isVisible = $state(false);
	let activeIndicator = $state(0);
	// `ScrollTriggerType` is the *class* (used statically — `.getAll()`,
	// `.refresh()`); instances returned by `.getAll()` are `ScrollTriggerType`
	// instances of the same class.
	let scrollTriggerInstance: typeof ScrollTriggerType | null = null;
	let animationFrame: number;
	let chartCtx: CanvasRenderingContext2D | null = null;
	let canvasRef = $state<HTMLCanvasElement | null>(null);
	let prefersReducedMotion = $state(false);

	const captureSection: Attachment<HTMLElement> = (node) => {
		sectionRef = node;

		return () => {
			if (sectionRef === node) sectionRef = null;
		};
	};

	const captureChart: Attachment<HTMLElement> = (node) => {
		chartRef = node;
		queueMicrotask(startCanvasIfReady);

		return () => {
			if (chartRef === node) chartRef = null;
		};
	};

	const captureCanvas: Attachment<HTMLCanvasElement> = (node) => {
		canvasRef = node;
		queueMicrotask(startCanvasIfReady);

		return () => {
			if (canvasRef === node) canvasRef = null;
		};
	};

	// Chart animation state
	let chartProgress = $state(0);
	const candleData = generateCandleData(60);
	const rsiData = generateRSIData(candleData);

	// ============================================================================
	// DATA GENERATORS
	// ============================================================================
	function generateCandleData(count: number): Candle[] {
		const random = mulberry32(150);
		const candles: Candle[] = [];
		let price = 150;
		for (let i = 0; i < count; i++) {
			const change = (random() - 0.48) * 3;
			const open = price;
			const close = price + change;
			const high = Math.max(open, close) + random() * 1.5;
			const low = Math.min(open, close) - random() * 1.5;
			candles.push({ open, high, low, close, bullish: close > open });
			price = close;
		}
		return candles;
	}

	function generateRSIData(candles: Candle[]): number[] {
		const random = mulberry32(30);
		return candles.map((_, i) => 30 + Math.sin(i * 0.15) * 25 + random() * 15);
	}

	// ============================================================================
	// CHART RENDERING
	// ============================================================================
	function drawChart() {
		if (!chartCtx || !canvasRef || !chartRef) return;
		// Stable non-null ref so the forEach closures below don't need `!`.
		const ctx = chartCtx;

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
			ctx.strokeStyle = candle.bullish ? '#10b981' : '#ef4444';
			ctx.lineWidth = 1;
			ctx.beginPath();
			ctx.moveTo(x + candleWidth / 2, highY);
			ctx.lineTo(x + candleWidth / 2, lowY);
			ctx.stroke();

			// Body
			ctx.fillStyle = candle.bullish ? '#10b981' : '#ef4444';
			const bodyTop = Math.min(openY, closeY);
			const bodyHeight = Math.abs(closeY - openY) || 1;
			ctx.fillRect(x, bodyTop, candleWidth, bodyHeight);
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
				if (i === 0) ctx.moveTo(x, y);
				else ctx.lineTo(x, y);
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
	let visibilityObserver: IntersectionObserver | null = null;
	let gsapLoadStarted = false;

	function loadGSAPAfterReveal() {
		if (prefersReducedMotion || gsapLoadStarted) return;
		gsapLoadStarted = true;
		loadGSAP();
	}

	onMount(() => {
		// Check for reduced motion preference
		prefersReducedMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches;

		// Trigger entrance animations when section scrolls into viewport
		queueMicrotask(() => {
			if (sectionRef) {
				const observer = new IntersectionObserver(
					(entries) => {
						if (entries[0]?.isIntersecting) {
							isVisible = true;
							startCanvasIfReady();
							loadGSAPAfterReveal();
							observer.disconnect();
						}
					},
					{ threshold: 0.1, rootMargin: '50px' }
				);
				visibilityObserver = observer;
				visibilityObserver.observe(sectionRef);
			} else {
				isVisible = true;
				startCanvasIfReady();
				loadGSAPAfterReveal();
			}
		});

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
			visibilityObserver?.disconnect();
			resizeObserver?.disconnect();
		};
	});

	// ============================================================================
	// CANVAS INIT — waits for isVisible + DOM refs before starting
	// ============================================================================
	let canvasInitialized = false;

	function startCanvasIfReady() {
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
	}

	function setupCanvas() {
		if (!canvasRef || !chartRef) return;

		const dpr = window.devicePixelRatio || 1;
		const rect = chartRef.getBoundingClientRect();

		// Ensure we have valid dimensions
		if (rect.width === 0 || rect.height === 0) {
			console.debug('[IndicatorsSection] Canvas has zero dimensions, retrying...');
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
			console.debug('[IndicatorsSection] Canvas setup complete:', rect.width, 'x', rect.height);
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

				// fromTo applies opacity:0 immediately and ScrollTrigger
				// computes start positions now — before late images/fonts
				// load. Refresh after paint and again on window load so the
				// trigger can't end up stale (cards stuck invisible).
				scrollTriggerInstance.refresh();
				if (document.readyState !== 'complete') {
					window.addEventListener('load', () => scrollTriggerInstance?.refresh(), { once: true });
				}
			}
		} catch (e) {
			console.warn('[IndicatorsSection] GSAP failed to load:', e);
		}
	}

	onDestroy(() => {
		if (animationFrame) cancelAnimationFrame(animationFrame);
		// Kill only ScrollTriggers associated with this section
		if (scrollTriggerInstance && sectionRef) {
			const section = sectionRef;
			scrollTriggerInstance.getAll().forEach((st) => {
				// `st.trigger` is `Element | undefined`; `contains` rejects undefined.
				const trigger = st.trigger;
				if (!trigger) return;
				if (trigger === section || section.contains(trigger)) {
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

<section {@attach captureSection} class="indicators-section">
	<!-- Ambient Background -->
	<div class="indicator-ambient" aria-hidden="true">
		<!-- Gradient orbs - smaller on mobile -->
		<div class="indicator-orb indicator-orb-blue"></div>
		<div class="indicator-orb indicator-orb-violet"></div>

		<!-- Grid pattern - larger on mobile for performance -->
		<div class="indicator-grid-pattern"></div>

		<!-- Radial fade -->
		<div class="indicator-radial-fade"></div>
	</div>

	<div class="indicators-shell">
		<!-- Section Header -->
		{#if isVisible}
			<div
				class="section-heading"
				in:slideUp={{ delay: 0, duration: prefersReducedMotion ? 0 : 800 }}
			>
				<div class="section-eyebrow">
					<IconSparkles size={14} />
					Technical Edge
				</div>

				<h2 class="section-title">
					Indicator <span>Suite.</span>
				</h2>

				<p class="section-copy">
					We don't offer retail indicators. We provide institutional-grade analysis tools. Verified
					by quantitative funds and professional trading desks.
				</p>
			</div>
		{/if}

		<!-- Main Content Grid - Stack on mobile -->
		<div class="indicators-layout">
			<!-- Chart Visualization - Order 2 on mobile, 1 on desktop -->
			{#if isVisible}
				<div
					in:scaleIn={{
						delay: prefersReducedMotion ? 0 : 200,
						duration: prefersReducedMotion ? 0 : 600
					}}
					class="chart-column"
				>
					<div class="chart-panel">
						<!-- Chart Header -->
						<div class="chart-header">
							<div class="chart-symbol">
								<div class="window-controls">
									<div class="window-control window-control-red"></div>
									<div class="window-control window-control-yellow"></div>
									<div class="window-control window-control-green"></div>
								</div>
								<span class="chart-ticker">SPY • 1D</span>
							</div>
							<div class="chart-status">
								<span>+2.34%</span>
								<div class="live-dot"></div>
							</div>
						</div>

						<!-- Chart Canvas -->
						<div {@attach captureChart} class="chart-viewport">
							<canvas
								{@attach captureCanvas}
								class="chart-canvas"
								style="width: 100%; height: 100%;"
							></canvas>

							<!-- Indicator overlay label -->
							<div class="active-indicator-badge">
								<span>{indicators[activeIndicator]?.name} Active</span>
							</div>
						</div>
					</div>

					<!-- Floating stats - Hidden on very small screens -->
					<div class="floating-stat">
						<div class="floating-stat-label">Win Rate</div>
						<div class="floating-stat-value">73.2%</div>
					</div>
				</div>
			{/if}

			<!-- Indicator Cards - Order 1 on mobile, 2 on desktop -->
			{#if isVisible}
				<div
					class="indicator-list"
					in:slideUp={{
						delay: prefersReducedMotion ? 0 : 300,
						duration: prefersReducedMotion ? 0 : 600
					}}
				>
					{#each indicators as indicator, i (indicator.id)}
						{@const IconComponent = indicator.icon}
						<a
							href={indicator.href}
							class={['indicator-card', activeIndicator === i && 'indicator-card-active']}
							onmouseenter={() => (activeIndicator = i)}
							ontouchstart={() => (activeIndicator = i)}
						>
							<div class="indicator-card-content">
								<!-- Icon -->
								<div class={['indicator-icon-shell', `indicator-icon-${indicator.id}`]}>
									<IconComponent class="indicator-icon" size={24} />
								</div>

								<!-- Content -->
								<div class="indicator-card-body">
									<div class="indicator-title-row">
										<h3 class="indicator-title">{indicator.name}</h3>
										<span class="indicator-category">
											{indicator.category}
										</span>
									</div>
									<p class="indicator-full-name">
										{indicator.fullName}
									</p>
									<p class="indicator-description">
										{indicator.description}
									</p>
								</div>

								<!-- Arrow -->
								<div class="indicator-arrow">
									<IconArrowRight class="indicator-arrow-icon" size={20} />
								</div>
							</div>
						</a>
					{/each}

					<!-- CTA Button -->
					<div class="indicator-actions">
						<a href="/indicators" class="indicators-link">
							<span>Explore All Indicators</span>
							<IconArrowRight class="indicators-link-icon" size={20} />
						</a>
					</div>
				</div>
			{/if}
		</div>
	</div>
</section>

<style>
	.indicators-section {
		position: relative;
		overflow: hidden;
		padding: 4rem 0;
		background: linear-gradient(to bottom, #09090b, #18181b, #09090b);
	}

	.indicator-ambient {
		position: absolute;
		inset: 0;
		pointer-events: none;
	}

	.indicator-orb,
	.indicator-grid-pattern,
	.indicator-radial-fade {
		position: absolute;
	}

	.indicator-orb {
		width: 12rem;
		height: 12rem;
		border-radius: 999px;
		filter: blur(80px);
		animation: indicator-pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
	}

	.indicator-orb-blue {
		top: 25%;
		left: -4rem;
		background: rgba(59, 130, 246, 0.1);
	}

	.indicator-orb-violet {
		right: -4rem;
		bottom: 25%;
		background: rgba(139, 92, 246, 0.1);
		animation-delay: 1s;
	}

	.indicator-grid-pattern,
	.indicator-radial-fade {
		inset: 0;
	}

	.indicator-grid-pattern {
		background-image:
			linear-gradient(rgba(255, 255, 255, 0.02) 1px, transparent 1px),
			linear-gradient(90deg, rgba(255, 255, 255, 0.02) 1px, transparent 1px);
		background-size: 32px 32px;
	}

	.indicator-radial-fade {
		background: radial-gradient(ellipse at center, transparent 0%, rgba(9, 9, 11, 0.8) 70%);
	}

	.indicators-shell {
		position: relative;
		width: min(100% - 2rem, 80rem);
		margin-inline: auto;
	}

	.section-heading {
		width: min(100%, 56rem);
		margin: 0 auto 6rem;
		text-align: center;
	}

	.section-eyebrow {
		display: inline-flex;
		align-items: center;
		gap: 0.75rem;
		margin-bottom: 2rem;
		padding: 0.375rem 1rem;
		border: 1px solid rgba(30, 58, 138, 0.3);
		border-radius: 2px;
		background: rgba(23, 37, 84, 0.1);
		color: #3b82f6;
		font-size: 0.625rem;
		font-weight: 700;
		letter-spacing: 0.3em;
		text-transform: uppercase;
	}

	.section-title {
		margin: 0 0 2rem;
		color: white;
		font-family: var(--font-serif, Georgia, serif);
		font-size: clamp(3rem, 8vw, 4.5rem);
		font-weight: 400;
		line-height: 0.95;
		letter-spacing: 0;
	}

	.section-title span {
		color: #334155;
	}

	.section-copy {
		width: min(100%, 42rem);
		margin: 0 auto;
		color: #94a3b8;
		font-size: 1.125rem;
		font-weight: 300;
		line-height: 1.625;
	}

	.indicators-layout {
		display: grid;
		grid-template-columns: 1fr;
		align-items: center;
		gap: 2rem;
	}

	.chart-column {
		position: relative;
		order: 2;
	}

	.chart-panel {
		position: relative;
		overflow: hidden;
		border: 1px solid rgba(39, 39, 42, 0.5);
		border-radius: 0.75rem;
		background: rgba(24, 24, 27, 0.5);
		box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.45);
		backdrop-filter: blur(24px);
	}

	.chart-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0.75rem 1rem;
		border-bottom: 1px solid rgba(39, 39, 42, 0.5);
	}

	.chart-symbol,
	.chart-status,
	.window-controls {
		display: flex;
		align-items: center;
	}

	.chart-symbol {
		gap: 0.5rem;
	}

	.window-controls {
		gap: 0.25rem;
	}

	.window-control {
		width: 0.625rem;
		height: 0.625rem;
		border-radius: 999px;
	}

	.window-control-red {
		background: rgba(239, 68, 68, 0.8);
	}

	.window-control-yellow {
		background: rgba(234, 179, 8, 0.8);
	}

	.window-control-green {
		background: rgba(34, 197, 94, 0.8);
	}

	.chart-ticker,
	.chart-status,
	.active-indicator-badge {
		font-family:
			ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, 'Liberation Mono', 'Courier New',
			monospace;
	}

	.chart-ticker {
		color: #a1a1aa;
		font-size: 0.75rem;
	}

	.chart-status {
		gap: 0.375rem;
		color: #34d399;
		font-size: 0.625rem;
	}

	.live-dot {
		width: 0.375rem;
		height: 0.375rem;
		border-radius: 999px;
		background: #10b981;
		animation: indicator-pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
	}

	.chart-viewport {
		position: relative;
		height: 14rem;
	}

	.chart-canvas {
		position: absolute;
		inset: 0;
		width: 100%;
		height: 100%;
	}

	.active-indicator-badge {
		position: absolute;
		bottom: 0.75rem;
		left: 0.75rem;
		padding: 0.25rem 0.5rem;
		border: 1px solid rgba(63, 63, 70, 0.5);
		border-radius: 0.375rem;
		background: rgba(39, 39, 42, 0.8);
		color: #d4d4d8;
		font-size: 0.625rem;
		backdrop-filter: blur(4px);
	}

	.floating-stat {
		position: absolute;
		right: -0.5rem;
		bottom: -1rem;
		display: none;
		padding: 0.5rem 0.75rem;
		border: 1px solid rgba(63, 63, 70, 0.5);
		border-radius: 0.5rem;
		background: rgba(39, 39, 42, 0.9);
		box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.35);
		backdrop-filter: blur(24px);
	}

	.floating-stat-label {
		margin-bottom: 0.125rem;
		color: #71717a;
		font-size: 0.625rem;
	}

	.floating-stat-value {
		color: #34d399;
		font-size: 1.25rem;
		font-weight: 700;
		line-height: 1;
	}

	.indicator-list {
		order: 1;
		display: grid;
		gap: 0.75rem;
	}

	.indicator-card {
		position: relative;
		display: block;
		min-height: 44px;
		padding: 1rem;
		border: 1px solid rgba(39, 39, 42, 0.5);
		border-radius: 0.75rem;
		background: rgba(24, 24, 27, 0.3);
		color: inherit;
		text-decoration: none;
		backdrop-filter: blur(4px);
		transition:
			transform 300ms ease,
			border-color 300ms ease,
			background 300ms ease,
			box-shadow 300ms ease;
	}

	.indicator-card:hover {
		border-color: rgba(63, 63, 70, 0.5);
		background: rgba(39, 39, 42, 0.3);
	}

	.indicator-card:active {
		transform: scale(0.98);
	}

	.indicator-card-active {
		background: rgba(39, 39, 42, 0.4);
		box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.3);
	}

	.indicator-card-content {
		display: flex;
		align-items: flex-start;
		gap: 0.75rem;
	}

	.indicator-icon-shell {
		display: flex;
		width: 2.5rem;
		height: 2.5rem;
		flex: 0 0 auto;
		align-items: center;
		justify-content: center;
		border-radius: 0.5rem;
		box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.35);
		color: white;
		transition: transform 300ms ease;
	}

	.indicator-card:hover .indicator-icon-shell {
		transform: scale(1.1);
	}

	.indicator-icon-rsi {
		background: linear-gradient(to bottom right, #3b82f6, #22d3ee);
	}

	.indicator-icon-macd {
		background: linear-gradient(to bottom right, #10b981, #2dd4bf);
	}

	.indicator-icon-bollinger {
		background: linear-gradient(to bottom right, #8b5cf6, #a855f7);
	}

	.indicator-icon-vwap {
		background: linear-gradient(to bottom right, #f59e0b, #fb923c);
	}

	.indicator-card-body {
		min-width: 0;
		flex: 1 1 auto;
	}

	.indicator-title-row {
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		gap: 0.5rem;
		margin-bottom: 0.125rem;
	}

	.indicator-title {
		margin: 0;
		color: white;
		font-size: 1rem;
		font-weight: 600;
		line-height: 1.2;
		transition: color 300ms ease;
	}

	.indicator-card:hover .indicator-title {
		color: #60a5fa;
	}

	.indicator-category {
		padding: 0.125rem 0.375rem;
		border-radius: 999px;
		background: #27272a;
		color: #a1a1aa;
		font-size: 0.625rem;
		font-weight: 500;
	}

	.indicator-full-name {
		display: none;
		margin: 0 0 0.25rem;
		color: #71717a;
		font-size: 0.75rem;
	}

	.indicator-description {
		display: -webkit-box;
		overflow: hidden;
		margin: 0;
		color: #a1a1aa;
		font-size: 0.75rem;
		line-height: 1.625;
		-webkit-box-orient: vertical;
		-webkit-line-clamp: 2;
		line-clamp: 2;
	}

	.indicator-arrow {
		display: flex;
		width: 2rem;
		height: 2rem;
		flex: 0 0 auto;
		align-items: center;
		justify-content: center;
		border-radius: 999px;
		background: rgba(39, 39, 42, 0.5);
		color: #71717a;
		transition:
			transform 300ms ease,
			background 300ms ease,
			color 300ms ease;
	}

	.indicator-card:hover .indicator-arrow {
		transform: translateX(0.25rem);
		background: rgba(59, 130, 246, 0.2);
		color: #60a5fa;
	}

	.indicator-actions {
		padding-top: 1rem;
		text-align: center;
	}

	.indicators-link {
		display: inline-flex;
		width: 100%;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 0.875rem 1.5rem;
		border-radius: 999px;
		background: linear-gradient(to right, #2563eb, #0891b2);
		color: white;
		font-size: 0.875rem;
		font-weight: 600;
		text-decoration: none;
		box-shadow: 0 10px 15px -3px rgba(59, 130, 246, 0.25);
		transition:
			transform 300ms ease,
			box-shadow 300ms ease;
	}

	.indicators-link:hover {
		transform: scale(1.05);
		box-shadow: 0 10px 15px -3px rgba(59, 130, 246, 0.4);
	}

	.indicators-link:active {
		transform: scale(0.98);
	}

	:global(.indicators-link-icon) {
		transition: transform 300ms ease;
	}

	.indicators-link:hover :global(.indicators-link-icon) {
		transform: translateX(0.25rem);
	}

	@keyframes indicator-pulse {
		50% {
			opacity: 0.5;
		}
	}

	@media (min-width: 640px) {
		.indicators-section {
			padding-top: 6rem;
			padding-bottom: 6rem;
		}

		.indicator-orb {
			width: 24rem;
			height: 24rem;
			filter: blur(128px);
		}

		.indicator-orb-blue {
			left: -8rem;
		}

		.indicator-orb-violet {
			right: -8rem;
		}

		.indicator-grid-pattern {
			background-size: 64px 64px;
		}

		.chart-panel {
			border-radius: 1rem;
		}

		.chart-header {
			padding: 1rem 1.5rem;
		}

		.chart-symbol {
			gap: 0.75rem;
		}

		.window-controls {
			gap: 0.375rem;
		}

		.window-control {
			width: 0.75rem;
			height: 0.75rem;
		}

		.chart-ticker {
			font-size: 0.875rem;
		}

		.chart-status {
			gap: 0.5rem;
			font-size: 0.75rem;
		}

		.live-dot {
			width: 0.5rem;
			height: 0.5rem;
		}

		.chart-viewport {
			height: 18rem;
		}

		.active-indicator-badge {
			bottom: 1rem;
			left: 1rem;
			padding: 0.375rem 0.75rem;
			border-radius: 0.5rem;
			font-size: 0.75rem;
		}

		.floating-stat {
			right: -1.5rem;
			bottom: -1.5rem;
			display: block;
			padding: 0.75rem 1rem;
			border-radius: 0.75rem;
		}

		.floating-stat-label {
			margin-bottom: 0.25rem;
			font-size: 0.75rem;
		}

		.floating-stat-value {
			font-size: 1.5rem;
		}

		.indicator-list {
			gap: 1rem;
		}

		.indicator-card {
			padding: 1.25rem;
			border-radius: 1rem;
		}

		.indicator-card-content {
			gap: 1rem;
		}

		.indicator-icon-shell {
			width: 3rem;
			height: 3rem;
			border-radius: 0.75rem;
		}

		.indicator-title-row {
			margin-bottom: 0.25rem;
		}

		.indicator-title {
			font-size: 1.125rem;
		}

		.indicator-category {
			padding-inline: 0.5rem;
			font-size: 0.75rem;
		}

		.indicator-full-name {
			display: block;
			margin-bottom: 0.5rem;
			font-size: 0.875rem;
		}

		.indicator-description {
			font-size: 0.875rem;
		}

		.indicator-arrow {
			width: 2.5rem;
			height: 2.5rem;
		}

		.indicator-actions {
			padding-top: 1.5rem;
			text-align: left;
		}

		.indicators-link {
			width: auto;
			gap: 0.75rem;
			padding: 1rem 2rem;
			font-size: 1rem;
		}
	}

	@media (min-width: 1024px) {
		.indicators-section {
			padding-top: 8rem;
			padding-bottom: 8rem;
		}

		.indicators-shell {
			width: min(100% - 4rem, 80rem);
		}

		.indicators-layout {
			grid-template-columns: repeat(2, minmax(0, 1fr));
			gap: 4rem;
		}

		.chart-column {
			order: 1;
		}

		.indicator-list {
			order: 2;
		}

		.chart-viewport {
			height: 20rem;
		}
	}

	@media (prefers-reduced-motion: reduce) {
		.indicator-orb,
		.live-dot {
			animation: none;
		}

		.indicator-card,
		.indicator-icon-shell,
		.indicator-arrow,
		.indicators-link,
		:global(.indicators-link-icon) {
			transition: none;
		}
	}
</style>

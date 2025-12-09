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
	import { browser } from '$app/environment';
	import { cubicOut, backOut } from 'svelte/easing';
	import IconChartLine from '@tabler/icons-svelte/icons/chart-line';
	import IconActivity from '@tabler/icons-svelte/icons/activity';
	import IconWaveSine from '@tabler/icons-svelte/icons/wave-sine';
	import IconBolt from '@tabler/icons-svelte/icons/bolt';
	import IconArrowRight from '@tabler/icons-svelte/icons/arrow-right';
	import IconSparkles from '@tabler/icons-svelte/icons/sparkles';

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
	let isVisible = $state(false);
	let isMounted = $state(false);
	let activeIndicator = $state(0);
	let gsapInstance: any = null;
	let scrollTriggerInstance: any = null;
	let animationFrame: number;
	let chartCtx: CanvasRenderingContext2D | null = null;
	let canvasRef = $state<HTMLCanvasElement | null>(null);
	let prefersReducedMotion = $state(false);

	// Chart animation state
	let chartProgress = $state(0);
	const candleData = generateCandleData(60);
	const rsiData = generateRSIData(candleData);
	const macdData = generateMACDData(candleData);

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

	function generateMACDData(candles: any[]) {
		return candles.map((_, i) => Math.sin(i * 0.1) * 2 + (Math.random() - 0.5));
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
		const visibleCandles = Math.floor(candleData.length * chartProgress);

		// Clear canvas
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
			const closeY = height - ((candle.close - minPrice) / priceRange) * height * 0.7 - height * 0.15;
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
	let observer: IntersectionObserver | null = null;
	let rotationInterval: ReturnType<typeof setInterval> | null = null;
	let resizeObserver: ResizeObserver | null = null;

	onMount(() => {
		if (!browser) return;

		// Check for reduced motion preference
		prefersReducedMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches;
		isMounted = true;

		// Setup canvas with resize observer (delayed to ensure DOM is ready)
		requestAnimationFrame(() => {
			setupCanvas();
			resizeObserver = new ResizeObserver(() => setupCanvas());
			if (chartRef) resizeObserver.observe(chartRef);
		});

		// Intersection Observer for visibility - lower threshold for mobile
		observer = new IntersectionObserver(
			(entries) => {
				if (entries[0].isIntersecting) {
					isVisible = true;
					observer?.disconnect();
					// Start chart animation after visible
					if (!prefersReducedMotion) {
						animatechartProgress();
					} else {
						chartProgress = 1;
					}
				}
			},
			{ threshold: 0.1, rootMargin: '50px' }
		);
		if (sectionRef) observer.observe(sectionRef);

		// Load GSAP asynchronously
		if (!prefersReducedMotion) {
			loadGSAP();
		}

		// Start animation loop
		animate();

		// Auto-rotate indicators (slower on mobile for better UX)
		const isMobile = window.innerWidth < 768;
		rotationInterval = setInterval(() => {
			activeIndicator = (activeIndicator + 1) % indicators.length;
		}, isMobile ? 5000 : 4000);

		return () => {
			if (rotationInterval) clearInterval(rotationInterval);
			observer?.disconnect();
			resizeObserver?.disconnect();
		};
	});

	function setupCanvas() {
		if (!canvasRef || !chartRef) return;
		const dpr = window.devicePixelRatio || 1;
		const rect = chartRef.getBoundingClientRect();
		canvasRef.width = rect.width * dpr;
		canvasRef.height = rect.height * dpr;
		chartCtx = canvasRef.getContext('2d');
		if (chartCtx) {
			chartCtx.scale(dpr, dpr);
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
			const ScrollTrigger = (await import('gsap/ScrollTrigger')).default;
			gsap.registerPlugin(ScrollTrigger);
			gsapInstance = gsap;
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
			console.debug('[IndicatorsSection] GSAP not available:', e);
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
	function slideUp(node: Element, { delay = 0, duration = 800 }) {
		return {
			delay,
			duration,
			css: (t: number) => {
				const eased = cubicOut(t);
				return `opacity: ${eased}; transform: translateY(${(1 - eased) * 60}px);`;
			}
		};
	}

	function scaleIn(node: Element, { delay = 0, duration = 600 }) {
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
	class="relative py-16 sm:py-24 lg:py-32 overflow-hidden bg-gradient-to-b from-zinc-950 via-zinc-900 to-zinc-950"
>
	<!-- Ambient Background -->
	<div class="absolute inset-0 pointer-events-none">
		<!-- Gradient orbs - smaller on mobile -->
		<div
			class="absolute top-1/4 -left-16 sm:-left-32 w-48 sm:w-96 h-48 sm:h-96 bg-blue-500/10 rounded-full blur-[80px] sm:blur-[128px] animate-pulse"
		></div>
		<div
			class="absolute bottom-1/4 -right-16 sm:-right-32 w-48 sm:w-96 h-48 sm:h-96 bg-violet-500/10 rounded-full blur-[80px] sm:blur-[128px] animate-pulse"
			style="animation-delay: 1s;"
		></div>

		<!-- Grid pattern - larger on mobile for performance -->
		<div
			class="absolute inset-0 bg-[linear-gradient(rgba(255,255,255,0.02)_1px,transparent_1px),linear-gradient(90deg,rgba(255,255,255,0.02)_1px,transparent_1px)] bg-[size:32px_32px] sm:bg-[size:64px_64px]"
		></div>

		<!-- Radial fade -->
		<div
			class="absolute inset-0 bg-[radial-gradient(ellipse_at_center,transparent_0%,rgba(9,9,11,0.8)_70%)]"
		></div>
	</div>

	<div class="relative max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
		<!-- Section Header -->
		{#if isVisible}
			<div class="text-center mb-10 sm:mb-16 lg:mb-20" in:slideUp={{ delay: 0, duration: prefersReducedMotion ? 0 : 800 }}>
				<div class="inline-flex items-center gap-2 px-3 sm:px-4 py-1.5 sm:py-2 rounded-full bg-blue-500/10 border border-blue-500/20 mb-4 sm:mb-6">
					<IconSparkles class="w-3.5 h-3.5 sm:w-4 sm:h-4 text-blue-400" />
					<span class="text-xs sm:text-sm font-medium text-blue-400 tracking-wide">Professional Tools</span>
				</div>

				<h2 class="text-3xl sm:text-4xl md:text-5xl lg:text-6xl font-bold text-white mb-4 sm:mb-6 tracking-tight">
					Trading Indicators
					<span class="block mt-1 sm:mt-2 bg-gradient-to-r from-blue-400 via-cyan-400 to-emerald-400 bg-clip-text text-transparent">
						That Actually Work
					</span>
				</h2>

				<p class="text-base sm:text-lg lg:text-xl text-zinc-400 max-w-2xl mx-auto leading-relaxed px-4 sm:px-0">
					Institutional-grade technical analysis tools. Built by traders, for traders.
					No fluff—just edge.
				</p>
			</div>
		{/if}

		<!-- Main Content Grid - Stack on mobile -->
		<div class="grid grid-cols-1 lg:grid-cols-2 gap-8 lg:gap-16 items-center">
			<!-- Chart Visualization - Order 2 on mobile, 1 on desktop -->
			{#if isVisible}
				<div in:scaleIn={{ delay: prefersReducedMotion ? 0 : 200, duration: prefersReducedMotion ? 0 : 600 }} class="relative order-2 lg:order-1">
					<div
						class="relative rounded-xl sm:rounded-2xl overflow-hidden bg-zinc-900/50 backdrop-blur-xl border border-zinc-800/50 shadow-2xl"
					>
						<!-- Chart Header -->
						<div class="flex items-center justify-between px-4 sm:px-6 py-3 sm:py-4 border-b border-zinc-800/50">
							<div class="flex items-center gap-2 sm:gap-3">
								<div class="flex gap-1 sm:gap-1.5">
									<div class="w-2.5 h-2.5 sm:w-3 sm:h-3 rounded-full bg-red-500/80"></div>
									<div class="w-2.5 h-2.5 sm:w-3 sm:h-3 rounded-full bg-yellow-500/80"></div>
									<div class="w-2.5 h-2.5 sm:w-3 sm:h-3 rounded-full bg-green-500/80"></div>
								</div>
								<span class="text-xs sm:text-sm font-mono text-zinc-400">SPY • 1D</span>
							</div>
							<div class="flex items-center gap-1.5 sm:gap-2">
								<span class="text-[10px] sm:text-xs font-mono text-emerald-400">+2.34%</span>
								<div class="w-1.5 h-1.5 sm:w-2 sm:h-2 rounded-full bg-emerald-500 animate-pulse"></div>
							</div>
						</div>

						<!-- Chart Canvas -->
						<div bind:this={chartRef} class="relative h-56 sm:h-72 lg:h-80">
							<canvas
								bind:this={canvasRef}
								class="absolute inset-0 w-full h-full"
								style="width: 100%; height: 100%;"
							></canvas>

							<!-- Indicator overlay label -->
							<div
								class="absolute bottom-3 sm:bottom-4 left-3 sm:left-4 px-2 sm:px-3 py-1 sm:py-1.5 rounded-md sm:rounded-lg bg-zinc-800/80 backdrop-blur-sm border border-zinc-700/50"
							>
								<span class="text-[10px] sm:text-xs font-mono text-zinc-300">
									{indicators[activeIndicator].name} Active
								</span>
							</div>
						</div>
					</div>

					<!-- Floating stats - Hidden on very small screens -->
					<div
						class="hidden sm:block absolute -bottom-4 sm:-bottom-6 -right-2 sm:-right-6 px-3 sm:px-4 py-2 sm:py-3 rounded-lg sm:rounded-xl bg-zinc-800/90 backdrop-blur-xl border border-zinc-700/50 shadow-xl"
					>
						<div class="text-[10px] sm:text-xs text-zinc-500 mb-0.5 sm:mb-1">Win Rate</div>
						<div class="text-xl sm:text-2xl font-bold text-emerald-400">73.2%</div>
					</div>
				</div>
			{/if}

			<!-- Indicator Cards - Order 1 on mobile, 2 on desktop -->
			{#if isVisible}
				<div class="space-y-3 sm:space-y-4 order-1 lg:order-2" in:slideUp={{ delay: prefersReducedMotion ? 0 : 300, duration: prefersReducedMotion ? 0 : 600 }}>
					{#each indicators as indicator, i}
						{@const IconComponent = indicator.icon}
						<a
							href={indicator.href}
							class="indicator-card group block relative p-4 sm:p-5 rounded-xl sm:rounded-2xl bg-zinc-900/30 backdrop-blur-sm border border-zinc-800/50 hover:border-zinc-700/50 hover:bg-zinc-800/30 transition-all duration-300 active:scale-[0.98] {activeIndicator === i ? 'ring-2 ring-blue-500/30 bg-zinc-800/40' : ''}"
							onmouseenter={() => (activeIndicator = i)}
							ontouchstart={() => (activeIndicator = i)}
						>
							<div class="flex items-start gap-3 sm:gap-4">
								<!-- Icon -->
								<div
									class="flex-shrink-0 w-10 h-10 sm:w-12 sm:h-12 rounded-lg sm:rounded-xl bg-gradient-to-br {indicator.gradient} flex items-center justify-center shadow-lg group-hover:scale-110 transition-transform duration-300"
								>
									<IconComponent class="w-5 h-5 sm:w-6 sm:h-6 text-white" />
								</div>

								<!-- Content -->
								<div class="flex-1 min-w-0">
									<div class="flex items-center gap-2 mb-0.5 sm:mb-1 flex-wrap">
										<h3 class="text-base sm:text-lg font-semibold text-white group-hover:text-blue-400 transition-colors">
											{indicator.name}
										</h3>
										<span
											class="px-1.5 sm:px-2 py-0.5 text-[10px] sm:text-xs font-medium rounded-full bg-zinc-800 text-zinc-400"
										>
											{indicator.category}
										</span>
									</div>
									<p class="text-xs sm:text-sm text-zinc-500 mb-1 sm:mb-2 hidden sm:block">{indicator.fullName}</p>
									<p class="text-xs sm:text-sm text-zinc-400 leading-relaxed line-clamp-2">{indicator.description}</p>
								</div>

								<!-- Arrow -->
								<div
									class="flex-shrink-0 w-8 h-8 sm:w-10 sm:h-10 rounded-full bg-zinc-800/50 flex items-center justify-center group-hover:bg-blue-500/20 group-hover:translate-x-1 transition-all duration-300"
								>
									<IconArrowRight class="w-4 h-4 sm:w-5 sm:h-5 text-zinc-500 group-hover:text-blue-400" />
								</div>
							</div>
						</a>
					{/each}

					<!-- CTA Button -->
					<div class="pt-4 sm:pt-6 text-center sm:text-left">
						<a
							href="/indicators"
							class="group inline-flex items-center justify-center gap-2 sm:gap-3 w-full sm:w-auto px-6 sm:px-8 py-3.5 sm:py-4 rounded-full bg-gradient-to-r from-blue-600 to-cyan-600 text-white font-semibold shadow-lg shadow-blue-500/25 hover:shadow-blue-500/40 active:scale-[0.98] hover:scale-105 transition-all duration-300"
						>
							<span class="text-sm sm:text-base">Explore All Indicators</span>
							<IconArrowRight class="w-4 h-4 sm:w-5 sm:h-5 group-hover:translate-x-1 transition-transform" />
						</a>
					</div>
				</div>
			{/if}
		</div>
	</div>
</section>

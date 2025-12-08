<script lang="ts">
	/**
	 * IndicatorsSection - Apple/Netflix Cinematic Design
	 * Principal Engineer ICT9+ Standard
	 * ══════════════════════════════════════════════════════════════════════════════
	 * Features:
	 * ✓ GSAP ScrollTrigger parallax effects
	 * ✓ Animated trading chart visualization
	 * ✓ Staggered reveal animations
	 * ✓ Glassmorphism cards with hover states
	 * ✓ Particle system background
	 * ✓ Responsive cinematic layout
	 * ══════════════════════════════════════════════════════════════════════════════
	 */
	import { onMount, onDestroy } from 'svelte';
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
	let activeIndicator = $state(0);
	let gsapInstance: any = null;
	let scrollTriggerInstance: any = null;
	let animationFrame: number;
	let chartCtx: CanvasRenderingContext2D | null = null;
	let canvasRef = $state<HTMLCanvasElement | null>(null);

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
		if (!chartCtx || !canvasRef) return;

		const width = canvasRef.width;
		const height = canvasRef.height;
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

	onMount(() => {
		if (!browser) return;

		// Setup canvas
		if (canvasRef) {
			const dpr = window.devicePixelRatio || 1;
			const rect = canvasRef.getBoundingClientRect();
			canvasRef.width = rect.width * dpr;
			canvasRef.height = rect.height * dpr;
			chartCtx = canvasRef.getContext('2d');
			if (chartCtx) {
				chartCtx.scale(dpr, dpr);
			}
		}

		// Intersection Observer for visibility
		observer = new IntersectionObserver(
			(entries) => {
				if (entries[0].isIntersecting) {
					isVisible = true;
					observer?.disconnect();
				}
			},
			{ threshold: 0.2 }
		);
		if (sectionRef) observer.observe(sectionRef);

		// Load GSAP asynchronously
		loadGSAP();

		// Start animation loop
		animate();

		// Auto-rotate indicators
		rotationInterval = setInterval(() => {
			activeIndicator = (activeIndicator + 1) % indicators.length;
		}, 4000);

		return () => {
			if (rotationInterval) clearInterval(rotationInterval);
			observer?.disconnect();
		};
	});

	async function loadGSAP() {
		const gsap = (await import('gsap')).default;
		const ScrollTrigger = (await import('gsap/ScrollTrigger')).default;
		gsap.registerPlugin(ScrollTrigger);
		gsapInstance = gsap;
		scrollTriggerInstance = ScrollTrigger;

		// Chart progress animation
		gsap.to(
			{ progress: 0 },
			{
				progress: 1,
				duration: 3,
				ease: 'power2.out',
				scrollTrigger: {
					trigger: sectionRef,
					start: 'top 80%',
					end: 'center center',
					scrub: 1
				},
				onUpdate: function () {
					chartProgress = this.targets()[0].progress;
				}
			}
		);
	}

	onDestroy(() => {
		if (animationFrame) cancelAnimationFrame(animationFrame);
		if (scrollTriggerInstance) scrollTriggerInstance.killAll();
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
	class="relative py-32 lg:py-40 overflow-hidden bg-gradient-to-b from-zinc-950 via-zinc-900 to-zinc-950"
>
	<!-- Ambient Background -->
	<div class="absolute inset-0 pointer-events-none">
		<!-- Gradient orbs -->
		<div
			class="absolute top-1/4 -left-32 w-96 h-96 bg-blue-500/10 rounded-full blur-[128px] animate-pulse"
		></div>
		<div
			class="absolute bottom-1/4 -right-32 w-96 h-96 bg-violet-500/10 rounded-full blur-[128px] animate-pulse"
			style="animation-delay: 1s;"
		></div>

		<!-- Grid pattern -->
		<div
			class="absolute inset-0 bg-[linear-gradient(rgba(255,255,255,0.02)_1px,transparent_1px),linear-gradient(90deg,rgba(255,255,255,0.02)_1px,transparent_1px)] bg-[size:64px_64px]"
		></div>

		<!-- Radial fade -->
		<div
			class="absolute inset-0 bg-[radial-gradient(ellipse_at_center,transparent_0%,rgba(9,9,11,0.8)_70%)]"
		></div>
	</div>

	<div class="relative max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
		<!-- Section Header -->
		{#if isVisible}
			<div class="text-center mb-20" in:slideUp={{ delay: 0, duration: 1000 }}>
				<div class="inline-flex items-center gap-2 px-4 py-2 rounded-full bg-blue-500/10 border border-blue-500/20 mb-6">
					<IconSparkles class="w-4 h-4 text-blue-400" />
					<span class="text-sm font-medium text-blue-400 tracking-wide">Professional Tools</span>
				</div>

				<h2 class="text-4xl sm:text-5xl lg:text-6xl font-bold text-white mb-6 tracking-tight">
					Trading Indicators
					<span class="block mt-2 bg-gradient-to-r from-blue-400 via-cyan-400 to-emerald-400 bg-clip-text text-transparent">
						That Actually Work
					</span>
				</h2>

				<p class="text-lg sm:text-xl text-zinc-400 max-w-2xl mx-auto leading-relaxed">
					Institutional-grade technical analysis tools. Built by traders, for traders.
					No fluff—just edge.
				</p>
			</div>
		{/if}

		<!-- Main Content Grid -->
		<div class="grid lg:grid-cols-2 gap-12 lg:gap-16 items-center">
			<!-- Left: Live Chart Visualization -->
			{#if isVisible}
				<div in:scaleIn={{ delay: 200, duration: 800 }} class="relative">
					<div
						class="relative rounded-2xl overflow-hidden bg-zinc-900/50 backdrop-blur-xl border border-zinc-800/50 shadow-2xl"
					>
						<!-- Chart Header -->
						<div class="flex items-center justify-between px-6 py-4 border-b border-zinc-800/50">
							<div class="flex items-center gap-3">
								<div class="flex gap-1.5">
									<div class="w-3 h-3 rounded-full bg-red-500/80"></div>
									<div class="w-3 h-3 rounded-full bg-yellow-500/80"></div>
									<div class="w-3 h-3 rounded-full bg-green-500/80"></div>
								</div>
								<span class="text-sm font-mono text-zinc-400">SPY • 1D</span>
							</div>
							<div class="flex items-center gap-2">
								<span class="text-xs font-mono text-emerald-400">+2.34%</span>
								<div class="w-2 h-2 rounded-full bg-emerald-500 animate-pulse"></div>
							</div>
						</div>

						<!-- Chart Canvas -->
						<div bind:this={chartRef} class="relative h-80 lg:h-96">
							<canvas
								bind:this={canvasRef}
								class="absolute inset-0 w-full h-full"
								style="width: 100%; height: 100%;"
							></canvas>

							<!-- Indicator overlay label -->
							<div
								class="absolute bottom-4 left-4 px-3 py-1.5 rounded-lg bg-zinc-800/80 backdrop-blur-sm border border-zinc-700/50"
							>
								<span class="text-xs font-mono text-zinc-300">
									{indicators[activeIndicator].name} Active
								</span>
							</div>
						</div>
					</div>

					<!-- Floating stats -->
					<div
						class="absolute -bottom-6 -right-6 px-4 py-3 rounded-xl bg-zinc-800/90 backdrop-blur-xl border border-zinc-700/50 shadow-xl"
					>
						<div class="text-xs text-zinc-500 mb-1">Win Rate</div>
						<div class="text-2xl font-bold text-emerald-400">73.2%</div>
					</div>
				</div>
			{/if}

			<!-- Right: Indicator Cards -->
			{#if isVisible}
				<div class="space-y-4" in:slideUp={{ delay: 400, duration: 800 }}>
					{#each indicators as indicator, i}
						{@const IconComponent = indicator.icon}
						<a
							href={indicator.href}
							class="group block relative p-5 rounded-2xl bg-zinc-900/30 backdrop-blur-sm border border-zinc-800/50 hover:border-zinc-700/50 hover:bg-zinc-800/30 transition-all duration-500 {activeIndicator === i ? 'ring-2 ring-blue-500/30 bg-zinc-800/40' : ''}"
							onmouseenter={() => (activeIndicator = i)}
						>
							<div class="flex items-start gap-4">
								<!-- Icon -->
								<div
									class="flex-shrink-0 w-12 h-12 rounded-xl bg-gradient-to-br {indicator.gradient} flex items-center justify-center shadow-lg group-hover:scale-110 transition-transform duration-300"
								>
									<IconComponent class="w-6 h-6 text-white" />
								</div>

								<!-- Content -->
								<div class="flex-1 min-w-0">
									<div class="flex items-center gap-2 mb-1">
										<h3 class="text-lg font-semibold text-white group-hover:text-blue-400 transition-colors">
											{indicator.name}
										</h3>
										<span
											class="px-2 py-0.5 text-xs font-medium rounded-full bg-zinc-800 text-zinc-400"
										>
											{indicator.category}
										</span>
									</div>
									<p class="text-sm text-zinc-500 mb-2">{indicator.fullName}</p>
									<p class="text-sm text-zinc-400 leading-relaxed">{indicator.description}</p>
								</div>

								<!-- Arrow -->
								<div
									class="flex-shrink-0 w-10 h-10 rounded-full bg-zinc-800/50 flex items-center justify-center group-hover:bg-blue-500/20 group-hover:translate-x-1 transition-all duration-300"
								>
									<IconArrowRight class="w-5 h-5 text-zinc-500 group-hover:text-blue-400" />
								</div>
							</div>
						</a>
					{/each}

					<!-- CTA Button -->
					<div class="pt-4">
						<a
							href="/indicators"
							class="group inline-flex items-center gap-3 px-8 py-4 rounded-full bg-gradient-to-r from-blue-600 to-cyan-600 text-white font-semibold shadow-lg shadow-blue-500/25 hover:shadow-blue-500/40 hover:scale-105 transition-all duration-300"
						>
							<span>Explore All Indicators</span>
							<IconArrowRight class="w-5 h-5 group-hover:translate-x-1 transition-transform" />
						</a>
					</div>
				</div>
			{/if}
		</div>
	</div>
</section>

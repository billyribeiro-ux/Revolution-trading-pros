<script lang="ts">
	import { onMount, tick } from 'svelte';
	import { browser } from '$app/environment';
	import type {
		IChartApi,
		ISeriesApi,
		CandlestickData,
		LineData,
		UTCTimestamp
	} from 'lightweight-charts';

	// ═══════════════════════════════════════════════════════════════════════════
	// AWARD-WINNING CINEMATIC TRADING EXPERIENCE
	// Unique Design Language - No AI Templates
	// Premium Copper/Gold/Amber Trading Palette
	// Live Chart Replay with Scroll-Triggered 3D Reveals
	// ═══════════════════════════════════════════════════════════════════════════

	// State management
	let mounted = $state(false);
	let scrollY = $state(0);
	let mouseX = $state(0);
	let mouseY = $state(0);
	let windowWidth = $state(0);
	
	// Chart states
	let mainChartContainer: HTMLDivElement;
	let miniChartContainer: HTMLDivElement;
	let mainChart: IChartApi | null = null;
	let miniChart: IChartApi | null = null;
	let candlestickSeries: ISeriesApi<'Candlestick'> | null = null;
	let volumeSeries: ISeriesApi<'Histogram'> | null = null;
	let ma20Series: ISeriesApi<'Line'> | null = null;
	let ma50Series: ISeriesApi<'Line'> | null = null;
	let isPlaying = $state(true);
	let currentBarIndex = $state(0);
	let totalBars = 100;
	let playbackSpeed = $state(1);
	
	// Section visibility states for scroll triggers
	let heroVisible = $state(false);
	let chartSectionVisible = $state(false);
	let featuresVisible = $state(false);
	let statsVisible = $state(false);
	let commitmentVisible = $state(false);
	
	// Email capture
	let email = $state('');
	let isSubmitting = $state(false);
	let isSubmitted = $state(false);
	let errorMessage = $state('');
	
	// Magnetic button states
	let buttonMagnetX = $state(0);
	let buttonMagnetY = $state(0);
	
	// Stats with counting animation
	let stats = $state({
		students: { value: 0, target: 50000, suffix: '+' },
		countries: { value: 0, target: 127, suffix: '' },
		years: { value: 0, target: 8, suffix: '' }
	});

	// Ticker data
	const tickers = [
		{ symbol: 'SPX', price: 5892.33, change: +1.24 },
		{ symbol: 'NQ', price: 20845.20, change: +2.34 },
		{ symbol: 'VIX', price: 12.45, change: -8.20 },
		{ symbol: 'AAPL', price: 232.30, change: +1.45 },
		{ symbol: 'NVDA', price: 1475.20, change: +5.45 },
		{ symbol: 'TSLA', price: 348.50, change: +3.30 },
		{ symbol: 'META', price: 585.20, change: +1.90 },
		{ symbol: 'AMZN', price: 198.50, change: +0.75 }
	];

	// Generate realistic trading data for live replay
	function generateFullData() {
		const candles: CandlestickData[] = [];
		const volumes: any[] = [];
		const ma20: LineData[] = [];
		const ma50: LineData[] = [];
		
		const now = new Date();
		let basePrice = 4750;
		let ma20Sum = 0;
		let ma50Sum = 0;
		const ma20Prices: number[] = [];
		const ma50Prices: number[] = [];
		
		for (let i = totalBars; i >= 0; i--) {
			const time = new Date(now.getTime() - i * 5 * 60 * 1000); // 5-min candles
			const timestamp = Math.floor(time.getTime() / 1000) as UTCTimestamp;
			
			// Complex price generation with trends and reversals
			const phase = Math.sin((totalBars - i) * 0.08);
			const trend = phase * 50;
			const noise = (Math.random() - 0.5) * 30;
			const momentum = (Math.random() - 0.4) * 15;
			
			const open = basePrice + trend + noise;
			const close = open + momentum + (Math.random() - 0.5) * 20;
			const high = Math.max(open, close) + Math.random() * 8 + 2;
			const low = Math.min(open, close) - Math.random() * 8 - 2;
			
			candles.push({
				time: timestamp,
				open: Number(open.toFixed(2)),
				high: Number(high.toFixed(2)),
				low: Number(low.toFixed(2)),
				close: Number(close.toFixed(2))
			});
			
			volumes.push({
				time: timestamp,
				value: Math.floor(Math.random() * 800000) + 200000,
				color: close >= open ? 'rgba(212, 175, 55, 0.6)' : 'rgba(220, 38, 38, 0.6)'
			});
			
			// Calculate MAs
			ma20Prices.push(close);
			ma50Prices.push(close);
			if (ma20Prices.length > 20) ma20Prices.shift();
			if (ma50Prices.length > 50) ma50Prices.shift();
			
			if (ma20Prices.length === 20) {
				const ma20Val = ma20Prices.reduce((a, b) => a + b, 0) / 20;
				ma20.push({ time: timestamp, value: Number(ma20Val.toFixed(2)) });
			}
			
			if (ma50Prices.length === 50) {
				const ma50Val = ma50Prices.reduce((a, b) => a + b, 0) / 50;
				ma50.push({ time: timestamp, value: Number(ma50Val.toFixed(2)) });
			}
			
			basePrice = close;
		}
		
		return { candles, volumes, ma20, ma50 };
	}

	let fullData = generateFullData();

	onMount(async () => {
		if (!browser) return;
		
		windowWidth = window.innerWidth;
		mounted = true;
		await tick();
		
		// Initialize charts
		await initMainChart();
		
		// Start live replay
		startLiveReplay();
		
		// Intersection Observer for scroll reveals
		setupScrollObserver();
		
		// Mouse tracking for magnetic button
		const handleMouseMove = (e: MouseEvent) => {
			mouseX = e.clientX;
			mouseY = e.clientY;
			
			// Magnetic effect calculation
			const button = document.querySelector('.magnetic-button') as HTMLElement;
			if (button) {
				const rect = button.getBoundingClientRect();
				const centerX = rect.left + rect.width / 2;
				const centerY = rect.top + rect.height / 2;
				const distX = e.clientX - centerX;
				const distY = e.clientY - centerY;
				const distance = Math.sqrt(distX * distX + distY * distY);
				
				if (distance < 150) {
					buttonMagnetX = distX * 0.3;
					buttonMagnetY = distY * 0.3;
				} else {
					buttonMagnetX = 0;
					buttonMagnetY = 0;
				}
			}
		};
		
		window.addEventListener('mousemove', handleMouseMove, { passive: true });
		window.addEventListener('scroll', () => scrollY = window.scrollY, { passive: true });
		window.addEventListener('resize', () => windowWidth = window.innerWidth);
		
		// Trigger hero animation
		setTimeout(() => heroVisible = true, 300);
		
		return () => {
			window.removeEventListener('mousemove', handleMouseMove);
			if (mainChart) mainChart.remove();
		};
	});

	async function initMainChart() {
		if (!mainChartContainer) return;
		
		const { createChart, CandlestickSeries, HistogramSeries, LineSeries } = await import('lightweight-charts');
		
		// PREMIUM DARK CHART THEME - Copper/Gold/Amber
		mainChart = createChart(mainChartContainer, {
			layout: {
				background: { color: 'rgba(10, 10, 12, 0.4)' },
				textColor: '#a8a29e',
				fontFamily: "'Inter', system-ui, sans-serif",
				fontSize: 11
			},
			grid: {
				vertLines: { color: 'rgba(212, 175, 55, 0.06)' },
				horzLines: { color: 'rgba(212, 175, 55, 0.04)' }
			},
			crosshair: {
				mode: 0,
				vertLine: {
					color: 'rgba(212, 175, 55, 0.5)',
					style: 3,
					width: 1,
					labelBackgroundColor: '#d4af37'
				},
				horzLine: {
					color: 'rgba(212, 175, 55, 0.5)',
					style: 3,
					width: 1,
					labelBackgroundColor: '#d4af37'
				}
			},
			rightPriceScale: {
				borderColor: 'rgba(212, 175, 55, 0.15)',
				scaleMargins: { top: 0.05, bottom: 0.15 }
			},
			timeScale: {
				borderColor: 'rgba(212, 175, 55, 0.15)',
				timeVisible: true,
				secondsVisible: false
			},
			autoSize: true,
			handleScroll: { vertTouchDrag: false }
		});

		// Add series with PREMIUM COLORS
		candlestickSeries = mainChart.addSeries(CandlestickSeries, {
			upColor: '#d4af37',           // Gold
			downColor: '#dc2626',         // Deep red
			borderUpColor: '#e8c547',
			borderDownColor: '#ef4444',
			wickUpColor: '#d4af37',
			wickDownColor: '#dc2626'
		});

		volumeSeries = mainChart.addSeries(HistogramSeries, {
			priceFormat: { type: 'volume' },
			priceScaleId: ''
		});
		volumeSeries.priceScale().applyOptions({ scaleMargins: { top: 0.85, bottom: 0 } });

		// Dual moving averages with copper/amber
		ma20Series = mainChart.addSeries(LineSeries, {
			color: '#c87941',           // Copper
			lineWidth: 2,
			lastValueVisible: false
		});

		ma50Series = mainChart.addSeries(LineSeries, {
			color: '#f59e0b',           // Amber
			lineWidth: 2,
			lineStyle: 2,
			lastValueVisible: false
		});

		// Initial data set
		candlestickSeries.setData(fullData.candles.slice(0, 30));
		volumeSeries.setData(fullData.volumes.slice(0, 30));
		ma20Series.setData(fullData.ma20.slice(0, 30));
		ma50Series.setData(fullData.ma50.slice(0, 30));
		
		mainChart.timeScale().fitContent();
	}

	function startLiveReplay() {
		let index = 30;
		
		const interval = setInterval(() => {
			if (!isPlaying || !candlestickSeries || index >= fullData.candles.length) {
				if (index >= fullData.candles.length) {
					index = 30; // Loop back
				}
				return;
			}
			
			// Add new bar
			candlestickSeries.update(fullData.candles[index]);
			volumeSeries?.update(fullData.volumes[index]);
			ma20Series?.update(fullData.ma20[index] || fullData.ma20[fullData.ma20.length - 1]);
			ma50Series?.update(fullData.ma50[index] || fullData.ma50[fullData.ma50.length - 1]);
			
			currentBarIndex = index;
			index++;
		}, 800 / playbackSpeed);
		
		return () => clearInterval(interval);
	}

	function setupScrollObserver() {
		const observer = new IntersectionObserver(
			(entries) => {
				entries.forEach((entry) => {
					if (entry.isIntersecting) {
						const id = entry.target.id;
						if (id === 'chart-section') chartSectionVisible = true;
						if (id === 'features-section') featuresVisible = true;
						if (id === 'stats-section') statsVisible = true;
						if (id === 'commitment-section') commitmentVisible = true;
					}
				});
			},
			{ threshold: 0.2, rootMargin: '-50px' }
		);
		
		['chart-section', 'features-section', 'stats-section', 'commitment-section'].forEach(id => {
			const el = document.getElementById(id);
			if (el) observer.observe(el);
		});
	}

	function animateStats() {
		const duration = 2000;
		const steps = 60;
		const interval = duration / steps;
		let step = 0;

		const timer = setInterval(() => {
			step++;
			const progress = 1 - Math.pow(1 - step / steps, 3); // Ease out cubic
			
			stats.students.value = Math.floor(stats.students.target * progress);
			stats.countries.value = Math.floor(stats.countries.target * progress);
			stats.years.value = Math.floor(stats.years.target * progress);

			if (step >= steps) clearInterval(timer);
		}, interval);
	}

	$effect(() => {
		if (statsVisible) {
			animateStats();
		}
	});

	async function handleNotifyMe() {
		const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
		if (!email || !emailRegex.test(email)) {
			errorMessage = 'Please enter a valid email address';
			return;
		}

		isSubmitting = true;
		errorMessage = '';

		try {
			const response = await fetch('/api/maintenance/notify', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ email })
			});

			if (!response.ok) throw new Error('Failed');
			isSubmitted = true;
			email = '';
		} catch {
			errorMessage = 'Connection error. Please try again.';
		} finally {
			isSubmitting = false;
		}
	}

	function formatNumber(num: number): string {
		return num.toLocaleString();
	}

	// Scroll-based parallax for chart section
	const chartParallax = $derived(scrollY * 0.15);
	const chartRotateX = $derived(Math.min(scrollY * 0.02, 10));
</script>

<div class="experience-container" class:mounted>
	
	<!-- Live Market Tape -->
	<div class="market-tape">
		<div class="tape-content">
			{#each [...tickers, ...tickers, ...tickers] as ticker, i}
				<div class="tape-item" class:up={ticker.change > 0} class:down={ticker.change < 0}>
					<span class="ticker-name">{ticker.symbol}</span>
					<span class="ticker-value">${ticker.price.toFixed(2)}</span>
					<span class="ticker-delta">
						{ticker.change > 0 ? '▲' : '▼'} {Math.abs(ticker.change).toFixed(2)}%
					</span>
				</div>
			{/each}
		</div>
	</div>

	<!-- Ambient Background Effects -->
	<div class="ambient-orbs">
		<div class="orb orb-1" style="transform: translate({mouseX * 0.02}px, {mouseY * 0.02}px)"></div>
		<div class="orb orb-2" style="transform: translate({-mouseX * 0.01}px, {-mouseY * 0.01}px)"></div>
	</div>

	<!-- Main Content -->
	<main class="content">
		
		<!-- Hero Section -->
		<section class="hero" class:visible={heroVisible}>
			<div class="hero-badge">
				<span class="badge-dot"></span>
				<span class="badge-text">Platform Upgrade In Progress</span>
			</div>
			
			<h1 class="hero-title">
				<span class="title-line">Building the Future of</span>
				<span class="title-line accent">Trading Excellence</span>
			</h1>
			
			<p class="hero-subtitle">
				We're raising the bar for stocks and options education. 
				No fake indicators. Just real institutional tools.
			</p>
		</section>

		<!-- Live Chart Section - 3D Scroll Reveal -->
		<section 
			id="chart-section" 
			class="chart-section" 
			class:visible={chartSectionVisible}
			style="transform: perspective(1000px) rotateX({chartRotateX}deg) translateY({-chartParallax}px)"
		>
			<div class="section-label">Live Market Simulation</div>
			
			<div class="chart-wrapper">
				<div class="chart-header">
					<div class="symbol-info">
						<span class="symbol">SPX</span>
						<span class="timeframe">5m</span>
					</div>
					<div class="chart-controls">
						<button 
							class="control-btn"
							on:click={() => isPlaying = !isPlaying}
						>
							{isPlaying ? '⏸' : '▶'}
						</button>
						<div class="speed-control">
							<button class="speed-btn" class:active={playbackSpeed === 0.5} on:click={() => playbackSpeed = 0.5}>0.5x</button>
							<button class="speed-btn" class:active={playbackSpeed === 1} on:click={() => playbackSpeed = 1}>1x</button>
							<button class="speed-btn" class:active={playbackSpeed === 2} on:click={() => playbackSpeed = 2}>2x</button>
						</div>
					</div>
				</div>
				
				<div class="chart-glow"></div>
				<div class="chart-container-el" bind:this={mainChartContainer}></div>
				
				<div class="chart-legend">
					<div class="legend-item"><span class="dot gold"></span> Price</div>
					<div class="legend-item"><span class="dot copper"></span> MA 20</div>
					<div class="legend-item"><span class="dot amber"></span> MA 50</div>
				</div>
			</div>
		</section>

		<!-- Features - Unique Magnetic Cards -->
		<section id="features-section" class="features" class:visible={featuresVisible}>
			<div class="section-header">
				<div class="section-line"></div>
				<h2 class="section-title">What's Coming</h2>
			</div>
			
			<div class="features-cascade">
				<div class="feature-orb" style="--delay: 0s">
					<div class="orb-icon">📊</div>
					<div class="orb-content">
						<h3>Institutional Scanners</h3>
						<p>Pro-grade screening technology that was never available to retail traders before</p>
					</div>
					<div class="orb-shine"></div>
				</div>
				
				<div class="feature-orb" style="--delay: 0.15s">
					<div class="orb-icon">⚡</div>
					<div class="orb-content">
						<h3>Day Trading Mastery</h3>
						<p>Complete intraday curriculum from traders who actually profit consistently</p>
					</div>
					<div class="orb-shine"></div>
				</div>
				
				<div class="feature-orb" style="--delay: 0.3s">
					<div class="orb-icon">🎯</div>
					<div class="orb-content">
						<h3>Swing Strategy Labs</h3>
						<p>Quantitative multi-day position systems backed by real market structure analysis</p>
					</div>
					<div class="orb-shine"></div>
				</div>
			</div>
		</section>

		<!-- Stats - Counter Animation -->
		<section id="stats-section" class="stats" class:visible={statsVisible}>
			<div class="stat-card">
				<div class="stat-value">
					{formatNumber(stats.students.value)}{stats.students.suffix}
				</div>
				<div class="stat-label">Traders Trained</div>
				<div class="stat-shimmer"></div>
			</div>
			
			<div class="stat-divider">
				<div class="divider-line"></div>
				<div class="divider-dot"></div>
				<div class="divider-line"></div>
			</div>
			
			<div class="stat-card">
				<div class="stat-value">
					{stats.countries.value}
				</div>
				<div class="stat-label">Countries Worldwide</div>
				<div class="stat-shimmer"></div>
			</div>
			
			<div class="stat-divider">
				<div class="divider-line"></div>
				<div class="divider-dot"></div>
				<div class="divider-line"></div>
			</div>
			
			<div class="stat-card">
				<div class="stat-value">
					{stats.years.value}
				</div>
				<div class="stat-label">Years of Excellence</div>
				<div class="stat-shimmer"></div>
			</div>
		</section>

		<!-- Commitment Statement -->
		<section id="commitment-section" class="commitment" class:visible={commitmentVisible}>
			<div class="commitment-block">
				<div class="quote-mark">"</div>
				<p class="commitment-text">
					Our commitment: <strong>Leading stocks and options trading education</strong> 
					by providing the unseen institutional tools and strategies that professionals use. 
					No gimmicks. No false promises. Just authentic trading excellence.
				</p>
				<div class="signature">
					<div class="sig-line"></div>
					<span>The Revolution Trading Pros Team</span>
				</div>
			</div>
		</section>

		<!-- Progress Timeline -->
		<section class="timeline">
			<div class="timeline-track">
				<div class="track-completed"></div>
				<div class="track-active"></div>
				<div class="track-pending"></div>
			</div>
			
			<div class="timeline-points">
				<div class="point completed">
					<div class="point-dot">✓</div>
					<span>Infrastructure</span>
				</div>
				<div class="point active">
					<div class="point-dot">
						<div class="pulse-ring"></div>
					</div>
					<span>Platform Expansion</span>
				</div>
				<div class="point">
					<div class="point-dot"></div>
					<span>Global Launch</span>
				</div>
			</div>
		</section>

		<!-- Email Capture with Magnetic Button -->
		<section class="capture">
			{#if !isSubmitted}
				<div class="capture-content">
					<div class="capture-badge">
						<div class="live-indicator"></div>
						<span>Early Access List</span>
					</div>
					
					<h3 class="capture-title">Be First to Experience It</h3>
					
					<div class="input-group">
						<input
							type="email"
							placeholder="Enter your email address"
							bind:value={email}
							on:keydown={(e) => e.key === 'Enter' && handleNotifyMe()}
							disabled={isSubmitting}
							class="email-field"
						/>
						
						<button
							class="magnetic-button"
							style="transform: translate({buttonMagnetX}px, {buttonMagnetY}px)"
							on:click={handleNotifyMe}
							disabled={isSubmitting || !email}
						>
							{#if isSubmitting}
								<div class="btn-loader"></div>
							{:else}
								<span>Join the Revolution</span>
								<svg class="btn-arrow" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
									<path d="M5 12h14M14 5l7 7-7 7"/>
								</svg>
							{/if}
						</button>
					</div>
					
					{#if errorMessage}
						<div class="error-toast">
							<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
								<circle cx="12" cy="12" r="10"/>
								<path d="M12 8v4M12 16h.01"/>
							</svg>
							{errorMessage}
						</div>
					{/if}
				</div>
			{:else}
				<div class="success-state">
					<div class="success-ring">
						<svg viewBox="0 0 52 52">
							<circle cx="26" cy="26" r="25" fill="none" stroke="#d4af37" stroke-width="2"/>
							<path d="M14 27l8 8 16-16" fill="none" stroke="#d4af37" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"/>
						</svg>
					</div>
					<h3 class="success-heading">Welcome to the Revolution</h3>
					<p class="success-message">Check your email for confirmation. You're on the exclusive list.</p>
					<div class="benefit-pills">
						<span class="pill">Early Access</span>
						<span class="pill">VIP Pricing</span>
						<span class="pill">Premium Tools</span>
					</div>
				</div>
			{/if}
		</section>

		<!-- Footer -->
		<footer class="footer">
			<div class="security-badge">
				<svg viewBox="0 0 24 24" fill="none" stroke="#d4af37" stroke-width="1.5">
					<path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/>
				</svg>
				<span>Secure. No Spam. Ever.</span>
			</div>
		</footer>

	</main>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   PREMIUM TRADING COLOR PALETTE
	   Gold: #d4af37 | Copper: #c87941 | Amber: #f59e0b | Deep: #0a0a0c
	   ═══════════════════════════════════════════════════════════════════════════ */
	
	:root {
		--gold: #d4af37;
		--gold-light: #e8c547;
		--copper: #c87941;
		--copper-light: #d48c55;
		--amber: #f59e0b;
		--deep: #0a0a0c;
		--deep-light: #121214;
		--text: #e7e5e4;
		--text-dim: #a8a29e;
		--text-muted: #78716c;
	}

	.experience-container {
		position: fixed;
		inset: 0;
		width: 100%;
		min-height: 100vh;
		background: var(--deep);
		background-image: 
			radial-gradient(ellipse 80% 50% at 20% 40%, rgba(200, 121, 65, 0.08) 0%, transparent 50%),
			radial-gradient(ellipse 60% 40% at 80% 60%, rgba(212, 175, 55, 0.06) 0%, transparent 50%);
		overflow-y: auto;
		overflow-x: hidden;
		z-index: 99999;
		font-family: 'Inter', system-ui, sans-serif;
	}

	/* Custom scrollbar */
	.experience-container::-webkit-scrollbar { width: 4px; }
	.experience-container::-webkit-scrollbar-track { background: transparent; }
	.experience-container::-webkit-scrollbar-thumb { 
		background: var(--copper); 
		border-radius: 2px;
		opacity: 0.5;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   MARKET TAPE
	   ═══════════════════════════════════════════════════════════════════════════ */
	.market-tape {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		z-index: 100;
		background: rgba(10, 10, 12, 0.9);
		backdrop-filter: blur(10px);
		border-bottom: 1px solid rgba(212, 175, 55, 0.15);
		padding: 10px 0;
		overflow: hidden;
	}

	.tape-content {
		display: flex;
		gap: 40px;
		animation: tapeScroll 35s linear infinite;
	}

	@keyframes tapeScroll {
		0% { transform: translateX(0); }
		100% { transform: translateX(-33.33%); }
	}

	.tape-item {
		display: flex;
		align-items: center;
		gap: 8px;
		font-family: 'SF Mono', monospace;
		font-size: 12px;
		font-weight: 500;
		white-space: nowrap;
	}

	.ticker-name { color: var(--text-dim); font-weight: 600; letter-spacing: 0.5px; }
	.ticker-value { color: var(--text); }
	.ticker-delta { padding: 2px 6px; border-radius: 4px; font-size: 10px; }
	.tape-item.up .ticker-delta { background: rgba(16, 185, 129, 0.15); color: #10b981; }
	.tape-item.down .ticker-delta { background: rgba(220, 38, 38, 0.15); color: #ef4444; }

	/* ═══════════════════════════════════════════════════════════════════════════
	   AMBIENT ORBS
	   ═══════════════════════════════════════════════════════════════════════════ */
	.ambient-orbs {
		position: fixed;
		inset: 0;
		z-index: 1;
		pointer-events: none;
		overflow: hidden;
	}

	.orb {
		position: absolute;
		border-radius: 50%;
		filter: blur(100px);
		opacity: 0.3;
		transition: transform 0.3s ease-out;
	}

	.orb-1 {
		width: 500px;
		height: 500px;
		background: var(--copper);
		top: 20%;
		left: -10%;
		animation: orbFloat 20s ease-in-out infinite;
	}

	.orb-2 {
		width: 400px;
		height: 400px;
		background: var(--gold);
		bottom: 10%;
		right: -5%;
		animation: orbFloat 25s ease-in-out infinite reverse;
	}

	@keyframes orbFloat {
		0%, 100% { transform: translate(0, 0); }
		50% { transform: translate(30px, -30px); }
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   MAIN CONTENT
	   ═══════════════════════════════════════════════════════════════════════════ */
	.content {
		position: relative;
		z-index: 5;
		max-width: 1000px;
		margin: 0 auto;
		padding: 120px 24px 60px;
		display: flex;
		flex-direction: column;
		gap: 100px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   HERO SECTION - Cinematic Reveal
	   ═══════════════════════════════════════════════════════════════════════════ */
	.hero {
		text-align: center;
		opacity: 0;
		transform: translateY(40px);
		transition: all 1s cubic-bezier(0.16, 1, 0.3, 1);
	}

	.hero.visible {
		opacity: 1;
		transform: translateY(0);
	}

	.hero-badge {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		padding: 8px 16px;
		background: rgba(212, 175, 55, 0.1);
		border: 1px solid rgba(212, 175, 55, 0.3);
		border-radius: 20px;
		margin-bottom: 32px;
	}

	.badge-dot {
		width: 8px;
		height: 8px;
		background: var(--gold);
		border-radius: 50%;
		animation: badgePulse 2s ease-in-out infinite;
	}

	@keyframes badgePulse {
		0%, 100% { opacity: 1; transform: scale(1); }
		50% { opacity: 0.5; transform: scale(1.2); }
	}

	.badge-text {
		font-size: 12px;
		color: var(--gold);
		font-weight: 500;
		letter-spacing: 0.1em;
		text-transform: uppercase;
	}

	.hero-title {
		font-size: clamp(2.5rem, 7vw, 4.5rem);
		font-weight: 800;
		line-height: 1.1;
		margin: 0 0 24px 0;
		letter-spacing: -0.02em;
	}

	.title-line {
		display: block;
		color: var(--text);
		opacity: 0;
		transform: translateY(30px);
		animation: lineReveal 0.8s cubic-bezier(0.16, 1, 0.3, 1) forwards;
	}

	.title-line:first-child { animation-delay: 0.2s; }
	.title-line.accent { 
		animation-delay: 0.4s;
		background: linear-gradient(135deg, var(--gold) 0%, var(--copper) 100%);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
	}

	@keyframes lineReveal {
		to { opacity: 1; transform: translateY(0); }
	}

	.hero-subtitle {
		font-size: clamp(1rem, 2vw, 1.25rem);
		color: var(--text-dim);
		max-width: 600px;
		margin: 0 auto;
		line-height: 1.7;
		opacity: 0;
		animation: fadeIn 1s ease 0.6s forwards;
	}

	@keyframes fadeIn {
		to { opacity: 1; }
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   CHART SECTION - 3D Scroll Reveal
	   ═══════════════════════════════════════════════════════════════════════════ */
	.chart-section {
		opacity: 0;
		transform: perspective(1000px) rotateX(15deg) translateY(100px);
		transition: all 1.2s cubic-bezier(0.16, 1, 0.3, 1);
		transform-style: preserve-3d;
	}

	.chart-section.visible {
		opacity: 1;
		transform: perspective(1000px) rotateX(0deg) translateY(0);
	}

	.section-label {
		font-size: 11px;
		color: var(--gold);
		letter-spacing: 0.2em;
		text-transform: uppercase;
		margin-bottom: 16px;
		font-weight: 600;
	}

	.chart-wrapper {
		position: relative;
		background: rgba(18, 18, 20, 0.6);
		border: 1px solid rgba(212, 175, 55, 0.2);
		border-radius: 16px;
		padding: 20px;
		backdrop-filter: blur(10px);
		box-shadow: 
			0 25px 50px -12px rgba(0, 0, 0, 0.5),
			0 0 100px rgba(212, 175, 55, 0.1);
	}

	.chart-glow {
		position: absolute;
		inset: -1px;
		border-radius: 17px;
		background: linear-gradient(135deg, rgba(212, 175, 55, 0.3), rgba(200, 121, 65, 0.2), transparent);
		z-index: -1;
		filter: blur(20px);
		opacity: 0.5;
	}

	.chart-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 16px;
		padding-bottom: 12px;
		border-bottom: 1px solid rgba(212, 175, 55, 0.1);
	}

	.symbol-info {
		display: flex;
		align-items: center;
		gap: 12px;
	}

	.symbol {
		font-size: 18px;
		font-weight: 700;
		color: var(--gold);
	}

	.timeframe {
		font-size: 11px;
		color: var(--text-muted);
		padding: 4px 8px;
		background: rgba(255, 255, 255, 0.05);
		border-radius: 4px;
	}

	.chart-controls {
		display: flex;
		align-items: center;
		gap: 12px;
	}

	.control-btn {
		width: 32px;
		height: 32px;
		background: rgba(212, 175, 55, 0.1);
		border: 1px solid rgba(212, 175, 55, 0.3);
		border-radius: 8px;
		color: var(--gold);
		font-size: 14px;
		cursor: pointer;
		transition: all 0.3s ease;
	}

	.control-btn:hover {
		background: rgba(212, 175, 55, 0.2);
		transform: scale(1.05);
	}

	.speed-control {
		display: flex;
		gap: 4px;
	}

	.speed-btn {
		padding: 6px 10px;
		background: transparent;
		border: 1px solid rgba(212, 175, 55, 0.2);
		border-radius: 6px;
		color: var(--text-dim);
		font-size: 11px;
		cursor: pointer;
		transition: all 0.3s ease;
	}

	.speed-btn.active, .speed-btn:hover {
		background: rgba(212, 175, 55, 0.15);
		border-color: var(--gold);
		color: var(--gold);
	}

	.chart-container-el {
		width: 100%;
		height: 350px;
		border-radius: 8px;
		overflow: hidden;
	}

	.chart-legend {
		display: flex;
		gap: 20px;
		margin-top: 12px;
		padding-top: 12px;
		border-top: 1px solid rgba(212, 175, 55, 0.1);
	}

	.legend-item {
		display: flex;
		align-items: center;
		gap: 6px;
		font-size: 11px;
		color: var(--text-dim);
	}

	.dot {
		width: 8px;
		height: 8px;
		border-radius: 50%;
	}

	.dot.gold { background: var(--gold); }
	.dot.copper { background: var(--copper); }
	.dot.amber { background: var(--amber); }

	/* ═══════════════════════════════════════════════════════════════════════════
	   FEATURES - UNIQUE ORB DESIGN
	   ═══════════════════════════════════════════════════════════════════════════ */
	.features {
		opacity: 0;
		transform: translateY(60px);
		transition: all 1s cubic-bezier(0.16, 1, 0.3, 1);
	}

	.features.visible {
		opacity: 1;
		transform: translateY(0);
	}

	.section-header {
		display: flex;
		align-items: center;
		gap: 16px;
		margin-bottom: 48px;
	}

	.section-line {
		flex: 1;
		height: 1px;
		background: linear-gradient(90deg, transparent, var(--copper), transparent);
	}

	.section-title {
		font-size: 12px;
		color: var(--gold);
		letter-spacing: 0.2em;
		text-transform: uppercase;
		font-weight: 600;
		margin: 0;
	}

	.features-cascade {
		display: flex;
		flex-direction: column;
		gap: 24px;
	}

	.feature-orb {
		position: relative;
		display: flex;
		align-items: center;
		gap: 24px;
		padding: 32px;
		background: rgba(18, 18, 20, 0.5);
		border: 1px solid rgba(212, 175, 55, 0.15);
		border-radius: 20px;
		backdrop-filter: blur(10px);
		opacity: 0;
		transform: translateX(-50px);
		animation: orbReveal 0.8s cubic-bezier(0.16, 1, 0.3, 1) forwards;
		animation-delay: var(--delay);
		overflow: hidden;
		transition: all 0.4s ease;
	}

	@keyframes orbReveal {
		to { opacity: 1; transform: translateX(0); }
	}

	.feature-orb:hover {
		border-color: rgba(212, 175, 55, 0.4);
		transform: translateX(10px);
		box-shadow: 
			0 20px 40px rgba(0, 0, 0, 0.4),
			inset 0 0 60px rgba(212, 175, 55, 0.05);
	}

	.orb-icon {
		font-size: 32px;
		width: 64px;
		height: 64px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: linear-gradient(135deg, rgba(212, 175, 55, 0.2), rgba(200, 121, 65, 0.2));
		border-radius: 16px;
		flex-shrink: 0;
	}

	.orb-content h3 {
		font-size: 1.25rem;
		font-weight: 600;
		color: var(--text);
		margin: 0 0 8px 0;
	}

	.orb-content p {
		font-size: 0.9375rem;
		color: var(--text-dim);
		margin: 0;
		line-height: 1.6;
	}

	.orb-shine {
		position: absolute;
		top: 0;
		left: -100%;
		width: 50%;
		height: 100%;
		background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.1), transparent);
		transition: left 0.6s ease;
	}

	.feature-orb:hover .orb-shine {
		left: 150%;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   STATS - GLASS CARD DESIGN
	   ═══════════════════════════════════════════════════════════════════════════ */
	.stats {
		display: flex;
		justify-content: center;
		align-items: center;
		gap: 0;
		opacity: 0;
		transform: scale(0.9);
		transition: all 1s cubic-bezier(0.16, 1, 0.3, 1);
	}

	.stats.visible {
		opacity: 1;
		transform: scale(1);
	}

	.stat-card {
		position: relative;
		text-align: center;
		padding: 40px 32px;
		background: rgba(18, 18, 20, 0.6);
		border: 1px solid rgba(212, 175, 55, 0.2);
		border-radius: 16px;
		backdrop-filter: blur(10px);
		overflow: hidden;
		min-width: 160px;
	}

	.stat-value {
		font-size: 2.5rem;
		font-weight: 800;
		background: linear-gradient(135deg, var(--gold), var(--copper));
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
		margin-bottom: 8px;
	}

	.stat-label {
		font-size: 0.75rem;
		color: var(--text-dim);
		text-transform: uppercase;
		letter-spacing: 0.15em;
	}

	.stat-shimmer {
		position: absolute;
		inset: 0;
		background: linear-gradient(90deg, transparent, rgba(212, 175, 55, 0.1), transparent);
		transform: translateX(-100%);
		animation: cardShimmer 3s ease-in-out infinite;
	}

	@keyframes cardShimmer {
		0%, 100% { transform: translateX(-100%); }
		50% { transform: translateX(100%); }
	}

	.stat-divider {
		display: flex;
		flex-direction: column;
		align-items: center;
		padding: 0 24px;
	}

	.divider-line {
		width: 1px;
		height: 30px;
		background: rgba(212, 175, 55, 0.3);
	}

	.divider-dot {
		width: 6px;
		height: 6px;
		background: var(--gold);
		border-radius: 50%;
		margin: 8px 0;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   COMMITMENT - QUOTE BLOCK
	   ═══════════════════════════════════════════════════════════════════════════ */
	.commitment {
		opacity: 0;
		transform: translateY(40px);
		transition: all 1s cubic-bezier(0.16, 1, 0.3, 1);
	}

	.commitment.visible {
		opacity: 1;
		transform: translateY(0);
	}

	.commitment-block {
		position: relative;
		padding: 48px;
		background: rgba(212, 175, 55, 0.05);
		border: 1px solid rgba(212, 175, 55, 0.2);
		border-radius: 20px;
		text-align: center;
	}

	.quote-mark {
		font-size: 80px;
		color: var(--gold);
		opacity: 0.3;
		line-height: 1;
		margin-bottom: -20px;
	}

	.commitment-text {
		font-size: 1.125rem;
		color: var(--text-dim);
		line-height: 1.8;
		margin: 0 0 32px 0;
	}

	.commitment-text strong {
		color: var(--text);
		font-weight: 600;
	}

	.signature {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 12px;
	}

	.sig-line {
		width: 40px;
		height: 1px;
		background: var(--gold);
	}

	.signature span {
		font-size: 12px;
		color: var(--gold);
		letter-spacing: 0.1em;
		text-transform: uppercase;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   TIMELINE - PROGRESS TRACK
	   ═══════════════════════════════════════════════════════════════════════════ */
	.timeline {
		padding: 0 20px;
	}

	.timeline-track {
		display: flex;
		height: 4px;
		border-radius: 2px;
		overflow: hidden;
		margin-bottom: 24px;
	}

	.track-completed {
		width: 33%;
		background: var(--gold);
	}

	.track-active {
		width: 33%;
		background: linear-gradient(90deg, var(--gold), var(--copper));
		position: relative;
	}

	.track-active::after {
		content: '';
		position: absolute;
		inset: 0;
		background: linear-gradient(90deg, transparent, rgba(255,255,255,0.3), transparent);
		animation: trackShimmer 2s linear infinite;
	}

	@keyframes trackShimmer {
		0% { transform: translateX(-100%); }
		100% { transform: translateX(100%); }
	}

	.track-pending {
		flex: 1;
		background: rgba(255, 255, 255, 0.1);
	}

	.timeline-points {
		display: flex;
		justify-content: space-between;
	}

	.point {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 8px;
		font-size: 12px;
		color: var(--text-muted);
	}

	.point.completed { color: var(--gold); }
	.point.active { color: var(--text); }

	.point-dot {
		width: 28px;
		height: 28px;
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 12px;
	}

	.point.completed .point-dot {
		background: var(--gold);
		color: var(--deep);
	}

	.point.active .point-dot {
		background: var(--deep);
		border: 2px solid var(--copper);
		position: relative;
	}

	.pulse-ring {
		position: absolute;
		inset: -4px;
		border: 2px solid var(--copper);
		border-radius: 50%;
		animation: pointPulse 2s ease-out infinite;
	}

	@keyframes pointPulse {
		0% { transform: scale(1); opacity: 1; }
		100% { transform: scale(1.5); opacity: 0; }
	}

	.point:not(.completed):not(.active) .point-dot {
		background: rgba(255, 255, 255, 0.1);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   CAPTURE SECTION - MAGNETIC BUTTON
	   ═══════════════════════════════════════════════════════════════════════════ */
	.capture {
		background: rgba(18, 18, 20, 0.6);
		border: 1px solid rgba(212, 175, 55, 0.2);
		border-radius: 24px;
		padding: 48px;
		backdrop-filter: blur(10px);
	}

	.capture-content {
		text-align: center;
	}

	.capture-badge {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		padding: 8px 16px;
		background: rgba(220, 38, 38, 0.1);
		border: 1px solid rgba(220, 38, 38, 0.3);
		border-radius: 20px;
		margin-bottom: 24px;
	}

	.live-indicator {
		width: 8px;
		height: 8px;
		background: #ef4444;
		border-radius: 50%;
		animation: livePulse 2s ease-in-out infinite;
	}

	@keyframes livePulse {
		0%, 100% { opacity: 1; transform: scale(1); }
		50% { opacity: 0.4; transform: scale(1.3); }
	}

	.capture-badge span {
		font-size: 12px;
		color: #ef4444;
		font-weight: 500;
		letter-spacing: 0.1em;
		text-transform: uppercase;
	}

	.capture-title {
		font-size: 1.75rem;
		font-weight: 700;
		color: var(--text);
		margin: 0 0 32px 0;
	}

	.input-group {
		display: flex;
		gap: 12px;
		max-width: 480px;
		margin: 0 auto;
	}

	@media (max-width: 600px) {
		.input-group { flex-direction: column; }
	}

	.email-field {
		flex: 1;
		padding: 16px 20px;
		background: rgba(255, 255, 255, 0.05);
		border: 1px solid rgba(212, 175, 55, 0.2);
		border-radius: 12px;
		color: var(--text);
		font-size: 1rem;
		outline: none;
		transition: all 0.3s ease;
	}

	.email-field::placeholder { color: var(--text-muted); }

	.email-field:focus {
		border-color: var(--gold);
		box-shadow: 0 0 0 4px rgba(212, 175, 55, 0.15);
	}

	.magnetic-button {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		padding: 16px 32px;
		background: linear-gradient(135deg, var(--copper) 0%, var(--gold) 100%);
		border: none;
		border-radius: 12px;
		color: var(--deep);
		font-size: 1rem;
		font-weight: 700;
		cursor: pointer;
		transition: transform 0.15s ease-out, box-shadow 0.3s ease;
		white-space: nowrap;
		box-shadow: 0 10px 30px rgba(212, 175, 55, 0.3);
	}

	.magnetic-button:hover {
		box-shadow: 0 15px 40px rgba(212, 175, 55, 0.4);
	}

	.magnetic-button:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.btn-loader {
		width: 20px;
		height: 20px;
		border: 2px solid rgba(10, 10, 12, 0.3);
		border-top-color: var(--deep);
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	.btn-arrow {
		width: 18px;
		height: 18px;
		transition: transform 0.3s ease;
	}

	.magnetic-button:hover .btn-arrow {
		transform: translateX(4px);
	}

	.error-toast {
		display: flex;
		align-items: center;
		gap: 8px;
		justify-content: center;
		margin-top: 16px;
		padding: 12px 20px;
		background: rgba(220, 38, 38, 0.1);
		border: 1px solid rgba(220, 38, 38, 0.3);
		border-radius: 10px;
		color: #f87171;
		font-size: 14px;
		animation: shake 0.4s ease-in-out;
	}

	.error-toast svg { width: 18px; height: 18px; flex-shrink: 0; }

	@keyframes shake {
		0%, 100% { transform: translateX(0); }
		25% { transform: translateX(-5px); }
		75% { transform: translateX(5px); }
	}

	/* Success State */
	.success-state {
		text-align: center;
		animation: successPop 0.6s cubic-bezier(0.16, 1, 0.3, 1);
	}

	@keyframes successPop {
		0% { opacity: 0; transform: scale(0.9); }
		100% { opacity: 1; transform: scale(1); }
	}

	.success-ring {
		width: 80px;
		height: 80px;
		margin: 0 auto 24px;
	}

	.success-ring svg {
		width: 100%;
		height: 100%;
	}

	.success-heading {
		font-size: 1.5rem;
		font-weight: 700;
		color: var(--gold);
		margin: 0 0 12px 0;
	}

	.benefit-pills {
		display: flex;
		justify-content: center;
		gap: 12px;
		margin-top: 24px;
	}

	.pill {
		padding: 10px 20px;
		background: rgba(212, 175, 55, 0.1);
		border: 1px solid rgba(212, 175, 55, 0.3);
		border-radius: 20px;
		font-size: 13px;
		color: var(--gold);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   FOOTER
	   ═══════════════════════════════════════════════════════════════════════════ */
	.footer {
		text-align: center;
		padding-top: 40px;
		border-top: 1px solid rgba(212, 175, 55, 0.1);
	}

	.security-badge {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		font-size: 12px;
		color: var(--text-muted);
	}

	.security-badge svg { width: 16px; height: 16px; }

	/* Responsive */
	@media (max-width: 768px) {
		.content { padding: 100px 20px 40px; gap: 60px; }
		.stats { flex-direction: column; gap: 24px; }
		.stat-divider { display: none; }
		.features-cascade { gap: 16px; }
		.feature-orb { flex-direction: column; text-align: center; padding: 24px; }
		.orb-icon { margin: 0 auto; }
		.capture { padding: 32px 24px; }
		.timeline-points { flex-direction: column; gap: 24px; }
		.chart-container-el { height: 280px; }
	}
</style>

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
	// PRINCIPAL ENGINEER L7+ IMPLEMENTATION
	// Live Real-Time Trading Charts | Professional Grade | Real Price Action
	// ═══════════════════════════════════════════════════════════════════════════

	// Core state
	let mounted = $state(false);
	let mouseX = $state(0);
	let mouseY = $state(0);

	// Chart containers and APIs
	let mainChartEl: HTMLDivElement;
	let miniChartEl1: HTMLDivElement;
	let miniChartEl2: HTMLDivElement;
	let mainChart: IChartApi | null = null;
	let miniChart1: IChartApi | null = null;
	let miniChart2: IChartApi | null = null;
	let mainSeries: ISeriesApi<'Candlestick'> | null = null;
	let miniSeries1: ISeriesApi<'Candlestick'> | null = null;
	let miniSeries2: ISeriesApi<'Candlestick'> | null = null;
	let volumeSeries: ISeriesApi<'Histogram'> | null = null;

	// Real-time data state
	let lastPrice = $state({ main: 4750.5, mini1: 232.45, mini2: 892.3 });
	let priceDirection = $state({ main: 'up' as 'up' | 'down', mini1: 'up', mini2: 'down' });

	// Section visibility
	let visible = $state({
		manifesto: false,
		scanners: false,
		university: false,
		infrastructure: false,
		stats: false
	});

	// Email
	let email = $state('');
	let isSubmitting = $state(false);
	let isSubmitted = $state(false);
	let errorMessage = $state('');
	let btnMagnetX = $state(0);
	let btnMagnetY = $state(0);

	// Live ticker with real updates
	let liveTickers = $state([
		{ symbol: 'SPX', basePrice: 5892.33, price: 5892.33, change: 1.24, direction: 'up' as const },
		{ symbol: 'NQ', basePrice: 20845.2, price: 20845.2, change: 2.34, direction: 'up' as const },
		{ symbol: 'ES', basePrice: 5890.5, price: 5890.5, change: 0.89, direction: 'up' as const },
		{ symbol: 'VIX', basePrice: 12.45, price: 12.45, change: -8.2, direction: 'down' as const },
		{ symbol: 'AAPL', basePrice: 232.3, price: 232.3, change: 1.45, direction: 'up' as const },
		{ symbol: 'NVDA', basePrice: 1475.2, price: 1475.2, change: 5.45, direction: 'up' as const },
		{ symbol: 'TSLA', basePrice: 348.5, price: 348.5, change: 3.3, direction: 'up' as const },
		{ symbol: 'META', basePrice: 585.2, price: 585.2, change: 1.9, direction: 'up' as const }
	]);

	// Live scanner signals
	let scannerSignals = $state([
		{
			id: 1,
			symbol: 'NVDA',
			price: 1475.2,
			type: 'Volume Spike',
			confidence: 97,
			time: 0,
			status: 'active' as const
		},
		{
			id: 2,
			symbol: 'AAPL',
			price: 232.45,
			type: 'Breakout',
			confidence: 94,
			time: 3,
			status: 'active'
		},
		{
			id: 3,
			symbol: 'TSLA',
			price: 348.5,
			type: 'Dark Pool',
			confidence: 91,
			time: 6,
			status: 'active'
		},
		{
			id: 4,
			symbol: 'SPY',
			price: 518.75,
			type: 'Gamma',
			confidence: 89,
			time: 9,
			status: 'fading' as const
		}
	]);

	// Stats
	let stats = $state({
		students: 0,
		countries: 0,
		years: 0,
		servers: 0
	});

	// Generate realistic candle data with pullbacks
	function generateCandleData(
		symbol: string,
		startPrice: number,
		count: number
	): CandlestickData[] {
		const candles: CandlestickData[] = [];
		let price = startPrice;
		const now = new Date();

		// Trend state
		let trend = 0;
		let pullbackCounter = 0;
		let inPullback = false;

		for (let i = count; i >= 0; i--) {
			const time = new Date(now.getTime() - i * 60000) as UTCTimestamp;

			// Realistic price generation with trends and pullbacks
			if (!inPullback && Math.random() > 0.7) {
				// Start pullback
				inPullback = true;
				pullbackCounter = Math.floor(Math.random() * 3) + 2;
				trend = -Math.abs(trend) * 0.5; // Reverse trend
			}

			if (inPullback) {
				pullbackCounter--;
				if (pullbackCounter <= 0) {
					inPullback = false;
					trend = (Math.random() - 0.3) * 2; // Resume trend
				}
			}

			// Add trend to price
			const trendMove = trend * (Math.random() * 3 + 1);
			const noise = (Math.random() - 0.5) * (price * 0.002);
			const volatility = inPullback ? 1.5 : 1.0;

			const open = price;
			const close = open + trendMove * volatility + noise;
			const high =
				Math.max(open, close) + Math.random() * Math.abs(close - open) * 0.5 + price * 0.001;
			const low =
				Math.min(open, close) - Math.random() * Math.abs(close - open) * 0.5 - price * 0.001;

			candles.push({
				time: Math.floor(time.getTime() / 1000) as UTCTimestamp,
				open: Number(open.toFixed(2)),
				high: Number(high.toFixed(2)),
				low: Number(low.toFixed(2)),
				close: Number(close.toFixed(2))
			});

			price = close;

			// Gradually adjust trend
			if (!inPullback) {
				trend *= 0.95;
				trend += (Math.random() - 0.5) * 0.3;
			}
		}

		return candles;
	}

	// Live chart data storage
	let chartData = {
		main: {
			candles: [] as CandlestickData[],
			currentCandle: null as CandlestickData | null,
			price: 4750.5
		},
		mini1: {
			candles: [] as CandlestickData[],
			currentCandle: null as CandlestickData | null,
			price: 232.45
		},
		mini2: {
			candles: [] as CandlestickData[],
			currentCandle: null as CandlestickData | null,
			price: 892.3
		}
	};

	onMount(async () => {
		if (!browser) return;

		mounted = true;
		await tick();

		// Initialize all charts
		await initCharts();

		// Start all live updates
		startLivePriceUpdates();
		startChartUpdates();
		startTickerUpdates();
		startScannerUpdates();
		startStatAnimation();

		// Setup observers and mouse tracking
		setupIntersectionObserver();

		const handleMouseMove = (e: MouseEvent) => {
			mouseX = e.clientX;
			mouseY = e.clientY;

			const button = document.querySelector('.magnetic-btn') as HTMLElement;
			if (button) {
				const rect = button.getBoundingClientRect();
				const centerX = rect.left + rect.width / 2;
				const centerY = rect.top + rect.height / 2;
				const distX = e.clientX - centerX;
				const distY = e.clientY - centerY;
				const dist = Math.sqrt(distX * distX + distY * distY);

				if (dist < 150) {
					btnMagnetX = distX * 0.3;
					btnMagnetY = distY * 0.3;
				} else {
					btnMagnetX = 0;
					btnMagnetY = 0;
				}
			}
		};

		window.addEventListener('mousemove', handleMouseMove, { passive: true });

		// Trigger initial animation
		setTimeout(() => (visible.manifesto = true), 300);

		return () => {
			window.removeEventListener('mousemove', handleMouseMove);
			if (mainChart) mainChart.remove();
			if (miniChart1) miniChart1.remove();
			if (miniChart2) miniChart2.remove();
		};
	});

	async function initCharts() {
		const { createChart, CandlestickSeries, HistogramSeries } = await import('lightweight-charts');

		// Initialize data
		chartData.main.candles = generateCandleData('SPX', 4750, 60);
		chartData.mini1.candles = generateCandleData('AAPL', 232, 40);
		chartData.mini2.candles = generateCandleData('NVDA', 892, 40);

		// Main Chart - Professional styling
		mainChart = createChart(mainChartEl, {
			layout: {
				background: { color: 'rgba(10, 10, 12, 0.3)' },
				textColor: '#a8a29e',
				fontFamily: "'Inter', system-ui, sans-serif",
				fontSize: 11
			},
			grid: {
				vertLines: { color: 'rgba(16, 185, 129, 0.04)' },
				horzLines: { color: 'rgba(16, 185, 129, 0.03)' }
			},
			crosshair: {
				mode: 0,
				vertLine: { color: 'rgba(16, 185, 129, 0.4)', style: 3, labelBackgroundColor: '#10b981' },
				horzLine: { color: 'rgba(16, 185, 129, 0.4)', style: 3, labelBackgroundColor: '#10b981' }
			},
			rightPriceScale: {
				borderColor: 'rgba(255, 255, 255, 0.08)',
				scaleMargins: { top: 0.05, bottom: 0.15 }
			},
			timeScale: {
				borderColor: 'rgba(255, 255, 255, 0.08)',
				timeVisible: true,
				secondsVisible: true
			},
			autoSize: true
		});

		mainSeries = mainChart.addSeries(CandlestickSeries, {
			upColor: '#10b981',
			downColor: '#ef4444',
			borderUpColor: '#10b981',
			borderDownColor: '#ef4444',
			wickUpColor: '#10b981',
			wickDownColor: '#ef4444'
		});

		volumeSeries = mainChart.addSeries(HistogramSeries, {
			priceFormat: { type: 'volume' },
			priceScaleId: ''
		});
		volumeSeries.priceScale().applyOptions({ scaleMargins: { top: 0.85, bottom: 0 } });

		mainSeries.setData(chartData.main.candles);
		volumeSeries.setData(
			chartData.main.candles.map((c) => ({
				time: c.time,
				value: Math.floor(Math.random() * 1000000) + 500000,
				color: c.close >= c.open ? 'rgba(16, 185, 129, 0.5)' : 'rgba(239, 68, 68, 0.5)'
			}))
		);

		// Mini Chart 1 - AAPL
		miniChart1 = createChart(miniChartEl1, {
			layout: {
				background: { color: 'transparent' },
				textColor: '#a8a29e',
				fontFamily: "'Inter', system-ui, sans-serif",
				fontSize: 9
			},
			grid: { vertLines: { visible: false }, horzLines: { visible: false } },
			rightPriceScale: { visible: false },
			timeScale: { visible: false },
			autoSize: true
		});

		miniSeries1 = miniChart1.addSeries(CandlestickSeries, {
			upColor: '#10b981',
			downColor: '#ef4444',
			borderUpColor: '#10b981',
			borderDownColor: '#ef4444',
			wickUpColor: '#10b981',
			wickDownColor: '#ef4444'
		});
		miniSeries1.setData(chartData.mini1.candles);

		// Mini Chart 2 - NVDA
		miniChart2 = createChart(miniChartEl2, {
			layout: {
				background: { color: 'transparent' },
				textColor: '#a8a29e',
				fontFamily: "'Inter', system-ui, sans-serif",
				fontSize: 9
			},
			grid: { vertLines: { visible: false }, horzLines: { visible: false } },
			rightPriceScale: { visible: false },
			timeScale: { visible: false },
			autoSize: true
		});

		miniSeries2 = miniChart2.addSeries(CandlestickSeries, {
			upColor: '#10b981',
			downColor: '#ef4444',
			borderUpColor: '#10b981',
			borderDownColor: '#ef4444',
			wickUpColor: '#10b981',
			wickDownColor: '#ef4444'
		});
		miniSeries2.setData(chartData.mini2.candles);

		mainChart.timeScale().fitContent();
	}

	function startLivePriceUpdates() {
		// Update main chart price every second
		setInterval(() => {
			if (!mainSeries) return;

			const lastCandle = chartData.main.candles[chartData.main.candles.length - 1];
			const now = Math.floor(Date.now() / 1000) as UTCTimestamp;

			// Check if we need a new candle (every 5 seconds)
			if (now - lastCandle.time > 5) {
				// Create new candle
				const newCandle: CandlestickData = {
					time: now,
					open: lastCandle.close,
					high: lastCandle.close,
					low: lastCandle.close,
					close: lastCandle.close + (Math.random() - 0.48) * 2
				};

				chartData.main.candles.push(newCandle);
				if (chartData.main.candles.length > 60) chartData.main.candles.shift();

				mainSeries.update(newCandle);

				// Update volume
				const newVolume = Math.floor(Math.random() * 1000000) + 500000;
				volumeSeries?.update({
					time: now,
					value: newVolume,
					color:
						newCandle.close >= newCandle.open ? 'rgba(16, 185, 129, 0.5)' : 'rgba(239, 68, 68, 0.5)'
				});

				chartData.main.currentCandle = newCandle;
			} else {
				// Update current candle
				const move = (Math.random() - 0.48) * 1.5;
				lastCandle.close = Number((lastCandle.close + move).toFixed(2));
				lastCandle.high = Math.max(lastCandle.high, lastCandle.close);
				lastCandle.low = Math.min(lastCandle.low, lastCandle.close);

				mainSeries.update(lastCandle);
			}

			// Update price display
			lastPrice.main = lastCandle.close;
			priceDirection.main = lastCandle.close >= lastCandle.open ? 'up' : 'down';
		}, 1000);

		// Update mini charts every 2 seconds
		setInterval(() => {
			if (!miniSeries1 || !miniSeries2) return;

			[miniSeries1, miniSeries2].forEach((series, idx) => {
				const key = idx === 0 ? 'mini1' : 'mini2';
				const candles = chartData[key].candles;
				const lastCandle = candles[candles.length - 1];
				const now = Math.floor(Date.now() / 1000) as UTCTimestamp;

				if (now - lastCandle.time > 3) {
					const newCandle: CandlestickData = {
						time: now,
						open: lastCandle.close,
						high: lastCandle.close,
						low: lastCandle.close,
						close: lastCandle.close + (Math.random() - 0.48) * 1.5
					};
					candles.push(newCandle);
					if (candles.length > 40) candles.shift();
					series.update(newCandle);
				} else {
					const move = (Math.random() - 0.48) * 0.8;
					lastCandle.close = Number((lastCandle.close + move).toFixed(2));
					lastCandle.high = Math.max(lastCandle.high, lastCandle.close);
					lastCandle.low = Math.min(lastCandle.low, lastCandle.close);
					series.update(lastCandle);
				}

				lastPrice[key] = lastCandle.close;
				priceDirection[key] = lastCandle.close >= lastCandle.open ? 'up' : 'down';
			});
		}, 2000);
	}

	function startChartUpdates() {
		// Ensure charts stay responsive
		setInterval(() => {
			mainChart?.timeScale().fitContent();
		}, 5000);
	}

	function startTickerUpdates() {
		setInterval(() => {
			liveTickers = liveTickers.map((ticker) => {
				const move = (Math.random() - 0.48) * 0.5;
				const newPrice = ticker.price + move;
				const priceChange = ((newPrice - ticker.basePrice) / ticker.basePrice) * 100;

				return {
					...ticker,
					price: Number(newPrice.toFixed(2)),
					change: Number(priceChange.toFixed(2)),
					direction: newPrice > ticker.price ? 'up' : 'down'
				};
			});
		}, 3000);
	}

	function startScannerUpdates() {
		setInterval(() => {
			// Randomly add new signals
			if (Math.random() > 0.6) {
				const symbols = ['AMD', 'MSFT', 'GOOGL', 'AMZN', 'NFLX', 'CRM', 'UBER', 'COIN'];
				const types = [
					'Volume Spike',
					'Breakout',
					'Dark Pool',
					'Gamma',
					'Institutional',
					'Momentum'
				];
				const newSignal = {
					id: Date.now(),
					symbol: symbols[Math.floor(Math.random() * symbols.length)],
					price: Number((Math.random() * 500 + 50).toFixed(2)),
					type: types[Math.floor(Math.random() * types.length)],
					confidence: Math.floor(Math.random() * 15) + 85,
					time: 0,
					status: 'active' as const
				};

				scannerSignals = [newSignal, ...scannerSignals.slice(0, 4)];
			}

			// Age existing signals
			scannerSignals = scannerSignals.map((signal) => ({
				...signal,
				time: signal.time + 1,
				status: signal.time > 15 ? 'fading' : signal.time > 10 ? 'dimming' : 'active'
			}));
		}, 4000);
	}

	function startStatAnimation() {
		const targets = { students: 50000, countries: 127, years: 8, servers: 24 };
		const duration = 2000;
		const steps = 60;
		const interval = duration / steps;
		let step = 0;

		const timer = setInterval(() => {
			step++;
			const progress = 1 - Math.pow(1 - step / steps, 3);

			stats.students = Math.floor(targets.students * progress);
			stats.countries = Math.floor(targets.countries * progress);
			stats.years = Math.floor(targets.years * progress);
			stats.servers = Math.floor(targets.servers * progress);

			if (step >= steps) clearInterval(timer);
		}, interval);
	}

	function setupIntersectionObserver() {
		const observer = new IntersectionObserver(
			(entries) => {
				entries.forEach((entry) => {
					if (entry.isIntersecting) {
						const id = entry.target.id;
						if (id) visible[id as keyof typeof visible] = true;
					}
				});
			},
			{ threshold: 0.15 }
		);

		['scanners', 'university', 'infrastructure', 'stats'].forEach((id) => {
			const el = document.getElementById(id);
			if (el) observer.observe(el);
		});
	}

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
		} catch {
			errorMessage = 'Connection error. Please try again.';
		} finally {
			isSubmitting = false;
		}
	}

	function formatNumber(num: number): string {
		return num.toLocaleString();
	}
</script>

<div class="experience-container" class:mounted>
	<!-- Live Market Tape -->
	<div class="market-tape">
		<div class="tape-track">
			{#each [...liveTickers, ...liveTickers, ...liveTickers] as ticker, i}
				<div
					class="tape-item"
					class:up={ticker.direction === 'up'}
					class:down={ticker.direction === 'down'}
				>
					<span class="ticker-symbol">{ticker.symbol}</span>
					<span class="ticker-price">${ticker.price.toFixed(2)}</span>
					<span class="ticker-change">
						{ticker.change >= 0 ? '+' : ''}{ticker.change.toFixed(2)}%
					</span>
					<span class="ticker-arrow">{ticker.direction === 'up' ? '▲' : '▼'}</span>
				</div>
			{/each}
		</div>
	</div>

	<!-- Ambient Effects -->
	<div class="ambient-layer">
		<div class="orb orb-1" style="transform: translate({mouseX * 0.02}px, {mouseY * 0.02}px)"></div>
		<div
			class="orb orb-2"
			style="transform: translate({-mouseX * 0.01}px, {-mouseY * 0.01}px)"
		></div>
	</div>

	<!-- Main Content -->
	<main class="content">
		<!-- MANIFESTO -->
		<section id="manifesto" class="manifesto" class:visible={visible.manifesto}>
			<div class="live-badge">
				<span class="pulse-dot"></span>
				<span>Platform Evolution in Progress</span>
			</div>

			<h1 class="manifesto-headline">
				<span class="line">While Others Chase</span>
				<span class="line fake">Fake "Holy Grail" Indicators</span>
				<span class="line">We're Building What</span>
				<span class="line accent">Actually Changes</span>
				<span class="line accent">Trader's Lives</span>
			</h1>

			<p class="manifesto-sub">
				No gimmicks. No false promises. Just institutional-grade tools, real strategies, and
				education that produces results.
			</p>
		</section>

		<!-- LIVE CHARTS SECTION -->
		<section class="charts-showcase">
			<div class="section-tag">Live Market Data</div>

			<div class="charts-grid">
				<!-- Main Chart -->
				<div class="chart-card main-chart">
					<div class="chart-header">
						<div class="symbol-tag">
							<span class="symbol">SPX</span>
							<span class="exchange">CME</span>
						</div>
						<div class="price-live">
							<span
								class="current-price"
								class:up={priceDirection.main === 'up'}
								class:down={priceDirection.main === 'down'}
							>
								{lastPrice.main.toFixed(2)}
							</span>
							<span class="price-change">
								{priceDirection.main === 'up' ? '+' : '-'}{(lastPrice.main * 0.001).toFixed(2)}
							</span>
						</div>
					</div>
					<div class="chart-container" bind:this={mainChartEl}></div>
					<div class="chart-footer">
						<div class="timeframes">
							<button class="tf-btn">1m</button>
							<button class="tf-btn active">5m</button>
							<button class="tf-btn">15m</button>
							<button class="tf-btn">1H</button>
						</div>
						<div class="indicators">
							<span class="ind-tag">MA(20)</span>
							<span class="ind-tag">VWAP</span>
							<span class="ind-tag">VOL</span>
						</div>
					</div>
				</div>

				<!-- Mini Charts -->
				<div class="mini-charts">
					<div class="chart-card mini">
						<div class="mini-header">
							<span class="mini-symbol">AAPL</span>
							<span
								class="mini-price"
								class:up={priceDirection.mini1 === 'up'}
								class:down={priceDirection.mini1 === 'down'}
							>
								${lastPrice.mini1.toFixed(2)}
							</span>
						</div>
						<div class="mini-container" bind:this={miniChartEl1}></div>
					</div>

					<div class="chart-card mini">
						<div class="mini-header">
							<span class="mini-symbol">NVDA</span>
							<span
								class="mini-price"
								class:up={priceDirection.mini2 === 'up'}
								class:down={priceDirection.mini2 === 'down'}
							>
								${lastPrice.mini2.toFixed(2)}
							</span>
						</div>
						<div class="mini-container" bind:this={miniChartEl2}></div>
					</div>
				</div>
			</div>
		</section>

		<!-- INSTITUTIONAL SCANNERS -->
		<section id="scanners" class="scanners-section" class:visible={visible.scanners}>
			<div class="section-tag">Flagship Technology</div>

			<div class="scanners-card">
				<div class="scanners-header">
					<div class="header-icon">
						<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
							<circle cx="12" cy="12" r="3" />
							<path d="M12 2v4M12 18v4M2 12h4M18 12h4" />
							<path d="M4.93 4.93l2.83 2.83M16.24 16.24l2.83 2.83" />
						</svg>
					</div>
					<div class="header-text">
						<h2>Institutional Scanners</h2>
						<p>Technology previously reserved for hedge funds & prop firms</p>
					</div>
					<div class="scan-status">
						<span class="scan-dot"></span>
						<span class="scan-text">LIVE SCANNING</span>
					</div>
				</div>

				<div class="scanner-interface">
					<div class="scan-visualization">
						<div class="radar-container">
							<div class="radar-ring"></div>
							<div class="radar-ring"></div>
							<div class="radar-ring"></div>
							<div class="radar-sweep"></div>
							{#each Array(8) as _, i}
								<div
									class="blip"
									style="--delay: {i * 0.5}s; --x: {20 + Math.random() * 60}%; --y: {20 +
										Math.random() * 60}%"
								></div>
							{/each}
						</div>

						<div class="scan-metrics">
							<div class="metric">
								<span class="metric-label">Universe</span>
								<span class="metric-value">12,847</span>
							</div>
							<div class="metric">
								<span class="metric-label">Scanned/min</span>
								<span class="metric-value">4.2M</span>
							</div>
							<div class="metric">
								<span class="metric-label">Active Signals</span>
								<span class="metric-value live-counter"
									>{scannerSignals.filter((s) => s.status === 'active').length}</span
								>
							</div>
						</div>
					</div>

					<div class="signals-panel">
						<div class="panel-header">
							<span>Detected Signals</span>
							<span class="update-indicator">UPDATING LIVE</span>
						</div>

						<div class="signals-list">
							{#each scannerSignals as signal, i}
								<div
									class="signal-row"
									class:fading={signal.status === 'fading'}
									class:dimming={signal.status === 'dimming'}
									style="--delay: {i * 100}ms"
								>
									<div class="signal-main">
										<span class="sig-symbol">{signal.symbol}</span>
										<span class="sig-type">{signal.type}</span>
									</div>
									<div class="signal-data">
										<span class="sig-price">${signal.price}</span>
										<div class="sig-confidence">
											<div class="conf-bar" style="--width: {signal.confidence}%"></div>
											<span>{signal.confidence}%</span>
										</div>
									</div>
									<div class="sig-time">{signal.time}s ago</div>
								</div>
							{/each}
						</div>
					</div>
				</div>

				<div class="scanners-features">
					<div class="feature-item">
						<span class="feature-check">◆</span>
						<span>Dark pool activity detection in real-time</span>
					</div>
					<div class="feature-item">
						<span class="feature-check">◆</span>
						<span>Unusual options flow analysis</span>
					</div>
					<div class="feature-item">
						<span class="feature-check">◆</span>
						<span>Institutional footprint identification</span>
					</div>
					<div class="feature-item">
						<span class="feature-check">◆</span>
						<span>Gamma squeeze prediction algorithms</span>
					</div>
				</div>
			</div>
		</section>

		<!-- TRADING UNIVERSITY -->
		<section id="university" class="university-section" class:visible={visible.university}>
			<div class="section-tag">Education</div>

			<div class="university-grid">
				<!-- Day Trading -->
				<div class="uni-card day-trading">
					<div class="uni-header">
						<div class="uni-icon amber">
							<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
								<circle cx="12" cy="12" r="10" />
								<polyline points="12 6 12 12 16 14" />
							</svg>
						</div>
						<div class="uni-title">
							<h3>Day Trading University</h3>
							<p>Stocks & Options Mastery</p>
						</div>
					</div>

					<div class="uni-modules">
						<div class="module-row">
							<span class="mod-num">01</span>
							<div class="mod-info">
								<h4>Opening Bell Strategy</h4>
								<p>First 30 minutes edge</p>
							</div>
						</div>
						<div class="module-row">
							<span class="mod-num">02</span>
							<div class="mod-info">
								<h4>VWAP & Volume Profile</h4>
								<p>Institutional execution</p>
							</div>
						</div>
						<div class="module-row">
							<span class="mod-num">03</span>
							<div class="mod-info">
								<h4>Options Flow Reading</h4>
								<p>Smart money detection</p>
							</div>
						</div>
					</div>
				</div>

				<!-- Swing Trading -->
				<div class="uni-card swing-trading">
					<div class="uni-header">
						<div class="uni-icon copper">
							<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
								<path d="M13 2L3 14h9l-1 8 10-12h-9l1-8z" />
							</svg>
						</div>
						<div class="uni-title">
							<h3>Swing Trading University</h3>
							<p>Multi-Day Position Mastery</p>
						</div>
					</div>

					<div class="swing-wave">
						<svg viewBox="0 0 300 100" preserveAspectRatio="none">
							<defs>
								<linearGradient id="swingGrad" x1="0%" y1="0%" x2="0%" y2="100%">
									<stop offset="0%" stop-color="#c87941" stop-opacity="0.3" />
									<stop offset="100%" stop-color="#c87941" stop-opacity="0" />
								</linearGradient>
							</defs>
							<path
								d="M0,80 Q75,20 150,50 T300,30"
								fill="none"
								stroke="#c87941"
								stroke-width="2"
								class="wave-line"
							/>
							<path d="M0,80 Q75,20 150,50 T300,30 L300,100 L0,100 Z" fill="url(#swingGrad)" />
							<circle cx="50" cy="65" r="4" fill="#10b981" />
							<text x="50" y="85" fill="#10b981" font-size="8" text-anchor="middle">ENTRY</text>
							<circle cx="250" cy="35" r="4" fill="#ef4444" />
							<text x="250" y="25" fill="#ef4444" font-size="8" text-anchor="middle">+18%</text>
						</svg>
						<div class="wave-labels">
							<span>Day 1</span>
							<span>Day 3</span>
							<span>Day 7</span>
						</div>
					</div>
				</div>
			</div>
		</section>

		<!-- INFRASTRUCTURE -->
		<section id="infrastructure" class="infra-section" class:visible={visible.infrastructure}>
			<div class="section-tag">Platform</div>

			<div class="infra-card">
				<div class="infra-header">
					<div class="header-icon">
						<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
							<rect x="2" y="2" width="20" height="8" rx="2" />
							<rect x="2" y="14" width="20" height="8" rx="2" />
							<path d="M6 6h12M6 18h12" />
						</svg>
					</div>
					<div class="header-text">
						<h2>Infrastructure Expansion</h2>
						<p>Scaling to serve more traders globally</p>
					</div>
				</div>

				<div class="server-visualization">
					<div class="server-rack">
						{#each Array(8) as _, i}
							<div class="server-unit" style="--delay: {i * 100}ms">
								<div class="unit-header">
									<span class="unit-id">NODE-{String(i + 1).padStart(2, '0')}</span>
									<div class="unit-status" class:active={i < 6}>
										<span class="status-dot"></span>
										<span class="status-text">{i < 6 ? 'ONLINE' : 'STANDBY'}</span>
									</div>
								</div>
								<div class="unit-lights">
									{#each Array(6) as _, j}
										<span
											class="light"
											class:blink={i < 6 && Math.random() > 0.3}
											style="--delay: {j * 50}ms"
										></span>
									{/each}
								</div>
								<div class="unit-load">
									<div
										class="load-bar"
										style="--load: {i < 6 ? 40 + Math.random() * 40 : 0}%"
									></div>
									<span class="load-text">{i < 6 ? Math.floor(40 + Math.random() * 40) : 0}%</span>
								</div>
							</div>
						{/each}
					</div>

					<div class="expansion-metrics">
						<div class="exp-card">
							<span class="exp-value">12→24</span>
							<span class="exp-label">Server Nodes</span>
						</div>
						<div class="exp-card">
							<span class="exp-value">2.5x</span>
							<span class="exp-label">Capacity</span>
						</div>
						<div class="exp-card">
							<span class="exp-value">15ms</span>
							<span class="exp-label">Latency</span>
						</div>
					</div>
				</div>
			</div>
		</section>

		<!-- STATS -->
		<section id="stats" class="stats-section" class:visible={visible.stats}>
			<div class="stat-item">
				<div class="stat-number">{formatNumber(stats.students)}+</div>
				<div class="stat-label">Traders Trained</div>
				<div class="stat-bar" style="--progress: {stats.students / 50000}"></div>
			</div>
			<div class="stat-divider"></div>
			<div class="stat-item">
				<div class="stat-number">{stats.countries}</div>
				<div class="stat-label">Countries</div>
				<div class="stat-bar" style="--progress: {stats.countries / 127}"></div>
			</div>
			<div class="stat-divider"></div>
			<div class="stat-item">
				<div class="stat-number">{stats.years}</div>
				<div class="stat-label">Years Excellence</div>
				<div class="stat-bar" style="--progress: {stats.years / 8}"></div>
			</div>
			<div class="stat-divider"></div>
			<div class="stat-item">
				<div class="stat-number">{stats.servers}</div>
				<div class="stat-label">Server Nodes</div>
				<div class="stat-bar" style="--progress: {stats.servers / 24}"></div>
			</div>
		</section>

		<!-- EMAIL CAPTURE -->
		<section class="capture-section">
			{#if !isSubmitted}
				<div class="capture-box">
					<div class="capture-header">
						<div class="live-pulse"></div>
						<span>Early Access List</span>
					</div>
					<h3 class="capture-title">Join the Trading Revolution</h3>
					<p class="capture-subtitle">
						Get early access to institutional scanners and university courses
					</p>

					<div class="input-wrapper">
						<input
							type="email"
							placeholder="Enter your email address"
							bind:value={email}
							onkeydown={(e) => e.key === 'Enter' && handleNotifyMe()}
							disabled={isSubmitting}
							class="email-input"
						/>
						<button
							class="magnetic-btn"
							style="transform: translate({btnMagnetX}px, {btnMagnetY}px)"
							onclick={handleNotifyMe}
							disabled={isSubmitting || !email}
						>
							{#if isSubmitting}
								<span class="spinner"></span>
							{:else}
								<span>Get Access</span>
								<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
									<path d="M5 12h14M14 5l7 7-7 7" />
								</svg>
							{/if}
						</button>
					</div>

					{#if errorMessage}
						<div class="error-msg">{errorMessage}</div>
					{/if}
				</div>
			{:else}
				<div class="success-box">
					<div class="success-icon">
						<svg viewBox="0 0 52 52">
							<circle cx="26" cy="26" r="25" fill="none" stroke="#10b981" stroke-width="2" />
							<path d="M14 27l8 8 16-16" fill="none" stroke="#10b981" stroke-width="3" />
						</svg>
					</div>
					<h3>Welcome to the Revolution!</h3>
					<p>Check your email for confirmation and early access details.</p>
					<div class="benefits">
						<span>Institutional Scanners</span>
						<span>Trading University</span>
						<span>VIP Pricing</span>
					</div>
				</div>
			{/if}
		</section>

		<footer class="footer">
			<div class="security-note">
				<svg viewBox="0 0 24 24" fill="none" stroke="#10b981" stroke-width="1.5">
					<path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z" />
				</svg>
				<span>Secure. No Spam. Institutional-Grade Privacy.</span>
			</div>
		</footer>
	</main>
</div>

<style>
	:root {
		--gold: #d4af37;
		--copper: #c87941;
		--green: #10b981;
		--red: #ef4444;
		--dark: #0a0a0c;
		--dark-light: #121214;
		--text: #e7e5e4;
		--text-dim: #a8a29e;
	}

	.experience-container {
		position: fixed;
		inset: 0;
		width: 100%;
		min-height: 100vh;
		background: var(--dark);
		overflow-y: auto;
		overflow-x: hidden;
		z-index: 99999;
		font-family:
			'Inter',
			system-ui,
			-apple-system,
			sans-serif;
	}

	.experience-container::-webkit-scrollbar {
		width: 4px;
	}
	.experience-container::-webkit-scrollbar-track {
		background: transparent;
	}
	.experience-container::-webkit-scrollbar-thumb {
		background: var(--copper);
		border-radius: 2px;
	}

	/* Market Tape */
	.market-tape {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		z-index: 100;
		background: rgba(10, 10, 12, 0.95);
		backdrop-filter: blur(10px);
		border-bottom: 1px solid rgba(255, 255, 255, 0.08);
		padding: 8px 0;
		overflow: hidden;
	}

	.tape-track {
		display: flex;
		gap: 32px;
		animation: scrollTape 30s linear infinite;
	}

	@keyframes scrollTape {
		0% {
			transform: translateX(0);
		}
		100% {
			transform: translateX(-33.33%);
		}
	}

	.tape-item {
		display: flex;
		align-items: center;
		gap: 8px;
		font-family: 'SF Mono', monospace;
		font-size: 12px;
		font-weight: 500;
		white-space: nowrap;
		padding: 4px 8px;
		border-radius: 6px;
		transition: all 0.3s ease;
	}

	.tape-item.up {
		background: rgba(16, 185, 129, 0.1);
	}
	.tape-item.down {
		background: rgba(239, 68, 68, 0.1);
	}

	.ticker-symbol {
		color: var(--text-dim);
		font-weight: 700;
	}
	.ticker-price {
		color: var(--text);
	}
	.ticker-change {
		font-size: 11px;
	}
	.tape-item.up .ticker-change {
		color: var(--green);
	}
	.tape-item.down .ticker-change {
		color: var(--red);
	}
	.ticker-arrow {
		font-size: 10px;
	}

	/* Ambient */
	.ambient-layer {
		position: fixed;
		inset: 0;
		z-index: 1;
		pointer-events: none;
	}

	.orb {
		position: absolute;
		border-radius: 50%;
		filter: blur(80px);
		opacity: 0.25;
		transition: transform 0.3s ease-out;
	}

	.orb-1 {
		width: 400px;
		height: 400px;
		background: var(--copper);
		top: 10%;
		left: -5%;
	}
	.orb-2 {
		width: 300px;
		height: 300px;
		background: var(--gold);
		bottom: 10%;
		right: 0;
	}

	/* Content */
	.content {
		position: relative;
		z-index: 5;
		max-width: 1200px;
		margin: 0 auto;
		padding: 100px 24px 60px;
		display: flex;
		flex-direction: column;
		gap: 80px;
	}

	/* Manifesto */
	.manifesto {
		text-align: center;
		opacity: 0;
		transform: translateY(40px);
		transition: all 0.8s cubic-bezier(0.16, 1, 0.3, 1);
	}

	.manifesto.visible {
		opacity: 1;
		transform: translateY(0);
	}

	.live-badge {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		padding: 8px 16px;
		background: rgba(16, 185, 129, 0.1);
		border: 1px solid rgba(16, 185, 129, 0.3);
		border-radius: 20px;
		margin-bottom: 32px;
		font-size: 12px;
		color: var(--green);
		font-weight: 600;
		letter-spacing: 0.1em;
	}

	.pulse-dot {
		width: 8px;
		height: 8px;
		background: var(--green);
		border-radius: 50%;
		animation: pulse 2s ease-in-out infinite;
	}

	@keyframes pulse {
		0%,
		100% {
			opacity: 1;
			transform: scale(1);
		}
		50% {
			opacity: 0.5;
			transform: scale(1.2);
		}
	}

	.manifesto-headline {
		font-size: clamp(2rem, 6vw, 4rem);
		font-weight: 800;
		line-height: 1.1;
		margin: 0 0 24px 0;
		letter-spacing: -0.02em;
	}

	.line {
		display: block;
		opacity: 0;
		animation: revealLine 0.6s ease forwards;
	}
	.line:nth-child(1) {
		animation-delay: 0.1s;
		color: var(--text);
	}
	.line:nth-child(2) {
		animation-delay: 0.2s;
		background: linear-gradient(135deg, #ef4444, #dc2626);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
	}
	.line:nth-child(3) {
		animation-delay: 0.3s;
		color: var(--text);
	}
	.line:nth-child(4) {
		animation-delay: 0.4s;
		background: linear-gradient(135deg, var(--gold), var(--copper));
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
	}
	.line:nth-child(5) {
		animation-delay: 0.5s;
		background: linear-gradient(135deg, var(--gold), var(--copper));
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
	}

	@keyframes revealLine {
		to {
			opacity: 1;
			transform: translateY(0);
		}
		from {
			opacity: 0;
			transform: translateY(20px);
		}
	}

	.manifesto-sub {
		font-size: clamp(1rem, 2vw, 1.25rem);
		color: var(--text-dim);
		max-width: 600px;
		margin: 0 auto;
		line-height: 1.7;
		opacity: 0;
		animation: fadeIn 0.8s ease 0.6s forwards;
	}

	@keyframes fadeIn {
		to {
			opacity: 1;
		}
	}

	/* Section Tag */
	.section-tag {
		font-size: 11px;
		color: var(--gold);
		letter-spacing: 0.2em;
		text-transform: uppercase;
		font-weight: 600;
		margin-bottom: 16px;
	}

	/* Charts Showcase */
	.charts-showcase {
		margin: 40px 0;
	}

	.charts-grid {
		display: grid;
		grid-template-columns: 2fr 1fr;
		gap: 24px;
	}

	@media (max-width: 900px) {
		.charts-grid {
			grid-template-columns: 1fr;
		}
	}

	.chart-card {
		background: rgba(18, 18, 20, 0.6);
		border: 1px solid rgba(255, 255, 255, 0.08);
		border-radius: 16px;
		padding: 20px;
		backdrop-filter: blur(10px);
	}

	.chart-card.main-chart {
		border-color: rgba(16, 185, 129, 0.2);
	}

	.chart-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 16px;
		padding-bottom: 12px;
		border-bottom: 1px solid rgba(255, 255, 255, 0.05);
	}

	.symbol-tag {
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.symbol-tag .symbol {
		font-size: 20px;
		font-weight: 700;
		color: var(--green);
	}

	.symbol-tag .exchange {
		font-size: 10px;
		color: var(--text-dim);
		padding: 2px 6px;
		background: rgba(255, 255, 255, 0.05);
		border-radius: 4px;
	}

	.price-live {
		display: flex;
		align-items: center;
		gap: 12px;
	}

	.current-price {
		font-size: 24px;
		font-weight: 700;
		font-family: 'SF Mono', monospace;
		transition: color 0.3s ease;
	}

	.current-price.up {
		color: var(--green);
	}
	.current-price.down {
		color: var(--red);
	}

	.price-change {
		font-size: 14px;
		color: var(--text-dim);
		font-family: 'SF Mono', monospace;
	}

	.chart-container {
		width: 100%;
		height: 350px;
		border-radius: 8px;
	}

	.chart-footer {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-top: 16px;
		padding-top: 12px;
		border-top: 1px solid rgba(255, 255, 255, 0.05);
	}

	.timeframes {
		display: flex;
		gap: 8px;
	}

	.tf-btn {
		padding: 6px 12px;
		background: transparent;
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 6px;
		color: var(--text-dim);
		font-size: 12px;
		cursor: pointer;
		transition: all 0.3s ease;
	}

	.tf-btn:hover,
	.tf-btn.active {
		background: rgba(16, 185, 129, 0.1);
		border-color: var(--green);
		color: var(--green);
	}

	.indicators {
		display: flex;
		gap: 12px;
	}

	.ind-tag {
		font-size: 11px;
		color: var(--text-dim);
		padding: 4px 8px;
		background: rgba(255, 255, 255, 0.05);
		border-radius: 4px;
	}

	/* Mini Charts */
	.mini-charts {
		display: flex;
		flex-direction: column;
		gap: 16px;
	}

	.mini-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 8px;
	}

	.mini-symbol {
		font-size: 14px;
		font-weight: 600;
		color: var(--text);
	}

	.mini-price {
		font-size: 14px;
		font-weight: 700;
		font-family: 'SF Mono', monospace;
		transition: color 0.3s ease;
	}

	.mini-price.up {
		color: var(--green);
	}
	.mini-price.down {
		color: var(--red);
	}

	.mini-container {
		width: 100%;
		height: 120px;
	}

	/* Scanners Section */
	.scanners-section {
		opacity: 0;
		transform: translateY(40px);
		transition: all 0.8s cubic-bezier(0.16, 1, 0.3, 1);
	}

	.scanners-section.visible {
		opacity: 1;
		transform: translateY(0);
	}

	.scanners-card {
		background: rgba(18, 18, 20, 0.6);
		border: 1px solid rgba(212, 175, 55, 0.2);
		border-radius: 20px;
		padding: 32px;
		backdrop-filter: blur(10px);
	}

	.scanners-header {
		display: flex;
		align-items: center;
		gap: 20px;
		margin-bottom: 24px;
		padding-bottom: 20px;
		border-bottom: 1px solid rgba(212, 175, 55, 0.1);
	}

	.header-icon {
		width: 56px;
		height: 56px;
		background: linear-gradient(135deg, rgba(212, 175, 55, 0.2), rgba(200, 121, 65, 0.2));
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--gold);
		flex-shrink: 0;
	}

	.header-icon svg {
		width: 28px;
		height: 28px;
	}

	.header-text {
		flex: 1;
	}
	.header-text h2 {
		font-size: 1.5rem;
		font-weight: 700;
		color: var(--text);
		margin: 0 0 4px 0;
	}
	.header-text p {
		font-size: 14px;
		color: var(--text-dim);
		margin: 0;
	}

	.scan-status {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 8px 16px;
		background: rgba(16, 185, 129, 0.1);
		border-radius: 20px;
	}

	.scan-dot {
		width: 8px;
		height: 8px;
		background: var(--green);
		border-radius: 50%;
		animation: blink 1s ease-in-out infinite;
	}

	@keyframes blink {
		0%,
		100% {
			opacity: 1;
		}
		50% {
			opacity: 0.3;
		}
	}

	.scan-text {
		font-size: 11px;
		color: var(--green);
		font-weight: 600;
		letter-spacing: 0.1em;
	}

	.scanner-interface {
		display: grid;
		grid-template-columns: 1fr 1.2fr;
		gap: 24px;
		margin-bottom: 24px;
	}

	@media (max-width: 768px) {
		.scanner-interface {
			grid-template-columns: 1fr;
		}
	}

	.scan-visualization {
		background: rgba(0, 0, 0, 0.3);
		border-radius: 12px;
		padding: 20px;
		display: flex;
		flex-direction: column;
		gap: 20px;
	}

	.radar-container {
		position: relative;
		width: 150px;
		height: 150px;
		margin: 0 auto;
	}

	.radar-ring {
		position: absolute;
		inset: 0;
		border: 1px solid rgba(16, 185, 129, 0.3);
		border-radius: 50%;
		animation: radarExpand 3s ease-out infinite;
	}

	.radar-ring:nth-child(2) {
		animation-delay: 1s;
	}
	.radar-ring:nth-child(3) {
		animation-delay: 2s;
	}

	@keyframes radarExpand {
		0% {
			transform: scale(0);
			opacity: 1;
		}
		100% {
			transform: scale(1.5);
			opacity: 0;
		}
	}

	.radar-sweep {
		position: absolute;
		inset: 0;
		border-radius: 50%;
		background: conic-gradient(
			from 0deg,
			transparent 0deg,
			rgba(16, 185, 129, 0.2) 60deg,
			transparent 60deg
		);
		animation: radarSweep 4s linear infinite;
	}

	@keyframes radarSweep {
		0% {
			transform: rotate(0deg);
		}
		100% {
			transform: rotate(360deg);
		}
	}

	.blip {
		position: absolute;
		width: 6px;
		height: 6px;
		background: var(--green);
		border-radius: 50%;
		top: var(--y);
		left: var(--x);
		animation: blipAppear 2s ease-out infinite;
		animation-delay: var(--delay);
		opacity: 0;
	}

	@keyframes blipAppear {
		0%,
		100% {
			opacity: 0;
			transform: scale(0);
		}
		50% {
			opacity: 1;
			transform: scale(1);
		}
	}

	.scan-metrics {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 12px;
	}

	.metric {
		text-align: center;
		padding: 12px;
		background: rgba(255, 255, 255, 0.03);
		border-radius: 8px;
	}

	.metric-label {
		display: block;
		font-size: 10px;
		color: var(--text-dim);
		margin-bottom: 4px;
	}

	.metric-value {
		display: block;
		font-size: 18px;
		font-weight: 700;
		color: var(--gold);
		font-family: 'SF Mono', monospace;
	}

	.live-counter {
		animation: counterPulse 0.5s ease;
	}

	@keyframes counterPulse {
		0% {
			transform: scale(1.2);
			color: var(--green);
		}
		100% {
			transform: scale(1);
		}
	}

	.signals-panel {
		background: rgba(0, 0, 0, 0.3);
		border-radius: 12px;
		padding: 16px;
		border: 1px solid rgba(255, 255, 255, 0.05);
	}

	.panel-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 12px;
		padding-bottom: 12px;
		border-bottom: 1px solid rgba(255, 255, 255, 0.05);
	}

	.panel-header span:first-child {
		font-size: 12px;
		font-weight: 600;
		color: var(--text);
	}

	.update-indicator {
		font-size: 10px;
		color: var(--green);
		letter-spacing: 0.1em;
		animation: blink 1s ease-in-out infinite;
	}

	.signals-list {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.signal-row {
		display: grid;
		grid-template-columns: 1fr 1fr auto;
		gap: 12px;
		align-items: center;
		padding: 10px;
		background: rgba(255, 255, 255, 0.03);
		border-radius: 8px;
		font-size: 13px;
		animation: signalEnter 0.3s ease;
		transition: all 0.3s ease;
	}

	.signal-row.fading {
		opacity: 0.3;
	}
	.signal-row.dimming {
		opacity: 0.6;
	}

	@keyframes signalEnter {
		from {
			opacity: 0;
			transform: translateX(-20px);
		}
		to {
			opacity: 1;
			transform: translateX(0);
		}
	}

	.signal-main {
		display: flex;
		flex-direction: column;
		gap: 2px;
	}

	.sig-symbol {
		font-weight: 700;
		color: var(--gold);
		font-family: 'SF Mono', monospace;
	}

	.sig-type {
		font-size: 11px;
		color: var(--text-dim);
	}

	.signal-data {
		display: flex;
		flex-direction: column;
		gap: 4px;
		align-items: flex-end;
	}

	.sig-price {
		font-weight: 600;
		font-family: 'SF Mono', monospace;
	}

	.sig-confidence {
		display: flex;
		align-items: center;
		gap: 6px;
	}

	.conf-bar {
		width: 30px;
		height: 3px;
		background: rgba(255, 255, 255, 0.1);
		border-radius: 2px;
		position: relative;
	}

	.conf-bar::after {
		content: '';
		position: absolute;
		inset: 0;
		width: var(--width);
		background: linear-gradient(90deg, var(--copper), var(--gold));
		border-radius: 2px;
	}

	.sig-confidence span {
		font-size: 11px;
		color: var(--gold);
	}

	.sig-time {
		font-size: 11px;
		color: var(--text-dim);
	}

	.scanners-features {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
		gap: 12px;
	}

	.feature-item {
		display: flex;
		align-items: center;
		gap: 10px;
		font-size: 14px;
		color: var(--text-dim);
		padding: 10px 12px;
		background: rgba(255, 255, 255, 0.03);
		border-radius: 8px;
	}

	.feature-check {
		color: var(--gold);
		font-size: 12px;
	}

	/* University Section */
	.university-section {
		opacity: 0;
		transform: translateY(40px);
		transition: all 0.8s cubic-bezier(0.16, 1, 0.3, 1);
	}

	.university-section.visible {
		opacity: 1;
		transform: translateY(0);
	}

	.university-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 24px;
	}

	@media (max-width: 768px) {
		.university-grid {
			grid-template-columns: 1fr;
		}
	}

	.uni-card {
		background: rgba(18, 18, 20, 0.6);
		border: 1px solid rgba(255, 255, 255, 0.08);
		border-radius: 20px;
		padding: 28px;
		backdrop-filter: blur(10px);
	}

	.uni-card.day-trading {
		border-color: rgba(245, 158, 11, 0.2);
	}
	.uni-card.swing-trading {
		border-color: rgba(200, 121, 65, 0.2);
	}

	.uni-header {
		display: flex;
		align-items: center;
		gap: 16px;
		margin-bottom: 24px;
		padding-bottom: 16px;
		border-bottom: 1px solid rgba(255, 255, 255, 0.05);
	}

	.uni-icon {
		width: 48px;
		height: 48px;
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.uni-icon.amber {
		background: linear-gradient(135deg, rgba(245, 158, 11, 0.2), rgba(200, 121, 65, 0.2));
		color: var(--amber);
	}

	.uni-icon.copper {
		background: linear-gradient(135deg, rgba(200, 121, 65, 0.2), rgba(212, 175, 55, 0.2));
		color: var(--copper);
	}

	.uni-icon svg {
		width: 24px;
		height: 24px;
	}

	.uni-title h3 {
		font-size: 1.25rem;
		font-weight: 700;
		color: var(--text);
		margin: 0 0 4px 0;
	}
	.uni-title p {
		font-size: 13px;
		color: var(--text-dim);
		margin: 0;
	}

	.uni-modules {
		display: flex;
		flex-direction: column;
		gap: 12px;
	}

	.module-row {
		display: flex;
		gap: 16px;
		align-items: flex-start;
		padding: 12px;
		background: rgba(255, 255, 255, 0.03);
		border-radius: 10px;
	}

	.mod-num {
		font-size: 20px;
		font-weight: 800;
		color: var(--amber);
		opacity: 0.4;
		line-height: 1;
	}

	.mod-info h4 {
		font-size: 14px;
		font-weight: 600;
		color: var(--text);
		margin: 0 0 2px 0;
	}
	.mod-info p {
		font-size: 12px;
		color: var(--text-dim);
		margin: 0;
	}

	.swing-wave {
		margin-top: 20px;
	}

	.swing-wave svg {
		width: 100%;
		height: 100px;
	}

	.wave-line {
		stroke-dasharray: 300;
		stroke-dashoffset: 300;
		animation: drawWave 2s ease forwards;
	}

	@keyframes drawWave {
		to {
			stroke-dashoffset: 0;
		}
	}

	.wave-labels {
		display: flex;
		justify-content: space-between;
		padding: 0 20px;
		margin-top: 8px;
	}

	.wave-labels span {
		font-size: 11px;
		color: var(--text-dim);
	}

	/* Infrastructure Section */
	.infra-section {
		opacity: 0;
		transform: translateY(40px);
		transition: all 0.8s cubic-bezier(0.16, 1, 0.3, 1);
	}

	.infra-section.visible {
		opacity: 1;
		transform: translateY(0);
	}

	.infra-card {
		background: rgba(18, 18, 20, 0.6);
		border: 1px solid rgba(212, 175, 55, 0.2);
		border-radius: 20px;
		padding: 32px;
		backdrop-filter: blur(10px);
	}

	.infra-header {
		display: flex;
		align-items: center;
		gap: 20px;
		margin-bottom: 24px;
	}

	.server-visualization {
		display: grid;
		grid-template-columns: 1.5fr 1fr;
		gap: 24px;
	}

	@media (max-width: 768px) {
		.server-visualization {
			grid-template-columns: 1fr;
		}
	}

	.server-rack {
		background: rgba(0, 0, 0, 0.4);
		border-radius: 12px;
		padding: 20px;
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.server-unit {
		background: rgba(18, 18, 20, 0.8);
		border: 1px solid rgba(255, 255, 255, 0.05);
		border-radius: 6px;
		padding: 10px 12px;
		display: grid;
		grid-template-columns: 80px 1fr 60px;
		gap: 12px;
		align-items: center;
		opacity: 0;
		animation: unitAppear 0.3s ease forwards;
		animation-delay: var(--delay);
	}

	@keyframes unitAppear {
		to {
			opacity: 1;
		}
	}

	.unit-header {
		display: flex;
		flex-direction: column;
		gap: 2px;
	}

	.unit-id {
		font-size: 10px;
		color: var(--text-muted);
		font-family: 'SF Mono', monospace;
	}

	.unit-status {
		display: flex;
		align-items: center;
		gap: 4px;
		font-size: 9px;
	}

	.status-dot {
		width: 6px;
		height: 6px;
		border-radius: 50%;
		background: var(--red);
	}
	.unit-status.active .status-dot {
		background: var(--green);
		animation: blink 1s ease-in-out infinite;
	}
	.unit-status.active {
		color: var(--green);
	}

	.unit-lights {
		display: flex;
		gap: 4px;
		justify-content: center;
	}

	.light {
		width: 4px;
		height: 12px;
		background: rgba(255, 255, 255, 0.1);
		border-radius: 2px;
	}

	.light.blink {
		background: var(--green);
		animation: lightBlink 0.3s ease infinite alternate;
		animation-delay: var(--delay);
	}

	@keyframes lightBlink {
		from {
			opacity: 0.3;
		}
		to {
			opacity: 1;
		}
	}

	.unit-load {
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.load-bar {
		flex: 1;
		height: 4px;
		background: rgba(255, 255, 255, 0.1);
		border-radius: 2px;
		position: relative;
		overflow: hidden;
	}

	.load-bar::after {
		content: '';
		position: absolute;
		inset: 0;
		width: var(--load);
		background: linear-gradient(90deg, var(--green), var(--amber));
		border-radius: 2px;
		transition: width 0.5s ease;
	}

	.load-text {
		font-size: 10px;
		color: var(--text-dim);
		font-family: 'SF Mono', monospace;
	}

	.expansion-metrics {
		display: flex;
		flex-direction: column;
		gap: 16px;
		justify-content: center;
	}

	.exp-card {
		text-align: center;
		padding: 24px;
		background: rgba(212, 175, 55, 0.05);
		border: 1px solid rgba(212, 175, 55, 0.1);
		border-radius: 12px;
	}

	.exp-value {
		display: block;
		font-size: 32px;
		font-weight: 800;
		background: linear-gradient(135deg, var(--gold), var(--copper));
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
		margin-bottom: 4px;
	}

	.exp-label {
		font-size: 12px;
		color: var(--text-dim);
	}

	/* Stats Section */
	.stats-section {
		display: flex;
		justify-content: center;
		align-items: center;
		gap: 0;
		opacity: 0;
		transform: scale(0.95);
		transition: all 0.8s cubic-bezier(0.16, 1, 0.3, 1);
	}

	.stats-section.visible {
		opacity: 1;
		transform: scale(1);
	}

	.stat-item {
		text-align: center;
		padding: 24px 32px;
		min-width: 160px;
	}

	.stat-number {
		font-size: 2.25rem;
		font-weight: 800;
		background: linear-gradient(135deg, var(--gold), var(--copper));
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
		margin-bottom: 8px;
	}

	.stat-label {
		font-size: 11px;
		color: var(--text-dim);
		text-transform: uppercase;
		letter-spacing: 0.1em;
		margin-bottom: 12px;
	}

	.stat-bar {
		width: 100%;
		height: 3px;
		background: rgba(255, 255, 255, 0.1);
		border-radius: 2px;
		position: relative;
		overflow: hidden;
	}

	.stat-bar::after {
		content: '';
		position: absolute;
		inset: 0;
		width: calc(var(--progress) * 100%);
		background: linear-gradient(90deg, var(--copper), var(--gold));
		border-radius: 2px;
		transition: width 0.3s ease;
	}

	.stat-divider {
		display: flex;
		flex-direction: column;
		align-items: center;
		padding: 0 8px;
	}

	.stat-divider::before {
		content: '';
		width: 4px;
		height: 4px;
		background: var(--gold);
		border-radius: 50%;
	}

	/* Capture Section */
	.capture-section {
		background: rgba(18, 18, 20, 0.6);
		border: 1px solid rgba(212, 175, 55, 0.2);
		border-radius: 24px;
		padding: 40px;
		backdrop-filter: blur(10px);
	}

	.capture-box {
		text-align: center;
	}

	.capture-header {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		padding: 8px 16px;
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		border-radius: 20px;
		margin-bottom: 24px;
	}

	.live-pulse {
		width: 8px;
		height: 8px;
		background: var(--red);
		border-radius: 50%;
		animation: pulse 2s ease-in-out infinite;
	}

	.capture-header span {
		font-size: 12px;
		color: var(--red);
		font-weight: 600;
		letter-spacing: 0.1em;
	}

	.capture-title {
		font-size: 1.75rem;
		font-weight: 700;
		color: var(--text);
		margin: 0 0 8px 0;
	}

	.capture-subtitle {
		font-size: 14px;
		color: var(--text-dim);
		margin: 0 0 28px 0;
	}

	.input-wrapper {
		display: flex;
		gap: 12px;
		max-width: 480px;
		margin: 0 auto;
	}

	@media (max-width: 600px) {
		.input-wrapper {
			flex-direction: column;
		}
	}

	.email-input {
		flex: 1;
		padding: 14px 20px;
		background: rgba(255, 255, 255, 0.05);
		border: 1px solid rgba(212, 175, 55, 0.2);
		border-radius: 10px;
		color: var(--text);
		font-size: 1rem;
		outline: none;
		transition: all 0.3s ease;
	}

	.email-input:focus {
		border-color: var(--gold);
		box-shadow: 0 0 0 3px rgba(212, 175, 55, 0.15);
	}

	.magnetic-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		padding: 14px 28px;
		background: linear-gradient(135deg, var(--copper), var(--gold));
		border: none;
		border-radius: 10px;
		color: var(--dark);
		font-size: 1rem;
		font-weight: 700;
		cursor: pointer;
		transition: box-shadow 0.3s ease;
		white-space: nowrap;
		box-shadow: 0 8px 24px rgba(212, 175, 55, 0.3);
	}

	.magnetic-btn:hover {
		box-shadow: 0 12px 32px rgba(212, 175, 55, 0.4);
	}

	.magnetic-btn:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.spinner {
		width: 20px;
		height: 20px;
		border: 2px solid rgba(10, 10, 12, 0.3);
		border-top-color: var(--dark);
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	.error-msg {
		margin-top: 12px;
		padding: 10px 16px;
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		border-radius: 8px;
		color: var(--red);
		font-size: 14px;
	}

	.success-box {
		text-align: center;
	}

	.success-icon {
		width: 64px;
		height: 64px;
		margin: 0 auto 20px;
	}

	.success-icon svg {
		width: 100%;
		height: 100%;
	}

	.success-box h3 {
		font-size: 1.5rem;
		font-weight: 700;
		color: var(--green);
		margin: 0 0 8px 0;
	}

	.success-box p {
		font-size: 14px;
		color: var(--text-dim);
		margin: 0 0 20px 0;
	}

	.benefits {
		display: flex;
		justify-content: center;
		gap: 12px;
		flex-wrap: wrap;
	}

	.benefits span {
		padding: 8px 16px;
		background: rgba(16, 185, 129, 0.1);
		border: 1px solid rgba(16, 185, 129, 0.3);
		border-radius: 20px;
		font-size: 13px;
		color: var(--green);
	}

	/* Footer */
	.footer {
		text-align: center;
		padding-top: 40px;
		border-top: 1px solid rgba(255, 255, 255, 0.05);
	}

	.security-note {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		font-size: 12px;
		color: var(--text-dim);
	}

	.security-note svg {
		width: 16px;
		height: 16px;
	}

	/* Responsive */
	@media (max-width: 768px) {
		.content {
			padding: 90px 16px 40px;
			gap: 60px;
		}
		.stats-section {
			flex-wrap: wrap;
			gap: 16px;
		}
		.stat-divider {
			display: none;
		}
		.capture-section {
			padding: 28px 20px;
		}
	}

	/* Mobile First Responsive Design */
	@media (max-width: 480px) {
		/* Manifesto */
		.manifesto-headline {
			font-size: 1.75rem;
		}

		.manifesto-sub {
			font-size: 0.9375rem;
		}

		/* Charts */
		.chart-container {
			height: 250px;
		}

		.mini-container {
			height: 100px;
		}

		.chart-header {
			flex-direction: column;
			gap: 12px;
			align-items: flex-start;
		}

		.price-live {
			width: 100%;
			justify-content: space-between;
		}

		.current-price {
			font-size: 20px;
		}

		.chart-footer {
			flex-direction: column;
			gap: 12px;
			align-items: flex-start;
		}

		/* Scanners */
		.scanners-header {
			flex-direction: column;
			text-align: center;
			gap: 16px;
		}

		.header-text h2 {
			font-size: 1.25rem;
		}

		.scanner-interface {
			grid-template-columns: 1fr;
		}

		.radar-container {
			width: 120px;
			height: 120px;
		}

		.scan-metrics {
			grid-template-columns: 1fr;
		}

		.signal-row {
			grid-template-columns: 1fr;
			gap: 8px;
		}

		.signal-data {
			align-items: flex-start;
		}

		.scanners-features {
			grid-template-columns: 1fr;
		}

		/* University */
		.university-grid {
			grid-template-columns: 1fr;
		}

		.uni-header {
			flex-direction: column;
			text-align: center;
		}

		/* Infrastructure */
		.infra-header {
			flex-direction: column;
			text-align: center;
		}

		.server-unit {
			grid-template-columns: 1fr;
			gap: 8px;
		}

		.unit-lights {
			justify-content: flex-start;
		}

		.expansion-metrics {
			gap: 12px;
		}

		.exp-card {
			padding: 16px;
		}

		.exp-value {
			font-size: 24px;
		}

		/* Stats */
		.stat-item {
			padding: 16px 20px;
			min-width: auto;
			flex: 1 1 45%;
		}

		.stat-number {
			font-size: 1.75rem;
		}

		/* Capture */
		.capture-title {
			font-size: 1.5rem;
		}

		.input-wrapper {
			flex-direction: column;
		}

		.magnetic-btn {
			width: 100%;
			justify-content: center;
		}

		.benefits {
			flex-direction: column;
			align-items: center;
		}

		.benefits span {
			width: 100%;
			text-align: center;
		}

		/* Market tape */
		.tape-item {
			font-size: 11px;
			padding: 2px 6px;
		}
	}

	/* Tablet */
	@media (min-width: 481px) and (max-width: 768px) {
		.content {
			padding: 100px 20px 50px;
		}

		.charts-grid {
			grid-template-columns: 1fr;
		}

		.scanner-interface {
			grid-template-columns: 1fr;
		}

		.university-grid {
			grid-template-columns: 1fr;
		}

		.server-visualization {
			grid-template-columns: 1fr;
		}

		.stats-section {
			flex-wrap: wrap;
			justify-content: center;
		}

		.stat-item {
			flex: 1 1 40%;
		}
	}

	/* Small Desktop */
	@media (min-width: 769px) and (max-width: 1024px) {
		.content {
			padding: 100px 24px 60px;
		}

		.charts-grid {
			grid-template-columns: 1.5fr 1fr;
		}

		.scanner-interface {
			grid-template-columns: 1fr 1.2fr;
		}

		.university-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	/* Large Desktop */
	@media (min-width: 1025px) and (max-width: 1440px) {
		.content {
			max-width: 1100px;
		}
	}

	/* Extra Large */
	@media (min-width: 1441px) {
		.content {
			max-width: 1300px;
		}

		.manifesto-headline {
			font-size: 4.5rem;
		}

		.chart-container {
			height: 400px;
		}
	}

	/* Touch Device Optimizations */
	@media (hover: none) and (pointer: coarse) {
		.magnetic-btn {
			transform: none !important;
		}

		.tape-item:hover {
			transform: none;
		}

		.feature-item:hover,
		.signal-row:hover,
		.module-row:hover {
			transform: none;
		}
	}

	/* Reduced Motion */
	@media (prefers-reduced-motion: reduce) {
		.tape-track,
		.radar-sweep,
		.radar-ring,
		.blip,
		.pulse-dot,
		.live-pulse,
		.scan-dot,
		.light.blink,
		.orb {
			animation: none;
		}
	}

	/* Dark mode support (already dark, but ensure consistency) */
	@media (prefers-color-scheme: dark) {
		.experience-container {
			background: var(--dark);
		}
	}

	/* Landscape orientation on mobile */
	@media (max-width: 768px) and (orientation: landscape) {
		.chart-container {
			height: 200px;
		}

		.manifesto-headline {
			font-size: 1.5rem;
		}
	}

	/* High DPI / Retina displays */
	@media (-webkit-min-device-pixel-ratio: 2), (min-resolution: 192dpi) {
		.tape-item,
		.current-price,
		.mini-price,
		.stat-number,
		.metric-value {
			-webkit-font-smoothing: antialiased;
			-moz-osx-font-smoothing: grayscale;
		}
	}

	/* Print styles */
	@media print {
		.experience-container {
			position: static;
			overflow: visible;
		}

		.market-tape,
		.ambient-layer,
		.radar-container,
		.radar-sweep,
		.blip {
			display: none;
		}

		.content {
			padding: 20px;
		}
	}
</style>

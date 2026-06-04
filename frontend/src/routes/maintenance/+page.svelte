<script lang="ts">
	import { browser } from '$app/environment';
	import { onMount, tick } from 'svelte';
	import type { Attachment } from 'svelte/attachments';
	import type {
		CandlestickData,
		HistogramData,
		IChartApi,
		ISeriesApi,
		UTCTimestamp
	} from 'lightweight-charts';

	type Direction = 'up' | 'down';
	type SectionId =
		| 'hero'
		| 'operations'
		| 'charts'
		| 'signals'
		| 'academy'
		| 'infrastructure'
		| 'access';
	type ChartKey = 'main' | 'aapl' | 'nvda';
	type SignalStatus = 'active' | 'watch' | 'cooling';
	type AcademyTrackId = 'day' | 'swing';

	interface Ticker {
		symbol: string;
		basePrice: number;
		price: number;
		change: number;
		direction: Direction;
	}

	interface Signal {
		id: number;
		symbol: string;
		type: string;
		price: number;
		confidence: number;
		age: number;
		status: SignalStatus;
	}

	interface NodeLoad {
		id: string;
		region: string;
		load: number;
		status: 'migrating' | 'online' | 'standby';
	}

	interface AcademyTrack {
		id: AcademyTrackId;
		number: string;
		title: string;
		tagline: string;
		stockFocus: string[];
		optionsFocus: string[];
		curriculum: string[];
		lab: string;
		outcome: string;
		accent: string;
	}

	interface OperationsChapter {
		number: string;
		title: string;
		copy: string;
		metric: string;
	}

	interface MarketParticle {
		x: number;
		y: number;
		vx: number;
		vy: number;
		size: number;
		phase: number;
		hue: number;
	}

	const timeframes = ['1m', '5m', '15m', '1H'];
	const revealSections: SectionId[] = [
		'hero',
		'operations',
		'charts',
		'signals',
		'academy',
		'infrastructure',
		'access'
	];

	let mounted = $state(false);
	let reducedMotion = $state(false);
	let activeTimeframe = $state('5m');
	let activeAcademyTrack = $state<AcademyTrackId>('day');
	let scrollProgress = $state(0);
	let glareX = $state(50);
	let glareY = $state(30);

	let email = $state('');
	let isSubmitting = $state(false);
	let isSubmitted = $state(false);
	let errorMessage = $state('');

	let visible = $state<Record<SectionId, boolean>>({
		hero: false,
		operations: false,
		charts: false,
		signals: false,
		academy: false,
		infrastructure: false,
		access: false
	});

	let stats = $state({
		traders: 0,
		scanners: 0,
		latency: 0,
		uptime: 0
	});

	let liveTickers = $state<Ticker[]>([
		{ symbol: 'SPX', basePrice: 5892.33, price: 5892.33, change: 1.24, direction: 'up' },
		{ symbol: 'NQ', basePrice: 20845.2, price: 20845.2, change: 2.34, direction: 'up' },
		{ symbol: 'ES', basePrice: 5890.5, price: 5890.5, change: 0.89, direction: 'up' },
		{ symbol: 'VIX', basePrice: 12.45, price: 12.45, change: -8.2, direction: 'down' },
		{ symbol: 'AAPL', basePrice: 232.3, price: 232.3, change: 1.45, direction: 'up' },
		{ symbol: 'NVDA', basePrice: 1475.2, price: 1475.2, change: 5.45, direction: 'up' },
		{ symbol: 'TSLA', basePrice: 348.5, price: 348.5, change: 3.3, direction: 'up' },
		{ symbol: 'NFLX', basePrice: 702.1, price: 702.1, change: -0.63, direction: 'down' }
	]);

	let scannerSignals = $state<Signal[]>([
		{
			id: 1,
			symbol: 'NVDA',
			type: 'Momentum expansion',
			price: 1475.2,
			confidence: 97,
			age: 2,
			status: 'active'
		},
		{
			id: 2,
			symbol: 'AAPL',
			type: 'Opening range break',
			price: 232.45,
			confidence: 94,
			age: 5,
			status: 'active'
		},
		{
			id: 3,
			symbol: 'SPY',
			type: 'Gamma pressure',
			price: 518.75,
			confidence: 91,
			age: 9,
			status: 'watch'
		},
		{
			id: 4,
			symbol: 'TSLA',
			type: 'Dark pool repeat',
			price: 348.5,
			confidence: 89,
			age: 14,
			status: 'cooling'
		}
	]);

	let nodes = $state<NodeLoad[]>([
		{ id: 'NY4-01', region: 'New York', load: 64, status: 'online' },
		{ id: 'CH2-02', region: 'Chicago', load: 58, status: 'online' },
		{ id: 'LD4-03', region: 'London', load: 47, status: 'migrating' },
		{ id: 'TY3-04', region: 'Tokyo', load: 31, status: 'standby' }
	]);

	const stageLines = [
		'Resilient data routes',
		'Scanner model deployment',
		'Institutional curriculum migration',
		'Member room hardening'
	];

	const operationsChapters: OperationsChapter[] = [
		{
			number: '01',
			title: 'Market Data Layer',
			copy: 'Tick, quote, chart, watchlist, and scanner data are being normalized into a faster decision layer for equities and index products.',
			metric: 'sub-20ms feed'
		},
		{
			number: '02',
			title: 'Options Risk Desk',
			copy: 'Delta, gamma, theta, IV rank, 0DTE exposure, spread structure, and event risk move into one options workflow.',
			metric: 'Greeks + IV'
		},
		{
			number: '03',
			title: 'Scanner Edge Engine',
			copy: 'Relative volume, liquidity sweeps, sector leadership, dark-pool repeats, and momentum quality are ranked before alerts reach the desk.',
			metric: '12,847 symbols'
		},
		{
			number: '04',
			title: 'Education Desk',
			copy: 'Day Trading University and Swing Trading University now separate stock execution, options strategy, risk protocol, and replay review into institution-grade tracks.',
			metric: 'stocks + options'
		}
	];

	const academyTracks: AcademyTrack[] = [
		{
			id: 'day',
			number: '01',
			title: 'Day Trading University',
			tagline:
				'An institutional intraday desk curriculum for stocks and options, built around execution quality, tape speed, and repeatable risk.',
			stockFocus: ['Opening range', 'VWAP execution', 'Tape reading', 'Liquidity sweeps'],
			optionsFocus: [
				'0DTE playbooks',
				'Greeks under pressure',
				'Premium decay',
				'Flow confirmation'
			],
			curriculum: [
				'Market structure, auction theory, and pre-market scenario mapping',
				'Equity momentum, relative volume, news catalysts, and sector rotation',
				'Options chain reading, delta selection, spread construction, and gamma risk',
				'Trade management: scaling, invalidation, max loss, review, and journaling'
			],
			lab: 'Live-market replay lab with execution scoring, risk heatmaps, and coach review checkpoints.',
			outcome:
				'Graduate with a written intraday stock/options playbook, daily prep routine, and measurable execution rules.',
			accent: '#16f2a9'
		},
		{
			id: 'swing',
			number: '02',
			title: 'Swing Trading University',
			tagline:
				'A professional portfolio curriculum for stocks and options, focused on multi-day thesis building, catalysts, and volatility-aware sizing.',
			stockFocus: ['Relative strength', 'Base breakouts', 'Catalyst mapping', 'Position sizing'],
			optionsFocus: ['Debit spreads', 'Calendars', 'IV rank', 'Event risk'],
			curriculum: [
				'Top-down market regime, sector leadership, breadth, and macro calendar analysis',
				'Equity screening, trend quality, accumulation, and breakout failure diagnostics',
				'Options strategy selection by volatility, duration, delta, theta, and event calendar',
				'Portfolio construction: correlation, drawdown control, exits, and post-trade audit'
			],
			lab: 'Multi-session portfolio simulator with catalyst timelines, volatility shifts, and scenario stress tests.',
			outcome:
				'Graduate with a swing stock/options model portfolio, risk template, watchlist process, and review cadence.',
			accent: '#f8d477'
		}
	];

	let shellEl: HTMLDivElement;
	let cinematicCanvasEl: HTMLCanvasElement;
	let mainChartEl: HTMLDivElement;
	let aaplChartEl: HTMLDivElement;
	let nvdaChartEl: HTMLDivElement;

	let mainChart: IChartApi | null = null;
	let aaplChart: IChartApi | null = null;
	let nvdaChart: IChartApi | null = null;
	let mainSeries: ISeriesApi<'Candlestick'> | null = null;
	let aaplSeries: ISeriesApi<'Area'> | null = null;
	let nvdaSeries: ISeriesApi<'Area'> | null = null;
	let volumeSeries: ISeriesApi<'Histogram'> | null = null;
	let chartsReady = false;
	let statsPlayed = false;
	let raf = 0;
	let sceneRaf = 0;
	let statsRaf = 0;

	let chartData = $state<Record<ChartKey, CandlestickData[]>>({
		main: [],
		aapl: [],
		nvda: []
	});

	const averageConfidence = $derived(
		Math.round(
			scannerSignals.reduce((total, signal) => total + signal.confidence, 0) / scannerSignals.length
		)
	);

	const activeSignals = $derived(
		scannerSignals.filter((signal) => signal.status === 'active').length
	);

	const activeAcademy = $derived(
		academyTracks.find((track) => track.id === activeAcademyTrack) ?? academyTracks[0]
	);

	const latestMainPrice = $derived(chartData.main.at(-1)?.close ?? 5892.33);
	const mainDirection = $derived(
		(chartData.main.at(-1)?.close ?? 0) >= (chartData.main.at(-1)?.open ?? 0) ? 'up' : 'down'
	);

	onMount(() => {
		if (!browser) return;

		mounted = true;
		reducedMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches;

		const cleanupReveal = setupRevealObserver();
		const cleanupIntervals = startLiveSystems();
		const cleanupPointer = setupPointerTracking();
		const cleanupCinema = startCinematicScene();

		visible.hero = true;

		return () => {
			cancelAnimationFrame(raf);
			cancelAnimationFrame(sceneRaf);
			cancelAnimationFrame(statsRaf);
			cleanupReveal?.();
			cleanupIntervals();
			cleanupPointer();
			cleanupCinema?.();
			mainChart?.remove();
			aaplChart?.remove();
			nvdaChart?.remove();
		};
	});

	const mountShell: Attachment<HTMLDivElement> = (node) => {
		shellEl = node;
	};

	const mountCinematicCanvas: Attachment<HTMLCanvasElement> = (node) => {
		cinematicCanvasEl = node;
	};

	const mountMainChart: Attachment<HTMLDivElement> = (node) => {
		mainChartEl = node;
	};

	const mountAaplChart: Attachment<HTMLDivElement> = (node) => {
		aaplChartEl = node;
	};

	const mountNvdaChart: Attachment<HTMLDivElement> = (node) => {
		nvdaChartEl = node;
	};

	function seededRandom(seed: number) {
		let value = seed % 2147483647;
		if (value <= 0) value += 2147483646;

		return () => {
			value = (value * 16807) % 2147483647;
			return (value - 1) / 2147483646;
		};
	}

	function generateCandles(seed: number, startPrice: number, count: number, volatility = 0.007) {
		const random = seededRandom(seed);
		const candles: CandlestickData[] = [];
		const start = Math.floor(Date.now() / 1000) - count * 60;
		let price = startPrice;
		let drift = 0.35;

		for (let i = 0; i < count; i += 1) {
			const wave = Math.sin(i / 6) * startPrice * volatility * 0.35;
			const shock = (random() - 0.46) * startPrice * volatility;
			const open = price;
			const close = open + wave * 0.08 + shock + drift;
			const high = Math.max(open, close) + random() * startPrice * volatility * 0.55;
			const low = Math.min(open, close) - random() * startPrice * volatility * 0.55;

			candles.push({
				time: (start + i * 60) as UTCTimestamp,
				open: Number(open.toFixed(2)),
				high: Number(high.toFixed(2)),
				low: Number(low.toFixed(2)),
				close: Number(close.toFixed(2))
			});

			price = close;
			drift = drift * 0.97 + (random() - 0.45) * 0.22;
		}

		return candles;
	}

	function setupRevealObserver() {
		if (!browser || !('IntersectionObserver' in window)) {
			revealSections.forEach((section) => {
				visible[section] = true;
			});
			void initCharts();
			playStats();
			return;
		}

		const observer = new IntersectionObserver(
			(entries) => {
				for (const entry of entries) {
					if (!entry.isIntersecting) continue;
					const id = entry.target.id as SectionId;
					visible[id] = true;

					if (id === 'charts') void initCharts();
					if (id === 'infrastructure') playStats();
				}
			},
			{ root: shellEl, threshold: 0.26, rootMargin: '0px 0px -8% 0px' }
		);

		revealSections.forEach((id) => {
			const element = document.getElementById(id);
			if (element) observer.observe(element);
		});

		return () => observer.disconnect();
	}

	function setupPointerTracking() {
		const handlePointerMove = (event: PointerEvent) => {
			glareX = Math.round((event.clientX / window.innerWidth) * 100);
			glareY = Math.round((event.clientY / window.innerHeight) * 100);
		};

		window.addEventListener('pointermove', handlePointerMove, { passive: true });
		return () => window.removeEventListener('pointermove', handlePointerMove);
	}

	function startCinematicScene() {
		if (!cinematicCanvasEl) return () => {};

		const canvas = cinematicCanvasEl;
		const context = canvas.getContext('2d', { alpha: true });
		if (!context) return () => {};
		const ctx: CanvasRenderingContext2D = context;

		let width = 0;
		let height = 0;
		let dpr = 1;
		let particles: MarketParticle[] = [];

		function createParticles() {
			const count = Math.min(118, Math.max(58, Math.floor((width * height) / 18000)));
			particles = Array.from({ length: count }, (_, index) => ({
				x: (index * 79 + Math.random() * width) % width,
				y: Math.random() * height,
				vx: (Math.random() - 0.5) * 0.24,
				vy: 0.18 + Math.random() * 0.34,
				size: 0.8 + Math.random() * 1.9,
				phase: Math.random() * Math.PI * 2,
				hue: index % 3 === 0 ? 156 : index % 3 === 1 ? 44 : 198
			}));
		}

		function resize() {
			dpr = Math.min(window.devicePixelRatio || 1, 2);
			width = window.innerWidth;
			height = window.innerHeight;
			canvas.width = Math.floor(width * dpr);
			canvas.height = Math.floor(height * dpr);
			canvas.style.width = `${width}px`;
			canvas.style.height = `${height}px`;
			ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
			createParticles();
		}

		function drawPricePath(time: number, offset: number, color: string, amplitude: number) {
			ctx.beginPath();
			for (let x = -20; x <= width + 20; x += 18) {
				const normalized = x / width;
				const y =
					height * offset +
					Math.sin(normalized * 10 + time * 0.00055) * amplitude +
					Math.sin(normalized * 27 + time * 0.00028) * amplitude * 0.42 -
					scrollProgress * 80;
				if (x === -20) ctx.moveTo(x, y);
				else ctx.lineTo(x, y);
			}
			ctx.strokeStyle = color;
			ctx.lineWidth = 1.4;
			ctx.shadowBlur = 18;
			ctx.shadowColor = color;
			ctx.stroke();
			ctx.shadowBlur = 0;
		}

		function draw(now = performance.now()) {
			ctx.clearRect(0, 0, width, height);
			ctx.globalCompositeOperation = 'lighter';

			const scrollLift = scrollProgress * height * 0.18;
			const scanX = ((now * 0.055) % (width + 360)) - 180;

			const beam = ctx.createLinearGradient(scanX - 160, 0, scanX + 160, height);
			beam.addColorStop(0, 'rgba(22, 242, 169, 0)');
			beam.addColorStop(0.5, 'rgba(22, 242, 169, 0.10)');
			beam.addColorStop(1, 'rgba(139, 211, 255, 0)');
			ctx.fillStyle = beam;
			ctx.fillRect(scanX - 160, 0, 320, height);

			for (let x = -80; x < width + 160; x += 120) {
				ctx.strokeStyle = 'rgba(248, 212, 119, 0.045)';
				ctx.lineWidth = 1;
				ctx.beginPath();
				ctx.moveTo(x + scrollProgress * 160, 0);
				ctx.lineTo(x - 240 + scrollProgress * 160, height);
				ctx.stroke();
			}

			drawPricePath(now, 0.24, 'rgba(22, 242, 169, 0.28)', 42);
			drawPricePath(now + 1600, 0.54, 'rgba(248, 212, 119, 0.22)', 56);
			drawPricePath(now + 3200, 0.78, 'rgba(139, 211, 255, 0.20)', 34);

			for (let index = 0; index < particles.length; index += 1) {
				const particle = particles[index];
				const pulse = 0.55 + Math.sin(now * 0.002 + particle.phase) * 0.45;
				particle.x += particle.vx + Math.sin(now * 0.00035 + particle.phase) * 0.08;
				particle.y += particle.vy + scrollProgress * 0.28;

				if (particle.y - scrollLift > height + 24) {
					particle.y = -24 + scrollLift;
					particle.x = Math.random() * width;
				}
				if (particle.x < -24) particle.x = width + 24;
				if (particle.x > width + 24) particle.x = -24;

				const x = particle.x;
				const y = particle.y - scrollLift;

				for (let otherIndex = index + 1; otherIndex < particles.length; otherIndex += 1) {
					const other = particles[otherIndex];
					const otherY = other.y - scrollLift;
					const dx = x - other.x;
					const dy = y - otherY;
					const distance = Math.hypot(dx, dy);
					if (distance > 118) continue;

					const alpha = (1 - distance / 118) * 0.13;
					ctx.strokeStyle = `hsla(${particle.hue}, 92%, 64%, ${alpha})`;
					ctx.lineWidth = 0.7;
					ctx.beginPath();
					ctx.moveTo(x, y);
					ctx.lineTo(other.x, otherY);
					ctx.stroke();
				}

				ctx.fillStyle = `hsla(${particle.hue}, 96%, 64%, ${0.18 + pulse * 0.34})`;
				ctx.shadowBlur = 12 + pulse * 10;
				ctx.shadowColor = `hsla(${particle.hue}, 96%, 64%, 0.56)`;
				ctx.beginPath();
				ctx.arc(x, y, particle.size + pulse * 0.8, 0, Math.PI * 2);
				ctx.fill();
				ctx.shadowBlur = 0;
			}

			ctx.globalCompositeOperation = 'source-over';

			if (!reducedMotion) {
				sceneRaf = requestAnimationFrame(draw);
			}
		}

		resize();
		window.addEventListener('resize', resize, { passive: true });
		draw();

		return () => {
			window.removeEventListener('resize', resize);
			cancelAnimationFrame(sceneRaf);
		};
	}

	function handleScroll() {
		cancelAnimationFrame(raf);
		raf = requestAnimationFrame(() => {
			const max = shellEl.scrollHeight - shellEl.clientHeight;
			scrollProgress = max > 0 ? shellEl.scrollTop / max : 0;
		});
	}

	async function initCharts() {
		if (chartsReady || !browser || !mainChartEl || !aaplChartEl || !nvdaChartEl) return;

		await tick();
		const { createChart, CandlestickSeries, AreaSeries, HistogramSeries } =
			await import('lightweight-charts');

		chartData = {
			main: generateCandles(1487, 5892.33, 76, 0.0045),
			aapl: generateCandles(412, 232.3, 48, 0.0065),
			nvda: generateCandles(911, 1475.2, 48, 0.0078)
		};

		const commonLayout = {
			background: { color: 'transparent' },
			textColor: 'rgba(238, 242, 255, 0.68)',
			fontFamily: 'Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, sans-serif',
			fontSize: 11
		};

		mainChart = createChart(mainChartEl, {
			autoSize: true,
			layout: commonLayout,
			grid: {
				vertLines: { color: 'rgba(96, 165, 250, 0.08)' },
				horzLines: { color: 'rgba(255, 255, 255, 0.05)' }
			},
			rightPriceScale: {
				borderColor: 'rgba(255, 255, 255, 0.12)',
				scaleMargins: { top: 0.08, bottom: 0.18 }
			},
			timeScale: {
				borderColor: 'rgba(255, 255, 255, 0.12)',
				timeVisible: true,
				secondsVisible: false
			},
			crosshair: {
				vertLine: { color: 'rgba(20, 184, 166, 0.45)', style: 3 },
				horzLine: { color: 'rgba(20, 184, 166, 0.45)', style: 3 }
			}
		});

		mainSeries = mainChart.addSeries(CandlestickSeries, {
			upColor: '#16f2a9',
			downColor: '#ff5f6d',
			borderUpColor: '#16f2a9',
			borderDownColor: '#ff5f6d',
			wickUpColor: '#16f2a9',
			wickDownColor: '#ff5f6d'
		});

		volumeSeries = mainChart.addSeries(HistogramSeries, {
			priceFormat: { type: 'volume' },
			priceScaleId: ''
		});

		mainSeries.setData(chartData.main);
		volumeSeries.priceScale().applyOptions({ scaleMargins: { top: 0.82, bottom: 0 } });
		volumeSeries.setData(
			chartData.main.map<HistogramData>((candle, index) => ({
				time: candle.time,
				value: 480000 + ((index * 13729) % 620000),
				color: candle.close >= candle.open ? 'rgba(22, 242, 169, 0.32)' : 'rgba(255, 95, 109, 0.32)'
			}))
		);

		aaplChart = createMiniChart(createChart, aaplChartEl);
		nvdaChart = createMiniChart(createChart, nvdaChartEl);

		aaplSeries = aaplChart.addSeries(AreaSeries, {
			lineColor: '#8bd3ff',
			topColor: 'rgba(139, 211, 255, 0.22)',
			bottomColor: 'rgba(139, 211, 255, 0)',
			lineWidth: 2
		});

		nvdaSeries = nvdaChart.addSeries(AreaSeries, {
			lineColor: '#f8d477',
			topColor: 'rgba(248, 212, 119, 0.22)',
			bottomColor: 'rgba(248, 212, 119, 0)',
			lineWidth: 2
		});

		aaplSeries.setData(
			chartData.aapl.map((candle) => ({ time: candle.time, value: candle.close }))
		);
		nvdaSeries.setData(
			chartData.nvda.map((candle) => ({ time: candle.time, value: candle.close }))
		);

		mainChart.timeScale().fitContent();
		aaplChart.timeScale().fitContent();
		nvdaChart.timeScale().fitContent();
		chartsReady = true;
	}

	function createMiniChart(
		createChart: typeof import('lightweight-charts').createChart,
		element: HTMLElement
	) {
		return createChart(element, {
			autoSize: true,
			layout: {
				background: { color: 'transparent' },
				textColor: 'rgba(238, 242, 255, 0.5)',
				fontFamily: 'Inter, ui-sans-serif, system-ui',
				fontSize: 10
			},
			grid: {
				vertLines: { visible: false },
				horzLines: { color: 'rgba(255, 255, 255, 0.04)' }
			},
			rightPriceScale: { visible: false },
			timeScale: { visible: false },
			crosshair: {
				vertLine: { visible: false },
				horzLine: { visible: false }
			}
		});
	}

	function updateCandle(key: ChartKey, amplitude: number) {
		const candles = chartData[key];
		const last = candles.at(-1);
		if (!last) return null;

		const now = Math.floor(Date.now() / 1000) as UTCTimestamp;
		const move = (Math.random() - 0.48) * amplitude;

		if (now - Number(last.time) > 45) {
			const next: CandlestickData = {
				time: now,
				open: last.close,
				high: Math.max(last.close, last.close + move),
				low: Math.min(last.close, last.close + move),
				close: Number((last.close + move).toFixed(2))
			};
			candles.push(next);
			if (candles.length > 84) candles.shift();
			return next;
		}

		last.close = Number((last.close + move).toFixed(2));
		last.high = Math.max(last.high, last.close);
		last.low = Math.min(last.low, last.close);
		return last;
	}

	function startLiveSystems() {
		const timers: number[] = [];

		timers.push(
			window.setInterval(() => {
				liveTickers = liveTickers.map((ticker) => {
					const move = (Math.random() - 0.48) * ticker.basePrice * 0.0009;
					const price = Number((ticker.price + move).toFixed(2));
					const change = Number((((price - ticker.basePrice) / ticker.basePrice) * 100).toFixed(2));

					return {
						...ticker,
						price,
						change,
						direction: price >= ticker.price ? 'up' : 'down'
					};
				});
			}, 2600)
		);

		timers.push(
			window.setInterval(() => {
				if (!chartsReady) return;

				const main = updateCandle('main', 7.4);
				const aapl = updateCandle('aapl', 0.72);
				const nvda = updateCandle('nvda', 5.4);

				if (main) {
					mainSeries?.update(main);
					volumeSeries?.update({
						time: main.time,
						value: Math.floor(520000 + Math.random() * 640000),
						color: main.close >= main.open ? 'rgba(22, 242, 169, 0.32)' : 'rgba(255, 95, 109, 0.32)'
					});
				}

				if (aapl) aaplSeries?.update({ time: aapl.time, value: aapl.close });
				if (nvda) nvdaSeries?.update({ time: nvda.time, value: nvda.close });
			}, 1400)
		);

		timers.push(
			window.setInterval(() => {
				const symbols = ['AMD', 'MSFT', 'GOOGL', 'AMZN', 'CRM', 'COIN', 'META', 'SHOP'];
				const types = [
					'Volume displacement',
					'Liquidity sweep',
					'Breakout retest',
					'Options impulse',
					'VWAP reclaim',
					'Relative strength'
				];

				if (Math.random() > 0.46) {
					const next: Signal = {
						id: Date.now(),
						symbol: symbols[Math.floor(Math.random() * symbols.length)],
						type: types[Math.floor(Math.random() * types.length)],
						price: Number((48 + Math.random() * 720).toFixed(2)),
						confidence: Math.floor(86 + Math.random() * 13),
						age: 0,
						status: 'active'
					};

					scannerSignals = [next, ...scannerSignals.slice(0, 4)];
				} else {
					scannerSignals = scannerSignals.map((signal) => {
						const age = signal.age + 2;
						return {
							...signal,
							age,
							status: age < 8 ? 'active' : age < 18 ? 'watch' : 'cooling'
						};
					});
				}
			}, 3600)
		);

		timers.push(
			window.setInterval(() => {
				nodes = nodes.map((node) => {
					const load = Math.max(
						18,
						Math.min(78, node.load + Math.round((Math.random() - 0.48) * 10))
					);
					return { ...node, load };
				});
			}, 2400)
		);

		return () => timers.forEach((timer) => window.clearInterval(timer));
	}

	function playStats() {
		if (statsPlayed) return;
		statsPlayed = true;
		cancelAnimationFrame(statsRaf);

		const targets = { traders: 50000, scanners: 12847, latency: 15, uptime: 99.99 };
		const duration = reducedMotion ? 1 : 1500;
		const start = performance.now();

		function frame(now: number) {
			const progress = Math.min(1, (now - start) / duration);
			const eased = 1 - Math.pow(1 - progress, 3);

			stats.traders = Math.round(targets.traders * eased);
			stats.scanners = Math.round(targets.scanners * eased);
			stats.latency = Math.round(targets.latency * eased);
			stats.uptime = Number((targets.uptime * eased).toFixed(2));

			if (progress < 1) {
				statsRaf = requestAnimationFrame(frame);
			} else {
				statsRaf = 0;
			}
		}

		statsRaf = requestAnimationFrame(frame);
	}

	async function handleNotifyMe() {
		const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
		if (!email || !emailRegex.test(email)) {
			errorMessage = 'Enter a valid email address.';
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

			if (!response.ok) throw new Error('Failed to subscribe');
			isSubmitted = true;
		} catch {
			errorMessage = 'Connection error. Please try again.';
		} finally {
			isSubmitting = false;
		}
	}

	function formatNumber(value: number) {
		return new Intl.NumberFormat('en-US').format(value);
	}
</script>

<svelte:head>
	<title>Platform Upgrade | Revolution Trading Pros</title>
	<meta
		name="description"
		content="Revolution Trading Pros is upgrading the trading platform, scanners, infrastructure, and member experience."
	/>
</svelte:head>

<div
	class="maintenance-experience"
	class:mounted
	style:--scroll-progress={scrollProgress}
	style:--glare-x={`${glareX}%`}
	style:--glare-y={`${glareY}%`}
	{@attach mountShell}
	onscroll={handleScroll}
>
	<div class="progress-rail" aria-hidden="true">
		<span></span>
	</div>

	<canvas class="cinematic-canvas" aria-hidden="true" {@attach mountCinematicCanvas}></canvas>

	<header class="market-tape" aria-label="Live market tape">
		<div class="tape-track" aria-hidden="true">
			{#each [...liveTickers, ...liveTickers, ...liveTickers] as ticker, index (ticker.symbol + '-' + index)}
				<div
					class="ticker-pill"
					class:up={ticker.direction === 'up'}
					class:down={ticker.direction === 'down'}
				>
					<span class="ticker-symbol">{ticker.symbol}</span>
					<span>${ticker.price.toFixed(2)}</span>
					<span>{ticker.change >= 0 ? '+' : ''}{ticker.change.toFixed(2)}%</span>
				</div>
			{/each}
		</div>
	</header>

	<div class="cinema-light" aria-hidden="true"></div>
	<div class="grid-field" aria-hidden="true"></div>

	<main class="page-shell">
		<section id="hero" class="hero reveal" class:visible={visible.hero}>
			<div class="hero-copy">
				<p class="eyebrow">
					<span></span>
					Platform evolution in progress
				</p>
				<h1>Trading intelligence is getting a serious upgrade.</h1>
				<p class="lede">
					We are rebuilding the member experience around faster market data, sharper scanners, and a
					cleaner academy path for traders who want repeatable process instead of noise.
				</p>

				<div class="hero-actions">
					<a class="primary-link" href="#access">Get notified</a>
					<a class="ghost-link" href="#operations">View trading stack</a>
				</div>
			</div>

			<div class="command-deck" aria-label="Upgrade status overview">
				<div class="deck-topline">
					<span>Upgrade Command</span>
					<span class="live-status">Live</span>
				</div>
				<div class="deploy-ring">
					<div class="ring-core">
						<span>72%</span>
						<small>rollout</small>
					</div>
				</div>
				<div class="deploy-steps">
					{#each stageLines as line, index (line)}
						<div class="deploy-step" class:complete={index < 3}>
							<span>{String(index + 1).padStart(2, '0')}</span>
							<p>{line}</p>
						</div>
					{/each}
				</div>
			</div>
		</section>

		<section id="operations" class="ops-section reveal" class:visible={visible.operations}>
			<div class="section-heading ops-heading">
				<p class="section-kicker">Trading operations center</p>
				<h2>A market-native buildout for faster decisions, cleaner risk, and sharper education.</h2>
			</div>

			<div class="ops-stage">
				<div class="ops-reel" aria-label="Trading operations stack">
					{#each operationsChapters as chapter, index (chapter.number)}
						<article class="reel-card" style:--chapter-delay={`${index * 110}ms`}>
							<span>{chapter.number}</span>
							<h3>{chapter.title}</h3>
							<p>{chapter.copy}</p>
							<strong>{chapter.metric}</strong>
						</article>
					{/each}
				</div>

				<div class="systems-theater" aria-label="Trading systems status">
					<div class="theater-topline">
						<span>Trading stack</span>
						<strong>Market Ops Online</strong>
					</div>

					<div class="launch-core" aria-hidden="true">
						<span></span>
						<span></span>
						<span></span>
						<div>
							<strong>99.99%</strong>
							<small>target uptime</small>
						</div>
					</div>

					<div class="terminal-grid" aria-hidden="true">
						<span>equities.data: normalized</span>
						<span>options.greeks: mapped</span>
						<span>scanner.edge: ranked</span>
						<span>risk.limits: active</span>
						<span>academy.replay: indexed</span>
						<span>member.access: hardened</span>
					</div>
				</div>
			</div>
		</section>

		<section id="charts" class="charts-section reveal" class:visible={visible.charts}>
			<div class="section-heading">
				<p class="section-kicker">Market data lab</p>
				<h2>Charts, liquidity, and scanner data stay connected while the page moves.</h2>
			</div>

			<div class="chart-stage">
				<div class="chart-toolbar">
					<div>
						<span class="market-label">SPX upgrade feed</span>
						<strong class:up={mainDirection === 'up'} class:down={mainDirection === 'down'}>
							{latestMainPrice.toFixed(2)}
						</strong>
					</div>
					<div class="timeframe-control" aria-label="Chart timeframe">
						{#each timeframes as timeframe (timeframe)}
							<button
								type="button"
								class:active={activeTimeframe === timeframe}
								aria-pressed={activeTimeframe === timeframe}
								onclick={() => (activeTimeframe = timeframe)}
							>
								{timeframe}
							</button>
						{/each}
					</div>
				</div>

				<div class="chart-canvas" {@attach mountMainChart}></div>

				<div class="chart-metrics">
					<div>
						<span>Latency</span>
						<strong>15ms</strong>
					</div>
					<div>
						<span>Signals/min</span>
						<strong>4.2M</strong>
					</div>
					<div>
						<span>Confidence</span>
						<strong>{averageConfidence}%</strong>
					</div>
				</div>
			</div>

			<div class="mini-feed-grid">
				<div class="mini-feed">
					<div>
						<span>AAPL</span>
						<strong>${(chartData.aapl.at(-1)?.close ?? 232.3).toFixed(2)}</strong>
					</div>
					<div class="mini-chart" {@attach mountAaplChart}></div>
				</div>

				<div class="mini-feed">
					<div>
						<span>NVDA</span>
						<strong>${(chartData.nvda.at(-1)?.close ?? 1475.2).toFixed(2)}</strong>
					</div>
					<div class="mini-chart" {@attach mountNvdaChart}></div>
				</div>
			</div>
		</section>

		<section id="signals" class="signals-section reveal" class:visible={visible.signals}>
			<div class="section-heading">
				<p class="section-kicker">Scanner engine</p>
				<h2>Built for traders who need signal clarity under pressure.</h2>
			</div>

			<div class="signals-layout">
				<div class="radar-panel">
					<div class="radar" aria-hidden="true">
						<span class="sweep"></span>
						{#each [16, 29, 42, 58, 73, 87] as position, index (position)}
							<i style:--x={`${position}%`} style:--y={`${22 + ((index * 17) % 58)}%`}></i>
						{/each}
					</div>
					<div class="radar-stats">
						<div>
							<span>Universe</span>
							<strong>12,847</strong>
						</div>
						<div>
							<span>Active</span>
							<strong>{activeSignals}</strong>
						</div>
					</div>
				</div>

				<div class="signals-table">
					<div class="table-head">
						<span>Detected signals</span>
						<span>{averageConfidence}% avg confidence</span>
					</div>
					{#each scannerSignals as signal (signal.id)}
						<article class="signal-row" class:cooling={signal.status === 'cooling'}>
							<div>
								<strong>{signal.symbol}</strong>
								<span>{signal.type}</span>
							</div>
							<div>
								<span>${signal.price.toFixed(2)}</span>
								<small>{signal.age}s ago</small>
							</div>
							<div class="confidence">
								<span style:--confidence={`${signal.confidence}%`}></span>
								<small>{signal.confidence}%</small>
							</div>
						</article>
					{/each}
				</div>
			</div>
		</section>

		<section id="academy" class="academy-section reveal" class:visible={visible.academy}>
			<div class="section-heading">
				<p class="section-kicker">Trading university</p>
				<h2>
					Stocks, options, and institution-grade desk training in one professional curriculum path.
				</h2>
			</div>

			<div class="academy-cinema">
				<div class="academy-selector" role="tablist" aria-label="Trading university curriculum">
					{#each academyTracks as track (track.id)}
						<button
							type="button"
							role="tab"
							aria-selected={activeAcademyTrack === track.id}
							class:active={activeAcademyTrack === track.id}
							style:--track-accent={track.accent}
							onclick={() => (activeAcademyTrack = track.id)}
						>
							<span>{track.number}</span>
							<strong>{track.title}</strong>
							<small>Stocks + options</small>
						</button>
					{/each}
				</div>

				<div class="academy-feature" style:--track-accent={activeAcademy.accent}>
					<div class="academy-hero-card">
						<div class="academy-orbit" aria-hidden="true">
							<span></span>
							<span></span>
							<span></span>
						</div>
						<p class="track-number">{activeAcademy.number}</p>
						<h3>{activeAcademy.title}</h3>
						<p>{activeAcademy.tagline}</p>
						<div class="asset-lanes" aria-label="{activeAcademy.title} asset focus">
							<div>
								<strong>Stocks</strong>
								{#each activeAcademy.stockFocus as item (item)}
									<span>{item}</span>
								{/each}
							</div>
							<div>
								<strong>Options</strong>
								{#each activeAcademy.optionsFocus as item (item)}
									<span>{item}</span>
								{/each}
							</div>
						</div>
					</div>

					<div class="curriculum-panel">
						<div class="curriculum-head">
							<span>Institutional curriculum</span>
							<strong>Desk-ready sequence</strong>
						</div>
						<div class="curriculum-timeline">
							{#each activeAcademy.curriculum as module, index (module)}
								<article style:--module-delay={`${index * 90}ms`}>
									<span>{String(index + 1).padStart(2, '0')}</span>
									<p>{module}</p>
								</article>
							{/each}
						</div>
						<div class="academy-lab">
							<div>
								<span>Capstone lab</span>
								<p>{activeAcademy.lab}</p>
							</div>
							<div>
								<span>Desk outcome</span>
								<p>{activeAcademy.outcome}</p>
							</div>
						</div>
					</div>
				</div>
			</div>
		</section>

		<section
			id="infrastructure"
			class="infra-section reveal"
			class:visible={visible.infrastructure}
		>
			<div class="section-heading">
				<p class="section-kicker">Infrastructure</p>
				<h2>Capacity, reliability, and speed are moving up together.</h2>
			</div>

			<div class="infra-grid">
				<div class="node-list">
					{#each nodes as node (node.id)}
						<article class="node-row" class:migrating={node.status === 'migrating'}>
							<div>
								<strong>{node.id}</strong>
								<span>{node.region}</span>
							</div>
							<div class="load-track" aria-label={`${node.id} load ${node.load}%`}>
								<span style:--load={`${node.load}%`}></span>
							</div>
							<small>{node.status}</small>
						</article>
					{/each}
				</div>

				<div class="stat-board">
					<div>
						<strong>{formatNumber(stats.traders)}+</strong>
						<span>Traders trained</span>
					</div>
					<div>
						<strong>{formatNumber(stats.scanners)}</strong>
						<span>Symbols scanned</span>
					</div>
					<div>
						<strong>{stats.latency}ms</strong>
						<span>Target latency</span>
					</div>
					<div>
						<strong>{stats.uptime.toFixed(2)}%</strong>
						<span>Target uptime</span>
					</div>
				</div>
			</div>
		</section>

		<section id="access" class="access-section reveal" class:visible={visible.access}>
			{#if isSubmitted}
				<div class="success-panel" role="status">
					<div class="success-mark" aria-hidden="true">
						<svg viewBox="0 0 56 56">
							<circle cx="28" cy="28" r="25" />
							<path d="M16 29.5 24.5 38 41 19" />
						</svg>
					</div>
					<h2>You are on the list.</h2>
					<p>We will send the next platform update to your inbox as soon as the rollout opens.</p>
				</div>
			{:else}
				<form
					class="access-panel"
					onsubmit={(event) => {
						event.preventDefault();
						void handleNotifyMe();
					}}
				>
					<div>
						<p class="section-kicker">Early access</p>
						<h2>Get the rollout note when the platform reopens.</h2>
						<p>Receive the update for scanners, academy access, and the upgraded member room.</p>
					</div>

					<div class="email-row">
						<label class="sr-only" for="maintenance-email">Email address</label>
						<input
							id="maintenance-email"
							type="email"
							placeholder="you@example.com"
							bind:value={email}
							disabled={isSubmitting}
							autocomplete="email"
						/>
						<button type="submit" disabled={isSubmitting || !email}>
							{isSubmitting ? 'Joining...' : 'Notify me'}
						</button>
					</div>

					{#if errorMessage}
						<p class="form-error" role="alert">{errorMessage}</p>
					{/if}
				</form>
			{/if}
		</section>
	</main>
</div>

<style>
	:global(html:has(.maintenance-experience)),
	:global(body:has(.maintenance-experience)) {
		overflow: hidden;
		background: #06070a;
	}

	.maintenance-experience {
		--bg: #06070a;
		--panel: rgba(16, 18, 24, 0.74);
		--panel-strong: rgba(23, 26, 34, 0.9);
		--line: rgba(255, 255, 255, 0.1);
		--line-strong: rgba(248, 212, 119, 0.28);
		--text: #f7f2e8;
		--muted: rgba(247, 242, 232, 0.68);
		--dim: rgba(247, 242, 232, 0.46);
		--gold: #f8d477;
		--copper: #d98a52;
		--green: #16f2a9;
		--red: #ff5f6d;
		--cyan: #8bd3ff;
		position: fixed;
		inset: 0;
		z-index: 99999;
		overflow-x: hidden;
		overflow-y: auto;
		background:
			linear-gradient(180deg, rgba(255, 255, 255, 0.04), transparent 18rem),
			linear-gradient(135deg, #06070a 0%, #111015 42%, #090b0d 100%);
		color: var(--text);
		font-family:
			Inter,
			ui-sans-serif,
			system-ui,
			-apple-system,
			BlinkMacSystemFont,
			'Segoe UI',
			sans-serif;
	}

	.maintenance-experience::-webkit-scrollbar {
		width: 8px;
	}

	.maintenance-experience::-webkit-scrollbar-thumb {
		background: rgba(248, 212, 119, 0.44);
		border: 2px solid #06070a;
		border-radius: 999px;
	}

	.progress-rail {
		position: fixed;
		inset: 0 auto 0 0;
		z-index: 30;
		width: 3px;
		background: rgba(255, 255, 255, 0.08);
	}

	.progress-rail span {
		display: block;
		width: 100%;
		height: calc(var(--scroll-progress) * 100%);
		background: linear-gradient(180deg, var(--gold), var(--green), var(--cyan));
		box-shadow: 0 0 22px rgba(22, 242, 169, 0.48);
	}

	.cinematic-canvas {
		position: fixed;
		inset: 0;
		z-index: 0;
		pointer-events: none;
		opacity: 0.54;
		mix-blend-mode: screen;
	}

	.market-tape {
		position: sticky;
		top: 0;
		z-index: 20;
		overflow: hidden;
		border-bottom: 1px solid var(--line);
		background: rgba(6, 7, 10, 0.82);
		backdrop-filter: blur(18px);
	}

	.tape-track {
		display: flex;
		width: max-content;
		gap: 10px;
		padding: 10px 0;
		animation: tape 38s linear infinite;
	}

	.ticker-pill {
		display: inline-flex;
		align-items: center;
		gap: 9px;
		min-width: max-content;
		border: 1px solid rgba(255, 255, 255, 0.08);
		border-radius: 999px;
		padding: 6px 12px;
		color: var(--muted);
		font-family: 'SF Mono', ui-monospace, Menlo, monospace;
		font-size: 12px;
		background: rgba(255, 255, 255, 0.04);
	}

	.ticker-pill.up span:last-child {
		color: var(--green);
	}

	.ticker-pill.down span:last-child {
		color: var(--red);
	}

	.ticker-symbol {
		color: var(--text);
		font-weight: 800;
	}

	.cinema-light {
		position: fixed;
		inset: 0;
		z-index: 0;
		pointer-events: none;
		background:
			radial-gradient(
				900px 420px at var(--glare-x) var(--glare-y),
				rgba(248, 212, 119, 0.12),
				transparent 64%
			),
			linear-gradient(90deg, rgba(139, 211, 255, 0.05), transparent 34%, rgba(22, 242, 169, 0.06));
	}

	.grid-field {
		position: fixed;
		inset: 0;
		z-index: 0;
		pointer-events: none;
		opacity: 0.34;
		background-image:
			linear-gradient(rgba(255, 255, 255, 0.05) 1px, transparent 1px),
			linear-gradient(90deg, rgba(255, 255, 255, 0.05) 1px, transparent 1px);
		background-size: 68px 68px;
		mask-image: linear-gradient(to bottom, transparent 0%, black 18%, black 72%, transparent 100%);
	}

	.page-shell {
		position: relative;
		z-index: 2;
		width: min(1180px, calc(100% - 36px));
		margin: 0 auto;
		padding: clamp(42px, 8vw, 96px) 0 72px;
	}

	.hero {
		display: grid;
		grid-template-columns: minmax(0, 1.08fr) minmax(330px, 0.62fr);
		gap: clamp(24px, 5vw, 70px);
		align-items: center;
		min-height: min(820px, calc(100vh - 48px));
		padding-bottom: 8vh;
	}

	.hero-copy {
		max-width: 760px;
	}

	.eyebrow,
	.section-kicker {
		display: inline-flex;
		align-items: center;
		gap: 10px;
		margin: 0 0 18px;
		color: var(--gold);
		font-size: 12px;
		font-weight: 800;
		letter-spacing: 0.14em;
		text-transform: uppercase;
	}

	.eyebrow span {
		width: 8px;
		height: 8px;
		border-radius: 999px;
		background: var(--green);
		box-shadow: 0 0 0 8px rgba(22, 242, 169, 0.1);
	}

	h1,
	h2,
	h3,
	p {
		letter-spacing: 0;
	}

	h1 {
		max-width: 840px;
		margin: 0;
		font-size: clamp(3.25rem, 8vw, 7.4rem);
		line-height: 0.89;
		font-weight: 900;
		text-wrap: balance;
	}

	h2 {
		margin: 0;
		font-size: clamp(2rem, 4.6vw, 4.8rem);
		line-height: 0.95;
		font-weight: 880;
		text-wrap: balance;
	}

	.lede {
		max-width: 660px;
		margin: 28px 0 0;
		color: var(--muted);
		font-size: clamp(1.05rem, 1.5vw, 1.28rem);
		line-height: 1.72;
	}

	.hero-actions {
		display: flex;
		flex-wrap: wrap;
		gap: 12px;
		margin-top: 34px;
	}

	.primary-link,
	.ghost-link,
	.email-row button {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		min-height: 46px;
		border-radius: 8px;
		padding: 0 18px;
		font-weight: 800;
		text-decoration: none;
		transition:
			transform 180ms ease,
			border-color 180ms ease,
			background 180ms ease;
	}

	.primary-link,
	.email-row button {
		border: 1px solid rgba(248, 212, 119, 0.8);
		background: linear-gradient(135deg, var(--gold), var(--copper));
		color: #161008;
		box-shadow: 0 18px 54px rgba(248, 212, 119, 0.18);
	}

	.ghost-link {
		border: 1px solid var(--line);
		background: rgba(255, 255, 255, 0.04);
		color: var(--text);
	}

	.primary-link:hover,
	.ghost-link:hover,
	.email-row button:hover {
		transform: translateY(-2px);
	}

	.command-deck,
	.chart-stage,
	.mini-feed,
	.radar-panel,
	.signals-table,
	.node-list,
	.stat-board,
	.access-panel,
	.success-panel {
		border: 1px solid var(--line);
		border-radius: 8px;
		background: var(--panel);
		box-shadow: 0 24px 80px rgba(0, 0, 0, 0.34);
		backdrop-filter: blur(24px);
	}

	.command-deck {
		padding: 18px;
	}

	.deck-topline,
	.chart-toolbar,
	.table-head {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 14px;
		color: var(--dim);
		font-size: 12px;
		font-weight: 800;
		text-transform: uppercase;
	}

	.live-status {
		color: var(--green);
	}

	.deploy-ring {
		display: grid;
		place-items: center;
		width: min(260px, 74vw);
		aspect-ratio: 1;
		margin: 26px auto;
		border-radius: 50%;
		background:
			conic-gradient(var(--green) 0 72%, rgba(255, 255, 255, 0.1) 72% 100%),
			radial-gradient(circle, rgba(248, 212, 119, 0.08), transparent 58%);
		animation: ring-breathe 3.4s ease-in-out infinite;
	}

	.ring-core {
		display: grid;
		place-items: center;
		width: 72%;
		aspect-ratio: 1;
		border-radius: 50%;
		background: #08090d;
		border: 1px solid var(--line);
	}

	.ring-core span {
		font-size: clamp(2.4rem, 5vw, 4rem);
		font-weight: 900;
	}

	.ring-core small {
		color: var(--muted);
		text-transform: uppercase;
	}

	.deploy-steps {
		display: grid;
		gap: 8px;
	}

	.deploy-step {
		display: grid;
		grid-template-columns: 38px 1fr;
		gap: 12px;
		align-items: center;
		padding: 12px;
		border: 1px solid var(--line);
		border-radius: 8px;
		background: rgba(255, 255, 255, 0.035);
	}

	.deploy-step span {
		color: var(--dim);
		font-family: 'SF Mono', ui-monospace, Menlo, monospace;
	}

	.deploy-step p {
		margin: 0;
		color: var(--muted);
	}

	.deploy-step.complete {
		border-color: rgba(22, 242, 169, 0.24);
	}

	.deploy-step.complete span,
	.deploy-step.complete p {
		color: var(--green);
	}

	.reveal {
		opacity: 0;
		transform: translateY(42px);
		transition:
			opacity 820ms cubic-bezier(0.16, 1, 0.3, 1),
			transform 820ms cubic-bezier(0.16, 1, 0.3, 1);
	}

	.reveal.visible {
		opacity: 1;
		transform: translateY(0);
	}

	.ops-section,
	.charts-section,
	.signals-section,
	.academy-section,
	.infra-section,
	.access-section {
		padding: clamp(52px, 9vw, 110px) 0;
	}

	.section-heading {
		max-width: 780px;
		margin-bottom: 24px;
	}

	.ops-section {
		padding-top: clamp(24px, 4vw, 58px);
	}

	.ops-heading {
		max-width: 920px;
	}

	.ops-stage {
		display: grid;
		grid-template-columns: minmax(0, 1.03fr) minmax(340px, 0.72fr);
		gap: 16px;
		align-items: stretch;
	}

	.ops-reel {
		display: grid;
		grid-template-columns: repeat(2, minmax(0, 1fr));
		gap: 12px;
	}

	.reel-card,
	.systems-theater {
		position: relative;
		overflow: hidden;
		border: 1px solid var(--line);
		border-radius: 8px;
		background:
			linear-gradient(135deg, rgba(22, 242, 169, 0.075), transparent 50%),
			linear-gradient(180deg, rgba(255, 255, 255, 0.045), transparent 42%), rgba(12, 15, 20, 0.82);
		box-shadow: 0 28px 90px rgba(0, 0, 0, 0.36);
		backdrop-filter: blur(28px);
	}

	.reel-card {
		min-height: 242px;
		display: grid;
		align-content: space-between;
		gap: 18px;
		padding: 18px;
		animation: reel-rise 760ms cubic-bezier(0.16, 1, 0.3, 1) both;
		animation-delay: var(--chapter-delay);
	}

	.reel-card::before {
		content: '';
		position: absolute;
		inset: 0;
		background:
			linear-gradient(100deg, transparent 0 34%, rgba(255, 255, 255, 0.075) 47%, transparent 62%),
			linear-gradient(180deg, rgba(248, 212, 119, 0.045), transparent);
		opacity: 0.78;
		transform: translateX(-120%);
		animation: projector-sweep 5.8s ease-in-out infinite;
		animation-delay: calc(var(--chapter-delay) + 700ms);
	}

	.reel-card > * {
		position: relative;
		z-index: 1;
	}

	.reel-card > span {
		width: max-content;
		border: 1px solid rgba(22, 242, 169, 0.24);
		border-radius: 999px;
		padding: 5px 10px;
		color: var(--green);
		font-family: 'SF Mono', ui-monospace, Menlo, monospace;
		font-size: 12px;
		font-weight: 900;
	}

	.reel-card h3 {
		max-width: 360px;
		margin: 0;
		font-size: clamp(1.45rem, 2.8vw, 2.35rem);
		line-height: 0.98;
	}

	.reel-card p {
		margin: 0;
		color: var(--muted);
		line-height: 1.6;
	}

	.reel-card strong {
		color: var(--gold);
		font-family: 'SF Mono', ui-monospace, Menlo, monospace;
		text-transform: uppercase;
	}

	.systems-theater {
		min-height: 496px;
		display: grid;
		align-content: space-between;
		padding: 18px;
		background:
			radial-gradient(circle at 50% 42%, rgba(22, 242, 169, 0.12), transparent 40%),
			linear-gradient(135deg, rgba(248, 212, 119, 0.06), transparent 46%), rgba(8, 10, 14, 0.88);
	}

	.theater-topline {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 14px;
		color: var(--dim);
		font-size: 12px;
		font-weight: 900;
		text-transform: uppercase;
	}

	.theater-topline strong {
		color: var(--green);
	}

	.launch-core {
		position: relative;
		display: grid;
		place-items: center;
		overflow: hidden;
		width: min(320px, 72vw);
		aspect-ratio: 1;
		margin: 18px auto;
		border-radius: 50%;
		background:
			radial-gradient(circle, rgba(255, 255, 255, 0.12), transparent 58%), rgba(7, 9, 13, 0.92);
	}

	.launch-core::before {
		content: '';
		position: absolute;
		inset: 0;
		border-radius: inherit;
		background: conic-gradient(from 180deg, var(--green), var(--cyan), var(--gold), var(--green));
		animation: launch-rotate 18s linear infinite;
	}

	.launch-core > span {
		position: absolute;
		z-index: 1;
		inset: calc(var(--ring) * 1px);
		border: 1px solid rgba(255, 255, 255, 0.18);
		border-radius: 50%;
		animation: launch-pulse 2.8s ease-in-out infinite;
	}

	.launch-core > span:nth-child(1) {
		--ring: 24;
	}

	.launch-core > span:nth-child(2) {
		--ring: 54;
		animation-delay: 360ms;
	}

	.launch-core > span:nth-child(3) {
		--ring: 84;
		animation-delay: 720ms;
	}

	.launch-core div {
		position: relative;
		z-index: 1;
		display: grid;
		place-items: center;
		width: 58%;
		aspect-ratio: 1;
		border: 1px solid rgba(255, 255, 255, 0.12);
		border-radius: 50%;
		background: #07090d;
	}

	.launch-core strong {
		font-family: 'SF Mono', ui-monospace, Menlo, monospace;
		font-size: clamp(2rem, 4vw, 3.4rem);
	}

	.launch-core small {
		color: var(--muted);
		text-transform: uppercase;
	}

	.terminal-grid {
		display: grid;
		gap: 8px;
	}

	.terminal-grid span {
		border: 1px solid rgba(139, 211, 255, 0.12);
		border-radius: 8px;
		padding: 10px 12px;
		color: rgba(139, 211, 255, 0.78);
		font-family: 'SF Mono', ui-monospace, Menlo, monospace;
		font-size: 12px;
		background: rgba(0, 0, 0, 0.22);
		animation: terminal-glow 3.2s ease-in-out infinite;
	}

	.terminal-grid span:nth-child(2n) {
		color: rgba(22, 242, 169, 0.82);
		animation-delay: 480ms;
	}

	.chart-stage {
		overflow: hidden;
		padding: 18px;
		border-color: rgba(22, 242, 169, 0.22);
		background:
			linear-gradient(180deg, rgba(22, 242, 169, 0.07), transparent 34%), var(--panel-strong);
	}

	.market-label {
		display: block;
		margin-bottom: 5px;
		color: var(--dim);
		font-size: 11px;
		text-transform: uppercase;
	}

	.chart-toolbar strong {
		font-family: 'SF Mono', ui-monospace, Menlo, monospace;
		font-size: clamp(1.4rem, 3vw, 2.4rem);
	}

	.up {
		color: var(--green);
	}

	.down {
		color: var(--red);
	}

	.timeframe-control {
		display: flex;
		gap: 6px;
		padding: 4px;
		border: 1px solid var(--line);
		border-radius: 8px;
		background: rgba(255, 255, 255, 0.04);
	}

	.timeframe-control button {
		width: 42px;
		height: 32px;
		border: 0;
		border-radius: 6px;
		background: transparent;
		color: var(--muted);
		font: inherit;
		cursor: pointer;
	}

	.timeframe-control button.active {
		background: rgba(22, 242, 169, 0.13);
		color: var(--green);
	}

	.chart-canvas {
		height: clamp(360px, 54vh, 560px);
		margin-top: 14px;
		border-radius: 8px;
		background: linear-gradient(180deg, rgba(255, 255, 255, 0.04), transparent), rgba(0, 0, 0, 0.2);
	}

	.chart-metrics,
	.radar-stats,
	.stat-board {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 10px;
		margin-top: 14px;
	}

	.chart-metrics div,
	.radar-stats div,
	.stat-board div {
		border: 1px solid var(--line);
		border-radius: 8px;
		padding: 14px;
		background: rgba(255, 255, 255, 0.035);
	}

	.chart-metrics span,
	.radar-stats span,
	.stat-board span,
	.node-row span {
		display: block;
		color: var(--dim);
		font-size: 12px;
	}

	.chart-metrics strong,
	.radar-stats strong,
	.stat-board strong {
		display: block;
		margin-top: 4px;
		color: var(--text);
		font-family: 'SF Mono', ui-monospace, Menlo, monospace;
		font-size: clamp(1.2rem, 2.2vw, 2rem);
	}

	.mini-feed-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 14px;
		margin-top: 14px;
	}

	.mini-feed {
		padding: 16px;
	}

	.mini-feed > div:first-child {
		display: flex;
		justify-content: space-between;
		gap: 14px;
		margin-bottom: 10px;
	}

	.mini-feed span {
		color: var(--dim);
		font-size: 12px;
		font-weight: 800;
	}

	.mini-feed strong {
		font-family: 'SF Mono', ui-monospace, Menlo, monospace;
	}

	.mini-chart {
		height: 150px;
	}

	.signals-layout,
	.infra-grid,
	.academy-feature {
		display: grid;
		grid-template-columns: minmax(0, 0.86fr) minmax(0, 1.14fr);
		gap: 16px;
	}

	.radar-panel,
	.signals-table,
	.node-list,
	.stat-board,
	.academy-hero-card,
	.curriculum-panel {
		padding: 18px;
	}

	.radar {
		position: relative;
		width: min(330px, 72vw);
		aspect-ratio: 1;
		margin: 8px auto 18px;
		overflow: hidden;
		border-radius: 50%;
		border: 1px solid rgba(22, 242, 169, 0.22);
		background:
			repeating-radial-gradient(circle, transparent 0 42px, rgba(22, 242, 169, 0.13) 43px 44px),
			linear-gradient(rgba(22, 242, 169, 0.1), rgba(139, 211, 255, 0.04));
	}

	.sweep {
		position: absolute;
		inset: 0;
		background: conic-gradient(from 0deg, rgba(22, 242, 169, 0.34), transparent 72deg);
		animation: rotate 3.8s linear infinite;
	}

	.radar i {
		position: absolute;
		top: var(--y);
		left: var(--x);
		width: 7px;
		height: 7px;
		border-radius: 999px;
		background: var(--gold);
		box-shadow: 0 0 16px rgba(248, 212, 119, 0.86);
	}

	.signals-table {
		display: grid;
		align-content: start;
		gap: 8px;
	}

	.signal-row {
		display: grid;
		grid-template-columns: minmax(0, 1fr) 112px 116px;
		gap: 12px;
		align-items: center;
		border: 1px solid var(--line);
		border-radius: 8px;
		padding: 12px;
		background: rgba(255, 255, 255, 0.035);
		animation: row-in 420ms cubic-bezier(0.16, 1, 0.3, 1);
	}

	.signal-row.cooling {
		opacity: 0.62;
	}

	.signal-row strong {
		display: block;
		color: var(--gold);
		font-family: 'SF Mono', ui-monospace, Menlo, monospace;
	}

	.signal-row span,
	.signal-row small {
		color: var(--muted);
	}

	.confidence {
		display: grid;
		gap: 6px;
	}

	.confidence > span {
		height: 5px;
		overflow: hidden;
		border-radius: 999px;
		background: rgba(255, 255, 255, 0.1);
	}

	.confidence > span::before {
		content: '';
		display: block;
		width: var(--confidence);
		height: 100%;
		border-radius: inherit;
		background: linear-gradient(90deg, var(--green), var(--gold));
	}

	.academy-cinema {
		display: grid;
		gap: 16px;
	}

	.academy-selector {
		display: grid;
		grid-template-columns: repeat(2, minmax(0, 1fr));
		gap: 12px;
	}

	.academy-selector button {
		position: relative;
		overflow: hidden;
		border: 1px solid var(--line);
		border-radius: 8px;
		padding: 16px;
		background: rgba(255, 255, 255, 0.035);
		color: var(--text);
		text-align: left;
		cursor: pointer;
		transition:
			transform 220ms ease,
			border-color 220ms ease,
			background 220ms ease;
	}

	.academy-selector button::before {
		content: '';
		position: absolute;
		inset: 0;
		opacity: 0;
		background:
			linear-gradient(
				135deg,
				color-mix(in srgb, var(--track-accent), transparent 82%),
				transparent
			),
			radial-gradient(
				circle at 88% 18%,
				color-mix(in srgb, var(--track-accent), transparent 72%),
				transparent 34%
			);
		transition: opacity 220ms ease;
	}

	.academy-selector button:hover,
	.academy-selector button.active {
		transform: translateY(-2px);
		border-color: color-mix(in srgb, var(--track-accent), transparent 48%);
		background: rgba(255, 255, 255, 0.06);
	}

	.academy-selector button.active::before {
		opacity: 1;
	}

	.academy-selector button > * {
		position: relative;
		z-index: 1;
		display: block;
	}

	.academy-selector button > span {
		color: color-mix(in srgb, var(--track-accent), white 10%);
		font-family: 'SF Mono', ui-monospace, Menlo, monospace;
		font-size: 13px;
	}

	.academy-selector button strong {
		margin-top: 8px;
		font-size: clamp(1.1rem, 2vw, 1.45rem);
	}

	.academy-selector button small {
		margin-top: 6px;
		color: var(--muted);
		text-transform: uppercase;
		letter-spacing: 0.08em;
	}

	.academy-feature {
		align-items: stretch;
	}

	.academy-hero-card,
	.curriculum-panel {
		position: relative;
		overflow: hidden;
		border: 1px solid var(--line);
		border-radius: 8px;
		background:
			linear-gradient(
				135deg,
				color-mix(in srgb, var(--track-accent), transparent 90%),
				transparent 52%
			),
			var(--panel);
		box-shadow: 0 24px 80px rgba(0, 0, 0, 0.34);
		backdrop-filter: blur(24px);
	}

	.academy-hero-card {
		min-height: 510px;
		display: flex;
		flex-direction: column;
		justify-content: space-between;
	}

	.academy-orbit {
		position: absolute;
		inset: 18px 18px auto auto;
		width: 172px;
		aspect-ratio: 1;
		border: 1px solid color-mix(in srgb, var(--track-accent), transparent 54%);
		border-radius: 50%;
		opacity: 0.54;
		animation: orbit-float 6s ease-in-out infinite;
	}

	.academy-orbit::before,
	.academy-orbit::after,
	.academy-orbit span {
		content: '';
		position: absolute;
		border-radius: 50%;
	}

	.academy-orbit::before {
		inset: 28px;
		border: 1px solid rgba(255, 255, 255, 0.14);
	}

	.academy-orbit::after {
		inset: 64px;
		background: color-mix(in srgb, var(--track-accent), transparent 24%);
		box-shadow: 0 0 34px color-mix(in srgb, var(--track-accent), transparent 42%);
	}

	.academy-orbit span {
		width: 9px;
		height: 9px;
		background: var(--text);
		box-shadow: 0 0 22px var(--track-accent);
		animation: academy-pulse 1.8s ease-in-out infinite;
	}

	.academy-orbit span:nth-child(1) {
		top: 18px;
		left: 54px;
	}

	.academy-orbit span:nth-child(2) {
		right: 24px;
		top: 84px;
		animation-delay: 420ms;
	}

	.academy-orbit span:nth-child(3) {
		bottom: 28px;
		left: 44px;
		animation-delay: 860ms;
	}

	.track-number {
		margin: 0;
		color: color-mix(in srgb, var(--track-accent), transparent 48%);
		font-size: clamp(5rem, 13vw, 9rem);
		font-weight: 900;
		line-height: 0.76;
	}

	.academy-hero-card h3 {
		max-width: 520px;
		margin: 30px 0 12px;
		font-size: clamp(2rem, 4vw, 3.6rem);
		line-height: 0.96;
	}

	.academy-hero-card p,
	.curriculum-panel p,
	.academy-lab p {
		color: var(--muted);
		line-height: 1.68;
	}

	.asset-lanes {
		display: grid;
		grid-template-columns: repeat(2, minmax(0, 1fr));
		gap: 10px;
		margin-top: 28px;
	}

	.asset-lanes div {
		display: grid;
		gap: 8px;
		border: 1px solid var(--line);
		border-radius: 8px;
		padding: 12px;
		background: rgba(0, 0, 0, 0.18);
	}

	.asset-lanes strong,
	.curriculum-head span,
	.academy-lab span {
		color: color-mix(in srgb, var(--track-accent), white 10%);
		font-size: 12px;
		font-weight: 900;
		letter-spacing: 0.1em;
		text-transform: uppercase;
	}

	.asset-lanes span {
		color: var(--text);
		font-size: 13px;
	}

	.curriculum-panel {
		display: grid;
		align-content: start;
		gap: 16px;
	}

	.curriculum-head {
		display: flex;
		align-items: end;
		justify-content: space-between;
		gap: 12px;
		padding-bottom: 16px;
		border-bottom: 1px solid var(--line);
	}

	.curriculum-head strong {
		color: var(--text);
		font-family: 'SF Mono', ui-monospace, Menlo, monospace;
		font-size: clamp(1rem, 2vw, 1.5rem);
	}

	.curriculum-timeline {
		display: grid;
		gap: 10px;
	}

	.curriculum-timeline article {
		display: grid;
		grid-template-columns: 52px minmax(0, 1fr);
		gap: 12px;
		align-items: start;
		border: 1px solid var(--line);
		border-radius: 8px;
		padding: 14px;
		background: rgba(255, 255, 255, 0.035);
		animation: curriculum-rise 540ms cubic-bezier(0.16, 1, 0.3, 1) both;
		animation-delay: var(--module-delay);
	}

	.curriculum-timeline article > span {
		display: grid;
		place-items: center;
		width: 38px;
		aspect-ratio: 1;
		border-radius: 50%;
		background: color-mix(in srgb, var(--track-accent), transparent 86%);
		color: var(--track-accent);
		font-family: 'SF Mono', ui-monospace, Menlo, monospace;
		font-size: 12px;
		font-weight: 900;
	}

	.curriculum-timeline p {
		margin: 0;
	}

	.academy-lab {
		display: grid;
		grid-template-columns: repeat(2, minmax(0, 1fr));
		gap: 10px;
		margin-top: 2px;
	}

	.academy-lab div {
		border: 1px solid color-mix(in srgb, var(--track-accent), transparent 76%);
		border-radius: 8px;
		padding: 14px;
		background: color-mix(in srgb, var(--track-accent), transparent 93%);
	}

	.node-list {
		display: grid;
		gap: 10px;
	}

	.node-row {
		display: grid;
		grid-template-columns: 110px 1fr 78px;
		gap: 12px;
		align-items: center;
		border: 1px solid var(--line);
		border-radius: 8px;
		padding: 12px;
		background: rgba(255, 255, 255, 0.035);
	}

	.node-row strong {
		display: block;
		font-family: 'SF Mono', ui-monospace, Menlo, monospace;
	}

	.node-row small {
		color: var(--green);
		text-transform: uppercase;
	}

	.node-row.migrating small {
		color: var(--gold);
	}

	.load-track {
		height: 8px;
		overflow: hidden;
		border-radius: 999px;
		background: rgba(255, 255, 255, 0.1);
	}

	.load-track span {
		display: block;
		width: var(--load);
		height: 100%;
		border-radius: inherit;
		background: linear-gradient(90deg, var(--cyan), var(--green), var(--gold));
		transition: width 520ms ease;
	}

	.stat-board {
		grid-template-columns: repeat(2, 1fr);
		margin-top: 0;
	}

	.access-section {
		padding-bottom: 12vh;
	}

	.access-panel,
	.success-panel {
		display: grid;
		grid-template-columns: minmax(0, 1fr) minmax(320px, 0.82fr);
		gap: 24px;
		align-items: end;
		padding: clamp(18px, 4vw, 34px);
		border-color: var(--line-strong);
	}

	.access-panel h2,
	.success-panel h2 {
		max-width: 740px;
	}

	.access-panel p,
	.success-panel p {
		max-width: 620px;
		color: var(--muted);
		line-height: 1.7;
	}

	.email-row {
		display: grid;
		grid-template-columns: minmax(0, 1fr) auto;
		gap: 10px;
		align-items: center;
	}

	.email-row input {
		width: 100%;
		min-height: 54px;
		border: 1px solid var(--line);
		border-radius: 8px;
		padding: 0 16px;
		background: rgba(0, 0, 0, 0.26);
		color: var(--text);
		font: inherit;
		outline: none;
	}

	.email-row input:focus {
		border-color: var(--gold);
		box-shadow: 0 0 0 3px rgba(248, 212, 119, 0.14);
	}

	.email-row button {
		min-height: 54px;
		cursor: pointer;
	}

	.email-row button:disabled,
	.email-row input:disabled {
		cursor: not-allowed;
		opacity: 0.55;
	}

	.form-error {
		grid-column: 2;
		margin: 8px 0 0;
		color: var(--red) !important;
		font-size: 14px;
	}

	.success-panel {
		grid-template-columns: 80px minmax(0, 1fr);
		align-items: center;
	}

	.success-mark svg {
		width: 68px;
		height: 68px;
	}

	.success-mark circle,
	.success-mark path {
		fill: none;
		stroke: var(--green);
		stroke-width: 3;
	}

	.sr-only {
		position: absolute;
		width: 1px;
		height: 1px;
		overflow: hidden;
		clip: rect(0, 0, 0, 0);
		white-space: nowrap;
	}

	@keyframes tape {
		to {
			transform: translateX(-33.33%);
		}
	}

	@keyframes ring-breathe {
		50% {
			filter: saturate(1.25) brightness(1.08);
			transform: scale(1.015);
		}
	}

	@keyframes rotate {
		to {
			transform: rotate(360deg);
		}
	}

	@keyframes row-in {
		from {
			opacity: 0;
			transform: translateY(12px);
		}
	}

	@keyframes reel-rise {
		from {
			opacity: 0;
			transform: translate3d(0, 28px, 0) scale(0.985);
		}
	}

	@keyframes projector-sweep {
		45%,
		100% {
			transform: translateX(120%);
		}
	}

	@keyframes launch-rotate {
		to {
			transform: rotate(360deg);
		}
	}

	@keyframes launch-pulse {
		50% {
			opacity: 0.28;
			transform: scale(1.025);
		}
	}

	@keyframes terminal-glow {
		50% {
			border-color: rgba(22, 242, 169, 0.32);
			box-shadow: 0 0 28px rgba(22, 242, 169, 0.08);
		}
	}

	@keyframes academy-pulse {
		50% {
			transform: scale(1.45);
			opacity: 0.56;
		}
	}

	@keyframes orbit-float {
		50% {
			transform: translate3d(-8px, 10px, 0) rotate(8deg);
		}
	}

	@keyframes curriculum-rise {
		from {
			opacity: 0;
			transform: translateY(14px) scale(0.985);
		}
	}

	@media (max-width: 960px) {
		.hero,
		.ops-stage,
		.signals-layout,
		.infra-grid,
		.academy-feature,
		.access-panel {
			grid-template-columns: 1fr;
		}

		.hero {
			min-height: auto;
			padding-top: 56px;
		}
	}

	@media (max-width: 720px) {
		.page-shell {
			width: min(100% - 24px, 1180px);
		}

		h1 {
			font-size: clamp(3rem, 16vw, 4.5rem);
		}

		.chart-toolbar,
		.email-row,
		.ops-reel,
		.mini-feed-grid,
		.academy-selector,
		.asset-lanes,
		.academy-lab,
		.chart-metrics,
		.radar-stats,
		.stat-board {
			grid-template-columns: 1fr;
		}

		.chart-toolbar,
		.email-row {
			display: grid;
		}

		.timeframe-control {
			width: 100%;
			justify-content: space-between;
		}

		.timeframe-control button {
			width: 100%;
		}

		.signal-row,
		.node-row {
			grid-template-columns: 1fr;
		}

		.chart-canvas {
			height: 340px;
		}

		.systems-theater {
			min-height: 420px;
		}

		.form-error {
			grid-column: 1;
		}
	}

	@media (prefers-reduced-motion: reduce) {
		*,
		*::before,
		*::after {
			animation-duration: 0.001ms !important;
			animation-iteration-count: 1 !important;
			scroll-behavior: auto !important;
			transition-duration: 0.001ms !important;
		}
	}
</style>

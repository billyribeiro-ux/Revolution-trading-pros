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
		| 'note'
		| 'operations'
		| 'charts'
		| 'signals'
		| 'academy'
		| 'infrastructure'
		| 'buildlog'
		| 'access';
	type ChartKey = 'main' | 'aapl' | 'nvda';
	type SignalStatus = 'active' | 'watch' | 'cooling';
	type AcademyTrackId = 'day' | 'swing';
	type GsapCore = typeof import('gsap').gsap;
	type ScrollTriggerStatic = typeof import('gsap/ScrollTrigger').ScrollTrigger;

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
	}

	interface OperationsChapter {
		number: string;
		title: string;
		copy: string;
		metric: string;
	}

	interface BuildLogEntry {
		date: string;
		tag: string;
		text: string;
	}

	const timeframes = ['1m', '5m', '15m', '1H'];
	const revealSections: SectionId[] = [
		'hero',
		'note',
		'operations',
		'charts',
		'signals',
		'academy',
		'infrastructure',
		'buildlog',
		'access'
	];

	/* ── Date-driven content ─────────────────────────────────────────────────
	 * Progress, the desk note, and the build log all rotate with the calendar,
	 * so the page changes every trading day without a backend.
	 */
	const BUILD_EPOCH_UTC = Date.UTC(2026, 5, 1); // day 1 of the rebuild
	const DAY_MS = 86_400_000;

	const deskNotes = [
		'Amateurs open the day with a prediction. Professionals open it with a plan for every scenario.',
		'Your edge is not the setup. It is the discipline to only trade the setup.',
		'Risk a fixed fraction, every time. Streaks end; sizing rules should not.',
		'The market pays you to wait and charges you to act. Make every action worth the fee.',
		'One clean trade, reviewed honestly, teaches more than fifty impulsive ones.',
		'Liquidity is oxygen. Trade where it is deep, or do not trade at all.',
		'If you cannot write the invalidation down before entry, you do not have a trade — you have a hope.',
		'Volatility is not risk. Position size without a plan is risk.',
		'The open is an auction, not a starting gun. Let it find balance before you commit.',
		'Every chart tells two stories. The tape tells you which one is true.',
		'Scaling out is a decision you make before the trade, not a feeling you have during it.',
		'Your worst drawdowns follow your best streaks. Confidence is a position — manage it.',
		'News moves price for minutes. Positioning moves it for days. Trade the second one.',
		'A red day executed to plan beats a green day that broke your rules.',
		'Options give you leverage on being right — and a clock that punishes being early.',
		'Theta does not care about your thesis. Respect the calendar as much as the chart.',
		'Relative strength at the lows shows you what is being accumulated. Watch what refuses to go down.',
		'The journal is the only indicator with a perfect hit rate on your own behavior.',
		'Process goals are the only goals you control. P&L is a byproduct, not a target.',
		'Big money leaves footprints: volume, repeated levels, and time. Learn to read all three.',
		'Cut the loser at your level, not at your pain threshold.',
		'The best trade of the week is often the one you did not take.',
		'Gap-and-go and gap-and-fade look identical at 9:31. The first thirty minutes of volume decides which one you are in.',
		'Consistency compounds. Intensity burns out. Build the boring routine that survives.'
	];

	const buildLogPool: Array<Omit<BuildLogEntry, 'date'>> = [
		{
			tag: 'Shipped',
			text: 'New scanner ranking engine online — relative volume, float rotation, and liquidity sweeps now score in real time.'
		},
		{
			tag: 'Hardened',
			text: 'Order-flow feed moved to redundant routes across NY4 and CH2 — zero dropped ticks in stress tests.'
		},
		{
			tag: 'Shipped',
			text: 'Day Trading University replay lab indexed — every session searchable by setup, symbol, and outcome.'
		},
		{
			tag: 'Tuned',
			text: 'Chart engine latency cut 38% — candles, VWAP, and volume profile now render in a single pass.'
		},
		{
			tag: 'Shipped',
			text: 'Options risk desk live internally — Greeks, IV rank, and 0DTE exposure in one panel.'
		},
		{
			tag: 'Tuned',
			text: 'Alert pipeline re-scored — the confidence model now weighs sector leadership and market regime.'
		},
		{
			tag: 'Hardened',
			text: 'Member room authentication rebuilt — faster sign-in with stricter session controls.'
		},
		{
			tag: 'Shipped',
			text: 'Swing Trading University portfolio simulator wired to live volatility surfaces.'
		},
		{
			tag: 'Tuned',
			text: 'Watchlist sync brought down to sub-second across devices.'
		},
		{
			tag: 'Hardened',
			text: 'Full failover drill passed — the platform survives a data-center loss without dropping a session.'
		},
		{
			tag: 'Shipped',
			text: 'Dark-pool repeat detection added to the scanner edge engine.'
		},
		{
			tag: 'Tuned',
			text: 'Mobile charts rebuilt for one-handed review — faster pan, cleaner drawing tools.'
		},
		{
			tag: 'Shipped',
			text: 'Trade journal templates added to both universities — entry, thesis, risk, review.'
		},
		{
			tag: 'Tuned',
			text: 'Market data normalization pass complete — equities and options now share one clock.'
		},
		{
			tag: 'Hardened',
			text: 'Rate limits and abuse controls tuned so alert bursts never queue.'
		},
		{
			tag: 'Shipped',
			text: 'Live-room stage rebuilt — screen share, tape, and chart sync for market-open sessions.'
		}
	];

	const fallbackBuildLog: BuildLogEntry[] = buildLogPool
		.slice(0, 4)
		.map((entry) => ({ ...entry, date: '' }));

	const logDateFormatter = browser
		? new Intl.DateTimeFormat('en-US', { month: 'short', day: 'numeric' })
		: null;

	const systemStatus = [
		{ label: 'Equities data', state: 'Normalized' },
		{ label: 'Options greeks', state: 'Mapped' },
		{ label: 'Scanner ranking', state: 'Active' },
		{ label: 'Risk limits', state: 'Enforced' },
		{ label: 'Replay library', state: 'Indexed' },
		{ label: 'Member access', state: 'Hardened' }
	];

	let mounted = $state(false);
	let reducedMotion = $state(false);
	let activeTimeframe = $state('5m');
	let activeAcademyTrack = $state<AcademyTrackId>('day');
	let scrollProgress = $state(0);

	let gsapActive = $state(false);
	let dayNumber = $state(0);
	let buildProgress = $state(88);
	let progressDisplay = $state(88);
	let countdown = $state({ hours: '--', minutes: '--', seconds: '--' });
	let buildLog = $state<BuildLogEntry[]>(fallbackBuildLog);

	let email = $state('');
	let isSubmitting = $state(false);
	let isSubmitted = $state(false);
	let errorMessage = $state('');

	let visible = $state<Record<SectionId, boolean>>({
		hero: false,
		note: false,
		operations: false,
		charts: false,
		signals: false,
		academy: false,
		infrastructure: false,
		buildlog: false,
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
				'Graduate with a written intraday stock/options playbook, daily prep routine, and measurable execution rules.'
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
				'Graduate with a swing stock/options model portfolio, risk template, watchlist process, and review cadence.'
		}
	];

	let shellEl: HTMLDivElement;
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
	let chartInitGeneration = 0;
	let notifyAbortController: AbortController | null = null;
	let raf = 0;
	let statsRaf = 0;
	let gsapContext: { revert: () => void } | null = null;
	let masterTimeline: { progress: (value: number) => unknown } | null = null;

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

	const deskNote = $derived(deskNotes[(Math.max(dayNumber, 1) - 1) % deskNotes.length]);
	const noteNumber = $derived(dayNumber > 0 ? String(dayNumber).padStart(3, '0') : '···');
	const completedStages = $derived(buildProgress >= 97 ? 4 : 3);

	onMount(() => {
		if (!browser) return;

		mounted = true;
		reducedMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches;

		refreshDailyContent();
		updateCountdown();

		const cleanupReveal = setupRevealObserver();
		const cleanupIntervals = startLiveSystems();

		// Failsafe: if the entrance hasn't finished shortly after load
		// (throttled tabs, broken rAF), settle everything to its final state.
		const entranceFailsafe = window.setTimeout(() => {
			masterTimeline?.progress(1);
		}, 2500);

		if (!reducedMotion) {
			void initMotion();
		}

		visible.hero = true;

		return () => {
			mounted = false;
			chartInitGeneration += 1;
			notifyAbortController?.abort();
			notifyAbortController = null;
			cancelAnimationFrame(raf);
			cancelAnimationFrame(statsRaf);
			window.clearTimeout(entranceFailsafe);
			gsapContext?.revert();
			gsapContext = null;
			masterTimeline = null;
			cleanupReveal?.();
			cleanupIntervals();
			disposeCharts();
		};
	});

	const mountShell: Attachment<HTMLDivElement> = (node) => {
		shellEl = node;
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

	function computeDayNumber(now: number) {
		return Math.max(1, Math.floor((now - BUILD_EPOCH_UTC) / DAY_MS) + 1);
	}

	function computeProgress(day: number) {
		return Math.min(97, Math.floor(78 + day * 0.25));
	}

	function refreshDailyContent() {
		const now = Date.now();
		dayNumber = computeDayNumber(now);
		buildProgress = computeProgress(dayNumber);
		progressDisplay = buildProgress;

		const entries: BuildLogEntry[] = [];
		for (let offset = 0; offset < 4; offset += 1) {
			const day = dayNumber - offset;
			if (day < 1) break;
			const item = buildLogPool[(day - 1) % buildLogPool.length];
			entries.push({
				...item,
				date: logDateFormatter?.format(new Date(now - offset * DAY_MS)) ?? ''
			});
		}
		if (entries.length) buildLog = entries;
	}

	function nextBuildDrop(nyNow: Date) {
		// The next weekday 9:30 AM is at most 3 days out (Friday afternoon → Monday).
		for (let offset = 0; offset <= 4; offset += 1) {
			const candidate = new Date(
				nyNow.getFullYear(),
				nyNow.getMonth(),
				nyNow.getDate() + offset,
				9,
				30,
				0,
				0
			);
			if (candidate.getDay() === 0 || candidate.getDay() === 6) continue;
			if (candidate.getTime() > nyNow.getTime()) return candidate;
		}
		return nyNow;
	}

	function updateCountdown() {
		// The daily update lands with the opening bell: 9:30 AM ET, Mon-Fri.
		const nyNow = new Date(new Date().toLocaleString('en-US', { timeZone: 'America/New_York' }));
		const target = nextBuildDrop(nyNow);

		const diff = Math.max(0, target.getTime() - nyNow.getTime());
		countdown = {
			hours: String(Math.floor(diff / 3_600_000)).padStart(2, '0'),
			minutes: String(Math.floor((diff % 3_600_000) / 60_000)).padStart(2, '0'),
			seconds: String(Math.floor((diff % 60_000) / 1000)).padStart(2, '0')
		};
	}

	/* ── Motion ──────────────────────────────────────────────────────────────
	 * Deliberately restrained: short fades and small rises, standard easing,
	 * nothing looping. Loaded dynamically; the prerendered page is complete
	 * without it, and reduced-motion skips it entirely.
	 */
	async function initMotion() {
		try {
			const [gsapModule, scrollTriggerModule] = await Promise.all([
				import('gsap'),
				import('gsap/ScrollTrigger')
			]);
			if (!mounted) return;

			const gsap = gsapModule.gsap;
			const ScrollTrigger = scrollTriggerModule.ScrollTrigger;
			gsap.registerPlugin(ScrollTrigger);

			gsapActive = true;
			await tick();
			if (!mounted) {
				gsapActive = false;
				return;
			}

			gsapContext = gsap.context(() => {
				buildMotion(gsap, ScrollTrigger);
			}, shellEl);
		} catch (error) {
			console.error('[Maintenance] Motion layer failed to load', error);
			gsapActive = false;
		}
	}

	function buildMotion(gsap: GsapCore, ScrollTrigger: ScrollTriggerStatic) {
		ScrollTrigger.defaults({ scroller: shellEl });

		// Entrance: one quiet pass over the hero, under a second in total.
		const master = gsap.timeline({ defaults: { ease: 'power2.out', duration: 0.55 } });
		masterTimeline = master;

		master.to('#hero [data-entrance]', { autoAlpha: 1, y: 0, stagger: 0.07 }, 0.05);

		// Progress figure settles from its fallback to today's value.
		const progressProxy = { value: Math.max(0, buildProgress - 6) };
		progressDisplay = progressProxy.value;
		master.to(
			progressProxy,
			{
				value: buildProgress,
				duration: 0.9,
				ease: 'power1.out',
				onUpdate: () => {
					progressDisplay = Math.round(progressProxy.value);
				}
			},
			0.35
		);

		// Sections: a single short fade-and-rise per block as it enters.
		const sections = gsap.utils.toArray<HTMLElement>('.page-shell section:not(#hero)');
		sections.forEach((section) => {
			const blocks = section.querySelectorAll(
				'.section-heading, [data-cine] > *, [data-cine-self]'
			);
			if (!blocks.length) return;

			gsap.from(blocks, {
				autoAlpha: 0,
				y: 14,
				duration: 0.5,
				ease: 'power2.out',
				stagger: 0.06,
				scrollTrigger: { trigger: section, start: 'top 82%', once: true }
			});
		});
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

	function handleScroll() {
		cancelAnimationFrame(raf);
		raf = requestAnimationFrame(() => {
			const max = shellEl.scrollHeight - shellEl.clientHeight;
			scrollProgress = max > 0 ? shellEl.scrollTop / max : 0;
		});
	}

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

	function disposeCharts() {
		mainChart?.remove();
		aaplChart?.remove();
		nvdaChart?.remove();
		mainChart = null;
		aaplChart = null;
		nvdaChart = null;
		mainSeries = null;
		aaplSeries = null;
		nvdaSeries = null;
		volumeSeries = null;
		chartsReady = false;
	}

	async function initCharts() {
		if (chartsReady || !browser || !mounted || !mainChartEl || !aaplChartEl || !nvdaChartEl) {
			return;
		}

		const initGeneration = ++chartInitGeneration;

		try {
			await tick();
			if (
				!mounted ||
				initGeneration !== chartInitGeneration ||
				chartsReady ||
				!mainChartEl ||
				!aaplChartEl ||
				!nvdaChartEl
			) {
				return;
			}

			const { createChart, CandlestickSeries, AreaSeries, HistogramSeries } =
				await import('lightweight-charts');

			if (
				!mounted ||
				initGeneration !== chartInitGeneration ||
				chartsReady ||
				!mainChartEl ||
				!aaplChartEl ||
				!nvdaChartEl
			) {
				return;
			}

			const nextChartData = {
				main: generateCandles(1487, 5892.33, 76, 0.0045),
				aapl: generateCandles(412, 232.3, 48, 0.0065),
				nvda: generateCandles(911, 1475.2, 48, 0.0078)
			};

			const commonLayout = {
				background: { color: 'transparent' },
				textColor: 'rgba(236, 234, 228, 0.62)',
				fontFamily:
					'Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, sans-serif',
				fontSize: 11
			};

			mainChart = createChart(mainChartEl, {
				autoSize: true,
				layout: commonLayout,
				grid: {
					vertLines: { color: 'rgba(255, 255, 255, 0.04)' },
					horzLines: { color: 'rgba(255, 255, 255, 0.04)' }
				},
				rightPriceScale: {
					borderColor: 'rgba(255, 255, 255, 0.1)',
					scaleMargins: { top: 0.08, bottom: 0.18 }
				},
				timeScale: {
					borderColor: 'rgba(255, 255, 255, 0.1)',
					timeVisible: true,
					secondsVisible: false
				},
				crosshair: {
					vertLine: { color: 'rgba(198, 161, 91, 0.4)', style: 3 },
					horzLine: { color: 'rgba(198, 161, 91, 0.4)', style: 3 }
				}
			});

			mainSeries = mainChart.addSeries(CandlestickSeries, {
				upColor: '#2e9c77',
				downColor: '#c25555',
				borderUpColor: '#2e9c77',
				borderDownColor: '#c25555',
				wickUpColor: '#2e9c77',
				wickDownColor: '#c25555'
			});

			volumeSeries = mainChart.addSeries(HistogramSeries, {
				priceFormat: { type: 'volume' },
				priceScaleId: ''
			});

			mainSeries.setData(nextChartData.main);
			volumeSeries.priceScale().applyOptions({ scaleMargins: { top: 0.82, bottom: 0 } });
			volumeSeries.setData(
				nextChartData.main.map<HistogramData>((candle, index) => ({
					time: candle.time,
					value: 480000 + ((index * 13729) % 620000),
					color:
						candle.close >= candle.open ? 'rgba(46, 156, 119, 0.28)' : 'rgba(194, 85, 85, 0.28)'
				}))
			);

			aaplChart = createMiniChart(createChart, aaplChartEl);
			nvdaChart = createMiniChart(createChart, nvdaChartEl);

			aaplSeries = aaplChart.addSeries(AreaSeries, {
				lineColor: 'rgba(143, 166, 184, 0.9)',
				topColor: 'rgba(143, 166, 184, 0.14)',
				bottomColor: 'rgba(143, 166, 184, 0)',
				lineWidth: 2
			});

			nvdaSeries = nvdaChart.addSeries(AreaSeries, {
				lineColor: 'rgba(198, 161, 91, 0.9)',
				topColor: 'rgba(198, 161, 91, 0.14)',
				bottomColor: 'rgba(198, 161, 91, 0)',
				lineWidth: 2
			});

			aaplSeries.setData(
				nextChartData.aapl.map((candle) => ({ time: candle.time, value: candle.close }))
			);
			nvdaSeries.setData(
				nextChartData.nvda.map((candle) => ({ time: candle.time, value: candle.close }))
			);

			mainChart.timeScale().fitContent();
			aaplChart.timeScale().fitContent();
			nvdaChart.timeScale().fitContent();
			chartData = nextChartData;
			chartsReady = true;
		} catch (error) {
			if (mounted && initGeneration === chartInitGeneration) {
				console.error('[Maintenance] Failed to initialize charts', error);
			}
		}
	}

	function createMiniChart(
		createChart: typeof import('lightweight-charts').createChart,
		element: HTMLElement
	) {
		return createChart(element, {
			autoSize: true,
			layout: {
				background: { color: 'transparent' },
				textColor: 'rgba(236, 234, 228, 0.45)',
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

		timers.push(window.setInterval(updateCountdown, 1000));
		timers.push(window.setInterval(refreshDailyContent, 3_600_000));

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
						color: main.close >= main.open ? 'rgba(46, 156, 119, 0.28)' : 'rgba(194, 85, 85, 0.28)'
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
		const duration = reducedMotion ? 1 : 800;
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
		if (isSubmitting) return;

		const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
		if (!email || !emailRegex.test(email)) {
			errorMessage = 'Enter a valid email address.';
			return;
		}

		isSubmitting = true;
		errorMessage = '';
		notifyAbortController?.abort();
		const controller = new AbortController();
		notifyAbortController = controller;

		try {
			const response = await fetch('/api/maintenance/notify', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ email }),
				signal: controller.signal
			});

			if (!response.ok) throw new Error('Failed to subscribe');
			if (!mounted || controller.signal.aborted) return;
			isSubmitted = true;
		} catch (_error) {
			if (!mounted || controller.signal.aborted) return;
			errorMessage = 'Connection error. Please try again.';
		} finally {
			if (notifyAbortController === controller) {
				notifyAbortController = null;
			}
			if (mounted && !controller.signal.aborted) {
				isSubmitting = false;
			}
		}
	}

	function formatNumber(value: number) {
		return new Intl.NumberFormat('en-US').format(value);
	}
</script>

<div
	class={{ 'maintenance-experience': true, mounted, 'gsap-on': gsapActive }}
	style:--scroll-progress={scrollProgress}
	{@attach mountShell}
	onscroll={handleScroll}
>
	<div class="progress-rail" aria-hidden="true">
		<span></span>
	</div>

	<header class="market-tape" aria-label="Market tape">
		<div class="tape-viewport">
			<div class="tape-track" aria-hidden="true">
				{#each [...liveTickers, ...liveTickers, ...liveTickers] as ticker, index (ticker.symbol + '-' + index)}
					<div class="tape-item">
						<span class="tape-symbol">{ticker.symbol}</span>
						<span class="tape-price">{ticker.price.toFixed(2)}</span>
						<span class={{ 'tape-change': true, up: ticker.change >= 0, down: ticker.change < 0 }}>
							{ticker.change >= 0 ? '+' : ''}{ticker.change.toFixed(2)}%
						</span>
					</div>
				{/each}
			</div>
		</div>
		<a class="tape-cta" href="#access">Request access</a>
	</header>

	<main class="page-shell">
		<section id="hero" class={{ hero: true, reveal: true, visible: visible.hero }}>
			<div class="hero-copy">
				<p class="eyebrow" data-entrance>Platform rebuild · Day {dayNumber || '—'}</p>
				<h1 class="hero-title" data-entrance>
					<span class="line">We didn’t go quiet.</span>
					<span class="line">We went to work.</span>
				</h1>
				<p class="lede" data-entrance>
					Revolution Trading Pros is being rebuilt end to end — faster market data, more rigorous
					scanners, and a trading curriculum run to an institutional standard. Progress is posted on
					this page every trading day.
				</p>

				<div class="hero-actions" data-entrance>
					<a class="primary-link" href="#access">Request first access</a>
					<a class="ghost-link" href="#operations">Review the work</a>
				</div>
				<p class="hero-note" data-entrance>
					Founding-member pricing at launch · one email at reopening · no marketing sequence
				</p>

				<p
					class="drop-timer"
					data-entrance
					role="timer"
					aria-label="Time until the next daily update at 9:30 AM Eastern"
				>
					<span class="timer-label">Next update</span>
					<span class="timer-clock">{countdown.hours}:{countdown.minutes}:{countdown.seconds}</span>
					<span class="timer-meta">9:30 AM ET · Monday–Friday</span>
				</p>
			</div>

			<aside class="status-card" aria-label="Rebuild status" data-entrance>
				<div class="card-topline">
					<span>Rebuild status</span>
					<span class="status-flag">In progress</span>
				</div>

				<div class="progress-block">
					<div class="progress-head">
						<span>Overall completion</span>
						<strong>{progressDisplay}%</strong>
					</div>
					<div class="progress-track">
						<span style:width={`${progressDisplay}%`}></span>
					</div>
				</div>

				<ol class="stage-list">
					{#each stageLines as line, index (line)}
						<li
							class={{
								complete: index < completedStages,
								current: index === completedStages
							}}
						>
							<span class="stage-index">{String(index + 1).padStart(2, '0')}</span>
							<span class="stage-name">{line}</span>
							<span class="stage-state">
								{index < completedStages ? 'Complete' : index === completedStages ? 'Underway' : 'Queued'}
							</span>
						</li>
					{/each}
				</ol>

				<div class="card-footline">
					<span>Day {dayNumber || '—'}</span>
					<span>Next update {countdown.hours}:{countdown.minutes}:{countdown.seconds}</span>
				</div>
			</aside>
		</section>

		<section id="note" class={{ 'note-section': true, reveal: true, visible: visible.note }}>
			<div class="section-heading">
				<p class="section-kicker">Daily desk note</p>
				<h2>One disciplined idea, every trading day.</h2>
			</div>

			<figure class="note-card" data-cine-self>
				<blockquote>{deskNote}</blockquote>
				<p class="note-foot">
					A new note is posted at the open, Monday through Friday.
				</p>
				<figcaption class="note-meta">
					<span>Desk note</span>
					<strong>№ {noteNumber}</strong>
				</figcaption>
			</figure>
		</section>

		<section
			id="operations"
			class={{ 'ops-section': true, reveal: true, visible: visible.operations }}
		>
			<div class="section-heading">
				<p class="section-kicker">Scope of work</p>
				<h2>Every system, rebuilt to institutional spec.</h2>
			</div>

			<div class="ops-stage">
				<div class="ops-grid" aria-label="Rebuild scope" data-cine>
					{#each operationsChapters as chapter (chapter.number)}
						<article class="ops-card">
							<span class="ops-index">{chapter.number}</span>
							<h3>{chapter.title}</h3>
							<p>{chapter.copy}</p>
							<span class="ops-metric">{chapter.metric}</span>
						</article>
					{/each}
				</div>

				<div class="systems-panel" aria-label="System status" data-cine-self>
					<div class="card-topline">
						<span>System status</span>
						<span class="status-flag">Operational</span>
					</div>
					<dl class="status-list">
						{#each systemStatus as item (item.label)}
							<div class="status-row">
								<dt>{item.label}</dt>
								<dd>{item.state}</dd>
							</div>
						{/each}
					</dl>
					<div class="card-footline">
						<span>Availability target</span>
						<span>99.99%</span>
					</div>
				</div>
			</div>
		</section>

		<section id="charts" class={{ 'charts-section': true, reveal: true, visible: visible.charts }}>
			<div class="section-heading">
				<p class="section-kicker">Market data</p>
				<h2>The data engine never went dark.</h2>
			</div>

			<div class="chart-stage" data-cine-self>
				<div class="chart-toolbar">
					<div>
						<span class="market-label">SPX · Upgrade feed</span>
						<strong class={{ up: mainDirection === 'up', down: mainDirection === 'down' }}>
							{latestMainPrice.toFixed(2)}
						</strong>
					</div>
					<div class="timeframe-control" aria-label="Chart timeframe">
						{#each timeframes as timeframe (timeframe)}
							<button
								type="button"
								class={{ active: activeTimeframe === timeframe }}
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
						<span>Avg. confidence</span>
						<strong>{averageConfidence}%</strong>
					</div>
				</div>
			</div>

			<div class="mini-feed-grid" data-cine>
				<div class="mini-feed">
					<div>
						<span>AAPL</span>
						<strong>{(chartData.aapl.at(-1)?.close ?? 232.3).toFixed(2)}</strong>
					</div>
					<div class="mini-chart" {@attach mountAaplChart}></div>
				</div>

				<div class="mini-feed">
					<div>
						<span>NVDA</span>
						<strong>{(chartData.nvda.at(-1)?.close ?? 1475.2).toFixed(2)}</strong>
					</div>
					<div class="mini-chart" {@attach mountNvdaChart}></div>
				</div>
			</div>
		</section>

		<section
			id="signals"
			class={{ 'signals-section': true, reveal: true, visible: visible.signals }}
		>
			<div class="section-heading">
				<p class="section-kicker">Scanner engine</p>
				<h2>Signal clarity, engineered under pressure.</h2>
			</div>

			<div class="signals-layout">
				<div class="coverage-panel" data-cine-self>
					<div class="card-topline">
						<span>Coverage</span>
					</div>
					<dl class="coverage-list">
						<div>
							<dt>Symbols monitored</dt>
							<dd>12,847</dd>
						</div>
						<div>
							<dt>Signals active now</dt>
							<dd>{activeSignals}</dd>
						</div>
						<div>
							<dt>Average confidence</dt>
							<dd>{averageConfidence}%</dd>
						</div>
						<div>
							<dt>Feed latency</dt>
							<dd>15ms</dd>
						</div>
					</dl>
				</div>

				<div class="signals-table" data-cine-self>
					<div class="table-head">
						<span>Detected signals</span>
						<span>Internal preview</span>
					</div>
					{#each scannerSignals as signal (signal.id)}
						<article class={{ 'signal-row': true, cooling: signal.status === 'cooling' }}>
							<div>
								<strong>{signal.symbol}</strong>
								<span>{signal.type}</span>
							</div>
							<div class="signal-figures">
								<span>{signal.price.toFixed(2)}</span>
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

		<section
			id="academy"
			class={{ 'academy-section': true, reveal: true, visible: visible.academy }}
		>
			<div class="section-heading">
				<p class="section-kicker">Trading university</p>
				<h2>Stocks, options, and desk discipline in one curriculum.</h2>
			</div>

			<div class="academy-block">
				<div class="academy-selector" role="tablist" aria-label="Curriculum track" data-cine>
					{#each academyTracks as track (track.id)}
						<button
							type="button"
							role="tab"
							aria-selected={activeAcademyTrack === track.id}
							class={{ active: activeAcademyTrack === track.id }}
							onclick={() => (activeAcademyTrack = track.id)}
						>
							<span class="track-index">{track.number}</span>
							<strong>{track.title}</strong>
							<small>Stocks + options</small>
						</button>
					{/each}
				</div>

				<div class="academy-feature" data-cine-self>
					<div class="academy-summary">
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
						<div class="table-head">
							<span>Curriculum</span>
							<span>Desk-ready sequence</span>
						</div>
						<ol class="curriculum-list">
							{#each activeAcademy.curriculum as module, index (module)}
								<li>
									<span>{String(index + 1).padStart(2, '0')}</span>
									<p>{module}</p>
								</li>
							{/each}
						</ol>
						<div class="academy-lab">
							<div>
								<span>Capstone lab</span>
								<p>{activeAcademy.lab}</p>
							</div>
							<div>
								<span>Outcome</span>
								<p>{activeAcademy.outcome}</p>
							</div>
						</div>
					</div>
				</div>
			</div>
		</section>

		<section
			id="infrastructure"
			class={{ 'infra-section': true, reveal: true, visible: visible.infrastructure }}
		>
			<div class="section-heading">
				<p class="section-kicker">Build telemetry</p>
				<h2>Capacity, reliability, and speed — measured daily.</h2>
			</div>

			<div class="infra-grid">
				<div class="node-list" data-cine-self>
					{#each nodes as node (node.id)}
						<article class="node-row">
							<div>
								<strong>{node.id}</strong>
								<span>{node.region}</span>
							</div>
							<div class="load-track" aria-label={`${node.id} load ${node.load}%`}>
								<span style:width={`${node.load}%`}></span>
							</div>
							<small class={{ migrating: node.status === 'migrating' }}>{node.status}</small>
						</article>
					{/each}
				</div>

				<div class="stat-board" data-cine-self>
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

		<section
			id="buildlog"
			class={{ 'buildlog-section': true, reveal: true, visible: visible.buildlog }}
		>
			<div class="section-heading">
				<p class="section-kicker">Build log</p>
				<h2>Completed while you were away.</h2>
			</div>

			<div class="log-panel" data-cine>
				{#each buildLog as entry (entry.text)}
					<article class="log-row">
						<span class="log-date">{entry.date || '···'}</span>
						<span class={`log-tag ${entry.tag.toLowerCase()}`}>{entry.tag}</span>
						<p>{entry.text}</p>
					</article>
				{/each}
			</div>
			<p class="log-note">Updated every trading day at the open.</p>
		</section>

		<section id="access" class={{ 'access-section': true, reveal: true, visible: visible.access }}>
			{#if isSubmitted}
				<div class="success-panel" role="status">
					<div class="success-mark" aria-hidden="true">
						<svg viewBox="0 0 56 56">
							<circle cx="28" cy="28" r="25" />
							<path d="M16 29.5 24.5 38 41 19" />
						</svg>
					</div>
					<div>
						<h2>You're on the list.</h2>
						<p>
							A confirmation is on its way to your inbox. The desk note and build log on this page
							are refreshed every trading day.
						</p>
					</div>
				</div>
			{:else}
				<form
					class="access-panel"
					data-cine
					onsubmit={(event) => {
						event.preventDefault();
						void handleNotifyMe();
					}}
				>
					<div>
						<p class="section-kicker">First access</p>
						<h2>Be notified when we reopen.</h2>
						<p>
							One email when the platform reopens — nothing else. Founding members receive launch
							pricing, launch-week live sessions, and early access to the new scanner suite.
						</p>
						<p class="perk-line">
							Founding-member pricing · Launch-week sessions · Scanner suite preview
						</p>
					</div>

					<div class="email-block">
						<div class="email-row">
							<label class="sr-only" for="maintenance-email">Email address</label>
							<input
								id="maintenance-email"
								type="email"
								placeholder="name@firm.com"
								bind:value={email}
								disabled={isSubmitting}
								autocomplete="email"
							/>
							<button type="submit" disabled={isSubmitting || !email}>
								{isSubmitting ? 'Submitting…' : 'Request access'}
							</button>
						</div>

						{#if errorMessage}
							<p class="form-error" role="alert">{errorMessage}</p>
						{/if}
					</div>
				</form>
			{/if}
		</section>

		<footer class="floor-footer">
			<div class="footer-brand">
				<strong>Revolution Trading Pros</strong>
				<span>Platform rebuild in progress. Reopening soon.</span>
			</div>
			<p class="footer-risk">
				Trading involves substantial risk of loss and is not suitable for every investor. Content on
				this site is educational and is not financial advice.
			</p>
			<span class="footer-legal">© 2026 Revolution Trading Pros</span>
		</footer>
	</main>
</div>

<style>
	:global(html:has(.maintenance-experience)),
	:global(body:has(.maintenance-experience)) {
		overflow: hidden;
		background: #0a0b0e;
	}

	.maintenance-experience {
		--bg: #0a0b0e;
		--panel: rgba(255, 255, 255, 0.02);
		--line: rgba(255, 255, 255, 0.07);
		--line-strong: rgba(255, 255, 255, 0.12);
		--text: #eceae4;
		--muted: rgba(236, 234, 228, 0.64);
		--dim: rgba(236, 234, 228, 0.42);
		--accent: #c6a15b;
		--up: #2e9c77;
		--down: #c25555;
		position: fixed;
		inset: 0;
		z-index: 99999;
		overflow-x: hidden;
		overflow-y: auto;
		background:
			radial-gradient(1100px 520px at 72% -12%, rgba(198, 161, 91, 0.05), transparent 62%),
			var(--bg);
		color: var(--text);
		font-family:
			Inter,
			ui-sans-serif,
			system-ui,
			-apple-system,
			BlinkMacSystemFont,
			'Segoe UI',
			sans-serif;
		-webkit-font-smoothing: antialiased;
	}

	.maintenance-experience::-webkit-scrollbar {
		width: 8px;
	}

	.maintenance-experience::-webkit-scrollbar-thumb {
		background: rgba(255, 255, 255, 0.16);
		border: 2px solid #0a0b0e;
		border-radius: 999px;
	}

	.progress-rail {
		position: fixed;
		inset: 0 auto 0 0;
		z-index: 30;
		width: 2px;
		background: rgba(255, 255, 255, 0.05);
	}

	.progress-rail span {
		display: block;
		width: 100%;
		height: calc(var(--scroll-progress) * 100%);
		background: var(--accent);
	}

	/* ── Tape ─────────────────────────────────────────────────────────────── */

	.market-tape {
		position: sticky;
		top: 0;
		z-index: 20;
		display: flex;
		align-items: center;
		gap: 16px;
		border-bottom: 1px solid var(--line);
		background: rgba(10, 11, 14, 0.92);
		backdrop-filter: blur(14px);
	}

	.tape-viewport {
		flex: 1;
		min-width: 0;
		overflow: hidden;
		mask-image: linear-gradient(90deg, transparent, black 3%, black 97%, transparent);
	}

	.market-tape:hover .tape-track {
		animation-play-state: paused;
	}

	.tape-track {
		display: flex;
		width: max-content;
		padding: 11px 0;
		animation: tape 56s linear infinite;
	}

	.tape-item {
		display: inline-flex;
		align-items: baseline;
		gap: 8px;
		padding: 0 22px;
		border-right: 1px solid var(--line);
		font-family: 'SF Mono', ui-monospace, Menlo, monospace;
		font-size: 12px;
		font-variant-numeric: tabular-nums;
		white-space: nowrap;
	}

	.tape-symbol {
		color: var(--text);
		font-weight: 600;
	}

	.tape-price {
		color: var(--muted);
	}

	.tape-change.up {
		color: var(--up);
	}

	.tape-change.down {
		color: var(--down);
	}

	.tape-cta {
		flex-shrink: 0;
		margin-right: 16px;
		border: 1px solid var(--line-strong);
		border-radius: 4px;
		padding: 7px 14px;
		color: var(--text);
		font-size: 12px;
		font-weight: 600;
		letter-spacing: 0.04em;
		text-decoration: none;
		transition:
			border-color 160ms ease,
			background 160ms ease;
	}

	.tape-cta:hover {
		border-color: var(--accent);
		background: rgba(198, 161, 91, 0.08);
	}

	/* ── Layout ───────────────────────────────────────────────────────────── */

	.page-shell {
		position: relative;
		z-index: 2;
		width: min(1180px, calc(100% - 40px));
		margin: 0 auto;
		padding: clamp(40px, 7vw, 88px) 0 40px;
	}

	.hero {
		display: grid;
		grid-template-columns: minmax(0, 1.1fr) minmax(330px, 0.68fr);
		gap: clamp(28px, 5vw, 72px);
		align-items: center;
		min-height: min(720px, calc(100vh - 120px));
		padding-bottom: 4vh;
	}

	.hero-copy {
		max-width: 720px;
	}

	.eyebrow,
	.section-kicker {
		display: block;
		margin: 0 0 18px;
		color: var(--accent);
		font-size: 11px;
		font-weight: 600;
		letter-spacing: 0.22em;
		text-transform: uppercase;
	}

	h1 {
		margin: 0;
		font-size: clamp(2.2rem, 3.9vw, 3.6rem);
		font-weight: 640;
		letter-spacing: -0.018em;
		line-height: 1.08;
		text-wrap: balance;
	}

	.hero-title .line {
		display: block;
	}

	h2 {
		margin: 0;
		font-size: clamp(1.55rem, 2.8vw, 2.5rem);
		font-weight: 620;
		letter-spacing: -0.014em;
		line-height: 1.12;
		text-wrap: balance;
	}

	h3 {
		letter-spacing: -0.01em;
	}

	.lede {
		max-width: 620px;
		margin: 22px 0 0;
		color: var(--muted);
		font-size: clamp(1rem, 1.3vw, 1.13rem);
		line-height: 1.75;
	}

	.hero-actions {
		display: flex;
		flex-wrap: wrap;
		gap: 12px;
		margin-top: 30px;
	}

	.hero-note {
		margin: 14px 0 0;
		color: var(--dim);
		font-size: 12.5px;
		letter-spacing: 0.01em;
	}

	.primary-link,
	.ghost-link,
	.email-row button {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		min-height: 44px;
		border-radius: 4px;
		padding: 0 20px;
		font-size: 14.5px;
		font-weight: 600;
		letter-spacing: 0.01em;
		text-decoration: none;
		transition:
			background 160ms ease,
			border-color 160ms ease,
			color 160ms ease;
	}

	.primary-link,
	.email-row button {
		border: 1px solid var(--text);
		background: var(--text);
		color: #101114;
	}

	.primary-link:hover,
	.email-row button:hover:not(:disabled) {
		background: #fbf9f4;
		border-color: #fbf9f4;
	}

	.ghost-link {
		border: 1px solid var(--line-strong);
		background: transparent;
		color: var(--text);
	}

	.ghost-link:hover {
		border-color: var(--accent);
	}

	/* ── Countdown ────────────────────────────────────────────────────────── */

	.drop-timer {
		display: flex;
		flex-wrap: wrap;
		align-items: baseline;
		gap: 8px 16px;
		margin: 28px 0 0;
		border-top: 1px solid var(--line);
		padding-top: 18px;
	}

	.timer-label {
		color: var(--accent);
		font-size: 11px;
		font-weight: 600;
		letter-spacing: 0.22em;
		text-transform: uppercase;
	}

	.timer-clock {
		color: var(--text);
		font-family: 'SF Mono', ui-monospace, Menlo, monospace;
		font-size: 20px;
		font-weight: 600;
		font-variant-numeric: tabular-nums;
	}

	.timer-meta {
		color: var(--dim);
		font-size: 12.5px;
	}

	/* ── Panels ───────────────────────────────────────────────────────────── */

	.status-card,
	.note-card,
	.ops-card,
	.systems-panel,
	.chart-stage,
	.mini-feed,
	.coverage-panel,
	.signals-table,
	.academy-selector button,
	.academy-feature,
	.node-list,
	.stat-board,
	.log-panel,
	.access-panel,
	.success-panel {
		border: 1px solid var(--line);
		border-radius: 6px;
		background: var(--panel);
	}

	.card-topline,
	.table-head {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 14px;
		color: var(--dim);
		font-size: 11px;
		font-weight: 600;
		letter-spacing: 0.14em;
		text-transform: uppercase;
	}

	.status-flag {
		color: var(--accent);
	}

	.status-card {
		padding: 22px;
	}

	.progress-block {
		margin: 26px 0 22px;
	}

	.progress-head {
		display: flex;
		align-items: baseline;
		justify-content: space-between;
		gap: 12px;
		margin-bottom: 10px;
	}

	.progress-head span {
		color: var(--muted);
		font-size: 13px;
	}

	.progress-head strong {
		font-family: 'SF Mono', ui-monospace, Menlo, monospace;
		font-size: 26px;
		font-weight: 600;
		font-variant-numeric: tabular-nums;
	}

	.progress-track {
		height: 3px;
		overflow: hidden;
		border-radius: 999px;
		background: rgba(255, 255, 255, 0.08);
	}

	.progress-track span {
		display: block;
		height: 100%;
		border-radius: inherit;
		background: var(--accent);
		transition: width 400ms ease;
	}

	.stage-list {
		display: grid;
		gap: 0;
		margin: 0;
		padding: 0;
		list-style: none;
	}

	.stage-list li {
		display: grid;
		grid-template-columns: 34px 1fr auto;
		gap: 12px;
		align-items: baseline;
		padding: 11px 2px;
	}

	.stage-list li + li {
		border-top: 1px solid var(--line);
	}

	.stage-index {
		color: var(--dim);
		font-family: 'SF Mono', ui-monospace, Menlo, monospace;
		font-size: 12px;
	}

	.stage-name {
		color: var(--muted);
		font-size: 14px;
	}

	.stage-state {
		color: var(--dim);
		font-size: 11px;
		font-weight: 600;
		letter-spacing: 0.1em;
		text-transform: uppercase;
	}

	.stage-list li.complete .stage-name {
		color: var(--text);
	}

	.stage-list li.complete .stage-state {
		color: var(--up);
	}

	.stage-list li.current .stage-state {
		color: var(--accent);
	}

	.card-footline {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 12px;
		margin-top: 16px;
		border-top: 1px solid var(--line);
		padding-top: 14px;
		color: var(--dim);
		font-family: 'SF Mono', ui-monospace, Menlo, monospace;
		font-size: 11.5px;
		font-variant-numeric: tabular-nums;
	}

	/* ── Reveal (CSS fallback when the motion layer is absent) ────────────── */

	.reveal {
		transition:
			opacity 600ms cubic-bezier(0.25, 0.1, 0.25, 1),
			transform 600ms cubic-bezier(0.25, 0.1, 0.25, 1);
	}

	.mounted .reveal {
		opacity: 0;
		transform: translateY(16px);
	}

	.reveal.visible {
		opacity: 1;
		transform: translateY(0);
	}

	.gsap-on .reveal {
		opacity: 1;
		transform: none;
		transition: none;
	}

	/* GSAP entrance initial state — applied only when the motion layer runs. */
	.gsap-on #hero [data-entrance] {
		opacity: 0;
		transform: translateY(12px);
	}

	/* ── Sections ─────────────────────────────────────────────────────────── */

	.note-section,
	.ops-section,
	.charts-section,
	.signals-section,
	.academy-section,
	.infra-section,
	.buildlog-section,
	.access-section {
		padding: clamp(48px, 8vw, 96px) 0;
	}

	.section-heading {
		max-width: 720px;
		margin-bottom: 30px;
	}

	/* ── Desk note ────────────────────────────────────────────────────────── */

	.note-section {
		padding-top: clamp(8px, 2vw, 24px);
	}

	.note-card {
		display: flex;
		flex-direction: column;
		max-width: 900px;
		margin: 0;
		padding: clamp(28px, 4.5vw, 48px);
	}

	.note-card blockquote {
		max-width: 760px;
		margin: 18px 0 0;
		font-family: Georgia, 'Iowan Old Style', 'Times New Roman', serif;
		font-size: clamp(1.35rem, 2.6vw, 1.9rem);
		font-style: italic;
		font-weight: 400;
		line-height: 1.42;
		letter-spacing: 0.002em;
		color: var(--text);
	}

	.note-foot {
		margin: 22px 0 0;
		color: var(--dim);
		font-size: 13px;
	}

	.note-meta {
		order: -1;
		display: flex;
		align-items: baseline;
		justify-content: space-between;
		gap: 12px;
		color: var(--dim);
		font-size: 11px;
		font-weight: 600;
		letter-spacing: 0.16em;
		text-transform: uppercase;
	}

	.note-meta strong {
		color: var(--accent);
		font-family: 'SF Mono', ui-monospace, Menlo, monospace;
		font-size: 12px;
		font-weight: 600;
	}

	/* ── Scope of work ────────────────────────────────────────────────────── */

	.ops-section {
		padding-top: clamp(24px, 4vw, 56px);
	}

	.ops-stage {
		display: grid;
		grid-template-columns: minmax(0, 1.05fr) minmax(320px, 0.68fr);
		gap: 14px;
		align-items: start;
	}

	.ops-grid {
		display: grid;
		grid-template-columns: repeat(2, minmax(0, 1fr));
		gap: 14px;
	}

	.ops-card {
		display: grid;
		align-content: start;
		gap: 14px;
		min-height: 220px;
		padding: 22px;
	}

	.ops-index {
		color: var(--dim);
		font-family: 'SF Mono', ui-monospace, Menlo, monospace;
		font-size: 12px;
	}

	.ops-card h3 {
		margin: 0;
		font-size: 1.15rem;
		font-weight: 620;
	}

	.ops-card p {
		margin: 0;
		color: var(--muted);
		font-size: 14px;
		line-height: 1.65;
	}

	.ops-metric {
		color: var(--accent);
		font-family: 'SF Mono', ui-monospace, Menlo, monospace;
		font-size: 11.5px;
		letter-spacing: 0.08em;
		text-transform: uppercase;
	}

	.systems-panel {
		padding: 22px;
	}

	.status-list {
		display: grid;
		margin: 14px 0 0;
	}

	.status-row {
		display: flex;
		align-items: baseline;
		justify-content: space-between;
		gap: 14px;
		padding: 11px 2px;
	}

	.status-row + .status-row {
		border-top: 1px solid var(--line);
	}

	.status-row dt {
		color: var(--muted);
		font-size: 14px;
	}

	.status-row dd {
		margin: 0;
		color: var(--up);
		font-size: 11.5px;
		font-weight: 600;
		letter-spacing: 0.1em;
		text-transform: uppercase;
	}

	/* ── Charts ───────────────────────────────────────────────────────────── */

	.chart-stage {
		overflow: hidden;
		padding: 20px;
	}

	.chart-toolbar {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 14px;
	}

	.market-label {
		display: block;
		margin-bottom: 6px;
		color: var(--dim);
		font-size: 11px;
		letter-spacing: 0.12em;
		text-transform: uppercase;
	}

	.chart-toolbar strong {
		font-family: 'SF Mono', ui-monospace, Menlo, monospace;
		font-size: clamp(1.3rem, 2.4vw, 1.9rem);
		font-weight: 600;
		font-variant-numeric: tabular-nums;
	}

	.up {
		color: var(--up);
	}

	.down {
		color: var(--down);
	}

	.timeframe-control {
		display: flex;
		gap: 2px;
		padding: 3px;
		border: 1px solid var(--line);
		border-radius: 4px;
	}

	.timeframe-control button {
		min-width: 42px;
		height: 30px;
		border: 0;
		border-radius: 3px;
		background: transparent;
		color: var(--muted);
		font: inherit;
		font-size: 12.5px;
		cursor: pointer;
	}

	.timeframe-control button.active {
		background: rgba(255, 255, 255, 0.07);
		color: var(--text);
	}

	.chart-canvas {
		height: clamp(340px, 48vh, 520px);
		margin-top: 16px;
	}

	.chart-metrics {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 10px;
		margin-top: 16px;
	}

	.chart-metrics div {
		border-top: 1px solid var(--line);
		padding: 12px 2px 0;
	}

	.chart-metrics span {
		display: block;
		color: var(--dim);
		font-size: 11.5px;
		letter-spacing: 0.08em;
		text-transform: uppercase;
	}

	.chart-metrics strong {
		display: block;
		margin-top: 4px;
		color: var(--text);
		font-family: 'SF Mono', ui-monospace, Menlo, monospace;
		font-size: 1.15rem;
		font-weight: 600;
		font-variant-numeric: tabular-nums;
	}

	.mini-feed-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 14px;
		margin-top: 14px;
	}

	.mini-feed {
		padding: 18px;
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
		font-weight: 600;
		letter-spacing: 0.08em;
	}

	.mini-feed strong {
		font-family: 'SF Mono', ui-monospace, Menlo, monospace;
		font-weight: 600;
		font-variant-numeric: tabular-nums;
	}

	.mini-chart {
		height: 140px;
	}

	/* ── Signals ──────────────────────────────────────────────────────────── */

	.signals-layout {
		display: grid;
		grid-template-columns: minmax(0, 0.72fr) minmax(0, 1.28fr);
		gap: 14px;
		align-items: start;
	}

	.coverage-panel,
	.signals-table {
		padding: 20px;
	}

	.coverage-list {
		display: grid;
		margin: 12px 0 0;
	}

	.coverage-list > div {
		display: flex;
		align-items: baseline;
		justify-content: space-between;
		gap: 14px;
		padding: 13px 2px;
	}

	.coverage-list > div + div {
		border-top: 1px solid var(--line);
	}

	.coverage-list dt {
		color: var(--muted);
		font-size: 14px;
	}

	.coverage-list dd {
		margin: 0;
		font-family: 'SF Mono', ui-monospace, Menlo, monospace;
		font-size: 15px;
		font-weight: 600;
		font-variant-numeric: tabular-nums;
	}

	.signals-table {
		display: grid;
		align-content: start;
		gap: 0;
	}

	.signal-row {
		display: grid;
		grid-template-columns: minmax(0, 1fr) 110px 120px;
		gap: 12px;
		align-items: center;
		padding: 13px 2px;
	}

	.signal-row + .signal-row {
		border-top: 1px solid var(--line);
	}

	.table-head + .signal-row {
		margin-top: 8px;
		border-top: 1px solid var(--line);
	}

	.signal-row.cooling {
		opacity: 0.55;
	}

	.signal-row strong {
		display: block;
		font-family: 'SF Mono', ui-monospace, Menlo, monospace;
		font-size: 14px;
		font-weight: 600;
	}

	.signal-row > div > span {
		color: var(--dim);
		font-size: 12.5px;
	}

	.signal-figures {
		font-variant-numeric: tabular-nums;
	}

	.signal-figures span {
		display: block;
		color: var(--muted);
		font-family: 'SF Mono', ui-monospace, Menlo, monospace;
		font-size: 13px;
	}

	.signal-figures small {
		color: var(--dim);
		font-size: 11px;
	}

	.confidence {
		display: grid;
		gap: 6px;
	}

	.confidence > span {
		height: 2px;
		overflow: hidden;
		border-radius: 999px;
		background: rgba(255, 255, 255, 0.08);
	}

	.confidence > span::before {
		content: '';
		display: block;
		width: var(--confidence);
		height: 100%;
		background: var(--accent);
	}

	.confidence small {
		color: var(--dim);
		font-family: 'SF Mono', ui-monospace, Menlo, monospace;
		font-size: 11px;
		font-variant-numeric: tabular-nums;
	}

	/* ── Academy ──────────────────────────────────────────────────────────── */

	.academy-block {
		display: grid;
		gap: 14px;
	}

	.academy-selector {
		display: grid;
		grid-template-columns: repeat(2, minmax(0, 1fr));
		gap: 14px;
	}

	.academy-selector button {
		padding: 18px 20px;
		color: var(--text);
		text-align: left;
		cursor: pointer;
		transition:
			border-color 160ms ease,
			background 160ms ease;
	}

	.academy-selector button:hover {
		border-color: var(--line-strong);
	}

	.academy-selector button.active {
		border-color: var(--accent);
		background: rgba(198, 161, 91, 0.05);
	}

	.academy-selector button > * {
		display: block;
	}

	.track-index {
		color: var(--dim);
		font-family: 'SF Mono', ui-monospace, Menlo, monospace;
		font-size: 12px;
	}

	.academy-selector button strong {
		margin-top: 8px;
		font-size: 1.05rem;
		font-weight: 620;
	}

	.academy-selector button small {
		margin-top: 5px;
		color: var(--dim);
		font-size: 11px;
		letter-spacing: 0.1em;
		text-transform: uppercase;
	}

	.academy-feature {
		display: grid;
		grid-template-columns: minmax(0, 0.86fr) minmax(0, 1.14fr);
		gap: 0;
	}

	.academy-summary {
		padding: 26px;
		border-right: 1px solid var(--line);
	}

	.academy-summary h3 {
		margin: 0 0 12px;
		font-size: clamp(1.3rem, 2.2vw, 1.7rem);
		font-weight: 620;
	}

	.academy-summary > p {
		margin: 0;
		color: var(--muted);
		font-size: 14.5px;
		line-height: 1.7;
	}

	.asset-lanes {
		display: grid;
		grid-template-columns: repeat(2, minmax(0, 1fr));
		gap: 12px;
		margin-top: 24px;
	}

	.asset-lanes div {
		display: grid;
		gap: 7px;
		align-content: start;
		border-top: 1px solid var(--line);
		padding-top: 12px;
	}

	.asset-lanes strong {
		color: var(--accent);
		font-size: 11px;
		font-weight: 600;
		letter-spacing: 0.14em;
		text-transform: uppercase;
	}

	.asset-lanes span {
		color: var(--muted);
		font-size: 13px;
	}

	.curriculum-panel {
		display: grid;
		align-content: start;
		gap: 14px;
		padding: 26px;
	}

	.curriculum-list {
		display: grid;
		margin: 0;
		padding: 0;
		list-style: none;
	}

	.curriculum-list li {
		display: grid;
		grid-template-columns: 34px minmax(0, 1fr);
		gap: 12px;
		align-items: baseline;
		padding: 12px 2px;
	}

	.curriculum-list li + li {
		border-top: 1px solid var(--line);
	}

	.curriculum-list li > span {
		color: var(--dim);
		font-family: 'SF Mono', ui-monospace, Menlo, monospace;
		font-size: 12px;
	}

	.curriculum-list p {
		margin: 0;
		color: var(--muted);
		font-size: 14px;
		line-height: 1.6;
	}

	.academy-lab {
		display: grid;
		grid-template-columns: repeat(2, minmax(0, 1fr));
		gap: 14px;
	}

	.academy-lab div {
		border-top: 1px solid var(--line);
		padding-top: 12px;
	}

	.academy-lab span {
		color: var(--accent);
		font-size: 11px;
		font-weight: 600;
		letter-spacing: 0.14em;
		text-transform: uppercase;
	}

	.academy-lab p {
		margin: 7px 0 0;
		color: var(--muted);
		font-size: 13.5px;
		line-height: 1.6;
	}

	/* ── Infrastructure ───────────────────────────────────────────────────── */

	.infra-grid {
		display: grid;
		grid-template-columns: minmax(0, 1.15fr) minmax(0, 0.85fr);
		gap: 14px;
		align-items: start;
	}

	.node-list {
		display: grid;
		padding: 8px 20px;
	}

	.node-row {
		display: grid;
		grid-template-columns: 120px 1fr 84px;
		gap: 14px;
		align-items: center;
		padding: 14px 0;
	}

	.node-row + .node-row {
		border-top: 1px solid var(--line);
	}

	.node-row strong {
		display: block;
		font-family: 'SF Mono', ui-monospace, Menlo, monospace;
		font-size: 13px;
		font-weight: 600;
	}

	.node-row span {
		display: block;
		color: var(--dim);
		font-size: 12px;
	}

	.node-row small {
		color: var(--muted);
		font-size: 11px;
		font-weight: 600;
		letter-spacing: 0.1em;
		text-align: right;
		text-transform: uppercase;
	}

	.node-row small.migrating {
		color: var(--accent);
	}

	.load-track {
		height: 2px;
		overflow: hidden;
		border-radius: 999px;
		background: rgba(255, 255, 255, 0.08);
	}

	.load-track span {
		display: block;
		height: 100%;
		background: rgba(236, 234, 228, 0.55);
		transition: width 520ms ease;
	}

	.stat-board {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 0;
		padding: 8px 20px;
	}

	.stat-board div {
		padding: 16px 0;
	}

	.stat-board div:nth-child(n + 3) {
		border-top: 1px solid var(--line);
	}

	.stat-board div:nth-child(2n) {
		border-left: 1px solid var(--line);
		padding-left: 20px;
	}

	.stat-board strong {
		display: block;
		font-family: 'SF Mono', ui-monospace, Menlo, monospace;
		font-size: clamp(1.2rem, 2vw, 1.6rem);
		font-weight: 600;
		font-variant-numeric: tabular-nums;
	}

	.stat-board span {
		display: block;
		margin-top: 4px;
		color: var(--dim);
		font-size: 12px;
	}

	/* ── Build log ────────────────────────────────────────────────────────── */

	.buildlog-section {
		padding-bottom: clamp(28px, 4vw, 52px);
	}

	.log-panel {
		display: grid;
		padding: 6px 20px;
	}

	.log-row {
		display: grid;
		grid-template-columns: 68px 92px minmax(0, 1fr);
		gap: 16px;
		align-items: baseline;
		padding: 15px 0;
	}

	.log-row + .log-row {
		border-top: 1px solid var(--line);
	}

	.log-date {
		color: var(--dim);
		font-family: 'SF Mono', ui-monospace, Menlo, monospace;
		font-size: 12px;
		letter-spacing: 0.04em;
		text-transform: uppercase;
	}

	.log-tag {
		width: max-content;
		font-size: 11px;
		font-weight: 600;
		letter-spacing: 0.12em;
		text-transform: uppercase;
	}

	.log-tag.shipped {
		color: var(--up);
	}

	.log-tag.tuned {
		color: var(--accent);
	}

	.log-tag.hardened {
		color: var(--muted);
	}

	.log-row p {
		margin: 0;
		color: var(--muted);
		font-size: 14px;
		line-height: 1.65;
	}

	.log-note {
		margin: 14px 0 0;
		color: var(--dim);
		font-size: 12.5px;
	}

	/* ── Access ───────────────────────────────────────────────────────────── */

	.access-section {
		padding-top: clamp(28px, 4vw, 52px);
		padding-bottom: 7vh;
	}

	.access-panel,
	.success-panel {
		display: grid;
		grid-template-columns: minmax(0, 1fr) minmax(320px, 0.78fr);
		gap: 28px;
		align-items: end;
		padding: clamp(24px, 4vw, 40px);
	}

	.access-panel h2,
	.success-panel h2 {
		max-width: 640px;
	}

	.access-panel p,
	.success-panel p {
		max-width: 560px;
		margin: 14px 0 0;
		color: var(--muted);
		font-size: 14.5px;
		line-height: 1.7;
	}

	.perk-line {
		color: var(--dim) !important;
		font-size: 13px !important;
	}

	.email-row {
		display: grid;
		grid-template-columns: minmax(0, 1fr) auto;
		gap: 10px;
		align-items: center;
	}

	.email-row input {
		width: 100%;
		min-height: 46px;
		border: 1px solid var(--line-strong);
		border-radius: 4px;
		padding: 0 14px;
		background: rgba(0, 0, 0, 0.25);
		color: var(--text);
		font: inherit;
		font-size: 14.5px;
		outline: none;
		transition: border-color 160ms ease;
	}

	.email-row input:focus {
		border-color: var(--accent);
	}

	.email-row button {
		min-height: 46px;
		cursor: pointer;
	}

	.email-row button:disabled,
	.email-row input:disabled {
		cursor: not-allowed;
		opacity: 0.55;
	}

	.form-error {
		margin: 8px 0 0;
		color: var(--down) !important;
		font-size: 13.5px;
	}

	.success-panel {
		grid-template-columns: 72px minmax(0, 1fr);
		align-items: center;
	}

	.success-mark svg {
		width: 56px;
		height: 56px;
	}

	.success-mark circle,
	.success-mark path {
		fill: none;
		stroke: var(--accent);
		stroke-width: 2.5;
	}

	/* ── Footer ───────────────────────────────────────────────────────────── */

	.floor-footer {
		display: grid;
		gap: 10px;
		border-top: 1px solid var(--line);
		padding: 28px 0 36px;
	}

	.footer-brand {
		display: flex;
		flex-wrap: wrap;
		align-items: baseline;
		gap: 12px;
	}

	.footer-brand strong {
		font-size: 14px;
		font-weight: 620;
		letter-spacing: 0.01em;
	}

	.footer-brand span {
		color: var(--dim);
		font-size: 13px;
	}

	.footer-risk {
		max-width: 680px;
		margin: 0;
		color: var(--dim);
		font-size: 12px;
		line-height: 1.6;
	}

	.footer-legal {
		color: var(--dim);
		font-family: 'SF Mono', ui-monospace, Menlo, monospace;
		font-size: 11px;
		letter-spacing: 0.04em;
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

	/* ── Responsive ───────────────────────────────────────────────────────── */

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
			padding-top: 48px;
		}

		.academy-summary {
			border-right: 0;
			border-bottom: 1px solid var(--line);
		}

		.tape-cta {
			display: none;
		}
	}

	@media (max-width: 720px) {
		.page-shell {
			width: min(100% - 28px, 1180px);
		}

		h1 {
			font-size: clamp(2.1rem, 9vw, 2.8rem);
		}

		.chart-toolbar,
		.email-row,
		.ops-grid,
		.mini-feed-grid,
		.academy-selector,
		.asset-lanes,
		.academy-lab,
		.chart-metrics {
			grid-template-columns: 1fr;
		}

		.chart-toolbar,
		.email-row {
			display: grid;
			gap: 12px;
		}

		.timeframe-control {
			width: 100%;
			justify-content: space-between;
		}

		.timeframe-control button {
			width: 100%;
		}

		.signal-row {
			grid-template-columns: 1fr;
			gap: 8px;
		}

		.node-row {
			grid-template-columns: 1fr;
			gap: 8px;
		}

		.node-row small {
			text-align: left;
		}

		.stat-board {
			grid-template-columns: 1fr;
		}

		.stat-board div:nth-child(2n) {
			border-left: 0;
			padding-left: 0;
		}

		.stat-board div + div {
			border-top: 1px solid var(--line);
		}

		.log-row {
			grid-template-columns: 1fr;
			gap: 6px;
		}

		.chart-canvas {
			height: 320px;
		}
	}

	@media (prefers-reduced-motion: reduce) {
		*,
		*::before,
		*::after {
			animation-duration: 0.001ms !important;
			animation-delay: 0s !important;
			animation-iteration-count: 1 !important;
			scroll-behavior: auto !important;
			transition-duration: 0.001ms !important;
		}
	}
</style>

<script lang="ts">
	import { browser } from '$app/env';
	import { onMount, tick } from 'svelte';
	import type { Attachment } from 'svelte/attachments';
	import type {
		CandlestickData,
		HistogramData,
		IChartApi,
		IPriceLine,
		ISeriesApi,
		UTCTimestamp
	} from 'lightweight-charts';

	/* ═══════════════════════════════════════════════════════════════════════
	 * REVOLUTION TRADING PROS — REBUILD IN SESSION
	 * A trading floor, shot like a film. The page opens as a title sequence
	 * (letterboxed, graded, scored by the tape), then moves scene by scene
	 * through the desk: live market modules, the data engine, the scope of
	 * work, both universities, the build log, and first access.
	 *
	 * Everything on screen is mechanically real — charts tick, books reprice,
	 * prints stream — but no market data leaves this file: the page is
	 * prerendered and self-contained so it stands even when the API is down.
	 * ═══════════════════════════════════════════════════════════════════════ */

	type Direction = 'up' | 'down';
	type SceneId = 'hero' | 'note' | 'desk' | 'charts' | 'scope' | 'academy' | 'buildlog' | 'access';
	type SignalStatus = 'active' | 'watch' | 'cooling';
	type AcademyTrackId = 'day' | 'swing';
	type SessionState = 'pre' | 'open' | 'after' | 'closed';
	type TimeframeId = '1m' | '5m' | '15m' | '1H';
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

	interface BookLevel {
		price: number;
		size: number;
	}

	interface TapePrint {
		id: number;
		time: string;
		price: number;
		size: number;
		side: 'buy' | 'sell';
		block: boolean;
	}

	interface SectorTile {
		code: string;
		name: string;
		change: number;
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

	interface ScopeChapter {
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

	interface TimeframeConfig {
		seed: number;
		count: number;
		volatility: number;
		barSpacing: number;
		/** Seconds per bar — a 5m rail must produce 5-minute bars, not 1-minute. */
		interval: number;
	}

	const revealScenes: SceneId[] = [
		'hero',
		'note',
		'desk',
		'charts',
		'scope',
		'academy',
		'buildlog',
		'access'
	];

	/* Wider bar spacing than the library default: chunky candles read like a
	 * graded film frame at viewing distance, not a cramped analysis screen. */
	const TIMEFRAMES: Record<TimeframeId, TimeframeConfig> = {
		'1m': { seed: 1487, count: 96, volatility: 0.0038, barSpacing: 9, interval: 60 },
		'5m': { seed: 2861, count: 76, volatility: 0.0052, barSpacing: 11, interval: 300 },
		'15m': { seed: 5233, count: 64, volatility: 0.0071, barSpacing: 12, interval: 900 },
		'1H': { seed: 7919, count: 56, volatility: 0.0104, barSpacing: 14, interval: 3600 }
	};
	const timeframeIds = Object.keys(TIMEFRAMES) as TimeframeId[];

	/* ── Date-driven content ─────────────────────────────────────────────────
	 * The desk note, day counter, progress figure, and build log rotate with
	 * the calendar, so the page changes every trading day without a backend.
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

	const etClockFormatter = browser
		? new Intl.DateTimeFormat('en-US', {
				timeZone: 'America/New_York',
				hour12: false,
				hour: '2-digit',
				minute: '2-digit',
				second: '2-digit'
			})
		: null;

	const scopeChapters: ScopeChapter[] = [
		{
			number: '01',
			title: 'Market data layer',
			copy: 'Tick, quote, chart, watchlist, and scanner data normalized into one decision layer for equities and index products — a single clock across the whole desk.',
			metric: 'sub-20ms feed'
		},
		{
			number: '02',
			title: 'Scanner edge engine',
			copy: 'Relative volume, liquidity sweeps, sector leadership, dark-pool repeats, and momentum quality — ranked and scored before an alert ever reaches the desk.',
			metric: '12,847 symbols'
		},
		{
			number: '03',
			title: 'Options risk desk',
			copy: 'Delta, gamma, theta, IV rank, 0DTE exposure, spread structure, and event risk brought into one options workflow with position-level guardrails.',
			metric: 'Greeks + IV'
		},
		{
			number: '04',
			title: 'Live trading room',
			copy: 'The market-open stage rebuilt end to end — synced charts, shared tape, screen share, and a session recording pipeline indexed for replay.',
			metric: 'open to close'
		},
		{
			number: '05',
			title: 'Trading universities',
			copy: 'Day Trading University and Swing Trading University split stock execution, options strategy, risk protocol, and replay review into institution-grade tracks.',
			metric: 'stocks + options'
		}
	];

	const systemStatus = [
		{ label: 'Equities data', state: 'Normalized' },
		{ label: 'Options greeks', state: 'Mapped' },
		{ label: 'Scanner ranking', state: 'Active' },
		{ label: 'Risk limits', state: 'Enforced' },
		{ label: 'Replay library', state: 'Indexed' },
		{ label: 'Member access', state: 'Hardened' }
	];

	const stageLines = [
		'Resilient data routes',
		'Scanner model deployment',
		'Institutional curriculum migration',
		'Member room hardening'
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

	/* ── Live state ────────────────────────────────────────────────────────── */

	let mounted = $state(false);
	let reducedMotion = $state(false);
	let gsapActive = $state(false);
	let scrollProgress = $state(0);

	let dayNumber = $state(0);
	let buildProgress = $state(88);
	let progressDisplay = $state(88);
	let countdown = $state({ hours: '--', minutes: '--', seconds: '--' });
	let buildLog = $state<BuildLogEntry[]>(fallbackBuildLog);

	let etClock = $state('--:--:--');
	let sessionState = $state<SessionState>('closed');
	let sessionLabel = $state('Market status');
	let feedLatency = $state(14);

	let activeTimeframe = $state<TimeframeId>('5m');
	let activeAcademyTrack = $state<AcademyTrackId>('day');

	let email = $state('');
	let isSubmitting = $state(false);
	let isSubmitted = $state(false);
	let errorMessage = $state('');

	let visible = $state<Record<SceneId, boolean>>({
		hero: false,
		note: false,
		desk: false,
		charts: false,
		scope: false,
		academy: false,
		buildlog: false,
		access: false
	});

	let stats = $state({ traders: 0, scanners: 0, latency: 0, uptime: 0 });

	let liveTickers = $state<Ticker[]>([
		{ symbol: 'SPX', basePrice: 5892.33, price: 5892.33, change: 1.24, direction: 'up' },
		{ symbol: 'NQ', basePrice: 20845.25, price: 20845.25, change: 2.34, direction: 'up' },
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

	/* Order book — five levels a side on a penny grid around the live AAPL mid.
	 * AAPL's book is penny-quoted with a 1-2 cent spread; a 2-cent grid with a
	 * 4-cent spread reads wrong to anyone who has watched an L2. */
	let bookBids = $state<BookLevel[]>([
		{ price: 232.29, size: 1240 },
		{ price: 232.28, size: 860 },
		{ price: 232.27, size: 1580 },
		{ price: 232.26, size: 640 },
		{ price: 232.25, size: 2210 }
	]);
	let bookAsks = $state<BookLevel[]>([
		{ price: 232.31, size: 980 },
		{ price: 232.32, size: 1420 },
		{ price: 232.33, size: 760 },
		{ price: 232.34, size: 1130 },
		{ price: 232.35, size: 1890 }
	]);

	/* Time & sales — the print stream beside the book. */
	let prints = $state<TapePrint[]>([]);
	let printCounter = 0;

	let sectors = $state<SectorTile[]>([
		{ code: 'XLK', name: 'Technology', change: 1.62 },
		{ code: 'XLC', name: 'Communications', change: 0.94 },
		{ code: 'XLY', name: 'Consumer disc.', change: 0.58 },
		{ code: 'XLF', name: 'Financials', change: 0.31 },
		{ code: 'XLI', name: 'Industrials', change: 0.12 },
		{ code: 'XLB', name: 'Materials', change: -0.08 },
		{ code: 'XLE', name: 'Energy', change: -0.34 },
		{ code: 'XLP', name: 'Staples', change: -0.19 },
		{ code: 'XLV', name: 'Health care', change: 0.44 },
		{ code: 'XLU', name: 'Utilities', change: -0.52 },
		{ code: 'XLRE', name: 'Real estate', change: -0.71 }
	]);

	/* ── Chart plumbing ────────────────────────────────────────────────────── */

	let shellEl: HTMLDivElement;
	let heroChartEl: HTMLDivElement;
	let mainChartEl: HTMLDivElement;
	let aaplChartEl: HTMLDivElement;
	let nvdaChartEl: HTMLDivElement;

	let heroChart: IChartApi | null = null;
	let heroSeries: ISeriesApi<'Candlestick'> | null = null;
	let heroFloorSeries: ISeriesApi<'Area'> | null = null;
	let heroPriceLine: IPriceLine | null = null;
	let heroChartReady = false;
	let heroFeedTimer = 0;
	let heroLastCandle: CandlestickData | null = null;
	let heroTickCount = 0;

	let mainChart: IChartApi | null = null;
	let mainSeries: ISeriesApi<'Candlestick'> | null = null;
	let mainFloorSeries: ISeriesApi<'Area'> | null = null;
	let volumeSeries: ISeriesApi<'Histogram'> | null = null;
	let aaplChart: IChartApi | null = null;
	let aaplSeries: ISeriesApi<'Area'> | null = null;
	let nvdaChart: IChartApi | null = null;
	let nvdaSeries: ISeriesApi<'Area'> | null = null;
	let chartsReady = false;
	let chartInitGeneration = 0;

	let statsPlayed = false;
	let statsRaf = 0;
	let raf = 0;
	let notifyAbortController: AbortController | null = null;
	let gsapContext: { revert: () => void } | null = null;
	let masterTimeline: { progress: (value: number) => unknown } | null = null;
	let entranceDeadlinePassed = false;
	let priceFlashTimer = 0;

	let mainCandles = $state<CandlestickData[]>([]);
	let miniCloses = $state({ aapl: 232.3, nvda: 1475.2 });

	/* Hero feed — an always-on AAPL session rendered behind the title card. */
	const HERO_BASE_PRICE = 232.3;
	let heroPrice = $state(HERO_BASE_PRICE);
	let heroPriceDirection = $state<Direction>('up');
	let heroPriceFlash = $state<'' | 'up' | 'down'>('');
	let heroSessionLow = $state(HERO_BASE_PRICE);
	let heroSessionHigh = $state(HERO_BASE_PRICE);
	let heroOpenPrice = $state(HERO_BASE_PRICE);

	/* ── Derived readouts ──────────────────────────────────────────────────── */

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

	const latestMainPrice = $derived(mainCandles.at(-1)?.close ?? 5892.33);
	const mainDirection = $derived(
		(mainCandles.at(-1)?.close ?? 0) >= (mainCandles.at(-1)?.open ?? 0) ? 'up' : 'down'
	);

	const deskNote = $derived(deskNotes[(Math.max(dayNumber, 1) - 1) % deskNotes.length]);
	const noteNumber = $derived(dayNumber > 0 ? String(dayNumber).padStart(3, '0') : '···');
	const completedStages = $derived(buildProgress >= 97 ? 4 : 3);

	const heroChangePct = $derived(((heroPrice - heroOpenPrice) / heroOpenPrice) * 100);
	const heroRangeSpan = $derived(Math.max(heroSessionHigh - heroSessionLow, 0.01));
	const heroRangePosition = $derived(
		Math.min(100, Math.max(0, ((heroPrice - heroSessionLow) / heroRangeSpan) * 100))
	);

	const bookMaxSize = $derived(
		Math.max(...bookBids.map((level) => level.size), ...bookAsks.map((level) => level.size), 1)
	);
	const bookSpread = $derived(
		Math.max(0.01, (bookAsks[0]?.price ?? 0) - (bookBids[0]?.price ?? 0))
	);

	const sessionShort = $derived(
		sessionState === 'open'
			? 'OPEN'
			: sessionState === 'pre'
				? 'PRE'
				: sessionState === 'after'
					? 'AH'
					: 'CLSD'
	);

	/* ── Lifecycle ─────────────────────────────────────────────────────────── */

	onMount(() => {
		if (!browser) return;

		mounted = true;
		reducedMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches;

		refreshDailyContent();
		updateClocks();
		seedPrints();

		const cleanupReveal = setupRevealObserver();
		const cleanupIntervals = startLiveSystems();

		// Failsafe: if the title sequence hasn't finished shortly after load
		// (throttled tab, broken rAF), settle everything to its final frame.
		const entranceFailsafe = window.setTimeout(() => {
			entranceDeadlinePassed = true;
			masterTimeline?.progress(1);
		}, 2600);

		if (!reducedMotion) {
			void initMotion();
		}
		void initHeroChart();

		visible.hero = true;

		return () => {
			mounted = false;
			chartInitGeneration += 1;
			notifyAbortController?.abort();
			notifyAbortController = null;
			cancelAnimationFrame(raf);
			cancelAnimationFrame(statsRaf);
			window.clearTimeout(entranceFailsafe);
			window.clearTimeout(priceFlashTimer);
			window.clearInterval(heroFeedTimer);
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

	const mountHeroChart: Attachment<HTMLDivElement> = (node) => {
		heroChartEl = node;
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

	/* ── Calendar, clock, and session ──────────────────────────────────────── */

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

	function easternNow() {
		return new Date(new Date().toLocaleString('en-US', { timeZone: 'America/New_York' }));
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

	function updateClocks() {
		const nyNow = easternNow();

		// Countdown to the next daily update at the opening bell.
		const target = nextBuildDrop(nyNow);
		const diff = Math.max(0, target.getTime() - nyNow.getTime());
		countdown = {
			hours: String(Math.floor(diff / 3_600_000)).padStart(2, '0'),
			minutes: String(Math.floor((diff % 3_600_000) / 60_000)).padStart(2, '0'),
			seconds: String(Math.floor((diff % 60_000) / 1000)).padStart(2, '0')
		};

		etClock = etClockFormatter?.format(new Date()) ?? '--:--:--';

		// NYSE session windows, Eastern time.
		const day = nyNow.getDay();
		const minutes = nyNow.getHours() * 60 + nyNow.getMinutes();
		if (day === 0 || day === 6) {
			sessionState = 'closed';
			sessionLabel = 'Market closed';
		} else if (minutes >= 240 && minutes < 570) {
			sessionState = 'pre';
			sessionLabel = 'Pre-market';
		} else if (minutes >= 570 && minutes < 960) {
			sessionState = 'open';
			sessionLabel = 'Market open';
		} else if (minutes >= 960 && minutes < 1200) {
			sessionState = 'after';
			sessionLabel = 'After hours';
		} else {
			sessionState = 'closed';
			sessionLabel = 'Market closed';
		}
	}

	/* ── Deterministic market fiction ──────────────────────────────────────── */

	/* lightweight-charts renders UTC timestamps verbatim, but every other clock
	 * on this page speaks Eastern time — without this shift the SPX axis reads
	 * "18:10" while the HUD says 14:10 ET, which no trading audience forgives.
	 * Shifting the fictional timestamps by the current ET offset makes the
	 * UTC-rendered labels read as ET wall-clock time. */
	const ET_SHIFT_SEC = browser
		? Math.round(
				(new Date(new Date().toLocaleString('en-US', { timeZone: 'America/New_York' })).getTime() -
					Date.now()) /
					60_000
			) * 60
		: 0;

	function chartNow() {
		return (Math.floor(Date.now() / 1000) + ET_SHIFT_SEC) as UTCTimestamp;
	}

	function seededRandom(seed: number) {
		let value = seed % 2147483647;
		if (value <= 0) value += 2147483646;

		return () => {
			value = (value * 16807) % 2147483647;
			return (value - 1) / 2147483646;
		};
	}

	function generateCandles(
		seed: number,
		startPrice: number,
		count: number,
		volatility = 0.007,
		intervalSec = 60
	) {
		const random = seededRandom(seed);
		const candles: CandlestickData[] = [];
		// Bars land on interval boundaries in ET wall-clock time, so a 5m rail
		// shows :00/:05/:10 labels rather than arbitrary minutes.
		const now = Number(chartNow());
		const start = Math.floor((now - count * intervalSec) / intervalSec) * intervalSec;
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
				time: (start + i * intervalSec) as UTCTimestamp,
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

	/* Re-center a seeded walk so it closes exactly on its base price. The raw
	 * generator drifts a few percent over a long series, which would put the
	 * chart's last price visibly out of line with the same symbol on the tape —
	 * the kind of inconsistency a trading audience clocks immediately. */
	function recenter(candles: CandlestickData[], base: number, k: number): CandlestickData[] {
		const rawClose = candles.at(-1)?.close ?? base;
		const squeeze = (value: number) => Number((base + (value - rawClose) * k).toFixed(2));
		return candles.map((candle) => ({
			time: candle.time,
			open: squeeze(candle.open),
			high: squeeze(candle.high),
			low: squeeze(candle.low),
			close: squeeze(candle.close)
		}));
	}

	function volumeFor(candles: CandlestickData[]): HistogramData[] {
		// Volume follows the bar's own energy — bigger bodies print bigger volume,
		// with seeded noise on top. The previous modular ramp drew two perfect
		// staircases with a cliff, which reads as fake to anyone who trades.
		const random = seededRandom(4242);
		return candles.map((candle) => {
			const body = Math.abs(candle.close - candle.open);
			const range = Math.max(candle.high - candle.low, 0.0001);
			const energy = 0.5 + (body / range) * 0.7 + random() * 0.9;
			return {
				time: candle.time,
				value: Math.floor(300000 * energy + random() * 240000),
				color: candle.close >= candle.open ? 'rgba(53, 185, 140, 0.5)' : 'rgba(209, 96, 96, 0.42)'
			};
		});
	}

	function disposeCharts() {
		mainChart?.remove();
		aaplChart?.remove();
		nvdaChart?.remove();
		heroChart?.remove();
		mainChart = null;
		aaplChart = null;
		nvdaChart = null;
		heroChart = null;
		mainSeries = null;
		mainFloorSeries = null;
		aaplSeries = null;
		nvdaSeries = null;
		volumeSeries = null;
		heroSeries = null;
		heroFloorSeries = null;
		heroPriceLine = null;
		chartsReady = false;
		heroChartReady = false;
	}

	/* ── Hero backdrop feed ────────────────────────────────────────────────── */

	async function initHeroChart() {
		if (heroChartReady || !browser || !mounted || !heroChartEl) return;

		try {
			const { createChart, CandlestickSeries, AreaSeries, LineStyle } =
				await import('lightweight-charts');
			if (!mounted || heroChartReady || !heroChartEl) return;

			heroChart = createChart(heroChartEl, {
				autoSize: true,
				layout: {
					background: { color: 'transparent' },
					textColor: 'rgba(236, 234, 228, 0.4)',
					fontFamily: "'SF Mono', ui-monospace, Menlo, monospace",
					fontSize: 10,
					attributionLogo: false
				},
				grid: {
					vertLines: { visible: false },
					horzLines: { color: 'rgba(236, 234, 228, 0.03)' }
				},
				rightPriceScale: {
					visible: true,
					borderVisible: false,
					scaleMargins: { top: 0.14, bottom: 0.16 }
				},
				timeScale: {
					visible: false,
					rightOffset: 6,
					barSpacing: 11,
					lockVisibleTimeRangeOnResize: true
				},
				crosshair: {
					vertLine: { visible: false, labelVisible: false },
					horzLine: { visible: false, labelVisible: false }
				},
				handleScroll: false,
				handleScale: false
			});

			// A whisper of gold beneath the candles gives the tape depth without
			// changing the palette. Added first so it renders under the series.
			heroFloorSeries = heroChart.addSeries(AreaSeries, {
				lineColor: 'rgba(198, 161, 91, 0.32)',
				topColor: 'rgba(198, 161, 91, 0.09)',
				bottomColor: 'transparent',
				lineWidth: 1,
				priceLineVisible: false,
				lastValueVisible: false,
				crosshairMarkerVisible: false
			});

			heroSeries = heroChart.addSeries(CandlestickSeries, {
				upColor: '#35b98c',
				downColor: '#d16060',
				borderVisible: true,
				borderUpColor: '#3fd39e',
				borderDownColor: '#e07272',
				wickUpColor: 'rgba(63, 211, 158, 0.8)',
				wickDownColor: 'rgba(224, 114, 114, 0.8)',
				priceFormat: { type: 'price', precision: 2, minMove: 0.01 },
				priceLineVisible: false,
				lastValueVisible: false
			});

			// Tighter squeeze here than the desk charts: the backdrop reads as one
			// intraday session, so its range stays around 1.5%.
			const seed = recenter(
				generateCandles(232, HERO_BASE_PRICE, 96, 0.0035),
				HERO_BASE_PRICE,
				0.18
			);
			heroSeries.setData(seed);
			heroFloorSeries.setData(seed.map((candle) => ({ time: candle.time, value: candle.close })));
			heroLastCandle = seed.at(-1) ?? null;

			heroOpenPrice = seed[0]?.open ?? HERO_BASE_PRICE;
			heroSessionLow = Math.min(...seed.map((candle) => candle.low));
			heroSessionHigh = Math.max(...seed.map((candle) => candle.high));

			// The dashed bronze line doubles as the last-price element: it
			// carries both the reference line and the gold axis chip.
			heroPriceLine = heroSeries.createPriceLine({
				price: heroLastCandle?.close ?? HERO_BASE_PRICE,
				color: 'rgba(198, 161, 91, 0.5)',
				lineWidth: 1,
				lineStyle: LineStyle.Dashed,
				axisLabelVisible: true,
				axisLabelColor: 'rgba(198, 161, 91, 0.92)',
				axisLabelTextColor: '#07080b',
				title: ''
			});
			if (heroLastCandle) {
				heroPrice = heroLastCandle.close;
				heroPriceDirection = heroLastCandle.close >= heroLastCandle.open ? 'up' : 'down';
			}
			heroChart.timeScale().scrollToRealTime();
			heroChartReady = true;

			// The feed never stops; reduced-motion readers get a still frame.
			if (!reducedMotion) {
				heroFeedTimer = window.setInterval(tickHeroFeed, 700);
			}
		} catch (error) {
			if (mounted) console.error('[Maintenance] Failed to initialize hero chart', error);
		}
	}

	function tickHeroFeed() {
		if (!heroSeries || !heroLastCandle) return;

		heroTickCount += 1;

		// Mean-reverting jitter keeps the tape believable around the base price.
		const reversion = (HERO_BASE_PRICE - heroLastCandle.close) * 0.02;
		const move = (Math.random() - 0.5) * 0.16 + reversion;

		if (heroTickCount % 5 === 0) {
			// Roll a fresh candle roughly every 3.5 seconds.
			const open = heroLastCandle.close;
			const close = Number((open + move).toFixed(2));
			heroLastCandle = {
				time: (Number(heroLastCandle.time) + 60) as UTCTimestamp,
				open,
				high: Math.max(open, close),
				low: Math.min(open, close),
				close
			};
		} else {
			const close = Number((heroLastCandle.close + move).toFixed(2));
			heroLastCandle = {
				...heroLastCandle,
				close,
				high: Math.max(heroLastCandle.high, close),
				low: Math.min(heroLastCandle.low, close)
			};
		}

		heroSeries.update(heroLastCandle);
		heroFloorSeries?.update({ time: heroLastCandle.time, value: heroLastCandle.close });
		heroPriceLine?.applyOptions({ price: heroLastCandle.close });

		const previous = heroPrice;
		heroPrice = heroLastCandle.close;
		heroPriceDirection = heroLastCandle.close >= heroLastCandle.open ? 'up' : 'down';
		heroSessionLow = Math.min(heroSessionLow, heroLastCandle.low);
		heroSessionHigh = Math.max(heroSessionHigh, heroLastCandle.high);

		// Direction flash on the big figure — cleared after the pulse lands.
		if (heroPrice !== previous) {
			heroPriceFlash = heroPrice > previous ? 'up' : 'down';
			window.clearTimeout(priceFlashTimer);
			priceFlashTimer = window.setTimeout(() => {
				heroPriceFlash = '';
			}, 520);
		}
	}

	/* ── Market data scene ─────────────────────────────────────────────────── */

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

			const { createChart, CandlestickSeries, AreaSeries, HistogramSeries, LineStyle } =
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

			const timeframe = TIMEFRAMES[activeTimeframe];
			const seededMain = recenter(
				generateCandles(
					timeframe.seed,
					5892.33,
					timeframe.count,
					timeframe.volatility,
					timeframe.interval
				),
				5892.33,
				0.35
			);
			const seededAapl = recenter(generateCandles(412, 232.3, 48, 0.0065), 232.3, 0.35);
			const seededNvda = recenter(generateCandles(911, 1475.2, 48, 0.0078), 1475.2, 0.35);

			const commonLayout = {
				background: { color: 'transparent' },
				textColor: 'rgba(236, 234, 228, 0.62)',
				fontFamily:
					'Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, sans-serif',
				fontSize: 11,
				attributionLogo: false
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
					secondsVisible: false,
					barSpacing: timeframe.barSpacing,
					rightOffset: 5
				},
				crosshair: {
					vertLine: { color: 'rgba(198, 161, 91, 0.4)', style: 3 },
					horzLine: { color: 'rgba(198, 161, 91, 0.4)', style: 3 }
				}
			});

			// A luminous gold floor beneath the candles — same grammar as the hero
			// backdrop — so the tape sits in an atmosphere instead of on a void.
			// Added first so it renders under the candle series.
			mainFloorSeries = mainChart.addSeries(AreaSeries, {
				lineColor: 'rgba(198, 161, 91, 0.28)',
				topColor: 'rgba(198, 161, 91, 0.08)',
				bottomColor: 'transparent',
				lineWidth: 1,
				priceLineVisible: false,
				lastValueVisible: false,
				crosshairMarkerVisible: false
			});

			mainSeries = mainChart.addSeries(CandlestickSeries, {
				upColor: '#35b98c',
				downColor: '#d16060',
				borderUpColor: '#3fd39e',
				borderDownColor: '#e07272',
				wickUpColor: 'rgba(63, 211, 158, 0.85)',
				wickDownColor: 'rgba(224, 114, 114, 0.85)',
				priceLineVisible: true,
				priceLineColor: 'rgba(198, 161, 91, 0.55)',
				priceLineStyle: LineStyle.Dashed,
				lastValueVisible: true
			});

			volumeSeries = mainChart.addSeries(HistogramSeries, {
				priceFormat: { type: 'volume' },
				priceScaleId: '',
				// No axis pill or price line for volume — the right axis belongs to
				// price alone; the volume tag was occluding price labels.
				lastValueVisible: false,
				priceLineVisible: false
			});

			mainFloorSeries.setData(
				seededMain.map((candle) => ({ time: candle.time, value: candle.close }))
			);
			mainSeries.setData(seededMain);
			volumeSeries.priceScale().applyOptions({ scaleMargins: { top: 0.85, bottom: 0 } });
			volumeSeries.setData(volumeFor(seededMain));

			aaplChart = createMiniChart(createChart, aaplChartEl);
			nvdaChart = createMiniChart(createChart, nvdaChartEl);

			aaplSeries = aaplChart.addSeries(AreaSeries, {
				lineColor: 'rgba(168, 196, 220, 0.95)',
				topColor: 'rgba(168, 196, 220, 0.2)',
				bottomColor: 'rgba(168, 196, 220, 0)',
				lineWidth: 2
			});

			nvdaSeries = nvdaChart.addSeries(AreaSeries, {
				lineColor: 'rgba(216, 181, 116, 0.95)',
				topColor: 'rgba(216, 181, 116, 0.2)',
				bottomColor: 'rgba(216, 181, 116, 0)',
				lineWidth: 2
			});

			aaplSeries.setData(seededAapl.map((candle) => ({ time: candle.time, value: candle.close })));
			nvdaSeries.setData(seededNvda.map((candle) => ({ time: candle.time, value: candle.close })));

			mainChart.timeScale().fitContent();
			aaplChart.timeScale().fitContent();
			nvdaChart.timeScale().fitContent();

			mainCandles = seededMain;
			miniCloses = {
				aapl: seededAapl.at(-1)?.close ?? 232.3,
				nvda: seededNvda.at(-1)?.close ?? 1475.2
			};
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
				fontSize: 10,
				attributionLogo: false
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

	/* The timeframe rail is a real control: switching regenerates the tape at
	 * that granularity and re-fits the view — not a decorative row of chips. */
	function setTimeframe(next: TimeframeId) {
		if (next === activeTimeframe) return;
		activeTimeframe = next;

		if (!chartsReady || !mainSeries || !mainChart) return;
		const timeframe = TIMEFRAMES[next];
		const candles = recenter(
			generateCandles(
				timeframe.seed,
				5892.33,
				timeframe.count,
				timeframe.volatility,
				timeframe.interval
			),
			5892.33,
			0.35
		);
		mainSeries.setData(candles);
		mainFloorSeries?.setData(candles.map((candle) => ({ time: candle.time, value: candle.close })));
		volumeSeries?.setData(volumeFor(candles));
		mainChart.timeScale().applyOptions({ barSpacing: timeframe.barSpacing });
		mainChart.timeScale().fitContent();
		mainCandles = candles;
	}

	function rollMainCandle() {
		const last = mainCandles.at(-1);
		if (!last || !mainSeries) return;

		// The live tick only develops the CURRENT bar. Appending wall-clock bars
		// would break interval alignment on the 5m/15m/1H rails — a new bar can
		// only legitimately open on an interval boundary.
		const move = (Math.random() - 0.48) * 7.4;
		last.close = Number((last.close + move).toFixed(2));
		last.high = Math.max(last.high, last.close);
		last.low = Math.min(last.low, last.close);

		mainSeries.update(last);
		mainFloorSeries?.update({ time: last.time, value: last.close });
		volumeSeries?.update({
			time: last.time,
			value: Math.floor(520000 + Math.random() * 640000),
			color: last.close >= last.open ? 'rgba(53, 185, 140, 0.5)' : 'rgba(209, 96, 96, 0.42)'
		});
	}

	/* ── The desk, live: book, prints, sectors, scanner ────────────────────── */

	function repriceBook() {
		const mid = heroPrice;
		const drift = () => Math.round((Math.random() - 0.5) * 380);

		bookBids = bookBids.map((level, index) => ({
			price: Number((mid - 0.01 - index * 0.01).toFixed(2)),
			size: Math.max(120, Math.min(2600, level.size + drift()))
		}));
		bookAsks = bookAsks.map((level, index) => ({
			price: Number((mid + 0.01 + index * 0.01).toFixed(2)),
			size: Math.max(120, Math.min(2600, level.size + drift()))
		}));
	}

	function printTime() {
		return etClockFormatter?.format(new Date()) ?? '--:--:--';
	}

	function makePrint(): TapePrint {
		printCounter += 1;
		const side: 'buy' | 'sell' = Math.random() > 0.47 ? 'buy' : 'sell';
		const offset = (Math.random() * bookSpread) / 2;
		const price = Number((side === 'buy' ? heroPrice + offset : heroPrice - offset).toFixed(2));
		const block = Math.random() > 0.93;
		const size = block
			? Math.floor(2400 + Math.random() * 7600)
			: Math.floor(100 + Math.random() * 900);

		return { id: printCounter, time: printTime(), price, size, side, block };
	}

	function seedPrints() {
		const seeded: TapePrint[] = [];
		for (let i = 0; i < 12; i += 1) seeded.push(makePrint());
		prints = seeded;
	}

	function streamPrint() {
		const burst = Math.random() > 0.72 ? 2 : 1;
		const next = [...prints];
		for (let i = 0; i < burst; i += 1) next.unshift(makePrint());
		prints = next.slice(0, 14);
	}

	function driftSectors() {
		sectors = sectors.map((sector) => ({
			...sector,
			change: Number(
				Math.max(-2.8, Math.min(2.8, sector.change + (Math.random() - 0.5) * 0.14)).toFixed(2)
			)
		}));
	}

	function sectorHeat(change: number) {
		// Interpolate panel color by signed magnitude; text stays ivory.
		const magnitude = Math.min(1, Math.abs(change) / 2.2);
		const alpha = 0.05 + magnitude * 0.34;
		return change >= 0
			? `rgba(46, 156, 119, ${alpha.toFixed(3)})`
			: `rgba(194, 85, 85, ${alpha.toFixed(3)})`;
	}

	function startLiveSystems() {
		const timers: number[] = [];

		timers.push(window.setInterval(updateClocks, 1000));
		timers.push(window.setInterval(refreshDailyContent, 3_600_000));

		timers.push(
			window.setInterval(() => {
				feedLatency = 11 + Math.floor(Math.random() * 8);
			}, 2200)
		);

		timers.push(
			window.setInterval(() => {
				liveTickers = liveTickers.map((ticker) => {
					// VIX moves in percentage terms far more than an index; and the
					// futures (ES/NQ) only ever print on the CME 0.25 tick grid —
					// a 5891.64 ES print is the kind of detail a trader clocks.
					const scale = ticker.symbol === 'VIX' ? 0.0042 : 0.0009;
					const tick = ticker.symbol === 'ES' || ticker.symbol === 'NQ' ? 0.25 : 0.01;
					const move = (Math.random() - 0.48) * ticker.basePrice * scale;
					const price = Number((Math.round((ticker.price + move) / tick) * tick).toFixed(2));
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
				if (chartsReady) rollMainCandle();
			}, 1400)
		);

		if (!reducedMotion) {
			timers.push(window.setInterval(repriceBook, 1100));
			timers.push(window.setInterval(streamPrint, 850));
			timers.push(window.setInterval(driftSectors, 3200));
		} else {
			// A single settle so reduced-motion readers still see live-shaped data.
			repriceBook();
			driftSectors();
		}

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

	/* ── Reveal + scroll ───────────────────────────────────────────────────── */

	function setupRevealObserver() {
		if (!browser || !('IntersectionObserver' in window)) {
			revealScenes.forEach((scene) => {
				visible[scene] = true;
			});
			void initCharts();
			playStats();
			return;
		}

		const observer = new IntersectionObserver(
			(entries) => {
				for (const entry of entries) {
					if (!entry.isIntersecting) continue;
					const id = entry.target.id as SceneId;
					visible[id] = true;

					if (id === 'charts') void initCharts();
					if (id === 'scope') playStats();
				}
			},
			{ root: shellEl, threshold: 0.22, rootMargin: '0px 0px -8% 0px' }
		);

		revealScenes.forEach((id) => {
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

	/* ── Motion: the title sequence ────────────────────────────────────────────
	 * Shot like a cold open. The chrome slides in, the letterbox mattes close,
	 * the backdrop resolves from a slow push-in, the title rises out of its
	 * masks, the price figure pulls sharp, and the status slate lands last.
	 * The mattes retract as the viewer scrolls past the title card. Loaded
	 * dynamically; the prerendered page is complete without it, and
	 * reduced-motion skips the entire layer.
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

		const settle = { autoAlpha: 1, y: 0, filter: 'blur(0px)' };

		const master = gsap.timeline({ defaults: { ease: 'power3.out', duration: 0.7 } });
		masterTimeline = master;

		master
			.addLabel('chrome', 0)
			.fromTo(
				'.chrome-top',
				{ yPercent: -100 },
				{ yPercent: 0, duration: 0.8, ease: 'power2.out' },
				'chrome'
			)
			.fromTo(
				'.hud',
				{ yPercent: 100 },
				{ yPercent: 0, duration: 0.8, ease: 'power2.out' },
				'chrome'
			)
			.fromTo(
				'.letterbox.top',
				{ scaleY: 0 },
				{ scaleY: 1, transformOrigin: 'top center', duration: 0.9, ease: 'power2.inOut' },
				'chrome'
			)
			.fromTo(
				'.letterbox.bottom',
				{ scaleY: 0 },
				{ scaleY: 1, transformOrigin: 'bottom center', duration: 0.9, ease: 'power2.inOut' },
				'chrome'
			)
			.addLabel('backdrop', 0.25)
			.fromTo(
				'.hero-chart',
				{ autoAlpha: 0, scale: 1.06, clipPath: 'inset(0% 0% 14% 0%)' },
				{ autoAlpha: 1, scale: 1, clipPath: 'inset(0% 0% 0% 0%)', duration: 2.2, ease: 'expo.out' },
				'backdrop'
			)
			.fromTo(
				'.hero-flare',
				{ autoAlpha: 0, xPercent: -18 },
				{ autoAlpha: 1, xPercent: 0, duration: 2, ease: 'power2.out' },
				'backdrop+=0.15'
			)
			.fromTo(
				'.hero-watermark',
				{ autoAlpha: 0, x: '-2%' },
				{ autoAlpha: 0.9, x: '0%', duration: 1.8, ease: 'power2.out' },
				'backdrop+=0.1'
			)
			.to('#hero .eyebrow', { ...settle, duration: 0.6 }, 0.45)
			.addLabel('headline', 0.58)
			.fromTo(
				'#hero .hero-title .line-inner',
				{ y: '108%', rotation: 0.4 },
				{ y: '0%', rotation: 0, duration: 1.05, ease: 'expo.out', stagger: 0.16 },
				'headline'
			)
			.addLabel('price', 0.95)
			.to('#hero .hero-quote', { ...settle, duration: 0.9 }, 'price')
			.addLabel('copy', 1.1)
			.to('#hero .lede', { ...settle }, 'copy')
			.to('#hero .hero-actions', { ...settle }, 'copy+=0.14')
			.to('#hero .hero-note', { ...settle, duration: 0.6 }, 'copy+=0.26')
			.to('#hero .drop-timer', { ...settle }, 'copy+=0.36')
			.addLabel('card', 1.35)
			.to('#hero .status-card', { ...settle, duration: 1.1 }, 'card')
			.addLabel('cue', 2.2)
			.to('#hero .scroll-cue', { autoAlpha: 1, duration: 0.5 }, 'cue')
			.fromTo(
				'#hero .scroll-cue i',
				{ scaleX: 0 },
				{ scaleX: 1, transformOrigin: 'left center', duration: 0.5, ease: 'power2.out' },
				'cue+=0.1'
			);

		// The progress figure settles from its fallback to today's value.
		const progressProxy = { value: Math.max(0, buildProgress - 6) };
		progressDisplay = progressProxy.value;
		master.to(
			progressProxy,
			{
				value: buildProgress,
				duration: 1.1,
				ease: 'power1.out',
				onUpdate: () => {
					progressDisplay = Math.round(progressProxy.value);
				}
			},
			1.1
		);

		// The mattes retract as the title card scrolls away — the film opens up.
		gsap.to('.letterbox.top', {
			scaleY: 0,
			transformOrigin: 'top center',
			ease: 'none',
			scrollTrigger: { trigger: '#hero', start: 'top top', end: '62% top', scrub: 0.6 }
		});
		gsap.to('.letterbox.bottom', {
			scaleY: 0,
			transformOrigin: 'bottom center',
			ease: 'none',
			scrollTrigger: { trigger: '#hero', start: 'top top', end: '62% top', scrub: 0.6 }
		});

		// Depth: layered parallax as the hero leaves the frame.
		gsap.to('.hero-chart', {
			yPercent: -8,
			ease: 'none',
			scrollTrigger: { trigger: '#hero', start: 'top top', end: 'bottom top', scrub: 0.8 }
		});
		gsap.to('.hero-watermark', {
			y: -110,
			ease: 'none',
			scrollTrigger: { trigger: '#hero', start: 'top top', end: 'bottom top', scrub: 0.8 }
		});
		gsap.to('.note-card blockquote', {
			yPercent: -3,
			ease: 'none',
			scrollTrigger: { trigger: '#note', start: 'top bottom', end: 'bottom top', scrub: 0.8 }
		});

		// Scene grammar: the slate rises, the heading wipes open on a clip-path,
		// then content blocks stage in with a soft focus pull.
		const scenes = gsap.utils.toArray<HTMLElement>('.page-shell section:not(#hero)');
		scenes.forEach((scene) => {
			const kicker = scene.querySelector('.scene-slate');
			const heading = scene.querySelector('.scene-head h2');
			const blocks = scene.querySelectorAll('[data-cine] > *, [data-cine-self]');

			const timeline = gsap.timeline({
				defaults: { ease: 'power4.out' },
				scrollTrigger: { trigger: scene, start: 'top 80%', once: true }
			});

			if (kicker) {
				timeline.from(kicker, { autoAlpha: 0, y: 12, duration: 0.55, ease: 'power3.out' });
			}
			if (heading) {
				timeline.fromTo(
					heading,
					{ clipPath: 'inset(0% 0% 100% 0%)', y: 20 },
					{ clipPath: 'inset(0% 0% 0% 0%)', y: 0, duration: 0.9 },
					'<0.1'
				);
			}
			if (blocks.length) {
				timeline.from(
					blocks,
					{
						autoAlpha: 0,
						y: 18,
						filter: 'blur(4px)',
						duration: 0.7,
						stagger: 0.08,
						ease: 'power3.out'
					},
					'<0.2'
				);
			}
		});

		// If the settle deadline already passed while GSAP was still loading
		// (slow network, throttled tab), jump straight to the final frame.
		if (entranceDeadlinePassed) {
			master.progress(1);
		}
	}

	/* ── Access ────────────────────────────────────────────────────────────── */

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

	function formatSize(value: number) {
		return value >= 1000 ? `${(value / 1000).toFixed(1)}k` : String(value);
	}
</script>

<div
	class={{ 'maintenance-experience': true, mounted, 'gsap-on': gsapActive }}
	style:--scroll-progress={scrollProgress}
	{@attach mountShell}
	onscroll={handleScroll}
>
	<div class="aurora" aria-hidden="true"></div>
	<div class="grain" aria-hidden="true"></div>
	<div class="vignette" aria-hidden="true"></div>

	<div class="letterbox top" aria-hidden="true"></div>
	<div class="letterbox bottom" aria-hidden="true"></div>

	<header class="chrome-top" aria-label="Market tape">
		<span class="chrome-brand">RTP<i>//</i>REBUILD</span>
		<div class="tape-viewport">
			<div class="tape-track" aria-hidden="true">
				{#each [...liveTickers, ...liveTickers, ...liveTickers] as ticker, index (ticker.symbol + '-' + index)}
					<div class="tape-item">
						<span class="tape-symbol">{ticker.symbol}</span>
						<span class="tape-price">{ticker.price.toFixed(2)}</span>
						<span class={{ 'tape-change': true, up: ticker.change >= 0, down: ticker.change < 0 }}>
							{ticker.change >= 0 ? '▲' : '▼'}
							{ticker.change >= 0 ? '+' : ''}{ticker.change.toFixed(2)}%
						</span>
					</div>
				{/each}
			</div>
		</div>
		<a class="tape-cta" href="#access">Request access</a>
	</header>

	<main class="page-shell">
		<!-- ═══ TITLE CARD ═══════════════════════════════════════════════════ -->
		<section id="hero" class={{ hero: true, reveal: true, visible: visible.hero }}>
			<span class="hero-watermark" aria-hidden="true">AAPL</span>

			<div class="hero-chart" aria-hidden="true">
				<div class="hero-chart-canvas" {@attach mountHeroChart}></div>
			</div>
			<div class="hero-scrim" aria-hidden="true"></div>
			<div class="hero-flare" aria-hidden="true"></div>

			<div class="hero-copy">
				<p class="eyebrow" data-entrance>
					<span class="eyebrow-presents">Revolution Trading Pros presents</span>
					<span class="eyebrow-day">Rebuild · Day {dayNumber || '—'}</span>
				</p>

				<!-- The line-inner spans deliberately carry no data-entrance: that CSS
				     gate hides via opacity, but their entrance tween animates only the
				     masked translateY — an alpha gate would never be released and the
				     headline would stay invisible. The .line overflow mask plus the
				     .gsap-on transform gate hides them until the rise plays. -->
				<h1 class="hero-title">
					<span class="line"><span class="line-inner">The market didn’t wait.</span></span>
					<span class="line"
						><span class="line-inner">Neither did <em class="serif-accent">we.</em></span></span
					>
				</h1>

				<div class="hero-lower">
					<div class="hero-lower-copy">
						<div class="hero-quote" data-entrance aria-label="AAPL live quote">
							<div class="quote-head">
								<span class="quote-symbol">AAPL</span>
								<span class="quote-live"><i></i>Live feed</span>
							</div>
							<div
								class={{
									'quote-figure': true,
									up: heroPriceDirection === 'up',
									down: heroPriceDirection === 'down',
									'flash-up': heroPriceFlash === 'up',
									'flash-down': heroPriceFlash === 'down'
								}}
							>
								{heroPrice.toFixed(2)}
							</div>
							<div class="quote-meta">
								<span class={{ up: heroChangePct >= 0, down: heroChangePct < 0 }}>
									{heroChangePct >= 0 ? '+' : ''}{heroChangePct.toFixed(2)}%
								</span>
								<span class="quote-session">session</span>
							</div>
							<div class="range-meter" aria-hidden="true">
								<span class="range-low">{heroSessionLow.toFixed(2)}</span>
								<div class="range-track">
									<i style:left={`${heroRangePosition}%`}></i>
								</div>
								<span class="range-high">{heroSessionHigh.toFixed(2)}</span>
							</div>
						</div>

						<p class="lede" data-entrance>
							Revolution Trading Pros is being rebuilt end to end — faster market data, more
							rigorous scanners, a live trading room, and two universities run to an institutional
							standard. Progress is posted here every trading day.
						</p>

						<div class="hero-actions" data-entrance>
							<a class="primary-link" href="#access">Request first access</a>
							<a class="ghost-link" href="#desk">Watch the desk</a>
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
							<span class="timer-clock"
								>{countdown.hours}:{countdown.minutes}:{countdown.seconds}</span
							>
							<span class="timer-meta">9:30 AM ET · Monday–Friday</span>
						</p>
					</div>

					<aside class="status-card panel" aria-label="Rebuild status" data-entrance>
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
										{index < completedStages
											? 'Complete'
											: index === completedStages
												? 'Underway'
												: 'Queued'}
									</span>
								</li>
							{/each}
						</ol>

						<div class="card-footline">
							<span>Day {dayNumber || '—'}</span>
							<span>Next update {countdown.hours}:{countdown.minutes}:{countdown.seconds}</span>
						</div>
					</aside>
				</div>
			</div>

			<div class="scroll-cue" aria-hidden="true" data-entrance>
				<span>Scroll</span>
				<i></i>
			</div>
		</section>

		<!-- ═══ SCENE 01 · DESK NOTE ═════════════════════════════════════════ -->
		<section id="note" class={{ 'note-section': true, reveal: true, visible: visible.note }}>
			<div class="scene-head">
				<p class="scene-slate">
					<span class="slate-no">Scene 01</span><span class="slate-title">Daily desk note</span>
				</p>
				<h2>One disciplined idea, every trading day.</h2>
			</div>

			<figure class="note-card panel panel-feature" data-cine-self>
				<blockquote>{deskNote}</blockquote>
				<p class="note-foot">A new note is posted at the open, Monday through Friday.</p>
				<figcaption class="note-meta">
					<span>Desk note</span>
					<strong>№ {noteNumber}</strong>
				</figcaption>
			</figure>
		</section>

		<!-- ═══ SCENE 02 · THE DESK, LIVE ════════════════════════════════════ -->
		<section id="desk" class={{ 'desk-section': true, reveal: true, visible: visible.desk }}>
			<div class="scene-head">
				<p class="scene-slate">
					<span class="slate-no">Scene 02</span><span class="slate-title">The desk, live</span>
				</p>
				<h2>Live to the tick.</h2>
				<p class="scene-lede">
					Order book, tape, sector map, and the scanner feed — the same instruments the rebuilt
					platform ships with, running on this page right now. Outside market hours, the desk
					replays the previous session through the new engine.
				</p>
			</div>

			<div class="desk-grid" data-cine>
				<article class="desk-module panel" aria-label="AAPL order book">
					<div class="module-head">
						<span class="module-title">Order book</span>
						<span class="module-tag">AAPL · L2</span>
					</div>
					<div class="book-columns">
						<div class="book-side bids">
							<div class="book-legend"><span>Size</span><span>Bid</span></div>
							{#each bookBids as level, levelIndex (levelIndex)}
								<div class="book-row">
									<i class="depth" style:width={`${(level.size / bookMaxSize) * 100}%`}></i>
									<span class="book-size">{formatNumber(level.size)}</span>
									<span class="book-price">{level.price.toFixed(2)}</span>
								</div>
							{/each}
						</div>
						<div class="book-side asks">
							<div class="book-legend"><span>Ask</span><span>Size</span></div>
							{#each bookAsks as level, levelIndex (levelIndex)}
								<div class="book-row">
									<i class="depth" style:width={`${(level.size / bookMaxSize) * 100}%`}></i>
									<span class="book-price">{level.price.toFixed(2)}</span>
									<span class="book-size">{formatNumber(level.size)}</span>
								</div>
							{/each}
						</div>
					</div>
					<div class="module-foot">
						<span>Spread</span>
						<strong>{bookSpread.toFixed(2)}</strong>
					</div>
				</article>

				<article class="desk-module panel" aria-label="Time and sales">
					<div class="module-head">
						<span class="module-title">Time &amp; sales</span>
						<span class="module-tag"
							>{sessionState === 'open' ? 'Live prints' : 'Session replay'}</span
						>
					</div>
					<div class="prints-stream" role="log" aria-live="off" aria-label="Recent prints">
						{#each prints as print (print.id)}
							<div
								class={{
									'print-row': true,
									buy: print.side === 'buy',
									sell: print.side === 'sell',
									block: print.block
								}}
							>
								<span class="print-time">{print.time}</span>
								<span class="print-price">{print.price.toFixed(2)}</span>
								<span class="print-size">{formatSize(print.size)}</span>
								<span class="print-side">{print.side === 'buy' ? 'B' : 'S'}</span>
							</div>
						{/each}
					</div>
					<div class="module-foot">
						<span>Feed latency</span>
						<strong>{feedLatency}ms</strong>
					</div>
				</article>

				<article class="desk-module panel wide" aria-label="Sector heat map">
					<div class="module-head">
						<span class="module-title">Sector map</span>
						<span class="module-tag">S&amp;P sectors · intraday</span>
					</div>
					<div class="sector-grid">
						{#each sectors as sector (sector.code)}
							<div class="sector-tile" style:background={sectorHeat(sector.change)}>
								<span class="sector-code">{sector.code}</span>
								<span class="sector-name">{sector.name}</span>
								<strong class={{ up: sector.change >= 0, down: sector.change < 0 }}>
									{sector.change >= 0 ? '+' : ''}{sector.change.toFixed(2)}%
								</strong>
							</div>
						{/each}
					</div>
				</article>

				<article class="desk-module panel wide" aria-label="Scanner feed">
					<div class="module-head">
						<span class="module-title">Scanner feed</span>
						<span class="module-tag">{activeSignals} active · avg {averageConfidence}%</span>
					</div>
					<div class="signal-stream">
						{#each scannerSignals as signal (signal.id)}
							<div class={{ 'signal-row': true, cooling: signal.status === 'cooling' }}>
								<div class="signal-id">
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
							</div>
						{/each}
					</div>
					<div class="module-foot">
						<span>Symbols monitored</span>
						<strong>12,847</strong>
					</div>
				</article>
			</div>
		</section>

		<!-- ═══ SCENE 03 · MARKET DATA ═══════════════════════════════════════ -->
		<section id="charts" class={{ 'charts-section': true, reveal: true, visible: visible.charts }}>
			<div class="scene-head">
				<p class="scene-slate">
					<span class="slate-no">Scene 03</span><span class="slate-title">Market data</span>
				</p>
				<h2>The data engine never went dark.</h2>
			</div>

			<div class="chart-stage panel panel-feature" data-cine-self>
				<div class="chart-toolbar">
					<div>
						<span class="market-label">SPX · Upgrade feed</span>
						<strong class={{ up: mainDirection === 'up', down: mainDirection === 'down' }}>
							{latestMainPrice.toFixed(2)}
						</strong>
					</div>
					<div class="timeframe-control" aria-label="Chart timeframe">
						{#each timeframeIds as timeframe (timeframe)}
							<button
								type="button"
								class={{ active: activeTimeframe === timeframe }}
								aria-pressed={activeTimeframe === timeframe}
								onclick={() => setTimeframe(timeframe)}>{timeframe}</button
							>
						{/each}
					</div>
				</div>

				<div class="chart-canvas" {@attach mountMainChart}></div>

				<div class="chart-metrics">
					<div><span>Latency</span><strong>{feedLatency}ms</strong></div>
					<div><span>Ticks/min</span><strong>4.2M</strong></div>
					<div>
						<span>Avg. confidence</span>
						<strong>{averageConfidence}%</strong>
					</div>
				</div>
			</div>

			<div class="mini-feed-grid" data-cine>
				<div class="mini-feed panel">
					<div>
						<span>AAPL</span>
						<strong>{miniCloses.aapl.toFixed(2)}</strong>
					</div>
					<div class="mini-chart" {@attach mountAaplChart}></div>
				</div>

				<div class="mini-feed panel">
					<div>
						<span>NVDA</span>
						<strong>{miniCloses.nvda.toFixed(2)}</strong>
					</div>
					<div class="mini-chart" {@attach mountNvdaChart}></div>
				</div>
			</div>
		</section>

		<!-- ═══ SCENE 04 · SCOPE OF WORK ═════════════════════════════════════ -->
		<section id="scope" class={{ 'scope-section': true, reveal: true, visible: visible.scope }}>
			<div class="scene-head">
				<p class="scene-slate">
					<span class="slate-no">Scene 04</span><span class="slate-title">Scope of work</span>
				</p>
				<h2>Every system, rebuilt to institutional spec.</h2>
			</div>

			<div class="scope-stage">
				<div class="scope-manifest" data-cine>
					{#each scopeChapters as chapter (chapter.number)}
						<article class="scope-row">
							<span class="scope-index">{chapter.number}</span>
							<div class="scope-body">
								<h3>{chapter.title}</h3>
								<p>{chapter.copy}</p>
							</div>
							<span class="scope-metric">{chapter.metric}</span>
						</article>
					{/each}
				</div>

				<div class="scope-rail">
					<div class="systems-panel panel" aria-label="System status" data-cine-self>
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

					<div class="node-list panel" data-cine-self aria-label="Data-center load">
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

					<div class="stat-board panel" data-cine-self>
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
			</div>
		</section>

		<!-- ═══ SCENE 05 · TRADING UNIVERSITIES ══════════════════════════════ -->
		<section
			id="academy"
			class={{ 'academy-section': true, reveal: true, visible: visible.academy }}
		>
			<div class="scene-head">
				<p class="scene-slate">
					<span class="slate-no">Scene 05</span><span class="slate-title">Trading universities</span
					>
				</p>
				<h2>Two universities. One standard.</h2>
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

				<div class="academy-feature panel" data-cine-self>
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

		<!-- ═══ SCENE 06 · BUILD LOG ═════════════════════════════════════════ -->
		<section
			id="buildlog"
			class={{ 'buildlog-section': true, reveal: true, visible: visible.buildlog }}
		>
			<div class="scene-head">
				<p class="scene-slate">
					<span class="slate-no">Scene 06</span><span class="slate-title">Build log</span>
				</p>
				<h2>Shipped while you were away.</h2>
			</div>

			<div class="log-terminal panel" data-cine-self>
				<div class="terminal-bar" aria-hidden="true">
					<i></i><i></i><i></i>
					<span>rebuild — log · updated at the open</span>
				</div>
				<div class="log-body" data-cine>
					{#each buildLog as entry (entry.text)}
						<article class="log-row">
							<span class="log-date">{entry.date || '···'}</span>
							<span class={`log-tag ${entry.tag.toLowerCase()}`}>{entry.tag}</span>
							<p>{entry.text}</p>
						</article>
					{/each}
					<p class="log-cursor" aria-hidden="true">▮</p>
				</div>
			</div>
			<p class="log-note">Updated every trading day at the open.</p>
		</section>

		<!-- ═══ FINAL SCENE · FIRST ACCESS ═══════════════════════════════════ -->
		<section id="access" class={{ 'access-section': true, reveal: true, visible: visible.access }}>
			{#if isSubmitted}
				<div class="success-panel panel panel-feature" role="status">
					<div class="success-mark" aria-hidden="true">
						<svg viewBox="0 0 56 56">
							<circle cx="28" cy="28" r="25"></circle>
							<path d="M16 29.5 24.5 38 41 19"></path>
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
					class="access-panel panel panel-feature"
					data-cine
					onsubmit={(event) => {
						event.preventDefault();
						void handleNotifyMe();
					}}
				>
					<div>
						<p class="scene-slate">
							<span class="slate-no">Final scene</span><span class="slate-title">First access</span>
						</p>
						<h2>Be first through the doors.</h2>
						<p>
							One email when the platform reopens — nothing else. Founding members receive launch
							pricing, launch-week live sessions, and early access to the new scanner suite.
						</p>
						<p class="perk-line">
							Founding-member pricing · Launch-week sessions · Scanner suite preview
						</p>
					</div>

					<div class="email-block">
						<div class={{ 'email-row': true, 'has-error': Boolean(errorMessage) }}>
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

	<!-- ═══ BOTTOM HUD — the persistent terminal chrome ═════════════════════ -->
	<footer class="hud" aria-label="Session status">
		<i class="hud-progress" style:width={`${scrollProgress * 100}%`} aria-hidden="true"></i>
		<div class="hud-cluster">
			<span class="hud-clock" aria-label="New York time">{etClock} ET</span>
			<span class={{ 'hud-session': true, [sessionState]: true }}>
				<i aria-hidden="true"></i>{sessionLabel}
			</span>
		</div>
		<div class="hud-cluster hud-center">
			<span class="hud-day">Rebuild day {dayNumber || '—'}</span>
			<span class="hud-bar" aria-label={`Build progress ${progressDisplay}%`}>
				<i style:width={`${progressDisplay}%`}></i>
			</span>
			<span class="hud-pct">{progressDisplay}%</span>
		</div>
		<div class="hud-cluster hud-right">
			<span class="hud-latency">feed {feedLatency}ms</span>
			<span class="hud-mark">{sessionShort}</span>
		</div>
	</footer>
</div>

<style>
	:global(html:has(.maintenance-experience)),
	:global(body:has(.maintenance-experience)) {
		overflow: hidden;
		background: #06070a;
	}

	.maintenance-experience {
		--bg: #06070a;
		--ivory: #eceae4;
		--gold: #c6a15b;
		--panel: rgba(255, 255, 255, 0.02);
		--panel-2: rgba(255, 255, 255, 0.03);
		--panel-solid: rgba(6, 7, 10, 0.78);
		--line: rgba(255, 255, 255, 0.07);
		--line-strong: rgba(255, 255, 255, 0.13);
		--hair-faint: rgba(255, 255, 255, 0.05);
		--text: var(--ivory);
		--muted: rgba(236, 234, 228, 0.66);
		--dim: rgba(236, 234, 228, 0.44);
		--faint: rgba(236, 234, 228, 0.28);
		--accent: var(--gold);
		--gold-06: rgba(198, 161, 91, 0.06);
		--gold-25: rgba(198, 161, 91, 0.25);
		--gold-30: rgba(198, 161, 91, 0.3);
		--up: #35b98c;
		--down: #d16060;
		--up-soft: rgba(53, 185, 140, 0.18);
		--down-soft: rgba(209, 96, 96, 0.18);
		--edge-hairline: linear-gradient(
			140deg,
			rgba(236, 234, 228, 0.08),
			var(--gold-25) 42%,
			rgba(255, 255, 255, 0) 72%
		);
		--shadow-md: 0 12px 32px -18px rgba(0, 0, 0, 0.75);
		--shadow-lg: 0 40px 80px -40px rgba(0, 0, 0, 0.85), 0 8px 24px -16px rgba(0, 0, 0, 0.6);
		--font-display: Montserrat, Inter, ui-sans-serif, system-ui, sans-serif;
		--font-serif: Georgia, 'Iowan Old Style', 'Times New Roman', serif;
		--font-mono: 'SF Mono', ui-monospace, 'SFMono-Regular', Menlo, monospace;
		--chrome-h: 46px;
		--hud-h: 42px;
		position: fixed;
		inset: 0;
		z-index: 99999;
		overflow-x: hidden;
		overflow-y: auto;
		background:
			radial-gradient(1200px 560px at 78% -12%, var(--gold-06), transparent 60%),
			radial-gradient(1100px 620px at 12% 118%, rgba(46, 156, 119, 0.035), transparent 62%),
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

	/* Film texture: hairline scanlines + grain + a graded vignette. */
	.maintenance-experience::before {
		content: '';
		position: fixed;
		inset: 0;
		z-index: 0;
		pointer-events: none;
		background-image: repeating-linear-gradient(
			to bottom,
			var(--hair-faint) 0 1px,
			transparent 1px 32px
		);
		mask-image: linear-gradient(to bottom, transparent, #000 14%, #000 56%, transparent 92%);
		opacity: 0.5;
	}

	/* Aurora: three blurred color fields drifting almost imperceptibly — the
	 * grade that stops the void reading as flat black. Composited transform
	 * only, and switched off under reduced motion. */
	.aurora {
		position: fixed;
		inset: -20%;
		z-index: 0;
		pointer-events: none;
		filter: blur(64px);
		background:
			radial-gradient(38% 30% at 20% 22%, rgba(198, 161, 91, 0.075), transparent 70%),
			radial-gradient(34% 28% at 82% 68%, rgba(53, 185, 140, 0.055), transparent 70%),
			radial-gradient(30% 26% at 68% 12%, rgba(143, 166, 184, 0.05), transparent 70%);
		animation: aurora-drift 46s ease-in-out infinite alternate;
	}

	@keyframes aurora-drift {
		from {
			transform: translate3d(-2%, -1%, 0);
		}
		to {
			transform: translate3d(2%, 2.5%, 0);
		}
	}

	.grain {
		position: fixed;
		inset: 0;
		z-index: 40;
		pointer-events: none;
		opacity: 0.05;
		background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='140' height='140'%3E%3Cfilter id='g'%3E%3CfeTurbulence type='fractalNoise' baseFrequency='0.9' numOctaves='2' stitchTiles='stitch'/%3E%3CfeColorMatrix type='saturate' values='0'/%3E%3C/filter%3E%3Crect width='100%25' height='100%25' filter='url(%23g)'/%3E%3C/svg%3E");
		background-size: 140px 140px;
	}

	.vignette {
		position: fixed;
		inset: 0;
		z-index: 39;
		pointer-events: none;
		background: radial-gradient(120% 90% at 50% 42%, transparent 58%, rgba(0, 0, 0, 0.38) 100%);
	}

	/* Cinema mattes. Static hairline bars without the motion layer; GSAP
	 * scales them in for the title sequence and retracts them on scroll. */
	.letterbox {
		position: fixed;
		left: 0;
		right: 0;
		z-index: 38;
		height: clamp(28px, 4.6vh, 52px);
		background: #030406;
		pointer-events: none;
	}

	.letterbox.top {
		top: var(--chrome-h);
		border-bottom: 1px solid rgba(198, 161, 91, 0.18);
	}

	.letterbox.bottom {
		bottom: var(--hud-h);
		border-top: 1px solid rgba(198, 161, 91, 0.18);
	}

	.maintenance-experience:not(.gsap-on) .letterbox {
		height: 10px;
	}

	.maintenance-experience::-webkit-scrollbar {
		width: 8px;
	}

	.maintenance-experience::-webkit-scrollbar-thumb {
		background: rgba(255, 255, 255, 0.16);
		border: 2px solid #06070a;
		border-radius: 999px;
	}

	/* ── Top chrome: brand + tape ─────────────────────────────────────────── */

	.chrome-top {
		position: sticky;
		top: 0;
		z-index: 45;
		display: flex;
		align-items: center;
		gap: 18px;
		height: var(--chrome-h);
		padding-left: clamp(16px, 3vw, 32px);
		border-bottom: 1px solid var(--line);
		background: rgba(6, 7, 10, 0.92);
		backdrop-filter: blur(14px);
	}

	.chrome-brand {
		flex: none;
		font-family: var(--font-mono);
		font-size: 11px;
		font-weight: 700;
		letter-spacing: 0.18em;
		color: var(--text);
		white-space: nowrap;
	}

	.chrome-brand i {
		font-style: normal;
		color: var(--accent);
		padding: 0 2px;
	}

	.tape-viewport {
		flex: 1;
		overflow: hidden;
		mask-image: linear-gradient(to right, transparent, #000 4%, #000 96%, transparent);
	}

	.tape-track {
		display: flex;
		gap: 34px;
		width: max-content;
		padding: 13px 0;
		animation: tape-scroll 46s linear infinite;
	}

	@keyframes tape-scroll {
		to {
			transform: translateX(-33.333%);
		}
	}

	.tape-item {
		display: flex;
		gap: 9px;
		align-items: baseline;
		font-family: var(--font-mono);
		font-size: 11px;
		letter-spacing: 0.04em;
		white-space: nowrap;
	}

	.tape-symbol {
		color: var(--muted);
		font-weight: 700;
	}

	.tape-price {
		color: var(--text);
		font-variant-numeric: tabular-nums;
	}

	.tape-change {
		font-variant-numeric: tabular-nums;
	}

	.tape-change.up {
		color: var(--up);
	}

	.tape-change.down {
		color: var(--down);
	}

	.tape-cta {
		flex: none;
		align-self: stretch;
		display: inline-flex;
		align-items: center;
		padding: 0 clamp(16px, 2.4vw, 28px);
		border-left: 1px solid var(--line);
		font-size: 11px;
		font-weight: 600;
		letter-spacing: 0.14em;
		text-transform: uppercase;
		text-decoration: none;
		color: var(--accent);
		background: linear-gradient(180deg, rgba(198, 161, 91, 0.07), transparent);
		transition:
			background 0.25s ease,
			color 0.25s ease;
	}

	.tape-cta:hover,
	.tape-cta:focus-visible {
		background: var(--accent);
		color: #06070a;
	}

	/* ── Shell + scene grammar ────────────────────────────────────────────── */

	.page-shell {
		position: relative;
		z-index: 1;
		max-width: 1240px;
		margin: 0 auto;
		padding: 0 clamp(20px, 4vw, 48px) calc(var(--hud-h) + 40px);
	}

	section {
		padding: clamp(72px, 11vh, 130px) 0;
		border-bottom: 1px solid var(--hair-faint);
	}

	section:last-of-type {
		border-bottom: 0;
	}

	.reveal {
		opacity: 0;
		transform: translateY(14px);
		transition:
			opacity 0.8s ease,
			transform 0.8s ease;
	}

	.reveal.visible {
		opacity: 1;
		transform: none;
	}

	.scene-head {
		max-width: 780px;
		margin-bottom: clamp(32px, 5vh, 56px);
	}

	.scene-slate {
		display: inline-flex;
		align-items: center;
		gap: 12px;
		margin: 0 0 18px;
		padding: 6px 12px 6px 8px;
		border: 1px solid var(--line);
		border-left: 3px solid var(--accent);
		background: var(--panel);
		font-family: var(--font-mono);
		font-size: 10px;
		letter-spacing: 0.2em;
		text-transform: uppercase;
	}

	.slate-no {
		color: var(--accent);
		font-weight: 700;
	}

	.slate-title {
		color: var(--dim);
	}

	.scene-head h2 {
		margin: 0;
		font-family: var(--font-display);
		font-size: clamp(30px, 4.6vw, 52px);
		font-weight: 700;
		letter-spacing: -0.02em;
		line-height: 1.04;
		background: linear-gradient(180deg, #ffffff 8%, var(--ivory) 58%, rgba(236, 234, 228, 0.7));
		-webkit-background-clip: text;
		background-clip: text;
		color: transparent;
	}

	.scene-lede {
		margin: 18px 0 0;
		max-width: 620px;
		font-size: 15px;
		line-height: 1.7;
		color: var(--muted);
	}

	/* ── Panels ───────────────────────────────────────────────────────────── */

	.panel {
		position: relative;
		border: 1px solid var(--line);
		background: linear-gradient(180deg, var(--panel-2), var(--panel));
		box-shadow: var(--shadow-md);
	}

	.panel::before {
		content: '';
		position: absolute;
		inset: 0 auto auto 0;
		width: 100%;
		height: 1px;
		background: var(--edge-hairline);
		pointer-events: none;
	}

	.panel-feature {
		box-shadow: var(--shadow-lg);
		overflow: hidden;
	}

	/* Specular sweep: one pass of studio light across a feature panel as its
	 * scene becomes visible — the frame is lit, then settles. */
	section.visible .panel-feature::after {
		content: '';
		position: absolute;
		top: -40%;
		bottom: -40%;
		left: -60%;
		width: 34%;
		background: linear-gradient(
			105deg,
			transparent,
			rgba(236, 234, 228, 0.045) 45%,
			rgba(198, 161, 91, 0.1) 50%,
			rgba(236, 234, 228, 0.045) 55%,
			transparent
		);
		transform: skewX(-14deg);
		pointer-events: none;
		animation: specular-sweep 1.5s cubic-bezier(0.4, 0, 0.2, 1) 0.55s 1 forwards;
	}

	@keyframes specular-sweep {
		to {
			left: 130%;
		}
	}

	.card-topline,
	.card-footline {
		display: flex;
		justify-content: space-between;
		align-items: center;
		font-family: var(--font-mono);
		font-size: 10px;
		letter-spacing: 0.16em;
		text-transform: uppercase;
		color: var(--dim);
	}

	.status-flag {
		color: var(--accent);
	}

	/* ── Hero: the title card ─────────────────────────────────────────────── */

	.hero {
		position: relative;
		display: flex;
		flex-direction: column;
		justify-content: center;
		min-height: calc(100dvh - var(--chrome-h) - var(--hud-h));
		padding-top: clamp(36px, 5.5vh, 60px);
		padding-bottom: clamp(52px, 8vh, 96px);
		border-bottom: 1px solid var(--hair-faint);
	}

	.hero-watermark {
		position: absolute;
		top: 4%;
		left: -2%;
		z-index: 0;
		font-family: var(--font-display);
		font-size: clamp(150px, 26vw, 380px);
		font-weight: 800;
		letter-spacing: -0.04em;
		line-height: 1;
		color: transparent;
		-webkit-text-stroke: 1px rgba(236, 234, 228, 0.05);
		pointer-events: none;
		user-select: none;
	}

	.hero-chart {
		position: absolute;
		inset: 0 calc(clamp(20px, 4vw, 48px) * -1);
		z-index: 0;
		opacity: 0.85;
		mask-image: linear-gradient(to bottom, transparent 2%, #000 22%, #000 78%, transparent 98%);
	}

	.hero-chart-canvas {
		position: absolute;
		inset: 0;
		/* Neon bloom: drop-shadow keys off the canvas alpha, so only the drawn
		 * candles glow — the transparent field casts nothing. */
		filter: drop-shadow(0 0 14px rgba(198, 161, 91, 0.16));
	}

	.hero-flare {
		position: absolute;
		top: 30%;
		left: -10%;
		right: -10%;
		height: 3px;
		z-index: 0;
		background: linear-gradient(
			90deg,
			transparent,
			rgba(198, 161, 91, 0.4) 30%,
			rgba(236, 234, 228, 0.65) 50%,
			rgba(198, 161, 91, 0.4) 70%,
			transparent
		);
		filter: blur(1.5px);
		box-shadow: 0 0 24px rgba(198, 161, 91, 0.35);
		opacity: 0.5;
		pointer-events: none;
		animation: flare-drift 14s ease-in-out infinite alternate;
	}

	@keyframes flare-drift {
		from {
			transform: translateY(-14px);
			opacity: 0.32;
		}
		to {
			transform: translateY(26px);
			opacity: 0.6;
		}
	}

	/* Grounds the copy column: the backdrop tape dims where text sits, the way
	 * a DP would flag the subject before rolling — not a box, a light decision. */
	.hero-scrim {
		position: absolute;
		inset: 0;
		z-index: 1;
		pointer-events: none;
		background: radial-gradient(
			58% 64% at 26% 66%,
			rgba(6, 7, 10, 0.62),
			rgba(6, 7, 10, 0.28) 55%,
			transparent 78%
		);
	}

	.hero-copy {
		position: relative;
		z-index: 2;
	}

	.eyebrow {
		display: flex;
		flex-wrap: wrap;
		gap: 10px 22px;
		margin: 0 0 26px;
		font-family: var(--font-mono);
		font-size: 11px;
		letter-spacing: 0.24em;
		text-transform: uppercase;
	}

	.eyebrow-presents {
		color: var(--dim);
	}

	.eyebrow-day {
		color: var(--accent);
	}

	.hero-title {
		margin: 0 0 clamp(22px, 3.4vh, 36px);
		font-family: var(--font-display);
		font-size: clamp(46px, 8.6vw, 108px);
		font-weight: 800;
		letter-spacing: -0.03em;
		line-height: 0.98;
		text-transform: uppercase;
		color: var(--text);
	}

	.hero-title .line {
		display: block;
		overflow: hidden;
		padding-bottom: 0.06em;
	}

	.hero-title .line-inner {
		display: inline-block;
		will-change: transform;
		/* Graded ink: bright at the cap line falling to warm ivory, with a wide
		 * low-alpha bloom behind the glyphs — screen-title treatment, not UI
		 * text. The clip lives on the SAME element GSAP transforms: putting
		 * background-clip:text on an ancestor of a transformed span misplaces
		 * the rasterized glyph mask in Chromium and shatters the headline. */
		background: linear-gradient(180deg, #ffffff 6%, var(--ivory) 52%, rgba(236, 234, 228, 0.68));
		-webkit-background-clip: text;
		background-clip: text;
		color: transparent;
		text-shadow: 0 0 90px rgba(198, 161, 91, 0.16);
	}

	.serif-accent {
		font-family: var(--font-serif);
		font-style: italic;
		font-weight: 400;
		letter-spacing: -0.01em;
		text-transform: none;
		color: var(--accent);
		text-shadow: 0 0 36px rgba(198, 161, 91, 0.4);
	}

	.hero-lower {
		display: grid;
		grid-template-columns: minmax(0, 1.25fr) minmax(300px, 0.95fr);
		gap: clamp(28px, 4vw, 64px);
		align-items: end;
	}

	/* The live quote block — the price is the co-star of the title card. Glass
	 * chip so the figure stays legible over whatever the tape is doing. */
	.hero-quote {
		margin-bottom: clamp(18px, 2.8vh, 26px);
		max-width: 420px;
		padding: 14px 20px 13px;
		border: 1px solid var(--line);
		background: rgba(6, 7, 10, 0.55);
		backdrop-filter: blur(12px);
		box-shadow:
			inset 0 1px 0 rgba(236, 234, 228, 0.06),
			0 18px 44px -26px rgba(0, 0, 0, 0.8);
	}

	.quote-head {
		display: flex;
		align-items: center;
		gap: 14px;
		margin-bottom: 6px;
		font-family: var(--font-mono);
		font-size: 11px;
		letter-spacing: 0.18em;
		text-transform: uppercase;
	}

	.quote-symbol {
		font-weight: 700;
		color: var(--text);
	}

	.quote-live {
		display: inline-flex;
		align-items: center;
		gap: 7px;
		color: var(--dim);
	}

	.quote-live i {
		width: 6px;
		height: 6px;
		border-radius: 50%;
		background: var(--up);
		box-shadow: 0 0 0 0 rgba(46, 156, 119, 0.5);
		animation: live-pulse 2.2s ease-out infinite;
	}

	@keyframes live-pulse {
		0% {
			box-shadow: 0 0 0 0 rgba(46, 156, 119, 0.5);
		}
		70% {
			box-shadow: 0 0 0 8px rgba(46, 156, 119, 0);
		}
		100% {
			box-shadow: 0 0 0 0 rgba(46, 156, 119, 0);
		}
	}

	.quote-figure {
		font-family: var(--font-mono);
		font-size: clamp(40px, 5.2vw, 60px);
		font-weight: 700;
		font-variant-numeric: tabular-nums;
		letter-spacing: -0.02em;
		line-height: 1;
		color: var(--text);
		transition:
			color 0.4s ease,
			text-shadow 0.4s ease;
	}

	.quote-figure.flash-up {
		color: var(--up);
		text-shadow: 0 0 34px rgba(46, 156, 119, 0.45);
	}

	.quote-figure.flash-down {
		color: var(--down);
		text-shadow: 0 0 34px rgba(194, 85, 85, 0.45);
	}

	.quote-meta {
		display: flex;
		align-items: baseline;
		gap: 10px;
		margin-top: 8px;
		font-family: var(--font-mono);
		font-size: 13px;
		font-variant-numeric: tabular-nums;
	}

	.quote-meta .up {
		color: var(--up);
	}

	.quote-meta .down {
		color: var(--down);
	}

	.quote-session {
		font-size: 10px;
		letter-spacing: 0.18em;
		text-transform: uppercase;
		color: var(--faint);
	}

	.range-meter {
		display: flex;
		align-items: center;
		gap: 10px;
		margin-top: 14px;
		font-family: var(--font-mono);
		font-size: 10px;
		font-variant-numeric: tabular-nums;
		color: var(--dim);
	}

	.range-track {
		position: relative;
		flex: 1;
		height: 2px;
		background: rgba(255, 255, 255, 0.1);
	}

	.range-track i {
		position: absolute;
		top: 50%;
		width: 9px;
		height: 9px;
		border-radius: 50%;
		background: var(--accent);
		transform: translate(-50%, -50%);
		box-shadow: 0 0 12px rgba(198, 161, 91, 0.55);
		transition: left 0.5s cubic-bezier(0.22, 1, 0.36, 1);
	}

	.lede {
		max-width: 520px;
		margin: 0 0 26px;
		font-size: clamp(15px, 1.35vw, 17px);
		line-height: 1.7;
		color: var(--muted);
	}

	.hero-actions {
		display: flex;
		flex-wrap: wrap;
		gap: 14px;
		margin-bottom: 16px;
	}

	.primary-link {
		display: inline-flex;
		align-items: center;
		padding: 14px 30px;
		background: linear-gradient(135deg, #dab671, #b8934f);
		color: #06070a;
		font-size: 12px;
		font-weight: 700;
		letter-spacing: 0.14em;
		text-transform: uppercase;
		text-decoration: none;
		box-shadow: 0 10px 30px -14px rgba(198, 161, 91, 0.5);
		transition:
			transform 0.25s cubic-bezier(0.22, 1, 0.36, 1),
			box-shadow 0.25s ease;
	}

	.primary-link:hover,
	.primary-link:focus-visible {
		transform: translateY(-2px);
		box-shadow:
			0 16px 40px -14px rgba(198, 161, 91, 0.65),
			0 0 24px rgba(198, 161, 91, 0.2);
	}

	.ghost-link {
		display: inline-flex;
		align-items: center;
		padding: 14px 26px;
		border: 1px solid var(--line-strong);
		color: var(--text);
		font-size: 12px;
		font-weight: 600;
		letter-spacing: 0.14em;
		text-transform: uppercase;
		text-decoration: none;
		transition:
			border-color 0.25s ease,
			background 0.25s ease;
	}

	.ghost-link:hover,
	.ghost-link:focus-visible {
		border-color: var(--gold-30);
		background: var(--gold-06);
	}

	.hero-note {
		margin: 0 0 22px;
		font-size: 12px;
		letter-spacing: 0.02em;
		color: var(--dim);
	}

	.drop-timer {
		display: inline-flex;
		align-items: baseline;
		gap: 14px;
		margin: 0;
		padding: 10px 16px;
		border: 1px solid var(--line);
		background: var(--panel);
		font-family: var(--font-mono);
	}

	.timer-label {
		font-size: 10px;
		letter-spacing: 0.18em;
		text-transform: uppercase;
		color: var(--dim);
	}

	.timer-clock {
		font-size: 18px;
		font-weight: 700;
		font-variant-numeric: tabular-nums;
		color: var(--accent);
	}

	.timer-meta {
		font-size: 10px;
		letter-spacing: 0.08em;
		color: var(--faint);
	}

	/* Status card */
	.status-card {
		padding: 22px 24px;
		background: var(--panel-solid);
		backdrop-filter: blur(18px);
	}

	.progress-block {
		margin: 20px 0;
	}

	.progress-head {
		display: flex;
		justify-content: space-between;
		align-items: baseline;
		margin-bottom: 10px;
		font-size: 12px;
		color: var(--muted);
	}

	.progress-head strong {
		font-family: var(--font-mono);
		font-size: 26px;
		font-weight: 700;
		font-variant-numeric: tabular-nums;
		color: var(--text);
	}

	.progress-track {
		height: 3px;
		background: rgba(255, 255, 255, 0.08);
		overflow: hidden;
	}

	.progress-track span {
		display: block;
		height: 100%;
		background: linear-gradient(90deg, rgba(198, 161, 91, 0.5), var(--accent));
		transition: width 0.6s cubic-bezier(0.22, 1, 0.36, 1);
	}

	.stage-list {
		margin: 0 0 18px;
		padding: 0;
		list-style: none;
	}

	.stage-list li {
		display: grid;
		grid-template-columns: 30px 1fr auto;
		gap: 10px;
		align-items: baseline;
		padding: 9px 0;
		border-top: 1px solid var(--hair-faint);
		font-size: 13px;
	}

	.stage-index {
		font-family: var(--font-mono);
		font-size: 10px;
		color: var(--faint);
	}

	.stage-name {
		color: var(--muted);
	}

	.stage-state {
		font-family: var(--font-mono);
		font-size: 10px;
		letter-spacing: 0.12em;
		text-transform: uppercase;
		color: var(--faint);
	}

	.stage-list li.complete .stage-state {
		color: var(--up);
	}

	.stage-list li.current .stage-name {
		color: var(--text);
	}

	.stage-list li.current .stage-state {
		color: var(--accent);
	}

	.card-footline {
		padding-top: 14px;
		border-top: 1px solid var(--hair-faint);
	}

	.scroll-cue {
		position: absolute;
		bottom: clamp(18px, 3.5vh, 34px);
		left: 0;
		display: flex;
		align-items: center;
		gap: 12px;
		font-family: var(--font-mono);
		font-size: 10px;
		letter-spacing: 0.28em;
		text-transform: uppercase;
		color: var(--dim);
	}

	.scroll-cue i {
		display: block;
		width: 64px;
		height: 1px;
		background: linear-gradient(90deg, var(--accent), transparent);
	}

	/* ── Scene 01: desk note ──────────────────────────────────────────────── */

	.note-card {
		padding: clamp(34px, 5vw, 60px);
	}

	.note-card blockquote {
		margin: 0 0 22px;
		font-family: var(--font-serif);
		font-style: italic;
		font-size: clamp(24px, 3.2vw, 38px);
		line-height: 1.35;
		letter-spacing: -0.01em;
		color: var(--text);
	}

	.note-card blockquote::before {
		content: '“';
		display: block;
		margin-bottom: -0.35em;
		font-size: 2.2em;
		line-height: 1;
		color: var(--gold-30);
	}

	.note-foot {
		margin: 0 0 18px;
		font-size: 12px;
		color: var(--dim);
	}

	.note-meta {
		display: flex;
		justify-content: space-between;
		align-items: baseline;
		padding-top: 16px;
		border-top: 1px solid var(--hair-faint);
		font-family: var(--font-mono);
		font-size: 10px;
		letter-spacing: 0.16em;
		text-transform: uppercase;
		color: var(--dim);
	}

	.note-meta strong {
		font-size: 13px;
		color: var(--accent);
	}

	/* ── Scene 02: the desk, live ─────────────────────────────────────────── */

	.desk-grid {
		display: grid;
		grid-template-columns: repeat(2, minmax(0, 1fr));
		gap: 18px;
	}

	.desk-module {
		display: flex;
		flex-direction: column;
		padding: 18px 20px;
		min-height: 320px;
	}

	/* Camera-framing corner brackets: the HUD grammar that marks each module
	 * as a live instrument in frame. Eight gradient strips, two per corner. */
	.desk-module::after {
		content: '';
		position: absolute;
		inset: 0;
		pointer-events: none;
		background-image:
			linear-gradient(var(--gold-30), var(--gold-30)),
			linear-gradient(var(--gold-30), var(--gold-30)),
			linear-gradient(var(--gold-30), var(--gold-30)),
			linear-gradient(var(--gold-30), var(--gold-30)),
			linear-gradient(var(--gold-30), var(--gold-30)),
			linear-gradient(var(--gold-30), var(--gold-30)),
			linear-gradient(var(--gold-30), var(--gold-30)),
			linear-gradient(var(--gold-30), var(--gold-30));
		background-size:
			14px 1px,
			1px 14px,
			14px 1px,
			1px 14px,
			14px 1px,
			1px 14px,
			14px 1px,
			1px 14px;
		background-position:
			top left,
			top left,
			top right,
			top right,
			bottom left,
			bottom left,
			bottom right,
			bottom right;
		background-repeat: no-repeat;
	}

	.desk-module.wide {
		grid-column: span 1;
	}

	.module-head {
		display: flex;
		justify-content: space-between;
		align-items: baseline;
		margin-bottom: 14px;
		padding-bottom: 12px;
		border-bottom: 1px solid var(--hair-faint);
	}

	.module-title {
		font-size: 12px;
		font-weight: 700;
		letter-spacing: 0.14em;
		text-transform: uppercase;
		color: var(--text);
	}

	.module-tag {
		font-family: var(--font-mono);
		font-size: 10px;
		letter-spacing: 0.1em;
		text-transform: uppercase;
		color: var(--dim);
	}

	.module-foot {
		display: flex;
		justify-content: space-between;
		align-items: baseline;
		margin-top: auto;
		padding-top: 12px;
		border-top: 1px solid var(--hair-faint);
		font-family: var(--font-mono);
		font-size: 10px;
		letter-spacing: 0.14em;
		text-transform: uppercase;
		color: var(--dim);
	}

	.module-foot strong {
		font-size: 13px;
		font-variant-numeric: tabular-nums;
		color: var(--accent);
	}

	/* Order book */
	.book-columns {
		flex: 1;
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 14px;
	}

	.book-side {
		display: flex;
		flex-direction: column;
	}

	.book-legend {
		display: flex;
		justify-content: space-between;
		margin-bottom: 8px;
		font-family: var(--font-mono);
		font-size: 9px;
		letter-spacing: 0.16em;
		text-transform: uppercase;
		color: var(--faint);
	}

	.book-row {
		position: relative;
		flex: 1;
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 6px 8px;
		margin-bottom: 3px;
		font-family: var(--font-mono);
		font-size: 12px;
		font-variant-numeric: tabular-nums;
		overflow: hidden;
	}

	.book-row .depth {
		position: absolute;
		top: 0;
		bottom: 0;
		transition: width 0.55s cubic-bezier(0.22, 1, 0.36, 1);
	}

	.bids .book-row .depth {
		right: 0;
		background: linear-gradient(90deg, rgba(53, 185, 140, 0.05), var(--up-soft));
		border-right: 1px solid rgba(53, 185, 140, 0.45);
	}

	.asks .book-row .depth {
		left: 0;
		background: linear-gradient(90deg, var(--down-soft), rgba(209, 96, 96, 0.05));
		border-left: 1px solid rgba(209, 96, 96, 0.45);
	}

	.book-size {
		position: relative;
		color: var(--dim);
	}

	.book-price {
		position: relative;
		font-weight: 700;
	}

	.bids .book-price {
		color: var(--up);
	}

	.asks .book-price {
		color: var(--down);
	}

	/* Time & sales */
	.prints-stream {
		flex: 1;
		display: flex;
		flex-direction: column;
		gap: 2px;
		overflow: hidden;
	}

	.print-row {
		display: grid;
		grid-template-columns: auto 1fr auto 18px;
		gap: 12px;
		align-items: baseline;
		padding: 4px 8px;
		font-family: var(--font-mono);
		font-size: 11px;
		font-variant-numeric: tabular-nums;
		animation: print-in 0.3s cubic-bezier(0.22, 1, 0.36, 1);
	}

	@keyframes print-in {
		from {
			opacity: 0;
			transform: translateY(-6px);
		}
		to {
			opacity: 1;
			transform: none;
		}
	}

	.print-time {
		color: var(--faint);
	}

	.print-price {
		font-weight: 700;
	}

	.print-row.buy .print-price,
	.print-row.buy .print-side {
		color: var(--up);
	}

	.print-row.sell .print-price,
	.print-row.sell .print-side {
		color: var(--down);
	}

	.print-size {
		color: var(--dim);
		text-align: right;
	}

	.print-row.block {
		background: var(--gold-06);
		outline: 1px solid var(--gold-25);
	}

	.print-row.block .print-size {
		color: var(--accent);
		font-weight: 700;
	}

	/* Sector map */
	.sector-grid {
		flex: 1;
		display: grid;
		grid-template-columns: repeat(4, minmax(0, 1fr));
		gap: 6px;
	}

	.sector-tile {
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		gap: 6px;
		padding: 12px;
		border: 1px solid rgba(255, 255, 255, 0.05);
		box-shadow:
			inset 0 1px 0 rgba(255, 255, 255, 0.06),
			inset 0 -16px 26px -20px rgba(0, 0, 0, 0.7);
		transition: background 1.2s ease;
	}

	.sector-code {
		font-family: var(--font-mono);
		font-size: 11px;
		font-weight: 700;
		letter-spacing: 0.08em;
		color: var(--text);
	}

	.sector-name {
		font-size: 10px;
		color: var(--muted);
	}

	.sector-tile strong {
		font-family: var(--font-mono);
		font-size: 13px;
		font-variant-numeric: tabular-nums;
	}

	.sector-tile strong.up {
		color: #7fd7b6;
	}

	.sector-tile strong.down {
		color: #e59a9a;
	}

	/* Scanner feed */
	.signal-stream {
		flex: 1;
		display: flex;
		flex-direction: column;
	}

	.signal-row {
		display: grid;
		grid-template-columns: minmax(0, 1.4fr) auto minmax(120px, 0.8fr);
		gap: 16px;
		align-items: center;
		padding: 11px 8px;
		border-top: 1px solid var(--hair-faint);
		transition: opacity 0.4s ease;
	}

	.signal-row:first-of-type {
		border-top: 0;
	}

	.signal-row.cooling {
		opacity: 0.45;
	}

	.signal-id {
		display: flex;
		flex-direction: column;
		gap: 2px;
		min-width: 0;
	}

	.signal-id strong {
		font-family: var(--font-mono);
		font-size: 14px;
		letter-spacing: 0.06em;
		color: var(--text);
	}

	.signal-id span {
		font-size: 11px;
		color: var(--dim);
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.signal-figures {
		display: flex;
		flex-direction: column;
		align-items: flex-end;
		gap: 2px;
		font-family: var(--font-mono);
		font-variant-numeric: tabular-nums;
	}

	.signal-figures span {
		font-size: 13px;
		color: var(--text);
	}

	.signal-figures small {
		font-size: 10px;
		color: var(--faint);
	}

	.confidence {
		display: flex;
		align-items: center;
		gap: 10px;
	}

	.confidence span {
		position: relative;
		flex: 1;
		height: 2px;
		background: rgba(255, 255, 255, 0.08);
	}

	.confidence span::after {
		content: '';
		position: absolute;
		inset: 0 auto 0 0;
		width: var(--confidence);
		background: var(--accent);
		box-shadow: 0 0 8px rgba(198, 161, 91, 0.55);
		transition: width 0.6s ease;
	}

	.confidence small {
		font-family: var(--font-mono);
		font-size: 11px;
		font-variant-numeric: tabular-nums;
		color: var(--muted);
	}

	/* ── Scene 03: market data ────────────────────────────────────────────── */

	.chart-stage {
		padding: 20px 22px;
		margin-bottom: 18px;
	}

	.chart-toolbar {
		display: flex;
		justify-content: space-between;
		align-items: center;
		gap: 18px;
		margin-bottom: 16px;
	}

	.market-label {
		display: block;
		margin-bottom: 4px;
		font-family: var(--font-mono);
		font-size: 10px;
		letter-spacing: 0.16em;
		text-transform: uppercase;
		color: var(--dim);
	}

	.chart-toolbar strong {
		font-family: var(--font-mono);
		font-size: 24px;
		font-weight: 700;
		font-variant-numeric: tabular-nums;
	}

	.chart-toolbar strong.up {
		color: var(--up);
	}

	.chart-toolbar strong.down {
		color: var(--down);
	}

	.timeframe-control {
		display: inline-flex;
		border: 1px solid var(--line);
	}

	.timeframe-control button {
		padding: 8px 16px;
		border: 0;
		border-right: 1px solid var(--line);
		background: transparent;
		color: var(--dim);
		font-family: var(--font-mono);
		font-size: 11px;
		cursor: pointer;
		transition:
			background 0.2s ease,
			color 0.2s ease;
	}

	.timeframe-control button:last-child {
		border-right: 0;
	}

	.timeframe-control button:hover {
		color: var(--text);
	}

	.timeframe-control button.active {
		background: var(--accent);
		color: #06070a;
		font-weight: 700;
	}

	.chart-canvas {
		height: clamp(300px, 44vh, 430px);
	}

	.chart-metrics {
		display: flex;
		gap: clamp(24px, 4vw, 60px);
		margin-top: 16px;
		padding-top: 14px;
		border-top: 1px solid var(--hair-faint);
	}

	.chart-metrics span {
		display: block;
		margin-bottom: 3px;
		font-family: var(--font-mono);
		font-size: 9px;
		letter-spacing: 0.16em;
		text-transform: uppercase;
		color: var(--faint);
	}

	.chart-metrics strong {
		font-family: var(--font-mono);
		font-size: 16px;
		font-variant-numeric: tabular-nums;
		color: var(--text);
	}

	.mini-feed-grid {
		display: grid;
		grid-template-columns: repeat(2, minmax(0, 1fr));
		gap: 18px;
	}

	.mini-feed {
		padding: 16px 18px;
	}

	.mini-feed > div:first-child {
		display: flex;
		justify-content: space-between;
		align-items: baseline;
		margin-bottom: 10px;
	}

	.mini-feed span {
		font-family: var(--font-mono);
		font-size: 11px;
		font-weight: 700;
		letter-spacing: 0.1em;
		color: var(--muted);
	}

	.mini-feed strong {
		font-family: var(--font-mono);
		font-size: 15px;
		font-variant-numeric: tabular-nums;
		color: var(--text);
	}

	.mini-chart {
		height: 130px;
		filter: drop-shadow(0 0 8px rgba(198, 161, 91, 0.14));
	}

	/* ── Scene 04: scope of work ──────────────────────────────────────────── */

	.scope-stage {
		display: grid;
		grid-template-columns: minmax(0, 1.5fr) minmax(300px, 1fr);
		gap: clamp(24px, 3.5vw, 48px);
		align-items: start;
	}

	.scope-manifest {
		display: flex;
		flex-direction: column;
	}

	.scope-row {
		display: grid;
		grid-template-columns: 56px minmax(0, 1fr) auto;
		gap: clamp(14px, 2vw, 28px);
		align-items: start;
		padding: 26px 0;
		border-top: 1px solid var(--hair-faint);
	}

	.scope-row:first-child {
		border-top: 0;
		padding-top: 0;
	}

	.scope-index {
		font-family: var(--font-mono);
		font-size: 13px;
		font-weight: 700;
		color: var(--accent);
	}

	.scope-body h3 {
		margin: 0 0 8px;
		font-family: var(--font-display);
		font-size: 20px;
		font-weight: 700;
		letter-spacing: -0.01em;
		color: var(--text);
	}

	.scope-body p {
		margin: 0;
		font-size: 13.5px;
		line-height: 1.65;
		color: var(--muted);
	}

	.scope-metric {
		padding: 5px 10px;
		border: 1px solid var(--gold-25);
		font-family: var(--font-mono);
		font-size: 10px;
		letter-spacing: 0.1em;
		text-transform: uppercase;
		color: var(--accent);
		white-space: nowrap;
	}

	.scope-rail {
		display: flex;
		flex-direction: column;
		gap: 18px;
	}

	.systems-panel {
		padding: 20px 22px;
	}

	.status-list {
		margin: 16px 0;
	}

	.status-row {
		display: flex;
		justify-content: space-between;
		align-items: baseline;
		padding: 9px 0;
		border-top: 1px solid var(--hair-faint);
	}

	.status-row dt {
		font-size: 12.5px;
		color: var(--muted);
	}

	.status-row dd {
		margin: 0;
		font-family: var(--font-mono);
		font-size: 10px;
		letter-spacing: 0.12em;
		text-transform: uppercase;
		color: var(--up);
	}

	.node-list {
		padding: 18px 22px;
	}

	.node-row {
		display: grid;
		grid-template-columns: minmax(90px, auto) 1fr auto;
		gap: 14px;
		align-items: center;
		padding: 10px 0;
		border-top: 1px solid var(--hair-faint);
	}

	.node-row:first-child {
		border-top: 0;
		padding-top: 0;
	}

	.node-row strong {
		display: block;
		font-family: var(--font-mono);
		font-size: 12px;
		color: var(--text);
	}

	.node-row > div:first-child span {
		font-size: 10.5px;
		color: var(--faint);
	}

	.load-track {
		position: relative;
		height: 2px;
		background: rgba(255, 255, 255, 0.08);
	}

	.load-track span {
		position: absolute;
		inset: 0 auto 0 0;
		background: var(--accent);
		transition: width 0.8s cubic-bezier(0.22, 1, 0.36, 1);
	}

	.node-row small {
		font-family: var(--font-mono);
		font-size: 9px;
		letter-spacing: 0.14em;
		text-transform: uppercase;
		color: var(--dim);
	}

	.node-row small.migrating {
		color: var(--accent);
	}

	.stat-board {
		display: grid;
		grid-template-columns: repeat(2, minmax(0, 1fr));
	}

	.stat-board > div {
		padding: 18px 20px;
		border-top: 1px solid var(--hair-faint);
	}

	.stat-board > div:nth-child(-n + 2) {
		border-top: 0;
	}

	.stat-board > div:nth-child(odd) {
		border-right: 1px solid var(--hair-faint);
	}

	.stat-board strong {
		display: block;
		margin-bottom: 4px;
		font-family: var(--font-mono);
		font-size: clamp(18px, 2vw, 24px);
		font-weight: 700;
		font-variant-numeric: tabular-nums;
		color: var(--text);
	}

	.stat-board span {
		font-size: 10.5px;
		letter-spacing: 0.06em;
		color: var(--dim);
	}

	/* ── Scene 05: universities ───────────────────────────────────────────── */

	.academy-selector {
		display: grid;
		grid-template-columns: repeat(2, minmax(0, 1fr));
		gap: 14px;
		margin-bottom: 18px;
	}

	.academy-selector button {
		display: flex;
		flex-direction: column;
		gap: 4px;
		padding: 18px 22px;
		border: 1px solid var(--line);
		background: var(--panel);
		color: var(--muted);
		text-align: left;
		cursor: pointer;
		transition:
			border-color 0.25s ease,
			background 0.25s ease,
			transform 0.25s ease;
	}

	.academy-selector button:hover {
		border-color: var(--line-strong);
		transform: translateY(-2px);
	}

	.academy-selector button.active {
		border-color: var(--gold-30);
		background: linear-gradient(180deg, var(--gold-06), transparent);
	}

	.track-index {
		font-family: var(--font-mono);
		font-size: 10px;
		letter-spacing: 0.2em;
		color: var(--accent);
	}

	.academy-selector strong {
		font-family: var(--font-display);
		font-size: 17px;
		font-weight: 700;
		color: var(--text);
	}

	.academy-selector small {
		font-family: var(--font-mono);
		font-size: 10px;
		letter-spacing: 0.14em;
		text-transform: uppercase;
		color: var(--dim);
	}

	.academy-feature {
		display: grid;
		grid-template-columns: minmax(0, 0.9fr) minmax(0, 1.1fr);
		gap: clamp(24px, 3vw, 44px);
		padding: clamp(26px, 3.5vw, 44px);
	}

	.academy-summary h3 {
		margin: 0 0 12px;
		font-family: var(--font-display);
		font-size: clamp(22px, 2.6vw, 30px);
		font-weight: 700;
		letter-spacing: -0.015em;
		color: var(--text);
	}

	.academy-summary > p {
		margin: 0 0 22px;
		font-size: 14px;
		line-height: 1.7;
		color: var(--muted);
	}

	.asset-lanes {
		display: grid;
		grid-template-columns: repeat(2, minmax(0, 1fr));
		gap: 16px;
	}

	.asset-lanes > div {
		display: flex;
		flex-direction: column;
		gap: 7px;
	}

	.asset-lanes strong {
		margin-bottom: 4px;
		font-family: var(--font-mono);
		font-size: 10px;
		letter-spacing: 0.18em;
		text-transform: uppercase;
		color: var(--accent);
	}

	.asset-lanes span {
		padding: 6px 10px;
		border: 1px solid var(--hair-faint);
		background: var(--panel);
		font-size: 12px;
		color: var(--muted);
	}

	.table-head {
		display: flex;
		justify-content: space-between;
		align-items: baseline;
		margin-bottom: 14px;
		padding-bottom: 12px;
		border-bottom: 1px solid var(--hair-faint);
		font-family: var(--font-mono);
		font-size: 10px;
		letter-spacing: 0.16em;
		text-transform: uppercase;
		color: var(--dim);
	}

	.curriculum-list {
		margin: 0 0 20px;
		padding: 0;
		list-style: none;
	}

	.curriculum-list li {
		display: grid;
		grid-template-columns: 34px 1fr;
		gap: 12px;
		padding: 11px 0;
		border-top: 1px solid var(--hair-faint);
	}

	.curriculum-list li:first-child {
		border-top: 0;
		padding-top: 0;
	}

	.curriculum-list span {
		font-family: var(--font-mono);
		font-size: 11px;
		color: var(--accent);
	}

	.curriculum-list p {
		margin: 0;
		font-size: 13.5px;
		line-height: 1.6;
		color: var(--muted);
	}

	.academy-lab {
		display: grid;
		grid-template-columns: repeat(2, minmax(0, 1fr));
		gap: 18px;
		padding-top: 18px;
		border-top: 1px solid var(--hair-faint);
	}

	.academy-lab span {
		display: block;
		margin-bottom: 6px;
		font-family: var(--font-mono);
		font-size: 9px;
		letter-spacing: 0.18em;
		text-transform: uppercase;
		color: var(--accent);
	}

	.academy-lab p {
		margin: 0;
		font-size: 12.5px;
		line-height: 1.6;
		color: var(--muted);
	}

	/* ── Scene 06: build log ──────────────────────────────────────────────── */

	.log-terminal {
		overflow: hidden;
	}

	.terminal-bar {
		display: flex;
		align-items: center;
		gap: 7px;
		padding: 12px 18px;
		border-bottom: 1px solid var(--line);
		background: rgba(255, 255, 255, 0.02);
	}

	.terminal-bar i {
		width: 10px;
		height: 10px;
		border-radius: 50%;
		background: rgba(255, 255, 255, 0.12);
	}

	.terminal-bar i:first-child {
		background: rgba(194, 85, 85, 0.55);
	}

	.terminal-bar i:nth-child(2) {
		background: rgba(198, 161, 91, 0.55);
	}

	.terminal-bar i:nth-child(3) {
		background: rgba(46, 156, 119, 0.55);
	}

	.terminal-bar span {
		margin-left: 10px;
		font-family: var(--font-mono);
		font-size: 10px;
		letter-spacing: 0.1em;
		color: var(--dim);
	}

	.log-body {
		padding: 20px 24px;
	}

	.log-row {
		display: grid;
		grid-template-columns: 64px 88px 1fr;
		gap: 16px;
		align-items: baseline;
		padding: 12px 0;
		border-top: 1px solid var(--hair-faint);
	}

	.log-row:first-child {
		border-top: 0;
		padding-top: 0;
	}

	.log-date {
		font-family: var(--font-mono);
		font-size: 11px;
		color: var(--faint);
	}

	.log-tag {
		justify-self: start;
		padding: 3px 9px;
		font-family: var(--font-mono);
		font-size: 9px;
		font-weight: 700;
		letter-spacing: 0.14em;
		text-transform: uppercase;
	}

	.log-tag.shipped {
		color: var(--up);
		border: 1px solid rgba(46, 156, 119, 0.35);
	}

	.log-tag.hardened {
		color: var(--accent);
		border: 1px solid var(--gold-25);
	}

	.log-tag.tuned {
		color: rgba(143, 166, 184, 0.95);
		border: 1px solid rgba(143, 166, 184, 0.3);
	}

	.log-row p {
		margin: 0;
		font-size: 13.5px;
		line-height: 1.6;
		color: var(--muted);
	}

	.log-cursor {
		margin: 14px 0 0;
		font-family: var(--font-mono);
		font-size: 12px;
		color: var(--accent);
		animation: cursor-blink 1.1s steps(1) infinite;
	}

	@keyframes cursor-blink {
		50% {
			opacity: 0;
		}
	}

	.log-note {
		margin: 14px 0 0;
		font-family: var(--font-mono);
		font-size: 10px;
		letter-spacing: 0.14em;
		text-transform: uppercase;
		color: var(--faint);
	}

	/* ── Final scene: access ──────────────────────────────────────────────── */

	.access-panel,
	.success-panel {
		display: grid;
		grid-template-columns: minmax(0, 1.1fr) minmax(0, 0.9fr);
		gap: clamp(28px, 4vw, 56px);
		align-items: center;
		padding: clamp(32px, 5vw, 64px);
	}

	.access-panel h2,
	.success-panel h2 {
		margin: 0 0 14px;
		font-family: var(--font-display);
		font-size: clamp(26px, 3.4vw, 40px);
		font-weight: 700;
		letter-spacing: -0.02em;
		line-height: 1.08;
		color: var(--text);
	}

	.access-panel p,
	.success-panel p {
		margin: 0 0 12px;
		font-size: 14px;
		line-height: 1.7;
		color: var(--muted);
	}

	.perk-line {
		font-family: var(--font-mono);
		font-size: 11px;
		letter-spacing: 0.04em;
		color: var(--accent);
	}

	.email-block {
		width: 100%;
	}

	.email-row {
		display: flex;
		border: 1px solid var(--line-strong);
		background: rgba(0, 0, 0, 0.3);
		transition:
			border-color 0.25s ease,
			box-shadow 0.25s ease;
	}

	.email-row:focus-within {
		border-color: var(--gold-30);
		box-shadow: 0 0 0 3px rgba(198, 161, 91, 0.12);
	}

	.email-row.has-error {
		border-color: rgba(194, 85, 85, 0.6);
	}

	.email-row input {
		flex: 1;
		min-width: 0;
		padding: 16px 18px;
		border: 0;
		background: transparent;
		color: var(--text);
		font-size: 14px;
		font-family: inherit;
	}

	.email-row input::placeholder {
		color: var(--faint);
	}

	.email-row input:focus {
		outline: none;
	}

	.email-row button {
		flex: none;
		padding: 0 26px;
		border: 0;
		background: linear-gradient(135deg, #dab671, #b8934f);
		color: #06070a;
		font-size: 11px;
		font-weight: 700;
		letter-spacing: 0.14em;
		text-transform: uppercase;
		cursor: pointer;
		transition: opacity 0.25s ease;
	}

	.email-row button:disabled {
		opacity: 0.45;
		cursor: not-allowed;
	}

	.form-error {
		margin: 10px 0 0;
		font-size: 12.5px;
		color: #e59a9a;
	}

	.success-mark svg {
		width: 72px;
		height: 72px;
		fill: none;
		stroke: var(--up);
		stroke-width: 2.4;
		stroke-linecap: round;
		stroke-linejoin: round;
	}

	.success-mark circle {
		opacity: 0.35;
	}

	/* ── Footer ───────────────────────────────────────────────────────────── */

	.floor-footer {
		display: grid;
		gap: 16px;
		padding: clamp(40px, 6vh, 64px) 0 26px;
		border-top: 1px solid var(--hair-faint);
	}

	.footer-brand strong {
		display: block;
		margin-bottom: 4px;
		font-family: var(--font-display);
		font-size: 15px;
		font-weight: 700;
		letter-spacing: 0.01em;
		color: var(--text);
	}

	.footer-brand span {
		font-size: 12px;
		color: var(--dim);
	}

	.footer-risk {
		margin: 0;
		max-width: 640px;
		font-size: 11.5px;
		line-height: 1.6;
		color: var(--faint);
	}

	.footer-legal {
		font-family: var(--font-mono);
		font-size: 10px;
		letter-spacing: 0.1em;
		color: var(--faint);
	}

	/* ── Bottom HUD ───────────────────────────────────────────────────────── */

	.hud {
		position: fixed;
		left: 0;
		right: 0;
		bottom: 0;
		z-index: 45;
		display: flex;
		justify-content: space-between;
		align-items: center;
		gap: 18px;
		height: var(--hud-h);
		padding: 0 clamp(16px, 3vw, 32px);
		border-top: 1px solid var(--line);
		background: rgba(6, 7, 10, 0.92);
		backdrop-filter: blur(14px);
		font-family: var(--font-mono);
		font-size: 11px;
	}

	.hud-progress {
		position: absolute;
		top: -1px;
		left: 0;
		height: 1px;
		background: var(--accent);
		box-shadow: 0 0 10px rgba(198, 161, 91, 0.6);
		transition: width 0.15s linear;
	}

	.hud-cluster {
		display: flex;
		align-items: center;
		gap: 16px;
		min-width: 0;
	}

	.hud-clock {
		font-variant-numeric: tabular-nums;
		color: var(--text);
		white-space: nowrap;
	}

	.hud-session {
		display: inline-flex;
		align-items: center;
		gap: 7px;
		font-size: 10px;
		letter-spacing: 0.12em;
		text-transform: uppercase;
		color: var(--dim);
		white-space: nowrap;
	}

	.hud-session i {
		width: 6px;
		height: 6px;
		border-radius: 50%;
		background: var(--faint);
	}

	.hud-session.open i {
		background: var(--up);
	}

	.hud-session.pre i,
	.hud-session.after i {
		background: var(--accent);
	}

	.hud-session.closed i {
		background: var(--down);
	}

	.hud-center {
		flex: 1;
		justify-content: center;
	}

	.hud-day {
		font-size: 10px;
		letter-spacing: 0.14em;
		text-transform: uppercase;
		color: var(--dim);
		white-space: nowrap;
	}

	.hud-bar {
		position: relative;
		width: clamp(70px, 12vw, 160px);
		height: 2px;
		background: rgba(255, 255, 255, 0.1);
	}

	.hud-bar i {
		position: absolute;
		inset: 0 auto 0 0;
		background: var(--accent);
	}

	.hud-pct {
		font-variant-numeric: tabular-nums;
		color: var(--accent);
	}

	.hud-latency {
		color: var(--dim);
		font-variant-numeric: tabular-nums;
		text-transform: uppercase;
		letter-spacing: 0.1em;
		font-size: 10px;
		white-space: nowrap;
	}

	.hud-mark {
		padding: 3px 8px;
		border: 1px solid var(--line-strong);
		font-size: 10px;
		font-weight: 700;
		letter-spacing: 0.12em;
		color: var(--text);
	}

	/* ── Entrance gating (motion layer only) ──────────────────────────────── */

	.gsap-on [data-entrance] {
		opacity: 0;
		visibility: hidden;
	}

	.gsap-on .hero-chart,
	.gsap-on .hero-flare,
	.gsap-on .hero-watermark {
		opacity: 0;
		visibility: hidden;
	}

	.gsap-on .hero-title .line-inner {
		transform: translateY(108%);
	}

	.gsap-on .scroll-cue {
		opacity: 0;
		visibility: hidden;
	}

	/* ── Utilities ────────────────────────────────────────────────────────── */

	.sr-only {
		position: absolute;
		width: 1px;
		height: 1px;
		padding: 0;
		margin: -1px;
		overflow: hidden;
		clip: rect(0, 0, 0, 0);
		white-space: nowrap;
		border: 0;
	}

	/* ── Responsive ───────────────────────────────────────────────────────── */

	@media (max-width: 1020px) {
		.hero-lower {
			grid-template-columns: 1fr;
		}

		.scope-stage {
			grid-template-columns: 1fr;
		}

		.academy-feature {
			grid-template-columns: 1fr;
		}

		.access-panel,
		.success-panel {
			grid-template-columns: 1fr;
		}

		.hud-center {
			display: none;
		}
	}

	@media (max-width: 760px) {
		.desk-grid {
			grid-template-columns: 1fr;
		}

		.desk-module {
			min-height: 0;
		}

		.sector-grid {
			grid-template-columns: repeat(3, minmax(0, 1fr));
		}

		.mini-feed-grid {
			grid-template-columns: 1fr;
		}

		.chart-toolbar {
			flex-direction: column;
			align-items: flex-start;
		}

		.scope-row {
			grid-template-columns: 40px 1fr;
		}

		.scope-metric {
			grid-column: 2;
			justify-self: start;
		}

		.academy-selector {
			grid-template-columns: 1fr;
		}

		.log-row {
			grid-template-columns: 56px 1fr;
		}

		.log-row p {
			grid-column: 1 / -1;
		}

		.chrome-brand {
			display: none;
		}

		.hud-latency {
			display: none;
		}

		.email-row {
			flex-direction: column;
		}

		.email-row button {
			padding: 14px;
		}
	}

	/* ── Reduced motion ───────────────────────────────────────────────────── */

	@media (prefers-reduced-motion: reduce) {
		.reveal {
			opacity: 1;
			transform: none;
			transition: none;
		}

		.aurora {
			animation: none;
		}

		section.visible .panel-feature::after {
			display: none;
		}

		.tape-track {
			animation: none;
		}

		.hero-flare {
			animation: none;
			opacity: 0.35;
		}

		.quote-live i {
			animation: none;
		}

		.print-row {
			animation: none;
		}

		.log-cursor {
			animation: none;
		}

		.range-track i,
		.book-row .depth,
		.progress-track span,
		.load-track span,
		.confidence span::after {
			transition: none;
		}
	}
</style>

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

	/* ── Date-driven content engine ──────────────────────────────────────────
	 * Everything below rotates with the calendar so the page is different on
	 * every visit: build progress climbs, the desk note changes at the open,
	 * and the build log always shows the last few trading days.
	 */
	const BUILD_EPOCH_UTC = Date.UTC(2026, 5, 1); // day 1 of the rebuild
	const DAY_MS = 86_400_000;

	const bootWordmark = 'REVOLUTION';

	const heroLines = [
		['We', 'didn’t', 'go', 'quiet.'],
		['We', 'went', 'to', 'work.']
	];
	// Rendered between word masks — Svelte trims plain whitespace at each-block
	// boundaries, so the gap has to be an explicit text expression.
	const wordGap = ' ';

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

	let mounted = $state(false);
	let reducedMotion = $state(false);
	let activeTimeframe = $state('5m');
	let activeAcademyTrack = $state<AcademyTrackId>('day');
	let scrollProgress = $state(0);
	let glareX = $state(50);
	let glareY = $state(30);

	let gsapActive = $state(false);
	let bootComplete = $state(false);
	let dayNumber = $state(0);
	let buildProgress = $state(88);
	let ringDisplay = $state(88);
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
	let chartInitGeneration = 0;
	let notifyAbortController: AbortController | null = null;
	let raf = 0;
	let sceneRaf = 0;
	let statsRaf = 0;
	let gsapContext: { revert: () => void } | null = null;
	let masterTimeline: { progress: (value: number) => unknown } | null = null;
	let cinemaCleanups: Array<() => void> = [];

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
		const cleanupPointer = setupPointerTracking();
		const cleanupCinema = startCinematicScene();

		// Failsafe: the intro must never trap the page. If the entrance hasn't
		// finished by now (throttled rAF, background tab, broken ticker), snap
		// the master timeline to its end state and drop the overlay.
		const bootFailsafe = window.setTimeout(() => {
			if (!bootComplete) {
				masterTimeline?.progress(1);
				bootComplete = true;
			}
		}, 4200);

		if (reducedMotion) {
			bootComplete = true;
		} else {
			void initCinematics();
		}

		visible.hero = true;

		return () => {
			mounted = false;
			chartInitGeneration += 1;
			notifyAbortController?.abort();
			notifyAbortController = null;
			cancelAnimationFrame(raf);
			cancelAnimationFrame(sceneRaf);
			cancelAnimationFrame(statsRaf);
			window.clearTimeout(bootFailsafe);
			cinemaCleanups.forEach((cleanup) => cleanup());
			cinemaCleanups = [];
			gsapContext?.revert();
			gsapContext = null;
			masterTimeline = null;
			cleanupReveal?.();
			cleanupIntervals();
			cleanupPointer();
			cleanupCinema?.();
			disposeCharts();
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
		ringDisplay = buildProgress;

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
		// "Next build drop" lands with the opening bell: 9:30 AM ET, Mon-Fri.
		const nyNow = new Date(new Date().toLocaleString('en-US', { timeZone: 'America/New_York' }));
		const target = nextBuildDrop(nyNow);

		const diff = Math.max(0, target.getTime() - nyNow.getTime());
		countdown = {
			hours: String(Math.floor(diff / 3_600_000)).padStart(2, '0'),
			minutes: String(Math.floor((diff % 3_600_000) / 60_000)).padStart(2, '0'),
			seconds: String(Math.floor((diff % 60_000) / 1000)).padStart(2, '0')
		};
	}

	/* ── GSAP cinematic layer ────────────────────────────────────────────────
	 * Loaded dynamically so the prerendered page stays lean and works with no
	 * JS at all (a CSS fallback exits the boot overlay on its own). When GSAP
	 * is up, the `.gsap-on` class hands every entrance over to timelines.
	 */
	async function initCinematics() {
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
				buildCinema(gsap, ScrollTrigger);
			}, shellEl);
		} catch (error) {
			console.error('[Maintenance] Cinematic layer failed to load', error);
			gsapActive = false;
			bootComplete = true;
		}
	}

	function buildCinema(gsap: GsapCore, ScrollTrigger: ScrollTriggerStatic) {
		ScrollTrigger.defaults({ scroller: shellEl });

		const master = gsap.timeline({ defaults: { ease: 'power3.out' } });
		masterTimeline = master;

		// Act I — boot sequence.
		master
			.to('.boot-line', { autoAlpha: 1, y: 0, duration: 0.45, stagger: 0.13 }, 0.1)
			.to('.boot-sub', { autoAlpha: 1, duration: 0.5 }, 0.4)
			.to(
				'.boot-letter-inner',
				{ yPercent: 0, duration: 0.75, ease: 'power4.out', stagger: 0.045 },
				0.26
			)
			.to('.boot-progress span', { scaleX: 1, duration: 1.1, ease: 'power2.inOut' }, 0.2)
			.addLabel('exit', 1.5)
			.to('.boot-overlay', { yPercent: -100, duration: 0.8, ease: 'power4.inOut' }, 'exit')
			.call(
				() => {
					bootComplete = true;
				},
				undefined,
				'exit+=0.8'
			);

		// Act II — hero entrance, overlapping the curtain lift.
		master
			.to('.market-tape', { y: 0, autoAlpha: 1, duration: 0.7 }, 'exit+=0.12')
			.to(
				'#hero .eyebrow',
				{ autoAlpha: 1, y: 0, filter: 'blur(0px)', duration: 0.6 },
				'exit+=0.28'
			)
			.to(
				'#hero .word-inner',
				{ y: 0, duration: 0.85, ease: 'power4.out', stagger: 0.055 },
				'exit+=0.36'
			)
			.to('#hero .lede', { autoAlpha: 1, y: 0, duration: 0.7 }, 'exit+=0.72')
			.to('#hero .hero-actions', { autoAlpha: 1, y: 0, duration: 0.7 }, 'exit+=0.82')
			.to('#hero .hero-note', { autoAlpha: 1, duration: 0.6 }, 'exit+=0.95')
			.to('#hero .drop-timer', { autoAlpha: 1, y: 0, duration: 0.7 }, 'exit+=1.02')
			.to(
				'#hero .command-deck',
				{ autoAlpha: 1, x: 0, rotateY: 0, duration: 0.9, ease: 'power4.out' },
				'exit+=0.55'
			);

		// The rollout ring rolls up from zero to today's real progress.
		const ringProxy = { value: 0 };
		ringDisplay = 0;
		master.to(
			ringProxy,
			{
				value: buildProgress,
				duration: 1.6,
				ease: 'power2.out',
				onUpdate: () => {
					ringDisplay = Math.round(ringProxy.value);
				}
			},
			'exit+=0.85'
		);

		// Act III — every section below the fold gets the same reveal grammar:
		// kicker slides in, headline rises, children stagger up.
		const sections = gsap.utils.toArray<HTMLElement>('.page-shell section:not(#hero)');
		sections.forEach((section) => {
			const kicker = section.querySelector('.section-kicker');
			const heading = section.querySelector('h2');
			const items = section.querySelectorAll('[data-cine] > *, [data-cine-self]');

			const timeline = gsap.timeline({
				defaults: { ease: 'power3.out' },
				scrollTrigger: { trigger: section, start: 'top 78%', once: true }
			});

			if (kicker) timeline.from(kicker, { x: -32, autoAlpha: 0, duration: 0.55 });
			if (heading) {
				timeline.from(heading, { y: 64, autoAlpha: 0, duration: 0.85, ease: 'power4.out' }, '<0.12');
			}
			if (items.length) {
				timeline.from(items, { y: 44, autoAlpha: 0, duration: 0.75, stagger: 0.07 }, '<0.2');
			}
		});

		// Depth: slow scrubbed drift on the hero deck, desk note, and chart stage.
		gsap.to('.command-deck', {
			yPercent: -9,
			ease: 'none',
			scrollTrigger: { trigger: '#hero', start: 'top top', end: 'bottom top', scrub: 0.5 }
		});
		gsap.fromTo(
			'.note-card',
			{ y: 48 },
			{
				y: -26,
				ease: 'none',
				scrollTrigger: { trigger: '#note', start: 'top bottom', end: 'bottom top', scrub: 0.6 }
			}
		);
		gsap.from('.note-card', {
			autoAlpha: 0,
			duration: 0.9,
			scrollTrigger: { trigger: '#note', start: 'top 75%', once: true }
		});
		gsap.fromTo(
			'.chart-stage',
			{ y: 44 },
			{
				y: -20,
				ease: 'none',
				scrollTrigger: { trigger: '#charts', start: 'top bottom', end: 'bottom top', scrub: 0.6 }
			}
		);
		gsap.from('.chart-stage', {
			autoAlpha: 0,
			duration: 0.9,
			scrollTrigger: { trigger: '#charts', start: 'top 75%', once: true }
		});

		wireMagnetic(gsap);
		wireTilt(gsap);
	}

	function wireMagnetic(gsap: GsapCore) {
		if (!window.matchMedia('(pointer: fine)').matches) return;

		shellEl.querySelectorAll<HTMLElement>('[data-magnetic]').forEach((node) => {
			const xTo = gsap.quickTo(node, 'x', { duration: 0.35, ease: 'power3.out' });
			const yTo = gsap.quickTo(node, 'y', { duration: 0.35, ease: 'power3.out' });

			const onMove = (event: PointerEvent) => {
				const rect = node.getBoundingClientRect();
				xTo(((event.clientX - rect.left) / rect.width - 0.5) * 14);
				yTo(((event.clientY - rect.top) / rect.height - 0.5) * 12);
			};
			const onLeave = () => {
				xTo(0);
				yTo(0);
			};

			node.addEventListener('pointermove', onMove, { passive: true });
			node.addEventListener('pointerleave', onLeave);
			cinemaCleanups.push(() => {
				node.removeEventListener('pointermove', onMove);
				node.removeEventListener('pointerleave', onLeave);
			});
		});
	}

	function wireTilt(gsap: GsapCore) {
		if (!window.matchMedia('(pointer: fine)').matches) return;

		shellEl.querySelectorAll<HTMLElement>('[data-tilt]').forEach((node) => {
			gsap.set(node, { transformPerspective: 900 });
			const rotateXTo = gsap.quickTo(node, 'rotationX', { duration: 0.5, ease: 'power2.out' });
			const rotateYTo = gsap.quickTo(node, 'rotationY', { duration: 0.5, ease: 'power2.out' });

			const onMove = (event: PointerEvent) => {
				const rect = node.getBoundingClientRect();
				const offsetX = (event.clientX - rect.left) / rect.width - 0.5;
				const offsetY = (event.clientY - rect.top) / rect.height - 0.5;
				rotateYTo(offsetX * 7);
				rotateXTo(offsetY * -7);
			};
			const onLeave = () => {
				rotateXTo(0);
				rotateYTo(0);
			};

			node.addEventListener('pointermove', onMove, { passive: true });
			node.addEventListener('pointerleave', onLeave);
			cinemaCleanups.push(() => {
				node.removeEventListener('pointermove', onMove);
				node.removeEventListener('pointerleave', onLeave);
			});
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
				textColor: 'rgba(238, 242, 255, 0.68)',
				fontFamily:
					'Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, sans-serif',
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

			mainSeries.setData(nextChartData.main);
			volumeSeries.priceScale().applyOptions({ scaleMargins: { top: 0.82, bottom: 0 } });
			volumeSeries.setData(
				nextChartData.main.map<HistogramData>((candle, index) => ({
					time: candle.time,
					value: 480000 + ((index * 13729) % 620000),
					color:
						candle.close >= candle.open ? 'rgba(22, 242, 169, 0.32)' : 'rgba(255, 95, 109, 0.32)'
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
	style:--glare-x={`${glareX}%`}
	style:--glare-y={`${glareY}%`}
	{@attach mountShell}
	onscroll={handleScroll}
>
	<div class="boot-overlay" class:done={bootComplete} aria-hidden="true">
		<div class="boot-terminal">
			<p class="boot-line">▸ routing market data <span>ok</span></p>
			<p class="boot-line">▸ arming scanner engine <span>ok</span></p>
			<p class="boot-line">▸ unlocking the floor <span>…</span></p>
		</div>
		<p class="boot-word">
			{#each bootWordmark.split('') as letter, index (index)}
				<span class="boot-letter" style:--boot-index={index}
					><span class="boot-letter-inner">{letter}</span></span
				>
			{/each}
		</p>
		<p class="boot-sub">Trading Pros — full platform rebuild</p>
		<div class="boot-progress"><span></span></div>
	</div>

	<div class="progress-rail" aria-hidden="true">
		<span></span>
	</div>

	<canvas class="cinematic-canvas" aria-hidden="true" {@attach mountCinematicCanvas}></canvas>

	<header class="market-tape" aria-label="Live market tape">
		<div class="tape-viewport">
			<div class="tape-track" aria-hidden="true">
				{#each [...liveTickers, ...liveTickers, ...liveTickers] as ticker, index (ticker.symbol + '-' + index)}
					<div
						class={{
							'ticker-pill': true,
							up: ticker.direction === 'up',
							down: ticker.direction === 'down'
						}}
					>
						<span class="ticker-symbol">{ticker.symbol}</span>
						<span>${ticker.price.toFixed(2)}</span>
						<span>{ticker.change >= 0 ? '+' : ''}{ticker.change.toFixed(2)}%</span>
					</div>
				{/each}
			</div>
		</div>
		<a class="tape-cta" href="#access" data-magnetic>Reserve access</a>
	</header>

	<div class="cinema-light" aria-hidden="true"></div>
	<div class="grid-field" aria-hidden="true"></div>

	<main class="page-shell">
		<section id="hero" class={{ hero: true, reveal: true, visible: visible.hero }}>
			<div class="hero-copy">
				<p class="eyebrow">
					<span></span>
					Rebuild day {dayNumber || '—'} — systems coming online
				</p>
				<h1 class="hero-title" aria-label="We didn't go quiet. We went to work.">
					{#each heroLines as line, lineIndex (lineIndex)}
						<span class="line" aria-hidden="true">
							{#each line as word, wordIndex (wordIndex)}<span class="word"
									><span class="word-inner">{word}</span></span
								>{wordGap}{/each}
						</span>
					{/each}
				</h1>
				<p class="lede">
					Faster data. Scanners that surface real edge. A trading university run like an
					institutional desk. We are rebuilding all of it — and the work lands on this page every
					trading day.
				</p>

				<div class="hero-actions">
					<a class="primary-link" href="#access" data-magnetic>Reserve first access</a>
					<a class="ghost-link" href="#operations">Watch the build</a>
				</div>
				<p class="hero-note">Founding-member pricing at launch · one email, zero spam</p>

				<div
					class="drop-timer"
					role="timer"
					aria-label="Countdown to the next build drop at 9:30 AM Eastern"
				>
					<div class="timer-label"><span class="pulse-dot"></span>Next build drop</div>
					<div class="timer-cells" aria-hidden="true">
						<div class="cell">
							{#key countdown.hours}<strong class="tick">{countdown.hours}</strong>{/key}
							<span>hrs</span>
						</div>
						<div class="sep">:</div>
						<div class="cell">
							{#key countdown.minutes}<strong class="tick">{countdown.minutes}</strong>{/key}
							<span>min</span>
						</div>
						<div class="sep">:</div>
						<div class="cell">
							{#key countdown.seconds}<strong class="tick">{countdown.seconds}</strong>{/key}
							<span>sec</span>
						</div>
					</div>
					<div class="timer-meta">9:30 AM ET · Mon–Fri</div>
				</div>
			</div>

			<div class="command-deck" aria-label="Rebuild status overview" data-tilt>
				<div class="deck-topline">
					<span>Rebuild command</span>
					<span class="live-status">Live</span>
				</div>
				<div class="deploy-ring" style:--ring-pct={ringDisplay}>
					<div class="ring-core">
						<span>{ringDisplay}%</span>
						<small>restored</small>
					</div>
				</div>
				<div class="deploy-steps">
					{#each stageLines as line, index (line)}
						<div
							class={{
								'deploy-step': true,
								complete: index < completedStages,
								active: index === completedStages
							}}
						>
							<span>{String(index + 1).padStart(2, '0')}</span>
							<p>{line}</p>
						</div>
					{/each}
				</div>
				<div class="deck-day">
					<span>Day {dayNumber || '—'}</span>
					<span>Next drop {countdown.hours}:{countdown.minutes}:{countdown.seconds}</span>
				</div>
			</div>
		</section>

		<section id="note" class={{ 'note-section': true, reveal: true, visible: visible.note }}>
			<div class="section-heading">
				<p class="section-kicker">Daily desk note</p>
				<h2>One sharp idea, every trading day.</h2>
			</div>

			<figure class="note-card" data-tilt>
				<div class="note-glyph" aria-hidden="true">“</div>
				<blockquote>{deskNote}</blockquote>
				<p class="note-foot">
					A new note drops at the open, Monday through Friday. <strong>Come back tomorrow.</strong>
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
			<div class="section-heading ops-heading">
				<p class="section-kicker">Trading operations center</p>
				<h2>Every system, rebuilt to institutional spec.</h2>
			</div>

			<div class="ops-stage">
				<div class="ops-reel" aria-label="Trading operations stack" data-cine>
					{#each operationsChapters as chapter, index (chapter.number)}
						<article class="reel-card" style:--chapter-delay={`${index * 110}ms`}>
							<span>{chapter.number}</span>
							<h3>{chapter.title}</h3>
							<p>{chapter.copy}</p>
							<strong>{chapter.metric}</strong>
						</article>
					{/each}
				</div>

				<div class="systems-theater" aria-label="Trading systems status" data-cine-self>
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

		<section id="charts" class={{ 'charts-section': true, reveal: true, visible: visible.charts }}>
			<div class="section-heading">
				<p class="section-kicker">Market data lab</p>
				<h2>The data engine never went dark.</h2>
			</div>

			<div class="chart-stage">
				<div class="chart-toolbar">
					<div>
						<span class="market-label">SPX upgrade feed</span>
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
						<span>Confidence</span>
						<strong>{averageConfidence}%</strong>
					</div>
				</div>
			</div>

			<div class="mini-feed-grid" data-cine>
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

		<section
			id="signals"
			class={{ 'signals-section': true, reveal: true, visible: visible.signals }}
		>
			<div class="section-heading">
				<p class="section-kicker">Scanner engine</p>
				<h2>Built for traders who need signal clarity under pressure.</h2>
			</div>

			<div class="signals-layout">
				<div class="radar-panel" data-cine-self>
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

				<div class="signals-table" data-cine-self>
					<div class="table-head">
						<span>Detected signals</span>
						<span>{averageConfidence}% avg confidence</span>
					</div>
					{#each scannerSignals as signal (signal.id)}
						<article class={{ 'signal-row': true, cooling: signal.status === 'cooling' }}>
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

		<section
			id="academy"
			class={{ 'academy-section': true, reveal: true, visible: visible.academy }}
		>
			<div class="section-heading">
				<p class="section-kicker">Trading university</p>
				<h2>
					Stocks, options, and institution-grade desk training in one professional curriculum path.
				</h2>
			</div>

			<div class="academy-cinema">
				<div
					class="academy-selector"
					role="tablist"
					aria-label="Trading university curriculum"
					data-cine
				>
					{#each academyTracks as track (track.id)}
						<button
							type="button"
							role="tab"
							aria-selected={activeAcademyTrack === track.id}
							class={{ active: activeAcademyTrack === track.id }}
							style:--track-accent={track.accent}
							onclick={() => (activeAcademyTrack = track.id)}
						>
							<span>{track.number}</span>
							<strong>{track.title}</strong>
							<small>Stocks + options</small>
						</button>
					{/each}
				</div>

				<div class="academy-feature" style:--track-accent={activeAcademy.accent} data-cine-self>
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
			class={{ 'infra-section': true, reveal: true, visible: visible.infrastructure }}
		>
			<div class="section-heading">
				<p class="section-kicker">Build telemetry</p>
				<h2>Watch the numbers climb while we work.</h2>
			</div>

			<div class="infra-grid">
				<div class="node-list" data-cine-self>
					{#each nodes as node (node.id)}
						<article class={{ 'node-row': true, migrating: node.status === 'migrating' }}>
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
				<h2>Shipped while you were away.</h2>
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
			<p class="log-note">Updated every trading day — tomorrow's entry lands at the open.</p>
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
						<h2>You're on the floor list.</h2>
						<p>
							Confirmation is on its way to your inbox. Come back tomorrow at the open — the desk
							note and build log refresh every trading day.
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
						<h2>Be first through the doors.</h2>
						<p>
							One email the moment we reopen — nothing else. Founding members get launch pricing,
							launch-week live sessions, and the first look at the new scanner suite.
						</p>
						<ul class="perk-row">
							<li>Founding-member pricing</li>
							<li>Launch-week live sessions</li>
							<li>First look at the scanners</li>
						</ul>
					</div>

					<div class="email-block">
						<div class="email-row">
							<label class="sr-only" for="maintenance-email">Email address</label>
							<input
								id="maintenance-email"
								type="email"
								placeholder="you@yourdesk.com"
								bind:value={email}
								disabled={isSubmitting}
								autocomplete="email"
							/>
							<button type="submit" disabled={isSubmitting || !email}>
								{isSubmitting ? 'Reserving…' : 'Reserve my seat'}
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
				<span>Rebuilt in public. Reopening soon.</span>
			</div>
			<p class="footer-risk">
				Trading involves substantial risk of loss and is not suitable for every investor. Education —
				not financial advice.
			</p>
			<span class="footer-legal">© 2026 Revolution Trading Pros</span>
		</footer>
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

	/* ── Boot overlay ─────────────────────────────────────────────────────── */

	.boot-overlay {
		position: fixed;
		inset: 0;
		z-index: 90;
		display: grid;
		place-content: center;
		justify-items: center;
		gap: 22px;
		padding: 24px;
		background:
			radial-gradient(720px 420px at 50% 38%, rgba(248, 212, 119, 0.07), transparent 68%),
			#05060a;
		animation: boot-exit 0.7s 2.3s cubic-bezier(0.76, 0, 0.24, 1) forwards;
	}

	.boot-overlay.done {
		display: none !important;
	}

	.boot-terminal {
		display: grid;
		gap: 6px;
		min-height: 66px;
	}

	.boot-line {
		display: flex;
		align-items: baseline;
		gap: 10px;
		margin: 0;
		color: rgba(139, 211, 255, 0.72);
		font-family: 'SF Mono', ui-monospace, Menlo, monospace;
		font-size: 12px;
		letter-spacing: 0.04em;
		opacity: 0;
		transform: translateY(10px);
		animation: boot-fade 0.5s ease forwards;
	}

	.boot-line span {
		color: var(--green);
		font-weight: 700;
	}

	.boot-line:nth-child(1) {
		animation-delay: 0.15s;
	}

	.boot-line:nth-child(2) {
		animation-delay: 0.32s;
	}

	.boot-line:nth-child(3) {
		animation-delay: 0.49s;
	}

	.boot-word {
		display: flex;
		margin: 0;
		font-size: clamp(2.6rem, 9vw, 6.2rem);
		font-weight: 900;
		letter-spacing: 0.08em;
		line-height: 1;
	}

	.boot-letter {
		display: inline-block;
		overflow: hidden;
		padding-bottom: 0.06em;
	}

	.boot-letter-inner {
		display: inline-block;
		background: linear-gradient(180deg, #fff8e8, var(--gold) 62%, var(--copper));
		background-clip: text;
		-webkit-background-clip: text;
		color: transparent;
		transform: translateY(112%);
		animation: boot-rise 0.7s cubic-bezier(0.16, 1, 0.3, 1) forwards;
		animation-delay: calc(0.28s + var(--boot-index) * 0.05s);
	}

	.boot-sub {
		margin: 0;
		color: var(--dim);
		font-size: 12px;
		font-weight: 700;
		letter-spacing: 0.34em;
		text-transform: uppercase;
		opacity: 0;
		animation: boot-fade 0.6s 0.55s ease forwards;
	}

	.boot-progress {
		width: min(340px, 64vw);
		height: 2px;
		overflow: hidden;
		border-radius: 999px;
		background: rgba(255, 255, 255, 0.1);
	}

	.boot-progress span {
		display: block;
		height: 100%;
		background: linear-gradient(90deg, var(--gold), var(--green));
		box-shadow: 0 0 18px rgba(22, 242, 169, 0.55);
		transform: scaleX(0);
		transform-origin: left center;
		animation: boot-bar 1.7s 0.2s cubic-bezier(0.65, 0, 0.35, 1) forwards;
	}

	/* When GSAP owns the show, park the CSS fallback and hold initial states. */

	.gsap-on .boot-overlay,
	.gsap-on .boot-line,
	.gsap-on .boot-letter-inner,
	.gsap-on .boot-sub,
	.gsap-on .boot-progress span {
		animation: none;
	}

	.gsap-on .boot-line {
		opacity: 0;
		transform: translateY(10px);
	}

	.gsap-on .boot-letter-inner {
		transform: translateY(112%);
	}

	.gsap-on .boot-sub {
		opacity: 0;
	}

	.gsap-on .boot-progress span {
		transform: scaleX(0);
	}

	.gsap-on .market-tape {
		opacity: 0;
		transform: translateY(-102%);
	}

	.gsap-on #hero .eyebrow {
		opacity: 0;
		transform: translateY(18px);
		filter: blur(6px);
	}

	.gsap-on #hero .word-inner {
		transform: translateY(115%);
	}

	.gsap-on #hero .lede,
	.gsap-on #hero .hero-actions,
	.gsap-on #hero .drop-timer {
		opacity: 0;
		transform: translateY(26px);
	}

	.gsap-on #hero .hero-note {
		opacity: 0;
	}

	.gsap-on #hero .command-deck {
		opacity: 0;
		transform: translateX(48px) rotateY(-6deg);
	}

	.gsap-on .reel-card,
	.gsap-on .curriculum-timeline article {
		animation: none;
	}

	/* ── Chrome ───────────────────────────────────────────────────────────── */

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
		display: flex;
		align-items: center;
		gap: 12px;
		border-bottom: 1px solid var(--line);
		background: rgba(6, 7, 10, 0.82);
		backdrop-filter: blur(18px);
	}

	.tape-viewport {
		flex: 1;
		min-width: 0;
		overflow: hidden;
		mask-image: linear-gradient(90deg, transparent, black 4%, black 96%, transparent);
	}

	.market-tape:hover .tape-track {
		animation-play-state: paused;
	}

	.tape-track {
		display: flex;
		width: max-content;
		gap: 10px;
		padding: 10px 0;
		animation: tape 38s linear infinite;
	}

	.tape-cta {
		flex-shrink: 0;
		margin-right: 14px;
		border: 1px solid rgba(248, 212, 119, 0.5);
		border-radius: 999px;
		padding: 7px 16px;
		color: var(--gold);
		font-size: 12px;
		font-weight: 800;
		letter-spacing: 0.08em;
		text-decoration: none;
		text-transform: uppercase;
		background: rgba(248, 212, 119, 0.08);
		transition:
			background 180ms ease,
			color 180ms ease;
	}

	.tape-cta:hover {
		background: var(--gold);
		color: #161008;
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
		padding: clamp(42px, 8vw, 96px) 0 40px;
	}

	/* ── Hero ─────────────────────────────────────────────────────────────── */

	.hero {
		display: grid;
		grid-template-columns: minmax(0, 1.08fr) minmax(330px, 0.62fr);
		gap: clamp(24px, 5vw, 70px);
		align-items: center;
		min-height: min(860px, calc(100vh - 48px));
		padding-bottom: 6vh;
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
		font-size: clamp(3rem, 7vw, 6.4rem);
		font-weight: 900;
		letter-spacing: -0.02em;
		text-wrap: balance;
	}

	.hero-title .line {
		display: block;
		line-height: 1;
	}

	.hero-title .line + .line {
		margin-top: -0.06em;
	}

	.hero-title .word {
		display: inline-block;
		overflow: hidden;
		padding: 0 0.02em 0.08em;
		margin-bottom: -0.08em;
		vertical-align: top;
	}

	.hero-title .word-inner {
		display: inline-block;
		will-change: transform;
	}

	.hero-title .line:last-child .word-inner {
		background: linear-gradient(180deg, #fff8e8, var(--gold) 58%, var(--copper));
		background-clip: text;
		-webkit-background-clip: text;
		color: transparent;
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
		margin: 22px 0 0;
		color: var(--muted);
		font-size: clamp(1.05rem, 1.5vw, 1.28rem);
		line-height: 1.7;
	}

	.hero-actions {
		display: flex;
		flex-wrap: wrap;
		gap: 12px;
		margin-top: 26px;
	}

	.hero-note {
		margin: 14px 0 0;
		color: var(--dim);
		font-size: 13px;
		letter-spacing: 0.02em;
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

	/* ── Countdown ────────────────────────────────────────────────────────── */

	.drop-timer {
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		gap: 14px 22px;
		width: max-content;
		max-width: 100%;
		margin-top: 24px;
		border: 1px solid var(--line);
		border-radius: 8px;
		padding: 14px 18px;
		background: rgba(8, 10, 14, 0.6);
		backdrop-filter: blur(18px);
	}

	.timer-label {
		display: inline-flex;
		align-items: center;
		gap: 9px;
		color: var(--dim);
		font-size: 11px;
		font-weight: 900;
		letter-spacing: 0.16em;
		text-transform: uppercase;
	}

	.pulse-dot {
		width: 7px;
		height: 7px;
		border-radius: 999px;
		background: var(--green);
		animation: dot-pulse 1.8s ease-in-out infinite;
	}

	.timer-cells {
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.timer-cells .cell {
		display: grid;
		justify-items: center;
		gap: 2px;
		min-width: 52px;
		border: 1px solid rgba(255, 255, 255, 0.08);
		border-radius: 6px;
		padding: 7px 8px 5px;
		background: rgba(255, 255, 255, 0.035);
	}

	.timer-cells strong {
		overflow: hidden;
		color: var(--text);
		font-family: 'SF Mono', ui-monospace, Menlo, monospace;
		font-size: 20px;
		font-weight: 800;
		line-height: 1;
	}

	.timer-cells .tick {
		display: inline-block;
		animation: digit-in 260ms cubic-bezier(0.16, 1, 0.3, 1);
	}

	.timer-cells .cell span {
		color: var(--dim);
		font-size: 9px;
		font-weight: 800;
		letter-spacing: 0.14em;
		text-transform: uppercase;
	}

	.timer-cells .sep {
		color: var(--dim);
		font-family: 'SF Mono', ui-monospace, Menlo, monospace;
		font-weight: 800;
	}

	.timer-meta {
		color: var(--dim);
		font-family: 'SF Mono', ui-monospace, Menlo, monospace;
		font-size: 11px;
		letter-spacing: 0.06em;
	}

	/* ── Panels ───────────────────────────────────────────────────────────── */

	.command-deck,
	.chart-stage,
	.mini-feed,
	.radar-panel,
	.signals-table,
	.node-list,
	.stat-board,
	.access-panel,
	.success-panel,
	.note-card,
	.log-panel {
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
		width: min(250px, 74vw);
		aspect-ratio: 1;
		margin: 24px auto;
		border-radius: 50%;
		background:
			conic-gradient(
				var(--green) 0 calc(var(--ring-pct, 0) * 1%),
				rgba(255, 255, 255, 0.1) calc(var(--ring-pct, 0) * 1%) 100%
			),
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
		font-variant-numeric: tabular-nums;
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

	.deploy-step.active {
		border-color: rgba(248, 212, 119, 0.34);
		animation: step-live 2.4s ease-in-out infinite;
	}

	.deploy-step.active span,
	.deploy-step.active p {
		color: var(--gold);
	}

	.deck-day {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 12px;
		margin-top: 12px;
		border-top: 1px solid var(--line);
		padding-top: 12px;
		color: var(--dim);
		font-family: 'SF Mono', ui-monospace, Menlo, monospace;
		font-size: 11px;
		letter-spacing: 0.06em;
		text-transform: uppercase;
	}

	.reveal {
		transition:
			opacity 820ms cubic-bezier(0.16, 1, 0.3, 1),
			transform 820ms cubic-bezier(0.16, 1, 0.3, 1);
	}

	/* Hide-for-reveal only once JS is live — the prerendered page (no JS,
	   pre-hydration, ancient browsers) stays fully readable. */
	.mounted .reveal {
		opacity: 0;
		transform: translateY(42px);
	}

	.reveal.visible {
		opacity: 1;
		transform: translateY(0);
	}

	/* GSAP owns every entrance when active — the CSS reveal stands down. */
	.gsap-on .reveal {
		opacity: 1;
		transform: none;
		transition: none;
	}

	.note-section,
	.ops-section,
	.charts-section,
	.signals-section,
	.academy-section,
	.infra-section,
	.buildlog-section,
	.access-section {
		padding: clamp(52px, 9vw, 110px) 0;
	}

	.section-heading {
		max-width: 780px;
		margin-bottom: 24px;
	}

	/* ── Daily desk note ──────────────────────────────────────────────────── */

	.note-section {
		padding-top: clamp(10px, 2vw, 30px);
	}

	.note-card {
		position: relative;
		display: flex;
		flex-direction: column;
		overflow: hidden;
		max-width: 980px;
		padding: clamp(26px, 5vw, 52px);
		border-color: var(--line-strong);
		background:
			radial-gradient(620px 300px at 88% 0%, rgba(248, 212, 119, 0.1), transparent 62%),
			linear-gradient(180deg, rgba(255, 255, 255, 0.045), transparent 46%),
			rgba(12, 15, 20, 0.82);
		margin: 0;
	}

	.note-glyph {
		position: absolute;
		top: -0.12em;
		right: 18px;
		color: rgba(248, 212, 119, 0.14);
		font-family: Georgia, 'Times New Roman', serif;
		font-size: clamp(9rem, 22vw, 17rem);
		font-weight: 700;
		line-height: 1;
		pointer-events: none;
	}

	.note-meta {
		order: -1;
		display: flex;
		align-items: baseline;
		gap: 12px;
		color: var(--dim);
		font-size: 12px;
		font-weight: 900;
		letter-spacing: 0.16em;
		text-transform: uppercase;
	}

	.note-meta strong {
		color: var(--gold);
		font-family: 'SF Mono', ui-monospace, Menlo, monospace;
		font-size: 14px;
	}

	.note-card blockquote {
		max-width: 820px;
		margin: 22px 0 0;
		font-size: clamp(1.5rem, 3.4vw, 2.7rem);
		font-weight: 800;
		line-height: 1.22;
		text-wrap: balance;
	}

	.note-foot {
		margin: 26px 0 0;
		color: var(--muted);
		font-size: 14px;
	}

	.note-foot strong {
		color: var(--gold);
	}

	/* ── Operations ───────────────────────────────────────────────────────── */

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

	/* ── Charts ───────────────────────────────────────────────────────────── */

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

	/* ── Signals ──────────────────────────────────────────────────────────── */

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

	/* ── Academy ──────────────────────────────────────────────────────────── */

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

	/* ── Infrastructure ───────────────────────────────────────────────────── */

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

	/* ── Build log ────────────────────────────────────────────────────────── */

	.buildlog-section {
		padding-bottom: clamp(30px, 5vw, 60px);
	}

	.log-panel {
		display: grid;
		gap: 0;
		overflow: hidden;
		padding: 6px;
	}

	.log-row {
		display: grid;
		grid-template-columns: 74px 96px minmax(0, 1fr);
		gap: 14px;
		align-items: baseline;
		border-radius: 6px;
		padding: 15px 14px;
	}

	.log-row + .log-row {
		border-top: 1px solid rgba(255, 255, 255, 0.06);
	}

	.log-row:hover {
		background: rgba(255, 255, 255, 0.03);
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
		border-radius: 999px;
		padding: 3px 10px;
		font-family: 'SF Mono', ui-monospace, Menlo, monospace;
		font-size: 10px;
		font-weight: 900;
		letter-spacing: 0.12em;
		text-transform: uppercase;
	}

	.log-tag.shipped {
		border: 1px solid rgba(22, 242, 169, 0.32);
		color: var(--green);
		background: rgba(22, 242, 169, 0.08);
	}

	.log-tag.tuned {
		border: 1px solid rgba(248, 212, 119, 0.32);
		color: var(--gold);
		background: rgba(248, 212, 119, 0.08);
	}

	.log-tag.hardened {
		border: 1px solid rgba(139, 211, 255, 0.32);
		color: var(--cyan);
		background: rgba(139, 211, 255, 0.08);
	}

	.log-row p {
		margin: 0;
		color: var(--muted);
		line-height: 1.6;
	}

	.log-note {
		margin: 14px 0 0;
		color: var(--dim);
		font-family: 'SF Mono', ui-monospace, Menlo, monospace;
		font-size: 12px;
		letter-spacing: 0.04em;
	}

	/* ── Access ───────────────────────────────────────────────────────────── */

	.access-section {
		padding-top: clamp(30px, 5vw, 60px);
		padding-bottom: 8vh;
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

	.perk-row {
		display: flex;
		flex-wrap: wrap;
		gap: 8px;
		margin: 18px 0 0;
		padding: 0;
		list-style: none;
	}

	.perk-row li {
		display: inline-flex;
		align-items: center;
		gap: 7px;
		border: 1px solid rgba(22, 242, 169, 0.22);
		border-radius: 999px;
		padding: 6px 12px;
		color: var(--muted);
		font-size: 12px;
		font-weight: 700;
		background: rgba(22, 242, 169, 0.05);
	}

	.perk-row li::before {
		content: '✓';
		color: var(--green);
		font-weight: 900;
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

	/* ── Footer ───────────────────────────────────────────────────────────── */

	.floor-footer {
		display: grid;
		gap: 10px;
		border-top: 1px solid var(--line);
		padding: 26px 0 34px;
	}

	.footer-brand {
		display: flex;
		flex-wrap: wrap;
		align-items: baseline;
		gap: 12px;
	}

	.footer-brand strong {
		font-size: 15px;
		font-weight: 900;
		letter-spacing: 0.02em;
	}

	.footer-brand span {
		color: var(--dim);
		font-size: 13px;
	}

	.footer-risk {
		max-width: 720px;
		margin: 0;
		color: var(--dim);
		font-size: 12px;
		line-height: 1.6;
	}

	.footer-legal {
		color: var(--dim);
		font-family: 'SF Mono', ui-monospace, Menlo, monospace;
		font-size: 11px;
		letter-spacing: 0.06em;
	}

	.sr-only {
		position: absolute;
		width: 1px;
		height: 1px;
		overflow: hidden;
		clip: rect(0, 0, 0, 0);
		white-space: nowrap;
	}

	/* ── Keyframes ────────────────────────────────────────────────────────── */

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

	@keyframes boot-fade {
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	@keyframes boot-rise {
		to {
			transform: translateY(0);
		}
	}

	@keyframes boot-bar {
		to {
			transform: scaleX(1);
		}
	}

	@keyframes boot-exit {
		to {
			transform: translateY(-100%);
			visibility: hidden;
		}
	}

	@keyframes dot-pulse {
		50% {
			opacity: 0.35;
			transform: scale(0.8);
		}
	}

	@keyframes digit-in {
		from {
			opacity: 0;
			transform: translateY(45%);
			filter: blur(2px);
		}
	}

	@keyframes step-live {
		50% {
			box-shadow: 0 0 24px rgba(248, 212, 119, 0.14);
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
			padding-top: 56px;
		}

		.tape-cta {
			display: none;
		}
	}

	@media (max-width: 720px) {
		.page-shell {
			width: min(100% - 24px, 1180px);
		}

		h1 {
			font-size: clamp(3rem, 15vw, 4.5rem);
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

		.log-row {
			grid-template-columns: 1fr;
			gap: 8px;
		}

		.drop-timer {
			width: 100%;
		}

		.chart-canvas {
			height: 340px;
		}

		.systems-theater {
			min-height: 420px;
		}
	}

	@media (prefers-reduced-motion: reduce) {
		.boot-overlay {
			display: none !important;
		}

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

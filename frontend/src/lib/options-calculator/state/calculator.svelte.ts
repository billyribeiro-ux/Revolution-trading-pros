// ============================================================
// CALCULATOR STATE — Svelte 5 Runes
// Single source of truth for all calculator state.
//
// A class with `$state` / `$derived` fields (Svelte docs: "$state → Classes").
// Methods are arrow-function fields so they keep `this` when passed as
// callbacks (`onclick={calc.resetInputs}`). State that consumers may only read
// lives in a private `#field` behind a getter; derived values and the reactive
// `liveOverrides` set are `readonly` fields.
// Construct one per calculator and share it through `setCalculator()` in
// `./context.ts` — never as a module-level singleton (SSR would leak it).
// ============================================================

import {
	price,
	priceOption,
	intrinsicValue,
	moneyness,
	breakeven
} from '../engine/black-scholes.js';
import { allGreeks } from '../engine/greeks.js';
import { calculateProbabilities } from '../engine/probability.js';
import {
	calculateStrategyGreeks,
	calculateStrategyPayoff,
	findBreakevens,
	findMaxProfitLoss,
	generatePriceRange
} from '../engine/strategies.js';
import { DEFAULT_INPUTS } from '../engine/constants.js';
import { runMonteCarlo, runLightMonteCarlo } from '../engine/monte-carlo.js';
import {
	SCENARIO_PRESETS,
	calculateScenarios,
	generateTimeMachineSnapshots,
	generateSensitivityData
} from '../engine/scenarios.js';
import type {
	BSInputs,
	OptionType,
	BSPricingResult,
	AllGreeks,
	ProbabilityResult,
	StrategyLeg,
	PayoffPoint,
	ThemeMode,
	ChartTab,
	HeatmapGreek,
	CalculatorMode,
	MonteCarloConfig,
	MonteCarloResult,
	Scenario,
	ScenarioResult,
	ShareableState,
	TimeMachineSnapshot,
	SensitivityData,
	ToastItem,
	ToastType
} from '../engine/types.js';
import type {
	MonteCarloWorkerRequest,
	MonteCarloWorkerResponse
} from '../engine/monte-carlo.worker.js';
import type { DataMode, MarketSnapshot } from '../data/types.js';
import { SvelteSet } from 'svelte/reactivity';
import { logger } from '#lib/utils/logger.js';

/** Inputs a market snapshot fills in (see `applyMarketSnapshot`). */
const LIVE_INPUT_KEYS = [
	'spotPrice',
	'volatility',
	'dividendYield',
	'riskFreeRate'
] as const satisfies readonly (keyof BSInputs)[];

/**
 * Run one Monte Carlo simulation in a dedicated Web Worker, terminating it
 * once it answers. Rejects where workers are unavailable (SSR, a blocked
 * worker script) so the caller can fall back to the main thread.
 */
function simulateInWorker(request: MonteCarloWorkerRequest): Promise<MonteCarloResult> {
	return new Promise((resolve, reject) => {
		const worker = new Worker(new URL('../engine/monte-carlo.worker.ts', import.meta.url), {
			type: 'module'
		});
		worker.onmessage = (event: MessageEvent<MonteCarloWorkerResponse>) => {
			worker.terminate();
			if (event.data.ok) resolve(event.data.result);
			else reject(new Error(event.data.error));
		};
		worker.onerror = (event) => {
			worker.terminate();
			reject(new Error(event.message || 'Monte Carlo worker failed to load'));
		};
		try {
			worker.postMessage(request);
		} catch (err) {
			worker.terminate();
			reject(err);
		}
	});
}

/**
 * Reactive state for one options calculator instance.
 * Create it once in the root Calculator component (`new CalculatorState()`).
 */
export class CalculatorState {
	// ── Core Inputs ──────────────────────────────────────
	spotPrice = $state(DEFAULT_INPUTS.spotPrice);
	strikePrice = $state(DEFAULT_INPUTS.strikePrice);
	volatility = $state(DEFAULT_INPUTS.volatility);
	timeToExpiry = $state(DEFAULT_INPUTS.timeToExpiry);
	riskFreeRate = $state(DEFAULT_INPUTS.riskFreeRate);
	dividendYield = $state(DEFAULT_INPUTS.dividendYield);

	// ── UI State ─────────────────────────────────────────
	optionType = $state<OptionType>('call');
	#theme = $state<ThemeMode>('dark');
	activeTab = $state<ChartTab>('payoff');
	heatmapGreek = $state<HeatmapGreek>('delta');
	calculatorMode = $state<CalculatorMode>('single');
	showAdvancedGreeks = $state(false);
	showProbabilities = $state(true);
	isInputPanelOpen = $state(true);

	/** Read-only; change it with `toggleTheme()`. */
	get theme(): ThemeMode {
		return this.#theme;
	}

	// ── Strategy State ───────────────────────────────────
	// Raw: legs are replaced, never mutated in place, so a deep proxy buys nothing.
	#strategyLegs = $state.raw<StrategyLeg[]>([]);

	get strategyLegs(): StrategyLeg[] {
		return this.#strategyLegs;
	}

	// ── Derived: Assembled Inputs ────────────────────────
	readonly inputs = $derived<BSInputs>({
		spotPrice: this.spotPrice,
		strikePrice: this.strikePrice,
		volatility: this.volatility,
		timeToExpiry: this.timeToExpiry,
		riskFreeRate: this.riskFreeRate,
		dividendYield: this.dividendYield
	});

	// ── Derived: Pricing ─────────────────────────────────
	readonly pricing = $derived<BSPricingResult>(price(this.inputs));

	readonly currentPrice = $derived(
		this.optionType === 'call' ? this.pricing.callPrice : this.pricing.putPrice
	);

	readonly currentIntrinsic = $derived(
		intrinsicValue(this.spotPrice, this.strikePrice, this.optionType)
	);

	readonly currentExtrinsic = $derived(Math.max(this.currentPrice - this.currentIntrinsic, 0));

	readonly currentMoneyness = $derived(
		moneyness(this.spotPrice, this.strikePrice, this.optionType)
	);

	readonly currentBreakeven = $derived(
		breakeven(this.strikePrice, this.currentPrice, this.optionType)
	);

	// ── Derived: Greeks ──────────────────────────────────
	readonly callGreeks = $derived<AllGreeks>(allGreeks(this.inputs, 'call'));
	readonly putGreeks = $derived<AllGreeks>(allGreeks(this.inputs, 'put'));
	readonly currentGreeks = $derived<AllGreeks>(
		this.optionType === 'call' ? this.callGreeks : this.putGreeks
	);

	// ── Derived: Probabilities ───────────────────────────
	readonly probabilities = $derived<ProbabilityResult>(
		calculateProbabilities(this.inputs, this.optionType)
	);

	// ── Derived: Strategy Pricing ────────────────────────
	/**
	 * The strategy's legs priced at the current inputs: each leg's `premium` is the cash flow
	 * of buying one unit at its Black-Scholes value (negative: paid), the convention of the
	 * payoff engine and of the single-option payoff below. Pricing live (rather than trusting
	 * a stored premium) keeps shared links and saved configs, which carry no prices, exact.
	 */
	readonly pricedStrategyLegs = $derived<StrategyLeg[]>(
		this.#strategyLegs.map((leg) => ({
			...leg,
			premium: -priceOption(
				{ ...this.inputs, strikePrice: leg.strike, timeToExpiry: leg.expiry },
				leg.type
			)
		}))
	);

	/** Net premium of the strategy: positive is a credit received, negative a debit paid. */
	readonly strategyNetPremium = $derived(
		this.pricedStrategyLegs.reduce((sum, leg) => sum + leg.premium * leg.position * leg.quantity, 0)
	);

	/** Position-weighted first-order Greeks of the whole strategy. */
	readonly strategyGreeks = $derived(calculateStrategyGreeks(this.#strategyLegs, this.inputs));

	// ── Derived: Payoff Diagram ──────────────────────────
	readonly payoffData = $derived.by<PayoffPoint[]>(() => {
		if (this.calculatorMode === 'single' || this.#strategyLegs.length === 0) {
			const priceRange = generatePriceRange(this.spotPrice, [this.strikePrice]);
			const singleLeg: StrategyLeg = {
				id: 'single',
				type: this.optionType,
				strike: this.strikePrice,
				expiry: this.timeToExpiry,
				position: 1,
				quantity: 1,
				premium: -this.currentPrice
			};
			return calculateStrategyPayoff([singleLeg], priceRange);
		}
		const legs = this.pricedStrategyLegs;
		const priceRange = generatePriceRange(
			this.spotPrice,
			legs.map((l) => l.strike)
		);
		return calculateStrategyPayoff(legs, priceRange);
	});

	// ── Derived: Strategy Metrics ────────────────────────
	readonly strategyMaxProfitLoss = $derived(findMaxProfitLoss(this.payoffData));
	readonly strategyBreakevens = $derived(findBreakevens(this.payoffData));

	// ── Phase 2: Monte Carlo State ──────────────────────
	monteCarloConfig = $state<MonteCarloConfig>({ numPaths: 2000, numSteps: 100 });
	// Raw: thousands of paths, only ever replaced wholesale.
	#monteCarloResult = $state.raw<MonteCarloResult | null>(null);
	#isMonteCarloRunning = $state(false);
	/** Incremented per run so a slower, superseded run never overwrites a newer result. */
	#monteCarloRun = 0;

	get monteCarloResult(): MonteCarloResult | null {
		return this.#monteCarloResult;
	}

	get isMonteCarloRunning(): boolean {
		return this.#isMonteCarloRunning;
	}

	// ── Phase 2: Scenario State ─────────────────────────
	#activeScenarios = $state.raw<Scenario[]>([]);
	readonly scenarioPresets: Scenario[] = SCENARIO_PRESETS;
	isScenarioPanelOpen = $state(false);

	get activeScenarios(): Scenario[] {
		return this.#activeScenarios;
	}

	readonly scenarioResults = $derived<ScenarioResult[]>(
		this.#activeScenarios.length > 0
			? calculateScenarios(this.inputs, this.#activeScenarios, this.optionType)
			: []
	);

	// ── Phase 2: Time Machine State ─────────────────────
	timeMachineDay = $state(0);
	readonly timeMachineMaxDays = $derived(Math.max(1, Math.round(this.timeToExpiry * 365)));
	readonly timeMachineSnapshots = $derived<TimeMachineSnapshot[]>(
		generateTimeMachineSnapshots(this.inputs, this.optionType, 100)
	);
	readonly currentTimeMachineSnapshot = $derived.by<TimeMachineSnapshot | null>(() => {
		const snapshots = this.timeMachineSnapshots;
		if (snapshots.length === 0) return null;
		const idx = Math.min(
			Math.round((this.timeMachineDay / this.timeMachineMaxDays) * (snapshots.length - 1)),
			snapshots.length - 1
		);
		return snapshots[Math.max(0, idx)];
	});

	// ── Phase 2: Sensitivity Data ───────────────────────
	readonly sensitivityData = $derived<SensitivityData[]>(generateSensitivityData(this.inputs));

	// ── Phase 3: Market Data State ──────────────────────
	dataMode = $state<DataMode>('manual');
	activeTicker = $state('');
	/** Inputs last filled from live market data (a reactive set: `.has(key)` is tracked per key). */
	readonly liveOverrides = new SvelteSet<keyof BSInputs>();

	// ── Phase 4: Education, Export, UI State ───────────
	educationMode = $state(false);
	#toasts = $state.raw<ToastItem[]>([]);
	showCommandPalette = $state(false);
	showShortcutsHelp = $state(false);
	showSaveModal = $state(false);
	showSavedConfigs = $state(false);
	showExportPNG = $state(false);
	showShareLink = $state(false);
	showEmbedCode = $state(false);

	get toasts(): ToastItem[] {
		return this.#toasts;
	}

	// ── Methods ──────────────────────────────────────────
	updateInput = (key: keyof BSInputs, value: number): void => {
		this[key] = value;
	};

	resetInputs = (): void => {
		this.spotPrice = DEFAULT_INPUTS.spotPrice;
		this.strikePrice = DEFAULT_INPUTS.strikePrice;
		this.volatility = DEFAULT_INPUTS.volatility;
		this.timeToExpiry = DEFAULT_INPUTS.timeToExpiry;
		this.riskFreeRate = DEFAULT_INPUTS.riskFreeRate;
		this.dividendYield = DEFAULT_INPUTS.dividendYield;
	};

	toggleTheme = (): void => {
		this.#theme = this.#theme === 'dark' ? 'light' : 'dark';
	};

	addStrategyLeg = (leg: Omit<StrategyLeg, 'id'>): void => {
		this.#strategyLegs = [...this.#strategyLegs, { ...leg, id: crypto.randomUUID() }];
	};

	/** Replace the leg with the same `id` in place, so its row keeps its place, colour and focus. */
	updateStrategyLeg = (leg: StrategyLeg): void => {
		this.#strategyLegs = this.#strategyLegs.map((l) => (l.id === leg.id ? leg : l));
	};

	removeStrategyLeg = (id: string): void => {
		this.#strategyLegs = this.#strategyLegs.filter((l) => l.id !== id);
	};

	clearStrategy = (): void => {
		this.#strategyLegs = [];
	};

	/**
	 * Apply a shared setup (a share link, an embed URL, a saved configuration). Only the
	 * fields present are applied; strategy legs are replaced only when some are given.
	 */
	applyShareableState = (state: Partial<ShareableState>): void => {
		if (state.inputs) {
			this.spotPrice = state.inputs.spotPrice;
			this.strikePrice = state.inputs.strikePrice;
			this.volatility = state.inputs.volatility;
			this.timeToExpiry = state.inputs.timeToExpiry;
			this.riskFreeRate = state.inputs.riskFreeRate;
			this.dividendYield = state.inputs.dividendYield;
		}
		if (state.optionType) this.optionType = state.optionType;
		if (state.mode) this.calculatorMode = state.mode;
		if (state.activeTab) this.activeTab = state.activeTab;
		if (state.ticker) this.activeTicker = state.ticker;
		if (state.strategyLegs?.length) {
			this.#strategyLegs = state.strategyLegs.map((leg) => ({ ...leg, id: crypto.randomUUID() }));
		}
	};

	// ── Phase 2: Monte Carlo Methods ────────────────────
	runMC = async (): Promise<void> => {
		const run = ++this.#monteCarloRun;
		this.#isMonteCarloRunning = true;
		// Plain copies: `monteCarloConfig` is a deep $state proxy, which postMessage cannot clone.
		const request: MonteCarloWorkerRequest = {
			inputs: $state.snapshot(this.inputs),
			config: $state.snapshot(this.monteCarloConfig)
		};

		let result: MonteCarloResult;
		try {
			result = await simulateInWorker(request);
		} catch (err) {
			logger.warn('[MonteCarlo] Worker unavailable, simulating on the main thread:', err);
			// Yield a frame so the running state paints before the blocking simulation.
			await new Promise((resolve) => setTimeout(resolve, 10));
			result = runMonteCarlo(request.inputs, request.config);
		}

		if (run !== this.#monteCarloRun) return;
		this.#monteCarloResult = result;
		this.#isMonteCarloRunning = false;
	};

	runLightMC = (): void => {
		this.#monteCarloResult = runLightMonteCarlo(this.inputs, 200, 50);
	};

	// ── Phase 2: Scenario Methods ───────────────────────
	toggleScenario = (scenario: Scenario): void => {
		const active = this.#activeScenarios;
		this.#activeScenarios = active.some((s) => s.id === scenario.id)
			? active.filter((s) => s.id !== scenario.id)
			: [...active, scenario];
	};

	clearScenarios = (): void => {
		this.#activeScenarios = [];
	};

	// ── Phase 4: Toast Methods ───────────────────────
	addToast = (type: ToastType, message: string, duration?: number): string => {
		const id = crypto.randomUUID();
		this.#toasts = [...this.#toasts, { id, type, message, duration }];
		return id;
	};

	removeToast = (id: string): void => {
		this.#toasts = this.#toasts.filter((t) => t.id !== id);
	};

	// ── Phase 3: Market Data Methods ───────────────────
	applyMarketSnapshot = (snapshot: MarketSnapshot): void => {
		const { suggestedInputs } = snapshot;
		this.spotPrice = suggestedInputs.spotPrice;
		this.volatility = suggestedInputs.volatility;
		this.dividendYield = suggestedInputs.dividendYield;
		this.riskFreeRate = suggestedInputs.riskFreeRate;
		this.activeTicker = snapshot.quote.ticker;
		this.liveOverrides.clear();
		for (const key of LIVE_INPUT_KEYS) this.liveOverrides.add(key);
	};

	clearLiveOverride = (key: keyof BSInputs): void => {
		this.liveOverrides.delete(key);
	};

	isLivePopulated = (key: keyof BSInputs): boolean => {
		return this.dataMode === 'live' && this.liveOverrides.has(key);
	};
}

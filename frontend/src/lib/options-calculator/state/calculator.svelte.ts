// ============================================================
// CALCULATOR STATE — Svelte 5 Runes
// Single source of truth for all calculator state
// ============================================================

import { price, intrinsicValue, moneyness, breakeven } from '../engine/black-scholes.js';
import { allGreeks } from '../engine/greeks.js';
import { calculateProbabilities } from '../engine/probability.js';
import {
	calculateStrategyPayoff,
	findBreakevens,
	findMaxProfitLoss,
	generatePriceRange,
} from '../engine/strategies.js';
import { DEFAULT_INPUTS } from '../engine/constants.js';
import { runMonteCarlo, runLightMonteCarlo } from '../engine/monte-carlo.js';
import {
	SCENARIO_PRESETS,
	calculateScenarios,
	generateTimeMachineSnapshots,
	generateSensitivityData,
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
	TimeMachineSnapshot,
	SensitivityData,
	ToastItem,
	ToastType,
} from '../engine/types.js';
import type { DataMode, MarketSnapshot } from '../data/types.js';

/**
 * Create the calculator state store.
 * Call this once in the root Calculator component.
 */
export function createCalculatorState() {
	// ── Core Inputs ──────────────────────────────────────
	let spotPrice = $state(DEFAULT_INPUTS.spotPrice);
	let strikePrice = $state(DEFAULT_INPUTS.strikePrice);
	let volatility = $state(DEFAULT_INPUTS.volatility);
	let timeToExpiry = $state(DEFAULT_INPUTS.timeToExpiry);
	let riskFreeRate = $state(DEFAULT_INPUTS.riskFreeRate);
	let dividendYield = $state(DEFAULT_INPUTS.dividendYield);

	// ── UI State ─────────────────────────────────────────
	let optionType = $state<OptionType>('call');
	let theme = $state<ThemeMode>('dark');
	let activeTab = $state<ChartTab>('payoff');
	let heatmapGreek = $state<HeatmapGreek>('delta');
	let calculatorMode = $state<CalculatorMode>('single');
	let showAdvancedGreeks = $state(false);
	let showProbabilities = $state(true);
	let isInputPanelOpen = $state(true);

	// ── Strategy State ───────────────────────────────────
	let strategyLegs = $state<StrategyLeg[]>([]);

	// ── Derived: Assembled Inputs ────────────────────────
	let inputs = $derived<BSInputs>({
		spotPrice,
		strikePrice,
		volatility,
		timeToExpiry,
		riskFreeRate,
		dividendYield,
	});

	// ── Derived: Pricing ─────────────────────────────────
	let pricing = $derived<BSPricingResult>(price(inputs));

	let currentPrice = $derived(optionType === 'call' ? pricing.callPrice : pricing.putPrice);

	let currentIntrinsic = $derived(intrinsicValue(spotPrice, strikePrice, optionType));

	let currentExtrinsic = $derived(Math.max(currentPrice - currentIntrinsic, 0));

	let currentMoneyness = $derived(moneyness(spotPrice, strikePrice, optionType));

	let currentBreakeven = $derived(breakeven(strikePrice, currentPrice, optionType));

	// ── Derived: Greeks ──────────────────────────────────
	let callGreeks = $derived<AllGreeks>(allGreeks(inputs, 'call'));
	let putGreeks = $derived<AllGreeks>(allGreeks(inputs, 'put'));
	let currentGreeks = $derived<AllGreeks>(optionType === 'call' ? callGreeks : putGreeks);

	// ── Derived: Probabilities ───────────────────────────
	let probabilities = $derived<ProbabilityResult>(calculateProbabilities(inputs, optionType));

	// ── Derived: Payoff Diagram ──────────────────────────
	let payoffData = $derived.by<PayoffPoint[]>(() => {
		if (calculatorMode === 'single' || strategyLegs.length === 0) {
			const strikes = [strikePrice];
			const priceRange = generatePriceRange(spotPrice, strikes);
			const singleLeg: StrategyLeg = {
				id: 'single',
				type: optionType,
				strike: strikePrice,
				expiry: timeToExpiry,
				position: 1,
				quantity: 1,
				premium: -currentPrice,
			};
			return calculateStrategyPayoff([singleLeg], priceRange);
		} else {
			const strikes = strategyLegs.map((l) => l.strike);
			const priceRange = generatePriceRange(spotPrice, strikes);
			return calculateStrategyPayoff(strategyLegs, priceRange);
		}
	});

	// ── Derived: Strategy Metrics ────────────────────────
	let strategyMaxProfitLoss = $derived(findMaxProfitLoss(payoffData));
	let strategyBreakevens = $derived(findBreakevens(payoffData));

	// ── Phase 2: Monte Carlo State ──────────────────────
	let monteCarloConfig = $state<MonteCarloConfig>({ numPaths: 2000, numSteps: 100 });
	let monteCarloResult = $state<MonteCarloResult | null>(null);
	let isMonteCarloRunning = $state(false);

	// ── Phase 2: Scenario State ─────────────────────────
	let activeScenarios = $state<Scenario[]>([]);
	let scenarioResults = $derived<ScenarioResult[]>(
		activeScenarios.length > 0 ? calculateScenarios(inputs, activeScenarios, optionType) : [],
	);

	// ── Phase 2: Time Machine State ─────────────────────
	let timeMachineDay = $state(0);
	let timeMachineMaxDays = $derived(Math.max(1, Math.round(timeToExpiry * 365)));
	let timeMachineSnapshots = $derived<TimeMachineSnapshot[]>(
		generateTimeMachineSnapshots(inputs, optionType, 100),
	);
	let currentTimeMachineSnapshot = $derived.by<TimeMachineSnapshot | null>(() => {
		if (timeMachineSnapshots.length === 0) return null;
		const idx = Math.min(
			Math.round((timeMachineDay / timeMachineMaxDays) * (timeMachineSnapshots.length - 1)),
			timeMachineSnapshots.length - 1,
		);
		return timeMachineSnapshots[Math.max(0, idx)];
	});

	// ── Phase 2: Sensitivity Data ───────────────────────
	let sensitivityData = $derived<SensitivityData[]>(generateSensitivityData(inputs));

	// ── Phase 2: Scenario Panel Visibility ───────────────
	let isScenarioPanelOpen = $state(false);

	// ── Phase 3: Market Data State ──────────────────────
	let dataMode = $state<DataMode>('manual');
	let activeTicker = $state('');
	let liveOverrides = $state<Set<keyof BSInputs>>(new Set());

	// ── Phase 4: Education, Export, UI State ───────────
	let educationMode = $state(false);
	let toasts = $state<ToastItem[]>([]);
	let showCommandPalette = $state(false);
	let showShortcutsHelp = $state(false);
	let showSaveModal = $state(false);
	let showSavedConfigs = $state(false);
	let showExportPNG = $state(false);
	let showShareLink = $state(false);
	let showEmbedCode = $state(false);

	// ── Methods ──────────────────────────────────────────
	function updateInput(key: keyof BSInputs, value: number) {
		switch (key) {
			case 'spotPrice':
				spotPrice = value;
				break;
			case 'strikePrice':
				strikePrice = value;
				break;
			case 'volatility':
				volatility = value;
				break;
			case 'timeToExpiry':
				timeToExpiry = value;
				break;
			case 'riskFreeRate':
				riskFreeRate = value;
				break;
			case 'dividendYield':
				dividendYield = value;
				break;
		}
	}

	function resetInputs() {
		spotPrice = DEFAULT_INPUTS.spotPrice;
		strikePrice = DEFAULT_INPUTS.strikePrice;
		volatility = DEFAULT_INPUTS.volatility;
		timeToExpiry = DEFAULT_INPUTS.timeToExpiry;
		riskFreeRate = DEFAULT_INPUTS.riskFreeRate;
		dividendYield = DEFAULT_INPUTS.dividendYield;
	}

	function toggleTheme() {
		theme = theme === 'dark' ? 'light' : 'dark';
	}

	function addStrategyLeg(leg: Omit<StrategyLeg, 'id'>) {
		const id = crypto.randomUUID();
		strategyLegs = [...strategyLegs, { ...leg, id }];
	}

	function removeStrategyLeg(id: string) {
		strategyLegs = strategyLegs.filter((l) => l.id !== id);
	}

	function clearStrategy() {
		strategyLegs = [];
	}

	// ── Phase 2: Monte Carlo Methods ────────────────────
	async function runMC() {
		isMonteCarloRunning = true;
		await new Promise((resolve) => setTimeout(resolve, 10));
		monteCarloResult = runMonteCarlo(inputs, monteCarloConfig);
		isMonteCarloRunning = false;
	}

	function runLightMC() {
		monteCarloResult = runLightMonteCarlo(inputs, 200, 50);
	}

	// ── Phase 2: Scenario Methods ───────────────────────
	function toggleScenario(scenario: Scenario) {
		const exists = activeScenarios.find((s) => s.id === scenario.id);
		if (exists) {
			activeScenarios = activeScenarios.filter((s) => s.id !== scenario.id);
		} else {
			activeScenarios = [...activeScenarios, scenario];
		}
	}

	function clearScenarios() {
		activeScenarios = [];
	}

	// ── Phase 4: Toast Methods ───────────────────────
	function addToast(type: ToastType, message: string, duration?: number): string {
		const id = crypto.randomUUID();
		toasts = [...toasts, { id, type, message, duration }];
		return id;
	}

	function removeToast(id: string): void {
		toasts = toasts.filter((t) => t.id !== id);
	}

	// ── Phase 3: Market Data Methods ───────────────────
	function applyMarketSnapshot(snapshot: MarketSnapshot) {
		spotPrice = snapshot.suggestedInputs.spotPrice;
		volatility = snapshot.suggestedInputs.volatility;
		dividendYield = snapshot.suggestedInputs.dividendYield;
		riskFreeRate = snapshot.suggestedInputs.riskFreeRate;
		activeTicker = snapshot.quote.ticker;
		liveOverrides = new Set(['spotPrice', 'volatility', 'dividendYield', 'riskFreeRate']);
	}

	function clearLiveOverride(key: keyof BSInputs) {
		liveOverrides = new Set([...liveOverrides].filter((k) => k !== key));
	}

	function isLivePopulated(key: keyof BSInputs): boolean {
		return dataMode === 'live' && liveOverrides.has(key);
	}

	// ── Return Public API ────────────────────────────────
	return {
		get spotPrice() {
			return spotPrice;
		},
		set spotPrice(v: number) {
			spotPrice = v;
		},
		get strikePrice() {
			return strikePrice;
		},
		set strikePrice(v: number) {
			strikePrice = v;
		},
		get volatility() {
			return volatility;
		},
		set volatility(v: number) {
			volatility = v;
		},
		get timeToExpiry() {
			return timeToExpiry;
		},
		set timeToExpiry(v: number) {
			timeToExpiry = v;
		},
		get riskFreeRate() {
			return riskFreeRate;
		},
		set riskFreeRate(v: number) {
			riskFreeRate = v;
		},
		get dividendYield() {
			return dividendYield;
		},
		set dividendYield(v: number) {
			dividendYield = v;
		},
		get optionType() {
			return optionType;
		},
		set optionType(v: OptionType) {
			optionType = v;
		},
		get theme() {
			return theme;
		},
		get activeTab() {
			return activeTab;
		},
		set activeTab(v: ChartTab) {
			activeTab = v;
		},
		get heatmapGreek() {
			return heatmapGreek;
		},
		set heatmapGreek(v: HeatmapGreek) {
			heatmapGreek = v;
		},
		get calculatorMode() {
			return calculatorMode;
		},
		set calculatorMode(v: CalculatorMode) {
			calculatorMode = v;
		},
		get showAdvancedGreeks() {
			return showAdvancedGreeks;
		},
		set showAdvancedGreeks(v: boolean) {
			showAdvancedGreeks = v;
		},
		get showProbabilities() {
			return showProbabilities;
		},
		set showProbabilities(v: boolean) {
			showProbabilities = v;
		},
		get isInputPanelOpen() {
			return isInputPanelOpen;
		},
		set isInputPanelOpen(v: boolean) {
			isInputPanelOpen = v;
		},

		get inputs() {
			return inputs;
		},
		get pricing() {
			return pricing;
		},
		get currentPrice() {
			return currentPrice;
		},
		get currentIntrinsic() {
			return currentIntrinsic;
		},
		get currentExtrinsic() {
			return currentExtrinsic;
		},
		get currentMoneyness() {
			return currentMoneyness;
		},
		get currentBreakeven() {
			return currentBreakeven;
		},
		get callGreeks() {
			return callGreeks;
		},
		get putGreeks() {
			return putGreeks;
		},
		get currentGreeks() {
			return currentGreeks;
		},
		get probabilities() {
			return probabilities;
		},
		get payoffData() {
			return payoffData;
		},
		get strategyMaxProfitLoss() {
			return strategyMaxProfitLoss;
		},
		get strategyBreakevens() {
			return strategyBreakevens;
		},
		get strategyLegs() {
			return strategyLegs;
		},

		// Phase 2: Monte Carlo
		get monteCarloConfig() { return monteCarloConfig; },
		set monteCarloConfig(v: MonteCarloConfig) { monteCarloConfig = v; },
		get monteCarloResult() { return monteCarloResult; },
		get isMonteCarloRunning() { return isMonteCarloRunning; },
		runMC,
		runLightMC,

		// Phase 2: Scenarios
		get activeScenarios() { return activeScenarios; },
		get scenarioResults() { return scenarioResults; },
		get scenarioPresets() { return SCENARIO_PRESETS; },
		toggleScenario,
		clearScenarios,
		get isScenarioPanelOpen() { return isScenarioPanelOpen; },
		set isScenarioPanelOpen(v: boolean) { isScenarioPanelOpen = v; },

		// Phase 2: Time Machine
		get timeMachineDay() { return timeMachineDay; },
		set timeMachineDay(v: number) { timeMachineDay = v; },
		get timeMachineMaxDays() { return timeMachineMaxDays; },
		get timeMachineSnapshots() { return timeMachineSnapshots; },
		get currentTimeMachineSnapshot() { return currentTimeMachineSnapshot; },

		// Phase 2: Sensitivity
		get sensitivityData() { return sensitivityData; },

		// Phase 3: Market Data
		get dataMode() { return dataMode; },
		set dataMode(v: DataMode) { dataMode = v; },
		get activeTicker() { return activeTicker; },
		set activeTicker(v: string) { activeTicker = v; },
		get liveOverrides() { return liveOverrides; },
		applyMarketSnapshot,
		clearLiveOverride,
		isLivePopulated,

		// Phase 4: Education, Export, UI
		get educationMode() { return educationMode; },
		set educationMode(v: boolean) { educationMode = v; },
		get toasts() { return toasts; },
		addToast,
		removeToast,
		get showCommandPalette() { return showCommandPalette; },
		set showCommandPalette(v: boolean) { showCommandPalette = v; },
		get showShortcutsHelp() { return showShortcutsHelp; },
		set showShortcutsHelp(v: boolean) { showShortcutsHelp = v; },
		get showSaveModal() { return showSaveModal; },
		set showSaveModal(v: boolean) { showSaveModal = v; },
		get showSavedConfigs() { return showSavedConfigs; },
		set showSavedConfigs(v: boolean) { showSavedConfigs = v; },
		get showExportPNG() { return showExportPNG; },
		set showExportPNG(v: boolean) { showExportPNG = v; },
		get showShareLink() { return showShareLink; },
		set showShareLink(v: boolean) { showShareLink = v; },
		get showEmbedCode() { return showEmbedCode; },
		set showEmbedCode(v: boolean) { showEmbedCode = v; },

		updateInput,
		resetInputs,
		toggleTheme,
		addStrategyLeg,
		removeStrategyLeg,
		clearStrategy,
	};
}

/** Type for the calculator state */
export type CalculatorState = ReturnType<typeof createCalculatorState>;

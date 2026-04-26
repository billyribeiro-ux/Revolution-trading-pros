// ============================================================
// BLACK-SCHOLES OPTIONS CALCULATOR — TYPE DEFINITIONS
// Revolution Trading Pros © 2026
// ============================================================

/** Option type: Call or Put */
export type OptionType = 'call' | 'put';

/** Option style */
export type OptionStyle = 'european' | 'american';

/** Core input parameters for Black-Scholes pricing */
export interface BSInputs {
	/** Current underlying price (S) */
	spotPrice: number;
	/** Option strike price (K) */
	strikePrice: number;
	/** Annual implied volatility as decimal (σ), e.g., 0.25 = 25% */
	volatility: number;
	/** Time to expiration in years (T), e.g., 0.25 = 3 months */
	timeToExpiry: number;
	/** Annual risk-free interest rate as decimal (r), e.g., 0.05 = 5% */
	riskFreeRate: number;
	/** Annual continuous dividend yield as decimal (q), e.g., 0.02 = 2% */
	dividendYield: number;
}

/** Black-Scholes pricing result */
export interface BSPricingResult {
	callPrice: number;
	putPrice: number;
	d1: number;
	d2: number;
	/** Put-call parity check value (should be ~0) */
	parityCheck: number;
}

/** First-order Greeks */
export interface FirstOrderGreeks {
	/** Rate of change of option price w.r.t. underlying price (∂V/∂S) */
	delta: number;
	/** Rate of change of delta w.r.t. underlying price (∂²V/∂S²) */
	gamma: number;
	/** Rate of change of option price w.r.t. time (∂V/∂t) — per day */
	theta: number;
	/** Rate of change of option price w.r.t. volatility (∂V/∂σ) */
	vega: number;
	/** Rate of change of option price w.r.t. interest rate (∂V/∂r) */
	rho: number;
}

/** Second-order Greeks */
export interface SecondOrderGreeks {
	/** Rate of change of delta w.r.t. time (∂Δ/∂t) — aka delta decay */
	charm: number;
	/** Rate of change of delta w.r.t. volatility (∂Δ/∂σ) — aka DdeltaDvol */
	vanna: number;
	/** Rate of change of vega w.r.t. volatility (∂V²/∂σ²) — aka vega convexity */
	volga: number;
	/** Rate of change of vega w.r.t. underlying (∂vega/∂S) — same as vanna */
	veta: number;
	/** Rate of change of gamma w.r.t. underlying (∂Γ/∂S) — aka DgammaDspot */
	speed: number;
	/** Rate of change of gamma w.r.t. time (∂Γ/∂t) */
	color: number;
	/** Rate of change of gamma w.r.t. volatility (∂Γ/∂σ) */
	zomma: number;
}

/** Complete Greeks for an option */
export interface AllGreeks {
	first: FirstOrderGreeks;
	second: SecondOrderGreeks;
}

/** Full calculation result for a single option */
export interface OptionResult {
	inputs: BSInputs;
	type: OptionType;
	price: number;
	intrinsicValue: number;
	extrinsicValue: number;
	greeks: AllGreeks;
	/** Moneyness: ITM, ATM, OTM */
	moneyness: 'ITM' | 'ATM' | 'OTM';
	/** Break-even price at expiration */
	breakeven: number;
}

/** Strategy leg definition */
export interface StrategyLeg {
	id: string;
	type: OptionType;
	strike: number;
	expiry: number;
	/** Position: long (+1) or short (-1) */
	position: 1 | -1;
	/** Number of contracts */
	quantity: number;
	/** Premium paid/received per contract */
	premium: number;
}

/** Pre-defined strategy template */
export interface StrategyTemplate {
	id: string;
	name: string;
	description: string;
	legs: Omit<StrategyLeg, 'id' | 'premium'>[];
	/** Sentiment: bullish, bearish, neutral */
	sentiment: 'bullish' | 'bearish' | 'neutral';
	/** Risk profile */
	riskProfile: 'defined' | 'undefined';
}

/** Strategy analysis result */
export interface StrategyResult {
	legs: OptionResult[];
	maxProfit: number | 'unlimited';
	maxLoss: number | 'unlimited';
	breakevens: number[];
	netPremium: number;
	netDelta: number;
	netGamma: number;
	netTheta: number;
	netVega: number;
	probabilityOfProfit: number;
	riskRewardRatio: number;
}

/** Payoff diagram data point */
export interface PayoffPoint {
	underlyingPrice: number;
	payoff: number;
	profit: number;
}

/** Greeks heatmap cell */
export interface HeatmapCell {
	strike: number;
	expiry: number;
	value: number;
}

/** Probability calculation results */
export interface ProbabilityResult {
	/** Probability option expires ITM */
	probabilityITM: number;
	/** Probability option expires OTM */
	probabilityOTM: number;
	/** Probability underlying touches strike before expiry */
	probabilityOfTouching: number;
	/** Expected move (1 standard deviation) by expiration */
	expectedMove: number;
	/** Expected move as percentage */
	expectedMovePercent: number;
	/** 1 SD range */
	oneSDRange: [number, number];
	/** 2 SD range */
	twoSDRange: [number, number];
}

/** Implied volatility solver result */
export interface IVSolverResult {
	impliedVolatility: number;
	iterations: number;
	converged: boolean;
	error: number;
}

/** Theme mode */
export type ThemeMode = 'dark' | 'light';

/** Chart tab options */
export type ChartTab =
	| 'payoff'
	| 'heatmap'
	| 'surface'
	| 'montecarlo'
	| 'volsmile'
	| 'theta'
	| 'sensitivity'
	| 'chain';

/** Greek to display in heatmap */
export type HeatmapGreek =
	| 'delta'
	| 'gamma'
	| 'theta'
	| 'vega'
	| 'rho'
	| 'charm'
	| 'vanna'
	| 'volga';

/** Calculator mode */
export type CalculatorMode = 'single' | 'strategy' | 'compare';

/** Input field configuration */
export interface InputFieldConfig {
	key: keyof BSInputs;
	label: string;
	min: number;
	max: number;
	step: number;
	unit: string;
	tooltip: string;
	icon: string;
	/** Format for display (e.g., multiply vol by 100 for percentage) */
	displayMultiplier?: number;
	displayDecimals?: number;
}

// ============================================================
// PHASE 2 TYPE ADDITIONS
// ============================================================

/** Monte Carlo simulation configuration */
export interface MonteCarloConfig {
	numPaths: number;
	numSteps: number;
	seed?: number;
}

/** Single Monte Carlo path */
export interface MonteCarloPath {
	id: number;
	prices: number[];
	finalPrice: number;
	maxPrice: number;
	minPrice: number;
}

/** Monte Carlo simulation results */
export interface MonteCarloResult {
	paths: MonteCarloPath[];
	timePoints: number[];
	stats: {
		meanFinalPrice: number;
		medianFinalPrice: number;
		stdDev: number;
		percentile5: number;
		percentile25: number;
		percentile75: number;
		percentile95: number;
		probabilityAboveStrike: number;
		probabilityBelowStrike: number;
		expectedPayoffCall: number;
		expectedPayoffPut: number;
		bsCallPrice: number;
		bsPutPrice: number;
	};
	computeTimeMs: number;
}

/** Scenario definition */
export interface Scenario {
	id: string;
	name: string;
	description: string;
	adjustments: {
		spotPriceChange?: number;
		spotPriceChangePct?: number;
		volatilityChange?: number;
		timeChange?: number;
		rateChange?: number;
	};
	color: string;
}

/** Scenario result */
export interface ScenarioResult {
	scenario: Scenario;
	inputs: BSInputs;
	callPrice: number;
	putPrice: number;
	callPriceChange: number;
	putPriceChange: number;
	callPriceChangePct: number;
	putPriceChangePct: number;
	greeks: AllGreeks;
}

/** Time Machine snapshot */
export interface TimeMachineSnapshot {
	daysFromNow: number;
	timeToExpiry: number;
	callPrice: number;
	putPrice: number;
	greeks: AllGreeks;
	probabilities: ProbabilityResult;
}

/** Volatility smile data point */
export interface VolSmilePoint {
	strike: number;
	impliedVol: number;
	moneyness: number;
	strikeRatio: number;
}

/** Sensitivity analysis data */
export interface SensitivityData {
	parameter: string;
	label: string;
	baseValue: number;
	range: number[];
	callPrices: number[];
	putPrices: number[];
	sensitivity: number;
}

/** Strategy leg with full computed data for UI */
export interface StrategyLegUI extends StrategyLeg {
	price: number;
	greeks: FirstOrderGreeks;
	color: string;
}

// ============================================================
// PHASE 4 TYPE ADDITIONS
// Education, Export, Saved Configs, Power User, Growth
// ============================================================

/** Toast notification severity */
export type ToastType = 'success' | 'error' | 'info' | 'warning';

/** A single toast notification */
export interface ToastItem {
	id: string;
	type: ToastType;
	message: string;
	/** Auto-dismiss duration in ms. Default 3000. */
	duration?: number;
}

/** A saved calculator configuration persisted to localStorage */
export interface SavedConfig {
	id: string;
	name: string;
	description?: string;
	createdAt: string;
	updatedAt: string;
	inputs: BSInputs;
	optionType: OptionType;
	mode: CalculatorMode;
	ticker?: string;
	strategyLegs?: StrategyLeg[];
	tags?: string[];
}

/** Parameters that can be encoded into a shareable URL */
export interface ShareableState {
	inputs: BSInputs;
	optionType: OptionType;
	mode: CalculatorMode;
	activeTab: ChartTab;
	ticker?: string;
	strategyLegs?: StrategyLeg[];
}

/** What area of the calculator to capture in a screenshot */
export type CaptureZone = 'results-only' | 'results-chart' | 'chart-only' | 'full-calculator';

/** Social-ready aspect ratio presets for screenshots */
export type AspectRatio = 'auto' | '16:9' | '4:5' | '1:1';

/** Configuration for branded screenshot export */
export interface ScreenshotConfig {
	zone: CaptureZone;
	aspectRatio: AspectRatio;
	filename?: string;
	/** Render scale factor (2 = retina, 3 = ultra-crisp) */
	scale?: number;
	backgroundColor?: string;
	showLogo: boolean;
	showInfoBar: boolean;
	showFrame: boolean;
	ticker?: string;
	/** Summary text for info bar, e.g. "Call $5.45 | Δ 0.57 | Θ -0.04" */
	summaryText?: string;
	theme?: ThemeMode;
}

/** A single educational content entry */
export interface EducationEntry {
	id: string;
	term: string;
	symbol?: string;
	shortDescription: string;
	/** Full explanation — supports markdown */
	fullExplanation: string;
	/** Practical trading insight */
	proTip: string;
	/** What happens when this value increases? */
	whenIncreases?: string;
	/** What happens when this value decreases? */
	whenDecreases?: string;
	/** Related concept IDs */
	relatedTerms?: string[];
}

/** A keyboard shortcut definition */
export interface ShortcutDef {
	id: string;
	/** Key combos that trigger this shortcut, e.g. ['ctrl+k', 'meta+k'] */
	keys: string[];
	/** Human-readable display string, e.g. "⌘K" */
	display: string;
	description: string;
	category: 'navigation' | 'input' | 'export' | 'view' | 'general';
}

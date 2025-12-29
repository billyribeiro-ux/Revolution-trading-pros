/**
 * A/B Test Statistical Significance Calculator
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Enterprise-grade statistical analysis for A/B testing:
 * - Z-test for conversion rate comparison
 * - Bayesian probability estimation
 * - Sample size calculator
 * - Minimum Detectable Effect (MDE)
 * - Confidence interval calculation
 * - Statistical power analysis
 * - Auto-winner detection
 *
 * Aligned with Google/Microsoft/Netflix standards.
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 * @license MIT
 */

// ═══════════════════════════════════════════════════════════════════════════
// Types
// ═══════════════════════════════════════════════════════════════════════════

export interface VariantData {
	name: string;
	visitors: number;
	conversions: number;
	conversionRate?: number;
}

export interface ABTestResult {
	isSignificant: boolean;
	confidence: number;
	pValue: number;
	zScore: number;
	winner: string | null;
	uplift: number;
	upliftPercentage: number;
	confidenceInterval: {
		lower: number;
		upper: number;
	};
	requiredSampleSize: number;
	currentPower: number;
	bayesianProbability: {
		controlWins: number;
		treatmentWins: number;
	};
	recommendation: string;
}

export interface SampleSizeParams {
	baselineConversionRate: number;
	minimumDetectableEffect: number;
	statisticalPower?: number;
	significanceLevel?: number;
}

// ═══════════════════════════════════════════════════════════════════════════
// Statistical Functions
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Standard normal cumulative distribution function.
 */
function normalCDF(x: number): number {
	const a1 = 0.254829592;
	const a2 = -0.284496736;
	const a3 = 1.421413741;
	const a4 = -1.453152027;
	const a5 = 1.061405429;
	const p = 0.3275911;

	const sign = x < 0 ? -1 : 1;
	x = Math.abs(x) / Math.sqrt(2);

	const t = 1.0 / (1.0 + p * x);
	const y = 1.0 - ((((a5 * t + a4) * t + a3) * t + a2) * t + a1) * t * Math.exp(-x * x);

	return 0.5 * (1.0 + sign * y);
}

/**
 * Inverse standard normal (quantile function).
 */
function normalQuantile(p: number): number {
	if (p <= 0 || p >= 1) {
		throw new Error('p must be between 0 and 1');
	}

	// Rational approximation for lower region
	const a = [
		-3.969683028665376e1, 2.209460984245205e2,
		-2.759285104469687e2, 1.383577518672690e2,
		-3.066479806614716e1, 2.506628277459239e0,
	];
	const b = [
		-5.447609879822406e1, 1.615858368580409e2,
		-1.556989798598866e2, 6.680131188771972e1,
		-1.328068155288572e1,
	];
	const c = [
		-7.784894002430293e-3, -3.223964580411365e-1,
		-2.400758277161838e0, -2.549732539343734e0,
		4.374664141464968e0, 2.938163982698783e0,
	];
	const d = [
		7.784695709041462e-3, 3.224671290700398e-1,
		2.445134137142996e0, 3.754408661907416e0,
	];

	const pLow = 0.02425;
	const pHigh = 1 - pLow;

	let q: number, r: number;

	if (p < pLow) {
		q = Math.sqrt(-2 * Math.log(p));
		const c0 = c[0] ?? 0, c1 = c[1] ?? 0, c2 = c[2] ?? 0, c3 = c[3] ?? 0, c4 = c[4] ?? 0, c5 = c[5] ?? 0;
	const d0 = d[0] ?? 0, d1 = d[1] ?? 0, d2 = d[2] ?? 0, d3 = d[3] ?? 0;
	return (((((c0 * q + c1) * q + c2) * q + c3) * q + c4) * q + c5) /
			((((d0 * q + d1) * q + d2) * q + d3) * q + 1);
	} else if (p <= pHigh) {
		q = p - 0.5;
		r = q * q;
		const a0 = a[0] ?? 0, a1 = a[1] ?? 0, a2 = a[2] ?? 0, a3 = a[3] ?? 0, a4 = a[4] ?? 0, a5 = a[5] ?? 0;
	const b0 = b[0] ?? 0, b1 = b[1] ?? 0, b2 = b[2] ?? 0, b3 = b[3] ?? 0, b4 = b[4] ?? 0;
	return (((((a0 * r + a1) * r + a2) * r + a3) * r + a4) * r + a5) * q /
			(((((b0 * r + b1) * r + b2) * r + b3) * r + b4) * r + 1);
	} else {
		q = Math.sqrt(-2 * Math.log(1 - p));
		const c0 = c[0] ?? 0, c1 = c[1] ?? 0, c2 = c[2] ?? 0, c3 = c[3] ?? 0, c4 = c[4] ?? 0, c5 = c[5] ?? 0;
		const d0 = d[0] ?? 0, d1 = d[1] ?? 0, d2 = d[2] ?? 0, d3 = d[3] ?? 0;
		return -(((((c0 * q + c1) * q + c2) * q + c3) * q + c4) * q + c5) /
			((((d0 * q + d1) * q + d2) * q + d3) * q + 1);
	}
}

// ═══════════════════════════════════════════════════════════════════════════
// A/B Test Analysis Functions
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Calculate Z-score for two proportions test.
 */
export function calculateZScore(control: VariantData, treatment: VariantData): number {
	const p1 = control.conversions / control.visitors;
	const p2 = treatment.conversions / treatment.visitors;
	const n1 = control.visitors;
	const n2 = treatment.visitors;

	// Pooled proportion
	const pPooled = (control.conversions + treatment.conversions) / (n1 + n2);

	// Standard error
	const se = Math.sqrt(pPooled * (1 - pPooled) * (1 / n1 + 1 / n2));

	if (se === 0) return 0;

	return (p2 - p1) / se;
}

/**
 * Calculate p-value from Z-score (two-tailed test).
 */
export function calculatePValue(zScore: number): number {
	return 2 * (1 - normalCDF(Math.abs(zScore)));
}

/**
 * Calculate confidence interval for the difference in proportions.
 */
export function calculateConfidenceInterval(
	control: VariantData,
	treatment: VariantData,
	confidenceLevel: number = 0.95
): { lower: number; upper: number } {
	const p1 = control.conversions / control.visitors;
	const p2 = treatment.conversions / treatment.visitors;
	const n1 = control.visitors;
	const n2 = treatment.visitors;

	const diff = p2 - p1;
	const se = Math.sqrt((p1 * (1 - p1)) / n1 + (p2 * (1 - p2)) / n2);

	const zCritical = normalQuantile(1 - (1 - confidenceLevel) / 2);

	return {
		lower: diff - zCritical * se,
		upper: diff + zCritical * se,
	};
}

/**
 * Calculate required sample size for desired statistical power.
 */
export function calculateRequiredSampleSize(params: SampleSizeParams): number {
	const {
		baselineConversionRate,
		minimumDetectableEffect,
		statisticalPower = 0.8,
		significanceLevel = 0.05,
	} = params;

	const p1 = baselineConversionRate;
	const p2 = baselineConversionRate * (1 + minimumDetectableEffect);
	const pBar = (p1 + p2) / 2;

	const zAlpha = normalQuantile(1 - significanceLevel / 2);
	const zBeta = normalQuantile(statisticalPower);

	const numerator = 2 * pBar * (1 - pBar) * Math.pow(zAlpha + zBeta, 2);
	const denominator = Math.pow(p2 - p1, 2);

	return Math.ceil(numerator / denominator);
}

/**
 * Calculate current statistical power.
 */
export function calculateStatisticalPower(
	control: VariantData,
	treatment: VariantData,
	significanceLevel: number = 0.05
): number {
	const p1 = control.conversions / control.visitors;
	const p2 = treatment.conversions / treatment.visitors;
	const n1 = control.visitors;
	const n2 = treatment.visitors;

	const se = Math.sqrt((p1 * (1 - p1)) / n1 + (p2 * (1 - p2)) / n2);

	if (se === 0) return 0;

	const zCritical = normalQuantile(1 - significanceLevel / 2);
	const effectSize = Math.abs(p2 - p1) / se;

	return normalCDF(effectSize - zCritical);
}

/**
 * Calculate Bayesian probability of each variant winning.
 */
export function calculateBayesianProbability(
	control: VariantData,
	treatment: VariantData,
	simulations: number = 10000
): { controlWins: number; treatmentWins: number } {
	let controlWins = 0;
	let treatmentWins = 0;

	// Beta distribution parameters
	const alphaControl = control.conversions + 1;
	const betaControl = control.visitors - control.conversions + 1;
	const alphaTreatment = treatment.conversions + 1;
	const betaTreatment = treatment.visitors - treatment.conversions + 1;

	// Monte Carlo simulation
	for (let i = 0; i < simulations; i++) {
		const sampleControl = sampleBeta(alphaControl, betaControl);
		const sampleTreatment = sampleBeta(alphaTreatment, betaTreatment);

		if (sampleTreatment > sampleControl) {
			treatmentWins++;
		} else {
			controlWins++;
		}
	}

	return {
		controlWins: controlWins / simulations,
		treatmentWins: treatmentWins / simulations,
	};
}

/**
 * Sample from a Beta distribution using inverse transform.
 */
function sampleBeta(alpha: number, beta: number): number {
	// Use gamma sampling method
	const x = sampleGamma(alpha);
	const y = sampleGamma(beta);
	return x / (x + y);
}

/**
 * Sample from a Gamma distribution using Marsaglia and Tsang's method.
 */
function sampleGamma(shape: number): number {
	if (shape < 1) {
		return sampleGamma(1 + shape) * Math.pow(Math.random(), 1 / shape);
	}

	const d = shape - 1 / 3;
	const c = 1 / Math.sqrt(9 * d);

	while (true) {
		let x: number, v: number;
		do {
			x = normalRandom();
			v = 1 + c * x;
		} while (v <= 0);

		v = v * v * v;
		const u = Math.random();

		if (u < 1 - 0.0331 * (x * x) * (x * x)) return d * v;
		if (Math.log(u) < 0.5 * x * x + d * (1 - v + Math.log(v))) return d * v;
	}
}

/**
 * Generate standard normal random number (Box-Muller).
 */
function normalRandom(): number {
	const u1 = Math.random();
	const u2 = Math.random();
	return Math.sqrt(-2 * Math.log(u1)) * Math.cos(2 * Math.PI * u2);
}

// ═══════════════════════════════════════════════════════════════════════════
// Main Analysis Function
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Perform comprehensive A/B test analysis.
 */
export function analyzeABTest(
	control: VariantData,
	treatment: VariantData,
	options: {
		significanceLevel?: number;
		minimumDetectableEffect?: number;
		targetPower?: number;
	} = {}
): ABTestResult {
	const {
		significanceLevel = 0.05,
		minimumDetectableEffect = 0.1,
		targetPower = 0.8,
	} = options;

	// Calculate conversion rates
	const controlRate = control.visitors > 0 ? control.conversions / control.visitors : 0;
	const treatmentRate = treatment.visitors > 0 ? treatment.conversions / treatment.visitors : 0;

	// Z-test
	const zScore = calculateZScore(control, treatment);
	const pValue = calculatePValue(zScore);
	const isSignificant = pValue < significanceLevel;

	// Confidence interval
	const confidenceInterval = calculateConfidenceInterval(
		control,
		treatment,
		1 - significanceLevel
	);

	// Uplift
	const uplift = treatmentRate - controlRate;
	const upliftPercentage = controlRate > 0 ? (uplift / controlRate) * 100 : 0;

	// Statistical power
	const currentPower = calculateStatisticalPower(control, treatment, significanceLevel);

	// Required sample size
	const requiredSampleSize = calculateRequiredSampleSize({
		baselineConversionRate: controlRate || 0.05,
		minimumDetectableEffect,
		statisticalPower: targetPower,
		significanceLevel,
	});

	// Bayesian probability
	const bayesianProbability = calculateBayesianProbability(control, treatment);

	// Determine winner
	let winner: string | null = null;
	if (isSignificant) {
		winner = treatmentRate > controlRate ? treatment.name : control.name;
	}

	// Generate recommendation
	let recommendation: string;
	if (isSignificant && currentPower >= targetPower) {
		if (winner === treatment.name) {
			recommendation = `IMPLEMENT: ${treatment.name} is the winner with ${(1 - significanceLevel) * 100}% confidence. Implement this variant.`;
		} else {
			recommendation = `KEEP CONTROL: ${control.name} performs better. Keep the current implementation.`;
		}
	} else if (control.visitors + treatment.visitors < requiredSampleSize) {
		const remaining = requiredSampleSize - (control.visitors + treatment.visitors);
		recommendation = `CONTINUE TEST: Need approximately ${remaining.toLocaleString()} more visitors for statistical significance.`;
	} else if (currentPower < targetPower) {
		recommendation = `LOW POWER: Test has ${(currentPower * 100).toFixed(1)}% power. Consider increasing sample size or effect size.`;
	} else {
		recommendation = `NO DIFFERENCE: No statistically significant difference detected between variants.`;
	}

	return {
		isSignificant,
		confidence: (1 - pValue) * 100,
		pValue,
		zScore,
		winner,
		uplift,
		upliftPercentage,
		confidenceInterval,
		requiredSampleSize,
		currentPower,
		bayesianProbability,
		recommendation,
	};
}

/**
 * Check if a test has reached statistical significance.
 */
export function hasReachedSignificance(
	control: VariantData,
	treatment: VariantData,
	significanceLevel: number = 0.05
): boolean {
	const zScore = calculateZScore(control, treatment);
	const pValue = calculatePValue(zScore);
	return pValue < significanceLevel;
}

/**
 * Estimate time remaining for test completion.
 */
export function estimateTimeRemaining(
	control: VariantData,
	treatment: VariantData,
	dailyVisitors: number,
	options: {
		minimumDetectableEffect?: number;
		targetPower?: number;
	} = {}
): { daysRemaining: number; percentComplete: number } {
	const { minimumDetectableEffect = 0.1, targetPower = 0.8 } = options;

	const controlRate = control.visitors > 0 ? control.conversions / control.visitors : 0.05;

	const requiredSampleSize = calculateRequiredSampleSize({
		baselineConversionRate: controlRate,
		minimumDetectableEffect,
		statisticalPower: targetPower,
	});

	const currentTotal = control.visitors + treatment.visitors;
	const remaining = Math.max(0, requiredSampleSize * 2 - currentTotal);

	return {
		daysRemaining: dailyVisitors > 0 ? Math.ceil(remaining / dailyVisitors) : Infinity,
		percentComplete: Math.min(100, (currentTotal / (requiredSampleSize * 2)) * 100),
	};
}

export default analyzeABTest;

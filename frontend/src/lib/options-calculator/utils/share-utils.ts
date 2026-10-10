// ============================================================
// SHARE UTILITIES — URL-based State Serialization
// Encode calculator state into a shareable URL.
// Decode URL params back into calculator state.
// ============================================================

import type {
	BSInputs,
	CalculatorMode,
	ChartTab,
	ShareableState,
	StrategyLeg
} from '../engine/types.js';
import { DEFAULT_INPUTS } from '../engine/constants.js';

/** Short URL keys for each pricing input, in the order they are written. */
const INPUT_PARAMS = {
	spotPrice: 's',
	strikePrice: 'k',
	volatility: 'v',
	timeToExpiry: 't',
	riskFreeRate: 'r',
	dividendYield: 'q'
} as const satisfies Record<keyof BSInputs, string>;

const INPUT_KEYS = Object.keys(INPUT_PARAMS) as (keyof BSInputs)[];

/** Every chart tab, so a shared `tab` value can be checked (`Record` keeps it exhaustive). */
const CHART_TABS: Record<ChartTab, true> = {
	payoff: true,
	heatmap: true,
	surface: true,
	montecarlo: true,
	volsmile: true,
	theta: true,
	sensitivity: true,
	chain: true
};

const CALCULATOR_MODES: Record<CalculatorMode, true> = {
	single: true,
	strategy: true,
	compare: true
};

function isChartTab(value: string): value is ChartTab {
	return Object.hasOwn(CHART_TABS, value);
}

function isCalculatorMode(value: string): value is CalculatorMode {
	return Object.hasOwn(CALCULATOR_MODES, value);
}

/** A finite number from a URL value, or `fallback` when it is missing or malformed. */
function parseNumber(value: string | null | undefined, fallback: number): number {
	if (!value) return fallback;
	const parsed = Number.parseFloat(value);
	return Number.isFinite(parsed) ? parsed : fallback;
}

/**
 * Encode calculator state into URL search params.
 * Uses short keys to keep URLs compact.
 */
export function encodeState(state: ShareableState): string {
	const params = new URLSearchParams();

	for (const key of INPUT_KEYS) {
		params.set(INPUT_PARAMS[key], state.inputs[key].toString());
	}

	params.set('type', state.optionType);

	if (state.mode !== 'single') params.set('mode', state.mode);
	if (state.activeTab !== 'payoff') params.set('tab', state.activeTab);
	if (state.ticker) params.set('ticker', state.ticker);

	if (state.strategyLegs && state.strategyLegs.length > 0) {
		const legsEncoded = state.strategyLegs
			.map(
				(leg) =>
					`${leg.type[0]}${leg.position > 0 ? '+' : '-'}${leg.strike}x${leg.quantity}@${leg.expiry}`
			)
			.join('|');
		params.set('legs', legsEncoded);
	}

	return params.toString();
}

/**
 * Decode one `legs` entry (`c+105x1@0.25`: type, side, strike x quantity @ expiry in years).
 * Returns `null` for a malformed entry.
 */
function decodeLeg(encoded: string, index: number): StrategyLeg | null {
	const match = /^([cp])([+-])([^x@]+)x([^@]*)@(.+)$/.exec(encoded);
	if (!match) return null;
	const [, type, side, strikeText, quantityText, expiryText] = match;

	const strike = Number.parseFloat(strikeText);
	const expiry = Number.parseFloat(expiryText);
	if (!Number.isFinite(strike) || !Number.isFinite(expiry)) return null;

	return {
		id: `shared-${index}`,
		type: type === 'c' ? 'call' : 'put',
		position: side === '+' ? 1 : -1,
		strike,
		quantity: Number.parseInt(quantityText, 10) || 1,
		expiry,
		premium: 0
	};
}

/**
 * Decode URL search params back into a partial ShareableState.
 * Missing or malformed params fall back to defaults or are left out.
 * Accepts anything with a `get` method, including SvelteKit's read-only `page.url.searchParams`.
 */
export function decodeState(searchParams: Pick<URLSearchParams, 'get'>): Partial<ShareableState> {
	const state: Partial<ShareableState> = {};

	if (INPUT_KEYS.some((key) => searchParams.get(INPUT_PARAMS[key]))) {
		const inputs = { ...DEFAULT_INPUTS };
		for (const key of INPUT_KEYS) {
			inputs[key] = parseNumber(searchParams.get(INPUT_PARAMS[key]), DEFAULT_INPUTS[key]);
		}
		state.inputs = inputs;
	}

	const type = searchParams.get('type');
	if (type === 'call' || type === 'put') state.optionType = type;

	const mode = searchParams.get('mode');
	if (mode && isCalculatorMode(mode)) state.mode = mode;

	const tab = searchParams.get('tab');
	if (tab && isChartTab(tab)) state.activeTab = tab;

	const ticker = searchParams.get('ticker');
	if (ticker) state.ticker = ticker;

	const legs = searchParams.get('legs');
	if (legs) {
		const decoded = legs.split('|').flatMap((leg, index) => decodeLeg(leg, index) ?? []);
		if (decoded.length > 0) state.strategyLegs = decoded;
	}

	return state;
}

/**
 * Generate a full shareable URL from calculator state.
 * Uses window.location.origin as base if no baseURL provided (browser only).
 */
export function generateShareURL(state: ShareableState, baseURL?: string): string {
	const base = baseURL ?? `${window.location.origin}/tools/options-calculator`;
	const params = encodeState(state);
	return `${base}?${params}`;
}

/**
 * Copy text to clipboard with fallback for older browsers.
 * Returns true on success, false on failure.
 */
export async function copyToClipboard(text: string): Promise<boolean> {
	try {
		await navigator.clipboard.writeText(text);
		return true;
	} catch {
		// Fallback (insecure contexts, older browsers): select a temporary textarea and copy.
		const textarea = document.createElement('textarea');
		textarea.value = text;
		textarea.setAttribute('readonly', '');
		textarea.style.position = 'fixed';
		textarea.style.opacity = '0';
		textarea.style.pointerEvents = 'none';
		document.body.appendChild(textarea);
		textarea.select();
		let success = false;
		try {
			success = document.execCommand('copy');
		} catch {
			// execCommand failed too
		} finally {
			textarea.remove();
		}
		return success;
	}
}

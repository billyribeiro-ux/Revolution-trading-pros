// ============================================================
// SHARE UTILITIES — URL-based State Serialization
// Encode calculator state into a shareable URL.
// Decode URL params back into calculator state.
// ============================================================

import type { CalculatorMode, ChartTab, ShareableState } from '../engine/types.js';
import { DEFAULT_INPUTS } from '../engine/constants.js';

/**
 * Encode calculator state into URL search params.
 * Uses short keys to keep URLs compact.
 */
export function encodeState(state: ShareableState): string {
	const params = new URLSearchParams();

	params.set('s', state.inputs.spotPrice.toString());
	params.set('k', state.inputs.strikePrice.toString());
	params.set('v', state.inputs.volatility.toString());
	params.set('t', state.inputs.timeToExpiry.toString());
	params.set('r', state.inputs.riskFreeRate.toString());
	params.set('q', state.inputs.dividendYield.toString());

	params.set('type', state.optionType);

	if (state.mode !== 'single') params.set('mode', state.mode);
	if (state.activeTab !== 'payoff') params.set('tab', state.activeTab);
	if (state.ticker) params.set('ticker', state.ticker);

	if (state.strategyLegs && state.strategyLegs.length > 0) {
		const legsEncoded = state.strategyLegs
			.map(
				(leg) =>
					`${leg.type[0]}${leg.position > 0 ? '+' : '-'}${leg.strike}x${leg.quantity}@${leg.expiry}`,
			)
			.join('|');
		params.set('legs', legsEncoded);
	}

	return params.toString();
}

/**
 * Decode URL search params back into a partial ShareableState.
 * Missing params fall back to defaults.
 */
export function decodeState(searchParams: URLSearchParams): Partial<ShareableState> {
	const state: Partial<ShareableState> = {};

	const s = searchParams.get('s');
	const k = searchParams.get('k');
	const v = searchParams.get('v');
	const t = searchParams.get('t');
	const r = searchParams.get('r');
	const q = searchParams.get('q');

	if (s || k || v || t || r || q) {
		state.inputs = {
			spotPrice: s ? parseFloat(s) : DEFAULT_INPUTS.spotPrice,
			strikePrice: k ? parseFloat(k) : DEFAULT_INPUTS.strikePrice,
			volatility: v ? parseFloat(v) : DEFAULT_INPUTS.volatility,
			timeToExpiry: t ? parseFloat(t) : DEFAULT_INPUTS.timeToExpiry,
			riskFreeRate: r ? parseFloat(r) : DEFAULT_INPUTS.riskFreeRate,
			dividendYield: q ? parseFloat(q) : DEFAULT_INPUTS.dividendYield,
		};
	}

	const type = searchParams.get('type');
	if (type === 'call' || type === 'put') state.optionType = type;

	const mode = searchParams.get('mode') as CalculatorMode | null;
	if (mode === 'single' || mode === 'strategy' || mode === 'compare') state.mode = mode;

	const tab = searchParams.get('tab') as ChartTab | null;
	if (tab) state.activeTab = tab;

	const ticker = searchParams.get('ticker');
	if (ticker) state.ticker = ticker;

	const legs = searchParams.get('legs');
	if (legs) {
		try {
			state.strategyLegs = legs.split('|').map((legStr, idx) => {
				const legType: 'call' | 'put' = legStr[0] === 'c' ? 'call' : 'put';
				const position: 1 | -1 = legStr[1] === '+' ? 1 : -1;
				const rest = legStr.slice(2);
				const [strikeQty, expiry] = rest.split('@');
				const [strike, quantity] = strikeQty.split('x');
				return {
					id: `shared-${idx}`,
					type: legType,
					position,
					strike: parseFloat(strike),
					quantity: parseInt(quantity) || 1,
					expiry: parseFloat(expiry),
					premium: 0,
				};
			});
		} catch {
			// Invalid legs encoding — silently ignore
		}
	}

	return state;
}

/**
 * Generate a full shareable URL from calculator state.
 * Uses window.location.origin as base if no baseURL provided.
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
		// Fallback: create a temporary textarea
		const textarea = document.createElement('textarea');
		textarea.value = text;
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
		}
		document.body.removeChild(textarea);
		return success;
	}
}

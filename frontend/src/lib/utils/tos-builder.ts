/**
 * TOS String Builder - ThinkOrSwim Format Generator
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Generates TOS-formatted trade strings for options and shares.
 *
 * OPTIONS FORMAT:
 * BUY +1 AAPL 100 (Weeklys) 27 FEB 26 255 CALL @9.55 LMT
 *
 * SHARES FORMAT:
 * BUY +100 AAPL @255.03 LMT
 *
 * @version 1.0.0
 */

import type {
	TradeType,
	TradeAction,
	OptionType,
	OrderType,
	ContractType,
	TosStringParams
} from '$lib/types/trading';

// ═══════════════════════════════════════════════════════════════════════════
// DATE FORMATTING
// ═══════════════════════════════════════════════════════════════════════════

const MONTHS = ['JAN', 'FEB', 'MAR', 'APR', 'MAY', 'JUN', 'JUL', 'AUG', 'SEP', 'OCT', 'NOV', 'DEC'];

/**
 * Format date to TOS expiration format: "27 FEB 26"
 */
export function formatExpirationDate(dateStr: string): string {
	const date = new Date(dateStr);
	const day = date.getUTCDate();
	const month = MONTHS[date.getUTCMonth()];
	const year = date.getUTCFullYear().toString().slice(-2);
	return `${day} ${month} ${year}`;
}

/**
 * Format price with 2 decimal places
 */
export function formatPrice(price: number): string {
	return price.toFixed(2);
}

// ═══════════════════════════════════════════════════════════════════════════
// TOS STRING BUILDER
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Build TOS-formatted trade string
 *
 * @example Options:
 * buildTosString({
 *   trade_type: 'options',
 *   action: 'BUY',
 *   quantity: 1,
 *   ticker: 'AAPL',
 *   option_type: 'CALL',
 *   strike: 255,
 *   expiration: '2026-02-27',
 *   contract_type: 'Weeklys',
 *   order_type: 'LMT',
 *   limit_price: 9.55
 * })
 * // Returns: "BUY +1 AAPL 100 (Weeklys) 27 FEB 26 255 CALL @9.55 LMT"
 *
 * @example Shares:
 * buildTosString({
 *   trade_type: 'shares',
 *   action: 'BUY',
 *   quantity: 100,
 *   ticker: 'AAPL',
 *   order_type: 'LMT',
 *   limit_price: 255.03
 * })
 * // Returns: "BUY +100 AAPL @255.03 LMT"
 */
export function buildTosString(params: TosStringParams): string {
	const { trade_type, action, quantity, ticker, order_type, limit_price } = params;

	// Determine sign based on action
	const sign = action === 'BUY' ? '+' : '-';
	const qtyStr = `${sign}${quantity}`;

	// Build price portion
	const priceStr = order_type === 'MKT' ? 'MKT' : `@${formatPrice(limit_price!)} LMT`;

	if (trade_type === 'shares') {
		// SHARES FORMAT: BUY +100 AAPL @255.03 LMT
		return `${action} ${qtyStr} ${ticker} ${priceStr}`;
	}

	// OPTIONS FORMAT: BUY +1 AAPL 100 (Weeklys) 27 FEB 26 255 CALL @9.55 LMT
	const { option_type, strike, expiration, contract_type } = params;

	if (!option_type || !strike || !expiration) {
		throw new Error('Options trades require option_type, strike, and expiration');
	}

	const expStr = formatExpirationDate(expiration);
	const strikeStr = Number.isInteger(strike) ? strike.toString() : formatPrice(strike);
	const contractStr = contract_type || 'Standard';

	return `${action} ${qtyStr} ${ticker} 100 (${contractStr}) ${expStr} ${strikeStr} ${option_type} ${priceStr}`;
}

// ═══════════════════════════════════════════════════════════════════════════
// TOS STRING PARSER
// ═══════════════════════════════════════════════════════════════════════════

export interface ParsedTosString {
	trade_type: TradeType;
	action: TradeAction;
	quantity: number;
	ticker: string;
	option_type?: OptionType;
	strike?: number;
	expiration?: string;
	contract_type?: ContractType;
	order_type: OrderType;
	limit_price?: number;
}

/**
 * Parse a TOS string back to components (best effort)
 */
export function parseTosString(tosString: string): ParsedTosString | null {
	try {
		const parts = tosString.trim().split(/\s+/);

		// First part is action
		const action = parts[0] as TradeAction;
		if (!['BUY', 'SELL'].includes(action)) return null;

		// Second part is quantity with sign
		const qtyMatch = parts[1].match(/^([+-])(\d+)$/);
		if (!qtyMatch) return null;
		const quantity = parseInt(qtyMatch[2], 10);

		// Third part is ticker
		const ticker = parts[2];

		// Check if shares or options
		if (parts[3] === '100' && parts[4]?.startsWith('(')) {
			// OPTIONS FORMAT
			const contractType = parts[4].replace(/[()]/g, '') as ContractType;
			const day = parts[5];
			const month = parts[6];
			const year = parts[7];
			const strike = parseFloat(parts[8]);
			const optionType = parts[9] as OptionType;

			// Parse price
			const priceStr = parts.slice(10).join(' ');
			const orderType: OrderType = priceStr === 'MKT' ? 'MKT' : 'LMT';
			const limitPrice =
				orderType === 'LMT' ? parseFloat(priceStr.replace('@', '').replace(' LMT', '')) : undefined;

			// Convert date back to ISO
			const monthIndex = MONTHS.indexOf(month);
			const fullYear = 2000 + parseInt(year, 10);
			const expiration = new Date(Date.UTC(fullYear, monthIndex, parseInt(day, 10)))
				.toISOString()
				.split('T')[0];

			return {
				trade_type: 'options',
				action,
				quantity,
				ticker,
				option_type: optionType,
				strike,
				expiration,
				contract_type: contractType,
				order_type: orderType,
				limit_price: limitPrice
			};
		} else {
			// SHARES FORMAT
			const priceStr = parts.slice(3).join(' ');
			const orderType: OrderType = priceStr === 'MKT' ? 'MKT' : 'LMT';
			const limitPrice =
				orderType === 'LMT' ? parseFloat(priceStr.replace('@', '').replace(' LMT', '')) : undefined;

			return {
				trade_type: 'shares',
				action,
				quantity,
				ticker,
				order_type: orderType,
				limit_price: limitPrice
			};
		}
	} catch {
		return null;
	}
}

// ═══════════════════════════════════════════════════════════════════════════
// VALIDATION
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Validate TOS string parameters before building
 */
export function validateTosParams(params: Partial<TosStringParams>): string[] {
	const errors: string[] = [];

	if (!params.trade_type) {
		errors.push('Trade type is required');
	}

	if (!params.action || !['BUY', 'SELL'].includes(params.action)) {
		errors.push('Action must be BUY or SELL');
	}

	if (!params.quantity || params.quantity <= 0) {
		errors.push('Quantity must be greater than 0');
	}

	if (!params.ticker || params.ticker.length === 0) {
		errors.push('Ticker is required');
	}

	if (!params.order_type || !['MKT', 'LMT'].includes(params.order_type)) {
		errors.push('Order type must be MKT or LMT');
	}

	if (params.order_type === 'LMT' && (!params.limit_price || params.limit_price <= 0)) {
		errors.push('Limit price is required for LMT orders');
	}

	if (params.trade_type === 'options') {
		if (!params.option_type || !['CALL', 'PUT'].includes(params.option_type)) {
			errors.push('Option type must be CALL or PUT');
		}

		if (!params.strike || params.strike <= 0) {
			errors.push('Strike price is required for options');
		}

		if (!params.expiration) {
			errors.push('Expiration date is required for options');
		}
	}

	return errors;
}

// ═══════════════════════════════════════════════════════════════════════════
// HELPERS
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Get the next Friday from a given date (for weekly options)
 */
export function getNextFriday(from: Date = new Date()): Date {
	const date = new Date(from);
	const day = date.getDay();
	const daysUntilFriday = (5 - day + 7) % 7 || 7;
	date.setDate(date.getDate() + daysUntilFriday);
	return date;
}

/**
 * Get standard monthly expiration (3rd Friday of month)
 */
export function getMonthlyExpiration(month: number, year: number): Date {
	const date = new Date(year, month, 1);
	// Find first Friday
	const dayOfWeek = date.getDay();
	const firstFriday = dayOfWeek <= 5 ? 5 - dayOfWeek + 1 : 5 - dayOfWeek + 8;
	// Third Friday = first Friday + 14
	date.setDate(firstFriday + 14);
	return date;
}

/**
 * Determine contract type based on expiration date
 */
export function determineContractType(expiration: string): ContractType {
	const date = new Date(expiration);
	const now = new Date();

	// Calculate days until expiration
	const daysUntil = Math.ceil((date.getTime() - now.getTime()) / (1000 * 60 * 60 * 24));

	if (daysUntil <= 7) return 'Weeklys';
	if (daysUntil <= 45) return 'Monthly';
	return 'LEAPS';
}

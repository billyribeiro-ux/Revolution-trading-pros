// ============================================================
// NUMBER & DISPLAY FORMATTERS
// ============================================================

const currencyFormatter = new Intl.NumberFormat('en-US', {
	style: 'currency',
	currency: 'USD',
	minimumFractionDigits: 2,
	maximumFractionDigits: 2,
});

const precisionCurrencyFormatter = new Intl.NumberFormat('en-US', {
	style: 'currency',
	currency: 'USD',
	minimumFractionDigits: 4,
	maximumFractionDigits: 4,
});

const percentFormatter = new Intl.NumberFormat('en-US', {
	style: 'percent',
	minimumFractionDigits: 1,
	maximumFractionDigits: 1,
});

const precisionPercentFormatter = new Intl.NumberFormat('en-US', {
	style: 'percent',
	minimumFractionDigits: 2,
	maximumFractionDigits: 2,
});

/** Format as currency: $1,234.56 */
export function formatCurrency(value: number): string {
	return currencyFormatter.format(value);
}

/** Format as precise currency: $1.2345 */
export function formatPreciseCurrency(value: number): string {
	return precisionCurrencyFormatter.format(value);
}

/** Format as percentage: 25.0% (input 0.25) */
export function formatPercent(value: number): string {
	return percentFormatter.format(value);
}

/** Format as precise percentage: 25.00% */
export function formatPrecisePercent(value: number): string {
	return precisionPercentFormatter.format(value);
}

/** Format a Greek value with appropriate precision */
export function formatGreek(value: number, greekName: string): string {
	const decimals = getGreekDecimals(greekName);
	const formatted = value.toFixed(decimals);

	if (value > 0 && !formatted.startsWith('-')) {
		return `+${formatted}`;
	}
	return formatted;
}

function getGreekDecimals(greekName: string): number {
	switch (greekName) {
		case 'delta':
			return 4;
		case 'gamma':
			return 6;
		case 'theta':
			return 4;
		case 'vega':
			return 4;
		case 'rho':
			return 4;
		case 'charm':
			return 6;
		case 'vanna':
			return 6;
		case 'volga':
			return 4;
		case 'speed':
			return 8;
		case 'color':
			return 8;
		case 'zomma':
			return 6;
		default:
			return 4;
	}
}

/** Format days to expiry from years */
export function formatDaysToExpiry(years: number): string {
	const days = Math.round(years * 365);
	if (days === 0) return 'Expiry';
	if (days === 1) return '1 day';
	return `${days} days`;
}

/** Format time to expiry in a human-readable way */
export function formatTimeToExpiry(years: number): string {
	const totalDays = years * 365;
	if (totalDays < 1) return `${Math.round(totalDays * 24)}h`;
	if (totalDays < 7) return `${Math.round(totalDays)}d`;
	if (totalDays < 30) return `${Math.round(totalDays / 7)}w ${Math.round(totalDays % 7)}d`;
	if (totalDays < 365) return `${Math.round(totalDays / 30)}mo`;
	return `${(totalDays / 365).toFixed(1)}yr`;
}

/** Format large numbers with K/M suffixes */
export function formatCompact(value: number): string {
	if (Math.abs(value) >= 1_000_000) return `${(value / 1_000_000).toFixed(1)}M`;
	if (Math.abs(value) >= 1_000) return `${(value / 1_000).toFixed(1)}K`;
	return value.toFixed(2);
}

/** Determine color class based on value (positive = green, negative = red) */
export function valueColor(value: number): 'profit' | 'loss' | 'neutral' {
	if (value > 0.0001) return 'profit';
	if (value < -0.0001) return 'loss';
	return 'neutral';
}

/**
 * ═══════════════════════════════════════════════════════════════════════════════
 * Explosive Swings Dashboard - Formatting Utilities
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * @description Formatting functions for the Explosive Swings dashboard
 * @version 4.1.0 - Visual Polish Pass
 * @standards Apple Principal Engineer ICT 7+ Standards
 */

/**
 * Format a percentage value with sign and decimal places
 * @param value - The percentage value
 * @param decimals - Number of decimal places (default: 1)
 * @returns Formatted string like "+5.7%" or "-2.1%"
 */
export function formatPercent(value: number, decimals: number = 1): string {
	const sign = value >= 0 ? '+' : '';
	return `${sign}${value.toFixed(decimals)}%`;
}

/**
 * Format a price value as currency
 * @param value - The price value
 * @param decimals - Number of decimal places (default: 2)
 * @returns Formatted string like "$142.50"
 */
export function formatPrice(value: number, decimals: number = 2): string {
	return `$${value.toLocaleString('en-US', {
		minimumFractionDigits: decimals,
		maximumFractionDigits: decimals
	})}`;
}

/**
 * Format a date as relative time ("12 min ago", "2 hours ago", etc.)
 * @param date - The date to format
 * @returns Relative time string
 */
export function formatRelativeTime(date: Date): string {
	const now = new Date();
	const diffMs = now.getTime() - date.getTime();
	const diffSeconds = Math.floor(diffMs / 1000);
	const diffMinutes = Math.floor(diffSeconds / 60);
	const diffHours = Math.floor(diffMinutes / 60);
	const diffDays = Math.floor(diffHours / 24);

	if (diffSeconds < 60) {
		return 'Just now';
	} else if (diffMinutes < 60) {
		return `${diffMinutes} min ago`;
	} else if (diffHours < 24) {
		return `${diffHours}h ago`;
	} else if (diffDays === 1) {
		return 'Yesterday';
	} else if (diffDays < 7) {
		return `${diffDays} days ago`;
	} else {
		return formatDate(date);
	}
}

/**
 * Format a date as a readable string
 * @param date - The date to format
 * @returns Formatted string like "January 13, 2026"
 */
export function formatDate(date: Date | null | undefined): string {
	if (!date || !(date instanceof Date) || isNaN(date.getTime())) return '';
	return date.toLocaleDateString('en-US', {
		month: 'long',
		day: 'numeric',
		year: 'numeric'
	});
}

/**
 * Format a date as short format
 * @param date - The date to format
 * @returns Formatted string like "Jan 13"
 */
export function formatDateShort(date: Date | null | undefined): string {
	if (!date || !(date instanceof Date) || isNaN(date.getTime())) return '';
	return date.toLocaleDateString('en-US', {
		month: 'short',
		day: 'numeric'
	});
}

/**
 * Format a date with time
 * @param date - The date to format
 * @returns Formatted string like "January 13, 2026 at 9:00 AM ET"
 */
export function formatDateTime(date: Date | null | undefined): string {
	if (!date || !(date instanceof Date) || isNaN(date.getTime())) return '';
	const dateStr = formatDate(date);
	const timeStr = date.toLocaleTimeString('en-US', {
		hour: 'numeric',
		minute: '2-digit',
		hour12: true
	});
	return `${dateStr} at ${timeStr} ET`;
}

/**
 * Format win/loss ratio as a string
 * @param wins - Number of wins
 * @param total - Total trades
 * @returns Formatted string like "6/7"
 */
export function formatWinLossRatio(wins: number, total: number): string {
	return `${wins}/${total}`;
}

/**
 * Format risk/reward ratio
 * @param ratio - The R/R ratio value
 * @returns Formatted string like "3.2:1"
 */
export function formatRiskReward(ratio: number): string {
	return `${ratio.toFixed(1)}:1`;
}

/**
 * Truncate text to a maximum length with ellipsis
 * @param text - The text to truncate
 * @param maxLength - Maximum length before truncation
 * @returns Truncated string with ellipsis if needed
 */
export function truncateText(text: string, maxLength: number): string {
	if (text.length <= maxLength) return text;
	return `${text.slice(0, maxLength - 3)}...`;
}

/**
 * Format a ticker symbol (uppercase, trimmed)
 * @param ticker - The ticker symbol
 * @returns Formatted ticker
 */
export function formatTicker(ticker: string): string {
	return ticker.trim().toUpperCase();
}

/**
 * Format a number with commas for thousands
 * @param value - The number to format
 * @returns Formatted string like "1,234,567"
 */
export function formatNumber(value: number): string {
	return value.toLocaleString('en-US');
}

/**
 * Format duration in seconds to mm:ss or hh:mm:ss
 * @param seconds - Duration in seconds
 * @returns Formatted duration string
 */
export function formatDuration(seconds: number): string {
	const hrs = Math.floor(seconds / 3600);
	const mins = Math.floor((seconds % 3600) / 60);
	const secs = seconds % 60;

	if (hrs > 0) {
		return `${hrs}:${mins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
	}
	return `${mins}:${secs.toString().padStart(2, '0')}`;
}

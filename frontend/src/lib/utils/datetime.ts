/**
 * Date & Time Utilities
 * ═══════════════════════════════════════════════════════════════════════════
 */

/**
 * Format date to readable string
 */
export function formatDate(
	date: Date | string | number,
	options: Intl.DateTimeFormatOptions = {}
): string {
	const d = typeof date === 'string' || typeof date === 'number' ? new Date(date) : date;

	const defaultOptions: Intl.DateTimeFormatOptions = {
		year: 'numeric',
		month: 'long',
		day: 'numeric',
		...options
	};

	return new Intl.DateTimeFormat('en-US', defaultOptions).format(d);
}

/**
 * Format date to relative time (e.g., "2 hours ago")
 */
export function formatRelativeTime(date: Date | string | number): string {
	const d = typeof date === 'string' || typeof date === 'number' ? new Date(date) : date;
	const now = new Date();
	const diffMs = now.getTime() - d.getTime();
	const diffSec = Math.floor(diffMs / 1000);
	const diffMin = Math.floor(diffSec / 60);
	const diffHour = Math.floor(diffMin / 60);
	const diffDay = Math.floor(diffHour / 24);
	const diffWeek = Math.floor(diffDay / 7);
	const diffMonth = Math.floor(diffDay / 30);
	const diffYear = Math.floor(diffDay / 365);

	if (diffSec < 60) return 'just now';
	if (diffMin < 60) return `${diffMin} minute${diffMin > 1 ? 's' : ''} ago`;
	if (diffHour < 24) return `${diffHour} hour${diffHour > 1 ? 's' : ''} ago`;
	if (diffDay < 7) return `${diffDay} day${diffDay > 1 ? 's' : ''} ago`;
	if (diffWeek < 4) return `${diffWeek} week${diffWeek > 1 ? 's' : ''} ago`;
	if (diffMonth < 12) return `${diffMonth} month${diffMonth > 1 ? 's' : ''} ago`;
	return `${diffYear} year${diffYear > 1 ? 's' : ''} ago`;
}

/**
 * Check if date is today
 */
export function isToday(date: Date | string | number): boolean {
	const d = typeof date === 'string' || typeof date === 'number' ? new Date(date) : date;
	const today = new Date();

	return (
		d.getDate() === today.getDate() &&
		d.getMonth() === today.getMonth() &&
		d.getFullYear() === today.getFullYear()
	);
}

/**
 * Check if date is in the past
 */
export function isPast(date: Date | string | number): boolean {
	const d = typeof date === 'string' || typeof date === 'number' ? new Date(date) : date;
	return d.getTime() < Date.now();
}

/**
 * Check if date is in the future
 */
export function isFuture(date: Date | string | number): boolean {
	const d = typeof date === 'string' || typeof date === 'number' ? new Date(date) : date;
	return d.getTime() > Date.now();
}

/**
 * Add days to date
 */
export function addDays(date: Date, days: number): Date {
	const result = new Date(date);
	result.setDate(result.getDate() + days);
	return result;
}

/**
 * Format timestamp to ISO string
 */
export function toISOString(date: Date | string | number): string {
	const d = typeof date === 'string' || typeof date === 'number' ? new Date(date) : date;
	return d.toISOString();
}

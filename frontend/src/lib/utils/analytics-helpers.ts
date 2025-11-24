/**
 * Analytics Helper Utilities
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Utility functions for analytics tracking, formatting, and data manipulation.
 *
 * @version 1.0.0
 */

// ═══════════════════════════════════════════════════════════════════════════
// Formatting Utilities
// ═══════════════════════════════════════════════════════════════════════════

export function formatNumber(num: number, decimals: number = 0): string {
	if (num >= 1000000000) {
		return (num / 1000000000).toFixed(decimals) + 'B';
	}
	if (num >= 1000000) {
		return (num / 1000000).toFixed(decimals) + 'M';
	}
	if (num >= 1000) {
		return (num / 1000).toFixed(decimals) + 'K';
	}
	return num.toFixed(decimals);
}

export function formatCurrency(num: number, currency: string = 'USD'): string {
	return new Intl.NumberFormat('en-US', {
		style: 'currency',
		currency,
		minimumFractionDigits: 0,
		maximumFractionDigits: 2
	}).format(num);
}

export function formatPercentage(num: number, decimals: number = 1): string {
	return num.toFixed(decimals) + '%';
}

export function formatDuration(seconds: number): string {
	if (seconds < 60) {
		return `${Math.round(seconds)}s`;
	}
	if (seconds < 3600) {
		return `${Math.round(seconds / 60)}m`;
	}
	if (seconds < 86400) {
		return `${Math.round(seconds / 3600)}h`;
	}
	return `${Math.round(seconds / 86400)}d`;
}

export function formatDate(
	date: string | Date,
	format: 'short' | 'long' | 'relative' = 'short'
): string {
	const d = typeof date === 'string' ? new Date(date) : date;

	if (format === 'relative') {
		return formatRelativeTime(d);
	}

	if (format === 'long') {
		return d.toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'long',
			day: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	return d.toLocaleDateString('en-US', {
		month: 'short',
		day: 'numeric',
		year: 'numeric'
	});
}

export function formatRelativeTime(date: Date): string {
	const now = new Date();
	const diffMs = now.getTime() - date.getTime();
	const diffSec = Math.floor(diffMs / 1000);
	const diffMin = Math.floor(diffSec / 60);
	const diffHour = Math.floor(diffMin / 60);
	const diffDay = Math.floor(diffHour / 24);

	if (diffSec < 60) return 'just now';
	if (diffMin < 60) return `${diffMin}m ago`;
	if (diffHour < 24) return `${diffHour}h ago`;
	if (diffDay < 7) return `${diffDay}d ago`;
	if (diffDay < 30) return `${Math.floor(diffDay / 7)}w ago`;
	if (diffDay < 365) return `${Math.floor(diffDay / 30)}mo ago`;
	return `${Math.floor(diffDay / 365)}y ago`;
}

// ═══════════════════════════════════════════════════════════════════════════
// Period Utilities
// ═══════════════════════════════════════════════════════════════════════════

export interface PeriodOption {
	value: string;
	label: string;
	days: number;
}

export const PERIOD_OPTIONS: PeriodOption[] = [
	{ value: '1d', label: 'Last 24 Hours', days: 1 },
	{ value: '7d', label: 'Last 7 Days', days: 7 },
	{ value: '14d', label: 'Last 14 Days', days: 14 },
	{ value: '30d', label: 'Last 30 Days', days: 30 },
	{ value: '60d', label: 'Last 60 Days', days: 60 },
	{ value: '90d', label: 'Last 90 Days', days: 90 },
	{ value: '180d', label: 'Last 6 Months', days: 180 },
	{ value: '365d', label: 'Last Year', days: 365 }
];

export function getPeriodDates(period: string): { start: Date; end: Date } {
	const end = new Date();
	const start = new Date();

	const option = PERIOD_OPTIONS.find((p) => p.value === period);
	if (option) {
		start.setDate(start.getDate() - option.days);
	} else {
		start.setDate(start.getDate() - 30); // Default to 30 days
	}

	return { start, end };
}

// ═══════════════════════════════════════════════════════════════════════════
// Color Utilities
// ═══════════════════════════════════════════════════════════════════════════

export function getTrendColor(trend: 'up' | 'down' | 'stable', isPositive: boolean = true): string {
	if (trend === 'stable') return 'text-gray-400';
	if (trend === 'up') return isPositive ? 'text-green-400' : 'text-red-400';
	return isPositive ? 'text-red-400' : 'text-green-400';
}

export function getChangeColor(change: number, isPositive: boolean = true): string {
	if (Math.abs(change) < 0.1) return 'text-gray-400';
	if (change > 0) return isPositive ? 'text-green-400' : 'text-red-400';
	return isPositive ? 'text-red-400' : 'text-green-400';
}

export function getPercentileColor(percentile: number): string {
	if (percentile >= 90) return 'text-green-500';
	if (percentile >= 75) return 'text-green-400';
	if (percentile >= 50) return 'text-yellow-400';
	if (percentile >= 25) return 'text-orange-400';
	return 'text-red-400';
}

// ═══════════════════════════════════════════════════════════════════════════
// Chart Utilities
// ═══════════════════════════════════════════════════════════════════════════

export function generateChartColors(count: number): string[] {
	const baseColors = [
		'#fbbf24', // yellow
		'#60a5fa', // blue
		'#34d399', // green
		'#f87171', // red
		'#a78bfa', // purple
		'#fb923c', // orange
		'#2dd4bf', // teal
		'#f472b6' // pink
	];

	if (count <= baseColors.length) {
		return baseColors.slice(0, count);
	}

	// Generate additional colors
	const colors = [...baseColors];
	while (colors.length < count) {
		const hue = (colors.length * 137.5) % 360;
		colors.push(`hsl(${hue}, 70%, 60%)`);
	}

	return colors;
}

export function interpolateColor(color1: string, color2: string, factor: number): string {
	// Simple RGB interpolation
	const c1 = hexToRgb(color1);
	const c2 = hexToRgb(color2);

	if (!c1 || !c2) return color1;

	const r = Math.round(c1.r + (c2.r - c1.r) * factor);
	const g = Math.round(c1.g + (c2.g - c1.g) * factor);
	const b = Math.round(c1.b + (c2.b - c1.b) * factor);

	return `rgb(${r}, ${g}, ${b})`;
}

function hexToRgb(hex: string): { r: number; g: number; b: number } | null {
	const result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
	return result
		? {
				r: parseInt(result[1], 16),
				g: parseInt(result[2], 16),
				b: parseInt(result[3], 16)
			}
		: null;
}

// ═══════════════════════════════════════════════════════════════════════════
// Data Transformation
// ═══════════════════════════════════════════════════════════════════════════

export function calculateGrowthRate(current: number, previous: number): number {
	if (previous === 0) return current > 0 ? 100 : 0;
	return ((current - previous) / previous) * 100;
}

export function calculateCompoundGrowthRate(values: number[]): number {
	if (values.length < 2) return 0;

	const first = values[0];
	const last = values[values.length - 1];
	const periods = values.length - 1;

	if (first === 0) return 0;

	return (Math.pow(last / first, 1 / periods) - 1) * 100;
}

export function smoothData(data: number[], windowSize: number = 3): number[] {
	if (data.length < windowSize) return data;

	const smoothed: number[] = [];
	const halfWindow = Math.floor(windowSize / 2);

	for (let i = 0; i < data.length; i++) {
		let sum = 0;
		let count = 0;

		for (let j = Math.max(0, i - halfWindow); j <= Math.min(data.length - 1, i + halfWindow); j++) {
			sum += data[j];
			count++;
		}

		smoothed.push(sum / count);
	}

	return smoothed;
}

export function detectAnomalies(data: number[], threshold: number = 2): boolean[] {
	const mean = data.reduce((sum, val) => sum + val, 0) / data.length;
	const variance = data.reduce((sum, val) => sum + Math.pow(val - mean, 2), 0) / data.length;
	const stdDev = Math.sqrt(variance);

	return data.map((val) => Math.abs(val - mean) > threshold * stdDev);
}

// ═══════════════════════════════════════════════════════════════════════════
// Event Tracking Helpers
// ═══════════════════════════════════════════════════════════════════════════

export function getDeviceType(): 'mobile' | 'tablet' | 'desktop' {
	const width = window.innerWidth;
	if (width < 768) return 'mobile';
	if (width < 1024) return 'tablet';
	return 'desktop';
}

export function getBrowserInfo(): {
	name: string;
	version: string;
	os: string;
} {
	const ua = navigator.userAgent;
	let browserName = 'Unknown';
	let browserVersion = 'Unknown';
	let os = 'Unknown';

	// Browser detection
	if (ua.indexOf('Firefox') > -1) {
		browserName = 'Firefox';
		browserVersion = ua.match(/Firefox\/(\d+\.\d+)/)?.[1] || 'Unknown';
	} else if (ua.indexOf('Chrome') > -1) {
		browserName = 'Chrome';
		browserVersion = ua.match(/Chrome\/(\d+\.\d+)/)?.[1] || 'Unknown';
	} else if (ua.indexOf('Safari') > -1) {
		browserName = 'Safari';
		browserVersion = ua.match(/Version\/(\d+\.\d+)/)?.[1] || 'Unknown';
	} else if (ua.indexOf('Edge') > -1) {
		browserName = 'Edge';
		browserVersion = ua.match(/Edge\/(\d+\.\d+)/)?.[1] || 'Unknown';
	}

	// OS detection
	if (ua.indexOf('Win') > -1) os = 'Windows';
	else if (ua.indexOf('Mac') > -1) os = 'macOS';
	else if (ua.indexOf('Linux') > -1) os = 'Linux';
	else if (ua.indexOf('Android') > -1) os = 'Android';
	else if (ua.indexOf('iOS') > -1) os = 'iOS';

	return { name: browserName, version: browserVersion, os };
}

export function getPageMetadata(): {
	title: string;
	url: string;
	path: string;
	referrer: string;
	language: string;
} {
	return {
		title: document.title,
		url: window.location.href,
		path: window.location.pathname,
		referrer: document.referrer,
		language: navigator.language
	};
}

// ═══════════════════════════════════════════════════════════════════════════
// Validation
// ═══════════════════════════════════════════════════════════════════════════

export function isValidEventName(name: string): boolean {
	return /^[a-z0-9_]+$/.test(name) && name.length <= 100;
}

export function sanitizeEventProperties(properties: Record<string, any>): Record<string, any> {
	const sanitized: Record<string, any> = {};

	for (const [key, value] of Object.entries(properties)) {
		// Skip invalid keys
		if (!/^[a-z0-9_]+$/.test(key)) continue;

		// Sanitize values
		if (typeof value === 'string') {
			sanitized[key] = value.substring(0, 1000); // Limit string length
		} else if (typeof value === 'number' || typeof value === 'boolean') {
			sanitized[key] = value;
		} else if (value === null || value === undefined) {
			sanitized[key] = null;
		} else {
			sanitized[key] = JSON.stringify(value).substring(0, 1000);
		}
	}

	return sanitized;
}

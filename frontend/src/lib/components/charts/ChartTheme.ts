/**
 * ChartTheme - Unified chart theming for all chart libraries
 * Ensures consistent styling across lightweight-charts, ApexCharts, Chart.js
 * Apple ICT 11+ Principal Engineer - Full light/dark mode support
 *
 * @version 3.0.0
 * @author Revolution Trading Pros
 */

// Dark theme chart colors (original)
export const ChartColorsDark = {
	// Primary palette
	primary: '#6366f1',
	primaryLight: '#818cf8',
	primaryDark: '#4f46e5',

	// Semantic colors
	success: '#10b981',
	successLight: '#34d399',
	error: '#ef4444',
	errorLight: '#f87171',
	warning: '#f59e0b',
	warningLight: '#fbbf24',
	info: '#3b82f6',
	infoLight: '#60a5fa',

	// Neutral colors
	text: '#e2e8f0',
	textSecondary: '#94a3b8',
	textMuted: '#64748b',
	background: '#0f172a',
	surface: '#1e293b',
	border: 'rgba(99, 102, 241, 0.1)',
	grid: 'rgba(99, 102, 241, 0.1)',

	// Chart-specific
	upColor: '#10b981',
	downColor: '#ef4444',
	volumeUp: 'rgba(16, 185, 129, 0.5)',
	volumeDown: 'rgba(239, 68, 68, 0.5)',

	// Gradient colors
	gradientTop: 'rgba(99, 102, 241, 0.4)',
	gradientBottom: 'rgba(99, 102, 241, 0.0)',
} as const;

// Light theme chart colors - Apple-inspired clean aesthetic
export const ChartColorsLight = {
	// Primary palette
	primary: '#4f46e5',
	primaryLight: '#6366f1',
	primaryDark: '#4338ca',

	// Semantic colors
	success: '#059669',
	successLight: '#10b981',
	error: '#dc2626',
	errorLight: '#ef4444',
	warning: '#d97706',
	warningLight: '#f59e0b',
	info: '#2563eb',
	infoLight: '#3b82f6',

	// Neutral colors
	text: '#1d1d1f',
	textSecondary: '#424245',
	textMuted: '#6e6e73',
	background: '#f5f5f7',
	surface: '#ffffff',
	border: 'rgba(0, 0, 0, 0.08)',
	grid: 'rgba(0, 0, 0, 0.06)',

	// Chart-specific
	upColor: '#059669',
	downColor: '#dc2626',
	volumeUp: 'rgba(5, 150, 105, 0.4)',
	volumeDown: 'rgba(220, 38, 38, 0.4)',

	// Gradient colors
	gradientTop: 'rgba(79, 70, 229, 0.3)',
	gradientBottom: 'rgba(79, 70, 229, 0.0)',
} as const;

// Default to light theme colors (Apple Principal Engineer preference)
export const ChartColors = ChartColorsLight;

// Function to get theme-aware colors
export function getChartColors(isDark: boolean = false) {
	return isDark ? ChartColorsDark : ChartColorsLight;
}

export const ChartFonts = {
	family: "'Inter', system-ui, -apple-system, sans-serif",
	size: {
		xs: 10,
		sm: 11,
		md: 12,
		lg: 14,
	},
} as const;

/**
 * Lightweight Charts configuration
 */
export function getLightweightChartsOptions(height = 300) {
	return {
		width: 0, // Will be set dynamically
		height,
		layout: {
			background: { type: 'solid' as const, color: 'transparent' },
			textColor: ChartColors.textSecondary,
			fontFamily: ChartFonts.family,
			fontSize: ChartFonts.size.sm,
		},
		grid: {
			vertLines: { color: ChartColors.grid },
			horzLines: { color: ChartColors.grid },
		},
		crosshair: {
			mode: 1, // Normal
			vertLine: {
				color: ChartColors.primary,
				width: 1,
				style: 2, // Dashed
				labelBackgroundColor: ChartColors.primary,
			},
			horzLine: {
				color: ChartColors.primary,
				width: 1,
				style: 2,
				labelBackgroundColor: ChartColors.primary,
			},
		},
		rightPriceScale: {
			borderColor: ChartColors.border,
			scaleMargins: { top: 0.1, bottom: 0.1 },
		},
		timeScale: {
			borderColor: ChartColors.border,
			timeVisible: true,
			secondsVisible: false,
		},
		handleScale: {
			axisPressedMouseMove: true,
		},
		handleScroll: {
			mouseWheel: true,
			pressedMouseMove: true,
			horzTouchDrag: true,
			vertTouchDrag: false,
		},
	};
}

/**
 * Area series options
 */
export function getAreaSeriesOptions(color = ChartColors.primary) {
	return {
		lineColor: color,
		topColor: `${color}66`, // 40% opacity
		bottomColor: `${color}00`, // 0% opacity
		lineWidth: 2,
	};
}

/**
 * Line series options
 */
export function getLineSeriesOptions(color = ChartColors.primary) {
	return {
		color,
		lineWidth: 2,
	};
}

/**
 * Candlestick series options
 */
export function getCandlestickSeriesOptions() {
	return {
		upColor: ChartColors.upColor,
		downColor: ChartColors.downColor,
		borderUpColor: ChartColors.upColor,
		borderDownColor: ChartColors.downColor,
		wickUpColor: ChartColors.upColor,
		wickDownColor: ChartColors.downColor,
	};
}

/**
 * Histogram series options
 */
export function getHistogramSeriesOptions(color = ChartColors.primary) {
	return {
		color,
		priceFormat: { type: 'volume' as const },
	};
}

/**
 * ApexCharts theme configuration
 * Now supports light/dark mode
 */
export function getApexChartsTheme(isDark: boolean = false) {
	const colors = getChartColors(isDark);
	return {
		chart: {
			background: 'transparent',
			foreColor: colors.textSecondary,
			fontFamily: ChartFonts.family,
			toolbar: {
				show: false,
			},
			zoom: {
				enabled: false,
			},
		},
		colors: [colors.primary, colors.success, colors.warning, colors.info],
		grid: {
			borderColor: colors.grid,
			strokeDashArray: 4,
		},
		stroke: {
			curve: 'smooth' as const,
			width: 2,
		},
		tooltip: {
			theme: isDark ? 'dark' : 'light',
			style: {
				fontSize: '12px',
				fontFamily: ChartFonts.family,
			},
		},
		xaxis: {
			axisBorder: { color: colors.border },
			axisTicks: { color: colors.border },
			labels: {
				style: {
					colors: colors.textMuted,
					fontSize: '11px',
				},
			},
		},
		yaxis: {
			labels: {
				style: {
					colors: colors.textMuted,
					fontSize: '11px',
				},
			},
		},
		legend: {
			labels: {
				colors: colors.textSecondary,
			},
		},
	};
}

/**
 * Chart.js theme configuration
 * Now supports light/dark mode
 */
export function getChartJsTheme(isDark: boolean = false) {
	const colors = getChartColors(isDark);
	return {
		responsive: true,
		maintainAspectRatio: false,
		plugins: {
			legend: {
				labels: {
					color: colors.textSecondary,
					font: {
						family: ChartFonts.family,
						size: ChartFonts.size.sm,
					},
				},
			},
			tooltip: {
				backgroundColor: isDark ? colors.surface : '#ffffff',
				titleColor: colors.text,
				bodyColor: colors.textSecondary,
				borderColor: colors.border,
				borderWidth: 1,
				titleFont: {
					family: ChartFonts.family,
					size: ChartFonts.size.md,
					weight: '600',
				},
				bodyFont: {
					family: ChartFonts.family,
					size: ChartFonts.size.sm,
				},
			},
		},
		scales: {
			x: {
				grid: {
					color: colors.grid,
				},
				ticks: {
					color: colors.textMuted,
					font: {
						family: ChartFonts.family,
						size: ChartFonts.size.xs,
					},
				},
			},
			y: {
				grid: {
					color: colors.grid,
				},
				ticks: {
					color: colors.textMuted,
					font: {
						family: ChartFonts.family,
						size: ChartFonts.size.xs,
					},
				},
			},
		},
	};
}

export default {
	colors: ChartColors,
	colorsDark: ChartColorsDark,
	colorsLight: ChartColorsLight,
	fonts: ChartFonts,
	getChartColors,
	getLightweightChartsOptions,
	getAreaSeriesOptions,
	getLineSeriesOptions,
	getCandlestickSeriesOptions,
	getHistogramSeriesOptions,
	getApexChartsTheme,
	getChartJsTheme,
};

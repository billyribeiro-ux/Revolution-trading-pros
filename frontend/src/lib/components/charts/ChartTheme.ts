/**
 * ChartTheme - Unified chart theming for all chart libraries
 * Ensures consistent styling across lightweight-charts, ApexCharts, Chart.js
 * 
 * @version 2.0.0
 * @author Revolution Trading Pros
 */

export const ChartColors = {
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
 */
export function getApexChartsTheme() {
	return {
		chart: {
			background: 'transparent',
			foreColor: ChartColors.textSecondary,
			fontFamily: ChartFonts.family,
			toolbar: {
				show: false,
			},
			zoom: {
				enabled: false,
			},
		},
		colors: [ChartColors.primary, ChartColors.success, ChartColors.warning, ChartColors.info],
		grid: {
			borderColor: ChartColors.grid,
			strokeDashArray: 4,
		},
		stroke: {
			curve: 'smooth' as const,
			width: 2,
		},
		tooltip: {
			theme: 'dark',
			style: {
				fontSize: '12px',
				fontFamily: ChartFonts.family,
			},
		},
		xaxis: {
			axisBorder: { color: ChartColors.border },
			axisTicks: { color: ChartColors.border },
			labels: {
				style: {
					colors: ChartColors.textMuted,
					fontSize: '11px',
				},
			},
		},
		yaxis: {
			labels: {
				style: {
					colors: ChartColors.textMuted,
					fontSize: '11px',
				},
			},
		},
		legend: {
			labels: {
				colors: ChartColors.textSecondary,
			},
		},
	};
}

/**
 * Chart.js theme configuration
 */
export function getChartJsTheme() {
	return {
		responsive: true,
		maintainAspectRatio: false,
		plugins: {
			legend: {
				labels: {
					color: ChartColors.textSecondary,
					font: {
						family: ChartFonts.family,
						size: ChartFonts.size.sm,
					},
				},
			},
			tooltip: {
				backgroundColor: ChartColors.surface,
				titleColor: ChartColors.text,
				bodyColor: ChartColors.textSecondary,
				borderColor: ChartColors.border,
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
					color: ChartColors.grid,
				},
				ticks: {
					color: ChartColors.textMuted,
					font: {
						family: ChartFonts.family,
						size: ChartFonts.size.xs,
					},
				},
			},
			y: {
				grid: {
					color: ChartColors.grid,
				},
				ticks: {
					color: ChartColors.textMuted,
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
	fonts: ChartFonts,
	getLightweightChartsOptions,
	getAreaSeriesOptions,
	getLineSeriesOptions,
	getCandlestickSeriesOptions,
	getHistogramSeriesOptions,
	getApexChartsTheme,
	getChartJsTheme,
};

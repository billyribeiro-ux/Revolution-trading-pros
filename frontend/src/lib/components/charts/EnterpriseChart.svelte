<script lang="ts">
	/**
	 * EnterpriseChart - Google Analytics-style Chart Component
	 * Uses lightweight-charts for high-performance financial charts
	 *
	 * @version 1.0.0
	 * @author Revolution Trading Pros
	 * @level L8 Principal Engineer
	 */
	import { onMount, onDestroy } from 'svelte';
	import { browser } from '$app/environment';
	import { gsap } from 'gsap';
	import SkeletonLoader from '$lib/components/ui/SkeletonLoader.svelte';

	interface ChartDataPoint {
		time: string;
		value: number;
		open?: number;
		high?: number;
		low?: number;
		close?: number;
	}

	interface Props {
		type?: 'line' | 'area' | 'bar' | 'candlestick' | 'histogram';
		data?: ChartDataPoint[];
		title?: string;
		subtitle?: string;
		height?: number;
		loading?: boolean;
		showLegend?: boolean;
		showTooltip?: boolean;
		color?: string;
		gradientTopColor?: string;
		gradientBottomColor?: string;
		timeVisible?: boolean;
		priceLineVisible?: boolean;
		animateOnMount?: boolean;
		onhover?: (value: { time: string; value: number }) => void;
		controls?: import('svelte').Snippet;
	}

	let {
		type = 'area',
		data = [],
		title = '',
		subtitle = '',
		height = 300,
		loading = false,
		showLegend = true,
		showTooltip = true,
		color = '#6366f1',
		gradientTopColor = 'rgba(99, 102, 241, 0.4)',
		gradientBottomColor = 'rgba(99, 102, 241, 0.0)',
		timeVisible = true,
		priceLineVisible = true,
		animateOnMount = true,
		onhover,
		controls
	}: Props = $props();

	let chartContainer: HTMLDivElement | null = $state(null);
	let chart: unknown;
	let series: unknown;
	let currentValue: { time: string; value: number } | null = $state(null);

	async function initChart() {
		if (!browser || !chartContainer) return;

		const { createChart, ColorType, CrosshairMode, LineStyle } = await import('lightweight-charts');

		// Create chart with dark theme
		chart = createChart(chartContainer, {
			width: chartContainer.clientWidth,
			height,
			layout: {
				background: { type: ColorType.Solid, color: 'transparent' },
				textColor: '#94a3b8'
			},
			grid: {
				vertLines: { color: 'rgba(99, 102, 241, 0.1)' },
				horzLines: { color: 'rgba(99, 102, 241, 0.1)' }
			},
			crosshair: {
				mode: CrosshairMode.Normal,
				vertLine: {
					color: '#6366f1',
					width: 1,
					style: LineStyle.Dashed,
					labelBackgroundColor: '#6366f1'
				},
				horzLine: {
					color: '#6366f1',
					width: 1,
					style: LineStyle.Dashed,
					labelBackgroundColor: '#6366f1'
				}
			},
			rightPriceScale: {
				borderColor: 'rgba(99, 102, 241, 0.2)',
				visible: priceLineVisible
			},
			timeScale: {
				borderColor: 'rgba(99, 102, 241, 0.2)',
				timeVisible,
				secondsVisible: false
			},
			handleScale: {
				axisPressedMouseMove: true
			},
			handleScroll: {
				mouseWheel: true,
				pressedMouseMove: true,
				horzTouchDrag: true,
				vertTouchDrag: false
			}
		});

		// Create series based on type
		switch (type) {
			case 'area':
				series = (chart as any).addAreaSeries({
					lineColor: color,
					topColor: gradientTopColor,
					bottomColor: gradientBottomColor,
					lineWidth: 2
				});
				break;
			case 'line':
				series = (chart as any).addLineSeries({
					color,
					lineWidth: 2
				});
				break;
			case 'bar':
				series = (chart as any).addHistogramSeries({
					color,
					priceFormat: {
						type: 'volume'
					}
				});
				break;
			case 'candlestick':
				series = (chart as any).addCandlestickSeries({
					upColor: '#10b981',
					downColor: '#ef4444',
					borderUpColor: '#10b981',
					borderDownColor: '#ef4444',
					wickUpColor: '#10b981',
					wickDownColor: '#ef4444'
				});
				break;
			case 'histogram':
				series = (chart as any).addHistogramSeries({
					color,
					priceFormat: {
						type: 'volume'
					}
				});
				break;
		}

		// Set data
		if (data.length > 0) {
			if (type === 'candlestick') {
				(series as any).setData(
					data.map((d) => ({
						time: d.time,
						open: d.open ?? d.value,
						high: d.high ?? d.value,
						low: d.low ?? d.value,
						close: d.close ?? d.value
					}))
				);
			} else {
				(series as any).setData(data);
			}
		}

		// Fit content
		(chart as any).timeScale().fitContent();

		// Subscribe to crosshair move for tooltip
		if (showTooltip) {
			(chart as any).subscribeCrosshairMove((param: any) => {
				if (param.time && param.seriesData) {
					const value = param.seriesData.get(series);
					if (value) {
						currentValue = {
							time: param.time,
							value: value.value ?? value.close ?? 0
						};
						onhover?.(currentValue);
					}
				} else {
					currentValue = null;
				}
			});
		}

		// Handle resize
		const resizeObserver = new ResizeObserver((entries) => {
			for (const entry of entries) {
				const { width } = entry.contentRect;
				(chart as any).applyOptions({ width });
			}
		});
		resizeObserver.observe(chartContainer);

		// Animate chart on mount
		if (animateOnMount) {
			gsap.fromTo(
				chartContainer,
				{ opacity: 0, y: 20 },
				{ opacity: 1, y: 0, duration: 0.6, ease: 'power3.out' }
			);
		}

		return () => {
			resizeObserver.disconnect();
			if (chart) {
				(chart as any).remove();
			}
		};
	}

	// Update data reactively
	$effect(() => {
		if (browser && series && data.length > 0) {
		if (type === 'candlestick') {
			(series as any).setData(
				data.map((d) => ({
					time: d.time,
					open: d.open ?? d.value,
					high: d.high ?? d.value,
					low: d.low ?? d.value,
					close: d.close ?? d.value
				}))
			);
		} else {
			(series as any).setData(data);
		}
		if (chart) {
			(chart as any).timeScale().fitContent();
		}
		}
	});

	onMount(() => {
		initChart();
	});

	onDestroy(() => {
		if (chart) {
			(chart as any).remove();
		}
	});

	function formatValue(value: number): string {
		if (value >= 1000000) {
			return `$${(value / 1000000).toFixed(2)}M`;
		} else if (value >= 1000) {
			return `$${(value / 1000).toFixed(1)}K`;
		}
		return `$${value.toFixed(2)}`;
	}
</script>

<div class="enterprise-chart relative">
	<!-- Header -->
	{#if title || showLegend}
		<div class="flex items-start justify-between mb-4">
			<div>
				{#if title}
					<h3 class="text-lg font-semibold text-white">{title}</h3>
				{/if}
				{#if subtitle}
					<p class="text-sm text-slate-400">{subtitle}</p>
				{/if}
			</div>

			{#if showLegend && currentValue}
				<div class="text-right">
					<p class="text-2xl font-bold text-white">{formatValue(currentValue.value)}</p>
					<p class="text-xs text-slate-500">{currentValue.time}</p>
				</div>
			{/if}
		</div>
	{/if}

	<!-- Chart Container -->
	{#if loading}
		<SkeletonLoader variant="rectangular" width="100%" height="{height}px" />
	{:else}
		<div
			bind:this={chartContainer}
			class="chart-container rounded-xl overflow-hidden"
			style="height: {height}px;"
		></div>
	{/if}

	<!-- Chart Type Selector -->
	{@render controls?.()}
</div>

<style>
	.enterprise-chart {
		contain: layout style;
	}

	.chart-container {
		background: linear-gradient(to bottom, rgba(30, 41, 59, 0.5), rgba(15, 23, 42, 0.5));
	}

	/* Lightweight charts styling overrides */
	:global(.tv-lightweight-charts) {
		border-radius: 0.75rem;
	}
</style>

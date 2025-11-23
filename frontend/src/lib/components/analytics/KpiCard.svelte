<script lang="ts">
	/**
	 * KpiCard - Enterprise KPI Display Widget
	 *
	 * Displays a single KPI metric with trend indicator,
	 * change percentage, and optional sparkline.
	 */
	import { onMount } from 'svelte';
	import type { KpiValue } from '$lib/api/analytics';

	export let kpi: KpiValue;
	export let showSparkline: boolean = false;
	export let sparklineData: number[] = [];
	export let size: 'sm' | 'md' | 'lg' = 'md';
	export let clickable: boolean = false;

	// Icon mapping
	const iconMap: Record<string, string> = {
		'currency-dollar': '💰',
		'shopping-cart': '🛒',
		'users': '👥',
		'clock': '⏱️',
		'target': '🎯',
		'trending-up': '📈',
		'user-plus': '➕',
		'check-circle': '✅',
		'heart': '❤️',
		'crown': '👑',
		'star': '⭐'
	};

	// Color classes based on KPI color
	const colorClasses: Record<string, string> = {
		green: 'bg-green-500/10 text-green-600 border-green-500/20',
		blue: 'bg-blue-500/10 text-blue-600 border-blue-500/20',
		purple: 'bg-purple-500/10 text-purple-600 border-purple-500/20',
		orange: 'bg-orange-500/10 text-orange-600 border-orange-500/20',
		red: 'bg-red-500/10 text-red-600 border-red-500/20',
		pink: 'bg-pink-500/10 text-pink-600 border-pink-500/20',
		cyan: 'bg-cyan-500/10 text-cyan-600 border-cyan-500/20',
		gold: 'bg-yellow-500/10 text-yellow-600 border-yellow-500/20',
		gray: 'bg-gray-500/10 text-gray-600 border-gray-500/20'
	};

	$: trendClass = kpi.trend === 'up'
		? 'text-green-500'
		: kpi.trend === 'down'
			? 'text-red-500'
			: 'text-gray-500';

	$: trendIcon = kpi.trend === 'up' ? '↑' : kpi.trend === 'down' ? '↓' : '→';

	$: sizeClasses = {
		sm: 'p-3',
		md: 'p-4',
		lg: 'p-6'
	}[size];

	$: valueSizeClasses = {
		sm: 'text-xl',
		md: 'text-2xl',
		lg: 'text-4xl'
	}[size];

	// Simple sparkline SVG
	function generateSparklinePath(data: number[]): string {
		if (data.length < 2) return '';

		const width = 80;
		const height = 24;
		const min = Math.min(...data);
		const max = Math.max(...data);
		const range = max - min || 1;

		const points = data.map((value, index) => {
			const x = (index / (data.length - 1)) * width;
			const y = height - ((value - min) / range) * height;
			return `${x},${y}`;
		});

		return `M ${points.join(' L ')}`;
	}
</script>

<div
	class="relative rounded-xl border {colorClasses[kpi.color || 'blue']} {sizeClasses}
		transition-all duration-200 {clickable ? 'cursor-pointer hover:scale-[1.02] hover:shadow-lg' : ''}"
	class:ring-2={kpi.is_anomaly}
	class:ring-red-500={kpi.is_anomaly}
>
	<!-- Anomaly Badge -->
	{#if kpi.is_anomaly}
		<div class="absolute -top-2 -right-2 bg-red-500 text-white text-xs px-2 py-0.5 rounded-full">
			Anomaly
		</div>
	{/if}

	<!-- Header -->
	<div class="flex items-center justify-between mb-2">
		<div class="flex items-center gap-2">
			{#if kpi.icon}
				<span class="text-lg">{iconMap[kpi.icon] || '📊'}</span>
			{/if}
			<span class="text-sm font-medium opacity-80">{kpi.name}</span>
		</div>

		{#if kpi.is_primary}
			<span class="text-xs bg-white/20 px-2 py-0.5 rounded-full">Primary</span>
		{/if}
	</div>

	<!-- Value -->
	<div class="flex items-end gap-3">
		<span class="{valueSizeClasses} font-bold tracking-tight">
			{kpi.formatted_value}
		</span>

		<!-- Change Indicator -->
		<div class="flex items-center gap-1 pb-1 {trendClass}">
			<span class="text-sm font-medium">{trendIcon}</span>
			<span class="text-sm font-semibold">
				{Math.abs(kpi.change_percentage).toFixed(1)}%
			</span>
		</div>
	</div>

	<!-- Sparkline -->
	{#if showSparkline && sparklineData.length > 1}
		<div class="mt-3 opacity-60">
			<svg width="80" height="24" class="overflow-visible">
				<path
					d={generateSparklinePath(sparklineData)}
					fill="none"
					stroke="currentColor"
					stroke-width="1.5"
					stroke-linecap="round"
					stroke-linejoin="round"
				/>
			</svg>
		</div>
	{/if}

	<!-- Target Status -->
	{#if kpi.target_status}
		<div class="mt-2">
			<span class="text-xs px-2 py-0.5 rounded-full {
				kpi.target_status === 'on_track' ? 'bg-green-500/20 text-green-600' :
				kpi.target_status === 'at_risk' ? 'bg-yellow-500/20 text-yellow-600' :
				'bg-red-500/20 text-red-600'
			}">
				{kpi.target_status === 'on_track' ? 'On Track' :
				 kpi.target_status === 'at_risk' ? 'At Risk' : 'Behind'}
			</span>
		</div>
	{/if}
</div>

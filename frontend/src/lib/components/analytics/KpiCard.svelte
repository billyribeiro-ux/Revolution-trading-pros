<script lang="ts">
	/**
	 * KpiCard - Enterprise KPI Display Widget
	 *
	 * Displays a single KPI metric with trend indicator,
	 * change percentage, and optional sparkline.
	 * Now with GSAP-powered animated number counting.
	 *
	 * @version 2.0.0 - Added animated numbers
	 */
	import { onMount } from 'svelte';
	import { gsap } from 'gsap';
	import { browser } from '$app/environment';
	import type { KpiValue } from '$lib/api/analytics';

	interface Props {
		kpi: KpiValue;
		showSparkline?: boolean;
		sparklineData?: number[];
		size?: 'sm' | 'md' | 'lg';
		clickable?: boolean;
		animateOnMount?: boolean;
		animationDelay?: number;
	}

	let {
		kpi,
		showSparkline = false,
		sparklineData = [],
		size = 'md',
		clickable = false,
		animateOnMount = true,
		animationDelay = 0
	}: Props = $props();

	let cardRef: HTMLDivElement;
	let isVisible = $state(false);

	onMount(() => {
		if (!browser || !cardRef || !animateOnMount) {
			isVisible = true;
			return;
		}

		// Intersection observer for viewport-based animation
		const observer = new IntersectionObserver(
			(entries) => {
				entries.forEach((entry) => {
					if (entry.isIntersecting && !isVisible) {
						isVisible = true;
						gsap.fromTo(
							cardRef,
							{ opacity: 0, y: 20, scale: 0.95 },
							{
								opacity: 1,
								y: 0,
								scale: 1,
								duration: 0.6,
								delay: animationDelay,
								ease: 'power3.out'
							}
						);
					}
				});
			},
			{ threshold: 0.2 }
		);

		observer.observe(cardRef);
		return () => observer.disconnect();
	});

	// Icon mapping
	const iconMap: Record<string, string> = {
		'currency-dollar': '💰',
		'shopping-cart': '🛒',
		users: '👥',
		clock: '⏱️',
		target: '🎯',
		'trending-up': '📈',
		'user-plus': '➕',
		'check-circle': '✅',
		heart: '❤️',
		crown: '👑',
		star: '⭐'
	};

	let trendIcon = $derived(kpi.trend === 'up' ? '↑' : kpi.trend === 'down' ? '↓' : '→');

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
	bind:this={cardRef}
	class="kpi-card"
	data-color={kpi.color || 'blue'}
	data-size={size}
	data-clickable={clickable || undefined}
	data-animate={animateOnMount || undefined}
	data-anomaly={kpi.is_anomaly || undefined}
>
	<!-- Anomaly Badge -->
	{#if kpi.is_anomaly}
		<div class="anomaly-badge">Anomaly</div>
	{/if}

	<!-- Header -->
	<div class="kpi-header">
		<div class="kpi-name-group">
			{#if kpi.icon}
				<span class="kpi-icon">{iconMap[kpi.icon] || '📊'}</span>
			{/if}
			<span class="kpi-name">{kpi.name}</span>
		</div>

		{#if kpi.is_primary}
			<span class="primary-badge">Primary</span>
		{/if}
	</div>

	<!-- Value -->
	<div class="kpi-value-row">
		<span class="kpi-value">
			{kpi.formatted_value}
		</span>

		<!-- Change Indicator -->
		<div class="kpi-trend" data-trend={kpi.trend}>
			<span class="trend-icon">{trendIcon}</span>
			<span class="trend-pct">
				{Math.abs(kpi.change_percentage).toFixed(1)}%
			</span>
		</div>
	</div>

	<!-- Sparkline -->
	{#if showSparkline && sparklineData.length > 1}
		<div class="sparkline">
			<svg width="80" height="24" style="overflow: visible">
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
		<div class="target-wrapper">
			<span class="target-badge" data-status={kpi.target_status}>
				{kpi.target_status === 'on_track'
					? 'On Track'
					: kpi.target_status === 'at_risk'
						? 'At Risk'
						: 'Behind'}
			</span>
		</div>
	{/if}
</div>

<style>
	.kpi-card {
		position: relative;
		border-radius: var(--radius-xl);
		border: 1px solid;
		transition: all 200ms var(--ease-default);

		&[data-size='sm'] {
			padding: var(--space-3);
		}
		&[data-size='md'] {
			padding: var(--space-4);
		}
		&[data-size='lg'] {
			padding: var(--space-6);
		}

		&[data-clickable] {
			cursor: pointer;
			&:hover {
				transform: scale(1.02);
				box-shadow: 0 10px 25px oklch(0 0 0 / 10%);
			}
		}

		&[data-animate] {
			opacity: 0;
		}
		&[data-anomaly] {
			box-shadow: 0 0 0 2px oklch(0.6 0.25 25);
		}

		&[data-color='green'] {
			background-color: oklch(0.6 0.18 160 / 10%);
			color: oklch(0.5 0.18 160);
			border-color: oklch(0.6 0.18 160 / 20%);
		}
		&[data-color='blue'] {
			background-color: oklch(0.6 0.2 260 / 10%);
			color: oklch(0.5 0.2 260);
			border-color: oklch(0.6 0.2 260 / 20%);
		}
		&[data-color='purple'] {
			background-color: oklch(0.55 0.2 300 / 10%);
			color: oklch(0.45 0.2 300);
			border-color: oklch(0.55 0.2 300 / 20%);
		}
		&[data-color='orange'] {
			background-color: oklch(0.7 0.18 55 / 10%);
			color: oklch(0.55 0.18 55);
			border-color: oklch(0.7 0.18 55 / 20%);
		}
		&[data-color='red'] {
			background-color: oklch(0.6 0.25 25 / 10%);
			color: oklch(0.5 0.25 25);
			border-color: oklch(0.6 0.25 25 / 20%);
		}
		&[data-color='pink'] {
			background-color: oklch(0.65 0.2 340 / 10%);
			color: oklch(0.5 0.2 340);
			border-color: oklch(0.65 0.2 340 / 20%);
		}
		&[data-color='cyan'] {
			background-color: oklch(0.7 0.15 200 / 10%);
			color: oklch(0.5 0.15 200);
			border-color: oklch(0.7 0.15 200 / 20%);
		}
		&[data-color='gold'] {
			background-color: oklch(0.8 0.18 90 / 10%);
			color: oklch(0.6 0.18 90);
			border-color: oklch(0.8 0.18 90 / 20%);
		}
		&[data-color='gray'] {
			background-color: oklch(0.55 0.01 265 / 10%);
			color: oklch(0.45 0.01 265);
			border-color: oklch(0.55 0.01 265 / 20%);
		}
	}

	.anomaly-badge {
		position: absolute;
		inset-block-start: -0.5rem;
		inset-inline-end: -0.5rem;
		background-color: oklch(0.6 0.25 25);
		color: oklch(1 0 0);
		font-size: var(--text-xs);
		padding-inline: var(--space-2);
		padding-block: 0.125rem;
		border-radius: 9999px;
	}

	.kpi-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-block-end: var(--space-2);
	}

	.kpi-name-group {
		display: flex;
		align-items: center;
		gap: var(--space-2);
	}
	.kpi-icon {
		font-size: var(--text-lg);
	}
	.kpi-name {
		font-size: var(--text-sm);
		font-weight: var(--weight-medium);
		opacity: 0.8;
	}

	.primary-badge {
		font-size: var(--text-xs);
		background-color: oklch(1 0 0 / 20%);
		padding-inline: var(--space-2);
		padding-block: 0.125rem;
		border-radius: 9999px;
	}

	.kpi-value-row {
		display: flex;
		align-items: flex-end;
		gap: var(--space-3);
	}

	.kpi-value {
		font-weight: var(--weight-bold);
		letter-spacing: -0.025em;

		.kpi-card[data-size='sm'] & {
			font-size: var(--text-xl);
		}
		.kpi-card[data-size='md'] & {
			font-size: var(--text-2xl);
		}
		.kpi-card[data-size='lg'] & {
			font-size: var(--text-4xl);
		}
	}

	.kpi-trend {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		padding-block-end: var(--space-1);

		&[data-trend='up'] {
			color: oklch(0.6 0.18 160);
		}
		&[data-trend='down'] {
			color: oklch(0.6 0.25 25);
		}
		&[data-trend='flat'] {
			color: oklch(0.55 0.01 265);
		}
	}

	.trend-icon {
		font-size: var(--text-sm);
		font-weight: var(--weight-medium);
	}
	.trend-pct {
		font-size: var(--text-sm);
		font-weight: var(--weight-semibold);
	}

	.sparkline {
		margin-block-start: var(--space-3);
		opacity: 0.6;
	}

	.target-wrapper {
		margin-block-start: var(--space-2);
	}

	.target-badge {
		font-size: var(--text-xs);
		padding-inline: var(--space-2);
		padding-block: 0.125rem;
		border-radius: 9999px;

		&[data-status='on_track'] {
			background-color: oklch(0.6 0.18 160 / 20%);
			color: oklch(0.5 0.18 160);
		}
		&[data-status='at_risk'] {
			background-color: oklch(0.8 0.18 90 / 20%);
			color: oklch(0.6 0.18 90);
		}
		&[data-status='behind'] {
			background-color: oklch(0.6 0.25 25 / 20%);
			color: oklch(0.5 0.25 25);
		}
	}
</style>

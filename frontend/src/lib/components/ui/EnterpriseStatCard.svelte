<script lang="ts">
	/**
	 * EnterpriseStatCard - Google-style Animated Statistics Card
	 * Features animated number counting, staggered entrance, and skeleton loading
	 *
	 * @version 1.0.0
	 * @author Revolution Trading Pros
	 * @level L8 Principal Engineer
	 */
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import AnimatedNumber from './AnimatedNumber.svelte';
	import SkeletonLoader from './SkeletonLoader.svelte';
	import { Icon } from '$lib/icons';

	interface Props {
		title: string;
		value: number;
		format?: 'number' | 'currency' | 'percent' | 'compact';
		decimals?: number;
		prefix?: string;
		suffix?: string;
		trend?: number | null;
		trendLabel?: string;
		icon?: any;
		color?: 'blue' | 'green' | 'purple' | 'orange' | 'pink' | 'cyan' | 'red';
		loading?: boolean;
		delay?: number;
		clickable?: boolean;
		sparklineData?: number[];
		target?: number | null;
		targetLabel?: string;
		onclick?: () => void;
	}

	let {
		title,
		value,
		format = 'number',
		decimals = 0,
		prefix = '',
		suffix = '',
		trend = null,
		trendLabel = 'vs last period',
		icon = null,
		color = 'blue',
		loading = false,
		delay = 0,
		clickable = false,
		sparklineData = [],
		target = null,
		targetLabel = 'Target',
		onclick
	}: Props = $props();

	let cardRef: HTMLDivElement;
	let isVisible = $state(false);
	let hasAnimated = $state(false);

	let trendIsPositive = $derived(trend !== null && trend >= 0);
	let progressPercent = $derived(target ? Math.min((value / target) * 100, 100) : 0);

	function generateSparklinePath(data: number[]): string {
		if (data.length < 2) return '';
		const width = 100;
		const height = 30;
		const min = Math.min(...data);
		const max = Math.max(...data);
		const range = max - min || 1;

		const points = data.map((val, idx) => {
			const x = (idx / (data.length - 1)) * width;
			const y = height - ((val - min) / range) * height;
			return `${x},${y}`;
		});

		return `M ${points.join(' L ')}`;
	}

	function handleClick() {
		if (clickable) {
			onclick?.();
		}
	}

	function handleKeypress(e: KeyboardEvent) {
		if (clickable && (e.key === 'Enter' || e.key === ' ')) {
			onclick?.();
		}
	}

	onMount(() => {
		if (!browser || !cardRef) return;

		// Intersection observer for viewport-based animation
		const observer = new IntersectionObserver(
			(entries) => {
				entries.forEach((entry) => {
					if (entry.isIntersecting && !hasAnimated) {
						isVisible = true;
						hasAnimated = true;
					}
				});
			},
			{ threshold: 0.2 }
		);

		observer.observe(cardRef);

		return () => observer.disconnect();
	});
</script>

<!-- svelte-ignore a11y_no_noninteractive_tabindex -->
<div
	bind:this={cardRef}
	class="enterprise-stat-card"
	data-color={color}
	data-clickable={clickable || undefined}
	data-animated={hasAnimated || undefined}
	style:transition-delay="{delay}s"
	role={clickable ? 'button' : undefined}
	tabindex={clickable ? 0 : undefined}
	onclick={clickable ? handleClick : undefined}
	onkeypress={clickable ? handleKeypress : undefined}
>
	{#if loading}
		<SkeletonLoader variant="stat" />
	{:else}
		<!-- Header with icon -->
		<div class="stat-header">
			{#if icon}
				{@const iconStr = icon}
				<div class="stat-icon">
					<Icon icon={iconStr} size={24} />
				</div>
			{/if}

			{#if trend !== null}
				<div
					class="stat-trend"
					data-positive={trendIsPositive || undefined}
					data-negative={!trendIsPositive || undefined}
				>
					<span class="stat-trend-arrow">{trendIsPositive ? '↑' : '↓'}</span>
					<span>{Math.abs(trend).toFixed(1)}%</span>
				</div>
			{/if}
		</div>

		<!-- Title -->
		<p class="stat-title">{title}</p>

		<!-- Animated Value -->
		<div class="stat-value">
			{#if isVisible}
				<AnimatedNumber
					{value}
					{format}
					{decimals}
					{prefix}
					{suffix}
					delay={delay + 0.2}
					duration={1.5}
				/>
			{:else}
				<span class="stat-value-hidden">{prefix}0{suffix}</span>
			{/if}
		</div>

		<!-- Trend label -->
		{#if trend !== null}
			<p class="stat-trend-label">{trendLabel}</p>
		{/if}

		<!-- Target progress -->
		{#if target !== null}
			<div class="stat-target">
				<div class="stat-target-header">
					<span>{targetLabel}</span>
					<span>{Math.round(progressPercent)}%</span>
				</div>
				<div class="stat-progress-track">
					<div
						class="stat-progress-fill"
						style:width="{hasAnimated ? progressPercent : 0}%"
						style:transition-delay="{delay + 0.5}s"
					></div>
				</div>
			</div>
		{/if}

		<!-- Sparkline -->
		{#if sparklineData.length > 1}
			<div class="stat-sparkline">
				<svg width="100" height="30" class="stat-sparkline-svg" preserveAspectRatio="none">
					<path
						d={generateSparklinePath(sparklineData)}
						fill="none"
						stroke="currentColor"
						stroke-width="2"
						stroke-linecap="round"
						stroke-linejoin="round"
					/>
				</svg>
			</div>
		{/if}
	{/if}
</div>

<style>
	.enterprise-stat-card {
		position: relative;
		padding: var(--space-6);
		border-radius: var(--radius-xl);
		border: 1px solid;
		backdrop-filter: blur(16px);
		transition: all 500ms var(--ease-default);
		will-change: transform, opacity;
		opacity: 0;
		transform: translateY(30px) scale(0.95);

		&[data-animated] {
			opacity: 1;
			transform: translateY(0) scale(1);
		}

		&:hover {
			transform: translateY(-2px);
		}

		&[data-clickable] {
			cursor: pointer;
			&:hover {
				transform: scale(1.02);
				box-shadow: var(--shadow-xl);
			}
		}

		/* Color variants */
		&[data-color='blue'] {
			background: linear-gradient(
				to bottom right,
				oklch(0.55 0.2 260 / 20%),
				oklch(0.5 0.2 260 / 10%)
			);
			border-color: oklch(0.55 0.2 260 / 30%);
			--_accent: oklch(0.7 0.18 260);
			--_accent-bg: oklch(0.55 0.2 260 / 20%);
			--_gradient: linear-gradient(to right, oklch(0.55 0.2 260), oklch(0.5 0.2 260));
		}
		&[data-color='green'] {
			background: linear-gradient(
				to bottom right,
				oklch(0.6 0.18 160 / 20%),
				oklch(0.55 0.18 160 / 10%)
			);
			border-color: oklch(0.6 0.18 160 / 30%);
			--_accent: oklch(0.7 0.18 160);
			--_accent-bg: oklch(0.6 0.18 160 / 20%);
			--_gradient: linear-gradient(to right, oklch(0.6 0.18 160), oklch(0.55 0.18 160));
		}
		&[data-color='purple'] {
			background: linear-gradient(
				to bottom right,
				oklch(0.55 0.2 300 / 20%),
				oklch(0.5 0.2 300 / 10%)
			);
			border-color: oklch(0.55 0.2 300 / 30%);
			--_accent: oklch(0.7 0.18 300);
			--_accent-bg: oklch(0.55 0.2 300 / 20%);
			--_gradient: linear-gradient(to right, oklch(0.55 0.2 300), oklch(0.5 0.2 300));
		}
		&[data-color='orange'] {
			background: linear-gradient(
				to bottom right,
				oklch(0.65 0.2 55 / 20%),
				oklch(0.6 0.2 55 / 10%)
			);
			border-color: oklch(0.65 0.2 55 / 30%);
			--_accent: oklch(0.75 0.18 55);
			--_accent-bg: oklch(0.65 0.2 55 / 20%);
			--_gradient: linear-gradient(to right, oklch(0.65 0.2 55), oklch(0.6 0.2 55));
		}
		&[data-color='pink'] {
			background: linear-gradient(
				to bottom right,
				oklch(0.6 0.2 350 / 20%),
				oklch(0.55 0.2 350 / 10%)
			);
			border-color: oklch(0.6 0.2 350 / 30%);
			--_accent: oklch(0.72 0.18 350);
			--_accent-bg: oklch(0.6 0.2 350 / 20%);
			--_gradient: linear-gradient(to right, oklch(0.6 0.2 350), oklch(0.55 0.2 350));
		}
		&[data-color='cyan'] {
			background: linear-gradient(
				to bottom right,
				oklch(0.65 0.15 195 / 20%),
				oklch(0.6 0.15 195 / 10%)
			);
			border-color: oklch(0.65 0.15 195 / 30%);
			--_accent: oklch(0.75 0.13 195);
			--_accent-bg: oklch(0.65 0.15 195 / 20%);
			--_gradient: linear-gradient(to right, oklch(0.65 0.15 195), oklch(0.6 0.15 195));
		}
		&[data-color='red'] {
			background: linear-gradient(
				to bottom right,
				oklch(0.55 0.22 25 / 20%),
				oklch(0.5 0.22 25 / 10%)
			);
			border-color: oklch(0.55 0.22 25 / 30%);
			--_accent: oklch(0.7 0.2 25);
			--_accent-bg: oklch(0.55 0.22 25 / 20%);
			--_gradient: linear-gradient(to right, oklch(0.55 0.22 25), oklch(0.5 0.22 25));
		}
	}

	.stat-header {
		display: flex;
		align-items: flex-start;
		justify-content: space-between;
		margin-block-end: var(--space-4);
	}

	.stat-icon {
		inline-size: 3rem;
		block-size: 3rem;
		border-radius: var(--radius-lg);
		background-color: var(--_accent-bg);
		color: var(--_accent);
		display: flex;
		align-items: center;
		justify-content: center;
		transform-origin: center;
	}

	/* Lightweight icon "pop" without GSAP */
	[data-animated] .stat-icon {
		animation: esc-icon-pop 600ms cubic-bezier(0.175, 0.885, 0.32, 1.275);
		animation-delay: 0.3s;
		animation-fill-mode: both;
	}

	.stat-trend {
		display: flex;
		align-items: center;
		gap: var(--space-1);
		padding-inline: 0.625rem;
		padding-block: var(--space-1);
		border-radius: 9999px;
		font-size: var(--text-sm);
		font-weight: var(--weight-semibold);

		&[data-positive] {
			background-color: oklch(0.6 0.18 160 / 20%);
			color: oklch(0.7 0.18 160);
		}
		&[data-negative] {
			background-color: oklch(0.55 0.22 25 / 20%);
			color: oklch(0.7 0.2 25);
		}
	}

	.stat-trend-arrow {
		font-size: var(--text-xs);
	}

	.stat-title {
		font-size: var(--text-sm);
		font-weight: var(--weight-medium);
		color: oklch(0.65 0.01 250);
		margin-block-end: var(--space-1);
	}

	.stat-value {
		font-size: 1.875rem;
		font-weight: var(--weight-bold);
		color: oklch(1 0 0);
		margin-block-end: var(--space-2);
	}

	.stat-value-hidden {
		opacity: 0;
	}

	.stat-trend-label {
		font-size: var(--text-xs);
		color: oklch(0.55 0.01 250);
	}

	.stat-target {
		margin-block-start: var(--space-4);
	}

	.stat-target-header {
		display: flex;
		justify-content: space-between;
		font-size: var(--text-xs);
		color: oklch(0.65 0.01 250);
		margin-block-end: var(--space-1);
	}

	.stat-progress-track {
		block-size: 0.5rem;
		background-color: oklch(0.3 0.01 250 / 50%);
		border-radius: 9999px;
		overflow: hidden;
	}

	.stat-progress-fill {
		block-size: 100%;
		border-radius: 9999px;
		background: var(--_gradient);
		transition: width 900ms ease;
	}

	.stat-sparkline {
		margin-block-start: var(--space-4);
		opacity: 0.6;
		color: var(--_accent);
	}

	.stat-sparkline-svg {
		inline-size: 100%;
	}

	@keyframes esc-icon-pop {
		0% {
			transform: scale(0) rotate(-180deg);
		}
		100% {
			transform: scale(1) rotate(0);
		}
	}
</style>

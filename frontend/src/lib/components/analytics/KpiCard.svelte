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
	import Icon from '$lib/components/Icon.svelte';
	import * as Icons from '$lib/icons';

	type IconName = keyof typeof Icons;

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

	let cardRef = $state<HTMLDivElement | null>(null);
	let isVisible = $state(false);

	function captureCard(element: HTMLDivElement) {
		cardRef = element;

		return () => {
			if (cardRef === element) {
				cardRef = null;
			}
		};
	}

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

	// Icon mapping: KPI key → Tabler icon name from `$lib/icons`.
	const iconMap: Record<string, IconName> = {
		'currency-dollar': 'IconCurrencyDollar',
		'shopping-cart': 'IconShoppingCart',
		users: 'IconUsers',
		clock: 'IconClock',
		target: 'IconTarget',
		'trending-up': 'IconTrendingUp',
		'user-plus': 'IconUserPlus',
		'check-circle': 'IconCircleCheck',
		heart: 'IconHeart',
		crown: 'IconCrown',
		star: 'IconStar'
	};

	function resolveIcon(key: string | undefined | null): IconName {
		if (key && iconMap[key]) return iconMap[key];
		return 'IconChartBar';
	}

	const supportedColors = new Set([
		'green',
		'blue',
		'purple',
		'orange',
		'red',
		'pink',
		'cyan',
		'gold',
		'gray'
	]);

	function getColorClass(color: string | undefined | null): string {
		const normalized = color && supportedColors.has(color) ? color : 'blue';
		return `kpi-card--${normalized}`;
	}

	let trendClass = $derived(
		kpi.trend === 'up'
			? 'kpi-card__trend--positive'
			: kpi.trend === 'down'
				? 'kpi-card__trend--negative'
				: 'kpi-card__trend--neutral'
	);

	let trendIcon = $derived(kpi.trend === 'up' ? '↑' : kpi.trend === 'down' ? '↓' : '→');

	let sizeClasses = $derived(
		{
			sm: 'kpi-card--sm',
			md: 'kpi-card--md',
			lg: 'kpi-card--lg'
		}[size]
	);

	let valueSizeClasses = $derived(
		{
			sm: 'kpi-card__value--sm',
			md: 'kpi-card__value--md',
			lg: 'kpi-card__value--lg'
		}[size]
	);

	let targetStatusClass = $derived(
		kpi.target_status === 'on_track'
			? 'kpi-card__target--on-track'
			: kpi.target_status === 'at_risk'
				? 'kpi-card__target--at-risk'
				: 'kpi-card__target--behind'
	);

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
	{@attach captureCard}
	class={[
		'kpi-card',
		getColorClass(kpi.color),
		sizeClasses,
		clickable && 'kpi-card--clickable',
		animateOnMount && !isVisible && 'kpi-card--pending',
		kpi.is_anomaly && 'kpi-card--anomaly'
	]}
>
	<!-- Anomaly Badge -->
	{#if kpi.is_anomaly}
		<div class="kpi-card__anomaly-badge">Anomaly</div>
	{/if}

	<!-- Header -->
	<div class="kpi-card__header">
		<div class="kpi-card__heading">
			{#if kpi.icon}
				<span class="kpi-card-icon" aria-hidden="true">
					<Icon name={resolveIcon(kpi.icon)} size="md" />
				</span>
			{/if}
			<span class="kpi-card__name">{kpi.name}</span>
		</div>

		{#if kpi.is_primary}
			<span class="kpi-card__primary-badge">Primary</span>
		{/if}
	</div>

	<!-- Value -->
	<div class="kpi-card__metric">
		<span class={['kpi-card__value', valueSizeClasses]}>
			{kpi.formatted_value}
		</span>

		<!-- Change Indicator -->
		<div class={['kpi-card__trend', trendClass]}>
			<span class="kpi-card__trend-icon">{trendIcon}</span>
			<span class="kpi-card__trend-value">
				{Math.abs(kpi.change_percentage).toFixed(1)}%
			</span>
		</div>
	</div>

	<!-- Sparkline -->
	{#if showSparkline && sparklineData.length > 1}
		<div class="kpi-card__sparkline">
			<svg aria-hidden="true" width="80" height="24">
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
		<div class="kpi-card__target">
			<span class={['kpi-card__target-badge', targetStatusClass]}>
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
		border: 1px solid currentColor;
		border-radius: 0.75rem;
		transition:
			box-shadow 0.2s ease,
			transform 0.2s ease,
			opacity 0.2s ease;
	}

	.kpi-card--sm {
		padding: 0.75rem;
	}

	.kpi-card--md {
		padding: 1rem;
	}

	.kpi-card--lg {
		padding: 1.5rem;
	}

	.kpi-card--pending {
		opacity: 0;
	}

	.kpi-card--clickable {
		cursor: pointer;
	}

	.kpi-card--clickable:hover {
		box-shadow:
			0 10px 15px -3px rgb(0 0 0 / 0.1),
			0 4px 6px -4px rgb(0 0 0 / 0.1);
		transform: scale(1.02);
	}

	.kpi-card--anomaly {
		box-shadow: 0 0 0 2px #ef4444;
	}

	.kpi-card--green {
		border-color: rgb(34 197 94 / 0.2);
		background: rgb(34 197 94 / 0.1);
		color: #16a34a;
	}

	.kpi-card--blue {
		border-color: rgb(59 130 246 / 0.2);
		background: rgb(59 130 246 / 0.1);
		color: #2563eb;
	}

	.kpi-card--purple {
		border-color: rgb(168 85 247 / 0.2);
		background: rgb(168 85 247 / 0.1);
		color: #9333ea;
	}

	.kpi-card--orange {
		border-color: rgb(249 115 22 / 0.2);
		background: rgb(249 115 22 / 0.1);
		color: #ea580c;
	}

	.kpi-card--red {
		border-color: rgb(239 68 68 / 0.2);
		background: rgb(239 68 68 / 0.1);
		color: #dc2626;
	}

	.kpi-card--pink {
		border-color: rgb(236 72 153 / 0.2);
		background: rgb(236 72 153 / 0.1);
		color: #db2777;
	}

	.kpi-card--cyan {
		border-color: rgb(6 182 212 / 0.2);
		background: rgb(6 182 212 / 0.1);
		color: #0891b2;
	}

	.kpi-card--gold {
		border-color: rgb(234 179 8 / 0.2);
		background: rgb(234 179 8 / 0.1);
		color: #ca8a04;
	}

	.kpi-card--gray {
		border-color: rgb(107 114 128 / 0.2);
		background: rgb(107 114 128 / 0.1);
		color: #4b5563;
	}

	.kpi-card__anomaly-badge {
		position: absolute;
		top: -0.5rem;
		right: -0.5rem;
		border-radius: 9999px;
		background: #ef4444;
		color: #ffffff;
		font-size: 0.75rem;
		line-height: 1rem;
		padding: 0.125rem 0.5rem;
	}

	.kpi-card__header,
	.kpi-card__heading,
	.kpi-card__metric,
	.kpi-card__trend {
		display: flex;
	}

	.kpi-card__header {
		align-items: center;
		justify-content: space-between;
		gap: 0.75rem;
		margin-bottom: 0.5rem;
	}

	.kpi-card__heading {
		align-items: center;
		gap: 0.5rem;
		min-width: 0;
	}

	.kpi-card__name {
		font-size: 0.875rem;
		font-weight: 500;
		line-height: 1.25rem;
		opacity: 0.8;
	}

	.kpi-card__primary-badge,
	.kpi-card__target-badge {
		display: inline-flex;
		align-items: center;
		border-radius: 9999px;
		font-size: 0.75rem;
		line-height: 1rem;
		white-space: nowrap;
	}

	.kpi-card__primary-badge {
		background: rgb(255 255 255 / 0.2);
		padding: 0.125rem 0.5rem;
	}

	.kpi-card__metric {
		align-items: end;
		gap: 0.75rem;
	}

	.kpi-card__value {
		font-weight: 700;
		letter-spacing: 0;
		line-height: 1;
	}

	.kpi-card__value--sm {
		font-size: 1.25rem;
	}

	.kpi-card__value--md {
		font-size: 1.5rem;
	}

	.kpi-card__value--lg {
		font-size: 2.25rem;
	}

	.kpi-card__trend {
		align-items: center;
		gap: 0.25rem;
		padding-bottom: 0.25rem;
	}

	.kpi-card__trend--positive {
		color: #22c55e;
	}

	.kpi-card__trend--negative {
		color: #ef4444;
	}

	.kpi-card__trend--neutral {
		color: #6b7280;
	}

	.kpi-card__trend-icon {
		font-size: 0.875rem;
		font-weight: 500;
		line-height: 1.25rem;
	}

	.kpi-card__trend-value {
		font-size: 0.875rem;
		font-weight: 600;
		line-height: 1.25rem;
	}

	.kpi-card__sparkline {
		margin-top: 0.75rem;
		opacity: 0.6;
	}

	.kpi-card__sparkline svg {
		overflow: visible;
	}

	.kpi-card__target {
		margin-top: 0.5rem;
	}

	.kpi-card__target-badge {
		padding: 0.125rem 0.5rem;
	}

	.kpi-card__target--on-track {
		background: rgb(34 197 94 / 0.2);
		color: #16a34a;
	}

	.kpi-card__target--at-risk {
		background: rgb(234 179 8 / 0.2);
		color: #ca8a04;
	}

	.kpi-card__target--behind {
		background: rgb(239 68 68 / 0.2);
		color: #dc2626;
	}
</style>

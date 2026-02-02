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

	let props: Props = $props();
	let title = $derived(props.title);
	let value = $derived(props.value);
	let format = $derived(props.format ?? 'number');
	let decimals = $derived(props.decimals ?? 0);
	let prefix = $derived(props.prefix ?? '');
	let suffix = $derived(props.suffix ?? '');
	let trend = $derived(props.trend ?? null);
	let trendLabel = $derived(props.trendLabel ?? 'vs last period');
	let icon = $derived(props.icon ?? null);
	let color = $derived(props.color ?? 'blue');
	let loading = $derived(props.loading ?? false);
	let delay = $derived(props.delay ?? 0);
	let clickable = $derived(props.clickable ?? false);
	let sparklineData = $derived(props.sparklineData ?? []);
	let target = $derived(props.target ?? null);
	let targetLabel = $derived(props.targetLabel ?? 'Target');
	let onclick = $derived(props.onclick);

	let cardRef: HTMLDivElement;
	let isVisible = $state(false);
	let hasAnimated = $state(false);

	const colorConfig = {
		blue: {
			bg: 'from-blue-500/20 to-blue-600/10',
			border: 'border-blue-500/30',
			icon: 'text-blue-400',
			iconBg: 'bg-blue-500/20',
			glow: 'shadow-blue-500/20',
			gradient: 'from-blue-500 to-blue-600'
		},
		green: {
			bg: 'from-emerald-500/20 to-emerald-600/10',
			border: 'border-emerald-500/30',
			icon: 'text-emerald-400',
			iconBg: 'bg-emerald-500/20',
			glow: 'shadow-emerald-500/20',
			gradient: 'from-emerald-500 to-emerald-600'
		},
		purple: {
			bg: 'from-purple-500/20 to-purple-600/10',
			border: 'border-purple-500/30',
			icon: 'text-purple-400',
			iconBg: 'bg-purple-500/20',
			glow: 'shadow-purple-500/20',
			gradient: 'from-purple-500 to-purple-600'
		},
		orange: {
			bg: 'from-orange-500/20 to-orange-600/10',
			border: 'border-orange-500/30',
			icon: 'text-orange-400',
			iconBg: 'bg-orange-500/20',
			glow: 'shadow-orange-500/20',
			gradient: 'from-orange-500 to-orange-600'
		},
		pink: {
			bg: 'from-pink-500/20 to-pink-600/10',
			border: 'border-pink-500/30',
			icon: 'text-pink-400',
			iconBg: 'bg-pink-500/20',
			glow: 'shadow-pink-500/20',
			gradient: 'from-pink-500 to-pink-600'
		},
		cyan: {
			bg: 'from-cyan-500/20 to-cyan-600/10',
			border: 'border-cyan-500/30',
			icon: 'text-cyan-400',
			iconBg: 'bg-cyan-500/20',
			glow: 'shadow-cyan-500/20',
			gradient: 'from-cyan-500 to-cyan-600'
		},
		red: {
			bg: 'from-red-500/20 to-red-600/10',
			border: 'border-red-500/30',
			icon: 'text-red-400',
			iconBg: 'bg-red-500/20',
			glow: 'shadow-red-500/20',
			gradient: 'from-red-500 to-red-600'
		}
	};

	let colors = $derived(colorConfig[color]);
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
	class="enterprise-stat-card relative p-6 rounded-2xl border bg-gradient-to-br {colors.bg} {colors.border}
		backdrop-blur-xl transition-all duration-500
		{!hasAnimated ? 'esc-enter' : ''} {hasAnimated ? 'esc-enter-active' : ''}
		{clickable ? 'cursor-pointer hover:scale-[1.02] hover:shadow-xl' : ''}
		hover:{colors.glow}"
	style={`transition-delay: ${delay}s`}
	role={clickable ? 'button' : undefined}
	tabindex={clickable ? 0 : undefined}
	onclick={clickable ? handleClick : undefined}
	onkeypress={clickable ? handleKeypress : undefined}
>
	{#if loading}
		<SkeletonLoader variant="stat" />
	{:else}
		<!-- Header with icon -->
		<div class="flex items-start justify-between mb-4">
			{#if icon}
				{@const IconComponent = icon}
				<div
					class="stat-icon w-12 h-12 rounded-xl {colors.iconBg} flex items-center justify-center"
				>
					<IconComponent size={24} class={colors.icon} />
				</div>
			{/if}

			{#if trend !== null}
				<div
					class="flex items-center gap-1 px-2.5 py-1 rounded-full text-sm font-semibold
						{trendIsPositive ? 'bg-emerald-500/20 text-emerald-400' : 'bg-red-500/20 text-red-400'}"
				>
					<span class="text-xs">{trendIsPositive ? '↑' : '↓'}</span>
					<span>{Math.abs(trend).toFixed(1)}%</span>
				</div>
			{/if}
		</div>

		<!-- Title -->
		<p class="text-sm font-medium text-slate-400 mb-1">{title}</p>

		<!-- Animated Value -->
		<div class="text-3xl font-bold text-white mb-2">
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
				<span class="opacity-0">{prefix}0{suffix}</span>
			{/if}
		</div>

		<!-- Trend label -->
		{#if trend !== null}
			<p class="text-xs text-slate-500">{trendLabel}</p>
		{/if}

		<!-- Target progress -->
		{#if target !== null}
			<div class="mt-4">
				<div class="flex justify-between text-xs text-slate-400 mb-1">
					<span>{targetLabel}</span>
					<span>{Math.round(progressPercent)}%</span>
				</div>
				<div class="h-2 bg-slate-700/50 rounded-full overflow-hidden">
					<div
						class="progress-fill h-full rounded-full bg-gradient-to-r {colors.gradient}"
						style={`width: ${hasAnimated ? progressPercent : 0}%; transition: width 900ms ease; transition-delay: ${delay + 0.5}s;`}
					></div>
				</div>
			</div>
		{/if}

		<!-- Sparkline -->
		{#if sparklineData.length > 1}
			<div class="mt-4 opacity-60">
				<svg width="100" height="30" class="w-full" preserveAspectRatio="none">
					<path
						d={generateSparklinePath(sparklineData)}
						fill="none"
						stroke="currentColor"
						stroke-width="2"
						stroke-linecap="round"
						stroke-linejoin="round"
						class={colors.icon}
					/>
				</svg>
			</div>
		{/if}
	{/if}
</div>

<style>
	.enterprise-stat-card {
		will-change: transform, opacity;
	}

	.enterprise-stat-card:hover {
		transform: translateY(-2px);
	}

	.esc-enter {
		opacity: 0;
		transform: translateY(30px) scale(0.95);
	}

	.esc-enter-active {
		opacity: 1;
		transform: translateY(0) scale(1);
	}

	.stat-icon {
		transform-origin: center;
	}

	/* Lightweight icon “pop” without GSAP */
	.esc-enter-active .stat-icon {
		animation: esc-icon-pop 600ms cubic-bezier(0.175, 0.885, 0.32, 1.275);
		animation-delay: 0.3s;
		animation-fill-mode: both;
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

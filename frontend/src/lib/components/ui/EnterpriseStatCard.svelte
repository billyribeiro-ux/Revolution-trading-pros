<script lang="ts">
	/**
	 * EnterpriseStatCard - Google-style Animated Statistics Card
	 * Features animated number counting, staggered entrance, and skeleton loading
	 *
	 * @version 1.0.0
	 * @author Revolution Trading Pros
	 * @level L8 Principal Engineer
	 */
	import { onMount, createEventDispatcher } from 'svelte';
	import { gsap } from 'gsap';
	import { browser } from '$app/environment';
	import AnimatedNumber from './AnimatedNumber.svelte';
	import SkeletonLoader from './SkeletonLoader.svelte';

	const dispatch = createEventDispatcher();

	// Props
	export let title: string;
	export let value: number;
	export let format: 'number' | 'currency' | 'percent' | 'compact' = 'number';
	export let decimals: number = 0;
	export let prefix: string = '';
	export let suffix: string = '';
	export let trend: number | null = null; // Percentage change
	export let trendLabel: string = 'vs last period';
	export let icon: any = null;
	export let color: 'blue' | 'green' | 'purple' | 'orange' | 'pink' | 'cyan' | 'red' = 'blue';
	export let loading: boolean = false;
	export let delay: number = 0;
	export let clickable: boolean = false;
	export let sparklineData: number[] = [];
	export let target: number | null = null;
	export let targetLabel: string = 'Target';

	let cardRef: HTMLDivElement;
	let isVisible = false;
	let hasAnimated = false;

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

	$: colors = colorConfig[color];
	$: trendIsPositive = trend !== null && trend >= 0;
	$: progressPercent = target ? Math.min((value / target) * 100, 100) : 0;

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
			dispatch('click');
		}
	}

	function handleKeypress(e: KeyboardEvent) {
		if (clickable && (e.key === 'Enter' || e.key === ' ')) {
			dispatch('click');
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

						// Entrance animation
						gsap.fromTo(
							cardRef,
							{
								opacity: 0,
								y: 30,
								scale: 0.95
							},
							{
								opacity: 1,
								y: 0,
								scale: 1,
								duration: 0.8,
								delay: delay,
								ease: 'power3.out'
							}
						);

						// Icon bounce animation
						const iconEl = cardRef.querySelector('.stat-icon');
						if (iconEl) {
							gsap.fromTo(
								iconEl,
								{ scale: 0, rotation: -180 },
								{
									scale: 1,
									rotation: 0,
									duration: 0.6,
									delay: delay + 0.3,
									ease: 'back.out(1.7)'
								}
							);
						}

						// Progress bar animation
						const progressBar = cardRef.querySelector('.progress-fill');
						if (progressBar) {
							gsap.fromTo(
								progressBar,
								{ width: 0 },
								{
									width: `${progressPercent}%`,
									duration: 1,
									delay: delay + 0.5,
									ease: 'power2.out'
								}
							);
						}
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
		backdrop-blur-xl transition-all duration-300 opacity-0
		{clickable ? 'cursor-pointer hover:scale-[1.02] hover:shadow-xl' : ''}
		hover:{colors.glow}"
	role={clickable ? 'button' : undefined}
	tabindex={clickable ? 0 : undefined}
	on:click={clickable ? handleClick : undefined}
	on:keypress={clickable ? handleKeypress : undefined}
>
	{#if loading}
		<SkeletonLoader variant="stat" />
	{:else}
		<!-- Header with icon -->
		<div class="flex items-start justify-between mb-4">
			{#if icon}
				<div
					class="stat-icon w-12 h-12 rounded-xl {colors.iconBg} flex items-center justify-center"
				>
					<svelte:component this={icon} size={24} class={colors.icon} />
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
						style="width: 0%;"
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
</style>

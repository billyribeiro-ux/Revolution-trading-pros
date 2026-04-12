<script lang="ts">
	import { browser } from '$app/environment';
	import { onMount } from 'svelte';
	import AnimatedNumber from './AnimatedNumber.svelte';

	interface Props {
		label: string;
		value: number;
		decimals?: number;
		prefix?: string;
		suffix?: string;
		tooltip?: string;
		colorize?: boolean;
		class?: string;
	}

	let {
		label,
		value,
		decimals = 4,
		prefix = '',
		suffix = '',
		tooltip = '',
		colorize = true,
		class: className = ''
	}: Props = $props();

	let metricEl: HTMLDivElement | undefined = $state();
	let gsapInstance: typeof import('gsap').default | undefined;

	// --- Ring buffer for sparkline (last 10 values) ---
	let history: number[] = $state([]);
	let prevValue: number | null = $state(null);

	let sparklinePath = $derived.by(() => {
		if (history.length < 2) return '';
		const min = Math.min(...history);
		const max = Math.max(...history);
		const range = max - min || 1;
		const points = history.map((v, i) => {
			const x = (i / (history.length - 1)) * 56 + 2;
			const y = 18 - ((v - min) / range) * 16;
			return `${x},${y}`;
		});
		return points.join(' ');
	});

	let sparklineColor = $derived(
		history.length >= 2
			? history[history.length - 1] >= history[0]
				? '#22c55e'
				: '#ef4444'
			: '#6366f1'
	);

	let glowClass = $derived(
		colorize ? (value > 0.0001 ? 'metric-positive' : value < -0.0001 ? 'metric-negative' : '') : ''
	);

	// --- Track value changes for sparkline + GSAP bounce ---
	$effect(() => {
		const v = value;
		if (prevValue === null || v !== prevValue) {
			history = [...history.slice(-(10 - 1)), v];
			// GSAP spring bounce
			if (prevValue !== null && gsapInstance && metricEl) {
				gsapInstance.killTweensOf(metricEl);
				gsapInstance.fromTo(
					metricEl,
					{ scale: 1 },
					{ scale: 1.05, duration: 0.3, ease: 'back.out(2)', yoyo: true, repeat: 1 }
				);
			}
			prevValue = v;
		}
	});

	onMount(() => {
		if (!browser) return;
		(async () => {
			gsapInstance = (await import('gsap')).default;
		})();
	});
</script>

<div
	bind:this={metricEl}
	class="relative flex flex-col gap-1 rounded-xl px-3 py-2.5 transition-all {glowClass} {className}"
	style="background: var(--calc-surface); border: 1px solid var(--calc-border);"
	title={tooltip}
>
	<span
		class="text-[10px] font-medium uppercase tracking-wider"
		style="color: var(--calc-text-muted); font-family: var(--calc-font-body);"
	>
		{label}
	</span>
	<AnimatedNumber {value} {decimals} {prefix} {suffix} {colorize} class="text-sm font-semibold" />
	{#if history.length >= 2}
		<svg
			class="metric-sparkline"
			width="60"
			height="20"
			viewBox="0 0 60 20"
			fill="none"
			xmlns="http://www.w3.org/2000/svg"
		>
			<polyline
				points={sparklinePath}
				stroke={sparklineColor}
				stroke-width="1.5"
				stroke-linecap="round"
				stroke-linejoin="round"
				fill="none"
				opacity="0.8"
			/>
		</svg>
	{/if}
</div>

<style>
	.metric-sparkline {
		margin-top: 2px;
		opacity: 0.7;
		transition: opacity 0.3s ease;
	}

	div:hover .metric-sparkline {
		opacity: 1;
	}
</style>

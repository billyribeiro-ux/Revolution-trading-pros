<script lang="ts">
	import { prefersReducedMotion } from 'svelte/motion';
	import type { Attachment } from 'svelte/attachments';
	import type { ClassValue } from 'svelte/elements';
	import AnimatedNumber from './AnimatedNumber.svelte';

	interface Props {
		label: string;
		value: number;
		decimals?: number;
		prefix?: string;
		suffix?: string;
		tooltip?: string;
		colorize?: boolean;
		class?: ClassValue;
	}

	let {
		label,
		value,
		decimals = 4,
		prefix = '',
		suffix = '',
		tooltip = '',
		colorize = true,
		class: className
	}: Props = $props();

	/** Values within ±EPSILON read as zero and get no glow. */
	const EPSILON = 0.0001;

	// ── Sparkline: the last SPARKLINE_POINTS distinct values ──
	const SPARKLINE_POINTS = 10;
	const SPARKLINE_WIDTH = 60;
	const SPARKLINE_HEIGHT = 20;

	// A history is memory over time, which a pure derivation cannot hold, so the derived
	// keeps a private buffer. It re-runs only when `value` changes, appending each distinct
	// value exactly once — no effect writing state, no extra render pass, and the first
	// point is already there on the server.
	let buffer: number[] = [];
	const history = $derived.by(() => {
		const current = value;
		if (buffer[buffer.length - 1] !== current) {
			buffer = [...buffer.slice(-(SPARKLINE_POINTS - 1)), current];
		}
		return buffer;
	});

	const sparklinePoints = $derived.by(() => {
		if (history.length < 2) return '';
		const min = Math.min(...history);
		const max = Math.max(...history);
		const range = max - min || 1;
		const step = (SPARKLINE_WIDTH - 4) / (history.length - 1);
		return history
			.map((v, i) => `${2 + i * step},${SPARKLINE_HEIGHT - 2 - ((v - min) / range) * 16}`)
			.join(' ');
	});

	const sparklineColor = $derived(
		history.length >= 2
			? history[history.length - 1] >= history[0]
				? 'var(--calc-call)'
				: 'var(--calc-put)'
			: 'var(--calc-accent)'
	);

	/** Spring bounce whenever the value changes after the first render. */
	const bounceOnChange: Attachment<HTMLDivElement> = (element) => {
		let seen: number | undefined;
		let bounce: Animation | undefined;

		$effect(() => {
			const current = value;
			const before = seen;
			seen = current;
			if (before === undefined || current === before || prefersReducedMotion.current) return;

			bounce?.cancel();
			bounce = element.animate(
				[{ transform: 'scale(1)' }, { transform: 'scale(1.05)' }, { transform: 'scale(1)' }],
				{ duration: 600, easing: 'cubic-bezier(0.34, 1.56, 0.64, 1)' }
			);
		});

		return () => bounce?.cancel();
	};
</script>

<div
	{@attach bounceOnChange}
	class={[
		'relative flex flex-col gap-1 rounded-xl px-3 py-2.5 transition-all motion-reduce:transition-none',
		colorize && { 'metric-positive': value > EPSILON, 'metric-negative': value < -EPSILON },
		className
	]}
	style="background: var(--calc-surface); border: 1px solid var(--calc-border);"
	title={tooltip || undefined}
>
	<span
		class="text-[10px] font-medium uppercase tracking-wider"
		style="color: var(--calc-text-muted); font-family: var(--calc-font-body);"
	>
		{label}
	</span>
	<AnimatedNumber {value} {decimals} {prefix} {suffix} {colorize} class="text-sm font-semibold" />
	<!-- Space is reserved before the second value arrives so the grid never shifts. -->
	<svg
		class="metric-sparkline"
		width={SPARKLINE_WIDTH}
		height={SPARKLINE_HEIGHT}
		viewBox="0 0 {SPARKLINE_WIDTH} {SPARKLINE_HEIGHT}"
		fill="none"
		aria-hidden="true"
	>
		{#if sparklinePoints}
			<polyline
				points={sparklinePoints}
				style:stroke={sparklineColor}
				stroke-width="1.5"
				stroke-linecap="round"
				stroke-linejoin="round"
				fill="none"
			/>
		{/if}
	</svg>
</div>

<style>
	.metric-sparkline {
		margin-top: 2px;
		opacity: 0.7;
		transition: opacity 0.3s ease;
	}

	div:hover > .metric-sparkline {
		opacity: 1;
	}
</style>

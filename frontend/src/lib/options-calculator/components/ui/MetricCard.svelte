<script lang="ts">
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
		class: className = '',
	}: Props = $props();

	let glowClass = $derived(
		colorize
			? value > 0.0001
				? 'metric-positive'
				: value < -0.0001
					? 'metric-negative'
					: ''
			: ''
	);
</script>

<div
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
	<AnimatedNumber
		{value}
		{decimals}
		{prefix}
		{suffix}
		{colorize}
		class="text-sm font-semibold"
	/>
</div>

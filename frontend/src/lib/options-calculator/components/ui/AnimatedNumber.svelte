<script lang="ts">
	interface Props {
		value: number;
		decimals?: number;
		prefix?: string;
		suffix?: string;
		duration?: number;
		colorize?: boolean;
		class?: string;
	}

	let {
		value,
		decimals = 2,
		prefix = '',
		suffix = '',
		colorize = false,
		class: className = ''
	}: Props = $props();

	let colorClass = $derived(
		colorize ? (value > 0.0001 ? 'text-call' : value < -0.0001 ? 'text-put' : '') : ''
	);
	let displayText = $derived(`${prefix}${value.toFixed(decimals)}${suffix}`);
</script>

<span class={['number-display', colorClass, className]}>
	{displayText}
</span>

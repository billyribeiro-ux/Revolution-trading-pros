<script lang="ts">
	/**
	 * AnimatedNumber - Enterprise Animated Number Counter
	 * Google Analytics-style smooth number counting animation
	 *
	 * @version 1.0.0
	 * @author Revolution Trading Pros
	 * @level L8 Principal Engineer
	 */
	import { untrack } from 'svelte';
	import { Tween } from 'svelte/motion';
	import { cubicOut, linear, quadOut, sineOut } from 'svelte/easing';

	interface Props {
		value: number;
		duration?: number;
		format?: 'number' | 'currency' | 'percent' | 'compact';
		decimals?: number;
		prefix?: string;
		suffix?: string;
		locale?: string;
		currency?: string;
		delay?: number;
		easing?: string;
	}

	let props: Props = $props();
	let value = $derived(props.value);
	let duration = $derived(props.duration ?? 1.5);
	let format = $derived(props.format ?? 'number');
	let decimals = $derived(props.decimals ?? 0);
	let prefix = $derived(props.prefix ?? '');
	let suffix = $derived(props.suffix ?? '');
	let locale = $derived(props.locale ?? 'en-US');
	let currency = $derived(props.currency ?? 'USD');
	let easing = $derived(props.easing ?? 'power2.out');
	const initialDelay = untrack(() => props.delay ?? 0);

	const easingMap = {
		linear,
		'power1.out': sineOut,
		'power2.out': quadOut,
		'power3.out': cubicOut
	};

	let tween = Tween.of(() => value, {
		delay: initialDelay * 1000,
		duration: () => duration * 1000,
		easing: (t) => (easingMap[easing as keyof typeof easingMap] ?? quadOut)(t)
	});

	function formatValue(val: number): string {
		let formatted: string;

		switch (format) {
			case 'currency':
				formatted = new Intl.NumberFormat(locale, {
					style: 'currency',
					currency,
					minimumFractionDigits: decimals,
					maximumFractionDigits: decimals
				}).format(val);
				break;
			case 'percent':
				formatted = new Intl.NumberFormat(locale, {
					style: 'percent',
					minimumFractionDigits: decimals,
					maximumFractionDigits: decimals
				}).format(val / 100);
				break;
			case 'compact':
				formatted = new Intl.NumberFormat(locale, {
					notation: 'compact',
					compactDisplay: 'short',
					minimumFractionDigits: decimals,
					maximumFractionDigits: decimals
				}).format(val);
				break;
			default:
				formatted = new Intl.NumberFormat(locale, {
					minimumFractionDigits: decimals,
					maximumFractionDigits: decimals
				}).format(val);
		}

		return `${prefix}${formatted}${suffix}`;
	}
</script>

<span class="animated-number tabular-nums" aria-live="polite" aria-atomic="true">
	{formatValue(tween.current)}
</span>

<style>
	.animated-number {
		font-variant-numeric: tabular-nums;
		font-feature-settings: 'tnum';
	}
</style>

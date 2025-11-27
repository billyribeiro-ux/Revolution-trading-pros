<script lang="ts">
	/**
	 * AnimatedNumber - Enterprise Animated Number Counter
	 * Google Analytics-style smooth number counting animation
	 *
	 * @version 1.0.0
	 * @author Revolution Trading Pros
	 * @level L8 Principal Engineer
	 */
	import { onMount, onDestroy } from 'svelte';
	import { gsap } from 'gsap';
	import { browser } from '$app/environment';

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

	let {
		value,
		duration = 1.5,
		format = 'number',
		decimals = 0,
		prefix = '',
		suffix = '',
		locale = 'en-US',
		currency = 'USD',
		delay = 0,
		easing = 'power2.out'
	}: Props = $props();

	let displayValue = $state(0);
	let element: HTMLSpanElement;
	let tween: gsap.core.Tween | null = null;

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

	function animate(targetValue: number) {
		if (!browser) return;

		// Kill any existing tween
		if (tween) {
			tween.kill();
		}

		// Create the counting animation
		const obj = { val: displayValue };
		tween = gsap.to(obj, {
			val: targetValue,
			duration,
			delay,
			ease: easing,
			onUpdate: () => {
				displayValue = obj.val;
			},
			onComplete: () => {
				displayValue = targetValue;
			}
		});
	}

	onMount(() => {
		// Initial animation from 0 to value
		displayValue = 0;
		animate(value);
	});

	onDestroy(() => {
		if (tween) {
			tween.kill();
		}
	});

	// Reactively animate when value changes
	$effect(() => {
		if (browser && value !== undefined) {
			animate(value);
		}
	});
</script>

<span
	bind:this={element}
	class="animated-number tabular-nums"
	aria-live="polite"
	aria-atomic="true"
>
	{formatValue(displayValue)}
</span>

<style>
	.animated-number {
		font-variant-numeric: tabular-nums;
		font-feature-settings: 'tnum';
	}
</style>

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

	export let value: number;
	export let duration: number = 1.5;
	export let format: 'number' | 'currency' | 'percent' | 'compact' = 'number';
	export let decimals: number = 0;
	export let prefix: string = '';
	export let suffix: string = '';
	export let locale: string = 'en-US';
	export let currency: string = 'USD';
	export let delay: number = 0;
	export let easing: string = 'power2.out';

	let displayValue = 0;
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
	$: if (browser && value !== undefined) {
		animate(value);
	}
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

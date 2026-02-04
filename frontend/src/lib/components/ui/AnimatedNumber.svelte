<script lang="ts">
	/**
	 * AnimatedNumber - Enterprise Animated Number Counter
	 * Google Analytics-style smooth number counting animation
	 *
	 * @version 1.0.0
	 * @author Revolution Trading Pros
	 * @level L8 Principal Engineer
	 */
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

	let props: Props = $props();
	let value = $derived(props.value);
	let duration = $derived(props.duration ?? 1.5);
	let format = $derived(props.format ?? 'number');
	let decimals = $derived(props.decimals ?? 0);
	let prefix = $derived(props.prefix ?? '');
	let suffix = $derived(props.suffix ?? '');
	let locale = $derived(props.locale ?? 'en-US');
	let currency = $derived(props.currency ?? 'USD');
	let delay = $derived(props.delay ?? 0);
	let easing = $derived(props.easing ?? 'power2.out');

	let displayValue = $state(0);
	let _element: HTMLSpanElement;
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

	// Initialize and reactively animate when value changes
	$effect(() => {
		if (!browser) return;

		// Initial animation from 0 to value, and reactive updates
		if (value !== undefined) {
			animate(value);
		}

		// Cleanup: kill tween on destroy
		return () => {
			if (tween) {
				tween.kill();
			}
		};
	});
</script>

<span
	bind:this={_element}
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

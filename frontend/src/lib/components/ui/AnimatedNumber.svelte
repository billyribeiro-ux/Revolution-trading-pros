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
	import { browser } from '$app/environment';
	import { cubicOut } from 'svelte/easing';

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
	let rafId: number | null = null;

	function resolveEasingFn(name: string): (t: number) => number {
		// GSAP-like names mapped to a small set of lightweight easing functions.
		// We intentionally keep this small to avoid pulling in any heavy easing deps.
		switch (name) {
			case 'linear':
			case 'none':
				return (t) => t;
			case 'power1.out':
			case 'power2.out':
			case 'power3.out':
			case 'power4.out':
			default:
				return cubicOut;
		}
	}

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
		if (!browser) {
			displayValue = targetValue;
			return;
		}

		if (rafId !== null) {
			cancelAnimationFrame(rafId);
			rafId = null;
		}

		const ease = resolveEasingFn(easing);
		const startValue = displayValue;
		const durationMs = Math.max(0, duration) * 1000;
		const delayMs = Math.max(0, delay) * 1000;
		const scheduledStart = performance.now() + delayMs;

		const tick = (now: number) => {
			if (now < scheduledStart) {
				rafId = requestAnimationFrame(tick);
				return;
			}

			if (durationMs === 0) {
				displayValue = targetValue;
				rafId = null;
				return;
			}

			const t = Math.min((now - scheduledStart) / durationMs, 1);
			displayValue = startValue + (targetValue - startValue) * ease(t);

			if (t < 1) {
				rafId = requestAnimationFrame(tick);
			} else {
				displayValue = targetValue;
				rafId = null;
			}
		};

		rafId = requestAnimationFrame(tick);
	}

	onMount(() => {
		// Initial animation from 0 to value
		displayValue = 0;
		animate(value);
	});

	onDestroy(() => {
		if (rafId !== null) cancelAnimationFrame(rafId);
		rafId = null;
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

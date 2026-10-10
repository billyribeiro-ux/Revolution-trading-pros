<script lang="ts">
	import { prefersReducedMotion } from 'svelte/motion';
	import type { Attachment } from 'svelte/attachments';

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

	const FLASH_FALLBACK = { up: '#22c55e', down: '#ef4444' } as const;

	/**
	 * Flashes the number green when it rises and red when it falls, fading back
	 * to its resting colour. A Web Animation restarts cleanly on every change,
	 * so rapid updates (a slider drag) never leave a stale flash behind.
	 */
	const directionFlash: Attachment<HTMLSpanElement> = (element) => {
		let previous: number | undefined;
		let flash: Animation | undefined;

		$effect(() => {
			const current = value;
			const before = previous;
			previous = current;
			if (before === undefined || current === before || prefersReducedMotion.current) return;

			const direction = current > before ? 'up' : 'down';
			const token = direction === 'up' ? '--calc-call' : '--calc-put';
			const color =
				getComputedStyle(element).getPropertyValue(token).trim() || FLASH_FALLBACK[direction];

			flash?.cancel();
			flash = element.animate([{ color, textShadow: `0 0 8px ${color}`, offset: 0 }], {
				duration: 500,
				easing: 'ease-out'
			});
		});

		return () => flash?.cancel();
	};
</script>

<span {@attach directionFlash} class={['number-display', colorClass, className]}>
	{displayText}
</span>

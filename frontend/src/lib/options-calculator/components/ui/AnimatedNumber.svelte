<script lang="ts">
	import { prefersReducedMotion } from 'svelte/motion';
	import type { Attachment } from 'svelte/attachments';
	import type { ClassValue } from 'svelte/elements';

	interface Props {
		value: number;
		decimals?: number;
		prefix?: string;
		suffix?: string;
		colorize?: boolean;
		class?: ClassValue;
	}

	let {
		value,
		decimals = 2,
		prefix = '',
		suffix = '',
		colorize = false,
		class: className
	}: Props = $props();

	/** Values within ±EPSILON read as zero and stay uncoloured. */
	const EPSILON = 0.0001;
	const FLASH_FALLBACK = { up: '#22c55e', down: '#ef4444' } as const;

	/**
	 * The number itself always shows the exact current value (no count-up tween): it is
	 * read while a slider drags. Instead it flashes green when it rises and red when it
	 * falls, fading back to its resting colour. A Web Animation restarts cleanly on every
	 * change, so rapid updates never leave a stale flash behind.
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

<span
	{@attach directionFlash}
	class={[
		'number-display',
		colorize && { 'text-call': value > EPSILON, 'text-put': value < -EPSILON },
		className
	]}
>
	{prefix}{value.toFixed(decimals)}{suffix}
</span>

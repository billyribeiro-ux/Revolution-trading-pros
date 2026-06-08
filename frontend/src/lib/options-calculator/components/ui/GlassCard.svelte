<script lang="ts">
	import { cardEntrance } from '../../utils/animations.js';
	import type { Snippet } from 'svelte';

	interface Props {
		class?: string;
		glow?: 'call' | 'put' | 'accent' | 'none';
		animate?: boolean;
		delay?: number;
		header?: Snippet;
		children: Snippet;
	}

	let {
		class: className = '',
		glow = 'none',
		animate = true,
		delay = 0,
		header,
		children
	}: Props = $props();

	let cardEl: HTMLDivElement | undefined;

	let glowClass = $derived(
		glow === 'call'
			? 'glow-call'
			: glow === 'put'
				? 'glow-put'
				: glow === 'accent'
					? 'shadow-[0_0_20px_var(--calc-accent-glow)]'
					: ''
	);

	function setupCard(element: HTMLDivElement) {
		cardEl = element;
		if (animate) {
			cardEntrance(element, delay);
		}

		return () => {
			if (cardEl === element) {
				cardEl = undefined;
			}
		};
	}
</script>

<div {@attach setupCard} class={['glass-card', glowClass, className]}>
	{#if header}
		<div class="border-b border-[var(--calc-glass-border)] px-5 py-3">
			{@render header()}
		</div>
	{/if}
	<div class="p-5">
		{@render children()}
	</div>
</div>

<script lang="ts">
	import IconTrendingUp from '@tabler/icons-svelte-runes/icons/trending-up';
	import IconTrendingDown from '@tabler/icons-svelte-runes/icons/trending-down';
	import IconMinus from '@tabler/icons-svelte-runes/icons/minus';
	import type { Icon } from '@tabler/icons-svelte-runes';
	import { cubicOut } from 'svelte/easing';
	import { prefersReducedMotion } from 'svelte/motion';
	import { fly, type FlyParams } from 'svelte/transition';
	import { STRATEGY_TEMPLATES } from '../engine/constants.js';
	import { getCalculator } from '../state/context.js';
	import type { StrategyTemplate } from '../engine/types.js';

	interface Props {
		/** Called once a preset has replaced the current strategy. */
		onSelect: () => void;
	}

	let { onSelect }: Props = $props();

	const calc = getCalculator();

	const SENTIMENTS: Record<StrategyTemplate['sentiment'], { icon: Icon; color: string }> = {
		bullish: { icon: IconTrendingUp, color: 'var(--calc-call)' },
		bearish: { icon: IconTrendingDown, color: 'var(--calc-put)' },
		neutral: { icon: IconMinus, color: 'var(--calc-accent)' }
	};

	let filter = $state('');

	const filtered = $derived.by(() => {
		if (!filter) return STRATEGY_TEMPLATES;
		const q = filter.toLowerCase();
		return STRATEGY_TEMPLATES.filter(
			(t) => t.name.toLowerCase().includes(q) || t.sentiment.toLowerCase().includes(q)
		);
	});

	function applyPreset(template: StrategyTemplate) {
		calc.clearStrategy();
		for (const leg of template.legs) {
			calc.addStrategyLeg({
				type: leg.type,
				strike: calc.spotPrice + leg.strike,
				expiry: calc.timeToExpiry,
				position: leg.position,
				quantity: leg.quantity,
				premium: 0
			});
		}
		calc.calculatorMode = 'strategy';
		onSelect();
	}

	/** Staggered entrance for the preset cards; instant under reduced motion. */
	function cardIntro(index: number): FlyParams {
		if (prefersReducedMotion.current) return { duration: 0 };
		return { y: 20, duration: 300, delay: index * 40, easing: cubicOut };
	}
</script>

<div class="flex flex-col gap-3">
	<!-- Search -->
	<input
		type="text"
		bind:value={filter}
		placeholder="Search strategies..."
		aria-label="Search strategies"
		class="text-xs px-3 py-2 rounded-lg outline-none w-full"
		style="background: var(--calc-surface-hover); color: var(--calc-text); border: 1px solid var(--calc-border); font-family: var(--calc-font-body);"
	/>

	<!-- Grid -->
	<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-2">
		{#each filtered as template, i (template.id)}
			{const sentiment = $derived(SENTIMENTS[template.sentiment])}
			<button
				type="button"
				in:fly|global={cardIntro(i)}
				onclick={() => applyPreset(template)}
				class="preset-card flex flex-col gap-1.5 rounded-xl px-3 py-2.5 text-left transition-all motion-reduce:transition-none cursor-pointer"
				style="background: var(--calc-surface); border: 1px solid var(--calc-border);"
			>
				<div class="flex items-center justify-between">
					<span
						class="text-xs font-semibold"
						style="color: var(--calc-text); font-family: var(--calc-font-display);"
					>
						{template.name}
					</span>
					<span
						class="sentiment-badge text-[9px] font-bold px-1.5 py-0.5 rounded-full uppercase"
						style:--sentiment={sentiment.color}
					>
						<sentiment.icon size={10} class="inline -mt-0.5" aria-hidden="true" />
						{template.sentiment}
					</span>
				</div>
				<span class="text-[10px]" style="color: var(--calc-text-muted);">
					{template.legs.length} legs · {template.riskProfile} risk
				</span>
			</button>
		{/each}
	</div>
</div>

<style>
	.sentiment-badge {
		color: var(--sentiment);
		/* 0x22 alpha tint of the sentiment colour (the colours are CSS variables, so mix rather than append hex). */
		background: color-mix(in srgb, var(--sentiment) 13%, transparent);
	}
</style>

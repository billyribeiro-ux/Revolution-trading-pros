<script lang="ts">
	import gsap from 'gsap';
	import { TrendingUp, TrendingDown, Minus } from '@lucide/svelte';
	import { STRATEGY_TEMPLATES } from '../engine/constants.js';
	import type { CalculatorState } from '../state/calculator.svelte.js';
	import type { StrategyTemplate } from '../engine/types.js';

	interface Props {
		calc: CalculatorState;
		onSelect: () => void;
	}

	let { calc, onSelect }: Props = $props();

	let gridEl: HTMLDivElement | undefined = $state();
	let filter = $state('');

	let filtered = $derived.by(() => {
		if (!filter) return STRATEGY_TEMPLATES;
		const q = filter.toLowerCase();
		return STRATEGY_TEMPLATES.filter(
			(t) => t.name.toLowerCase().includes(q) || t.sentiment.toLowerCase().includes(q)
		);
	});

	$effect(() => {
		if (gridEl) {
			const cards = gridEl.querySelectorAll('.preset-card');
			gsap.fromTo(
				cards,
				{ y: 20, opacity: 0 },
				{ y: 0, opacity: 1, duration: 0.3, stagger: 0.04, ease: 'power2.out' }
			);
		}
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

	function sentimentColor(sentiment: string): string {
		switch (sentiment) {
			case 'bullish':
				return 'var(--calc-call)';
			case 'bearish':
				return 'var(--calc-put)';
			default:
				return 'var(--calc-accent)';
		}
	}
</script>

<div class="flex flex-col gap-3">
	<!-- Search -->
	<input
		type="text"
		bind:value={filter}
		placeholder="Search strategies..."
		class="text-xs px-3 py-2 rounded-lg outline-none w-full"
		style="background: var(--calc-surface-hover); color: var(--calc-text); border: 1px solid var(--calc-border); font-family: var(--calc-font-body);"
	/>

	<!-- Grid -->
	<div bind:this={gridEl} class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-2">
		{#each filtered as template (template.name)}
			<button
				onclick={() => applyPreset(template)}
				class="preset-card flex flex-col gap-1.5 rounded-xl px-3 py-2.5 text-left transition-all cursor-pointer"
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
						class="text-[9px] font-bold px-1.5 py-0.5 rounded-full uppercase"
						style="color: {sentimentColor(template.sentiment)}; background: {sentimentColor(
							template.sentiment
						)}22;"
					>
						{#if template.sentiment === 'bullish'}
							<TrendingUp size={10} class="inline -mt-0.5" />
						{:else if template.sentiment === 'bearish'}
							<TrendingDown size={10} class="inline -mt-0.5" />
						{:else}
							<Minus size={10} class="inline -mt-0.5" />
						{/if}
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

<script lang="ts">
	import { Lightbulb } from '@lucide/svelte';
	import gsap from 'gsap';
	import { GREEK_EDUCATION } from './education-content.js';
	import type { EducationEntry } from '../../engine/types.js';

	interface Props {
		greekId: string;
		value: number;
	}

	let { greekId, value }: Props = $props();

	let cardEl: HTMLDivElement | undefined = $state();

	let entry = $derived<EducationEntry | undefined>(GREEK_EDUCATION[greekId]);

	$effect(() => {
		if (cardEl && entry) {
			gsap.fromTo(
				cardEl,
				{ y: 12, opacity: 0 },
				{ y: 0, opacity: 1, duration: 0.3, ease: 'power2.out' }
			);
		}
	});

	let absValue = $derived(Math.abs(value));

	let interpretation = $derived.by(() => {
		if (!entry) return '';
		switch (greekId) {
			case 'delta':
				return `For every $1 the stock moves, this option moves ~$${absValue.toFixed(2)}.${absValue > 0.5 ? ' This option is behaving more like stock.' : ' This option has a lower probability of expiring ITM.'}`;
			case 'gamma':
				return `Delta changes by ${absValue.toFixed(4)} for each $1 stock move.${absValue > 0.05 ? ' High gamma \u2014 your delta is shifting rapidly.' : ' Gamma is moderate \u2014 delta is relatively stable.'}`;
			case 'theta':
				return `This option loses $${absValue.toFixed(4)} per day from time decay.${absValue > 0.05 ? ' Significant daily bleed \u2014 time is working against buyers.' : ' Modest decay \u2014 plenty of time value remaining.'}`;
			case 'vega':
				return `A 1% IV change moves this option by $${absValue.toFixed(4)}.${absValue > 0.1 ? ' High vega exposure \u2014 watch for IV crush.' : ' Low vega \u2014 less sensitive to vol changes.'}`;
			case 'rho':
				return `A 1% rate change moves this option by $${absValue.toFixed(4)}. ${absValue < 0.01 ? 'Negligible for short-dated options.' : 'Meaningful for longer-dated positions.'}`;
			case 'charm':
				return `Delta is bleeding ${absValue.toFixed(6)} per day due to time passage.`;
			case 'vanna':
				return `Delta shifts by ${absValue.toFixed(6)} for each 1% IV change.`;
			case 'volga':
				return `Vega changes by ${absValue.toFixed(4)} for each 1% IV change.`;
			case 'speed':
				return `Gamma shifts by ${absValue.toFixed(8)} for each $1 stock move.`;
			case 'color':
				return `Gamma is changing by ${absValue.toFixed(8)} per day.`;
			case 'zomma':
				return `Gamma shifts by ${absValue.toFixed(6)} for each 1% IV change.`;
			default:
				return entry.shortDescription;
		}
	});
</script>

{#if entry}
	<div
		bind:this={cardEl}
		class="rounded-xl p-3 flex flex-col gap-2.5"
		style="background: var(--calc-surface); border: 1px solid var(--calc-border);"
	>
		<!-- Header: symbol + name + value -->
		<div class="flex items-center gap-2">
			{#if entry.symbol}
				<span
					class="text-xs font-bold px-1.5 py-0.5 rounded-md"
					style="
						background: var(--calc-accent-glow);
						color: var(--calc-accent);
						font-family: var(--calc-font-mono);
					">{entry.symbol}</span
				>
			{/if}
			<span class="text-xs font-semibold" style="color: var(--calc-text);">
				{entry.term}
			</span>
			<span
				class="text-xs font-mono ml-auto tabular-nums"
				style="color: var(--calc-accent); font-family: var(--calc-font-mono);"
			>
				{value >= 0 ? '+' : ''}{value.toFixed(4)}
			</span>
		</div>

		<!-- Contextual interpretation -->
		<p class="text-[10px] leading-relaxed" style="color: var(--calc-text-secondary);">
			{interpretation}
		</p>

		<!-- Pro Tip -->
		<div
			class="flex items-start gap-1.5 rounded-lg px-2.5 py-2"
			style="background: rgba(245,158,11,0.06); border: 1px solid rgba(245,158,11,0.1);"
		>
			<Lightbulb size={10} style="color: var(--color-warning); flex-shrink: 0; margin-top: 2px;" />
			<p class="text-[9px] leading-relaxed" style="color: var(--calc-text-muted);">
				{entry.proTip}
			</p>
		</div>
	</div>
{/if}

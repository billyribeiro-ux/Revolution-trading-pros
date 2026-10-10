<script lang="ts">
	import IconBulb from '@tabler/icons-svelte-runes/icons/bulb';
	import { cubicOut } from 'svelte/easing';
	import { prefersReducedMotion } from 'svelte/motion';
	import { fly } from 'svelte/transition';
	import { getGreekEducation } from './education-content.js';

	interface Props {
		greekId: string;
		value: number;
	}

	let { greekId, value }: Props = $props();

	const entry = $derived(getGreekEducation(greekId));

	const absValue = $derived(Math.abs(value));

	const interpretation = $derived.by(() => {
		if (!entry) return '';
		switch (greekId) {
			case 'delta':
				return `For every $1 the stock moves, this option moves ~$${absValue.toFixed(2)}.${absValue > 0.5 ? ' This option is behaving more like stock.' : ' This option has a lower probability of expiring ITM.'}`;
			case 'gamma':
				return `Delta changes by ${absValue.toFixed(4)} for each $1 stock move.${absValue > 0.05 ? ' High gamma — your delta is shifting rapidly.' : ' Gamma is moderate — delta is relatively stable.'}`;
			case 'theta':
				return `This option loses $${absValue.toFixed(4)} per day from time decay.${absValue > 0.05 ? ' Significant daily bleed — time is working against buyers.' : ' Modest decay — plenty of time value remaining.'}`;
			case 'vega':
				return `A 1% IV change moves this option by $${absValue.toFixed(4)}.${absValue > 0.1 ? ' High vega exposure — watch for IV crush.' : ' Low vega — less sensitive to vol changes.'}`;
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
	<!-- Keyed so the card rises in again whenever it switches to another Greek; global so it
	     also does when the card first appears with its parent. -->
	{#key entry}
		<div
			in:fly|global={{ y: 12, duration: prefersReducedMotion.current ? 0 : 300, easing: cubicOut }}
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
				<IconBulb
					size={10}
					style="color: #f59e0b; flex-shrink: 0; margin-top: 2px;"
					aria-hidden="true"
				/>
				<p class="text-[9px] leading-relaxed" style="color: var(--calc-text-muted);">
					{entry.proTip}
				</p>
			</div>
		</div>
	{/key}
{/if}

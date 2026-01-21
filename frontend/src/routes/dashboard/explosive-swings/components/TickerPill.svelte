<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * TickerPill Component - Closed Trade Display
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @description Displays a closed trade as a compact pill with percentage result
	 * @version 4.0.0 - January 2026 - Nuclear Build Specification
	 * @standards Apple Principal Engineer ICT 7+ Standards
	 */
	import type { ClosedTrade } from '../types';
	import { formatPercent } from '../utils/formatters';

	interface Props {
		trade: ClosedTrade;
	}

	const { trade }: Props = $props();

	const bgColor = $derived(
		trade.isWinner
			? 'bg-emerald-50 border-emerald-200 hover:bg-emerald-100'
			: 'bg-red-50 border-red-200 hover:bg-red-100'
	);

	const textColor = $derived(trade.isWinner ? 'text-emerald-700' : 'text-red-600');
</script>

<div
	class="ticker-pill {bgColor}"
	role="listitem"
	aria-label="{trade.ticker}: {formatPercent(trade.percentageGain)}"
>
	<span class="ticker-symbol">{trade.ticker}</span>
	<span class="ticker-percent {textColor}">{formatPercent(trade.percentageGain)}</span>
	<span class="ticker-icon" aria-hidden="true">
		{#if trade.isWinner}
			<svg viewBox="0 0 20 20" fill="currentColor" class="w-4 h-4 text-emerald-600">
				<path
					fill-rule="evenodd"
					d="M16.704 4.153a.75.75 0 01.143 1.052l-8 10.5a.75.75 0 01-1.127.075l-4.5-4.5a.75.75 0 011.06-1.06l3.894 3.893 7.48-9.817a.75.75 0 011.05-.143z"
					clip-rule="evenodd"
				/>
			</svg>
		{:else}
			<svg viewBox="0 0 20 20" fill="currentColor" class="w-4 h-4 text-red-500">
				<path
					d="M6.28 5.22a.75.75 0 00-1.06 1.06L8.94 10l-3.72 3.72a.75.75 0 101.06 1.06L10 11.06l3.72 3.72a.75.75 0 101.06-1.06L11.06 10l3.72-3.72a.75.75 0 00-1.06-1.06L10 8.94 6.28 5.22z"
				/>
			</svg>
		{/if}
	</span>
</div>

<style>
	.ticker-pill {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		min-width: 80px;
		max-width: 100px;
		padding: 12px 16px;
		border-radius: 8px;
		border: 1px solid;
		gap: 4px;
		transition: all 0.2s ease-out;
		cursor: default;
		flex-shrink: 0;
	}

	.ticker-symbol {
		font-size: 14px;
		font-weight: 700;
		text-transform: uppercase;
		color: #1e293b;
		letter-spacing: 0.025em;
	}

	.ticker-percent {
		font-size: 18px;
		font-weight: 700;
		font-variant-numeric: tabular-nums;
	}

	.ticker-icon {
		display: flex;
		align-items: center;
		justify-content: center;
	}
</style>

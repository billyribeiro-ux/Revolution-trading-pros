<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * TickerPill Component - Closed Trade Display
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @description Displays a closed trade as a compact pill with percentage result
	 * @version 4.1.0 - Visual Polish Pass
	 * @standards Apple Principal Engineer ICT 7+ Standards
	 */
	import type { ClosedTrade } from '../types';
	import { formatPercent } from '../utils/formatters';

	interface Props {
		trade: ClosedTrade;
	}

	const { trade }: Props = $props();

	const isWinner = $derived(trade.isWinner);
</script>

<div
	class="ticker-pill"
	class:winner={isWinner}
	class:loser={!isWinner}
	role="listitem"
	aria-label="{trade.ticker}: {formatPercent(trade.percentageGain)}"
>
	<!-- Ticker Symbol -->
	<span class="ticker-symbol">{trade.ticker}</span>
	
	<!-- Percentage Result -->
	<span class="ticker-percent">
		{formatPercent(trade.percentageGain)}
	</span>
	
	<!-- Result Icon -->
	<span class="ticker-icon" aria-hidden="true">
		{#if isWinner}
			<svg viewBox="0 0 20 20" fill="currentColor">
				<path
					fill-rule="evenodd"
					d="M16.704 4.153a.75.75 0 01.143 1.052l-8 10.5a.75.75 0 01-1.127.075l-4.5-4.5a.75.75 0 011.06-1.06l3.894 3.893 7.48-9.817a.75.75 0 011.05-.143z"
					clip-rule="evenodd"
				/>
			</svg>
		{:else}
			<svg viewBox="0 0 20 20" fill="currentColor">
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
		gap: 2px;
		min-width: 88px;
		padding: 14px 18px 12px;
		border-radius: 10px;
		border: 1.5px solid;
		flex-shrink: 0;
		scroll-snap-align: start;
		cursor: default;
		transition: all 0.2s ease-out;
	}

	/* Winner State */
	.ticker-pill.winner {
		background: linear-gradient(135deg, #ecfdf5 0%, #d1fae5 100%);
		border-color: #a7f3d0;
	}

	.ticker-pill.winner:hover {
		background: linear-gradient(135deg, #d1fae5 0%, #a7f3d0 100%);
		border-color: #6ee7b7;
		transform: translateY(-2px);
		box-shadow: 0 4px 12px rgba(16, 185, 129, 0.2);
	}

	.ticker-pill.winner .ticker-percent {
		color: #059669;
	}

	.ticker-pill.winner .ticker-icon {
		color: #10b981;
	}

	/* Loser State */
	.ticker-pill.loser {
		background: linear-gradient(135deg, #fef2f2 0%, #fee2e2 100%);
		border-color: #fecaca;
	}

	.ticker-pill.loser:hover {
		background: linear-gradient(135deg, #fee2e2 0%, #fecaca 100%);
		border-color: #fca5a5;
		transform: translateY(-2px);
		box-shadow: 0 4px 12px rgba(239, 68, 68, 0.15);
	}

	.ticker-pill.loser .ticker-percent {
		color: #dc2626;
	}

	.ticker-pill.loser .ticker-icon {
		color: #ef4444;
	}

	/* Ticker Symbol */
	.ticker-symbol {
		font-size: 13px;
		font-weight: 800;
		text-transform: uppercase;
		color: #1e293b;
		letter-spacing: 0.03em;
		line-height: 1;
	}

	/* Percentage */
	.ticker-percent {
		font-size: 19px;
		font-weight: 700;
		font-variant-numeric: tabular-nums;
		line-height: 1.1;
	}

	/* Icon */
	.ticker-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		margin-top: 2px;
	}

	.ticker-icon svg {
		width: 16px;
		height: 16px;
	}
</style>
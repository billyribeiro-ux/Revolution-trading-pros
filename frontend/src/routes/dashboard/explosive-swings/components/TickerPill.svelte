<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * TickerPill Component - COMPACT Closed Trade Display
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @version 5.0.0 - High Density Refactor: 40px height, horizontal layout
	 * @standards Apple Principal Engineer ICT 7+ | WCAG 2.1 AA
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
	class="pill"
	class:win={isWinner}
	class:loss={!isWinner}
	role="listitem"
	aria-label="{trade.ticker}: {formatPercent(trade.percentageGain)}"
>
	<span class="icon" aria-hidden="true">
		{#if isWinner}
			<svg viewBox="0 0 20 20" fill="currentColor">
				<path fill-rule="evenodd" d="M16.704 4.153a.75.75 0 01.143 1.052l-8 10.5a.75.75 0 01-1.127.075l-4.5-4.5a.75.75 0 011.06-1.06l3.894 3.893 7.48-9.817a.75.75 0 011.05-.143z" clip-rule="evenodd" />
			</svg>
		{:else}
			<svg viewBox="0 0 20 20" fill="currentColor">
				<path d="M6.28 5.22a.75.75 0 00-1.06 1.06L8.94 10l-3.72 3.72a.75.75 0 101.06 1.06L10 11.06l3.72 3.72a.75.75 0 101.06-1.06L11.06 10l3.72-3.72a.75.75 0 00-1.06-1.06L10 8.94 6.28 5.22z" />
			</svg>
		{/if}
	</span>
	<span class="ticker">{trade.ticker}</span>
	<span class="pct">{formatPercent(trade.percentageGain)}</span>
</div>

<style>
	/* COMPACT PILL - Horizontal Layout */
	.pill {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		padding: 6px 10px;
		border-radius: var(--radius-md);
		border: 1px solid;
		flex-shrink: 0;
		scroll-snap-align: start;
		cursor: default;
		transition: var(--transition-shadow);
	}

	/* Win State */
	.pill.win {
		background: var(--color-profit-bg);
		border-color: var(--color-profit-border);
	}

	.pill.win:hover {
		box-shadow: 0 2px 8px rgba(16, 185, 129, 0.15);
	}

	.pill.win .pct {
		color: var(--color-profit);
	}

	.pill.win .icon {
		color: var(--color-profit-light);
	}

	/* Loss State */
	.pill.loss {
		background: var(--color-loss-bg);
		border-color: var(--color-loss-border);
	}

	.pill.loss:hover {
		box-shadow: 0 2px 8px rgba(239, 68, 68, 0.12);
	}

	.pill.loss .pct {
		color: var(--color-loss);
	}

	.pill.loss .icon {
		color: var(--color-loss-light);
	}

	/* Icon */
	.icon {
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
	}

	.icon svg {
		width: 12px;
		height: 12px;
	}

	/* Ticker */
	.ticker {
		font-size: 12px;
		font-weight: var(--font-extrabold);
		text-transform: uppercase;
		color: var(--color-text-primary);
		letter-spacing: 0.02em;
	}

	/* Percentage */
	.pct {
		font-size: 13px;
		font-weight: var(--font-bold);
		font-variant-numeric: tabular-nums;
	}
</style>
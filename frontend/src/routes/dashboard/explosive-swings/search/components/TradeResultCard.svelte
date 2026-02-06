<script lang="ts">
	/**
	 * TradeResultCard - Individual trade search result
	 * @standards Svelte 5 January 2026 | Apple Principal Engineer ICT 7+
	 */
	import type { TradeSearchResult } from '../search.state.svelte';

	interface Props {
		trade: TradeSearchResult;
		query: string;
	}

	let { trade, query }: Props = $props();

	// Highlight matching search terms in text
	function highlightMatch(text: string): string {
		if (!query || !text) return text;
		const escaped = query.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
		const regex = new RegExp(`(${escaped})`, 'gi');
		return text.replace(regex, '<mark>$1</mark>');
	}

	// Determine if trade was a win/loss
	const isWin = $derived(trade.result === 'WIN' || (trade.pnl_percent ?? 0) > 0);
	const isOpen = $derived(trade.status === 'open');

	// Format date
	function formatDate(dateStr: string): string {
		const date = new Date(dateStr);
		return date.toLocaleDateString('en-US', {
			month: 'short',
			day: 'numeric',
			year: 'numeric'
		});
	}

	// Format price
	function formatPrice(price: number | null): string {
		if (price === null) return '-';
		return `$${price.toFixed(2)}`;
	}

	// Format percentage
	function formatPercent(pct: number | null): string {
		if (pct === null) return '-';
		const sign = pct >= 0 ? '+' : '';
		return `${sign}${pct.toFixed(2)}%`;
	}

	// Relevance score as percentage
	const relevancePercent = $derived(Math.round(trade.relevance_score * 100));
</script>

<article
	class="trade-card"
	class:win={isWin && !isOpen}
	class:loss={!isWin && !isOpen}
	class:open={isOpen}
>
	<div class="card-header">
		<span class="ticker-badge">{trade.ticker}</span>
		<span
			class="direction-badge"
			class:long={trade.direction === 'long'}
			class:short={trade.direction === 'short'}
		>
			{trade.direction.toUpperCase()}
		</span>
		<span class="status-badge" class:open={isOpen} class:closed={!isOpen}>
			{trade.status.toUpperCase()}
		</span>
		<span class="date">{formatDate(trade.entry_date)}</span>
		<span class="relevance" title="Relevance score">
			<svg width="12" height="12" viewBox="0 0 24 24" fill="currentColor">
				<polygon
					points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"
				/>
			</svg>
			{relevancePercent}%
		</span>
	</div>

	<div class="trade-details">
		<div class="detail-row">
			<span class="detail-label">Entry</span>
			<span class="detail-value">{formatPrice(trade.entry_price)}</span>
		</div>

		{#if trade.exit_price !== null}
			<div class="detail-row">
				<span class="detail-label">Exit</span>
				<span class="detail-value">{formatPrice(trade.exit_price)}</span>
			</div>
		{/if}

		{#if trade.pnl_percent !== null}
			<div class="detail-row">
				<span class="detail-label">P/L</span>
				<span class="detail-value pnl" class:positive={isWin} class:negative={!isWin}>
					{formatPercent(trade.pnl_percent)}
				</span>
			</div>
		{/if}

		{#if trade.result}
			<div class="detail-row">
				<span class="detail-label">Result</span>
				<span class="result-badge" class:win={isWin} class:loss={!isWin}>
					{trade.result}
				</span>
			</div>
		{/if}
	</div>

	{#if trade.notes}
		<div class="card-highlight">
			{@html highlightMatch(trade.highlight)}
		</div>
	{/if}
</article>

<style>
	.trade-card {
		padding: 16px 20px;
		border-left: 4px solid var(--color-border-default);
		transition: background 0.15s;
	}

	.trade-card:hover {
		background: var(--color-bg-subtle);
	}

	.trade-card:not(:last-child) {
		border-bottom: 1px solid var(--color-border-default);
	}

	.trade-card.win {
		border-left-color: var(--color-win);
	}

	.trade-card.loss {
		border-left-color: var(--color-loss);
	}

	.trade-card.open {
		border-left-color: var(--color-brand-primary);
	}

	.card-header {
		display: flex;
		align-items: center;
		gap: 10px;
		margin-bottom: 12px;
		flex-wrap: wrap;
	}

	.ticker-badge {
		display: inline-flex;
		padding: 3px 8px;
		background: var(--color-brand-primary);
		color: white;
		font-size: 11px;
		font-weight: 700;
		border-radius: 4px;
		letter-spacing: 0.5px;
	}

	.direction-badge {
		display: inline-flex;
		padding: 3px 8px;
		font-size: 10px;
		font-weight: 600;
		border-radius: 4px;
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.direction-badge.long {
		background: rgba(20, 184, 166, 0.15);
		color: #0d9488;
	}

	.direction-badge.short {
		background: rgba(239, 68, 68, 0.15);
		color: #dc2626;
	}

	.status-badge {
		display: inline-flex;
		padding: 3px 8px;
		font-size: 10px;
		font-weight: 600;
		border-radius: 4px;
	}

	.status-badge.open {
		background: rgba(59, 130, 246, 0.15);
		color: #2563eb;
	}

	.status-badge.closed {
		background: var(--color-bg-subtle);
		color: var(--color-text-tertiary);
	}

	.date {
		font-size: 12px;
		color: var(--color-text-tertiary);
	}

	.relevance {
		display: inline-flex;
		align-items: center;
		gap: 4px;
		margin-left: auto;
		font-size: 11px;
		color: var(--color-text-tertiary);
	}

	.relevance svg {
		color: var(--color-brand-secondary);
	}

	.trade-details {
		display: flex;
		flex-wrap: wrap;
		gap: 16px;
		margin-bottom: 12px;
	}

	.detail-row {
		display: flex;
		flex-direction: column;
		gap: 2px;
	}

	.detail-label {
		font-size: 11px;
		color: var(--color-text-tertiary);
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.detail-value {
		font-size: 14px;
		font-weight: 600;
		color: var(--color-text-primary);
	}

	.detail-value.pnl.positive {
		color: var(--color-win);
	}

	.detail-value.pnl.negative {
		color: var(--color-loss);
	}

	.result-badge {
		display: inline-flex;
		padding: 2px 8px;
		font-size: 11px;
		font-weight: 700;
		border-radius: 4px;
	}

	.result-badge.win {
		background: rgba(16, 185, 129, 0.15);
		color: #059669;
	}

	.result-badge.loss {
		background: rgba(239, 68, 68, 0.15);
		color: #dc2626;
	}

	.card-highlight {
		font-size: 13px;
		color: var(--color-text-secondary);
		line-height: 1.5;
	}

	.card-highlight :global(mark) {
		background: rgba(var(--color-brand-primary-rgb), 0.2);
		color: var(--color-text-primary);
		padding: 1px 3px;
		border-radius: 2px;
	}

	/* Mobile */
	@media (max-width: 768px) {
		.trade-card {
			padding: 12px 16px;
		}

		.relevance {
			order: 99;
			width: 100%;
			margin-left: 0;
			margin-top: 4px;
		}

		.trade-details {
			gap: 12px;
		}
	}
</style>

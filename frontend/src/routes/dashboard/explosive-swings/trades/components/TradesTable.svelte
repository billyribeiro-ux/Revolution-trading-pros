<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * TradesTable Component - Responsive Table/Card Hybrid with Accessibility
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * Uses native <table> elements for proper screen reader navigation on desktop.
	 * Transforms to stacked card layout on mobile for optimal touch experience.
	 * Supports expandable notes rows and admin actions.
	 *
	 * @version 2.0.0 - 2026 Responsive Rewrite
	 * @standards Apple Principal Engineer ICT 7+ | WCAG 2.1 AA
	 */
	import type { Trade } from '../types';

	interface Props {
		trades: Trade[];
		isAdmin?: boolean;
		onCloseTrade?: (trade: Trade) => void;
	}

	const { trades, isAdmin = false, onCloseTrade }: Props = $props();

	// Track expanded notes
	let expandedNotes = $state<Set<number>>(new Set());

	function toggleNotes(tradeId: number) {
		const newSet = new Set(expandedNotes);
		if (newSet.has(tradeId)) {
			newSet.delete(tradeId);
		} else {
			newSet.add(tradeId);
		}
		expandedNotes = newSet;
	}

	function formatPrice(price: number | null): string {
		if (price === null) return '—';
		return `$${price.toFixed(2)}`;
	}

	function formatProfit(profit: number): string {
		if (profit === 0) return '—';
		return `$${profit.toLocaleString()}`;
	}

	function formatPercent(percent: number): string {
		if (percent === 0) return '—';
		const sign = percent > 0 ? '+' : '';
		return `${sign}${percent.toFixed(1)}%`;
	}
</script>

<!-- Mobile Card Layout (< 768px) -->
<div class="mobile-cards md:hidden">
	{#each trades as trade (trade.id)}
		{@const hasNotes = trade.notes && trade.notes.trim().length > 0}
		{@const isExpanded = expandedNotes.has(trade.id)}

		<article
			class="trade-card py-3"
			class:active={trade.result === 'ACTIVE'}
			aria-label="{trade.ticker} trade"
		>
			<!-- Card Header: Ticker + Result Badge -->
			<div class="card-header">
				<div class="ticker-section">
					{#if hasNotes}
						<button
							type="button"
							class="ticker-btn"
							onclick={() => toggleNotes(trade.id)}
							aria-expanded={isExpanded}
							aria-controls="mobile-notes-{trade.id}"
						>
							<span class="expand-icon" class:expanded={isExpanded}>
								<svg
									viewBox="0 0 24 24"
									fill="none"
									stroke="currentColor"
									stroke-width="2"
									width="14"
									height="14"
								>
									<path d="M9 18l6-6-6-6" />
								</svg>
							</span>
							<span class="ticker-text">{trade.ticker}</span>
						</button>
					{:else}
						<span class="ticker-text">{trade.ticker}</span>
					{/if}
				</div>
				<span class="result-badge result--{trade.result.toLowerCase()}">
					{trade.result}
				</span>
			</div>

			<!-- Priority Data: Date + P/L -->
			<div class="card-priority-row">
				<div class="data-item">
					<span class="data-label">Entry</span>
					<span class="data-value">{trade.entryDate}</span>
				</div>
				<div
					class="data-item profit-cell"
					class:profit={trade.profit > 0}
					class:loss={trade.profit < 0}
				>
					<span class="data-label">P/L</span>
					<span class="data-value">{formatProfit(trade.profit)}</span>
				</div>
				<div
					class="data-item"
					class:profit={trade.profitPercent > 0}
					class:loss={trade.profitPercent < 0}
				>
					<span class="data-label">%</span>
					<span class="data-value">{formatPercent(trade.profitPercent)}</span>
				</div>
			</div>

			<!-- Secondary Data (collapsed by default, expandable) -->
			<details class="secondary-details">
				<summary class="secondary-toggle">More details</summary>
				<div class="secondary-grid">
					<div class="data-item">
						<span class="data-label">Exit</span>
						<span class="data-value">{trade.exitDate ?? 'Active'}</span>
					</div>
					<div class="data-item">
						<span class="data-label">Entry $</span>
						<span class="data-value">{formatPrice(trade.entryPrice)}</span>
					</div>
					<div class="data-item">
						<span class="data-label">Exit $</span>
						<span class="data-value">{formatPrice(trade.exitPrice)}</span>
					</div>
					<div class="data-item">
						<span class="data-label">Days</span>
						<span class="data-value">{trade.duration || '—'}</span>
					</div>
					<div class="data-item">
						<span class="data-label">Setup</span>
						<span class="data-value"><span class="setup-badge">{trade.setup}</span></span>
					</div>
				</div>
			</details>

			<!-- Admin Actions -->
			{#if isAdmin}
				<div class="card-actions">
					{#if trade.result === 'ACTIVE'}
						<button
							type="button"
							class="close-trade-btn"
							onclick={() => onCloseTrade?.(trade)}
							aria-label="Close {trade.ticker} trade"
						>
							Close Trade
						</button>
					{:else}
						<span class="closed-indicator">Trade Closed</span>
					{/if}
				</div>
			{/if}

			<!-- Expandable Notes -->
			{#if hasNotes && isExpanded}
				<div
					id="mobile-notes-{trade.id}"
					class="notes-content"
					role="region"
					aria-label="Trade notes for {trade.ticker}"
				>
					<strong>Notes:</strong>
					{trade.notes}
				</div>
			{/if}
		</article>
	{/each}
</div>

<!-- Desktop/Tablet Table Layout (768px+) -->
<div class="table-container hidden md:block">
	<div class="table-scroll-wrapper">
		<table class="trades-table" aria-label="Trade history">
			<thead>
				<tr>
					<th scope="col">Ticker</th>
					<th scope="col">Entry</th>
					<th scope="col" class="lg:table-cell hidden">Exit</th>
					<th scope="col" class="numeric lg:table-cell hidden">Entry $</th>
					<th scope="col" class="numeric lg:table-cell hidden">Exit $</th>
					<th scope="col" class="numeric">Profit</th>
					<th scope="col" class="numeric">%</th>
					<th scope="col" class="numeric xl:table-cell hidden">Days</th>
					<th scope="col" class="xl:table-cell hidden">Setup</th>
					<th scope="col">Result</th>
					{#if isAdmin}
						<th scope="col">Actions</th>
					{/if}
				</tr>
			</thead>
			<tbody>
				{#each trades as trade (trade.id)}
					{@const hasNotes = trade.notes && trade.notes.trim().length > 0}
					{@const isExpanded = expandedNotes.has(trade.id)}

					<tr class="sm:py-2" class:active={trade.result === 'ACTIVE'} class:expandable={hasNotes}>
						<td class="ticker-cell">
							{#if hasNotes}
								<button
									type="button"
									class="ticker-btn"
									onclick={() => toggleNotes(trade.id)}
									aria-expanded={isExpanded}
									aria-controls="notes-{trade.id}"
								>
									<span class="expand-icon" class:expanded={isExpanded}>
										<svg
											viewBox="0 0 24 24"
											fill="none"
											stroke="currentColor"
											stroke-width="2"
											width="14"
											height="14"
										>
											<path d="M9 18l6-6-6-6" />
										</svg>
									</span>
									{trade.ticker}
								</button>
							{:else}
								{trade.ticker}
							{/if}
						</td>
						<td>{trade.entryDate}</td>
						<td class="lg:table-cell hidden">{trade.exitDate ?? 'Active'}</td>
						<td class="numeric lg:table-cell hidden">{formatPrice(trade.entryPrice)}</td>
						<td class="numeric lg:table-cell hidden">{formatPrice(trade.exitPrice)}</td>
						<td class="numeric" class:profit={trade.profit > 0} class:loss={trade.profit < 0}>
							{formatProfit(trade.profit)}
						</td>
						<td
							class="numeric"
							class:profit={trade.profitPercent > 0}
							class:loss={trade.profitPercent < 0}
						>
							{formatPercent(trade.profitPercent)}
						</td>
						<td class="numeric xl:table-cell hidden">{trade.duration || '—'}</td>
						<td class="xl:table-cell hidden"><span class="setup-badge">{trade.setup}</span></td>
						<td>
							<span class="result-badge result--{trade.result.toLowerCase()}">
								{trade.result}
							</span>
						</td>
						{#if isAdmin}
							<td class="actions-cell">
								{#if trade.result === 'ACTIVE'}
									<button
										type="button"
										class="close-trade-btn"
										onclick={() => onCloseTrade?.(trade)}
										aria-label="Close {trade.ticker} trade"
									>
										Close
									</button>
								{:else}
									<span class="closed-indicator">Closed</span>
								{/if}
							</td>
						{/if}
					</tr>

					<!-- Expandable Notes Row -->
					{#if hasNotes}
						<tr
							id="notes-{trade.id}"
							class="notes-row"
							class:expanded={isExpanded}
							aria-hidden={!isExpanded}
						>
							<td colspan={isAdmin ? 11 : 10}>
								<div class="notes-content">
									<strong>Notes:</strong>
									{trade.notes}
								</div>
							</td>
						</tr>
					{/if}
				{/each}
			</tbody>
		</table>
	</div>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════════
	 * MOBILE CARD LAYOUT (< 768px)
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.mobile-cards {
		display: flex;
		flex-direction: column;
		gap: var(--space-3);
		padding: var(--space-4);
	}

	.trade-card {
		background: var(--color-bg-card);
		border-radius: var(--radius-lg);
		padding: var(--space-4);
		box-shadow: var(--shadow-sm);
		border: 1px solid var(--color-border-subtle);
		transition: var(--transition-shadow);
	}

	.trade-card:active {
		box-shadow: var(--shadow-md);
	}

	.trade-card.active {
		background: var(--color-active-bg);
		border-color: var(--color-active-border);
	}

	.card-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: var(--space-3);
	}

	.ticker-section {
		display: flex;
		align-items: center;
	}

	.ticker-text {
		font-weight: var(--font-bold);
		font-size: var(--text-lg);
		color: var(--color-brand-primary);
		font-family: var(--font-display);
	}

	.card-priority-row {
		display: grid;
		grid-template-columns: 1fr 1fr 1fr;
		gap: var(--space-3);
		padding: var(--space-3) 0;
		border-top: 1px solid var(--color-border-subtle);
		border-bottom: 1px solid var(--color-border-subtle);
	}

	.data-item {
		display: flex;
		flex-direction: column;
		gap: var(--space-1);
	}

	.data-label {
		font-size: var(--text-xs);
		font-weight: var(--font-medium);
		color: var(--color-text-tertiary);
		text-transform: uppercase;
		letter-spacing: var(--tracking-wider);
	}

	.data-value {
		font-size: var(--text-sm);
		font-weight: var(--font-semibold);
		color: var(--color-text-primary);
	}

	.secondary-details {
		margin-top: var(--space-3);
	}

	.secondary-toggle {
		font-size: var(--text-xs);
		color: var(--color-text-tertiary);
		cursor: pointer;
		padding: var(--space-2) 0;
		user-select: none;
		-webkit-tap-highlight-color: transparent;
	}

	.secondary-toggle::-webkit-details-marker {
		display: none;
	}

	.secondary-toggle::before {
		content: '+ ';
	}

	.secondary-details[open] .secondary-toggle::before {
		content: '− ';
	}

	.secondary-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: var(--space-3);
		padding-top: var(--space-3);
	}

	.card-actions {
		margin-top: var(--space-3);
		padding-top: var(--space-3);
		border-top: 1px solid var(--color-border-subtle);
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * DESKTOP/TABLET TABLE LAYOUT (768px+)
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.table-container {
		max-width: 1400px;
		margin: 0 auto;
		background: var(--color-bg-card);
		border-radius: var(--radius-xl);
		padding: var(--space-4);
		box-shadow: var(--shadow-md);
	}

	.table-scroll-wrapper {
		overflow-x: auto;
		-webkit-overflow-scrolling: touch;
		scroll-snap-type: x proximity;
		scrollbar-width: thin;
		scrollbar-color: var(--color-border-default) transparent;

		/* Scroll shadow affordance */
		background:
			linear-gradient(to right, var(--color-bg-card) 30%, transparent),
			linear-gradient(to right, transparent, var(--color-bg-card) 70%) 100% 0,
			radial-gradient(farthest-side at 0 50%, rgba(0, 0, 0, 0.12), transparent),
			radial-gradient(farthest-side at 100% 50%, rgba(0, 0, 0, 0.12), transparent) 100% 0;
		background-repeat: no-repeat;
		background-size:
			40px 100%,
			40px 100%,
			14px 100%,
			14px 100%;
		background-attachment: local, local, scroll, scroll;
	}

	.table-scroll-wrapper::-webkit-scrollbar {
		height: 8px;
	}

	.table-scroll-wrapper::-webkit-scrollbar-track {
		background: transparent;
	}

	.table-scroll-wrapper::-webkit-scrollbar-thumb {
		background-color: var(--color-border-default);
		border-radius: 4px;
	}

	.trades-table {
		width: 100%;
		border-collapse: collapse;
		table-layout: auto;
	}

	/* Header */
	thead {
		background: var(--color-bg-subtle);
	}

	th {
		padding: var(--space-3) var(--space-3);
		font-size: var(--text-xs);
		font-weight: var(--font-bold);
		text-transform: uppercase;
		letter-spacing: var(--tracking-wider);
		color: var(--color-text-tertiary);
		text-align: left;
		white-space: nowrap;
		position: sticky;
		top: 0;
		background: var(--color-bg-subtle);
	}

	th:first-child {
		border-radius: var(--radius-md) 0 0 var(--radius-md);
	}

	th:last-child {
		border-radius: 0 var(--radius-md) var(--radius-md) 0;
	}

	th.numeric {
		text-align: right;
	}

	/* Responsive column visibility */
	th.hidden {
		display: none;
	}

	td.hidden {
		display: none;
	}

	@media (min-width: 1024px) {
		th.lg\:table-cell,
		td.lg\:table-cell {
			display: table-cell;
		}
	}

	@media (min-width: 1280px) {
		th.xl\:table-cell,
		td.xl\:table-cell {
			display: table-cell;
		}
	}

	/* Body */
	td {
		padding: var(--space-3) var(--space-3);
		font-size: var(--text-sm);
		color: var(--color-text-secondary);
		border-bottom: 1px solid var(--color-border-subtle);
		vertical-align: middle;
		scroll-snap-align: start;
	}

	td.numeric {
		text-align: right;
		font-variant-numeric: tabular-nums;
	}

	tbody tr:hover {
		background: var(--color-bg-card-hover);
	}

	tbody tr.active {
		background: var(--color-active-bg);
	}

	tbody tr.active:hover {
		background: #fef0c7;
	}

	/* Ticker Cell */
	.ticker-cell {
		font-weight: var(--font-bold);
		color: var(--color-brand-primary);
		font-size: var(--text-base);
		font-family: var(--font-display);
	}

	.ticker-btn {
		display: inline-flex;
		align-items: center;
		gap: var(--space-1);
		background: none;
		border: none;
		padding: var(--space-1);
		margin: calc(-1 * var(--space-1));
		font: inherit;
		color: inherit;
		cursor: pointer;
		font-weight: var(--font-bold);
		border-radius: var(--radius-sm);
		-webkit-tap-highlight-color: transparent;
		touch-action: manipulation;
	}

	.ticker-btn:hover {
		color: var(--color-brand-primary-hover);
	}

	.ticker-btn:focus-visible {
		outline: 2px solid var(--color-brand-primary);
		outline-offset: 2px;
		border-radius: var(--radius-sm);
	}

	.expand-icon {
		display: inline-flex;
		transition: transform var(--duration-fast) var(--ease-out);
	}

	.expand-icon.expanded {
		transform: rotate(90deg);
	}

	/* Profit/Loss */
	.profit {
		color: var(--color-profit);
		font-weight: var(--font-semibold);
	}

	.loss {
		color: var(--color-loss);
		font-weight: var(--font-semibold);
	}

	/* Badges */
	.setup-badge {
		display: inline-block;
		background: var(--color-bg-muted);
		padding: var(--space-1) var(--space-2);
		border-radius: var(--radius-sm);
		font-size: var(--text-xs);
		font-weight: var(--font-semibold);
		color: var(--color-text-secondary);
	}

	.result-badge {
		display: inline-block;
		padding: var(--space-1) var(--space-2);
		border-radius: var(--radius-sm);
		font-size: var(--text-xs);
		font-weight: var(--font-bold);
		text-transform: uppercase;
		letter-spacing: var(--tracking-wide);
		white-space: nowrap;
	}

	.result--win {
		background: var(--color-profit-bg);
		color: var(--color-profit);
		border: 1px solid var(--color-profit-border);
	}

	.result--loss {
		background: var(--color-loss-bg);
		color: var(--color-loss);
		border: 1px solid var(--color-loss-border);
	}

	.result--active {
		background: var(--color-active-bg);
		color: var(--color-active-text);
		border: 1px solid var(--color-active-border);
	}

	.result--invalidated {
		background: var(--color-bg-muted);
		color: var(--color-text-muted);
		border: 1px solid var(--color-border-default);
		text-decoration: line-through;
	}

	/* Actions */
	.actions-cell {
		white-space: nowrap;
	}

	.close-trade-btn {
		background: var(--color-brand-secondary);
		color: white;
		border: none;
		padding: var(--space-2) var(--space-4);
		border-radius: var(--radius-md);
		font-size: var(--text-sm);
		font-weight: var(--font-semibold);
		cursor: pointer;
		transition: var(--transition-colors);
		-webkit-tap-highlight-color: transparent;
		touch-action: manipulation;
		min-height: 44px;
		min-width: 44px;
	}

	.close-trade-btn:hover {
		background: var(--color-brand-secondary-hover);
	}

	.close-trade-btn:focus-visible {
		outline: 2px solid var(--color-brand-secondary);
		outline-offset: 2px;
	}

	.closed-indicator {
		color: var(--color-text-muted);
		font-size: var(--text-xs);
	}

	/* Notes Row */
	.notes-row {
		display: none;
	}

	.notes-row.expanded {
		display: table-row;
	}

	.notes-row td {
		padding: 0 var(--space-3) var(--space-4);
		border-bottom: 2px solid var(--color-border-default);
	}

	.notes-content {
		font-size: var(--text-sm);
		color: var(--color-text-tertiary);
		line-height: 1.6;
		padding: var(--space-3) var(--space-4);
		background: var(--color-bg-subtle);
		border-radius: var(--radius-md);
		margin-top: var(--space-3);
	}

	.notes-content strong {
		color: var(--color-text-secondary);
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * TAILWIND UTILITY CLASSES (scoped)
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.hidden {
		display: none;
	}

	@media (min-width: 768px) {
		.md\:hidden {
			display: none !important;
		}

		.md\:block {
			display: block !important;
		}

		.md\:table-cell {
			display: table-cell !important;
		}
	}
</style>

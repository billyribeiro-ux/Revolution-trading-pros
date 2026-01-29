<script lang="ts">
	/**
	 * SearchResults - Display search results grouped by type
	 * @standards Svelte 5 January 2026 | Apple Principal Engineer ICT 7+ | WCAG 2.1 AA
	 */
	import type { SearchResults as SearchResultsType } from '../search.state.svelte';
	import AlertResultCard from './AlertResultCard.svelte';
	import TradeResultCard from './TradeResultCard.svelte';
	import TradePlanResultCard from './TradePlanResultCard.svelte';

	interface Props {
		results: SearchResultsType;
		query: string;
	}

	let { results, query }: Props = $props();

	// Count by type
	const alertCount = $derived(results.alerts.length);
	const tradeCount = $derived(results.trades.length);
	const planCount = $derived(results.trade_plans.length);
</script>

<div class="search-results">
	<!-- Alerts Section -->
	{#if alertCount > 0}
		<section class="result-section">
			<header class="section-header">
				<div class="section-title">
					<svg
						width="18"
						height="18"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
					>
						<path d="M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9" />
						<path d="M13.73 21a2 2 0 0 1-3.46 0" />
					</svg>
					<h2>Alerts</h2>
					<span class="count-badge">{alertCount}</span>
				</div>
			</header>
			<div class="result-list">
				{#each results.alerts as alert (alert.id)}
					<AlertResultCard {alert} {query} />
				{/each}
			</div>
		</section>
	{/if}

	<!-- Trades Section -->
	{#if tradeCount > 0}
		<section class="result-section">
			<header class="section-header">
				<div class="section-title">
					<svg
						width="18"
						height="18"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
					>
						<polyline points="22 7 13.5 15.5 8.5 10.5 2 17" />
						<polyline points="16 7 22 7 22 13" />
					</svg>
					<h2>Trades</h2>
					<span class="count-badge">{tradeCount}</span>
				</div>
			</header>
			<div class="result-list">
				{#each results.trades as trade (trade.id)}
					<TradeResultCard {trade} {query} />
				{/each}
			</div>
		</section>
	{/if}

	<!-- Trade Plans Section -->
	{#if planCount > 0}
		<section class="result-section">
			<header class="section-header">
				<div class="section-title">
					<svg
						width="18"
						height="18"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
					>
						<path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
						<polyline points="14 2 14 8 20 8" />
						<line x1="16" y1="13" x2="8" y2="13" />
						<line x1="16" y1="17" x2="8" y2="17" />
					</svg>
					<h2>Trade Plans</h2>
					<span class="count-badge">{planCount}</span>
				</div>
			</header>
			<div class="result-list">
				{#each results.trade_plans as plan (plan.id)}
					<TradePlanResultCard {plan} {query} />
				{/each}
			</div>
		</section>
	{/if}
</div>

<style>
	.search-results {
		display: flex;
		flex-direction: column;
		gap: 24px;
	}

	.result-section {
		background: var(--color-bg-card);
		border: 1px solid var(--color-border-default);
		border-radius: 12px;
		overflow: hidden;
	}

	.section-header {
		padding: 16px 20px;
		background: var(--color-bg-subtle);
		border-bottom: 1px solid var(--color-border-default);
	}

	.section-title {
		display: flex;
		align-items: center;
		gap: 10px;
		color: var(--color-text-primary);
	}

	.section-title svg {
		color: var(--color-text-tertiary);
	}

	.section-title h2 {
		font-size: 16px;
		font-weight: 600;
		margin: 0;
	}

	.count-badge {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		min-width: 24px;
		padding: 2px 8px;
		background: var(--color-brand-primary);
		color: white;
		font-size: 12px;
		font-weight: 600;
		border-radius: 12px;
	}

	.result-list {
		display: flex;
		flex-direction: column;
	}

	/* Mobile */
	@media (max-width: 768px) {
		.section-header {
			padding: 12px 16px;
		}
	}
</style>

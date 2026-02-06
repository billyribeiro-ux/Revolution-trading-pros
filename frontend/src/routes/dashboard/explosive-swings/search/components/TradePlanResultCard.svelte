<script lang="ts">
	/**
	 * TradePlanResultCard - Individual trade plan search result
	 * @standards Svelte 5 January 2026 | Apple Principal Engineer ICT 7+
	 */
	import type { TradePlanSearchResult } from '../search.state.svelte';

	interface Props {
		plan: TradePlanSearchResult;
		query: string;
	}

	let { plan, query }: Props = $props();

	// Highlight matching search terms in text
	function highlightMatch(text: string): string {
		if (!query || !text) return text;
		const escaped = query.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
		const regex = new RegExp(`(${escaped})`, 'gi');
		return text.replace(regex, '<mark>$1</mark>');
	}

	// Bias styling
	const biasStyles: Record<string, { bg: string; text: string }> = {
		BULLISH: { bg: 'bg-bullish', text: 'text-bullish' },
		BEARISH: { bg: 'bg-bearish', text: 'text-bearish' },
		NEUTRAL: { bg: 'bg-neutral', text: 'text-neutral' }
	};

	const biasStyle = $derived(biasStyles[plan.bias.toUpperCase()] || biasStyles.NEUTRAL);

	// Format date
	function formatDate(dateStr: string): string {
		const date = new Date(dateStr);
		return date.toLocaleDateString('en-US', {
			month: 'short',
			day: 'numeric',
			year: 'numeric'
		});
	}

	// Relevance score as percentage
	const relevancePercent = $derived(Math.round(plan.relevance_score * 100));
</script>

<article class="plan-card" class:inactive={!plan.is_active}>
	<div class="card-header">
		<span class="ticker-badge">{plan.ticker}</span>
		<span class="bias-badge {biasStyle.bg}">{plan.bias.toUpperCase()}</span>
		{#if !plan.is_active}
			<span class="inactive-badge">INACTIVE</span>
		{/if}
		<span class="date">Week of {formatDate(plan.week_of)}</span>
		<span class="relevance" title="Relevance score">
			<svg width="12" height="12" viewBox="0 0 24 24" fill="currentColor">
				<polygon
					points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"
				/>
			</svg>
			{relevancePercent}%
		</span>
	</div>

	<div class="plan-levels">
		{#if plan.entry}
			<div class="level">
				<span class="level-label">Entry</span>
				<span class="level-value">{plan.entry}</span>
			</div>
		{/if}

		{#if plan.target1}
			<div class="level">
				<span class="level-label">Target 1</span>
				<span class="level-value target">{plan.target1}</span>
			</div>
		{/if}

		{#if plan.stop}
			<div class="level">
				<span class="level-label">Stop</span>
				<span class="level-value stop">{plan.stop}</span>
			</div>
		{/if}
	</div>

	{#if plan.notes}
		<div class="card-highlight">
			{@html highlightMatch(plan.highlight)}
		</div>
	{/if}
</article>

<style>
	.plan-card {
		padding: 16px 20px;
		border-left: 4px solid var(--color-brand-secondary);
		transition: background 0.15s;
	}

	.plan-card:hover {
		background: var(--color-bg-subtle);
	}

	.plan-card:not(:last-child) {
		border-bottom: 1px solid var(--color-border-default);
	}

	.plan-card.inactive {
		opacity: 0.6;
		border-left-color: var(--color-text-tertiary);
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

	.bias-badge {
		display: inline-flex;
		padding: 3px 8px;
		font-size: 10px;
		font-weight: 600;
		border-radius: 4px;
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.bg-bullish {
		background: rgba(16, 185, 129, 0.15);
		color: #059669;
	}

	.bg-bearish {
		background: rgba(239, 68, 68, 0.15);
		color: #dc2626;
	}

	.bg-neutral {
		background: rgba(107, 114, 128, 0.15);
		color: #6b7280;
	}

	.inactive-badge {
		display: inline-flex;
		padding: 3px 8px;
		background: var(--color-bg-subtle);
		color: var(--color-text-tertiary);
		font-size: 10px;
		font-weight: 600;
		border-radius: 4px;
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

	.plan-levels {
		display: flex;
		flex-wrap: wrap;
		gap: 16px;
		margin-bottom: 12px;
	}

	.level {
		display: flex;
		flex-direction: column;
		gap: 2px;
	}

	.level-label {
		font-size: 11px;
		color: var(--color-text-tertiary);
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.level-value {
		font-size: 14px;
		font-weight: 600;
		color: var(--color-text-primary);
	}

	.level-value.target {
		color: var(--color-win);
	}

	.level-value.stop {
		color: var(--color-loss);
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
		.plan-card {
			padding: 12px 16px;
		}

		.relevance {
			order: 99;
			width: 100%;
			margin-left: 0;
			margin-top: 4px;
		}

		.plan-levels {
			gap: 12px;
		}
	}
</style>

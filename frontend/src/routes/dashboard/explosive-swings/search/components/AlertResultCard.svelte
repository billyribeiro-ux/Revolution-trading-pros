<script lang="ts">
	/**
	 * AlertResultCard - Individual alert search result
	 * @standards Svelte 5 January 2026 | Apple Principal Engineer ICT 7+
	 */
	import type { AlertSearchResult } from '../search.state.svelte';

	interface Props {
		alert: AlertSearchResult;
		query: string;
	}

	let { alert, query }: Props = $props();

	// Alert type styling
	const typeStyles: Record<string, { bg: string; text: string; border: string }> = {
		ENTRY: { bg: 'bg-teal', text: 'text-teal', border: 'border-teal' },
		EXIT: { bg: 'bg-emerald', text: 'text-emerald', border: 'border-emerald' },
		UPDATE: { bg: 'bg-amber', text: 'text-amber', border: 'border-amber' }
	};

	const style = typeStyles[alert.alert_type] || typeStyles.UPDATE;

	// Format date
	function formatDate(dateStr: string): string {
		const date = new Date(dateStr);
		return date.toLocaleDateString('en-US', {
			month: 'short',
			day: 'numeric',
			year: 'numeric',
			hour: 'numeric',
			minute: '2-digit'
		});
	}

	// Format relevance score as percentage
	const relevancePercent = $derived(Math.round(alert.relevance_score * 100));
</script>

<article class="alert-card {style.border}">
	<div class="card-header">
		<span class="ticker-badge">{alert.ticker}</span>
		<span class="type-badge {style.bg}">{alert.alert_type}</span>
		<span class="date">{formatDate(alert.published_at)}</span>
		<span class="relevance" title="Relevance score">
			<svg width="12" height="12" viewBox="0 0 24 24" fill="currentColor">
				<polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2" />
			</svg>
			{relevancePercent}%
		</span>
	</div>

	{#if alert.title}
		<h3 class="card-title">{alert.title}</h3>
	{/if}

	<div class="card-highlight">
		{@html alert.highlight}
	</div>

	<div class="card-message">
		{alert.message}
	</div>
</article>

<style>
	.alert-card {
		padding: 16px 20px;
		border-left: 4px solid;
		transition: background 0.15s;
	}

	.alert-card:hover {
		background: var(--color-bg-subtle);
	}

	.alert-card:not(:last-child) {
		border-bottom: 1px solid var(--color-border-default);
	}

	.border-teal {
		border-left-color: var(--color-win);
	}

	.border-emerald {
		border-left-color: #10b981;
	}

	.border-amber {
		border-left-color: #f59e0b;
	}

	.card-header {
		display: flex;
		align-items: center;
		gap: 10px;
		margin-bottom: 8px;
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

	.type-badge {
		display: inline-flex;
		padding: 3px 8px;
		font-size: 10px;
		font-weight: 600;
		border-radius: 4px;
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.bg-teal {
		background: rgba(20, 184, 166, 0.15);
		color: #0d9488;
	}

	.bg-emerald {
		background: rgba(16, 185, 129, 0.15);
		color: #059669;
	}

	.bg-amber {
		background: rgba(245, 158, 11, 0.15);
		color: #d97706;
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

	.card-title {
		font-size: 15px;
		font-weight: 600;
		color: var(--color-text-primary);
		margin: 0 0 8px 0;
	}

	.card-highlight {
		font-size: 14px;
		color: var(--color-text-secondary);
		line-height: 1.5;
		margin-bottom: 8px;
	}

	.card-highlight :global(mark) {
		background: rgba(var(--color-brand-primary-rgb), 0.2);
		color: var(--color-text-primary);
		padding: 1px 3px;
		border-radius: 2px;
	}

	.card-message {
		font-size: 13px;
		color: var(--color-text-tertiary);
		line-height: 1.4;
		overflow: hidden;
		text-overflow: ellipsis;
		display: -webkit-box;
		-webkit-line-clamp: 2;
		-webkit-box-orient: vertical;
	}

	/* Mobile */
	@media (max-width: 768px) {
		.alert-card {
			padding: 12px 16px;
		}

		.relevance {
			order: 99;
			width: 100%;
			margin-left: 0;
			margin-top: 4px;
		}
	}
</style>

<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * EmptyState Component - No Trades Found
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @version 1.0.0
	 * @standards Apple Principal Engineer ICT 7+ | WCAG 2.1 AA
	 */
	import type { FilterStatus } from '../types';

	interface Props {
		filter: FilterStatus;
		onReset?: () => void;
	}

	const { filter, onReset }: Props = $props();

	const messages: Record<FilterStatus, { title: string; description: string }> = {
		all: {
			title: 'No trades yet',
			description: 'Trades will appear here once they are recorded.'
		},
		active: {
			title: 'No active trades',
			description: 'All positions have been closed.'
		},
		win: {
			title: 'No winning trades',
			description: 'Winning trades will appear here.'
		},
		loss: {
			title: 'No losing trades',
			description: 'Great job! No losses recorded.'
		}
	};
</script>

<div class="empty-state" role="status">
	<div class="empty-icon">
		<svg
			viewBox="0 0 24 24"
			fill="none"
			stroke="currentColor"
			stroke-width="1.5"
			width="48"
			height="48"
		>
			<path
				d="M9 17v-2m3 2v-4m3 4v-6m2 10H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
			/>
		</svg>
	</div>
	<h3 class="empty-title">{messages[filter].title}</h3>
	<p class="empty-description">{messages[filter].description}</p>
	{#if filter !== 'all' && onReset}
		<button type="button" class="reset-btn" onclick={onReset}> View All Trades </button>
	{/if}
</div>

<style>
	.empty-state {
		text-align: center;
		padding: var(--space-12) var(--space-6);
	}

	.empty-icon {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 80px;
		height: 80px;
		background: var(--color-bg-subtle);
		border-radius: var(--radius-xl);
		margin-bottom: var(--space-5);
		color: var(--color-text-muted);
	}

	.empty-title {
		font-size: var(--text-lg);
		font-weight: var(--font-semibold);
		color: var(--color-text-primary);
		margin: 0 0 var(--space-2) 0;
	}

	.empty-description {
		font-size: var(--text-sm);
		color: var(--color-text-tertiary);
		margin: 0 0 var(--space-5) 0;
	}

	.reset-btn {
		background: var(--color-brand-primary);
		color: white;
		border: none;
		padding: var(--space-2) var(--space-5);
		border-radius: var(--radius-md);
		font-size: var(--text-sm);
		font-weight: var(--font-semibold);
		cursor: pointer;
		transition: var(--transition-colors);
	}

	.reset-btn:hover {
		background: var(--color-brand-primary-hover);
	}

	.reset-btn:focus-visible {
		outline: 2px solid var(--color-brand-primary);
		outline-offset: 2px;
	}
</style>

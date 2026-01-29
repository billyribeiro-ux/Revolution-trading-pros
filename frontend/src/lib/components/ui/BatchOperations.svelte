<script lang="ts">
	/**
	 * BatchOperations Component
	 * Toolbar for batch operations on selected items
	 *
	 * @version 1.0.0
	 * @author Revolution Trading Pros
	 */

	interface Action {
		id: string;
		label: string;
		icon?: string;
		variant?: 'default' | 'danger' | 'success';
		disabled?: boolean;
	}

	interface Props {
		selectedCount: number;
		totalCount?: number;
		actions?: Action[];
		onAction?: (actionId: string) => void;
		onSelectAll?: () => void;
		onClearSelection?: () => void;
	}

	const {
		selectedCount,
		totalCount,
		actions = [],
		onAction,
		onSelectAll,
		onClearSelection
	}: Props = $props();

	const isVisible = $derived(selectedCount > 0);
</script>

{#if isVisible}
	<div class="batch-operations">
		<div class="selection-info">
			<span class="selection-count">{selectedCount} selected</span>
			{#if totalCount}
				<span class="selection-total">of {totalCount}</span>
			{/if}

			<div class="selection-actions">
				{#if onSelectAll && totalCount && selectedCount < totalCount}
					<button type="button" class="link-btn" onclick={onSelectAll}> Select all </button>
				{/if}
				{#if onClearSelection}
					<button type="button" class="link-btn" onclick={onClearSelection}> Clear </button>
				{/if}
			</div>
		</div>

		<div class="action-buttons">
			{#each actions as action}
				<button
					type="button"
					class="action-btn variant-{action.variant ?? 'default'}"
					disabled={action.disabled}
					onclick={() => onAction?.(action.id)}
				>
					{#if action.icon}
						<span class="action-icon">{action.icon}</span>
					{/if}
					<span>{action.label}</span>
				</button>
			{/each}
		</div>
	</div>
{/if}

<style>
	.batch-operations {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		padding: 0.75rem 1rem;
		background: var(--color-bg-secondary, #f3f4f6);
		border: 1px solid var(--color-border-default, #e5e7eb);
		border-radius: var(--radius-lg, 0.5rem);
		flex-wrap: wrap;
	}

	.selection-info {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.875rem;
	}

	.selection-count {
		font-weight: 600;
		color: var(--color-text-primary, #111827);
	}

	.selection-total {
		color: var(--color-text-muted, #9ca3af);
	}

	.selection-actions {
		display: flex;
		gap: 0.75rem;
		margin-left: 0.5rem;
		padding-left: 0.75rem;
		border-left: 1px solid var(--color-border-default, #e5e7eb);
	}

	.link-btn {
		background: none;
		border: none;
		color: var(--color-primary, #6366f1);
		font-size: 0.875rem;
		cursor: pointer;
		padding: 0;
	}

	.link-btn:hover {
		text-decoration: underline;
	}

	.action-buttons {
		display: flex;
		gap: 0.5rem;
		flex-wrap: wrap;
	}

	.action-btn {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.5rem 0.875rem;
		font-size: 0.875rem;
		font-weight: 500;
		border-radius: var(--radius-md, 0.375rem);
		cursor: pointer;
		transition: all 0.15s;
		border: none;
		min-height: 36px;
	}

	.action-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.variant-default {
		background: var(--color-bg-tertiary, #e5e7eb);
		color: var(--color-text-primary, #111827);
	}

	.variant-default:hover:not(:disabled) {
		background: var(--color-bg-hover, #d1d5db);
	}

	.variant-danger {
		background: var(--color-error, #dc2626);
		color: white;
	}

	.variant-danger:hover:not(:disabled) {
		background: var(--color-error-hover, #b91c1c);
	}

	.variant-success {
		background: var(--color-success, #16a34a);
		color: white;
	}

	.variant-success:hover:not(:disabled) {
		background: var(--color-success-hover, #15803d);
	}

	/* Mobile */
	@media (max-width: 640px) {
		.batch-operations {
			flex-direction: column;
			align-items: stretch;
		}

		.action-buttons {
			justify-content: center;
		}

		.action-btn {
			min-height: 44px;
		}
	}
</style>

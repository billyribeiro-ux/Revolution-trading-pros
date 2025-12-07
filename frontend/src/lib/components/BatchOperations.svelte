<script lang="ts">
	/**
	 * Batch Operations Component - Apple ICT9+ Design
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * Floating action bar for bulk operations on selected items.
	 *
	 * @version 1.0.0
	 */

	import { fade, fly } from 'svelte/transition';
	import { quintOut } from 'svelte/easing';
	import {
		IconTrash,
		IconArchive,
		IconDownload,
		IconTag,
		IconMail,
		IconX,
		IconCheck,
		IconLoader2
	} from '@tabler/icons-svelte';

	interface BatchAction {
		id: string;
		label: string;
		icon: typeof IconTrash;
		variant?: 'default' | 'danger' | 'success';
		action: (selectedIds: string[]) => Promise<void>;
	}

	interface Props {
		selectedIds: string[];
		totalCount: number;
		resourceName?: string;
		actions?: BatchAction[];
		onClearSelection?: () => void;
		onSelectAll?: () => void;
	}

	let {
		selectedIds = [],
		totalCount = 0,
		resourceName = 'items',
		actions = [],
		onClearSelection,
		onSelectAll
	}: Props = $props();

	let isProcessing = $state(false);
	let processingAction = $state<string | null>(null);

	// Default actions if none provided
	const defaultActions: BatchAction[] = [
		{
			id: 'delete',
			label: 'Delete',
			icon: IconTrash,
			variant: 'danger',
			action: async (ids) => {
				console.log('Deleting:', ids);
				await new Promise(r => setTimeout(r, 1000));
			}
		},
		{
			id: 'archive',
			label: 'Archive',
			icon: IconArchive,
			action: async (ids) => {
				console.log('Archiving:', ids);
				await new Promise(r => setTimeout(r, 1000));
			}
		},
		{
			id: 'export',
			label: 'Export',
			icon: IconDownload,
			action: async (ids) => {
				console.log('Exporting:', ids);
				await new Promise(r => setTimeout(r, 1000));
			}
		}
	];

	let effectiveActions = $derived(actions.length > 0 ? actions : defaultActions);
	let isVisible = $derived(selectedIds.length > 0);
	let allSelected = $derived(selectedIds.length === totalCount && totalCount > 0);

	async function handleAction(action: BatchAction) {
		if (isProcessing) return;

		isProcessing = true;
		processingAction = action.id;

		try {
			await action.action(selectedIds);
			onClearSelection?.();
		} catch (error) {
			console.error('Batch action failed:', error);
		} finally {
			isProcessing = false;
			processingAction = null;
		}
	}

	function getVariantClass(variant?: string) {
		switch (variant) {
			case 'danger': return 'action-danger';
			case 'success': return 'action-success';
			default: return '';
		}
	}
</script>

{#if isVisible}
	<div
		class="batch-operations"
		in:fly={{ y: 50, duration: 300, easing: quintOut }}
		out:fade={{ duration: 150 }}
	>
		<div class="batch-container">
			<!-- Selection Info -->
			<div class="selection-info">
				<div class="selection-count">
					<IconCheck size={16} />
					<span class="count">{selectedIds.length}</span>
					<span class="label">selected</span>
				</div>

				{#if !allSelected && totalCount > selectedIds.length}
					<button class="select-all-btn" onclick={onSelectAll}>
						Select all {totalCount.toLocaleString()} {resourceName}
					</button>
				{/if}

				<button class="clear-btn" onclick={onClearSelection} title="Clear selection">
					<IconX size={16} />
				</button>
			</div>

			<!-- Divider -->
			<div class="divider"></div>

			<!-- Actions -->
			<div class="batch-actions">
				{#each effectiveActions as action}
					<button
						class="action-btn {getVariantClass(action.variant)}"
						onclick={() => handleAction(action)}
						disabled={isProcessing}
					>
						{#if processingAction === action.id}
							<IconLoader2 size={18} class="spinning" />
						{:else}
							<svelte:component this={action.icon} size={18} />
						{/if}
						<span>{action.label}</span>
					</button>
				{/each}
			</div>
		</div>
	</div>
{/if}

<style>
	.batch-operations {
		position: fixed;
		bottom: 2rem;
		left: 50%;
		transform: translateX(-50%);
		z-index: 1000;
	}

	.batch-container {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 0.875rem 1.25rem;
		background: linear-gradient(135deg, rgba(30, 41, 59, 0.98), rgba(15, 23, 42, 0.98));
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 16px;
		box-shadow:
			0 20px 50px rgba(0, 0, 0, 0.4),
			0 0 0 1px rgba(255, 255, 255, 0.05) inset;
		backdrop-filter: blur(20px);
	}

	.selection-info {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.selection-count {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.5rem 0.875rem;
		background: rgba(99, 102, 241, 0.15);
		border-radius: 10px;
		color: #a5b4fc;
	}

	.selection-count .count {
		font-weight: 700;
		font-size: 1rem;
	}

	.selection-count .label {
		font-size: 0.8125rem;
		color: #94a3b8;
	}

	.select-all-btn {
		padding: 0.5rem 0.875rem;
		background: transparent;
		border: 1px solid rgba(99, 102, 241, 0.3);
		border-radius: 8px;
		color: #a5b4fc;
		font-size: 0.8125rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.select-all-btn:hover {
		background: rgba(99, 102, 241, 0.1);
		border-color: rgba(99, 102, 241, 0.5);
	}

	.clear-btn {
		width: 32px;
		height: 32px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(99, 102, 241, 0.1);
		border: none;
		border-radius: 8px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
	}

	.clear-btn:hover {
		background: rgba(99, 102, 241, 0.2);
		color: #f1f5f9;
	}

	.divider {
		width: 1px;
		height: 32px;
		background: rgba(99, 102, 241, 0.2);
	}

	.batch-actions {
		display: flex;
		gap: 0.5rem;
	}

	.action-btn {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1rem;
		background: rgba(99, 102, 241, 0.1);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 10px;
		color: #a5b4fc;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.action-btn:hover:not(:disabled) {
		background: rgba(99, 102, 241, 0.2);
		transform: translateY(-1px);
	}

	.action-btn:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.action-btn.action-danger {
		background: rgba(239, 68, 68, 0.1);
		border-color: rgba(239, 68, 68, 0.2);
		color: #f87171;
	}

	.action-btn.action-danger:hover:not(:disabled) {
		background: rgba(239, 68, 68, 0.2);
	}

	.action-btn.action-success {
		background: rgba(16, 185, 129, 0.1);
		border-color: rgba(16, 185, 129, 0.2);
		color: #34d399;
	}

	.action-btn.action-success:hover:not(:disabled) {
		background: rgba(16, 185, 129, 0.2);
	}

	:global(.spinning) {
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		from { transform: rotate(0deg); }
		to { transform: rotate(360deg); }
	}

	@media (max-width: 768px) {
		.batch-operations {
			left: 1rem;
			right: 1rem;
			transform: none;
		}

		.batch-container {
			flex-wrap: wrap;
			justify-content: center;
		}

		.select-all-btn {
			display: none;
		}

		.action-btn span {
			display: none;
		}

		.action-btn {
			padding: 0.625rem;
		}
	}
</style>

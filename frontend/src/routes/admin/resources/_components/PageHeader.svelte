<script lang="ts">
	import { IconRefresh, IconPlus } from '$lib/icons';

	type Props = {
		selectedCount: number;
		isLoading: boolean;
		hasSelectedRoom: boolean;
		onBulk: () => void;
		onRefresh: () => void;
		onCreate: () => void;
	};

	const { selectedCount, isLoading, hasSelectedRoom, onBulk, onRefresh, onCreate }: Props =
		$props();
</script>

<div class="page-header">
	<h1>Room Resources</h1>
	<p class="subtitle">Manage videos, PDFs, documents, and images for each trading room</p>
	<div class="header-actions">
		{#if selectedCount > 0}
			<button class="btn-warning" onclick={onBulk}>
				Bulk Actions ({selectedCount})
			</button>
		{/if}
		<button class="btn-secondary" onclick={onRefresh} disabled={isLoading}>
			<IconRefresh size={18} class={isLoading ? 'spinning' : ''} />
		</button>
		<button class="btn-primary" onclick={onCreate} disabled={!hasSelectedRoom}>
			<IconPlus size={18} />
			Add Resource
		</button>
	</div>
</div>

<style>
	/* Header - Centered */
	.page-header {
		text-align: center;
		margin-bottom: 2rem;
	}

	.page-header h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0 0 0.5rem 0;
	}

	.subtitle {
		color: #64748b;
		font-size: 0.875rem;
		margin: 0 0 1rem 0;
	}

	.header-actions {
		display: flex;
		justify-content: center;
		gap: 0.75rem;
	}

	/* Buttons */
	.btn-primary {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background: linear-gradient(135deg, var(--primary-500) 0%, var(--primary-600) 100%);
		color: var(--bg-base);
		border: none;
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-primary:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow: 0 4px 15px rgba(230, 184, 0, 0.4);
	}

	.btn-primary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.btn-secondary {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background: rgba(230, 184, 0, 0.1);
		color: #94a3b8;
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-secondary:hover {
		background: rgba(230, 184, 0, 0.2);
		color: #e2e8f0;
	}

	.btn-warning {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background: linear-gradient(135deg, #f59e0b 0%, #d97706 100%);
		color: #fff;
		border: none;
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-warning:hover {
		transform: translateY(-2px);
		box-shadow: 0 4px 15px rgba(245, 158, 11, 0.4);
	}

	/* Mobile */
	@media (max-width: 639px) {
		.page-header h1 {
			font-size: 1.375rem;
		}

		.subtitle {
			font-size: 0.8125rem;
		}

		.header-actions {
			flex-wrap: wrap;
			gap: 0.5rem;
		}

		.btn-primary,
		.btn-secondary {
			min-height: 44px;
			padding: 0.625rem 1rem;
			font-size: 0.875rem;
		}
	}

	@media (min-width: 640px) {
		.page-header h1 {
			font-size: 1.5rem;
		}
	}

	@media (min-width: 768px) {
		.page-header h1 {
			font-size: 1.75rem;
		}
	}
</style>

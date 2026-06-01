<script lang="ts">
	/**
	 * R20-C extraction (2026-05-20): page-header for the product edit page.
	 * Pure leaf: back-button + title + delete trigger.
	 */
	import { IconArrowLeft, IconTrash } from '$lib/icons';

	interface Props {
		productName: string | null;
		deleting: boolean;
		onBack: () => void;
		onDelete: () => void;
	}

	let { productName, deleting, onBack, onDelete }: Props = $props();
</script>

<div class="page-header">
	<button class="back-btn" onclick={onBack}>
		<IconArrowLeft size={20} />
		Back to Products
	</button>
	<div class="header-content">
		<div>
			<h1>Edit Product</h1>
			{#if productName}
				<p>Editing: {productName}</p>
			{/if}
		</div>
		{#if productName}
			<button class="btn-danger" onclick={onDelete} disabled={deleting}>
				<IconTrash size={18} />
				Delete Product
			</button>
		{/if}
	</div>
</div>

<style>
	.page-header {
		margin-bottom: 2rem;
	}

	.back-btn {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		background: transparent;
		border: none;
		color: #94a3b8;
		font-size: 0.875rem;
		cursor: pointer;
		margin-bottom: 1rem;
		padding: 0;
		transition: color 0.2s;
	}

	.back-btn:hover {
		color: #f1f5f9;
	}

	.header-content {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
	}

	.page-header h1 {
		font-size: 2rem;
		font-weight: 700;
		color: #f1f5f9;
		margin-bottom: 0.5rem;
	}

	.page-header p {
		color: #94a3b8;
		font-size: 0.95rem;
	}

	.btn-danger {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		border-radius: 8px;
		font-weight: 600;
		border: 1px solid rgba(239, 68, 68, 0.3);
		cursor: pointer;
		transition: all 0.2s;
		background: rgba(239, 68, 68, 0.1);
		color: #f87171;
	}

	.btn-danger:hover:not(:disabled) {
		background: rgba(239, 68, 68, 0.2);
	}

	.btn-danger:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	@media (max-width: 1023.98px) {
		.header-content {
			flex-direction: column;
			gap: 1rem;
		}
	}
</style>

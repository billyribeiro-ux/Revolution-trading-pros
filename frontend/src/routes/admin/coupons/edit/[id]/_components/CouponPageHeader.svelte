<script lang="ts">
	/**
	 * R23-C extraction (2026-05-20): page header — title, preview badge,
	 * cancel / delete / save action buttons. Discrete callback props
	 * (R8-C discriminated-callback style: each callback names its action).
	 */
	import { IconTicket, IconCheck, IconX, IconRefresh, IconTrash } from '$lib/icons';
	import type { Coupon } from '$lib/api/admin';

	interface Props {
		originalCoupon: Coupon | null;
		isActive: boolean;
		discountDisplay: string;
		hasCode: boolean;
		loading: boolean;
		saving: boolean;
		deleting: boolean;
		hasChanges: boolean;
		onCancel: () => void;
		onDelete: () => void;
		onSave: () => void;
	}

	let {
		originalCoupon,
		isActive,
		discountDisplay,
		hasCode,
		loading,
		saving,
		deleting,
		hasChanges,
		onCancel,
		onDelete,
		onSave
	}: Props = $props();
</script>

<div class="page-header">
	<div class="header-content">
		<div class="header-main">
			<h1>
				<IconTicket size={32} />
				Edit Coupon
			</h1>
			{#if originalCoupon}
				<p class="header-subtitle">Editing: <strong>{originalCoupon.code}</strong></p>
			{/if}
		</div>

		{#if !loading && hasCode}
			<div class="preview-badge">
				<span class="preview-value">{discountDisplay}</span>
				<span class={['preview-status', { active: isActive }]}>
					{isActive ? 'Active' : 'Inactive'}
				</span>
			</div>
		{/if}
	</div>

	<div class="header-actions">
		<button class="btn-ghost" onclick={onCancel}>
			<IconX size={18} />
			Cancel
		</button>
		<button class="btn-danger" onclick={onDelete} disabled={deleting || loading}>
			{#if deleting}
				<IconRefresh size={18} class="spinning" />
			{:else}
				<IconTrash size={18} />
			{/if}
			Delete
		</button>
		<button class="btn-primary" onclick={onSave} disabled={saving || loading || !hasChanges}>
			{#if saving}
				<IconRefresh size={18} class="spinning" />
				Saving...
			{:else}
				<IconCheck size={18} />
				Save Changes
			{/if}
		</button>
	</div>
</div>

<style>
	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 2rem;
		padding-bottom: 1.5rem;
		border-bottom: 1px solid rgba(148, 163, 184, 0.1);
	}

	.header-content {
		flex: 1;
	}

	.header-main h1 {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		font-size: 2rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0 0 0.5rem 0;
	}

	.header-subtitle {
		color: #94a3b8;
		font-size: 0.95rem;
		margin: 0;
	}

	.header-subtitle strong {
		color: #60a5fa;
		font-family: monospace;
	}

	.preview-badge {
		display: inline-flex;
		align-items: center;
		gap: 1rem;
		padding: 0.75rem 1.25rem;
		background: rgba(30, 41, 59, 0.8);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 12px;
		margin-top: 1rem;
	}

	.preview-value {
		font-size: 1.25rem;
		font-weight: 600;
		color: #3b82f6;
	}

	.preview-status {
		padding: 0.25rem 0.75rem;
		background: rgba(239, 68, 68, 0.1);
		color: #f87171;
		border-radius: 6px;
		font-size: 0.8rem;
		font-weight: 600;
	}

	.preview-status.active {
		background: rgba(16, 185, 129, 0.1);
		color: #34d399;
	}

	.header-actions {
		display: flex;
		gap: 0.75rem;
	}

	.btn-primary,
	.btn-ghost,
	.btn-danger {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		border-radius: 8px;
		font-weight: 600;
		border: none;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-primary {
		background: linear-gradient(135deg, var(--primary-500), var(--primary-600));
		color: white;
	}

	.btn-primary:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow: 0 10px 30px rgba(59, 130, 246, 0.3);
	}

	.btn-ghost {
		background: transparent;
		color: #94a3b8;
	}

	.btn-ghost:hover {
		background: rgba(148, 163, 184, 0.1);
	}

	.btn-danger {
		background: rgba(239, 68, 68, 0.1);
		color: #f87171;
		border: 1px solid rgba(239, 68, 68, 0.3);
	}

	.btn-danger:hover:not(:disabled) {
		background: rgba(239, 68, 68, 0.2);
	}

	button:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	:global(.spinning) {
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}

	@media (max-width: 767.98px) {
		.page-header {
			flex-direction: column;
			gap: 1rem;
		}

		.header-actions {
			width: 100%;
			justify-content: flex-end;
		}
	}
</style>

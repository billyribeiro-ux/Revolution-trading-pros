<script lang="ts">
	/**
	 * R27-C extraction (2026-05-20): page header for the admin coupons list.
	 * Title + subtitle + Refresh / Create action buttons. The parent owns
	 * the `loading` flag (drives the spinning refresh icon + disabled state)
	 * and the two click handlers — this component is presentation-only.
	 */
	import { IconPlus, IconRefresh } from '$lib/icons';

	interface Props {
		loading: boolean;
		onRefresh: () => void;
		onCreate: () => void;
	}

	let { loading, onRefresh, onCreate }: Props = $props();
</script>

<header class="page-header">
	<h1>Coupons Management</h1>
	<p class="subtitle">Manage discount codes and promotional offers</p>
	<div class="header-actions">
		<button class="btn-secondary" onclick={onRefresh} disabled={loading}>
			<IconRefresh size={18} class={loading ? 'spinning' : ''} />
			Refresh
		</button>
		<button class="btn-primary" onclick={onCreate}>
			<IconPlus size={18} />
			Create Coupon
		</button>
	</div>
</header>

<style>
	.page-header {
		text-align: center;
		margin-bottom: 1.5rem;
	}

	.page-header h1 {
		font-size: 1.5rem;
		font-weight: 700;
		color: var(--text-primary);
		margin: 0 0 0.375rem 0;
	}

	.subtitle {
		color: var(--text-tertiary);
		font-size: 0.875rem;
		margin: 0 0 1.5rem 0;
	}

	.header-actions {
		display: flex;
		justify-content: center;
		align-items: center;
		gap: 0.75rem;
	}

	.btn-primary,
	.btn-secondary {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 0.5rem 1rem;
		border-radius: 8px;
		font-size: 0.875rem;
		font-weight: 600;
		border: none;
		cursor: pointer;
		transition: all 0.2s ease;
		white-space: nowrap;
		flex-shrink: 0;
	}

	.btn-primary {
		background: linear-gradient(135deg, var(--primary-500), var(--primary-600));
		color: var(--bg-base);
	}

	.btn-primary:hover:not(:disabled) {
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(230, 184, 0, 0.3);
	}

	.btn-secondary {
		background: var(--bg-surface);
		color: var(--text-primary);
		border: 1px solid var(--border-default);
	}

	.btn-secondary:hover:not(:disabled) {
		background: var(--bg-hover);
		border-color: var(--border-emphasis);
	}

	button:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	@media (max-width: 767.98px) {
		.page-header h1 {
			font-size: 1.25rem;
		}

		.header-actions {
			gap: 0.5rem;
		}

		.btn-primary,
		.btn-secondary {
			padding: 0.5rem 0.875rem;
			font-size: 0.8125rem;
		}
	}
</style>

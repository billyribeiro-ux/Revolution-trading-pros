<script lang="ts">
	/**
	 * R27-C extraction (2026-05-20): single coupon card rendered inside
	 * the coupons-grid on the admin list page. Owns the canonical-vs-
	 * legacy field fallback (`discount_type`/`type`, `expires_at`/
	 * `valid_until`, `min_purchase`/`minimum_amount`) so the parent
	 * loop stays clean and FIX-2026-04-26 (P2-10) lives in one place.
	 *
	 * All actions (toggle / edit / delete) are dispatched via callback
	 * props — the parent owns API calls, optimistic updates, and the
	 * delete-confirmation modal.
	 */
	import { IconEdit, IconRefresh, IconTrash } from '$lib/icons';
	import type { Coupon } from './types';

	interface Props {
		coupon: Coupon;
		deleting: boolean;
		onToggleStatus: (coupon: Coupon) => void;
		onEdit: (id: number) => void;
		onRequestDelete: (id: number) => void;
	}

	let { coupon, deleting, onToggleStatus, onEdit, onRequestDelete }: Props = $props();

	// Canonical-first reads, with legacy-alias fallback.
	let expiresAt = $derived(coupon.expires_at ?? coupon.valid_until);
	let minPurchase = $derived(coupon.min_purchase ?? coupon.minimum_amount);
	let discountTypeLabel = $derived(coupon.discount_type ?? coupon.type ?? '—');
	let expired = $derived(!!expiresAt && new Date(expiresAt) < new Date());

	function formatDiscountValue(c: Coupon): string {
		const dtype = c.discount_type ?? c.type;
		const dvalue = c.discount_value ?? c.value ?? 0;
		if (dtype === 'percentage') return `${dvalue}% off`;
		if (dtype === 'fixed') return `$${dvalue} off`;
		if (dtype === 'free_shipping') return 'Free Shipping';
		return `${dvalue}`;
	}

	function formatDate(dateString: string | null | undefined): string {
		if (!dateString) return 'No expiry';
		return new Date(dateString).toLocaleDateString('en-US', {
			month: 'short',
			day: 'numeric',
			year: 'numeric'
		});
	}
</script>

<div class={{ 'coupon-card': true, expired }}>
	<div class="coupon-header">
		<div class="coupon-code">{coupon.code}</div>
		<button
			class={{ 'coupon-status': true, active: coupon.is_active }}
			onclick={() => onToggleStatus(coupon)}
			title="Click to toggle status"
		>
			{coupon.is_active ? 'Active' : 'Inactive'}
		</button>
	</div>

	<div class="coupon-details">
		<span class="coupon-type">{discountTypeLabel}</span>
		<span class="coupon-value">{formatDiscountValue(coupon)}</span>
	</div>

	<div class="coupon-meta">
		{#if coupon.usage_limit}
			<span class="meta-item">
				Uses: {coupon.usage_count}/{coupon.usage_limit}
			</span>
		{/if}
		{#if expiresAt}
			<span class={{ 'meta-item': true, expired }}>
				Expires: {formatDate(expiresAt)}
			</span>
		{/if}
		{#if minPurchase}
			<span class="meta-item">
				Min: ${minPurchase}
			</span>
		{/if}
	</div>

	<div class="coupon-actions">
		<button class="action-btn edit" onclick={() => onEdit(coupon.id)}>
			<IconEdit size={16} />
			Edit
		</button>
		<button
			class="action-btn delete"
			onclick={() => onRequestDelete(coupon.id)}
			disabled={deleting}
		>
			{#if deleting}
				<IconRefresh size={16} class="spinning" />
			{:else}
				<IconTrash size={16} />
			{/if}
			Delete
		</button>
	</div>
</div>

<style>
	.coupon-card {
		background: var(--bg-elevated);
		border: 1px solid var(--border-muted);
		border-radius: 12px;
		padding: 1.25rem;
		transition: all 0.2s ease;
	}

	.coupon-card:hover {
		border-color: var(--border-default);
		transform: translateY(-2px);
		box-shadow: 0 8px 24px rgba(0, 0, 0, 0.2);
	}

	.coupon-card.expired {
		opacity: 0.6;
		border-color: var(--error-base);
	}

	.coupon-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 0.875rem;
	}

	.coupon-code {
		font-size: 1.125rem;
		font-weight: 700;
		color: var(--primary-400);
		font-family: 'SF Mono', Monaco, monospace;
		letter-spacing: 0.05em;
	}

	.coupon-details {
		display: flex;
		gap: 0.5rem;
		margin-bottom: 0.875rem;
	}

	.coupon-type,
	.coupon-value {
		padding: 0.25rem 0.5rem;
		background: var(--bg-surface);
		border-radius: 4px;
		font-size: 0.75rem;
		color: var(--text-secondary);
		text-transform: capitalize;
	}

	.coupon-value {
		background: rgba(230, 184, 0, 0.1);
		color: var(--primary-400);
		font-weight: 600;
	}

	.coupon-meta {
		display: flex;
		flex-wrap: wrap;
		gap: 0.75rem;
		margin-bottom: 0.875rem;
	}

	.meta-item {
		font-size: 0.75rem;
		color: var(--text-tertiary);
	}

	.meta-item.expired {
		color: var(--error-emphasis);
	}

	.coupon-status {
		display: inline-block;
		padding: 0.25rem 0.625rem;
		background: var(--error-soft);
		color: var(--error-emphasis);
		border: 1px solid var(--error-base);
		border-radius: 4px;
		font-size: 0.6875rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.15s ease;
		text-transform: uppercase;
		letter-spacing: 0.03em;
	}

	.coupon-status.active {
		background: var(--success-soft);
		color: var(--success-emphasis);
		border-color: var(--success-base);
	}

	.coupon-status:hover {
		transform: scale(1.02);
	}

	.coupon-actions {
		display: flex;
		gap: 0.5rem;
		padding-top: 0.875rem;
		border-top: 1px solid var(--border-muted);
	}

	.action-btn {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.375rem;
		padding: 0.5rem;
		background: var(--bg-surface);
		border: 1px solid var(--border-default);
		border-radius: 6px;
		color: var(--text-secondary);
		font-size: 0.8125rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.action-btn:hover:not(:disabled) {
		background: var(--bg-hover);
		color: var(--text-primary);
	}

	.action-btn.edit:hover:not(:disabled) {
		background: rgba(230, 184, 0, 0.1);
		border-color: rgba(230, 184, 0, 0.3);
		color: var(--primary-400);
	}

	.action-btn.delete:hover:not(:disabled) {
		background: var(--error-soft);
		border-color: var(--error-base);
		color: var(--error-emphasis);
	}

	.action-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}
</style>

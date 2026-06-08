<script lang="ts">
	/**
	 * R25-C extraction (2026-05-20): product / membership-plan restrictions —
	 * include/exclude tab + per-list checkbox grids + select-all/clear.
	 *
	 * R8-C compliance: the raw shape was 14 props / 8 callbacks (one per
	 * select/clear/toggle/tab edge). That exceeded the >5/>5 ceiling, so the
	 * mutation surface is consolidated behind a single discriminated `onAction`
	 * callback. The parent owns the Set updates (creating new `Set` instances
	 * so Svelte 5 sees the reassignment).
	 */
	import type { Product, SubscriptionPlan } from '$lib/api/admin';
	import type { RestrictionTab } from './types';

	export type CouponRestrictionAction =
		| { kind: 'tab-change'; tab: RestrictionTab }
		| { kind: 'toggle-product'; productId: number }
		| { kind: 'toggle-plan'; planId: number }
		| { kind: 'select-all-products' }
		| { kind: 'clear-products' }
		| { kind: 'select-all-plans' }
		| { kind: 'clear-plans' };

	interface Props {
		restrictionTab: RestrictionTab;
		loadingData: boolean;
		availableProducts: Product[];
		availablePlans: SubscriptionPlan[];
		selectedProducts: Set<number>;
		selectedPlans: Set<number>;
		hasRestrictions: boolean;
		onAction: (action: CouponRestrictionAction) => void;
	}

	let {
		restrictionTab,
		loadingData,
		availableProducts,
		availablePlans,
		selectedProducts,
		selectedPlans,
		hasRestrictions,
		onAction
	}: Props = $props();
</script>

<div class="form-section">
	<h2>Product &amp; Membership Restrictions</h2>
	<p class="section-description">
		Choose which products and memberships this coupon applies to. Leave empty to apply to all.
	</p>

	<!-- Include/Exclude Tabs -->
	<div class="restriction-tabs">
		<button
			type="button"
			class={{ 'restriction-tab': true, active: restrictionTab === 'include' }}
			onclick={() => onAction({ kind: 'tab-change', tab: 'include' })}
		>
			Include Only
		</button>
		<button
			type="button"
			class={{ 'restriction-tab': true, active: restrictionTab === 'exclude' }}
			onclick={() => onAction({ kind: 'tab-change', tab: 'exclude' })}
		>
			Exclude
		</button>
	</div>

	<div class="restriction-content">
		{#if restrictionTab === 'include'}
			<p class="tab-hint">Coupon will ONLY work for selected items below</p>
		{:else}
			<p class="tab-hint">Coupon will work for everything EXCEPT selected items below</p>
		{/if}

		<!-- Products -->
		<div class="restriction-group">
			<div class="restriction-header">
				<h3>Products</h3>
				<div class="restriction-actions">
					<button
						type="button"
						class="btn-link"
						onclick={() => onAction({ kind: 'select-all-products' })}>Select All</button
					>
					<button
						type="button"
						class="btn-link"
						onclick={() => onAction({ kind: 'clear-products' })}>Clear</button
					>
				</div>
			</div>
			<div class="checkbox-grid">
				{#if loadingData}
					<div class="loading-items">
						<div class="spinner-small"></div>
						<p>Loading products...</p>
					</div>
				{:else if availableProducts.length === 0}
					<p class="no-items">No products available</p>
				{:else}
					{#each availableProducts as product (product.id)}
						<label class="item-checkbox">
							<input
								type="checkbox"
								checked={selectedProducts.has(product.id)}
								onchange={() => onAction({ kind: 'toggle-product', productId: product.id })}
							/>
							<span class="item-name">{product.name}</span>
							<span class="item-price">${product.sale_price || product.price}</span>
						</label>
					{/each}
				{/if}
			</div>
		</div>

		<!-- Membership Plans -->
		<div class="restriction-group">
			<div class="restriction-header">
				<h3>Membership Plans</h3>
				<div class="restriction-actions">
					<button
						type="button"
						class="btn-link"
						onclick={() => onAction({ kind: 'select-all-plans' })}>Select All</button
					>
					<button type="button" class="btn-link" onclick={() => onAction({ kind: 'clear-plans' })}
						>Clear</button
					>
				</div>
			</div>
			<div class="checkbox-grid">
				{#if loadingData}
					<div class="loading-items">
						<div class="spinner-small"></div>
						<p>Loading membership plans...</p>
					</div>
				{:else if availablePlans.length === 0}
					<p class="no-items">No membership plans available</p>
				{:else}
					{#each availablePlans as plan (plan.id)}
						<label class="item-checkbox">
							<input
								type="checkbox"
								checked={selectedPlans.has(plan.id)}
								onchange={() => onAction({ kind: 'toggle-plan', planId: plan.id })}
							/>
							<span class="item-name">{plan.name}</span>
							<span class="item-price"
								>${plan.price}/{plan.interval === 'monthly'
									? 'mo'
									: plan.interval === 'yearly'
										? 'yr'
										: 'lifetime'}</span
							>
						</label>
					{/each}
				{/if}
			</div>
		</div>

		{#if !hasRestrictions}
			<div class="no-restrictions-notice">
				No items selected - coupon will apply to <strong>all products and plans</strong>
			</div>
		{/if}
	</div>
</div>

<style>
	.form-section {
		background: rgba(30, 41, 59, 0.4);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 12px;
		padding: 1.5rem;
	}

	.form-section h2 {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 1.125rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0 0 1.25rem 0;
	}

	.section-description {
		color: #94a3b8;
		font-size: 0.875rem;
		margin: -0.75rem 0 1.25rem 0;
	}

	.restriction-tabs {
		display: flex;
		gap: 0.5rem;
		margin-bottom: 1rem;
	}

	.restriction-tab {
		flex: 1;
		padding: 0.75rem 1rem;
		background: rgba(15, 23, 42, 0.6);
		border: 2px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		color: #94a3b8;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.restriction-tab:hover {
		border-color: rgba(148, 163, 184, 0.4);
	}

	.restriction-tab.active {
		background: rgba(230, 184, 0, 0.1);
		border-color: var(--primary-500);
		color: var(--primary-500);
	}

	.restriction-content {
		padding: 1rem;
		background: rgba(15, 23, 42, 0.4);
		border-radius: 8px;
	}

	.tab-hint {
		color: #94a3b8;
		font-size: 0.875rem;
		margin: 0 0 1rem 0;
		padding: 0.5rem;
		background: rgba(100, 116, 139, 0.1);
		border-radius: 6px;
		text-align: center;
	}

	.restriction-group {
		margin-bottom: 1.5rem;
	}

	.restriction-group:last-child {
		margin-bottom: 0;
	}

	.restriction-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 0.75rem;
	}

	.restriction-header h3 {
		font-size: 0.95rem;
		font-weight: 600;
		color: #e2e8f0;
		margin: 0;
	}

	.restriction-actions {
		display: flex;
		gap: 0.75rem;
	}

	.btn-link {
		background: none;
		border: none;
		color: #60a5fa;
		font-size: 0.8rem;
		cursor: pointer;
		padding: 0;
	}

	.btn-link:hover {
		text-decoration: underline;
	}

	.checkbox-grid {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.item-checkbox {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem 1rem;
		background: rgba(30, 41, 59, 0.4);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 8px;
		cursor: pointer;
		transition: all 0.2s;
	}

	.item-checkbox:hover {
		border-color: rgba(230, 184, 0, 0.3);
	}

	.item-checkbox input[type='checkbox'] {
		width: 18px;
		height: 18px;
		accent-color: var(--primary-500);
		flex-shrink: 0;
	}

	.item-name {
		flex: 1;
		color: #e2e8f0;
		font-weight: 500;
	}

	.item-price {
		color: #64748b;
		font-size: 0.875rem;
	}

	.no-restrictions-notice {
		padding: 1rem;
		background: rgba(59, 130, 246, 0.1);
		border: 1px solid rgba(59, 130, 246, 0.2);
		border-radius: 8px;
		color: #60a5fa;
		text-align: center;
		font-size: 0.9rem;
	}

	.loading-items {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 2rem;
		gap: 0.75rem;
		color: #64748b;
	}

	.spinner-small {
		width: 32px;
		height: 32px;
		border: 3px solid rgba(148, 163, 184, 0.2);
		border-top-color: var(--primary-500);
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	.no-items {
		padding: 2rem;
		text-align: center;
		color: #64748b;
		font-style: italic;
	}

	@keyframes spin {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}
</style>

<script lang="ts">
	/**
	 * PaymentSummary Component (FluentForms Pro 6.1.8 - Updated Jan 2026)
	 *
	 * Responsive order summary with touch-friendly design.
	 *
	 * @version 2.1.0 - Svelte 5 + Responsive Design
	 */

	interface LineItem {
		id: string;
		name: string;
		price: number;
		quantity: number;
		subtotal: number;
	}

	interface Props {
		items: LineItem[];
		currency?: string;
		discount?: {
			code: string;
			type: 'percentage' | 'fixed';
			value: number;
			amount: number;
		} | null;
		tax?: {
			rate: number;
			amount: number;
		} | null;
		showBreakdown?: boolean;
	}

	let {
		items = [],
		currency = 'USD',
		discount = null,
		tax = null,
		showBreakdown = true
	}: Props = $props();

	import Icon from '$lib/components/Icon.svelte';

	function formatCurrency(amount: number): string {
		return new Intl.NumberFormat('en-US', {
			style: 'currency',
			currency: currency
		}).format(amount);
	}

	const subtotal = $derived(items.reduce((sum, item) => sum + item.subtotal, 0));

	const discountAmount = $derived(discount?.amount || 0);

	const taxableAmount = $derived(subtotal - discountAmount);

	const taxAmount = $derived(tax?.amount || 0);

	const total = $derived(taxableAmount + taxAmount);
</script>

<!-- Responsive Payment Summary Component - Mobile-first design -->
<div class="payment-summary">
	<!-- Summary Header -->
	<div class="summary-header">
		<span class="summary-header__icon">
			<Icon name="IconShoppingCart" size={20} />
		</span>
		<span class="summary-title">Order Summary</span>
	</div>

	<!-- Line Items - Responsive list -->
	{#if showBreakdown && items.length > 0}
		<div class="line-items">
			{#each items as item, index (item.id)}
				<div class={['line-item', index < items.length - 1 && 'line-item--separated']}>
					<div class="line-item__details">
						<span class="line-item__name">{item.name}</span>
						{#if item.quantity > 1}
							<span class="quantity-pill">x{item.quantity}</span>
						{/if}
					</div>
					<span class="line-item__price">{formatCurrency(item.subtotal)}</span>
				</div>
			{/each}
		</div>
	{/if}

	<!-- Calculations Section -->
	<div class="calculation-section">
		<!-- Subtotal Row -->
		<div class="calculation-row">
			<span class="calculation-label">Subtotal</span>
			<span class="calculation-value">{formatCurrency(subtotal)}</span>
		</div>

		<!-- Discount Row -->
		{#if discount}
			<div class="calculation-row">
				<span class="discount-label">
					<span class="discount-label__icon">
						<Icon name="IconGift" size={14} />
					</span>
					<span class="discount-label__text">Discount ({discount.code})</span>
					{#if discount.type === 'percentage'}
						<span class="discount-pill">-{discount.value}%</span>
					{/if}
				</span>
				<span class="discount-value">-{formatCurrency(discountAmount)}</span>
			</div>
		{/if}

		<!-- Tax Row -->
		{#if tax}
			<div class="calculation-row">
				<span class="calculation-label">Tax ({tax.rate}%)</span>
				<span class="calculation-value">{formatCurrency(taxAmount)}</span>
			</div>
		{/if}
	</div>

	<!-- Total Section - Prominent display -->
	<div class="total-section">
		<span class="total-label">Total</span>
		<span class="total-value">{formatCurrency(total)}</span>
	</div>

	<!-- Savings Badge - Touch-friendly -->
	{#if discount}
		<div class="savings-badge">
			<span class="savings-badge__icon">
				<Icon name="IconCheck" size={16} />
			</span>
			<span class="savings-badge__text">You saved {formatCurrency(discountAmount)}!</span>
		</div>
	{/if}
</div>

<style>
	.payment-summary {
		width: 100%;
		max-width: 100%;
		margin-inline: auto;
		overflow: hidden;
		padding-bottom: env(safe-area-inset-bottom);
		border: 1px solid #e5e7eb;
		border-radius: 0.75rem;
		background: #fff;
		box-shadow: 0 1px 2px rgba(15, 23, 42, 0.06);
	}

	.summary-header {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem;
		border-bottom: 1px solid #e5e7eb;
		background: #f9fafb;
	}

	.summary-header__icon {
		display: flex;
		flex: 0 0 auto;
		color: #374151;
	}

	.summary-title {
		color: #111827;
		font-size: 0.875rem;
		font-weight: 600;
	}

	.line-items,
	.calculation-section {
		padding: 0.75rem;
		border-bottom: 1px solid #e5e7eb;
	}

	.line-item {
		display: flex;
		min-height: 44px;
		align-items: center;
		justify-content: space-between;
		padding-block: 0.625rem;
	}

	.line-item--separated {
		border-bottom: 1px dashed #e5e7eb;
	}

	.line-item__details {
		display: flex;
		min-width: 0;
		flex: 1 1 auto;
		align-items: center;
		gap: 0.5rem;
	}

	.line-item__name {
		overflow: hidden;
		color: #374151;
		font-size: 0.875rem;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.quantity-pill,
	.discount-pill {
		flex: 0 0 auto;
		border-radius: 0.25rem;
		font-size: 0.75rem;
	}

	.quantity-pill {
		padding: 0.125rem 0.375rem;
		background: #f3f4f6;
		color: #6b7280;
	}

	.line-item__price {
		flex: 0 0 auto;
		margin-left: 0.75rem;
		color: #111827;
		font-size: 0.875rem;
		font-weight: 500;
	}

	.calculation-section {
		display: grid;
		gap: 0.5rem;
	}

	.calculation-row {
		display: flex;
		min-height: 36px;
		align-items: center;
		justify-content: space-between;
		padding-block: 0.25rem;
	}

	.calculation-label,
	.calculation-value,
	.discount-label,
	.discount-value {
		font-size: 0.875rem;
	}

	.calculation-label {
		color: #6b7280;
	}

	.calculation-value {
		color: #374151;
	}

	.discount-label {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		color: #059669;
	}

	.discount-label__icon,
	.savings-badge__icon {
		display: flex;
		flex: 0 0 auto;
	}

	.discount-label__text {
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.discount-pill {
		padding: 0.125rem 0.375rem;
		background: #d1fae5;
		color: #047857;
	}

	.discount-value {
		flex: 0 0 auto;
		color: #059669;
		font-weight: 500;
	}

	.total-section {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 1rem;
		background: #1e3a8a;
		color: #fff;
	}

	.total-label {
		font-size: 1rem;
		font-weight: 500;
	}

	.total-value {
		font-size: 1.25rem;
		font-weight: 700;
	}

	.savings-badge {
		display: flex;
		min-height: 44px;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 0.75rem;
		background: #ecfdf5;
		color: #047857;
	}

	.savings-badge__text {
		font-size: 0.875rem;
		font-weight: 500;
	}

	@media (min-width: 640px) {
		.payment-summary {
			max-width: 32rem;
			border-radius: 1rem;
		}

		.summary-header,
		.line-items,
		.calculation-section,
		.savings-badge {
			padding: 1rem;
		}

		.summary-header {
			gap: 0.75rem;
		}

		.summary-title,
		.line-item__name,
		.line-item__price,
		.calculation-label,
		.calculation-value,
		.discount-label,
		.discount-value,
		.savings-badge__text {
			font-size: 1rem;
		}

		.line-item {
			padding-block: 0.75rem;
		}

		.line-item__details {
			gap: 0.75rem;
		}

		.quantity-pill {
			padding: 0.25rem 0.5rem;
			font-size: 0.875rem;
		}

		.calculation-section {
			gap: 0.75rem;
		}

		.calculation-row {
			min-height: 40px;
			padding-block: 0.375rem;
		}

		.discount-label {
			gap: 0.5rem;
		}

		.total-section {
			padding: 1.25rem;
		}

		.total-label {
			font-size: 1.125rem;
		}

		.total-value {
			font-size: 1.5rem;
		}
	}

	@media (min-width: 768px) {
		.payment-summary {
			max-width: 36rem;
		}

		.summary-header,
		.line-items,
		.calculation-section {
			padding: 1.25rem;
		}

		.summary-title {
			font-size: 1.125rem;
		}

		.total-section {
			padding: 1.5rem;
		}

		.total-value {
			font-size: 1.875rem;
		}
	}

	@media (min-width: 1024px) {
		.payment-summary {
			max-width: 42rem;
		}
	}
</style>

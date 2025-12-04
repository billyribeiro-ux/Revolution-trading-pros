<script lang="ts">
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

<div class="payment-summary">
	<div class="summary-header">
		<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
			<circle cx="9" cy="21" r="1"></circle>
			<circle cx="20" cy="21" r="1"></circle>
			<path d="M1 1h4l2.68 13.39a2 2 0 0 0 2 1.61h9.72a2 2 0 0 0 2-1.61L23 6H6"></path>
		</svg>
		<span>Order Summary</span>
	</div>

	{#if showBreakdown && items.length > 0}
		<div class="line-items">
			{#each items as item (item.id)}
				<div class="line-item">
					<div class="item-info">
						<span class="item-name">{item.name}</span>
						{#if item.quantity > 1}
							<span class="item-qty">x{item.quantity}</span>
						{/if}
					</div>
					<span class="item-price">{formatCurrency(item.subtotal)}</span>
				</div>
			{/each}
		</div>
	{/if}

	<div class="summary-calculations">
		<div class="calc-row">
			<span class="calc-label">Subtotal</span>
			<span class="calc-value">{formatCurrency(subtotal)}</span>
		</div>

		{#if discount}
			<div class="calc-row discount">
				<span class="calc-label">
					<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<polyline points="20 12 20 22 4 22 4 12"></polyline>
						<rect x="2" y="7" width="20" height="5"></rect>
						<line x1="12" y1="22" x2="12" y2="7"></line>
						<path d="M12 7H7.5a2.5 2.5 0 0 1 0-5C11 2 12 7 12 7z"></path>
						<path d="M12 7h4.5a2.5 2.5 0 0 0 0-5C13 2 12 7 12 7z"></path>
					</svg>
					Discount ({discount.code})
					{#if discount.type === 'percentage'}
						<span class="discount-percent">-{discount.value}%</span>
					{/if}
				</span>
				<span class="calc-value discount-value">-{formatCurrency(discountAmount)}</span>
			</div>
		{/if}

		{#if tax}
			<div class="calc-row tax">
				<span class="calc-label">
					Tax ({tax.rate}%)
				</span>
				<span class="calc-value">{formatCurrency(taxAmount)}</span>
			</div>
		{/if}
	</div>

	<div class="summary-total">
		<span class="total-label">Total</span>
		<span class="total-value">{formatCurrency(total)}</span>
	</div>

	{#if discount}
		<div class="savings-badge">
			<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
				<polyline points="20 6 9 17 4 12"></polyline>
			</svg>
			You saved {formatCurrency(discountAmount)}!
		</div>
	{/if}
</div>

<style>
	.payment-summary {
		background-color: white;
		border: 1px solid #e5e7eb;
		border-radius: 0.75rem;
		overflow: hidden;
	}

	.summary-header {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 1rem;
		background-color: #f9fafb;
		border-bottom: 1px solid #e5e7eb;
		font-weight: 600;
		color: #111827;
	}

	.line-items {
		padding: 1rem;
		border-bottom: 1px solid #e5e7eb;
	}

	.line-item {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 0.5rem 0;
	}

	.line-item:not(:last-child) {
		border-bottom: 1px dashed #e5e7eb;
	}

	.item-info {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.item-name {
		font-size: 0.875rem;
		color: #374151;
	}

	.item-qty {
		font-size: 0.75rem;
		color: #6b7280;
		padding: 0.125rem 0.375rem;
		background-color: #f3f4f6;
		border-radius: 0.25rem;
	}

	.item-price {
		font-size: 0.875rem;
		font-weight: 500;
		color: #111827;
	}

	.summary-calculations {
		padding: 1rem;
		border-bottom: 1px solid #e5e7eb;
	}

	.calc-row {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 0.375rem 0;
		font-size: 0.875rem;
	}

	.calc-label {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		color: #6b7280;
	}

	.calc-value {
		color: #374151;
	}

	.calc-row.discount .calc-label {
		color: #059669;
	}

	.discount-percent {
		font-size: 0.75rem;
		padding: 0.125rem 0.25rem;
		background-color: #dcfce7;
		border-radius: 0.25rem;
		margin-left: 0.25rem;
	}

	.discount-value {
		color: #059669;
		font-weight: 500;
	}

	.calc-row.tax .calc-label {
		color: #6b7280;
	}

	.summary-total {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1rem;
		background-color: #1e3a8a;
		color: white;
	}

	.total-label {
		font-weight: 500;
		font-size: 1rem;
	}

	.total-value {
		font-size: 1.5rem;
		font-weight: 700;
	}

	.savings-badge {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.375rem;
		padding: 0.5rem;
		background-color: #ecfdf5;
		color: #059669;
		font-size: 0.75rem;
		font-weight: 500;
	}
</style>

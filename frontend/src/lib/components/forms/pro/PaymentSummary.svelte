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

<div class="ps-wrap">
	<div class="ps-header">
		<svg class="ps-header-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
			<circle cx="9" cy="21" r="1"></circle>
			<circle cx="20" cy="21" r="1"></circle>
			<path d="M1 1h4l2.68 13.39a2 2 0 0 0 2 1.61h9.72a2 2 0 0 0 2-1.61L23 6H6"></path>
		</svg>
		<span class="ps-header-title">Order Summary</span>
	</div>

	{#if showBreakdown && items.length > 0}
		<div class="ps-items">
			{#each items as item, index (item.id)}
				<div class="ps-item" data-border={index < items.length - 1 ? '' : undefined}>
					<div class="ps-item-info">
						<span class="ps-item-name">{item.name}</span>
						{#if item.quantity > 1}
							<span class="ps-qty-badge">x{item.quantity}</span>
						{/if}
					</div>
					<span class="ps-item-price">{formatCurrency(item.subtotal)}</span>
				</div>
			{/each}
		</div>
	{/if}

	<div class="ps-calcs">
		<div class="ps-calc-row">
			<span class="ps-calc-label">Subtotal</span>
			<span class="ps-calc-value">{formatCurrency(subtotal)}</span>
		</div>

		{#if discount}
			<div class="ps-calc-row ps-discount-row">
				<span class="ps-discount-label">
					<svg class="ps-discount-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<polyline points="20 12 20 22 4 22 4 12"></polyline>
						<rect x="2" y="7" width="20" height="5"></rect>
						<line x1="12" y1="22" x2="12" y2="7"></line>
						<path d="M12 7H7.5a2.5 2.5 0 0 1 0-5C11 2 12 7 12 7z"></path>
						<path d="M12 7h4.5a2.5 2.5 0 0 0 0-5C13 2 12 7 12 7z"></path>
					</svg>
					<span class="ps-discount-text">Discount ({discount.code})</span>
					{#if discount.type === 'percentage'}
						<span class="ps-discount-pct">-{discount.value}%</span>
					{/if}
				</span>
				<span class="ps-discount-amount">-{formatCurrency(discountAmount)}</span>
			</div>
		{/if}

		{#if tax}
			<div class="ps-calc-row">
				<span class="ps-calc-label">Tax ({tax.rate}%)</span>
				<span class="ps-calc-value">{formatCurrency(taxAmount)}</span>
			</div>
		{/if}
	</div>

	<div class="ps-total">
		<span class="ps-total-label">Total</span>
		<span class="ps-total-value">{formatCurrency(total)}</span>
	</div>

	{#if discount}
		<div class="ps-savings">
			<svg class="ps-savings-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
				<polyline points="20 6 9 17 4 12"></polyline>
			</svg>
			<span class="ps-savings-text">You saved {formatCurrency(discountAmount)}!</span>
		</div>
	{/if}
</div>

<style>
	.ps-wrap {
		inline-size: 100%;
		max-inline-size: 100%;
		margin-inline: auto;
		background-color: oklch(1 0 0);
		border: 1px solid oklch(0.9 0.005 265);
		border-radius: var(--radius-xl);
		overflow: hidden;
		box-shadow: 0 1px 3px oklch(0 0 0 / 0.05);
		padding-block-end: env(safe-area-inset-bottom);

		@media (min-width: 640px) { max-inline-size: 32rem; border-radius: 1rem; }
		@media (min-width: 768px) { max-inline-size: 36rem; }
		@media (min-width: 1024px) { max-inline-size: 42rem; }
	}

	.ps-header {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem;
		background-color: oklch(0.97 0.005 265);
		border-block-end: 1px solid oklch(0.9 0.005 265);

		@media (min-width: 640px) { gap: 0.75rem; padding: 1rem; }
		@media (min-width: 768px) { padding: 1.25rem; }
	}

	.ps-header-icon {
		inline-size: 1.25rem;
		block-size: 1.25rem;
		color: oklch(0.35 0.01 265);
		flex-shrink: 0;

		@media (min-width: 640px) { inline-size: 1.5rem; block-size: 1.5rem; }
	}

	.ps-header-title {
		font-size: var(--text-sm);
		font-weight: var(--weight-semibold);
		color: oklch(0.15 0.01 265);

		@media (min-width: 640px) { font-size: var(--text-base); }
		@media (min-width: 768px) { font-size: var(--text-lg); }
	}

	.ps-items {
		padding: 0.75rem;
		border-block-end: 1px solid oklch(0.9 0.005 265);

		@media (min-width: 640px) { padding: 1rem; }
		@media (min-width: 768px) { padding: 1.25rem; }
	}

	.ps-item {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding-block: 0.625rem;
		min-block-size: 2.75rem;

		@media (min-width: 640px) { padding-block: 0.75rem; }
	}

	.ps-item[data-border] {
		border-block-end: 1px dashed oklch(0.9 0.005 265);
	}

	.ps-item-info {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		flex: 1;
		min-inline-size: 0;

		@media (min-width: 640px) { gap: 0.75rem; }
	}

	.ps-item-name {
		font-size: var(--text-sm);
		color: oklch(0.35 0.01 265);
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;

		@media (min-width: 640px) { font-size: var(--text-base); }
	}

	.ps-qty-badge {
		font-size: var(--text-xs);
		color: oklch(0.45 0.005 265);
		padding-inline: 0.375rem;
		padding-block: 0.125rem;
		background-color: oklch(0.95 0.005 265);
		border-radius: var(--radius-sm);
		flex-shrink: 0;

		@media (min-width: 640px) { font-size: var(--text-sm); padding-inline: 0.5rem; padding-block: 0.25rem; }
	}

	.ps-item-price {
		font-size: var(--text-sm);
		font-weight: var(--weight-medium);
		color: oklch(0.15 0.01 265);
		margin-inline-start: 0.75rem;
		flex-shrink: 0;

		@media (min-width: 640px) { font-size: var(--text-base); }
	}

	.ps-calcs {
		padding: 0.75rem;
		border-block-end: 1px solid oklch(0.9 0.005 265);
		display: flex;
		flex-direction: column;
		gap: 0.5rem;

		@media (min-width: 640px) { padding: 1rem; gap: 0.75rem; }
		@media (min-width: 768px) { padding: 1.25rem; }
	}

	.ps-calc-row {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding-block: 0.25rem;
		min-block-size: 2.25rem;

		@media (min-width: 640px) { padding-block: 0.375rem; min-block-size: 2.5rem; }
	}

	.ps-calc-label {
		font-size: var(--text-sm);
		color: oklch(0.45 0.005 265);

		@media (min-width: 640px) { font-size: var(--text-base); }
	}

	.ps-calc-value {
		font-size: var(--text-sm);
		color: oklch(0.35 0.01 265);

		@media (min-width: 640px) { font-size: var(--text-base); }
	}

	.ps-discount-row { color: oklch(0.45 0.15 160); }

	.ps-discount-label {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		font-size: var(--text-sm);
		color: oklch(0.45 0.15 160);

		@media (min-width: 640px) { gap: 0.5rem; font-size: var(--text-base); }
	}

	.ps-discount-icon {
		inline-size: 0.875rem;
		block-size: 0.875rem;
		flex-shrink: 0;

		@media (min-width: 640px) { inline-size: 1rem; block-size: 1rem; }
	}

	.ps-discount-text {
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.ps-discount-pct {
		font-size: var(--text-xs);
		padding-inline: 0.375rem;
		padding-block: 0.125rem;
		background-color: oklch(0.95 0.05 160);
		color: oklch(0.4 0.12 160);
		border-radius: var(--radius-sm);
		flex-shrink: 0;
	}

	.ps-discount-amount {
		font-size: var(--text-sm);
		font-weight: var(--weight-medium);
		color: oklch(0.45 0.15 160);
		flex-shrink: 0;

		@media (min-width: 640px) { font-size: var(--text-base); }
	}

	.ps-total {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1rem;
		background-color: oklch(0.25 0.1 260);
		color: oklch(1 0 0);

		@media (min-width: 640px) { padding: 1.25rem; }
		@media (min-width: 768px) { padding: 1.5rem; }
	}

	.ps-total-label {
		font-size: var(--text-base);
		font-weight: var(--weight-medium);

		@media (min-width: 640px) { font-size: var(--text-lg); }
	}

	.ps-total-value {
		font-size: var(--text-xl);
		font-weight: var(--weight-bold);

		@media (min-width: 640px) { font-size: 1.5rem; }
		@media (min-width: 768px) { font-size: 1.875rem; }
	}

	.ps-savings {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 0.75rem;
		background-color: oklch(0.97 0.04 160);
		color: oklch(0.4 0.12 160);
		min-block-size: 2.75rem;

		@media (min-width: 640px) { padding: 1rem; }
	}

	.ps-savings-icon {
		inline-size: 1rem;
		block-size: 1rem;
		flex-shrink: 0;

		@media (min-width: 640px) { inline-size: 1.25rem; block-size: 1.25rem; }
	}

	.ps-savings-text {
		font-size: var(--text-sm);
		font-weight: var(--weight-medium);

		@media (min-width: 640px) { font-size: var(--text-base); }
	}
</style>

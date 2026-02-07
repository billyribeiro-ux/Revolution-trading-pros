<script lang="ts">
	/**
	 * InventoryField Component (FluentForms 6.1.8 - December 2025)
	 *
	 * Inventory management for product selection fields.
	 * Tracks stock levels, prevents overselling, and shows availability status.
	 */

	interface ProductOption {
		id: string;
		name: string;
		price: number;
		stock: number;
		maxPerOrder?: number;
		image?: string;
		sku?: string;
		lowStockThreshold?: number;
	}

	interface Props {
		name: string;
		products: ProductOption[];
		value?: { productId: string; quantity: number }[];
		label?: string;
		required?: boolean;
		disabled?: boolean;
		showStock?: boolean;
		showPrice?: boolean;
		showSku?: boolean;
		allowMultiple?: boolean;
		currency?: string;
		layout?: 'list' | 'grid' | 'compact';
		columns?: number;
		error?: string;
		onchange?: (
			selection: { productId: string; quantity: number; product: ProductOption }[]
		) => void;
	}

	let {
		name,
		products = [],
		value = [],
		label = 'Select Products',
		required = false,
		disabled = false,
		showStock = true,
		showPrice = true,
		showSku = false,
		allowMultiple = true,
		currency = 'USD',
		layout = 'list',
		columns = 2,
		error = '',
		onchange
	}: Props = $props();

	let selections = $state<Map<string, number>>(new Map());

	// Initialize selections from value prop
	$effect(() => {
		const newSelections = new Map<string, number>();
		for (const item of value) {
			newSelections.set(item.productId, item.quantity);
		}
		selections = newSelections;
	});

	function getStockStatus(product: ProductOption): 'in-stock' | 'low-stock' | 'out-of-stock' {
		if (product.stock <= 0) return 'out-of-stock';
		const threshold = product.lowStockThreshold ?? 5;
		if (product.stock <= threshold) return 'low-stock';
		return 'in-stock';
	}

	function getStockLabel(product: ProductOption): string {
		const status = getStockStatus(product);
		if (status === 'out-of-stock') return 'Out of Stock';
		if (status === 'low-stock') return `Only ${product.stock} left`;
		return `${product.stock} in stock`;
	}

	function getMaxQuantity(product: ProductOption): number {
		const availableStock = product.stock;
		const maxPerOrder = product.maxPerOrder ?? availableStock;
		return Math.min(availableStock, maxPerOrder);
	}

	function handleQuantityChange(product: ProductOption, quantity: number) {
		if (disabled) return;

		const newSelections = new Map(selections);

		if (quantity <= 0) {
			newSelections.delete(product.id);
		} else {
			const maxQty = getMaxQuantity(product);
			newSelections.set(product.id, Math.min(quantity, maxQty));
		}

		if (!allowMultiple && quantity > 0) {
			// Single selection mode - clear others
			for (const [id] of newSelections) {
				if (id !== product.id) {
					newSelections.delete(id);
				}
			}
		}

		selections = newSelections;
		notifyChange();
	}

	function handleCheckboxChange(product: ProductOption, checked: boolean) {
		handleQuantityChange(product, checked ? 1 : 0);
	}

	function incrementQuantity(product: ProductOption) {
		const current = selections.get(product.id) ?? 0;
		handleQuantityChange(product, current + 1);
	}

	function decrementQuantity(product: ProductOption) {
		const current = selections.get(product.id) ?? 0;
		handleQuantityChange(product, current - 1);
	}

	function notifyChange() {
		if (onchange) {
			const selection = Array.from(selections.entries())
				.map(([productId, quantity]) => {
					const product = products.find((p) => p.id === productId);
					return product ? { productId, quantity, product } : null;
				})
				.filter((item): item is NonNullable<typeof item> => item !== null);
			onchange(selection);
		}
	}

	function formatPrice(price: number): string {
		return new Intl.NumberFormat('en-US', {
			style: 'currency',
			currency: currency
		}).format(price);
	}

	const totalAmount = $derived.by(() => {
		let total = 0;
		for (const [productId, quantity] of selections) {
			const product = products.find((p) => p.id === productId);
			if (product) {
				total += product.price * quantity;
			}
		}
		return total;
	});

	const totalItems = $derived.by(() => {
		let count = 0;
		for (const quantity of selections.values()) {
			count += quantity;
		}
		return count;
	});
</script>

<div class="inventory-field" class:disabled class:has-error={error}>
	<div class="field-header">
		<label class="field-label" for="{name}-products">
			{label}
			{#if required}
				<span class="required-marker">*</span>
			{/if}
		</label>
	</div>

	<div
		id="{name}-products"
		class="products-container"
		class:layout-grid={layout === 'grid'}
		class:layout-compact={layout === 'compact'}
		style={layout === 'grid' ? `--columns: ${columns}` : ''}
		role="group"
		aria-label={label}
	>
		{#each products as product}
			{@const stockStatus = getStockStatus(product)}
			{@const isSelected = selections.has(product.id)}
			{@const quantity = selections.get(product.id) ?? 0}
			{@const maxQty = getMaxQuantity(product)}

			<div
				class="product-item"
				class:selected={isSelected}
				class:out-of-stock={stockStatus === 'out-of-stock'}
			>
				{#if product.image}
					<div class="product-image">
						<img src={product.image} alt={product.name} loading="lazy" />
					</div>
				{/if}

				<div class="product-info">
					<div class="product-header">
						<h4 class="product-name">{product.name}</h4>
						{#if showSku && product.sku}
							<span class="product-sku">SKU: {product.sku}</span>
						{/if}
					</div>

					<div class="product-meta">
						{#if showPrice}
							<span class="product-price">{formatPrice(product.price)}</span>
						{/if}
						{#if showStock}
							<span class="stock-badge {stockStatus}">
								{getStockLabel(product)}
							</span>
						{/if}
					</div>
				</div>

				<div class="product-actions">
					{#if stockStatus === 'out-of-stock'}
						<span class="unavailable-label">Unavailable</span>
					{:else if allowMultiple}
						<div class="quantity-selector">
							<button
								type="button"
								class="qty-btn"
								onclick={() => decrementQuantity(product)}
								disabled={disabled || quantity <= 0}
								aria-label="Decrease quantity"
							>
								<svg
									width="16"
									height="16"
									viewBox="0 0 24 24"
									fill="none"
									stroke="currentColor"
									stroke-width="2"
								>
									<line x1="5" y1="12" x2="19" y2="12"></line>
								</svg>
							</button>
							<input
								type="number"
								class="qty-input"
								value={quantity}
								min="0"
								max={maxQty}
								oninput={(e: Event) =>
									handleQuantityChange(
										product,
										parseInt((e.target as HTMLInputElement).value) || 0
									)}
								{disabled}
							/>
							<button
								type="button"
								class="qty-btn"
								onclick={() => incrementQuantity(product)}
								disabled={disabled || quantity >= maxQty}
								aria-label="Increase quantity"
							>
								<svg
									width="16"
									height="16"
									viewBox="0 0 24 24"
									fill="none"
									stroke="currentColor"
									stroke-width="2"
								>
									<line x1="12" y1="5" x2="12" y2="19"></line>
									<line x1="5" y1="12" x2="19" y2="12"></line>
								</svg>
							</button>
						</div>
						{#if isSelected}
							<span class="line-total">{formatPrice(product.price * quantity)}</span>
						{/if}
					{:else}
						<label class="radio-label">
							<input
								type="radio"
								name="{name}_selection"
								value={product.id}
								checked={isSelected}
								onchange={(e: Event) =>
									handleCheckboxChange(product, (e.target as HTMLInputElement).checked)}
								{disabled}
							/>
							<span class="radio-custom"></span>
							Select
						</label>
					{/if}
				</div>
			</div>
		{/each}
	</div>

	{#if selections.size > 0}
		<div class="selection-summary">
			<div class="summary-row">
				<span class="summary-label">Selected Items:</span>
				<span class="summary-value">{totalItems}</span>
			</div>
			{#if showPrice}
				<div class="summary-row total">
					<span class="summary-label">Total:</span>
					<span class="summary-value">{formatPrice(totalAmount)}</span>
				</div>
			{/if}
		</div>
	{/if}

	<!-- Hidden inputs for form submission -->
	{#each Array.from(selections.entries()) as [productId, qty], index}
		<input type="hidden" name="{name}[{index}][product_id]" value={productId} />
		<input type="hidden" name="{name}[{index}][quantity]" value={qty} />
	{/each}

	{#if error}
		<div class="error-message">
			<svg
				width="14"
				height="14"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
			>
				<circle cx="12" cy="12" r="10"></circle>
				<line x1="12" y1="8" x2="12" y2="12"></line>
				<line x1="12" y1="16" x2="12.01" y2="16"></line>
			</svg>
			{error}
		</div>
	{/if}
</div>

<style>
	.inventory-field {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.field-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.field-label {
		font-size: 0.9375rem;
		font-weight: 600;
		color: #374151;
	}

	.required-marker {
		color: #ef4444;
		margin-left: 0.25rem;
	}

	.products-container {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.products-container.layout-grid {
		display: grid;
		grid-template-columns: repeat(var(--columns, 2), 1fr);
		gap: 1rem;
	}

	.products-container.layout-compact .product-item {
		padding: 0.75rem;
	}

	.product-item {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1rem;
		background-color: white;
		border: 2px solid #e5e7eb;
		border-radius: 0.75rem;
		transition: all 0.2s;
	}

	.product-item:hover:not(.out-of-stock) {
		border-color: #d1d5db;
	}

	.product-item.selected {
		border-color: #3b82f6;
		background-color: #eff6ff;
	}

	.product-item.out-of-stock {
		opacity: 0.6;
		background-color: #f9fafb;
	}

	.product-image {
		width: 60px;
		height: 60px;
		flex-shrink: 0;
		border-radius: 0.5rem;
		overflow: hidden;
		background-color: #f3f4f6;
	}

	.product-image img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.product-info {
		flex: 1;
		min-width: 0;
	}

	.product-header {
		display: flex;
		flex-direction: column;
		gap: 0.125rem;
	}

	.product-name {
		font-size: 0.9375rem;
		font-weight: 600;
		color: #111827;
		margin: 0;
	}

	.product-sku {
		font-size: 0.75rem;
		color: #9ca3af;
	}

	.product-meta {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		margin-top: 0.375rem;
	}

	.product-price {
		font-size: 1rem;
		font-weight: 700;
		color: #059669;
	}

	.stock-badge {
		font-size: 0.75rem;
		padding: 0.125rem 0.5rem;
		border-radius: 9999px;
	}

	.stock-badge.in-stock {
		background-color: #dcfce7;
		color: #166534;
	}

	.stock-badge.low-stock {
		background-color: #fef3c7;
		color: #92400e;
	}

	.stock-badge.out-of-stock {
		background-color: #fee2e2;
		color: #991b1b;
	}

	.product-actions {
		display: flex;
		flex-direction: column;
		align-items: flex-end;
		gap: 0.5rem;
	}

	.quantity-selector {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		background-color: #f3f4f6;
		border-radius: 0.5rem;
		padding: 0.25rem;
	}

	.qty-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		background-color: white;
		border: 1px solid #d1d5db;
		border-radius: 0.375rem;
		cursor: pointer;
		color: #374151;
		transition: all 0.15s;
	}

	.qty-btn:hover:not(:disabled) {
		background-color: #f3f4f6;
		border-color: #9ca3af;
	}

	.qty-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.qty-input {
		width: 48px;
		padding: 0.375rem;
		border: none;
		background: transparent;
		text-align: center;
		font-size: 0.9375rem;
		font-weight: 600;
		color: #111827;
	}

	.qty-input::-webkit-inner-spin-button,
	.qty-input::-webkit-outer-spin-button {
		-webkit-appearance: none;
		margin: 0;
	}

	.line-total {
		font-size: 0.8125rem;
		font-weight: 600;
		color: #6b7280;
	}

	.unavailable-label {
		font-size: 0.8125rem;
		color: #9ca3af;
		font-style: italic;
	}

	.radio-label {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		cursor: pointer;
		font-size: 0.875rem;
		color: #374151;
	}

	.radio-label input {
		position: absolute;
		opacity: 0;
	}

	.radio-custom {
		width: 20px;
		height: 20px;
		border: 2px solid #d1d5db;
		border-radius: 50%;
		background-color: white;
		transition: all 0.15s;
	}

	.radio-label input:checked + .radio-custom {
		border-color: #3b82f6;
		background-color: #3b82f6;
		box-shadow: inset 0 0 0 4px white;
	}

	.selection-summary {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		padding: 1rem;
		background-color: #f9fafb;
		border-radius: 0.5rem;
	}

	.summary-row {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.summary-label {
		font-size: 0.875rem;
		color: #6b7280;
	}

	.summary-value {
		font-size: 0.9375rem;
		font-weight: 600;
		color: #111827;
	}

	.summary-row.total {
		padding-top: 0.5rem;
		border-top: 1px solid #e5e7eb;
	}

	.summary-row.total .summary-label {
		font-weight: 600;
		color: #374151;
	}

	.summary-row.total .summary-value {
		font-size: 1.125rem;
		color: #059669;
	}

	/* Disabled State */
	.disabled {
		opacity: 0.6;
		pointer-events: none;
	}

	/* Error State */
	.has-error .product-item {
		border-color: #fca5a5;
	}

	.error-message {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		color: #dc2626;
		font-size: 0.8125rem;
	}

	/* Responsive */
	@media (max-width: 640px) {
		.products-container.layout-grid {
			grid-template-columns: 1fr;
		}

		.product-item {
			flex-wrap: wrap;
		}

		.product-actions {
			width: 100%;
			flex-direction: row;
			justify-content: space-between;
			align-items: center;
			margin-top: 0.5rem;
			padding-top: 0.5rem;
			border-top: 1px solid #e5e7eb;
		}
	}
</style>

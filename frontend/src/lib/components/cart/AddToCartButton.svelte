<script lang="ts">
	/**
	 * AddToCartButton Component
	 * Button to add products to cart
	 *
	 * @version 1.0.0
	 * @author Revolution Trading Pros
	 */

	interface Props {
		productId: string;
		productName?: string;
		price?: number;
		quantity?: number;
		disabled?: boolean;
		loading?: boolean;
		variant?: 'primary' | 'secondary' | 'outline';
		size?: 'sm' | 'md' | 'lg';
		fullWidth?: boolean;
		onAdd?: (productId: string, quantity: number) => void;
	}

	const {
		productId,
		productName = 'Product',
		price,
		quantity = 1,
		disabled = false,
		loading = false,
		variant = 'primary',
		size = 'md',
		fullWidth = false,
		onAdd
	}: Props = $props();

	let isAdding = $state(false);

	async function handleClick() {
		if (disabled || loading || isAdding) return;

		isAdding = true;
		try {
			onAdd?.(productId, quantity);
		} finally {
			setTimeout(() => {
				isAdding = false;
			}, 500);
		}
	}
</script>

<button
	type="button"
	class="add-to-cart-btn variant-{variant} size-{size}"
	class:full-width={fullWidth}
	class:loading={loading || isAdding}
	{disabled}
	onclick={handleClick}
	aria-label="Add {productName} to cart"
>
	{#if loading || isAdding}
		<span class="spinner"></span>
		<span>Adding...</span>
	{:else}
		<span class="cart-icon">🛒</span>
		<span>Add to Cart</span>
		{#if price}
			<span class="price">${price.toFixed(2)}</span>
		{/if}
	{/if}
</button>

<style>
	.add-to-cart-btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		font-weight: 600;
		border-radius: var(--radius-md, 0.375rem);
		cursor: pointer;
		transition: all 0.15s ease;
		border: none;
	}

	.add-to-cart-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.full-width {
		width: 100%;
	}

	/* Variants */
	.variant-primary {
		background: var(--color-primary, #6366f1);
		color: white;
	}

	.variant-primary:hover:not(:disabled) {
		background: var(--color-primary-hover, #4f46e5);
	}

	.variant-secondary {
		background: var(--color-bg-secondary, #f3f4f6);
		color: var(--color-text-primary, #111827);
	}

	.variant-secondary:hover:not(:disabled) {
		background: var(--color-bg-tertiary, #e5e7eb);
	}

	.variant-outline {
		background: transparent;
		border: 2px solid var(--color-primary, #6366f1);
		color: var(--color-primary, #6366f1);
	}

	.variant-outline:hover:not(:disabled) {
		background: var(--color-primary, #6366f1);
		color: white;
	}

	/* Sizes */
	.size-sm {
		padding: 0.5rem 1rem;
		font-size: 0.875rem;
		min-height: 36px;
	}

	.size-md {
		padding: 0.75rem 1.5rem;
		font-size: 1rem;
		min-height: 44px;
	}

	.size-lg {
		padding: 1rem 2rem;
		font-size: 1.125rem;
		min-height: 52px;
	}

	.spinner {
		width: 16px;
		height: 16px;
		border: 2px solid currentColor;
		border-top-color: transparent;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.price {
		opacity: 0.9;
	}
</style>

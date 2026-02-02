<script lang="ts">
	import type { FormField } from '$lib/api/forms';

	interface PaymentItem {
		id: string;
		label: string;
		price: number;
		quantity?: number;
		image?: string;
	}

	interface Props {
		field: FormField;
		value?: PaymentItem[] | PaymentItem | null;
		currency?: string;
		error?: string[];
		onchange?: (value: PaymentItem[] | PaymentItem | null) => void;
	}

	let props: Props = $props();

	const paymentType = $derived(props.field.attributes?.payment_type || 'single');
	const items = $derived<PaymentItem[]>((props.field.options as PaymentItem[]) || []);
	const allowQuantity = $derived(props.field.attributes?.allow_quantity || false);

	function formatCurrency(amount: number): string {
		return new Intl.NumberFormat('en-US', {
			style: 'currency',
			currency: props.currency ?? 'USD'
		}).format(amount);
	}

	function handleSingleSelect(item: PaymentItem) {
		props.onchange?.(item);
	}

	function handleMultiSelect(item: PaymentItem, checked: boolean) {
		const currentItems = Array.isArray(props.value) ? value : [];
		if (checked) {
			props.onchange?.([...currentItems, { ...item, quantity: 1 }]);
		} else {
			props.onchange?.(currentItems.filter((i) => i.id !== item.id));
		}
	}

	function handleQuantityChange(itemId: string, quantity: number) {
		if (!Array.isArray(props.value)) return;
		const updatedItems = value.map((i) => (i.id === itemId ? { ...i, quantity } : i));
		props.onchange?.(updatedItems);
	}

	function isSelected(itemId: string): boolean {
		if (Array.isArray(props.value)) {
			return value.some((i) => i.id === itemId);
		}
		return props.value?.id === itemId;
	}

	function getQuantity(itemId: string): number {
		if (Array.isArray(props.value)) {
			const item = value.find((i) => i.id === itemId);
			return item?.quantity || 1;
		}
		return 1;
	}

	function calculateTotal(): number {
		if (Array.isArray(props.value)) {
			return value.reduce((sum, item) => sum + item.price * (item.quantity || 1), 0);
		}
		return props.value?.price || 0;
	}
</script>

<div class="payment-field">
	<label class="field-label" for="payment-field-{props.field.name}">
		{props.field.label}
		{#if props.field.required}
			<span class="required">*</span>
		{/if}
	</label>

	{#if props.field.help_text}
		<p class="field-help">{props.field.help_text}</p>
	{/if}

	<div class="payment-items" class:single={paymentType === 'single'}>
		{#each items as item (item.id)}
			<div
				class="payment-item"
				class:selected={isSelected(item.id)}
				role="button"
				tabindex="0"
				onclick={() => {
					if (paymentType === 'single') {
						handleSingleSelect(item);
					}
				}}
				onkeydown={(e: KeyboardEvent) => {
					if (e.key === 'Enter' || e.key === ' ') {
						if (paymentType === 'single') handleSingleSelect(item);
					}
				}}
			>
				{#if paymentType === 'multiple'}
					<input
						id="payment-item-{item.id}"
						type="checkbox"
						checked={isSelected(item.id)}
						onchange={(e: Event) =>
							handleMultiSelect(item, (e.currentTarget as HTMLInputElement).checked)}
					/>
				{:else}
					<input
						id="payment-item-{item.id}"
						type="radio"
						name={props.field.name}
						checked={isSelected(item.id)}
						onchange={() => handleSingleSelect(item)}
					/>
				{/if}

				{#if item.image}
					<img src={item.image} alt={item.label} class="item-image" />
				{/if}

				<div class="item-details">
					<span class="item-label">{item.label}</span>
					<span class="item-price">{formatCurrency(item.price)}</span>
				</div>

				{#if allowQuantity && isSelected(item.id)}
					<div class="quantity-control">
						<button
							type="button"
							class="qty-btn"
							aria-label="Decrease quantity"
							onclick={(e: MouseEvent) => {
								e.stopPropagation();
								const qty = getQuantity(item.id);
								if (qty > 1) handleQuantityChange(item.id, qty - 1);
							}}
						>
							-
						</button>
						<input
							id="qty-{item.id}"
							type="number"
							min="1"
							value={getQuantity(item.id)}
							class="qty-input"
							aria-label="Quantity"
							onclick={(e: MouseEvent) => e.stopPropagation()}
							onchange={(e: Event) =>
								handleQuantityChange(
									item.id,
									parseInt((e.currentTarget as HTMLInputElement).value) || 1
								)}
						/>
						<button
							type="button"
							class="qty-btn"
							aria-label="Increase quantity"
							onclick={(e: MouseEvent) => {
								e.stopPropagation();
								handleQuantityChange(item.id, getQuantity(item.id) + 1);
							}}
						>
							+
						</button>
					</div>
				{/if}
			</div>
		{/each}
	</div>

	<div class="payment-total">
		<span class="total-label">Total:</span>
		<span class="total-amount">{formatCurrency(calculateTotal())}</span>
	</div>

	{#if error && error.length > 0}
		<div class="field-error">
			{#each error as err}
				<p>{err}</p>
			{/each}
		</div>
	{/if}
</div>

<style>
	.payment-field {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.field-label {
		font-weight: 500;
		font-size: 0.875rem;
		color: #374151;
	}

	.required {
		color: #dc2626;
		margin-left: 0.25rem;
	}

	.field-help {
		font-size: 0.75rem;
		color: #6b7280;
		margin: 0;
	}

	.payment-items {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.payment-items.single {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
	}

	.payment-item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem;
		border: 2px solid #e5e7eb;
		border-radius: 0.5rem;
		cursor: pointer;
		transition: all 0.2s;
		background-color: white;
	}

	.payment-item:hover {
		border-color: #d1d5db;
		background-color: #f9fafb;
	}

	.payment-item.selected {
		border-color: #2563eb;
		background-color: #eff6ff;
	}

	.payment-item input[type='radio'],
	.payment-item input[type='checkbox'] {
		width: 1.125rem;
		height: 1.125rem;
		accent-color: #2563eb;
		cursor: pointer;
	}

	.item-image {
		width: 48px;
		height: 48px;
		object-fit: cover;
		border-radius: 0.375rem;
	}

	.item-details {
		flex: 1;
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.item-label {
		font-weight: 500;
		color: #111827;
	}

	.item-price {
		font-size: 1.125rem;
		font-weight: 600;
		color: #059669;
	}

	.quantity-control {
		display: flex;
		align-items: center;
		gap: 0.25rem;
	}

	.qty-btn {
		width: 28px;
		height: 28px;
		display: flex;
		align-items: center;
		justify-content: center;
		border: 1px solid #d1d5db;
		border-radius: 0.25rem;
		background-color: white;
		cursor: pointer;
		font-size: 1rem;
		font-weight: 600;
	}

	.qty-btn:hover {
		background-color: #f3f4f6;
	}

	.qty-input {
		width: 50px;
		height: 28px;
		text-align: center;
		border: 1px solid #d1d5db;
		border-radius: 0.25rem;
		font-size: 0.875rem;
	}

	.payment-total {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1rem;
		background-color: #f3f4f6;
		border-radius: 0.5rem;
		margin-top: 0.5rem;
	}

	.total-label {
		font-weight: 500;
		color: #374151;
	}

	.total-amount {
		font-size: 1.5rem;
		font-weight: 700;
		color: #059669;
	}

	.field-error {
		font-size: 0.75rem;
		color: #dc2626;
	}

	.field-error p {
		margin: 0;
	}
</style>

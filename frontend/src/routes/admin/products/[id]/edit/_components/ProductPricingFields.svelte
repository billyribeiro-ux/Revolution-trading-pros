<script lang="ts">
	/**
	 * R20-C extraction (2026-05-20): pricing + thumbnail fields.
	 * Mutates the parent's `formData` proxy directly.
	 */
	import { IconPhoto } from '$lib/icons';

	interface FormFields {
		price: string;
		sale_price: string;
		thumbnail: string;
	}

	interface Props {
		formData: FormFields;
		getFieldError: (field: string) => string | undefined;
	}

	let { formData = $bindable(), getFieldError }: Props = $props();
</script>

<div class="form-row">
	<div class={['form-group', { 'has-error': getFieldError('price') }]}>
		<label for="price">Price (USD) *</label>
		<div class="price-input">
			<span class="currency">$</span>
			<input
				id="price"
				name="price"
				type="number"
				bind:value={formData.price}
				placeholder="99.00"
				step="0.01"
				min="0"
			/>
		</div>
		{#if getFieldError('price')}
			<span class="field-error">{getFieldError('price')}</span>
		{/if}
	</div>

	<div class="form-group">
		<label for="sale_price">Sale Price (Optional)</label>
		<div class="price-input">
			<span class="currency">$</span>
			<input
				id="sale_price"
				name="sale_price"
				type="number"
				bind:value={formData.sale_price}
				placeholder="79.00"
				step="0.01"
				min="0"
			/>
		</div>
	</div>
</div>

<div class="form-group">
	<label for="thumbnail">
		<IconPhoto size={16} />
		Thumbnail URL
	</label>
	<input
		id="thumbnail"
		name="thumbnail"
		type="url"
		bind:value={formData.thumbnail}
		placeholder="https://example.com/image.jpg"
	/>
</div>

<style>
	.form-group {
		margin-bottom: 1.5rem;
	}

	.form-group.has-error input {
		border-color: #ef4444;
	}

	.form-group label {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-weight: 500;
		color: #f1f5f9;
		margin-bottom: 0.5rem;
		font-size: 0.9375rem;
	}

	.form-group input[type='number'],
	.form-group input[type='url'] {
		width: 100%;
		padding: 0.75rem 1rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.25);
		border-radius: 8px;
		color: #f1f5f9;
		font-size: 0.9375rem;
		transition: all 0.2s;
	}

	.form-group input:focus {
		outline: none;
		border-color: #3b82f6;
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.15);
	}

	.field-error {
		display: block;
		color: #f87171;
		font-size: 0.8125rem;
		margin-top: 0.375rem;
	}

	.form-row {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1rem;
	}

	.price-input {
		position: relative;
		display: flex;
		align-items: center;
	}

	.price-input .currency {
		position: absolute;
		left: 1rem;
		color: #3b82f6;
		font-weight: 600;
		font-size: 1rem;
	}

	.price-input input {
		padding-left: 2.25rem;
	}

	@media (max-width: 1023.98px) {
		.form-row {
			grid-template-columns: 1fr;
		}
	}
</style>

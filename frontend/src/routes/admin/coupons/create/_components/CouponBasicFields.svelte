<script lang="ts">
	/**
	 * R25-C extraction (2026-05-20): coupon code, description, discount-type
	 * toggle, value input, and min/max purchase fields. Mutates the parent's
	 * `formData` proxy via `$bindable()` so all existing `bind:value` semantics
	 * carry over unchanged.
	 *
	 * `discountPreview` is computed in the parent (it's used by the preview
	 * card inside the value-input wrapper) and passed in to avoid duplicating
	 * the `$derived` logic across two surfaces.
	 */
	import { IconTag, IconSparkles, IconPercentage, IconCurrencyDollar } from '$lib/icons';
	import type { CouponFormData } from './types';

	interface Props {
		formData: CouponFormData;
		generating: boolean;
		discountPreview: string;
		onGenerate: () => void;
	}

	let { formData = $bindable(), generating, discountPreview, onGenerate }: Props = $props();
</script>

<h2><IconTag size={20} /> Coupon Details</h2>

<!-- Coupon Code -->
<div class="form-group">
	<label for="coupon-code">Coupon Code</label>
	<div class="input-with-button">
		<input
			id="coupon-code"
			name="code"
			type="text"
			class="input input-code"
			bind:value={formData.code}
			placeholder="SUMMER2024"
			autocomplete="off"
			required
		/>
		<button type="button" class="btn-generate" onclick={onGenerate} disabled={generating}>
			<IconSparkles size={18} />
			{generating ? 'Generating...' : 'Generate'}
		</button>
	</div>
	<span class="help-text">Letters, numbers, dashes, and underscores only</span>
</div>

<!-- Description -->
<div class="form-group">
	<label for="coupon-description">Description (Optional)</label>
	<input
		id="coupon-description"
		name="description"
		type="text"
		class="input"
		bind:value={formData.description}
		placeholder="e.g., Summer sale discount for all courses"
	/>
</div>

<!-- Discount Type Toggle -->
<div class="form-group">
	<span class="form-label">Discount Type</span>
	<div class="discount-type-toggle" role="group" aria-label="Discount Type">
		<button
			type="button"
			class="type-btn"
			class:active={formData.discount_type === 'percentage'}
			onclick={() => (formData.discount_type = 'percentage')}
		>
			<IconPercentage size={20} />
			Percentage
		</button>
		<button
			type="button"
			class="type-btn"
			class:active={formData.discount_type === 'fixed'}
			onclick={() => (formData.discount_type = 'fixed')}
		>
			<IconCurrencyDollar size={20} />
			Fixed Amount
		</button>
	</div>
</div>

<!-- Discount Value -->
<div class="form-group">
	<label for="discount-value">Discount Value</label>
	<div class="value-input-wrapper">
		{#if formData.discount_type === 'fixed'}
			<span class="value-prefix">$</span>
		{/if}
		<input
			id="discount-value"
			name="discount_value"
			type="number"
			class="input input-value"
			bind:value={formData.discount_value}
			min="0"
			max={formData.discount_type === 'percentage' ? 100 : undefined}
			step="0.01"
			required
		/>
		{#if formData.discount_type === 'percentage'}
			<span class="value-suffix">%</span>
		{/if}
	</div>
	{#if discountPreview}
		<div class="discount-preview">
			Customers will receive <strong>{discountPreview}</strong>
		</div>
	{/if}
</div>

<!-- Min Purchase & Max Discount -->
<div class="form-row">
	<div class="form-group">
		<label for="min-purchase">Minimum Purchase (Optional)</label>
		<input
			id="min-purchase"
			name="min_purchase"
			type="number"
			class="input"
			bind:value={formData.min_purchase}
			min="0"
			step="0.01"
			placeholder="No minimum"
		/>
	</div>
	<div class="form-group">
		<label for="max-discount">Maximum Discount (Optional)</label>
		<input
			id="max-discount"
			name="max_discount"
			type="number"
			class="input"
			bind:value={formData.max_discount}
			min="0"
			step="0.01"
			placeholder="Unlimited"
		/>
		<span class="help-text">Caps total discount for percentage coupons</span>
	</div>
</div>

<style>
	h2 {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 1.125rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0 0 1.25rem 0;
	}

	.form-group {
		margin-bottom: 1.25rem;
	}

	.form-group:last-child {
		margin-bottom: 0;
	}

	.form-row {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
		gap: 1.25rem;
	}

	label,
	.form-label {
		display: block;
		font-weight: 600;
		color: #e2e8f0;
		margin-bottom: 0.5rem;
		font-size: 0.9rem;
	}

	.input {
		width: 100%;
		padding: 0.75rem 1rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		color: #f1f5f9;
		font-size: 0.95rem;
		transition: all 0.2s;
	}

	.input:focus {
		outline: none;
		border-color: rgba(230, 184, 0, 0.5);
		box-shadow: 0 0 0 3px rgba(230, 184, 0, 0.1);
	}

	.input::placeholder {
		color: #64748b;
	}

	.input-code {
		font-family: 'SF Mono', 'Monaco', 'Inconsolata', monospace;
		font-size: 1.1rem;
		font-weight: 600;
		letter-spacing: 0.05em;
	}

	.input-with-button {
		display: flex;
		gap: 0.75rem;
	}

	.input-with-button .input {
		flex: 1;
	}

	.btn-generate {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1rem;
		background: rgba(100, 116, 139, 0.2);
		border: 1px solid rgba(100, 116, 139, 0.3);
		border-radius: 8px;
		color: #cbd5e1;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
		white-space: nowrap;
	}

	.btn-generate:hover:not(:disabled) {
		background: rgba(100, 116, 139, 0.3);
	}

	.btn-generate:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.help-text {
		display: block;
		margin-top: 0.375rem;
		font-size: 0.8rem;
		color: #64748b;
	}

	.discount-type-toggle {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 0.75rem;
	}

	.type-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 1rem;
		background: rgba(15, 23, 42, 0.6);
		border: 2px solid rgba(148, 163, 184, 0.2);
		border-radius: 10px;
		color: #94a3b8;
		font-weight: 600;
		font-size: 0.95rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.type-btn:hover {
		border-color: rgba(230, 184, 0, 0.3);
		color: #cbd5e1;
	}

	.type-btn.active {
		background: rgba(230, 184, 0, 0.1);
		border-color: var(--primary-500);
		color: var(--primary-500);
	}

	.value-input-wrapper {
		display: flex;
		align-items: center;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		overflow: hidden;
	}

	.value-input-wrapper:focus-within {
		border-color: rgba(230, 184, 0, 0.5);
		box-shadow: 0 0 0 3px rgba(230, 184, 0, 0.1);
	}

	.value-prefix,
	.value-suffix {
		padding: 0.75rem 1rem;
		color: #64748b;
		font-weight: 600;
		font-size: 1.1rem;
		background: rgba(30, 41, 59, 0.5);
	}

	.value-prefix {
		border-right: 1px solid rgba(148, 163, 184, 0.2);
	}

	.value-suffix {
		border-left: 1px solid rgba(148, 163, 184, 0.2);
	}

	.input-value {
		flex: 1;
		border: none;
		background: transparent;
		font-size: 1.25rem;
		font-weight: 600;
		text-align: center;
	}

	.input-value:focus {
		outline: none;
		box-shadow: none;
	}

	.discount-preview {
		margin-top: 0.75rem;
		padding: 0.75rem 1rem;
		background: rgba(230, 184, 0, 0.1);
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 8px;
		color: var(--primary-500);
		font-size: 0.9rem;
		text-align: center;
	}

	.discount-preview strong {
		font-size: 1.1rem;
	}

	@media (max-width: 639.98px) {
		.discount-type-toggle {
			grid-template-columns: 1fr;
		}

		.form-row {
			grid-template-columns: 1fr;
		}

		.input-with-button {
			flex-direction: column;
		}
	}
</style>

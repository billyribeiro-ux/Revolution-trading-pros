<script lang="ts">
	import type { FormField } from '$lib/api/forms';

	interface CouponResult {
		valid: boolean;
		code: string;
		discount_type: 'percentage' | 'fixed';
		discount_value: number;
		message: string;
	}

	interface Props {
		field: FormField;
		value?: string;
		error?: string[];
		formId?: number;
		onchange?: (value: string) => void;
		onApply?: (result: CouponResult) => void;
	}

	let { field, value = '', error, formId, onchange, onApply }: Props = $props();

	let isValidating = $state(false);
	let validationResult = $state<CouponResult | null>(null);
	let inputValue = $state(value);

	async function validateCoupon() {
		if (!inputValue.trim()) {
			validationResult = null;
			return;
		}

		isValidating = true;
		validationResult = null;

		try {
			const response = await fetch('/api/forms/coupons/validate', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({
					code: inputValue.trim().toUpperCase(),
					form_id: formId
				})
			});

			const result: CouponResult = await response.json();
			validationResult = result;

			if (result.valid) {
				onchange?.(result.code);
				onApply?.(result);
			}
		} catch {
			validationResult = {
				valid: false,
				code: inputValue,
				discount_type: 'percentage',
				discount_value: 0,
				message: 'Failed to validate coupon. Please try again.'
			};
		} finally {
			isValidating = false;
		}
	}

	function handleInput(event: Event) {
		const target = event.target as HTMLInputElement;
		inputValue = target.value.toUpperCase();
		validationResult = null;
	}

	function clearCoupon() {
		inputValue = '';
		validationResult = null;
		onchange?.('');
	}
</script>

<div class="coupon-field">
	<label class="field-label" for={`field-${field.name}`}>
		{field.label || 'Coupon Code'}
		{#if field.required}
			<span class="required">*</span>
		{/if}
	</label>

	{#if field.help_text}
		<p class="field-help">{field.help_text}</p>
	{/if}

	<div class="coupon-input-wrapper">
		<input
			type="text"
			id={`field-${field.name}`}
			name={field.name}
			placeholder={field.placeholder || 'Enter coupon code'}
			value={inputValue}
			class="coupon-input"
			class:valid={validationResult?.valid}
			class:invalid={validationResult && !validationResult.valid}
			disabled={isValidating || validationResult?.valid}
			oninput={handleInput}
			onkeydown={(e) => {
				if (e.key === 'Enter') {
					e.preventDefault();
					validateCoupon();
				}
			}}
		/>

		{#if validationResult?.valid}
			<button type="button" class="clear-btn" onclick={clearCoupon} aria-label="Remove coupon">
				<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<line x1="18" y1="6" x2="6" y2="18"></line>
					<line x1="6" y1="6" x2="18" y2="18"></line>
				</svg>
			</button>
		{:else}
			<button
				type="button"
				class="apply-btn"
				onclick={validateCoupon}
				disabled={isValidating || !inputValue.trim()}
			>
				{#if isValidating}
					<span class="spinner"></span>
				{:else}
					Apply
				{/if}
			</button>
		{/if}
	</div>

	{#if validationResult}
		<div class="validation-result" class:success={validationResult.valid} class:error={!validationResult.valid}>
			{#if validationResult.valid}
				<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<polyline points="20 6 9 17 4 12"></polyline>
				</svg>
				<span>
					{validationResult.message ||
						`${validationResult.discount_type === 'percentage' ? validationResult.discount_value + '%' : '$' + validationResult.discount_value} discount applied!`}
				</span>
			{:else}
				<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<circle cx="12" cy="12" r="10"></circle>
					<line x1="12" y1="8" x2="12" y2="12"></line>
					<line x1="12" y1="16" x2="12.01" y2="16"></line>
				</svg>
				<span>{validationResult.message || 'Invalid coupon code'}</span>
			{/if}
		</div>
	{/if}

	{#if error && error.length > 0}
		<div class="field-error">
			{#each error as err}
				<p>{err}</p>
			{/each}
		</div>
	{/if}
</div>

<style>
	.coupon-field {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
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

	.coupon-input-wrapper {
		display: flex;
		gap: 0.5rem;
	}

	.coupon-input {
		flex: 1;
		padding: 0.625rem 0.875rem;
		border: 2px solid #e5e7eb;
		border-radius: 0.375rem;
		font-size: 0.875rem;
		font-family: monospace;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		transition: all 0.2s;
	}

	.coupon-input:focus {
		outline: none;
		border-color: #2563eb;
		box-shadow: 0 0 0 3px rgba(37, 99, 235, 0.1);
	}

	.coupon-input.valid {
		border-color: #059669;
		background-color: #ecfdf5;
	}

	.coupon-input.invalid {
		border-color: #dc2626;
	}

	.coupon-input:disabled {
		background-color: #f3f4f6;
		cursor: not-allowed;
	}

	.apply-btn {
		padding: 0.625rem 1.25rem;
		background-color: #2563eb;
		color: white;
		border: none;
		border-radius: 0.375rem;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
		min-width: 80px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.apply-btn:hover:not(:disabled) {
		background-color: #1d4ed8;
	}

	.apply-btn:disabled {
		background-color: #9ca3af;
		cursor: not-allowed;
	}

	.clear-btn {
		padding: 0.625rem;
		background-color: #f3f4f6;
		color: #6b7280;
		border: 1px solid #d1d5db;
		border-radius: 0.375rem;
		cursor: pointer;
		transition: all 0.2s;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.clear-btn:hover {
		background-color: #e5e7eb;
		color: #374151;
	}

	.spinner {
		width: 16px;
		height: 16px;
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-top-color: white;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.validation-result {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 0.75rem;
		border-radius: 0.375rem;
		font-size: 0.875rem;
	}

	.validation-result.success {
		background-color: #ecfdf5;
		color: #059669;
	}

	.validation-result.error {
		background-color: #fef2f2;
		color: #dc2626;
	}

	.field-error {
		font-size: 0.75rem;
		color: #dc2626;
	}

	.field-error p {
		margin: 0;
	}
</style>

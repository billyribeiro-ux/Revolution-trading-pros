<script lang="ts">
	/**
	 * R23-C extraction (2026-05-20): basic information section — code +
	 * discount type + value. Mutates the parent's `formData` proxy via
	 * `$bindable()` so existing `bind:value` semantics carry over.
	 *
	 * FIX-2026-04-26 (P0-7): `oninput` re-write was removed in the parent —
	 * display uppercase via CSS, coerce `.toUpperCase()` at submit.
	 */
	import type { CouponFormData } from './types';

	interface Props {
		formData: CouponFormData;
		getFieldError: (field: string) => string | null;
	}

	let { formData = $bindable(), getFieldError }: Props = $props();
</script>

<div class="form-section">
	<h2>Basic Information</h2>

	<div class="form-group" data-field="code">
		<label for="code">
			Coupon Code *
			<span class="label-hint">Unique identifier</span>
		</label>
		<input
			type="text"
			id="code"
			name="code"
			bind:value={formData.code}
			placeholder="SUMMER2024"
			class="input input-large"
			class:error={getFieldError('code')}
			required
			style="text-transform: uppercase;"
		/>
		{#if getFieldError('code')}
			<span class="field-error">{getFieldError('code')}</span>
		{/if}
	</div>

	<div class="form-row">
		<div class="form-group" data-field="type">
			<label for="type">Discount Type *</label>
			<select id="type" bind:value={formData.type} class="input">
				<option value="percentage">Percentage Off</option>
				<option value="fixed">Fixed Amount Off</option>
				<option value="free_shipping">Free Shipping</option>
			</select>
		</div>

		<div class="form-group" data-field="value">
			<label for="value">
				{formData.type === 'percentage' ? 'Percentage' : 'Amount'} *
			</label>
			<div class="input-with-suffix">
				<input
					type="number"
					id="value"
					name="value"
					bind:value={formData.value}
					min="0"
					max={formData.type === 'percentage' ? 100 : undefined}
					step="0.01"
					class="input"
					class:error={getFieldError('value')}
					disabled={formData.type === 'free_shipping'}
				/>
				<span class="input-suffix">
					{formData.type === 'percentage' ? '%' : '$'}
				</span>
			</div>
			{#if getFieldError('value')}
				<span class="field-error">{getFieldError('value')}</span>
			{/if}
		</div>
	</div>
</div>

<style>
	.form-section {
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 12px;
		padding: 2rem;
	}

	.form-section h2 {
		font-size: 1.25rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0 0 1.5rem 0;
	}

	.form-group {
		margin-bottom: 1.5rem;
	}

	.form-group:last-child {
		margin-bottom: 0;
	}

	.form-row {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
		gap: 1.5rem;
	}

	.form-row .form-group {
		margin-bottom: 0;
	}

	label {
		display: block;
		font-weight: 600;
		color: #f1f5f9;
		margin-bottom: 0.5rem;
		font-size: 0.95rem;
	}

	.label-hint {
		font-weight: 400;
		color: #94a3b8;
		font-size: 0.875rem;
		margin-left: 0.5rem;
	}

	.input {
		width: 100%;
		padding: 0.75rem 1rem;
		background: rgba(15, 23, 42, 0.8);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		color: #f1f5f9;
		font-size: 0.95rem;
		transition: all 0.2s;
	}

	.input:focus {
		outline: none;
		border-color: rgba(59, 130, 246, 0.5);
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
	}

	.input.error {
		border-color: rgba(239, 68, 68, 0.5);
	}

	.input-large {
		font-size: 1.25rem;
		font-weight: 600;
		letter-spacing: 0.05em;
	}

	.input-with-suffix {
		position: relative;
	}

	.input-with-suffix .input {
		padding-right: 3rem;
	}

	.input-suffix {
		position: absolute;
		right: 1rem;
		top: 50%;
		transform: translateY(-50%);
		color: #64748b;
		font-weight: 600;
	}

	.field-error {
		display: block;
		margin-top: 0.5rem;
		font-size: 0.875rem;
		color: #f87171;
	}

	@media (max-width: 767.98px) {
		.form-row {
			grid-template-columns: 1fr;
		}
	}
</style>

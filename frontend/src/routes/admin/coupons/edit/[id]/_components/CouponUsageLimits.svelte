<script lang="ts">
	/**
	 * R23-C extraction (2026-05-20): usage-limits section — minimum
	 * purchase, total uses, plus the Stripe Coupon `duration` semantics
	 * (Batch 3.5+). When `duration !== 'repeating'` we clear
	 * `duration_in_months` so the wire payload stays clean.
	 */
	import type { CouponFormData } from './types';

	interface Props {
		formData: CouponFormData;
		usageCount: number | null;
		getFieldError: (field: string) => string | null;
	}

	let { formData = $bindable(), usageCount, getFieldError }: Props = $props();
</script>

<div class="form-section">
	<h2>Usage Limits</h2>

	<div class="form-row">
		<div class="form-group" data-field="minimum_amount">
			<label for="minimum_amount">Minimum Purchase ($)</label>
			<input
				type="number"
				id="minimum_amount"
				name="minimum_amount"
				bind:value={formData.minimum_amount}
				min="0"
				step="0.01"
				class="input"
				placeholder="No minimum"
			/>
		</div>

		<div class="form-group" data-field="usage_limit">
			<label for="usage_limit">Total Uses</label>
			<input
				type="number"
				id="usage_limit"
				name="usage_limit"
				bind:value={formData.usage_limit}
				min="1"
				class="input"
				placeholder="Unlimited"
			/>
			{#if usageCount}
				<p class="help-text">Current usage: {usageCount}</p>
			{/if}
		</div>
	</div>

	<!-- Subscription Duration (Stripe Coupon `duration` semantics) -->
	<div class="form-row">
		<div class="form-group" data-field="duration">
			<label for="coupon-duration">Subscription Duration</label>
			<select
				id="coupon-duration"
				name="duration"
				class="input"
				bind:value={formData.duration}
				onchange={() => {
					if (formData.duration !== 'repeating') {
						formData.duration_in_months = null;
					}
				}}
			>
				<option value="once">Once — first billing period only</option>
				<option value="forever">Forever — every billing period</option>
				<option value="repeating">Repeating — first N months</option>
			</select>
			<p class="help-text">
				Editing the duration creates a new Stripe Coupon and flips the pointer; existing subscribers
				keep their current discount.
			</p>
		</div>

		{#if formData.duration === 'repeating'}
			<div class="form-group" data-field="duration_in_months">
				<label for="duration-in-months">Number of Months</label>
				<input
					type="number"
					id="duration-in-months"
					name="duration_in_months"
					bind:value={formData.duration_in_months}
					min="1"
					max="36"
					class="input"
					class:error={getFieldError('duration_in_months')}
					placeholder="e.g. 3"
				/>
				{#if getFieldError('duration_in_months')}
					<span class="field-error">{getFieldError('duration_in_months')}</span>
				{/if}
			</div>
		{/if}
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
		margin-bottom: 1.5rem;
	}

	.form-row:last-child {
		margin-bottom: 0;
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

	.field-error {
		display: block;
		margin-top: 0.5rem;
		font-size: 0.875rem;
		color: #f87171;
	}

	.help-text {
		margin-top: 0.5rem;
		font-size: 0.875rem;
		color: #94a3b8;
	}

	@media (max-width: 767.98px) {
		.form-row {
			grid-template-columns: 1fr;
		}
	}
</style>

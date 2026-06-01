<script lang="ts">
	/**
	 * R25-C extraction (2026-05-20): Stripe-coupon `duration` semantics —
	 * once / forever / repeating + conditional duration_in_months input.
	 * Wraps the parent's auto-clear logic (set duration_in_months to null
	 * whenever duration leaves 'repeating') inside the `onchange` handler.
	 */
	import type { CouponFormData } from './types';

	interface Props {
		formData: CouponFormData;
	}

	let { formData = $bindable() }: Props = $props();

	function handleDurationChange(): void {
		if (formData.duration !== 'repeating') {
			formData.duration_in_months = null;
		}
	}
</script>

<div class="form-row">
	<div class="form-group">
		<label for="coupon-duration">Subscription Duration</label>
		<select
			id="coupon-duration"
			name="duration"
			class="input"
			bind:value={formData.duration}
			onchange={handleDurationChange}
		>
			<option value="once">Once — first billing period only</option>
			<option value="forever">Forever — every billing period</option>
			<option value="repeating">Repeating — first N months</option>
		</select>
		<span class="help-text"
			>Applies only to subscriptions; one-time products always charge once.</span
		>
	</div>
	{#if formData.duration === 'repeating'}
		<div class="form-group">
			<label for="duration-in-months">Number of Months</label>
			<input
				id="duration-in-months"
				name="duration_in_months"
				type="number"
				class="input"
				bind:value={formData.duration_in_months}
				min="1"
				max="36"
				placeholder="e.g. 3"
			/>
			<span class="help-text">How many billing periods get the discount.</span>
		</div>
	{/if}
</div>

<style>
	.form-row {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
		gap: 1.25rem;
		margin-bottom: 1.25rem;
	}

	.form-group {
		margin: 0;
	}

	label {
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

	.help-text {
		display: block;
		margin-top: 0.375rem;
		font-size: 0.8rem;
		color: #64748b;
	}

	@media (max-width: 639.98px) {
		.form-row {
			grid-template-columns: 1fr;
		}
	}
</style>

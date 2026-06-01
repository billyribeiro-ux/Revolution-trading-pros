<script lang="ts">
	/**
	 * R25-C extraction (2026-05-20): usage limits (total + per-user), start /
	 * expiration dates, and the is_active checkbox. All fields bind directly
	 * to the parent's `formData` proxy via `$bindable()`.
	 */
	import type { CouponFormData } from './types';

	interface Props {
		formData: CouponFormData;
	}

	let { formData = $bindable() }: Props = $props();
</script>

<!-- Usage Limits -->
<div class="form-row">
	<div class="form-group">
		<label for="usage-limit">Total Usage Limit (Optional)</label>
		<input
			id="usage-limit"
			name="usage_limit"
			type="number"
			class="input"
			bind:value={formData.usage_limit}
			min="1"
			placeholder="Unlimited"
		/>
	</div>
	<div class="form-group">
		<label for="usage-limit-per-user">Per User Limit</label>
		<input
			id="usage-limit-per-user"
			name="usage_limit_per_user"
			type="number"
			class="input"
			bind:value={formData.usage_limit_per_user}
			min="1"
			placeholder="1"
		/>
	</div>
</div>

<!-- Dates -->
<div class="form-row">
	<div class="form-group">
		<label for="starts-at">Start Date</label>
		<input
			id="starts-at"
			name="starts_at"
			type="datetime-local"
			class="input"
			bind:value={formData.starts_at}
		/>
	</div>
	<div class="form-group">
		<label for="expires-at">Expiration Date</label>
		<input
			id="expires-at"
			name="expires_at"
			type="datetime-local"
			class="input"
			bind:value={formData.expires_at}
		/>
	</div>
</div>

<!-- Active Toggle -->
<div class="form-group">
	<label class="checkbox-label">
		<input id="is-active" name="is_active" type="checkbox" bind:checked={formData.is_active} />
		Coupon is active and can be used
	</label>
</div>

<style>
	.form-row {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
		gap: 1.25rem;
		margin-bottom: 1.25rem;
	}

	.form-group {
		margin: 0 0 1.25rem 0;
	}

	.form-group:last-child {
		margin-bottom: 0;
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

	.checkbox-label {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		cursor: pointer;
		color: #cbd5e1;
		font-weight: 500;
		margin-bottom: 0;
	}

	.checkbox-label input[type='checkbox'] {
		width: 18px;
		height: 18px;
		accent-color: var(--primary-500);
	}

	@media (max-width: 639.98px) {
		.form-row {
			grid-template-columns: 1fr;
		}
	}
</style>

<script lang="ts">
	/**
	 * R23-C extraction (2026-05-20): settings section — `is_active` and
	 * `stackable` toggles. Toggle visual is the same custom switch the
	 * other admin pages use.
	 */
	import type { CouponFormData } from './types';

	interface Props {
		formData: CouponFormData;
	}

	let { formData = $bindable() }: Props = $props();
</script>

<div class="form-section">
	<h2>Settings</h2>

	<div class="form-checkboxes">
		<label class="toggle-label">
			<input
				id="page-formdata-is-active"
				name="page-formdata-is-active"
				type="checkbox"
				bind:checked={formData.is_active}
				class="toggle-checkbox"
			/>
			<span class="toggle-switch"></span>
			<span class="toggle-text">Active</span>
			<span class="toggle-hint">Coupon can be used by customers</span>
		</label>

		<label class="toggle-label">
			<input
				id="page-formdata-stackable"
				name="page-formdata-stackable"
				type="checkbox"
				bind:checked={formData.stackable}
				class="toggle-checkbox"
			/>
			<span class="toggle-switch"></span>
			<span class="toggle-text">Stackable</span>
			<span class="toggle-hint">Can be combined with other coupons</span>
		</label>
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

	.form-checkboxes {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.toggle-label {
		display: flex;
		align-items: center;
		gap: 1rem;
		cursor: pointer;
		margin-bottom: 0;
	}

	.toggle-checkbox {
		display: none;
	}

	.toggle-switch {
		position: relative;
		width: 52px;
		height: 28px;
		background: rgba(148, 163, 184, 0.2);
		border: 1px solid rgba(148, 163, 184, 0.3);
		border-radius: 28px;
		transition: 0.3s;
		flex-shrink: 0;
	}

	.toggle-switch:before {
		content: '';
		position: absolute;
		height: 20px;
		width: 20px;
		left: 3px;
		bottom: 3px;
		background: #94a3b8;
		border-radius: 50%;
		transition: 0.3s;
	}

	.toggle-checkbox:checked + .toggle-switch {
		background: linear-gradient(135deg, var(--primary-500), var(--primary-600));
		border-color: transparent;
	}

	.toggle-checkbox:checked + .toggle-switch:before {
		transform: translateX(24px);
		background: white;
	}

	.toggle-text {
		color: #f1f5f9;
		font-weight: 600;
	}

	.toggle-hint {
		color: #64748b;
		font-size: 0.875rem;
		font-weight: 400;
	}
</style>

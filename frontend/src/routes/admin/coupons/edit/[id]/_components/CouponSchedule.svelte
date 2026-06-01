<script lang="ts">
	/**
	 * R23-C extraction (2026-05-20): schedule section — start / expiration
	 * datetimes. Stored as `datetime-local` strings; the parent coerces
	 * to ISO on submit.
	 */
	import type { CouponFormData } from './types';

	interface Props {
		formData: CouponFormData;
		getFieldError: (field: string) => string | null;
	}

	let { formData = $bindable(), getFieldError }: Props = $props();
</script>

<div class="form-section">
	<h2>Schedule</h2>

	<div class="form-row">
		<div class="form-group" data-field="valid_from">
			<label for="valid_from">Start Date</label>
			<input
				type="datetime-local"
				id="valid_from"
				name="valid_from"
				bind:value={formData.valid_from}
				class="input"
			/>
		</div>

		<div class="form-group" data-field="valid_until">
			<label for="valid_until">Expiration Date</label>
			<input
				type="datetime-local"
				id="valid_until"
				name="valid_until"
				bind:value={formData.valid_until}
				class="input"
				class:error={getFieldError('valid_until')}
			/>
			{#if getFieldError('valid_until')}
				<span class="field-error">{getFieldError('valid_until')}</span>
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
		margin-bottom: 0;
	}

	.form-row {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
		gap: 1.5rem;
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

	@media (max-width: 767.98px) {
		.form-row {
			grid-template-columns: 1fr;
		}
	}
</style>

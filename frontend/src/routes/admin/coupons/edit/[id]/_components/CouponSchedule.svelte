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
			<input autocomplete="off"
				type="datetime-local"
				id="valid_from"
				name="valid_from"
				bind:value={formData.valid_from}
				class="input"
			/>
		</div>

		<div class="form-group" data-field="valid_until">
			<label for="valid_until">Expiration Date</label>
			<input autocomplete="off"
				type="datetime-local"
				id="valid_until"
				name="valid_until"
				bind:value={formData.valid_until}
				class={['input', { error: getFieldError('valid_until') }]}
			/>
			{#if getFieldError('valid_until')}
				<span class="field-error">{getFieldError('valid_until')}</span>
			{/if}
		</div>
	</div>
</div>

<style>
	.form-section {
		background: var(--bg-elevated, #161b22);
		border: 1px solid var(--border-muted, #21262d);
		border-radius: 12px;
		padding: 2rem;
	}

	.form-section h2 {
		font-size: 1.25rem;
		font-weight: 600;
		color: var(--text-primary, #f0f6fc);
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
		color: var(--text-primary, #f0f6fc);
		margin-bottom: 0.5rem;
		font-size: 0.95rem;
	}

	.input {
		width: 100%;
		padding: 0.75rem 1rem;
		background: var(--bg-base, #0d1117);
		border: 1px solid var(--border-default, #30363d);
		border-radius: 8px;
		color: var(--text-primary, #f0f6fc);
		font-size: 0.95rem;
		transition: all 0.2s;
	}

	.input:focus {
		outline: none;
		border-color: rgba(88, 166, 255, 0.5);
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
	}

	.input.error {
		border-color: rgba(248, 81, 73, 0.5);
	}

	.field-error {
		display: block;
		margin-top: 0.5rem;
		font-size: 0.875rem;
		color: var(--error-emphasis, #f85149);
	}

	@media (max-width: 767.98px) {
		.form-row {
			grid-template-columns: 1fr;
		}
	}
</style>

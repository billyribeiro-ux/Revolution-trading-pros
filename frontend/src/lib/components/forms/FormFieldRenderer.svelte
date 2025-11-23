<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import type { FormField } from '$lib/api/forms';

	export let field: FormField;
	export let value: any = '';
	export let error: string[] | undefined = undefined;

	const dispatch = createEventDispatcher();

	function handleChange(event: Event) {
		const target = event.target as HTMLInputElement | HTMLTextAreaElement | HTMLSelectElement;

		let newValue: any;

		if (target instanceof HTMLInputElement) {
			if (target.type === 'checkbox') {
				if (field.field_type === 'checkbox') {
					// Multiple checkboxes - array
					const currentValues = Array.isArray(value) ? value : [];
					if (target.checked) {
						newValue = [...currentValues, target.value];
					} else {
						newValue = currentValues.filter((v: string) => v !== target.value);
					}
				} else {
					// Single checkbox
					newValue = target.checked;
				}
			} else if (target.type === 'file') {
				newValue = target.files?.[0] || null;
			} else if (target.type === 'number' || target.type === 'range') {
				newValue = target.value ? parseFloat(target.value) : null;
			} else {
				newValue = target.value;
			}
		} else {
			newValue = target.value;
		}

		dispatch('change', newValue);
	}

	function handleCheckboxChange(optionValue: string, checked: boolean) {
		const currentValues = Array.isArray(value) ? value : [];
		const newValue = checked
			? [...currentValues, optionValue]
			: currentValues.filter((v: string) => v !== optionValue);

		dispatch('change', newValue);
	}

	function isChecked(optionValue: string): boolean {
		return Array.isArray(value) && value.includes(optionValue);
	}

	function getInputClasses(): string {
		const baseClasses = 'form-input';
		const errorClass = error && error.length > 0 ? 'input-error' : '';
		return `${baseClasses} ${errorClass}`.trim();
	}
</script>

<div class="form-field">
	{#if field.field_type === 'heading'}
		<h3 class="field-heading">{field.label}</h3>
	{:else if field.field_type === 'divider'}
		<hr class="field-divider" />
	{:else if field.field_type === 'html'}
		<div class="field-html">
			{@html field.placeholder || ''}
		</div>
	{:else}
		<label class="field-label" for={`field-${field.name}`}>
			{field.label}
			{#if field.required}
				<span class="required">*</span>
			{/if}
		</label>

		{#if field.help_text}
			<div class="field-help">{field.help_text}</div>
		{/if}

		<!-- Text Input -->
		{#if field.field_type === 'text'}
			<input
				type="text"
				id={`field-${field.name}`}
				name={field.name}
				placeholder={field.placeholder || ''}
				{value}
				required={field.required}
				class={getInputClasses()}
				on:input={handleChange}
				{...field.attributes || {}}
			/>

			<!-- Email -->
		{:else if field.field_type === 'email'}
			<input
				type="email"
				id={`field-${field.name}`}
				name={field.name}
				placeholder={field.placeholder || ''}
				{value}
				required={field.required}
				class={getInputClasses()}
				on:input={handleChange}
				{...field.attributes || {}}
			/>

			<!-- Number -->
		{:else if field.field_type === 'number'}
			<input
				type="number"
				id={`field-${field.name}`}
				name={field.name}
				placeholder={field.placeholder || ''}
				value={value || ''}
				required={field.required}
				class={getInputClasses()}
				on:input={handleChange}
				min={field.validation?.min}
				max={field.validation?.max}
				step={typeof field.validation?.step === 'number' ||
				typeof field.validation?.step === 'string'
					? field.validation.step
					: 'any'}
				{...field.attributes || {}}
			/>

			<!-- Phone -->
		{:else if field.field_type === 'tel'}
			<input
				type="tel"
				id={`field-${field.name}`}
				name={field.name}
				placeholder={field.placeholder || ''}
				{value}
				required={field.required}
				class={getInputClasses()}
				on:input={handleChange}
				{...field.attributes || {}}
			/>

			<!-- URL -->
		{:else if field.field_type === 'url'}
			<input
				type="url"
				id={`field-${field.name}`}
				name={field.name}
				placeholder={field.placeholder || ''}
				{value}
				required={field.required}
				class={getInputClasses()}
				on:input={handleChange}
				{...field.attributes || {}}
			/>

			<!-- Textarea -->
		{:else if field.field_type === 'textarea'}
			<textarea
				id={`field-${field.name}`}
				name={field.name}
				placeholder={field.placeholder || ''}
				{value}
				required={field.required}
				class={getInputClasses()}
				on:input={handleChange}
				rows={typeof field.attributes?.rows === 'number'
					? field.attributes.rows
					: typeof field.attributes?.rows === 'string'
						? parseInt(field.attributes.rows, 10)
						: 5}
				minlength={field.validation?.min_length}
				maxlength={field.validation?.max_length}
				{...field.attributes || {}}
			/>

			<!-- Select Dropdown -->
		{:else if field.field_type === 'select'}
			<select
				id={`field-${field.name}`}
				name={field.name}
				{value}
				required={field.required}
				class={getInputClasses()}
				on:change={handleChange}
				{...field.attributes || {}}
			>
				<option value="">-- Select --</option>
				{#if field.options}
					{#each field.options as option}
						<option value={option}>{option}</option>
					{/each}
				{/if}
			</select>

			<!-- Radio Buttons -->
		{:else if field.field_type === 'radio'}
			<div class="radio-group">
				{#if field.options}
					{#each field.options as option}
						<label class="radio-label">
							<input
								type="radio"
								name={field.name}
								value={option}
								checked={value === option}
								required={field.required}
								on:change={handleChange}
								{...field.attributes || {}}
							/>
							<span>{option}</span>
						</label>
					{/each}
				{/if}
			</div>

			<!-- Checkboxes -->
		{:else if field.field_type === 'checkbox'}
			<div class="checkbox-group">
				{#if field.options}
					{#each field.options as option}
						<label class="checkbox-label">
							<input
								type="checkbox"
								value={option}
								checked={isChecked(option)}
								on:change={(e) => handleCheckboxChange(option, e.currentTarget.checked)}
								{...field.attributes || {}}
							/>
							<span>{option}</span>
						</label>
					{/each}
				{/if}
			</div>

			<!-- File Upload -->
		{:else if field.field_type === 'file'}
			<input
				type="file"
				id={`field-${field.name}`}
				name={field.name}
				required={field.required}
				class={getInputClasses()}
				on:change={handleChange}
				accept={typeof field.validation?.accept === 'string' ? field.validation.accept : undefined}
				{...field.attributes || {}}
			/>
			{#if field.validation?.max_size}
				<small class="field-help">Maximum file size: {field.validation.max_size}</small>
			{/if}

			<!-- Date -->
		{:else if field.field_type === 'date'}
			<input
				type="date"
				id={`field-${field.name}`}
				name={field.name}
				{value}
				required={field.required}
				class={getInputClasses()}
				on:input={handleChange}
				min={field.validation?.min}
				max={field.validation?.max}
				{...field.attributes || {}}
			/>

			<!-- Time -->
		{:else if field.field_type === 'time'}
			<input
				type="time"
				id={`field-${field.name}`}
				name={field.name}
				{value}
				required={field.required}
				class={getInputClasses()}
				on:input={handleChange}
				{...field.attributes || {}}
			/>

			<!-- Date & Time -->
		{:else if field.field_type === 'datetime'}
			<input
				type="datetime-local"
				id={`field-${field.name}`}
				name={field.name}
				{value}
				required={field.required}
				class={getInputClasses()}
				on:input={handleChange}
				{...field.attributes || {}}
			/>

			<!-- Range Slider -->
		{:else if field.field_type === 'range'}
			<div class="range-wrapper">
				<input
					type="range"
					id={`field-${field.name}`}
					name={field.name}
					value={value || field.validation?.min || 0}
					required={field.required}
					class="form-range"
					on:input={handleChange}
					min={field.validation?.min || 0}
					max={field.validation?.max || 100}
					step={typeof field.validation?.step === 'number' ||
					typeof field.validation?.step === 'string'
						? field.validation.step
						: 1}
					{...field.attributes || {}}
				/>
				<output class="range-value">{value || field.validation?.min || 0}</output>
			</div>

			<!-- Color Picker -->
		{:else if field.field_type === 'color'}
			<input
				type="color"
				id={`field-${field.name}`}
				name={field.name}
				{value}
				required={field.required}
				class="form-color"
				on:input={handleChange}
				{...field.attributes || {}}
			/>

			<!-- Hidden Field -->
		{:else if field.field_type === 'hidden'}
			<input type="hidden" name={field.name} {value} {...field.attributes || {}} />

			<!-- Rating (Stars) -->
		{:else if field.field_type === 'rating'}
			<div class="rating-wrapper">
				{#each Array(field.validation?.max || 5) as _, i}
					<button
						type="button"
						class="star-button"
						class:active={value > i}
						on:click={() => dispatch('change', i + 1)}
						aria-label={`Rate ${i + 1} stars`}
					>
						★
					</button>
				{/each}
			</div>

			<!-- Signature Pad -->
		{:else if field.field_type === 'signature'}
			<div class="signature-wrapper">
				<canvas id={`field-${field.name}`} class="signature-canvas" width="400" height="150"
				></canvas>
				<button type="button" class="btn-clear-signature">Clear</button>
			</div>
		{/if}

		{#if error && error.length > 0}
			<div class="field-error">
				{#each error as err}
					<p>{err}</p>
				{/each}
			</div>
		{/if}
	{/if}
</div>

<style>
	.form-field {
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
	}

	.form-input,
	.form-range,
	.form-color {
		width: 100%;
		padding: 0.625rem 0.875rem;
		border: 1px solid #d1d5db;
		border-radius: 0.375rem;
		font-size: 0.875rem;
		transition: all 0.2s;
	}

	.form-input:focus {
		outline: none;
		border-color: #2563eb;
		box-shadow: 0 0 0 3px rgba(37, 99, 235, 0.1);
	}

	.input-error {
		border-color: #dc2626 !important;
	}

	.input-error:focus {
		box-shadow: 0 0 0 3px rgba(220, 38, 38, 0.1) !important;
	}

	textarea.form-input {
		resize: vertical;
		min-height: 100px;
	}

	select.form-input {
		cursor: pointer;
		background-color: white;
	}

	.radio-group,
	.checkbox-group {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.radio-label,
	.checkbox-label {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		cursor: pointer;
		font-size: 0.875rem;
	}

	.radio-label input,
	.checkbox-label input {
		cursor: pointer;
	}

	.range-wrapper {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.form-range {
		flex: 1;
		padding: 0;
	}

	.range-value {
		min-width: 3rem;
		text-align: center;
		font-size: 0.875rem;
		font-weight: 500;
	}

	.rating-wrapper {
		display: flex;
		gap: 0.25rem;
	}

	.star-button {
		background: none;
		border: none;
		font-size: 2rem;
		color: #d1d5db;
		cursor: pointer;
		transition: color 0.2s;
		padding: 0;
	}

	.star-button:hover,
	.star-button.active {
		color: #fbbf24;
	}

	.signature-wrapper {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.signature-canvas {
		border: 1px solid #d1d5db;
		border-radius: 0.375rem;
		background-color: white;
		touch-action: none;
	}

	.btn-clear-signature {
		align-self: flex-end;
		padding: 0.5rem 1rem;
		background-color: #6b7280;
		color: white;
		border: none;
		border-radius: 0.375rem;
		font-size: 0.75rem;
		cursor: pointer;
	}

	.btn-clear-signature:hover {
		background-color: #4b5563;
	}

	.field-error {
		font-size: 0.75rem;
		color: #dc2626;
	}

	.field-error p {
		margin: 0;
	}

	.field-heading {
		font-size: 1.25rem;
		font-weight: 600;
		color: #111827;
		margin: 0.5rem 0;
	}

	.field-divider {
		border: none;
		border-top: 1px solid #e5e7eb;
		margin: 1rem 0;
	}

	.field-html {
		font-size: 0.875rem;
		color: #374151;
	}
</style>

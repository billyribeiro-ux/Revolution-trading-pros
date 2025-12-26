<script lang="ts">
	import type { FormField } from '$lib/api/forms';
	import { sanitizeFormContent } from '$lib/utils/sanitize';

	interface Props {
		field: FormField;
		value?: any;
		error?: string[];
		onchange?: (value: any) => void;
	}

	let { field, value = '', error, onchange }: Props = $props();

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

		onchange?.(newValue);
	}

	function handleCheckboxChange(optionValue: string, checked: boolean) {
		const currentValues = Array.isArray(value) ? value : [];
		const newValue = checked
			? [...currentValues, optionValue]
			: currentValues.filter((v: string) => v !== optionValue);

		onchange?.(newValue);
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
			{@html sanitizeFormContent(field.placeholder || '')}
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
				oninput={handleChange}
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
				oninput={handleChange}
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
				oninput={handleChange}
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
				oninput={handleChange}
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
				oninput={handleChange}
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
				oninput={handleChange}
				rows={typeof field.attributes?.rows === 'number'
					? field.attributes.rows
					: typeof field.attributes?.rows === 'string'
						? parseInt(field.attributes.rows, 10)
						: 5}
				minlength={field.validation?.min_length}
				maxlength={field.validation?.max_length}
				{...field.attributes || {}}
			></textarea>

			<!-- Select Dropdown -->
		{:else if field.field_type === 'select'}
			<select
				id={`field-${field.name}`}
				name={field.name}
				{value}
				required={field.required}
				class={getInputClasses()}
				onchange={handleChange}
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
								onchange={handleChange}
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
						{@const optionValue = typeof option === 'string' ? option : option.value}
						{@const optionLabel = typeof option === 'string' ? option : option.label}
						<label class="checkbox-label">
							<input
								type="checkbox"
								value={optionValue}
								checked={isChecked(optionValue)}
								onchange={(e: Event) => handleCheckboxChange(optionValue, (e.currentTarget as HTMLInputElement).checked)}
								{...field.attributes || {}}
							/>
							<span>{optionLabel}</span>
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
				onchange={handleChange}
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
				oninput={handleChange}
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
				oninput={handleChange}
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
				oninput={handleChange}
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
					oninput={handleChange}
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
				oninput={handleChange}
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
						onclick={() => onchange?.(i + 1)}
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

			<!-- Newsletter Subscribe Checkbox -->
		{:else if field.field_type === 'newsletter_subscribe'}
			<div class="newsletter-subscribe-wrapper">
				<label class="newsletter-checkbox-label">
					<input
						type="checkbox"
						id={`field-${field.name}`}
						name={field.name}
						checked={value === true || value === '1' || value === 'yes' || value === 'on'}
						onchange={(e: Event) => onchange?.((e.currentTarget as HTMLInputElement).checked)}
						{...field.attributes || {}}
					/>
					<span class="newsletter-checkbox-text">
						{field.options?.checkbox_label || 'Yes, I want to receive newsletters'}
					</span>
				</label>
				{#if field.options?.show_privacy_link !== false}
					<a
						href={field.options?.privacy_url || '/privacy-policy'}
						target="_blank"
						rel="noopener noreferrer"
						class="privacy-link"
					>
						Privacy Policy
					</a>
				{/if}
			</div>

			<!-- Newsletter Categories Multi-Select -->
		{:else if field.field_type === 'newsletter_categories'}
			<div class="newsletter-categories-wrapper">
				{#if field.options && Array.isArray(field.options)}
					{#each field.options as option}
						{@const optionValue = typeof option === 'string' ? option : option.value}
						{@const optionLabel = typeof option === 'string' ? option : option.label}
						{@const optionDescription = typeof option === 'object' ? option.description : ''}
						{@const optionIcon = typeof option === 'object' ? option.icon : null}
						{@const optionColor = typeof option === 'object' ? option.color : null}
						<label class="category-card" style={optionColor ? `border-left-color: ${optionColor}` : ''}>
							<input
								type="checkbox"
								value={optionValue}
								checked={isChecked(optionValue)}
								onchange={(e: Event) => handleCheckboxChange(optionValue, (e.currentTarget as HTMLInputElement).checked)}
							/>
							<div class="category-content">
								{#if optionIcon}
									<span class="category-icon">{optionIcon}</span>
								{/if}
								<div class="category-text">
									<span class="category-label">{optionLabel}</span>
									{#if optionDescription}
										<span class="category-description">{optionDescription}</span>
									{/if}
								</div>
							</div>
						</label>
					{/each}
				{/if}
				{#if field.attributes?.min_selections || field.attributes?.max_selections}
					<small class="category-hint">
						{#if field.attributes.min_selections && field.attributes.max_selections}
							Select between {field.attributes.min_selections} and {field.attributes.max_selections} topics
						{:else if field.attributes.min_selections}
							Select at least {field.attributes.min_selections} topic(s)
						{:else if field.attributes.max_selections}
							Select up to {field.attributes.max_selections} topics
						{/if}
					</small>
				{/if}
			</div>

			<!-- Newsletter Frequency Select -->
		{:else if field.field_type === 'newsletter_frequency'}
			<select
				id={`field-${field.name}`}
				name={field.name}
				value={value || field.default_value || 'weekly'}
				required={field.required}
				class={getInputClasses()}
				onchange={handleChange}
				{...field.attributes || {}}
			>
				{#if field.options && Array.isArray(field.options)}
					{#each field.options as option}
						{@const optionValue = typeof option === 'string' ? option : option.value}
						{@const optionLabel = typeof option === 'string' ? option : option.label}
						<option value={optionValue}>{optionLabel}</option>
					{/each}
				{:else}
					<option value="daily">Daily Digest</option>
					<option value="weekly">Weekly Summary</option>
					<option value="biweekly">Bi-Weekly Updates</option>
					<option value="monthly">Monthly Newsletter</option>
				{/if}
			</select>
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

	/* Newsletter Subscribe Styles */
	.newsletter-subscribe-wrapper {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.newsletter-checkbox-label {
		display: flex;
		align-items: flex-start;
		gap: 0.75rem;
		cursor: pointer;
		padding: 0.75rem;
		background-color: #f9fafb;
		border: 1px solid #e5e7eb;
		border-radius: 0.5rem;
		transition: all 0.2s;
	}

	.newsletter-checkbox-label:hover {
		background-color: #f3f4f6;
		border-color: #d1d5db;
	}

	.newsletter-checkbox-label input {
		margin-top: 0.125rem;
		width: 1.125rem;
		height: 1.125rem;
		cursor: pointer;
		accent-color: #2563eb;
	}

	.newsletter-checkbox-text {
		font-size: 0.875rem;
		color: #374151;
		line-height: 1.5;
	}

	.privacy-link {
		font-size: 0.75rem;
		color: #6b7280;
		text-decoration: none;
		transition: color 0.2s;
	}

	.privacy-link:hover {
		color: #2563eb;
		text-decoration: underline;
	}

	/* Newsletter Categories Styles */
	.newsletter-categories-wrapper {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
		gap: 0.75rem;
	}

	.category-card {
		display: flex;
		align-items: flex-start;
		gap: 0.75rem;
		padding: 1rem;
		background-color: #ffffff;
		border: 1px solid #e5e7eb;
		border-left-width: 3px;
		border-left-color: #d1d5db;
		border-radius: 0.5rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.category-card:hover {
		border-color: #d1d5db;
		background-color: #f9fafb;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
	}

	.category-card:has(input:checked) {
		border-color: #2563eb;
		border-left-color: currentColor;
		background-color: #eff6ff;
	}

	.category-card input {
		margin-top: 0.125rem;
		width: 1rem;
		height: 1rem;
		cursor: pointer;
		accent-color: #2563eb;
	}

	.category-content {
		display: flex;
		align-items: flex-start;
		gap: 0.5rem;
		flex: 1;
	}

	.category-icon {
		font-size: 1.25rem;
		line-height: 1;
	}

	.category-text {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.category-label {
		font-size: 0.875rem;
		font-weight: 500;
		color: #111827;
	}

	.category-description {
		font-size: 0.75rem;
		color: #6b7280;
		line-height: 1.4;
	}

	.category-hint {
		grid-column: 1 / -1;
		font-size: 0.75rem;
		color: #6b7280;
		margin-top: 0.25rem;
	}
</style>

<script lang="ts">
	/**
	 * EnhancedCheckbox Component (FluentForms 6.1.5 - November 2025)
	 *
	 * Enhanced checkbox input field with a new "Others" option with a text box,
	 * allowing users to give their own thoughts, opinions, or suggestions.
	 */

	interface Option {
		value: string;
		label: string;
	}

	interface Props {
		name: string;
		options: Option[];
		value?: string[];
		required?: boolean;
		disabled?: boolean;
		showOther?: boolean;
		otherLabel?: string;
		otherPlaceholder?: string;
		minSelected?: number;
		maxSelected?: number;
		layout?: 'vertical' | 'horizontal' | 'grid';
		columns?: number;
		error?: string;
		onchange?: (value: string[], otherValue: string) => void;
	}

	let {
		name,
		options = [],
		value = [],
		required = false,
		disabled = false,
		showOther = true,
		otherLabel = 'Other',
		otherPlaceholder = 'Please specify...',
		minSelected = 0,
		maxSelected = 0,
		layout = 'vertical',
		columns = 2,
		error = '',
		onchange
	}: Props = $props();

	let selectedValues = $state<string[]>([]);
	let otherChecked = $state(false);
	let otherText = $state('');

	// Sync with prop value changes
	$effect(() => {
		selectedValues = value;
	});

	function handleCheckboxChange(optionValue: string, checked: boolean) {
		if (disabled) return;

		let newValues = [...selectedValues];

		if (checked) {
			// Check max limit
			if (maxSelected > 0 && newValues.length >= maxSelected) {
				return;
			}
			newValues.push(optionValue);
		} else {
			newValues = newValues.filter((v) => v !== optionValue);
		}

		selectedValues = newValues;
		notifyChange();
	}

	function handleOtherChange(checked: boolean) {
		if (disabled) return;

		if (checked && maxSelected > 0 && selectedValues.length >= maxSelected) {
			return;
		}

		otherChecked = checked;
		if (!checked) {
			otherText = '';
		}
		notifyChange();
	}

	function handleOtherTextChange(e: Event) {
		const target = e.target as HTMLInputElement;
		otherText = target.value;
		notifyChange();
	}

	function notifyChange() {
		if (onchange) {
			onchange(selectedValues, otherChecked ? otherText : '');
		}
	}

	function isAtMaxLimit(): boolean {
		return maxSelected > 0 && selectedValues.length + (otherChecked ? 1 : 0) >= maxSelected;
	}
</script>

<div class="enhanced-checkbox" class:disabled class:has-error={error}>
	<div
		class="options-container"
		class:layout-horizontal={layout === 'horizontal'}
		class:layout-grid={layout === 'grid'}
		style={layout === 'grid' ? `--columns: ${columns}` : ''}
	>
		{#each options as option}
			<label class="checkbox-option" class:disabled={disabled || (isAtMaxLimit() && !selectedValues.includes(option.value))}>
				<input
					type="checkbox"
					{name}
					value={option.value}
					checked={selectedValues.includes(option.value)}
					disabled={disabled || (isAtMaxLimit() && !selectedValues.includes(option.value))}
					onchange={(e: Event) => handleCheckboxChange(option.value, (e.target as HTMLInputElement).checked)}
				/>
				<span class="checkbox-custom">
					<svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3">
						<polyline points="20 6 9 17 4 12"></polyline>
					</svg>
				</span>
				<span class="checkbox-label">{option.label}</span>
			</label>
		{/each}

		{#if showOther}
			<label class="checkbox-option other-option" class:disabled={disabled || (isAtMaxLimit() && !otherChecked)}>
				<input
					type="checkbox"
					name="{name}_other"
					checked={otherChecked}
					disabled={disabled || (isAtMaxLimit() && !otherChecked)}
					onchange={(e: Event) => handleOtherChange((e.target as HTMLInputElement).checked)}
				/>
				<span class="checkbox-custom">
					<svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3">
						<polyline points="20 6 9 17 4 12"></polyline>
					</svg>
				</span>
				<span class="checkbox-label">{otherLabel}</span>
			</label>

			{#if otherChecked}
				<div class="other-input-container">
					<input
						type="text"
						class="other-input"
						name="{name}_other_text"
						value={otherText}
						placeholder={otherPlaceholder}
						oninput={handleOtherTextChange}
						{disabled}
						aria-label="{otherLabel} - please specify"
					/>
				</div>
			{/if}
		{/if}
	</div>

	{#if minSelected > 0 || maxSelected > 0}
		<div class="selection-hint">
			{#if minSelected > 0 && maxSelected > 0}
				Select {minSelected} to {maxSelected} options
			{:else if minSelected > 0}
				Select at least {minSelected} option{minSelected > 1 ? 's' : ''}
			{:else if maxSelected > 0}
				Select up to {maxSelected} option{maxSelected > 1 ? 's' : ''}
			{/if}
			<span class="selection-count">({selectedValues.length + (otherChecked ? 1 : 0)} selected)</span>
		</div>
	{/if}

	{#if required}
		<span class="required-note">* Required</span>
	{/if}

	{#if error}
		<div class="error-message">
			<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
				<circle cx="12" cy="12" r="10"></circle>
				<line x1="12" y1="8" x2="12" y2="12"></line>
				<line x1="12" y1="16" x2="12.01" y2="16"></line>
			</svg>
			{error}
		</div>
	{/if}
</div>

<style>
	.enhanced-checkbox {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.options-container {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.options-container.layout-horizontal {
		flex-direction: row;
		flex-wrap: wrap;
		gap: 1rem;
	}

	.options-container.layout-grid {
		display: grid;
		grid-template-columns: repeat(var(--columns, 2), 1fr);
		gap: 0.75rem;
	}

	.checkbox-option {
		display: flex;
		align-items: center;
		gap: 0.625rem;
		cursor: pointer;
		user-select: none;
	}

	.checkbox-option.disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.checkbox-option input[type='checkbox'] {
		position: absolute;
		opacity: 0;
		width: 0;
		height: 0;
	}

	.checkbox-custom {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 20px;
		height: 20px;
		border: 2px solid #d1d5db;
		border-radius: 0.25rem;
		background-color: white;
		transition: all 0.15s ease;
		flex-shrink: 0;
	}

	.checkbox-custom svg {
		opacity: 0;
		color: white;
		transition: opacity 0.15s ease;
	}

	.checkbox-option input:checked + .checkbox-custom {
		background-color: #3b82f6;
		border-color: #3b82f6;
	}

	.checkbox-option input:checked + .checkbox-custom svg {
		opacity: 1;
	}

	.checkbox-option:hover:not(.disabled) .checkbox-custom {
		border-color: #9ca3af;
	}

	.checkbox-option input:focus + .checkbox-custom {
		outline: 2px solid #93c5fd;
		outline-offset: 2px;
	}

	.checkbox-label {
		font-size: 0.9375rem;
		color: #374151;
	}

	.other-option {
		margin-top: 0.25rem;
	}

	.other-input-container {
		margin-left: 2rem;
		margin-top: 0.5rem;
	}

	.other-input {
		width: 100%;
		max-width: 300px;
		padding: 0.5rem 0.75rem;
		border: 1px solid #d1d5db;
		border-radius: 0.375rem;
		font-size: 0.875rem;
		color: #374151;
		transition: all 0.15s;
	}

	.other-input:focus {
		outline: none;
		border-color: #3b82f6;
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
	}

	.other-input::placeholder {
		color: #9ca3af;
	}

	.selection-hint {
		font-size: 0.8125rem;
		color: #6b7280;
	}

	.selection-count {
		color: #9ca3af;
	}

	.required-note {
		font-size: 0.75rem;
		color: #ef4444;
	}

	/* Disabled State */
	.disabled {
		opacity: 0.6;
	}

	/* Error State */
	.has-error .checkbox-custom {
		border-color: #fca5a5;
	}

	.has-error .other-input {
		border-color: #fca5a5;
	}

	.error-message {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		color: #dc2626;
		font-size: 0.8125rem;
	}

	/* Animations */
	.other-input-container {
		animation: slideDown 0.2s ease-out;
	}

	@keyframes slideDown {
		from {
			opacity: 0;
			transform: translateY(-5px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}
</style>

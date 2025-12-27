<script lang="ts">
	/**
	 * DynamicField Component (FluentForms Pro 6.1.8)
	 *
	 * Dynamically populates field options from various sources:
	 * - Posts (any post type)
	 * - Users (with role filtering)
	 * - Taxonomy Terms
	 * - Form Submissions (from other forms)
	 * - CSV Data
	 */

	interface DynamicOption {
		value: string;
		label: string;
		meta?: Record<string, unknown>;
	}

	interface DynamicConfig {
		source: 'post' | 'user' | 'term' | 'form_submission' | 'csv';
		// Post source config
		postType?: string;
		postStatus?: string[];
		templateValue?: string;
		templateLabel?: string;
		// User source config
		userRoles?: string[];
		// Term source config
		taxonomy?: string;
		// Form submission config
		sourceFormId?: number;
		sourceField?: string;
		// General config
		filters?: FilterConfig[];
		sortBy?: string;
		orderBy?: 'ASC' | 'DESC';
		resultLimit?: number;
		uniqueResult?: boolean;
	}

	interface FilterConfig {
		column: string;
		operator: string;
		value: string | number;
	}

	interface Props {
		name: string;
		value?: string | string[];
		label?: string;
		required?: boolean;
		disabled?: boolean;
		fieldType?: 'select' | 'multi_select' | 'radio' | 'checkbox';
		dynamicConfig: DynamicConfig;
		enableSearchable?: boolean;
		shuffleOptions?: boolean;
		placeholder?: string;
		error?: string;
		helpText?: string;
		onchange?: (value: string | string[]) => void;
		fetchEndpoint?: string;
	}

	let {
		name,
		value = '',
		label = 'Dynamic Field',
		required = false,
		disabled = false,
		fieldType = 'select',
		dynamicConfig,
		enableSearchable = false,
		shuffleOptions = false,
		placeholder = 'Select an option...',
		error = '',
		helpText = '',
		onchange,
		fetchEndpoint = '/api/forms/dynamic-field'
	}: Props = $props();

	let options = $state<DynamicOption[]>([]);
	let loading = $state(true);
	let fetchError = $state('');
	let searchQuery = $state('');
	let showDropdown = $state(false);
	let selectedValues = $state<string[]>([]);

	// Sync with prop value changes
	$effect(() => {
		selectedValues = Array.isArray(value) ? value : value ? [value] : [];
	});

	// Fetch options on mount
	$effect(() => {
		fetchOptions();
	});

	async function fetchOptions() {
		loading = true;
		fetchError = '';

		try {
			const response = await fetch(fetchEndpoint, {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ config: dynamicConfig })
			});

			if (!response.ok) {
				throw new Error('Failed to fetch options');
			}

			const data = await response.json();
			let fetchedOptions: DynamicOption[] = data.options || [];

			// Shuffle if enabled
			if (shuffleOptions) {
				fetchedOptions = shuffleArray(fetchedOptions);
			}

			options = fetchedOptions;
		} catch (err) {
			fetchError = err instanceof Error ? err.message : 'Failed to load options';
			options = [];
		} finally {
			loading = false;
		}
	}

	function shuffleArray<T>(array: T[]): T[] {
		const shuffled = [...array];
		for (let i = shuffled.length - 1; i > 0; i--) {
			const j = Math.floor(Math.random() * (i + 1));
			[shuffled[i], shuffled[j]] = [shuffled[j], shuffled[i]];
		}
		return shuffled;
	}

	function handleSelectChange(e: Event) {
		const select = e.target as HTMLSelectElement;
		if (fieldType === 'multi_select') {
			const selected = Array.from(select.selectedOptions).map((opt) => opt.value);
			selectedValues = selected;
			if (onchange) onchange(selected);
		} else {
			selectedValues = [select.value];
			if (onchange) onchange(select.value);
		}
	}

	function handleCheckboxChange(optionValue: string, checked: boolean) {
		if (checked) {
			selectedValues = [...selectedValues, optionValue];
		} else {
			selectedValues = selectedValues.filter((v) => v !== optionValue);
		}
		if (onchange) onchange(selectedValues);
	}

	function handleRadioChange(optionValue: string) {
		selectedValues = [optionValue];
		if (onchange) onchange(optionValue);
	}

	const filteredOptions = $derived(
		enableSearchable && searchQuery
			? options.filter((opt) =>
					opt.label.toLowerCase().includes(searchQuery.toLowerCase())
				)
			: options
	);

	const isMultiple = $derived(fieldType === 'multi_select' || fieldType === 'checkbox');
	const sourceLabel = $derived(
		{
			post: 'Posts',
			user: 'Users',
			term: 'Taxonomy Terms',
			form_submission: 'Form Submissions',
			csv: 'CSV Data'
		}[dynamicConfig.source] || 'Data'
	);
</script>

<div class="dynamic-field" class:disabled class:has-error={error}>
	{#if label}
		<div class="field-label">
			{label}
			{#if required}
				<span class="required">*</span>
			{/if}
		</div>
	{/if}

	{#if loading}
		<div class="loading-state">
			<svg class="spinner" viewBox="0 0 24 24">
				<circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="3" fill="none" />
			</svg>
			<span>Loading {sourceLabel}...</span>
		</div>
	{:else if fetchError}
		<div class="error-state">
			<span class="error-icon">⚠️</span>
			<span>{fetchError}</span>
			<button type="button" class="retry-btn" onclick={fetchOptions}>Retry</button>
		</div>
	{:else if fieldType === 'select' || fieldType === 'multi_select'}
		<div class="select-wrapper">
			{#if enableSearchable}
				<input
					type="text"
					class="search-input"
					placeholder="Search options..."
					value={searchQuery}
					oninput={(e: Event) => (searchQuery = (e.target as HTMLInputElement).value)}
					onfocus={() => (showDropdown = true)}
				/>
			{/if}
			<select
				{name}
				multiple={isMultiple}
				{disabled}
				onchange={handleSelectChange}
				class="select-input"
				class:searchable={enableSearchable}
			>
				{#if !isMultiple && placeholder}
					<option value="" disabled selected={!selectedValues.length}>{placeholder}</option>
				{/if}
				{#each filteredOptions as option}
					<option value={option.value} selected={selectedValues.includes(option.value)}>
						{option.label}
					</option>
				{/each}
			</select>
		</div>
	{:else if fieldType === 'radio'}
		<div class="radio-group">
			{#each filteredOptions as option}
				<label class="radio-option">
					<input
						type="radio"
						{name}
						value={option.value}
						checked={selectedValues.includes(option.value)}
						{disabled}
						onchange={() => handleRadioChange(option.value)}
					/>
					<span class="radio-label">{option.label}</span>
				</label>
			{/each}
		</div>
	{:else if fieldType === 'checkbox'}
		<div class="checkbox-group">
			{#each filteredOptions as option}
				<label class="checkbox-option">
					<input
						type="checkbox"
						name="{name}[]"
						value={option.value}
						checked={selectedValues.includes(option.value)}
						{disabled}
						onchange={(e: Event) =>
							handleCheckboxChange(option.value, (e.target as HTMLInputElement).checked)}
					/>
					<span class="checkbox-label">{option.label}</span>
				</label>
			{/each}
		</div>
	{/if}

	{#if options.length === 0 && !loading && !fetchError}
		<p class="no-options">No options available from {sourceLabel}</p>
	{/if}

	{#if helpText && !error}
		<p class="help-text">{helpText}</p>
	{/if}

	{#if error}
		<p class="error-text">{error}</p>
	{/if}

	<!-- Hidden inputs for form submission -->
	{#each selectedValues as val}
		<input type="hidden" name={isMultiple ? `${name}[]` : name} value={val} />
	{/each}
</div>

<style>
	.dynamic-field {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.field-label {
		font-size: 0.875rem;
		font-weight: 500;
		color: #374151;
	}

	.required {
		color: #ef4444;
		margin-left: 0.25rem;
	}

	.loading-state {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 1rem;
		background-color: #f9fafb;
		border: 1px solid #e5e7eb;
		border-radius: 0.5rem;
		color: #6b7280;
	}

	.spinner {
		width: 20px;
		height: 20px;
		animation: spin 1s linear infinite;
	}

	.spinner circle {
		stroke-dasharray: 60;
		stroke-dashoffset: 45;
		stroke-linecap: round;
	}

	@keyframes spin {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}

	.error-state {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem;
		background-color: #fef2f2;
		border: 1px solid #fecaca;
		border-radius: 0.5rem;
		color: #dc2626;
	}

	.retry-btn {
		margin-left: auto;
		padding: 0.25rem 0.75rem;
		font-size: 0.75rem;
		color: #dc2626;
		background-color: white;
		border: 1px solid #fecaca;
		border-radius: 0.25rem;
		cursor: pointer;
	}

	.retry-btn:hover {
		background-color: #fef2f2;
	}

	.select-wrapper {
		position: relative;
	}

	.search-input {
		width: 100%;
		padding: 0.625rem 0.875rem;
		border: 1px solid #d1d5db;
		border-bottom: none;
		border-radius: 0.5rem 0.5rem 0 0;
		font-size: 0.9375rem;
	}

	.search-input:focus {
		outline: none;
		border-color: #3b82f6;
	}

	.select-input {
		width: 100%;
		padding: 0.625rem 0.875rem;
		border: 1px solid #d1d5db;
		border-radius: 0.5rem;
		font-size: 0.9375rem;
		background-color: white;
		cursor: pointer;
	}

	.select-input.searchable {
		border-radius: 0 0 0.5rem 0.5rem;
	}

	.select-input:focus {
		outline: none;
		border-color: #3b82f6;
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
	}

	.select-input[multiple] {
		height: auto;
		min-height: 120px;
	}

	.radio-group,
	.checkbox-group {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.radio-option,
	.checkbox-option {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		cursor: pointer;
		padding: 0.5rem;
		border-radius: 0.375rem;
		transition: background-color 0.15s;
	}

	.radio-option:hover,
	.checkbox-option:hover {
		background-color: #f9fafb;
	}

	.radio-label,
	.checkbox-label {
		font-size: 0.9375rem;
		color: #374151;
	}

	.no-options {
		padding: 1rem;
		text-align: center;
		color: #6b7280;
		background-color: #f9fafb;
		border: 1px dashed #d1d5db;
		border-radius: 0.5rem;
		margin: 0;
	}

	.help-text {
		font-size: 0.75rem;
		color: #6b7280;
		margin: 0;
	}

	.error-text {
		font-size: 0.75rem;
		color: #ef4444;
		margin: 0;
	}

	.disabled {
		opacity: 0.6;
		pointer-events: none;
	}

	.has-error .select-input,
	.has-error .search-input {
		border-color: #fca5a5;
	}
</style>

<script lang="ts">
	/**
	 * ChainedSelectField Component (FluentForms Pro)
	 *
	 * Dependent dropdown fields where each selection filters the next.
	 * Example: Country → State → City
	 */

	interface SelectOption {
		value: string;
		label: string;
		parent?: string; // Parent value for filtering
	}

	interface ChainLevel {
		name: string;
		label: string;
		placeholder?: string;
		options: SelectOption[];
	}

	interface Props {
		name: string;
		levels: ChainLevel[];
		value?: Record<string, string>;
		required?: boolean;
		disabled?: boolean;
		layout?: 'vertical' | 'horizontal';
		error?: string;
		helpText?: string;
		onchange?: (values: Record<string, string>) => void;
	}

	let {
		name,
		levels = [],
		value = {},
		required = false,
		disabled = false,
		layout = 'vertical',
		error = '',
		helpText = '',
		onchange
	}: Props = $props();

	let selections = $state<Record<string, string>>({});

	// Sync with prop value changes
	$effect(() => {
		selections = { ...value };
	});

	function getFilteredOptions(levelIndex: number): SelectOption[] {
		if (levelIndex === 0) {
			return levels[0]?.options || [];
		}

		const parentLevel = levels[levelIndex - 1];
		const parentValue = selections[parentLevel?.name];

		if (!parentValue) {
			return [];
		}

		return levels[levelIndex]?.options.filter((opt) => opt.parent === parentValue) || [];
	}

	function handleChange(levelName: string, newValue: string, levelIndex: number) {
		const newSelections = { ...selections };
		newSelections[levelName] = newValue;

		// Clear all subsequent selections when a parent changes
		for (let i = levelIndex + 1; i < levels.length; i++) {
			newSelections[levels[i].name] = '';
		}

		selections = newSelections;

		if (onchange) {
			onchange(selections);
		}
	}

	function isLevelDisabled(levelIndex: number): boolean {
		if (disabled) return true;
		if (levelIndex === 0) return false;

		const parentLevel = levels[levelIndex - 1];
		return !selections[parentLevel?.name];
	}
</script>

<div
	class="chained-select-field"
	class:disabled
	class:has-error={error}
	class:horizontal={layout === 'horizontal'}
>
	<div class="selects-container" class:horizontal={layout === 'horizontal'}>
		{#each levels as level, index}
			{@const options = getFilteredOptions(index)}
			{@const isDisabled = isLevelDisabled(index)}

			<div class="select-group">
				<label for="{name}_{level.name}" class="field-label">
					{level.label}
					{#if required && index === 0}
						<span class="required">*</span>
					{/if}
				</label>

				<select
					id="{name}_{level.name}"
					name="{name}[{level.name}]"
					value={selections[level.name] || ''}
					onchange={(e: Event) =>
						handleChange(level.name, (e.target as HTMLSelectElement).value, index)}
					disabled={isDisabled}
					class="select-input"
					class:placeholder={!selections[level.name]}
				>
					<option value="">{level.placeholder || `Select ${level.label}`}</option>
					{#each options as option}
						<option value={option.value}>{option.label}</option>
					{/each}
				</select>

				{#if isDisabled && index > 0}
					<span class="disabled-hint">Select {levels[index - 1].label} first</span>
				{/if}
			</div>
		{/each}
	</div>

	{#if helpText && !error}
		<p class="help-text">{helpText}</p>
	{/if}

	{#if error}
		<p class="error-text">{error}</p>
	{/if}
</div>

<style>
	.chained-select-field {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.selects-container {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.selects-container.horizontal {
		flex-direction: row;
		flex-wrap: wrap;
	}

	.select-group {
		display: flex;
		flex-direction: column;
		gap: 0.375rem;
		flex: 1;
		min-width: 150px;
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

	.select-input {
		width: 100%;
		padding: 0.75rem;
		border: 1px solid #d1d5db;
		border-radius: 0.5rem;
		font-size: 0.9375rem;
		color: #374151;
		background-color: white;
		cursor: pointer;
		transition: all 0.15s;
	}

	.select-input:focus {
		outline: none;
		border-color: #3b82f6;
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
	}

	.select-input:disabled {
		background-color: #f3f4f6;
		cursor: not-allowed;
		color: #9ca3af;
	}

	.select-input.placeholder {
		color: #9ca3af;
	}

	.disabled-hint {
		font-size: 0.75rem;
		color: #9ca3af;
		font-style: italic;
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

	.has-error .select-input {
		border-color: #ef4444;
	}

	@media (max-width: 640px) {
		.selects-container.horizontal {
			flex-direction: column;
		}
	}
</style>

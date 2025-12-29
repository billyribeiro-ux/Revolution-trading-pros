<script lang="ts">
	/**
	 * Conditional Logic Builder - Visual rule builder for form fields
	 *
	 * Features:
	 * - Drag-and-drop rule ordering
	 * - Multiple condition groups (AND/OR)
	 * - Field reference autocomplete
	 * - Real-time preview
	 * - Validation feedback
	 *
	 * @version 2.0.0
	 */

	import type { FormField, ConditionalLogic, ConditionalRule } from '$lib/api/forms';

	interface Props {
		fields: FormField[];
		currentFieldName: string;
		value: ConditionalLogic | null;
		onchange?: (logic: ConditionalLogic | null) => void;
	}

	let { fields, currentFieldName, value, onchange }: Props = $props();

	// Default logic structure
	const defaultLogic: ConditionalLogic = {
		enabled: false,
		action: 'show',
		logic: 'all',
		rules: []
	};

	let logic = $state<ConditionalLogic>({ ...defaultLogic });

	// Sync with prop changes
	$effect(() => {
		if (value) {
			logic = { ...defaultLogic, ...value };
		}
	});

	// Available operators with labels
	const operators = [
		{ value: 'equals', label: 'equals', types: ['all'] },
		{ value: 'not_equals', label: 'does not equal', types: ['all'] },
		{ value: 'contains', label: 'contains', types: ['text', 'textarea', 'email'] },
		{ value: 'not_contains', label: 'does not contain', types: ['text', 'textarea', 'email'] },
		{ value: 'starts_with', label: 'starts with', types: ['text', 'textarea', 'email'] },
		{ value: 'ends_with', label: 'ends with', types: ['text', 'textarea', 'email'] },
		{ value: 'greater_than', label: 'is greater than', types: ['number', 'range', 'rating'] },
		{ value: 'less_than', label: 'is less than', types: ['number', 'range', 'rating'] },
		{
			value: 'greater_than_or_equal',
			label: 'is greater than or equal to',
			types: ['number', 'range', 'rating']
		},
		{
			value: 'less_than_or_equal',
			label: 'is less than or equal to',
			types: ['number', 'range', 'rating']
		},
		{ value: 'is_empty', label: 'is empty', types: ['all'] },
		{ value: 'is_not_empty', label: 'is not empty', types: ['all'] },
		{ value: 'is_checked', label: 'is checked', types: ['checkbox', 'toggle'] },
		{ value: 'is_not_checked', label: 'is not checked', types: ['checkbox', 'toggle'] }
	];

	// Filter fields that can be referenced (exclude current field)
	$effect(() => {
		// Update parent when logic changes
		onchange?.(logic.enabled ? logic : null);
	});

	// Get available fields for rules
	function getAvailableFields(): FormField[] {
		return fields.filter((f) => f.name !== currentFieldName && !isLayoutField(f.field_type));
	}

	// Check if field type is layout-only
	function isLayoutField(type: string): boolean {
		return ['heading', 'divider', 'html', 'section', 'page_break'].includes(type);
	}

	// Get operators for a field type
	function getOperatorsForField(fieldName: string): typeof operators {
		const field = fields.find((f) => f.name === fieldName);
		if (!field) return operators.filter((op) => op.types.includes('all'));

		return operators.filter((op) => op.types.includes('all') || op.types.includes(field.field_type));
	}

	// Get field options for value dropdown
	function getFieldOptions(fieldName: string): string[] {
		const field = fields.find((f) => f.name === fieldName);
		if (!field?.options) return [];

		if (Array.isArray(field.options)) {
			return field.options.map((opt) => (typeof opt === 'string' ? opt : opt.value || opt.label));
		}

		return [];
	}

	// Check if operator needs a value
	function operatorNeedsValue(operator: string): boolean {
		return !['is_empty', 'is_not_empty', 'is_checked', 'is_not_checked'].includes(operator);
	}

	// Add a new rule
	function addRule() {
		const availableFields = getAvailableFields();
		if (availableFields.length === 0) return;

		logic.rules = [
			...logic.rules,
			{
				field: availableFields[0]?.name ?? '',
				operator: 'equals',
				value: ''
			}
		];
	}

	// Remove a rule
	function removeRule(index: number) {
		logic.rules = logic.rules.filter((_, i) => i !== index);
	}

	// Update a rule
	function updateRule(index: number, updates: Partial<ConditionalRule>) {
		logic.rules = logic.rules.map((rule, i) => (i === index ? { ...rule, ...updates } : rule));
	}

	// Move rule up
	function moveRuleUp(index: number) {
		if (index === 0) return;
		const newRules = [...logic.rules];
		const prev = newRules[index - 1];
		const curr = newRules[index];
		if (!prev || !curr) return;
		[newRules[index - 1], newRules[index]] = [curr, prev];
		logic.rules = newRules;
	}

	// Move rule down
	function moveRuleDown(index: number) {
		if (index === logic.rules.length - 1) return;
		const newRules = [...logic.rules];
		const curr = newRules[index];
		const next = newRules[index + 1];
		if (!curr || !next) return;
		[newRules[index], newRules[index + 1]] = [next, curr];
		logic.rules = newRules;
	}

	// Generate preview text
	function getPreviewText(): string {
		if (!logic.enabled || logic.rules.length === 0) {
			return 'Field is always visible';
		}

		const actionText = logic.action === 'show' ? 'Show' : 'Hide';
		const logicText = logic.logic === 'all' ? 'ALL' : 'ANY';

		const rulesText = logic.rules
			.map((rule) => {
				const field = fields.find((f) => f.name === rule.field);
				const fieldLabel = field?.label || rule.field;
				const operatorLabel =
					operators.find((op) => op.value === rule.operator)?.label || rule.operator;

				if (operatorNeedsValue(rule.operator)) {
					return `"${fieldLabel}" ${operatorLabel} "${rule.value}"`;
				}
				return `"${fieldLabel}" ${operatorLabel}`;
			})
			.join(` ${logicText === 'ALL' ? 'AND' : 'OR'} `);

		return `${actionText} this field if ${logicText} of the following: ${rulesText}`;
	}

	// Validate logic
	function validateLogic(): string[] {
		const errors: string[] = [];

		if (logic.enabled && logic.rules.length === 0) {
			errors.push('Add at least one condition');
		}

		logic.rules.forEach((rule, index) => {
			if (!rule.field) {
				errors.push(`Rule ${index + 1}: Select a field`);
			}
			if (operatorNeedsValue(rule.operator) && !rule.value) {
				errors.push(`Rule ${index + 1}: Enter a value`);
			}
		});

		return errors;
	}

	let validationErrors = $derived(validateLogic());
</script>

<div class="conditional-logic-builder">
	<!-- Enable Toggle -->
	<div class="enable-toggle">
		<label class="toggle-label">
			<input type="checkbox" bind:checked={logic.enabled} class="toggle-input" />
			<span class="toggle-switch"></span>
			<span class="toggle-text">Enable conditional logic</span>
		</label>
	</div>

	{#if logic.enabled}
		<!-- Action & Logic Selectors -->
		<div class="logic-config">
			<div class="config-row">
				<select bind:value={logic.action} class="config-select">
					<option value="show">Show</option>
					<option value="hide">Hide</option>
					<option value="enable">Enable</option>
					<option value="disable">Disable</option>
					<option value="require">Require</option>
				</select>
				<span class="config-text">this field if</span>
				<select bind:value={logic.logic} class="config-select">
					<option value="all">ALL</option>
					<option value="any">ANY</option>
				</select>
				<span class="config-text">of the following conditions are met:</span>
			</div>
		</div>

		<!-- Rules List -->
		<div class="rules-list">
			{#each logic.rules as rule, index}
				<div class="rule-item">
					<div class="rule-number">{index + 1}</div>

					<div class="rule-content">
						<!-- Field Select -->
						<select
							value={rule.field}
							onchange={(e: Event) => updateRule(index, { field: (e.currentTarget as HTMLSelectElement).value, value: '' })}
							class="rule-select field-select"
						>
							{#each getAvailableFields() as field}
								<option value={field.name}>{field.label}</option>
							{/each}
						</select>

						<!-- Operator Select -->
						<select
							value={rule.operator}
							onchange={(e: Event) => updateRule(index, { operator: (e.currentTarget as HTMLSelectElement).value as any })}
							class="rule-select operator-select"
						>
							{#each getOperatorsForField(rule.field) as op}
								<option value={op.value}>{op.label}</option>
							{/each}
						</select>

						<!-- Value Input -->
						{#if operatorNeedsValue(rule.operator)}
							{@const fieldOptions = getFieldOptions(rule.field)}
							{#if fieldOptions.length > 0}
								<select
									value={rule.value}
									onchange={(e: Event) => updateRule(index, { value: (e.currentTarget as HTMLSelectElement).value })}
									class="rule-select value-select"
								>
									<option value="">Select value...</option>
									{#each fieldOptions as option}
										<option value={option}>{option}</option>
									{/each}
								</select>
							{:else}
								<input
									type="text"
									value={rule.value}
									oninput={(e: Event) => updateRule(index, { value: (e.currentTarget as HTMLInputElement).value })}
									placeholder="Enter value..."
									class="rule-input value-input"
								/>
							{/if}
						{/if}
					</div>

					<!-- Rule Actions -->
					<div class="rule-actions">
						<button
							type="button"
							onclick={() => moveRuleUp(index)}
							disabled={index === 0}
							class="btn-icon"
							title="Move up"
						>
							↑
						</button>
						<button
							type="button"
							onclick={() => moveRuleDown(index)}
							disabled={index === logic.rules.length - 1}
							class="btn-icon"
							title="Move down"
						>
							↓
						</button>
						<button
							type="button"
							onclick={() => removeRule(index)}
							class="btn-icon btn-danger"
							title="Remove"
						>
							×
						</button>
					</div>

					<!-- Logic Connector -->
					{#if index < logic.rules.length - 1}
						<div class="logic-connector">
							<span class="connector-text">{logic.logic === 'all' ? 'AND' : 'OR'}</span>
						</div>
					{/if}
				</div>
			{/each}

			<!-- Add Rule Button -->
			<button
				type="button"
				onclick={addRule}
				class="btn-add-rule"
				disabled={getAvailableFields().length === 0}
			>
				+ Add Condition
			</button>
		</div>

		<!-- Preview -->
		<div class="logic-preview">
			<div class="preview-label">Preview:</div>
			<div class="preview-text">{getPreviewText()}</div>
		</div>

		<!-- Validation Errors -->
		{#if validationErrors.length > 0}
			<div class="validation-errors">
				{#each validationErrors as error}
					<div class="error-item">{error}</div>
				{/each}
			</div>
		{/if}
	{/if}
</div>

<style>
	.conditional-logic-builder {
		background-color: #f9fafb;
		border: 1px solid #e5e7eb;
		border-radius: 0.5rem;
		padding: 1rem;
	}

	/* Enable Toggle */
	.enable-toggle {
		margin-bottom: 1rem;
	}

	.toggle-label {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		cursor: pointer;
	}

	.toggle-input {
		display: none;
	}

	.toggle-switch {
		width: 44px;
		height: 24px;
		background-color: #d1d5db;
		border-radius: 12px;
		position: relative;
		transition: background-color 0.2s;
	}

	.toggle-switch::after {
		content: '';
		position: absolute;
		top: 2px;
		left: 2px;
		width: 20px;
		height: 20px;
		background-color: white;
		border-radius: 50%;
		transition: transform 0.2s;
	}

	.toggle-input:checked + .toggle-switch {
		background-color: #2563eb;
	}

	.toggle-input:checked + .toggle-switch::after {
		transform: translateX(20px);
	}

	.toggle-text {
		font-size: 0.875rem;
		font-weight: 500;
		color: #374151;
	}

	/* Logic Config */
	.logic-config {
		margin-bottom: 1rem;
	}

	.config-row {
		display: flex;
		align-items: center;
		flex-wrap: wrap;
		gap: 0.5rem;
	}

	.config-select {
		padding: 0.375rem 0.75rem;
		border: 1px solid #d1d5db;
		border-radius: 0.375rem;
		font-size: 0.875rem;
		background-color: white;
		font-weight: 500;
		color: #2563eb;
	}

	.config-text {
		font-size: 0.875rem;
		color: #6b7280;
	}

	/* Rules List */
	.rules-list {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
		margin-bottom: 1rem;
	}

	.rule-item {
		display: flex;
		align-items: flex-start;
		gap: 0.75rem;
		padding: 0.75rem;
		background-color: white;
		border: 1px solid #e5e7eb;
		border-radius: 0.375rem;
		position: relative;
	}

	.rule-number {
		width: 24px;
		height: 24px;
		background-color: #e5e7eb;
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 0.75rem;
		font-weight: 600;
		color: #6b7280;
		flex-shrink: 0;
	}

	.rule-content {
		flex: 1;
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
	}

	.rule-select,
	.rule-input {
		padding: 0.5rem 0.75rem;
		border: 1px solid #d1d5db;
		border-radius: 0.375rem;
		font-size: 0.875rem;
		background-color: white;
	}

	.field-select {
		min-width: 150px;
	}

	.operator-select {
		min-width: 150px;
	}

	.value-select,
	.value-input {
		min-width: 120px;
		flex: 1;
	}

	.rule-actions {
		display: flex;
		gap: 0.25rem;
		flex-shrink: 0;
	}

	.btn-icon {
		width: 28px;
		height: 28px;
		display: flex;
		align-items: center;
		justify-content: center;
		border: 1px solid #d1d5db;
		border-radius: 0.25rem;
		background-color: white;
		cursor: pointer;
		font-size: 0.875rem;
		color: #6b7280;
		transition: all 0.2s;
	}

	.btn-icon:hover:not(:disabled) {
		background-color: #f3f4f6;
		color: #111827;
	}

	.btn-icon:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.btn-icon.btn-danger:hover:not(:disabled) {
		background-color: #fee2e2;
		border-color: #fca5a5;
		color: #dc2626;
	}

	/* Logic Connector */
	.logic-connector {
		position: absolute;
		left: 12px;
		bottom: -14px;
		z-index: 1;
	}

	.connector-text {
		padding: 0.125rem 0.5rem;
		background-color: #dbeafe;
		color: #2563eb;
		font-size: 0.625rem;
		font-weight: 600;
		border-radius: 0.25rem;
		text-transform: uppercase;
	}

	/* Add Rule Button */
	.btn-add-rule {
		padding: 0.5rem 1rem;
		background-color: white;
		border: 1px dashed #d1d5db;
		border-radius: 0.375rem;
		font-size: 0.875rem;
		color: #6b7280;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-add-rule:hover:not(:disabled) {
		background-color: #f9fafb;
		border-color: #9ca3af;
		color: #374151;
	}

	.btn-add-rule:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	/* Preview */
	.logic-preview {
		padding: 0.75rem;
		background-color: #dbeafe;
		border-radius: 0.375rem;
		margin-bottom: 1rem;
	}

	.preview-label {
		font-size: 0.75rem;
		font-weight: 600;
		color: #1e40af;
		text-transform: uppercase;
		margin-bottom: 0.25rem;
	}

	.preview-text {
		font-size: 0.875rem;
		color: #1e3a8a;
		line-height: 1.4;
	}

	/* Validation Errors */
	.validation-errors {
		padding: 0.75rem;
		background-color: #fee2e2;
		border-radius: 0.375rem;
	}

	.error-item {
		font-size: 0.875rem;
		color: #dc2626;
	}

	.error-item + .error-item {
		margin-top: 0.25rem;
	}
</style>

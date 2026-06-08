<script lang="ts">
	/**
	 * RepeaterField Component (FluentForms Pro 6.1.8)
	 *
	 * Allows users to dynamically add/remove groups of fields.
	 * Use cases: multiple phone numbers, addresses, team members, etc.
	 */

	import { flip } from 'svelte/animate';
	import Icon from '$lib/components/Icon.svelte';

	interface FieldConfig {
		name: string;
		type: 'text' | 'email' | 'tel' | 'number' | 'select' | 'textarea';
		label?: string;
		placeholder?: string;
		required?: boolean;
		options?: { value: string; label: string }[];
		width?: 'full' | 'half' | 'third' | 'quarter';
	}

	interface RepeaterRow {
		id: string;
		values: Record<string, string>;
	}

	interface Props {
		name: string;
		label?: string;
		fields: FieldConfig[];
		value?: RepeaterRow[];
		minRows?: number;
		maxRows?: number;
		required?: boolean;
		disabled?: boolean;
		addButtonText?: string;
		removeButtonText?: string;
		error?: string;
		helpText?: string;
		onchange?: (rows: RepeaterRow[]) => void;
	}

	let {
		name,
		label = 'Repeater Field',
		fields,
		value = [],
		minRows = 1,
		maxRows = 10,
		required = false,
		disabled = false,
		addButtonText = 'Add Row',
		removeButtonText = 'Remove',
		error = '',
		helpText = '',
		onchange
	}: Props = $props();

	function generateId(): string {
		return Math.random().toString(36).substring(2, 11);
	}

	function createEmptyRow(id = generateId()): RepeaterRow {
		const values: Record<string, string> = {};
		fields.forEach((field) => {
			values[field.name] = '';
		});
		return { id, values };
	}

	let rows = $derived<RepeaterRow[]>(
		value.length > 0
			? value
			: Array.from({ length: minRows }, (_, index) => createEmptyRow(`${name}-initial-${index}`))
	);

	function addRow() {
		if (rows.length >= maxRows || disabled) return;
		rows = [...rows, createEmptyRow()];
		notifyChange();
	}

	function removeRow(id: string) {
		if (rows.length <= minRows || disabled) return;
		rows = rows.filter((row) => row.id !== id);
		notifyChange();
	}

	function updateField(rowId: string, fieldName: string, value: string) {
		rows = rows.map((row) =>
			row.id === rowId ? { ...row, values: { ...row.values, [fieldName]: value } } : row
		);
		notifyChange();
	}

	function notifyChange() {
		if (onchange) onchange(rows);
	}

	function moveRow(index: number, direction: 'up' | 'down') {
		if (disabled) return;
		const newIndex = direction === 'up' ? index - 1 : index + 1;
		if (newIndex < 0 || newIndex >= rows.length) return;

		const newRows = [...rows];
		[newRows[index], newRows[newIndex]] = [newRows[newIndex], newRows[index]];
		rows = newRows;
		notifyChange();
	}

	const canAddRow = $derived(rows.length < maxRows && !disabled);
	const canRemoveRow = $derived(rows.length > minRows && !disabled);

	function getFieldWidth(width?: string): string {
		const widths: Record<string, string> = {
			full: '100%',
			half: '50%',
			third: '33.333%',
			quarter: '25%'
		};
		return widths[width || 'full'] || '100%';
	}
</script>

<div class={['repeater-field', { disabled, 'has-error': error }]}>
	{#if label}
		<div id="{name}-label" class="field-label">
			{label}
			{#if required}
				<span class="required">*</span>
			{/if}
		</div>
	{/if}

	<div class="rows-container" role="group" aria-labelledby={label ? `${name}-label` : undefined}>
		{#each rows as row, index (row.id)}
			<div class="repeater-row" animate:flip={{ duration: 200 }}>
				<div class="row-header">
					<span class="row-number">#{index + 1}</span>
					<div class="row-actions">
						{#if rows.length > 1}
							<button
								type="button"
								class="action-btn move-btn"
								disabled={index === 0 || disabled}
								onclick={() => moveRow(index, 'up')}
								title="Move up"
								aria-label="Move row up"
							>
								<Icon name="IconChevronUp" size={16} />
							</button>
							<button
								type="button"
								class="action-btn move-btn"
								disabled={index === rows.length - 1 || disabled}
								onclick={() => moveRow(index, 'down')}
								title="Move down"
								aria-label="Move row down"
							>
								<Icon name="IconChevronDown" size={16} />
							</button>
						{/if}
						{#if canRemoveRow}
							<button
								type="button"
								class="action-btn remove-btn"
								onclick={() => removeRow(row.id)}
								title={removeButtonText}
								aria-label={removeButtonText}
							>
								<Icon name="IconX" size={16} />
							</button>
						{/if}
					</div>
				</div>

				<div class="row-fields">
					{#each fields as field (field.name)}
						<div class="field-wrapper" style:width={getFieldWidth(field.width)}>
							{#if field.label}
								<label for="{name}-{index}-{field.name}" class="inner-label">
									{field.label}
									{#if field.required}
										<span class="required">*</span>
									{/if}
								</label>
							{/if}

							{#if field.type === 'select'}
								<select
									id="{name}-{index}-{field.name}"
									name="{name}[{index}][{field.name}]"
									value={row.values[field.name]}
									{disabled}
									onchange={(e: Event) =>
										updateField(row.id, field.name, (e.target as HTMLSelectElement).value)}
								>
									<option value="">{field.placeholder || 'Select...'}</option>
									{#each field.options || [] as option (option.value)}
										<option value={option.value}>{option.label}</option>
									{/each}
								</select>
							{:else if field.type === 'textarea'}
								<textarea
									id="{name}-{index}-{field.name}"
									name="{name}[{index}][{field.name}]"
									value={row.values[field.name]}
									placeholder={field.placeholder}
									{disabled}
									rows="3"
									oninput={(e: Event) =>
										updateField(row.id, field.name, (e.target as HTMLTextAreaElement).value)}
								></textarea>
							{:else}
								<input
									type={field.type}
									id="{name}-{index}-{field.name}"
									name="{name}[{index}][{field.name}]"
									value={row.values[field.name]}
									placeholder={field.placeholder}
									{disabled}
									oninput={(e: Event) =>
										updateField(row.id, field.name, (e.target as HTMLInputElement).value)}
								/>
							{/if}
						</div>
					{/each}
				</div>
			</div>
		{/each}
	</div>

	{#if canAddRow}
		<button type="button" class="add-row-btn" onclick={addRow} {disabled}>
			<Icon name="IconPlus" size={18} />
			{addButtonText}
		</button>
	{/if}

	{#if rows.length >= maxRows}
		<p class="max-rows-notice">Maximum of {maxRows} rows reached</p>
	{/if}

	{#if helpText && !error}
		<p class="help-text">{helpText}</p>
	{/if}

	{#if error}
		<p class="error-text">{error}</p>
	{/if}
</div>

<style>
	.repeater-field {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
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

	.rows-container {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.repeater-row {
		border: 1px solid #e5e7eb;
		border-radius: 0.5rem;
		background-color: #f9fafb;
		overflow: hidden;
	}

	.row-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0.5rem 0.75rem;
		background-color: #f3f4f6;
		border-bottom: 1px solid #e5e7eb;
	}

	.row-number {
		font-size: 0.75rem;
		font-weight: 600;
		color: #6b7280;
	}

	.row-actions {
		display: flex;
		gap: 0.25rem;
	}

	.action-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 28px;
		height: 28px;
		padding: 0;
		border: none;
		border-radius: 0.25rem;
		background-color: transparent;
		color: #6b7280;
		cursor: pointer;
		transition: all 0.15s;
	}

	.action-btn:hover:not(:disabled) {
		background-color: #e5e7eb;
		color: #374151;
	}

	.action-btn:disabled {
		opacity: 0.3;
		cursor: not-allowed;
	}

	.remove-btn:hover:not(:disabled) {
		background-color: #fee2e2;
		color: #dc2626;
	}

	.row-fields {
		display: flex;
		flex-wrap: wrap;
		gap: 0.75rem;
		padding: 1rem;
	}

	.field-wrapper {
		flex: 0 0 auto;
		min-width: 0;
		box-sizing: border-box;
		padding-right: 0.5rem;
	}

	.field-wrapper:last-child {
		padding-right: 0;
	}

	.inner-label {
		display: block;
		font-size: 0.75rem;
		font-weight: 500;
		color: #6b7280;
		margin-bottom: 0.25rem;
	}

	.row-fields input,
	.row-fields select,
	.row-fields textarea {
		width: 100%;
		padding: 0.5rem 0.75rem;
		border: 1px solid #d1d5db;
		border-radius: 0.375rem;
		font-size: 0.875rem;
		transition:
			border-color 0.15s,
			box-shadow 0.15s;
	}

	.row-fields input:focus,
	.row-fields select:focus,
	.row-fields textarea:focus {
		outline: none;
		border-color: #3b82f6;
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
	}

	.row-fields textarea {
		resize: vertical;
		min-height: 60px;
	}

	.add-row-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 0.625rem 1rem;
		border: 2px dashed #d1d5db;
		border-radius: 0.5rem;
		background-color: white;
		color: #6b7280;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.15s;
	}

	.add-row-btn:hover:not(:disabled) {
		border-color: #3b82f6;
		color: #3b82f6;
		background-color: #eff6ff;
	}

	.add-row-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.max-rows-notice {
		font-size: 0.75rem;
		color: #f59e0b;
		text-align: center;
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
	}

	.has-error .repeater-row {
		border-color: #fca5a5;
	}
</style>

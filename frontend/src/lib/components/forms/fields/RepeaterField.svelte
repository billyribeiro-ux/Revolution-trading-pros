<script lang="ts">
	/**
	 * Repeater Field - Dynamic repeating field groups
	 *
	 * Features:
	 * - Add/remove field groups dynamically
	 * - Drag-and-drop reordering
	 * - Min/max item limits
	 * - Nested field rendering
	 * - Collapse/expand groups
	 *
	 * @version 2.0.0
	 */

	import type { FormField } from '$lib/api/forms';

	interface RepeaterRow {
		id: string;
		collapsed: boolean;
		values: Record<string, any>;
	}

	interface Props {
		field: FormField;
		value?: RepeaterRow[];
		error?: string[];
		onchange?: (value: RepeaterRow[]) => void;
	}

	let { field, value = [], error, onchange }: Props = $props();

	// Get repeater configuration from field attributes
	const minItems = $derived(field.attributes?.min_items ?? 0);
	const maxItems = $derived(field.attributes?.max_items ?? 10);
	const addButtonText = $derived(field.attributes?.add_button_text ?? 'Add Item');
	const itemLabel = $derived(field.attributes?.item_label ?? 'Item');
	const collapsible = $derived(field.attributes?.collapsible ?? true);
	const confirmDelete = $derived(field.attributes?.confirm_delete ?? true);

	// Sub-fields configuration - use $derived to maintain reactivity
	const subFields = $derived<Partial<FormField>[]>(field.attributes?.sub_fields ?? [
		{ name: 'title', label: 'Title', field_type: 'text', required: true },
		{ name: 'description', label: 'Description', field_type: 'textarea' }
	]);

	// Initialize rows state
	let rows = $state<RepeaterRow[]>([]);

	// Sync rows with value prop and minItems using $effect
	$effect(() => {
		if (value && value.length > 0) {
			rows = value;
		} else if (minItems > 0 && rows.length === 0) {
			rows = Array.from({ length: minItems }, () => createNewRow());
		}
	});

	// Drag state
	let draggedIndex = $state<number | null>(null);
	let dragOverIndex = $state<number | null>(null);

	// Generate unique ID
	function generateId(): string {
		return `row_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
	}

	// Create new row
	function createNewRow(): RepeaterRow {
		const initialValues: Record<string, any> = {};
		subFields.forEach((sf) => {
			if (sf.name) {
				initialValues[sf.name] = sf.default_value ?? '';
			}
		});

		return {
			id: generateId(),
			collapsed: false,
			values: initialValues
		};
	}

	// Add row
	function addRow() {
		if (rows.length >= maxItems) return;

		rows = [...rows, createNewRow()];
		emitChange();
	}

	// Remove row
	function removeRow(index: number) {
		if (rows.length <= minItems) return;

		if (confirmDelete && !confirm(`Remove ${itemLabel} ${index + 1}?`)) {
			return;
		}

		rows = rows.filter((_, i) => i !== index);
		emitChange();
	}

	// Toggle collapse
	function toggleCollapse(index: number) {
		if (!collapsible) return;

		rows = rows.map((row, i) => (i === index ? { ...row, collapsed: !row.collapsed } : row));
	}

	// Update field value
	function updateFieldValue(rowIndex: number, fieldName: string, newValue: any) {
		rows = rows.map((row, i) =>
			i === rowIndex ? { ...row, values: { ...row.values, [fieldName]: newValue } } : row
		);
		emitChange();
	}

	// Drag handlers
	function handleDragStart(index: number) {
		draggedIndex = index;
	}

	function handleDragOver(e: DragEvent, index: number) {
		e.preventDefault();
		dragOverIndex = index;
	}

	function handleDragEnd() {
		if (draggedIndex !== null && dragOverIndex !== null && draggedIndex !== dragOverIndex) {
			const newRows = [...rows];
			const [draggedRow] = newRows.splice(draggedIndex, 1);
			newRows.splice(dragOverIndex, 0, draggedRow);
			rows = newRows;
			emitChange();
		}

		draggedIndex = null;
		dragOverIndex = null;
	}

	// Emit change
	function emitChange() {
		onchange?.(rows);
	}

	// Get summary text for collapsed row
	function getRowSummary(row: RepeaterRow): string {
		const firstValue = Object.values(row.values).find((v) => v && typeof v === 'string');
		return firstValue ? String(firstValue).substring(0, 50) : 'Empty';
	}

	// Check if can add more
	let canAdd = $derived(rows.length < maxItems);
	let canRemove = $derived(rows.length > minItems);
</script>

<div class="repeater-field">
	<label class="repeater-label" for={field.name + '_repeater'}>
		{field.label}
		{#if field.required}
			<span class="required">*</span>
		{/if}
	</label>

	{#if field.help_text}
		<p class="repeater-help">{field.help_text}</p>
	{/if}

	<!-- Hidden input for form association -->
	<input type="hidden" id={field.name + '_repeater'} name={field.name} value={JSON.stringify(rows)} />

	<div class="repeater-rows">
		{#each rows as row, index (row.id)}
			<div
				class="repeater-row"
				class:collapsed={row.collapsed}
				class:dragging={draggedIndex === index}
				class:drag-over={dragOverIndex === index && draggedIndex !== index}
				draggable="true"
				ondragstart={() => handleDragStart(index)}
				ondragover={(e: DragEvent) => handleDragOver(e, index)}
				ondragend={handleDragEnd}
				role="group"
				aria-label="{itemLabel} {index + 1}"
			>
				<!-- Row Header -->
				<div class="row-header">
					<div class="row-drag-handle" title="Drag to reorder">⋮⋮</div>

					<button
						type="button"
						class="row-toggle"
						onclick={() => toggleCollapse(index)}
						disabled={!collapsible}
					>
						<span class="toggle-icon">{row.collapsed ? '▶' : '▼'}</span>
						<span class="row-title">{itemLabel} {index + 1}</span>
						{#if row.collapsed}
							<span class="row-summary">— {getRowSummary(row)}</span>
						{/if}
					</button>

					<button
						type="button"
						class="btn-remove"
						onclick={() => removeRow(index)}
						disabled={!canRemove}
						title="Remove"
					>
						×
					</button>
				</div>

				<!-- Row Content -->
				{#if !row.collapsed}
					<div class="row-content">
						{#each subFields as subField}
							{#if subField.name}
								<div class="sub-field" style="width: {subField.width ?? 100}%">
									<label class="sub-field-label" for="{field.name}_{row.id}_{subField.name}">
										{subField.label}
										{#if subField.required}
											<span class="required">*</span>
										{/if}
									</label>

									{#if subField.field_type === 'text'}
										<input
											type="text"
											id="{field.name}_{row.id}_{subField.name}"
											value={row.values[subField.name] ?? ''}
											oninput={(e: Event) => updateFieldValue(index, subField.name!, (e.currentTarget as HTMLInputElement).value)}
											placeholder={subField.placeholder ?? ''}
											class="sub-field-input"
										/>
									{:else if subField.field_type === 'textarea'}
										<textarea
											id="{field.name}_{row.id}_{subField.name}"
											value={row.values[subField.name] ?? ''}
											oninput={(e: Event) => updateFieldValue(index, subField.name!, (e.currentTarget as HTMLTextAreaElement).value)}
											placeholder={subField.placeholder ?? ''}
											class="sub-field-textarea"
											rows={3}
										></textarea>
									{:else if subField.field_type === 'number'}
										<input
											type="number"
											id="{field.name}_{row.id}_{subField.name}"
											value={row.values[subField.name] ?? ''}
											oninput={(e: Event) =>
												updateFieldValue(index, subField.name!, (e.currentTarget as HTMLInputElement).valueAsNumber)}
											placeholder={subField.placeholder ?? ''}
											class="sub-field-input"
										/>
									{:else if subField.field_type === 'email'}
										<input
											type="email"
											id="{field.name}_{row.id}_{subField.name}"
											value={row.values[subField.name] ?? ''}
											oninput={(e: Event) => updateFieldValue(index, subField.name!, (e.currentTarget as HTMLInputElement).value)}
											placeholder={subField.placeholder ?? ''}
											class="sub-field-input"
										/>
									{:else if subField.field_type === 'checkbox'}
										<label class="sub-field-checkbox">
											<input
												type="checkbox"
												checked={row.values[subField.name] ?? false}
												onchange={(e: Event) =>
													updateFieldValue(index, subField.name!, (e.currentTarget as HTMLInputElement).checked)}
											/>
											<span>{subField.placeholder ?? 'Yes'}</span>
										</label>
									{:else if subField.field_type === 'select'}
										<select
											id="{field.name}_{row.id}_{subField.name}"
											value={row.values[subField.name] ?? ''}
											onchange={(e: Event) =>
												updateFieldValue(index, subField.name!, (e.currentTarget as HTMLSelectElement).value)}
											class="sub-field-select"
										>
											<option value="">Select...</option>
											{#if Array.isArray(subField.options)}
												{#each subField.options as opt}
													<option value={typeof opt === 'string' ? opt : opt.value}>
														{typeof opt === 'string' ? opt : opt.label}
													</option>
												{/each}
											{/if}
										</select>
									{:else if subField.field_type === 'date'}
										<input
											type="date"
											id="{field.name}_{row.id}_{subField.name}"
											value={row.values[subField.name] ?? ''}
											onchange={(e: Event) =>
												updateFieldValue(index, subField.name!, (e.currentTarget as HTMLInputElement).value)}
											class="sub-field-input"
										/>
									{:else}
										<input
											type="text"
											id="{field.name}_{row.id}_{subField.name}"
											value={row.values[subField.name] ?? ''}
											oninput={(e: Event) => updateFieldValue(index, subField.name!, (e.currentTarget as HTMLInputElement).value)}
											placeholder={subField.placeholder ?? ''}
											class="sub-field-input"
										/>
									{/if}

									{#if subField.help_text}
										<span class="sub-field-help">{subField.help_text}</span>
									{/if}
								</div>
							{/if}
						{/each}
					</div>
				{/if}
			</div>
		{/each}
	</div>

	<!-- Add Button -->
	<button type="button" class="btn-add" onclick={addRow} disabled={!canAdd}>
		+ {addButtonText}
	</button>

	<!-- Item Count -->
	<div class="item-count">
		{rows.length} / {maxItems} items
		{#if minItems > 0}
			<span class="min-notice">(minimum {minItems})</span>
		{/if}
	</div>

	<!-- Errors -->
	{#if error && error.length > 0}
		<div class="field-errors">
			{#each error as err}
				<p>{err}</p>
			{/each}
		</div>
	{/if}
</div>

<style>
	.repeater-field {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.repeater-label {
		font-weight: 500;
		font-size: 0.875rem;
		color: #374151;
	}

	.required {
		color: #dc2626;
		margin-left: 0.25rem;
	}

	.repeater-help {
		font-size: 0.75rem;
		color: #6b7280;
		margin: 0;
	}

	.repeater-rows {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.repeater-row {
		border: 1px solid #e5e7eb;
		border-radius: 0.5rem;
		background-color: white;
		transition: all 0.2s;
	}

	.repeater-row.dragging {
		opacity: 0.5;
		border-style: dashed;
	}

	.repeater-row.drag-over {
		border-color: #2563eb;
		background-color: #eff6ff;
	}

	.row-header {
		display: flex;
		align-items: center;
		padding: 0.75rem;
		background-color: #f9fafb;
		border-bottom: 1px solid #e5e7eb;
		border-radius: 0.5rem 0.5rem 0 0;
	}

	.repeater-row.collapsed .row-header {
		border-bottom: none;
		border-radius: 0.5rem;
	}

	.row-drag-handle {
		cursor: grab;
		padding: 0.25rem;
		color: #9ca3af;
		user-select: none;
	}

	.row-drag-handle:active {
		cursor: grabbing;
	}

	.row-toggle {
		flex: 1;
		display: flex;
		align-items: center;
		gap: 0.5rem;
		background: none;
		border: none;
		text-align: left;
		cursor: pointer;
		font-size: 0.875rem;
		color: #374151;
		padding: 0.25rem;
	}

	.row-toggle:disabled {
		cursor: default;
	}

	.toggle-icon {
		font-size: 0.625rem;
		color: #9ca3af;
	}

	.row-title {
		font-weight: 500;
	}

	.row-summary {
		color: #9ca3af;
		font-size: 0.75rem;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.btn-remove {
		width: 28px;
		height: 28px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: none;
		border: 1px solid transparent;
		border-radius: 0.25rem;
		cursor: pointer;
		font-size: 1.25rem;
		color: #9ca3af;
		transition: all 0.2s;
	}

	.btn-remove:hover:not(:disabled) {
		background-color: #fee2e2;
		border-color: #fca5a5;
		color: #dc2626;
	}

	.btn-remove:disabled {
		opacity: 0.3;
		cursor: not-allowed;
	}

	.row-content {
		display: flex;
		flex-wrap: wrap;
		gap: 1rem;
		padding: 1rem;
	}

	.sub-field {
		display: flex;
		flex-direction: column;
		gap: 0.375rem;
		min-width: 0;
	}

	.sub-field-label {
		font-size: 0.75rem;
		font-weight: 500;
		color: #6b7280;
	}

	.sub-field-input,
	.sub-field-textarea,
	.sub-field-select {
		padding: 0.5rem 0.75rem;
		border: 1px solid #d1d5db;
		border-radius: 0.375rem;
		font-size: 0.875rem;
		transition: border-color 0.2s;
	}

	.sub-field-input:focus,
	.sub-field-textarea:focus,
	.sub-field-select:focus {
		outline: none;
		border-color: #2563eb;
		box-shadow: 0 0 0 3px rgba(37, 99, 235, 0.1);
	}

	.sub-field-textarea {
		resize: vertical;
	}

	.sub-field-checkbox {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		cursor: pointer;
		font-size: 0.875rem;
		color: #374151;
	}

	.sub-field-help {
		font-size: 0.75rem;
		color: #9ca3af;
	}

	.btn-add {
		padding: 0.75rem 1.5rem;
		background-color: white;
		border: 1px dashed #d1d5db;
		border-radius: 0.5rem;
		font-size: 0.875rem;
		color: #6b7280;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-add:hover:not(:disabled) {
		background-color: #f9fafb;
		border-color: #2563eb;
		color: #2563eb;
	}

	.btn-add:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.item-count {
		font-size: 0.75rem;
		color: #9ca3af;
		text-align: center;
	}

	.min-notice {
		color: #f59e0b;
	}

	.field-errors {
		padding: 0.5rem 0.75rem;
		background-color: #fee2e2;
		border-radius: 0.375rem;
	}

	.field-errors p {
		margin: 0;
		font-size: 0.75rem;
		color: #dc2626;
	}
</style>

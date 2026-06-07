<script lang="ts">
	import { flip } from 'svelte/animate';
	import Icon from '$lib/components/Icon.svelte';
	import { slide } from 'svelte/transition';
	import type { JsonValue } from '$lib/api/_types';

	// Each repeater row holds the form values for one configured group: keys
	// are `FieldConfig.name`, values are the string/number/boolean payloads
	// produced by the inputs below. JSON-shaped because the row round-trips
	// through PostgreSQL JSONB columns on submit.
	type RepeaterRow = Record<string, JsonValue | undefined>;
	type RepeaterItem = { id: string; data: RepeaterRow; collapsed: boolean };

	/**
	 * RepeaterField - Dynamic Repeating Field Groups
	 *
	 * Matches FluentForm Pro repeater features:
	 * - Add/remove field groups dynamically
	 * - Drag-and-drop reordering
	 * - Min/max limits
	 * - Customizable field templates
	 * - Collapse/expand groups
	 */

	interface FieldConfig {
		name: string;
		label: string;
		type: 'text' | 'email' | 'number' | 'select' | 'textarea' | 'date';
		placeholder?: string;
		required?: boolean;
		options?: { value: string; label: string }[];
		width?: number; // percentage
	}

	interface Props {
		id: string;
		label: string;
		description?: string;
		fields: FieldConfig[];
		minItems?: number;
		maxItems?: number;
		addButtonText?: string;
		removeButtonText?: string;
		collapsible?: boolean;
		itemLabel?: string;
		required?: boolean;
		value?: RepeaterRow[];
		error?: string[];
		onchange?: (value: RepeaterRow[]) => void;
	}

	let {
		id,
		label,
		description,
		fields,
		minItems = 0,
		maxItems = 10,
		addButtonText = 'Add Item',
		removeButtonText = 'Remove',
		collapsible = true,
		itemLabel = 'Item',
		required = false,
		value = $bindable([]),
		error,
		onchange
	}: Props = $props();

	let items: RepeaterItem[] = $state(createInitialItems());
	let draggedIndex: number | null = $state(null);
	let dragOverIndex: number | null = $state(null);

	function createInitialItems(): RepeaterItem[] {
		if (value?.length) {
			return value.map((data, index) => ({
				id: `item-${index}-${Date.now()}`,
				data: { ...data },
				collapsed: false
			}));
		}

		return Array.from({ length: Math.min(minItems, maxItems) }, () => ({
			id: generateId(),
			data: createEmptyData(),
			collapsed: false
		}));
	}

	// Generate unique ID
	function generateId(): string {
		return `item-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
	}

	// Create empty item data
	function createEmptyData(): RepeaterRow {
		const data: RepeaterRow = {};
		fields.forEach((field) => {
			data[field.name] = '';
		});
		return data;
	}

	// Add new item
	function addItem(): void {
		if (items.length >= maxItems) return;

		items = [
			...items,
			{
				id: generateId(),
				data: createEmptyData(),
				collapsed: false
			}
		];

		updateValue();
	}

	// Remove item
	function removeItem(index: number): void {
		if (items.length <= minItems) return;

		items = items.filter((_, i) => i !== index);
		updateValue();
	}

	// Duplicate item
	function duplicateItem(index: number): void {
		if (items.length >= maxItems) return;

		const sourceItem = items[index];
		if (!sourceItem) return;
		const newItem = {
			id: generateId(),
			data: { ...sourceItem.data },
			collapsed: false
		};

		items = [...items.slice(0, index + 1), newItem, ...items.slice(index + 1)];
		updateValue();
	}

	// Toggle collapse
	function toggleCollapse(index: number): void {
		const item = items[index];
		if (!item) return;
		item.collapsed = !item.collapsed;
		items = [...items];
	}

	// Update field value
	function updateField(index: number, fieldName: string, fieldValue: JsonValue | undefined): void {
		const item = items[index];
		if (!item) return;
		item.data[fieldName] = fieldValue;
		items = [...items];
		updateValue();
	}

	// Update parent value
	function updateValue(): void {
		value = items.map((item) => item.data);
		onchange?.(value);
	}

	// Drag and drop handlers
	function handleDragStart(e: DragEvent, index: number): void {
		draggedIndex = index;
		if (e.dataTransfer) {
			e.dataTransfer.effectAllowed = 'move';
			e.dataTransfer.setData('text/plain', String(index));
		}
	}

	function handleDragOver(e: DragEvent, index: number): void {
		e.preventDefault();
		if (e.dataTransfer) {
			e.dataTransfer.dropEffect = 'move';
		}
		dragOverIndex = index;
	}

	function handleDragLeave(): void {
		dragOverIndex = null;
	}

	function handleDrop(e: DragEvent, dropIndex: number): void {
		e.preventDefault();

		if (draggedIndex === null || draggedIndex === dropIndex) {
			draggedIndex = null;
			dragOverIndex = null;
			return;
		}

		const newItems = [...items];
		const [draggedItem] = newItems.splice(draggedIndex, 1);
		if (!draggedItem) return;
		newItems.splice(dropIndex, 0, draggedItem);

		items = newItems;
		draggedIndex = null;
		dragOverIndex = null;
		updateValue();
	}

	function handleDragEnd(): void {
		draggedIndex = null;
		dragOverIndex = null;
	}

	// Move item up/down (keyboard alternative to drag)
	function moveItem(index: number, direction: 'up' | 'down'): void {
		const newIndex = direction === 'up' ? index - 1 : index + 1;
		if (newIndex < 0 || newIndex >= items.length) return;

		const newItems = [...items];
		const curr = newItems[index];
		const target = newItems[newIndex];
		if (!curr || !target) return;
		[newItems[index], newItems[newIndex]] = [target, curr];
		items = newItems;
		updateValue();
	}

	// Get item preview text. Note: this takes a full *item wrapper* (with the
	// `data` key) — kept for backward compat with the call site, even though
	// the wrapping is awkward.
	function getItemPreview(item: Pick<RepeaterItem, 'data'>): string {
		const firstField = fields[0];
		if (firstField && item.data?.[firstField.name]) {
			return String(item.data[firstField.name]).substring(0, 50);
		}
		return '';
	}

	// Derived
	let canAdd = $derived(items.length < maxItems);
	let canRemove = $derived(items.length > minItems);
</script>

<fieldset
	class={{
		'repeater-field': true,
		'repeater-field--error': Boolean(error?.length)
	}}
>
	<div class="repeater-field__header">
		<legend class="repeater-field__label">
			{label}
			{#if required}
				<span class="repeater-field__required">*</span>
			{/if}
		</legend>
		<span class="repeater-field__count">{items.length}/{maxItems}</span>
	</div>

	{#if description}
		<p class="repeater-field__description">{description}</p>
	{/if}

	<div class="repeater-items" role="list" aria-label="{label} items">
		{#each items as item, index (item.id)}
			<div
				class={{
					'repeater-item': true,
					'repeater-item--collapsed': item.collapsed,
					'repeater-item--dragging': draggedIndex === index,
					'repeater-item--drag-over': dragOverIndex === index
				}}
				animate:flip={{ duration: 200 }}
				transition:slide={{ duration: 200 }}
				draggable="true"
				role="listitem"
				aria-label="Item {index + 1} of {items.length}"
				ondragstart={(e: DragEvent) => handleDragStart(e, index)}
				ondragover={(e: DragEvent) => handleDragOver(e, index)}
				ondragleave={handleDragLeave}
				ondrop={(e: DragEvent) => handleDrop(e, index)}
				ondragend={handleDragEnd}
			>
				<div class="repeater-item__header">
					<button type="button" class="repeater-item__drag-handle" aria-label="Drag to reorder">
						<Icon name="IconGripVertical" size={16} />
					</button>

					<span class="repeater-item__title">
						{itemLabel}
						{index + 1}
						{#if item.collapsed && getItemPreview(item)}
							<span class="repeater-item__preview">— {getItemPreview(item)}</span>
						{/if}
					</span>

					<div class="repeater-item__actions">
						<button
							type="button"
							class="repeater-item__action"
							onclick={() => moveItem(index, 'up')}
							disabled={index === 0}
							aria-label="Move up"
						>
							<Icon name="IconChevronUp" size={16} />
						</button>

						<button
							type="button"
							class="repeater-item__action"
							onclick={() => moveItem(index, 'down')}
							disabled={index === items.length - 1}
							aria-label="Move down"
						>
							<Icon name="IconChevronDown" size={16} />
						</button>

						{#if canAdd}
							<button
								type="button"
								class="repeater-item__action"
								onclick={() => duplicateItem(index)}
								aria-label="Duplicate"
							>
								<Icon name="IconCopy" size={16} />
							</button>
						{/if}

						{#if collapsible}
							<button
								type="button"
								class="repeater-item__action"
								onclick={() => toggleCollapse(index)}
								aria-label={item.collapsed ? 'Expand' : 'Collapse'}
								style:transform={`rotate(${item.collapsed ? -90 : 0}deg)`}
							>
								<Icon name="IconChevronDown" size={16} />
							</button>
						{/if}

						{#if canRemove}
							<button
								type="button"
								class="repeater-item__action repeater-item__action--danger"
								onclick={() => removeItem(index)}
								aria-label={removeButtonText}
							>
								<Icon name="IconTrash" size={16} />
							</button>
						{/if}
					</div>
				</div>

				{#if !item.collapsed}
					<div class="repeater-item__body" transition:slide={{ duration: 200 }}>
						<div class="repeater-item__fields">
							{#each fields as field (field.name)}
								<div class="repeater-item__field" style:width={`${field.width || 100}%`}>
									<label for="{id}-{index}-{field.name}" class="repeater-item__field-label">
										{field.label}
										{#if field.required}
											<span class="repeater-field__required">*</span>
										{/if}
									</label>

									{#if field.type === 'select'}
										<select
											id="{id}-{index}-{field.name}"
											class="repeater-item__input"
											value={typeof item.data[field.name] === 'string' ||
											typeof item.data[field.name] === 'number'
												? (item.data[field.name] as string | number)
												: ''}
											onchange={(e: Event) =>
												updateField(
													index,
													field.name,
													(e.currentTarget as HTMLSelectElement).value
												)}
											required={field.required}
										>
											<option value="">Select...</option>
											{#each field.options || [] as option (option.value)}
												<option value={option.value}>{option.label}</option>
											{/each}
										</select>
									{:else if field.type === 'textarea'}
										<textarea
											id="{id}-{index}-{field.name}"
											class="repeater-item__input repeater-item__input--textarea"
											value={typeof item.data[field.name] === 'string' ||
											typeof item.data[field.name] === 'number'
												? (item.data[field.name] as string | number)
												: ''}
											placeholder={field.placeholder}
											required={field.required}
											oninput={(e: Event) =>
												updateField(
													index,
													field.name,
													(e.currentTarget as HTMLTextAreaElement).value
												)}
										></textarea>
									{:else}
										<input
											type={field.type}
											id="{id}-{index}-{field.name}"
											class="repeater-item__input"
											value={typeof item.data[field.name] === 'string' ||
											typeof item.data[field.name] === 'number'
												? (item.data[field.name] as string | number)
												: ''}
											placeholder={field.placeholder}
											required={field.required}
											oninput={(e: Event) =>
												updateField(index, field.name, (e.currentTarget as HTMLInputElement).value)}
										/>
									{/if}
								</div>
							{/each}
						</div>
					</div>
				{/if}
			</div>
		{/each}
	</div>

	{#if canAdd}
		<button type="button" class="repeater-add-btn" onclick={addItem}>
			<Icon name="IconPlus" size={16} />
			{addButtonText}
		</button>
	{/if}

	{#if error && error.length > 0}
		<div class="repeater-field__errors">
			{#each error as err (err)}
				<p class="repeater-field__error">{err}</p>
			{/each}
		</div>
	{/if}
</fieldset>

<style>
	/* Reset fieldset default styles and add spacing */
	.repeater-field {
		border: none;
		padding: 0;
		margin: 0 0 1.5rem 0;
	}

	.repeater-field__header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 0.5rem;
	}

	.repeater-field__label {
		font-weight: 500;
		font-size: 1rem;
		color: #1a1a1a;
	}

	.repeater-field__required {
		color: #dc2626;
		margin-left: 0.25rem;
	}

	.repeater-field__count {
		font-size: 0.75rem;
		color: #6b7280;
		background: #f3f4f6;
		padding: 0.125rem 0.5rem;
		border-radius: 0.25rem;
	}

	.repeater-field__description {
		font-size: 0.875rem;
		color: #6b7280;
		margin: 0 0 1rem;
	}

	.repeater-items {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.repeater-item {
		border: 1px solid #e5e7eb;
		border-radius: 0.75rem;
		background: white;
		overflow: hidden;
		transition: all 0.2s;
	}

	.repeater-item--dragging {
		opacity: 0.5;
		transform: scale(0.98);
	}

	.repeater-item--drag-over {
		border-color: #2563eb;
		box-shadow: 0 0 0 2px rgba(37, 99, 235, 0.2);
	}

	.repeater-item__header {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1rem;
		background: #f9fafb;
		border-bottom: 1px solid #e5e7eb;
	}

	.repeater-item--collapsed .repeater-item__header {
		border-bottom: none;
	}

	.repeater-item__drag-handle {
		padding: 0.25rem;
		background: none;
		border: none;
		cursor: grab;
		color: #9ca3af;
		transition: color 0.2s;
	}

	.repeater-item__drag-handle:hover {
		color: #374151;
	}

	.repeater-item__drag-handle:active {
		cursor: grabbing;
	}

	.repeater-item__title {
		flex: 1;
		font-size: 0.875rem;
		font-weight: 500;
		color: #374151;
	}

	.repeater-item__preview {
		font-weight: 400;
		color: #9ca3af;
	}

	.repeater-item__actions {
		display: flex;
		gap: 0.25rem;
	}

	.repeater-item__action {
		padding: 0.375rem;
		background: none;
		border: none;
		cursor: pointer;
		color: #6b7280;
		border-radius: 0.375rem;
		transition: all 0.2s;
	}

	.repeater-item__action:hover:not(:disabled) {
		background: #e5e7eb;
		color: #374151;
	}

	.repeater-item__action:disabled {
		opacity: 0.3;
		cursor: not-allowed;
	}

	.repeater-item__action--danger:hover:not(:disabled) {
		background: #fee2e2;
		color: #dc2626;
	}

	.repeater-item__body {
		padding: 1rem;
	}

	.repeater-item__fields {
		display: flex;
		flex-wrap: wrap;
		gap: 1rem;
	}

	.repeater-item__field {
		min-width: 0;
	}

	.repeater-item__field-label {
		display: block;
		font-size: 0.8125rem;
		font-weight: 500;
		color: #374151;
		margin-bottom: 0.375rem;
	}

	.repeater-item__input {
		width: 100%;
		padding: 0.5rem 0.75rem;
		border: 1px solid #d1d5db;
		border-radius: 0.5rem;
		font-size: 0.875rem;
		transition:
			border-color 0.2s,
			box-shadow 0.2s;
	}

	.repeater-item__input:focus {
		outline: none;
		border-color: #2563eb;
		box-shadow: 0 0 0 3px rgba(37, 99, 235, 0.1);
	}

	.repeater-item__input--textarea {
		min-height: 80px;
		resize: vertical;
	}

	.repeater-add-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		width: 100%;
		padding: 0.75rem;
		margin-top: 0.75rem;
		border: 2px dashed #d1d5db;
		border-radius: 0.75rem;
		background: transparent;
		color: #6b7280;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.repeater-add-btn:hover {
		border-color: #2563eb;
		color: #2563eb;
		background: #f0f7ff;
	}

	.repeater-field__errors {
		margin-top: 0.5rem;
	}

	.repeater-field__error {
		font-size: 0.8125rem;
		color: #dc2626;
		margin: 0;
	}

	.repeater-field--error .repeater-item {
		border-color: #fca5a5;
	}

	@media (max-width: 639.98px) {
		.repeater-item__field {
			width: 100%;
		}
	}
</style>

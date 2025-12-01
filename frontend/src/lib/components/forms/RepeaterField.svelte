<script lang="ts">
	import { flip } from 'svelte/animate';
	import { fade, slide } from 'svelte/transition';

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
		value?: Record<string, any>[];
		error?: string[];
		onchange?: (value: Record<string, any>[]) => void;
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

	// State
	let items: { id: string; data: Record<string, any>; collapsed: boolean }[] = $state([]);
	let draggedIndex: number | null = $state(null);
	let dragOverIndex: number | null = $state(null);

	// Initialize items from value
	$effect(() => {
		if (value && value.length > 0 && items.length === 0) {
			items = value.map((data, index) => ({
				id: `item-${index}-${Date.now()}`,
				data: { ...data },
				collapsed: false
			}));
		} else if (items.length === 0 && minItems > 0) {
			// Add minimum required items
			for (let i = 0; i < minItems; i++) {
				addItem();
			}
		}
	});

	// Generate unique ID
	function generateId(): string {
		return `item-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
	}

	// Create empty item data
	function createEmptyData(): Record<string, any> {
		const data: Record<string, any> = {};
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

		const newItem = {
			id: generateId(),
			data: { ...items[index].data },
			collapsed: false
		};

		items = [...items.slice(0, index + 1), newItem, ...items.slice(index + 1)];
		updateValue();
	}

	// Toggle collapse
	function toggleCollapse(index: number): void {
		items[index].collapsed = !items[index].collapsed;
		items = [...items];
	}

	// Update field value
	function updateField(index: number, fieldName: string, fieldValue: any): void {
		items[index].data[fieldName] = fieldValue;
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
		[newItems[index], newItems[newIndex]] = [newItems[newIndex], newItems[index]];
		items = newItems;
		updateValue();
	}

	// Get item preview text
	function getItemPreview(item: Record<string, any>): string {
		const firstField = fields[0];
		if (firstField && item.data[firstField.name]) {
			return String(item.data[firstField.name]).substring(0, 50);
		}
		return '';
	}

	// Derived
	let canAdd = $derived(items.length < maxItems);
	let canRemove = $derived(items.length > minItems);
</script>

<fieldset class="repeater-field" class:repeater-field--error={error && error.length > 0}>
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
				class="repeater-item"
				class:repeater-item--collapsed={item.collapsed}
				class:repeater-item--dragging={draggedIndex === index}
				class:repeater-item--drag-over={dragOverIndex === index}
				animate:flip={{ duration: 200 }}
				transition:slide={{ duration: 200 }}
				draggable="true"
				role="listitem"
				aria-label="Item {index + 1} of {items.length}"
				ondragstart={(e) => handleDragStart(e, index)}
				ondragover={(e) => handleDragOver(e, index)}
				ondragleave={handleDragLeave}
				ondrop={(e) => handleDrop(e, index)}
				ondragend={handleDragEnd}
			>
				<div class="repeater-item__header">
					<button
						type="button"
						class="repeater-item__drag-handle"
						aria-label="Drag to reorder"
					>
						<svg
							xmlns="http://www.w3.org/2000/svg"
							width="16"
							height="16"
							viewBox="0 0 24 24"
							fill="currentColor"
						>
							<circle cx="9" cy="6" r="1.5"></circle>
							<circle cx="15" cy="6" r="1.5"></circle>
							<circle cx="9" cy="12" r="1.5"></circle>
							<circle cx="15" cy="12" r="1.5"></circle>
							<circle cx="9" cy="18" r="1.5"></circle>
							<circle cx="15" cy="18" r="1.5"></circle>
						</svg>
					</button>

					<span class="repeater-item__title">
						{itemLabel} {index + 1}
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
							<svg
								xmlns="http://www.w3.org/2000/svg"
								width="16"
								height="16"
								viewBox="0 0 24 24"
								fill="none"
								stroke="currentColor"
								stroke-width="2"
							>
								<polyline points="18 15 12 9 6 15"></polyline>
							</svg>
						</button>

						<button
							type="button"
							class="repeater-item__action"
							onclick={() => moveItem(index, 'down')}
							disabled={index === items.length - 1}
							aria-label="Move down"
						>
							<svg
								xmlns="http://www.w3.org/2000/svg"
								width="16"
								height="16"
								viewBox="0 0 24 24"
								fill="none"
								stroke="currentColor"
								stroke-width="2"
							>
								<polyline points="6 9 12 15 18 9"></polyline>
							</svg>
						</button>

						{#if canAdd}
							<button
								type="button"
								class="repeater-item__action"
								onclick={() => duplicateItem(index)}
								aria-label="Duplicate"
							>
								<svg
									xmlns="http://www.w3.org/2000/svg"
									width="16"
									height="16"
									viewBox="0 0 24 24"
									fill="none"
									stroke="currentColor"
									stroke-width="2"
								>
									<rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect>
									<path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path>
								</svg>
							</button>
						{/if}

						{#if collapsible}
							<button
								type="button"
								class="repeater-item__action"
								onclick={() => toggleCollapse(index)}
								aria-label={item.collapsed ? 'Expand' : 'Collapse'}
							>
								<svg
									xmlns="http://www.w3.org/2000/svg"
									width="16"
									height="16"
									viewBox="0 0 24 24"
									fill="none"
									stroke="currentColor"
									stroke-width="2"
									style="transform: rotate({item.collapsed ? -90 : 0}deg); transition: transform 0.2s"
								>
									<polyline points="6 9 12 15 18 9"></polyline>
								</svg>
							</button>
						{/if}

						{#if canRemove}
							<button
								type="button"
								class="repeater-item__action repeater-item__action--danger"
								onclick={() => removeItem(index)}
								aria-label={removeButtonText}
							>
								<svg
									xmlns="http://www.w3.org/2000/svg"
									width="16"
									height="16"
									viewBox="0 0 24 24"
									fill="none"
									stroke="currentColor"
									stroke-width="2"
								>
									<polyline points="3 6 5 6 21 6"></polyline>
									<path
										d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"
									></path>
								</svg>
							</button>
						{/if}
					</div>
				</div>

				{#if !item.collapsed}
					<div class="repeater-item__body" transition:slide={{ duration: 200 }}>
						<div class="repeater-item__fields">
							{#each fields as field}
								<div class="repeater-item__field" style="width: {field.width || 100}%">
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
											value={item.data[field.name]}
											onchange={(e) => updateField(index, field.name, e.currentTarget.value)}
											required={field.required}
										>
											<option value="">Select...</option>
											{#each field.options || [] as option}
												<option value={option.value}>{option.label}</option>
											{/each}
										</select>
									{:else if field.type === 'textarea'}
										<textarea
											id="{id}-{index}-{field.name}"
											class="repeater-item__input repeater-item__input--textarea"
											value={item.data[field.name]}
											placeholder={field.placeholder}
											required={field.required}
											oninput={(e) => updateField(index, field.name, e.currentTarget.value)}
										></textarea>
									{:else}
										<input
											type={field.type}
											id="{id}-{index}-{field.name}"
											class="repeater-item__input"
											value={item.data[field.name]}
											placeholder={field.placeholder}
											required={field.required}
											oninput={(e) => updateField(index, field.name, e.currentTarget.value)}
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
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="16"
				height="16"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
			>
				<line x1="12" y1="5" x2="12" y2="19"></line>
				<line x1="5" y1="12" x2="19" y2="12"></line>
			</svg>
			{addButtonText}
		</button>
	{/if}

	{#if error && error.length > 0}
		<div class="repeater-field__errors">
			{#each error as err}
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
		transition: border-color 0.2s, box-shadow 0.2s;
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

	@media (max-width: 640px) {
		.repeater-item__field {
			width: 100% !important;
		}
	}
</style>

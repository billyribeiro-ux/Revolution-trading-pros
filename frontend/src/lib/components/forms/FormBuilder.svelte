<script lang="ts">
	import type { Form, FormField } from '$lib/api/forms';
	import { getFieldTypes, createForm, updateForm } from '$lib/api/forms';
	import FieldEditor from './FieldEditor.svelte';

	interface Props {
		form?: Partial<Form> | null;
		isEditing?: boolean;
		onsave?: () => void;
		oncancel?: () => void;
	}

	let { form = null, isEditing = false, onsave, oncancel }: Props = $props();

	let formData: Partial<Form> = $state({
		title: '',
		description: '',
		settings: {
			success_message: 'Thank you for your submission!',
			submit_text: 'Submit',
			send_email: false,
			email_to: ''
		},
		styles: {},
		status: 'draft',
		fields: []
	});

	let fields: FormField[] = $state([]);

	// Sync with prop changes
	$effect(() => {
		if (form) {
			formData = {
				title: '',
				description: '',
				settings: {
					success_message: 'Thank you for your submission!',
					submit_text: 'Submit',
					send_email: false,
					email_to: '',
					...form.settings
				},
				styles: {},
				status: 'draft',
				fields: [],
				...form
			};
			fields = form.fields || [];
		}
	});
	let availableFieldTypes: { type: string; label: string; icon?: string }[] = $state([]);
	let fieldTypeMap: Record<string, string> = $state({});
	let showFieldEditor = $state(false);
	let editingField: FormField | null = $state(null);
	let editingFieldIndex = $state(-1);
	let isSaving = $state(false);
	let saveError = $state('');

	// Load field types on mount
	(async () => {
		try {
			const types = await getFieldTypes();
			// Handle both array and object formats
			if (Array.isArray(types)) {
				availableFieldTypes = types.map((item) => {
					if (typeof item === 'object' && 'type' in item && 'label' in item) {
						return { type: item.type, label: item.label, icon: item.icon };
					}
					return { type: String(item), label: String(item) };
				});
				// Also build a lookup map for display
				fieldTypeMap = availableFieldTypes.reduce(
					(acc, ft) => {
						acc[ft.type] = ft.label;
						return acc;
					},
					{} as Record<string, string>
				);
			} else if (typeof types === 'object') {
				// Convert object to array format
				availableFieldTypes = Object.entries(types).map(([type, label]) => ({
					type,
					label: String(label)
				}));
				fieldTypeMap = types as Record<string, string>;
			}
		} catch (err) {
			console.error('Failed to load field types:', err);
		}
	})();

	function handleAddField(fieldType: string) {
		const fieldTypeInfo = availableFieldTypes.find((ft) => ft.type === fieldType);
		const newField: FormField = {
			field_type: fieldType as import('$lib/api/forms').FieldType,
			label: fieldTypeInfo?.label || fieldType,
			name: `field_${Date.now()}`,
			placeholder: '',
			required: false,
			order: fields.length,
			width: 12
		};

		editingField = newField;
		editingFieldIndex = -1;
		showFieldEditor = true;
	}

	function handleEditField(index: number) {
		editingField = { ...fields[index] };
		editingFieldIndex = index;
		showFieldEditor = true;
	}

	function handleDeleteField(index: number) {
		if (confirm('Delete this field?')) {
			fields = fields.filter((_, i) => i !== index);
			updateFieldOrders();
		}
	}

	function handleMoveField(index: number, direction: 'up' | 'down') {
		const newIndex = direction === 'up' ? index - 1 : index + 1;
		if (newIndex < 0 || newIndex >= fields.length) return;

		const temp = fields[index];
		fields[index] = fields[newIndex];
		fields[newIndex] = temp;

		updateFieldOrders();
	}

	function handleSaveField(field: FormField) {

		if (editingFieldIndex === -1) {
			// Add new field
			fields = [...fields, field];
		} else {
			// Update existing field
			fields[editingFieldIndex] = field;
			fields = [...fields];
		}

		updateFieldOrders();
		showFieldEditor = false;
		editingField = null;
		editingFieldIndex = -1;
	}

	function handleCancelFieldEdit() {
		showFieldEditor = false;
		editingField = null;
		editingFieldIndex = -1;
	}

	function updateFieldOrders() {
		fields = fields.map((field, index) => ({
			...field,
			order: index
		}));
	}

	async function handleSaveForm() {
		if (!formData.title) {
			saveError = 'Please enter a form title';
			return;
		}

		isSaving = true;
		saveError = '';

		try {
			const dataToSave = {
				...formData,
				fields: fields.map(({ id, created_at, updated_at, ...field }) => field)
			};

			if (isEditing && form?.id) {
				await updateForm(form.id, dataToSave);
			} else {
				await createForm(dataToSave);
			}

			onsave?.();
		} catch (err) {
			saveError = err instanceof Error ? err.message : 'Failed to save form';
		} finally {
			isSaving = false;
		}
	}

	function handleCancel() {
		oncancel?.();
	}
</script>

<div class="form-builder">
	{#if !showFieldEditor}
		<div class="builder-header">
			<h2>{isEditing ? 'Edit Form' : 'Create New Form'}</h2>
		</div>

		<div class="form-settings">
			<div class="settings-section">
				<h3>Form Details</h3>

				<div class="form-group">
					<label for="form-title">Form Title *</label>
					<input
						id="form-title"
						type="text"
						bind:value={formData.title}
						placeholder="Enter form title"
						class="form-input"
					/>
				</div>

				<div class="form-group">
					<label for="form-description">Description</label>
					<textarea
						id="form-description"
						bind:value={formData.description}
						placeholder="Optional form description"
						class="form-input"
						rows="3"
					></textarea>
				</div>
			</div>

			<div class="settings-section">
				<h3>Form Settings</h3>

				<div class="form-group">
					<label for="success-message">Success Message</label>
					<input
						id="success-message"
						type="text"
						bind:value={formData.settings!.success_message}
						placeholder="Thank you message"
						class="form-input"
					/>
				</div>

				<div class="form-group">
					<label for="submit-text">Submit Button Text</label>
					<input
						id="submit-text"
						type="text"
						bind:value={formData.settings!.submit_text}
						placeholder="Submit"
						class="form-input"
					/>
				</div>

				<div class="form-group">
					<label for="send-email" class="checkbox-label">
						<input id="send-email" type="checkbox" bind:checked={formData.settings!.send_email} />
						<span>Send email notifications</span>
					</label>
				</div>

				{#if formData.settings?.send_email}
					<div class="form-group">
						<label for="email-to">Send To Email</label>
						<input
							id="email-to"
							type="email"
							bind:value={formData.settings!.email_to}
							placeholder="admin@example.com"
							class="form-input"
						/>
					</div>
				{/if}
			</div>
		</div>

		<div class="fields-section">
			<div class="section-header">
				<h3>Form Fields</h3>
			</div>

			{#if fields.length === 0}
				<div class="empty-fields">
					<p>No fields added yet. Add your first field to get started.</p>
				</div>
			{:else}
				<div class="fields-list">
					{#each fields as field, index (field.name)}
						<div class="field-item">
							<div class="field-info">
								<div class="field-type-badge">
									{fieldTypeMap[field.field_type] || field.field_type}
								</div>
								<div class="field-details">
									<strong>{field.label}</strong>
									<span class="field-name">{field.name}</span>
									{#if field.required}
										<span class="required-badge">Required</span>
									{/if}
								</div>
							</div>

							<div class="field-actions">
								<button
									class="btn-icon"
									onclick={() => handleMoveField(index, 'up')}
									disabled={index === 0}
									title="Move up"
								>
									↑
								</button>
								<button
									class="btn-icon"
									onclick={() => handleMoveField(index, 'down')}
									disabled={index === fields.length - 1}
									title="Move down"
								>
									↓
								</button>
								<button class="btn-icon" onclick={() => handleEditField(index)} title="Edit">
									✏️
								</button>
								<button
									class="btn-icon btn-danger"
									onclick={() => handleDeleteField(index)}
									title="Delete"
								>
									🗑️
								</button>
							</div>
						</div>
					{/each}
				</div>
			{/if}

			<div class="add-field-section">
				<h4>Add Field</h4>
				<div class="field-types-grid">
					{#each availableFieldTypes as { type, label }}
						<button class="field-type-button" onclick={() => handleAddField(type)}>
							{label}
						</button>
					{/each}
				</div>
			</div>
		</div>

		{#if saveError}
			<div class="error-message">{saveError}</div>
		{/if}

		<div class="builder-actions">
			<button class="btn btn-secondary" onclick={handleCancel}> Cancel </button>
			<button class="btn btn-primary" onclick={handleSaveForm} disabled={isSaving}>
				{isSaving ? 'Saving...' : isEditing ? 'Update Form' : 'Create Form'}
			</button>
		</div>
	{:else}
		<FieldEditor
			field={editingField}
			availableFields={fields.filter((_, i) => i !== editingFieldIndex)}
			onsave={handleSaveField}
			oncancel={handleCancelFieldEdit}
		/>
	{/if}
</div>

<style>
	.form-builder {
		max-width: 900px;
		margin: 0 auto;
		padding: 0;
	}

	.builder-header h2 {
		font-size: 1.5rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0 0 2rem 0;
	}

	.form-settings {
		display: flex;
		flex-direction: column;
		gap: 2rem;
		margin-bottom: 3rem;
	}

	.settings-section {
		background: rgba(99, 102, 241, 0.05);
		border: 1px solid rgba(99, 102, 241, 0.1);
		padding: 1.5rem;
		border-radius: 10px;
	}

	.settings-section h3 {
		font-size: 1.125rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0 0 1rem 0;
	}

	.form-group {
		margin-bottom: 1rem;
	}

	.form-group:last-child {
		margin-bottom: 0;
	}

	.form-group label {
		display: block;
		font-size: 0.875rem;
		font-weight: 500;
		color: #94a3b8;
		margin-bottom: 0.5rem;
	}

	.form-input {
		width: 100%;
		padding: 0.625rem 0.875rem;
		background: #0f172a;
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 8px;
		font-size: 0.875rem;
		color: #e2e8f0;
	}

	.form-input:focus {
		outline: none;
		border-color: #6366f1;
		box-shadow: 0 0 0 3px rgba(99, 102, 241, 0.1);
	}

	.checkbox-label {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		cursor: pointer;
		color: #94a3b8;
	}

	.checkbox-label input {
		cursor: pointer;
	}

	.fields-section {
		background: #1e293b;
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 10px;
		padding: 1.5rem;
		margin-bottom: 2rem;
	}

	.section-header {
		margin-bottom: 1.5rem;
	}

	.section-header h3 {
		font-size: 1.125rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0;
	}

	.empty-fields {
		text-align: center;
		padding: 3rem 1rem;
		color: #94a3b8;
		background: rgba(99, 102, 241, 0.05);
		border-radius: 8px;
	}

	.fields-list {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
		margin-bottom: 2rem;
	}

	.field-item {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1rem;
		background: rgba(99, 102, 241, 0.05);
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 8px;
	}

	.field-info {
		display: flex;
		align-items: center;
		gap: 1rem;
		flex: 1;
	}

	.field-type-badge {
		background: rgba(99, 102, 241, 0.2);
		color: #a5b4fc;
		padding: 0.25rem 0.75rem;
		border-radius: 9999px;
		font-size: 0.75rem;
		font-weight: 500;
		white-space: nowrap;
	}

	.field-details {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		flex-wrap: wrap;
	}

	.field-details strong {
		color: #f1f5f9;
	}

	.field-name {
		font-size: 0.75rem;
		color: #94a3b8;
		font-family: monospace;
	}

	.required-badge {
		background: rgba(239, 68, 68, 0.2);
		color: #f87171;
		padding: 0.125rem 0.5rem;
		border-radius: 9999px;
		font-size: 0.625rem;
		font-weight: 500;
	}

	.field-actions {
		display: flex;
		gap: 0.5rem;
	}

	.btn-icon {
		background: none;
		border: none;
		font-size: 1.125rem;
		cursor: pointer;
		padding: 0.25rem 0.5rem;
		opacity: 0.7;
		transition: opacity 0.2s;
	}

	.btn-icon:hover:not(:disabled) {
		opacity: 1;
	}

	.btn-icon:disabled {
		opacity: 0.3;
		cursor: not-allowed;
	}

	.btn-danger:hover {
		filter: brightness(0.9);
	}

	.add-field-section {
		margin-top: 2rem;
	}

	.add-field-section h4 {
		font-size: 0.875rem;
		font-weight: 600;
		color: #94a3b8;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		margin: 0 0 1rem 0;
	}

	.field-types-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(140px, 1fr));
		gap: 0.75rem;
	}

	.field-type-button {
		padding: 0.75rem;
		background: #0f172a;
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 8px;
		font-size: 0.75rem;
		font-weight: 500;
		color: #e2e8f0;
		cursor: pointer;
		transition: all 0.2s;
	}

	.field-type-button:hover {
		background: rgba(99, 102, 241, 0.1);
		border-color: #6366f1;
		color: #a5b4fc;
	}

	.error-message {
		background: rgba(239, 68, 68, 0.1);
		color: #f87171;
		padding: 1rem;
		border-radius: 8px;
		margin-bottom: 1.5rem;
		font-size: 0.875rem;
		border: 1px solid rgba(239, 68, 68, 0.2);
	}

	.builder-actions {
		display: flex;
		justify-content: flex-end;
		gap: 1rem;
		padding-top: 2rem;
		border-top: 1px solid rgba(99, 102, 241, 0.1);
	}

	.btn {
		padding: 0.75rem 1.5rem;
		border-radius: 8px;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
		border: none;
	}

	.btn-primary {
		background: linear-gradient(135deg, #6366f1, #8b5cf6);
		color: white;
	}

	.btn-primary:hover:not(:disabled) {
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(99, 102, 241, 0.3);
	}

	.btn-secondary {
		background: rgba(99, 102, 241, 0.1);
		color: #a5b4fc;
		border: 1px solid rgba(99, 102, 241, 0.2);
	}

	.btn-secondary:hover {
		background: rgba(99, 102, 241, 0.2);
	}

	.btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}
</style>

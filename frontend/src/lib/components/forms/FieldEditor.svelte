<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import type { FormField, ConditionalLogic, ConditionalRule } from '$lib/api/forms';

	export let field: FormField | null;
	export let availableFields: FormField[] = [];

	const dispatch = createEventDispatcher();

	let fieldData: FormField = {
		field_type: 'text',
		label: '',
		name: '',
		placeholder: '',
		help_text: '',
		default_value: '',
		options: null,
		validation: null,
		conditional_logic: null,
		attributes: null,
		required: false,
		order: 0,
		width: 100,
		...field
	};

	let showConditionalLogic = !!fieldData.conditional_logic;
	let optionsText = fieldData.options ? fieldData.options.join('\n') : '';

	const needsOptions = ['select', 'radio', 'checkbox'].includes(fieldData.field_type);
	const supportsValidation = !['heading', 'divider', 'html', 'hidden'].includes(
		fieldData.field_type
	);

	function handleSave() {
		// Parse options from textarea - convert strings to FieldOption objects
		if (needsOptions && optionsText.trim()) {
			fieldData.options = optionsText
				.split('\n')
				.map((s) => s.trim())
				.filter(Boolean)
				.map((text) => ({ label: text, value: text }));
		} else {
			fieldData.options = null;
		}

		// Clear conditional logic if not enabled
		if (!showConditionalLogic) {
			fieldData.conditional_logic = null;
		}

		// Generate name from label if empty
		if (!fieldData.name && fieldData.label) {
			fieldData.name = fieldData.label
				.toLowerCase()
				.replace(/[^a-z0-9]+/g, '_')
				.replace(/^_+|_+$/g, '');
		}

		dispatch('save', fieldData);
	}

	function handleCancel() {
		dispatch('cancel');
	}

	function addConditionalRule() {
		if (!fieldData.conditional_logic) {
			fieldData.conditional_logic = {
				enabled: true,
				action: 'show',
				logic: 'all',
				rules: []
			};
		}

		fieldData.conditional_logic.rules = [
			...fieldData.conditional_logic.rules,
			{
				field: '',
				operator: 'equals',
				value: ''
			}
		];
	}

	function removeConditionalRule(index: number) {
		if (!fieldData.conditional_logic) return;

		fieldData.conditional_logic.rules = fieldData.conditional_logic.rules.filter(
			(_, i) => i !== index
		);

		if (fieldData.conditional_logic.rules.length === 0) {
			showConditionalLogic = false;
			fieldData.conditional_logic = null;
		}
	}

	$: if (showConditionalLogic && !fieldData.conditional_logic) {
		addConditionalRule();
	}
</script>

<div class="field-editor">
	<div class="editor-header">
		<h3>Configure Field</h3>
	</div>

	<div class="editor-body">
		<!-- Basic Properties -->
		<div class="form-group">
			<label for="field-label">Field Label *</label>
			<input
				id="field-label"
				type="text"
				bind:value={fieldData.label}
				placeholder="Enter field label"
				class="form-input"
				required
			/>
		</div>

		<div class="form-group">
			<label for="field-name">Field Name *</label>
			<input
				id="field-name"
				type="text"
				bind:value={fieldData.name}
				placeholder="field_name (auto-generated from label)"
				class="form-input"
			/>
			<small class="help-text">Unique identifier for this field</small>
		</div>

		{#if fieldData.field_type !== 'heading' && fieldData.field_type !== 'divider' && fieldData.field_type !== 'html'}
			<div class="form-group">
				<label for="field-placeholder">Placeholder Text</label>
				<input
					id="field-placeholder"
					type="text"
					bind:value={fieldData.placeholder}
					placeholder="Placeholder text"
					class="form-input"
				/>
			</div>

			<div class="form-group">
				<label for="field-help">Help Text</label>
				<input
					id="field-help"
					type="text"
					bind:value={fieldData.help_text}
					placeholder="Helper text shown below the field"
					class="form-input"
				/>
			</div>

			<div class="form-row">
				<div class="form-group">
					<label class="checkbox-label">
						<input type="checkbox" bind:checked={fieldData.required} />
						<span>Required Field</span>
					</label>
				</div>

				<div class="form-group">
					<label for="field-width">Width (%)</label>
					<input
						id="field-width"
						type="number"
						bind:value={fieldData.width}
						min="25"
						max="100"
						step="25"
						class="form-input"
					/>
				</div>
			</div>
		{/if}

		<!-- Options for select/radio/checkbox -->
		{#if needsOptions}
			<div class="form-group">
				<label for="field-options">Options (one per line) *</label>
				<textarea
					id="field-options"
					bind:value={optionsText}
					placeholder="Option 1&#10;Option 2&#10;Option 3"
					class="form-input"
					rows="5"
					required
				></textarea>
			</div>
		{/if}

		<!-- Validation -->
		{#if supportsValidation}
			<div class="form-section">
				<h4>Validation</h4>

				{#if fieldData.field_type === 'textarea' || fieldData.field_type === 'text'}
					<div class="form-row">
						<div class="form-group">
							<label for="min-length">Min Length</label>
							<input
								id="min-length"
								type="number"
								bind:value={fieldData.validation.min_length}
								class="form-input"
								min="0"
							/>
						</div>

						<div class="form-group">
							<label for="max-length">Max Length</label>
							<input
								id="max-length"
								type="number"
								bind:value={fieldData.validation.max_length}
								class="form-input"
								min="1"
							/>
						</div>
					</div>
				{/if}

				{#if fieldData.field_type === 'number' || fieldData.field_type === 'range'}
					<div class="form-row">
						<div class="form-group">
							<label for="min-value">Min Value</label>
							<input
								id="min-value"
								type="number"
								bind:value={fieldData.validation.min}
								class="form-input"
							/>
						</div>

						<div class="form-group">
							<label for="max-value">Max Value</label>
							<input
								id="max-value"
								type="number"
								bind:value={fieldData.validation.max}
								class="form-input"
							/>
						</div>
					</div>
				{/if}

				{#if fieldData.field_type === 'file'}
					<div class="form-group">
						<label for="file-accept">Accepted File Types</label>
						<input
							id="file-accept"
							type="text"
							bind:value={fieldData.validation.accept}
							placeholder=".pdf,.doc,.docx"
							class="form-input"
						/>
					</div>

					<div class="form-group">
						<label for="file-size">Max File Size (MB)</label>
						<input
							id="file-size"
							type="number"
							bind:value={fieldData.validation.max_size}
							class="form-input"
							min="1"
							step="1"
						/>
					</div>
				{/if}
			</div>
		{/if}

		<!-- Conditional Logic -->
		<div class="form-section">
			<div class="section-header">
				<h4>Conditional Logic</h4>
				<label class="checkbox-label">
					<input type="checkbox" bind:checked={showConditionalLogic} />
					<span>Enable conditional logic</span>
				</label>
			</div>

			{#if showConditionalLogic && fieldData.conditional_logic}
				<div class="conditional-config">
					<div class="form-row">
						<div class="form-group">
							<label for="conditional-action">Action</label>
							<select id="conditional-action" bind:value={fieldData.conditional_logic.action} class="form-input">
								<option value="show">Show this field if</option>
								<option value="hide">Hide this field if</option>
							</select>
						</div>

						<div class="form-group">
							<label for="conditional-match">Match</label>
							<select id="conditional-match" bind:value={fieldData.conditional_logic.logic} class="form-input">
								<option value="all">All conditions are met</option>
								<option value="any">Any condition is met</option>
							</select>
						</div>
					</div>

					<div class="conditional-rules">
						{#each fieldData.conditional_logic.rules as rule, index}
							<div class="rule-row">
								<select bind:value={rule.field} class="form-input">
									<option value="">Select field...</option>
									{#each availableFields as availField}
										<option value={availField.name}>{availField.label}</option>
									{/each}
								</select>

								<select bind:value={rule.operator} class="form-input">
									<option value="equals">Equals</option>
									<option value="not_equals">Not Equals</option>
									<option value="contains">Contains</option>
									<option value="greater_than">Greater Than</option>
									<option value="less_than">Less Than</option>
									<option value="is_empty">Is Empty</option>
									<option value="is_not_empty">Is Not Empty</option>
								</select>

								{#if rule.operator !== 'is_empty' && rule.operator !== 'is_not_empty'}
									<input
										type="text"
										bind:value={rule.value}
										placeholder="Value"
										class="form-input"
									/>
								{/if}

								<button
									type="button"
									class="btn-remove"
									on:click={() => removeConditionalRule(index)}
									title="Remove rule"
								>
									✕
								</button>
							</div>
						{/each}
					</div>

					<button type="button" class="btn-add-rule" on:click={addConditionalRule}>
						+ Add Rule
					</button>
				</div>
			{/if}
		</div>
	</div>

	<div class="editor-actions">
		<button class="btn btn-secondary" on:click={handleCancel}> Cancel </button>
		<button class="btn btn-primary" on:click={handleSave}> Save Field </button>
	</div>
</div>

<style>
	.field-editor {
		max-width: 800px;
		margin: 0 auto;
	}

	.editor-header {
		margin-bottom: 2rem;
	}

	.editor-header h3 {
		font-size: 1.25rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0;
	}

	.editor-body {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}

	.form-group {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.form-group label {
		font-size: 0.875rem;
		font-weight: 500;
		color: #94a3b8;
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

	.form-row {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1rem;
	}

	.checkbox-label {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		cursor: pointer;
		font-size: 0.875rem;
		font-weight: 500;
		color: #94a3b8;
	}

	.checkbox-label input {
		cursor: pointer;
	}

	.help-text {
		font-size: 0.75rem;
		color: #94a3b8;
	}

	.form-section {
		background: rgba(99, 102, 241, 0.05);
		border: 1px solid rgba(99, 102, 241, 0.1);
		padding: 1.5rem;
		border-radius: 10px;
	}

	.form-section h4 {
		font-size: 0.875rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0 0 1rem 0;
	}

	.section-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1rem;
	}

	.section-header h4 {
		margin: 0;
	}

	.conditional-config {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.conditional-rules {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.rule-row {
		display: grid;
		grid-template-columns: 1fr 1fr 1fr auto;
		gap: 0.5rem;
		align-items: center;
	}

	.btn-remove {
		background: rgba(239, 68, 68, 0.1);
		color: #f87171;
		border: 1px solid rgba(239, 68, 68, 0.2);
		border-radius: 8px;
		width: 2rem;
		height: 2rem;
		font-size: 1rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-remove:hover {
		background: rgba(239, 68, 68, 0.2);
	}

	.btn-add-rule {
		align-self: flex-start;
		padding: 0.5rem 1rem;
		background: rgba(99, 102, 241, 0.1);
		color: #a5b4fc;
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 8px;
		font-size: 0.75rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-add-rule:hover {
		background: rgba(99, 102, 241, 0.2);
	}

	.editor-actions {
		display: flex;
		justify-content: flex-end;
		gap: 1rem;
		margin-top: 2rem;
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

	.btn-primary:hover {
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
</style>

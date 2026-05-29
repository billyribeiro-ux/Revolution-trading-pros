<script lang="ts">
	/**
	 * FieldEditor — configure a single form field's properties.
	 *
	 * Lifecycle contract (owned by `FormBuilder.svelte`):
	 *   • Parent gates this component behind `{#if showFieldEditor}` and toggles
	 *     it false on save/cancel, so a fresh editor mounts for each session.
	 *   • Parent passes `field` as a shallow clone of the row it wants to edit
	 *     (`editingField = { ...fields[i] }`) — this component owns its own
	 *     deep-cloned working copy from there and never mutates the prop.
	 *   • On save we hand the normalized `FormField` back through `onsave`;
	 *     on cancel the parent simply discards `editingField`.
	 *
	 * No `$effect`s — `fieldData` is initialised once at mount via $state,
	 * `optionsText` and the conditional-logic checkbox are wired through
	 * function bindings (`bind:value={get, set}`), and all mutations live in
	 * event handlers rather than reactive side effects.
	 */
	import { untrack } from 'svelte';
	import type {
		FormField,
		FieldValidation,
		ConditionalLogic,
		ConditionalRule
	} from '$lib/api/forms';

	interface Props {
		/** The field to edit. `null`/`undefined` boots an empty `text` field. */
		field?: FormField | null;
		/** Other fields in the form, used as targets for conditional-logic rules. */
		availableFields?: FormField[];
		/** Commit handler. Receives the normalized working copy. */
		onsave?: (data: FormField) => void;
		oncancel?: () => void;
	}

	let { field, availableFields = [], onsave, oncancel }: Props = $props();

	// ─────────────────────────────────────────────────────────────────────────
	// Working copy
	// ─────────────────────────────────────────────────────────────────────────

	/**
	 * Build a fully-defaulted, deep-cloned working copy from the parent's field.
	 *
	 * Deep-cloning matters: the parent only `{...field}`-clones the top level,
	 * so without this the inner `validation`/`conditional_logic`/`attributes`
	 * objects would still reference the original — and `addConditionalRule`
	 * would silently mutate the row the user is supposedly only "previewing"
	 * (Cancel becomes a no-op).
	 */
	function buildWorkingCopy(source: FormField | null | undefined): FormField {
		// `structuredClone` reads through the parent's $state proxy, producing a
		// plain object we can safely mutate without touching the upstream row.
		const base: FormField = source
			? (structuredClone(source) as FormField)
			: {
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
					width: 12
				};

		// Normalise sparse fields so the template's bindings (which all expect
		// strings, not `undefined`) don't have to defend themselves.
		base.field_type = base.field_type ?? 'text';
		base.label = base.label ?? '';
		base.name = base.name ?? '';
		base.placeholder = base.placeholder ?? '';
		base.help_text = base.help_text ?? '';
		base.default_value = base.default_value ?? '';
		base.options = base.options ?? null;
		base.conditional_logic = base.conditional_logic ?? null;
		base.attributes = base.attributes ?? null;
		base.required = base.required ?? false;
		base.order = base.order ?? 0;
		base.width = base.width ?? 12;

		// `validation` stays an object so `bind:value={fieldData.validation.*}`
		// is always a valid member expression. We strip empty keys on save.
		base.validation = base.validation ?? {};

		return base;
	}

	// Initialise once at mount. `untrack` makes the intent explicit: the
	// parent's lifecycle (`{#if showFieldEditor}` remount) is what swaps
	// fields, not in-place prop updates, so we deliberately don't re-read
	// `field` reactively.
	let fieldData: FormField = $state(untrack(() => buildWorkingCopy(field)));

	// ─────────────────────────────────────────────────────────────────────────
	// Derived view of the field shape (drives which sections render)
	// ─────────────────────────────────────────────────────────────────────────

	const needsOptions = $derived(['select', 'radio', 'checkbox'].includes(fieldData.field_type));
	const supportsValidation = $derived(
		!['heading', 'divider', 'html', 'hidden'].includes(fieldData.field_type)
	);
	const showsBasicInputs = $derived(!['heading', 'divider', 'html'].includes(fieldData.field_type));

	// ─────────────────────────────────────────────────────────────────────────
	// Function bindings: textarea <-> fieldData.options
	//
	// `options` is a `JsonValue` that may hold a `FieldOption[]`, a `string[]`,
	// or a non-array shape (newsletter config, etc.). The textarea only edits
	// the `FieldOption[]` flavour; non-array shapes are passed through
	// untouched (read as empty, save leaves them alone).
	// ─────────────────────────────────────────────────────────────────────────

	function readOptionLabel(o: unknown): string {
		if (typeof o === 'string') return o;
		if (o && typeof o === 'object' && !Array.isArray(o)) {
			const obj = o as Record<string, unknown>;
			if (typeof obj.label === 'string') return obj.label;
			if (typeof obj.value === 'string') return obj.value;
		}
		return '';
	}

	function getOptionsText(): string {
		if (!Array.isArray(fieldData.options)) return '';
		return fieldData.options.map(readOptionLabel).filter(Boolean).join('\n');
	}

	function setOptionsText(text: string): void {
		const lines = text
			.split('\n')
			.map((s) => s.trim())
			.filter(Boolean);
		if (lines.length === 0) {
			fieldData.options = null;
			return;
		}
		// `FieldOption` has typed fields but no index signature, so it isn't
		// structurally `JsonValue`. The plain object literal below IS — `label`
		// and `value` are both strings, which satisfy JsonValue.
		fieldData.options = lines.map((label) => ({
			label,
			value: label.toLowerCase().replace(/\s+/g, '_')
		}));
	}

	// ─────────────────────────────────────────────────────────────────────────
	// Function bindings: checkbox <-> fieldData.conditional_logic existence
	// ─────────────────────────────────────────────────────────────────────────

	const BLANK_RULE: ConditionalRule = { field: '', operator: 'equals', value: '' };

	function isConditionalEnabled(): boolean {
		return fieldData.conditional_logic !== null;
	}

	function setConditionalEnabled(enabled: boolean): void {
		if (enabled) {
			if (!fieldData.conditional_logic) {
				fieldData.conditional_logic = {
					enabled: true,
					action: 'show',
					logic: 'all',
					rules: [{ ...BLANK_RULE }]
				};
			}
		} else {
			fieldData.conditional_logic = null;
		}
	}

	function addConditionalRule(): void {
		const cl: ConditionalLogic = fieldData.conditional_logic ?? {
			enabled: true,
			action: 'show',
			logic: 'all',
			rules: []
		};
		fieldData.conditional_logic = {
			...cl,
			rules: [...cl.rules, { ...BLANK_RULE }]
		};
	}

	function removeConditionalRule(index: number): void {
		const cl = fieldData.conditional_logic;
		if (!cl) return;
		const remaining = cl.rules.filter((_, i) => i !== index);
		if (remaining.length === 0) {
			fieldData.conditional_logic = null;
		} else {
			fieldData.conditional_logic = { ...cl, rules: remaining };
		}
	}

	// ─────────────────────────────────────────────────────────────────────────
	// Commit
	// ─────────────────────────────────────────────────────────────────────────

	/** Slug-ify a label into a unique-looking field name. */
	function slugify(label: string): string {
		return label
			.toLowerCase()
			.replace(/[^a-z0-9]+/g, '_')
			.replace(/^_+|_+$/g, '');
	}

	function pruneValidation(v: FieldValidation | null | undefined): FieldValidation | null {
		if (!v) return null;
		const out: Record<string, unknown> = {};
		for (const [k, val] of Object.entries(v)) {
			if (val !== undefined && val !== '' && val !== null) out[k] = val;
		}
		return Object.keys(out).length ? (out as FieldValidation) : null;
	}

	function handleSave(): void {
		// Auto-derive `name` from `label` when the user didn't set one.
		if (!fieldData.name && fieldData.label) {
			fieldData.name = slugify(fieldData.label);
		}
		// Drop empty validation keys before round-tripping through JSONB.
		fieldData.validation = pruneValidation(fieldData.validation);
		onsave?.(fieldData);
	}

	function handleCancel(): void {
		oncancel?.();
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

		{#if showsBasicInputs}
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
					<label for="field-required" class="checkbox-label">
						<input id="field-required" type="checkbox" bind:checked={fieldData.required} />
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
					bind:value={getOptionsText, setOptionsText}
					placeholder="Option 1&#10;Option 2&#10;Option 3"
					class="form-input"
					rows="5"
					required
				></textarea>
			</div>
		{/if}

		<!-- Validation -->
		{#if supportsValidation && fieldData.validation}
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
				<label for="conditional-enable" class="checkbox-label">
					<input
						id="conditional-enable"
						type="checkbox"
						bind:checked={isConditionalEnabled, setConditionalEnabled}
					/>
					<span>Enable conditional logic</span>
				</label>
			</div>

			{#if fieldData.conditional_logic}
				<div class="conditional-config">
					<div class="form-row">
						<div class="form-group">
							<label for="conditional-action">Action</label>
							<select
								id="conditional-action"
								bind:value={fieldData.conditional_logic.action}
								class="form-input"
							>
								<option value="show">Show this field if</option>
								<option value="hide">Hide this field if</option>
							</select>
						</div>

						<div class="form-group">
							<label for="conditional-match">Match</label>
							<select
								id="conditional-match"
								bind:value={fieldData.conditional_logic.logic}
								class="form-input"
							>
								<option value="all">All conditions are met</option>
								<option value="any">Any condition is met</option>
							</select>
						</div>
					</div>

					<div class="conditional-rules">
						{#each fieldData.conditional_logic.rules as rule, index (index)}
							<div class="rule-row">
								<label for="rule-field-{index}" class="sr-only">Rule field</label>
								<select id="rule-field-{index}" bind:value={rule.field} class="form-input">
									<option value="">Select field...</option>
									{#each availableFields as availField (availField.name)}
										<option value={availField.name}>{availField.label}</option>
									{/each}
								</select>

								<label for="rule-operator-{index}" class="sr-only">Rule operator</label>
								<select id="rule-operator-{index}" bind:value={rule.operator} class="form-input">
									<option value="equals">Equals</option>
									<option value="not_equals">Not Equals</option>
									<option value="contains">Contains</option>
									<option value="greater_than">Greater Than</option>
									<option value="less_than">Less Than</option>
									<option value="is_empty">Is Empty</option>
									<option value="is_not_empty">Is Not Empty</option>
								</select>

								{#if rule.operator !== 'is_empty' && rule.operator !== 'is_not_empty'}
									<label for="rule-value-{index}" class="sr-only">Rule value</label>
									<input
										id="rule-value-{index}"
										type="text"
										bind:value={rule.value}
										placeholder="Value"
										class="form-input"
									/>
								{/if}

								<button
									type="button"
									class="btn-remove"
									onclick={() => removeConditionalRule(index)}
									title="Remove rule"
									aria-label="Remove rule {index + 1}"
								>
									✕
								</button>
							</div>
						{/each}
					</div>

					<button type="button" class="btn-add-rule" onclick={addConditionalRule}>
						+ Add Rule
					</button>
				</div>
			{/if}
		</div>
	</div>

	<div class="editor-actions">
		<button type="button" class="btn btn-secondary" onclick={handleCancel}>Cancel</button>
		<button type="button" class="btn btn-primary" onclick={handleSave}>Save Field</button>
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

	/* 2026 Mobile-First: 44px touch targets, 16px font */
	.form-input {
		width: 100%;
		padding: 0.75rem 1rem;
		min-height: 44px; /* Touch target */
		background: #0f172a;
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 8px;
		font-size: 1rem; /* 16px prevents iOS zoom */
		color: #e2e8f0;
		touch-action: manipulation;
		-webkit-appearance: none;
		appearance: none;
	}

	.form-input:focus {
		outline: none;
		border-color: #6366f1;
		box-shadow: 0 0 0 3px rgba(99, 102, 241, 0.1);
	}

	/* 2026 Mobile-First: Stack on mobile, side-by-side on larger screens */
	.form-row {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	@media (min-width: 640px) {
		.form-row {
			display: grid;
			grid-template-columns: 1fr 1fr;
		}
	}

	.checkbox-label {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		cursor: pointer;
		font-size: 1rem; /* 16px for readability */
		font-weight: 500;
		color: #94a3b8;
		min-height: 44px; /* Touch target */
		padding: 0.5rem 0;
		touch-action: manipulation;
	}

	.checkbox-label input {
		cursor: pointer;
		width: 20px;
		height: 20px;
		min-width: 20px;
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

	/* 2026 Mobile-First: Stack rule fields on mobile */
	.rule-row {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		align-items: stretch;
	}

	@media (min-width: 640px) {
		.rule-row {
			display: grid;
			grid-template-columns: 1fr 1fr 1fr auto;
			align-items: center;
		}
	}

	.btn-remove {
		background: rgba(239, 68, 68, 0.1);
		color: #f87171;
		border: 1px solid rgba(239, 68, 68, 0.2);
		border-radius: 8px;
		width: 44px; /* Touch target */
		height: 44px; /* Touch target */
		min-width: 44px;
		min-height: 44px;
		font-size: 1rem;
		cursor: pointer;
		transition: all 0.2s;
		display: flex;
		align-items: center;
		justify-content: center;
		touch-action: manipulation;
	}

	.btn-remove:hover {
		background: rgba(239, 68, 68, 0.2);
	}

	.btn-add-rule {
		align-self: flex-start;
		padding: 0.75rem 1rem;
		min-height: 44px; /* Touch target */
		background: rgba(99, 102, 241, 0.1);
		color: #a5b4fc;
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 8px;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
		touch-action: manipulation;
	}

	.btn-add-rule:hover {
		background: rgba(99, 102, 241, 0.2);
	}

	/* 2026 Mobile-First: Safe area insets and full-width buttons on mobile */
	.editor-actions {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
		margin-top: 2rem;
		padding-top: 2rem;
		padding-bottom: env(safe-area-inset-bottom, 0);
		border-top: 1px solid rgba(99, 102, 241, 0.1);
	}

	@media (min-width: 640px) {
		.editor-actions {
			flex-direction: row;
			justify-content: flex-end;
			gap: 1rem;
		}
	}

	.btn {
		padding: 0.875rem 1.5rem;
		min-height: 48px; /* Enhanced touch target */
		border-radius: 8px;
		font-size: 1rem; /* 16px prevents iOS zoom */
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
		border: none;
		width: 100%;
		touch-action: manipulation;
	}

	@media (min-width: 640px) {
		.btn {
			width: auto;
		}
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

	.sr-only {
		position: absolute;
		width: 1px;
		height: 1px;
		padding: 0;
		margin: -1px;
		overflow: hidden;
		clip: rect(0, 0, 0, 0);
		white-space: nowrap;
		border-width: 0;
	}
</style>

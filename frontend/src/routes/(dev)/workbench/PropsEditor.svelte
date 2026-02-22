<script lang="ts">
import { logger } from '$lib/utils/logger';
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * Props Editor - Auto-Generated Props Form
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @description Generates form inputs for all prop types
	 * @version 1.0.0 - January 2026
	 * @standards Apple Principal Engineer ICT Level 7+
	 */
	import type { PropDefinition } from './+page.server';

	interface Props {
		props: PropDefinition[];
		values: Record<string, unknown>;
		onChange: (name: string, value: unknown) => void;
	}

	let { props, values, onChange }: Props = $props();

	// ═══════════════════════════════════════════════════════════════════════════
	// PROP TYPE DETECTION
	// ═══════════════════════════════════════════════════════════════════════════

	type PropInputType =
		| 'text'
		| 'number'
		| 'boolean'
		| 'select'
		| 'array'
		| 'object'
		| 'function'
		| 'snippet';

	function detectInputType(prop: PropDefinition): PropInputType {
		const type = prop.type.toLowerCase().trim();

		// Boolean
		if (type === 'boolean') return 'boolean';

		// Number
		if (type === 'number') return 'number';

		// Function/callback
		if (type.includes('=>') || type.startsWith('(')) return 'function';

		// Snippet
		if (type.includes('snippet')) return 'snippet';

		// Array
		if (type.includes('[]') || type.startsWith('array')) return 'array';

		// Union types (enums)
		if (type.includes("'") && type.includes('|')) return 'select';

		// Object types
		if (type.startsWith('{') || type === 'object' || type.includes('record')) return 'object';

		// Default to text
		return 'text';
	}

	function parseUnionOptions(type: string): string[] {
		const matches = type.match(/'([^']+)'/g);
		if (matches) {
			return matches.map((m) => m.replace(/'/g, ''));
		}
		return [];
	}

	function getDefaultValue(prop: PropDefinition, inputType: PropInputType): unknown {
		if (values[prop.name] !== undefined) return values[prop.name];
		if (prop.defaultValue) {
			try {
				// Try to parse as JSON
				return JSON.parse(prop.defaultValue);
			} catch {
				// Return as string, removing quotes
				return prop.defaultValue.replace(/^['"]|['"]$/g, '');
			}
		}

		switch (inputType) {
			case 'boolean':
				return false;
			case 'number':
				return 0;
			case 'array':
				return [];
			case 'object':
				return {};
			default:
				return '';
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// HANDLERS
	// ═══════════════════════════════════════════════════════════════════════════

	function handleTextChange(prop: PropDefinition, event: Event) {
		const target = event.target as HTMLInputElement;
		onChange(prop.name, target.value);
	}

	function handleNumberChange(prop: PropDefinition, event: Event) {
		const target = event.target as HTMLInputElement;
		onChange(prop.name, parseFloat(target.value) || 0);
	}

	function handleBooleanChange(prop: PropDefinition, event: Event) {
		const target = event.target as HTMLInputElement;
		onChange(prop.name, target.checked);
	}

	function handleSelectChange(prop: PropDefinition, event: Event) {
		const target = event.target as HTMLSelectElement;
		onChange(prop.name, target.value);
	}

	function handleArrayChange(prop: PropDefinition, value: string) {
		try {
			const parsed = JSON.parse(value);
			if (Array.isArray(parsed)) {
				onChange(prop.name, parsed);
			}
		} catch {
			// Keep existing value on parse error
		}
	}

	function handleObjectChange(prop: PropDefinition, value: string) {
		try {
			const parsed = JSON.parse(value);
			if (typeof parsed === 'object' && parsed !== null) {
				onChange(prop.name, parsed);
			}
		} catch {
			// Keep existing value on parse error
		}
	}

	function handleFunctionCall(prop: PropDefinition) {
		logger.info(`[Workbench] ${prop.name}() called`);
		const fn = values[prop.name];
		if (typeof fn === 'function') {
			fn();
		}
	}

	function resetToDefault(prop: PropDefinition) {
		const inputType = detectInputType(prop);
		const defaultVal = getDefaultValue(prop, inputType);
		onChange(prop.name, defaultVal);
	}

	function copyPropsAsJSON() {
		const json = JSON.stringify(values, null, 2);
		navigator.clipboard.writeText(json);
	}
</script>

<div class="props-editor">
	<header class="editor-header">
		<h3 class="editor-title">Props</h3>
		<button class="copy-btn" onclick={copyPropsAsJSON} title="Copy as JSON"> 📋 </button>
	</header>

	{#if props.length === 0}
		<p class="no-props">No props defined</p>
	{:else}
		<div class="props-list">
			{#each props as prop}
				{@const inputType = detectInputType(prop)}
				{@const currentValue = values[prop.name] ?? getDefaultValue(prop, inputType)}

				<div class="prop-field">
					<div class="prop-label">
						<span class="prop-name" id="prop-label-{prop.name}">
							{prop.name}
							{#if prop.required}
								<span class="required">*</span>
							{/if}
						</span>
						<span class="prop-type">{prop.type}</span>
					</div>

					<div class="prop-input" role="group" aria-labelledby="prop-label-{prop.name}">
						{#if inputType === 'boolean'}
							<label class="toggle">
								<input
									type="checkbox"
									checked={Boolean(currentValue)}
									onchange={(e) => handleBooleanChange(prop, e)}
								/>
								<span class="toggle-slider"></span>
								<span class="toggle-label">{currentValue ? 'true' : 'false'}</span>
							</label>
						{:else if inputType === 'number'}
							<input
								type="number"
								value={currentValue}
								onchange={(e) => handleNumberChange(prop, e)}
								class="input input--number"
							/>
						{:else if inputType === 'select'}
							{@const options = parseUnionOptions(prop.type)}
							<select
								value={currentValue}
								onchange={(e) => handleSelectChange(prop, e)}
								class="input input--select"
							>
								{#each options as option}
									<option value={option}>{option}</option>
								{/each}
							</select>
						{:else if inputType === 'array'}
							<textarea
								class="input input--code"
								rows="3"
								value={JSON.stringify(currentValue, null, 2)}
								onchange={(e) => handleArrayChange(prop, (e.target as HTMLTextAreaElement).value)}
							></textarea>
						{:else if inputType === 'object'}
							<textarea
								class="input input--code"
								rows="4"
								value={JSON.stringify(currentValue, null, 2)}
								onchange={(e) => handleObjectChange(prop, (e.target as HTMLTextAreaElement).value)}
							></textarea>
						{:else if inputType === 'function'}
							<button class="action-btn" onclick={() => handleFunctionCall(prop)}>
								▶️ Call {prop.name}()
							</button>
						{:else if inputType === 'snippet'}
							<p class="snippet-note">Edit in Snippet Editor →</p>
						{:else}
							<input
								type="text"
								value={currentValue}
								oninput={(e) => handleTextChange(prop, e)}
								class="input input--text"
							/>
						{/if}

						{#if prop.defaultValue && inputType !== 'function' && inputType !== 'snippet'}
							<button
								class="reset-btn"
								onclick={() => resetToDefault(prop)}
								title="Reset to default"
							>
								↺
							</button>
						{/if}
					</div>
				</div>
			{/each}
		</div>
	{/if}
</div>

<style>
	.props-editor {
		width: 320px;
		min-width: 320px;
		height: 100%;
		overflow-y: auto;
		background: #111;
		border-left: 1px solid rgba(255, 255, 255, 0.1);
	}

	.editor-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0.75rem 1rem;
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
		position: sticky;
		top: 0;
		background: #111;
		z-index: 10;
	}

	.editor-title {
		margin: 0;
		font-size: 0.875rem;
		font-weight: 600;
		color: #fafafa;
	}

	.copy-btn {
		padding: 0.25rem 0.5rem;
		background: transparent;
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 0.25rem;
		font-size: 0.875rem;
		cursor: pointer;
		transition: background 0.15s;
	}

	.copy-btn:hover {
		background: rgba(255, 255, 255, 0.1);
	}

	.no-props {
		padding: 2rem;
		text-align: center;
		color: #71717a;
		font-size: 0.875rem;
	}

	.props-list {
		padding: 0.75rem;
	}

	.prop-field {
		margin-bottom: 1rem;
	}

	.prop-label {
		display: flex;
		justify-content: space-between;
		align-items: baseline;
		margin-bottom: 0.375rem;
	}

	.prop-name {
		font-size: 0.8125rem;
		font-weight: 500;
		color: #fafafa;
	}

	.required {
		color: #ef4444;
		margin-left: 0.125rem;
	}

	.prop-type {
		font-size: 0.625rem;
		color: #71717a;
		font-family: ui-monospace, monospace;
		max-width: 120px;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.prop-input {
		display: flex;
		gap: 0.375rem;
		align-items: flex-start;
	}

	.input {
		flex: 1;
		padding: 0.5rem 0.75rem;
		background: rgba(255, 255, 255, 0.05);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 0.375rem;
		color: #fafafa;
		font-size: 0.8125rem;
	}

	.input:focus {
		outline: none;
		border-color: rgba(139, 92, 246, 0.5);
	}

	.input--code {
		font-family: ui-monospace, monospace;
		font-size: 0.75rem;
		resize: vertical;
	}

	.input--select {
		cursor: pointer;
	}

	.toggle {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		cursor: pointer;
	}

	.toggle input {
		display: none;
	}

	.toggle-slider {
		position: relative;
		width: 36px;
		height: 20px;
		background: rgba(255, 255, 255, 0.1);
		border-radius: 9999px;
		transition: background 0.2s;
	}

	.toggle-slider::after {
		content: '';
		position: absolute;
		left: 2px;
		top: 2px;
		width: 16px;
		height: 16px;
		background: #fafafa;
		border-radius: 50%;
		transition: transform 0.2s;
	}

	.toggle input:checked + .toggle-slider {
		background: #8b5cf6;
	}

	.toggle input:checked + .toggle-slider::after {
		transform: translateX(16px);
	}

	.toggle-label {
		font-size: 0.75rem;
		color: #a1a1aa;
		font-family: ui-monospace, monospace;
	}

	.action-btn {
		flex: 1;
		padding: 0.5rem;
		background: rgba(139, 92, 246, 0.2);
		border: 1px solid rgba(139, 92, 246, 0.4);
		border-radius: 0.375rem;
		color: #c4b5fd;
		font-size: 0.8125rem;
		cursor: pointer;
		transition: all 0.15s;
	}

	.action-btn:hover {
		background: rgba(139, 92, 246, 0.3);
	}

	.reset-btn {
		padding: 0.5rem;
		background: transparent;
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 0.375rem;
		color: #71717a;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.15s;
	}

	.reset-btn:hover {
		background: rgba(255, 255, 255, 0.1);
		color: #fafafa;
	}

	.snippet-note {
		margin: 0;
		padding: 0.5rem;
		color: #71717a;
		font-size: 0.75rem;
		font-style: italic;
	}
</style>

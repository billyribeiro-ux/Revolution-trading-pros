<script lang="ts">
	/**
	 * ColorPickerField Component (FluentForms Pro)
	 *
	 * Color picker input field using native color picker with optional presets.
	 */

	interface Props {
		name: string;
		value?: string;
		label?: string;
		placeholder?: string;
		required?: boolean;
		disabled?: boolean;
		presets?: string[];
		showInput?: boolean;
		format?: 'hex' | 'rgb' | 'hsl';
		error?: string;
		helpText?: string;
		onchange?: (color: string) => void;
	}

	let {
		name,
		value = '#E6B800',
		label = '',
		placeholder = 'Select a color',
		required = false,
		disabled = false,
		presets = ['#ef4444', '#f97316', '#eab308', '#22c55e', '#E6B800', '#B38F00', '#ec4899', '#000000', '#ffffff'],
		showInput = true,
		format = 'hex',
		error = '',
		helpText = '',
		onchange
	}: Props = $props();

	let colorValue = $state('#E6B800');

	// Sync with prop value changes
	$effect(() => {
		colorValue = value;
	});

	function handleColorChange(e: Event) {
		const target = e.target as HTMLInputElement;
		colorValue = target.value;
		if (onchange) onchange(formatColor(colorValue));
	}

	function handleInputChange(e: Event) {
		const target = e.target as HTMLInputElement;
		const newValue = target.value;
		if (isValidColor(newValue)) {
			colorValue = newValue;
			if (onchange) onchange(formatColor(colorValue));
		}
	}

	function selectPreset(color: string) {
		colorValue = color;
		if (onchange) onchange(formatColor(colorValue));
	}

	function isValidColor(color: string): boolean {
		const s = new Option().style;
		s.color = color;
		return s.color !== '';
	}

	function formatColor(color: string): string {
		if (format === 'hex') return color;
		// Convert hex to other formats if needed
		const hex = color.replace('#', '');
		const r = parseInt(hex.substring(0, 2), 16);
		const g = parseInt(hex.substring(2, 4), 16);
		const b = parseInt(hex.substring(4, 6), 16);

		if (format === 'rgb') {
			return `rgb(${r}, ${g}, ${b})`;
		}
		if (format === 'hsl') {
			const rNorm = r / 255;
			const gNorm = g / 255;
			const bNorm = b / 255;
			const max = Math.max(rNorm, gNorm, bNorm);
			const min = Math.min(rNorm, gNorm, bNorm);
			let h = 0;
			let s = 0;
			const l = (max + min) / 2;

			if (max !== min) {
				const d = max - min;
				s = l > 0.5 ? d / (2 - max - min) : d / (max + min);
				switch (max) {
					case rNorm: h = ((gNorm - bNorm) / d + (gNorm < bNorm ? 6 : 0)) / 6; break;
					case gNorm: h = ((bNorm - rNorm) / d + 2) / 6; break;
					case bNorm: h = ((rNorm - gNorm) / d + 4) / 6; break;
				}
			}
			return `hsl(${Math.round(h * 360)}, ${Math.round(s * 100)}%, ${Math.round(l * 100)}%)`;
		}
		return color;
	}
</script>

<div class="color-picker-field" class:disabled class:has-error={error}>
	{#if label}
		<label for={name} class="field-label">
			{label}
			{#if required}
				<span class="required">*</span>
			{/if}
		</label>
	{/if}

	<div class="picker-container">
		<div class="color-preview" style="background-color: {colorValue}">
			<input
				type="color"
				id={name}
				{name}
				value={colorValue}
				oninput={handleColorChange}
				{disabled}
				class="color-input"
			/>
		</div>

		{#if showInput}
			<input
				type="text"
				value={colorValue}
				oninput={handleInputChange}
				{placeholder}
				{disabled}
				class="text-input"
			/>
		{/if}
	</div>

	{#if presets.length > 0}
		<div class="presets">
			{#each presets as preset}
				<button
					type="button"
					class="preset-btn"
					class:active={colorValue.toLowerCase() === preset.toLowerCase()}
					style="background-color: {preset}"
					onclick={() => selectPreset(preset)}
					{disabled}
					title={preset}
				></button>
			{/each}
		</div>
	{/if}

	{#if helpText && !error}
		<p class="help-text">{helpText}</p>
	{/if}

	{#if error}
		<p class="error-text">{error}</p>
	{/if}

	<input type="hidden" name="{name}_formatted" value={formatColor(colorValue)} />
</div>

<style>
	.color-picker-field {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
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

	.picker-container {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.color-preview {
		position: relative;
		width: 48px;
		height: 48px;
		border-radius: 0.5rem;
		border: 2px solid #e5e7eb;
		cursor: pointer;
		overflow: hidden;
		transition: border-color 0.15s;
	}

	.color-preview:hover {
		border-color: #9ca3af;
	}

	.color-input {
		position: absolute;
		inset: 0;
		width: 100%;
		height: 100%;
		opacity: 0;
		cursor: pointer;
	}

	.text-input {
		flex: 1;
		padding: 0.75rem;
		border: 1px solid #d1d5db;
		border-radius: 0.5rem;
		font-size: 0.9375rem;
		font-family: monospace;
		color: #374151;
	}

	.text-input:focus {
		outline: none;
		border-color: #E6B800;
		box-shadow: 0 0 0 3px rgba(230, 184, 0, 0.1);
	}

	.presets {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
	}

	.preset-btn {
		width: 28px;
		height: 28px;
		border: 2px solid transparent;
		border-radius: 0.375rem;
		cursor: pointer;
		transition: all 0.15s;
	}

	.preset-btn:hover {
		transform: scale(1.1);
	}

	.preset-btn.active {
		border-color: #E6B800;
		box-shadow: 0 0 0 2px rgba(230, 184, 0, 0.3);
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

	.has-error .text-input {
		border-color: #ef4444;
	}
</style>

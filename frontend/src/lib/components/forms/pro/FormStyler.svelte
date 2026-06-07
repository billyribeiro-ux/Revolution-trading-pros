<script lang="ts">
	/**
	 * FormStyler Component (FluentForms 6.1.5 - November 2025)
	 *
	 * Gutenberg Block Styler integration for form customization.
	 * Provides comprehensive styling options for form appearance.
	 */

	interface StyleSettings {
		// Container
		containerBg?: string;
		containerPadding?: string;
		containerBorderRadius?: string;
		containerBorderColor?: string;
		containerBorderWidth?: string;
		containerShadow?: 'none' | 'sm' | 'md' | 'lg' | 'xl';

		// Labels
		labelColor?: string;
		labelFontSize?: string;
		labelFontWeight?: string;
		labelSpacing?: string;

		// Inputs
		inputBg?: string;
		inputTextColor?: string;
		inputBorderColor?: string;
		inputBorderRadius?: string;
		inputFocusBorderColor?: string;
		inputPlaceholderColor?: string;
		inputPadding?: string;
		inputFontSize?: string;

		// Buttons
		buttonBg?: string;
		buttonTextColor?: string;
		buttonHoverBg?: string;
		buttonBorderRadius?: string;
		buttonPadding?: string;
		buttonFontSize?: string;
		buttonFontWeight?: string;
		buttonWidth?: 'auto' | 'full';

		// Error States
		errorColor?: string;
		errorBgColor?: string;

		// Success States
		successColor?: string;
		successBgColor?: string;

		// Typography
		fontFamily?: string;

		// Spacing
		fieldGap?: string;
		sectionGap?: string;
	}

	interface Props {
		settings?: StyleSettings;
		previewMode?: boolean;
		onSettingsChange?: (settings: StyleSettings) => void;
	}

	let { settings = {}, previewMode = false, onSettingsChange }: Props = $props();

	import Icon from '$lib/components/Icon.svelte';

	let activeTab = $state<'container' | 'inputs' | 'buttons' | 'typography'>('container');

	const shadowOptions = {
		none: 'none',
		sm: '0 1px 2px 0 rgb(0 0 0 / 0.05)',
		md: '0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)',
		lg: '0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1)',
		xl: '0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1)'
	};

	const fontFamilies = [
		{ value: 'inherit', label: 'System Default' },
		{ value: 'Inter, sans-serif', label: 'Inter' },
		{ value: 'Roboto, sans-serif', label: 'Roboto' },
		{ value: 'Open Sans, sans-serif', label: 'Open Sans' },
		{ value: 'Lato, sans-serif', label: 'Lato' },
		{ value: 'Poppins, sans-serif', label: 'Poppins' },
		{ value: 'Montserrat, sans-serif', label: 'Montserrat' },
		{ value: 'Georgia, serif', label: 'Georgia' },
		{ value: 'Times New Roman, serif', label: 'Times New Roman' }
	];

	function updateSetting<K extends keyof StyleSettings>(key: K, value: StyleSettings[K]) {
		settings = { ...settings, [key]: value };
		onSettingsChange?.(settings);
	}

	function generateCssVariables(): string {
		const vars: string[] = [];

		// Container
		if (settings.containerBg) vars.push(`--form-container-bg: ${settings.containerBg}`);
		if (settings.containerPadding)
			vars.push(`--form-container-padding: ${settings.containerPadding}`);
		if (settings.containerBorderRadius)
			vars.push(`--form-container-radius: ${settings.containerBorderRadius}`);
		if (settings.containerBorderColor)
			vars.push(`--form-container-border-color: ${settings.containerBorderColor}`);
		if (settings.containerBorderWidth)
			vars.push(`--form-container-border-width: ${settings.containerBorderWidth}`);
		if (settings.containerShadow)
			vars.push(`--form-container-shadow: ${shadowOptions[settings.containerShadow]}`);

		// Labels
		if (settings.labelColor) vars.push(`--form-label-color: ${settings.labelColor}`);
		if (settings.labelFontSize) vars.push(`--form-label-font-size: ${settings.labelFontSize}`);
		if (settings.labelFontWeight)
			vars.push(`--form-label-font-weight: ${settings.labelFontWeight}`);
		if (settings.labelSpacing) vars.push(`--form-label-spacing: ${settings.labelSpacing}`);

		// Inputs
		if (settings.inputBg) vars.push(`--form-input-bg: ${settings.inputBg}`);
		if (settings.inputTextColor) vars.push(`--form-input-color: ${settings.inputTextColor}`);
		if (settings.inputBorderColor)
			vars.push(`--form-input-border-color: ${settings.inputBorderColor}`);
		if (settings.inputBorderRadius) vars.push(`--form-input-radius: ${settings.inputBorderRadius}`);
		if (settings.inputFocusBorderColor)
			vars.push(`--form-input-focus-color: ${settings.inputFocusBorderColor}`);
		if (settings.inputPlaceholderColor)
			vars.push(`--form-input-placeholder-color: ${settings.inputPlaceholderColor}`);
		if (settings.inputPadding) vars.push(`--form-input-padding: ${settings.inputPadding}`);
		if (settings.inputFontSize) vars.push(`--form-input-font-size: ${settings.inputFontSize}`);

		// Buttons
		if (settings.buttonBg) vars.push(`--form-button-bg: ${settings.buttonBg}`);
		if (settings.buttonTextColor) vars.push(`--form-button-color: ${settings.buttonTextColor}`);
		if (settings.buttonHoverBg) vars.push(`--form-button-hover-bg: ${settings.buttonHoverBg}`);
		if (settings.buttonBorderRadius)
			vars.push(`--form-button-radius: ${settings.buttonBorderRadius}`);
		if (settings.buttonPadding) vars.push(`--form-button-padding: ${settings.buttonPadding}`);
		if (settings.buttonFontSize) vars.push(`--form-button-font-size: ${settings.buttonFontSize}`);
		if (settings.buttonFontWeight)
			vars.push(`--form-button-font-weight: ${settings.buttonFontWeight}`);

		// Error/Success
		if (settings.errorColor) vars.push(`--form-error-color: ${settings.errorColor}`);
		if (settings.errorBgColor) vars.push(`--form-error-bg: ${settings.errorBgColor}`);
		if (settings.successColor) vars.push(`--form-success-color: ${settings.successColor}`);
		if (settings.successBgColor) vars.push(`--form-success-bg: ${settings.successBgColor}`);

		// Typography
		if (settings.fontFamily) vars.push(`--form-font-family: ${settings.fontFamily}`);

		// Spacing
		if (settings.fieldGap) vars.push(`--form-field-gap: ${settings.fieldGap}`);
		if (settings.sectionGap) vars.push(`--form-section-gap: ${settings.sectionGap}`);

		return vars.join('; ');
	}

	function resetToDefaults() {
		settings = {};
		onSettingsChange?.(settings);
	}

	function exportSettings(): string {
		return JSON.stringify(settings, null, 2);
	}

	function copyToClipboard() {
		navigator.clipboard.writeText(generateCssVariables());
	}

	function getTabClasses(tab: typeof activeTab) {
		return ['tab', activeTab === tab && 'active'];
	}

	const cssOutput = $derived(generateCssVariables());
</script>

<div class="form-styler">
	<div class="styler-header">
		<h3 class="styler-title">Form Style Settings</h3>
		<div class="header-actions">
			<button type="button" class="btn-icon" onclick={copyToClipboard} title="Copy CSS Variables">
				<Icon name="IconCopy" size={16} />
			</button>
			<button type="button" class="btn-reset" onclick={resetToDefaults}> Reset to Defaults </button>
		</div>
	</div>

	<div class="styler-tabs">
		<button
			type="button"
			class={getTabClasses('container')}
			onclick={() => (activeTab = 'container')}
		>
			Container
		</button>
		<button type="button" class={getTabClasses('inputs')} onclick={() => (activeTab = 'inputs')}>
			Inputs
		</button>
		<button type="button" class={getTabClasses('buttons')} onclick={() => (activeTab = 'buttons')}>
			Buttons
		</button>
		<button
			type="button"
			class={getTabClasses('typography')}
			onclick={() => (activeTab = 'typography')}
		>
			Typography
		</button>
	</div>

	<div class="styler-content">
		{#if activeTab === 'container'}
			<div class="settings-group">
				<div class="setting-row">
					<label for="container-bg">Background Color</label>
					<input
						id="container-bg"
						type="color"
						value={settings.containerBg || '#ffffff'}
						oninput={(e: Event) =>
							updateSetting('containerBg', (e.target as HTMLInputElement).value)}
					/>
				</div>
				<div class="setting-row">
					<label for="container-padding">Padding</label>
					<input
						id="container-padding"
						type="text"
						value={settings.containerPadding || '1.5rem'}
						oninput={(e: Event) =>
							updateSetting('containerPadding', (e.target as HTMLInputElement).value)}
						placeholder="1.5rem"
					/>
				</div>
				<div class="setting-row">
					<label for="container-border-radius">Border Radius</label>
					<input
						id="container-border-radius"
						type="text"
						value={settings.containerBorderRadius || '0.5rem'}
						oninput={(e: Event) =>
							updateSetting('containerBorderRadius', (e.target as HTMLInputElement).value)}
						placeholder="0.5rem"
					/>
				</div>
				<div class="setting-row">
					<label for="container-border-color">Border Color</label>
					<input
						id="container-border-color"
						type="color"
						value={settings.containerBorderColor || '#e5e7eb'}
						oninput={(e: Event) =>
							updateSetting('containerBorderColor', (e.target as HTMLInputElement).value)}
					/>
				</div>
				<div class="setting-row">
					<label for="container-border-width">Border Width</label>
					<input
						id="container-border-width"
						type="text"
						value={settings.containerBorderWidth || '1px'}
						oninput={(e: Event) =>
							updateSetting('containerBorderWidth', (e.target as HTMLInputElement).value)}
						placeholder="1px"
					/>
				</div>
				<div class="setting-row">
					<label for="container-shadow">Shadow</label>
					<select
						id="container-shadow"
						value={settings.containerShadow || 'none'}
						onchange={(e: Event) =>
							updateSetting(
								'containerShadow',
								(e.target as HTMLSelectElement).value as StyleSettings['containerShadow']
							)}
					>
						<option value="none">None</option>
						<option value="sm">Small</option>
						<option value="md">Medium</option>
						<option value="lg">Large</option>
						<option value="xl">Extra Large</option>
					</select>
				</div>
			</div>
		{:else if activeTab === 'inputs'}
			<div class="settings-group">
				<div class="setting-row">
					<label for="input-bg">Background Color</label>
					<input
						id="input-bg"
						type="color"
						value={settings.inputBg || '#ffffff'}
						oninput={(e: Event) => updateSetting('inputBg', (e.target as HTMLInputElement).value)}
					/>
				</div>
				<div class="setting-row">
					<label for="input-text-color">Text Color</label>
					<input
						id="input-text-color"
						type="color"
						value={settings.inputTextColor || '#374151'}
						oninput={(e: Event) =>
							updateSetting('inputTextColor', (e.target as HTMLInputElement).value)}
					/>
				</div>
				<div class="setting-row">
					<label for="input-border-color">Border Color</label>
					<input
						id="input-border-color"
						type="color"
						value={settings.inputBorderColor || '#d1d5db'}
						oninput={(e: Event) =>
							updateSetting('inputBorderColor', (e.target as HTMLInputElement).value)}
					/>
				</div>
				<div class="setting-row">
					<label for="input-focus-border-color">Focus Border Color</label>
					<input
						id="input-focus-border-color"
						type="color"
						value={settings.inputFocusBorderColor || '#3b82f6'}
						oninput={(e: Event) =>
							updateSetting('inputFocusBorderColor', (e.target as HTMLInputElement).value)}
					/>
				</div>
				<div class="setting-row">
					<label for="input-placeholder-color">Placeholder Color</label>
					<input
						id="input-placeholder-color"
						type="color"
						value={settings.inputPlaceholderColor || '#9ca3af'}
						oninput={(e: Event) =>
							updateSetting('inputPlaceholderColor', (e.target as HTMLInputElement).value)}
					/>
				</div>
				<div class="setting-row">
					<label for="input-border-radius">Border Radius</label>
					<input
						id="input-border-radius"
						type="text"
						value={settings.inputBorderRadius || '0.375rem'}
						oninput={(e: Event) =>
							updateSetting('inputBorderRadius', (e.target as HTMLInputElement).value)}
						placeholder="0.375rem"
					/>
				</div>
				<div class="setting-row">
					<label for="input-padding">Padding</label>
					<input
						id="input-padding"
						type="text"
						value={settings.inputPadding || '0.625rem 0.75rem'}
						oninput={(e: Event) =>
							updateSetting('inputPadding', (e.target as HTMLInputElement).value)}
						placeholder="0.625rem 0.75rem"
					/>
				</div>
				<div class="setting-row">
					<label for="input-font-size">Font Size</label>
					<input
						id="input-font-size"
						type="text"
						value={settings.inputFontSize || '0.9375rem'}
						oninput={(e: Event) =>
							updateSetting('inputFontSize', (e.target as HTMLInputElement).value)}
						placeholder="0.9375rem"
					/>
				</div>
				<div class="setting-row">
					<label for="label-color">Label Color</label>
					<input
						id="label-color"
						type="color"
						value={settings.labelColor || '#374151'}
						oninput={(e: Event) =>
							updateSetting('labelColor', (e.target as HTMLInputElement).value)}
					/>
				</div>
				<div class="setting-row">
					<label for="label-font-size">Label Font Size</label>
					<input
						id="label-font-size"
						type="text"
						value={settings.labelFontSize || '0.875rem'}
						oninput={(e: Event) =>
							updateSetting('labelFontSize', (e.target as HTMLInputElement).value)}
						placeholder="0.875rem"
					/>
				</div>
			</div>
		{:else if activeTab === 'buttons'}
			<div class="settings-group">
				<div class="setting-row">
					<label for="button-bg">Background Color</label>
					<input
						id="button-bg"
						type="color"
						value={settings.buttonBg || '#3b82f6'}
						oninput={(e: Event) => updateSetting('buttonBg', (e.target as HTMLInputElement).value)}
					/>
				</div>
				<div class="setting-row">
					<label for="button-text-color">Text Color</label>
					<input
						id="button-text-color"
						type="color"
						value={settings.buttonTextColor || '#ffffff'}
						oninput={(e: Event) =>
							updateSetting('buttonTextColor', (e.target as HTMLInputElement).value)}
					/>
				</div>
				<div class="setting-row">
					<label for="button-hover-bg">Hover Background</label>
					<input
						id="button-hover-bg"
						type="color"
						value={settings.buttonHoverBg || '#2563eb'}
						oninput={(e: Event) =>
							updateSetting('buttonHoverBg', (e.target as HTMLInputElement).value)}
					/>
				</div>
				<div class="setting-row">
					<label for="button-border-radius">Border Radius</label>
					<input
						id="button-border-radius"
						type="text"
						value={settings.buttonBorderRadius || '0.5rem'}
						oninput={(e: Event) =>
							updateSetting('buttonBorderRadius', (e.target as HTMLInputElement).value)}
						placeholder="0.5rem"
					/>
				</div>
				<div class="setting-row">
					<label for="button-padding">Padding</label>
					<input
						id="button-padding"
						type="text"
						value={settings.buttonPadding || '0.75rem 1.5rem'}
						oninput={(e: Event) =>
							updateSetting('buttonPadding', (e.target as HTMLInputElement).value)}
						placeholder="0.75rem 1.5rem"
					/>
				</div>
				<div class="setting-row">
					<label for="button-font-size">Font Size</label>
					<input
						id="button-font-size"
						type="text"
						value={settings.buttonFontSize || '1rem'}
						oninput={(e: Event) =>
							updateSetting('buttonFontSize', (e.target as HTMLInputElement).value)}
						placeholder="1rem"
					/>
				</div>
				<div class="setting-row">
					<label for="button-font-weight">Font Weight</label>
					<select
						id="button-font-weight"
						value={settings.buttonFontWeight || '600'}
						onchange={(e: Event) =>
							updateSetting('buttonFontWeight', (e.target as HTMLSelectElement).value)}
					>
						<option value="400">Normal</option>
						<option value="500">Medium</option>
						<option value="600">Semi-Bold</option>
						<option value="700">Bold</option>
					</select>
				</div>
				<div class="setting-row">
					<label for="button-width">Width</label>
					<select
						id="button-width"
						value={settings.buttonWidth || 'auto'}
						onchange={(e: Event) =>
							updateSetting(
								'buttonWidth',
								(e.target as HTMLSelectElement).value as StyleSettings['buttonWidth']
							)}
					>
						<option value="auto">Auto</option>
						<option value="full">Full Width</option>
					</select>
				</div>
			</div>
		{:else if activeTab === 'typography'}
			<div class="settings-group">
				<div class="setting-row">
					<label for="font-family">Font Family</label>
					<select
						id="font-family"
						value={settings.fontFamily || 'inherit'}
						onchange={(e: Event) =>
							updateSetting('fontFamily', (e.target as HTMLSelectElement).value)}
					>
						{#each fontFamilies as font (font.value)}
							<option value={font.value}>{font.label}</option>
						{/each}
					</select>
				</div>
				<div class="setting-row">
					<label for="error-color">Error Color</label>
					<input
						id="error-color"
						type="color"
						value={settings.errorColor || '#dc2626'}
						oninput={(e: Event) =>
							updateSetting('errorColor', (e.target as HTMLInputElement).value)}
					/>
				</div>
				<div class="setting-row">
					<label for="error-bg-color">Error Background</label>
					<input
						id="error-bg-color"
						type="color"
						value={settings.errorBgColor || '#fef2f2'}
						oninput={(e: Event) =>
							updateSetting('errorBgColor', (e.target as HTMLInputElement).value)}
					/>
				</div>
				<div class="setting-row">
					<label for="success-color">Success Color</label>
					<input
						id="success-color"
						type="color"
						value={settings.successColor || '#166534'}
						oninput={(e: Event) =>
							updateSetting('successColor', (e.target as HTMLInputElement).value)}
					/>
				</div>
				<div class="setting-row">
					<label for="success-bg-color">Success Background</label>
					<input
						id="success-bg-color"
						type="color"
						value={settings.successBgColor || '#dcfce7'}
						oninput={(e: Event) =>
							updateSetting('successBgColor', (e.target as HTMLInputElement).value)}
					/>
				</div>
				<div class="setting-row">
					<label for="field-gap">Field Gap</label>
					<input
						id="field-gap"
						type="text"
						value={settings.fieldGap || '1rem'}
						oninput={(e: Event) => updateSetting('fieldGap', (e.target as HTMLInputElement).value)}
						placeholder="1rem"
					/>
				</div>
				<div class="setting-row">
					<label for="section-gap">Section Gap</label>
					<input
						id="section-gap"
						type="text"
						value={settings.sectionGap || '2rem'}
						oninput={(e: Event) =>
							updateSetting('sectionGap', (e.target as HTMLInputElement).value)}
						placeholder="2rem"
					/>
				</div>
			</div>
		{/if}
	</div>

	{#if previewMode}
		<div class="preview-section">
			<h4>Preview</h4>
			<div class="form-preview" style={cssOutput}>
				<div class="preview-field">
					<label for="preview-input">Sample Label</label>
					<input id="preview-input" type="text" placeholder="Sample input field" />
				</div>
				<div class="preview-field">
					<label for="preview-textarea">Another Field</label>
					<textarea id="preview-textarea" placeholder="Sample textarea"></textarea>
				</div>
				<button type="button">Submit Button</button>
			</div>
		</div>
	{/if}

	<!-- Hidden output for form submission -->
	<input type="hidden" name="form_styles" value={exportSettings()} />
</div>

<style>
	.form-styler {
		display: flex;
		flex-direction: column;
		gap: 1rem;
		background-color: white;
		border: 1px solid #e5e7eb;
		border-radius: 0.75rem;
		padding: 1.25rem;
	}

	.styler-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding-bottom: 1rem;
		border-bottom: 1px solid #e5e7eb;
	}

	.styler-title {
		font-size: 1rem;
		font-weight: 600;
		color: #111827;
		margin: 0;
	}

	.header-actions {
		display: flex;
		gap: 0.5rem;
	}

	.btn-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 36px;
		height: 36px;
		background-color: #f9fafb;
		border: 1px solid #e5e7eb;
		border-radius: 0.375rem;
		color: #6b7280;
		cursor: pointer;
		transition: all 0.15s;
	}

	.btn-icon:hover {
		background-color: #f3f4f6;
		color: #374151;
	}

	.btn-reset {
		padding: 0.5rem 0.875rem;
		background-color: white;
		border: 1px solid #e5e7eb;
		border-radius: 0.375rem;
		font-size: 0.8125rem;
		color: #6b7280;
		cursor: pointer;
		transition: all 0.15s;
	}

	.btn-reset:hover {
		background-color: #f9fafb;
		color: #374151;
	}

	.styler-tabs {
		display: flex;
		gap: 0.25rem;
		padding: 0.25rem;
		background-color: #f3f4f6;
		border-radius: 0.5rem;
	}

	.tab {
		flex: 1;
		padding: 0.5rem 0.75rem;
		background: none;
		border: none;
		border-radius: 0.375rem;
		font-size: 0.8125rem;
		font-weight: 500;
		color: #6b7280;
		cursor: pointer;
		transition: all 0.15s;
	}

	.tab:hover {
		color: #374151;
	}

	.tab.active {
		background-color: white;
		color: #111827;
		box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
	}

	.styler-content {
		padding: 0.5rem 0;
	}

	.settings-group {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.setting-row {
		display: flex;
		justify-content: space-between;
		align-items: center;
		gap: 1rem;
	}

	.setting-row label {
		font-size: 0.8125rem;
		color: #4b5563;
		flex-shrink: 0;
	}

	.setting-row input[type='text'],
	.setting-row select {
		width: 140px;
		padding: 0.375rem 0.5rem;
		border: 1px solid #d1d5db;
		border-radius: 0.375rem;
		font-size: 0.8125rem;
		color: #374151;
	}

	.setting-row input[type='text']:focus,
	.setting-row select:focus {
		outline: none;
		border-color: #3b82f6;
	}

	.setting-row input[type='color'] {
		width: 40px;
		height: 32px;
		padding: 2px;
		border: 1px solid #d1d5db;
		border-radius: 0.375rem;
		cursor: pointer;
	}

	.preview-section {
		margin-top: 1rem;
		padding-top: 1rem;
		border-top: 1px solid #e5e7eb;
	}

	.preview-section h4 {
		font-size: 0.875rem;
		font-weight: 600;
		color: #374151;
		margin: 0 0 0.75rem;
	}

	.form-preview {
		display: flex;
		flex-direction: column;
		gap: var(--form-field-gap, 1rem);
		padding: var(--form-container-padding, 1.5rem);
		background-color: var(--form-container-bg, white);
		border: var(--form-container-border-width, 1px) solid
			var(--form-container-border-color, #e5e7eb);
		border-radius: var(--form-container-radius, 0.5rem);
		box-shadow: var(--form-container-shadow, none);
		font-family: var(--form-font-family, inherit);
	}

	.preview-field {
		display: flex;
		flex-direction: column;
		gap: var(--form-label-spacing, 0.375rem);
	}

	.preview-field label {
		font-size: var(--form-label-font-size, 0.875rem);
		font-weight: var(--form-label-font-weight, 500);
		color: var(--form-label-color, #374151);
	}

	.preview-field input,
	.preview-field textarea {
		padding: var(--form-input-padding, 0.625rem 0.75rem);
		background-color: var(--form-input-bg, white);
		color: var(--form-input-color, #374151);
		border: 1px solid var(--form-input-border-color, #d1d5db);
		border-radius: var(--form-input-radius, 0.375rem);
		font-size: var(--form-input-font-size, 0.9375rem);
		font-family: var(--form-font-family, inherit);
	}

	.preview-field input::placeholder,
	.preview-field textarea::placeholder {
		color: var(--form-input-placeholder-color, #9ca3af);
	}

	.preview-field input:focus,
	.preview-field textarea:focus {
		outline: none;
		border-color: var(--form-input-focus-color, #3b82f6);
	}

	.form-preview button {
		padding: var(--form-button-padding, 0.75rem 1.5rem);
		background-color: var(--form-button-bg, #3b82f6);
		color: var(--form-button-color, white);
		border: none;
		border-radius: var(--form-button-radius, 0.5rem);
		font-size: var(--form-button-font-size, 1rem);
		font-weight: var(--form-button-font-weight, 600);
		cursor: pointer;
		transition: background-color 0.15s;
		font-family: var(--form-font-family, inherit);
	}

	.form-preview button:hover {
		background-color: var(--form-button-hover-bg, #2563eb);
	}

	/* Responsive */
	@media (max-width: 479.98px) {
		.setting-row {
			flex-direction: column;
			align-items: flex-start;
			gap: 0.375rem;
		}

		.setting-row input[type='text'],
		.setting-row select {
			width: 100%;
		}

		.styler-tabs {
			flex-wrap: wrap;
		}

		.tab {
			flex: 1 1 45%;
		}
	}
</style>

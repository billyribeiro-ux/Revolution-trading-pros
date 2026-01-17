<script lang="ts">
	import type { FormTheme } from '$lib/data/formTemplates';
	import { themes } from '$lib/data/formTemplates';

	interface Props {
		selectedTheme?: FormTheme;
		onchange?: (theme: FormTheme) => void;
	}

	let { selectedTheme = $bindable(themes[0]), onchange }: Props = $props();

	let isCustomizing = $state(false);
	let customTheme: FormTheme = $state({ ...selectedTheme });

	function selectTheme(theme: FormTheme) {
		selectedTheme = theme;
		customTheme = { ...theme };
		isCustomizing = false;
		onchange?.(selectedTheme);
	}

	function enableCustomization() {
		isCustomizing = true;
		customTheme = { ...selectedTheme };
	}

	function applyCustomTheme() {
		selectedTheme = { ...customTheme, id: 'custom', name: 'Custom Theme' };
		onchange?.(selectedTheme);
	}

	function handleColorChange(property: keyof FormTheme['colors'], value: string) {
		customTheme = {
			...customTheme,
			colors: { ...customTheme.colors, [property]: value }
		};
	}

	function handleSpacingChange(value: 'compact' | 'normal' | 'spacious') {
		customTheme = { ...customTheme, spacing: value };
	}

	function handleBorderRadiusChange(value: 'none' | 'small' | 'medium' | 'large') {
		customTheme = { ...customTheme, borderRadius: value };
	}
</script>

<div class="theme-customizer">
	<div class="customizer-header">
		<h3>Theme Settings</h3>
		<p class="header-description">Choose a theme or create your own</p>
	</div>

	<div class="preset-themes">
		<div class="themes-label">Preset Themes</div>
		<div class="themes-grid">
			{#each themes as theme}
				<button
					class="theme-card"
					class:active={selectedTheme.id === theme.id && !isCustomizing}
					onclick={() => selectTheme(theme)}
				>
					<div
						class="theme-preview"
						style="background: {theme.colors.background}; border-color: {theme.colors.border}"
					>
						<div class="preview-header" style="background: {theme.colors.primary}"></div>
						<div class="preview-text" style="background: {theme.colors.text}20"></div>
						<div class="preview-text" style="background: {theme.colors.text}20"></div>
						<div class="preview-button" style="background: {theme.colors.primary}"></div>
					</div>
					<div class="theme-name">{theme.name}</div>
				</button>
			{/each}
		</div>
	</div>

	<div class="custom-section">
		{#if !isCustomizing}
			<button class="btn-customize" onclick={enableCustomization}> 🎨 Customize Theme </button>
		{:else}
			<div class="custom-controls">
				<div class="control-section">
					<h4>Colors</h4>
					<div class="color-controls">
						<div class="color-input-group">
							<label for="primary-color">Primary</label>
							<div class="color-input-wrapper">
								<input
									type="color"
									id="primary-color"
									value={customTheme.colors.primary}
									oninput={(e: Event) =>
										handleColorChange('primary', (e.currentTarget as HTMLInputElement).value)}
								/>
								<span class="color-value">{customTheme.colors.primary}</span>
							</div>
						</div>

						<div class="color-input-group">
							<label for="secondary-color">Secondary</label>
							<div class="color-input-wrapper">
								<input
									type="color"
									id="secondary-color"
									value={customTheme.colors.secondary}
									oninput={(e: Event) =>
										handleColorChange('secondary', (e.currentTarget as HTMLInputElement).value)}
								/>
								<span class="color-value">{customTheme.colors.secondary}</span>
							</div>
						</div>

						<div class="color-input-group">
							<label for="background-color">Background</label>
							<div class="color-input-wrapper">
								<input
									type="color"
									id="background-color"
									value={customTheme.colors.background}
									oninput={(e: Event) =>
										handleColorChange('background', (e.currentTarget as HTMLInputElement).value)}
								/>
								<span class="color-value">{customTheme.colors.background}</span>
							</div>
						</div>

						<div class="color-input-group">
							<label for="text-color">Text</label>
							<div class="color-input-wrapper">
								<input
									type="color"
									id="text-color"
									value={customTheme.colors.text}
									oninput={(e: Event) =>
										handleColorChange('text', (e.currentTarget as HTMLInputElement).value)}
								/>
								<span class="color-value">{customTheme.colors.text}</span>
							</div>
						</div>

						<div class="color-input-group">
							<label for="border-color">Border</label>
							<div class="color-input-wrapper">
								<input
									type="color"
									id="border-color"
									value={customTheme.colors.border}
									oninput={(e: Event) =>
										handleColorChange('border', (e.currentTarget as HTMLInputElement).value)}
								/>
								<span class="color-value">{customTheme.colors.border}</span>
							</div>
						</div>
					</div>
				</div>

				<div class="control-section">
					<h4>Spacing</h4>
					<div class="radio-group">
						<label class="radio-label">
							<input
								type="radio"
								name="spacing"
								value="compact"
								checked={customTheme.spacing === 'compact'}
								onchange={() => handleSpacingChange('compact')}
							/>
							<span>Compact</span>
						</label>
						<label class="radio-label">
							<input
								type="radio"
								name="spacing"
								value="normal"
								checked={customTheme.spacing === 'normal'}
								onchange={() => handleSpacingChange('normal')}
							/>
							<span>Normal</span>
						</label>
						<label class="radio-label">
							<input
								type="radio"
								name="spacing"
								value="spacious"
								checked={customTheme.spacing === 'spacious'}
								onchange={() => handleSpacingChange('spacious')}
							/>
							<span>Spacious</span>
						</label>
					</div>
				</div>

				<div class="control-section">
					<h4>Border Radius</h4>
					<div class="radio-group">
						<label class="radio-label">
							<input
								type="radio"
								name="borderRadius"
								value="none"
								checked={customTheme.borderRadius === 'none'}
								onchange={() => handleBorderRadiusChange('none')}
							/>
							<span>None</span>
						</label>
						<label class="radio-label">
							<input
								type="radio"
								name="borderRadius"
								value="small"
								checked={customTheme.borderRadius === 'small'}
								onchange={() => handleBorderRadiusChange('small')}
							/>
							<span>Small</span>
						</label>
						<label class="radio-label">
							<input
								type="radio"
								name="borderRadius"
								value="medium"
								checked={customTheme.borderRadius === 'medium'}
								onchange={() => handleBorderRadiusChange('medium')}
							/>
							<span>Medium</span>
						</label>
						<label class="radio-label">
							<input
								type="radio"
								name="borderRadius"
								value="large"
								checked={customTheme.borderRadius === 'large'}
								onchange={() => handleBorderRadiusChange('large')}
							/>
							<span>Large</span>
						</label>
					</div>
				</div>

				<div class="custom-actions">
					<button class="btn-cancel" onclick={() => (isCustomizing = false)}>Cancel</button>
					<button class="btn-apply" onclick={applyCustomTheme}>Apply Custom Theme</button>
				</div>
			</div>
		{/if}
	</div>

	<div class="theme-preview-section">
		<h4>Preview</h4>
		<div class="live-preview" style="background: {selectedTheme.colors.background}">
			<div class="preview-form-container">
				<h3 style="color: {selectedTheme.colors.text}; font-family: {selectedTheme.fonts.heading}">
					Sample Form
				</h3>
				<div class="preview-field-wrapper">
					<label
						for="preview-name"
						style="color: {selectedTheme.colors.text}; font-family: {selectedTheme.fonts.body}"
					>
						Your Name
					</label>
					<input
						id="preview-name"
						type="text"
						placeholder="Enter your name"
						style="border-color: {selectedTheme.colors.border}; color: {selectedTheme.colors
							.text}; font-family: {selectedTheme.fonts.body}"
					/>
				</div>
				<div class="preview-field-wrapper">
					<label
						for="preview-email"
						style="color: {selectedTheme.colors.text}; font-family: {selectedTheme.fonts.body}"
					>
						Email Address
					</label>
					<input
						id="preview-email"
						type="email"
						placeholder="you@example.com"
						style="border-color: {selectedTheme.colors.border}; color: {selectedTheme.colors
							.text}; font-family: {selectedTheme.fonts.body}"
					/>
				</div>
				<button
					class="preview-submit"
					style="background: {selectedTheme.colors.primary}; font-family: {selectedTheme.fonts
						.body}"
				>
					Submit
				</button>
			</div>
		</div>
	</div>
</div>

<style>
	.theme-customizer {
		background: #1e293b;
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 12px;
		padding: 1.5rem;
	}

	.customizer-header {
		margin-bottom: 1.5rem;
	}

	h3 {
		font-size: 1.25rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0 0 0.5rem 0;
	}

	.header-description {
		color: #94a3b8;
		font-size: 0.875rem;
		margin: 0;
	}

	.preset-themes {
		margin-bottom: 1.5rem;
	}

	.themes-label {
		font-size: 0.875rem;
		font-weight: 600;
		color: #94a3b8;
		margin-bottom: 0.75rem;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.themes-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
		gap: 1rem;
	}

	.theme-card {
		background: #0f172a;
		border: 2px solid rgba(99, 102, 241, 0.1);
		border-radius: 10px;
		padding: 0.75rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.theme-card:hover {
		border-color: rgba(99, 102, 241, 0.3);
		transform: translateY(-2px);
	}

	.theme-card.active {
		border-color: #6366f1;
		box-shadow: 0 0 0 2px rgba(99, 102, 241, 0.2);
	}

	.theme-preview {
		aspect-ratio: 4 / 3;
		border: 1px solid;
		border-radius: 6px;
		padding: 0.5rem;
		margin-bottom: 0.5rem;
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.preview-header {
		height: 0.5rem;
		border-radius: 2px;
	}

	.preview-text {
		height: 0.375rem;
		border-radius: 2px;
	}

	.preview-button {
		height: 0.625rem;
		border-radius: 3px;
		margin-top: auto;
	}

	.theme-name {
		font-size: 0.75rem;
		color: #e2e8f0;
		font-weight: 500;
		text-align: center;
	}

	.custom-section {
		margin-bottom: 1.5rem;
	}

	.btn-customize {
		width: 100%;
		padding: 0.875rem;
		background: rgba(99, 102, 241, 0.1);
		color: #a5b4fc;
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 8px;
		font-weight: 600;
		font-size: 0.9375rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-customize:hover {
		background: rgba(99, 102, 241, 0.2);
		border-color: rgba(99, 102, 241, 0.4);
	}

	.custom-controls {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}

	.control-section h4 {
		font-size: 0.875rem;
		font-weight: 600;
		color: #94a3b8;
		margin: 0 0 0.75rem 0;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.color-controls {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(140px, 1fr));
		gap: 1rem;
	}

	.color-input-group {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.color-input-group label {
		font-size: 0.8125rem;
		color: #94a3b8;
		font-weight: 500;
	}

	.color-input-wrapper {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.color-input-wrapper input[type='color'] {
		width: 40px;
		height: 40px;
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 6px;
		cursor: pointer;
		background: transparent;
	}

	.color-value {
		font-size: 0.75rem;
		color: #64748b;
		font-family: monospace;
	}

	.radio-group {
		display: flex;
		gap: 1rem;
		flex-wrap: wrap;
	}

	.radio-label {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		color: #94a3b8;
		font-size: 0.875rem;
		cursor: pointer;
	}

	.radio-label input[type='radio'] {
		accent-color: #6366f1;
	}

	.custom-actions {
		display: flex;
		gap: 1rem;
		padding-top: 1rem;
		border-top: 1px solid rgba(99, 102, 241, 0.1);
	}

	.btn-cancel,
	.btn-apply {
		flex: 1;
		padding: 0.75rem 1.5rem;
		border-radius: 8px;
		font-weight: 600;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-cancel {
		background: transparent;
		color: #94a3b8;
		border: 1px solid rgba(99, 102, 241, 0.2);
	}

	.btn-cancel:hover {
		background: rgba(99, 102, 241, 0.05);
		color: #a5b4fc;
	}

	.btn-apply {
		background: linear-gradient(135deg, #6366f1, #8b5cf6);
		color: white;
		border: none;
	}

	.btn-apply:hover {
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(99, 102, 241, 0.3);
	}

	.theme-preview-section h4 {
		font-size: 0.875rem;
		font-weight: 600;
		color: #94a3b8;
		margin: 0 0 0.75rem 0;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.live-preview {
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 10px;
		padding: 2rem;
		min-height: 300px;
	}

	.preview-form-container {
		max-width: 400px;
		margin: 0 auto;
	}

	.preview-form-container h3 {
		margin-bottom: 1.5rem;
	}

	.preview-field-wrapper {
		margin-bottom: 1.25rem;
	}

	.preview-field-wrapper label {
		display: block;
		margin-bottom: 0.5rem;
		font-size: 0.875rem;
		font-weight: 500;
	}

	.preview-field-wrapper input {
		width: 100%;
		padding: 0.75rem;
		border: 1px solid;
		border-radius: 6px;
		font-size: 0.9375rem;
		background: transparent;
	}

	.preview-submit {
		width: 100%;
		padding: 0.875rem;
		border: none;
		border-radius: 8px;
		color: white;
		font-weight: 600;
		font-size: 0.9375rem;
		cursor: pointer;
		margin-top: 0.5rem;
	}

	@media (max-width: 768px) {
		.themes-grid {
			grid-template-columns: repeat(2, 1fr);
		}

		.color-controls {
			grid-template-columns: 1fr;
		}

		.custom-actions {
			flex-direction: column;
		}
	}
</style>

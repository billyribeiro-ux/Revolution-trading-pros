<script lang="ts">
	/**
	 * Template Editor Component
	 *
	 * Full-featured template editor with live preview, color pickers,
	 * typography controls, and more.
	 */
	import { createEventDispatcher, onMount } from 'svelte';
	import type { BannerTemplate, TemplatePosition, ButtonVariant, AnimationType } from './types';
	import { BANNER_TEMPLATES, getTemplate } from './registry';

	export let template: BannerTemplate;
	export let isNew = false;

	const dispatch = createEventDispatcher<{
		save: BannerTemplate;
		cancel: void;
		preview: BannerTemplate;
	}>();

	// Create a working copy
	let editedTemplate: BannerTemplate = JSON.parse(JSON.stringify(template));

	// Editor tabs
	type EditorTab = 'layout' | 'colors' | 'typography' | 'copy' | 'buttons' | 'advanced';
	let activeTab: EditorTab = 'layout';

	// Position options
	const positionOptions: { value: TemplatePosition; label: string }[] = [
		{ value: 'bottom', label: 'Bottom Bar' },
		{ value: 'top', label: 'Top Bar' },
		{ value: 'bottom-left', label: 'Bottom Left' },
		{ value: 'bottom-right', label: 'Bottom Right' },
		{ value: 'top-left', label: 'Top Left' },
		{ value: 'top-right', label: 'Top Right' },
		{ value: 'center', label: 'Center Modal' },
		{ value: 'fullscreen', label: 'Fullscreen' },
	];

	// Button variant options
	const buttonVariantOptions: { value: ButtonVariant; label: string }[] = [
		{ value: 'solid', label: 'Solid' },
		{ value: 'outline', label: 'Outline' },
		{ value: 'ghost', label: 'Ghost' },
		{ value: 'gradient', label: 'Gradient' },
		{ value: 'pill', label: 'Pill' },
		{ value: 'rounded', label: 'Rounded' },
		{ value: 'square', label: 'Square' },
	];

	// Animation options
	const animationOptions: { value: AnimationType; label: string }[] = [
		{ value: 'none', label: 'None' },
		{ value: 'fade', label: 'Fade' },
		{ value: 'slide-up', label: 'Slide Up' },
		{ value: 'slide-down', label: 'Slide Down' },
		{ value: 'slide-left', label: 'Slide Left' },
		{ value: 'slide-right', label: 'Slide Right' },
		{ value: 'scale', label: 'Scale' },
		{ value: 'bounce', label: 'Bounce' },
	];

	// Icon type options
	const iconOptions = [
		{ value: 'cookie', label: 'Cookie' },
		{ value: 'shield', label: 'Shield' },
		{ value: 'lock', label: 'Lock' },
		{ value: 'checkmark', label: 'Checkmark' },
	];

	// Preset color themes
	const colorPresets = [
		{ name: 'Dark', bg: '#0a0a0a', text: '#ffffff', accent: '#3b82f6' },
		{ name: 'Light', bg: '#ffffff', text: '#18181b', accent: '#2563eb' },
		{ name: 'Purple', bg: 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)', text: '#ffffff', accent: '#ffffff' },
		{ name: 'Ocean', bg: 'linear-gradient(135deg, #0ea5e9 0%, #06b6d4 100%)', text: '#ffffff', accent: '#ffffff' },
		{ name: 'Sunset', bg: 'linear-gradient(135deg, #f97316 0%, #ec4899 100%)', text: '#ffffff', accent: '#ffffff' },
		{ name: 'Forest', bg: '#f0fdf4', text: '#14532d', accent: '#22c55e' },
		{ name: 'Neon', bg: '#0d0d0d', text: '#00ff88', accent: '#ff00ff' },
		{ name: 'Gold', bg: '#1a1a1a', text: '#fafafa', accent: '#d4af37' },
	];

	function applyColorPreset(preset: typeof colorPresets[0]) {
		editedTemplate.colors.background = preset.bg;
		editedTemplate.colors.text = preset.text;
		editedTemplate.colors.accent = preset.accent;
		editedTemplate.colors.acceptButton = preset.accent;
		editedTemplate = editedTemplate;
	}

	function handleSave() {
		dispatch('save', editedTemplate);
	}

	function handleCancel() {
		dispatch('cancel');
	}

	function handlePreview() {
		dispatch('preview', editedTemplate);
	}

	// Reset to original
	function handleReset() {
		editedTemplate = JSON.parse(JSON.stringify(template));
	}

	// Copy from another template
	function copyFromTemplate(templateId: string) {
		const source = getTemplate(templateId);
		if (source) {
			const currentName = editedTemplate.name;
			const currentId = editedTemplate.id;
			editedTemplate = JSON.parse(JSON.stringify(source));
			editedTemplate.name = currentName;
			editedTemplate.id = currentId;
			editedTemplate.category = 'custom';
		}
	}
</script>

<div class="template-editor">
	<!-- Header -->
	<div class="editor-header">
		<div class="header-left">
			<h2>{isNew ? 'Create New Template' : `Edit: ${template.name}`}</h2>
			<p class="header-subtitle">Customize every aspect of your consent banner</p>
		</div>
		<div class="header-actions">
			<button class="btn btn-ghost" on:click={handleReset}>Reset</button>
			<button class="btn btn-secondary" on:click={handlePreview}>Preview</button>
			<button class="btn btn-secondary" on:click={handleCancel}>Cancel</button>
			<button class="btn btn-primary" on:click={handleSave}>
				{isNew ? 'Create Template' : 'Save Changes'}
			</button>
		</div>
	</div>

	<!-- Tabs -->
	<div class="editor-tabs">
		<button
			class="tab"
			class:active={activeTab === 'layout'}
			on:click={() => (activeTab = 'layout')}
		>
			Layout
		</button>
		<button
			class="tab"
			class:active={activeTab === 'colors'}
			on:click={() => (activeTab = 'colors')}
		>
			Colors
		</button>
		<button
			class="tab"
			class:active={activeTab === 'typography'}
			on:click={() => (activeTab = 'typography')}
		>
			Typography
		</button>
		<button
			class="tab"
			class:active={activeTab === 'copy'}
			on:click={() => (activeTab = 'copy')}
		>
			Text & Copy
		</button>
		<button
			class="tab"
			class:active={activeTab === 'buttons'}
			on:click={() => (activeTab = 'buttons')}
		>
			Buttons
		</button>
		<button
			class="tab"
			class:active={activeTab === 'advanced'}
			on:click={() => (activeTab = 'advanced')}
		>
			Advanced
		</button>
	</div>

	<!-- Tab Content -->
	<div class="editor-content">
		<!-- Layout Tab -->
		{#if activeTab === 'layout'}
			<div class="tab-content">
				<div class="form-section">
					<h3>Basic Information</h3>
					<div class="form-row">
						<label class="form-label">Template Name</label>
						<input
							type="text"
							class="form-input"
							bind:value={editedTemplate.name}
							placeholder="My Custom Template"
						/>
					</div>
					<div class="form-row">
						<label class="form-label">Description</label>
						<textarea
							class="form-input"
							bind:value={editedTemplate.description}
							placeholder="A brief description of this template..."
							rows="2"
						></textarea>
					</div>
				</div>

				<div class="form-section">
					<h3>Position & Size</h3>
					<div class="form-grid">
						<div class="form-row">
							<label class="form-label">Position</label>
							<select class="form-select" bind:value={editedTemplate.position}>
								{#each positionOptions as opt}
									<option value={opt.value}>{opt.label}</option>
								{/each}
							</select>
						</div>
						<div class="form-row">
							<label class="form-label">Max Width</label>
							<input
								type="text"
								class="form-input"
								bind:value={editedTemplate.maxWidth}
								placeholder="100%"
							/>
						</div>
					</div>
				</div>

				<div class="form-section">
					<h3>Features</h3>
					<div class="checkbox-grid">
						<label class="checkbox-label">
							<input type="checkbox" bind:checked={editedTemplate.showIcon} />
							<span>Show Icon</span>
						</label>
						<label class="checkbox-label">
							<input type="checkbox" bind:checked={editedTemplate.showCloseButton} />
							<span>Show Close Button</span>
						</label>
						<label class="checkbox-label">
							<input type="checkbox" bind:checked={editedTemplate.showPrivacyLink} />
							<span>Show Privacy Link</span>
						</label>
						<label class="checkbox-label">
							<input type="checkbox" bind:checked={editedTemplate.showRejectButton} />
							<span>Show Reject Button</span>
						</label>
						<label class="checkbox-label">
							<input type="checkbox" bind:checked={editedTemplate.showCustomizeButton} />
							<span>Show Customize Button</span>
						</label>
					</div>

					{#if editedTemplate.showIcon}
						<div class="form-row" style="margin-top: 1rem;">
							<label class="form-label">Icon Type</label>
							<select class="form-select" bind:value={editedTemplate.iconType}>
								{#each iconOptions as opt}
									<option value={opt.value}>{opt.label}</option>
								{/each}
							</select>
						</div>
					{/if}
				</div>

				<div class="form-section">
					<h3>Animation</h3>
					<div class="form-grid">
						<div class="form-row">
							<label class="form-label">Animation Type</label>
							<select class="form-select" bind:value={editedTemplate.animation}>
								{#each animationOptions as opt}
									<option value={opt.value}>{opt.label}</option>
								{/each}
							</select>
						</div>
						<div class="form-row">
							<label class="form-label">Duration (ms)</label>
							<input
								type="number"
								class="form-input"
								bind:value={editedTemplate.animationDuration}
								min="0"
								max="2000"
								step="50"
							/>
						</div>
					</div>
				</div>
			</div>
		{/if}

		<!-- Colors Tab -->
		{#if activeTab === 'colors'}
			<div class="tab-content">
				<div class="form-section">
					<h3>Color Presets</h3>
					<div class="color-presets">
						{#each colorPresets as preset}
							<button
								class="preset-btn"
								style="background: {preset.bg}; color: {preset.text}"
								on:click={() => applyColorPreset(preset)}
							>
								{preset.name}
							</button>
						{/each}
					</div>
				</div>

				<div class="form-section">
					<h3>Background & Text</h3>
					<div class="form-grid">
						<div class="form-row">
							<label class="form-label">Background</label>
							<input
								type="text"
								class="form-input"
								bind:value={editedTemplate.colors.background}
								placeholder="#000000 or gradient"
							/>
						</div>
						<div class="form-row">
							<label class="form-label">Text Color</label>
							<div class="color-input-wrapper">
								<input
									type="color"
									class="color-picker"
									bind:value={editedTemplate.colors.text}
								/>
								<input
									type="text"
									class="form-input"
									bind:value={editedTemplate.colors.text}
								/>
							</div>
						</div>
						<div class="form-row">
							<label class="form-label">Muted Text</label>
							<div class="color-input-wrapper">
								<input
									type="color"
									class="color-picker"
									bind:value={editedTemplate.colors.textMuted}
								/>
								<input
									type="text"
									class="form-input"
									bind:value={editedTemplate.colors.textMuted}
								/>
							</div>
						</div>
						<div class="form-row">
							<label class="form-label">Accent Color</label>
							<div class="color-input-wrapper">
								<input
									type="color"
									class="color-picker"
									bind:value={editedTemplate.colors.accent}
								/>
								<input
									type="text"
									class="form-input"
									bind:value={editedTemplate.colors.accent}
								/>
							</div>
						</div>
					</div>
				</div>

				<div class="form-section">
					<h3>Button Colors</h3>
					<div class="form-grid">
						<div class="form-row">
							<label class="form-label">Accept Button BG</label>
							<input
								type="text"
								class="form-input"
								bind:value={editedTemplate.colors.acceptButton}
							/>
						</div>
						<div class="form-row">
							<label class="form-label">Accept Button Text</label>
							<div class="color-input-wrapper">
								<input
									type="color"
									class="color-picker"
									bind:value={editedTemplate.colors.acceptButtonText}
								/>
								<input
									type="text"
									class="form-input"
									bind:value={editedTemplate.colors.acceptButtonText}
								/>
							</div>
						</div>
						<div class="form-row">
							<label class="form-label">Reject Button BG</label>
							<input
								type="text"
								class="form-input"
								bind:value={editedTemplate.colors.rejectButton}
							/>
						</div>
						<div class="form-row">
							<label class="form-label">Reject Button Text</label>
							<div class="color-input-wrapper">
								<input
									type="color"
									class="color-picker"
									bind:value={editedTemplate.colors.rejectButtonText}
								/>
								<input
									type="text"
									class="form-input"
									bind:value={editedTemplate.colors.rejectButtonText}
								/>
							</div>
						</div>
					</div>
				</div>
			</div>
		{/if}

		<!-- Typography Tab -->
		{#if activeTab === 'typography'}
			<div class="tab-content">
				<div class="form-section">
					<h3>Font Settings</h3>
					<div class="form-row">
						<label class="form-label">Font Family</label>
						<input
							type="text"
							class="form-input"
							bind:value={editedTemplate.typography.fontFamily}
							placeholder="system-ui, sans-serif"
						/>
					</div>
					<div class="form-row">
						<label class="form-label">Line Height</label>
						<input
							type="text"
							class="form-input"
							bind:value={editedTemplate.typography.lineHeight}
							placeholder="1.5"
						/>
					</div>
				</div>

				<div class="form-section">
					<h3>Title</h3>
					<div class="form-grid">
						<div class="form-row">
							<label class="form-label">Size</label>
							<input
								type="text"
								class="form-input"
								bind:value={editedTemplate.typography.titleSize}
								placeholder="1.125rem"
							/>
						</div>
						<div class="form-row">
							<label class="form-label">Weight</label>
							<select class="form-select" bind:value={editedTemplate.typography.titleWeight}>
								<option value="400">Normal (400)</option>
								<option value="500">Medium (500)</option>
								<option value="600">Semibold (600)</option>
								<option value="700">Bold (700)</option>
							</select>
						</div>
					</div>
				</div>

				<div class="form-section">
					<h3>Description</h3>
					<div class="form-row">
						<label class="form-label">Size</label>
						<input
							type="text"
							class="form-input"
							bind:value={editedTemplate.typography.descriptionSize}
							placeholder="0.875rem"
						/>
					</div>
				</div>

				<div class="form-section">
					<h3>Buttons</h3>
					<div class="form-grid">
						<div class="form-row">
							<label class="form-label">Size</label>
							<input
								type="text"
								class="form-input"
								bind:value={editedTemplate.typography.buttonSize}
								placeholder="0.875rem"
							/>
						</div>
						<div class="form-row">
							<label class="form-label">Weight</label>
							<select class="form-select" bind:value={editedTemplate.typography.buttonWeight}>
								<option value="400">Normal (400)</option>
								<option value="500">Medium (500)</option>
								<option value="600">Semibold (600)</option>
								<option value="700">Bold (700)</option>
							</select>
						</div>
					</div>
				</div>
			</div>
		{/if}

		<!-- Copy Tab -->
		{#if activeTab === 'copy'}
			<div class="tab-content">
				<div class="form-section">
					<h3>Banner Text</h3>
					<div class="form-row">
						<label class="form-label">Title</label>
						<input
							type="text"
							class="form-input"
							bind:value={editedTemplate.copy.title}
							placeholder="We value your privacy"
						/>
					</div>
					<div class="form-row">
						<label class="form-label">Description</label>
						<textarea
							class="form-input"
							bind:value={editedTemplate.copy.description}
							rows="3"
							placeholder="We use cookies to enhance your browsing experience..."
						></textarea>
					</div>
				</div>

				<div class="form-section">
					<h3>Button Labels</h3>
					<div class="form-grid">
						<div class="form-row">
							<label class="form-label">Accept All</label>
							<input
								type="text"
								class="form-input"
								bind:value={editedTemplate.copy.acceptAll}
								placeholder="Accept All"
							/>
						</div>
						<div class="form-row">
							<label class="form-label">Reject All</label>
							<input
								type="text"
								class="form-input"
								bind:value={editedTemplate.copy.rejectAll}
								placeholder="Reject All"
							/>
						</div>
						<div class="form-row">
							<label class="form-label">Customize</label>
							<input
								type="text"
								class="form-input"
								bind:value={editedTemplate.copy.customize}
								placeholder="Customize"
							/>
						</div>
					</div>
				</div>

				<div class="form-section">
					<h3>Links</h3>
					<div class="form-grid">
						<div class="form-row">
							<label class="form-label">Privacy Policy Text</label>
							<input
								type="text"
								class="form-input"
								bind:value={editedTemplate.copy.privacyPolicy}
								placeholder="Privacy Policy"
							/>
						</div>
						<div class="form-row">
							<label class="form-label">Cookie Policy Text</label>
							<input
								type="text"
								class="form-input"
								bind:value={editedTemplate.copy.cookiePolicy}
								placeholder="Cookie Policy"
							/>
						</div>
					</div>
				</div>
			</div>
		{/if}

		<!-- Buttons Tab -->
		{#if activeTab === 'buttons'}
			<div class="tab-content">
				<div class="form-section">
					<h3>Button Style</h3>
					<div class="form-row">
						<label class="form-label">Variant</label>
						<select class="form-select" bind:value={editedTemplate.buttonVariant}>
							{#each buttonVariantOptions as opt}
								<option value={opt.value}>{opt.label}</option>
							{/each}
						</select>
					</div>
				</div>

				<div class="form-section">
					<h3>Button Spacing</h3>
					<div class="form-grid">
						<div class="form-row">
							<label class="form-label">Padding</label>
							<input
								type="text"
								class="form-input"
								bind:value={editedTemplate.spacing.buttonPadding}
								placeholder="0.75rem 1.5rem"
							/>
						</div>
						<div class="form-row">
							<label class="form-label">Border Radius</label>
							<input
								type="text"
								class="form-input"
								bind:value={editedTemplate.spacing.buttonBorderRadius}
								placeholder="8px"
							/>
						</div>
					</div>
				</div>

				<div class="form-section">
					<h3>Mobile Settings</h3>
					<div class="checkbox-grid">
						<label class="checkbox-label">
							<input type="checkbox" bind:checked={editedTemplate.mobile.stackButtons} />
							<span>Stack Buttons Vertically</span>
						</label>
						<label class="checkbox-label">
							<input type="checkbox" bind:checked={editedTemplate.mobile.fullWidthButtons} />
							<span>Full Width Buttons</span>
						</label>
						<label class="checkbox-label">
							<input type="checkbox" bind:checked={editedTemplate.mobile.useDrawer} />
							<span>Use Drawer on Mobile</span>
						</label>
						<label class="checkbox-label">
							<input type="checkbox" bind:checked={editedTemplate.mobile.compact} />
							<span>Compact Mode</span>
						</label>
					</div>
				</div>
			</div>
		{/if}

		<!-- Advanced Tab -->
		{#if activeTab === 'advanced'}
			<div class="tab-content">
				<div class="form-section">
					<h3>Effects</h3>
					<div class="form-grid">
						<div class="form-row">
							<label class="form-label">Box Shadow</label>
							<input
								type="text"
								class="form-input"
								bind:value={editedTemplate.boxShadow}
								placeholder="0 4px 20px rgba(0,0,0,0.3)"
							/>
						</div>
						<div class="form-row">
							<label class="form-label">Border</label>
							<input
								type="text"
								class="form-input"
								bind:value={editedTemplate.border}
								placeholder="1px solid rgba(255,255,255,0.1)"
							/>
						</div>
						<div class="form-row">
							<label class="form-label">Backdrop Blur</label>
							<input
								type="text"
								class="form-input"
								bind:value={editedTemplate.backdropBlur}
								placeholder="20px"
							/>
						</div>
						<div class="form-row">
							<label class="form-label">Border Radius</label>
							<input
								type="text"
								class="form-input"
								bind:value={editedTemplate.spacing.borderRadius}
								placeholder="12px"
							/>
						</div>
					</div>
				</div>

				<div class="form-section">
					<h3>Spacing</h3>
					<div class="form-grid">
						<div class="form-row">
							<label class="form-label">Padding</label>
							<input
								type="text"
								class="form-input"
								bind:value={editedTemplate.spacing.padding}
								placeholder="1.5rem"
							/>
						</div>
						<div class="form-row">
							<label class="form-label">Gap</label>
							<input
								type="text"
								class="form-input"
								bind:value={editedTemplate.spacing.gap}
								placeholder="1rem"
							/>
						</div>
					</div>
				</div>

				<div class="form-section">
					<h3>Custom CSS</h3>
					<div class="form-row">
						<label class="form-label">Additional Styles</label>
						<textarea
							class="form-input code"
							bind:value={editedTemplate.customCSS}
							rows="6"
							placeholder="/* Add custom CSS here */"
						></textarea>
					</div>
				</div>

				<div class="form-section">
					<h3>Copy From Template</h3>
					<div class="form-row">
						<label class="form-label">Base on existing template</label>
						<select
							class="form-select"
							on:change={(e) => copyFromTemplate(e.currentTarget.value)}
						>
							<option value="">Select a template...</option>
							{#each BANNER_TEMPLATES as t}
								<option value={t.id}>{t.name}</option>
							{/each}
						</select>
					</div>
				</div>
			</div>
		{/if}
	</div>
</div>

<style>
	.template-editor {
		background: #0a101c;
		border-radius: 12px;
		overflow: hidden;
	}

	.editor-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		padding: 1.5rem;
		background: rgba(255, 255, 255, 0.03);
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
	}

	.editor-header h2 {
		margin: 0;
		font-size: 1.25rem;
		color: #f1f5f9;
	}

	.header-subtitle {
		margin: 0.25rem 0 0;
		font-size: 0.875rem;
		color: #64748b;
	}

	.header-actions {
		display: flex;
		gap: 0.5rem;
	}

	.btn {
		display: inline-flex;
		align-items: center;
		padding: 0.5rem 1rem;
		border-radius: 8px;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
		border: none;
	}

	.btn-primary {
		background: linear-gradient(135deg, #0ea5e9, #06b6d4);
		color: white;
	}

	.btn-secondary {
		background: rgba(255, 255, 255, 0.1);
		color: white;
	}

	.btn-ghost {
		background: transparent;
		color: #94a3b8;
	}

	.btn:hover {
		filter: brightness(1.1);
	}

	.editor-tabs {
		display: flex;
		gap: 0.25rem;
		padding: 0 1.5rem;
		background: rgba(255, 255, 255, 0.02);
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
		overflow-x: auto;
	}

	.tab {
		padding: 1rem 1.25rem;
		background: transparent;
		border: none;
		color: #94a3b8;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		border-bottom: 2px solid transparent;
		transition: all 0.2s;
		white-space: nowrap;
	}

	.tab:hover {
		color: #e2e8f0;
	}

	.tab.active {
		color: #0ea5e9;
		border-bottom-color: #0ea5e9;
	}

	.editor-content {
		max-height: 60vh;
		overflow-y: auto;
	}

	.tab-content {
		padding: 1.5rem;
	}

	.form-section {
		margin-bottom: 2rem;
	}

	.form-section:last-child {
		margin-bottom: 0;
	}

	.form-section h3 {
		margin: 0 0 1rem;
		font-size: 0.9375rem;
		font-weight: 600;
		color: #e2e8f0;
	}

	.form-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 1rem;
	}

	@media (max-width: 640px) {
		.form-grid {
			grid-template-columns: 1fr;
		}
	}

	.form-row {
		display: flex;
		flex-direction: column;
		gap: 0.375rem;
	}

	.form-label {
		font-size: 0.75rem;
		font-weight: 500;
		color: #94a3b8;
		text-transform: uppercase;
	}

	.form-input,
	.form-select {
		padding: 0.625rem 0.875rem;
		background: rgba(255, 255, 255, 0.05);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 8px;
		color: #f1f5f9;
		font-size: 0.875rem;
		transition: all 0.2s;
	}

	.form-input:focus,
	.form-select:focus {
		outline: none;
		border-color: #0ea5e9;
		box-shadow: 0 0 0 2px rgba(14, 165, 233, 0.2);
	}

	.form-input.code {
		font-family: 'JetBrains Mono', 'Fira Code', monospace;
		font-size: 0.8125rem;
	}

	textarea.form-input {
		resize: vertical;
		min-height: 80px;
	}

	.checkbox-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
		gap: 0.75rem;
	}

	.checkbox-label {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		cursor: pointer;
		font-size: 0.875rem;
		color: #e2e8f0;
	}

	.checkbox-label input {
		width: 16px;
		height: 16px;
		accent-color: #0ea5e9;
	}

	.color-input-wrapper {
		display: flex;
		gap: 0.5rem;
	}

	.color-picker {
		width: 40px;
		height: 38px;
		padding: 2px;
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 6px;
		cursor: pointer;
	}

	.color-input-wrapper .form-input {
		flex: 1;
	}

	.color-presets {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
	}

	.preset-btn {
		padding: 0.5rem 1rem;
		border: 1px solid rgba(255, 255, 255, 0.2);
		border-radius: 8px;
		font-size: 0.75rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.preset-btn:hover {
		transform: translateY(-2px);
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
	}
</style>

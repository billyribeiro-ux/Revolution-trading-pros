<script lang="ts">
	/**
	 * Consent Banner Designer Component
	 *
	 * Visual designer for customizing consent banner appearance.
	 * Mirrors Consent Magic Pro WordPress plugin design options.
	 *
	 * Features:
	 * - Live preview
	 * - Color picker for all elements
	 * - Typography controls
	 * - Spacing controls
	 * - Animation settings
	 * - Layout selection
	 *
	 * @version 1.0.0 - December 2025
	 */

	interface DesignConfig {
		// Layout
		layout_type: 'bar' | 'popup' | 'floating' | 'drawer';
		position: 'top' | 'bottom' | 'center';
		position_horizontal: 'left' | 'center' | 'right';

		// Colors
		background_color: string;
		text_color: string;
		link_color: string;
		title_color: string;
		border_color: string;
		border_style: string;
		border_width: number;

		// Accept Button
		accept_btn_bg: string;
		accept_btn_text: string;
		accept_btn_hover_bg: string;

		// Reject Button
		reject_btn_bg: string;
		reject_btn_text: string;
		reject_btn_hover_bg: string;

		// Settings Button
		settings_btn_bg: string;
		settings_btn_text: string;
		settings_btn_border: string;

		// Toggle
		toggle_active_color: string;
		toggle_inactive_color: string;

		// Typography
		font_family: string;
		title_font_size: number;
		title_font_weight: number;
		body_font_size: number;
		body_font_weight: number;
		btn_font_size: number;
		btn_font_weight: number;

		// Spacing
		padding_top: number;
		padding_bottom: number;
		padding_left: number;
		padding_right: number;
		btn_padding_x: number;
		btn_padding_y: number;
		btn_margin: number;
		btn_border_radius: number;
		container_border_radius: number;
		container_max_width: number;

		// Animation
		animation_type: 'slide' | 'fade' | 'none';
		animation_duration: number;

		// Content
		title: string;
		message_text: string;
		accept_btn_label: string;
		reject_btn_label: string;
		settings_btn_label: string;
		privacy_link_text: string;
		privacy_link_url: string;

		// Behavior
		show_reject_btn: boolean;
		show_settings_btn: boolean;
		show_privacy_link: boolean;
		show_close_btn: boolean;

		// Logo
		logo_url: string;
		logo_size: number;
		logo_position: 'left' | 'center' | 'right';
	}

	interface Props {
		config: DesignConfig;
		onSave?: (config: DesignConfig) => void;
		onCancel?: () => void;
	}

	let { config = $bindable(), onSave, onCancel }: Props = $props();

	// Default config
	const defaultConfig: DesignConfig = {
		layout_type: 'bar',
		position: 'bottom',
		position_horizontal: 'center',
		background_color: '#1a1f2e',
		text_color: '#ffffff',
		link_color: '#3b82f6',
		title_color: '#ffffff',
		border_color: '#333333',
		border_style: 'solid',
		border_width: 0,
		accept_btn_bg: '#3b82f6',
		accept_btn_text: '#ffffff',
		accept_btn_hover_bg: '#2563eb',
		reject_btn_bg: '#374151',
		reject_btn_text: '#ffffff',
		reject_btn_hover_bg: '#4b5563',
		settings_btn_bg: 'transparent',
		settings_btn_text: '#3b82f6',
		settings_btn_border: '#3b82f6',
		toggle_active_color: '#3b82f6',
		toggle_inactive_color: '#6b7280',
		font_family: 'system-ui, -apple-system, sans-serif',
		title_font_size: 18,
		title_font_weight: 600,
		body_font_size: 14,
		body_font_weight: 400,
		btn_font_size: 14,
		btn_font_weight: 500,
		padding_top: 20,
		padding_bottom: 20,
		padding_left: 24,
		padding_right: 24,
		btn_padding_x: 20,
		btn_padding_y: 12,
		btn_margin: 8,
		btn_border_radius: 8,
		container_border_radius: 0,
		container_max_width: 1200,
		animation_type: 'slide',
		animation_duration: 300,
		title: 'We value your privacy',
		message_text:
			'We use cookies and similar technologies to enhance your experience, analyze site traffic, and for marketing purposes.',
		accept_btn_label: 'Accept All',
		reject_btn_label: 'Reject All',
		settings_btn_label: 'Manage Preferences',
		privacy_link_text: 'Privacy Policy',
		privacy_link_url: '/privacy',
		show_reject_btn: true,
		show_settings_btn: true,
		show_privacy_link: true,
		show_close_btn: false,
		logo_url: '',
		logo_size: 40,
		logo_position: 'left',
	};

	// Ensure config has all properties
	config = { ...defaultConfig, ...config };

	// Active design tab
	let activeTab: 'layout' | 'colors' | 'typography' | 'spacing' | 'content' | 'behavior' =
		'layout';

	// Font options
	const fontOptions = [
		{ value: 'system-ui, -apple-system, sans-serif', label: 'System Default' },
		{ value: 'Inter, sans-serif', label: 'Inter' },
		{ value: 'Roboto, sans-serif', label: 'Roboto' },
		{ value: 'Open Sans, sans-serif', label: 'Open Sans' },
		{ value: '"Helvetica Neue", Helvetica, Arial, sans-serif', label: 'Helvetica' },
		{ value: 'Georgia, serif', label: 'Georgia' },
	];

	// Weight options
	const weightOptions = [300, 400, 500, 600, 700, 800];

	// Generate CSS variables
	function getCssVars(): string {
		return `
			--cb-bg: ${config.background_color};
			--cb-text: ${config.text_color};
			--cb-link: ${config.link_color};
			--cb-title: ${config.title_color};
			--cb-border: ${config.border_color};
			--cb-accept-bg: ${config.accept_btn_bg};
			--cb-accept-text: ${config.accept_btn_text};
			--cb-reject-bg: ${config.reject_btn_bg};
			--cb-reject-text: ${config.reject_btn_text};
			--cb-settings-bg: ${config.settings_btn_bg};
			--cb-settings-text: ${config.settings_btn_text};
			--cb-settings-border: ${config.settings_btn_border};
			--cb-font: ${config.font_family};
			--cb-title-size: ${config.title_font_size}px;
			--cb-title-weight: ${config.title_font_weight};
			--cb-body-size: ${config.body_font_size}px;
			--cb-body-weight: ${config.body_font_weight};
			--cb-btn-size: ${config.btn_font_size}px;
			--cb-btn-weight: ${config.btn_font_weight};
			--cb-padding-top: ${config.padding_top}px;
			--cb-padding-bottom: ${config.padding_bottom}px;
			--cb-padding-left: ${config.padding_left}px;
			--cb-padding-right: ${config.padding_right}px;
			--cb-btn-px: ${config.btn_padding_x}px;
			--cb-btn-py: ${config.btn_padding_y}px;
			--cb-btn-margin: ${config.btn_margin}px;
			--cb-btn-radius: ${config.btn_border_radius}px;
			--cb-radius: ${config.container_border_radius}px;
			--cb-max-width: ${config.container_max_width}px;
			--cb-border-width: ${config.border_width}px;
			--cb-border-style: ${config.border_style};
		`;
	}

	function handleSave() {
		onSave?.(config);
	}

	function handleReset() {
		config = { ...defaultConfig };
	}
</script>

<div class="designer">
	<!-- Toolbar -->
	<div class="designer-toolbar">
		<div class="toolbar-tabs">
			<button class:active={activeTab === 'layout'} onclick={() => (activeTab = 'layout')}>
				Layout
			</button>
			<button class:active={activeTab === 'colors'} onclick={() => (activeTab = 'colors')}>
				Colors
			</button>
			<button class:active={activeTab === 'typography'} onclick={() => (activeTab = 'typography')}>
				Typography
			</button>
			<button class:active={activeTab === 'spacing'} onclick={() => (activeTab = 'spacing')}>
				Spacing
			</button>
			<button class:active={activeTab === 'content'} onclick={() => (activeTab = 'content')}>
				Content
			</button>
			<button class:active={activeTab === 'behavior'} onclick={() => (activeTab = 'behavior')}>
				Behavior
			</button>
		</div>
		<div class="toolbar-actions">
			<button class="btn btn-ghost" onclick={handleReset}>Reset</button>
			{#if onCancel}
				<button class="btn btn-secondary" onclick={onCancel}>Cancel</button>
			{/if}
			<button class="btn btn-primary" onclick={handleSave}>Save Design</button>
		</div>
	</div>

	<div class="designer-content">
		<!-- Controls Panel -->
		<div class="controls-panel">
			{#if activeTab === 'layout'}
				<div class="control-group">
					<label class="control-label">Banner Type</label>
					<div class="layout-options">
						<label class="layout-option" class:selected={config.layout_type === 'bar'}>
							<input type="radio" bind:group={config.layout_type} value="bar" />
							<div class="layout-icon bar"></div>
							<span>Bar</span>
						</label>
						<label class="layout-option" class:selected={config.layout_type === 'popup'}>
							<input type="radio" bind:group={config.layout_type} value="popup" />
							<div class="layout-icon popup"></div>
							<span>Popup</span>
						</label>
						<label class="layout-option" class:selected={config.layout_type === 'floating'}>
							<input type="radio" bind:group={config.layout_type} value="floating" />
							<div class="layout-icon floating"></div>
							<span>Floating</span>
						</label>
						<label class="layout-option" class:selected={config.layout_type === 'drawer'}>
							<input type="radio" bind:group={config.layout_type} value="drawer" />
							<div class="layout-icon drawer"></div>
							<span>Drawer</span>
						</label>
					</div>
				</div>

				<div class="control-group">
					<label class="control-label">Position</label>
					<select class="select" bind:value={config.position}>
						<option value="top">Top</option>
						<option value="bottom">Bottom</option>
						<option value="center">Center (Modal)</option>
					</select>
				</div>

				{#if config.layout_type === 'floating'}
					<div class="control-group">
						<label class="control-label">Horizontal Position</label>
						<select class="select" bind:value={config.position_horizontal}>
							<option value="left">Left</option>
							<option value="center">Center</option>
							<option value="right">Right</option>
						</select>
					</div>
				{/if}

				<div class="control-group">
					<label class="control-label">Max Width (px)</label>
					<input
						type="number"
						class="input"
						bind:value={config.container_max_width}
						min="300"
						max="2000"
					/>
				</div>

				<div class="control-group">
					<label class="control-label">Border Radius (px)</label>
					<input
						type="range"
						bind:value={config.container_border_radius}
						min="0"
						max="24"
					/>
					<span class="range-value">{config.container_border_radius}px</span>
				</div>

				<div class="control-group">
					<label class="control-label">Animation</label>
					<select class="select" bind:value={config.animation_type}>
						<option value="slide">Slide</option>
						<option value="fade">Fade</option>
						<option value="none">None</option>
					</select>
				</div>
			{/if}

			{#if activeTab === 'colors'}
				<div class="color-section">
					<h4>Background & Text</h4>
					<div class="control-group">
						<label class="control-label">Background</label>
						<div class="color-input">
							<input type="color" bind:value={config.background_color} />
							<input type="text" bind:value={config.background_color} />
						</div>
					</div>
					<div class="control-group">
						<label class="control-label">Text Color</label>
						<div class="color-input">
							<input type="color" bind:value={config.text_color} />
							<input type="text" bind:value={config.text_color} />
						</div>
					</div>
					<div class="control-group">
						<label class="control-label">Title Color</label>
						<div class="color-input">
							<input type="color" bind:value={config.title_color} />
							<input type="text" bind:value={config.title_color} />
						</div>
					</div>
					<div class="control-group">
						<label class="control-label">Link Color</label>
						<div class="color-input">
							<input type="color" bind:value={config.link_color} />
							<input type="text" bind:value={config.link_color} />
						</div>
					</div>
				</div>

				<div class="color-section">
					<h4>Accept Button</h4>
					<div class="control-group">
						<label class="control-label">Background</label>
						<div class="color-input">
							<input type="color" bind:value={config.accept_btn_bg} />
							<input type="text" bind:value={config.accept_btn_bg} />
						</div>
					</div>
					<div class="control-group">
						<label class="control-label">Text</label>
						<div class="color-input">
							<input type="color" bind:value={config.accept_btn_text} />
							<input type="text" bind:value={config.accept_btn_text} />
						</div>
					</div>
				</div>

				<div class="color-section">
					<h4>Reject Button</h4>
					<div class="control-group">
						<label class="control-label">Background</label>
						<div class="color-input">
							<input type="color" bind:value={config.reject_btn_bg} />
							<input type="text" bind:value={config.reject_btn_bg} />
						</div>
					</div>
					<div class="control-group">
						<label class="control-label">Text</label>
						<div class="color-input">
							<input type="color" bind:value={config.reject_btn_text} />
							<input type="text" bind:value={config.reject_btn_text} />
						</div>
					</div>
				</div>

				<div class="color-section">
					<h4>Settings Button</h4>
					<div class="control-group">
						<label class="control-label">Text/Border</label>
						<div class="color-input">
							<input type="color" bind:value={config.settings_btn_text} />
							<input type="text" bind:value={config.settings_btn_text} />
						</div>
					</div>
				</div>
			{/if}

			{#if activeTab === 'typography'}
				<div class="control-group">
					<label class="control-label">Font Family</label>
					<select class="select" bind:value={config.font_family}>
						{#each fontOptions as font}
							<option value={font.value}>{font.label}</option>
						{/each}
					</select>
				</div>

				<div class="typography-section">
					<h4>Title</h4>
					<div class="control-row">
						<div class="control-group">
							<label class="control-label">Size (px)</label>
							<input type="number" class="input" bind:value={config.title_font_size} min="12" max="48" />
						</div>
						<div class="control-group">
							<label class="control-label">Weight</label>
							<select class="select" bind:value={config.title_font_weight}>
								{#each weightOptions as weight}
									<option value={weight}>{weight}</option>
								{/each}
							</select>
						</div>
					</div>
				</div>

				<div class="typography-section">
					<h4>Body Text</h4>
					<div class="control-row">
						<div class="control-group">
							<label class="control-label">Size (px)</label>
							<input type="number" class="input" bind:value={config.body_font_size} min="10" max="24" />
						</div>
						<div class="control-group">
							<label class="control-label">Weight</label>
							<select class="select" bind:value={config.body_font_weight}>
								{#each weightOptions as weight}
									<option value={weight}>{weight}</option>
								{/each}
							</select>
						</div>
					</div>
				</div>

				<div class="typography-section">
					<h4>Buttons</h4>
					<div class="control-row">
						<div class="control-group">
							<label class="control-label">Size (px)</label>
							<input type="number" class="input" bind:value={config.btn_font_size} min="10" max="20" />
						</div>
						<div class="control-group">
							<label class="control-label">Weight</label>
							<select class="select" bind:value={config.btn_font_weight}>
								{#each weightOptions as weight}
									<option value={weight}>{weight}</option>
								{/each}
							</select>
						</div>
					</div>
				</div>
			{/if}

			{#if activeTab === 'spacing'}
				<div class="control-group">
					<label class="control-label">Container Padding</label>
					<div class="spacing-grid">
						<div>
							<label>Top</label>
							<input type="number" bind:value={config.padding_top} min="0" max="100" />
						</div>
						<div>
							<label>Bottom</label>
							<input type="number" bind:value={config.padding_bottom} min="0" max="100" />
						</div>
						<div>
							<label>Left</label>
							<input type="number" bind:value={config.padding_left} min="0" max="100" />
						</div>
						<div>
							<label>Right</label>
							<input type="number" bind:value={config.padding_right} min="0" max="100" />
						</div>
					</div>
				</div>

				<div class="control-group">
					<label class="control-label">Button Padding</label>
					<div class="control-row">
						<div>
							<label>Horizontal</label>
							<input type="number" bind:value={config.btn_padding_x} min="0" max="60" />
						</div>
						<div>
							<label>Vertical</label>
							<input type="number" bind:value={config.btn_padding_y} min="0" max="40" />
						</div>
					</div>
				</div>

				<div class="control-group">
					<label class="control-label">Button Margin (px)</label>
					<input type="range" bind:value={config.btn_margin} min="0" max="20" />
					<span class="range-value">{config.btn_margin}px</span>
				</div>

				<div class="control-group">
					<label class="control-label">Button Border Radius (px)</label>
					<input type="range" bind:value={config.btn_border_radius} min="0" max="24" />
					<span class="range-value">{config.btn_border_radius}px</span>
				</div>
			{/if}

			{#if activeTab === 'content'}
				<div class="control-group">
					<label class="control-label">Title</label>
					<input type="text" class="input" bind:value={config.title} />
				</div>

				<div class="control-group">
					<label class="control-label">Description</label>
					<textarea class="textarea" bind:value={config.message_text} rows="3"></textarea>
				</div>

				<div class="control-group">
					<label class="control-label">Accept Button Text</label>
					<input type="text" class="input" bind:value={config.accept_btn_label} />
				</div>

				<div class="control-group">
					<label class="control-label">Reject Button Text</label>
					<input type="text" class="input" bind:value={config.reject_btn_label} />
				</div>

				<div class="control-group">
					<label class="control-label">Settings Button Text</label>
					<input type="text" class="input" bind:value={config.settings_btn_label} />
				</div>

				<div class="control-group">
					<label class="control-label">Privacy Link Text</label>
					<input type="text" class="input" bind:value={config.privacy_link_text} />
				</div>

				<div class="control-group">
					<label class="control-label">Privacy Link URL</label>
					<input type="text" class="input" bind:value={config.privacy_link_url} placeholder="/privacy" />
				</div>
			{/if}

			{#if activeTab === 'behavior'}
				<div class="control-group toggle-control">
					<label class="control-label">Show Reject Button</label>
					<input type="checkbox" bind:checked={config.show_reject_btn} />
				</div>

				<div class="control-group toggle-control">
					<label class="control-label">Show Settings Button</label>
					<input type="checkbox" bind:checked={config.show_settings_btn} />
				</div>

				<div class="control-group toggle-control">
					<label class="control-label">Show Privacy Link</label>
					<input type="checkbox" bind:checked={config.show_privacy_link} />
				</div>

				<div class="control-group toggle-control">
					<label class="control-label">Show Close Button</label>
					<input type="checkbox" bind:checked={config.show_close_btn} />
				</div>
			{/if}
		</div>

		<!-- Preview Panel -->
		<div class="preview-panel">
			<div class="preview-header">
				<span>Live Preview</span>
			</div>
			<div class="preview-container">
				<div
					class="banner-preview"
					class:bar={config.layout_type === 'bar'}
					class:popup={config.layout_type === 'popup'}
					class:floating={config.layout_type === 'floating'}
					class:top={config.position === 'top'}
					class:bottom={config.position === 'bottom'}
					class:center={config.position === 'center'}
					style={getCssVars()}
				>
					<div class="preview-banner">
						<div class="preview-text">
							<h3>{config.title}</h3>
							<p>{config.message_text}</p>
							{#if config.show_privacy_link}
								<a href={config.privacy_link_url}>{config.privacy_link_text}</a>
							{/if}
						</div>
						<div class="preview-actions">
							{#if config.show_settings_btn}
								<button class="preview-btn settings">{config.settings_btn_label}</button>
							{/if}
							{#if config.show_reject_btn}
								<button class="preview-btn reject">{config.reject_btn_label}</button>
							{/if}
							<button class="preview-btn accept">{config.accept_btn_label}</button>
						</div>
					</div>
				</div>
			</div>
		</div>
	</div>
</div>

<style>
	.designer {
		background: #0f172a;
		border-radius: 12px;
		overflow: hidden;
	}

	.designer-toolbar {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 0.75rem 1rem;
		background: #1e293b;
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
	}

	.toolbar-tabs {
		display: flex;
		gap: 0.25rem;
	}

	.toolbar-tabs button {
		padding: 0.5rem 1rem;
		background: transparent;
		border: none;
		color: #64748b;
		font-size: 0.8125rem;
		font-weight: 500;
		cursor: pointer;
		border-radius: 6px;
		transition: all 0.2s;
	}

	.toolbar-tabs button:hover {
		background: rgba(255, 255, 255, 0.05);
		color: #e2e8f0;
	}

	.toolbar-tabs button.active {
		background: rgba(14, 165, 233, 0.15);
		color: #38bdf8;
	}

	.toolbar-actions {
		display: flex;
		gap: 0.5rem;
	}

	.btn {
		padding: 0.5rem 1rem;
		border-radius: 6px;
		font-size: 0.8125rem;
		font-weight: 500;
		cursor: pointer;
		border: none;
		transition: all 0.2s;
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
		color: #64748b;
	}

	.designer-content {
		display: grid;
		grid-template-columns: 320px 1fr;
		min-height: 500px;
	}

	.controls-panel {
		padding: 1rem;
		border-right: 1px solid rgba(255, 255, 255, 0.1);
		overflow-y: auto;
		max-height: 600px;
	}

	.control-group {
		margin-bottom: 1rem;
	}

	.control-label {
		display: block;
		font-size: 0.75rem;
		font-weight: 500;
		color: #94a3b8;
		margin-bottom: 0.5rem;
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.control-row {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 0.75rem;
	}

	.layout-options {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 0.5rem;
	}

	.layout-option {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem;
		background: rgba(255, 255, 255, 0.03);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 8px;
		cursor: pointer;
		transition: all 0.2s;
	}

	.layout-option input {
		display: none;
	}

	.layout-option:hover {
		background: rgba(255, 255, 255, 0.05);
	}

	.layout-option.selected {
		background: rgba(14, 165, 233, 0.15);
		border-color: #0ea5e9;
	}

	.layout-icon {
		width: 40px;
		height: 30px;
		background: rgba(255, 255, 255, 0.1);
		border-radius: 4px;
	}

	.layout-option span {
		font-size: 0.75rem;
		color: #94a3b8;
	}

	.input,
	.select,
	.textarea {
		width: 100%;
		padding: 0.5rem;
		background: rgba(255, 255, 255, 0.05);
		border: 1px solid rgba(255, 255, 255, 0.15);
		border-radius: 6px;
		color: #f1f5f9;
		font-size: 0.8125rem;
	}

	.textarea {
		resize: vertical;
	}

	.color-section {
		margin-bottom: 1.5rem;
	}

	.color-section h4,
	.typography-section h4 {
		font-size: 0.8rem;
		font-weight: 600;
		color: #e2e8f0;
		margin: 1rem 0 0.75rem;
		padding-bottom: 0.5rem;
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
	}

	.color-input {
		display: flex;
		gap: 0.5rem;
	}

	.color-input input[type='color'] {
		width: 40px;
		height: 32px;
		padding: 0;
		border: none;
		border-radius: 4px;
		cursor: pointer;
	}

	.color-input input[type='text'] {
		flex: 1;
		padding: 0.5rem;
		background: rgba(255, 255, 255, 0.05);
		border: 1px solid rgba(255, 255, 255, 0.15);
		border-radius: 6px;
		color: #f1f5f9;
		font-size: 0.75rem;
		font-family: monospace;
	}

	.typography-section {
		margin-bottom: 1rem;
	}

	.spacing-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 0.5rem;
	}

	.spacing-grid label {
		display: block;
		font-size: 0.7rem;
		color: #64748b;
		margin-bottom: 0.25rem;
	}

	.spacing-grid input {
		width: 100%;
		padding: 0.375rem;
		background: rgba(255, 255, 255, 0.05);
		border: 1px solid rgba(255, 255, 255, 0.15);
		border-radius: 4px;
		color: #f1f5f9;
		font-size: 0.75rem;
		text-align: center;
	}

	input[type='range'] {
		width: 100%;
		margin-bottom: 0.25rem;
	}

	.range-value {
		font-size: 0.75rem;
		color: #64748b;
	}

	.toggle-control {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 0.75rem;
		background: rgba(255, 255, 255, 0.03);
		border-radius: 6px;
	}

	.toggle-control .control-label {
		margin: 0;
	}

	.toggle-control input[type='checkbox'] {
		width: 40px;
		height: 22px;
	}

	.preview-panel {
		background: #1e293b;
		display: flex;
		flex-direction: column;
	}

	.preview-header {
		padding: 0.75rem 1rem;
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
		font-size: 0.8125rem;
		color: #64748b;
	}

	.preview-container {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 2rem;
		position: relative;
		background:
			linear-gradient(45deg, #0f172a 25%, transparent 25%),
			linear-gradient(-45deg, #0f172a 25%, transparent 25%),
			linear-gradient(45deg, transparent 75%, #0f172a 75%),
			linear-gradient(-45deg, transparent 75%, #0f172a 75%);
		background-size: 20px 20px;
		background-position:
			0 0,
			0 10px,
			10px -10px,
			-10px 0px;
	}

	.banner-preview {
		width: 100%;
		max-width: var(--cb-max-width);
	}

	.preview-banner {
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		padding: var(--cb-padding-top) var(--cb-padding-right) var(--cb-padding-bottom)
			var(--cb-padding-left);
		background: var(--cb-bg);
		color: var(--cb-text);
		font-family: var(--cb-font);
		border-radius: var(--cb-radius);
		border: var(--cb-border-width) var(--cb-border-style) var(--cb-border);
	}

	.preview-text {
		flex: 1;
		min-width: 200px;
	}

	.preview-text h3 {
		margin: 0 0 0.5rem;
		font-size: var(--cb-title-size);
		font-weight: var(--cb-title-weight);
		color: var(--cb-title);
	}

	.preview-text p {
		margin: 0 0 0.5rem;
		font-size: var(--cb-body-size);
		font-weight: var(--cb-body-weight);
		line-height: 1.5;
	}

	.preview-text a {
		color: var(--cb-link);
		font-size: var(--cb-body-size);
		text-decoration: underline;
	}

	.preview-actions {
		display: flex;
		flex-wrap: wrap;
		gap: var(--cb-btn-margin);
	}

	.preview-btn {
		padding: var(--cb-btn-py) var(--cb-btn-px);
		font-size: var(--cb-btn-size);
		font-weight: var(--cb-btn-weight);
		border-radius: var(--cb-btn-radius);
		cursor: pointer;
		border: none;
		transition: all 0.2s;
	}

	.preview-btn.accept {
		background: var(--cb-accept-bg);
		color: var(--cb-accept-text);
	}

	.preview-btn.reject {
		background: var(--cb-reject-bg);
		color: var(--cb-reject-text);
	}

	.preview-btn.settings {
		background: var(--cb-settings-bg);
		color: var(--cb-settings-text);
		border: 1px solid var(--cb-settings-border);
	}

	@media (max-width: 768px) {
		.designer-content {
			grid-template-columns: 1fr;
		}

		.controls-panel {
			border-right: none;
			border-bottom: 1px solid rgba(255, 255, 255, 0.1);
			max-height: 300px;
		}
	}
</style>

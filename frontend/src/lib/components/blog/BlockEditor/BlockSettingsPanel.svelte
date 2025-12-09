<script lang="ts">
	/**
	 * Block Settings Panel - Enterprise-Grade Settings Editor
	 * ========================================================
	 * Advanced block settings with typography, colors, spacing,
	 * borders, animations, and responsive controls.
	 *
	 * @version 1.0.0
	 * @author Revolution Trading Pros
	 */

	import type { Block, BlockSettings } from './types';

	interface Props {
		block: Block;
		onUpdate: (settings: Partial<BlockSettings>) => void;
	}

	let { block, onUpdate }: Props = $props();

	// Active settings tab
	let activeTab = $state<'style' | 'advanced' | 'responsive'>('style');

	// Expanded sections
	let expandedSections = $state<Set<string>>(new Set(['typography', 'colors']));

	// Color picker state
	let showColorPicker = $state<string | null>(null);

	// Preset colors
	const presetColors = [
		'#000000', '#333333', '#666666', '#999999', '#CCCCCC', '#FFFFFF',
		'#FF0000', '#FF6600', '#FFCC00', '#00FF00', '#00CCFF', '#0066FF',
		'#6600FF', '#FF00FF', '#FF3366', '#663300', '#006633', '#003366'
	];

	// Font families
	const fontFamilies = [
		{ value: 'inherit', label: 'Default' },
		{ value: 'Inter, sans-serif', label: 'Inter' },
		{ value: 'Roboto, sans-serif', label: 'Roboto' },
		{ value: 'Open Sans, sans-serif', label: 'Open Sans' },
		{ value: 'Lato, sans-serif', label: 'Lato' },
		{ value: 'Montserrat, sans-serif', label: 'Montserrat' },
		{ value: 'Poppins, sans-serif', label: 'Poppins' },
		{ value: 'Playfair Display, serif', label: 'Playfair Display' },
		{ value: 'Merriweather, serif', label: 'Merriweather' },
		{ value: 'Georgia, serif', label: 'Georgia' },
		{ value: 'Fira Code, monospace', label: 'Fira Code' },
		{ value: 'JetBrains Mono, monospace', label: 'JetBrains Mono' }
	];

	// Font weights
	const fontWeights = [
		{ value: '100', label: 'Thin' },
		{ value: '200', label: 'Extra Light' },
		{ value: '300', label: 'Light' },
		{ value: '400', label: 'Regular' },
		{ value: '500', label: 'Medium' },
		{ value: '600', label: 'Semi Bold' },
		{ value: '700', label: 'Bold' },
		{ value: '800', label: 'Extra Bold' },
		{ value: '900', label: 'Black' }
	];

	// Text alignments
	const textAlignments = [
		{ value: 'left', icon: '☰', label: 'Left' },
		{ value: 'center', icon: '☷', label: 'Center' },
		{ value: 'right', icon: '☲', label: 'Right' },
		{ value: 'justify', icon: '☱', label: 'Justify' }
	];

	// Border styles
	const borderStyles = [
		{ value: 'none', label: 'None' },
		{ value: 'solid', label: 'Solid' },
		{ value: 'dashed', label: 'Dashed' },
		{ value: 'dotted', label: 'Dotted' },
		{ value: 'double', label: 'Double' }
	];

	// Animation options
	const animations = [
		{ value: 'none', label: 'None' },
		{ value: 'fade-in', label: 'Fade In' },
		{ value: 'fade-up', label: 'Fade Up' },
		{ value: 'fade-down', label: 'Fade Down' },
		{ value: 'fade-left', label: 'Fade Left' },
		{ value: 'fade-right', label: 'Fade Right' },
		{ value: 'zoom-in', label: 'Zoom In' },
		{ value: 'zoom-out', label: 'Zoom Out' },
		{ value: 'flip', label: 'Flip' },
		{ value: 'bounce', label: 'Bounce' },
		{ value: 'slide-up', label: 'Slide Up' },
		{ value: 'slide-down', label: 'Slide Down' }
	];

	// Blend modes
	const blendModes = [
		'normal', 'multiply', 'screen', 'overlay', 'darken', 'lighten',
		'color-dodge', 'color-burn', 'hard-light', 'soft-light', 'difference'
	];

	// Section toggle
	function toggleSection(section: string): void {
		if (expandedSections.has(section)) {
			expandedSections.delete(section);
		} else {
			expandedSections.add(section);
		}
		expandedSections = new Set(expandedSections);
	}

	// Update a single setting
	function updateSetting(key: keyof BlockSettings, value: unknown): void {
		onUpdate({ [key]: value });
	}

	// Update nested setting
	function updateNestedSetting(parent: string, key: string, value: unknown): void {
		const current = (block.settings as Record<string, unknown>)[parent] || {};
		onUpdate({
			[parent]: {
				...(typeof current === 'object' ? current : {}),
				[key]: value
			}
		} as Partial<BlockSettings>);
	}

	// Parse spacing value
	function parseSpacing(value: string | undefined): { top: string; right: string; bottom: string; left: string } {
		if (!value) return { top: '0', right: '0', bottom: '0', left: '0' };
		const parts = value.split(' ');
		if (parts.length === 1) {
			return { top: parts[0], right: parts[0], bottom: parts[0], left: parts[0] };
		} else if (parts.length === 2) {
			return { top: parts[0], right: parts[1], bottom: parts[0], left: parts[1] };
		} else if (parts.length === 4) {
			return { top: parts[0], right: parts[1], bottom: parts[2], left: parts[3] };
		}
		return { top: '0', right: '0', bottom: '0', left: '0' };
	}

	// Build spacing value
	function buildSpacing(values: { top: string; right: string; bottom: string; left: string }): string {
		if (values.top === values.right && values.right === values.bottom && values.bottom === values.left) {
			return values.top;
		}
		if (values.top === values.bottom && values.left === values.right) {
			return `${values.top} ${values.right}`;
		}
		return `${values.top} ${values.right} ${values.bottom} ${values.left}`;
	}

	// Derived spacing values
	let paddingValues = $derived(parseSpacing(block.settings.padding));
	let marginValues = $derived(parseSpacing(block.settings.margin));
</script>

<div class="settings-panel">
	<!-- Tab Navigation -->
	<div class="tabs">
		<button
			class="tab"
			class:active={activeTab === 'style'}
			onclick={() => activeTab = 'style'}
		>
			Style
		</button>
		<button
			class="tab"
			class:active={activeTab === 'advanced'}
			onclick={() => activeTab = 'advanced'}
		>
			Advanced
		</button>
		<button
			class="tab"
			class:active={activeTab === 'responsive'}
			onclick={() => activeTab = 'responsive'}
		>
			Responsive
		</button>
	</div>

	<div class="tab-content">
		{#if activeTab === 'style'}
			<!-- Typography Section -->
			<div class="section">
				<button
					class="section-header"
					onclick={() => toggleSection('typography')}
				>
					<span>Typography</span>
					<span class="chevron" class:expanded={expandedSections.has('typography')}>▼</span>
				</button>
				{#if expandedSections.has('typography')}
					<div class="section-content">
						<div class="field">
							<label>Font Family</label>
							<select
								value={block.settings.fontFamily || 'inherit'}
								onchange={(e) => updateSetting('fontFamily', e.currentTarget.value)}
							>
								{#each fontFamilies as font}
									<option value={font.value}>{font.label}</option>
								{/each}
							</select>
						</div>

						<div class="field-row">
							<div class="field">
								<label>Size</label>
								<div class="input-with-unit">
									<input
										type="number"
										min="8"
										max="200"
										value={parseInt(block.settings.fontSize || '16')}
										onchange={(e) => updateSetting('fontSize', `${e.currentTarget.value}px`)}
									/>
									<span class="unit">px</span>
								</div>
							</div>
							<div class="field">
								<label>Weight</label>
								<select
									value={block.settings.fontWeight || '400'}
									onchange={(e) => updateSetting('fontWeight', e.currentTarget.value)}
								>
									{#each fontWeights as weight}
										<option value={weight.value}>{weight.label}</option>
									{/each}
								</select>
							</div>
						</div>

						<div class="field-row">
							<div class="field">
								<label>Line Height</label>
								<input
									type="number"
									min="0.5"
									max="3"
									step="0.1"
									value={parseFloat(block.settings.lineHeight || '1.6')}
									onchange={(e) => updateSetting('lineHeight', e.currentTarget.value)}
								/>
							</div>
							<div class="field">
								<label>Letter Spacing</label>
								<div class="input-with-unit">
									<input
										type="number"
										min="-5"
										max="20"
										step="0.5"
										value={parseFloat(block.settings.letterSpacing || '0')}
										onchange={(e) => updateSetting('letterSpacing', `${e.currentTarget.value}px`)}
									/>
									<span class="unit">px</span>
								</div>
							</div>
						</div>

						<div class="field">
							<label>Text Alignment</label>
							<div class="button-group">
								{#each textAlignments as align}
									<button
										class="icon-btn"
										class:active={block.settings.textAlign === align.value}
										onclick={() => updateSetting('textAlign', align.value)}
										title={align.label}
									>
										{align.icon}
									</button>
								{/each}
							</div>
						</div>

						<div class="field">
							<label>Text Transform</label>
							<div class="button-group">
								<button
									class="text-btn"
									class:active={!block.settings.textTransform || block.settings.textTransform === 'none'}
									onclick={() => updateSetting('textTransform', 'none')}
								>
									Aa
								</button>
								<button
									class="text-btn"
									class:active={block.settings.textTransform === 'uppercase'}
									onclick={() => updateSetting('textTransform', 'uppercase')}
								>
									AA
								</button>
								<button
									class="text-btn"
									class:active={block.settings.textTransform === 'lowercase'}
									onclick={() => updateSetting('textTransform', 'lowercase')}
								>
									aa
								</button>
								<button
									class="text-btn"
									class:active={block.settings.textTransform === 'capitalize'}
									onclick={() => updateSetting('textTransform', 'capitalize')}
								>
									Aa
								</button>
							</div>
						</div>
					</div>
				{/if}
			</div>

			<!-- Colors Section -->
			<div class="section">
				<button
					class="section-header"
					onclick={() => toggleSection('colors')}
				>
					<span>Colors</span>
					<span class="chevron" class:expanded={expandedSections.has('colors')}>▼</span>
				</button>
				{#if expandedSections.has('colors')}
					<div class="section-content">
						<div class="field">
							<label>Text Color</label>
							<div class="color-picker-trigger">
								<button
									class="color-swatch"
									style="background-color: {block.settings.textColor || '#000000'}"
									onclick={() => showColorPicker = showColorPicker === 'text' ? null : 'text'}
								></button>
								<input
									type="text"
									value={block.settings.textColor || '#000000'}
									onchange={(e) => updateSetting('textColor', e.currentTarget.value)}
								/>
							</div>
							{#if showColorPicker === 'text'}
								<div class="color-palette">
									{#each presetColors as color}
										<button
											class="preset-color"
											style="background-color: {color}"
											onclick={() => {
												updateSetting('textColor', color);
												showColorPicker = null;
											}}
										></button>
									{/each}
								</div>
							{/if}
						</div>

						<div class="field">
							<label>Background Color</label>
							<div class="color-picker-trigger">
								<button
									class="color-swatch"
									style="background-color: {block.settings.backgroundColor || 'transparent'}"
									onclick={() => showColorPicker = showColorPicker === 'bg' ? null : 'bg'}
								></button>
								<input
									type="text"
									value={block.settings.backgroundColor || 'transparent'}
									onchange={(e) => updateSetting('backgroundColor', e.currentTarget.value)}
								/>
							</div>
							{#if showColorPicker === 'bg'}
								<div class="color-palette">
									{#each presetColors as color}
										<button
											class="preset-color"
											style="background-color: {color}"
											onclick={() => {
												updateSetting('backgroundColor', color);
												showColorPicker = null;
											}}
										></button>
									{/each}
								</div>
							{/if}
						</div>

						<div class="field">
							<label>Background Gradient</label>
							<input
								type="text"
								placeholder="linear-gradient(135deg, #667eea 0%, #764ba2 100%)"
								value={block.settings.backgroundGradient || ''}
								onchange={(e) => updateSetting('backgroundGradient', e.currentTarget.value)}
							/>
						</div>
					</div>
				{/if}
			</div>

			<!-- Spacing Section -->
			<div class="section">
				<button
					class="section-header"
					onclick={() => toggleSection('spacing')}
				>
					<span>Spacing</span>
					<span class="chevron" class:expanded={expandedSections.has('spacing')}>▼</span>
				</button>
				{#if expandedSections.has('spacing')}
					<div class="section-content">
						<div class="spacing-control">
							<label>Padding</label>
							<div class="spacing-grid">
								<div class="spacing-top">
									<input
										type="number"
										min="0"
										value={parseInt(paddingValues.top) || 0}
										onchange={(e) => {
											const newValues = { ...paddingValues, top: `${e.currentTarget.value}px` };
											updateSetting('padding', buildSpacing(newValues));
										}}
									/>
								</div>
								<div class="spacing-left">
									<input
										type="number"
										min="0"
										value={parseInt(paddingValues.left) || 0}
										onchange={(e) => {
											const newValues = { ...paddingValues, left: `${e.currentTarget.value}px` };
											updateSetting('padding', buildSpacing(newValues));
										}}
									/>
								</div>
								<div class="spacing-center">
									<span class="spacing-label">px</span>
								</div>
								<div class="spacing-right">
									<input
										type="number"
										min="0"
										value={parseInt(paddingValues.right) || 0}
										onchange={(e) => {
											const newValues = { ...paddingValues, right: `${e.currentTarget.value}px` };
											updateSetting('padding', buildSpacing(newValues));
										}}
									/>
								</div>
								<div class="spacing-bottom">
									<input
										type="number"
										min="0"
										value={parseInt(paddingValues.bottom) || 0}
										onchange={(e) => {
											const newValues = { ...paddingValues, bottom: `${e.currentTarget.value}px` };
											updateSetting('padding', buildSpacing(newValues));
										}}
									/>
								</div>
							</div>
						</div>

						<div class="spacing-control">
							<label>Margin</label>
							<div class="spacing-grid">
								<div class="spacing-top">
									<input
										type="number"
										value={parseInt(marginValues.top) || 0}
										onchange={(e) => {
											const newValues = { ...marginValues, top: `${e.currentTarget.value}px` };
											updateSetting('margin', buildSpacing(newValues));
										}}
									/>
								</div>
								<div class="spacing-left">
									<input
										type="number"
										value={parseInt(marginValues.left) || 0}
										onchange={(e) => {
											const newValues = { ...marginValues, left: `${e.currentTarget.value}px` };
											updateSetting('margin', buildSpacing(newValues));
										}}
									/>
								</div>
								<div class="spacing-center">
									<span class="spacing-label">px</span>
								</div>
								<div class="spacing-right">
									<input
										type="number"
										value={parseInt(marginValues.right) || 0}
										onchange={(e) => {
											const newValues = { ...marginValues, right: `${e.currentTarget.value}px` };
											updateSetting('margin', buildSpacing(newValues));
										}}
									/>
								</div>
								<div class="spacing-bottom">
									<input
										type="number"
										value={parseInt(marginValues.bottom) || 0}
										onchange={(e) => {
											const newValues = { ...marginValues, bottom: `${e.currentTarget.value}px` };
											updateSetting('margin', buildSpacing(newValues));
										}}
									/>
								</div>
							</div>
						</div>
					</div>
				{/if}
			</div>

			<!-- Border Section -->
			<div class="section">
				<button
					class="section-header"
					onclick={() => toggleSection('border')}
				>
					<span>Border</span>
					<span class="chevron" class:expanded={expandedSections.has('border')}>▼</span>
				</button>
				{#if expandedSections.has('border')}
					<div class="section-content">
						<div class="field-row">
							<div class="field">
								<label>Style</label>
								<select
									value={block.settings.borderStyle || 'none'}
									onchange={(e) => updateSetting('borderStyle', e.currentTarget.value)}
								>
									{#each borderStyles as style}
										<option value={style.value}>{style.label}</option>
									{/each}
								</select>
							</div>
							<div class="field">
								<label>Width</label>
								<div class="input-with-unit">
									<input
										type="number"
										min="0"
										max="20"
										value={parseInt(block.settings.borderWidth || '0')}
										onchange={(e) => updateSetting('borderWidth', `${e.currentTarget.value}px`)}
									/>
									<span class="unit">px</span>
								</div>
							</div>
						</div>

						<div class="field">
							<label>Border Color</label>
							<div class="color-picker-trigger">
								<button
									class="color-swatch"
									style="background-color: {block.settings.borderColor || '#CCCCCC'}"
									onclick={() => showColorPicker = showColorPicker === 'border' ? null : 'border'}
								></button>
								<input
									type="text"
									value={block.settings.borderColor || '#CCCCCC'}
									onchange={(e) => updateSetting('borderColor', e.currentTarget.value)}
								/>
							</div>
							{#if showColorPicker === 'border'}
								<div class="color-palette">
									{#each presetColors as color}
										<button
											class="preset-color"
											style="background-color: {color}"
											onclick={() => {
												updateSetting('borderColor', color);
												showColorPicker = null;
											}}
										></button>
									{/each}
								</div>
							{/if}
						</div>

						<div class="field">
							<label>Border Radius</label>
							<div class="input-with-unit">
								<input
									type="number"
									min="0"
									max="100"
									value={parseInt(block.settings.borderRadius || '0')}
									onchange={(e) => updateSetting('borderRadius', `${e.currentTarget.value}px`)}
								/>
								<span class="unit">px</span>
							</div>
						</div>

						<div class="field">
							<label>Box Shadow</label>
							<input
								type="text"
								placeholder="0 4px 6px rgba(0, 0, 0, 0.1)"
								value={block.settings.boxShadow || ''}
								onchange={(e) => updateSetting('boxShadow', e.currentTarget.value)}
							/>
						</div>
					</div>
				{/if}
			</div>

		{:else if activeTab === 'advanced'}
			<!-- Animation Section -->
			<div class="section">
				<button
					class="section-header"
					onclick={() => toggleSection('animation')}
				>
					<span>Animation</span>
					<span class="chevron" class:expanded={expandedSections.has('animation')}>▼</span>
				</button>
				{#if expandedSections.has('animation')}
					<div class="section-content">
						<div class="field">
							<label>Entrance Animation</label>
							<select
								value={block.settings.animation || 'none'}
								onchange={(e) => updateSetting('animation', e.currentTarget.value)}
							>
								{#each animations as anim}
									<option value={anim.value}>{anim.label}</option>
								{/each}
							</select>
						</div>

						<div class="field-row">
							<div class="field">
								<label>Duration</label>
								<div class="input-with-unit">
									<input
										type="number"
										min="0"
										max="5"
										step="0.1"
										value={parseFloat(block.settings.animationDuration || '0.5')}
										onchange={(e) => updateSetting('animationDuration', `${e.currentTarget.value}s`)}
									/>
									<span class="unit">s</span>
								</div>
							</div>
							<div class="field">
								<label>Delay</label>
								<div class="input-with-unit">
									<input
										type="number"
										min="0"
										max="5"
										step="0.1"
										value={parseFloat(block.settings.animationDelay || '0')}
										onchange={(e) => updateSetting('animationDelay', `${e.currentTarget.value}s`)}
									/>
									<span class="unit">s</span>
								</div>
							</div>
						</div>
					</div>
				{/if}
			</div>

			<!-- Transform Section -->
			<div class="section">
				<button
					class="section-header"
					onclick={() => toggleSection('transform')}
				>
					<span>Transform</span>
					<span class="chevron" class:expanded={expandedSections.has('transform')}>▼</span>
				</button>
				{#if expandedSections.has('transform')}
					<div class="section-content">
						<div class="field-row">
							<div class="field">
								<label>Rotate</label>
								<div class="input-with-unit">
									<input
										type="number"
										min="-360"
										max="360"
										value={parseInt(block.settings.rotate || '0')}
										onchange={(e) => updateSetting('rotate', `${e.currentTarget.value}deg`)}
									/>
									<span class="unit">°</span>
								</div>
							</div>
							<div class="field">
								<label>Scale</label>
								<input
									type="number"
									min="0"
									max="3"
									step="0.1"
									value={parseFloat(block.settings.scale || '1')}
									onchange={(e) => updateSetting('scale', e.currentTarget.value)}
								/>
							</div>
						</div>

						<div class="field-row">
							<div class="field">
								<label>Translate X</label>
								<div class="input-with-unit">
									<input
										type="number"
										value={parseInt(block.settings.translateX || '0')}
										onchange={(e) => updateSetting('translateX', `${e.currentTarget.value}px`)}
									/>
									<span class="unit">px</span>
								</div>
							</div>
							<div class="field">
								<label>Translate Y</label>
								<div class="input-with-unit">
									<input
										type="number"
										value={parseInt(block.settings.translateY || '0')}
										onchange={(e) => updateSetting('translateY', `${e.currentTarget.value}px`)}
									/>
									<span class="unit">px</span>
								</div>
							</div>
						</div>

						<div class="field-row">
							<div class="field">
								<label>Skew X</label>
								<div class="input-with-unit">
									<input
										type="number"
										min="-45"
										max="45"
										value={parseInt(block.settings.skewX || '0')}
										onchange={(e) => updateSetting('skewX', `${e.currentTarget.value}deg`)}
									/>
									<span class="unit">°</span>
								</div>
							</div>
							<div class="field">
								<label>Skew Y</label>
								<div class="input-with-unit">
									<input
										type="number"
										min="-45"
										max="45"
										value={parseInt(block.settings.skewY || '0')}
										onchange={(e) => updateSetting('skewY', `${e.currentTarget.value}deg`)}
									/>
									<span class="unit">°</span>
								</div>
							</div>
						</div>
					</div>
				{/if}
			</div>

			<!-- Effects Section -->
			<div class="section">
				<button
					class="section-header"
					onclick={() => toggleSection('effects')}
				>
					<span>Effects</span>
					<span class="chevron" class:expanded={expandedSections.has('effects')}>▼</span>
				</button>
				{#if expandedSections.has('effects')}
					<div class="section-content">
						<div class="field">
							<label>Opacity</label>
							<div class="slider-field">
								<input
									type="range"
									min="0"
									max="1"
									step="0.01"
									value={block.settings.opacity || 1}
									oninput={(e) => updateSetting('opacity', parseFloat(e.currentTarget.value))}
								/>
								<span class="value">{Math.round((block.settings.opacity || 1) * 100)}%</span>
							</div>
						</div>

						<div class="field">
							<label>Blend Mode</label>
							<select
								value={block.settings.blendMode || 'normal'}
								onchange={(e) => updateSetting('blendMode', e.currentTarget.value)}
							>
								{#each blendModes as mode}
									<option value={mode}>{mode}</option>
								{/each}
							</select>
						</div>

						<div class="field">
							<label>CSS Filters</label>
							<div class="filter-controls">
								<div class="filter-item">
									<span>Blur</span>
									<input
										type="range"
										min="0"
										max="20"
										value={parseInt(block.settings.filterBlur || '0')}
										oninput={(e) => updateSetting('filterBlur', `${e.currentTarget.value}px`)}
									/>
									<span class="value">{block.settings.filterBlur || '0px'}</span>
								</div>
								<div class="filter-item">
									<span>Brightness</span>
									<input
										type="range"
										min="0"
										max="200"
										value={parseInt(block.settings.filterBrightness || '100')}
										oninput={(e) => updateSetting('filterBrightness', `${e.currentTarget.value}%`)}
									/>
									<span class="value">{block.settings.filterBrightness || '100%'}</span>
								</div>
								<div class="filter-item">
									<span>Contrast</span>
									<input
										type="range"
										min="0"
										max="200"
										value={parseInt(block.settings.filterContrast || '100')}
										oninput={(e) => updateSetting('filterContrast', `${e.currentTarget.value}%`)}
									/>
									<span class="value">{block.settings.filterContrast || '100%'}</span>
								</div>
								<div class="filter-item">
									<span>Grayscale</span>
									<input
										type="range"
										min="0"
										max="100"
										value={parseInt(block.settings.filterGrayscale || '0')}
										oninput={(e) => updateSetting('filterGrayscale', `${e.currentTarget.value}%`)}
									/>
									<span class="value">{block.settings.filterGrayscale || '0%'}</span>
								</div>
								<div class="filter-item">
									<span>Saturate</span>
									<input
										type="range"
										min="0"
										max="200"
										value={parseInt(block.settings.filterSaturate || '100')}
										oninput={(e) => updateSetting('filterSaturate', `${e.currentTarget.value}%`)}
									/>
									<span class="value">{block.settings.filterSaturate || '100%'}</span>
								</div>
							</div>
						</div>
					</div>
				{/if}
			</div>

			<!-- Custom CSS Section -->
			<div class="section">
				<button
					class="section-header"
					onclick={() => toggleSection('customCss')}
				>
					<span>Custom CSS</span>
					<span class="chevron" class:expanded={expandedSections.has('customCss')}>▼</span>
				</button>
				{#if expandedSections.has('customCss')}
					<div class="section-content">
						<div class="field">
							<label>CSS Class</label>
							<input
								type="text"
								placeholder="my-custom-class another-class"
								value={block.settings.customClass || ''}
								onchange={(e) => updateSetting('customClass', e.currentTarget.value)}
							/>
						</div>

						<div class="field">
							<label>CSS ID</label>
							<input
								type="text"
								placeholder="my-element-id"
								value={block.settings.customId || ''}
								onchange={(e) => updateSetting('customId', e.currentTarget.value)}
							/>
						</div>

						<div class="field">
							<label>Custom CSS</label>
							<textarea
								rows="6"
								placeholder=".selector {
  property: value;
}"
								value={block.settings.customCss || ''}
								onchange={(e) => updateSetting('customCss', e.currentTarget.value)}
							></textarea>
						</div>
					</div>
				{/if}
			</div>

		{:else if activeTab === 'responsive'}
			<!-- Visibility Section -->
			<div class="section">
				<button
					class="section-header"
					onclick={() => toggleSection('visibility')}
				>
					<span>Visibility</span>
					<span class="chevron" class:expanded={expandedSections.has('visibility')}>▼</span>
				</button>
				{#if expandedSections.has('visibility')}
					<div class="section-content">
						<div class="visibility-options">
							<label class="checkbox-label">
								<input
									type="checkbox"
									checked={block.settings.hideOnDesktop !== true}
									onchange={(e) => updateSetting('hideOnDesktop', !e.currentTarget.checked)}
								/>
								<span class="device-icon">🖥️</span>
								<span>Show on Desktop</span>
							</label>
							<label class="checkbox-label">
								<input
									type="checkbox"
									checked={block.settings.hideOnTablet !== true}
									onchange={(e) => updateSetting('hideOnTablet', !e.currentTarget.checked)}
								/>
								<span class="device-icon">📱</span>
								<span>Show on Tablet</span>
							</label>
							<label class="checkbox-label">
								<input
									type="checkbox"
									checked={block.settings.hideOnMobile !== true}
									onchange={(e) => updateSetting('hideOnMobile', !e.currentTarget.checked)}
								/>
								<span class="device-icon">📲</span>
								<span>Show on Mobile</span>
							</label>
						</div>
					</div>
				{/if}
			</div>

			<!-- Responsive Overrides Section -->
			<div class="section">
				<button
					class="section-header"
					onclick={() => toggleSection('responsive')}
				>
					<span>Responsive Overrides</span>
					<span class="chevron" class:expanded={expandedSections.has('responsive')}>▼</span>
				</button>
				{#if expandedSections.has('responsive')}
					<div class="section-content">
						<p class="info-text">
							Override settings for specific screen sizes. Leave empty to use default values.
						</p>

						<div class="responsive-device">
							<h4>📱 Tablet (768px - 1024px)</h4>
							<div class="field-row">
								<div class="field">
									<label>Font Size</label>
									<input
										type="text"
										placeholder="e.g., 14px"
										value={block.settings.tabletFontSize || ''}
										onchange={(e) => updateSetting('tabletFontSize', e.currentTarget.value)}
									/>
								</div>
								<div class="field">
									<label>Padding</label>
									<input
										type="text"
										placeholder="e.g., 10px 15px"
										value={block.settings.tabletPadding || ''}
										onchange={(e) => updateSetting('tabletPadding', e.currentTarget.value)}
									/>
								</div>
							</div>
						</div>

						<div class="responsive-device">
							<h4>📲 Mobile (&lt; 768px)</h4>
							<div class="field-row">
								<div class="field">
									<label>Font Size</label>
									<input
										type="text"
										placeholder="e.g., 12px"
										value={block.settings.mobileFontSize || ''}
										onchange={(e) => updateSetting('mobileFontSize', e.currentTarget.value)}
									/>
								</div>
								<div class="field">
									<label>Padding</label>
									<input
										type="text"
										placeholder="e.g., 8px 10px"
										value={block.settings.mobilePadding || ''}
										onchange={(e) => updateSetting('mobilePadding', e.currentTarget.value)}
									/>
								</div>
							</div>
						</div>
					</div>
				{/if}
			</div>

			<!-- Z-Index Section -->
			<div class="section">
				<button
					class="section-header"
					onclick={() => toggleSection('zindex')}
				>
					<span>Position & Z-Index</span>
					<span class="chevron" class:expanded={expandedSections.has('zindex')}>▼</span>
				</button>
				{#if expandedSections.has('zindex')}
					<div class="section-content">
						<div class="field">
							<label>Z-Index</label>
							<input
								type="number"
								min="-999"
								max="9999"
								value={block.settings.zIndex || 0}
								onchange={(e) => updateSetting('zIndex', parseInt(e.currentTarget.value))}
							/>
						</div>

						<div class="field">
							<label>Position</label>
							<select
								value={block.settings.position || 'static'}
								onchange={(e) => updateSetting('position', e.currentTarget.value)}
							>
								<option value="static">Static</option>
								<option value="relative">Relative</option>
								<option value="absolute">Absolute</option>
								<option value="fixed">Fixed</option>
								<option value="sticky">Sticky</option>
							</select>
						</div>

						{#if block.settings.position && block.settings.position !== 'static'}
							<div class="field-row">
								<div class="field">
									<label>Top</label>
									<input
										type="text"
										placeholder="auto"
										value={block.settings.positionTop || ''}
										onchange={(e) => updateSetting('positionTop', e.currentTarget.value)}
									/>
								</div>
								<div class="field">
									<label>Right</label>
									<input
										type="text"
										placeholder="auto"
										value={block.settings.positionRight || ''}
										onchange={(e) => updateSetting('positionRight', e.currentTarget.value)}
									/>
								</div>
							</div>
							<div class="field-row">
								<div class="field">
									<label>Bottom</label>
									<input
										type="text"
										placeholder="auto"
										value={block.settings.positionBottom || ''}
										onchange={(e) => updateSetting('positionBottom', e.currentTarget.value)}
									/>
								</div>
								<div class="field">
									<label>Left</label>
									<input
										type="text"
										placeholder="auto"
										value={block.settings.positionLeft || ''}
										onchange={(e) => updateSetting('positionLeft', e.currentTarget.value)}
									/>
								</div>
							</div>
						{/if}
					</div>
				{/if}
			</div>
		{/if}
	</div>
</div>

<style>
	.settings-panel {
		height: 100%;
		display: flex;
		flex-direction: column;
		background: var(--bg-primary, #ffffff);
	}

	.tabs {
		display: flex;
		border-bottom: 1px solid var(--border-color, #e5e7eb);
		flex-shrink: 0;
	}

	.tab {
		flex: 1;
		padding: 0.75rem;
		background: none;
		border: none;
		font-size: 0.875rem;
		font-weight: 500;
		color: var(--text-secondary, #6b7280);
		cursor: pointer;
		transition: all 0.2s;
	}

	.tab:hover {
		color: var(--text-primary, #1f2937);
		background: var(--bg-hover, #f9fafb);
	}

	.tab.active {
		color: var(--primary, #3b82f6);
		border-bottom: 2px solid var(--primary, #3b82f6);
		margin-bottom: -1px;
	}

	.tab-content {
		flex: 1;
		overflow-y: auto;
		padding: 0.5rem;
	}

	.section {
		margin-bottom: 0.5rem;
		border: 1px solid var(--border-color, #e5e7eb);
		border-radius: 0.5rem;
		overflow: hidden;
	}

	.section-header {
		width: 100%;
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 0.75rem 1rem;
		background: var(--bg-secondary, #f9fafb);
		border: none;
		font-size: 0.875rem;
		font-weight: 600;
		color: var(--text-primary, #1f2937);
		cursor: pointer;
		transition: background 0.2s;
	}

	.section-header:hover {
		background: var(--bg-hover, #f3f4f6);
	}

	.chevron {
		font-size: 0.75rem;
		transition: transform 0.2s;
	}

	.chevron.expanded {
		transform: rotate(180deg);
	}

	.section-content {
		padding: 1rem;
		border-top: 1px solid var(--border-color, #e5e7eb);
	}

	.field {
		margin-bottom: 1rem;
	}

	.field:last-child {
		margin-bottom: 0;
	}

	.field label {
		display: block;
		margin-bottom: 0.375rem;
		font-size: 0.75rem;
		font-weight: 500;
		color: var(--text-secondary, #6b7280);
		text-transform: uppercase;
		letter-spacing: 0.025em;
	}

	.field input[type="text"],
	.field input[type="number"],
	.field select,
	.field textarea {
		width: 100%;
		padding: 0.5rem 0.75rem;
		font-size: 0.875rem;
		border: 1px solid var(--border-color, #e5e7eb);
		border-radius: 0.375rem;
		background: var(--bg-primary, #ffffff);
		color: var(--text-primary, #1f2937);
		transition: border-color 0.2s, box-shadow 0.2s;
	}

	.field input:focus,
	.field select:focus,
	.field textarea:focus {
		outline: none;
		border-color: var(--primary, #3b82f6);
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
	}

	.field-row {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 0.75rem;
		margin-bottom: 1rem;
	}

	.field-row .field {
		margin-bottom: 0;
	}

	.input-with-unit {
		display: flex;
		align-items: center;
		border: 1px solid var(--border-color, #e5e7eb);
		border-radius: 0.375rem;
		overflow: hidden;
	}

	.input-with-unit input {
		flex: 1;
		border: none;
		border-radius: 0;
		padding: 0.5rem 0.75rem;
	}

	.input-with-unit input:focus {
		box-shadow: none;
	}

	.input-with-unit .unit {
		padding: 0 0.75rem;
		background: var(--bg-secondary, #f9fafb);
		color: var(--text-secondary, #6b7280);
		font-size: 0.75rem;
		border-left: 1px solid var(--border-color, #e5e7eb);
	}

	.button-group {
		display: flex;
		gap: 0.25rem;
	}

	.icon-btn,
	.text-btn {
		padding: 0.5rem 0.75rem;
		background: var(--bg-secondary, #f9fafb);
		border: 1px solid var(--border-color, #e5e7eb);
		border-radius: 0.375rem;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.icon-btn:hover,
	.text-btn:hover {
		background: var(--bg-hover, #f3f4f6);
	}

	.icon-btn.active,
	.text-btn.active {
		background: var(--primary, #3b82f6);
		color: white;
		border-color: var(--primary, #3b82f6);
	}

	.color-picker-trigger {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.color-swatch {
		width: 32px;
		height: 32px;
		border: 2px solid var(--border-color, #e5e7eb);
		border-radius: 0.375rem;
		cursor: pointer;
		flex-shrink: 0;
	}

	.color-picker-trigger input {
		flex: 1;
	}

	.color-palette {
		display: grid;
		grid-template-columns: repeat(6, 1fr);
		gap: 0.25rem;
		margin-top: 0.5rem;
		padding: 0.5rem;
		background: var(--bg-secondary, #f9fafb);
		border-radius: 0.375rem;
	}

	.preset-color {
		width: 100%;
		aspect-ratio: 1;
		border: 1px solid var(--border-color, #e5e7eb);
		border-radius: 0.25rem;
		cursor: pointer;
		transition: transform 0.2s;
	}

	.preset-color:hover {
		transform: scale(1.1);
	}

	.spacing-control {
		margin-bottom: 1rem;
	}

	.spacing-control:last-child {
		margin-bottom: 0;
	}

	.spacing-control > label {
		display: block;
		margin-bottom: 0.5rem;
		font-size: 0.75rem;
		font-weight: 500;
		color: var(--text-secondary, #6b7280);
		text-transform: uppercase;
	}

	.spacing-grid {
		display: grid;
		grid-template-columns: 1fr 1fr 1fr;
		grid-template-rows: auto auto auto;
		gap: 0.25rem;
		padding: 0.5rem;
		background: var(--bg-secondary, #f9fafb);
		border-radius: 0.5rem;
	}

	.spacing-top {
		grid-column: 2;
		grid-row: 1;
	}

	.spacing-left {
		grid-column: 1;
		grid-row: 2;
	}

	.spacing-center {
		grid-column: 2;
		grid-row: 2;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.spacing-right {
		grid-column: 3;
		grid-row: 2;
	}

	.spacing-bottom {
		grid-column: 2;
		grid-row: 3;
	}

	.spacing-grid input {
		width: 100%;
		padding: 0.375rem;
		text-align: center;
		font-size: 0.75rem;
		border: 1px solid var(--border-color, #e5e7eb);
		border-radius: 0.25rem;
	}

	.spacing-label {
		font-size: 0.625rem;
		color: var(--text-tertiary, #9ca3af);
		text-transform: uppercase;
	}

	.slider-field {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.slider-field input[type="range"] {
		flex: 1;
		height: 4px;
		-webkit-appearance: none;
		background: var(--border-color, #e5e7eb);
		border-radius: 2px;
		cursor: pointer;
	}

	.slider-field input[type="range"]::-webkit-slider-thumb {
		-webkit-appearance: none;
		width: 16px;
		height: 16px;
		background: var(--primary, #3b82f6);
		border-radius: 50%;
		cursor: pointer;
	}

	.slider-field .value {
		min-width: 3rem;
		font-size: 0.75rem;
		color: var(--text-secondary, #6b7280);
		text-align: right;
	}

	.filter-controls {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.filter-item {
		display: grid;
		grid-template-columns: 80px 1fr 50px;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.75rem;
	}

	.filter-item span:first-child {
		color: var(--text-secondary, #6b7280);
	}

	.filter-item input[type="range"] {
		width: 100%;
		height: 4px;
		-webkit-appearance: none;
		background: var(--border-color, #e5e7eb);
		border-radius: 2px;
	}

	.filter-item input[type="range"]::-webkit-slider-thumb {
		-webkit-appearance: none;
		width: 12px;
		height: 12px;
		background: var(--primary, #3b82f6);
		border-radius: 50%;
		cursor: pointer;
	}

	.filter-item .value {
		text-align: right;
		color: var(--text-tertiary, #9ca3af);
	}

	.visibility-options {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.checkbox-label {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		cursor: pointer;
		font-size: 0.875rem;
	}

	.checkbox-label input[type="checkbox"] {
		width: 16px;
		height: 16px;
		accent-color: var(--primary, #3b82f6);
	}

	.device-icon {
		font-size: 1rem;
	}

	.info-text {
		font-size: 0.75rem;
		color: var(--text-secondary, #6b7280);
		margin-bottom: 1rem;
		line-height: 1.5;
	}

	.responsive-device {
		margin-bottom: 1rem;
		padding-bottom: 1rem;
		border-bottom: 1px solid var(--border-color, #e5e7eb);
	}

	.responsive-device:last-child {
		margin-bottom: 0;
		padding-bottom: 0;
		border-bottom: none;
	}

	.responsive-device h4 {
		font-size: 0.875rem;
		font-weight: 600;
		color: var(--text-primary, #1f2937);
		margin-bottom: 0.75rem;
	}

	textarea {
		font-family: 'Fira Code', 'Monaco', 'Consolas', monospace;
		font-size: 0.75rem;
		line-height: 1.5;
		resize: vertical;
	}
</style>

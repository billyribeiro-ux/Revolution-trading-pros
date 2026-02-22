<script lang="ts">
import { logger } from '$lib/utils/logger';
	/**
	 * Block Settings Panel - Enterprise-Grade Settings Editor
	 * ========================================================
	 * Advanced block settings with typography, colors, spacing,
	 * borders, animations, and responsive controls.
	 *
	 * Features:
	 * - Datasource integration for dropdown fields
	 * - Dynamic option loading from datasources
	 * - Full typography, colors, spacing controls
	 *
	 * @version 1.1.0
	 * @author Revolution Trading Pros
	 */

	import { onMount } from 'svelte';
	import type { Block, BlockSettings } from './types';
	import { API_BASE_URL } from '$lib/api/config';
	import { getAuthToken } from '$lib/stores/auth.svelte';

	interface Props {
		block: Block;
		onupdate: (updates: Partial<Block>) => void;
	}

	let props: Props = $props();
	const block = $derived(props.block);
	const onupdate = $derived(props.onupdate);

	// Datasource types
	interface Datasource {
		id: string;
		name: string;
		slug: string;
		description: string | null;
		entry_count: number;
	}

	interface DatasourceEntry {
		name: string;
		value: string;
	}

	// Datasource state
	let datasources = $state<Datasource[]>([]);
	let datasourceEntries = $state<Record<string, DatasourceEntry[]>>({});
	let loadingDatasources = $state(false);
	let loadingEntries = $state<Record<string, boolean>>({});

	// Fetch available datasources
	async function fetchDatasources() {
		if (datasources.length > 0) return; // Already loaded

		loadingDatasources = true;
		try {
			const token = getAuthToken();
			const response = await fetch(`${API_BASE_URL}/cms/datasources?limit=100`, {
				headers: {
					Authorization: `Bearer ${token}`,
					'Content-Type': 'application/json'
				}
			});

			if (response.ok) {
				const data = await response.json();
				datasources = data.data || [];
			}
		} catch (err) {
			logger.error('Failed to fetch datasources:', err);
		} finally {
			loadingDatasources = false;
		}
	}

	// Fetch entries for a specific datasource
	async function fetchDatasourceEntries(slug: string): Promise<DatasourceEntry[]> {
		if (datasourceEntries[slug]) return datasourceEntries[slug];

		loadingEntries = { ...loadingEntries, [slug]: true };
		try {
			// Use public API endpoint for fetching entries
			const response = await fetch(`${API_BASE_URL}/cms/datasources/public/${slug}`);

			if (response.ok) {
				const data = await response.json();
				const entries = data.entries || [];
				datasourceEntries = { ...datasourceEntries, [slug]: entries };
				return entries;
			}
		} catch (err) {
			logger.error(`Failed to fetch entries for ${slug}:`, err);
		} finally {
			loadingEntries = { ...loadingEntries, [slug]: false };
		}
		return [];
	}

	// Load datasources when panel mounts
	onMount(() => {
		fetchDatasources();
	});

	// Wrapper to update settings
	function updateSettings(settings: Partial<BlockSettings>): void {
		onupdate({ settings: { ...block.settings, ...settings } });
	}

	// Active settings tab
	let activeTab = $state<'style' | 'advanced' | 'responsive'>('style');

	// Expanded sections
	let expandedSections = $state<Set<string>>(new Set(['typography', 'colors']));

	// Color picker state
	let showColorPicker = $state<string | null>(null);

	// Preset colors
	const presetColors = [
		'#000000',
		'#333333',
		'#666666',
		'#999999',
		'#CCCCCC',
		'#FFFFFF',
		'#FF0000',
		'#FF6600',
		'#FFCC00',
		'#00FF00',
		'#00CCFF',
		'#0066FF',
		'#6600FF',
		'#FF00FF',
		'#FF3366',
		'#663300',
		'#006633',
		'#003366'
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
		'normal',
		'multiply',
		'screen',
		'overlay',
		'darken',
		'lighten',
		'color-dodge',
		'color-burn',
		'hard-light',
		'soft-light',
		'difference'
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
		updateSettings({ [key]: value });
	}

	// Parse spacing value
	function parseSpacing(value: string | undefined): {
		top: string;
		right: string;
		bottom: string;
		left: string;
	} {
		if (!value) return { top: '0', right: '0', bottom: '0', left: '0' };
		const parts = value.split(' ');
		if (parts.length === 1) {
			return {
				top: parts[0] ?? '0',
				right: parts[0] ?? '0',
				bottom: parts[0] ?? '0',
				left: parts[0] ?? '0'
			};
		} else if (parts.length === 2) {
			return {
				top: parts[0] ?? '0',
				right: parts[1] ?? '0',
				bottom: parts[0] ?? '0',
				left: parts[1] ?? '0'
			};
		} else if (parts.length === 4) {
			return {
				top: parts[0] ?? '0',
				right: parts[1] ?? '0',
				bottom: parts[2] ?? '0',
				left: parts[3] ?? '0'
			};
		}
		return { top: '0', right: '0', bottom: '0', left: '0' };
	}

	// Build spacing value
	function buildSpacing(values: {
		top: string;
		right: string;
		bottom: string;
		left: string;
	}): string {
		if (
			values.top === values.right &&
			values.right === values.bottom &&
			values.bottom === values.left
		) {
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
		<button class="tab" class:active={activeTab === 'style'} onclick={() => (activeTab = 'style')}>
			Style
		</button>
		<button
			class="tab"
			class:active={activeTab === 'advanced'}
			onclick={() => (activeTab = 'advanced')}
		>
			Advanced
		</button>
		<button
			class="tab"
			class:active={activeTab === 'responsive'}
			onclick={() => (activeTab = 'responsive')}
		>
			Responsive
		</button>
	</div>

	<div class="tab-content">
		{#if activeTab === 'style'}
			<!-- Heading Level Selector (for heading blocks) -->
			{#if block.type === 'heading'}
				<div class="section heading-level-section">
					<div class="section-content" style="padding: 0.75rem 1rem;">
						<span class="field-label">Heading Level</span>
						<div class="heading-level-buttons">
							{#each [1, 2, 3, 4, 5, 6] as lvl}
								<button
									type="button"
									class="level-btn"
									class:active={block.settings.level === lvl}
									onclick={() => updateSetting('level', lvl)}
								>
									H{lvl}
								</button>
							{/each}
						</div>
					</div>
				</div>
			{/if}

			<!-- Typography Section -->
			<div class="section">
				<button class="section-header" onclick={() => toggleSection('typography')}>
					<span>Typography</span>
					<span class="chevron" class:expanded={expandedSections.has('typography')}>▼</span>
				</button>
				{#if expandedSections.has('typography')}
					<div class="section-content">
						<div class="field">
							<label>
								Font Family
								<select
									value={block.settings.fontFamily || 'inherit'}
									onchange={(e: Event) =>
										updateSetting('fontFamily', (e.currentTarget as HTMLInputElement).value)}
								>
									{#each fontFamilies as font}
										<option value={font.value}>{font.label}</option>
									{/each}
								</select>
							</label>
						</div>

						<div class="field-row">
							<div class="field">
								<label>
									Size
									<div class="input-with-unit">
										<input
											type="number"
											min="8"
											max="200"
											value={parseInt(block.settings.fontSize || '16')}
											onchange={(e: Event) =>
												updateSetting(
													'fontSize',
													`${(e.currentTarget as HTMLInputElement).value}px`
												)}
										/>
										<span class="unit">px</span>
									</div>
								</label>
							</div>
							<div class="field">
								<label>
									Weight
									<select
										value={block.settings.fontWeight || '400'}
										onchange={(e: Event) =>
											updateSetting('fontWeight', (e.currentTarget as HTMLInputElement).value)}
									>
										{#each fontWeights as weight}
											<option value={weight.value}>{weight.label}</option>
										{/each}
									</select>
								</label>
							</div>
						</div>

						<div class="field-row">
							<div class="field">
								<label>
									Line Height
									<input
										type="number"
										min="0.5"
										max="3"
										step="0.1"
										value={parseFloat(block.settings.lineHeight || '1.6')}
										onchange={(e: Event) =>
											updateSetting('lineHeight', (e.currentTarget as HTMLInputElement).value)}
									/>
								</label>
							</div>
							<div class="field">
								<label>
									Letter Spacing
									<div class="input-with-unit">
										<input
											type="number"
											min="-5"
											max="20"
											step="0.5"
											value={parseFloat(block.settings.letterSpacing || '0')}
											onchange={(e: Event) =>
												updateSetting(
													'letterSpacing',
													`${(e.currentTarget as HTMLInputElement).value}px`
												)}
										/>
										<span class="unit">px</span>
									</div>
								</label>
							</div>
						</div>

						<div class="field">
							<span class="field-label">Text Alignment</span>
							<div class="button-group" role="group" aria-label="Text alignment">
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
							<span class="field-label">Text Transform</span>
							<div class="button-group" role="group" aria-label="Text transform">
								<button
									class="text-btn"
									class:active={!block.settings.textTransform ||
										block.settings.textTransform === 'none'}
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
				<button class="section-header" onclick={() => toggleSection('colors')}>
					<span>Colors</span>
					<span class="chevron" class:expanded={expandedSections.has('colors')}>▼</span>
				</button>
				{#if expandedSections.has('colors')}
					<div class="section-content">
						<div class="field">
							<label>
								Text Color
								<div class="color-picker-trigger">
									<button
										type="button"
										class="color-swatch"
										style="background-color: {block.settings.textColor || '#000000'}"
										onclick={() => (showColorPicker = showColorPicker === 'text' ? null : 'text')}
										aria-label="Pick text color"
									></button>
									<input
										type="text"
										value={block.settings.textColor || '#000000'}
										onchange={(e: Event) =>
											updateSetting('textColor', (e.currentTarget as HTMLInputElement).value)}
									/>
								</div>
							</label>
							{#if showColorPicker === 'text'}
								<div class="color-palette">
									{#each presetColors as color}
										<button
											type="button"
											class="preset-color"
											style="background-color: {color}"
											onclick={() => {
												updateSetting('textColor', color);
												showColorPicker = null;
											}}
											aria-label="Select color {color}"
										></button>
									{/each}
								</div>
							{/if}
						</div>

						<div class="field">
							<label>
								Background Color
								<div class="color-picker-trigger">
									<button
										type="button"
										class="color-swatch"
										style="background-color: {block.settings.backgroundColor || 'transparent'}"
										onclick={() => (showColorPicker = showColorPicker === 'bg' ? null : 'bg')}
										aria-label="Pick background color"
									></button>
									<input
										type="text"
										value={block.settings.backgroundColor || 'transparent'}
										onchange={(e: Event) =>
											updateSetting('backgroundColor', (e.currentTarget as HTMLInputElement).value)}
									/>
								</div>
							</label>
							{#if showColorPicker === 'bg'}
								<div class="color-palette">
									{#each presetColors as color}
										<button
											type="button"
											class="preset-color"
											style="background-color: {color}"
											onclick={() => {
												updateSetting('backgroundColor', color);
												showColorPicker = null;
											}}
											aria-label="Select color {color}"
										></button>
									{/each}
								</div>
							{/if}
						</div>

						<div class="field">
							<label>
								Background Gradient
								<input
									type="text"
									placeholder="linear-gradient(135deg, #667eea 0%, #764ba2 100%)"
									value={block.settings.backgroundGradient || ''}
									onchange={(e: Event) =>
										updateSetting(
											'backgroundGradient',
											(e.currentTarget as HTMLInputElement).value
										)}
								/>
							</label>
						</div>
					</div>
				{/if}
			</div>

			<!-- Spacing Section -->
			<div class="section">
				<button class="section-header" onclick={() => toggleSection('spacing')}>
					<span>Spacing</span>
					<span class="chevron" class:expanded={expandedSections.has('spacing')}>▼</span>
				</button>
				{#if expandedSections.has('spacing')}
					<div class="section-content">
						<div class="spacing-control">
							<span class="field-label">Padding</span>
							<div class="spacing-grid" role="group" aria-label="Padding values">
								<div class="spacing-top">
									<input
										type="number"
										min="0"
										aria-label="Padding top"
										value={parseInt(paddingValues.top) || 0}
										onchange={(e: Event) => {
											const newValues = {
												...paddingValues,
												top: `${(e.currentTarget as HTMLInputElement).value}px`
											};
											updateSetting('padding', buildSpacing(newValues));
										}}
									/>
								</div>
								<div class="spacing-left">
									<input
										type="number"
										min="0"
										aria-label="Padding left"
										value={parseInt(paddingValues.left) || 0}
										onchange={(e: Event) => {
											const newValues = {
												...paddingValues,
												left: `${(e.currentTarget as HTMLInputElement).value}px`
											};
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
										aria-label="Padding right"
										value={parseInt(paddingValues.right) || 0}
										onchange={(e: Event) => {
											const newValues = {
												...paddingValues,
												right: `${(e.currentTarget as HTMLInputElement).value}px`
											};
											updateSetting('padding', buildSpacing(newValues));
										}}
									/>
								</div>
								<div class="spacing-bottom">
									<input
										type="number"
										min="0"
										aria-label="Padding bottom"
										value={parseInt(paddingValues.bottom) || 0}
										onchange={(e: Event) => {
											const newValues = {
												...paddingValues,
												bottom: `${(e.currentTarget as HTMLInputElement).value}px`
											};
											updateSetting('padding', buildSpacing(newValues));
										}}
									/>
								</div>
							</div>
						</div>

						<div class="spacing-control">
							<span class="field-label">Margin</span>
							<div class="spacing-grid" role="group" aria-label="Margin values">
								<div class="spacing-top">
									<input
										type="number"
										aria-label="Margin top"
										value={parseInt(marginValues.top) || 0}
										onchange={(e: Event) => {
											const newValues = {
												...marginValues,
												top: `${(e.currentTarget as HTMLInputElement).value}px`
											};
											updateSetting('margin', buildSpacing(newValues));
										}}
									/>
								</div>
								<div class="spacing-left">
									<input
										type="number"
										aria-label="Margin left"
										value={parseInt(marginValues.left) || 0}
										onchange={(e: Event) => {
											const newValues = {
												...marginValues,
												left: `${(e.currentTarget as HTMLInputElement).value}px`
											};
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
										aria-label="Margin right"
										value={parseInt(marginValues.right) || 0}
										onchange={(e: Event) => {
											const newValues = {
												...marginValues,
												right: `${(e.currentTarget as HTMLInputElement).value}px`
											};
											updateSetting('margin', buildSpacing(newValues));
										}}
									/>
								</div>
								<div class="spacing-bottom">
									<input
										type="number"
										aria-label="Margin bottom"
										value={parseInt(marginValues.bottom) || 0}
										onchange={(e: Event) => {
											const newValues = {
												...marginValues,
												bottom: `${(e.currentTarget as HTMLInputElement).value}px`
											};
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
				<button class="section-header" onclick={() => toggleSection('border')}>
					<span>Border</span>
					<span class="chevron" class:expanded={expandedSections.has('border')}>▼</span>
				</button>
				{#if expandedSections.has('border')}
					<div class="section-content">
						<div class="field-row">
							<div class="field">
								<label>
									Style
									<select
										value={block.settings.borderStyle || 'none'}
										onchange={(e: Event) =>
											updateSetting('borderStyle', (e.currentTarget as HTMLInputElement).value)}
									>
										{#each borderStyles as style}
											<option value={style.value}>{style.label}</option>
										{/each}
									</select>
								</label>
							</div>
							<div class="field">
								<label>
									Width
									<div class="input-with-unit">
										<input
											type="number"
											min="0"
											max="20"
											value={parseInt(block.settings.borderWidth || '0')}
											onchange={(e: Event) =>
												updateSetting(
													'borderWidth',
													`${(e.currentTarget as HTMLInputElement).value}px`
												)}
										/>
										<span class="unit">px</span>
									</div>
								</label>
							</div>
						</div>

						<div class="field">
							<label>
								Border Color
								<div class="color-picker-trigger">
									<button
										type="button"
										class="color-swatch"
										style="background-color: {block.settings.borderColor || '#CCCCCC'}"
										onclick={() =>
											(showColorPicker = showColorPicker === 'border' ? null : 'border')}
										aria-label="Pick border color"
									></button>
									<input
										type="text"
										value={block.settings.borderColor || '#CCCCCC'}
										onchange={(e: Event) =>
											updateSetting('borderColor', (e.currentTarget as HTMLInputElement).value)}
									/>
								</div>
							</label>
							{#if showColorPicker === 'border'}
								<div class="color-palette">
									{#each presetColors as color}
										<button
											type="button"
											class="preset-color"
											style="background-color: {color}"
											onclick={() => {
												updateSetting('borderColor', color);
												showColorPicker = null;
											}}
											aria-label="Select color {color}"
										></button>
									{/each}
								</div>
							{/if}
						</div>

						<div class="field">
							<label>
								Border Radius
								<div class="input-with-unit">
									<input
										type="number"
										min="0"
										max="100"
										value={parseInt(block.settings.borderRadius || '0')}
										onchange={(e: Event) =>
											updateSetting(
												'borderRadius',
												`${(e.currentTarget as HTMLInputElement).value}px`
											)}
									/>
									<span class="unit">px</span>
								</div>
							</label>
						</div>

						<div class="field">
							<label>
								Box Shadow
								<input
									type="text"
									placeholder="0 4px 6px rgba(0, 0, 0, 0.1)"
									value={block.settings.boxShadow || ''}
									onchange={(e: Event) =>
										updateSetting('boxShadow', (e.currentTarget as HTMLInputElement).value)}
								/>
							</label>
						</div>
					</div>
				{/if}
			</div>
		{:else if activeTab === 'advanced'}
			<!-- Animation Section -->
			<div class="section">
				<button class="section-header" onclick={() => toggleSection('animation')}>
					<span>Animation</span>
					<span class="chevron" class:expanded={expandedSections.has('animation')}>▼</span>
				</button>
				{#if expandedSections.has('animation')}
					<div class="section-content">
						<div class="field">
							<label>
								Entrance Animation
								<select
									value={block.settings.animation || 'none'}
									onchange={(e: Event) =>
										updateSetting('animation', (e.currentTarget as HTMLInputElement).value)}
								>
									{#each animations as anim}
										<option value={anim.value}>{anim.label}</option>
									{/each}
								</select>
							</label>
						</div>

						<div class="field-row">
							<div class="field">
								<label>
									Duration
									<div class="input-with-unit">
										<input
											type="number"
											min="0"
											max="5"
											step="0.1"
											value={parseFloat(block.settings.animationDuration || '0.5')}
											onchange={(e: Event) =>
												updateSetting(
													'animationDuration',
													`${(e.currentTarget as HTMLInputElement).value}s`
												)}
										/>
										<span class="unit">s</span>
									</div>
								</label>
							</div>
							<div class="field">
								<label>
									Delay
									<div class="input-with-unit">
										<input
											type="number"
											min="0"
											max="5"
											step="0.1"
											value={parseFloat(block.settings.animationDelay || '0')}
											onchange={(e: Event) =>
												updateSetting(
													'animationDelay',
													`${(e.currentTarget as HTMLInputElement).value}s`
												)}
										/>
										<span class="unit">s</span>
									</div>
								</label>
							</div>
						</div>
					</div>
				{/if}
			</div>

			<!-- Transform Section -->
			<div class="section">
				<button class="section-header" onclick={() => toggleSection('transform')}>
					<span>Transform</span>
					<span class="chevron" class:expanded={expandedSections.has('transform')}>▼</span>
				</button>
				{#if expandedSections.has('transform')}
					<div class="section-content">
						<div class="field-row">
							<div class="field">
								<label>
									Rotate
									<div class="input-with-unit">
										<input
											type="number"
											min="-360"
											max="360"
											value={parseInt(block.settings.rotate || '0')}
											onchange={(e: Event) =>
												updateSetting(
													'rotate',
													`${(e.currentTarget as HTMLInputElement).value}deg`
												)}
										/>
										<span class="unit">°</span>
									</div>
								</label>
							</div>
							<div class="field">
								<label>
									Scale
									<input
										type="number"
										min="0"
										max="3"
										step="0.1"
										value={parseFloat(block.settings.scale || '1')}
										onchange={(e: Event) =>
											updateSetting('scale', (e.currentTarget as HTMLInputElement).value)}
									/>
								</label>
							</div>
						</div>

						<div class="field-row">
							<div class="field">
								<label>
									Translate X
									<div class="input-with-unit">
										<input
											type="number"
											value={parseInt(block.settings.translateX || '0')}
											onchange={(e: Event) =>
												updateSetting(
													'translateX',
													`${(e.currentTarget as HTMLInputElement).value}px`
												)}
										/>
										<span class="unit">px</span>
									</div>
								</label>
							</div>
							<div class="field">
								<label>
									Translate Y
									<div class="input-with-unit">
										<input
											type="number"
											value={parseInt(block.settings.translateY || '0')}
											onchange={(e: Event) =>
												updateSetting(
													'translateY',
													`${(e.currentTarget as HTMLInputElement).value}px`
												)}
										/>
										<span class="unit">px</span>
									</div>
								</label>
							</div>
						</div>

						<div class="field-row">
							<div class="field">
								<label>
									Skew X
									<div class="input-with-unit">
										<input
											type="number"
											min="-45"
											max="45"
											value={parseInt(block.settings.skewX || '0')}
											onchange={(e: Event) =>
												updateSetting('skewX', `${(e.currentTarget as HTMLInputElement).value}deg`)}
										/>
										<span class="unit">°</span>
									</div>
								</label>
							</div>
							<div class="field">
								<label>
									Skew Y
									<div class="input-with-unit">
										<input
											type="number"
											min="-45"
											max="45"
											value={parseInt(block.settings.skewY || '0')}
											onchange={(e: Event) =>
												updateSetting('skewY', `${(e.currentTarget as HTMLInputElement).value}deg`)}
										/>
										<span class="unit">°</span>
									</div>
								</label>
							</div>
						</div>
					</div>
				{/if}
			</div>

			<!-- Effects Section -->
			<div class="section">
				<button class="section-header" onclick={() => toggleSection('effects')}>
					<span>Effects</span>
					<span class="chevron" class:expanded={expandedSections.has('effects')}>▼</span>
				</button>
				{#if expandedSections.has('effects')}
					<div class="section-content">
						<div class="field">
							<label>
								Opacity
								<div class="slider-field">
									<input
										type="range"
										min="0"
										max="1"
										step="0.01"
										value={block.settings.opacity || 1}
										oninput={(e: Event) =>
											updateSetting(
												'opacity',
												parseFloat((e.currentTarget as HTMLInputElement).value)
											)}
									/>
									<span class="value">{Math.round((block.settings.opacity || 1) * 100)}%</span>
								</div>
							</label>
						</div>

						<div class="field">
							<label>
								Blend Mode
								<select
									value={block.settings.blendMode || 'normal'}
									onchange={(e: Event) =>
										updateSetting('blendMode', (e.currentTarget as HTMLInputElement).value)}
								>
									{#each blendModes as mode}
										<option value={mode}>{mode}</option>
									{/each}
								</select>
							</label>
						</div>

						<div class="field">
							<span class="field-label">CSS Filters</span>
							<div class="filter-controls">
								<div class="filter-item">
									<span>Blur</span>
									<input
										type="range"
										min="0"
										max="20"
										value={parseInt(String(block.settings.filterBlur || '0'))}
										oninput={(e: Event) =>
											updateSetting(
												'filterBlur',
												`${(e.currentTarget as HTMLInputElement).value}px`
											)}
									/>
									<span class="value">{block.settings.filterBlur || '0px'}</span>
								</div>
								<div class="filter-item">
									<span>Brightness</span>
									<input
										type="range"
										min="0"
										max="200"
										value={parseInt(String(block.settings.filterBrightness || '100'))}
										oninput={(e: Event) =>
											updateSetting(
												'filterBrightness',
												`${(e.currentTarget as HTMLInputElement).value}%`
											)}
									/>
									<span class="value">{block.settings.filterBrightness || '100%'}</span>
								</div>
								<div class="filter-item">
									<span>Contrast</span>
									<input
										type="range"
										min="0"
										max="200"
										value={parseInt(String(block.settings.filterContrast || '100'))}
										oninput={(e: Event) =>
											updateSetting(
												'filterContrast',
												`${(e.currentTarget as HTMLInputElement).value}%`
											)}
									/>
									<span class="value">{block.settings.filterContrast || '100%'}</span>
								</div>
								<div class="filter-item">
									<span>Grayscale</span>
									<input
										type="range"
										min="0"
										max="100"
										value={parseInt(String(block.settings.filterGrayscale || '0'))}
										oninput={(e: Event) =>
											updateSetting(
												'filterGrayscale',
												`${(e.currentTarget as HTMLInputElement).value}%`
											)}
									/>
									<span class="value">{block.settings.filterGrayscale || '0%'}</span>
								</div>
								<div class="filter-item">
									<span>Saturate</span>
									<input
										type="range"
										min="0"
										max="200"
										value={parseInt(String(block.settings.filterSaturate || '100'))}
										oninput={(e: Event) =>
											updateSetting(
												'filterSaturate',
												`${(e.currentTarget as HTMLInputElement).value}%`
											)}
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
				<button class="section-header" onclick={() => toggleSection('customCss')}>
					<span>Custom CSS</span>
					<span class="chevron" class:expanded={expandedSections.has('customCss')}>▼</span>
				</button>
				{#if expandedSections.has('customCss')}
					<div class="section-content">
						<div class="field">
							<label>
								CSS Class
								<input
									type="text"
									placeholder="my-custom-class another-class"
									value={block.settings.customClass || ''}
									onchange={(e: Event) =>
										updateSetting('customClass', (e.currentTarget as HTMLInputElement).value)}
								/>
							</label>
						</div>

						<div class="field">
							<label>
								CSS ID
								<input
									type="text"
									placeholder="my-element-id"
									value={block.settings.customId || ''}
									onchange={(e: Event) =>
										updateSetting('customId', (e.currentTarget as HTMLInputElement).value)}
								/>
							</label>
						</div>

						<div class="field">
							<label>
								Custom CSS
								<textarea
									rows="6"
									placeholder=".selector &#123;
  property: value;
&#125;"
									value={block.settings.customCSS || ''}
									onchange={(e: Event) =>
										updateSetting('customCSS', (e.currentTarget as HTMLInputElement).value)}
								></textarea>
							</label>
						</div>
					</div>
				{/if}
			</div>

			<!-- Datasource Section (for dropdown/select blocks) -->
			<div class="section">
				<button class="section-header" onclick={() => toggleSection('datasource')}>
					<span>Datasource Options</span>
					<span class="chevron" class:expanded={expandedSections.has('datasource')}>▼</span>
				</button>
				{#if expandedSections.has('datasource')}
					<div class="section-content">
						<p class="section-info">
							Connect dropdown fields to reusable datasources for dynamic options.
						</p>

						<div class="field">
							<label>
								Option Source
								<select
									value={block.settings.optionSource || 'static'}
									onchange={(e: Event) =>
										updateSetting('optionSource', (e.currentTarget as HTMLSelectElement).value)}
								>
									<option value="static">Static Options</option>
									<option value="datasource">From Datasource</option>
								</select>
							</label>
						</div>

						{#if block.settings.optionSource === 'datasource'}
							<div class="field">
								<label>
									Select Datasource
									{#if loadingDatasources}
										<span class="loading-indicator">Loading...</span>
									{/if}
									<select
										value={block.settings.datasourceSlug || ''}
										onchange={async (e: Event) => {
											const slug = (e.currentTarget as HTMLSelectElement).value;
											updateSetting('datasourceSlug', slug);
											if (slug) {
												await fetchDatasourceEntries(slug);
											}
										}}
									>
										<option value="">-- Select a datasource --</option>
										{#each datasources as ds}
											<option value={ds.slug}>
												{ds.name} ({ds.entry_count} entries)
											</option>
										{/each}
									</select>
								</label>
							</div>

							{#if block.settings.datasourceSlug}
								<div class="datasource-preview">
									<span class="field-label">Preview Options:</span>
									{#if loadingEntries[block.settings.datasourceSlug]}
										<span class="loading-indicator">Loading entries...</span>
									{:else if datasourceEntries[block.settings.datasourceSlug]?.length > 0}
										<div class="preview-entries">
											{#each datasourceEntries[block.settings.datasourceSlug].slice(0, 5) as entry}
												<span class="preview-entry">
													{entry.name} <code>({entry.value})</code>
												</span>
											{/each}
											{#if datasourceEntries[block.settings.datasourceSlug].length > 5}
												<span class="preview-more">
													+{datasourceEntries[block.settings.datasourceSlug].length - 5} more
												</span>
											{/if}
										</div>
									{:else}
										<span class="no-entries">No entries in this datasource</span>
									{/if}
								</div>

								<div class="field">
									<label>
										Dimension (for translations)
										<input
											type="text"
											placeholder="default"
											value={block.settings.datasourceDimension || 'default'}
											onchange={(e: Event) =>
												updateSetting(
													'datasourceDimension',
													(e.currentTarget as HTMLInputElement).value
												)}
										/>
									</label>
									<p class="help-text">Use 'en', 'de', 'fr', etc. for localized options</p>
								</div>

								<div class="field">
									<label class="checkbox-label-inline">
										<input
											type="checkbox"
											checked={block.settings.datasourceIncludeEmpty !== false}
											onchange={(e: Event) =>
												updateSetting(
													'datasourceIncludeEmpty',
													(e.currentTarget as HTMLInputElement).checked
												)}
										/>
										<span>Include empty "Select..." option</span>
									</label>
								</div>
							{/if}
						{/if}

						{#if block.settings.optionSource !== 'datasource'}
							<div class="field">
								<label>
									Static Options (one per line)
									<textarea
										rows="4"
										placeholder="Option 1|value1&#10;Option 2|value2&#10;Option 3|value3"
										value={block.settings.staticOptions || ''}
										onchange={(e: Event) =>
											updateSetting(
												'staticOptions',
												(e.currentTarget as HTMLTextAreaElement).value
											)}
									></textarea>
								</label>
								<p class="help-text">Format: Label|value (one per line)</p>
							</div>
						{/if}
					</div>
				{/if}
			</div>
		{:else if activeTab === 'responsive'}
			<!-- Visibility Section -->
			<div class="section">
				<button class="section-header" onclick={() => toggleSection('visibility')}>
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
									onchange={(e: Event) =>
										updateSetting('hideOnDesktop', !(e.currentTarget as HTMLInputElement).checked)}
								/>
								<span class="device-icon">🖥️</span>
								<span>Show on Desktop</span>
							</label>
							<label class="checkbox-label">
								<input
									type="checkbox"
									checked={block.settings.hideOnTablet !== true}
									onchange={(e: Event) =>
										updateSetting('hideOnTablet', !(e.currentTarget as HTMLInputElement).checked)}
								/>
								<span class="device-icon">📱</span>
								<span>Show on Tablet</span>
							</label>
							<label class="checkbox-label">
								<input
									type="checkbox"
									checked={block.settings.hideOnMobile !== true}
									onchange={(e: Event) =>
										updateSetting('hideOnMobile', !(e.currentTarget as HTMLInputElement).checked)}
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
				<button class="section-header" onclick={() => toggleSection('responsive')}>
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
									<label>
										Font Size
										<input
											type="text"
											placeholder="e.g., 14px"
											value={block.settings.tabletFontSize || ''}
											onchange={(e: Event) =>
												updateSetting(
													'tabletFontSize',
													(e.currentTarget as HTMLInputElement).value
												)}
										/>
									</label>
								</div>
								<div class="field">
									<label>
										Padding
										<input
											type="text"
											placeholder="e.g., 10px 15px"
											value={block.settings.tabletPadding || ''}
											onchange={(e: Event) =>
												updateSetting('tabletPadding', (e.currentTarget as HTMLInputElement).value)}
										/>
									</label>
								</div>
							</div>
						</div>

						<div class="responsive-device">
							<h4>📲 Mobile (&lt; 768px)</h4>
							<div class="field-row">
								<div class="field">
									<label>
										Font Size
										<input
											type="text"
											placeholder="e.g., 12px"
											value={block.settings.mobileFontSize || ''}
											onchange={(e: Event) =>
												updateSetting(
													'mobileFontSize',
													(e.currentTarget as HTMLInputElement).value
												)}
										/>
									</label>
								</div>
								<div class="field">
									<label>
										Padding
										<input
											type="text"
											placeholder="e.g., 8px 10px"
											value={block.settings.mobilePadding || ''}
											onchange={(e: Event) =>
												updateSetting('mobilePadding', (e.currentTarget as HTMLInputElement).value)}
										/>
									</label>
								</div>
							</div>
						</div>
					</div>
				{/if}
			</div>

			<!-- Z-Index Section -->
			<div class="section">
				<button class="section-header" onclick={() => toggleSection('zindex')}>
					<span>Position & Z-Index</span>
					<span class="chevron" class:expanded={expandedSections.has('zindex')}>▼</span>
				</button>
				{#if expandedSections.has('zindex')}
					<div class="section-content">
						<div class="field">
							<label>
								Z-Index
								<input
									type="number"
									min="-999"
									max="9999"
									value={block.settings.zIndex || 0}
									onchange={(e: Event) =>
										updateSetting('zIndex', parseInt((e.currentTarget as HTMLInputElement).value))}
								/>
							</label>
						</div>

						<div class="field">
							<label>
								Position
								<select
									value={block.settings.position || 'static'}
									onchange={(e: Event) =>
										updateSetting('position', (e.currentTarget as HTMLInputElement).value)}
								>
									<option value="static">Static</option>
									<option value="relative">Relative</option>
									<option value="absolute">Absolute</option>
									<option value="fixed">Fixed</option>
									<option value="sticky">Sticky</option>
								</select>
							</label>
						</div>

						{#if block.settings.position && block.settings.position !== 'static'}
							<div class="field-row">
								<div class="field">
									<label>
										Top
										<input
											type="text"
											placeholder="auto"
											value={block.settings.positionTop || ''}
											onchange={(e: Event) =>
												updateSetting('positionTop', (e.currentTarget as HTMLInputElement).value)}
										/>
									</label>
								</div>
								<div class="field">
									<label>
										Right
										<input
											type="text"
											placeholder="auto"
											value={block.settings.positionRight || ''}
											onchange={(e: Event) =>
												updateSetting('positionRight', (e.currentTarget as HTMLInputElement).value)}
										/>
									</label>
								</div>
							</div>
							<div class="field-row">
								<div class="field">
									<label>
										Bottom
										<input
											type="text"
											placeholder="auto"
											value={block.settings.positionBottom || ''}
											onchange={(e: Event) =>
												updateSetting(
													'positionBottom',
													(e.currentTarget as HTMLInputElement).value
												)}
										/>
									</label>
								</div>
								<div class="field">
									<label>
										Left
										<input
											type="text"
											placeholder="auto"
											value={block.settings.positionLeft || ''}
											onchange={(e: Event) =>
												updateSetting('positionLeft', (e.currentTarget as HTMLInputElement).value)}
										/>
									</label>
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

	.field label,
	.field-label {
		display: block;
		margin-bottom: 0.375rem;
		font-size: 0.75rem;
		font-weight: 500;
		color: var(--text-secondary, #6b7280);
		text-transform: uppercase;
		letter-spacing: 0.025em;
	}

	.field input[type='text'],
	.field input[type='number'],
	.field select,
	.field textarea {
		width: 100%;
		padding: 0.5rem 0.75rem;
		font-size: 0.875rem;
		border: 1px solid var(--border-color, #e5e7eb);
		border-radius: 0.375rem;
		background: var(--bg-primary, #ffffff);
		color: var(--text-primary, #1f2937);
		transition:
			border-color 0.2s,
			box-shadow 0.2s;
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

	.spacing-control > .field-label {
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

	.slider-field input[type='range'] {
		flex: 1;
		height: 4px;
		-webkit-appearance: none;
		appearance: none;
		background: var(--border-color, #e5e7eb);
		border-radius: 2px;
		cursor: pointer;
	}

	.slider-field input[type='range']::-webkit-slider-thumb {
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

	.filter-item input[type='range'] {
		width: 100%;
		height: 4px;
		-webkit-appearance: none;
		appearance: none;
		background: var(--border-color, #e5e7eb);
		border-radius: 2px;
	}

	.filter-item input[type='range']::-webkit-slider-thumb {
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

	.checkbox-label input[type='checkbox'] {
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

	/* Heading Level Selector */
	.heading-level-section {
		background: linear-gradient(135deg, #3b82f610 0%, #8b5cf610 100%);
		border-color: #3b82f630;
	}

	.heading-level-buttons {
		display: flex;
		gap: 0.375rem;
		margin-top: 0.5rem;
	}

	.heading-level-buttons .level-btn {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 0.5rem;
		background: white;
		border: 1px solid var(--border-color, #e5e7eb);
		border-radius: 0.375rem;
		font-size: 0.8125rem;
		font-weight: 600;
		color: #666;
		cursor: pointer;
		transition: all 0.15s;
	}

	.heading-level-buttons .level-btn:hover {
		background: #f3f4f6;
		color: #1a1a1a;
	}

	.heading-level-buttons .level-btn.active {
		background: #3b82f6;
		border-color: #3b82f6;
		color: white;
	}

	/* Datasource Section Styles */
	.section-info {
		font-size: 0.75rem;
		color: var(--text-secondary, #6b7280);
		margin: 0 0 1rem 0;
		line-height: 1.5;
		padding: 0.75rem;
		background: rgba(59, 130, 246, 0.05);
		border-radius: 0.375rem;
		border-left: 3px solid var(--primary, #3b82f6);
	}

	.loading-indicator {
		display: inline-block;
		font-size: 0.7rem;
		color: var(--text-tertiary, #9ca3af);
		margin-left: 0.5rem;
	}

	.datasource-preview {
		margin: 1rem 0;
		padding: 0.75rem;
		background: var(--bg-secondary, #f9fafb);
		border-radius: 0.375rem;
		border: 1px solid var(--border-color, #e5e7eb);
	}

	.datasource-preview .field-label {
		display: block;
		font-size: 0.7rem;
		font-weight: 600;
		color: var(--text-secondary, #6b7280);
		text-transform: uppercase;
		margin-bottom: 0.5rem;
	}

	.preview-entries {
		display: flex;
		flex-direction: column;
		gap: 0.375rem;
	}

	.preview-entry {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.8rem;
		color: var(--text-primary, #1f2937);
	}

	.preview-entry code {
		font-size: 0.7rem;
		padding: 0.125rem 0.375rem;
		background: rgba(99, 102, 241, 0.1);
		color: var(--primary, #3b82f6);
		border-radius: 0.25rem;
	}

	.preview-more {
		font-size: 0.75rem;
		color: var(--text-tertiary, #9ca3af);
		font-style: italic;
		margin-top: 0.25rem;
	}

	.no-entries {
		font-size: 0.8rem;
		color: var(--text-tertiary, #9ca3af);
		font-style: italic;
	}

	.help-text {
		font-size: 0.7rem;
		color: var(--text-tertiary, #9ca3af);
		margin: 0.25rem 0 0 0;
	}

	.checkbox-label-inline {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		cursor: pointer;
		margin-top: 0.5rem;
	}

	.checkbox-label-inline input[type='checkbox'] {
		width: 16px;
		height: 16px;
		accent-color: var(--primary, #3b82f6);
	}

	.checkbox-label-inline span {
		font-size: 0.85rem;
		color: var(--text-primary, #1f2937);
	}
</style>

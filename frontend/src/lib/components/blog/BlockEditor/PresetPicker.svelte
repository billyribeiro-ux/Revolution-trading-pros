<!--
/**
 * Preset Picker - Storyblok-Style Component Presets/Templates
 * ═══════════════════════════════════════════════════════════════════════════
 * Advanced preset selection UI for quick block configuration
 *
 * Features:
 * - Grid of preset thumbnails with hover preview
 * - Category organization (default, brand, seasonal, etc.)
 * - Search and filtering
 * - Apply preset to current block
 * - Save current block as preset
 *
 * @version 1.0.0 - February 2026
 * @author Revolution Trading Pros
 */
-->

<script lang="ts">
	import { fade, fly, scale } from 'svelte/transition';
	import { createEventDispatcher, onMount } from 'svelte';
	import IconSearch from '@tabler/icons-svelte-runes/icons/search';
	import IconX from '@tabler/icons-svelte-runes/icons/x';
	import IconPlus from '@tabler/icons-svelte-runes/icons/plus';
	import IconBookmark from '@tabler/icons-svelte-runes/icons/bookmark';
	import IconBookmarkFilled from '@tabler/icons-svelte-runes/icons/bookmark-filled';
	import IconStar from '@tabler/icons-svelte-runes/icons/star';
	import IconStarFilled from '@tabler/icons-svelte-runes/icons/star-filled';
	import IconChartBar from '@tabler/icons-svelte-runes/icons/chart-bar';
	import IconTag from '@tabler/icons-svelte-runes/icons/tag';
	import IconPalette from '@tabler/icons-svelte-runes/icons/palette';
	import IconCalendar from '@tabler/icons-svelte-runes/icons/calendar';
	import IconBriefcase from '@tabler/icons-svelte-runes/icons/briefcase';
	import IconSettings from '@tabler/icons-svelte-runes/icons/settings';
	import IconDeviceFloppy from '@tabler/icons-svelte-runes/icons/device-floppy';
	import IconTemplate from '@tabler/icons-svelte-runes/icons/template';

	import type { Block, BlockContent, BlockSettings, BlockType } from './types';

	// ==========================================================================
	// Types
	// ==========================================================================

	interface PresetSummary {
		id: string;
		block_type: string;
		name: string;
		slug: string;
		description: string | null;
		thumbnail_url: string | null;
		thumbnail_blurhash: string | null;
		category: string;
		tags: string[] | null;
		is_default: boolean;
		is_locked: boolean;
		is_global: boolean;
		usage_count: number;
		created_at: string;
		updated_at: string;
	}

	interface PresetData {
		content: Partial<BlockContent>;
		settings: Partial<BlockSettings>;
	}

	interface FullPreset extends PresetSummary {
		preset_data: PresetData;
	}

	interface PresetsByCategory {
		category: string;
		presets: PresetSummary[];
	}

	interface GroupedPresetsResponse {
		block_type: string;
		categories: PresetsByCategory[];
		total_count: number;
	}

	// ==========================================================================
	// Props
	// ==========================================================================

	interface Props {
		/** Block type to show presets for */
		blockType: BlockType;
		/** Current block (for save as preset) */
		currentBlock?: Block | null;
		/** Show in modal mode */
		isModal?: boolean;
		/** Modal position */
		position?: { x: number; y: number } | null;
		/** Show save as preset option */
		showSaveOption?: boolean;
	}

	let props: Props = $props();
	const blockType = $derived(props.blockType);
	const currentBlock = $derived(props.currentBlock ?? null);
	const isModal = $derived(props.isModal ?? false);
	const position = $derived(props.position ?? null);
	const showSaveOption = $derived(props.showSaveOption ?? true);

	// ==========================================================================
	// Events
	// ==========================================================================

	const dispatch = createEventDispatcher<{
		select: { preset: FullPreset };
		apply: { content: Partial<BlockContent>; settings: Partial<BlockSettings> };
		close: void;
		saveAsPreset: { block: Block };
	}>();

	// ==========================================================================
	// State
	// ==========================================================================

	let searchQuery = $state('');
	let selectedCategory = $state<string | null>(null);
	let hoveredPreset = $state<PresetSummary | null>(null);
	let previewPreset = $state<FullPreset | null>(null);
	let isLoading = $state(false);
	let error = $state<string | null>(null);
	let groupedPresets = $state<GroupedPresetsResponse | null>(null);
	let showSaveModal = $state(false);
	let savePresetName = $state('');
	let savePresetDescription = $state('');
	let savePresetCategory = $state('custom');

	// ==========================================================================
	// Category Configuration
	// ==========================================================================

	const CATEGORY_CONFIG: Record<string, { name: string; icon: any; color: string }> = {
		default: { name: 'Default', icon: IconStarFilled, color: '#3b82f6' },
		brand: { name: 'Brand', icon: IconPalette, color: '#8b5cf6' },
		trading: { name: 'Trading', icon: IconChartBar, color: '#ef4444' },
		marketing: { name: 'Marketing', icon: IconBriefcase, color: '#10b981' },
		seasonal: { name: 'Seasonal', icon: IconCalendar, color: '#f59e0b' },
		custom: { name: 'Custom', icon: IconSettings, color: '#64748b' }
	};

	// ==========================================================================
	// Computed
	// ==========================================================================

	let filteredCategories = $derived(() => {
		if (!groupedPresets) return [];

		const query = searchQuery.toLowerCase().trim();
		const categories = groupedPresets.categories;

		if (!query && !selectedCategory) return categories;

		return categories
			.map((cat) => ({
				...cat,
				presets: cat.presets.filter((preset) => {
					// Category filter
					if (selectedCategory && cat.category !== selectedCategory) return false;

					// Search filter
					if (query) {
						const matchesName = preset.name.toLowerCase().includes(query);
						const matchesDesc = preset.description?.toLowerCase().includes(query) || false;
						const matchesTags = preset.tags?.some((t) => t.toLowerCase().includes(query)) || false;
						return matchesName || matchesDesc || matchesTags;
					}

					return true;
				})
			}))
			.filter((cat) => cat.presets.length > 0);
	});

	let totalPresets = $derived(() => {
		return filteredCategories().reduce((sum, cat) => sum + cat.presets.length, 0);
	});

	let allCategories = $derived(() => {
		if (!groupedPresets) return [];
		return groupedPresets.categories.map((c) => c.category);
	});

	// ==========================================================================
	// API Functions
	// ==========================================================================

	async function loadPresets() {
		isLoading = true;
		error = null;

		try {
			const response = await fetch(`/api/cms/presets/block/${blockType}`, {
				credentials: 'include'
			});

			if (!response.ok) {
				throw new Error('Failed to load presets');
			}

			groupedPresets = await response.json();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Unknown error';
			console.error('Failed to load presets:', e);
		} finally {
			isLoading = false;
		}
	}

	async function loadFullPreset(presetId: string): Promise<FullPreset | null> {
		try {
			const response = await fetch(`/api/cms/presets/${presetId}`, {
				credentials: 'include'
			});

			if (!response.ok) {
				throw new Error('Failed to load preset');
			}

			return await response.json();
		} catch (e) {
			console.error('Failed to load full preset:', e);
			return null;
		}
	}

	async function applyPreset(preset: PresetSummary) {
		const fullPreset = await loadFullPreset(preset.id);
		if (!fullPreset) {
			error = 'Failed to load preset data';
			return;
		}

		// Track usage
		fetch(`/api/cms/presets/${preset.id}/apply`, {
			method: 'POST',
			credentials: 'include'
		}).catch(console.error);

		dispatch('select', { preset: fullPreset });
		dispatch('apply', {
			content: fullPreset.preset_data.content,
			settings: fullPreset.preset_data.settings
		});
	}

	async function saveBlockAsPreset() {
		if (!currentBlock || !savePresetName.trim()) return;

		try {
			const response = await fetch('/api/cms/presets/save-from-block', {
				method: 'POST',
				credentials: 'include',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({
					block_type: currentBlock.type,
					name: savePresetName.trim(),
					description: savePresetDescription.trim() || null,
					content: currentBlock.content,
					settings: currentBlock.settings,
					category: savePresetCategory
				})
			});

			if (!response.ok) {
				throw new Error('Failed to save preset');
			}

			showSaveModal = false;
			savePresetName = '';
			savePresetDescription = '';
			savePresetCategory = 'custom';

			// Reload presets
			await loadPresets();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to save preset';
		}
	}

	// ==========================================================================
	// Event Handlers
	// ==========================================================================

	function handlePresetClick(preset: PresetSummary) {
		applyPreset(preset);
	}

	function handlePresetHover(preset: PresetSummary) {
		hoveredPreset = preset;
	}

	function handlePresetLeave() {
		hoveredPreset = null;
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') {
			if (showSaveModal) {
				showSaveModal = false;
			} else {
				dispatch('close');
			}
		}
	}

	function handleBlankOption() {
		dispatch('apply', { content: {}, settings: {} });
	}

	function handleSaveAsPreset() {
		if (currentBlock) {
			showSaveModal = true;
			savePresetName = '';
			savePresetDescription = '';
		}
	}

	// ==========================================================================
	// Lifecycle
	// ==========================================================================

	onMount(() => {
		loadPresets();
	});

	// Reload when block type changes
	$effect(() => {
		if (blockType) {
			loadPresets();
		}
	});
</script>

{#if isModal}
	<!-- Modal Overlay -->
	<div
		class="preset-overlay"
		onclick={() => dispatch('close')}
		onkeydown={handleKeydown}
		role="button"
		tabindex="0"
		transition:fade={{ duration: 150 }}
	>
		<div
			class="preset-modal"
			class:positioned={position}
			style:left={position ? `${position.x}px` : 'auto'}
			style:top={position ? `${position.y}px` : 'auto'}
			onclick={(e: MouseEvent) => e.stopPropagation()}
			onkeydown={(e: KeyboardEvent) => e.stopPropagation()}
			role="dialog"
			aria-modal="true"
			aria-label="Select preset"
			tabindex="-1"
			transition:scale={{ duration: 200, start: 0.95 }}
		>
			<div class="preset-header">
				<div class="header-title">
					<IconTemplate size={20} />
					<span>Presets for <strong>{blockType}</strong></span>
				</div>
				<button type="button" class="close-btn" onclick={() => dispatch('close')}>
					<IconX size={20} />
				</button>
			</div>

			<div class="preset-toolbar">
				<div class="search-box">
					<IconSearch size={18} />
					<input
						type="text"
						placeholder="Search presets..."
						bind:value={searchQuery}
						aria-label="Search presets"
					/>
					{#if searchQuery}
						<button type="button" class="clear-search" onclick={() => (searchQuery = '')}>
							<IconX size={16} />
						</button>
					{/if}
				</div>

				{#if showSaveOption && currentBlock}
					<button type="button" class="save-preset-btn" onclick={handleSaveAsPreset}>
						<IconDeviceFloppy size={18} />
						<span>Save as Preset</span>
					</button>
				{/if}
			</div>

			<!-- Category Tabs -->
			{#if allCategories().length > 1}
				<div class="category-tabs">
					<button
						type="button"
						class="category-tab"
						class:active={selectedCategory === null}
						onclick={() => (selectedCategory = null)}
					>
						All
					</button>
					{#each allCategories() as cat}
						{@const config = CATEGORY_CONFIG[cat] || CATEGORY_CONFIG.custom}
						<button
							type="button"
							class="category-tab"
							class:active={selectedCategory === cat}
							style:--tab-color={config.color}
							onclick={() => (selectedCategory = cat)}
						>
							<svelte:component this={config.icon} size={14} />
							{config.name}
						</button>
					{/each}
				</div>
			{/if}

			<div class="preset-content">
				{#if isLoading}
					<div class="loading-state">
						<div class="spinner"></div>
						<p>Loading presets...</p>
					</div>
				{:else if error}
					<div class="error-state">
						<p>{error}</p>
						<button type="button" onclick={loadPresets}>Retry</button>
					</div>
				{:else if totalPresets() === 0}
					<div class="empty-state">
						<IconTemplate size={48} stroke={1.5} />
						<p>No presets found</p>
						{#if searchQuery}
							<span>Try a different search term</span>
						{:else}
							<span>No presets available for this block type</span>
						{/if}
					</div>
				{:else}
					<!-- Blank Option -->
					<div class="blank-option">
						<button type="button" class="blank-btn" onclick={handleBlankOption}>
							<IconPlus size={24} />
							<span>Start Blank</span>
						</button>
					</div>

					<!-- Presets by Category -->
					{#each filteredCategories() as category}
						{@const config = CATEGORY_CONFIG[category.category] || CATEGORY_CONFIG.custom}
						<div class="category-section" transition:fly={{ y: -10, duration: 200 }}>
							<div class="category-header" style:--cat-color={config.color}>
								<span class="category-dot"></span>
								<svelte:component this={config.icon} size={16} />
								<span class="category-name">{config.name}</span>
								<span class="category-count">{category.presets.length}</span>
							</div>

							<div class="presets-grid">
								{#each category.presets as preset}
									<button
										type="button"
										class="preset-card"
										class:is-default={preset.is_default}
										onclick={() => handlePresetClick(preset)}
										onmouseenter={() => handlePresetHover(preset)}
										onmouseleave={handlePresetLeave}
									>
										{#if preset.thumbnail_url}
											<div class="preset-thumbnail">
												<img src={preset.thumbnail_url} alt={preset.name} loading="lazy" />
											</div>
										{:else}
											<div class="preset-thumbnail placeholder">
												<IconTemplate size={32} stroke={1.5} />
											</div>
										{/if}

										<div class="preset-info">
											<span class="preset-name">
												{#if preset.is_default}
													<IconStarFilled size={12} class="default-star" />
												{/if}
												{preset.name}
											</span>
											{#if preset.description}
												<span class="preset-desc">{preset.description}</span>
											{/if}
										</div>

										{#if preset.usage_count > 0}
											<div class="preset-usage" title="{preset.usage_count} uses">
												<IconChartBar size={12} />
												{preset.usage_count}
											</div>
										{/if}
									</button>
								{/each}
							</div>
						</div>
					{/each}
				{/if}
			</div>

			<!-- Hover Preview -->
			{#if hoveredPreset}
				<div class="preset-preview" transition:fade={{ duration: 100 }}>
					<h4>{hoveredPreset.name}</h4>
					{#if hoveredPreset.description}
						<p>{hoveredPreset.description}</p>
					{/if}
					{#if hoveredPreset.tags && hoveredPreset.tags.length > 0}
						<div class="preview-tags">
							{#each hoveredPreset.tags.slice(0, 5) as tag}
								<span class="tag">
									<IconTag size={10} />
									{tag}
								</span>
							{/each}
						</div>
					{/if}
					<div class="preview-meta">
						<span>Used {hoveredPreset.usage_count} times</span>
					</div>
				</div>
			{/if}
		</div>
	</div>
{:else}
	<!-- Inline/Sidebar Mode -->
	<div class="preset-inline">
		<div class="inline-header">
			<IconTemplate size={18} />
			<span>Presets</span>
		</div>

		{#if isLoading}
			<div class="loading-state small">
				<div class="spinner small"></div>
			</div>
		{:else if groupedPresets && groupedPresets.total_count > 0}
			<div class="inline-presets">
				{#each groupedPresets.categories.slice(0, 2) as category}
					{#each category.presets.slice(0, 4) as preset}
						<button
							type="button"
							class="inline-preset-btn"
							onclick={() => handlePresetClick(preset)}
							title={preset.description || preset.name}
						>
							{#if preset.is_default}
								<IconStarFilled size={12} class="default-indicator" />
							{/if}
							{preset.name}
						</button>
					{/each}
				{/each}
			</div>
		{:else}
			<p class="no-presets">No presets available</p>
		{/if}
	</div>
{/if}

<!-- Save as Preset Modal -->
{#if showSaveModal}
	<div
		class="save-modal-overlay"
		onclick={() => (showSaveModal = false)}
		onkeydown={handleKeydown}
		role="button"
		tabindex="0"
		transition:fade={{ duration: 150 }}
	>
		<div
			class="save-modal"
			onclick={(e: MouseEvent) => e.stopPropagation()}
			onkeydown={(e: KeyboardEvent) => e.stopPropagation()}
			role="dialog"
			aria-modal="true"
			tabindex="-1"
			transition:scale={{ duration: 200 }}
		>
			<div class="save-modal-header">
				<h3>Save as Preset</h3>
				<button type="button" class="close-btn" onclick={() => (showSaveModal = false)}>
					<IconX size={20} />
				</button>
			</div>

			<div class="save-modal-content">
				<div class="form-field">
					<label for="preset-name">Preset Name *</label>
					<input
						id="preset-name"
						type="text"
						bind:value={savePresetName}
						placeholder="e.g., Primary CTA Button"
						autofocus
					/>
				</div>

				<div class="form-field">
					<label for="preset-description">Description</label>
					<textarea
						id="preset-description"
						bind:value={savePresetDescription}
						placeholder="Describe what this preset is for..."
						rows="3"
					></textarea>
				</div>

				<div class="form-field">
					<label for="preset-category">Category</label>
					<select id="preset-category" bind:value={savePresetCategory}>
						{#each Object.entries(CATEGORY_CONFIG) as [key, config]}
							<option value={key}>{config.name}</option>
						{/each}
					</select>
				</div>
			</div>

			<div class="save-modal-footer">
				<button type="button" class="cancel-btn" onclick={() => (showSaveModal = false)}>
					Cancel
				</button>
				<button
					type="button"
					class="save-btn"
					disabled={!savePresetName.trim()}
					onclick={saveBlockAsPreset}
				>
					<IconDeviceFloppy size={18} />
					Save Preset
				</button>
			</div>
		</div>
	</div>
{/if}

<style>
	/* Overlay */
	.preset-overlay,
	.save-modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.4);
		display: flex;
		align-items: flex-start;
		justify-content: center;
		padding-top: 8vh;
		z-index: 1000;
	}

	/* Modal */
	.preset-modal {
		width: 100%;
		max-width: 720px;
		max-height: 80vh;
		background: white;
		border-radius: 16px;
		box-shadow:
			0 25px 50px -12px rgba(0, 0, 0, 0.25),
			0 0 0 1px rgba(0, 0, 0, 0.05);
		display: flex;
		flex-direction: column;
		overflow: hidden;
	}

	.preset-modal.positioned {
		position: absolute;
		max-width: 480px;
	}

	.preset-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 1rem 1.25rem;
		border-bottom: 1px solid #e5e7eb;
		background: linear-gradient(to bottom, #f9fafb, #ffffff);
	}

	.header-title {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.9375rem;
		color: #374151;
	}

	.header-title strong {
		color: #1f2937;
		text-transform: capitalize;
	}

	.close-btn {
		display: flex;
		padding: 0.5rem;
		background: transparent;
		border: none;
		border-radius: 8px;
		cursor: pointer;
		color: #6b7280;
		transition: all 0.15s;
	}

	.close-btn:hover {
		background: #f3f4f6;
		color: #1f2937;
	}

	.preset-toolbar {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem 1.25rem;
		border-bottom: 1px solid #f3f4f6;
	}

	.search-box {
		flex: 1;
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 0.75rem;
		background: #f5f5f5;
		border-radius: 8px;
		color: #6b7280;
	}

	.search-box input {
		flex: 1;
		border: none;
		background: transparent;
		font-size: 0.875rem;
		outline: none;
		color: #1f2937;
	}

	.search-box input::placeholder {
		color: #9ca3af;
	}

	.clear-search {
		display: flex;
		padding: 0.25rem;
		background: transparent;
		border: none;
		border-radius: 4px;
		cursor: pointer;
		color: #9ca3af;
	}

	.clear-search:hover {
		background: #e5e5e5;
		color: #6b7280;
	}

	.save-preset-btn {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.5rem 0.875rem;
		background: linear-gradient(to bottom, #3b82f6, #2563eb);
		color: white;
		border: none;
		border-radius: 8px;
		font-size: 0.8125rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.15s;
		white-space: nowrap;
	}

	.save-preset-btn:hover {
		background: linear-gradient(to bottom, #2563eb, #1d4ed8);
		box-shadow: 0 2px 8px rgba(37, 99, 235, 0.3);
	}

	/* Category Tabs */
	.category-tabs {
		display: flex;
		gap: 0.25rem;
		padding: 0.5rem 1.25rem;
		border-bottom: 1px solid #f3f4f6;
		overflow-x: auto;
	}

	.category-tab {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.375rem 0.75rem;
		background: transparent;
		border: none;
		border-radius: 6px;
		font-size: 0.75rem;
		font-weight: 500;
		color: #6b7280;
		cursor: pointer;
		transition: all 0.15s;
		white-space: nowrap;
	}

	.category-tab:hover {
		background: #f3f4f6;
		color: #374151;
	}

	.category-tab.active {
		background: var(--tab-color, #3b82f6);
		color: white;
	}

	/* Content */
	.preset-content {
		flex: 1;
		overflow-y: auto;
		padding: 1rem 1.25rem;
	}

	/* States */
	.loading-state,
	.error-state,
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 3rem 1rem;
		color: #6b7280;
		text-align: center;
	}

	.loading-state.small {
		padding: 1rem;
	}

	.spinner {
		width: 32px;
		height: 32px;
		border: 3px solid #e5e7eb;
		border-top-color: #3b82f6;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	.spinner.small {
		width: 20px;
		height: 20px;
		border-width: 2px;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.error-state button {
		margin-top: 0.75rem;
		padding: 0.5rem 1rem;
		background: #3b82f6;
		color: white;
		border: none;
		border-radius: 6px;
		cursor: pointer;
	}

	.empty-state p {
		margin: 0.75rem 0 0.25rem;
		font-weight: 500;
		color: #374151;
	}

	.empty-state span {
		font-size: 0.8125rem;
	}

	/* Blank Option */
	.blank-option {
		margin-bottom: 1.5rem;
	}

	.blank-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		width: 100%;
		padding: 1rem;
		background: #f8fafc;
		border: 2px dashed #d1d5db;
		border-radius: 12px;
		color: #6b7280;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.15s;
	}

	.blank-btn:hover {
		background: #f1f5f9;
		border-color: #9ca3af;
		color: #374151;
	}

	/* Category Section */
	.category-section {
		margin-bottom: 1.5rem;
	}

	.category-header {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		margin-bottom: 0.75rem;
		font-size: 0.75rem;
		font-weight: 600;
		color: #6b7280;
		text-transform: uppercase;
		letter-spacing: 0.025em;
	}

	.category-dot {
		width: 8px;
		height: 8px;
		border-radius: 50%;
		background: var(--cat-color, #6b7280);
	}

	.category-name {
		flex: 1;
	}

	.category-count {
		padding: 0.125rem 0.5rem;
		background: #f3f4f6;
		border-radius: 10px;
		font-size: 0.6875rem;
		color: #9ca3af;
	}

	/* Presets Grid */
	.presets-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(140px, 1fr));
		gap: 0.75rem;
	}

	.preset-card {
		position: relative;
		display: flex;
		flex-direction: column;
		background: white;
		border: 1px solid #e5e7eb;
		border-radius: 10px;
		overflow: hidden;
		cursor: pointer;
		transition: all 0.15s;
		text-align: left;
	}

	.preset-card:hover {
		border-color: #3b82f6;
		box-shadow: 0 4px 12px rgba(59, 130, 246, 0.15);
		transform: translateY(-2px);
	}

	.preset-card.is-default {
		border-color: #fbbf24;
	}

	.preset-thumbnail {
		height: 80px;
		background: #f8fafc;
		display: flex;
		align-items: center;
		justify-content: center;
		overflow: hidden;
	}

	.preset-thumbnail img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.preset-thumbnail.placeholder {
		color: #d1d5db;
	}

	.preset-info {
		padding: 0.625rem;
	}

	.preset-name {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		font-size: 0.8125rem;
		font-weight: 500;
		color: #1f2937;
		line-height: 1.3;
	}

	.preset-name :global(.default-star) {
		color: #f59e0b;
		flex-shrink: 0;
	}

	.preset-desc {
		display: block;
		margin-top: 0.25rem;
		font-size: 0.6875rem;
		color: #9ca3af;
		line-height: 1.3;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.preset-usage {
		position: absolute;
		top: 0.5rem;
		right: 0.5rem;
		display: flex;
		align-items: center;
		gap: 0.25rem;
		padding: 0.125rem 0.375rem;
		background: rgba(0, 0, 0, 0.6);
		border-radius: 4px;
		font-size: 0.625rem;
		color: white;
	}

	/* Preview Panel */
	.preset-preview {
		padding: 1rem 1.25rem;
		background: #f8fafc;
		border-top: 1px solid #e5e7eb;
	}

	.preset-preview h4 {
		margin: 0 0 0.25rem;
		font-size: 0.9375rem;
		font-weight: 600;
	}

	.preset-preview p {
		margin: 0 0 0.75rem;
		font-size: 0.8125rem;
		color: #6b7280;
	}

	.preview-tags {
		display: flex;
		gap: 0.375rem;
		flex-wrap: wrap;
		margin-bottom: 0.5rem;
	}

	.tag {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		padding: 0.125rem 0.5rem;
		background: #e5e7eb;
		border-radius: 4px;
		font-size: 0.6875rem;
		color: #6b7280;
	}

	.preview-meta {
		font-size: 0.6875rem;
		color: #9ca3af;
	}

	/* Inline Mode */
	.preset-inline {
		padding: 0.75rem;
	}

	.inline-header {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		margin-bottom: 0.75rem;
		font-size: 0.75rem;
		font-weight: 600;
		color: #6b7280;
		text-transform: uppercase;
	}

	.inline-presets {
		display: flex;
		flex-wrap: wrap;
		gap: 0.375rem;
	}

	.inline-preset-btn {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		padding: 0.375rem 0.625rem;
		background: #f5f5f5;
		border: none;
		border-radius: 6px;
		font-size: 0.75rem;
		color: #374151;
		cursor: pointer;
		transition: all 0.15s;
	}

	.inline-preset-btn:hover {
		background: #e5e5e5;
	}

	.inline-preset-btn :global(.default-indicator) {
		color: #f59e0b;
	}

	.no-presets {
		font-size: 0.75rem;
		color: #9ca3af;
	}

	/* Save Modal */
	.save-modal {
		width: 100%;
		max-width: 400px;
		background: white;
		border-radius: 16px;
		box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25);
		overflow: hidden;
	}

	.save-modal-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 1rem 1.25rem;
		border-bottom: 1px solid #e5e7eb;
	}

	.save-modal-header h3 {
		margin: 0;
		font-size: 1rem;
		font-weight: 600;
	}

	.save-modal-content {
		padding: 1.25rem;
	}

	.form-field {
		margin-bottom: 1rem;
	}

	.form-field:last-child {
		margin-bottom: 0;
	}

	.form-field label {
		display: block;
		margin-bottom: 0.375rem;
		font-size: 0.8125rem;
		font-weight: 500;
		color: #374151;
	}

	.form-field input,
	.form-field textarea,
	.form-field select {
		width: 100%;
		padding: 0.625rem 0.75rem;
		border: 1px solid #d1d5db;
		border-radius: 8px;
		font-size: 0.875rem;
		transition: border-color 0.15s;
	}

	.form-field input:focus,
	.form-field textarea:focus,
	.form-field select:focus {
		outline: none;
		border-color: #3b82f6;
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
	}

	.form-field textarea {
		resize: vertical;
	}

	.save-modal-footer {
		display: flex;
		justify-content: flex-end;
		gap: 0.75rem;
		padding: 1rem 1.25rem;
		background: #f9fafb;
		border-top: 1px solid #e5e7eb;
	}

	.cancel-btn {
		padding: 0.5rem 1rem;
		background: white;
		border: 1px solid #d1d5db;
		border-radius: 8px;
		font-size: 0.875rem;
		color: #374151;
		cursor: pointer;
		transition: all 0.15s;
	}

	.cancel-btn:hover {
		background: #f3f4f6;
	}

	.save-btn {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.5rem 1rem;
		background: #3b82f6;
		border: none;
		border-radius: 8px;
		font-size: 0.875rem;
		font-weight: 500;
		color: white;
		cursor: pointer;
		transition: all 0.15s;
	}

	.save-btn:hover:not(:disabled) {
		background: #2563eb;
	}

	.save-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}
</style>

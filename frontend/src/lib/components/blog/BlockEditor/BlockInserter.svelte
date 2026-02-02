<!--
/**
 * Block Inserter - Advanced Block Selection UI
 * ═══════════════════════════════════════════════════════════════════════════
 * Searchable, categorized block selection with previews
 */
-->

<script lang="ts">
	import { fade, fly, scale } from 'svelte/transition';
	import IconAlignLeft from '@tabler/icons-svelte/icons/align-left';
	import IconH1 from '@tabler/icons-svelte/icons/h-1';
	import IconQuote from '@tabler/icons-svelte/icons/quote';
	import IconBlockquote from '@tabler/icons-svelte/icons/blockquote';
	import IconCode from '@tabler/icons-svelte/icons/code';
	import IconFileCode from '@tabler/icons-svelte/icons/file-code';
	import IconList from '@tabler/icons-svelte/icons/list';
	import IconChecklist from '@tabler/icons-svelte/icons/checklist';
	import IconPhoto from '@tabler/icons-svelte/icons/photo';
	import IconPhotoFilled from '@tabler/icons-svelte/icons/photo-filled';
	import IconVideo from '@tabler/icons-svelte/icons/video';
	import IconVolume from '@tabler/icons-svelte/icons/volume';
	import IconFileDownload from '@tabler/icons-svelte/icons/file-download';
	import IconBrandYoutube from '@tabler/icons-svelte/icons/brand-youtube';
	import IconGif from '@tabler/icons-svelte/icons/gif';
	import IconColumns from '@tabler/icons-svelte/icons/columns';
	import IconBox from '@tabler/icons-svelte/icons/box';
	import IconMinus from '@tabler/icons-svelte/icons/minus';
	import IconSpacingVertical from '@tabler/icons-svelte/icons/spacing-vertical';
	import IconLayoutRows from '@tabler/icons-svelte/icons/layout-rows';
	import IconClick from '@tabler/icons-svelte/icons/click';
	import IconApps from '@tabler/icons-svelte/icons/apps';
	import IconLayoutNavbarCollapse from '@tabler/icons-svelte/icons/layout-navbar-collapse';
	import IconLayoutDistributeHorizontal from '@tabler/icons-svelte/icons/layout-distribute-horizontal';
	import IconToggleLeft from '@tabler/icons-svelte/icons/toggle-left';
	import IconListTree from '@tabler/icons-svelte/icons/list-tree';
	import IconTrendingUp from '@tabler/icons-svelte/icons/trending-up';
	import IconChartCandle from '@tabler/icons-svelte/icons/chart-candle';
	import IconAlertCircle from '@tabler/icons-svelte/icons/alert-circle';
	import IconBulb from '@tabler/icons-svelte/icons/bulb';
	import IconAlertTriangle from '@tabler/icons-svelte/icons/alert-triangle';
	import IconInfoCircle from '@tabler/icons-svelte/icons/info-circle';
	import IconId from '@tabler/icons-svelte/icons/id';
	import IconMessageCircle from '@tabler/icons-svelte/icons/message-circle';
	import IconSpeakerphone from '@tabler/icons-svelte/icons/speakerphone';
	import IconClock from '@tabler/icons-svelte/icons/clock';
	import IconShare from '@tabler/icons-svelte/icons/share';
	import IconUser from '@tabler/icons-svelte/icons/user';
	import IconArticle from '@tabler/icons-svelte/icons/article';
	import IconMail from '@tabler/icons-svelte/icons/mail';
	import IconRobot from '@tabler/icons-svelte/icons/robot';
	import IconFileDescription from '@tabler/icons-svelte/icons/file-description';
	import IconLanguage from '@tabler/icons-svelte/icons/language';
	import IconBrackets from '@tabler/icons-svelte/icons/brackets';
	import IconTemplate from '@tabler/icons-svelte/icons/template';
	import IconX from '@tabler/icons-svelte/icons/x';
	import IconSearch from '@tabler/icons-svelte/icons/search';
	import IconChevronLeft from '@tabler/icons-svelte/icons/chevron-left';
	import IconStarFilled from '@tabler/icons-svelte/icons/star-filled';

	import type { BlockType, BlockContent, BlockSettings } from './types';
	import { BLOCK_CATEGORIES, BLOCK_DEFINITIONS } from './types';
	import PresetPicker from './PresetPicker.svelte';

	// ==========================================================================
	// Props
	// ==========================================================================

	interface Props {
		searchQuery?: string;
		isModal?: boolean;
		position?: { x: number; y: number } | null;
		/** Show preset picker after block type selection */
		showPresets?: boolean;
		oninsert: (type: BlockType, presetData?: { content: Partial<BlockContent>; settings: Partial<BlockSettings> }) => void;
		onclose?: () => void;
	}

	let { searchQuery = '', isModal = false, position = null, showPresets = true, oninsert, onclose }: Props = $props();

	// ==========================================================================
	// State
	// ==========================================================================

	let localSearch = $derived(searchQuery);
	let activeCategory = $state<string | null>(null);
	let hoveredBlock = $state<BlockType | null>(null);

	// Preset picker state
	let selectedBlockType = $state<BlockType | null>(null);
	let showPresetPicker = $state(false);

	// Block types that have presets (loaded from API)
	let blockTypesWithPresets = $state<Set<string>>(new Set([
		'button', 'heading', 'callout', 'quote', 'card', 'separator', 'riskDisclaimer'
	]));

	// ==========================================================================
	// Icon Mapping
	// ==========================================================================

	const BLOCK_ICONS: Record<string, any> = {
		paragraph: IconAlignLeft,
		heading: IconH1,
		quote: IconQuote,
		pullquote: IconBlockquote,
		code: IconCode,
		preformatted: IconFileCode,
		list: IconList,
		checklist: IconChecklist,
		image: IconPhoto,
		gallery: IconPhotoFilled,
		video: IconVideo,
		audio: IconVolume,
		file: IconFileDownload,
		embed: IconBrandYoutube,
		gif: IconGif,
		columns: IconColumns,
		group: IconBox,
		separator: IconMinus,
		spacer: IconSpacingVertical,
		row: IconLayoutRows,
		button: IconClick,
		buttons: IconApps,
		accordion: IconLayoutNavbarCollapse,
		tabs: IconLayoutDistributeHorizontal,
		toggle: IconToggleLeft,
		toc: IconListTree,
		ticker: IconTrendingUp,
		chart: IconChartCandle,
		priceAlert: IconAlertCircle,
		tradingIdea: IconBulb,
		riskDisclaimer: IconAlertTriangle,
		callout: IconInfoCircle,
		card: IconId,
		testimonial: IconMessageCircle,
		cta: IconSpeakerphone,
		countdown: IconClock,
		socialShare: IconShare,
		author: IconUser,
		relatedPosts: IconArticle,
		newsletter: IconMail,
		aiGenerated: IconRobot,
		aiSummary: IconFileDescription,
		aiTranslation: IconLanguage,
		shortcode: IconBrackets,
		html: IconCode,
		reusable: IconTemplate
	};

	// ==========================================================================
	// Computed
	// ==========================================================================

	let filteredBlocks = $derived(() => {
		const query = localSearch.toLowerCase().trim();
		if (!query) return null;

		return Object.entries(BLOCK_DEFINITIONS)
			.filter(([, def]) => {
				return (
					def.name.toLowerCase().includes(query) ||
					def.description.toLowerCase().includes(query) ||
					def.keywords.some((k) => k.toLowerCase().includes(query))
				);
			})
			.map(([type]) => type as BlockType);
	});

	let displayMode = $derived(localSearch ? 'search' : 'categories');

	// ==========================================================================
	// Handlers
	// ==========================================================================

	function handleBlockClick(type: BlockType) {
		// Check if this block type has presets and we should show the preset picker
		if (showPresets && blockTypesWithPresets.has(type)) {
			selectedBlockType = type;
			showPresetPicker = true;
		} else {
			// Insert directly without preset
			oninsert(type);
			onclose?.();
		}
	}

	function handlePresetApply(event: CustomEvent<{ content: Partial<BlockContent>; settings: Partial<BlockSettings> }>) {
		if (selectedBlockType) {
			oninsert(selectedBlockType, event.detail);
			showPresetPicker = false;
			selectedBlockType = null;
			onclose?.();
		}
	}

	function handlePresetClose() {
		showPresetPicker = false;
		selectedBlockType = null;
	}

	function handleBackToBlocks() {
		showPresetPicker = false;
		selectedBlockType = null;
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') {
			if (showPresetPicker) {
				handlePresetClose();
			} else {
				onclose?.();
			}
		}
	}

	function getCategoryColor(categoryId: string): string {
		return BLOCK_CATEGORIES.find((c) => c.id === categoryId)?.color || '#6b7280';
	}

	function hasPresets(type: string): boolean {
		return blockTypesWithPresets.has(type);
	}
</script>

{#if isModal}
	<!-- Preset Picker Modal (shown after block type selection) -->
	{#if showPresetPicker && selectedBlockType}
		<PresetPicker
			blockType={selectedBlockType}
			isModal={true}
			position={null}
			showSaveOption={false}
			on:apply={handlePresetApply}
			on:close={handlePresetClose}
		/>
	{:else}
		<!-- Block Inserter Modal -->
		<div
			class="inserter-overlay"
			onclick={onclose}
			onkeydown={handleKeydown}
			role="button"
			tabindex="0"
			transition:fade={{ duration: 150 }}
		>
			<div
				class="inserter-modal"
				class:positioned={position}
				style:left={position ? `${position.x}px` : 'auto'}
				style:top={position ? `${position.y}px` : 'auto'}
				onclick={(e: MouseEvent) => e.stopPropagation()}
				onkeydown={(e: KeyboardEvent) => e.stopPropagation()}
				role="dialog"
				aria-modal="true"
				aria-label="Insert block"
				tabindex="-1"
				transition:scale={{ duration: 200, start: 0.95 }}
			>
				<div class="inserter-header">
					<div class="search-box">
						<IconSearch size={18} />
						<input
							type="text"
							placeholder="Search blocks..."
							bind:value={localSearch}
							aria-label="Search blocks"
						/>
						{#if localSearch}
							<button type="button" class="clear-search" onclick={() => (localSearch = '')}>
								<IconX size={16} />
							</button>
						{/if}
					</div>
					<button type="button" class="close-btn" onclick={onclose}>
						<IconX size={20} />
					</button>
				</div>

				<div class="inserter-content">
				{#if displayMode === 'search' && filteredBlocks()}
					<!-- Search Results -->
					<div class="search-results">
						{#if (filteredBlocks()?.length ?? 0) === 0}
							<div class="no-results">
								<p>No blocks found for "{localSearch}"</p>
							</div>
						{:else}
							<div class="blocks-grid">
								{#each filteredBlocks() as type}
									{@const def = BLOCK_DEFINITIONS[type]}
									{@const Icon = BLOCK_ICONS[type] || IconBox}
									<button
										type="button"
										class="block-item"
										class:has-presets={hasPresets(type)}
										onclick={() => handleBlockClick(type)}
										onmouseenter={() => (hoveredBlock = type)}
										onmouseleave={() => (hoveredBlock = null)}
									>
										<div
											class="block-icon"
											style:background={getCategoryColor(def.category) + '15'}
											style:color={getCategoryColor(def.category)}
										>
											<Icon size={24} />
										</div>
										<span class="block-name">{def.name}</span>
										{#if hasPresets(type)}
											<span class="presets-badge" title="Has presets available">
												<IconTemplate size={12} />
											</span>
										{/if}
									</button>
								{/each}
							</div>
						{/if}
					</div>
				{:else}
					<!-- Category View -->
					<div class="categories-view">
						{#each BLOCK_CATEGORIES as category}
							<div class="category-section">
								<button
									type="button"
									class="category-header"
									class:expanded={activeCategory === category.id}
									onclick={() =>
										(activeCategory = activeCategory === category.id ? null : category.id)}
								>
									<div class="category-label" style:--cat-color={category.color}>
										<span class="category-dot"></span>
										{category.name}
									</div>
									<span class="category-count">{category.blocks.length}</span>
								</button>

								{#if activeCategory === category.id || !activeCategory}
									<div class="category-blocks" transition:fly={{ y: -10, duration: 200 }}>
										{#each category.blocks as type}
											{@const def = BLOCK_DEFINITIONS[type]}
											{@const Icon = BLOCK_ICONS[type] || IconBox}
											<button
												type="button"
												class="block-item"
												class:has-presets={hasPresets(type)}
												onclick={() => handleBlockClick(type)}
												onmouseenter={() => (hoveredBlock = type)}
												onmouseleave={() => (hoveredBlock = null)}
											>
												<div
													class="block-icon"
													style:background={category.color + '15'}
													style:color={category.color}
												>
													<Icon size={20} />
												</div>
												<div class="block-info">
													<span class="block-name">{def.name}</span>
													<span class="block-desc">{def.description}</span>
												</div>
												{#if hasPresets(type)}
													<span class="presets-badge inline" title="Has presets available">
														<IconTemplate size={14} />
														Presets
													</span>
												{/if}
											</button>
										{/each}
									</div>
								{/if}
							</div>
						{/each}
					</div>
				{/if}
			</div>

			<!-- Block Preview -->
			{#if hoveredBlock}
				{@const def = BLOCK_DEFINITIONS[hoveredBlock]}
				<div class="block-preview" transition:fade={{ duration: 100 }}>
					<h4>{def.name}</h4>
					<p>{def.description}</p>
					<div class="preview-keywords">
						{#each def.keywords.slice(0, 4) as keyword}
							<span class="keyword">{keyword}</span>
						{/each}
					</div>
				</div>
			{/if}
		</div>
	</div>
	{/if}
{:else}
	<!-- Inline Inserter (Sidebar) -->
	<div class="inserter-inline">
		{#each BLOCK_CATEGORIES as category}
			<div class="category-section">
				<div class="category-label" style:--cat-color={category.color}>
					<span class="category-dot"></span>
					{category.name}
				</div>
				<div class="blocks-grid compact">
					{#each category.blocks as type}
						{@const def = BLOCK_DEFINITIONS[type]}
						{@const Icon = BLOCK_ICONS[type] || IconBox}
						<button
							type="button"
							class="block-btn"
							class:has-presets={hasPresets(type)}
							onclick={() => handleBlockClick(type)}
							title={hasPresets(type) ? `${def.description} - Has presets available` : def.description}
						>
							<Icon size={18} />
							<span>{def.name}</span>
							{#if hasPresets(type)}
								<IconTemplate size={14} class="preset-icon" />
							{/if}
						</button>
					{/each}
				</div>
			</div>
		{/each}
	</div>
{/if}

<style>
	/* Overlay */
	.inserter-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.3);
		display: flex;
		align-items: flex-start;
		justify-content: center;
		padding-top: 10vh;
		z-index: 1000;
	}

	/* Modal */
	.inserter-modal {
		width: 100%;
		max-width: 600px;
		max-height: 70vh;
		background: white;
		border-radius: 12px;
		box-shadow: 0 20px 40px rgba(0, 0, 0, 0.2);
		display: flex;
		flex-direction: column;
		overflow: hidden;
	}

	.inserter-modal.positioned {
		position: absolute;
		max-width: 400px;
	}

	.inserter-header {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem;
		border-bottom: 1px solid #e5e5e5;
	}

	.search-box {
		flex: 1;
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1rem;
		background: #f5f5f5;
		border-radius: 8px;
		color: #666;
	}

	.search-box input {
		flex: 1;
		border: none;
		background: transparent;
		font-size: 0.9375rem;
		outline: none;
		color: #1a1a1a;
	}

	.search-box input::placeholder {
		color: #999;
	}

	.clear-search {
		display: flex;
		padding: 0.25rem;
		background: transparent;
		border: none;
		border-radius: 4px;
		cursor: pointer;
		color: #999;
	}

	.clear-search:hover {
		background: #e5e5e5;
		color: #666;
	}

	.close-btn {
		display: flex;
		padding: 0.5rem;
		background: transparent;
		border: none;
		border-radius: 6px;
		cursor: pointer;
		color: #666;
	}

	.close-btn:hover {
		background: #f0f0f0;
	}

	.inserter-content {
		flex: 1;
		overflow-y: auto;
		padding: 1rem;
	}

	/* Categories */
	.categories-view {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.category-section {
		border-radius: 8px;
		overflow: hidden;
	}

	.category-header {
		width: 100%;
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0.75rem 1rem;
		background: #f8f9fa;
		border: none;
		cursor: pointer;
		transition: background 0.15s;
	}

	.category-header:hover {
		background: #f0f0f0;
	}

	.category-label {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-weight: 600;
		font-size: 0.8125rem;
		color: #1a1a1a;
		text-transform: uppercase;
		letter-spacing: 0.025em;
	}

	.category-dot {
		width: 8px;
		height: 8px;
		border-radius: 50%;
		background: var(--cat-color, #6b7280);
	}

	.category-count {
		font-size: 0.75rem;
		color: #999;
		background: #e5e5e5;
		padding: 0.125rem 0.5rem;
		border-radius: 10px;
	}

	.category-blocks {
		display: flex;
		flex-direction: column;
		padding: 0.5rem;
	}

	/* Block Items */
	.block-item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem;
		background: transparent;
		border: none;
		border-radius: 8px;
		cursor: pointer;
		text-align: left;
		transition: background 0.15s;
	}

	.block-item:hover {
		background: #f5f5f5;
	}

	.block-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 40px;
		height: 40px;
		border-radius: 8px;
		flex-shrink: 0;
	}

	.block-info {
		display: flex;
		flex-direction: column;
		gap: 0.125rem;
		overflow: hidden;
	}

	.block-name {
		font-weight: 500;
		font-size: 0.9375rem;
		color: #1a1a1a;
	}

	.block-desc {
		font-size: 0.8125rem;
		color: #666;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	/* Search Results */
	.search-results {
		min-height: 200px;
	}

	.no-results {
		display: flex;
		align-items: center;
		justify-content: center;
		height: 200px;
		color: #999;
	}

	.blocks-grid {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 0.75rem;
	}

	.blocks-grid .block-item {
		flex-direction: column;
		padding: 1rem 0.75rem;
		text-align: center;
	}

	.blocks-grid .block-name {
		font-size: 0.8125rem;
	}

	/* Block Preview */
	.block-preview {
		padding: 1rem;
		background: #f8f9fa;
		border-top: 1px solid #e5e5e5;
	}

	.block-preview h4 {
		font-size: 0.9375rem;
		font-weight: 600;
		margin: 0 0 0.25rem;
	}

	.block-preview p {
		font-size: 0.8125rem;
		color: #666;
		margin: 0 0 0.75rem;
	}

	.preview-keywords {
		display: flex;
		gap: 0.375rem;
		flex-wrap: wrap;
	}

	.keyword {
		font-size: 0.6875rem;
		padding: 0.125rem 0.5rem;
		background: #e5e5e5;
		border-radius: 4px;
		color: #666;
	}

	/* Inline Inserter */
	.inserter-inline .category-section {
		margin-bottom: 1.5rem;
	}

	.inserter-inline .category-label {
		margin-bottom: 0.75rem;
		font-size: 0.6875rem;
	}

	.blocks-grid.compact {
		grid-template-columns: 1fr;
		gap: 0.25rem;
	}

	.block-btn {
		display: flex;
		align-items: center;
		gap: 0.625rem;
		padding: 0.5rem 0.75rem;
		background: #f5f5f5;
		border: none;
		border-radius: 6px;
		cursor: pointer;
		font-size: 0.8125rem;
		color: #1a1a1a;
		text-align: left;
		transition: background 0.15s;
	}

	.block-btn:hover {
		background: #e5e5e5;
	}

	.block-btn span {
		flex: 1;
	}

	/* Preset Badges */
	.presets-badge {
		position: absolute;
		top: 0.375rem;
		right: 0.375rem;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 0.125rem;
		background: linear-gradient(135deg, #8b5cf6, #6366f1);
		border-radius: 4px;
		color: white;
	}

	.presets-badge.inline {
		position: static;
		display: flex;
		align-items: center;
		gap: 0.25rem;
		padding: 0.25rem 0.5rem;
		font-size: 0.6875rem;
		font-weight: 500;
		border-radius: 6px;
		margin-left: auto;
		flex-shrink: 0;
	}

	.block-item {
		position: relative;
	}

	.block-item.has-presets {
		border: 1px solid transparent;
	}

	.block-item.has-presets:hover {
		border-color: rgba(139, 92, 246, 0.3);
		background: rgba(139, 92, 246, 0.05);
	}

	.blocks-grid .block-item.has-presets .presets-badge {
		top: 0.25rem;
		right: 0.25rem;
	}

	.block-btn.has-presets {
		background: linear-gradient(135deg, #f5f5f5, rgba(139, 92, 246, 0.08));
		border: 1px solid rgba(139, 92, 246, 0.2);
	}

	.block-btn.has-presets:hover {
		background: linear-gradient(135deg, #e5e5e5, rgba(139, 92, 246, 0.15));
		border-color: rgba(139, 92, 246, 0.3);
	}

	.block-btn :global(.preset-icon) {
		color: #8b5cf6;
		flex-shrink: 0;
	}
</style>

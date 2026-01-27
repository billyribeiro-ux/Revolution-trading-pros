<!--
/**
 * Reusable Block Picker Modal - CMS Editor Component
 * ═══════════════════════════════════════════════════════════════════════════
 * Modal interface for browsing, searching, and inserting reusable content blocks.
 * Supports synced blocks (linked to source) and detached copies.
 *
 * FEATURES:
 * - Grid view with block thumbnails/previews
 * - Filter by category (general, trading, layout, callout, marketing)
 * - Search by name/description with debounce
 * - Usage count display for each block
 * - Preview block on hover
 * - Insert as synced or detached (copy)
 * - Full keyboard navigation and ARIA support
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 * @since January 2026
 */
-->

<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { fade, fly, scale } from 'svelte/transition';
	import { quintOut, backOut } from 'svelte/easing';
	import { browser } from '$app/environment';

	// Icons
	import IconSearch from '@tabler/icons-svelte/icons/search';
	import IconX from '@tabler/icons-svelte/icons/x';
	import IconLink from '@tabler/icons-svelte/icons/link';
	import IconCopy from '@tabler/icons-svelte/icons/copy';
	import IconChevronDown from '@tabler/icons-svelte/icons/chevron-down';
	import IconLayoutGrid from '@tabler/icons-svelte/icons/layout-grid';
	import IconChartCandle from '@tabler/icons-svelte/icons/chart-candle';
	import IconColumns from '@tabler/icons-svelte/icons/columns';
	import IconSpeakerphone from '@tabler/icons-svelte/icons/speakerphone';
	import IconBulb from '@tabler/icons-svelte/icons/bulb';
	import IconTemplate from '@tabler/icons-svelte/icons/template';
	import IconLoader2 from '@tabler/icons-svelte/icons/loader-2';
	import IconAlertCircle from '@tabler/icons-svelte/icons/alert-circle';
	import IconBox from '@tabler/icons-svelte/icons/box';
	import IconEye from '@tabler/icons-svelte/icons/eye';
	import IconUsersGroup from '@tabler/icons-svelte/icons/users-group';

	import type { PageBlock } from '$lib/page-builder/types';

	// ==========================================================================
	// Types
	// ==========================================================================

	export type BlockCategory = 'general' | 'trading' | 'layout' | 'callout' | 'marketing';

	export interface ReusableBlock {
		id: string;
		name: string;
		description: string | null;
		category: BlockCategory;
		content_blocks: PageBlock[];
		thumbnail_url: string | null;
		preview_html: string | null;
		usage_count: number;
		is_locked: boolean;
		created_at: string;
		updated_at: string;
		created_by: string | null;
	}

	interface CategoryMeta {
		id: BlockCategory | 'all';
		label: string;
		// eslint-disable-next-line @typescript-eslint/no-explicit-any
		icon: any;
		color: string;
		bgColor: string;
	}

	// ==========================================================================
	// Props
	// ==========================================================================

	interface Props {
		/** Whether the modal is open */
		isOpen: boolean;
		/** Callback when a block is selected */
		onSelect: (block: ReusableBlock, synced: boolean) => void;
		/** Callback when modal is closed */
		onClose: () => void;
		/** Optional category filter - only show blocks from these categories */
		categories?: BlockCategory[];
	}

	let { isOpen, onSelect, onClose, categories }: Props = $props();

	// ==========================================================================
	// Category Definitions
	// ==========================================================================

	const CATEGORIES: CategoryMeta[] = [
		{
			id: 'all',
			label: 'All Blocks',
			icon: IconLayoutGrid,
			color: 'text-slate-600',
			bgColor: 'bg-slate-100'
		},
		{
			id: 'general',
			label: 'General',
			icon: IconTemplate,
			color: 'text-blue-600',
			bgColor: 'bg-blue-100'
		},
		{
			id: 'trading',
			label: 'Trading',
			icon: IconChartCandle,
			color: 'text-emerald-600',
			bgColor: 'bg-emerald-100'
		},
		{
			id: 'layout',
			label: 'Layout',
			icon: IconColumns,
			color: 'text-purple-600',
			bgColor: 'bg-purple-100'
		},
		{
			id: 'callout',
			label: 'Callout',
			icon: IconBulb,
			color: 'text-amber-600',
			bgColor: 'bg-amber-100'
		},
		{
			id: 'marketing',
			label: 'Marketing',
			icon: IconSpeakerphone,
			color: 'text-rose-600',
			bgColor: 'bg-rose-100'
		}
	];

	// Filter categories if prop is provided
	const availableCategories = $derived.by(() => {
		if (!categories || categories.length === 0) {
			return CATEGORIES;
		}
		return CATEGORIES.filter(
			(cat) => cat.id === 'all' || categories.includes(cat.id as BlockCategory)
		);
	});

	// ==========================================================================
	// State
	// ==========================================================================

	let blocks = $state<ReusableBlock[]>([]);
	let isLoading = $state(false);
	let error = $state<string | null>(null);
	let searchQuery = $state('');
	let selectedCategory = $state<BlockCategory | 'all'>('all');
	let hoveredBlockId = $state<string | null>(null);
	let selectedBlockIndex = $state(-1);
	let showInsertMenu = $state<string | null>(null);
	let searchInputRef = $state<HTMLInputElement | null>(null);
	let modalRef = $state<HTMLDivElement | null>(null);
	let debounceTimer: ReturnType<typeof setTimeout> | null = null;
	let previouslyFocused: Element | null = null;

	// Unique IDs for ARIA
	const modalId = `reusable-block-picker-${Math.random().toString(36).substring(2, 9)}`;
	const searchId = `${modalId}-search`;
	const gridId = `${modalId}-grid`;

	// ==========================================================================
	// Computed Values
	// ==========================================================================

	const filteredBlocks = $derived.by(() => {
		let result = blocks;

		// Filter by category
		if (selectedCategory !== 'all') {
			result = result.filter((block) => block.category === selectedCategory);
		}

		// Filter by search query
		if (searchQuery.trim()) {
			const query = searchQuery.toLowerCase().trim();
			result = result.filter(
				(block) =>
					block.name.toLowerCase().includes(query) ||
					(block.description && block.description.toLowerCase().includes(query))
			);
		}

		return result;
	});

	const hasResults = $derived(filteredBlocks.length > 0);
	const isSearching = $derived(searchQuery.trim().length > 0);

	// ==========================================================================
	// API Functions
	// ==========================================================================

	async function fetchBlocks() {
		isLoading = true;
		error = null;

		try {
			const params = new URLSearchParams();
			if (categories && categories.length > 0) {
				categories.forEach((cat) => params.append('category', cat));
			}

			const url = `/api/cms/reusable-blocks${params.toString() ? `?${params.toString()}` : ''}`;
			const response = await fetch(url, {
				credentials: 'include'
			});

			if (!response.ok) {
				const errorData = await response.json().catch(() => ({ error: 'Failed to fetch blocks' }));
				throw new Error(errorData.error || `HTTP ${response.status}`);
			}

			const data = await response.json();
			blocks = data.data || data || [];
		} catch (err) {
			console.error('Failed to fetch reusable blocks:', err);
			error = err instanceof Error ? err.message : 'Failed to load blocks';
		} finally {
			isLoading = false;
		}
	}

	// ==========================================================================
	// Event Handlers
	// ==========================================================================

	function handleSearch(e: Event) {
		const target = e.target as HTMLInputElement;
		const value = target.value;

		// Debounce search
		if (debounceTimer) {
			clearTimeout(debounceTimer);
		}

		debounceTimer = setTimeout(() => {
			searchQuery = value;
			selectedBlockIndex = -1;
		}, 200);
	}

	function handleCategoryChange(category: BlockCategory | 'all') {
		selectedCategory = category;
		selectedBlockIndex = -1;
	}

	function handleBlockSelect(block: ReusableBlock, synced: boolean) {
		onSelect(block, synced);
		closeModal();
	}

	function handleBlockClick(block: ReusableBlock) {
		// Default to synced insert
		handleBlockSelect(block, true);
	}

	function handleInsertMenuToggle(blockId: string, e: MouseEvent) {
		e.stopPropagation();
		showInsertMenu = showInsertMenu === blockId ? null : blockId;
	}

	function handleInsertOption(block: ReusableBlock, synced: boolean, e: MouseEvent) {
		e.stopPropagation();
		showInsertMenu = null;
		handleBlockSelect(block, synced);
	}

	function handleKeydown(e: KeyboardEvent) {
		if (!isOpen) return;

		switch (e.key) {
			case 'Escape':
				e.preventDefault();
				closeModal();
				break;

			case 'ArrowDown':
				e.preventDefault();
				if (filteredBlocks.length > 0) {
					selectedBlockIndex = Math.min(selectedBlockIndex + 1, filteredBlocks.length - 1);
					scrollToSelectedBlock();
				}
				break;

			case 'ArrowUp':
				e.preventDefault();
				if (filteredBlocks.length > 0) {
					selectedBlockIndex = Math.max(selectedBlockIndex - 1, 0);
					scrollToSelectedBlock();
				}
				break;

			case 'Enter':
				e.preventDefault();
				if (selectedBlockIndex >= 0 && selectedBlockIndex < filteredBlocks.length) {
					const block = filteredBlocks[selectedBlockIndex];
					if (e.shiftKey) {
						// Shift+Enter inserts as copy (detached)
						handleBlockSelect(block, false);
					} else {
						// Enter inserts as synced
						handleBlockSelect(block, true);
					}
				}
				break;

			case 'Tab':
				// Allow normal tab behavior for focus trap
				handleTabTrap(e);
				break;
		}
	}

	function handleTabTrap(e: KeyboardEvent) {
		if (!modalRef) return;

		const focusableSelector =
			'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])';
		const focusableElements = modalRef.querySelectorAll(focusableSelector);
		const focusable = Array.from(focusableElements) as HTMLElement[];

		if (focusable.length === 0) return;

		const firstFocusable = focusable[0];
		const lastFocusable = focusable[focusable.length - 1];

		if (e.shiftKey) {
			if (document.activeElement === firstFocusable) {
				e.preventDefault();
				lastFocusable?.focus();
			}
		} else {
			if (document.activeElement === lastFocusable) {
				e.preventDefault();
				firstFocusable?.focus();
			}
		}
	}

	function scrollToSelectedBlock() {
		if (browser && selectedBlockIndex >= 0) {
			const blockEl = document.getElementById(`block-${filteredBlocks[selectedBlockIndex]?.id}`);
			blockEl?.scrollIntoView({ behavior: 'smooth', block: 'nearest' });
		}
	}

	function handleBackdropClick(e: MouseEvent) {
		if (e.target === e.currentTarget) {
			closeModal();
		}
	}

	function closeModal() {
		showInsertMenu = null;
		searchQuery = '';
		selectedBlockIndex = -1;
		onClose();

		// Return focus to trigger element
		if (browser && previouslyFocused && previouslyFocused instanceof HTMLElement) {
			previouslyFocused.focus();
		}
	}

	function handleBlockHover(blockId: string | null) {
		hoveredBlockId = blockId;
	}

	// Close insert menu when clicking outside
	function handleDocumentClick(e: MouseEvent) {
		if (showInsertMenu) {
			const target = e.target as HTMLElement;
			if (!target.closest('.insert-menu-container')) {
				showInsertMenu = null;
			}
		}
	}

	// ==========================================================================
	// Helper Functions
	// ==========================================================================

	function getCategoryMeta(category: BlockCategory): CategoryMeta | undefined {
		return CATEGORIES.find((c) => c.id === category);
	}

	function formatDate(dateString: string): string {
		const date = new Date(dateString);
		return date.toLocaleDateString('en-US', {
			month: 'short',
			day: 'numeric',
			year: 'numeric'
		});
	}

	function formatUsageCount(count: number): string {
		if (count >= 1000) {
			return `${(count / 1000).toFixed(1)}k`;
		}
		return count.toString();
	}

	// ==========================================================================
	// Effects
	// ==========================================================================

	// Fetch blocks when modal opens
	$effect(() => {
		if (isOpen && browser) {
			previouslyFocused = document.activeElement;
			fetchBlocks();

			// Focus search input after render
			requestAnimationFrame(() => {
				searchInputRef?.focus();
			});

			// Prevent body scroll
			document.body.style.overflow = 'hidden';
		} else if (!isOpen && browser) {
			document.body.style.overflow = '';
		}
	});

	// Cleanup on unmount
	onMount(() => {
		if (browser) {
			document.addEventListener('click', handleDocumentClick);
		}

		return () => {
			if (browser) {
				document.body.style.overflow = '';
				document.removeEventListener('click', handleDocumentClick);
			}
			if (debounceTimer) {
				clearTimeout(debounceTimer);
			}
		};
	});
</script>

<svelte:window onkeydown={handleKeydown} />

{#if isOpen}
	<!-- Modal Backdrop -->
	<div
		class="modal-backdrop"
		onclick={handleBackdropClick}
		role="presentation"
		transition:fade={{ duration: 150 }}
	>
		<!-- Modal Container -->
		<div
			bind:this={modalRef}
			class="modal-container"
			role="dialog"
			aria-modal="true"
			aria-labelledby="{modalId}-title"
			aria-describedby="{modalId}-desc"
			tabindex="-1"
			transition:fly={{ y: 20, duration: 200, easing: quintOut }}
		>
			<!-- Header -->
			<div class="modal-header">
				<div class="header-content">
					<div class="header-icon">
						<IconBox size={24} />
					</div>
					<div>
						<h2 id="{modalId}-title" class="modal-title">Reusable Blocks</h2>
						<p id="{modalId}-desc" class="modal-description">
							Insert pre-built content blocks into your page
						</p>
					</div>
				</div>
				<button
					type="button"
					class="close-button"
					onclick={closeModal}
					aria-label="Close modal"
				>
					<IconX size={20} />
				</button>
			</div>

			<!-- Search & Filters -->
			<div class="filters-section">
				<!-- Search Input -->
				<div class="search-container">
					<IconSearch size={18} class="search-icon" />
					<input
						bind:this={searchInputRef}
						id={searchId}
						type="text"
						placeholder="Search blocks by name or description..."
						class="search-input"
						value={searchQuery}
						oninput={handleSearch}
						aria-label="Search reusable blocks"
					/>
					{#if searchQuery}
						<button
							type="button"
							class="clear-search"
							onclick={() => {
								searchQuery = '';
								searchInputRef?.focus();
							}}
							aria-label="Clear search"
							transition:scale={{ duration: 100 }}
						>
							<IconX size={14} />
						</button>
					{/if}
				</div>

				<!-- Category Tabs -->
				<div class="category-tabs" role="tablist" aria-label="Block categories">
					{#each availableCategories as category}
						{@const Icon = category.icon}
						<button
							type="button"
							role="tab"
							class="category-tab"
							class:active={selectedCategory === category.id}
							onclick={() => handleCategoryChange(category.id)}
							aria-selected={selectedCategory === category.id}
							aria-controls={gridId}
						>
							<Icon size={16} />
							<span>{category.label}</span>
							{#if category.id !== 'all'}
								{@const count = blocks.filter((b) => b.category === category.id).length}
								{#if count > 0}
									<span class="category-count">{count}</span>
								{/if}
							{/if}
						</button>
					{/each}
				</div>
			</div>

			<!-- Content Area -->
			<div class="content-area" id={gridId} role="tabpanel">
				{#if isLoading}
					<!-- Loading State -->
					<div class="loading-state" transition:fade={{ duration: 150 }}>
						<div class="loading-spinner">
							<IconLoader2 size={32} class="animate-spin" />
						</div>
						<p>Loading blocks...</p>
					</div>
				{:else if error}
					<!-- Error State -->
					<div class="error-state" transition:fade={{ duration: 150 }}>
						<div class="error-icon">
							<IconAlertCircle size={40} />
						</div>
						<p class="error-message">{error}</p>
						<button type="button" class="retry-button" onclick={fetchBlocks}>
							Try Again
						</button>
					</div>
				{:else if !hasResults}
					<!-- Empty State -->
					<div class="empty-state" transition:fade={{ duration: 150 }}>
						<div class="empty-icon">
							<IconBox size={48} />
						</div>
						{#if isSearching}
							<p class="empty-title">No blocks found</p>
							<p class="empty-description">
								No blocks match "{searchQuery}". Try a different search term.
							</p>
						{:else}
							<p class="empty-title">No blocks available</p>
							<p class="empty-description">
								{#if selectedCategory !== 'all'}
									No blocks in the {selectedCategory} category yet.
								{:else}
									Create your first reusable block in the CMS.
								{/if}
							</p>
						{/if}
					</div>
				{:else}
					<!-- Blocks Grid -->
					<div class="blocks-grid" role="listbox" aria-label="Reusable blocks">
						{#each filteredBlocks as block, index (block.id)}
							{@const categoryMeta = getCategoryMeta(block.category)}
							{@const isSelected = index === selectedBlockIndex}
							{@const isHovered = hoveredBlockId === block.id}

							<div
								id="block-{block.id}"
								class="block-card"
								class:selected={isSelected}
								class:hovered={isHovered}
								role="option"
								tabindex="0"
								aria-selected={isSelected}
								onclick={() => handleBlockClick(block)}
								onkeydown={(e) => { if (e.key === 'Enter' || e.key === ' ') { e.preventDefault(); handleBlockClick(block); }}}
								onmouseenter={() => handleBlockHover(block.id)}
								onmouseleave={() => handleBlockHover(null)}
								transition:scale={{ duration: 200, start: 0.95, easing: backOut }}
							>
								<!-- Block Preview/Thumbnail -->
								<div class="block-preview">
									{#if block.thumbnail_url}
										<img
											src={block.thumbnail_url}
											alt="{block.name} preview"
											class="preview-image"
											loading="lazy"
										/>
									{:else if block.preview_html}
										<div class="preview-html">
											{@html block.preview_html}
										</div>
									{:else}
										<div class="preview-placeholder">
											<IconTemplate size={32} />
										</div>
									{/if}

									<!-- Hover Preview Overlay -->
									{#if isHovered && block.preview_html}
										<div
											class="preview-overlay"
											transition:fade={{ duration: 150 }}
										>
											<div class="preview-overlay-content">
												{@html block.preview_html}
											</div>
										</div>
									{/if}

									<!-- Category Badge -->
									{#if categoryMeta}
										{@const CategoryIcon = categoryMeta.icon}
										<div
											class="category-badge {categoryMeta.bgColor} {categoryMeta.color}"
										>
											<CategoryIcon size={12} />
											<span>{categoryMeta.label}</span>
										</div>
									{/if}
								</div>

								<!-- Block Info -->
								<div class="block-info">
									<h3 class="block-name">{block.name}</h3>
									{#if block.description}
										<p class="block-description">{block.description}</p>
									{/if}

									<div class="block-meta">
										<div class="meta-item" title="Usage count">
											<IconUsersGroup size={14} />
											<span>{formatUsageCount(block.usage_count)} uses</span>
										</div>
									</div>
								</div>

								<!-- Insert Actions -->
								<div class="block-actions insert-menu-container">
									<button
										type="button"
										class="insert-button"
										onclick={(e) => {
											e.stopPropagation();
											handleBlockSelect(block, true);
										}}
										title="Insert synced block"
									>
										<IconLink size={14} />
										<span>Insert</span>
									</button>

									<button
										type="button"
										class="dropdown-toggle"
										onclick={(e) => handleInsertMenuToggle(block.id, e)}
										aria-label="More insert options"
										aria-haspopup="true"
										aria-expanded={showInsertMenu === block.id}
									>
										<IconChevronDown size={14} />
									</button>

									<!-- Insert Dropdown Menu -->
									{#if showInsertMenu === block.id}
										<div
											class="insert-dropdown"
											role="menu"
											transition:scale={{
												duration: 150,
												start: 0.95,
												easing: quintOut
											}}
										>
											<button
												type="button"
												class="dropdown-item"
												role="menuitem"
												onclick={(e) => handleInsertOption(block, true, e)}
											>
												<IconLink size={16} />
												<div class="dropdown-item-content">
													<span class="dropdown-item-label">Insert Synced</span>
													<span class="dropdown-item-desc">
														Changes to source update all instances
													</span>
												</div>
											</button>
											<button
												type="button"
												class="dropdown-item"
												role="menuitem"
												onclick={(e) => handleInsertOption(block, false, e)}
											>
												<IconCopy size={16} />
												<div class="dropdown-item-content">
													<span class="dropdown-item-label">Insert as Copy</span>
													<span class="dropdown-item-desc">
														Independent copy, not synced
													</span>
												</div>
											</button>
										</div>
									{/if}
								</div>
							</div>
						{/each}
					</div>
				{/if}
			</div>

			<!-- Footer with keyboard hints -->
			<div class="modal-footer">
				<div class="keyboard-hints">
					<span class="hint">
						<kbd>Enter</kbd> Insert synced
					</span>
					<span class="hint">
						<kbd>Shift</kbd>+<kbd>Enter</kbd> Insert as copy
					</span>
					<span class="hint">
						<kbd>Esc</kbd> Close
					</span>
				</div>
				<div class="results-count">
					{filteredBlocks.length} block{filteredBlocks.length !== 1 ? 's' : ''}
				</div>
			</div>
		</div>
	</div>
{/if}

<style>
	/* ========================================================================
	   MODAL BACKDROP & CONTAINER
	   ======================================================================== */

	.modal-backdrop {
		position: fixed;
		inset: 0;
		z-index: 1000;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 1rem;
		background: rgba(0, 0, 0, 0.6);
		backdrop-filter: blur(4px);
		-webkit-backdrop-filter: blur(4px);
	}

	.modal-container {
		position: relative;
		width: 100%;
		max-width: 900px;
		max-height: calc(100vh - 2rem);
		display: flex;
		flex-direction: column;
		background: white;
		border-radius: 16px;
		box-shadow:
			0 25px 50px -12px rgba(0, 0, 0, 0.25),
			0 0 0 1px rgba(0, 0, 0, 0.05);
		overflow: hidden;
	}

	/* ========================================================================
	   MODAL HEADER
	   ======================================================================== */

	.modal-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 1.25rem 1.5rem;
		border-bottom: 1px solid #e5e7eb;
		background: linear-gradient(to bottom, #fafafa, white);
	}

	.header-content {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.header-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 48px;
		height: 48px;
		background: linear-gradient(135deg, #6366f1 0%, #8b5cf6 100%);
		border-radius: 12px;
		color: white;
	}

	.modal-title {
		font-size: 1.25rem;
		font-weight: 700;
		color: #1f2937;
		margin: 0;
	}

	.modal-description {
		font-size: 0.875rem;
		color: #6b7280;
		margin: 0.25rem 0 0;
	}

	.close-button {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 36px;
		height: 36px;
		background: transparent;
		border: none;
		border-radius: 8px;
		color: #6b7280;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.close-button:hover {
		background: #f3f4f6;
		color: #1f2937;
	}

	.close-button:focus-visible {
		outline: 2px solid #6366f1;
		outline-offset: 2px;
	}

	/* ========================================================================
	   FILTERS SECTION
	   ======================================================================== */

	.filters-section {
		padding: 1rem 1.5rem;
		border-bottom: 1px solid #e5e7eb;
		background: #fafafa;
	}

	.search-container {
		position: relative;
		margin-bottom: 1rem;
	}

	.search-container :global(.search-icon) {
		position: absolute;
		left: 1rem;
		top: 50%;
		transform: translateY(-50%);
		color: #9ca3af;
		pointer-events: none;
	}

	.search-input {
		width: 100%;
		padding: 0.75rem 2.5rem 0.75rem 2.75rem;
		background: white;
		border: 1px solid #e5e7eb;
		border-radius: 10px;
		font-size: 0.9375rem;
		color: #1f2937;
		outline: none;
		transition: all 0.15s ease;
	}

	.search-input:focus {
		border-color: #6366f1;
		box-shadow: 0 0 0 3px rgba(99, 102, 241, 0.1);
	}

	.search-input::placeholder {
		color: #9ca3af;
	}

	.clear-search {
		position: absolute;
		right: 0.75rem;
		top: 50%;
		transform: translateY(-50%);
		display: flex;
		align-items: center;
		justify-content: center;
		width: 24px;
		height: 24px;
		background: #e5e7eb;
		border: none;
		border-radius: 6px;
		color: #6b7280;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.clear-search:hover {
		background: #d1d5db;
		color: #1f2937;
	}

	/* Category Tabs */
	.category-tabs {
		display: flex;
		gap: 0.5rem;
		overflow-x: auto;
		padding-bottom: 0.25rem;
		-webkit-overflow-scrolling: touch;
		scrollbar-width: none;
	}

	.category-tabs::-webkit-scrollbar {
		display: none;
	}

	.category-tab {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.5rem 0.875rem;
		background: white;
		border: 1px solid #e5e7eb;
		border-radius: 8px;
		font-size: 0.8125rem;
		font-weight: 500;
		color: #6b7280;
		white-space: nowrap;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.category-tab:hover {
		border-color: #d1d5db;
		color: #374151;
	}

	.category-tab.active {
		background: #6366f1;
		border-color: #6366f1;
		color: white;
	}

	.category-count {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		min-width: 20px;
		height: 20px;
		padding: 0 0.375rem;
		background: rgba(0, 0, 0, 0.1);
		border-radius: 10px;
		font-size: 0.75rem;
		font-weight: 600;
	}

	.category-tab.active .category-count {
		background: rgba(255, 255, 255, 0.2);
	}

	/* ========================================================================
	   CONTENT AREA
	   ======================================================================== */

	.content-area {
		flex: 1;
		overflow-y: auto;
		padding: 1.5rem;
		min-height: 300px;
	}

	/* Loading State */
	.loading-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		height: 300px;
		color: #6b7280;
	}

	.loading-spinner {
		margin-bottom: 1rem;
		color: #6366f1;
	}

	.loading-spinner :global(.animate-spin) {
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}

	/* Error State */
	.error-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		height: 300px;
		text-align: center;
	}

	.error-icon {
		color: #ef4444;
		margin-bottom: 1rem;
	}

	.error-message {
		color: #6b7280;
		margin-bottom: 1rem;
	}

	.retry-button {
		padding: 0.5rem 1rem;
		background: #6366f1;
		border: none;
		border-radius: 8px;
		color: white;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: background 0.15s ease;
	}

	.retry-button:hover {
		background: #4f46e5;
	}

	/* Empty State */
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		height: 300px;
		text-align: center;
	}

	.empty-icon {
		color: #d1d5db;
		margin-bottom: 1rem;
	}

	.empty-title {
		font-size: 1rem;
		font-weight: 600;
		color: #374151;
		margin: 0 0 0.5rem;
	}

	.empty-description {
		font-size: 0.875rem;
		color: #6b7280;
		margin: 0;
		max-width: 280px;
	}

	/* ========================================================================
	   BLOCKS GRID
	   ======================================================================== */

	.blocks-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(260px, 1fr));
		gap: 1rem;
	}

	.block-card {
		position: relative;
		display: flex;
		flex-direction: column;
		background: white;
		border: 1px solid #e5e7eb;
		border-radius: 12px;
		overflow: hidden;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.block-card:hover,
	.block-card.hovered {
		border-color: #c7d2fe;
		box-shadow: 0 4px 12px rgba(99, 102, 241, 0.1);
		transform: translateY(-2px);
	}

	.block-card.selected {
		border-color: #6366f1;
		box-shadow:
			0 0 0 3px rgba(99, 102, 241, 0.15),
			0 4px 12px rgba(99, 102, 241, 0.15);
	}

	/* Block Preview */
	.block-preview {
		position: relative;
		height: 140px;
		background: #f9fafb;
		overflow: hidden;
	}

	.preview-image {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.preview-html {
		width: 100%;
		height: 100%;
		padding: 0.75rem;
		overflow: hidden;
		font-size: 0.625rem;
		transform: scale(0.5);
		transform-origin: top left;
		pointer-events: none;
	}

	.preview-placeholder {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 100%;
		height: 100%;
		color: #d1d5db;
	}

	.preview-overlay {
		position: absolute;
		inset: 0;
		background: rgba(0, 0, 0, 0.85);
		padding: 0.5rem;
		overflow: hidden;
	}

	.preview-overlay-content {
		width: 100%;
		height: 100%;
		background: white;
		border-radius: 4px;
		overflow: hidden;
		font-size: 0.5rem;
		transform: scale(0.4);
		transform-origin: top left;
	}

	/* Category Badge */
	.category-badge {
		position: absolute;
		top: 0.5rem;
		left: 0.5rem;
		display: flex;
		align-items: center;
		gap: 0.25rem;
		padding: 0.25rem 0.5rem;
		border-radius: 6px;
		font-size: 0.6875rem;
		font-weight: 600;
	}

	/* Block Info */
	.block-info {
		flex: 1;
		padding: 1rem;
		display: flex;
		flex-direction: column;
		gap: 0.375rem;
	}

	.block-name {
		font-size: 0.9375rem;
		font-weight: 600;
		color: #1f2937;
		margin: 0;
		line-height: 1.3;
	}

	.block-description {
		font-size: 0.8125rem;
		color: #6b7280;
		margin: 0;
		line-height: 1.4;
		display: -webkit-box;
		-webkit-line-clamp: 2;
		line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	.block-meta {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		margin-top: auto;
		padding-top: 0.5rem;
	}

	.meta-item {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		font-size: 0.75rem;
		color: #9ca3af;
	}

	/* Block Actions */
	.block-actions {
		position: relative;
		display: flex;
		align-items: stretch;
		border-top: 1px solid #e5e7eb;
	}

	.insert-button {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.375rem;
		padding: 0.625rem;
		background: transparent;
		border: none;
		font-size: 0.8125rem;
		font-weight: 500;
		color: #6366f1;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.insert-button:hover {
		background: #f0f0ff;
	}

	.dropdown-toggle {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 36px;
		background: transparent;
		border: none;
		border-left: 1px solid #e5e7eb;
		color: #9ca3af;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.dropdown-toggle:hover {
		background: #f3f4f6;
		color: #6b7280;
	}

	/* Insert Dropdown */
	.insert-dropdown {
		position: absolute;
		bottom: 100%;
		right: 0;
		width: 240px;
		background: white;
		border: 1px solid #e5e7eb;
		border-radius: 10px;
		box-shadow:
			0 10px 25px rgba(0, 0, 0, 0.15),
			0 0 0 1px rgba(0, 0, 0, 0.05);
		z-index: 10;
		overflow: hidden;
	}

	.dropdown-item {
		display: flex;
		align-items: flex-start;
		gap: 0.75rem;
		width: 100%;
		padding: 0.75rem 1rem;
		background: transparent;
		border: none;
		text-align: left;
		cursor: pointer;
		transition: background 0.15s ease;
	}

	.dropdown-item:hover {
		background: #f3f4f6;
	}

	.dropdown-item:first-child {
		border-bottom: 1px solid #e5e7eb;
	}

	.dropdown-item :global(svg) {
		flex-shrink: 0;
		margin-top: 0.125rem;
		color: #6b7280;
	}

	.dropdown-item-content {
		flex: 1;
	}

	.dropdown-item-label {
		display: block;
		font-size: 0.875rem;
		font-weight: 500;
		color: #1f2937;
	}

	.dropdown-item-desc {
		display: block;
		font-size: 0.75rem;
		color: #9ca3af;
		margin-top: 0.125rem;
	}

	/* ========================================================================
	   MODAL FOOTER
	   ======================================================================== */

	.modal-footer {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0.875rem 1.5rem;
		border-top: 1px solid #e5e7eb;
		background: #fafafa;
	}

	.keyboard-hints {
		display: flex;
		align-items: center;
		gap: 1.25rem;
	}

	.hint {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		font-size: 0.75rem;
		color: #9ca3af;
	}

	.hint kbd {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		min-width: 22px;
		height: 22px;
		padding: 0 0.375rem;
		background: white;
		border: 1px solid #e5e7eb;
		border-radius: 4px;
		font-family: inherit;
		font-size: 0.6875rem;
		font-weight: 500;
		color: #6b7280;
	}

	.results-count {
		font-size: 0.8125rem;
		color: #6b7280;
	}

	/* ========================================================================
	   SCROLLBAR
	   ======================================================================== */

	.content-area::-webkit-scrollbar {
		width: 8px;
	}

	.content-area::-webkit-scrollbar-track {
		background: #f3f4f6;
	}

	.content-area::-webkit-scrollbar-thumb {
		background: #d1d5db;
		border-radius: 4px;
	}

	.content-area::-webkit-scrollbar-thumb:hover {
		background: #9ca3af;
	}

	/* ========================================================================
	   RESPONSIVE
	   ======================================================================== */

	@media (max-width: 640px) {
		.modal-container {
			max-height: 100vh;
			border-radius: 0;
		}

		.modal-header {
			padding: 1rem;
		}

		.header-icon {
			width: 40px;
			height: 40px;
		}

		.modal-title {
			font-size: 1.125rem;
		}

		.filters-section {
			padding: 1rem;
		}

		.content-area {
			padding: 1rem;
		}

		.blocks-grid {
			grid-template-columns: 1fr;
		}

		.keyboard-hints {
			display: none;
		}

		.modal-footer {
			justify-content: center;
		}
	}

	/* ========================================================================
	   DARK MODE
	   ======================================================================== */

	@media (prefers-color-scheme: dark) {
		.modal-container {
			background: #1f2937;
			box-shadow:
				0 25px 50px -12px rgba(0, 0, 0, 0.5),
				0 0 0 1px rgba(255, 255, 255, 0.1);
		}

		.modal-header {
			border-color: #374151;
			background: linear-gradient(to bottom, #1f2937, #1f2937);
		}

		.modal-title {
			color: #f9fafb;
		}

		.modal-description {
			color: #9ca3af;
		}

		.close-button {
			color: #9ca3af;
		}

		.close-button:hover {
			background: #374151;
			color: #f9fafb;
		}

		.filters-section {
			background: #1a1f2e;
			border-color: #374151;
		}

		.search-input {
			background: #374151;
			border-color: #4b5563;
			color: #f9fafb;
		}

		.search-input:focus {
			border-color: #6366f1;
		}

		.clear-search {
			background: #4b5563;
			color: #9ca3af;
		}

		.clear-search:hover {
			background: #6b7280;
			color: #f9fafb;
		}

		.category-tab {
			background: #374151;
			border-color: #4b5563;
			color: #9ca3af;
		}

		.category-tab:hover {
			border-color: #6b7280;
			color: #f9fafb;
		}

		.content-area {
			background: #1f2937;
		}

		.loading-state,
		.error-state,
		.empty-state {
			color: #9ca3af;
		}

		.empty-title {
			color: #f9fafb;
		}

		.block-card {
			background: #374151;
			border-color: #4b5563;
		}

		.block-card:hover,
		.block-card.hovered {
			border-color: #6366f1;
		}

		.block-preview {
			background: #1f2937;
		}

		.block-name {
			color: #f9fafb;
		}

		.block-description {
			color: #9ca3af;
		}

		.block-actions {
			border-color: #4b5563;
		}

		.insert-button:hover {
			background: rgba(99, 102, 241, 0.15);
		}

		.dropdown-toggle {
			border-color: #4b5563;
		}

		.dropdown-toggle:hover {
			background: #4b5563;
		}

		.insert-dropdown {
			background: #374151;
			border-color: #4b5563;
		}

		.dropdown-item:hover {
			background: #4b5563;
		}

		.dropdown-item:first-child {
			border-color: #4b5563;
		}

		.dropdown-item-label {
			color: #f9fafb;
		}

		.modal-footer {
			background: #1a1f2e;
			border-color: #374151;
		}

		.hint kbd {
			background: #374151;
			border-color: #4b5563;
			color: #9ca3af;
		}

		.content-area::-webkit-scrollbar-track {
			background: #1f2937;
		}

		.content-area::-webkit-scrollbar-thumb {
			background: #4b5563;
		}

		.content-area::-webkit-scrollbar-thumb:hover {
			background: #6b7280;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE DESIGN - Mobile First (Apple ICT 7 Standards)
	   Touch targets: 44x44px minimum on mobile
	   ═══════════════════════════════════════════════════════════════════════════ */

	/* Mobile Base (< 640px) */
	.picker-modal {
		width: calc(100vw - 2rem);
		max-width: 100%;
		height: calc(100vh - 2rem);
		max-height: calc(100vh - 2rem);
		margin: 1rem;
	}

	.modal-header {
		padding: 0.75rem 1rem;
		flex-wrap: wrap;
		gap: 0.5rem;
	}

	.search-input {
		min-height: 44px;
		font-size: 1rem;
	}

	.category-tab {
		min-height: 44px;
		padding: 0.75rem 1rem;
		font-size: 0.8125rem;
	}

	.blocks-grid {
		grid-template-columns: 1fr;
		gap: 0.75rem;
		padding: 1rem;
	}

	.block-card {
		min-height: 44px;
	}

	.insert-button,
	.dropdown-toggle {
		min-height: 44px;
		padding: 0.75rem 1rem;
	}

	.modal-footer {
		padding: 0.75rem 1rem;
		flex-wrap: wrap;
		gap: 0.5rem;
	}

	.hint {
		display: none;
	}

	/* Tablet (≥ 640px) */
	@media (min-width: 640px) {
		.picker-modal {
			width: 90vw;
			max-width: 600px;
			height: auto;
			max-height: 80vh;
			margin: auto;
		}

		.blocks-grid {
			grid-template-columns: repeat(2, 1fr);
			gap: 1rem;
		}

		.category-tab {
			min-height: auto;
			padding: 0.5rem 0.875rem;
		}

		.search-input {
			min-height: auto;
		}

		.insert-button,
		.dropdown-toggle {
			min-height: auto;
			padding: 0.5rem 0.75rem;
		}

		.hint {
			display: flex;
		}
	}

	/* Desktop (≥ 768px) */
	@media (min-width: 768px) {
		.picker-modal {
			max-width: 700px;
		}

		.modal-header {
			padding: 1rem 1.25rem;
		}

		.modal-footer {
			padding: 0.75rem 1.25rem;
		}
	}

	/* Large Desktop (≥ 1024px) */
	@media (min-width: 1024px) {
		.picker-modal {
			max-width: 800px;
		}

		.blocks-grid {
			grid-template-columns: repeat(3, 1fr);
		}
	}

	/* Extra Large (≥ 1280px) */
	@media (min-width: 1280px) {
		.picker-modal {
			max-width: 900px;
		}
	}
</style>

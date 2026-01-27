<!--
	Slash Commands - CMS Editor Command Palette
	═══════════════════════════════════════════════════════════════════════════════

	A floating command palette that appears when user types "/" in the editor.
	Provides quick access to formatting, blocks, embeds, and trading-specific elements.

	Features:
	- Keyboard navigation (Arrow Up/Down, Enter, Escape)
	- Real-time filtering as user types
	- Grouped commands by category
	- Smooth animations
	- Full accessibility support

	@version 1.0.0
	@author Revolution Trading Pros
	@since January 2026
-->

<script lang="ts">
	import { onMount } from 'svelte';
	import { fade, fly, scale } from 'svelte/transition';
	import { quintOut, backOut } from 'svelte/easing';

	// Icons
	import IconH1 from '@tabler/icons-svelte/icons/h-1';
	import IconH2 from '@tabler/icons-svelte/icons/h-2';
	import IconH3 from '@tabler/icons-svelte/icons/h-3';
	import IconAlignLeft from '@tabler/icons-svelte/icons/align-left';
	import IconQuote from '@tabler/icons-svelte/icons/quote';
	import IconMinus from '@tabler/icons-svelte/icons/minus';
	import IconList from '@tabler/icons-svelte/icons/list';
	import IconListNumbers from '@tabler/icons-svelte/icons/list-numbers';
	import IconCode from '@tabler/icons-svelte/icons/code';
	import IconPhoto from '@tabler/icons-svelte/icons/photo';
	import IconVideo from '@tabler/icons-svelte/icons/video';
	import IconTable from '@tabler/icons-svelte/icons/table';
	import IconInfoCircle from '@tabler/icons-svelte/icons/info-circle';
	import IconAlertTriangle from '@tabler/icons-svelte/icons/alert-triangle';
	import IconChartCandle from '@tabler/icons-svelte/icons/chart-candle';
	import IconChartLine from '@tabler/icons-svelte/icons/chart-line';
	import IconTrendingUp from '@tabler/icons-svelte/icons/trending-up';
	import IconBrandYoutube from '@tabler/icons-svelte/icons/brand-youtube';
	import IconBrandTwitter from '@tabler/icons-svelte/icons/brand-twitter';
	import IconPlayerPlay from '@tabler/icons-svelte/icons/player-play';
	import IconTemplate from '@tabler/icons-svelte/icons/template';
	import IconColumns from '@tabler/icons-svelte/icons/columns';
	import IconSearch from '@tabler/icons-svelte/icons/search';
	import IconX from '@tabler/icons-svelte/icons/x';
	import IconArrowUp from '@tabler/icons-svelte/icons/arrow-up';
	import IconArrowDown from '@tabler/icons-svelte/icons/arrow-down';
	import IconCornerDownLeft from '@tabler/icons-svelte/icons/corner-down-left';

	// ==========================================================================
	// Types
	// ==========================================================================

	export interface SlashCommand {
		id: string;
		label: string;
		description: string;
		category: CommandCategory;
		// eslint-disable-next-line @typescript-eslint/no-explicit-any
		icon: any;
		keywords: string[];
		shortcut?: string;
	}

	export type CommandCategory =
		| 'formatting'
		| 'blocks'
		| 'trading'
		| 'embeds'
		| 'actions';

	interface CategoryMeta {
		id: CommandCategory;
		label: string;
		color: string;
	}

	// ==========================================================================
	// Props
	// ==========================================================================

	interface Props {
		position: { x: number; y: number };
		visible: boolean;
		filter: string;
		onSelect: (command: SlashCommand) => void;
		onClose: () => void;
	}

	let { position, visible, filter, onSelect, onClose }: Props = $props();

	// ==========================================================================
	// Command Definitions
	// ==========================================================================

	const CATEGORIES: CategoryMeta[] = [
		{ id: 'formatting', label: 'Formatting', color: '#3b82f6' },
		{ id: 'blocks', label: 'Blocks', color: '#8b5cf6' },
		{ id: 'trading', label: 'Trading-Specific', color: '#e6b800' },
		{ id: 'embeds', label: 'Embeds', color: '#ec4899' },
		{ id: 'actions', label: 'Actions', color: '#10b981' }
	];

	const COMMANDS: SlashCommand[] = [
		// Formatting
		{
			id: 'heading1',
			label: 'Heading 1',
			description: 'Large section heading',
			category: 'formatting',
			icon: IconH1,
			keywords: ['h1', 'title', 'header', 'large'],
			shortcut: '#'
		},
		{
			id: 'heading2',
			label: 'Heading 2',
			description: 'Medium section heading',
			category: 'formatting',
			icon: IconH2,
			keywords: ['h2', 'subtitle', 'header', 'medium'],
			shortcut: '##'
		},
		{
			id: 'heading3',
			label: 'Heading 3',
			description: 'Small section heading',
			category: 'formatting',
			icon: IconH3,
			keywords: ['h3', 'subheading', 'header', 'small'],
			shortcut: '###'
		},
		{
			id: 'paragraph',
			label: 'Paragraph',
			description: 'Plain text paragraph',
			category: 'formatting',
			icon: IconAlignLeft,
			keywords: ['text', 'body', 'normal', 'p']
		},
		{
			id: 'quote',
			label: 'Quote',
			description: 'Highlighted quote or citation',
			category: 'formatting',
			icon: IconQuote,
			keywords: ['blockquote', 'citation', 'pullquote'],
			shortcut: '>'
		},
		{
			id: 'divider',
			label: 'Divider',
			description: 'Horizontal line separator',
			category: 'formatting',
			icon: IconMinus,
			keywords: ['hr', 'line', 'separator', 'break'],
			shortcut: '---'
		},
		{
			id: 'bullet-list',
			label: 'Bullet List',
			description: 'Unordered list with bullets',
			category: 'formatting',
			icon: IconList,
			keywords: ['ul', 'unordered', 'bullets', 'dots'],
			shortcut: '-'
		},
		{
			id: 'numbered-list',
			label: 'Numbered List',
			description: 'Ordered list with numbers',
			category: 'formatting',
			icon: IconListNumbers,
			keywords: ['ol', 'ordered', 'numbers', 'sequence'],
			shortcut: '1.'
		},
		{
			id: 'code-block',
			label: 'Code Block',
			description: 'Syntax-highlighted code',
			category: 'formatting',
			icon: IconCode,
			keywords: ['pre', 'syntax', 'programming', 'snippet'],
			shortcut: '```'
		},

		// Blocks
		{
			id: 'image',
			label: 'Image',
			description: 'Upload or embed an image',
			category: 'blocks',
			icon: IconPhoto,
			keywords: ['picture', 'photo', 'img', 'media']
		},
		{
			id: 'video',
			label: 'Video',
			description: 'Embed a video player',
			category: 'blocks',
			icon: IconVideo,
			keywords: ['media', 'player', 'clip', 'movie']
		},
		{
			id: 'table',
			label: 'Table',
			description: 'Insert a data table',
			category: 'blocks',
			icon: IconTable,
			keywords: ['grid', 'data', 'rows', 'columns', 'spreadsheet']
		},
		{
			id: 'callout',
			label: 'Callout',
			description: 'Highlighted information box',
			category: 'blocks',
			icon: IconInfoCircle,
			keywords: ['info', 'note', 'tip', 'highlight', 'box']
		},
		{
			id: 'alert-box',
			label: 'Alert Box',
			description: 'Warning or important notice',
			category: 'blocks',
			icon: IconAlertTriangle,
			keywords: ['warning', 'danger', 'notice', 'caution', 'important']
		},

		// Trading-Specific
		{
			id: 'trade-setup',
			label: 'Trade Setup',
			description: 'Trade entry with price levels',
			category: 'trading',
			icon: IconChartCandle,
			keywords: ['entry', 'exit', 'stop', 'target', 'levels', 'position']
		},
		{
			id: 'performance-stats',
			label: 'Performance Stats',
			description: 'Trading metrics and statistics',
			category: 'trading',
			icon: IconTrendingUp,
			keywords: ['metrics', 'stats', 'win', 'loss', 'pnl', 'profit', 'results']
		},
		{
			id: 'tradingview-chart',
			label: 'TradingView Chart',
			description: 'Embed TradingView widget',
			category: 'trading',
			icon: IconChartLine,
			keywords: ['chart', 'widget', 'technical', 'analysis', 'candles']
		},

		// Embeds
		{
			id: 'youtube',
			label: 'YouTube',
			description: 'Embed a YouTube video',
			category: 'embeds',
			icon: IconBrandYoutube,
			keywords: ['video', 'media', 'google', 'stream']
		},
		{
			id: 'vimeo',
			label: 'Vimeo',
			description: 'Embed a Vimeo video',
			category: 'embeds',
			icon: IconPlayerPlay,
			keywords: ['video', 'media', 'player', 'stream']
		},
		{
			id: 'twitter',
			label: 'Twitter/X',
			description: 'Embed a tweet or post',
			category: 'embeds',
			icon: IconBrandTwitter,
			keywords: ['tweet', 'x', 'social', 'post', 'embed']
		},

		// Actions
		{
			id: 'reusable-block',
			label: 'Reusable Block',
			description: 'Open reusable block picker',
			category: 'actions',
			icon: IconTemplate,
			keywords: ['template', 'saved', 'pattern', 'preset']
		},
		{
			id: 'columns',
			label: 'Columns',
			description: 'Create multi-column layout',
			category: 'actions',
			icon: IconColumns,
			keywords: ['layout', 'grid', 'side', 'row', 'split']
		}
	];

	// ==========================================================================
	// State
	// ==========================================================================

	let selectedIndex = $state(0);
	let menuRef = $state<HTMLDivElement | null>(null);
	let itemRefs = $state<HTMLButtonElement[]>([]);

	// ==========================================================================
	// Derived State
	// ==========================================================================

	let normalizedFilter = $derived(filter.toLowerCase().replace(/^\//, '').trim());

	let filteredCommands = $derived.by(() => {
		if (!normalizedFilter) return COMMANDS;

		return COMMANDS.filter((cmd) => {
			const searchTarget = [
				cmd.id,
				cmd.label,
				cmd.description,
				...cmd.keywords
			]
				.join(' ')
				.toLowerCase();

			return searchTarget.includes(normalizedFilter);
		});
	});

	let groupedCommands = $derived.by(() => {
		const groups = new Map<CommandCategory, SlashCommand[]>();

		for (const cmd of filteredCommands) {
			const existing = groups.get(cmd.category) || [];
			groups.set(cmd.category, [...existing, cmd]);
		}

		return CATEGORIES.filter((cat) => groups.has(cat.id)).map((cat) => ({
			...cat,
			commands: groups.get(cat.id) || []
		}));
	});

	let flatCommands = $derived(groupedCommands.flatMap((g) => g.commands));

	let hasResults = $derived(flatCommands.length > 0);

	// Compute position with viewport bounds checking
	let computedPosition = $derived.by(() => {
		const menuWidth = 320;
		const menuHeight = 400;
		const padding = 16;

		let x = position.x;
		let y = position.y;

		if (typeof window !== 'undefined') {
			const maxX = window.innerWidth - menuWidth - padding;
			const maxY = window.innerHeight - menuHeight - padding;

			x = Math.min(Math.max(padding, x), maxX);
			y = Math.min(Math.max(padding, y), maxY);
		}

		return { x, y };
	});

	// ==========================================================================
	// Effects
	// ==========================================================================

	// Reset selection when filter changes
	$effect(() => {
		normalizedFilter;
		selectedIndex = 0;
	});

	// Scroll selected item into view
	$effect(() => {
		if (visible && itemRefs[selectedIndex]) {
			itemRefs[selectedIndex].scrollIntoView({
				block: 'nearest',
				behavior: 'smooth'
			});
		}
	});

	// ==========================================================================
	// Handlers
	// ==========================================================================

	function handleKeyDown(e: KeyboardEvent) {
		if (!visible) return;

		switch (e.key) {
			case 'ArrowDown':
				e.preventDefault();
				e.stopPropagation();
				selectedIndex = Math.min(selectedIndex + 1, flatCommands.length - 1);
				break;

			case 'ArrowUp':
				e.preventDefault();
				e.stopPropagation();
				selectedIndex = Math.max(selectedIndex - 1, 0);
				break;

			case 'Enter':
				e.preventDefault();
				e.stopPropagation();
				if (flatCommands[selectedIndex]) {
					handleSelect(flatCommands[selectedIndex]);
				}
				break;

			case 'Escape':
				e.preventDefault();
				e.stopPropagation();
				onClose();
				break;

			case 'Tab':
				e.preventDefault();
				e.stopPropagation();
				if (e.shiftKey) {
					selectedIndex = Math.max(selectedIndex - 1, 0);
				} else {
					selectedIndex = Math.min(selectedIndex + 1, flatCommands.length - 1);
				}
				break;
		}
	}

	function handleSelect(command: SlashCommand) {
		onSelect(command);
	}

	function handleMouseEnter(index: number) {
		selectedIndex = index;
	}

	function getCategoryColor(categoryId: CommandCategory): string {
		return CATEGORIES.find((c) => c.id === categoryId)?.color || '#64748b';
	}

	function getGlobalIndex(command: SlashCommand): number {
		return flatCommands.indexOf(command);
	}

	// ==========================================================================
	// Lifecycle
	// ==========================================================================

	onMount(() => {
		window.addEventListener('keydown', handleKeyDown);

		return () => {
			window.removeEventListener('keydown', handleKeyDown);
		};
	});
</script>

{#if visible}
	<!-- Backdrop for click-outside -->
	<div
		class="slash-commands-backdrop"
		onclick={onClose}
		onkeydown={(e) => e.key === 'Escape' && onClose()}
		role="button"
		tabindex="-1"
		aria-label="Close command palette"
		transition:fade={{ duration: 100 }}
	></div>

	<!-- Command Palette -->
	<div
		bind:this={menuRef}
		class="slash-commands-menu"
		role="listbox"
		aria-label="Editor commands"
		aria-activedescendant={flatCommands[selectedIndex]?.id}
		style:left="{computedPosition.x}px"
		style:top="{computedPosition.y}px"
		transition:scale={{ duration: 200, start: 0.95, easing: backOut }}
	>
		<!-- Search Header -->
		<div class="slash-header">
			<div class="slash-search-icon">
				<IconSearch size={16} />
			</div>
			<span class="slash-filter-text">
				{#if normalizedFilter}
					Filtering: <strong>{normalizedFilter}</strong>
				{:else}
					Type to filter commands...
				{/if}
			</span>
			<button
				type="button"
				class="slash-close-btn"
				onclick={onClose}
				aria-label="Close"
			>
				<IconX size={16} />
			</button>
		</div>

		<!-- Commands List -->
		<div class="slash-content">
			{#if hasResults}
				{#each groupedCommands as group, groupIndex (group.id)}
					<div
						class="slash-group"
						transition:fly={{ y: -8, duration: 150, delay: groupIndex * 30, easing: quintOut }}
					>
						<!-- Category Header -->
						<div
							class="slash-category-header"
							style:--category-color={group.color}
						>
							<span class="slash-category-dot"></span>
							<span class="slash-category-label">{group.label}</span>
							<span class="slash-category-count">{group.commands.length}</span>
						</div>

						<!-- Commands -->
						{#each group.commands as command, cmdIndex (command.id)}
							{@const globalIndex = getGlobalIndex(command)}
							{@const isSelected = selectedIndex === globalIndex}
							{@const Icon = command.icon}

							<button
								bind:this={itemRefs[globalIndex]}
								type="button"
								class="slash-command-item"
								class:selected={isSelected}
								id={command.id}
								role="option"
								aria-selected={isSelected}
								onclick={() => handleSelect(command)}
								onmouseenter={() => handleMouseEnter(globalIndex)}
								onfocus={() => handleMouseEnter(globalIndex)}
							>
								<div
									class="slash-command-icon"
									style:--icon-color={group.color}
								>
									<Icon size={18} />
								</div>
								<div class="slash-command-content">
									<span class="slash-command-label">{command.label}</span>
									<span class="slash-command-description">{command.description}</span>
								</div>
								{#if command.shortcut}
									<span class="slash-command-shortcut">{command.shortcut}</span>
								{/if}
								{#if isSelected}
									<div
										class="slash-command-selected-indicator"
										transition:scale={{ duration: 150, easing: quintOut }}
									>
										<IconCornerDownLeft size={14} />
									</div>
								{/if}
							</button>
						{/each}
					</div>
				{/each}
			{:else}
				<!-- No Results -->
				<div class="slash-no-results" transition:fade={{ duration: 150 }}>
					<IconSearch size={32} />
					<p class="slash-no-results-text">
						No commands found for "<strong>{normalizedFilter}</strong>"
					</p>
					<p class="slash-no-results-hint">Try a different search term</p>
				</div>
			{/if}
		</div>

		<!-- Footer with keyboard hints -->
		<div class="slash-footer">
			<div class="slash-footer-hints">
				<span class="slash-hint">
					<kbd><IconArrowUp size={10} /></kbd>
					<kbd><IconArrowDown size={10} /></kbd>
					<span>Navigate</span>
				</span>
				<span class="slash-hint">
					<kbd><IconCornerDownLeft size={10} /></kbd>
					<span>Select</span>
				</span>
				<span class="slash-hint">
					<kbd>esc</kbd>
					<span>Close</span>
				</span>
			</div>
		</div>
	</div>
{/if}

<style>
	/* Backdrop */
	.slash-commands-backdrop {
		position: fixed;
		inset: 0;
		z-index: 9998;
		background: transparent;
	}

	/* Menu Container */
	.slash-commands-menu {
		position: fixed;
		z-index: 9999;
		width: 320px;
		max-height: 420px;
		background: linear-gradient(
			135deg,
			rgba(30, 41, 59, 0.98),
			rgba(15, 23, 42, 0.98)
		);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 12px;
		box-shadow:
			0 25px 50px -12px rgba(0, 0, 0, 0.5),
			0 0 0 1px rgba(255, 255, 255, 0.05) inset,
			0 0 40px -10px rgba(99, 102, 241, 0.15);
		display: flex;
		flex-direction: column;
		overflow: hidden;
		backdrop-filter: blur(16px);
	}

	/* Header */
	.slash-header {
		display: flex;
		align-items: center;
		gap: 0.625rem;
		padding: 0.75rem 1rem;
		border-bottom: 1px solid rgba(99, 102, 241, 0.15);
		background: rgba(0, 0, 0, 0.2);
	}

	.slash-search-icon {
		color: #64748b;
		flex-shrink: 0;
	}

	.slash-filter-text {
		flex: 1;
		font-size: 0.8125rem;
		color: #94a3b8;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.slash-filter-text strong {
		color: #e6b800;
		font-weight: 600;
	}

	.slash-close-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 24px;
		height: 24px;
		background: transparent;
		border: none;
		border-radius: 6px;
		color: #64748b;
		cursor: pointer;
		transition: all 0.15s;
		flex-shrink: 0;
	}

	.slash-close-btn:hover {
		background: rgba(99, 102, 241, 0.15);
		color: #f1f5f9;
	}

	/* Content */
	.slash-content {
		flex: 1;
		overflow-y: auto;
		padding: 0.5rem;
		scrollbar-width: thin;
		scrollbar-color: rgba(99, 102, 241, 0.3) transparent;
	}

	.slash-content::-webkit-scrollbar {
		width: 6px;
	}

	.slash-content::-webkit-scrollbar-track {
		background: transparent;
	}

	.slash-content::-webkit-scrollbar-thumb {
		background: rgba(99, 102, 241, 0.3);
		border-radius: 3px;
	}

	.slash-content::-webkit-scrollbar-thumb:hover {
		background: rgba(99, 102, 241, 0.5);
	}

	/* Groups */
	.slash-group {
		margin-bottom: 0.5rem;
	}

	.slash-group:last-child {
		margin-bottom: 0;
	}

	.slash-category-header {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 0.75rem;
		font-size: 0.6875rem;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		color: #64748b;
	}

	.slash-category-dot {
		width: 6px;
		height: 6px;
		border-radius: 50%;
		background: var(--category-color, #64748b);
		box-shadow: 0 0 8px var(--category-color);
	}

	.slash-category-label {
		flex: 1;
	}

	.slash-category-count {
		font-size: 0.625rem;
		padding: 0.125rem 0.375rem;
		background: rgba(99, 102, 241, 0.1);
		border-radius: 4px;
		color: #94a3b8;
	}

	/* Command Items */
	.slash-command-item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		width: 100%;
		padding: 0.625rem 0.75rem;
		background: transparent;
		border: none;
		border-radius: 8px;
		color: #cbd5e1;
		font-size: 0.875rem;
		text-align: left;
		cursor: pointer;
		transition: all 0.15s;
		position: relative;
	}

	.slash-command-item:hover,
	.slash-command-item.selected {
		background: rgba(99, 102, 241, 0.12);
		color: #f1f5f9;
	}

	.slash-command-item.selected {
		background: rgba(99, 102, 241, 0.18);
	}

	.slash-command-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		background: rgba(99, 102, 241, 0.1);
		border-radius: 8px;
		color: var(--icon-color, #818cf8);
		flex-shrink: 0;
		transition: all 0.15s;
	}

	.slash-command-item.selected .slash-command-icon {
		background: rgba(99, 102, 241, 0.2);
		transform: scale(1.05);
	}

	.slash-command-content {
		flex: 1;
		min-width: 0;
		display: flex;
		flex-direction: column;
		gap: 0.125rem;
	}

	.slash-command-label {
		font-weight: 500;
		color: inherit;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.slash-command-description {
		font-size: 0.75rem;
		color: #64748b;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.slash-command-item.selected .slash-command-description {
		color: #94a3b8;
	}

	.slash-command-shortcut {
		font-size: 0.6875rem;
		font-family: 'Fira Code', 'Monaco', monospace;
		padding: 0.125rem 0.375rem;
		background: rgba(0, 0, 0, 0.3);
		border-radius: 4px;
		color: #64748b;
		flex-shrink: 0;
	}

	.slash-command-selected-indicator {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 22px;
		height: 22px;
		background: rgba(230, 184, 0, 0.2);
		border-radius: 6px;
		color: #e6b800;
		flex-shrink: 0;
	}

	/* No Results */
	.slash-no-results {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 2.5rem 1.5rem;
		color: #64748b;
		text-align: center;
	}

	.slash-no-results-text {
		margin-top: 1rem;
		font-size: 0.875rem;
		color: #94a3b8;
	}

	.slash-no-results-text strong {
		color: #e6b800;
	}

	.slash-no-results-hint {
		margin-top: 0.375rem;
		font-size: 0.75rem;
		color: #64748b;
	}

	/* Footer */
	.slash-footer {
		padding: 0.625rem 1rem;
		border-top: 1px solid rgba(99, 102, 241, 0.1);
		background: rgba(0, 0, 0, 0.15);
	}

	.slash-footer-hints {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.slash-hint {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		font-size: 0.6875rem;
		color: #64748b;
	}

	.slash-hint kbd {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		min-width: 18px;
		height: 18px;
		padding: 0 0.25rem;
		background: rgba(99, 102, 241, 0.1);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 4px;
		color: #94a3b8;
		font-family: inherit;
		font-size: 0.625rem;
		text-transform: lowercase;
	}

	/* Responsive */
	@media (max-width: 480px) {
		.slash-commands-menu {
			width: calc(100vw - 32px);
			max-width: 320px;
			left: 16px !important;
		}

		.slash-footer-hints {
			gap: 0.75rem;
		}

		.slash-hint span {
			display: none;
		}
	}

	/* Animation for items */
	@keyframes slideIn {
		from {
			opacity: 0;
			transform: translateY(-4px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	.slash-command-item {
		animation: slideIn 0.15s ease-out backwards;
	}

	.slash-group:nth-child(1) .slash-command-item:nth-child(1) {
		animation-delay: 0ms;
	}
	.slash-group:nth-child(1) .slash-command-item:nth-child(2) {
		animation-delay: 20ms;
	}
	.slash-group:nth-child(1) .slash-command-item:nth-child(3) {
		animation-delay: 40ms;
	}
	.slash-group:nth-child(2) .slash-command-item:nth-child(1) {
		animation-delay: 60ms;
	}
	.slash-group:nth-child(2) .slash-command-item:nth-child(2) {
		animation-delay: 80ms;
	}
</style>

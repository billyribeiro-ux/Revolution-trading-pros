<!--
/**
 * Revolution Trading Pros - CMS v2 Block Group Component
 * ═══════════════════════════════════════════════════════════════════════════
 * Groups multiple blocks together with flexible layout options
 *
 * Features:
 * - Multiple layout modes (stack, 2/3/4 columns, auto-fit grid)
 * - Styling options (background, padding, border radius, gap)
 * - Nested groups support
 * - Drag and drop reordering within group
 * - Visual indicators for grouping
 * - Responsive preview
 * - Full accessibility with ARIA attributes
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 * @since January 2026
 */
-->

<script module lang="ts">
	// ==========================================================================
	// Module-level Type Exports (Svelte 5 requirement)
	// ==========================================================================

	export type LayoutType = 'stack' | 'columns';
	export type PaddingSize = 'none' | 'sm' | 'md' | 'lg' | 'xl';
	export type GapSize = 'none' | 'sm' | 'md' | 'lg' | 'xl';
	export type BorderRadiusSize = 'none' | 'sm' | 'md' | 'lg' | 'xl' | 'full';

	export interface GroupBlockData {
		layout: LayoutType;
		columns: 1 | 2 | 3 | 4 | 'auto';
		gap: GapSize;
		backgroundColor?: string;
		backgroundImage?: string;
		backgroundOverlay?: string;
		padding: PaddingSize;
		borderRadius: BorderRadiusSize;
		[key: string]: unknown;
	}

	export interface Block {
		id: string;
		blockType: string;
		data: Record<string, unknown>;
		children?: Block[];
	}

	export interface GroupBlock extends Block {
		blockType: 'group';
		data: GroupBlockData;
		children: Block[];
	}

	export interface ChildRenderProps {
		block: Block;
		isNested: boolean;
		nestingLevel: number;
		onUpdate: (updated: Block) => void;
	}
</script>

<script lang="ts">
	import { flip } from 'svelte/animate';
	import { fade, scale } from 'svelte/transition';
	import { quintOut } from 'svelte/easing';
	import IconLayoutColumns from '@tabler/icons-svelte/icons/layout-columns';
	import IconLayoutRows from '@tabler/icons-svelte/icons/layout-rows';
	import IconLayout2 from '@tabler/icons-svelte/icons/layout-2';
	import IconLayoutGrid from '@tabler/icons-svelte/icons/layout-grid';
	import IconPalette from '@tabler/icons-svelte/icons/palette';
	import IconBoxPadding from '@tabler/icons-svelte/icons/box-padding';
	import IconBorderRadius from '@tabler/icons-svelte/icons/border-radius';
	import IconArrowsVertical from '@tabler/icons-svelte/icons/arrows-vertical';
	import IconGripVertical from '@tabler/icons-svelte/icons/grip-vertical';
	import IconTrash from '@tabler/icons-svelte/icons/trash';
	import IconPlus from '@tabler/icons-svelte/icons/plus';
	import IconSettings from '@tabler/icons-svelte/icons/settings';
	import IconChevronDown from '@tabler/icons-svelte/icons/chevron-down';
	import IconPhoto from '@tabler/icons-svelte/icons/photo';
	import IconStack2 from '@tabler/icons-svelte/icons/stack-2';
	import type { Snippet } from 'svelte';

	// ==========================================================================
	// Props
	// ==========================================================================

	interface Props {
		block: GroupBlock;
		isSelected?: boolean;
		isPreview?: boolean;
		nestingLevel?: number;
		onUpdate?: (block: GroupBlock) => void;
		onChildUpdate?: (childId: string, child: Block) => void;
		onChildRemove?: (childId: string) => void;
		onChildReorder?: (childIds: string[]) => void;
		onAddChild?: () => void;
		children?: Snippet<[ChildRenderProps]>;
	}

	let {
		block,
		isSelected = false,
		isPreview = false,
		nestingLevel = 0,
		onUpdate,
		onChildUpdate,
		onChildRemove,
		onChildReorder,
		onAddChild,
		children: renderChild
	}: Props = $props();

	// ==========================================================================
	// State
	// ==========================================================================

	let showToolbar = $state(false);
	let showStylePanel = $state(false);
	let isDragging = $state(false);
	let draggedChildId = $state<string | null>(null);
	let dropTargetIndex = $state<number | null>(null);
	let activeTab = $state<'layout' | 'style'>('layout');
	let containerRef = $state<HTMLDivElement | null>(null);

	// ==========================================================================
	// Derived Values
	// ==========================================================================

	const layoutClasses = $derived.by(() => {
		const { layout, columns, gap } = block.data;
		const classes: string[] = [];

		// Base layout
		if (layout === 'stack') {
			classes.push('flex', 'flex-col');
		} else if (layout === 'columns') {
			classes.push('grid');

			// Column configuration
			if (columns === 'auto') {
				classes.push('grid-cols-[repeat(auto-fit,minmax(200px,1fr))]');
			} else {
				switch (columns) {
					case 1:
						classes.push('grid-cols-1');
						break;
					case 2:
						classes.push('grid-cols-1', 'md:grid-cols-2');
						break;
					case 3:
						classes.push('grid-cols-1', 'md:grid-cols-2', 'lg:grid-cols-3');
						break;
					case 4:
						classes.push('grid-cols-1', 'sm:grid-cols-2', 'lg:grid-cols-4');
						break;
				}
			}
		}

		// Gap
		switch (gap) {
			case 'none':
				classes.push('gap-0');
				break;
			case 'sm':
				classes.push('gap-2');
				break;
			case 'md':
				classes.push('gap-4');
				break;
			case 'lg':
				classes.push('gap-6');
				break;
			case 'xl':
				classes.push('gap-8');
				break;
		}

		return classes.join(' ');
	});

	const paddingClasses = $derived.by(() => {
		switch (block.data.padding) {
			case 'none':
				return 'p-0';
			case 'sm':
				return 'p-2';
			case 'md':
				return 'p-4';
			case 'lg':
				return 'p-6';
			case 'xl':
				return 'p-8';
			default:
				return 'p-4';
		}
	});

	const borderRadiusClasses = $derived.by(() => {
		switch (block.data.borderRadius) {
			case 'none':
				return 'rounded-none';
			case 'sm':
				return 'rounded-sm';
			case 'md':
				return 'rounded-md';
			case 'lg':
				return 'rounded-lg';
			case 'xl':
				return 'rounded-xl';
			case 'full':
				return 'rounded-3xl';
			default:
				return 'rounded-none';
		}
	});

	const containerStyles = $derived.by(() => {
		const styles: Record<string, string> = {};

		if (block.data.backgroundColor) {
			styles.backgroundColor = block.data.backgroundColor;
		}

		if (block.data.backgroundImage) {
			styles.backgroundImage = `url(${block.data.backgroundImage})`;
			styles.backgroundSize = 'cover';
			styles.backgroundPosition = 'center';
		}

		return Object.entries(styles)
			.map(([key, value]) => `${key}: ${value}`)
			.join('; ');
	});

	const nestingColors = $derived.by(() => {
		const colors = [
			'border-blue-400',
			'border-purple-400',
			'border-green-400',
			'border-orange-400',
			'border-pink-400'
		];
		return colors[nestingLevel % colors.length];
	});

	const nestingBgColors = $derived.by(() => {
		const colors = [
			'bg-blue-50',
			'bg-purple-50',
			'bg-green-50',
			'bg-orange-50',
			'bg-pink-50'
		];
		return colors[nestingLevel % colors.length];
	});

	// ==========================================================================
	// Layout Options
	// ==========================================================================

	const layoutOptions: { value: LayoutType; label: string; icon: typeof IconLayoutRows }[] = [
		{ value: 'stack', label: 'Stack', icon: IconLayoutRows },
		{ value: 'columns', label: 'Columns', icon: IconLayoutColumns }
	];

	const columnOptions: { value: 1 | 2 | 3 | 4 | 'auto'; label: string }[] = [
		{ value: 1, label: '1' },
		{ value: 2, label: '2' },
		{ value: 3, label: '3' },
		{ value: 4, label: '4' },
		{ value: 'auto', label: 'Auto' }
	];

	const paddingOptions: { value: PaddingSize; label: string }[] = [
		{ value: 'none', label: 'None' },
		{ value: 'sm', label: 'S' },
		{ value: 'md', label: 'M' },
		{ value: 'lg', label: 'L' },
		{ value: 'xl', label: 'XL' }
	];

	const gapOptions: { value: GapSize; label: string }[] = [
		{ value: 'none', label: 'None' },
		{ value: 'sm', label: 'S' },
		{ value: 'md', label: 'M' },
		{ value: 'lg', label: 'L' },
		{ value: 'xl', label: 'XL' }
	];

	const borderRadiusOptions: { value: BorderRadiusSize; label: string }[] = [
		{ value: 'none', label: 'None' },
		{ value: 'sm', label: 'S' },
		{ value: 'md', label: 'M' },
		{ value: 'lg', label: 'L' },
		{ value: 'xl', label: 'XL' },
		{ value: 'full', label: 'Full' }
	];

	// ==========================================================================
	// Event Handlers
	// ==========================================================================

	function handleLayoutChange(layout: LayoutType) {
		onUpdate?.({
			...block,
			data: { ...block.data, layout }
		});
	}

	function handleColumnsChange(columns: 1 | 2 | 3 | 4 | 'auto') {
		onUpdate?.({
			...block,
			data: { ...block.data, columns }
		});
	}

	function handleGapChange(gap: GapSize) {
		onUpdate?.({
			...block,
			data: { ...block.data, gap }
		});
	}

	function handlePaddingChange(padding: PaddingSize) {
		onUpdate?.({
			...block,
			data: { ...block.data, padding }
		});
	}

	function handleBorderRadiusChange(borderRadius: BorderRadiusSize) {
		onUpdate?.({
			...block,
			data: { ...block.data, borderRadius }
		});
	}

	function handleBackgroundColorChange(e: Event) {
		const target = e.target as HTMLInputElement;
		onUpdate?.({
			...block,
			data: { ...block.data, backgroundColor: target.value }
		});
	}

	function handleBackgroundImageChange(e: Event) {
		const target = e.target as HTMLInputElement;
		onUpdate?.({
			...block,
			data: { ...block.data, backgroundImage: target.value }
		});
	}

	function clearBackgroundColor() {
		onUpdate?.({
			...block,
			data: { ...block.data, backgroundColor: undefined }
		});
	}

	function clearBackgroundImage() {
		onUpdate?.({
			...block,
			data: { ...block.data, backgroundImage: undefined }
		});
	}

	// ==========================================================================
	// Drag and Drop
	// ==========================================================================

	function handleDragStart(e: DragEvent, childId: string) {
		if (isPreview) return;

		isDragging = true;
		draggedChildId = childId;

		if (e.dataTransfer) {
			e.dataTransfer.effectAllowed = 'move';
			e.dataTransfer.setData('text/plain', childId);
			e.dataTransfer.setData('application/x-block-group-child', childId);
		}
	}

	function handleDragOver(e: DragEvent, index: number) {
		e.preventDefault();
		if (!isDragging || !draggedChildId) return;

		dropTargetIndex = index;

		if (e.dataTransfer) {
			e.dataTransfer.dropEffect = 'move';
		}
	}

	function handleDragLeave(e: DragEvent) {
		// Only reset if we're actually leaving the container
		const relatedTarget = e.relatedTarget as HTMLElement;
		if (!containerRef?.contains(relatedTarget)) {
			dropTargetIndex = null;
		}
	}

	function handleDrop(e: DragEvent, targetIndex: number) {
		e.preventDefault();

		if (!draggedChildId || !block.children) return;

		const currentIndex = block.children.findIndex(child => child.id === draggedChildId);
		if (currentIndex === -1 || currentIndex === targetIndex) {
			resetDragState();
			return;
		}

		const newOrder = [...block.children];
		const [movedItem] = newOrder.splice(currentIndex, 1);
		const adjustedIndex = targetIndex > currentIndex ? targetIndex - 1 : targetIndex;
		newOrder.splice(adjustedIndex, 0, movedItem);

		onChildReorder?.(newOrder.map(child => child.id));
		resetDragState();
	}

	function handleDragEnd() {
		resetDragState();
	}

	function resetDragState() {
		isDragging = false;
		draggedChildId = null;
		dropTargetIndex = null;
	}

	// ==========================================================================
	// Keyboard Navigation
	// ==========================================================================

	function handleKeyDown(e: KeyboardEvent, childId: string, index: number) {
		if (isPreview) return;

		const children = block.children || [];

		switch (e.key) {
			case 'ArrowUp':
			case 'ArrowLeft':
				if (index > 0) {
					e.preventDefault();
					const newOrder = [...children];
					[newOrder[index - 1], newOrder[index]] = [newOrder[index], newOrder[index - 1]];
					onChildReorder?.(newOrder.map(child => child.id));
				}
				break;
			case 'ArrowDown':
			case 'ArrowRight':
				if (index < children.length - 1) {
					e.preventDefault();
					const newOrder = [...children];
					[newOrder[index], newOrder[index + 1]] = [newOrder[index + 1], newOrder[index]];
					onChildReorder?.(newOrder.map(child => child.id));
				}
				break;
			case 'Delete':
			case 'Backspace':
				if (e.ctrlKey || e.metaKey) {
					e.preventDefault();
					onChildRemove?.(childId);
				}
				break;
		}
	}
</script>

<!-- Main Group Container -->
<div
	bind:this={containerRef}
	class="block-group relative transition-all duration-200"
	class:ring-2={isSelected}
	class:ring-blue-500={isSelected}
	class:ring-offset-2={isSelected}
	role="group"
	aria-label="Block group with {block.children?.length || 0} children"
	aria-describedby="group-{block.id}-description"
	onmouseenter={() => !isPreview && (showToolbar = true)}
	onmouseleave={() => {
		if (!showStylePanel) {
			showToolbar = false;
		}
	}}
	onfocusin={() => !isPreview && (showToolbar = true)}
	onfocusout={(e) => {
		const relatedTarget = e.relatedTarget as HTMLElement;
		if (!containerRef?.contains(relatedTarget) && !showStylePanel) {
			showToolbar = false;
		}
	}}
>
	<!-- Screen reader description -->
	<span id="group-{block.id}-description" class="sr-only">
		Group containing {block.children?.length || 0} blocks arranged in {block.data.layout} layout
		{#if block.data.layout === 'columns'}
			with {block.data.columns === 'auto' ? 'auto-fit' : block.data.columns} columns
		{/if}
	</span>

	<!-- Group Visual Indicator (nesting indicator) -->
	{#if !isPreview}
		<div
			class="absolute -left-1 top-0 bottom-0 w-1 {nestingColors} rounded-full transition-opacity duration-200"
			class:opacity-100={isSelected || showToolbar}
			class:opacity-0={!isSelected && !showToolbar}
			aria-hidden="true"
		></div>
	{/if}

	<!-- Group Toolbar -->
	{#if showToolbar && !isPreview}
		<div
			class="absolute -top-12 left-0 right-0 z-20 flex items-center justify-between gap-2 px-2 py-1.5 bg-white border border-gray-200 rounded-lg shadow-lg"
			transition:fade={{ duration: 150 }}
			role="toolbar"
			aria-label="Group toolbar"
		>
			<!-- Left: Group indicator -->
			<div class="flex items-center gap-2">
				<div class="flex items-center gap-1.5 px-2 py-1 bg-gray-100 rounded-md text-xs font-medium text-gray-600">
					<IconStack2 size={14} />
					<span>Group</span>
					{#if nestingLevel > 0}
						<span class="text-gray-400">L{nestingLevel + 1}</span>
					{/if}
				</div>
			</div>

			<!-- Center: Layout controls -->
			<div class="flex items-center gap-1">
				<!-- Layout Type -->
				<div class="flex items-center gap-0.5 p-0.5 bg-gray-100 rounded-md" role="radiogroup" aria-label="Layout type">
					{#each layoutOptions as option}
						<button
							type="button"
							class="p-1.5 rounded transition-all duration-150"
							class:bg-white={block.data.layout === option.value}
							class:shadow-sm={block.data.layout === option.value}
							class:text-blue-600={block.data.layout === option.value}
							class:text-gray-500={block.data.layout !== option.value}
							onclick={() => handleLayoutChange(option.value)}
							title={option.label}
							aria-checked={block.data.layout === option.value}
							role="radio"
						>
							<option.icon size={16} />
						</button>
					{/each}
				</div>

				<!-- Column Count (only for columns layout) -->
				{#if block.data.layout === 'columns'}
					<div class="flex items-center gap-0.5 p-0.5 bg-gray-100 rounded-md ml-1" role="radiogroup" aria-label="Column count">
						{#each columnOptions as option}
							<button
								type="button"
								class="px-2 py-1 text-xs font-medium rounded transition-all duration-150"
								class:bg-white={block.data.columns === option.value}
								class:shadow-sm={block.data.columns === option.value}
								class:text-blue-600={block.data.columns === option.value}
								class:text-gray-500={block.data.columns !== option.value}
								onclick={() => handleColumnsChange(option.value)}
								aria-checked={block.data.columns === option.value}
								role="radio"
							>
								{option.label}
							</button>
						{/each}
					</div>
				{/if}

				<!-- Gap -->
				<div class="flex items-center gap-1 ml-2">
					<IconArrowsVertical size={14} class="text-gray-400" aria-hidden="true" />
					<div class="flex items-center gap-0.5 p-0.5 bg-gray-100 rounded-md" role="radiogroup" aria-label="Gap size">
						{#each gapOptions as option}
							<button
								type="button"
								class="px-1.5 py-0.5 text-xs font-medium rounded transition-all duration-150"
								class:bg-white={block.data.gap === option.value}
								class:shadow-sm={block.data.gap === option.value}
								class:text-blue-600={block.data.gap === option.value}
								class:text-gray-500={block.data.gap !== option.value}
								onclick={() => handleGapChange(option.value)}
								aria-checked={block.data.gap === option.value}
								role="radio"
							>
								{option.label}
							</button>
						{/each}
					</div>
				</div>
			</div>

			<!-- Right: Style toggle -->
			<div class="flex items-center gap-1">
				<button
					type="button"
					class="p-1.5 rounded-md transition-colors"
					class:bg-blue-100={showStylePanel}
					class:text-blue-600={showStylePanel}
					class:text-gray-500={!showStylePanel}
					class:hover:bg-gray-100={!showStylePanel}
					onclick={() => (showStylePanel = !showStylePanel)}
					title="Style options"
					aria-expanded={showStylePanel}
					aria-controls="style-panel-{block.id}"
				>
					<IconSettings size={16} />
				</button>
			</div>
		</div>
	{/if}

	<!-- Style Panel -->
	{#if showStylePanel && !isPreview}
		<div
			id="style-panel-{block.id}"
			class="absolute -top-48 right-0 z-30 w-72 bg-white border border-gray-200 rounded-lg shadow-xl"
			transition:scale={{ duration: 200, start: 0.95 }}
			role="dialog"
			aria-label="Group style options"
		>
			<div class="p-3 border-b border-gray-100">
				<div class="flex items-center justify-between">
					<h3 class="text-sm font-semibold text-gray-900">Group Styling</h3>
					<button
						type="button"
						class="p-1 text-gray-400 hover:text-gray-600 rounded"
						onclick={() => (showStylePanel = false)}
						aria-label="Close style panel"
					>
						<IconChevronDown size={16} />
					</button>
				</div>

				<!-- Tab Navigation -->
				<div class="flex gap-1 mt-2 p-0.5 bg-gray-100 rounded-md" role="tablist">
					<button
						type="button"
						class="flex-1 px-3 py-1.5 text-xs font-medium rounded transition-all"
						class:bg-white={activeTab === 'layout'}
						class:shadow-sm={activeTab === 'layout'}
						class:text-gray-900={activeTab === 'layout'}
						class:text-gray-500={activeTab !== 'layout'}
						onclick={() => (activeTab = 'layout')}
						role="tab"
						aria-selected={activeTab === 'layout'}
						aria-controls="tab-layout-{block.id}"
					>
						Layout
					</button>
					<button
						type="button"
						class="flex-1 px-3 py-1.5 text-xs font-medium rounded transition-all"
						class:bg-white={activeTab === 'style'}
						class:shadow-sm={activeTab === 'style'}
						class:text-gray-900={activeTab === 'style'}
						class:text-gray-500={activeTab !== 'style'}
						onclick={() => (activeTab = 'style')}
						role="tab"
						aria-selected={activeTab === 'style'}
						aria-controls="tab-style-{block.id}"
					>
						Style
					</button>
				</div>
			</div>

			<div class="p-3 max-h-64 overflow-y-auto">
				{#if activeTab === 'layout'}
					<div id="tab-layout-{block.id}" role="tabpanel">
						<!-- Padding -->
						<div class="mb-4">
							<label class="flex items-center gap-2 text-xs font-medium text-gray-700 mb-2">
								<IconBoxPadding size={14} />
								Padding
							</label>
							<div class="flex gap-1" role="radiogroup" aria-label="Padding size">
								{#each paddingOptions as option}
									<button
										type="button"
										class="flex-1 px-2 py-1.5 text-xs font-medium border rounded transition-all"
										class:border-blue-500={block.data.padding === option.value}
										class:bg-blue-50={block.data.padding === option.value}
										class:text-blue-600={block.data.padding === option.value}
										class:border-gray-200={block.data.padding !== option.value}
										class:text-gray-600={block.data.padding !== option.value}
										onclick={() => handlePaddingChange(option.value)}
										role="radio"
										aria-checked={block.data.padding === option.value}
									>
										{option.label}
									</button>
								{/each}
							</div>
						</div>

						<!-- Border Radius -->
						<div>
							<label class="flex items-center gap-2 text-xs font-medium text-gray-700 mb-2">
								<IconBorderRadius size={14} />
								Border Radius
							</label>
							<div class="flex gap-1" role="radiogroup" aria-label="Border radius">
								{#each borderRadiusOptions as option}
									<button
										type="button"
										class="flex-1 px-2 py-1.5 text-xs font-medium border rounded transition-all"
										class:border-blue-500={block.data.borderRadius === option.value}
										class:bg-blue-50={block.data.borderRadius === option.value}
										class:text-blue-600={block.data.borderRadius === option.value}
										class:border-gray-200={block.data.borderRadius !== option.value}
										class:text-gray-600={block.data.borderRadius !== option.value}
										onclick={() => handleBorderRadiusChange(option.value)}
										role="radio"
										aria-checked={block.data.borderRadius === option.value}
									>
										{option.label}
									</button>
								{/each}
							</div>
						</div>
					</div>
				{:else}
					<div id="tab-style-{block.id}" role="tabpanel">
						<!-- Background Color -->
						<div class="mb-4">
							<label class="flex items-center gap-2 text-xs font-medium text-gray-700 mb-2">
								<IconPalette size={14} />
								Background Color
							</label>
							<div class="flex items-center gap-2">
								<div class="relative">
									<input
										type="color"
										class="w-10 h-10 rounded border border-gray-300 cursor-pointer"
										value={block.data.backgroundColor || '#ffffff'}
										oninput={handleBackgroundColorChange}
										aria-label="Background color picker"
									/>
								</div>
								<input
									type="text"
									class="flex-1 px-2 py-1.5 text-xs border border-gray-300 rounded"
									value={block.data.backgroundColor || ''}
									placeholder="#ffffff"
									oninput={handleBackgroundColorChange}
									aria-label="Background color hex value"
								/>
								{#if block.data.backgroundColor}
									<button
										type="button"
										class="p-1.5 text-gray-400 hover:text-red-500 transition-colors"
										onclick={clearBackgroundColor}
										title="Clear background color"
										aria-label="Clear background color"
									>
										<IconTrash size={14} />
									</button>
								{/if}
							</div>
						</div>

						<!-- Background Image -->
						<div>
							<label class="flex items-center gap-2 text-xs font-medium text-gray-700 mb-2">
								<IconPhoto size={14} />
								Background Image
							</label>
							<div class="flex items-center gap-2">
								<input
									type="text"
									class="flex-1 px-2 py-1.5 text-xs border border-gray-300 rounded"
									value={block.data.backgroundImage || ''}
									placeholder="https://example.com/image.jpg"
									oninput={handleBackgroundImageChange}
									aria-label="Background image URL"
								/>
								{#if block.data.backgroundImage}
									<button
										type="button"
										class="p-1.5 text-gray-400 hover:text-red-500 transition-colors"
										onclick={clearBackgroundImage}
										title="Clear background image"
										aria-label="Clear background image"
									>
										<IconTrash size={14} />
									</button>
								{/if}
							</div>
							{#if block.data.backgroundImage}
								<div class="mt-2 h-16 rounded overflow-hidden border border-gray-200">
									<img
										src={block.data.backgroundImage}
										alt="Background preview"
										class="w-full h-full object-cover"
									/>
								</div>
							{/if}
						</div>
					</div>
				{/if}
			</div>
		</div>
	{/if}

	<!-- Group Content Container -->
	<div
		class="group-content relative {paddingClasses} {borderRadiusClasses} transition-all duration-200"
		class:min-h-[100px]={!block.children?.length}
		class:border-2={!isPreview}
		class:border-dashed={!isPreview && !block.children?.length}
		class:{nestingColors}={!isPreview}
		class:{nestingBgColors}={!isPreview && !block.data.backgroundColor && !block.data.backgroundImage}
		style={containerStyles}
	>
		<!-- Background Overlay (for images) -->
		{#if block.data.backgroundImage && block.data.backgroundOverlay}
			<div
				class="absolute inset-0 {borderRadiusClasses}"
				style="background-color: {block.data.backgroundOverlay}"
				aria-hidden="true"
			></div>
		{/if}

		<!-- Children Container -->
		<div
			class="relative {layoutClasses}"
			role="list"
			aria-label="Group children"
		>
			{#if block.children && block.children.length > 0}
				{#each block.children as child, index (child.id)}
					<div
						class="group-child relative"
						class:opacity-50={draggedChildId === child.id}
						role="listitem"
						animate:flip={{ duration: 300, easing: quintOut }}
					>
						<!-- Drop Zone Indicator (before) -->
						{#if isDragging && dropTargetIndex === index && draggedChildId !== child.id}
							<div
								class="absolute -top-1 left-0 right-0 h-1 bg-blue-500 rounded-full z-10"
								transition:scale={{ duration: 150 }}
								aria-hidden="true"
							></div>
						{/if}

						<!-- Child Block -->
						<div
							class="relative"
							draggable={!isPreview}
							ondragstart={(e) => handleDragStart(e, child.id)}
							ondragover={(e) => handleDragOver(e, index)}
							ondragleave={handleDragLeave}
							ondrop={(e) => handleDrop(e, index)}
							ondragend={handleDragEnd}
							onkeydown={(e) => handleKeyDown(e, child.id, index)}
							tabindex={isPreview ? -1 : 0}
							role="button"
							aria-label="Drag to reorder {child.blockType} block"
							aria-describedby="child-{child.id}-instructions"
						>
							<!-- Drag Handle (visible on hover in edit mode) -->
							{#if !isPreview}
								<div
									class="absolute -left-6 top-1/2 -translate-y-1/2 opacity-0 group-child:hover:opacity-100 transition-opacity cursor-grab active:cursor-grabbing z-10"
									aria-hidden="true"
								>
									<div class="p-1 bg-white rounded shadow-sm border border-gray-200">
										<IconGripVertical size={14} class="text-gray-400" />
									</div>
								</div>

								<!-- Child Actions -->
								<div
									class="absolute -right-2 -top-2 opacity-0 group-child:hover:opacity-100 transition-opacity z-10"
								>
									<button
										type="button"
										class="p-1 bg-red-500 text-white rounded-full shadow-sm hover:bg-red-600 transition-colors"
										onclick={() => onChildRemove?.(child.id)}
										title="Remove from group"
										aria-label="Remove {child.blockType} from group"
									>
										<IconTrash size={12} />
									</button>
								</div>
							{/if}

							<!-- Screen reader instructions -->
							<span id="child-{child.id}-instructions" class="sr-only">
								Use arrow keys to reorder. Press Delete to remove.
							</span>

							<!-- Render Child Block -->
							{#if renderChild}
								{@render renderChild({
									block: child,
									isNested: true,
									nestingLevel: nestingLevel + 1,
									onUpdate: (updated: Block) => onChildUpdate?.(child.id, updated)
								})}
							{:else}
								<!-- Default child rendering -->
								<div
									class="p-4 bg-white rounded-lg border border-gray-200 shadow-sm"
									role="article"
									aria-label="{child.blockType} block"
								>
									<div class="text-sm font-medium text-gray-700 mb-1">
										{child.blockType}
									</div>
									<div class="text-xs text-gray-500">
										Block ID: {child.id}
									</div>
								</div>
							{/if}
						</div>

						<!-- Drop Zone Indicator (after last item) -->
						{#if isDragging && index === (block.children?.length ?? 0) - 1 && dropTargetIndex === index + 1}
							<div
								class="absolute -bottom-1 left-0 right-0 h-1 bg-blue-500 rounded-full z-10"
								transition:scale={{ duration: 150 }}
								aria-hidden="true"
							></div>
						{/if}
					</div>
				{/each}
			{:else}
				<!-- Empty State -->
				<div
					class="flex flex-col items-center justify-center py-8 text-center"
					role="region"
					aria-label="Drop zone for blocks"
					ondragover={(e) => {
						e.preventDefault();
						dropTargetIndex = 0;
					}}
					ondrop={(e) => handleDrop(e, 0)}
				>
					<div
						class="w-12 h-12 mb-3 flex items-center justify-center rounded-full bg-gray-100"
						aria-hidden="true"
					>
						<IconStack2 size={24} class="text-gray-400" />
					</div>
					<p class="text-sm text-gray-500 mb-3">
						This group is empty
					</p>
					{#if !isPreview && onAddChild}
						<button
							type="button"
							class="flex items-center gap-1.5 px-3 py-1.5 text-sm font-medium text-blue-600 bg-blue-50 rounded-md hover:bg-blue-100 transition-colors"
							onclick={onAddChild}
						>
							<IconPlus size={16} />
							Add Block
						</button>
					{/if}
				</div>
			{/if}
		</div>

		<!-- Add Block Button (shown at bottom when group has children) -->
		{#if !isPreview && onAddChild && block.children && block.children.length > 0}
			<div class="mt-4 flex justify-center">
				<button
					type="button"
					class="flex items-center gap-1.5 px-3 py-1.5 text-xs font-medium text-gray-500 bg-white border border-dashed border-gray-300 rounded-md hover:border-blue-400 hover:text-blue-600 hover:bg-blue-50 transition-all"
					onclick={onAddChild}
				>
					<IconPlus size={14} />
					Add to Group
				</button>
			</div>
		{/if}
	</div>
</div>

<style>
	/* Ensure drag handle visibility on child hover */
	.group-child:hover .opacity-0 {
		opacity: 1;
	}

	/* Custom scrollbar for style panel */
	.max-h-64::-webkit-scrollbar {
		width: 4px;
	}

	.max-h-64::-webkit-scrollbar-track {
		background: #f1f1f1;
		border-radius: 2px;
	}

	.max-h-64::-webkit-scrollbar-thumb {
		background: #ccc;
		border-radius: 2px;
	}

	.max-h-64::-webkit-scrollbar-thumb:hover {
		background: #aaa;
	}

	/* Color input styling */
	input[type='color'] {
		appearance: none;
		-webkit-appearance: none;
		padding: 0;
		border: none;
	}

	input[type='color']::-webkit-color-swatch-wrapper {
		padding: 0;
	}

	input[type='color']::-webkit-color-swatch {
		border: none;
		border-radius: 4px;
	}

	/* Focus visible styles */
	button:focus-visible,
	[role='button']:focus-visible {
		outline: 2px solid #3b82f6;
		outline-offset: 2px;
	}

	/* Screen reader only */
	.sr-only {
		position: absolute;
		width: 1px;
		height: 1px;
		padding: 0;
		margin: -1px;
		overflow: hidden;
		clip: rect(0, 0, 0, 0);
		white-space: nowrap;
		border-width: 0;
	}
</style>

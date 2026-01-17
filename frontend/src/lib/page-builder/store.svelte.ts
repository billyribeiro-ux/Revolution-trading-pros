/**
 * Page Builder Store - Svelte 5 Runes
 * Apple Principal Engineer ICT 7 Grade - January 2026
 *
 * Reactive state management for the page builder using Svelte 5 runes.
 * Handles layout, drag state, selection, and persistence.
 */

import type { PageLayout, PageBlock, ComponentType, DragState, ComponentConfig } from './types';
import { createDefaultConfig } from './registry';

// ═══════════════════════════════════════════════════════════════════════════════
// UTILITY FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════════

function generateId(): string {
	return `block_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
}

// ═══════════════════════════════════════════════════════════════════════════════
// CREATE BUILDER STORE
// ═══════════════════════════════════════════════════════════════════════════════

export function createBuilderStore(initialLayout?: PageLayout) {
	// Layout state
	let layout = $state<PageLayout>(
		initialLayout ?? {
			title: 'Untitled Course',
			blocks: [],
			status: 'draft'
		}
	);

	// Selection state
	let selectedBlockId = $state<string | null>(null);

	// Drag state
	let dragState = $state<DragState>({
		isDragging: false,
		draggedType: null,
		draggedBlockId: null,
		dropTargetIndex: null
	});

	// UI state
	let isPreviewMode = $state(false);
	let isSaving = $state(false);
	let hasUnsavedChanges = $state(false);

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED STATE
	// ═══════════════════════════════════════════════════════════════════════════

	const selectedBlock = $derived(layout.blocks.find((b) => b.id === selectedBlockId) ?? null);

	const sortedBlocks = $derived([...layout.blocks].sort((a, b) => a.order - b.order));

	const blockCount = $derived(layout.blocks.length);

	// ═══════════════════════════════════════════════════════════════════════════
	// BLOCK OPERATIONS
	// ═══════════════════════════════════════════════════════════════════════════

	function addBlock(type: ComponentType, atIndex?: number): PageBlock {
		const insertIndex = atIndex ?? layout.blocks.length;

		const newBlock: PageBlock = {
			id: generateId(),
			type,
			config: createDefaultConfig(type) as ComponentConfig,
			order: insertIndex
		};

		// Shift existing blocks
		layout.blocks = layout.blocks.map((block) => ({
			...block,
			order: block.order >= insertIndex ? block.order + 1 : block.order
		}));

		layout.blocks = [...layout.blocks, newBlock];
		hasUnsavedChanges = true;
		selectedBlockId = newBlock.id;

		return newBlock;
	}

	function removeBlock(blockId: string): void {
		const block = layout.blocks.find((b) => b.id === blockId);
		if (!block) return;

		const removedOrder = block.order;

		// Remove and reorder
		layout.blocks = layout.blocks
			.filter((b) => b.id !== blockId)
			.map((b) => ({
				...b,
				order: b.order > removedOrder ? b.order - 1 : b.order
			}));

		if (selectedBlockId === blockId) {
			selectedBlockId = null;
		}
		hasUnsavedChanges = true;
	}

	function duplicateBlock(blockId: string): PageBlock | null {
		const block = layout.blocks.find((b) => b.id === blockId);
		if (!block) return null;

		const newBlock: PageBlock = {
			id: generateId(),
			type: block.type,
			config: JSON.parse(JSON.stringify(block.config)),
			order: block.order + 1
		};

		// Shift blocks after
		layout.blocks = layout.blocks.map((b) => ({
			...b,
			order: b.order > block.order ? b.order + 1 : b.order
		}));

		layout.blocks = [...layout.blocks, newBlock];
		hasUnsavedChanges = true;
		selectedBlockId = newBlock.id;

		return newBlock;
	}

	function moveBlock(blockId: string, newIndex: number): void {
		const block = layout.blocks.find((b) => b.id === blockId);
		if (!block) return;

		const oldIndex = block.order;
		if (oldIndex === newIndex) return;

		layout.blocks = layout.blocks.map((b) => {
			if (b.id === blockId) {
				return { ...b, order: newIndex };
			}
			if (oldIndex < newIndex) {
				// Moving down
				if (b.order > oldIndex && b.order <= newIndex) {
					return { ...b, order: b.order - 1 };
				}
			} else {
				// Moving up
				if (b.order >= newIndex && b.order < oldIndex) {
					return { ...b, order: b.order + 1 };
				}
			}
			return b;
		});

		hasUnsavedChanges = true;
	}

	function updateBlockConfig(blockId: string, config: Partial<ComponentConfig>): void {
		layout.blocks = layout.blocks.map((b) => {
			if (b.id === blockId) {
				return { ...b, config: { ...b.config, ...config } };
			}
			return b;
		});
		hasUnsavedChanges = true;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// SELECTION
	// ═══════════════════════════════════════════════════════════════════════════

	function selectBlock(blockId: string | null): void {
		selectedBlockId = blockId;
	}

	function clearSelection(): void {
		selectedBlockId = null;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// DRAG AND DROP
	// ═══════════════════════════════════════════════════════════════════════════

	function startDragNew(type: ComponentType): void {
		dragState = {
			isDragging: true,
			draggedType: type,
			draggedBlockId: null,
			dropTargetIndex: null
		};
	}

	function startDragExisting(blockId: string): void {
		dragState = {
			isDragging: true,
			draggedType: null,
			draggedBlockId: blockId,
			dropTargetIndex: null
		};
	}

	function updateDropTarget(index: number | null): void {
		dragState = { ...dragState, dropTargetIndex: index };
	}

	function endDrag(): void {
		const { draggedType, draggedBlockId, dropTargetIndex } = dragState;

		if (dropTargetIndex !== null) {
			if (draggedType) {
				// Adding new block
				addBlock(draggedType, dropTargetIndex);
			} else if (draggedBlockId) {
				// Moving existing block
				moveBlock(draggedBlockId, dropTargetIndex);
			}
		}

		dragState = {
			isDragging: false,
			draggedType: null,
			draggedBlockId: null,
			dropTargetIndex: null
		};
	}

	function cancelDrag(): void {
		dragState = {
			isDragging: false,
			draggedType: null,
			draggedBlockId: null,
			dropTargetIndex: null
		};
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// LAYOUT OPERATIONS
	// ═══════════════════════════════════════════════════════════════════════════

	function setLayout(newLayout: PageLayout): void {
		layout = newLayout;
		selectedBlockId = null;
		hasUnsavedChanges = false;
	}

	function updateLayoutMeta(
		meta: Partial<Pick<PageLayout, 'title' | 'courseId' | 'courseSlug' | 'status'>>
	): void {
		layout = { ...layout, ...meta };
		hasUnsavedChanges = true;
	}

	function clearLayout(): void {
		layout = {
			title: 'Untitled Course',
			blocks: [],
			status: 'draft'
		};
		selectedBlockId = null;
		hasUnsavedChanges = true;
	}

	function importLayout(data: {
		title?: string;
		blocks?: unknown;
		status?: string;
		course_id?: string;
	}): void {
		layout = {
			title: data.title || 'Untitled Course',
			blocks: Array.isArray(data.blocks) ? (data.blocks as PageBlock[]) : [],
			status: (data.status as 'draft' | 'published') || 'draft',
			courseId: data.course_id
		};
		selectedBlockId = null;
		hasUnsavedChanges = false;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// UI STATE
	// ═══════════════════════════════════════════════════════════════════════════

	function togglePreviewMode(): void {
		isPreviewMode = !isPreviewMode;
		if (isPreviewMode) {
			selectedBlockId = null;
		}
	}

	function setSaving(saving: boolean): void {
		isSaving = saving;
	}

	function markSaved(): void {
		hasUnsavedChanges = false;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// EXPORT
	// ═══════════════════════════════════════════════════════════════════════════

	function exportLayout(): PageLayout {
		return JSON.parse(JSON.stringify(layout));
	}

	return {
		// State (getters)
		get layout() {
			return layout;
		},
		get selectedBlockId() {
			return selectedBlockId;
		},
		get selectedBlock() {
			return selectedBlock;
		},
		get sortedBlocks() {
			return sortedBlocks;
		},
		get blockCount() {
			return blockCount;
		},
		get dragState() {
			return dragState;
		},
		get isPreviewMode() {
			return isPreviewMode;
		},
		get isSaving() {
			return isSaving;
		},
		get hasUnsavedChanges() {
			return hasUnsavedChanges;
		},

		// Block operations
		addBlock,
		removeBlock,
		duplicateBlock,
		moveBlock,
		updateBlockConfig,

		// Selection
		selectBlock,
		clearSelection,

		// Drag and drop
		startDragNew,
		startDragExisting,
		updateDropTarget,
		endDrag,
		cancelDrag,

		// Layout operations
		setLayout,
		updateLayoutMeta,
		clearLayout,
		importLayout,
		exportLayout,

		// UI state
		togglePreviewMode,
		setSaving,
		markSaved
	};
}

// Type for the store
export type BuilderStore = ReturnType<typeof createBuilderStore>;

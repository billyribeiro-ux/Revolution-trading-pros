<!--
/**
 * Revolution Trading Pros - Enterprise Block Editor
 * ═══════════════════════════════════════════════════════════════════════════
 * World-class blog editor surpassing WordPress Elementor Pro
 *
 * FEATURES:
 * - 30+ block types with drag-and-drop
 * - AI-powered writing assistant
 * - Real-time preview (desktop/tablet/mobile)
 * - Undo/Redo with unlimited history
 * - Auto-save with revision control
 * - SEO analyzer with real-time scoring
 * - Keyboard shortcuts
 * - Full accessibility (ARIA)
 * - Collaboration comments
 *
 * @version 4.0.0 Enterprise
 * @author Revolution Trading Pros
 */
-->

<script lang="ts">
	import { onMount, onDestroy, tick } from 'svelte';
	import { flip } from 'svelte/animate';
	import { fade } from 'svelte/transition';
	import { quintOut } from 'svelte/easing';
	import {
		IconPlus,
		IconGripVertical,
		IconTrash,
		IconCopy,
		IconSettings,
		IconChevronUp,
		IconChevronDown,
		IconArrowBackUp,
		IconArrowForwardUp,
		IconDeviceDesktop,
		IconDeviceTablet,
		IconDeviceMobile,
		IconEye,
		IconEdit,
		IconMaximize,
		IconMinimize,
		IconDeviceFloppy,
		IconCloudUpload,
		IconSearch,
		IconRobot,
		IconSeo,
		IconStack2,
		IconKeyboard,
		IconHistory
	} from '$lib/icons';

	import type { Block, BlockType, EditorState, SEOAnalysis, Revision, DropAction } from './types';

	import { BLOCK_DEFINITIONS } from './types';

	// ==========================================================================
	// Custom Spring Animation (Apple-grade physics)
	// ==========================================================================

	interface SpringConfig {
		stiffness: number;
		damping: number;
		mass: number;
	}

	const SPRING_PRESETS = {
		snappy: { stiffness: 400, damping: 30, mass: 1 },
		smooth: { stiffness: 200, damping: 25, mass: 1 },
		bouncy: { stiffness: 300, damping: 15, mass: 1 },
		gentle: { stiffness: 150, damping: 20, mass: 1 }
	} as const;

	// Import sub-components (we'll create these)
	import BlockInserter from './BlockInserter.svelte';
	import BlockRenderer from './BlockRenderer.svelte';
	import BlockSettingsPanel from './BlockSettingsPanel.svelte';
	import AIAssistant from './AIAssistant.svelte';
	import SEOAnalyzer from './SEOAnalyzer.svelte';
	import RevisionHistory from './RevisionHistory.svelte';
	import KeyboardShortcuts from './KeyboardShortcuts.svelte';

	// ==========================================================================
	// Props
	// ==========================================================================

	interface Props {
		blocks?: Block[];
		postTitle?: string;
		postSlug?: string;
		postExcerpt?: string;
		metaTitle?: string;
		metaDescription?: string;
		focusKeyword?: string;
		onchange?: (blocks: Block[]) => void;
		onsave?: (blocks: Block[]) => void;
		onpublish?: (blocks: Block[]) => void;
		autosaveInterval?: number;
		readOnly?: boolean;
	}

	let {
		blocks = $bindable([]),
		postTitle = '',
		postSlug = '',
		metaDescription = '',
		focusKeyword = '',
		onchange,
		onsave,
		onpublish,
		autosaveInterval = 30000,
		readOnly = false
	}: Props = $props();

	// ==========================================================================
	// State
	// ==========================================================================

	let editorState = $state<EditorState>({
		blocks: blocks,
		selectedBlockId: null,
		selectedBlockIds: [], // Multi-selection
		hoveredBlockId: null,
		focusedBlockId: null,
		clipboard: null,
		clipboardMulti: [], // Multi-block clipboard
		history: {
			past: [],
			present: blocks,
			future: [],
			maxSize: 100
		},
		isDragging: false,
		draggedBlockId: null,
		draggedBlockIds: [], // Multi-block drag
		dropTargetIndex: null,
		dropPosition: null, // Enhanced drop positioning
		viewMode: 'edit',
		devicePreview: 'desktop',
		zoom: 100,
		showGrid: false,
		showOutlines: false,
		sidebarTab: 'blocks',
		isFullscreen: false,
		autosaveEnabled: true,
		lastSaved: null,
		hasUnsavedChanges: false,
		// Enhanced drag-drop state
		dragPreviewOffset: null,
		lastDropAction: null
	});

	// ==========================================================================
	// Enhanced Drag-Drop State (Apple Principal Engineer Grade)
	// ==========================================================================

	// Touch drag state
	let touchDragState = $state<{
		isActive: boolean;
		startY: number;
		currentY: number;
		longPressTimer: ReturnType<typeof setTimeout> | null;
		touchStartTime: number;
		isDragging: boolean;
	}>({
		isActive: false,
		startY: 0,
		currentY: 0,
		longPressTimer: null,
		touchStartTime: 0,
		isDragging: false
	});

	// Auto-scroll state
	let autoScrollState = $state<{
		isScrolling: boolean;
		direction: 'up' | 'down' | null;
		speed: number;
		rafId: number | null;
	}>({
		isScrolling: false,
		direction: null,
		speed: 0,
		rafId: null
	});

	// Drag visual feedback state
	let dragVisuals = $state<{
		ghostElement: HTMLElement | null;
		dropIndicatorY: number;
		dropIndicatorVisible: boolean;
		springValue: number;
		targetSpringValue: number;
	}>({
		ghostElement: null,
		dropIndicatorY: 0,
		dropIndicatorVisible: false,
		springValue: 0,
		targetSpringValue: 0
	});

	// Screen reader announcements
	let announcements = $state<string[]>([]);
	let liveRegionRef: HTMLDivElement;

	// UI State
	let showBlockInserter = $state(false);
	let inserterPosition = $state<{ x: number; y: number } | null>(null);
	let inserterIndex = $state(0);
	let searchQuery = $state('');
	let showKeyboardHelp = $state(false);
	let showRevisions = $state(false);
	let isSaving = $state(false);
	let saveError = $state<string | null>(null);

	// SEO State
	let seoAnalysis = $state<SEOAnalysis | null>(null);

	// Revisions State
	let revisions = $state<Revision[]>([]);

	// Refs
	let editorContainer: HTMLDivElement;
	let autosaveTimer: ReturnType<typeof setInterval>;

	// ==========================================================================
	// Computed
	// ==========================================================================

	let canUndo = $derived(editorState.history.past.length > 0);
	let canRedo = $derived(editorState.history.future.length > 0);
	let selectedBlock = $derived(
		editorState.selectedBlockId
			? editorState.blocks.find((b) => b.id === editorState.selectedBlockId)
			: null
	);
	let wordCount = $derived(calculateWordCount(editorState.blocks));
	let readTime = $derived(Math.ceil(wordCount / 200));
	let blockCount = $derived(editorState.blocks.length);

	// ==========================================================================
	// Lifecycle
	// ==========================================================================

	onMount(() => {
		// Initialize history
		editorState.history.present = [...blocks];

		// Start autosave
		if (autosaveInterval > 0 && editorState.autosaveEnabled) {
			autosaveTimer = setInterval(handleAutosave, autosaveInterval);
		}

		// Add keyboard listeners
		window.addEventListener('keydown', handleGlobalKeydown);

		// Initial SEO analysis
		runSEOAnalysis();
	});

	onDestroy(() => {
		if (autosaveTimer) clearInterval(autosaveTimer);
		window.removeEventListener('keydown', handleGlobalKeydown);
		// Clean up touch drag timer
		if (touchDragState.longPressTimer) {
			clearTimeout(touchDragState.longPressTimer);
		}
		// Clean up auto-scroll
		stopAutoScroll();
	});

	// Watch for block changes
	$effect(() => {
		if (JSON.stringify(editorState.blocks) !== JSON.stringify(blocks)) {
			blocks = [...editorState.blocks];
			editorState.hasUnsavedChanges = true;
			onchange?.(blocks);
			runSEOAnalysis();
		}
	});

	// ==========================================================================
	// Block Operations
	// ==========================================================================

	function generateBlockId(): string {
		return `block_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
	}

	function createBlock(type: BlockType): Block {
		const definition = BLOCK_DEFINITIONS[type];
		return {
			id: generateBlockId(),
			type,
			content: { ...definition.defaultContent },
			settings: { ...definition.defaultSettings },
			metadata: {
				createdAt: new Date().toISOString(),
				updatedAt: new Date().toISOString(),
				version: 1
			}
		};
	}

	function addBlock(type: BlockType, index?: number) {
		const newBlock = createBlock(type);
		const insertIndex = index ?? editorState.blocks.length;

		pushToHistory();

		const newBlocks = [...editorState.blocks];
		newBlocks.splice(insertIndex, 0, newBlock);
		editorState.blocks = newBlocks;
		editorState.selectedBlockId = newBlock.id;

		showBlockInserter = false;

		// Focus the new block
		tick().then(() => {
			const blockEl = document.querySelector(`[data-block-id="${newBlock.id}"]`);
			if (blockEl) {
				(blockEl as HTMLElement).focus();
			}
		});
	}

	function updateBlock(blockId: string, updates: Partial<Block>) {
		pushToHistory();

		editorState.blocks = editorState.blocks.map((block) =>
			block.id === blockId
				? {
						...block,
						...updates,
						metadata: {
							...block.metadata,
							updatedAt: new Date().toISOString(),
							version: block.metadata.version + 1
						}
					}
				: block
		);
	}

	function deleteBlock(blockId: string) {
		pushToHistory();

		const index = editorState.blocks.findIndex((b) => b.id === blockId);
		editorState.blocks = editorState.blocks.filter((b) => b.id !== blockId);

		// Select adjacent block
		if (editorState.blocks.length > 0) {
			const newIndex = Math.min(index, editorState.blocks.length - 1);
			editorState.selectedBlockId = editorState.blocks[newIndex]?.id ?? null;
		} else {
			editorState.selectedBlockId = null;
		}
	}

	function duplicateBlock(blockId: string) {
		const block = editorState.blocks.find((b) => b.id === blockId);
		if (!block) return;

		pushToHistory();

		const duplicated: Block = {
			...JSON.parse(JSON.stringify(block)),
			id: generateBlockId(),
			metadata: {
				...block.metadata,
				createdAt: new Date().toISOString(),
				updatedAt: new Date().toISOString(),
				version: 1
			}
		};

		const index = editorState.blocks.findIndex((b) => b.id === blockId);
		const newBlocks = [...editorState.blocks];
		newBlocks.splice(index + 1, 0, duplicated);
		editorState.blocks = newBlocks;
		editorState.selectedBlockId = duplicated.id;
	}

	function moveBlock(blockId: string, direction: 'up' | 'down') {
		const index = editorState.blocks.findIndex((b) => b.id === blockId);
		if (index === -1) return;

		const newIndex = direction === 'up' ? index - 1 : index + 1;
		if (newIndex < 0 || newIndex >= editorState.blocks.length) return;

		pushToHistory();

		const newBlocks = [...editorState.blocks];
		const blockA = newBlocks[index];
		const blockB = newBlocks[newIndex];
		if (blockA && blockB) {
			[newBlocks[index], newBlocks[newIndex]] = [blockB, blockA];
			editorState.blocks = newBlocks;
		}
	}

	// ==========================================================================
	// Enhanced Drag and Drop (Apple Principal Engineer Grade)
	// ==========================================================================

	// Screen reader announcement helper
	function announce(message: string, priority: 'polite' | 'assertive' = 'polite') {
		announcements = [...announcements, message];
		// Clear old announcements after they've been read
		setTimeout(() => {
			announcements = announcements.slice(1);
		}, 1000);
	}

	// Spring animation for smooth drop animations
	function createSpringAnimation(
		config: SpringConfig = SPRING_PRESETS.snappy
	): (target: number, current: number) => number {
		let velocity = 0;
		return (target: number, current: number) => {
			const force = (target - current) * config.stiffness;
			const damping = velocity * config.damping;
			const acceleration = (force - damping) / config.mass;
			velocity += acceleration * (1 / 60); // Assuming 60fps
			return current + velocity * (1 / 60);
		};
	}

	const springAnimate = createSpringAnimation(SPRING_PRESETS.snappy);

	// Multi-selection helpers
	function isBlockSelected(blockId: string): boolean {
		return (
			editorState.selectedBlockIds.includes(blockId) || editorState.selectedBlockId === blockId
		);
	}

	function toggleBlockSelection(blockId: string, addToSelection: boolean) {
		if (addToSelection) {
			// Shift/Cmd+click: add to selection
			if (editorState.selectedBlockIds.includes(blockId)) {
				editorState.selectedBlockIds = editorState.selectedBlockIds.filter((id) => id !== blockId);
			} else {
				editorState.selectedBlockIds = [...editorState.selectedBlockIds, blockId];
			}
			editorState.selectedBlockId = blockId;
		} else {
			// Normal click: single selection
			editorState.selectedBlockIds = [blockId];
			editorState.selectedBlockId = blockId;
		}
	}

	function selectBlockRange(fromId: string, toId: string) {
		const fromIndex = editorState.blocks.findIndex((b) => b.id === fromId);
		const toIndex = editorState.blocks.findIndex((b) => b.id === toId);
		if (fromIndex === -1 || toIndex === -1) return;

		const start = Math.min(fromIndex, toIndex);
		const end = Math.max(fromIndex, toIndex);
		editorState.selectedBlockIds = editorState.blocks.slice(start, end + 1).map((b) => b.id);
		editorState.selectedBlockId = toId;
	}

	// Enhanced drag start with multi-block support
	function handleDragStart(e: DragEvent, blockId: string) {
		if (readOnly) return;

		// Determine which blocks to drag
		const blocksToDrag = isBlockSelected(blockId)
			? editorState.selectedBlockIds.length > 0
				? editorState.selectedBlockIds
				: [blockId]
			: [blockId];

		editorState.isDragging = true;
		editorState.draggedBlockId = blockId;
		editorState.draggedBlockIds = blocksToDrag;

		if (e.dataTransfer) {
			e.dataTransfer.effectAllowed = 'move';
			e.dataTransfer.setData('text/plain', JSON.stringify(blocksToDrag));

			// Create custom drag preview
			const preview = createDragPreview(blocksToDrag);
			if (preview) {
				e.dataTransfer.setDragImage(preview, 20, 20);
				// Clean up preview after drag
				requestAnimationFrame(() => {
					if (preview.parentNode) {
						preview.parentNode.removeChild(preview);
					}
				});
			}
		}

		// Store offset for positioning
		const target = e.target as HTMLElement;
		const rect = target.getBoundingClientRect();
		editorState.dragPreviewOffset = {
			x: e.clientX - rect.left,
			y: e.clientY - rect.top
		};

		// Announce for screen readers
		const blockNames = blocksToDrag.map((id) => {
			const block = editorState.blocks.find((b) => b.id === id);
			return block ? BLOCK_DEFINITIONS[block.type]?.name || block.type : 'Block';
		});
		announce(
			`Started dragging ${blocksToDrag.length > 1 ? `${blocksToDrag.length} blocks` : blockNames[0]}. Use arrow keys to move, Enter to drop, Escape to cancel.`,
			'assertive'
		);
	}

	// Create custom drag preview element
	function createDragPreview(blockIds: string[]): HTMLElement | null {
		const preview = document.createElement('div');
		preview.className = 'drag-preview';
		preview.style.cssText = `
			position: fixed;
			top: -1000px;
			left: -1000px;
			padding: 12px 16px;
			background: linear-gradient(135deg, #3b82f6 0%, #1d4ed8 100%);
			color: white;
			border-radius: 8px;
			font-size: 14px;
			font-weight: 500;
			box-shadow: 0 10px 40px rgba(59, 130, 246, 0.4), 0 4px 12px rgba(0, 0, 0, 0.15);
			display: flex;
			align-items: center;
			gap: 8px;
			pointer-events: none;
			z-index: 10000;
		`;

		if (blockIds.length > 1) {
			preview.innerHTML = `
				<span style="display: flex; align-items: center; justify-content: center; width: 24px; height: 24px; background: rgba(255,255,255,0.2); border-radius: 50%; font-size: 12px;">${blockIds.length}</span>
				<span>blocks selected</span>
			`;
		} else {
			const block = editorState.blocks.find((b) => b.id === blockIds[0]);
			const blockName = block ? BLOCK_DEFINITIONS[block.type]?.name || block.type : 'Block';
			preview.textContent = blockName;
		}

		document.body.appendChild(preview);
		return preview;
	}

	// Enhanced drag over with position detection
	function handleDragOver(e: DragEvent, index: number) {
		e.preventDefault();
		if (!editorState.isDragging) return;

		const target = e.currentTarget as HTMLElement;
		const rect = target.getBoundingClientRect();
		const midpoint = rect.top + rect.height / 2;

		// Determine if drop should be before or after this block
		const position: 'before' | 'after' = e.clientY < midpoint ? 'before' : 'after';

		editorState.dropTargetIndex = index;
		editorState.dropPosition = position;

		// Update drop indicator position with spring animation
		const indicatorY = position === 'before' ? rect.top : rect.bottom;
		dragVisuals.targetSpringValue = indicatorY;
		dragVisuals.dropIndicatorVisible = true;
		dragVisuals.dropIndicatorY = indicatorY;

		// Handle auto-scroll
		handleAutoScroll(e.clientY);
	}

	// Auto-scroll when dragging near edges
	function handleAutoScroll(clientY: number) {
		const canvas = document.querySelector('.editor-canvas') as HTMLElement;
		if (!canvas) return;

		const rect = canvas.getBoundingClientRect();
		const threshold = 80;
		const maxSpeed = 15;

		if (clientY < rect.top + threshold) {
			// Scroll up
			const distance = rect.top + threshold - clientY;
			const speed = Math.min(maxSpeed, (distance / threshold) * maxSpeed);
			startAutoScroll('up', speed);
		} else if (clientY > rect.bottom - threshold) {
			// Scroll down
			const distance = clientY - (rect.bottom - threshold);
			const speed = Math.min(maxSpeed, (distance / threshold) * maxSpeed);
			startAutoScroll('down', speed);
		} else {
			stopAutoScroll();
		}
	}

	function startAutoScroll(direction: 'up' | 'down', speed: number) {
		if (autoScrollState.isScrolling && autoScrollState.direction === direction) {
			autoScrollState.speed = speed;
			return;
		}

		stopAutoScroll();
		autoScrollState.isScrolling = true;
		autoScrollState.direction = direction;
		autoScrollState.speed = speed;

		const scroll = () => {
			const canvas = document.querySelector('.editor-canvas') as HTMLElement;
			if (!canvas || !autoScrollState.isScrolling) return;

			canvas.scrollTop +=
				autoScrollState.direction === 'up' ? -autoScrollState.speed : autoScrollState.speed;
			autoScrollState.rafId = requestAnimationFrame(scroll);
		};

		autoScrollState.rafId = requestAnimationFrame(scroll);
	}

	function stopAutoScroll() {
		if (autoScrollState.rafId) {
			cancelAnimationFrame(autoScrollState.rafId);
		}
		autoScrollState.isScrolling = false;
		autoScrollState.direction = null;
		autoScrollState.speed = 0;
		autoScrollState.rafId = null;
	}

	// Enhanced drag end with spring animation
	function handleDragEnd() {
		stopAutoScroll();
		dragVisuals.dropIndicatorVisible = false;

		if (editorState.draggedBlockIds.length > 0 && editorState.dropTargetIndex !== null) {
			const blockIds = editorState.draggedBlockIds;
			const toIndex = editorState.dropTargetIndex + (editorState.dropPosition === 'after' ? 1 : 0);

			// Get current indices
			const fromIndices = blockIds
				.map((id) => editorState.blocks.findIndex((b) => b.id === id))
				.filter((i) => i !== -1)
				.sort((a, b) => a - b);

			if (fromIndices.length > 0) {
				// Check if this is a meaningful move
				const firstFromIndex = fromIndices[0];
				const adjustedToIndex = fromIndices.filter((i) => i < toIndex).length;
				const effectiveToIndex = toIndex - adjustedToIndex;

				if (firstFromIndex !== effectiveToIndex) {
					pushToHistory();

					// Store drop action for potential undo
					editorState.lastDropAction = {
						type: blockIds.length > 1 ? 'multi-reorder' : 'reorder',
						fromIndices,
						toIndex: effectiveToIndex,
						blockIds,
						timestamp: Date.now()
					};

					// Extract blocks to move
					const blocksToMove = fromIndices
						.map((i) => editorState.blocks[i])
						.filter((b): b is Block => b !== undefined);

					// Remove from current positions (in reverse to preserve indices)
					let newBlocks = [...editorState.blocks];
					for (let i = fromIndices.length - 1; i >= 0; i--) {
						const idx = fromIndices[i];
						if (idx !== undefined) {
							newBlocks.splice(idx, 1);
						}
					}

					// Calculate adjusted insert position
					let insertAt = effectiveToIndex;
					if (insertAt > newBlocks.length) {
						insertAt = newBlocks.length;
					}

					// Insert at new position
					newBlocks.splice(insertAt, 0, ...blocksToMove);
					editorState.blocks = newBlocks;

					// Announce completion
					const blockName =
						blockIds.length > 1
							? `${blockIds.length} blocks`
							: BLOCK_DEFINITIONS[blocksToMove[0]?.type || 'paragraph']?.name || 'Block';
					announce(`${blockName} moved to position ${effectiveToIndex + 1}`, 'assertive');

					// Trigger haptic-style visual feedback
					triggerDropFeedback();
				}
			}
		}

		// Reset all drag state
		editorState.isDragging = false;
		editorState.draggedBlockId = null;
		editorState.draggedBlockIds = [];
		editorState.dropTargetIndex = null;
		editorState.dropPosition = null;
		editorState.dragPreviewOffset = null;
	}

	// Haptic-style visual feedback for drop
	function triggerDropFeedback() {
		const canvas = document.querySelector('.canvas-wrapper');
		if (canvas) {
			canvas.classList.add('drop-feedback');
			setTimeout(() => canvas.classList.remove('drop-feedback'), 300);
		}
	}

	function handleDrop(e: DragEvent, index: number) {
		e.preventDefault();
		editorState.dropTargetIndex = index;
		handleDragEnd();
	}

	// Touch support for mobile
	function handleTouchStart(e: TouchEvent, blockId: string) {
		if (readOnly) return;

		const touch = e.touches[0];
		if (!touch) return;

		touchDragState.isActive = true;
		touchDragState.startY = touch.clientY;
		touchDragState.currentY = touch.clientY;
		touchDragState.touchStartTime = Date.now();
		touchDragState.isDragging = false;

		// Long-press to initiate drag (300ms)
		touchDragState.longPressTimer = setTimeout(() => {
			if (touchDragState.isActive) {
				touchDragState.isDragging = true;

				// Determine blocks to drag
				const blocksToDrag = isBlockSelected(blockId)
					? editorState.selectedBlockIds.length > 0
						? editorState.selectedBlockIds
						: [blockId]
					: [blockId];

				editorState.isDragging = true;
				editorState.draggedBlockId = blockId;
				editorState.draggedBlockIds = blocksToDrag;

				// Haptic feedback on supported devices
				if ('vibrate' in navigator) {
					navigator.vibrate(50);
				}

				announce(
					`Drag mode activated for ${blocksToDrag.length > 1 ? `${blocksToDrag.length} blocks` : 'block'}`,
					'assertive'
				);
			}
		}, 300);
	}

	function handleTouchMove(e: TouchEvent) {
		if (!touchDragState.isActive) return;

		const touch = e.touches[0];
		if (!touch) return;

		touchDragState.currentY = touch.clientY;

		// If movement detected before long-press, cancel drag initiation
		if (!touchDragState.isDragging) {
			const moveDistance = Math.abs(touch.clientY - touchDragState.startY);
			if (moveDistance > 10) {
				cancelTouchDrag();
				return;
			}
		}

		if (touchDragState.isDragging) {
			e.preventDefault();

			// Find drop target
			const elements = document.elementsFromPoint(touch.clientX, touch.clientY);
			const blockWrapper = elements.find((el) =>
				el.classList.contains('block-wrapper')
			) as HTMLElement;

			if (blockWrapper) {
				const index = parseInt(blockWrapper.dataset.blockIndex || '0', 10);
				const rect = blockWrapper.getBoundingClientRect();
				const midpoint = rect.top + rect.height / 2;
				const position: 'before' | 'after' = touch.clientY < midpoint ? 'before' : 'after';

				editorState.dropTargetIndex = index;
				editorState.dropPosition = position;
				dragVisuals.dropIndicatorVisible = true;
				dragVisuals.dropIndicatorY = position === 'before' ? rect.top : rect.bottom;
			}

			// Handle auto-scroll
			handleAutoScroll(touch.clientY);
		}
	}

	function handleTouchEnd() {
		if (touchDragState.isDragging) {
			handleDragEnd();
		}
		cancelTouchDrag();
	}

	function cancelTouchDrag() {
		if (touchDragState.longPressTimer) {
			clearTimeout(touchDragState.longPressTimer);
		}
		touchDragState.isActive = false;
		touchDragState.isDragging = false;
		touchDragState.longPressTimer = null;
		stopAutoScroll();
	}

	// Keyboard-based reordering
	function handleKeyboardReorder(e: KeyboardEvent, blockId: string) {
		if (!e.altKey && !e.metaKey) return;

		const index = editorState.blocks.findIndex((b) => b.id === blockId);
		if (index === -1) return;

		let newIndex = index;
		let direction = '';

		if (e.key === 'ArrowUp' && index > 0) {
			newIndex = index - 1;
			direction = 'up';
		} else if (e.key === 'ArrowDown' && index < editorState.blocks.length - 1) {
			newIndex = index + 1;
			direction = 'down';
		} else {
			return;
		}

		e.preventDefault();
		pushToHistory();

		const newBlocks = [...editorState.blocks];
		const blockA = newBlocks[index];
		const blockB = newBlocks[newIndex];
		if (blockA && blockB) {
			[newBlocks[index], newBlocks[newIndex]] = [blockB, blockA];
			editorState.blocks = newBlocks;

			// Announce for screen readers
			const blockName = BLOCK_DEFINITIONS[blockA.type]?.name || blockA.type;
			announce(`${blockName} moved ${direction} to position ${newIndex + 1}`, 'polite');

			// Focus the moved block
			tick().then(() => {
				const movedBlock = document.querySelector(`[data-block-id="${blockId}"]`) as HTMLElement;
				movedBlock?.focus();
			});
		}
	}

	// Undo last drop action (Ctrl/Cmd + Shift + Z specifically for drops)
	function undoLastDrop() {
		if (!editorState.lastDropAction) return;

		const action = editorState.lastDropAction;
		// Only allow undo within 10 seconds
		if (Date.now() - action.timestamp > 10000) {
			editorState.lastDropAction = null;
			return;
		}

		undo();
		editorState.lastDropAction = null;
		announce('Drop action undone', 'polite');
	}

	// ==========================================================================
	// History (Undo/Redo)
	// ==========================================================================

	function pushToHistory() {
		const currentState = JSON.parse(JSON.stringify(editorState.blocks));

		editorState.history = {
			...editorState.history,
			past: [...editorState.history.past, editorState.history.present].slice(
				-editorState.history.maxSize
			),
			present: currentState,
			future: []
		};
	}

	function undo() {
		if (!canUndo) return;

		const previous = editorState.history.past[editorState.history.past.length - 1];
		const newPast = editorState.history.past.slice(0, -1);

		if (previous) {
			editorState.history = {
				...editorState.history,
				past: newPast,
				present: previous,
				future: [editorState.history.present, ...editorState.history.future]
			};

			editorState.blocks = [...previous];
		}
	}

	function redo() {
		if (!canRedo) return;

		const next = editorState.history.future[0];
		const newFuture = editorState.history.future.slice(1);

		if (next) {
			editorState.history = {
				...editorState.history,
				past: [...editorState.history.past, editorState.history.present],
				present: next,
				future: newFuture
			};

			editorState.blocks = [...next];
		}
	}

	// ==========================================================================
	// Clipboard
	// ==========================================================================

	function copyBlock(blockId: string) {
		const block = editorState.blocks.find((b) => b.id === blockId);
		if (block) {
			editorState.clipboard = JSON.parse(JSON.stringify(block));
		}
	}

	function cutBlock(blockId: string) {
		copyBlock(blockId);
		deleteBlock(blockId);
	}

	function pasteBlock() {
		if (!editorState.clipboard) return;

		const index = editorState.selectedBlockId
			? editorState.blocks.findIndex((b) => b.id === editorState.selectedBlockId) + 1
			: editorState.blocks.length;

		pushToHistory();

		const pastedBlock: Block = {
			...JSON.parse(JSON.stringify(editorState.clipboard)),
			id: generateBlockId(),
			metadata: {
				...editorState.clipboard.metadata,
				createdAt: new Date().toISOString(),
				updatedAt: new Date().toISOString(),
				version: 1
			}
		};

		const newBlocks = [...editorState.blocks];
		newBlocks.splice(index, 0, pastedBlock);
		editorState.blocks = newBlocks;
		editorState.selectedBlockId = pastedBlock.id;
	}

	// ==========================================================================
	// Keyboard Shortcuts
	// ==========================================================================

	function handleGlobalKeydown(e: KeyboardEvent) {
		const isMeta = e.metaKey || e.ctrlKey;

		// Undo: Ctrl/Cmd + Z
		if (isMeta && e.key === 'z' && !e.shiftKey) {
			e.preventDefault();
			undo();
		}

		// Redo: Ctrl/Cmd + Shift + Z or Ctrl/Cmd + Y
		if ((isMeta && e.shiftKey && e.key === 'z') || (isMeta && e.key === 'y')) {
			e.preventDefault();
			redo();
		}

		// Save: Ctrl/Cmd + S
		if (isMeta && e.key === 's') {
			e.preventDefault();
			handleSave();
		}

		// Copy: Ctrl/Cmd + C (when block selected)
		if (isMeta && e.key === 'c' && editorState.selectedBlockId) {
			// Let default copy work for text selection
			if (!window.getSelection()?.toString()) {
				e.preventDefault();
				copyBlock(editorState.selectedBlockId);
			}
		}

		// Cut: Ctrl/Cmd + X (when block selected)
		if (isMeta && e.key === 'x' && editorState.selectedBlockId) {
			if (!window.getSelection()?.toString()) {
				e.preventDefault();
				cutBlock(editorState.selectedBlockId);
			}
		}

		// Paste: Ctrl/Cmd + V
		if (isMeta && e.key === 'v' && editorState.clipboard) {
			// Only paste block if no text input focused
			const activeEl = document.activeElement;
			if (
				activeEl?.tagName !== 'INPUT' &&
				activeEl?.tagName !== 'TEXTAREA' &&
				!activeEl?.getAttribute('contenteditable')
			) {
				e.preventDefault();
				pasteBlock();
			}
		}

		// Delete block: Backspace/Delete when block selected
		if ((e.key === 'Backspace' || e.key === 'Delete') && editorState.selectedBlockId) {
			const activeEl = document.activeElement;
			if (
				activeEl?.tagName !== 'INPUT' &&
				activeEl?.tagName !== 'TEXTAREA' &&
				!activeEl?.getAttribute('contenteditable')
			) {
				e.preventDefault();
				deleteBlock(editorState.selectedBlockId);
			}
		}

		// Duplicate: Ctrl/Cmd + D
		if (isMeta && e.key === 'd' && editorState.selectedBlockId) {
			e.preventDefault();
			duplicateBlock(editorState.selectedBlockId);
		}

		// Add block: Ctrl/Cmd + Enter or /
		if ((isMeta && e.key === 'Enter') || (e.key === '/' && !e.target)) {
			e.preventDefault();
			showBlockInserter = true;
		}

		// Escape: Close modals, deselect
		if (e.key === 'Escape') {
			if (showBlockInserter) {
				showBlockInserter = false;
			} else if (editorState.selectedBlockId) {
				editorState.selectedBlockId = null;
			}
		}

		// Toggle preview: Ctrl/Cmd + P
		if (isMeta && e.key === 'p') {
			e.preventDefault();
			editorState.viewMode = editorState.viewMode === 'edit' ? 'preview' : 'edit';
		}

		// Fullscreen: Ctrl/Cmd + Shift + F
		if (isMeta && e.shiftKey && e.key === 'f') {
			e.preventDefault();
			toggleFullscreen();
		}

		// Keyboard help: Ctrl/Cmd + /
		if (isMeta && e.key === '/') {
			e.preventDefault();
			showKeyboardHelp = true;
		}

		// Navigate blocks: Arrow keys
		if (e.key === 'ArrowUp' || e.key === 'ArrowDown') {
			if (editorState.selectedBlockId) {
				const index = editorState.blocks.findIndex((b) => b.id === editorState.selectedBlockId);
				if (index !== -1) {
					const newIndex = e.key === 'ArrowUp' ? index - 1 : index + 1;
					if (newIndex >= 0 && newIndex < editorState.blocks.length) {
						const nextBlock = editorState.blocks[newIndex];
						if (nextBlock) {
							editorState.selectedBlockId = nextBlock.id;
						}
					}
				}
			}
		}

		// Move block: Alt + Arrow
		if (e.altKey && (e.key === 'ArrowUp' || e.key === 'ArrowDown') && editorState.selectedBlockId) {
			e.preventDefault();
			moveBlock(editorState.selectedBlockId, e.key === 'ArrowUp' ? 'up' : 'down');
		}
	}

	// ==========================================================================
	// Save/Autosave
	// ==========================================================================

	async function handleSave() {
		isSaving = true;
		saveError = null;

		try {
			await onsave?.(editorState.blocks);
			editorState.lastSaved = new Date().toISOString();
			editorState.hasUnsavedChanges = false;
		} catch (err) {
			saveError = 'Failed to save. Please try again.';
			console.error('Save failed:', err);
		} finally {
			isSaving = false;
		}
	}

	async function handleAutosave() {
		if (editorState.hasUnsavedChanges && editorState.autosaveEnabled) {
			await handleSave();
		}
	}

	async function handlePublish() {
		isSaving = true;
		try {
			await onpublish?.(editorState.blocks);
			editorState.hasUnsavedChanges = false;
		} catch (err) {
			saveError = 'Failed to publish.';
		} finally {
			isSaving = false;
		}
	}

	// ==========================================================================
	// View Controls
	// ==========================================================================

	function toggleFullscreen() {
		if (!document.fullscreenElement) {
			editorContainer?.requestFullscreen();
			editorState.isFullscreen = true;
		} else {
			document.exitFullscreen();
			editorState.isFullscreen = false;
		}
	}

	function setDevicePreview(device: 'desktop' | 'tablet' | 'mobile') {
		editorState.devicePreview = device;
	}

	// ==========================================================================
	// Block Inserter
	// ==========================================================================

	function openBlockInserter(index: number, e?: MouseEvent) {
		inserterIndex = index;
		if (e) {
			const rect = (e.target as HTMLElement).getBoundingClientRect();
			inserterPosition = { x: rect.left, y: rect.bottom + 8 };
		} else {
			inserterPosition = null;
		}
		showBlockInserter = true;
	}

	function handleBlockInsert(type: BlockType) {
		addBlock(type, inserterIndex);
	}

	// ==========================================================================
	// SEO Analysis
	// ==========================================================================

	function runSEOAnalysis() {
		// Basic SEO analysis
		const content = getPlainTextContent(editorState.blocks);
		const words = content.split(/\s+/).filter((w) => w.length > 0);
		const sentences = content.split(/[.!?]+/).filter((s) => s.trim().length > 0);

		const issues: SEOAnalysis['issues'] = [];
		const suggestions: string[] = [];

		// Title checks
		if (!postTitle) {
			issues.push({
				type: 'error',
				category: 'title',
				message: 'Post title is missing',
				impact: 'high'
			});
		} else if (postTitle.length < 30) {
			issues.push({
				type: 'warning',
				category: 'title',
				message: 'Title is too short (recommended: 50-60 characters)',
				impact: 'medium'
			});
		} else if (postTitle.length > 70) {
			issues.push({
				type: 'warning',
				category: 'title',
				message: 'Title is too long (recommended: 50-60 characters)',
				impact: 'medium'
			});
		}

		// Meta description checks
		if (!metaDescription) {
			issues.push({
				type: 'error',
				category: 'description',
				message: 'Meta description is missing',
				impact: 'high'
			});
		} else if (metaDescription.length < 120) {
			issues.push({
				type: 'warning',
				category: 'description',
				message: 'Meta description is too short (recommended: 150-160 characters)',
				impact: 'medium'
			});
		}

		// Content length
		if (words.length < 300) {
			issues.push({
				type: 'warning',
				category: 'content',
				message: 'Content is too short (recommended: 1000+ words for SEO)',
				impact: 'medium'
			});
			suggestions.push('Add more content to improve SEO ranking');
		}

		// Heading structure
		const headings = editorState.blocks.filter((b) => b.type === 'heading');
		if (headings.length === 0) {
			issues.push({
				type: 'warning',
				category: 'headings',
				message: 'No headings found. Add H2/H3 headings to structure content',
				impact: 'medium'
			});
		}

		// Images without alt
		const imagesWithoutAlt = editorState.blocks.filter(
			(b) => b.type === 'image' && !b.content.mediaAlt
		).length;
		if (imagesWithoutAlt > 0) {
			issues.push({
				type: 'warning',
				category: 'images',
				message: `${imagesWithoutAlt} image(s) missing alt text`,
				impact: 'medium'
			});
		}

		// Keyword density (if focus keyword provided)
		let keywordDensity = 0;
		if (focusKeyword && words.length > 0) {
			const keywordCount = content.toLowerCase().split(focusKeyword.toLowerCase()).length - 1;
			keywordDensity = (keywordCount / words.length) * 100;

			if (keywordDensity < 0.5) {
				issues.push({
					type: 'warning',
					category: 'keywords',
					message: 'Focus keyword density is too low',
					impact: 'medium'
				});
			} else if (keywordDensity > 3) {
				issues.push({
					type: 'warning',
					category: 'keywords',
					message: 'Focus keyword density is too high (keyword stuffing)',
					impact: 'medium'
				});
			}
		}

		// Readability (Flesch-Kincaid approximation)
		const avgWordsPerSentence = sentences.length > 0 ? words.length / sentences.length : 0;
		const avgSyllablesPerWord = 1.5; // Simplified
		const readabilityScore = 206.835 - 1.015 * avgWordsPerSentence - 84.6 * avgSyllablesPerWord;

		let readabilityGrade = 'Easy';
		if (readabilityScore < 30) readabilityGrade = 'Very Difficult';
		else if (readabilityScore < 50) readabilityGrade = 'Difficult';
		else if (readabilityScore < 60) readabilityGrade = 'Fairly Difficult';
		else if (readabilityScore < 70) readabilityGrade = 'Standard';
		else if (readabilityScore < 80) readabilityGrade = 'Fairly Easy';

		// Calculate overall score
		const errorCount = issues.filter((i) => i.type === 'error').length;
		const warningCount = issues.filter((i) => i.type === 'warning').length;
		let score = 100 - errorCount * 20 - warningCount * 5;
		score = Math.max(0, Math.min(100, score));

		let grade: SEOAnalysis['grade'] = 'A';
		if (score < 60) grade = 'F';
		else if (score < 70) grade = 'D';
		else if (score < 80) grade = 'C';
		else if (score < 90) grade = 'B';

		seoAnalysis = {
			score,
			grade,
			issues,
			suggestions,
			keywordDensity,
			readabilityScore: readabilityGrade,
			readabilityGrade: Math.round(readabilityScore),
			wordCount: words.length,
			estimatedReadTime: readTime,
			readingTime: readTime,
			headingStructure: [],
			linksCount: { internal: 0, external: 0 },
			imagesWithoutAlt
		};
	}

	// ==========================================================================
	// Helpers
	// ==========================================================================

	function calculateWordCount(blocks: Block[]): number {
		return blocks.reduce((count, block) => {
			const text = block.content.text || block.content.html || '';
			const plainText = text.replace(/<[^>]*>/g, '');
			return count + plainText.split(/\s+/).filter((w) => w.length > 0).length;
		}, 0);
	}

	function getPlainTextContent(blocks: Block[]): string {
		return blocks
			.map((block) => {
				const text = block.content.text || block.content.html || '';
				return text.replace(/<[^>]*>/g, '');
			})
			.join(' ');
	}

	function formatLastSaved(iso: string | null): string {
		if (!iso) return 'Not saved';
		const date = new Date(iso);
		const now = new Date();
		const diff = now.getTime() - date.getTime();

		if (diff < 60000) return 'Just now';
		if (diff < 3600000) return `${Math.floor(diff / 60000)}m ago`;
		return date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
	}

	function getDeviceWidth(): string {
		switch (editorState.devicePreview) {
			case 'mobile':
				return '375px';
			case 'tablet':
				return '768px';
			default:
				return '100%';
		}
	}
</script>

<div
	bind:this={editorContainer}
	class="block-editor"
	class:fullscreen={editorState.isFullscreen}
	class:preview-mode={editorState.viewMode === 'preview'}
>
	<!-- Top Toolbar -->
	<header class="editor-header">
		<div class="header-left">
			<div class="undo-redo" role="group" aria-label="Undo and redo actions">
				<button
					type="button"
					class="toolbar-btn"
					disabled={!canUndo}
					onclick={undo}
					title="Undo (Ctrl+Z)"
					aria-label="Undo last action"
					aria-keyshortcuts="Control+Z"
				>
					<IconArrowBackUp size={18} aria-hidden="true" />
				</button>
				<button
					type="button"
					class="toolbar-btn"
					disabled={!canRedo}
					onclick={redo}
					title="Redo (Ctrl+Shift+Z)"
					aria-label="Redo last undone action"
					aria-keyshortcuts="Control+Shift+Z"
				>
					<IconArrowForwardUp size={18} aria-hidden="true" />
				</button>
			</div>

			<div class="block-info">
				<span class="block-count">{blockCount} blocks</span>
				<span class="word-count">{wordCount} words</span>
				<span class="read-time">{readTime} min read</span>
			</div>
		</div>

		<div class="header-center">
			<div class="device-preview" role="group" aria-label="Device preview options">
				<button
					type="button"
					class="device-btn"
					class:active={editorState.devicePreview === 'desktop'}
					onclick={() => setDevicePreview('desktop')}
					title="Desktop Preview"
					aria-label="Preview as desktop"
					aria-pressed={editorState.devicePreview === 'desktop'}
				>
					<IconDeviceDesktop size={18} aria-hidden="true" />
				</button>
				<button
					type="button"
					class="device-btn"
					class:active={editorState.devicePreview === 'tablet'}
					onclick={() => setDevicePreview('tablet')}
					title="Tablet Preview"
					aria-label="Preview as tablet"
					aria-pressed={editorState.devicePreview === 'tablet'}
				>
					<IconDeviceTablet size={18} aria-hidden="true" />
				</button>
				<button
					type="button"
					class="device-btn"
					class:active={editorState.devicePreview === 'mobile'}
					onclick={() => setDevicePreview('mobile')}
					title="Mobile Preview"
					aria-label="Preview as mobile"
					aria-pressed={editorState.devicePreview === 'mobile'}
				>
					<IconDeviceMobile size={18} aria-hidden="true" />
				</button>
			</div>

			<div class="view-toggle" role="group" aria-label="View mode">
				<button
					type="button"
					class="view-btn"
					class:active={editorState.viewMode === 'edit'}
					onclick={() => (editorState.viewMode = 'edit')}
					aria-label="Edit mode"
					aria-pressed={editorState.viewMode === 'edit'}
				>
					<IconEdit size={16} aria-hidden="true" />
					Edit
				</button>
				<button
					type="button"
					class="view-btn"
					class:active={editorState.viewMode === 'preview'}
					onclick={() => (editorState.viewMode = 'preview')}
					aria-label="Preview mode"
					aria-pressed={editorState.viewMode === 'preview'}
				>
					<IconEye size={16} aria-hidden="true" />
					Preview
				</button>
			</div>
		</div>

		<div class="header-right">
			<div class="save-status">
				{#if saveError}
					<span class="error">{saveError}</span>
				{:else if isSaving}
					<span class="saving"><IconCloudUpload size={16} class="spin" /> Saving...</span>
				{:else if editorState.hasUnsavedChanges}
					<span class="unsaved">Unsaved changes</span>
				{:else}
					<span class="saved">Saved {formatLastSaved(editorState.lastSaved)}</span>
				{/if}
			</div>
			{#if seoAnalysis && seoAnalysis.grade}
				<div class="seo-score" title="SEO Score: {seoAnalysis.score}/100">
					<span class="grade grade-{seoAnalysis.grade.toLowerCase()}">{seoAnalysis.grade}</span>
				</div>
			{/if}

			<div class="header-actions" role="group" aria-label="Editor actions">
				<button
					type="button"
					class="toolbar-btn"
					onclick={() => (showRevisions = true)}
					title="Revision History"
					aria-label="View revision history"
				>
					<IconHistory size={18} aria-hidden="true" />
				</button>
				<button
					type="button"
					class="toolbar-btn"
					onclick={() => (showKeyboardHelp = true)}
					title="Keyboard Shortcuts"
					aria-label="Show keyboard shortcuts"
					aria-keyshortcuts="Control+/"
				>
					<IconKeyboard size={18} aria-hidden="true" />
				</button>
				<button
					type="button"
					class="toolbar-btn"
					onclick={toggleFullscreen}
					title="Toggle Fullscreen"
					aria-label={editorState.isFullscreen ? 'Exit fullscreen' : 'Enter fullscreen'}
					aria-pressed={editorState.isFullscreen}
				>
					{#if editorState.isFullscreen}
						<IconMinimize size={18} aria-hidden="true" />
					{:else}
						<IconMaximize size={18} aria-hidden="true" />
					{/if}
				</button>
			</div>

			<button
				type="button"
				class="btn-save"
				onclick={handleSave}
				disabled={isSaving || !editorState.hasUnsavedChanges}
				aria-label="Save draft"
				aria-keyshortcuts="Control+S"
			>
				<IconDeviceFloppy size={18} aria-hidden="true" />
				Save Draft
			</button>
			<button
				type="button"
				class="btn-publish"
				onclick={handlePublish}
				disabled={isSaving}
				aria-label="Publish post"
			>
				<IconCloudUpload size={18} aria-hidden="true" />
				Publish
			</button>
		</div>
	</header>

	<div class="editor-body">
		<!-- Sidebar -->
		<aside class="editor-sidebar" aria-label="Editor sidebar">
			<div class="sidebar-tabs" role="tablist" aria-label="Editor panels">
				<button
					type="button"
					class="tab-btn"
					class:active={editorState.sidebarTab === 'blocks'}
					onclick={() => (editorState.sidebarTab = 'blocks')}
					title="Add Blocks"
					role="tab"
					aria-selected={editorState.sidebarTab === 'blocks'}
					aria-controls="panel-blocks"
					aria-label="Add blocks panel"
				>
					<IconPlus size={20} aria-hidden="true" />
				</button>
				<button
					type="button"
					class="tab-btn"
					class:active={editorState.sidebarTab === 'settings'}
					onclick={() => (editorState.sidebarTab = 'settings')}
					title="Block Settings"
					role="tab"
					aria-selected={editorState.sidebarTab === 'settings'}
					aria-controls="panel-settings"
					aria-label="Block settings panel"
				>
					<IconSettings size={20} aria-hidden="true" />
				</button>
				<button
					type="button"
					class="tab-btn"
					class:active={editorState.sidebarTab === 'layers'}
					onclick={() => (editorState.sidebarTab = 'layers')}
					title="Block Layers"
					role="tab"
					aria-selected={editorState.sidebarTab === 'layers'}
					aria-controls="panel-layers"
					aria-label="Block layers panel"
				>
					<IconStack2 size={20} aria-hidden="true" />
				</button>
				<button
					type="button"
					class="tab-btn"
					class:active={editorState.sidebarTab === 'ai'}
					onclick={() => (editorState.sidebarTab = 'ai')}
					title="AI Assistant"
					role="tab"
					aria-selected={editorState.sidebarTab === 'ai'}
					aria-controls="panel-ai"
					aria-label="AI assistant panel"
				>
					<IconRobot size={20} aria-hidden="true" />
				</button>
				<button
					type="button"
					class="tab-btn"
					class:active={editorState.sidebarTab === 'seo'}
					onclick={() => (editorState.sidebarTab = 'seo')}
					title="SEO Analysis"
					role="tab"
					aria-selected={editorState.sidebarTab === 'seo'}
					aria-controls="panel-seo"
					aria-label="SEO analysis panel"
				>
					<IconSeo size={20} aria-hidden="true" />
				</button>
			</div>

			<div class="sidebar-content">
				{#if editorState.sidebarTab === 'blocks'}
					<div class="panel-header" id="panel-blocks" role="tabpanel" aria-labelledby="tab-blocks">
						<h3>Add Block</h3>
					</div>
					<div class="block-search">
						<IconSearch size={16} aria-hidden="true" />
						<input
							type="text"
							id="search-blocks"
							name="search"
							placeholder="Search blocks..."
							bind:value={searchQuery}
							aria-label="Search for blocks"
						/>
					</div>
					<BlockInserter {searchQuery} oninsert={handleBlockInsert} />
				{:else if editorState.sidebarTab === 'settings'}
					<div id="panel-settings" role="tabpanel" aria-labelledby="tab-settings">
						{#if selectedBlock}
							<BlockSettingsPanel
								block={selectedBlock}
								onupdate={(updates) => updateBlock(selectedBlock.id, updates)}
							/>
						{:else}
							<div class="empty-panel">
								<IconSettings size={48} stroke={1} aria-hidden="true" />
								<p>Select a block to edit its settings</p>
							</div>
						{/if}
					</div>
				{:else if editorState.sidebarTab === 'layers'}
					<div id="panel-layers" role="tabpanel" aria-labelledby="tab-layers">
						<div class="panel-header">
							<h3>Block Layers</h3>
						</div>
						<div class="layers-list" role="listbox" aria-label="Content blocks">
							{#each editorState.blocks as block, i (block.id)}
								<button
									type="button"
									class="layer-item"
									class:selected={block.id === editorState.selectedBlockId}
									onclick={() => (editorState.selectedBlockId = block.id)}
									role="option"
									aria-selected={block.id === editorState.selectedBlockId}
									aria-label="{BLOCK_DEFINITIONS[block.type]?.name || block.type}, position {i + 1}"
								>
									<IconGripVertical size={14} aria-hidden="true" />
									<span class="layer-type">{BLOCK_DEFINITIONS[block.type]?.name || block.type}</span
									>
									<span class="layer-index">#{i + 1}</span>
								</button>
							{/each}
						</div>
					</div>
				{:else if editorState.sidebarTab === 'ai'}
					<div id="panel-ai" role="tabpanel" aria-labelledby="tab-ai">
						<AIAssistant
							{editorState}
							onapply={(content) => {
								// Handle AI content insertion
								if (editorState.selectedBlockId) {
									updateBlock(editorState.selectedBlockId, {
										content: { ...selectedBlock?.content, text: content }
									});
								} else {
									const newBlock = createBlock('paragraph');
									newBlock.content.text = content;
									editorState.blocks = [...editorState.blocks, newBlock];
								}
							}}
						/>
					</div>
				{:else if editorState.sidebarTab === 'seo'}
					<div id="panel-seo" role="tabpanel" aria-labelledby="tab-seo">
						<SEOAnalyzer
							title={postTitle}
							content={getPlainTextContent(editorState.blocks)}
							{metaDescription}
							{focusKeyword}
							slug={postSlug}
						/>
					</div>
				{/if}
			</div>
		</aside>

		<!-- Main Canvas -->
		<main
			class="editor-canvas"
			style:--device-width={getDeviceWidth()}
			style:--zoom={editorState.zoom / 100}
		>
			<div class="canvas-wrapper">
				{#if editorState.blocks.length === 0}
					<!-- Empty State -->
					<div class="empty-state" transition:fade>
						<div class="empty-icon">
							<IconPlus size={48} />
						</div>
						<h2>Start creating your content</h2>
						<p>Click the button below to add your first block</p>
						<button type="button" class="btn-add-first" onclick={() => openBlockInserter(0)}>
							<IconPlus size={18} />
							Add Block
						</button>
					</div>
				{:else}
					<!-- Block List -->
					<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
					<div
						class="blocks-container"
						class:drag-active={editorState.isDragging}
						role="listbox"
						aria-label="Content blocks"
						aria-describedby="drag-instructions"
						aria-multiselectable="true"
					>
						<!-- Hidden instructions for screen readers -->
						<div id="drag-instructions" class="sr-only">
							Use Shift or Cmd/Ctrl click to select multiple blocks. Press Alt plus Arrow Up or
							Arrow Down to reorder selected blocks. Drag blocks to reorder them visually.
						</div>
						{#each editorState.blocks as block, index (block.id)}
							{@const isMultiSelected = editorState.selectedBlockIds.includes(block.id)}
							{@const isDragTarget = editorState.draggedBlockIds.includes(block.id)}
							{@const isDropBefore =
								index === editorState.dropTargetIndex && editorState.dropPosition === 'before'}
							{@const isDropAfter =
								index === editorState.dropTargetIndex && editorState.dropPosition === 'after'}
							<!-- svelte-ignore a11y_no_noninteractive_tabindex a11y_no_noninteractive_element_interactions -->
							<div
								class="block-wrapper"
								class:selected={block.id === editorState.selectedBlockId}
								class:multi-selected={isMultiSelected && editorState.selectedBlockIds.length > 1}
								class:hovered={block.id === editorState.hoveredBlockId}
								class:dragging={isDragTarget}
								class:drop-target={index === editorState.dropTargetIndex}
								class:drop-before={isDropBefore && editorState.isDragging}
								class:drop-after={isDropAfter && editorState.isDragging}
								data-block-id={block.id}
								data-block-index={index}
								draggable={!readOnly && editorState.viewMode === 'edit'}
								ondragstart={(e: DragEvent) => handleDragStart(e, block.id)}
								ondragover={(e: DragEvent) => handleDragOver(e, index)}
								ondragend={handleDragEnd}
								ondrop={(e: DragEvent) => handleDrop(e, index)}
								onclick={(e: MouseEvent) => {
									const addToSelection = e.shiftKey || e.metaKey || e.ctrlKey;
									if (e.shiftKey && editorState.selectedBlockId) {
										selectBlockRange(editorState.selectedBlockId, block.id);
									} else {
										toggleBlockSelection(block.id, addToSelection);
									}
								}}
								onkeydown={(e: KeyboardEvent) => {
									if (e.key === 'Enter' || e.key === ' ') {
										e.preventDefault();
										editorState.selectedBlockId = block.id;
									}
									handleKeyboardReorder(e, block.id);
								}}
								onmouseenter={() => (editorState.hoveredBlockId = block.id)}
								onmouseleave={() => (editorState.hoveredBlockId = null)}
								ontouchstart={(e: TouchEvent) => handleTouchStart(e, block.id)}
								ontouchmove={handleTouchMove}
								ontouchend={handleTouchEnd}
								ontouchcancel={cancelTouchDrag}
								tabindex="0"
								role="option"
								aria-grabbed={isDragTarget}
								aria-dropeffect={editorState.isDragging && !isDragTarget ? 'move' : 'none'}
								aria-selected={block.id === editorState.selectedBlockId || isMultiSelected}
								aria-label={`${BLOCK_DEFINITIONS[block.type]?.name || block.type} block, position ${index + 1} of ${editorState.blocks.length}${isMultiSelected ? ', selected' : ''}`}
								style="touch-action: {editorState.isDragging
									? 'none'
									: 'pan-y'}; will-change: {editorState.isDragging
									? 'transform, opacity'
									: 'auto'};"
								animate:flip={{ duration: 300, easing: quintOut }}
							>
								<!-- Block Toolbar (on hover/select) -->
								{#if (block.id === editorState.selectedBlockId || block.id === editorState.hoveredBlockId) && editorState.viewMode === 'edit'}
									<div class="block-toolbar" transition:fade={{ duration: 150 }}>
										<button
											type="button"
											class="block-tool"
											draggable="true"
											title="Drag to reorder"
										>
											<IconGripVertical size={16} />
										</button>
										<button
											type="button"
											class="block-tool"
											onclick={(e: MouseEvent) => {
												e.stopPropagation();
												moveBlock(block.id, 'up');
											}}
											disabled={index === 0}
											title="Move up"
										>
											<IconChevronUp size={16} />
										</button>
										<button
											type="button"
											class="block-tool"
											onclick={(e: MouseEvent) => {
												e.stopPropagation();
												moveBlock(block.id, 'down');
											}}
											disabled={index === editorState.blocks.length - 1}
											title="Move down"
										>
											<IconChevronDown size={16} />
										</button>
										<div class="tool-divider"></div>
										<button
											type="button"
											class="block-tool"
											onclick={(e: MouseEvent) => {
												e.stopPropagation();
												duplicateBlock(block.id);
											}}
											title="Duplicate"
										>
											<IconCopy size={16} />
										</button>
										<button
											type="button"
											class="block-tool danger"
											onclick={(e: MouseEvent) => {
												e.stopPropagation();
												deleteBlock(block.id);
											}}
											title="Delete"
										>
											<IconTrash size={16} />
										</button>
									</div>
								{/if}

								<!-- Block Content -->
								<BlockRenderer
									{block}
									isSelected={block.id === editorState.selectedBlockId}
									isEditing={editorState.viewMode === 'edit'}
									onUpdate={(updates) => updateBlock(block.id, updates)}
								/>

								<!-- Add Block Between Button -->
								{#if editorState.viewMode === 'edit'}
									<button
										type="button"
										class="add-between-btn"
										onclick={(e: MouseEvent) => {
											e.stopPropagation();
											openBlockInserter(index + 1, e);
										}}
										title="Add block"
									>
										<IconPlus size={16} />
									</button>
								{/if}
							</div>
						{/each}
					</div>
				{/if}
			</div>
		</main>
	</div>
</div>

<!-- Screen Reader Live Region for Drag-Drop Announcements -->
<div bind:this={liveRegionRef} class="sr-only" role="status" aria-live="polite" aria-atomic="true">
	{#each announcements as announcement}
		<p>{announcement}</p>
	{/each}
</div>

<!-- Drop Indicator (visible during drag operations) -->
{#if dragVisuals.dropIndicatorVisible && editorState.isDragging}
	<div
		class="drop-indicator"
		style="top: {dragVisuals.dropIndicatorY}px;"
		transition:fade={{ duration: 100 }}
	>
		<div class="drop-indicator-line"></div>
		<div class="drop-indicator-dot drop-indicator-dot-left"></div>
		<div class="drop-indicator-dot drop-indicator-dot-right"></div>
	</div>
{/if}

<!-- Multi-Selection Count Badge -->
{#if editorState.selectedBlockIds.length > 1 && !editorState.isDragging}
	<div class="multi-selection-badge" transition:fade={{ duration: 150 }}>
		{editorState.selectedBlockIds.length} blocks selected
		<button
			type="button"
			class="clear-selection-btn"
			onclick={() => {
				editorState.selectedBlockIds = [];
			}}
			aria-label="Clear selection"
		>
			Clear
		</button>
	</div>
{/if}

<!-- Block Inserter Modal -->
{#if showBlockInserter}
	<BlockInserter
		isModal={true}
		position={inserterPosition}
		{searchQuery}
		oninsert={(type) => {
			addBlock(type, inserterIndex);
			showBlockInserter = false;
		}}
		onclose={() => (showBlockInserter = false)}
	/>
{/if}

<!-- Keyboard Shortcuts Modal -->
{#if showKeyboardHelp}
	<KeyboardShortcuts isOpen={showKeyboardHelp} onClose={() => (showKeyboardHelp = false)} />
{/if}

<!-- Revision History Modal -->
{#if showRevisions}
	<RevisionHistory
		currentBlocks={editorState.blocks}
		{revisions}
		onRestore={(revision) => {
			pushToHistory();
			editorState.blocks = revision.blocks;
			showRevisions = false;
		}}
	/>
{/if}

<style>
	.block-editor {
		display: flex;
		flex-direction: column;
		height: 100vh;
		background: #f8f9fa;
		color: #1a1a1a;
		font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
	}

	.block-editor.fullscreen {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		z-index: 9999;
	}

	/* Header */
	.editor-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0.75rem 1rem;
		background: white;
		border-bottom: 1px solid #e5e5e5;
		gap: 1rem;
	}

	.header-left,
	.header-center,
	.header-right {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.undo-redo {
		display: flex;
		gap: 0.25rem;
	}

	.toolbar-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		border: none;
		background: transparent;
		border-radius: 6px;
		cursor: pointer;
		color: #666;
		transition: all 0.15s;
	}

	.toolbar-btn:hover:not(:disabled) {
		background: #f0f0f0;
		color: #1a1a1a;
	}

	.toolbar-btn:disabled {
		opacity: 0.4;
		cursor: not-allowed;
	}

	/* Focus visible styles for keyboard navigation - WCAG 2.1 AA */
	.toolbar-btn:focus-visible,
	.device-btn:focus-visible,
	.view-btn:focus-visible,
	.tab-btn:focus-visible,
	.btn-save:focus-visible,
	.btn-publish:focus-visible,
	.block-tool:focus-visible,
	.layer-item:focus-visible,
	.add-between-btn:focus-visible,
	.btn-add-first:focus-visible {
		outline: 2px solid #3b82f6;
		outline-offset: 2px;
	}

	.block-wrapper:focus-visible {
		outline: 2px solid #3b82f6;
		outline-offset: 4px;
	}

	/* Screen reader only class */
	.sr-only {
		position: absolute;
		width: 1px;
		height: 1px;
		padding: 0;
		margin: -1px;
		overflow: hidden;
		clip: rect(0, 0, 0, 0);
		white-space: nowrap;
		border: 0;
	}

	.block-info {
		display: flex;
		gap: 1rem;
		font-size: 0.8125rem;
		color: #666;
	}

	.device-preview {
		display: flex;
		background: #f0f0f0;
		border-radius: 8px;
		padding: 0.25rem;
	}

	.device-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		border: none;
		background: transparent;
		border-radius: 6px;
		cursor: pointer;
		color: #666;
		transition: all 0.15s;
	}

	.device-btn:hover {
		color: #1a1a1a;
	}

	.device-btn.active {
		background: white;
		color: #3b82f6;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
	}

	.view-toggle {
		display: flex;
		background: #f0f0f0;
		border-radius: 8px;
		padding: 0.25rem;
	}

	.view-btn {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.375rem 0.75rem;
		border: none;
		background: transparent;
		border-radius: 6px;
		cursor: pointer;
		color: #666;
		font-size: 0.8125rem;
		font-weight: 500;
		transition: all 0.15s;
	}

	.view-btn.active {
		background: white;
		color: #1a1a1a;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
	}

	.save-status {
		font-size: 0.8125rem;
	}

	.save-status .saving {
		color: #3b82f6;
		display: flex;
		align-items: center;
		gap: 0.375rem;
	}

	.save-status .unsaved {
		color: #f59e0b;
	}

	.save-status .saved {
		color: #10b981;
	}

	.header-actions {
		display: flex;
		gap: 0.25rem;
	}

	.btn-save,
	.btn-publish {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.5rem 1rem;
		border: none;
		border-radius: 6px;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.15s;
	}

	.btn-save {
		background: #f0f0f0;
		color: #1a1a1a;
	}

	.btn-save:hover:not(:disabled) {
		background: #e5e5e5;
	}

	.btn-publish {
		background: #3b82f6;
		color: white;
	}

	.btn-publish:hover:not(:disabled) {
		background: #2563eb;
	}

	.btn-save:disabled,
	.btn-publish:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	/* Editor Body */
	.editor-body {
		display: flex;
		flex: 1;
		overflow: hidden;
	}

	/* Sidebar */
	.editor-sidebar {
		width: 320px;
		background: white;
		border-right: 1px solid #e5e5e5;
		display: flex;
		flex-direction: column;
	}

	.sidebar-tabs {
		display: flex;
		border-bottom: 1px solid #e5e5e5;
		padding: 0.5rem;
		gap: 0.25rem;
	}

	.tab-btn {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 0.625rem;
		border: none;
		background: transparent;
		border-radius: 6px;
		cursor: pointer;
		color: #666;
		transition: all 0.15s;
	}

	.tab-btn:hover {
		background: #f0f0f0;
		color: #1a1a1a;
	}

	.tab-btn.active {
		background: #3b82f6;
		color: white;
	}

	.sidebar-content {
		flex: 1;
		overflow-y: auto;
		padding: 1rem;
	}

	.panel-header {
		margin-bottom: 1rem;
	}

	.panel-header h3 {
		font-size: 0.9375rem;
		font-weight: 600;
		color: #1a1a1a;
		margin: 0;
	}

	.block-search {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 0.75rem;
		background: #f5f5f5;
		border-radius: 8px;
		margin-bottom: 1rem;
	}

	.block-search input {
		flex: 1;
		border: none;
		background: transparent;
		font-size: 0.875rem;
		outline: none;
	}

	.block-search input::placeholder {
		color: #999;
	}

	.empty-panel {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		height: 200px;
		color: #999;
		text-align: center;
	}

	.empty-panel p {
		margin-top: 0.75rem;
		font-size: 0.875rem;
	}

	.layers-list {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.layer-item {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 0.75rem;
		background: #f5f5f5;
		border: 1px solid transparent;
		border-radius: 6px;
		cursor: pointer;
		font-size: 0.8125rem;
		text-align: left;
		transition: all 0.15s;
	}

	.layer-item:hover {
		background: #e5e5e5;
	}

	.layer-item.selected {
		background: #dbeafe;
		border-color: #3b82f6;
	}

	.layer-type {
		flex: 1;
		font-weight: 500;
	}

	.layer-index {
		color: #999;
		font-size: 0.75rem;
	}

	/* Canvas */
	.editor-canvas {
		flex: 1;
		overflow: auto;
		padding: 2rem;
		display: flex;
		justify-content: center;
	}

	.canvas-wrapper {
		width: var(--device-width, 100%);
		max-width: 900px;
		background: white;
		border-radius: 8px;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
		padding: 2rem;
		min-height: 600px;
		transform: scale(var(--zoom, 1));
		transform-origin: top center;
		transition: width 0.3s ease;
	}

	/* Empty State */
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		height: 400px;
		text-align: center;
	}

	.empty-icon {
		width: 80px;
		height: 80px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: #f0f0f0;
		border-radius: 16px;
		color: #999;
		margin-bottom: 1.5rem;
	}

	.empty-state h2 {
		font-size: 1.25rem;
		font-weight: 600;
		color: #1a1a1a;
		margin: 0 0 0.5rem;
	}

	.empty-state p {
		color: #666;
		margin: 0 0 1.5rem;
	}

	.btn-add-first {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		background: #3b82f6;
		color: white;
		border: none;
		border-radius: 8px;
		font-size: 0.9375rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.15s;
	}

	.btn-add-first:hover {
		background: #2563eb;
	}

	/* Block Wrapper */
	.blocks-container {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.block-wrapper {
		position: relative;
		border-radius: 4px;
		transition: all 0.15s;
		outline: none;
	}

	.block-wrapper:focus {
		outline: none;
	}

	.block-wrapper.hovered {
		outline: 2px dashed #d0d0d0;
		outline-offset: 2px;
	}

	.block-wrapper.selected {
		outline: 2px solid #3b82f6;
		outline-offset: 2px;
	}

	.block-wrapper.dragging {
		opacity: 0.5;
	}

	.block-wrapper.drop-target::before {
		content: '';
		position: absolute;
		top: -4px;
		left: 0;
		right: 0;
		height: 4px;
		background: #3b82f6;
		border-radius: 2px;
	}

	/* Block Toolbar */
	.block-toolbar {
		position: absolute;
		left: -48px;
		top: 0;
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
		padding: 0.25rem;
		background: white;
		border: 1px solid #e5e5e5;
		border-radius: 8px;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
	}

	.block-tool {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 28px;
		height: 28px;
		border: none;
		background: transparent;
		border-radius: 4px;
		cursor: pointer;
		color: #666;
		transition: all 0.15s;
	}

	.block-tool:hover:not(:disabled) {
		background: #f0f0f0;
		color: #1a1a1a;
	}

	.block-tool:disabled {
		opacity: 0.4;
		cursor: not-allowed;
	}

	.block-tool.danger:hover {
		background: #fef2f2;
		color: #ef4444;
	}

	.tool-divider {
		height: 1px;
		background: #e5e5e5;
		margin: 0.25rem 0;
	}

	/* Add Between Button */
	.add-between-btn {
		position: absolute;
		bottom: -16px;
		left: 50%;
		transform: translateX(-50%);
		width: 24px;
		height: 24px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: white;
		border: 1px solid #e5e5e5;
		border-radius: 50%;
		cursor: pointer;
		color: #999;
		opacity: 0;
		transition: all 0.15s;
		z-index: 10;
	}

	.block-wrapper:hover .add-between-btn {
		opacity: 1;
	}

	.add-between-btn:hover {
		background: #3b82f6;
		border-color: #3b82f6;
		color: white;
	}

	/* Preview Mode */
	.preview-mode .block-toolbar,
	.preview-mode .add-between-btn {
		display: none;
	}

	.preview-mode .block-wrapper {
		outline: none !important;
	}

	/* Animations */
	:global(.spin) {
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

	/* Responsive */
	@media (max-width: 1024px) {
		.editor-sidebar {
			width: 280px;
		}

		.block-info {
			display: none;
		}
	}

	@media (max-width: 768px) {
		.editor-header {
			flex-wrap: wrap;
		}

		.editor-sidebar {
			position: fixed;
			left: 0;
			top: 60px;
			bottom: 0;
			z-index: 100;
			transform: translateX(-100%);
			transition: transform 0.3s ease;
		}
	}

	/* ==========================================================================
	   ENHANCED DRAG-AND-DROP STYLES (Apple Principal Engineer Grade)
	   ========================================================================== */

	/* Screen Reader Only - Hidden visually but accessible */
	.sr-only {
		position: absolute;
		width: 1px;
		height: 1px;
		padding: 0;
		margin: -1px;
		overflow: hidden;
		clip: rect(0, 0, 0, 0);
		white-space: nowrap;
		border: 0;
	}

	/* Multi-selection visual feedback */
	.block-wrapper.multi-selected {
		outline: 2px solid #8b5cf6;
		outline-offset: 2px;
		background: linear-gradient(135deg, rgba(139, 92, 246, 0.05) 0%, rgba(139, 92, 246, 0.02) 100%);
	}

	.block-wrapper.multi-selected::before {
		content: '';
		position: absolute;
		top: -2px;
		left: -6px;
		width: 4px;
		height: calc(100% + 4px);
		background: linear-gradient(180deg, #8b5cf6 0%, #6366f1 100%);
		border-radius: 2px;
		opacity: 0.8;
	}

	/* Enhanced dragging state with GPU acceleration */
	.block-wrapper.dragging {
		opacity: 0.4;
		transform: scale(0.98);
		outline: 2px dashed #3b82f6;
		outline-offset: 4px;
		background: repeating-linear-gradient(
			45deg,
			transparent,
			transparent 10px,
			rgba(59, 130, 246, 0.03) 10px,
			rgba(59, 130, 246, 0.03) 20px
		);
		transition:
			opacity 0.2s cubic-bezier(0.4, 0, 0.2, 1),
			transform 0.2s cubic-bezier(0.4, 0, 0.2, 1);
		will-change: transform, opacity;
	}

	/* Enhanced drop target indicator - before position */
	.block-wrapper.drop-before::before {
		content: '';
		position: absolute;
		top: -4px;
		left: 0;
		right: 0;
		height: 4px;
		background: linear-gradient(90deg, #3b82f6 0%, #60a5fa 50%, #3b82f6 100%);
		border-radius: 2px;
		box-shadow:
			0 0 12px rgba(59, 130, 246, 0.5),
			0 0 24px rgba(59, 130, 246, 0.3);
		animation: dropIndicatorPulse 1s ease-in-out infinite;
	}

	/* Enhanced drop target indicator - after position */
	.block-wrapper.drop-after::after {
		content: '';
		position: absolute;
		bottom: -4px;
		left: 0;
		right: 0;
		height: 4px;
		background: linear-gradient(90deg, #3b82f6 0%, #60a5fa 50%, #3b82f6 100%);
		border-radius: 2px;
		box-shadow:
			0 0 12px rgba(59, 130, 246, 0.5),
			0 0 24px rgba(59, 130, 246, 0.3);
		animation: dropIndicatorPulse 1s ease-in-out infinite;
	}

	@keyframes dropIndicatorPulse {
		0%,
		100% {
			opacity: 1;
			transform: scaleX(1);
		}
		50% {
			opacity: 0.7;
			transform: scaleX(0.98);
		}
	}

	/* Floating drop indicator */
	.drop-indicator {
		position: fixed;
		left: 0;
		right: 0;
		height: 4px;
		pointer-events: none;
		z-index: 10000;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.drop-indicator-line {
		position: absolute;
		left: 50%;
		transform: translateX(-50%);
		width: min(900px, calc(100% - 4rem));
		height: 4px;
		background: linear-gradient(
			90deg,
			transparent 0%,
			#3b82f6 5%,
			#60a5fa 50%,
			#3b82f6 95%,
			transparent 100%
		);
		border-radius: 2px;
		box-shadow:
			0 0 20px rgba(59, 130, 246, 0.6),
			0 0 40px rgba(59, 130, 246, 0.3);
	}

	.drop-indicator-dot {
		position: absolute;
		width: 12px;
		height: 12px;
		background: #3b82f6;
		border: 2px solid white;
		border-radius: 50%;
		box-shadow: 0 2px 8px rgba(59, 130, 246, 0.5);
		animation: dropDotPulse 1s ease-in-out infinite;
	}

	.drop-indicator-dot-left {
		left: calc(50% - min(450px, calc(50% - 2rem)));
	}

	.drop-indicator-dot-right {
		right: calc(50% - min(450px, calc(50% - 2rem)));
	}

	@keyframes dropDotPulse {
		0%,
		100% {
			transform: scale(1);
		}
		50% {
			transform: scale(1.2);
		}
	}

	/* Multi-selection badge */
	.multi-selection-badge {
		position: fixed;
		bottom: 24px;
		left: 50%;
		transform: translateX(-50%);
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 12px 20px;
		background: linear-gradient(135deg, #1e1b4b 0%, #312e81 100%);
		color: white;
		border-radius: 12px;
		font-size: 14px;
		font-weight: 500;
		box-shadow:
			0 10px 40px rgba(30, 27, 75, 0.4),
			0 4px 12px rgba(0, 0, 0, 0.2);
		z-index: 1000;
		backdrop-filter: blur(8px);
	}

	.clear-selection-btn {
		padding: 6px 12px;
		background: rgba(255, 255, 255, 0.15);
		border: 1px solid rgba(255, 255, 255, 0.2);
		border-radius: 6px;
		color: white;
		font-size: 12px;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.15s;
	}

	.clear-selection-btn:hover {
		background: rgba(255, 255, 255, 0.25);
		border-color: rgba(255, 255, 255, 0.3);
	}

	/* Drop feedback animation (haptic-style) */
	.canvas-wrapper.drop-feedback {
		animation: dropFeedback 0.3s cubic-bezier(0.4, 0, 0.2, 1);
	}

	@keyframes dropFeedback {
		0% {
			transform: scale(var(--zoom, 1));
		}
		30% {
			transform: scale(calc(var(--zoom, 1) * 0.995));
		}
		100% {
			transform: scale(var(--zoom, 1));
		}
	}

	/* Enhanced block wrapper hover during drag */
	.block-wrapper:not(.dragging):hover {
		transition:
			outline 0.15s ease,
			background 0.15s ease;
	}

	/* Drag handle visual enhancement */
	.block-tool[draggable='true'] {
		cursor: grab;
	}

	.block-tool[draggable='true']:active {
		cursor: grabbing;
	}

	/* Focus ring for keyboard navigation */
	.block-wrapper:focus-visible {
		outline: 2px solid #3b82f6;
		outline-offset: 2px;
		box-shadow: 0 0 0 4px rgba(59, 130, 246, 0.2);
	}

	/* Touch-friendly drag handle */
	@media (pointer: coarse) {
		.block-tool {
			width: 36px;
			height: 36px;
		}

		.block-toolbar {
			left: -52px;
		}

		.block-wrapper {
			/* Prevent text selection on long-press */
			-webkit-user-select: none;
			user-select: none;
		}
	}

	/* Reduced motion preference */
	@media (prefers-reduced-motion: reduce) {
		.block-wrapper.dragging,
		.block-wrapper.drop-before::before,
		.block-wrapper.drop-after::after,
		.canvas-wrapper.drop-feedback,
		.drop-indicator-dot,
		.drop-indicator-line {
			animation: none;
			transition: none;
		}

		.drop-indicator-line {
			box-shadow: 0 0 8px rgba(59, 130, 246, 0.5);
		}
	}

	/* Dark mode support */
	@media (prefers-color-scheme: dark) {
		.drop-indicator-dot {
			border-color: #1e1b4b;
		}

		.multi-selection-badge {
			background: linear-gradient(135deg, #1e293b 0%, #334155 100%);
		}
	}

	/* Spring animation for smooth block reordering */
	.blocks-container {
		perspective: 1000px;
	}

	.block-wrapper {
		transform-style: preserve-3d;
		backface-visibility: hidden;
	}

	/* GPU acceleration hints for drag operations */
	.block-wrapper.dragging,
	.block-wrapper.drop-target {
		will-change: transform, opacity;
		transform: translateZ(0);
	}

	/* Gradient drop zone highlight */
	.blocks-container.drag-active {
		background: linear-gradient(
			180deg,
			transparent 0%,
			rgba(59, 130, 246, 0.02) 10%,
			rgba(59, 130, 246, 0.02) 90%,
			transparent 100%
		);
	}
</style>

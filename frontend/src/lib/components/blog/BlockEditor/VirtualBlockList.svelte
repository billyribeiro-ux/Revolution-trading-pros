<!--
/**
 * VirtualBlockList - High-Performance Virtual Scrolling for Block Editor
 * ═══════════════════════════════════════════════════════════════════════════
 * Efficiently renders 100+ blocks using viewport-based virtualization
 *
 * FEATURES:
 * - Custom virtualization with dynamic height measurement
 * - Smooth scrolling with momentum
 * - Drag-and-drop support with visual feedback
 * - Keyboard navigation with auto-scroll to focused block
 * - Performance metrics logging
 * - Overscan buffer for seamless scrolling
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 */
-->

<script lang="ts" module>
	import type { Block } from './types';

	// ==========================================================================
	// Types
	// ==========================================================================

	export interface VirtualBlockListProps {
		blocks: Block[];
		selectedBlockId: string | null;
		onSelectBlock: (id: string) => void;
		onUpdateBlock: (id: string, updates: Partial<Block>) => void;
		onDeleteBlock: (id: string) => void;
		onMoveBlock: (fromIndex: number, toIndex: number) => void;
		isEditing: boolean;
	}

	export interface VisibleRange {
		start: number;
		end: number;
	}

	export interface BlockMeasurement {
		height: number;
		measured: boolean;
	}

	export interface PerformanceMetrics {
		renderTime: number;
		visibleCount: number;
		totalBlocks: number;
		measuredBlocks: number;
		scrollPosition: number;
		fps: number;
	}
</script>

<script lang="ts">
	import { tick, onMount, onDestroy } from 'svelte';
	import { IconGripVertical, IconTrash, IconCopy, IconChevronUp, IconChevronDown } from '$lib/icons';
	import BlockRenderer from './BlockRenderer.svelte';

	// ==========================================================================
	// Props
	// ==========================================================================

	let {
		blocks,
		selectedBlockId,
		onSelectBlock,
		onUpdateBlock,
		onDeleteBlock,
		onMoveBlock,
		isEditing
	}: VirtualBlockListProps = $props();

	// ==========================================================================
	// Constants
	// ==========================================================================

	const OVERSCAN_COUNT = 3; // Number of items to render outside viewport
	const DEFAULT_BLOCK_HEIGHT = 80; // Estimated height before measurement
	const MIN_BLOCK_HEIGHT = 40;
	const SCROLL_THRESHOLD = 100; // Distance from edge to trigger auto-scroll
	const AUTO_SCROLL_SPEED = 8;
	const MEASUREMENT_DEBOUNCE_MS = 50;
	const PERFORMANCE_LOG_INTERVAL_MS = 1000;

	// ==========================================================================
	// State
	// ==========================================================================

	// Container and scroll state
	let containerRef = $state<HTMLDivElement | undefined>(undefined);
	let scrollTop = $state(0);
	let containerHeight = $state(0);
	let isScrolling = $state(false);
	let scrollTimeout: ReturnType<typeof setTimeout> | null = null;

	// Block measurements - Map of block ID to measured height
	let blockMeasurements = $state<Map<string, BlockMeasurement>>(new Map());
	let blockRefs = $state<Map<string, HTMLDivElement>>(new Map());

	// Drag-and-drop state
	let isDragging = $state(false);
	let draggedBlockId = $state<string | null>(null);
	let draggedBlockIndex = $state<number | null>(null);
	let dropTargetIndex = $state<number | null>(null);
	let dragStartY = $state(0);
	let dragCurrentY = $state(0);
	let autoScrollDirection = $state<'up' | 'down' | null>(null);
	let autoScrollRaf: number | null = null;

	// Performance tracking
	let lastRenderTime = $state(0);
	let frameCount = $state(0);
	let lastFpsUpdate = $state(0);
	let currentFps = $state(60);

	// ResizeObserver for block height changes
	let resizeObserver: ResizeObserver | null = null;

	// ==========================================================================
	// Derived State
	// ==========================================================================

	// Calculate cumulative heights for positioning
	let blockPositions = $derived.by(() => {
		const positions: { top: number; height: number }[] = [];
		let currentTop = 0;

		for (const block of blocks) {
			const measurement = blockMeasurements.get(block.id);
			const height = measurement?.measured ? measurement.height : DEFAULT_BLOCK_HEIGHT;
			positions.push({ top: currentTop, height });
			currentTop += height;
		}

		return positions;
	});

	// Total content height
	let totalHeight = $derived(
		blockPositions.length > 0
			? blockPositions[blockPositions.length - 1].top +
					blockPositions[blockPositions.length - 1].height
			: 0
	);

	// Calculate visible range based on scroll position
	let visibleRange = $derived.by((): VisibleRange => {
		if (blocks.length === 0) {
			return { start: 0, end: 0 };
		}

		const viewportTop = scrollTop;
		const viewportBottom = scrollTop + containerHeight;

		// Binary search for start index
		let startIndex = 0;
		let low = 0;
		let high = blocks.length - 1;

		while (low <= high) {
			const mid = Math.floor((low + high) / 2);
			const blockBottom = blockPositions[mid].top + blockPositions[mid].height;

			if (blockBottom < viewportTop) {
				low = mid + 1;
			} else {
				startIndex = mid;
				high = mid - 1;
			}
		}

		// Find end index
		let endIndex = startIndex;
		for (let i = startIndex; i < blocks.length; i++) {
			if (blockPositions[i].top > viewportBottom) {
				break;
			}
			endIndex = i;
		}

		// Apply overscan
		const overscanStart = Math.max(0, startIndex - OVERSCAN_COUNT);
		const overscanEnd = Math.min(blocks.length - 1, endIndex + OVERSCAN_COUNT);

		return { start: overscanStart, end: overscanEnd };
	});

	// Blocks to render
	let visibleBlocks = $derived(
		blocks.slice(visibleRange.start, visibleRange.end + 1).map((block, index) => ({
			block,
			index: visibleRange.start + index,
			position: blockPositions[visibleRange.start + index]
		}))
	);

	// Performance metrics
	let performanceMetrics = $derived<PerformanceMetrics>({
		renderTime: lastRenderTime,
		visibleCount: visibleRange.end - visibleRange.start + 1,
		totalBlocks: blocks.length,
		measuredBlocks: Array.from(blockMeasurements.values()).filter((m) => m.measured).length,
		scrollPosition: scrollTop,
		fps: currentFps
	});

	// ==========================================================================
	// Effects
	// ==========================================================================

	// Initialize ResizeObserver for measuring block heights
	$effect(() => {
		if (typeof window !== 'undefined') {
			resizeObserver = new ResizeObserver((entries) => {
				for (const entry of entries) {
					const blockId = (entry.target as HTMLElement).dataset.blockId;
					if (blockId) {
						const height = Math.max(entry.contentRect.height, MIN_BLOCK_HEIGHT);
						const current = blockMeasurements.get(blockId);

						if (!current || Math.abs(current.height - height) > 1) {
							blockMeasurements.set(blockId, { height, measured: true });
							blockMeasurements = new Map(blockMeasurements);
						}
					}
				}
			});
		}

		return () => {
			resizeObserver?.disconnect();
		};
	});

	// Note: Block observation is now handled by the measureBlock action
	// which automatically observes/unobserves blocks as they enter/leave the DOM

	// Track container size
	$effect(() => {
		if (!containerRef) return;

		const observer = new ResizeObserver((entries) => {
			for (const entry of entries) {
				containerHeight = entry.contentRect.height;
			}
		});

		observer.observe(containerRef);

		return () => observer.disconnect();
	});

	// Auto-scroll during drag
	$effect(() => {
		if (isDragging && autoScrollDirection && containerRef) {
			const scroll = () => {
				if (!containerRef || !isDragging) {
					autoScrollRaf = null;
					return;
				}

				const delta = autoScrollDirection === 'up' ? -AUTO_SCROLL_SPEED : AUTO_SCROLL_SPEED;
				containerRef.scrollTop += delta;

				autoScrollRaf = requestAnimationFrame(scroll);
			};

			autoScrollRaf = requestAnimationFrame(scroll);
		}

		return () => {
			if (autoScrollRaf) {
				cancelAnimationFrame(autoScrollRaf);
				autoScrollRaf = null;
			}
		};
	});

	// Performance logging
	$effect(() => {
		if (typeof window === 'undefined') return;

		let logInterval: ReturnType<typeof setInterval>;

		if (import.meta.env.DEV) {
			logInterval = setInterval(() => {
				console.log('[VirtualBlockList Performance]', {
					...performanceMetrics,
					timestamp: new Date().toISOString()
				});
			}, PERFORMANCE_LOG_INTERVAL_MS);
		}

		return () => {
			if (logInterval) clearInterval(logInterval);
		};
	});

	// FPS tracking
	$effect(() => {
		if (typeof window === 'undefined') return;

		let rafId: number;
		let lastTime = performance.now();
		let frames = 0;

		const trackFps = (currentTime: number) => {
			frames++;

			if (currentTime - lastTime >= 1000) {
				currentFps = Math.round((frames * 1000) / (currentTime - lastTime));
				frames = 0;
				lastTime = currentTime;
			}

			rafId = requestAnimationFrame(trackFps);
		};

		rafId = requestAnimationFrame(trackFps);

		return () => cancelAnimationFrame(rafId);
	});

	// ==========================================================================
	// Scroll Handling
	// ==========================================================================

	function handleScroll(event: Event) {
		const target = event.target as HTMLDivElement;
		const startTime = performance.now();

		scrollTop = target.scrollTop;

		// Track render time
		tick().then(() => {
			lastRenderTime = performance.now() - startTime;
		});

		// Set scrolling state for momentum styling
		isScrolling = true;

		if (scrollTimeout) {
			clearTimeout(scrollTimeout);
		}

		scrollTimeout = setTimeout(() => {
			isScrolling = false;
		}, 150);
	}

	// ==========================================================================
	// Block Ref Management via Svelte Action
	// ==========================================================================

	function measureBlock(element: HTMLDivElement, blockId: string) {
		// Store the reference
		blockRefs.set(blockId, element);
		blockRefs = new Map(blockRefs);

		// Observe for height changes
		if (resizeObserver) {
			resizeObserver.observe(element);
		}

		// Return cleanup function
		return {
			destroy() {
				if (resizeObserver) {
					resizeObserver.unobserve(element);
				}
				blockRefs.delete(blockId);
				blockRefs = new Map(blockRefs);
			},
			update(newBlockId: string) {
				// Handle block ID changes (rare but possible)
				if (newBlockId !== blockId) {
					blockRefs.delete(blockId);
					blockRefs.set(newBlockId, element);
					blockRefs = new Map(blockRefs);
					blockId = newBlockId;
				}
			}
		};
	}

	// ==========================================================================
	// Public Methods (exposed via bind:this)
	// ==========================================================================

	export function scrollToBlock(id: string): void {
		const blockIndex = blocks.findIndex((b) => b.id === id);
		if (blockIndex === -1 || !containerRef) return;

		const position = blockPositions[blockIndex];
		if (!position) return;

		// Calculate target scroll position to center block in viewport
		const targetScrollTop = position.top - containerHeight / 2 + position.height / 2;
		const clampedScrollTop = Math.max(0, Math.min(targetScrollTop, totalHeight - containerHeight));

		// Smooth scroll
		containerRef.scrollTo({
			top: clampedScrollTop,
			behavior: 'smooth'
		});
	}

	export function getVisibleRange(): VisibleRange {
		return { ...visibleRange };
	}

	export function getPerformanceMetrics(): PerformanceMetrics {
		return { ...performanceMetrics };
	}

	// ==========================================================================
	// Keyboard Navigation
	// ==========================================================================

	function handleKeyDown(event: KeyboardEvent) {
		if (!selectedBlockId || !isEditing) return;

		const currentIndex = blocks.findIndex((b) => b.id === selectedBlockId);
		if (currentIndex === -1) return;

		let newIndex: number | null = null;

		switch (event.key) {
			case 'ArrowUp':
				if (currentIndex > 0) {
					event.preventDefault();
					newIndex = currentIndex - 1;
				}
				break;
			case 'ArrowDown':
				if (currentIndex < blocks.length - 1) {
					event.preventDefault();
					newIndex = currentIndex + 1;
				}
				break;
			case 'Home':
				if (event.ctrlKey || event.metaKey) {
					event.preventDefault();
					newIndex = 0;
				}
				break;
			case 'End':
				if (event.ctrlKey || event.metaKey) {
					event.preventDefault();
					newIndex = blocks.length - 1;
				}
				break;
			case 'Delete':
			case 'Backspace':
				if (event.ctrlKey || event.metaKey) {
					event.preventDefault();
					onDeleteBlock(selectedBlockId);
				}
				break;
		}

		if (newIndex !== null) {
			const newBlock = blocks[newIndex];
			onSelectBlock(newBlock.id);

			// Scroll to bring block into view
			tick().then(() => {
				scrollToBlock(newBlock.id);
			});
		}
	}

	// ==========================================================================
	// Drag and Drop
	// ==========================================================================

	function handleDragStart(event: DragEvent, blockId: string, blockIndex: number) {
		if (!event.dataTransfer || !isEditing) return;

		isDragging = true;
		draggedBlockId = blockId;
		draggedBlockIndex = blockIndex;
		dragStartY = event.clientY;
		dragCurrentY = event.clientY;

		// Set drag data
		event.dataTransfer.effectAllowed = 'move';
		event.dataTransfer.setData('text/plain', blockId);

		// Create custom drag image
		const dragElement = event.target as HTMLElement;
		const rect = dragElement.getBoundingClientRect();
		event.dataTransfer.setDragImage(dragElement, rect.width / 2, 20);

		// Select the dragged block
		onSelectBlock(blockId);
	}

	function handleDragOver(event: DragEvent, targetIndex: number) {
		event.preventDefault();
		if (!event.dataTransfer || draggedBlockIndex === null) return;

		event.dataTransfer.dropEffect = 'move';
		dragCurrentY = event.clientY;

		// Calculate drop position
		const blockElement = (event.target as HTMLElement).closest('[data-block-index]');
		if (!blockElement) return;

		const rect = blockElement.getBoundingClientRect();
		const midpoint = rect.top + rect.height / 2;

		// Determine if dropping before or after
		if (event.clientY < midpoint) {
			dropTargetIndex = targetIndex;
		} else {
			dropTargetIndex = targetIndex + 1;
		}

		// Handle auto-scroll near edges
		if (containerRef) {
			const containerRect = containerRef.getBoundingClientRect();
			const distanceFromTop = event.clientY - containerRect.top;
			const distanceFromBottom = containerRect.bottom - event.clientY;

			if (distanceFromTop < SCROLL_THRESHOLD) {
				autoScrollDirection = 'up';
			} else if (distanceFromBottom < SCROLL_THRESHOLD) {
				autoScrollDirection = 'down';
			} else {
				autoScrollDirection = null;
			}
		}
	}

	function handleDragEnd() {
		isDragging = false;
		draggedBlockId = null;
		draggedBlockIndex = null;
		dropTargetIndex = null;
		autoScrollDirection = null;
	}

	function handleDrop(event: DragEvent) {
		event.preventDefault();

		if (draggedBlockIndex === null || dropTargetIndex === null) {
			handleDragEnd();
			return;
		}

		// Adjust target index if moving down
		let targetIndex = dropTargetIndex;
		if (draggedBlockIndex < targetIndex) {
			targetIndex--;
		}

		// Only move if position changed
		if (draggedBlockIndex !== targetIndex) {
			onMoveBlock(draggedBlockIndex, targetIndex);
		}

		handleDragEnd();
	}

	// ==========================================================================
	// Touch Drag Support
	// ==========================================================================

	let touchLongPressTimer: ReturnType<typeof setTimeout> | null = null;
	let isTouchDragging = $state(false);

	function handleTouchStart(event: TouchEvent, blockId: string, blockIndex: number) {
		if (!isEditing) return;

		const touch = event.touches[0];
		dragStartY = touch.clientY;
		dragCurrentY = touch.clientY;

		// Long press to initiate drag
		touchLongPressTimer = setTimeout(() => {
			isTouchDragging = true;
			isDragging = true;
			draggedBlockId = blockId;
			draggedBlockIndex = blockIndex;
			onSelectBlock(blockId);

			// Haptic feedback if available
			if (navigator.vibrate) {
				navigator.vibrate(50);
			}
		}, 500);
	}

	function handleTouchMove(event: TouchEvent) {
		if (!isTouchDragging || draggedBlockIndex === null) {
			// Cancel long press if moved too far
			if (touchLongPressTimer) {
				const touch = event.touches[0];
				const distance = Math.abs(touch.clientY - dragStartY);
				if (distance > 10) {
					clearTimeout(touchLongPressTimer);
					touchLongPressTimer = null;
				}
			}
			return;
		}

		event.preventDefault();
		const touch = event.touches[0];
		dragCurrentY = touch.clientY;

		// Find drop target
		const elements = document.elementsFromPoint(touch.clientX, touch.clientY);
		for (const element of elements) {
			const blockElement = (element as HTMLElement).closest('[data-block-index]');
			if (blockElement) {
				const index = parseInt(blockElement.getAttribute('data-block-index') || '0', 10);
				const rect = blockElement.getBoundingClientRect();
				const midpoint = rect.top + rect.height / 2;

				dropTargetIndex = touch.clientY < midpoint ? index : index + 1;
				break;
			}
		}

		// Auto-scroll
		if (containerRef) {
			const containerRect = containerRef.getBoundingClientRect();
			const distanceFromTop = touch.clientY - containerRect.top;
			const distanceFromBottom = containerRect.bottom - touch.clientY;

			if (distanceFromTop < SCROLL_THRESHOLD) {
				autoScrollDirection = 'up';
			} else if (distanceFromBottom < SCROLL_THRESHOLD) {
				autoScrollDirection = 'down';
			} else {
				autoScrollDirection = null;
			}
		}
	}

	function handleTouchEnd() {
		if (touchLongPressTimer) {
			clearTimeout(touchLongPressTimer);
			touchLongPressTimer = null;
		}

		if (isTouchDragging && draggedBlockIndex !== null && dropTargetIndex !== null) {
			let targetIndex = dropTargetIndex;
			if (draggedBlockIndex < targetIndex) {
				targetIndex--;
			}

			if (draggedBlockIndex !== targetIndex) {
				onMoveBlock(draggedBlockIndex, targetIndex);
			}
		}

		isTouchDragging = false;
		handleDragEnd();
	}

	// ==========================================================================
	// Block Actions
	// ==========================================================================

	function handleBlockClick(blockId: string) {
		onSelectBlock(blockId);
	}

	function handleBlockUpdate(blockId: string, updates: Partial<Block>) {
		onUpdateBlock(blockId, updates);
	}

	function handleMoveUp(blockIndex: number) {
		if (blockIndex > 0) {
			onMoveBlock(blockIndex, blockIndex - 1);
		}
	}

	function handleMoveDown(blockIndex: number) {
		if (blockIndex < blocks.length - 1) {
			onMoveBlock(blockIndex, blockIndex + 1);
		}
	}

	function handleDuplicate(blockId: string) {
		const block = blocks.find((b) => b.id === blockId);
		if (!block) return;

		// Emit a custom event or callback for duplication
		// This would need to be added to props if needed
		console.log('[VirtualBlockList] Duplicate block:', blockId);
	}

	// ==========================================================================
	// Lifecycle
	// ==========================================================================

	onMount(() => {
		// Log initial performance
		console.log('[VirtualBlockList] Mounted with', blocks.length, 'blocks');
	});

	onDestroy(() => {
		if (scrollTimeout) {
			clearTimeout(scrollTimeout);
		}
		if (autoScrollRaf) {
			cancelAnimationFrame(autoScrollRaf);
		}
		if (touchLongPressTimer) {
			clearTimeout(touchLongPressTimer);
		}
	});
</script>

<!-- svelte-ignore a11y_no_noninteractive_tabindex a11y_no_noninteractive_element_interactions -->
<div
	bind:this={containerRef}
	class="virtual-block-list"
	class:is-scrolling={isScrolling}
	class:is-dragging={isDragging}
	onscroll={handleScroll}
	onkeydown={handleKeyDown}
	ondrop={handleDrop}
	ondragover={(e) => e.preventDefault()}
	ontouchmove={handleTouchMove}
	ontouchend={handleTouchEnd}
	role="listbox"
	aria-label="Block list with {blocks.length} blocks"
	aria-multiselectable="false"
	tabindex="0"
>
	<!-- Spacer for total content height -->
	<div class="virtual-spacer" style:height="{totalHeight}px">
		<!-- Render only visible blocks -->
		{#each visibleBlocks as { block, index, position } (block.id)}
			<!-- svelte-ignore a11y_click_events_have_key_events a11y_no_noninteractive_element_interactions -->
			<div
				class="virtual-block-wrapper"
				class:is-selected={selectedBlockId === block.id}
				class:is-dragging={draggedBlockId === block.id}
				class:is-drop-target={dropTargetIndex === index}
				class:is-drop-after={dropTargetIndex === index + 1}
				style:transform="translateY({position.top}px)"
				data-block-id={block.id}
				data-block-index={index}
				role="option"
				aria-selected={selectedBlockId === block.id}
				tabindex={selectedBlockId === block.id ? 0 : -1}
				onclick={() => handleBlockClick(block.id)}
				ondragstart={(e) => handleDragStart(e, block.id, index)}
				ondragover={(e) => handleDragOver(e, index)}
				ondragend={handleDragEnd}
				ontouchstart={(e) => handleTouchStart(e, block.id, index)}
				draggable={isEditing}
			>
				<!-- Block Toolbar -->
				{#if isEditing && selectedBlockId === block.id}
					<div class="block-toolbar" role="toolbar" aria-label="Block actions">
						<button
							type="button"
							class="toolbar-btn drag-handle"
							aria-label="Drag to reorder"
							title="Drag to reorder"
						>
							<IconGripVertical size={16} />
						</button>

						<div class="toolbar-divider"></div>

						<button
							type="button"
							class="toolbar-btn"
							onclick={(e) => {
								e.stopPropagation();
								handleMoveUp(index);
							}}
							disabled={index === 0}
							aria-label="Move up"
							title="Move up"
						>
							<IconChevronUp size={16} />
						</button>

						<button
							type="button"
							class="toolbar-btn"
							onclick={(e) => {
								e.stopPropagation();
								handleMoveDown(index);
							}}
							disabled={index === blocks.length - 1}
							aria-label="Move down"
							title="Move down"
						>
							<IconChevronDown size={16} />
						</button>

						<div class="toolbar-divider"></div>

						<button
							type="button"
							class="toolbar-btn"
							onclick={(e) => {
								e.stopPropagation();
								handleDuplicate(block.id);
							}}
							aria-label="Duplicate block"
							title="Duplicate"
						>
							<IconCopy size={16} />
						</button>

						<button
							type="button"
							class="toolbar-btn toolbar-btn-danger"
							onclick={(e) => {
								e.stopPropagation();
								onDeleteBlock(block.id);
							}}
							aria-label="Delete block"
							title="Delete"
						>
							<IconTrash size={16} />
						</button>
					</div>
				{/if}

				<!-- Block Content -->
				<div
					class="block-content"
					use:measureBlock={block.id}
				>
					<BlockRenderer
						{block}
						isSelected={selectedBlockId === block.id}
						{isEditing}
						onUpdate={(updates) => handleBlockUpdate(block.id, updates)}
					/>
				</div>

				<!-- Drop indicator -->
				{#if isDragging && dropTargetIndex === index}
					<div class="drop-indicator drop-indicator-before"></div>
				{/if}
				{#if isDragging && dropTargetIndex === index + 1}
					<div class="drop-indicator drop-indicator-after"></div>
				{/if}
			</div>
		{/each}
	</div>

	<!-- Empty state -->
	{#if blocks.length === 0}
		<div class="empty-state">
			<p>No blocks yet. Click the + button to add your first block.</p>
		</div>
	{/if}

	<!-- Performance debug overlay (dev only) -->
	{#if import.meta.env.DEV}
		<div class="performance-overlay">
			<div class="perf-stat">
				<span class="perf-label">Visible:</span>
				<span class="perf-value">{performanceMetrics.visibleCount}/{performanceMetrics.totalBlocks}</span>
			</div>
			<div class="perf-stat">
				<span class="perf-label">Render:</span>
				<span class="perf-value">{performanceMetrics.renderTime.toFixed(1)}ms</span>
			</div>
			<div class="perf-stat">
				<span class="perf-label">FPS:</span>
				<span class="perf-value" class:perf-warning={performanceMetrics.fps < 30}>{performanceMetrics.fps}</span>
			</div>
			<div class="perf-stat">
				<span class="perf-label">Measured:</span>
				<span class="perf-value">{performanceMetrics.measuredBlocks}</span>
			</div>
		</div>
	{/if}
</div>

<style>
	.virtual-block-list {
		position: relative;
		height: 100%;
		overflow-y: auto;
		overflow-x: hidden;
		scroll-behavior: smooth;
		-webkit-overflow-scrolling: touch;
		outline: none;
	}

	/* Momentum scrolling enhancement */
	.virtual-block-list.is-scrolling {
		scroll-behavior: auto;
	}

	/* Disable pointer events during drag for better performance */
	.virtual-block-list.is-dragging .block-content {
		pointer-events: none;
	}

	.virtual-spacer {
		position: relative;
		width: 100%;
	}

	.virtual-block-wrapper {
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		width: 100%;
		padding: 0.5rem 1rem;
		transition:
			box-shadow 0.2s ease,
			opacity 0.2s ease;
		will-change: transform;
		contain: layout style;
	}

	.virtual-block-wrapper:hover {
		background-color: rgba(59, 130, 246, 0.02);
	}

	.virtual-block-wrapper.is-selected {
		background-color: rgba(59, 130, 246, 0.05);
		box-shadow: inset 0 0 0 2px rgba(59, 130, 246, 0.3);
		border-radius: 8px;
	}

	.virtual-block-wrapper.is-dragging {
		opacity: 0.5;
		transform: scale(0.98);
	}

	.virtual-block-wrapper.is-drop-target,
	.virtual-block-wrapper.is-drop-after {
		position: relative;
	}

	/* Block toolbar */
	.block-toolbar {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		position: absolute;
		top: 0.5rem;
		left: -3rem;
		padding: 0.25rem;
		background: white;
		border: 1px solid #e5e7eb;
		border-radius: 8px;
		box-shadow:
			0 4px 6px -1px rgba(0, 0, 0, 0.1),
			0 2px 4px -2px rgba(0, 0, 0, 0.1);
		z-index: 10;
		opacity: 0;
		transform: translateX(-0.5rem);
		transition:
			opacity 0.15s ease,
			transform 0.15s ease;
	}

	.virtual-block-wrapper.is-selected .block-toolbar {
		opacity: 1;
		transform: translateX(0);
	}

	.toolbar-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 28px;
		height: 28px;
		padding: 0;
		background: transparent;
		border: none;
		border-radius: 4px;
		color: #6b7280;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.toolbar-btn:hover:not(:disabled) {
		background: #f3f4f6;
		color: #1f2937;
	}

	.toolbar-btn:disabled {
		opacity: 0.4;
		cursor: not-allowed;
	}

	.toolbar-btn-danger:hover:not(:disabled) {
		background: #fef2f2;
		color: #dc2626;
	}

	.toolbar-divider {
		width: 1px;
		height: 20px;
		background: #e5e7eb;
		margin: 0 0.25rem;
	}

	.drag-handle {
		cursor: grab;
	}

	.drag-handle:active {
		cursor: grabbing;
	}

	/* Block content */
	.block-content {
		position: relative;
		min-height: 2rem;
	}

	/* Drop indicators */
	.drop-indicator {
		position: absolute;
		left: 0;
		right: 0;
		height: 3px;
		background: linear-gradient(90deg, #3b82f6 0%, #60a5fa 50%, #3b82f6 100%);
		border-radius: 2px;
		z-index: 20;
		animation: pulse-drop 1s ease-in-out infinite;
	}

	.drop-indicator-before {
		top: 0;
		transform: translateY(-50%);
	}

	.drop-indicator-after {
		bottom: 0;
		transform: translateY(50%);
	}

	@keyframes pulse-drop {
		0%,
		100% {
			opacity: 1;
			transform: scaleX(1) translateY(-50%);
		}
		50% {
			opacity: 0.7;
			transform: scaleX(0.98) translateY(-50%);
		}
	}

	.drop-indicator-after {
		animation-name: pulse-drop-after;
	}

	@keyframes pulse-drop-after {
		0%,
		100% {
			opacity: 1;
			transform: scaleX(1) translateY(50%);
		}
		50% {
			opacity: 0.7;
			transform: scaleX(0.98) translateY(50%);
		}
	}

	/* Empty state */
	.empty-state {
		display: flex;
		align-items: center;
		justify-content: center;
		height: 200px;
		color: #9ca3af;
		text-align: center;
		font-size: 0.875rem;
	}

	/* Performance overlay */
	.performance-overlay {
		position: fixed;
		bottom: 1rem;
		right: 1rem;
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
		padding: 0.75rem;
		background: rgba(0, 0, 0, 0.85);
		border-radius: 8px;
		font-family: 'SF Mono', 'Fira Code', monospace;
		font-size: 0.625rem;
		color: #fff;
		z-index: 9999;
		backdrop-filter: blur(8px);
	}

	.perf-stat {
		display: flex;
		justify-content: space-between;
		gap: 1rem;
	}

	.perf-label {
		color: #9ca3af;
	}

	.perf-value {
		font-weight: 600;
		color: #10b981;
	}

	.perf-value.perf-warning {
		color: #f59e0b;
	}

	/* Scrollbar styling */
	.virtual-block-list::-webkit-scrollbar {
		width: 8px;
	}

	.virtual-block-list::-webkit-scrollbar-track {
		background: transparent;
	}

	.virtual-block-list::-webkit-scrollbar-thumb {
		background: #d1d5db;
		border-radius: 4px;
	}

	.virtual-block-list::-webkit-scrollbar-thumb:hover {
		background: #9ca3af;
	}

	/* Touch drag visual feedback */
	@media (hover: none) and (pointer: coarse) {
		.block-toolbar {
			position: static;
			opacity: 1;
			transform: none;
			margin-bottom: 0.5rem;
		}

		.virtual-block-wrapper.is-selected .block-toolbar {
			display: flex;
		}

		.toolbar-btn {
			width: 36px;
			height: 36px;
		}
	}

	/* Reduced motion */
	@media (prefers-reduced-motion: reduce) {
		.virtual-block-list {
			scroll-behavior: auto;
		}

		.block-toolbar,
		.virtual-block-wrapper {
			transition: none;
		}

		.drop-indicator {
			animation: none;
		}
	}
</style>

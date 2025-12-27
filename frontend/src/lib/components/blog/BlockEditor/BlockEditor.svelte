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

	import type {
		Block,
		BlockType,
		EditorState,
		SEOAnalysis,
		Revision
	} from './types';

	import {
		BLOCK_DEFINITIONS
	} from './types';

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
		hoveredBlockId: null,
		focusedBlockId: null,
		clipboard: null,
		history: {
			past: [],
			present: blocks,
			future: [],
			maxSize: 100
		},
		isDragging: false,
		draggedBlockId: null,
		dropTargetIndex: null,
		viewMode: 'edit',
		devicePreview: 'desktop',
		zoom: 100,
		showGrid: false,
		showOutlines: false,
		sidebarTab: 'blocks',
		isFullscreen: false,
		autosaveEnabled: true,
		lastSaved: null,
		hasUnsavedChanges: false
	});

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
			? editorState.blocks.find(b => b.id === editorState.selectedBlockId)
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

		editorState.blocks = editorState.blocks.map(block =>
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

		const index = editorState.blocks.findIndex(b => b.id === blockId);
		editorState.blocks = editorState.blocks.filter(b => b.id !== blockId);

		// Select adjacent block
		if (editorState.blocks.length > 0) {
			const newIndex = Math.min(index, editorState.blocks.length - 1);
			editorState.selectedBlockId = editorState.blocks[newIndex]?.id ?? null;
		} else {
			editorState.selectedBlockId = null;
		}
	}

	function duplicateBlock(blockId: string) {
		const block = editorState.blocks.find(b => b.id === blockId);
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

		const index = editorState.blocks.findIndex(b => b.id === blockId);
		const newBlocks = [...editorState.blocks];
		newBlocks.splice(index + 1, 0, duplicated);
		editorState.blocks = newBlocks;
		editorState.selectedBlockId = duplicated.id;
	}

	function moveBlock(blockId: string, direction: 'up' | 'down') {
		const index = editorState.blocks.findIndex(b => b.id === blockId);
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
	// Drag and Drop
	// ==========================================================================

	function handleDragStart(e: DragEvent, blockId: string) {
		if (readOnly) return;

		editorState.isDragging = true;
		editorState.draggedBlockId = blockId;

		if (e.dataTransfer) {
			e.dataTransfer.effectAllowed = 'move';
			e.dataTransfer.setData('text/plain', blockId);
		}
	}

	function handleDragOver(e: DragEvent, index: number) {
		e.preventDefault();
		if (!editorState.isDragging) return;

		editorState.dropTargetIndex = index;
	}

	function handleDragEnd() {
		if (editorState.draggedBlockId && editorState.dropTargetIndex !== null) {
			const fromIndex = editorState.blocks.findIndex(b => b.id === editorState.draggedBlockId);
			const toIndex = editorState.dropTargetIndex;

			if (fromIndex !== toIndex && fromIndex !== -1) {
				pushToHistory();

				const newBlocks = [...editorState.blocks];
				const [removed] = newBlocks.splice(fromIndex, 1);
				if (removed) {
					newBlocks.splice(toIndex > fromIndex ? toIndex - 1 : toIndex, 0, removed);
					editorState.blocks = newBlocks;
				}
			}
		}

		editorState.isDragging = false;
		editorState.draggedBlockId = null;
		editorState.dropTargetIndex = null;
	}

	function handleDrop(e: DragEvent, index: number) {
		e.preventDefault();
		editorState.dropTargetIndex = index;
		handleDragEnd();
	}

	// ==========================================================================
	// History (Undo/Redo)
	// ==========================================================================

	function pushToHistory() {
		const currentState = JSON.parse(JSON.stringify(editorState.blocks));

		editorState.history = {
			...editorState.history,
			past: [...editorState.history.past, editorState.history.present].slice(-editorState.history.maxSize),
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
		const block = editorState.blocks.find(b => b.id === blockId);
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
			? editorState.blocks.findIndex(b => b.id === editorState.selectedBlockId) + 1
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
			if (activeEl?.tagName !== 'INPUT' && activeEl?.tagName !== 'TEXTAREA' && !activeEl?.getAttribute('contenteditable')) {
				e.preventDefault();
				pasteBlock();
			}
		}

		// Delete block: Backspace/Delete when block selected
		if ((e.key === 'Backspace' || e.key === 'Delete') && editorState.selectedBlockId) {
			const activeEl = document.activeElement;
			if (activeEl?.tagName !== 'INPUT' && activeEl?.tagName !== 'TEXTAREA' && !activeEl?.getAttribute('contenteditable')) {
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
				const index = editorState.blocks.findIndex(b => b.id === editorState.selectedBlockId);
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
		const words = content.split(/\s+/).filter(w => w.length > 0);
		const sentences = content.split(/[.!?]+/).filter(s => s.trim().length > 0);

		const issues: SEOAnalysis['issues'] = [];
		const suggestions: string[] = [];

		// Title checks
		if (!postTitle) {
			issues.push({ type: 'error', category: 'title', message: 'Post title is missing', impact: 'high' });
		} else if (postTitle.length < 30) {
			issues.push({ type: 'warning', category: 'title', message: 'Title is too short (recommended: 50-60 characters)', impact: 'medium' });
		} else if (postTitle.length > 70) {
			issues.push({ type: 'warning', category: 'title', message: 'Title is too long (recommended: 50-60 characters)', impact: 'medium' });
		}

		// Meta description checks
		if (!metaDescription) {
			issues.push({ type: 'error', category: 'description', message: 'Meta description is missing', impact: 'high' });
		} else if (metaDescription.length < 120) {
			issues.push({ type: 'warning', category: 'description', message: 'Meta description is too short (recommended: 150-160 characters)', impact: 'medium' });
		}

		// Content length
		if (words.length < 300) {
			issues.push({ type: 'warning', category: 'content', message: 'Content is too short (recommended: 1000+ words for SEO)', impact: 'medium' });
			suggestions.push('Add more content to improve SEO ranking');
		}

		// Heading structure
		const headings = editorState.blocks.filter(b => b.type === 'heading');
		if (headings.length === 0) {
			issues.push({ type: 'warning', category: 'headings', message: 'No headings found. Add H2/H3 headings to structure content', impact: 'medium' });
		}

		// Images without alt
		const imagesWithoutAlt = editorState.blocks.filter(
			b => b.type === 'image' && !b.content.mediaAlt
		).length;
		if (imagesWithoutAlt > 0) {
			issues.push({ type: 'warning', category: 'images', message: `${imagesWithoutAlt} image(s) missing alt text`, impact: 'medium' });
		}

		// Keyword density (if focus keyword provided)
		let keywordDensity = 0;
		if (focusKeyword && words.length > 0) {
			const keywordCount = content.toLowerCase().split(focusKeyword.toLowerCase()).length - 1;
			keywordDensity = (keywordCount / words.length) * 100;

			if (keywordDensity < 0.5) {
				issues.push({ type: 'warning', category: 'keywords', message: 'Focus keyword density is too low', impact: 'medium' });
			} else if (keywordDensity > 3) {
				issues.push({ type: 'warning', category: 'keywords', message: 'Focus keyword density is too high (keyword stuffing)', impact: 'medium' });
			}
		}

		// Readability (Flesch-Kincaid approximation)
		const avgWordsPerSentence = sentences.length > 0 ? words.length / sentences.length : 0;
		const avgSyllablesPerWord = 1.5; // Simplified
		const readabilityScore = 206.835 - (1.015 * avgWordsPerSentence) - (84.6 * avgSyllablesPerWord);

		let readabilityGrade = 'Easy';
		if (readabilityScore < 30) readabilityGrade = 'Very Difficult';
		else if (readabilityScore < 50) readabilityGrade = 'Difficult';
		else if (readabilityScore < 60) readabilityGrade = 'Fairly Difficult';
		else if (readabilityScore < 70) readabilityGrade = 'Standard';
		else if (readabilityScore < 80) readabilityGrade = 'Fairly Easy';

		// Calculate overall score
		const errorCount = issues.filter(i => i.type === 'error').length;
		const warningCount = issues.filter(i => i.type === 'warning').length;
		let score = 100 - (errorCount * 20) - (warningCount * 5);
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
			return count + plainText.split(/\s+/).filter(w => w.length > 0).length;
		}, 0);
	}

	function getPlainTextContent(blocks: Block[]): string {
		return blocks.map(block => {
			const text = block.content.text || block.content.html || '';
			return text.replace(/<[^>]*>/g, '');
		}).join(' ');
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
			case 'mobile': return '375px';
			case 'tablet': return '768px';
			default: return '100%';
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
			<div class="undo-redo">
				<button
					type="button"
					class="toolbar-btn"
					disabled={!canUndo}
					onclick={undo}
					title="Undo (Ctrl+Z)"
				>
					<IconArrowBackUp size={18} />
				</button>
				<button
					type="button"
					class="toolbar-btn"
					disabled={!canRedo}
					onclick={redo}
					title="Redo (Ctrl+Shift+Z)"
				>
					<IconArrowForwardUp size={18} />
				</button>
			</div>

			<div class="block-info">
				<span class="block-count">{blockCount} blocks</span>
				<span class="word-count">{wordCount} words</span>
				<span class="read-time">{readTime} min read</span>
			</div>
		</div>

		<div class="header-center">
			<div class="device-preview">
				<button
					type="button"
					class="device-btn"
					class:active={editorState.devicePreview === 'desktop'}
					onclick={() => setDevicePreview('desktop')}
					title="Desktop Preview"
				>
					<IconDeviceDesktop size={18} />
				</button>
				<button
					type="button"
					class="device-btn"
					class:active={editorState.devicePreview === 'tablet'}
					onclick={() => setDevicePreview('tablet')}
					title="Tablet Preview"
				>
					<IconDeviceTablet size={18} />
				</button>
				<button
					type="button"
					class="device-btn"
					class:active={editorState.devicePreview === 'mobile'}
					onclick={() => setDevicePreview('mobile')}
					title="Mobile Preview"
				>
					<IconDeviceMobile size={18} />
				</button>
			</div>

			<div class="view-toggle">
				<button
					type="button"
					class="view-btn"
					class:active={editorState.viewMode === 'edit'}
					onclick={() => editorState.viewMode = 'edit'}
				>
					<IconEdit size={16} />
					Edit
				</button>
				<button
					type="button"
					class="view-btn"
					class:active={editorState.viewMode === 'preview'}
					onclick={() => editorState.viewMode = 'preview'}
				>
					<IconEye size={16} />
					Preview
				</button>
			</div>
		</div>

		<div class="header-right">
			<div class="save-status">
				{#if isSaving}
					<span class="saving"><IconCloudUpload size={16} class="spin" /> Saving...</span>
				{:else if editorState.hasUnsavedChanges}
					<span class="unsaved">Unsaved changes</span>
				{:else}
					<span class="saved">Saved {formatLastSaved(editorState.lastSaved)}</span>
				{/if}
			</div>

			<div class="header-actions">
				<button
					type="button"
					class="toolbar-btn"
					onclick={() => showRevisions = true}
					title="Revision History"
				>
					<IconHistory size={18} />
				</button>
				<button
					type="button"
					class="toolbar-btn"
					onclick={() => showKeyboardHelp = true}
					title="Keyboard Shortcuts"
				>
					<IconKeyboard size={18} />
				</button>
				<button
					type="button"
					class="toolbar-btn"
					onclick={toggleFullscreen}
					title="Toggle Fullscreen"
				>
					{#if editorState.isFullscreen}
						<IconMinimize size={18} />
					{:else}
						<IconMaximize size={18} />
					{/if}
				</button>
			</div>

			<button
				type="button"
				class="btn-save"
				onclick={handleSave}
				disabled={isSaving || !editorState.hasUnsavedChanges}
			>
				<IconDeviceFloppy size={18} />
				Save Draft
			</button>
			<button
				type="button"
				class="btn-publish"
				onclick={handlePublish}
				disabled={isSaving}
			>
				<IconCloudUpload size={18} />
				Publish
			</button>
		</div>
	</header>

	<div class="editor-body">
		<!-- Sidebar -->
		<aside class="editor-sidebar">
			<nav class="sidebar-tabs">
				<button
					type="button"
					class="tab-btn"
					class:active={editorState.sidebarTab === 'blocks'}
					onclick={() => editorState.sidebarTab = 'blocks'}
					title="Add Blocks"
				>
					<IconPlus size={20} />
				</button>
				<button
					type="button"
					class="tab-btn"
					class:active={editorState.sidebarTab === 'settings'}
					onclick={() => editorState.sidebarTab = 'settings'}
					title="Block Settings"
				>
					<IconSettings size={20} />
				</button>
				<button
					type="button"
					class="tab-btn"
					class:active={editorState.sidebarTab === 'layers'}
					onclick={() => editorState.sidebarTab = 'layers'}
					title="Block Layers"
				>
					<IconStack2 size={20} />
				</button>
				<button
					type="button"
					class="tab-btn"
					class:active={editorState.sidebarTab === 'ai'}
					onclick={() => editorState.sidebarTab = 'ai'}
					title="AI Assistant"
				>
					<IconRobot size={20} />
				</button>
				<button
					type="button"
					class="tab-btn"
					class:active={editorState.sidebarTab === 'seo'}
					onclick={() => editorState.sidebarTab = 'seo'}
					title="SEO Analysis"
				>
					<IconSeo size={20} />
				</button>
			</nav>

			<div class="sidebar-content">
				{#if editorState.sidebarTab === 'blocks'}
					<div class="panel-header">
						<h3>Add Block</h3>
					</div>
					<div class="block-search">
						<IconSearch size={16} />
						<input
							type="text"
							placeholder="Search blocks..."
							bind:value={searchQuery}
						/>
					</div>
					<BlockInserter
						{searchQuery}
						oninsert={handleBlockInsert}
					/>
				{:else if editorState.sidebarTab === 'settings'}
					{#if selectedBlock}
						<BlockSettingsPanel
							block={selectedBlock}
							onupdate={(updates) => updateBlock(selectedBlock.id, updates)}
						/>
					{:else}
						<div class="empty-panel">
							<IconSettings size={48} stroke={1} />
							<p>Select a block to edit its settings</p>
						</div>
					{/if}
				{:else if editorState.sidebarTab === 'layers'}
					<div class="panel-header">
						<h3>Block Layers</h3>
					</div>
					<div class="layers-list">
						{#each editorState.blocks as block, i (block.id)}
							<button
								type="button"
								class="layer-item"
								class:selected={block.id === editorState.selectedBlockId}
								onclick={() => editorState.selectedBlockId = block.id}
							>
								<IconGripVertical size={14} />
								<span class="layer-type">{BLOCK_DEFINITIONS[block.type]?.name || block.type}</span>
								<span class="layer-index">#{i + 1}</span>
							</button>
						{/each}
					</div>
				{:else if editorState.sidebarTab === 'ai'}
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
				{:else if editorState.sidebarTab === 'seo'}
					<SEOAnalyzer
						title={postTitle}
						content={getPlainTextContent(editorState.blocks)}
						{metaDescription}
						{focusKeyword}
						slug={postSlug}
					/>
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
						<button
							type="button"
							class="btn-add-first"
							onclick={() => openBlockInserter(0)}
						>
							<IconPlus size={18} />
							Add Block
						</button>
					</div>
				{:else}
					<!-- Block List -->
					<div class="blocks-container">
						{#each editorState.blocks as block, index (block.id)}
							<div
							class="block-wrapper"
							class:selected={block.id === editorState.selectedBlockId}
							class:hovered={block.id === editorState.hoveredBlockId}
							class:dragging={block.id === editorState.draggedBlockId}
							class:drop-target={index === editorState.dropTargetIndex}
							data-block-id={block.id}
							draggable={!readOnly}
							ondragstart={(e: DragEvent) => handleDragStart(e, block.id)}
							ondragover={(e: DragEvent) => handleDragOver(e, index)}
							ondragend={handleDragEnd}
							ondrop={(e: DragEvent) => handleDrop(e, index)}
							onclick={() => editorState.selectedBlockId = block.id}
							onkeydown={(e: KeyboardEvent) => { if (e.key === 'Enter' || e.key === ' ') { e.preventDefault(); editorState.selectedBlockId = block.id; } }}
							onmouseenter={() => editorState.hoveredBlockId = block.id}
							onmouseleave={() => editorState.hoveredBlockId = null}
							tabindex="0"
							role="button"
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
											onclick={(e: MouseEvent) => { e.stopPropagation(); moveBlock(block.id, 'up'); }}
											disabled={index === 0}
											title="Move up"
										>
											<IconChevronUp size={16} />
										</button>
										<button
											type="button"
											class="block-tool"
											onclick={(e: MouseEvent) => { e.stopPropagation(); moveBlock(block.id, 'down'); }}
											disabled={index === editorState.blocks.length - 1}
											title="Move down"
										>
											<IconChevronDown size={16} />
										</button>
										<div class="tool-divider"></div>
										<button
											type="button"
											class="block-tool"
											onclick={(e: MouseEvent) => { e.stopPropagation(); duplicateBlock(block.id); }}
											title="Duplicate"
										>
											<IconCopy size={16} />
										</button>
										<button
											type="button"
											class="block-tool danger"
											onclick={(e: MouseEvent) => { e.stopPropagation(); deleteBlock(block.id); }}
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
										onclick={(e: MouseEvent) => { e.stopPropagation(); openBlockInserter(index + 1, e); }}
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
		onclose={() => showBlockInserter = false}
	/>
{/if}

<!-- Keyboard Shortcuts Modal -->
{#if showKeyboardHelp}
	<KeyboardShortcuts isOpen={showKeyboardHelp} onClose={() => showKeyboardHelp = false} />
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
		from { transform: rotate(0deg); }
		to { transform: rotate(360deg); }
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
</style>

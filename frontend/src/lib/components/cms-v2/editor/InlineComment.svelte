<!--
  CMS Inline Comment Component
  ============================
  Enterprise-grade inline commenting for CMS editor blocks.

  Features:
  - Highlight text selections with comments
  - Floating "Add comment" button on text selection
  - Click highlight to scroll to comment in sidebar
  - Comments attached to specific block + text range
  - Smooth animations and accessible

  @version 2.0.0 - January 2026
  @author Revolution Trading Pros
-->
<script lang="ts">
	import { onMount, onDestroy, tick } from 'svelte';
	import {
		commentsStore,
		type CmsComment,
		type TextSelection,
		type CmsUser
	} from '$lib/stores/comments.svelte';

	// ==========================================================================
	// Props
	// ==========================================================================

	interface Props {
		blockId: string;
		contentId: string;
		blockElement?: HTMLElement | null;
		onCommentCreated?: (comment: CmsComment) => void;
		onCommentClicked?: (comment: CmsComment) => void;
	}

	let { blockId, contentId, blockElement = null, onCommentCreated, onCommentClicked }: Props = $props();

	// ==========================================================================
	// Local State
	// ==========================================================================

	let showAddButton = $state(false);
	let showCommentPopover = $state(false);
	let buttonPosition = $state({ top: 0, left: 0 });
	let popoverPosition = $state({ top: 0, left: 0 });
	let currentSelection = $state<TextSelection | null>(null);
	let commentBody = $state('');
	let activeHighlight = $state<string | null>(null);
	let mentionCursorPosition = $state(0);
	let textareaRef = $state<HTMLTextAreaElement | null>(null);

	// Derived state from store
	let inlineComments = $derived(commentsStore.getInlineCommentsForBlock(blockId));
	let isSaving = $derived(commentsStore.isSaving);
	let users = $derived(commentsStore.users);
	let mentionSuggestions = $derived(commentsStore.mentionSuggestions);
	let showMentionDropdown = $derived(commentsStore.showMentionDropdown);

	// ==========================================================================
	// Highlight Markers
	// ==========================================================================

	// Create highlight overlays for existing comments
	let highlightMarkers = $derived.by(() => {
		if (!blockElement) return [];

		return inlineComments.map((comment) => ({
			id: comment.id,
			comment,
			start: comment.selection_start!,
			end: comment.selection_end!,
			isResolved: comment.is_resolved
		}));
	});

	// ==========================================================================
	// Lifecycle
	// ==========================================================================

	let observer = $state<MutationObserver | null>(null);

	onMount(() => {
		if (blockElement) {
			setupSelectionListener();
			applyHighlights();
		}
	});

	onDestroy(() => {
		observer?.disconnect();
		removeSelectionListener();
	});

	// Watch for block element changes
	$effect(() => {
		if (blockElement) {
			applyHighlights();
		}
	});

	// ==========================================================================
	// Selection Handling
	// ==========================================================================

	function setupSelectionListener() {
		document.addEventListener('mouseup', handleMouseUp);
		document.addEventListener('keyup', handleKeyUp);
	}

	function removeSelectionListener() {
		document.removeEventListener('mouseup', handleMouseUp);
		document.removeEventListener('keyup', handleKeyUp);
	}

	function handleMouseUp(event: MouseEvent) {
		// Small delay to ensure selection is complete
		setTimeout(() => {
			checkSelection(event);
		}, 10);
	}

	function handleKeyUp(event: KeyboardEvent) {
		if (event.shiftKey) {
			checkSelection();
		}
	}

	function checkSelection(event?: MouseEvent | null) {
		const selection = window.getSelection();

		if (!selection || selection.isCollapsed || !blockElement) {
			hideAddButton();
			return;
		}

		const range = selection.getRangeAt(0);

		// Check if selection is within our block
		if (!blockElement.contains(range.commonAncestorContainer)) {
			hideAddButton();
			return;
		}

		// Get text content and offsets
		const textContent = blockElement.textContent || '';
		const selectedText = selection.toString().trim();

		if (!selectedText) {
			hideAddButton();
			return;
		}

		// Calculate selection offsets relative to block
		const preSelectionRange = range.cloneRange();
		preSelectionRange.selectNodeContents(blockElement);
		preSelectionRange.setEnd(range.startContainer, range.startOffset);
		const start = preSelectionRange.toString().length;
		const end = start + selectedText.length;

		// Check if this selection overlaps with existing comment
		const existingComment = commentsStore.hasCommentInRange(blockId, start, end);
		if (existingComment) {
			// Click the existing highlight instead
			activeHighlight = existingComment.id;
			onCommentClicked?.(existingComment);
			hideAddButton();
			return;
		}

		// Store selection info
		currentSelection = {
			blockId,
			start,
			end,
			text: selectedText,
			rect: range.getBoundingClientRect()
		};

		// Position the add button
		const rect = range.getBoundingClientRect();
		const blockRect = blockElement.getBoundingClientRect();

		buttonPosition = {
			top: rect.top - blockRect.top - 40,
			left: rect.left - blockRect.left + rect.width / 2 - 20
		};

		showAddButton = true;
		commentsStore.setTextSelection(currentSelection);
	}

	function hideAddButton() {
		showAddButton = false;
		currentSelection = null;
		commentsStore.clearTextSelection();
	}

	// ==========================================================================
	// Comment Popover
	// ==========================================================================

	function openCommentPopover() {
		if (!currentSelection) return;

		showCommentPopover = true;
		showAddButton = false;
		commentBody = '';

		// Position popover
		popoverPosition = {
			top: buttonPosition.top + 50,
			left: Math.max(0, buttonPosition.left - 120)
		};

		// Focus textarea after render
		tick().then(() => {
			textareaRef?.focus();
		});
	}

	function closeCommentPopover() {
		showCommentPopover = false;
		commentBody = '';
		currentSelection = null;
		commentsStore.hideMentionDropdown();
		commentsStore.clearTextSelection();
	}

	async function submitComment() {
		if (!commentBody.trim() || !currentSelection) return;

		const comment = await commentsStore.createComment({
			content_id: contentId,
			block_id: blockId,
			selection_start: currentSelection.start,
			selection_end: currentSelection.end,
			body: commentBody
		});

		if (comment) {
			closeCommentPopover();
			onCommentCreated?.(comment);
			await tick();
			applyHighlights();
		}
	}

	// ==========================================================================
	// @Mention Handling
	// ==========================================================================

	function handleTextareaInput(event: Event) {
		const target = event.target as HTMLTextAreaElement;
		const value = target.value;
		const cursorPos = target.selectionStart;
		mentionCursorPosition = cursorPos;

		// Check for @ trigger
		const textBeforeCursor = value.substring(0, cursorPos);
		const mentionMatch = textBeforeCursor.match(/@(\w*)$/);

		if (mentionMatch) {
			commentsStore.setMentionQuery(mentionMatch[1]);
		} else {
			commentsStore.hideMentionDropdown();
		}
	}

	function insertMention(user: CmsUser) {
		const textBeforeCursor = commentBody.substring(0, mentionCursorPosition);
		const textAfterCursor = commentBody.substring(mentionCursorPosition);

		// Find the @ position
		const mentionStart = textBeforeCursor.lastIndexOf('@');
		commentBody =
			textBeforeCursor.substring(0, mentionStart) +
			commentsStore.formatMention(user) +
			' ' +
			textAfterCursor;

		commentsStore.hideMentionDropdown();

		// Focus back on textarea
		setTimeout(() => {
			textareaRef?.focus();
		}, 0);
	}

	// ==========================================================================
	// Highlight Rendering
	// ==========================================================================

	function applyHighlights() {
		if (!blockElement) return;

		// Remove existing highlights
		const existingHighlights = blockElement.querySelectorAll('.inline-comment-highlight');
		existingHighlights.forEach((el) => {
			const parent = el.parentNode;
			if (parent) {
				while (el.firstChild) {
					parent.insertBefore(el.firstChild, el);
				}
				parent.removeChild(el);
			}
		});

		// Apply new highlights
		for (const marker of highlightMarkers) {
			try {
				highlightRange(marker.start, marker.end, marker.id, marker.isResolved);
			} catch (e) {
				console.warn('[InlineComment] Failed to apply highlight:', e);
			}
		}
	}

	function highlightRange(start: number, end: number, commentId: string, isResolved: boolean) {
		if (!blockElement) return;

		const textContent = blockElement.textContent || '';
		if (start >= textContent.length || end > textContent.length) return;

		// Create tree walker to find text nodes
		const walker = document.createTreeWalker(blockElement, NodeFilter.SHOW_TEXT, null);

		let currentOffset = 0;
		let startNode: Text | null = null;
		let startOffset = 0;
		let endNode: Text | null = null;
		let endOffset = 0;

		let node: Text | null;
		while ((node = walker.nextNode() as Text | null)) {
			const nodeLength = node.length;

			if (!startNode && currentOffset + nodeLength > start) {
				startNode = node;
				startOffset = start - currentOffset;
			}

			if (currentOffset + nodeLength >= end) {
				endNode = node;
				endOffset = end - currentOffset;
				break;
			}

			currentOffset += nodeLength;
		}

		if (!startNode || !endNode) return;

		// Create range
		const range = document.createRange();
		range.setStart(startNode, startOffset);
		range.setEnd(endNode, endOffset);

		// Wrap with highlight span
		const highlight = document.createElement('span');
		highlight.className = `inline-comment-highlight ${isResolved ? 'resolved' : 'active'}`;
		highlight.dataset.commentId = commentId;
		highlight.setAttribute('role', 'button');
		highlight.setAttribute('tabindex', '0');
		highlight.setAttribute('aria-label', 'View comment');

		// Add click handler
		highlight.addEventListener('click', () => handleHighlightClick(commentId));
		highlight.addEventListener('keydown', (e) => {
			if (e.key === 'Enter' || e.key === ' ') {
				e.preventDefault();
				handleHighlightClick(commentId);
			}
		});

		try {
			range.surroundContents(highlight);
		} catch (e) {
			// If surroundContents fails (spans multiple elements), fall back to basic approach
			console.warn('[InlineComment] surroundContents failed, using fallback');
		}
	}

	function handleHighlightClick(commentId: string) {
		activeHighlight = commentId;
		const comment = commentsStore.getComment(commentId);
		if (comment) {
			commentsStore.setActiveComment(commentId);
			onCommentClicked?.(comment);
		}
	}

	// ==========================================================================
	// Utilities
	// ==========================================================================

	function getUserInitials(user?: CmsUser): string {
		if (!user?.display_name) return '?';
		return user.display_name
			.split(' ')
			.map((n) => n[0])
			.join('')
			.toUpperCase()
			.slice(0, 2);
	}
</script>

<!-- Add Comment Button (Floating) -->
{#if showAddButton}
	<div
		class="add-comment-button"
		style="top: {buttonPosition.top}px; left: {buttonPosition.left}px;"
		role="button"
		tabindex="0"
		aria-label="Add comment to selection"
	>
		<button
			type="button"
			onclick={openCommentPopover}
			aria-label="Add comment"
			class="flex items-center justify-center rounded-lg bg-blue-600 p-2 text-white shadow-lg
				transition-all hover:bg-blue-700 hover:shadow-xl focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
		>
			<svg class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="2"
					d="M7 8h10M7 12h4m1 8l-4-4H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-3l-4 4z"
				/>
			</svg>
		</button>
	</div>
{/if}

<!-- Comment Popover -->
{#if showCommentPopover}
	<!-- Backdrop -->
	<div
		class="comment-backdrop"
		onclick={closeCommentPopover}
		onkeydown={(e) => e.key === 'Escape' && closeCommentPopover()}
		role="button"
		tabindex="-1"
		aria-label="Close comment popover"
	></div>

	<div
		class="comment-popover"
		style="top: {popoverPosition.top}px; left: {popoverPosition.left}px;"
		role="dialog"
		aria-label="Add comment"
		aria-modal="true"
	>
		<div class="rounded-lg border border-gray-200 bg-white p-4 shadow-xl">
			<div class="mb-3 flex items-center justify-between">
				<h3 class="text-sm font-semibold text-gray-900">Add Comment</h3>
				<button
					type="button"
					onclick={closeCommentPopover}
					class="rounded p-1 text-gray-400 hover:bg-gray-100 hover:text-gray-600"
					aria-label="Close"
				>
					<svg class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M6 18L18 6M6 6l12 12"
						/>
					</svg>
				</button>
			</div>

			<!-- Selected Text Preview -->
			{#if currentSelection}
				<div class="mb-3 rounded border border-gray-200 bg-gray-50 p-2">
					<p class="text-xs text-gray-500">Selected text:</p>
					<p class="mt-1 line-clamp-2 text-sm italic text-gray-700">
						"{currentSelection.text}"
					</p>
				</div>
			{/if}

			<!-- Comment Input -->
			<div class="relative">
				<textarea
					bind:this={textareaRef}
					bind:value={commentBody}
					oninput={handleTextareaInput}
					onkeydown={(e) => {
						if (e.key === 'Escape') {
							closeCommentPopover();
						} else if (e.key === 'Enter' && (e.metaKey || e.ctrlKey)) {
							submitComment();
						}
					}}
					placeholder="Add your comment... Use @ to mention"
					class="w-full resize-none rounded-lg border border-gray-300 p-3 text-sm
						placeholder:text-gray-400 focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
					rows="3"
					aria-label="Comment text"
				></textarea>

				<!-- Mention Dropdown -->
				{#if showMentionDropdown && mentionSuggestions.length > 0}
					<div
						class="absolute left-0 right-0 z-10 mt-1 max-h-32 overflow-auto rounded-lg border border-gray-200 bg-white shadow-lg"
						role="listbox"
						aria-label="User suggestions"
					>
						{#each mentionSuggestions as user}
							<button
								type="button"
								role="option"
								aria-selected="false"
								onclick={() => insertMention(user)}
								class="flex w-full items-center gap-2 px-3 py-2 text-left text-sm hover:bg-gray-50"
							>
								<span
									class="flex h-5 w-5 items-center justify-center rounded-full bg-gray-200 text-xs font-medium text-gray-600"
								>
									{getUserInitials(user)}
								</span>
								<span class="text-gray-900">{user.display_name}</span>
							</button>
						{/each}
					</div>
				{/if}
			</div>

			<!-- Actions -->
			<div class="mt-3 flex justify-end gap-2">
				<button
					type="button"
					onclick={closeCommentPopover}
					class="rounded-lg px-3 py-1.5 text-sm font-medium text-gray-600 hover:bg-gray-100"
				>
					Cancel
				</button>
				<button
					type="button"
					onclick={submitComment}
					disabled={!commentBody.trim() || isSaving}
					class="rounded-lg bg-blue-600 px-3 py-1.5 text-sm font-medium text-white
						hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2
						disabled:cursor-not-allowed disabled:opacity-50"
				>
					{isSaving ? 'Adding...' : 'Add Comment'}
				</button>
			</div>

			<!-- Keyboard Shortcut Hint -->
			<p class="mt-2 text-center text-xs text-gray-400">
				Press <kbd class="rounded bg-gray-100 px-1 py-0.5 font-mono text-gray-500">Cmd+Enter</kbd> to submit
			</p>
		</div>
	</div>
{/if}

<!-- Comment Count Indicator -->
{#if inlineComments.length > 0}
	<div
		class="comment-indicator"
		aria-label="{inlineComments.length} comments on this block"
	>
		<div
			class="flex items-center gap-1 rounded-full bg-yellow-100 px-2 py-1 text-xs font-medium text-yellow-800"
		>
			<svg class="h-3.5 w-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="2"
					d="M7 8h10M7 12h4m1 8l-4-4H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-3l-4 4z"
				/>
			</svg>
			<span>{inlineComments.length}</span>
		</div>
	</div>
{/if}

<style>
	/* Add Comment Button */
	.add-comment-button {
		position: absolute;
		z-index: 50;
		animation: fadeIn 0.15s ease-out;
	}

	/* Comment Popover */
	.comment-popover {
		position: absolute;
		z-index: 60;
		width: 320px;
		animation: slideIn 0.2s ease-out;
	}

	/* Backdrop */
	.comment-backdrop {
		position: fixed;
		inset: 0;
		z-index: 55;
		background-color: transparent;
	}

	/* Comment Indicator */
	.comment-indicator {
		position: absolute;
		top: 0;
		right: -8px;
		transform: translateX(100%);
		z-index: 10;
	}

	/* Highlight Styles */
	:global(.inline-comment-highlight) {
		cursor: pointer;
		transition: background-color 0.2s ease;
	}

	:global(.inline-comment-highlight.active) {
		background-color: rgb(254 249 195);
		border-bottom: 2px solid rgb(250 204 21);
	}

	:global(.inline-comment-highlight.active:hover) {
		background-color: rgb(254 240 138);
	}

	:global(.inline-comment-highlight.resolved) {
		background-color: rgb(220 252 231);
		border-bottom: 2px solid rgb(134 239 172);
	}

	:global(.inline-comment-highlight.resolved:hover) {
		background-color: rgb(187 247 208);
	}

	/* Focus states for highlights */
	:global(.inline-comment-highlight:focus) {
		outline: 2px solid rgb(59 130 246);
		outline-offset: 1px;
	}

	/* Animations */
	@keyframes fadeIn {
		from {
			opacity: 0;
			transform: scale(0.9);
		}
		to {
			opacity: 1;
			transform: scale(1);
		}
	}

	@keyframes slideIn {
		from {
			opacity: 0;
			transform: translateY(-10px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	/* Keyboard styling */
	kbd {
		font-size: 0.75rem;
	}

	/* Line clamp for selected text */
	.line-clamp-2 {
		display: -webkit-box;
		-webkit-line-clamp: 2;
		line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}
</style>

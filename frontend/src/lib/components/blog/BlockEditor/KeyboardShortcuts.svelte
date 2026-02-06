<script lang="ts">
	/**
	 * Keyboard Shortcuts Panel - Enterprise-Grade Hotkeys
	 * ====================================================
	 * Comprehensive keyboard shortcuts display with
	 * customization and quick reference.
	 *
	 * @version 1.0.0
	 * @author Revolution Trading Pros
	 */

	interface ShortcutCategory {
		name: string;
		shortcuts: Shortcut[];
	}

	interface Shortcut {
		keys: string[];
		description: string;
		action: string;
	}

	interface Props {
		isOpen: boolean;
		onClose: () => void;
	}

	let props: Props = $props();
	const isOpen = $derived(props.isOpen);
	const onClose = $derived(props.onClose);

	// Detect platform
	const isMac = typeof navigator !== 'undefined' && /Mac|iPod|iPhone|iPad/.test(navigator.platform);
	const modKey = isMac ? '⌘' : 'Ctrl';
	const altKey = isMac ? '⌥' : 'Alt';
	const shiftKey = isMac ? '⇧' : 'Shift';

	// Search query
	let searchQuery = $state('');

	// Shortcut categories
	const shortcutCategories: ShortcutCategory[] = [
		{
			name: 'General',
			shortcuts: [
				{ keys: [modKey, 'S'], description: 'Save document', action: 'save' },
				{ keys: [modKey, 'Z'], description: 'Undo', action: 'undo' },
				{ keys: [modKey, shiftKey, 'Z'], description: 'Redo', action: 'redo' },
				{ keys: [modKey, 'A'], description: 'Select all', action: 'selectAll' },
				{ keys: [modKey, 'P'], description: 'Preview', action: 'preview' },
				{ keys: ['Escape'], description: 'Close panel / Deselect', action: 'escape' },
				{ keys: [modKey, '/'], description: 'Show keyboard shortcuts', action: 'showShortcuts' }
			]
		},
		{
			name: 'Block Operations',
			shortcuts: [
				{ keys: ['Enter'], description: 'Add new block below', action: 'addBlock' },
				{ keys: [shiftKey, 'Enter'], description: 'Add line break', action: 'lineBreak' },
				{ keys: [modKey, 'D'], description: 'Duplicate block', action: 'duplicate' },
				{ keys: [modKey, 'Backspace'], description: 'Delete block', action: 'delete' },
				{ keys: [modKey, '↑'], description: 'Move block up', action: 'moveUp' },
				{ keys: [modKey, '↓'], description: 'Move block down', action: 'moveDown' },
				{ keys: ['/'], description: 'Open block inserter', action: 'openInserter' },
				{ keys: [modKey, altKey, 'T'], description: 'Transform block type', action: 'transform' }
			]
		},
		{
			name: 'Text Formatting',
			shortcuts: [
				{ keys: [modKey, 'B'], description: 'Bold', action: 'bold' },
				{ keys: [modKey, 'I'], description: 'Italic', action: 'italic' },
				{ keys: [modKey, 'U'], description: 'Underline', action: 'underline' },
				{ keys: [modKey, shiftKey, 'S'], description: 'Strikethrough', action: 'strikethrough' },
				{ keys: [modKey, 'K'], description: 'Insert link', action: 'link' },
				{ keys: [modKey, shiftKey, 'M'], description: 'Inline code', action: 'inlineCode' },
				{ keys: [modKey, shiftKey, 'X'], description: 'Subscript', action: 'subscript' },
				{ keys: [modKey, shiftKey, '^'], description: 'Superscript', action: 'superscript' }
			]
		},
		{
			name: 'Block Types',
			shortcuts: [
				{ keys: [modKey, altKey, '1'], description: 'Heading 1', action: 'h1' },
				{ keys: [modKey, altKey, '2'], description: 'Heading 2', action: 'h2' },
				{ keys: [modKey, altKey, '3'], description: 'Heading 3', action: 'h3' },
				{ keys: [modKey, altKey, '4'], description: 'Heading 4', action: 'h4' },
				{ keys: [modKey, altKey, '0'], description: 'Paragraph', action: 'paragraph' },
				{ keys: [modKey, altKey, '7'], description: 'Bulleted list', action: 'bulletList' },
				{ keys: [modKey, altKey, '8'], description: 'Numbered list', action: 'numberedList' },
				{ keys: [modKey, altKey, '9'], description: 'Checklist', action: 'checklist' },
				{ keys: [modKey, altKey, 'Q'], description: 'Quote', action: 'quote' },
				{ keys: [modKey, altKey, 'C'], description: 'Code block', action: 'code' }
			]
		},
		{
			name: 'Navigation',
			shortcuts: [
				{ keys: ['↑'], description: 'Move to previous block', action: 'prevBlock' },
				{ keys: ['↓'], description: 'Move to next block', action: 'nextBlock' },
				{ keys: [modKey, 'Home'], description: 'Go to first block', action: 'firstBlock' },
				{ keys: [modKey, 'End'], description: 'Go to last block', action: 'lastBlock' },
				{ keys: ['Tab'], description: 'Indent / Focus next field', action: 'tab' },
				{
					keys: [shiftKey, 'Tab'],
					description: 'Outdent / Focus previous field',
					action: 'shiftTab'
				}
			]
		},
		{
			name: 'Selection',
			shortcuts: [
				{ keys: [shiftKey, '↑'], description: 'Extend selection up', action: 'selectUp' },
				{ keys: [shiftKey, '↓'], description: 'Extend selection down', action: 'selectDown' },
				{
					keys: [modKey, shiftKey, 'A'],
					description: 'Select all blocks',
					action: 'selectAllBlocks'
				},
				{ keys: ['Escape'], description: 'Clear selection', action: 'clearSelection' }
			]
		},
		{
			name: 'Clipboard',
			shortcuts: [
				{ keys: [modKey, 'C'], description: 'Copy', action: 'copy' },
				{ keys: [modKey, 'X'], description: 'Cut', action: 'cut' },
				{ keys: [modKey, 'V'], description: 'Paste', action: 'paste' },
				{ keys: [modKey, shiftKey, 'V'], description: 'Paste as plain text', action: 'pastePlain' },
				{
					keys: [modKey, shiftKey, 'D'],
					description: 'Duplicate to clipboard',
					action: 'duplicateClipboard'
				}
			]
		},
		{
			name: 'View',
			shortcuts: [
				{ keys: [modKey, '\\'], description: 'Toggle sidebar', action: 'toggleSidebar' },
				{ keys: [modKey, shiftKey, 'F'], description: 'Toggle fullscreen', action: 'fullscreen' },
				{ keys: [modKey, '+'], description: 'Zoom in', action: 'zoomIn' },
				{ keys: [modKey, '-'], description: 'Zoom out', action: 'zoomOut' },
				{ keys: [modKey, '0'], description: 'Reset zoom', action: 'resetZoom' }
			]
		}
	];

	// Filter shortcuts based on search
	let filteredCategories = $derived.by(() => {
		if (!searchQuery) return shortcutCategories;

		const query = searchQuery.toLowerCase();
		return shortcutCategories
			.map((category) => ({
				...category,
				shortcuts: category.shortcuts.filter(
					(s) =>
						s.description.toLowerCase().includes(query) ||
						s.keys.join(' ').toLowerCase().includes(query)
				)
			}))
			.filter((category) => category.shortcuts.length > 0);
	});

	// Close on escape
	function handleKeydown(e: KeyboardEvent): void {
		if (e.key === 'Escape' && isOpen) {
			onClose();
		}
	}
</script>

<svelte:window onkeydown={handleKeydown} />

{#if isOpen}
	<div
		class="shortcuts-overlay"
		onclick={onClose}
		onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && onClose()}
		role="dialog"
		aria-modal="true"
		tabindex="-1"
	>
		<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
		<!-- svelte-ignore a11y_click_events_have_key_events -->
		<div class="shortcuts-modal" onclick={(e: MouseEvent) => e.stopPropagation()} role="document">
			<div class="shortcuts-header">
				<h2>Keyboard Shortcuts</h2>
				<button class="close-btn" onclick={onClose} aria-label="Close"> ✕ </button>
			</div>

			<div class="search-bar">
				<span class="search-icon">🔍</span>
				<input
					type="text"
					placeholder="Search shortcuts..."
					bind:value={searchQuery}
					aria-label="Search keyboard shortcuts"
				/>
				{#if searchQuery}
					<button class="clear-btn" onclick={() => (searchQuery = '')}> ✕ </button>
				{/if}
			</div>

			<div class="shortcuts-content">
				{#each filteredCategories as category}
					<div class="category">
						<h3 class="category-name">{category.name}</h3>
						<div class="shortcuts-list">
							{#each category.shortcuts as shortcut}
								<div class="shortcut-item">
									<div class="shortcut-keys">
										{#each shortcut.keys as key, i}
											<kbd class="key">{key}</kbd>
											{#if i < shortcut.keys.length - 1}
												<span class="plus">+</span>
											{/if}
										{/each}
									</div>
									<span class="shortcut-description">{shortcut.description}</span>
								</div>
							{/each}
						</div>
					</div>
				{/each}

				{#if filteredCategories.length === 0}
					<div class="no-results">
						<p>No shortcuts found for "{searchQuery}"</p>
					</div>
				{/if}
			</div>

			<div class="shortcuts-footer">
				<p class="platform-note">
					{#if isMac}
						Showing shortcuts for macOS. ⌘ = Command, ⌥ = Option, ⇧ = Shift
					{:else}
						Showing shortcuts for Windows/Linux.
					{/if}
				</p>
			</div>
		</div>
	</div>
{/if}

<style>
	.shortcuts-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.5);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
		padding: 2rem;
		animation: fadeIn 0.2s ease;
	}

	@keyframes fadeIn {
		from {
			opacity: 0;
		}
		to {
			opacity: 1;
		}
	}

	.shortcuts-modal {
		width: 100%;
		max-width: 700px;
		max-height: 80vh;
		background: var(--bg-primary, #ffffff);
		border-radius: 0.75rem;
		display: flex;
		flex-direction: column;
		overflow: hidden;
		box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25);
		animation: slideUp 0.2s ease;
	}

	@keyframes slideUp {
		from {
			opacity: 0;
			transform: translateY(10px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	.shortcuts-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1.25rem 1.5rem;
		border-bottom: 1px solid var(--border-color, #e5e7eb);
	}

	.shortcuts-header h2 {
		margin: 0;
		font-size: 1.25rem;
		font-weight: 600;
		color: var(--text-primary, #1f2937);
	}

	.close-btn {
		width: 32px;
		height: 32px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: var(--bg-secondary, #f9fafb);
		border: none;
		border-radius: 0.375rem;
		cursor: pointer;
		font-size: 1rem;
		color: var(--text-secondary, #6b7280);
		transition: all 0.2s;
	}

	.close-btn:hover {
		background: var(--bg-hover, #f3f4f6);
		color: var(--text-primary, #1f2937);
	}

	.search-bar {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem 1.5rem;
		border-bottom: 1px solid var(--border-color, #e5e7eb);
		background: var(--bg-secondary, #f9fafb);
	}

	.search-icon {
		font-size: 1rem;
		opacity: 0.5;
	}

	.search-bar input {
		flex: 1;
		padding: 0.5rem;
		border: none;
		background: transparent;
		font-size: 0.9375rem;
		outline: none;
		color: var(--text-primary, #1f2937);
	}

	.search-bar input::placeholder {
		color: var(--text-tertiary, #9ca3af);
	}

	.clear-btn {
		padding: 0.25rem 0.5rem;
		background: var(--bg-primary, #ffffff);
		border: 1px solid var(--border-color, #e5e7eb);
		border-radius: 0.25rem;
		font-size: 0.75rem;
		cursor: pointer;
		color: var(--text-secondary, #6b7280);
	}

	.clear-btn:hover {
		background: var(--bg-hover, #f3f4f6);
	}

	.shortcuts-content {
		flex: 1;
		overflow-y: auto;
		padding: 1rem 1.5rem;
	}

	.category {
		margin-bottom: 1.5rem;
	}

	.category:last-child {
		margin-bottom: 0;
	}

	.category-name {
		margin: 0 0 0.75rem;
		font-size: 0.6875rem;
		font-weight: 700;
		color: var(--text-secondary, #6b7280);
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.shortcuts-list {
		display: grid;
		gap: 0.5rem;
	}

	.shortcut-item {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 0.625rem 0.875rem;
		background: var(--bg-secondary, #f9fafb);
		border-radius: 0.375rem;
		transition: background 0.2s;
	}

	.shortcut-item:hover {
		background: var(--bg-hover, #f3f4f6);
	}

	.shortcut-keys {
		display: flex;
		align-items: center;
		gap: 0.25rem;
	}

	.key {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		min-width: 24px;
		padding: 0.25rem 0.5rem;
		background: var(--bg-primary, #ffffff);
		border: 1px solid var(--border-color, #e5e7eb);
		border-radius: 0.25rem;
		font-family: inherit;
		font-size: 0.75rem;
		font-weight: 500;
		color: var(--text-primary, #1f2937);
		box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
	}

	.plus {
		font-size: 0.75rem;
		color: var(--text-tertiary, #9ca3af);
	}

	.shortcut-description {
		font-size: 0.8125rem;
		color: var(--text-secondary, #6b7280);
	}

	.no-results {
		padding: 3rem;
		text-align: center;
	}

	.no-results p {
		margin: 0;
		font-size: 0.9375rem;
		color: var(--text-tertiary, #9ca3af);
	}

	.shortcuts-footer {
		padding: 0.875rem 1.5rem;
		border-top: 1px solid var(--border-color, #e5e7eb);
		background: var(--bg-secondary, #f9fafb);
	}

	.platform-note {
		margin: 0;
		font-size: 0.75rem;
		color: var(--text-tertiary, #9ca3af);
		text-align: center;
	}

	/* Scrollbar styling */
	.shortcuts-content::-webkit-scrollbar {
		width: 8px;
	}

	.shortcuts-content::-webkit-scrollbar-track {
		background: transparent;
	}

	.shortcuts-content::-webkit-scrollbar-thumb {
		background: var(--border-color, #e5e7eb);
		border-radius: 4px;
	}

	.shortcuts-content::-webkit-scrollbar-thumb:hover {
		background: var(--text-tertiary, #9ca3af);
	}
</style>

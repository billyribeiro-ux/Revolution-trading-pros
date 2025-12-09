<script lang="ts">
	/**
	 * Block Toolbar - Enterprise-Grade Formatting Toolbar
	 * ====================================================
	 * Context-aware formatting toolbar with rich text editing,
	 * block transformations, and quick actions.
	 *
	 * @version 1.0.0
	 * @author Revolution Trading Pros
	 */

	import type { Block, BlockType } from './types';

	interface Props {
		block: Block;
		onUpdate: (content: Partial<Block['content']>) => void;
		onTransform: (type: BlockType) => void;
		onDuplicate: () => void;
		onDelete: () => void;
		onMoveUp: () => void;
		onMoveDown: () => void;
		onInsertBefore: () => void;
		onInsertAfter: () => void;
		position?: 'top' | 'floating';
		canMoveUp?: boolean;
		canMoveDown?: boolean;
	}

	let {
		block,
		onUpdate,
		onTransform,
		onDuplicate,
		onDelete,
		onMoveUp,
		onMoveDown,
		onInsertBefore,
		onInsertAfter,
		position = 'floating',
		canMoveUp = true,
		canMoveDown = true
	}: Props = $props();

	// Dropdown states
	let showTransformMenu = $state(false);
	let showMoreMenu = $state(false);
	let showAlignMenu = $state(false);
	let showLinkModal = $state(false);

	// Link input state
	let linkUrl = $state('');
	let linkText = $state('');

	// Transform options based on block type
	const transformOptions: { type: BlockType; label: string; icon: string }[] = [
		{ type: 'paragraph', label: 'Paragraph', icon: '¶' },
		{ type: 'heading', label: 'Heading', icon: 'H' },
		{ type: 'quote', label: 'Quote', icon: '"' },
		{ type: 'list', label: 'List', icon: '•' },
		{ type: 'code', label: 'Code', icon: '</>' },
		{ type: 'callout', label: 'Callout', icon: '!' }
	];

	// Heading levels
	const headingLevels = [
		{ level: 1, label: 'H1' },
		{ level: 2, label: 'H2' },
		{ level: 3, label: 'H3' },
		{ level: 4, label: 'H4' },
		{ level: 5, label: 'H5' },
		{ level: 6, label: 'H6' }
	];

	// Text alignment options
	const alignOptions = [
		{ value: 'left', icon: '☰', label: 'Left' },
		{ value: 'center', icon: '☷', label: 'Center' },
		{ value: 'right', icon: '☲', label: 'Right' },
		{ value: 'justify', icon: '☱', label: 'Justify' }
	];

	// Apply text formatting (execCommand simulation)
	function applyFormat(format: string): void {
		// In a real implementation, this would use Selection API
		// For now, we'll update the block content
		document.execCommand(format, false);
	}

	// Handle transform
	function handleTransform(type: BlockType): void {
		onTransform(type);
		showTransformMenu = false;
	}

	// Handle heading level change
	function setHeadingLevel(level: number): void {
		onUpdate({ level });
	}

	// Handle alignment change
	function setAlignment(align: string): void {
		onUpdate({ align });
		showAlignMenu = false;
	}

	// Handle link insertion
	function insertLink(): void {
		if (linkUrl) {
			// Apply link formatting
			const selection = window.getSelection();
			if (selection && selection.rangeCount > 0) {
				const range = selection.getRangeAt(0);
				const link = document.createElement('a');
				link.href = linkUrl;
				link.textContent = linkText || linkUrl;
				link.target = '_blank';
				link.rel = 'noopener noreferrer';
				range.deleteContents();
				range.insertNode(link);
			}
			linkUrl = '';
			linkText = '';
			showLinkModal = false;
		}
	}

	// Close all dropdowns
	function closeAllDropdowns(): void {
		showTransformMenu = false;
		showMoreMenu = false;
		showAlignMenu = false;
	}

	// Handle click outside
	function handleClickOutside(e: MouseEvent): void {
		const target = e.target as HTMLElement;
		if (!target.closest('.dropdown')) {
			closeAllDropdowns();
		}
	}

	// Check if block supports text formatting
	let supportsTextFormatting = $derived(
		['paragraph', 'heading', 'quote', 'pullquote', 'list', 'checklist', 'callout'].includes(block.type)
	);

	// Check if block is a heading
	let isHeading = $derived(block.type === 'heading');
</script>

<svelte:window onclick={handleClickOutside} />

<div class="block-toolbar" class:floating={position === 'floating'}>
	<!-- Block Type / Transform -->
	<div class="toolbar-group">
		<div class="dropdown">
			<button
				class="toolbar-btn transform-btn"
				title="Transform block"
				onclick={(e) => {
					e.stopPropagation();
					showTransformMenu = !showTransformMenu;
					showMoreMenu = false;
					showAlignMenu = false;
				}}
			>
				<span class="block-type-icon">
					{#if block.type === 'paragraph'}¶
					{:else if block.type === 'heading'}H{block.content.level || 2}
					{:else if block.type === 'quote'}"
					{:else if block.type === 'list'}•
					{:else if block.type === 'code'}</>
					{:else if block.type === 'image'}🖼
					{:else if block.type === 'video'}▶
					{:else}⬡
					{/if}
				</span>
				<span class="dropdown-arrow">▼</span>
			</button>
			{#if showTransformMenu}
				<div class="dropdown-menu">
					<div class="dropdown-section">
						<span class="dropdown-label">Transform to</span>
						{#each transformOptions as option}
							<button
								class="dropdown-item"
								class:active={block.type === option.type}
								onclick={() => handleTransform(option.type)}
							>
								<span class="item-icon">{option.icon}</span>
								<span>{option.label}</span>
							</button>
						{/each}
					</div>
					{#if isHeading}
						<div class="dropdown-section">
							<span class="dropdown-label">Heading Level</span>
							<div class="heading-levels">
								{#each headingLevels as h}
									<button
										class="level-btn"
										class:active={block.content.level === h.level}
										onclick={() => setHeadingLevel(h.level)}
									>
										{h.label}
									</button>
								{/each}
							</div>
						</div>
					{/if}
				</div>
			{/if}
		</div>
	</div>

	<!-- Divider -->
	<div class="toolbar-divider"></div>

	<!-- Text Formatting (only for text blocks) -->
	{#if supportsTextFormatting}
		<div class="toolbar-group">
			<button
				class="toolbar-btn"
				title="Bold (Ctrl+B)"
				onclick={() => applyFormat('bold')}
			>
				<strong>B</strong>
			</button>
			<button
				class="toolbar-btn"
				title="Italic (Ctrl+I)"
				onclick={() => applyFormat('italic')}
			>
				<em>I</em>
			</button>
			<button
				class="toolbar-btn"
				title="Underline (Ctrl+U)"
				onclick={() => applyFormat('underline')}
			>
				<span style="text-decoration: underline">U</span>
			</button>
			<button
				class="toolbar-btn"
				title="Strikethrough"
				onclick={() => applyFormat('strikethrough')}
			>
				<span style="text-decoration: line-through">S</span>
			</button>
			<button
				class="toolbar-btn"
				title="Insert Link (Ctrl+K)"
				onclick={() => showLinkModal = true}
			>
				🔗
			</button>
			<button
				class="toolbar-btn"
				title="Inline Code"
				onclick={() => applyFormat('code')}
			>
				<code>&lt;/&gt;</code>
			</button>
		</div>

		<!-- Divider -->
		<div class="toolbar-divider"></div>

		<!-- Alignment -->
		<div class="toolbar-group">
			<div class="dropdown">
				<button
					class="toolbar-btn"
					title="Text alignment"
					onclick={(e) => {
						e.stopPropagation();
						showAlignMenu = !showAlignMenu;
						showTransformMenu = false;
						showMoreMenu = false;
					}}
				>
					{alignOptions.find(a => a.value === (block.content.align || 'left'))?.icon || '☰'}
					<span class="dropdown-arrow">▼</span>
				</button>
				{#if showAlignMenu}
					<div class="dropdown-menu align-menu">
						{#each alignOptions as align}
							<button
								class="dropdown-item"
								class:active={block.content.align === align.value}
								onclick={() => setAlignment(align.value)}
							>
								<span class="item-icon">{align.icon}</span>
								<span>{align.label}</span>
							</button>
						{/each}
					</div>
				{/if}
			</div>
		</div>

		<!-- Divider -->
		<div class="toolbar-divider"></div>
	{/if}

	<!-- Block Actions -->
	<div class="toolbar-group">
		<button
			class="toolbar-btn"
			title="Move up (Ctrl+↑)"
			onclick={onMoveUp}
			disabled={!canMoveUp}
		>
			↑
		</button>
		<button
			class="toolbar-btn"
			title="Move down (Ctrl+↓)"
			onclick={onMoveDown}
			disabled={!canMoveDown}
		>
			↓
		</button>
	</div>

	<!-- Divider -->
	<div class="toolbar-divider"></div>

	<!-- More Actions -->
	<div class="toolbar-group">
		<button
			class="toolbar-btn"
			title="Duplicate (Ctrl+D)"
			onclick={onDuplicate}
		>
			⧉
		</button>
		<div class="dropdown">
			<button
				class="toolbar-btn"
				title="More options"
				onclick={(e) => {
					e.stopPropagation();
					showMoreMenu = !showMoreMenu;
					showTransformMenu = false;
					showAlignMenu = false;
				}}
			>
				⋮
			</button>
			{#if showMoreMenu}
				<div class="dropdown-menu more-menu">
					<button class="dropdown-item" onclick={onInsertBefore}>
						<span class="item-icon">↑</span>
						<span>Insert before</span>
					</button>
					<button class="dropdown-item" onclick={onInsertAfter}>
						<span class="item-icon">↓</span>
						<span>Insert after</span>
					</button>
					<div class="dropdown-divider"></div>
					<button class="dropdown-item" onclick={onDuplicate}>
						<span class="item-icon">⧉</span>
						<span>Duplicate</span>
						<span class="shortcut">Ctrl+D</span>
					</button>
					<button class="dropdown-item" onclick={() => navigator.clipboard.writeText(JSON.stringify(block))}>
						<span class="item-icon">📋</span>
						<span>Copy block</span>
					</button>
					<div class="dropdown-divider"></div>
					<button class="dropdown-item danger" onclick={onDelete}>
						<span class="item-icon">🗑</span>
						<span>Delete</span>
						<span class="shortcut">Del</span>
					</button>
				</div>
			{/if}
		</div>
	</div>

	<!-- Delete Button -->
	<button
		class="toolbar-btn delete-btn"
		title="Delete block"
		onclick={onDelete}
	>
		🗑
	</button>
</div>

<!-- Link Modal -->
{#if showLinkModal}
	<div class="modal-overlay" onclick={() => showLinkModal = false}>
		<div class="link-modal" onclick={(e) => e.stopPropagation()}>
			<h4>Insert Link</h4>
			<div class="field">
				<label>URL</label>
				<input
					type="url"
					placeholder="https://example.com"
					bind:value={linkUrl}
					autofocus
				/>
			</div>
			<div class="field">
				<label>Link Text (optional)</label>
				<input
					type="text"
					placeholder="Click here"
					bind:value={linkText}
				/>
			</div>
			<div class="modal-actions">
				<button class="cancel-btn" onclick={() => showLinkModal = false}>
					Cancel
				</button>
				<button class="submit-btn" onclick={insertLink} disabled={!linkUrl}>
					Insert
				</button>
			</div>
		</div>
	</div>
{/if}

<style>
	.block-toolbar {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		padding: 0.375rem 0.5rem;
		background: var(--bg-primary, #ffffff);
		border: 1px solid var(--border-color, #e5e7eb);
		border-radius: 0.5rem;
		box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1),
			0 2px 4px -1px rgba(0, 0, 0, 0.06);
	}

	.block-toolbar.floating {
		position: absolute;
		top: -48px;
		left: 50%;
		transform: translateX(-50%);
		z-index: 100;
	}

	.toolbar-group {
		display: flex;
		align-items: center;
		gap: 0.125rem;
	}

	.toolbar-divider {
		width: 1px;
		height: 24px;
		background: var(--border-color, #e5e7eb);
		margin: 0 0.375rem;
	}

	.toolbar-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		min-width: 32px;
		height: 32px;
		padding: 0 0.5rem;
		background: transparent;
		border: none;
		border-radius: 0.375rem;
		font-size: 0.875rem;
		color: var(--text-primary, #1f2937);
		cursor: pointer;
		transition: all 0.15s;
	}

	.toolbar-btn:hover:not(:disabled) {
		background: var(--bg-hover, #f3f4f6);
	}

	.toolbar-btn:active:not(:disabled) {
		background: var(--bg-active, #e5e7eb);
	}

	.toolbar-btn:disabled {
		opacity: 0.4;
		cursor: not-allowed;
	}

	.toolbar-btn.delete-btn {
		color: #ef4444;
	}

	.toolbar-btn.delete-btn:hover:not(:disabled) {
		background: #fef2f2;
	}

	.transform-btn {
		gap: 0.25rem;
	}

	.block-type-icon {
		font-weight: 600;
		font-size: 0.75rem;
	}

	.dropdown-arrow {
		font-size: 0.5rem;
		opacity: 0.6;
	}

	.dropdown {
		position: relative;
	}

	.dropdown-menu {
		position: absolute;
		top: calc(100% + 0.375rem);
		left: 0;
		min-width: 180px;
		background: var(--bg-primary, #ffffff);
		border: 1px solid var(--border-color, #e5e7eb);
		border-radius: 0.5rem;
		box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1),
			0 4px 6px -2px rgba(0, 0, 0, 0.05);
		padding: 0.375rem;
		z-index: 150;
		animation: dropdownFade 0.15s ease;
	}

	@keyframes dropdownFade {
		from {
			opacity: 0;
			transform: translateY(-4px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	.dropdown-menu.align-menu {
		min-width: 140px;
	}

	.dropdown-menu.more-menu {
		min-width: 200px;
		left: auto;
		right: 0;
	}

	.dropdown-section {
		margin-bottom: 0.5rem;
	}

	.dropdown-section:last-child {
		margin-bottom: 0;
	}

	.dropdown-label {
		display: block;
		padding: 0.375rem 0.625rem;
		font-size: 0.6875rem;
		font-weight: 600;
		color: var(--text-secondary, #6b7280);
		text-transform: uppercase;
		letter-spacing: 0.025em;
	}

	.dropdown-item {
		display: flex;
		align-items: center;
		width: 100%;
		padding: 0.5rem 0.625rem;
		background: transparent;
		border: none;
		border-radius: 0.25rem;
		font-size: 0.8125rem;
		color: var(--text-primary, #1f2937);
		cursor: pointer;
		transition: background 0.15s;
		text-align: left;
		gap: 0.5rem;
	}

	.dropdown-item:hover {
		background: var(--bg-hover, #f3f4f6);
	}

	.dropdown-item.active {
		background: #eff6ff;
		color: var(--primary, #3b82f6);
	}

	.dropdown-item.danger {
		color: #ef4444;
	}

	.dropdown-item.danger:hover {
		background: #fef2f2;
	}

	.item-icon {
		width: 20px;
		text-align: center;
		flex-shrink: 0;
	}

	.shortcut {
		margin-left: auto;
		font-size: 0.6875rem;
		color: var(--text-tertiary, #9ca3af);
	}

	.dropdown-divider {
		height: 1px;
		background: var(--border-color, #e5e7eb);
		margin: 0.375rem 0;
	}

	.heading-levels {
		display: flex;
		gap: 0.25rem;
		padding: 0.375rem 0.625rem;
	}

	.level-btn {
		width: 32px;
		height: 32px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: var(--bg-secondary, #f9fafb);
		border: 1px solid var(--border-color, #e5e7eb);
		border-radius: 0.25rem;
		font-size: 0.75rem;
		font-weight: 600;
		color: var(--text-primary, #1f2937);
		cursor: pointer;
		transition: all 0.15s;
	}

	.level-btn:hover {
		background: var(--bg-hover, #f3f4f6);
	}

	.level-btn.active {
		background: var(--primary, #3b82f6);
		color: white;
		border-color: var(--primary, #3b82f6);
	}

	/* Link Modal */
	.modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.4);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 200;
	}

	.link-modal {
		width: 100%;
		max-width: 400px;
		background: var(--bg-primary, #ffffff);
		border-radius: 0.75rem;
		padding: 1.5rem;
		box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25);
	}

	.link-modal h4 {
		margin: 0 0 1rem;
		font-size: 1rem;
		font-weight: 600;
		color: var(--text-primary, #1f2937);
	}

	.field {
		margin-bottom: 1rem;
	}

	.field label {
		display: block;
		margin-bottom: 0.375rem;
		font-size: 0.8125rem;
		font-weight: 500;
		color: var(--text-secondary, #6b7280);
	}

	.field input {
		width: 100%;
		padding: 0.625rem 0.875rem;
		border: 1px solid var(--border-color, #e5e7eb);
		border-radius: 0.375rem;
		font-size: 0.875rem;
		color: var(--text-primary, #1f2937);
		transition: border-color 0.2s, box-shadow 0.2s;
	}

	.field input:focus {
		outline: none;
		border-color: var(--primary, #3b82f6);
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
	}

	.modal-actions {
		display: flex;
		justify-content: flex-end;
		gap: 0.75rem;
		margin-top: 1.5rem;
	}

	.cancel-btn,
	.submit-btn {
		padding: 0.625rem 1.25rem;
		border-radius: 0.375rem;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.cancel-btn {
		background: var(--bg-secondary, #f9fafb);
		border: 1px solid var(--border-color, #e5e7eb);
		color: var(--text-secondary, #6b7280);
	}

	.cancel-btn:hover {
		background: var(--bg-hover, #f3f4f6);
	}

	.submit-btn {
		background: var(--primary, #3b82f6);
		border: none;
		color: white;
	}

	.submit-btn:hover:not(:disabled) {
		background: var(--primary-hover, #2563eb);
	}

	.submit-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}
</style>

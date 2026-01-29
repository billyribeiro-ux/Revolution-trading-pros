<!--
/**
 * EditorToolbar - Block Editor Formatting Toolbar
 * ================================================================
 * Main formatting toolbar for the CMS v2 block editor.
 * Provides comprehensive text formatting, block type selection,
 * document statistics, and save controls.
 *
 * Features:
 * - Formatting buttons with keyboard shortcut hints
 * - Block type dropdown
 * - Focus mode toggle
 * - Word count / reading time display
 * - Save status indicator
 * - Undo/Redo buttons
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 */
-->

<script lang="ts">
	import { onMount } from 'svelte';
	import {
		IconBold,
		IconItalic,
		IconUnderline,
		IconStrikethrough,
		IconCode,
		IconLink,
		IconAlignLeft,
		IconAlignCenter,
		IconAlignRight,
		IconList,
		IconListNumbers,
		IconArrowBackUp,
		IconArrowForwardUp,
		IconEye,
		IconFocus,
		IconDeviceFloppy,
		IconChevronDown,
		IconDots,
		IconCheck,
		IconCloud,
		IconCloudUpload,
		IconAlertCircle,
		IconCalendarEvent,
		IconSend,
		IconH1,
		IconH2,
		IconH3,
		IconBlockquote,
		IconTextCaption
	} from '$lib/icons';

	// ==========================================================================
	// Types
	// ==========================================================================

	type BlockType = 'paragraph' | 'h1' | 'h2' | 'h3' | 'quote' | 'code' | 'list';
	type TextFormat = 'bold' | 'italic' | 'underline' | 'strikethrough' | 'code' | 'link';
	type TextAlign = 'left' | 'center' | 'right';
	type ListType = 'bullet' | 'numbered';
	type SaveStatus = 'saving' | 'saved' | 'unsaved' | 'error';

	interface BlockTypeOption {
		value: BlockType;
		label: string;
		icon: typeof IconTextCaption;
	}

	// ==========================================================================
	// Props
	// ==========================================================================

	interface Props {
		onFormat?: (format: TextFormat) => void;
		onBlockTypeChange?: (type: BlockType) => void;
		onAlign?: (align: TextAlign) => void;
		onList?: (type: ListType) => void;
		onUndo?: () => void;
		onRedo?: () => void;
		onSave?: () => void;
		onPreview?: () => void;
		onFocusMode?: () => void;
		onPublish?: () => void;
		onSchedule?: () => void;
		canUndo?: boolean;
		canRedo?: boolean;
		isSaving?: boolean;
		lastSaved?: Date | null;
		hasUnsavedChanges?: boolean;
		wordCount?: number;
		characterCount?: number;
		showCharacterCount?: boolean;
		isFocusMode?: boolean;
		activeFormats?: TextFormat[];
		currentBlockType?: BlockType;
		currentAlign?: TextAlign;
		disabled?: boolean;
	}

	let {
		onFormat,
		onBlockTypeChange,
		onAlign,
		onList,
		onUndo,
		onRedo,
		onSave,
		onPreview,
		onFocusMode,
		onPublish,
		onSchedule,
		canUndo = false,
		canRedo = false,
		isSaving = false,
		lastSaved = null,
		hasUnsavedChanges = false,
		wordCount = 0,
		characterCount = 0,
		showCharacterCount = false,
		isFocusMode = false,
		activeFormats = [],
		currentBlockType = 'paragraph',
		currentAlign = 'left',
		disabled = false
	}: Props = $props();

	// ==========================================================================
	// State
	// ==========================================================================

	let showBlockTypeDropdown = $state(false);
	let showMoreActionsDropdown = $state(false);
	let blockTypeButtonRef: HTMLButtonElement;
	let moreActionsButtonRef: HTMLButtonElement;

	// Detect platform for shortcut display
	const isMac = typeof navigator !== 'undefined' && /Mac|iPod|iPhone|iPad/.test(navigator.platform);
	const modKey = isMac ? 'Cmd' : 'Ctrl';

	// ==========================================================================
	// Computed
	// ==========================================================================

	const readingTime = $derived(Math.max(1, Math.ceil(wordCount / 200)));

	const saveStatus = $derived.by((): SaveStatus => {
		if (isSaving) return 'saving';
		if (hasUnsavedChanges) return 'unsaved';
		return 'saved';
	});

	const lastSavedText = $derived.by(() => {
		if (!lastSaved) return 'Never saved';
		const now = new Date();
		const diff = now.getTime() - lastSaved.getTime();

		if (diff < 60000) return 'Just now';
		if (diff < 3600000) return `${Math.floor(diff / 60000)}m ago`;
		if (diff < 86400000) return `${Math.floor(diff / 3600000)}h ago`;
		return lastSaved.toLocaleDateString();
	});

	// Block type options
	const blockTypeOptions: BlockTypeOption[] = [
		{ value: 'paragraph', label: 'Paragraph', icon: IconTextCaption },
		{ value: 'h1', label: 'Heading 1', icon: IconH1 },
		{ value: 'h2', label: 'Heading 2', icon: IconH2 },
		{ value: 'h3', label: 'Heading 3', icon: IconH3 },
		{ value: 'quote', label: 'Quote', icon: IconBlockquote },
		{ value: 'code', label: 'Code', icon: IconCode },
		{ value: 'list', label: 'List', icon: IconList }
	];

	const currentBlockTypeOption = $derived(
		blockTypeOptions.find((opt) => opt.value === currentBlockType) || blockTypeOptions[0]
	);

	// Current block type icon - derived to avoid @const in invalid context
	const CurrentBlockIcon = $derived(currentBlockTypeOption?.icon || IconTextCaption);

	// ==========================================================================
	// Event Handlers
	// ==========================================================================

	function handleFormat(format: TextFormat) {
		if (disabled) return;
		onFormat?.(format);
	}

	function handleBlockTypeSelect(type: BlockType) {
		if (disabled) return;
		onBlockTypeChange?.(type);
		showBlockTypeDropdown = false;
	}

	function handleAlignClick(align: TextAlign) {
		if (disabled) return;
		onAlign?.(align);
	}

	function handleListClick(type: ListType) {
		if (disabled) return;
		onList?.(type);
	}

	function handleKeydown(e: KeyboardEvent) {
		const isMeta = e.metaKey || e.ctrlKey;

		// Handle toolbar keyboard shortcuts
		if (isMeta && !e.shiftKey) {
			switch (e.key.toLowerCase()) {
				case 'b':
					e.preventDefault();
					handleFormat('bold');
					break;
				case 'i':
					e.preventDefault();
					handleFormat('italic');
					break;
				case 'u':
					e.preventDefault();
					handleFormat('underline');
					break;
				case 'e':
					e.preventDefault();
					handleFormat('code');
					break;
				case 'k':
					e.preventDefault();
					handleFormat('link');
					break;
				case 's':
					e.preventDefault();
					onSave?.();
					break;
				case 'p':
					e.preventDefault();
					onPreview?.();
					break;
				case 'z':
					e.preventDefault();
					onUndo?.();
					break;
			}
		}

		if (isMeta && e.shiftKey && e.key.toLowerCase() === 'z') {
			e.preventDefault();
			onRedo?.();
		}

		// Close dropdowns on Escape
		if (e.key === 'Escape') {
			showBlockTypeDropdown = false;
			showMoreActionsDropdown = false;
		}
	}

	function handleClickOutside(e: MouseEvent) {
		const target = e.target as HTMLElement;
		if (
			showBlockTypeDropdown &&
			blockTypeButtonRef &&
			!blockTypeButtonRef.contains(target) &&
			!target.closest('.block-type-dropdown')
		) {
			showBlockTypeDropdown = false;
		}
		if (
			showMoreActionsDropdown &&
			moreActionsButtonRef &&
			!moreActionsButtonRef.contains(target) &&
			!target.closest('.more-actions-dropdown')
		) {
			showMoreActionsDropdown = false;
		}
	}

	// ==========================================================================
	// Lifecycle
	// ==========================================================================

	onMount(() => {
		document.addEventListener('keydown', handleKeydown);
		document.addEventListener('click', handleClickOutside);

		return () => {
			document.removeEventListener('keydown', handleKeydown);
			document.removeEventListener('click', handleClickOutside);
		};
	});

	// ==========================================================================
	// Helper Functions
	// ==========================================================================

	function isFormatActive(format: TextFormat): boolean {
		return activeFormats.includes(format);
	}

	function getShortcutText(key: string, shift = false): string {
		return shift ? `${modKey}+Shift+${key}` : `${modKey}+${key}`;
	}
</script>

<div
	class="editor-toolbar"
	class:disabled
	class:focus-mode={isFocusMode}
	role="toolbar"
	aria-label="Editor formatting toolbar"
	aria-disabled={disabled}
>
	<!-- Left Section: Block Type + Formatting -->
	<div class="toolbar-section toolbar-left">
		<!-- Block Type Dropdown -->
		<div class="toolbar-group">
			<div class="dropdown-container">
				<button
					bind:this={blockTypeButtonRef}
					type="button"
					class="block-type-button"
					class:active={showBlockTypeDropdown}
					onclick={() => (showBlockTypeDropdown = !showBlockTypeDropdown)}
					{disabled}
					aria-haspopup="listbox"
					aria-expanded={showBlockTypeDropdown}
					aria-label="Select block type"
				>
					<CurrentBlockIcon size={16} />
					<span class="block-type-label">{currentBlockTypeOption?.label || 'Paragraph'}</span>
					<IconChevronDown size={14} class="dropdown-icon" />
				</button>

				{#if showBlockTypeDropdown}
					<div class="block-type-dropdown" role="listbox" aria-label="Block types">
						{#each blockTypeOptions as option (option.value)}
							{@const OptionIcon = option.icon}
							<button
								type="button"
								class="dropdown-item"
								class:selected={currentBlockType === option.value}
								onclick={() => handleBlockTypeSelect(option.value)}
								role="option"
								aria-selected={currentBlockType === option.value}
							>
								<OptionIcon size={16} />
								<span>{option.label}</span>
								{#if currentBlockType === option.value}
									<IconCheck size={14} class="check-icon" />
								{/if}
							</button>
						{/each}
					</div>
				{/if}
			</div>
		</div>

		<div class="toolbar-divider" role="separator"></div>

		<!-- Text Formatting -->
		<div class="toolbar-group" role="group" aria-label="Text formatting">
			<button
				type="button"
				class="toolbar-button"
				class:active={isFormatActive('bold')}
				onclick={() => handleFormat('bold')}
				{disabled}
				title="Bold ({getShortcutText('B')})"
				aria-label="Bold"
				aria-pressed={isFormatActive('bold')}
			>
				<IconBold size={18} />
			</button>

			<button
				type="button"
				class="toolbar-button"
				class:active={isFormatActive('italic')}
				onclick={() => handleFormat('italic')}
				{disabled}
				title="Italic ({getShortcutText('I')})"
				aria-label="Italic"
				aria-pressed={isFormatActive('italic')}
			>
				<IconItalic size={18} />
			</button>

			<button
				type="button"
				class="toolbar-button"
				class:active={isFormatActive('underline')}
				onclick={() => handleFormat('underline')}
				{disabled}
				title="Underline ({getShortcutText('U')})"
				aria-label="Underline"
				aria-pressed={isFormatActive('underline')}
			>
				<IconUnderline size={18} />
			</button>

			<button
				type="button"
				class="toolbar-button"
				class:active={isFormatActive('strikethrough')}
				onclick={() => handleFormat('strikethrough')}
				{disabled}
				title="Strikethrough"
				aria-label="Strikethrough"
				aria-pressed={isFormatActive('strikethrough')}
			>
				<IconStrikethrough size={18} />
			</button>

			<button
				type="button"
				class="toolbar-button"
				class:active={isFormatActive('code')}
				onclick={() => handleFormat('code')}
				{disabled}
				title="Inline code ({getShortcutText('E')})"
				aria-label="Inline code"
				aria-pressed={isFormatActive('code')}
			>
				<IconCode size={18} />
			</button>

			<button
				type="button"
				class="toolbar-button"
				class:active={isFormatActive('link')}
				onclick={() => handleFormat('link')}
				{disabled}
				title="Insert link ({getShortcutText('K')})"
				aria-label="Insert link"
				aria-pressed={isFormatActive('link')}
			>
				<IconLink size={18} />
			</button>
		</div>

		<div class="toolbar-divider" role="separator"></div>

		<!-- Text Alignment -->
		<div class="toolbar-group" role="group" aria-label="Text alignment">
			<button
				type="button"
				class="toolbar-button"
				class:active={currentAlign === 'left'}
				onclick={() => handleAlignClick('left')}
				{disabled}
				title="Align left"
				aria-label="Align left"
				aria-pressed={currentAlign === 'left'}
			>
				<IconAlignLeft size={18} />
			</button>

			<button
				type="button"
				class="toolbar-button"
				class:active={currentAlign === 'center'}
				onclick={() => handleAlignClick('center')}
				{disabled}
				title="Align center"
				aria-label="Align center"
				aria-pressed={currentAlign === 'center'}
			>
				<IconAlignCenter size={18} />
			</button>

			<button
				type="button"
				class="toolbar-button"
				class:active={currentAlign === 'right'}
				onclick={() => handleAlignClick('right')}
				{disabled}
				title="Align right"
				aria-label="Align right"
				aria-pressed={currentAlign === 'right'}
			>
				<IconAlignRight size={18} />
			</button>
		</div>

		<div class="toolbar-divider" role="separator"></div>

		<!-- Lists -->
		<div class="toolbar-group" role="group" aria-label="Lists">
			<button
				type="button"
				class="toolbar-button"
				onclick={() => handleListClick('bullet')}
				{disabled}
				title="Bullet list"
				aria-label="Bullet list"
			>
				<IconList size={18} />
			</button>

			<button
				type="button"
				class="toolbar-button"
				onclick={() => handleListClick('numbered')}
				{disabled}
				title="Numbered list"
				aria-label="Numbered list"
			>
				<IconListNumbers size={18} />
			</button>
		</div>
	</div>

	<!-- Center Section: Document Stats -->
	<div class="toolbar-section toolbar-center">
		<div class="document-stats" role="status" aria-label="Document statistics">
			<span class="stat-item" title="Word count">
				<span class="stat-value">{wordCount.toLocaleString()}</span>
				<span class="stat-label">words</span>
			</span>

			<span class="stat-divider">|</span>

			<span class="stat-item" title="Estimated reading time">
				<span class="stat-value">{readingTime}</span>
				<span class="stat-label">min read</span>
			</span>

			{#if showCharacterCount}
				<span class="stat-divider">|</span>
				<span class="stat-item" title="Character count">
					<span class="stat-value">{characterCount.toLocaleString()}</span>
					<span class="stat-label">chars</span>
				</span>
			{/if}
		</div>
	</div>

	<!-- Right Section: Actions -->
	<div class="toolbar-section toolbar-right">
		<!-- Undo/Redo -->
		<div class="toolbar-group" role="group" aria-label="History">
			<button
				type="button"
				class="toolbar-button"
				onclick={() => onUndo?.()}
				disabled={disabled || !canUndo}
				title="Undo ({getShortcutText('Z')})"
				aria-label="Undo"
			>
				<IconArrowBackUp size={18} />
			</button>

			<button
				type="button"
				class="toolbar-button"
				onclick={() => onRedo?.()}
				disabled={disabled || !canRedo}
				title="Redo ({getShortcutText('Z', true)})"
				aria-label="Redo"
			>
				<IconArrowForwardUp size={18} />
			</button>
		</div>

		<div class="toolbar-divider" role="separator"></div>

		<!-- Focus Mode -->
		<button
			type="button"
			class="toolbar-button"
			class:active={isFocusMode}
			onclick={() => onFocusMode?.()}
			{disabled}
			title="Focus mode"
			aria-label="Toggle focus mode"
			aria-pressed={isFocusMode}
		>
			<IconFocus size={18} />
		</button>

		<!-- Preview -->
		<button
			type="button"
			class="toolbar-button"
			onclick={() => onPreview?.()}
			{disabled}
			title="Preview ({getShortcutText('P')})"
			aria-label="Preview"
		>
			<IconEye size={18} />
		</button>

		<div class="toolbar-divider" role="separator"></div>

		<!-- Save Status -->
		<div class="save-status" role="status" aria-live="polite">
			{#if saveStatus === 'saving'}
				<span class="status-indicator status-saving">
					<IconCloudUpload size={16} class="spin" />
					<span>Saving...</span>
				</span>
			{:else if saveStatus === 'saved'}
				<span class="status-indicator status-saved" title="Last saved: {lastSavedText}">
					<IconCloud size={16} />
					<span>Saved</span>
				</span>
			{:else if saveStatus === 'unsaved'}
				<span class="status-indicator status-unsaved">
					<IconAlertCircle size={16} />
					<span>Unsaved changes</span>
				</span>
			{/if}
		</div>

		<!-- Save Button -->
		<button
			type="button"
			class="save-button"
			onclick={() => onSave?.()}
			disabled={disabled || isSaving || !hasUnsavedChanges}
			title="Save ({getShortcutText('S')})"
			aria-label="Save document"
		>
			<IconDeviceFloppy size={16} />
			<span>Save</span>
		</button>

		<!-- More Actions Dropdown -->
		<div class="dropdown-container">
			<button
				bind:this={moreActionsButtonRef}
				type="button"
				class="toolbar-button more-actions-button"
				class:active={showMoreActionsDropdown}
				onclick={() => (showMoreActionsDropdown = !showMoreActionsDropdown)}
				{disabled}
				aria-haspopup="menu"
				aria-expanded={showMoreActionsDropdown}
				aria-label="More actions"
				title="More actions"
			>
				<IconDots size={18} />
			</button>

			{#if showMoreActionsDropdown}
				<div class="more-actions-dropdown" role="menu" aria-label="More actions">
					<button
						type="button"
						class="dropdown-item"
						onclick={() => {
							onPublish?.();
							showMoreActionsDropdown = false;
						}}
						role="menuitem"
					>
						<IconSend size={16} />
						<span>Publish</span>
					</button>

					<button
						type="button"
						class="dropdown-item"
						onclick={() => {
							onSchedule?.();
							showMoreActionsDropdown = false;
						}}
						role="menuitem"
					>
						<IconCalendarEvent size={16} />
						<span>Schedule</span>
					</button>

					<div class="dropdown-divider" role="separator"></div>

					<button
						type="button"
						class="dropdown-item"
						onclick={() => {
							onPreview?.();
							showMoreActionsDropdown = false;
						}}
						role="menuitem"
					>
						<IconEye size={16} />
						<span>Preview</span>
						<span class="shortcut-hint">{modKey}+P</span>
					</button>
				</div>
			{/if}
		</div>
	</div>
</div>

<style>
	.editor-toolbar {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0.5rem 1rem;
		background: white;
		border-bottom: 1px solid #e5e7eb;
		gap: 1rem;
		min-height: 48px;
		flex-wrap: wrap;
	}

	.editor-toolbar.disabled {
		opacity: 0.6;
		pointer-events: none;
	}

	.editor-toolbar.focus-mode {
		background: #fafafa;
	}

	/* Toolbar Sections */
	.toolbar-section {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.toolbar-left {
		flex: 1;
		min-width: 0;
	}

	.toolbar-center {
		flex-shrink: 0;
	}

	.toolbar-right {
		flex: 1;
		justify-content: flex-end;
		min-width: 0;
	}

	/* Toolbar Groups */
	.toolbar-group {
		display: flex;
		align-items: center;
		gap: 2px;
	}

	/* Toolbar Divider */
	.toolbar-divider {
		width: 1px;
		height: 24px;
		background: #e5e7eb;
		margin: 0 0.375rem;
		flex-shrink: 0;
	}

	/* Toolbar Button */
	.toolbar-button {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		padding: 0;
		background: transparent;
		border: none;
		border-radius: 6px;
		color: #6b7280;
		cursor: pointer;
		transition: all 0.15s ease;
		flex-shrink: 0;
	}

	.toolbar-button:hover:not(:disabled) {
		background: #f3f4f6;
		color: #1f2937;
	}

	.toolbar-button:focus-visible {
		outline: 2px solid #3b82f6;
		outline-offset: 2px;
	}

	.toolbar-button.active {
		background: #dbeafe;
		color: #2563eb;
	}

	.toolbar-button.active:hover:not(:disabled) {
		background: #bfdbfe;
	}

	.toolbar-button:disabled {
		opacity: 0.4;
		cursor: not-allowed;
	}

	/* Block Type Button */
	.block-type-button {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.375rem 0.75rem;
		background: #f9fafb;
		border: 1px solid #e5e7eb;
		border-radius: 6px;
		color: #374151;
		font-size: 0.8125rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.15s ease;
		min-width: 120px;
	}

	.block-type-button:hover:not(:disabled) {
		background: #f3f4f6;
		border-color: #d1d5db;
	}

	.block-type-button:focus-visible {
		outline: 2px solid #3b82f6;
		outline-offset: 2px;
	}

	.block-type-button.active {
		background: #eff6ff;
		border-color: #3b82f6;
	}

	.block-type-button:disabled {
		opacity: 0.4;
		cursor: not-allowed;
	}

	.block-type-label {
		flex: 1;
		text-align: left;
	}

	.block-type-button :global(.dropdown-icon) {
		transition: transform 0.15s ease;
	}

	.block-type-button.active :global(.dropdown-icon) {
		transform: rotate(180deg);
	}

	/* Dropdown Container */
	.dropdown-container {
		position: relative;
	}

	/* Block Type Dropdown */
	.block-type-dropdown {
		position: absolute;
		top: calc(100% + 4px);
		left: 0;
		min-width: 160px;
		background: white;
		border: 1px solid #e5e7eb;
		border-radius: 8px;
		box-shadow:
			0 4px 6px -1px rgba(0, 0, 0, 0.1),
			0 2px 4px -1px rgba(0, 0, 0, 0.06);
		padding: 0.375rem;
		z-index: 50;
		animation: dropdownFadeIn 0.15s ease;
	}

	/* More Actions Dropdown */
	.more-actions-dropdown {
		position: absolute;
		top: calc(100% + 4px);
		right: 0;
		min-width: 180px;
		background: white;
		border: 1px solid #e5e7eb;
		border-radius: 8px;
		box-shadow:
			0 4px 6px -1px rgba(0, 0, 0, 0.1),
			0 2px 4px -1px rgba(0, 0, 0, 0.06);
		padding: 0.375rem;
		z-index: 50;
		animation: dropdownFadeIn 0.15s ease;
	}

	@keyframes dropdownFadeIn {
		from {
			opacity: 0;
			transform: translateY(-4px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	/* Dropdown Item */
	.dropdown-item {
		display: flex;
		align-items: center;
		gap: 0.625rem;
		width: 100%;
		padding: 0.5rem 0.75rem;
		background: transparent;
		border: none;
		border-radius: 6px;
		color: #374151;
		font-size: 0.8125rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.15s ease;
		text-align: left;
	}

	.dropdown-item:hover {
		background: #f3f4f6;
	}

	.dropdown-item:focus-visible {
		outline: 2px solid #3b82f6;
		outline-offset: -2px;
	}

	.dropdown-item.selected {
		background: #eff6ff;
		color: #2563eb;
	}

	.dropdown-item :global(.check-icon) {
		margin-left: auto;
	}

	.shortcut-hint {
		margin-left: auto;
		font-size: 0.6875rem;
		color: #9ca3af;
		font-weight: 400;
	}

	.dropdown-divider {
		height: 1px;
		background: #e5e7eb;
		margin: 0.375rem 0;
	}

	/* Document Stats */
	.document-stats {
		display: flex;
		align-items: center;
		gap: 0.625rem;
		font-size: 0.8125rem;
		color: #6b7280;
	}

	.stat-item {
		display: flex;
		align-items: baseline;
		gap: 0.25rem;
	}

	.stat-value {
		font-weight: 600;
		color: #374151;
	}

	.stat-label {
		font-weight: 400;
	}

	.stat-divider {
		color: #d1d5db;
	}

	/* Save Status */
	.save-status {
		display: flex;
		align-items: center;
		margin-right: 0.5rem;
	}

	.status-indicator {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		font-size: 0.75rem;
		font-weight: 500;
		padding: 0.25rem 0.5rem;
		border-radius: 9999px;
	}

	.status-saving {
		color: #3b82f6;
		background: #eff6ff;
	}

	.status-saved {
		color: #10b981;
		background: #ecfdf5;
	}

	.status-unsaved {
		color: #f59e0b;
		background: #fffbeb;
	}

	.status-indicator :global(.spin) {
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

	/* Save Button */
	.save-button {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.5rem 1rem;
		background: #3b82f6;
		border: none;
		border-radius: 6px;
		color: white;
		font-size: 0.8125rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.save-button:hover:not(:disabled) {
		background: #2563eb;
	}

	.save-button:focus-visible {
		outline: 2px solid #3b82f6;
		outline-offset: 2px;
	}

	.save-button:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	/* More Actions Button */
	.more-actions-button {
		width: 32px;
		height: 32px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE DESIGN - Mobile First (Apple ICT 7 Standards)
	   Touch targets: 44x44px minimum on mobile
	   ═══════════════════════════════════════════════════════════════════════════ */

	/* Mobile Base (< 640px) */
	.editor-toolbar {
		flex-wrap: wrap;
		padding: 0.5rem;
		gap: 0.5rem;
	}

	.toolbar-left {
		order: 2;
		flex-basis: 100%;
		justify-content: center;
		overflow-x: auto;
		-webkit-overflow-scrolling: touch;
	}

	.toolbar-right {
		order: 1;
		flex-basis: 100%;
		justify-content: space-between;
		padding-bottom: 0.5rem;
		border-bottom: 1px solid #e5e7eb;
		margin-bottom: 0.5rem;
	}

	.toolbar-center {
		display: none;
	}

	.toolbar-divider {
		display: none;
	}

	.toolbar-button {
		width: 36px;
		height: 36px;
		min-width: 44px;
		min-height: 44px;
	}

	.toolbar-group {
		gap: 2px;
	}

	.block-type-button {
		min-width: 0;
		min-height: 44px;
	}

	.block-type-label {
		display: none;
	}

	.save-status span:not(:first-child) {
		display: none;
	}

	.save-button {
		min-height: 44px;
		padding: 0.5rem 0.75rem;
	}

	.save-button span {
		display: none;
	}

	.document-stats {
		display: none;
	}

	/* Tablet (≥ 640px) */
	@media (min-width: 640px) {
		.editor-toolbar {
			flex-wrap: nowrap;
			padding: 0.5rem 0.75rem;
			gap: 0.75rem;
		}

		.toolbar-left {
			order: 0;
			flex-basis: auto;
			justify-content: flex-start;
		}

		.toolbar-right {
			order: 0;
			flex-basis: auto;
			justify-content: flex-end;
			padding-bottom: 0;
			border-bottom: none;
			margin-bottom: 0;
		}

		.toolbar-button {
			width: 32px;
			height: 32px;
			min-width: 32px;
			min-height: 32px;
		}

		.block-type-button {
			min-height: auto;
		}

		.save-button {
			min-height: auto;
		}
	}

	/* Desktop (≥ 768px) */
	@media (min-width: 768px) {
		.toolbar-divider {
			display: block;
		}

		.block-type-label {
			display: inline;
		}

		.save-button span {
			display: inline;
		}
	}

	/* Large Desktop (≥ 1024px) */
	@media (min-width: 1024px) {
		.editor-toolbar {
			padding: 0.625rem 1rem;
			gap: 1rem;
		}

		.toolbar-center {
			display: flex;
		}

		.document-stats {
			display: flex;
		}
	}
</style>

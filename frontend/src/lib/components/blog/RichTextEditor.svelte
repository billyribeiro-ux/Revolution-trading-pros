<script lang="ts">
	import { onMount } from 'svelte';
	import {
		IconBold,
		IconItalic,
		IconUnderline,
		IconStrikethrough,
		IconAlignLeft,
		IconAlignCenter,
		IconAlignRight,
		IconAlignJustified,
		IconList,
		IconListNumbers,
		IconLink,
		IconPhoto,
		IconTable,
		IconCode,
		IconQuote,
		IconArrowBackUp,
		IconArrowForwardUp,
		IconChevronDown
	} from '$lib/icons';

	interface Props {
		content?: string;
		placeholder?: string;
		onchange?: (content: string) => void;
	}

	let { content = $bindable(''), placeholder = 'Start writing...', onchange }: Props = $props();

	let editor: HTMLDivElement;
	let showLinkDialog = $state(false);
	let linkUrl = $state('');
	let showImageDialog = $state(false);
	let imageUrl = $state('');
	let showColorPicker = $state(false);
	let textColor = $state('#000000');
	let showHeadingDropdown = $state(false);
	let selectedHeading = $state('Paragraph');

	const headingOptions = [
		{ label: 'Paragraph', value: 'p' },
		{ label: 'Heading 1', value: 'h1' },
		{ label: 'Heading 2', value: 'h2' },
		{ label: 'Heading 3', value: 'h3' },
		{ label: 'Heading 4', value: 'h4' },
		{ label: 'Heading 5', value: 'h5' },
		{ label: 'Heading 6', value: 'h6' }
	];

	const fonts = [
		'Arial',
		'Helvetica',
		'Times New Roman',
		'Georgia',
		'Verdana',
		'Courier New',
		'Comic Sans MS'
	];

	const fontSizes = [
		'12px',
		'14px',
		'16px',
		'18px',
		'20px',
		'24px',
		'28px',
		'32px',
		'36px',
		'48px'
	];

	let selectedFont = $state('Arial');
	let selectedFontSize = $state('16px');

	onMount(() => {
		if (content) {
			editor.innerHTML = content;
		}
	});

	function execCommand(command: string, value: string | undefined = undefined) {
		document.execCommand(command, false, value);
		editor.focus();
		updateContent();
	}

	function updateContent() {
		content = editor.innerHTML;
		onchange?.(content);
	}

	function insertHeading(value: string) {
		execCommand('formatBlock', value);
		const option = headingOptions.find(h => h.value === value);
		selectedHeading = option?.label || 'Paragraph';
		showHeadingDropdown = false;
	}

	function insertLink() {
		if (linkUrl) {
			execCommand('createLink', linkUrl);
			showLinkDialog = false;
			linkUrl = '';
		}
	}

	function insertImage() {
		if (imageUrl) {
			execCommand('insertImage', imageUrl);
			showImageDialog = false;
			imageUrl = '';
		}
	}

	function insertTable() {
		const rows = prompt('Number of rows:');
		const cols = prompt('Number of columns:');

		if (rows && cols) {
			const numRows = parseInt(rows, 10);
			const numCols = parseInt(cols, 10);

			if (isNaN(numRows) || isNaN(numCols) || numRows <= 0 || numCols <= 0) {
				alert('Please enter valid positive numbers for rows and columns.');
				return;
			}

			let table = '<table border="1" style="border-collapse: collapse; width: 100%;"><tbody>';

			for (let i = 0; i < numRows; i++) {
				table += '<tr>';
				for (let j = 0; j < numCols; j++) {
					table += '<td style="padding: 8px; border: 1px solid #ddd;">&nbsp;</td>';
				}
				table += '</tr>';
			}

			table += '</tbody></table><p><br></p>';
			execCommand('insertHTML', table);
		}
	}

	function changeFont(font: string) {
		selectedFont = font;
		execCommand('fontName', font);
	}

	function changeFontSize(size: string) {
		selectedFontSize = size;
		const selection = window.getSelection();
		if (selection && selection.toString()) {
			execCommand('fontSize', '7');
			// Change all font tags to have the selected size
			const fontTags = editor.querySelectorAll('font[size="7"]');
			fontTags.forEach((tag) => {
				tag.removeAttribute('size');
				(tag as HTMLElement).style.fontSize = size;
			});
			updateContent();
		}
	}

	function changeTextColor(color: string) {
		textColor = color;
		execCommand('foreColor', color);
		showColorPicker = false;
	}

	function handlePaste(e: ClipboardEvent) {
		e.preventDefault();
		const text = e.clipboardData?.getData('text/plain');
		if (text) {
			execCommand('insertText', text);
		}
	}
</script>

<div class="rich-text-editor">
	<div class="toolbar">
		<!-- Undo/Redo -->
		<div class="toolbar-group">
			<button type="button" class="toolbar-btn" onclick={() => execCommand('undo')} title="Undo">
				<IconArrowBackUp size={18} />
			</button>
			<button type="button" class="toolbar-btn" onclick={() => execCommand('redo')} title="Redo">
				<IconArrowForwardUp size={18} />
			</button>
		</div>

		<!-- Font Selection -->
		<div class="toolbar-group">
			<select
				class="font-select"
				bind:value={selectedFont}
				onchange={() => changeFont(selectedFont)}
			>
				{#each fonts as font}
					<option value={font}>{font}</option>
				{/each}
			</select>

			<select
				class="size-select"
				bind:value={selectedFontSize}
				onchange={() => changeFontSize(selectedFontSize)}
			>
				{#each fontSizes as size}
					<option value={size}>{size}</option>
				{/each}
			</select>
		</div>

		<!-- Headings Dropdown -->
		<div class="toolbar-group">
			<div class="heading-dropdown-wrapper">
				<button 
					type="button" 
					class="heading-dropdown-btn"
					onclick={() => showHeadingDropdown = !showHeadingDropdown}
				>
					<span>{selectedHeading}</span>
					<IconChevronDown size={16} />
				</button>
				{#if showHeadingDropdown}
					<div class="heading-dropdown">
						{#each headingOptions as option}
							<button 
								type="button"
								class="heading-option"
								class:active={selectedHeading === option.label}
								onclick={() => insertHeading(option.value)}
							>
								{option.label}
							</button>
						{/each}
					</div>
				{/if}
			</div>
		</div>

		<!-- Text Formatting -->
		<div class="toolbar-group">
			<button type="button" class="toolbar-btn" onclick={() => execCommand('bold')} title="Bold">
				<IconBold size={18} />
			</button>
			<button
				type="button"
				class="toolbar-btn"
				onclick={() => execCommand('italic')}
				title="Italic"
			>
				<IconItalic size={18} />
			</button>
			<button
				type="button"
				class="toolbar-btn"
				onclick={() => execCommand('underline')}
				title="Underline"
			>
				<IconUnderline size={18} />
			</button>
			<button
				type="button"
				class="toolbar-btn"
				onclick={() => execCommand('strikeThrough')}
				title="Strikethrough"
			>
				<IconStrikethrough size={18} />
			</button>
		</div>

		<!-- Color -->
		<div class="toolbar-group">
			<div class="color-picker-wrapper">
				<button
					type="button"
					class="toolbar-btn color-btn"
					onclick={() => (showColorPicker = !showColorPicker)}
					title="Text Color"
					style="background: {textColor}"
				>
					A
				</button>
				{#if showColorPicker}
					<div class="color-picker-dropdown">
						<input
							type="color"
							bind:value={textColor}
							onchange={() => changeTextColor(textColor)}
						/>
						<div class="preset-colors">
							{#each ['#000000', '#FF0000', '#00FF00', '#0000FF', '#FFFF00', '#FF00FF', '#00FFFF', '#FFFFFF'] as color}
								<button
									type="button"
									class="color-preset"
									style="background: {color}"
									onclick={() => changeTextColor(color)}
									aria-label="Set text color to {color}"
									title="Set text color to {color}"
								></button>
							{/each}
						</div>
					</div>
				{/if}
			</div>
		</div>

		<!-- Alignment -->
		<div class="toolbar-group">
			<button
				type="button"
				class="toolbar-btn"
				onclick={() => execCommand('justifyLeft')}
				title="Align Left"
			>
				<IconAlignLeft size={18} />
			</button>
			<button
				type="button"
				class="toolbar-btn"
				onclick={() => execCommand('justifyCenter')}
				title="Align Center"
			>
				<IconAlignCenter size={18} />
			</button>
			<button
				type="button"
				class="toolbar-btn"
				onclick={() => execCommand('justifyRight')}
				title="Align Right"
			>
				<IconAlignRight size={18} />
			</button>
			<button
				type="button"
				class="toolbar-btn"
				onclick={() => execCommand('justifyFull')}
				title="Justify"
			>
				<IconAlignJustified size={18} />
			</button>
		</div>

		<!-- Lists -->
		<div class="toolbar-group">
			<button
				type="button"
				class="toolbar-btn"
				onclick={() => execCommand('insertUnorderedList')}
				title="Bullet List"
			>
				<IconList size={18} />
			</button>
			<button
				type="button"
				class="toolbar-btn"
				onclick={() => execCommand('insertOrderedList')}
				title="Numbered List"
			>
				<IconListNumbers size={18} />
			</button>
			<button
				type="button"
				class="toolbar-btn"
				onclick={() => execCommand('formatBlock', 'blockquote')}
				title="Quote"
			>
				<IconQuote size={18} />
			</button>
		</div>

		<!-- Insert Elements -->
		<div class="toolbar-group">
			<button
				type="button"
				class="toolbar-btn"
				onclick={() => (showLinkDialog = true)}
				title="Insert Link"
			>
				<IconLink size={18} />
			</button>
			<button
				type="button"
				class="toolbar-btn"
				onclick={() => (showImageDialog = true)}
				title="Insert Image"
			>
				<IconPhoto size={18} />
			</button>
			<button type="button" class="toolbar-btn" onclick={insertTable} title="Insert Table">
				<IconTable size={18} />
			</button>
			<button
				type="button"
				class="toolbar-btn"
				onclick={() => execCommand('formatBlock', 'pre')}
				title="Code Block"
			>
				<IconCode size={18} />
			</button>
		</div>
	</div>

	<div
		bind:this={editor}
		class="editor-content"
		contenteditable="true"
		oninput={updateContent}
		onpaste={handlePaste}
		data-placeholder={placeholder}
	></div>
</div>

<!-- Link Dialog -->
{#if showLinkDialog}
	<div
		class="dialog-overlay"
		role="button"
		tabindex="0"
		onclick={() => (showLinkDialog = false)}
		onkeydown={(e) => e.key === 'Escape' && (showLinkDialog = false)}
	>
		<div
			class="dialog"
			role="dialog"
			aria-modal="true"
			tabindex="-1"
			onclick={(e) => e.stopPropagation()}
			onkeydown={(e) => e.stopPropagation()}
		>
			<h3>Insert Link</h3>
			<input type="url" bind:value={linkUrl} placeholder="https://example.com" />
			<div class="dialog-actions">
				<button type="button" class="btn-cancel" onclick={() => (showLinkDialog = false)}
					>Cancel</button
				>
				<button type="button" class="btn-insert" onclick={insertLink}>Insert</button>
			</div>
		</div>
	</div>
{/if}

<!-- Image Dialog -->
{#if showImageDialog}
	<div
		class="dialog-overlay"
		role="button"
		tabindex="0"
		onclick={() => (showImageDialog = false)}
		onkeydown={(e) => e.key === 'Escape' && (showImageDialog = false)}
	>
		<div
			class="dialog"
			role="dialog"
			aria-modal="true"
			tabindex="-1"
			onclick={(e) => e.stopPropagation()}
			onkeydown={(e) => e.stopPropagation()}
		>
			<h3>Insert Image</h3>
			<input type="url" bind:value={imageUrl} placeholder="https://example.com/image.jpg" />
			<div class="dialog-actions">
				<button type="button" class="btn-cancel" onclick={() => (showImageDialog = false)}
					>Cancel</button
				>
				<button type="button" class="btn-insert" onclick={insertImage}>Insert</button>
			</div>
		</div>
	</div>
{/if}

<style>
	.rich-text-editor {
		border: 1px solid #e5e5e5;
		border-radius: 8px;
		overflow: hidden;
		background: white;
	}

	.toolbar {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
		padding: 0.75rem;
		border-bottom: 1px solid #e5e5e5;
		background: #f8f9fa;
	}

	.toolbar-group {
		display: flex;
		gap: 0.25rem;
		padding: 0 0.5rem;
		border-right: 1px solid #e0e0e0;
	}

	.toolbar-group:last-child {
		border-right: none;
	}

	.toolbar-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		border: none;
		background: white;
		border-radius: 4px;
		cursor: pointer;
		color: #666;
		transition: all 0.2s;
	}

	.toolbar-btn:hover {
		background: #e0e0e0;
		color: #1a1a1a;
	}

	.toolbar-btn:active {
		background: #d0d0d0;
	}

	.color-btn {
		font-weight: 700;
		color: white;
		width: 36px;
	}

	.font-select,
	.size-select {
		padding: 0.375rem 0.5rem;
		border: 1px solid #e0e0e0;
		border-radius: 4px;
		font-size: 0.875rem;
		background: white;
		cursor: pointer;
	}

	.font-select {
		min-width: 120px;
	}

	.size-select {
		min-width: 70px;
	}

	/* Heading Dropdown */
	.heading-dropdown-wrapper {
		position: relative;
	}

	.heading-dropdown-btn {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.375rem 0.75rem;
		border: 1px solid #e0e0e0;
		border-radius: 4px;
		font-size: 0.875rem;
		background: white;
		cursor: pointer;
		min-width: 120px;
		justify-content: space-between;
		color: #1a1a1a;
	}

	.heading-dropdown-btn:hover {
		background: #f8f9fa;
	}

	.heading-dropdown {
		position: absolute;
		top: 100%;
		left: 0;
		margin-top: 0.25rem;
		background: white;
		border: 1px solid #e0e0e0;
		border-radius: 6px;
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
		z-index: 100;
		min-width: 140px;
		overflow: hidden;
	}

	.heading-option {
		display: block;
		width: 100%;
		padding: 0.625rem 1rem;
		border: none;
		background: none;
		text-align: left;
		cursor: pointer;
		font-size: 0.875rem;
		color: #1a1a1a;
		transition: background 0.15s;
	}

	.heading-option:hover {
		background: #f0f0f0;
	}

	.heading-option.active {
		background: #e0f2fe;
		color: #0369a1;
		font-weight: 500;
	}

	.color-picker-wrapper {
		position: relative;
	}

	.color-picker-dropdown {
		position: absolute;
		top: 100%;
		left: 0;
		margin-top: 0.5rem;
		padding: 1rem;
		background: white;
		border: 1px solid #e0e0e0;
		border-radius: 8px;
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
		z-index: 100;
	}

	.preset-colors {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 0.5rem;
		margin-top: 0.75rem;
	}

	.color-preset {
		width: 32px;
		height: 32px;
		border: 1px solid #e0e0e0;
		border-radius: 4px;
		cursor: pointer;
	}

	.editor-content {
		min-height: 300px;
		max-height: 600px;
		overflow-y: auto;
		padding: 1.5rem;
		font-size: 1rem;
		line-height: 1.6;
		outline: none;
		color: #1a1a1a;
		background: white;
	}

	.editor-content:empty::before {
		content: attr(data-placeholder);
		color: #999;
	}

	.editor-content :global(h1) {
		font-size: 2rem;
		font-weight: 700;
		margin: 1rem 0;
		color: #1a1a1a;
	}

	.editor-content :global(h2) {
		font-size: 1.5rem;
		font-weight: 600;
		margin: 0.875rem 0;
		color: #1a1a1a;
	}

	.editor-content :global(h3) {
		font-size: 1.25rem;
		font-weight: 600;
		margin: 0.75rem 0;
		color: #1a1a1a;
	}

	.editor-content :global(h4) {
		font-size: 1.125rem;
		font-weight: 600;
		margin: 0.75rem 0;
		color: #1a1a1a;
	}

	.editor-content :global(h5) {
		font-size: 1rem;
		font-weight: 600;
		margin: 0.5rem 0;
		color: #1a1a1a;
	}

	.editor-content :global(h6) {
		font-size: 0.875rem;
		font-weight: 600;
		margin: 0.5rem 0;
		color: #1a1a1a;
	}

	.editor-content :global(p) {
		margin: 0.5rem 0;
		color: #1a1a1a;
	}

	.editor-content :global(blockquote) {
		border-left: 4px solid #3b82f6;
		padding-left: 1rem;
		margin: 1rem 0;
		color: #666;
		font-style: italic;
	}

	.editor-content :global(pre) {
		background: #1e293b;
		color: #e2e8f0;
		padding: 1rem;
		border-radius: 6px;
		overflow-x: auto;
		font-family: 'Courier New', monospace;
		margin: 1rem 0;
	}

	.editor-content :global(table) {
		border-collapse: collapse;
		width: 100%;
		margin: 1rem 0;
	}

	.editor-content :global(table td),
	.editor-content :global(table th) {
		border: 1px solid #ddd;
		padding: 8px;
	}

	.editor-content :global(img) {
		max-width: 100%;
		height: auto;
		display: block;
		margin: 1rem 0;
	}

	.editor-content :global(a) {
		color: #3b82f6;
		text-decoration: underline;
	}

	.dialog-overlay {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background: rgba(0, 0, 0, 0.5);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
	}

	.dialog {
		background: white;
		padding: 2rem;
		border-radius: 12px;
		width: 100%;
		max-width: 500px;
		box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1);
	}

	.dialog h3 {
		margin: 0 0 1rem;
		font-size: 1.25rem;
		font-weight: 600;
	}

	.dialog input {
		width: 100%;
		padding: 0.75rem;
		border: 1px solid #e5e5e5;
		border-radius: 6px;
		font-size: 0.95rem;
		margin-bottom: 1rem;
	}

	.dialog-actions {
		display: flex;
		gap: 0.75rem;
		justify-content: flex-end;
	}

	.btn-cancel,
	.btn-insert {
		padding: 0.625rem 1.25rem;
		border-radius: 6px;
		font-weight: 500;
		cursor: pointer;
		border: none;
		transition: all 0.2s;
	}

	.btn-cancel {
		background: white;
		color: #666;
		border: 1px solid #e5e5e5;
	}

	.btn-cancel:hover {
		background: #f8f9fa;
	}

	.btn-insert {
		background: #3b82f6;
		color: white;
	}

	.btn-insert:hover {
		background: #2563eb;
	}
</style>

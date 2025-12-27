<!--
/**
 * Block Renderer - Renders different block types
 * ═══════════════════════════════════════════════════════════════════════════
 * Dynamic block rendering with inline editing support
 */
-->

<script lang="ts">
	import {
		IconPhoto,
		IconVideo,
		IconCheck,
		IconChartCandle,
		IconAlertTriangle,
		IconPlus,
		IconX,
		IconGripVertical
	} from '$lib/icons';

	import type { Block, BlockContent } from './types';

	// ==========================================================================
	// Props
	// ==========================================================================

	interface Props {
		block: Block;
		isSelected?: boolean;
		isEditing?: boolean;
		onUpdate: (updates: Partial<Block>) => void;
	}

	let {
		block,
		isSelected = false,
		isEditing = true,
		onUpdate
	}: Props = $props();

	// ==========================================================================
	// State
	// ==========================================================================


	// ==========================================================================
	// Content Update
	// ==========================================================================

	function updateContent(updates: Partial<BlockContent>) {
		onUpdate({
			content: { ...block.content, ...updates }
		});
	}

	function handleTextInput(e: Event) {
		const target = e.target as HTMLElement;
		updateContent({ text: target.textContent || '' });
	}

	function handlePaste(e: ClipboardEvent) {
		e.preventDefault();
		const text = e.clipboardData?.getData('text/plain') || '';
		document.execCommand('insertText', false, text);
	}

	// List handling
	function addListItem(index: number) {
		const items = [...(block.content.listItems || [])];
		items.splice(index + 1, 0, '');
		updateContent({ listItems: items });
	}

	function updateListItem(index: number, value: string) {
		const items = [...(block.content.listItems || [])];
		items[index] = value;
		updateContent({ listItems: items });
	}

	function removeListItem(index: number) {
		const items = [...(block.content.listItems || [])];
		if (items.length > 1) {
			items.splice(index, 1);
			updateContent({ listItems: items });
		}
	}

	// Checklist handling
	function toggleCheckItem(itemId: string) {
		const items = block.content.items?.map(item =>
			item.id === itemId ? { ...item, checked: !item.checked } : item
		) || [];
		updateContent({ items });
	}

	// ==========================================================================
	// Style Computation
	// ==========================================================================

	function getBlockStyles(): string {
		const s = block.settings;
		const styles: string[] = [];

		if (s.textAlign) styles.push(`text-align: ${s.textAlign}`);
		if (s.fontSize) styles.push(`font-size: ${s.fontSize}`);
		if (s.fontWeight) styles.push(`font-weight: ${s.fontWeight}`);
		if (s.fontFamily) styles.push(`font-family: ${s.fontFamily}`);
		if (s.lineHeight) styles.push(`line-height: ${s.lineHeight}`);
		if (s.textColor) styles.push(`color: ${s.textColor}`);
		if (s.backgroundColor) styles.push(`background-color: ${s.backgroundColor}`);
		if (s.marginTop) styles.push(`margin-top: ${s.marginTop}`);
		if (s.marginBottom) styles.push(`margin-bottom: ${s.marginBottom}`);
		if (s.paddingTop) styles.push(`padding-top: ${s.paddingTop}`);
		if (s.paddingBottom) styles.push(`padding-bottom: ${s.paddingBottom}`);
		if (s.paddingLeft) styles.push(`padding-left: ${s.paddingLeft}`);
		if (s.paddingRight) styles.push(`padding-right: ${s.paddingRight}`);
		if (s.borderWidth) styles.push(`border-width: ${s.borderWidth}`);
		if (s.borderColor) styles.push(`border-color: ${s.borderColor}`);
		if (s.borderStyle) styles.push(`border-style: ${s.borderStyle}`);
		if (s.borderRadius) styles.push(`border-radius: ${s.borderRadius}`);
		if (s.boxShadow) styles.push(`box-shadow: ${s.boxShadow}`);

		return styles.join('; ');
	}
</script>

<div class="block-renderer block-{block.type}" style={getBlockStyles()}>
	<!-- Paragraph Block -->
	{#if block.type === 'paragraph'}
		<p
			bind:this={editableRef}
			contenteditable={isEditing}
			class="editable-content"
			class:placeholder={!block.content.text}
			oninput={handleTextInput}
			onpaste={handlePaste}
			data-placeholder="Start writing or type / for commands..."
		>
			{block.content.text || ''}
		</p>

	<!-- Heading Block -->
	{:else if block.type === 'heading'}
		{@const level = block.settings.level || 2}
		<div class="heading-block-wrapper">
			{#if isEditing && isSelected}
				<div class="heading-level-selector">
					{#each [1, 2, 3, 4, 5, 6] as lvl}
						<button
							type="button"
							class="level-btn"
							class:active={level === lvl}
							onclick={() => onUpdate({ settings: { ...block.settings, level: lvl as 1 | 2 | 3 | 4 | 5 | 6 } })}
							title="Heading {lvl}"
						>
							H{lvl}
						</button>
					{/each}
				</div>
			{/if}
			<svelte:element
				this={`h${level}`}
				bind:this={editableRef}
				contenteditable={isEditing}
				class="editable-content heading-{level}"
				class:placeholder={!block.content.text}
				oninput={handleTextInput}
				onpaste={handlePaste}
				data-placeholder={`Heading ${level}`}
				id={block.settings.anchor || undefined}
			>
				{block.content.text || ''}
			</svelte:element>
		</div>

	<!-- Quote Block -->
	{:else if block.type === 'quote'}
		<blockquote class="quote-block">
			<div
				contenteditable={isEditing}
				class="quote-text editable-content"
				class:placeholder={!block.content.text}
				oninput={handleTextInput}
				onpaste={handlePaste}
				data-placeholder="Write a quote..."
			>
				{block.content.text || ''}
			</div>
			{#if block.content.html}
				<cite class="quote-cite">{block.content.html}</cite>
			{/if}
		</blockquote>

	<!-- Pull Quote Block -->
	{:else if block.type === 'pullquote'}
		<figure class="pullquote-block">
			<blockquote
				contenteditable={isEditing}
				class="pullquote-text editable-content"
				class:placeholder={!block.content.text}
				oninput={handleTextInput}
				onpaste={handlePaste}
				data-placeholder="Add a notable quote..."
			>
				{block.content.text || ''}
			</blockquote>
		</figure>

	<!-- Code Block -->
	{:else if block.type === 'code'}
		<div class="code-block">
			<div class="code-header">
				<select
					value={block.content.language || 'javascript'}
					onchange={(e: Event) => updateContent({ language: (e.target as HTMLSelectElement).value })}
					disabled={!isEditing}
				>
					<option value="javascript">JavaScript</option>
					<option value="typescript">TypeScript</option>
					<option value="python">Python</option>
					<option value="html">HTML</option>
					<option value="css">CSS</option>
					<option value="json">JSON</option>
					<option value="bash">Bash</option>
					<option value="sql">SQL</option>
					<option value="php">PHP</option>
					<option value="rust">Rust</option>
					<option value="go">Go</option>
				</select>
			</div>
			<pre class="code-content"><code
				contenteditable={isEditing}
				class="language-{block.content.language || 'javascript'}"
				oninput={(e: Event) => updateContent({ code: (e.target as HTMLElement).textContent || '' })}
				onpaste={handlePaste}
			>{block.content.code || ''}</code></pre>
		</div>

	<!-- List Block -->
	{:else if block.type === 'list'}
		{@const ListTag = block.content.listType === 'number' ? 'ol' : 'ul'}
		<svelte:element this={ListTag} class="list-block">
			{#each (block.content.listItems || ['']) as item, index}
				<li class="list-item">
					<span
						contenteditable={isEditing}
						class="list-text editable-content"
						oninput={(e: Event) => updateListItem(index, (e.target as HTMLElement).textContent || '')}
						onkeydown={(e: KeyboardEvent) => {
							if (e.key === 'Enter') {
								e.preventDefault();
								addListItem(index);
							} else if (e.key === 'Backspace' && !item && (block.content.listItems?.length || 0) > 1) {
								e.preventDefault();
								removeListItem(index);
							}
						}}
						onpaste={handlePaste}
						data-placeholder="List item"
					>
						{item}
					</span>
					{#if isEditing && (block.content.listItems?.length || 0) > 1}
						<button
							type="button"
							class="remove-item"
							onclick={() => removeListItem(index)}
						>
							<IconX size={14} />
						</button>
					{/if}
				</li>
			{/each}
		</svelte:element>
		{#if isEditing}
			<button type="button" class="add-item-btn" onclick={() => addListItem((block.content.listItems?.length || 1) - 1)}>
				<IconPlus size={14} />
				Add item
			</button>
		{/if}

	<!-- Checklist Block -->
	{:else if block.type === 'checklist'}
		<div class="checklist-block">
			{#each (block.content.items || []) as item}
				<label class="check-item" class:checked={item.checked}>
					<input
						type="checkbox"
						checked={item.checked}
						onchange={() => toggleCheckItem(item.id)}
						disabled={!isEditing}
					/>
					<span class="check-icon">
						{#if item.checked}
							<IconCheck size={14} />
						{/if}
					</span>
					<span
						class="check-text editable-content"
						oninput={(e: Event) => {
							const items = block.content.items?.map(i =>
							i.id === item.id ? { ...i, content: (e.target as HTMLElement).textContent || '' } : i
						);
						if (items) updateContent({ items });
						}}
						onpaste={handlePaste}
					>
						{item.content}
					</span>
				</label>
			{/each}
			{#if isEditing}
				<button
					type="button"
					class="add-item-btn"
					onclick={() => {
						const items = [...(block.content.items || []), {
							id: `item_${Date.now()}`,
							content: '',
							checked: false
						}];
						updateContent({ items });
					}}
				>
					<IconPlus size={14} />
					Add item
				</button>
			{/if}
		</div>

	<!-- Image Block -->
	{:else if block.type === 'image'}
		<figure class="image-block">
			{#if block.content.mediaUrl}
				<img
					src={block.content.mediaUrl}
					alt={block.content.mediaAlt || ''}
					style:object-fit={block.settings.objectFit || 'cover'}
				/>
				{#if block.content.mediaCaption || isEditing}
					<figcaption
						contenteditable={isEditing}
						class="image-caption editable-content"
						class:placeholder={!block.content.mediaCaption}
						oninput={(e: Event) => updateContent({ mediaCaption: (e.target as HTMLElement).textContent || '' })}
						data-placeholder="Add a caption..."
					>
						{block.content.mediaCaption || ''}
					</figcaption>
				{/if}
			{:else if isEditing}
				<div class="image-placeholder">
					<IconPhoto size={48} />
					<span>Click to add an image</span>
					<input
						type="file"
						accept="image/*"
						onchange={(e: Event) => {
							const file = (e.target as HTMLInputElement).files?.[0];
							if (file) {
								const url = URL.createObjectURL(file);
								updateContent({ mediaUrl: url });
							}
						}}
					/>
				</div>
			{/if}
		</figure>

	<!-- Video Block -->
	{:else if block.type === 'video'}
		<div class="video-block">
			{#if block.content.embedUrl}
				{@const isYouTube = block.content.embedUrl.includes('youtube') || block.content.embedUrl.includes('youtu.be')}
				{@const isVimeo = block.content.embedUrl.includes('vimeo')}
				{#if isYouTube}
					{@const videoId = block.content.embedUrl.match(/(?:youtube\.com\/watch\?v=|youtu\.be\/)([^&]+)/)?.[1]}
					<div class="video-embed">
						<iframe
							src="https://www.youtube.com/embed/{videoId}"
							title="YouTube video"
							frameborder="0"
							allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
							allowfullscreen
						></iframe>
					</div>
				{:else if isVimeo}
					{@const videoId = block.content.embedUrl.match(/vimeo\.com\/(\d+)/)?.[1]}
					<div class="video-embed">
						<iframe
							src="https://player.vimeo.com/video/{videoId}"
							title="Vimeo video"
							frameborder="0"
							allow="autoplay; fullscreen; picture-in-picture"
							allowfullscreen
						></iframe>
					</div>
				{:else}
					<video src={block.content.embedUrl} controls>
						<track kind="captions" src="" label="Captions" default />
						Your browser does not support the video tag.
					</video>
				{/if}
			{:else if isEditing}
				<div class="video-placeholder">
					<IconVideo size={48} />
					<span>Paste a YouTube or Vimeo URL</span>
					<input
						type="url"
						placeholder="https://youtube.com/watch?v=..."
						onchange={(e: Event) => updateContent({ embedUrl: (e.target as HTMLInputElement).value })}
					/>
				</div>
			{/if}
		</div>

	<!-- Separator Block -->
	{:else if block.type === 'separator'}
		<hr class="separator-block" />

	<!-- Spacer Block -->
	{:else if block.type === 'spacer'}
		<div
			class="spacer-block"
			style:height={block.settings.height || '40px'}
		>
			{#if isEditing}
				<div class="spacer-handle">
					<IconGripVertical size={14} />
					<span>{block.settings.height || '40px'}</span>
				</div>
			{/if}
		</div>

	<!-- Button Block -->
	{:else if block.type === 'button'}
		<div class="button-block-wrapper">
			<a
				href={block.settings.linkUrl || '#'}
				class="button-block button-{block.settings.buttonStyle || 'primary'} button-size-{block.settings.buttonSize || 'medium'}"
				target={block.settings.linkTarget || '_self'}
				onclick={(e: MouseEvent) => isEditing && e.preventDefault()}
			>
				<span
					contenteditable={isEditing}
					class="button-text"
					oninput={handleTextInput}
					onpaste={handlePaste}
				>
					{block.content.text || 'Click Here'}
				</span>
			</a>
		</div>

	<!-- Callout Block -->
	{:else if block.type === 'callout'}
		<div class="callout-block">
			<div class="callout-icon">
				<IconAlertTriangle size={24} />
			</div>
			<div
				contenteditable={isEditing}
				class="callout-content editable-content"
				class:placeholder={!block.content.text}
				oninput={handleTextInput}
				onpaste={handlePaste}
				data-placeholder="Add a callout message..."
			>
				{block.content.text || ''}
			</div>
		</div>

	<!-- Trading Chart Block -->
	{:else if block.type === 'chart'}
		<div class="chart-block">
			<div class="chart-header">
				<IconChartCandle size={20} />
				<input
					type="text"
					value={block.content.ticker || 'SPY'}
					placeholder="Ticker symbol"
					onchange={(e: Event) => updateContent({ ticker: (e.target as HTMLInputElement).value })}
					disabled={!isEditing}
					class="ticker-input"
				/>
			</div>
			<div class="chart-placeholder" style:height={block.settings.height || '400px'}>
				<span>Trading chart for {block.content.ticker || 'SPY'}</span>
				<small>TradingView widget will render here</small>
			</div>
		</div>

	<!-- Risk Disclaimer Block -->
	{:else if block.type === 'riskDisclaimer'}
		<div class="disclaimer-block">
			<IconAlertTriangle size={24} />
			<p>
				{block.content.text || 'Trading involves substantial risk of loss and is not suitable for all investors.'}
			</p>
		</div>

	<!-- Custom HTML Block -->
	{:else if block.type === 'html'}
		{#if isEditing}
			<div class="html-block-editor">
				<div class="html-header">Custom HTML</div>
				<textarea
					value={block.content.html || ''}
					oninput={(e: Event) => updateContent({ html: (e.target as HTMLTextAreaElement).value })}
					placeholder="<div>Your HTML here...</div>"
					rows="8"
				></textarea>
			</div>
		{:else}
			<div class="html-block-preview">
				{@html block.content.html || ''}
			</div>
		{/if}

	<!-- Default/Unknown Block -->
	{:else}
		<div class="unknown-block">
			<span>Unknown block type: {block.type}</span>
		</div>
	{/if}
</div>

<style>
	.block-renderer {
		min-height: 1em;
		position: relative;
	}

	/* Editable Content */
	.editable-content {
		outline: none;
		min-height: 1em;
	}

	.editable-content.placeholder:empty::before {
		content: attr(data-placeholder);
		color: #999;
		pointer-events: none;
	}

	/* Paragraph */
	.block-paragraph p {
		margin: 0;
		line-height: 1.7;
		font-size: 1rem;
	}

	/* Headings */
	.heading-block-wrapper {
		position: relative;
	}

	.heading-level-selector {
		display: flex;
		gap: 0.25rem;
		margin-bottom: 0.5rem;
		padding: 0.375rem;
		background: #f5f5f5;
		border-radius: 8px;
		width: fit-content;
	}

	.level-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		min-width: 32px;
		height: 28px;
		padding: 0 0.5rem;
		background: white;
		border: 1px solid #e5e5e5;
		border-radius: 4px;
		font-size: 0.75rem;
		font-weight: 600;
		color: #666;
		cursor: pointer;
		transition: all 0.15s;
	}

	.level-btn:hover {
		background: #e5e5e5;
		color: #1a1a1a;
	}

	.level-btn.active {
		background: #3b82f6;
		border-color: #3b82f6;
		color: white;
	}

	.heading-1 { font-size: 2.25rem; font-weight: 700; margin: 0; }
	.heading-2 { font-size: 1.875rem; font-weight: 600; margin: 0; }
	.heading-3 { font-size: 1.5rem; font-weight: 600; margin: 0; }
	.heading-4 { font-size: 1.25rem; font-weight: 600; margin: 0; }
	.heading-5 { font-size: 1.125rem; font-weight: 600; margin: 0; }
	.heading-6 { font-size: 1rem; font-weight: 600; margin: 0; }

	/* Quote */
	.quote-block {
		border-left: 4px solid #3b82f6;
		padding-left: 1.5rem;
		margin: 0;
	}

	.quote-text {
		font-size: 1.125rem;
		line-height: 1.7;
		font-style: italic;
		color: #374151;
	}

	.quote-cite {
		display: block;
		margin-top: 0.75rem;
		font-size: 0.875rem;
		color: #6b7280;
		font-style: normal;
	}

	/* Pull Quote */
	.pullquote-block {
		margin: 0;
		padding: 2rem;
		text-align: center;
		border-top: 2px solid #e5e7eb;
		border-bottom: 2px solid #e5e7eb;
	}

	.pullquote-text {
		font-size: 1.5rem;
		font-weight: 500;
		line-height: 1.5;
		color: #1f2937;
		margin: 0;
	}

	/* Code */
	.code-block {
		border-radius: 8px;
		overflow: hidden;
		background: #1e293b;
	}

	.code-header {
		padding: 0.5rem 1rem;
		background: #0f172a;
	}

	.code-header select {
		padding: 0.25rem 0.5rem;
		background: #334155;
		color: #e2e8f0;
		border: none;
		border-radius: 4px;
		font-size: 0.75rem;
	}

	.code-content {
		margin: 0;
		padding: 1rem;
		overflow-x: auto;
	}

	.code-content code {
		font-family: 'Fira Code', 'Consolas', monospace;
		font-size: 0.875rem;
		color: #e2e8f0;
		outline: none;
	}

	/* List */
	.list-block {
		margin: 0;
		padding-left: 1.5rem;
	}

	.list-item {
		display: flex;
		align-items: flex-start;
		gap: 0.5rem;
		margin-bottom: 0.375rem;
	}

	.list-text {
		flex: 1;
		line-height: 1.6;
	}

	.remove-item {
		display: flex;
		padding: 0.125rem;
		background: transparent;
		border: none;
		border-radius: 4px;
		cursor: pointer;
		color: #999;
		opacity: 0;
		transition: opacity 0.15s;
	}

	.list-item:hover .remove-item {
		opacity: 1;
	}

	.remove-item:hover {
		color: #ef4444;
	}

	.add-item-btn {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.375rem 0.75rem;
		margin-top: 0.5rem;
		background: transparent;
		border: 1px dashed #d1d5db;
		border-radius: 4px;
		color: #6b7280;
		font-size: 0.8125rem;
		cursor: pointer;
		transition: all 0.15s;
	}

	.add-item-btn:hover {
		border-color: #3b82f6;
		color: #3b82f6;
	}

	/* Checklist */
	.checklist-block {
		display: flex;
		flex-direction: column;
		gap: 0.375rem;
	}

	.check-item {
		display: flex;
		align-items: flex-start;
		gap: 0.625rem;
		cursor: pointer;
	}

	.check-item input[type="checkbox"] {
		position: absolute;
		opacity: 0;
	}

	.check-icon {
		width: 20px;
		height: 20px;
		display: flex;
		align-items: center;
		justify-content: center;
		border: 2px solid #d1d5db;
		border-radius: 4px;
		flex-shrink: 0;
		margin-top: 2px;
		transition: all 0.15s;
	}

	.check-item.checked .check-icon {
		background: #3b82f6;
		border-color: #3b82f6;
		color: white;
	}

	.check-text {
		flex: 1;
		line-height: 1.5;
	}

	.check-item.checked .check-text {
		text-decoration: line-through;
		color: #9ca3af;
	}

	/* Image */
	.image-block {
		margin: 0;
	}

	.image-block img {
		display: block;
		width: 100%;
		max-width: 100%;
		height: auto;
		border-radius: 8px;
	}

	.image-caption {
		margin-top: 0.75rem;
		text-align: center;
		font-size: 0.875rem;
		color: #6b7280;
	}

	.image-placeholder {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 3rem;
		background: #f9fafb;
		border: 2px dashed #d1d5db;
		border-radius: 8px;
		color: #9ca3af;
		cursor: pointer;
		position: relative;
	}

	.image-placeholder input {
		position: absolute;
		inset: 0;
		opacity: 0;
		cursor: pointer;
	}

	.image-placeholder span {
		margin-top: 0.75rem;
	}

	/* Video */
	.video-embed {
		position: relative;
		padding-bottom: 56.25%;
		height: 0;
		overflow: hidden;
		border-radius: 8px;
	}

	.video-embed iframe {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
	}

	.video-placeholder {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 3rem;
		background: #f9fafb;
		border: 2px dashed #d1d5db;
		border-radius: 8px;
		color: #9ca3af;
	}

	.video-placeholder input {
		margin-top: 1rem;
		padding: 0.5rem 1rem;
		border: 1px solid #d1d5db;
		border-radius: 6px;
		width: 100%;
		max-width: 400px;
	}

	/* Separator */
	.separator-block {
		border: none;
		border-top: 2px solid #e5e7eb;
		margin: 1rem 0;
	}

	/* Spacer */
	.spacer-block {
		position: relative;
		background: repeating-linear-gradient(
			45deg,
			transparent,
			transparent 10px,
			#f3f4f6 10px,
			#f3f4f6 20px
		);
		border-radius: 4px;
	}

	.spacer-handle {
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.25rem 0.5rem;
		background: white;
		border-radius: 4px;
		font-size: 0.75rem;
		color: #6b7280;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
	}

	/* Button */
	.button-block-wrapper {
		display: flex;
	}

	.button-block {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		text-decoration: none;
		border-radius: 8px;
		font-weight: 500;
		transition: all 0.15s;
	}

	.button-primary {
		background: #3b82f6;
		color: white;
	}

	.button-primary:hover {
		background: #2563eb;
	}

	.button-secondary {
		background: #f3f4f6;
		color: #1f2937;
	}

	.button-outline {
		background: transparent;
		border: 2px solid #3b82f6;
		color: #3b82f6;
	}

	.button-ghost {
		background: transparent;
		color: #3b82f6;
	}

	.button-size-small {
		padding: 0.5rem 1rem;
		font-size: 0.875rem;
	}

	.button-size-medium {
		padding: 0.75rem 1.5rem;
		font-size: 1rem;
	}

	.button-size-large {
		padding: 1rem 2rem;
		font-size: 1.125rem;
	}

	.button-text {
		outline: none;
	}

	/* Callout */
	.callout-block {
		display: flex;
		gap: 1rem;
		padding: 1.25rem;
		background: #fef3c7;
		border-left: 4px solid #f59e0b;
		border-radius: 0 8px 8px 0;
	}

	.callout-icon {
		color: #d97706;
		flex-shrink: 0;
	}

	.callout-content {
		flex: 1;
		line-height: 1.6;
	}

	/* Chart */
	.chart-block {
		border: 1px solid #e5e7eb;
		border-radius: 8px;
		overflow: hidden;
	}

	.chart-header {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem 1rem;
		background: #f9fafb;
		border-bottom: 1px solid #e5e7eb;
	}

	.ticker-input {
		padding: 0.375rem 0.75rem;
		border: 1px solid #d1d5db;
		border-radius: 4px;
		font-family: monospace;
		font-weight: 600;
		text-transform: uppercase;
		width: 80px;
	}

	.chart-placeholder {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		background: #f3f4f6;
		color: #6b7280;
	}

	.chart-placeholder small {
		margin-top: 0.5rem;
		font-size: 0.75rem;
		color: #9ca3af;
	}

	/* Disclaimer */
	.disclaimer-block {
		display: flex;
		gap: 1rem;
		padding: 1.25rem;
		background: #fef2f2;
		border: 1px solid #fecaca;
		border-radius: 8px;
		color: #991b1b;
	}

	.disclaimer-block p {
		margin: 0;
		font-size: 0.875rem;
		line-height: 1.5;
	}

	/* HTML Block */
	.html-block-editor {
		border: 1px solid #e5e7eb;
		border-radius: 8px;
		overflow: hidden;
	}

	.html-header {
		padding: 0.5rem 1rem;
		background: #f9fafb;
		font-size: 0.75rem;
		font-weight: 500;
		color: #6b7280;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.html-block-editor textarea {
		width: 100%;
		padding: 1rem;
		border: none;
		font-family: monospace;
		font-size: 0.875rem;
		resize: vertical;
		outline: none;
	}

	/* Unknown */
	.unknown-block {
		padding: 1rem;
		background: #f9fafb;
		border: 1px dashed #d1d5db;
		border-radius: 8px;
		color: #6b7280;
		text-align: center;
	}
</style>

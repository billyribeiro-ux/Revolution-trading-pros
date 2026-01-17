<script lang="ts">
	/**
	 * MediaPreview Component
	 *
	 * Full-screen media preview with metadata display,
	 * variant comparison, and optimization controls.
	 */
	import type { MediaItem, MediaVariant } from '$lib/api/media';

	interface Props {
		item?: MediaItem | null;
		isOpen?: boolean;
		onclose?: () => void;
		onoptimize?: (item: MediaItem) => void;
		ondelete?: (item: MediaItem) => void;
		onupdate?: (data: { id: string; data: Partial<MediaItem> }) => void;
	}

	let {
		item = null,
		isOpen = $bindable(false),
		onclose,
		onoptimize,
		ondelete,
		onupdate
	}: Props = $props();

	// State
	let activeTab: 'preview' | 'variants' | 'metadata' = $state('preview');
	let selectedVariant: MediaVariant | null = $state(null);
	let isEditing = $state(false);
	let editData = $state({
		alt_text: '',
		title: '',
		caption: ''
	});

	$effect(() => {
		if (item) {
			editData = {
				alt_text: item.alt_text || '',
				title: item.title || '',
				caption: item.caption || ''
			};
		}
	});

	function handleClose() {
		isOpen = false;
		onclose?.();
	}

	function handleOptimize() {
		if (item) onoptimize?.(item);
	}

	function handleDelete() {
		if (item && confirm('Are you sure you want to delete this file?')) {
			ondelete?.(item);
		}
	}

	function handleSave() {
		if (item) {
			onupdate?.({ id: item.id, data: editData });
			isEditing = false;
		}
	}

	function formatBytes(bytes: number): string {
		if (bytes < 1024) return bytes + ' B';
		if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB';
		return (bytes / 1024 / 1024).toFixed(2) + ' MB';
	}

	function formatDate(dateStr: string): string {
		return new Date(dateStr).toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'long',
			day: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') handleClose();
	}
</script>

<svelte:window onkeydown={handleKeydown} />

{#if isOpen && item}
	<!-- svelte-ignore a11y_click_events_have_key_events -->
	<!-- svelte-ignore a11y_interactive_supports_focus -->
	<div class="preview-overlay" onclick={handleClose} role="dialog" aria-modal="true">
		<!-- svelte-ignore a11y_click_events_have_key_events -->
		<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
		<div class="preview-modal" onclick={(e: MouseEvent) => e.stopPropagation()} role="document">
			<!-- Header -->
			<div class="preview-header">
				<h2 class="preview-title">{item.filename}</h2>
				<button type="button" class="close-btn" onclick={handleClose} aria-label="Close">
					<svg
						width="24"
						height="24"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
					>
						<path d="M18 6L6 18M6 6l12 12" />
					</svg>
				</button>
			</div>

			<!-- Tabs -->
			<div class="preview-tabs">
				<button
					type="button"
					class="tab-btn"
					class:active={activeTab === 'preview'}
					onclick={() => (activeTab = 'preview')}
				>
					Preview
				</button>
				{#if item.variants && item.variants.length > 0}
					<button
						type="button"
						class="tab-btn"
						class:active={activeTab === 'variants'}
						onclick={() => (activeTab = 'variants')}
					>
						Variants ({item.variants.length})
					</button>
				{/if}
				<button
					type="button"
					class="tab-btn"
					class:active={activeTab === 'metadata'}
					onclick={() => (activeTab = 'metadata')}
				>
					Metadata
				</button>
			</div>

			<!-- Content -->
			<div class="preview-content">
				{#if activeTab === 'preview'}
					<div class="preview-main">
						<!-- Image preview -->
						<div class="preview-image">
							{#if item.file_type === 'image'}
								<img src={selectedVariant?.url || item.url} alt={item.alt_text || item.filename} />
							{:else if item.file_type === 'video'}
								<video controls src={item.url}>
									<track kind="captions" />
								</video>
							{:else}
								<div class="file-preview">
									<svg
										width="64"
										height="64"
										viewBox="0 0 24 24"
										fill="none"
										stroke="currentColor"
										stroke-width="1.5"
									>
										<path d="M14 2H6a2 2 0 00-2 2v16a2 2 0 002 2h12a2 2 0 002-2V8z" />
										<path d="M14 2v6h6M16 13H8M16 17H8M10 9H8" />
									</svg>
									<span>{item.filename}</span>
								</div>
							{/if}
						</div>

						<!-- Sidebar -->
						<div class="preview-sidebar">
							<!-- Status -->
							<div class="info-section">
								<h3>Status</h3>
								<div class="status-row">
									<span
										class="status-badge"
										class:optimized={item.is_optimized}
										class:pending={!item.is_optimized}
									>
										{item.is_optimized ? 'Optimized' : 'Not Optimized'}
									</span>
									{#if !item.is_optimized && item.file_type === 'image'}
										<button type="button" class="btn-primary btn-sm" onclick={handleOptimize}>
											Optimize Now
										</button>
									{/if}
								</div>
							</div>

							<!-- Details -->
							<div class="info-section">
								<h3>Details</h3>
								<dl class="info-list">
									<dt>Type</dt>
									<dd>{item.mime_type}</dd>
									<dt>Size</dt>
									<dd>{formatBytes(item.file_size)}</dd>
									{#if item.width && item.height}
										<dt>Dimensions</dt>
										<dd>{item.width} x {item.height}</dd>
									{/if}
									<dt>Uploaded</dt>
									<dd>{item.uploaded_at ? formatDate(item.uploaded_at) : 'Unknown'}</dd>
									{#if item.collection}
										<dt>Collection</dt>
										<dd>{item.collection}</dd>
									{/if}
								</dl>
							</div>

							<!-- Edit metadata -->
							<div class="info-section">
								<div class="section-header">
									<h3>Metadata</h3>
									{#if !isEditing}
										<button type="button" class="btn-text" onclick={() => (isEditing = true)}>
											Edit
										</button>
									{/if}
								</div>

								{#if isEditing}
									<div class="edit-form">
										<label>
											<span>Alt Text</span>
											<input
												type="text"
												bind:value={editData.alt_text}
												placeholder="Describe the image..."
											/>
										</label>
										<label>
											<span>Title</span>
											<input type="text" bind:value={editData.title} />
										</label>
										<label>
											<span>Caption</span>
											<textarea bind:value={editData.caption} rows="2"></textarea>
										</label>
										<div class="edit-actions">
											<button
												type="button"
												class="btn-secondary btn-sm"
												onclick={() => (isEditing = false)}
											>
												Cancel
											</button>
											<button type="button" class="btn-primary btn-sm" onclick={handleSave}>
												Save
											</button>
										</div>
									</div>
								{:else}
									<dl class="info-list">
										<dt>Alt Text</dt>
										<dd>{item.alt_text || '—'}</dd>
										<dt>Title</dt>
										<dd>{item.title || '—'}</dd>
										<dt>Caption</dt>
										<dd>{item.caption || '—'}</dd>
									</dl>
								{/if}
							</div>

							<!-- Actions -->
							<div class="info-section">
								<h3>Actions</h3>
								<div class="action-buttons">
									<a href={item.url} download={item.filename} class="btn-secondary btn-sm">
										Download Original
									</a>
									<button type="button" class="btn-danger btn-sm" onclick={handleDelete}>
										Delete
									</button>
								</div>
							</div>
						</div>
					</div>
				{:else if activeTab === 'variants'}
					<!-- Variants comparison -->
					<div class="variants-grid">
						{#each item.variants || [] as variant}
							<div
								class="variant-card"
								class:selected={selectedVariant === variant}
								onclick={() => {
									selectedVariant = selectedVariant === variant ? null : variant;
									activeTab = 'preview';
								}}
								role="button"
								tabindex="0"
								onkeydown={(e: KeyboardEvent) => e.key === 'Enter' && (selectedVariant = variant)}
							>
								<div class="variant-preview">
									<img src={variant.url} alt="{variant.type} variant" loading="lazy" />
								</div>
								<div class="variant-info">
									<span class="variant-type">
										{variant.type}
										{variant.size ? `(${variant.size})` : ''}
										{variant.is_retina ? '@2x' : ''}
									</span>
									<span class="variant-details">
										{variant.width}x{variant.height} &bull; {formatBytes(variant.file_size)}
									</span>
									{#if variant.savings_percent}
										<span class="variant-savings">
											{variant.savings_percent.toFixed(1)}% smaller
										</span>
									{/if}
								</div>
							</div>
						{/each}
					</div>
				{:else if activeTab === 'metadata'}
					<!-- Full metadata -->
					<div class="metadata-view">
						{#if item.exif && Object.keys(item.exif).length > 0}
							<div class="metadata-section">
								<h3>EXIF Data</h3>
								<dl class="metadata-list">
									{#each Object.entries(item.exif) as [key, value]}
										{#if value}
											<dt>{key}</dt>
											<dd>{value}</dd>
										{/if}
									{/each}
								</dl>
							</div>
						{/if}

						{#if item.metadata}
							<div class="metadata-section">
								<h3>Optimization Data</h3>
								<pre>{JSON.stringify(item.metadata, null, 2)}</pre>
							</div>
						{/if}

						{#if item.tags && item.tags.length > 0}
							<div class="metadata-section">
								<h3>Tags</h3>
								<div class="tags-list">
									{#each item.tags as tag}
										<span class="tag">{tag}</span>
									{/each}
								</div>
							</div>
						{/if}
					</div>
				{/if}
			</div>
		</div>
	</div>
{/if}

<style>
	.preview-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.8);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
		padding: 2rem;
	}

	.preview-modal {
		width: 100%;
		max-width: 1200px;
		max-height: 90vh;
		background: var(--bg-primary, white);
		border-radius: 12px;
		display: flex;
		flex-direction: column;
		overflow: hidden;
	}

	.preview-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 1rem 1.5rem;
		border-bottom: 1px solid var(--border-color, #e5e7eb);
	}

	.preview-title {
		font-size: 1.125rem;
		font-weight: 600;
		color: var(--text-primary, #111827);
		margin: 0;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.close-btn {
		background: none;
		border: none;
		padding: 0.5rem;
		cursor: pointer;
		color: var(--text-muted, #6b7280);
		border-radius: 6px;
	}

	.close-btn:hover {
		background: var(--bg-hover, #f3f4f6);
		color: var(--text-primary, #111827);
	}

	.preview-tabs {
		display: flex;
		gap: 0.5rem;
		padding: 0 1.5rem;
		border-bottom: 1px solid var(--border-color, #e5e7eb);
	}

	.tab-btn {
		padding: 0.75rem 1rem;
		background: none;
		border: none;
		border-bottom: 2px solid transparent;
		font-size: 0.875rem;
		font-weight: 500;
		color: var(--text-muted, #6b7280);
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.tab-btn:hover {
		color: var(--text-primary, #111827);
	}

	.tab-btn.active {
		color: var(--primary-color, #3b82f6);
		border-bottom-color: var(--primary-color, #3b82f6);
	}

	.preview-content {
		flex: 1;
		overflow: auto;
		padding: 1.5rem;
	}

	.preview-main {
		display: grid;
		grid-template-columns: 1fr 320px;
		gap: 1.5rem;
		height: 100%;
	}

	@media (max-width: 768px) {
		.preview-main {
			grid-template-columns: 1fr;
		}
	}

	.preview-image {
		display: flex;
		align-items: center;
		justify-content: center;
		background: var(--bg-secondary, #f9fafb);
		border-radius: 8px;
		overflow: hidden;
		min-height: 400px;
	}

	.preview-image img,
	.preview-image video {
		max-width: 100%;
		max-height: 60vh;
		object-fit: contain;
	}

	.file-preview {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 1rem;
		color: var(--text-muted, #9ca3af);
	}

	.preview-sidebar {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}

	.info-section {
		background: var(--bg-secondary, #f9fafb);
		border-radius: 8px;
		padding: 1rem;
	}

	.info-section h3 {
		font-size: 0.75rem;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		color: var(--text-muted, #6b7280);
		margin: 0 0 0.75rem 0;
	}

	.section-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.section-header h3 {
		margin: 0;
	}

	.info-list {
		display: grid;
		grid-template-columns: auto 1fr;
		gap: 0.5rem 1rem;
		margin: 0;
	}

	.info-list dt {
		font-size: 0.75rem;
		color: var(--text-muted, #9ca3af);
	}

	.info-list dd {
		font-size: 0.875rem;
		color: var(--text-primary, #111827);
		margin: 0;
	}

	.status-row {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.status-badge {
		display: inline-flex;
		align-items: center;
		padding: 0.25rem 0.75rem;
		border-radius: 9999px;
		font-size: 0.75rem;
		font-weight: 500;
	}

	.status-badge.optimized {
		background: var(--success-bg, #d1fae5);
		color: var(--success-color, #059669);
	}

	.status-badge.pending {
		background: var(--warning-bg, #fef3c7);
		color: var(--warning-color, #d97706);
	}

	.edit-form {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.edit-form label {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.edit-form label span {
		font-size: 0.75rem;
		color: var(--text-muted, #6b7280);
	}

	.edit-form input,
	.edit-form textarea {
		padding: 0.5rem;
		border: 1px solid var(--border-color, #d1d5db);
		border-radius: 6px;
		font-size: 0.875rem;
	}

	.edit-actions {
		display: flex;
		gap: 0.5rem;
		margin-top: 0.5rem;
	}

	.action-buttons {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	/* Variants grid */
	.variants-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
		gap: 1rem;
	}

	.variant-card {
		background: var(--bg-secondary, #f9fafb);
		border: 2px solid transparent;
		border-radius: 8px;
		overflow: hidden;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.variant-card:hover,
	.variant-card.selected {
		border-color: var(--primary-color, #3b82f6);
	}

	.variant-preview {
		aspect-ratio: 16/9;
		background: var(--bg-tertiary, #e5e7eb);
	}

	.variant-preview img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.variant-info {
		padding: 0.75rem;
	}

	.variant-type {
		display: block;
		font-size: 0.875rem;
		font-weight: 500;
		color: var(--text-primary, #111827);
		text-transform: capitalize;
	}

	.variant-details {
		display: block;
		font-size: 0.75rem;
		color: var(--text-muted, #6b7280);
		margin-top: 0.25rem;
	}

	.variant-savings {
		display: inline-block;
		font-size: 0.75rem;
		color: var(--success-color, #10b981);
		font-weight: 500;
		margin-top: 0.25rem;
	}

	/* Metadata view */
	.metadata-view {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}

	.metadata-section {
		background: var(--bg-secondary, #f9fafb);
		border-radius: 8px;
		padding: 1rem;
	}

	.metadata-section h3 {
		font-size: 0.875rem;
		font-weight: 600;
		color: var(--text-primary, #111827);
		margin: 0 0 0.75rem 0;
	}

	.metadata-list {
		display: grid;
		grid-template-columns: auto 1fr;
		gap: 0.5rem 1rem;
		margin: 0;
	}

	.metadata-section pre {
		background: var(--bg-primary, white);
		padding: 0.75rem;
		border-radius: 6px;
		font-size: 0.75rem;
		overflow-x: auto;
		margin: 0;
	}

	.tags-list {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
	}

	.tag {
		display: inline-flex;
		padding: 0.25rem 0.75rem;
		background: var(--bg-primary, white);
		border-radius: 9999px;
		font-size: 0.75rem;
		color: var(--text-primary, #374151);
	}

	/* Buttons */
	.btn-primary,
	.btn-secondary,
	.btn-danger,
	.btn-text {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		padding: 0.5rem 1rem;
		border-radius: 6px;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s ease;
		border: none;
		text-decoration: none;
	}

	.btn-sm {
		padding: 0.375rem 0.75rem;
		font-size: 0.75rem;
	}

	.btn-primary {
		background: var(--primary-color, #3b82f6);
		color: white;
	}

	.btn-primary:hover {
		background: var(--primary-dark, #2563eb);
	}

	.btn-secondary {
		background: var(--bg-primary, white);
		color: var(--text-primary, #374151);
		border: 1px solid var(--border-color, #d1d5db);
	}

	.btn-secondary:hover {
		background: var(--bg-hover, #f9fafb);
	}

	.btn-danger {
		background: var(--error-color, #ef4444);
		color: white;
	}

	.btn-danger:hover {
		background: var(--error-dark, #dc2626);
	}

	.btn-text {
		background: none;
		padding: 0.25rem 0.5rem;
		color: var(--primary-color, #3b82f6);
	}

	.btn-text:hover {
		background: var(--bg-hover, #f3f4f6);
	}
</style>

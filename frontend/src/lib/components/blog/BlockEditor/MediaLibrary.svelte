<script lang="ts">
	/**
	 * Media Library - Enterprise-Grade Asset Manager
	 * ===============================================
	 * Full-featured media library with upload, editing,
	 * search, and organization capabilities.
	 *
	 * @version 1.0.0
	 * @author Revolution Trading Pros
	 */

	interface MediaItem {
		id: string;
		name: string;
		url: string;
		thumbnailUrl?: string;
		type: 'image' | 'video' | 'audio' | 'document' | 'other';
		mimeType: string;
		size: number;
		width?: number;
		height?: number;
		duration?: number;
		alt?: string;
		caption?: string;
		folder?: string;
		tags?: string[];
		createdAt: Date;
		updatedAt: Date;
	}

	interface Props {
		isOpen: boolean;
		onClose: () => void;
		onSelect: (media: MediaItem) => void;
		allowMultiple?: boolean;
		acceptTypes?: string[];
		initialFolder?: string;
	}

	let {
		isOpen,
		onClose,
		onSelect,
		allowMultiple = false,
		acceptTypes = ['image/*', 'video/*', 'audio/*', 'application/pdf'],
		initialFolder = ''
	}: Props = $props();

	// State
	let activeTab = $state<'library' | 'upload'>('library');
	let viewMode = $state<'grid' | 'list'>('grid');
	let selectedItems = $state<Set<string>>(new Set());
	let searchQuery = $state('');
	let currentFolder = $derived(initialFolder);
	let sortBy = $state<'date' | 'name' | 'size'>('date');
	let sortOrder = $state<'asc' | 'desc'>('desc');
	let filterType = $state<'all' | 'image' | 'video' | 'audio' | 'document'>('all');

	// Upload state
	let isDragging = $state(false);
	let uploadQueue = $state<File[]>([]);
	let uploadProgress = $state<Record<string, number>>({});
	let isUploading = $state(false);

	// Edit state
	let editingItem = $state<MediaItem | null>(null);
	let editForm = $state({ alt: '', caption: '', tags: '' });

	// Sample media items (in real app, fetch from API)
	let mediaItems = $state<MediaItem[]>([
		{
			id: '1',
			name: 'hero-image.jpg',
			url: '/uploads/hero-image.jpg',
			thumbnailUrl: '/uploads/thumbnails/hero-image.jpg',
			type: 'image',
			mimeType: 'image/jpeg',
			size: 245000,
			width: 1920,
			height: 1080,
			alt: 'Hero banner image',
			tags: ['hero', 'banner'],
			createdAt: new Date('2024-01-15'),
			updatedAt: new Date('2024-01-15')
		},
		{
			id: '2',
			name: 'product-demo.mp4',
			url: '/uploads/product-demo.mp4',
			thumbnailUrl: '/uploads/thumbnails/product-demo.jpg',
			type: 'video',
			mimeType: 'video/mp4',
			size: 15000000,
			width: 1280,
			height: 720,
			duration: 120,
			tags: ['product', 'demo'],
			createdAt: new Date('2024-01-10'),
			updatedAt: new Date('2024-01-10')
		}
	]);

	// Folders
	let folders = $state<string[]>(['Images', 'Videos', 'Documents', 'Marketing']);

	// Filtered and sorted items
	let filteredItems = $derived(() => {
		let items = [...mediaItems];

		// Filter by type
		if (filterType !== 'all') {
			items = items.filter(item => item.type === filterType);
		}

		// Filter by folder
		if (currentFolder) {
			items = items.filter(item => item.folder === currentFolder);
		}

		// Filter by search
		if (searchQuery) {
			const query = searchQuery.toLowerCase();
			items = items.filter(item =>
				item.name.toLowerCase().includes(query) ||
				item.alt?.toLowerCase().includes(query) ||
				item.tags?.some(t => t.toLowerCase().includes(query))
			);
		}

		// Sort
		items.sort((a, b) => {
			let comparison = 0;
			switch (sortBy) {
				case 'name':
					comparison = a.name.localeCompare(b.name);
					break;
				case 'size':
					comparison = a.size - b.size;
					break;
				case 'date':
				default:
					comparison = a.createdAt.getTime() - b.createdAt.getTime();
			}
			return sortOrder === 'desc' ? -comparison : comparison;
		});

		return items;
	});

	// Format file size
	function formatFileSize(bytes: number): string {
		if (bytes < 1024) return `${bytes} B`;
		if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
		if (bytes < 1024 * 1024 * 1024) return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
		return `${(bytes / (1024 * 1024 * 1024)).toFixed(1)} GB`;
	}

	// Format duration
	function formatDuration(seconds: number): string {
		const mins = Math.floor(seconds / 60);
		const secs = seconds % 60;
		return `${mins}:${secs.toString().padStart(2, '0')}`;
	}

	// Get type icon
	function getTypeIcon(type: string): string {
		switch (type) {
			case 'image': return '🖼️';
			case 'video': return '🎬';
			case 'audio': return '🎵';
			case 'document': return '📄';
			default: return '📁';
		}
	}

	// Handle file selection
	function handleFileSelect(e: Event): void {
		const input = e.target as HTMLInputElement;
		if (input.files) {
			addToUploadQueue(Array.from(input.files));
		}
	}

	// Handle drag over
	function handleDragOver(e: DragEvent): void {
		e.preventDefault();
		isDragging = true;
	}

	// Handle drag leave
	function handleDragLeave(): void {
		isDragging = false;
	}

	// Handle drop
	function handleDrop(e: DragEvent): void {
		e.preventDefault();
		isDragging = false;
		if (e.dataTransfer?.files) {
			addToUploadQueue(Array.from(e.dataTransfer.files));
		}
	}

	// Add files to upload queue
	function addToUploadQueue(files: File[]): void {
		const validFiles = files.filter(file =>
			acceptTypes.some(type => {
				if (type.endsWith('/*')) {
					return file.type.startsWith(type.slice(0, -1));
				}
				return file.type === type;
			})
		);
		uploadQueue = [...uploadQueue, ...validFiles];
	}

	// Remove from queue
	function removeFromQueue(index: number): void {
		uploadQueue = uploadQueue.filter((_, i) => i !== index);
	}

	// Start upload
	async function startUpload(): Promise<void> {
		if (uploadQueue.length === 0 || isUploading) return;

		isUploading = true;

		for (const file of uploadQueue) {
			uploadProgress[file.name] = 0;

			// Simulate upload progress
			for (let i = 0; i <= 100; i += 10) {
				await new Promise(resolve => setTimeout(resolve, 100));
				uploadProgress[file.name] = i;
				uploadProgress = { ...uploadProgress };
			}

			// Create media item
			const newItem: MediaItem = {
				id: crypto.randomUUID(),
				name: file.name,
				url: URL.createObjectURL(file),
				type: file.type.startsWith('image/') ? 'image' :
					file.type.startsWith('video/') ? 'video' :
					file.type.startsWith('audio/') ? 'audio' :
					file.type.includes('pdf') ? 'document' : 'other',
				mimeType: file.type,
				size: file.size,
				folder: currentFolder,
				createdAt: new Date(),
				updatedAt: new Date()
			};

			// Add dimensions for images
			if (file.type.startsWith('image/')) {
				const img = new Image();
				img.src = newItem.url;
				await new Promise<void>(resolve => {
					img.onload = () => {
						newItem.width = img.naturalWidth;
						newItem.height = img.naturalHeight;
						resolve();
					};
				});
			}

			mediaItems = [newItem, ...mediaItems];
		}

		uploadQueue = [];
		uploadProgress = {};
		isUploading = false;
		activeTab = 'library';
	}

	// Toggle item selection
	function toggleSelection(item: MediaItem): void {
		if (allowMultiple) {
			if (selectedItems.has(item.id)) {
				selectedItems.delete(item.id);
			} else {
				selectedItems.add(item.id);
			}
			selectedItems = new Set(selectedItems);
		} else {
			selectedItems = new Set([item.id]);
		}
	}

	// Handle insert
	function handleInsert(): void {
		const selected = mediaItems.filter(item => selectedItems.has(item.id));
		if (selected.length > 0 && selected[0]) {
			onSelect(selected[0]); // For single select
			onClose();
		}
	}

	// Open edit modal
	function openEditModal(item: MediaItem): void {
		editingItem = item;
		editForm = {
			alt: item.alt || '',
			caption: item.caption || '',
			tags: item.tags?.join(', ') || ''
		};
	}

	// Save edit
	function saveEdit(): void {
		if (editingItem) {
			const index = mediaItems.findIndex(i => i.id === editingItem!.id);
			if (index !== -1 && mediaItems[index]) {
				mediaItems[index] = {
					...mediaItems[index]!,
					alt: editForm.alt,
					caption: editForm.caption,
					tags: editForm.tags.split(',').map(t => t.trim()).filter(Boolean),
					updatedAt: new Date()
				};
				mediaItems = [...mediaItems];
			}
			editingItem = null;
		}
	}

	// Delete item
	function deleteItem(id: string): void {
		if (confirm('Are you sure you want to delete this item?')) {
			mediaItems = mediaItems.filter(item => item.id !== id);
			selectedItems.delete(id);
			selectedItems = new Set(selectedItems);
		}
	}

	// Close on escape
	function handleKeydown(e: KeyboardEvent): void {
		if (e.key === 'Escape' && isOpen) {
			if (editingItem) {
				editingItem = null;
			} else {
				onClose();
			}
		}
	}
</script>

<svelte:window onkeydown={handleKeydown} />

{#if isOpen}
	<div 
		class="media-overlay" 
		onclick={onClose}
		onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && onClose()}
		role="dialog"
		aria-modal="true"
		tabindex="-1"
	>
		<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
		<!-- svelte-ignore a11y_click_events_have_key_events -->
		<div 
			class="media-modal" 
			onclick={(e: MouseEvent) => e.stopPropagation()}
			role="document"
		>
			<!-- Header -->
			<div class="modal-header">
				<h2>Media Library</h2>
				<button class="close-btn" onclick={onClose} aria-label="Close">
					✕
				</button>
			</div>

			<!-- Tabs -->
			<div class="tabs">
				<button
					class="tab"
					class:active={activeTab === 'library'}
					onclick={() => activeTab = 'library'}
				>
					Library
				</button>
				<button
					class="tab"
					class:active={activeTab === 'upload'}
					onclick={() => activeTab = 'upload'}
				>
					Upload
					{#if uploadQueue.length > 0}
						<span class="badge">{uploadQueue.length}</span>
					{/if}
				</button>
			</div>

			<!-- Content -->
			<div class="modal-content">
				{#if activeTab === 'library'}
					<!-- Toolbar -->
					<div class="toolbar">
						<div class="toolbar-left">
							<div class="search-box">
								<span class="search-icon">🔍</span>
								<input
									type="text"
									placeholder="Search media..."
									bind:value={searchQuery}
								/>
							</div>

							<select bind:value={filterType} class="filter-select">
								<option value="all">All Types</option>
								<option value="image">Images</option>
								<option value="video">Videos</option>
								<option value="audio">Audio</option>
								<option value="document">Documents</option>
							</select>

							<select bind:value={sortBy} class="sort-select">
								<option value="date">Date</option>
								<option value="name">Name</option>
								<option value="size">Size</option>
							</select>

							<button
								class="sort-order-btn"
								onclick={() => sortOrder = sortOrder === 'asc' ? 'desc' : 'asc'}
								title={sortOrder === 'asc' ? 'Ascending' : 'Descending'}
							>
								{sortOrder === 'asc' ? '↑' : '↓'}
							</button>
						</div>

						<div class="toolbar-right">
							<div class="view-toggle">
								<button
									class:active={viewMode === 'grid'}
									onclick={() => viewMode = 'grid'}
									title="Grid view"
								>
									⊞
								</button>
								<button
									class:active={viewMode === 'list'}
									onclick={() => viewMode = 'list'}
									title="List view"
								>
									☰
								</button>
							</div>
						</div>
					</div>

					<!-- Sidebar + Grid -->
					<div class="library-layout">
						<!-- Folders Sidebar -->
						<div class="sidebar">
							<h4>Folders</h4>
							<ul class="folder-list">
								<li>
									<button
										class:active={currentFolder === ''}
										onclick={() => currentFolder = ''}
									>
										📁 All Media
									</button>
								</li>
								{#each folders as folder}
									<li>
										<button
											class:active={currentFolder === folder}
											onclick={() => currentFolder = folder}
										>
											📁 {folder}
										</button>
									</li>
								{/each}
							</ul>
						</div>

						<!-- Media Grid/List -->
						<div class="media-container">
							{#if filteredItems().length === 0}
								<div class="empty-state">
									<span class="empty-icon">📭</span>
									<p>No media found</p>
									<button class="upload-btn" onclick={() => activeTab = 'upload'}>
										Upload Files
									</button>
								</div>
							{:else if viewMode === 'grid'}
								<div class="media-grid">
									{#each filteredItems() as item}
										<div
											class="media-item"
											class:selected={selectedItems.has(item.id)}
											role="button"
											tabindex="0"
											onclick={() => toggleSelection(item)}
											ondblclick={() => {
												onSelect(item);
												onClose();
											}}
											onkeydown={(e: KeyboardEvent) => {
												if (e.key === 'Enter' || e.key === ' ') {
													toggleSelection(item);
												}
											}}
										>
											<div class="item-preview">
												{#if item.type === 'image'}
													<img src={item.thumbnailUrl || item.url} alt={item.alt || item.name} />
												{:else}
													<span class="type-icon">{getTypeIcon(item.type)}</span>
												{/if}
												{#if item.duration}
													<span class="duration">{formatDuration(item.duration)}</span>
												{/if}
											</div>
											<div class="item-info">
												<span class="item-name" title={item.name}>{item.name}</span>
												<span class="item-size">{formatFileSize(item.size)}</span>
											</div>
											<div class="item-actions">
												<button onclick={(e: MouseEvent) => { e.stopPropagation(); openEditModal(item); }} title="Edit">
													✏️
												</button>
												<button onclick={(e: MouseEvent) => { e.stopPropagation(); deleteItem(item.id); }} title="Delete">
													🗑️
												</button>
											</div>
										</div>
									{/each}
								</div>
							{:else}
								<div class="media-list">
									<table>
										<thead>
											<tr>
												<th></th>
												<th>Name</th>
												<th>Type</th>
												<th>Size</th>
												<th>Date</th>
												<th>Actions</th>
											</tr>
										</thead>
										<tbody>
											{#each filteredItems() as item}
												<tr
													class:selected={selectedItems.has(item.id)}
													onclick={() => toggleSelection(item)}
													ondblclick={() => {
														onSelect(item);
														onClose();
													}}
												>
													<td class="preview-cell">
														{#if item.type === 'image'}
															<img src={item.thumbnailUrl || item.url} alt={item.alt || item.name} />
														{:else}
															<span class="type-icon-small">{getTypeIcon(item.type)}</span>
														{/if}
													</td>
													<td class="name-cell">{item.name}</td>
													<td class="type-cell">{item.type}</td>
													<td class="size-cell">{formatFileSize(item.size)}</td>
													<td class="date-cell">{item.createdAt.toLocaleDateString()}</td>
													<td class="actions-cell">
														<button onclick={(e: MouseEvent) => { e.stopPropagation(); openEditModal(item); }}>
															✏️
														</button>
														<button onclick={(e: MouseEvent) => { e.stopPropagation(); deleteItem(item.id); }}>
															🗑️
														</button>
													</td>
												</tr>
											{/each}
										</tbody>
									</table>
								</div>
							{/if}
						</div>
					</div>

				{:else}
					<!-- Upload Tab -->
					<div class="upload-area">
						<div
							class="drop-zone"
							class:dragging={isDragging}
							ondragover={handleDragOver}
							ondragleave={handleDragLeave}
							ondrop={handleDrop}
							role="region"
							aria-label="File upload drop zone"
						>
							<span class="upload-icon">📤</span>
							<p class="upload-title">Drag and drop files here</p>
							<p class="upload-subtitle">or</p>
							<label class="browse-btn">
								Browse Files
								<input
									type="file"
									multiple={allowMultiple}
									accept={acceptTypes.join(',')}
									onchange={handleFileSelect}
									hidden
								/>
							</label>
							<p class="upload-hint">
								Accepts: {acceptTypes.map(t => t.replace('/*', 's')).join(', ')}
							</p>
						</div>

						{#if uploadQueue.length > 0}
							<div class="upload-queue">
								<h4>Upload Queue ({uploadQueue.length})</h4>
								<ul>
									{#each uploadQueue as file, i}
										<li>
											<span class="file-icon">{getTypeIcon(file.type.split('/')[0] ?? 'other')}</span>
											<div class="file-info">
												<span class="file-name">{file.name}</span>
												<span class="file-size">{formatFileSize(file.size)}</span>
												{#if uploadProgress[file.name] !== undefined}
													<div class="progress-bar">
														<div
															class="progress-fill"
															style="width: {uploadProgress[file.name]}%"
														></div>
													</div>
												{/if}
											</div>
											{#if !isUploading}
												<button class="remove-btn" onclick={() => removeFromQueue(i)}>
													✕
												</button>
											{/if}
										</li>
									{/each}
								</ul>
								<button
									class="start-upload-btn"
									onclick={startUpload}
									disabled={isUploading}
								>
									{isUploading ? 'Uploading...' : 'Start Upload'}
								</button>
							</div>
						{/if}
					</div>
				{/if}
			</div>

			<!-- Footer -->
			<div class="modal-footer">
				<div class="selected-count">
					{selectedItems.size} item{selectedItems.size !== 1 ? 's' : ''} selected
				</div>
				<div class="footer-actions">
					<button class="cancel-btn" onclick={onClose}>
						Cancel
					</button>
					<button
						class="insert-btn"
						onclick={handleInsert}
						disabled={selectedItems.size === 0}
					>
						Insert Selected
					</button>
				</div>
			</div>
		</div>
	</div>
{/if}

<!-- Edit Modal -->
{#if editingItem}
	<div 
		class="edit-overlay" 
		onclick={() => editingItem = null}
		onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && (editingItem = null)}
		role="dialog"
		aria-modal="true"
		tabindex="-1"
	>
		<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
		<!-- svelte-ignore a11y_click_events_have_key_events -->
		<div 
			class="edit-modal" 
			onclick={(e: MouseEvent) => e.stopPropagation()}
			role="document"
		>
			<h3>Edit Media</h3>
			<div class="edit-preview">
				{#if editingItem.type === 'image'}
					<img src={editingItem.url} alt={editingItem.name} />
				{:else}
					<span class="type-icon-large">{getTypeIcon(editingItem.type)}</span>
				{/if}
			</div>
			<div class="edit-form">
				<div class="field">
					<label for="media-alt-text">Alt Text</label>
					<input id="media-alt-text" type="text" bind:value={editForm.alt} placeholder="Describe this image" />
				</div>
				<div class="field">
					<label for="media-caption">Caption</label>
					<textarea id="media-caption" bind:value={editForm.caption} placeholder="Optional caption"></textarea>
				</div>
				<div class="field">
					<label for="media-tags">Tags</label>
					<input id="media-tags" type="text" bind:value={editForm.tags} placeholder="tag1, tag2, tag3" />
				</div>
			</div>
			<div class="edit-actions">
				<button class="cancel-btn" onclick={() => editingItem = null}>Cancel</button>
				<button class="save-btn" onclick={saveEdit}>Save Changes</button>
			</div>
		</div>
	</div>
{/if}

<style>
	.media-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.6);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
		padding: 2rem;
	}

	.media-modal {
		width: 100%;
		max-width: 1000px;
		height: 80vh;
		max-height: 700px;
		background: var(--bg-primary, #ffffff);
		border-radius: 0.75rem;
		display: flex;
		flex-direction: column;
		overflow: hidden;
		box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25);
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1rem 1.5rem;
		border-bottom: 1px solid var(--border-color, #e5e7eb);
	}

	.modal-header h2 {
		margin: 0;
		font-size: 1.25rem;
		font-weight: 600;
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
		transition: background 0.2s;
	}

	.close-btn:hover {
		background: var(--bg-hover, #f3f4f6);
	}

	.tabs {
		display: flex;
		padding: 0 1.5rem;
		border-bottom: 1px solid var(--border-color, #e5e7eb);
	}

	.tab {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.875rem 1.25rem;
		background: none;
		border: none;
		font-size: 0.875rem;
		font-weight: 500;
		color: var(--text-secondary, #6b7280);
		cursor: pointer;
		border-bottom: 2px solid transparent;
		margin-bottom: -1px;
		transition: all 0.2s;
	}

	.tab:hover {
		color: var(--text-primary, #1f2937);
	}

	.tab.active {
		color: var(--primary, #3b82f6);
		border-bottom-color: var(--primary, #3b82f6);
	}

	.badge {
		background: var(--primary, #3b82f6);
		color: white;
		padding: 0.125rem 0.5rem;
		border-radius: 9999px;
		font-size: 0.75rem;
	}

	.modal-content {
		flex: 1;
		overflow: hidden;
		display: flex;
		flex-direction: column;
	}

	.toolbar {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 0.75rem 1.5rem;
		border-bottom: 1px solid var(--border-color, #e5e7eb);
		gap: 1rem;
	}

	.toolbar-left {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.search-box {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 0.75rem;
		background: var(--bg-secondary, #f9fafb);
		border: 1px solid var(--border-color, #e5e7eb);
		border-radius: 0.375rem;
	}

	.search-icon {
		font-size: 0.875rem;
		opacity: 0.5;
	}

	.search-box input {
		width: 200px;
		border: none;
		background: transparent;
		font-size: 0.875rem;
		outline: none;
	}

	.filter-select,
	.sort-select {
		padding: 0.5rem 0.75rem;
		border: 1px solid var(--border-color, #e5e7eb);
		border-radius: 0.375rem;
		font-size: 0.875rem;
		background: var(--bg-primary, #ffffff);
	}

	.sort-order-btn {
		width: 32px;
		height: 32px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: var(--bg-secondary, #f9fafb);
		border: 1px solid var(--border-color, #e5e7eb);
		border-radius: 0.375rem;
		cursor: pointer;
	}

	.view-toggle {
		display: flex;
		border: 1px solid var(--border-color, #e5e7eb);
		border-radius: 0.375rem;
		overflow: hidden;
	}

	.view-toggle button {
		padding: 0.5rem 0.75rem;
		background: var(--bg-primary, #ffffff);
		border: none;
		cursor: pointer;
		transition: background 0.2s;
	}

	.view-toggle button:not(:last-child) {
		border-right: 1px solid var(--border-color, #e5e7eb);
	}

	.view-toggle button.active {
		background: var(--primary, #3b82f6);
		color: white;
	}

	.library-layout {
		flex: 1;
		display: flex;
		overflow: hidden;
	}

	.sidebar {
		width: 180px;
		padding: 1rem;
		border-right: 1px solid var(--border-color, #e5e7eb);
		overflow-y: auto;
	}

	.sidebar h4 {
		margin: 0 0 0.75rem;
		font-size: 0.6875rem;
		font-weight: 600;
		color: var(--text-secondary, #6b7280);
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.folder-list {
		list-style: none;
		margin: 0;
		padding: 0;
	}

	.folder-list li button {
		width: 100%;
		padding: 0.5rem 0.75rem;
		background: none;
		border: none;
		border-radius: 0.375rem;
		text-align: left;
		font-size: 0.8125rem;
		cursor: pointer;
		transition: background 0.2s;
	}

	.folder-list li button:hover {
		background: var(--bg-hover, #f3f4f6);
	}

	.folder-list li button.active {
		background: #eff6ff;
		color: var(--primary, #3b82f6);
	}

	.media-container {
		flex: 1;
		overflow-y: auto;
		padding: 1rem;
	}

	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		height: 100%;
		color: var(--text-secondary, #6b7280);
	}

	.empty-icon {
		font-size: 3rem;
		margin-bottom: 1rem;
	}

	.upload-btn {
		margin-top: 1rem;
		padding: 0.625rem 1.25rem;
		background: var(--primary, #3b82f6);
		color: white;
		border: none;
		border-radius: 0.375rem;
		cursor: pointer;
	}

	.media-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(150px, 1fr));
		gap: 1rem;
	}

	.media-item {
		display: flex;
		flex-direction: column;
		background: var(--bg-secondary, #f9fafb);
		border: 2px solid transparent;
		border-radius: 0.5rem;
		overflow: hidden;
		cursor: pointer;
		transition: all 0.2s;
	}

	.media-item:hover {
		border-color: var(--border-color, #e5e7eb);
	}

	.media-item.selected {
		border-color: var(--primary, #3b82f6);
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.2);
	}

	.item-preview {
		position: relative;
		aspect-ratio: 4/3;
		display: flex;
		align-items: center;
		justify-content: center;
		background: #e5e7eb;
	}

	.item-preview img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.type-icon {
		font-size: 2rem;
	}

	.duration {
		position: absolute;
		bottom: 0.25rem;
		right: 0.25rem;
		padding: 0.125rem 0.375rem;
		background: rgba(0, 0, 0, 0.7);
		color: white;
		border-radius: 0.25rem;
		font-size: 0.6875rem;
	}

	.item-info {
		padding: 0.5rem;
	}

	.item-name {
		display: block;
		font-size: 0.75rem;
		font-weight: 500;
		color: var(--text-primary, #1f2937);
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.item-size {
		font-size: 0.625rem;
		color: var(--text-tertiary, #9ca3af);
	}

	.item-actions {
		display: none;
		padding: 0.25rem 0.5rem;
		justify-content: flex-end;
		gap: 0.25rem;
	}

	.media-item:hover .item-actions {
		display: flex;
	}

	.item-actions button {
		padding: 0.25rem;
		background: none;
		border: none;
		cursor: pointer;
		opacity: 0.7;
	}

	.item-actions button:hover {
		opacity: 1;
	}

	.media-list {
		overflow-x: auto;
	}

	.media-list table {
		width: 100%;
		border-collapse: collapse;
	}

	.media-list th,
	.media-list td {
		padding: 0.75rem;
		text-align: left;
		border-bottom: 1px solid var(--border-color, #e5e7eb);
	}

	.media-list th {
		font-size: 0.75rem;
		font-weight: 600;
		color: var(--text-secondary, #6b7280);
		text-transform: uppercase;
	}

	.media-list tr {
		cursor: pointer;
		transition: background 0.2s;
	}

	.media-list tbody tr:hover {
		background: var(--bg-hover, #f9fafb);
	}

	.media-list tr.selected {
		background: #eff6ff;
	}

	.preview-cell {
		width: 50px;
	}

	.preview-cell img {
		width: 40px;
		height: 40px;
		object-fit: cover;
		border-radius: 0.25rem;
	}

	.type-icon-small {
		font-size: 1.5rem;
	}

	.upload-area {
		flex: 1;
		padding: 2rem;
		overflow-y: auto;
	}

	.drop-zone {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 3rem;
		border: 2px dashed var(--border-color, #e5e7eb);
		border-radius: 0.75rem;
		transition: all 0.2s;
	}

	.drop-zone.dragging {
		border-color: var(--primary, #3b82f6);
		background: #eff6ff;
	}

	.upload-icon {
		font-size: 3rem;
		margin-bottom: 1rem;
	}

	.upload-title {
		margin: 0;
		font-size: 1rem;
		font-weight: 600;
		color: var(--text-primary, #1f2937);
	}

	.upload-subtitle {
		margin: 0.5rem 0;
		color: var(--text-secondary, #6b7280);
	}

	.browse-btn {
		padding: 0.625rem 1.25rem;
		background: var(--primary, #3b82f6);
		color: white;
		border-radius: 0.375rem;
		cursor: pointer;
		font-size: 0.875rem;
		font-weight: 500;
	}

	.upload-hint {
		margin: 1rem 0 0;
		font-size: 0.75rem;
		color: var(--text-tertiary, #9ca3af);
	}

	.upload-queue {
		margin-top: 2rem;
		padding: 1rem;
		background: var(--bg-secondary, #f9fafb);
		border-radius: 0.5rem;
	}

	.upload-queue h4 {
		margin: 0 0 1rem;
		font-size: 0.875rem;
		font-weight: 600;
	}

	.upload-queue ul {
		list-style: none;
		margin: 0;
		padding: 0;
	}

	.upload-queue li {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem;
		background: var(--bg-primary, #ffffff);
		border-radius: 0.375rem;
		margin-bottom: 0.5rem;
	}

	.file-icon {
		font-size: 1.25rem;
	}

	.file-info {
		flex: 1;
	}

	.file-name {
		display: block;
		font-size: 0.875rem;
		font-weight: 500;
	}

	.file-size {
		font-size: 0.75rem;
		color: var(--text-tertiary, #9ca3af);
	}

	.progress-bar {
		height: 4px;
		background: #e5e7eb;
		border-radius: 2px;
		margin-top: 0.5rem;
		overflow: hidden;
	}

	.progress-fill {
		height: 100%;
		background: var(--primary, #3b82f6);
		transition: width 0.2s;
	}

	.remove-btn {
		padding: 0.25rem 0.5rem;
		background: none;
		border: none;
		cursor: pointer;
		color: var(--text-tertiary, #9ca3af);
	}

	.start-upload-btn {
		width: 100%;
		padding: 0.75rem;
		background: var(--primary, #3b82f6);
		color: white;
		border: none;
		border-radius: 0.375rem;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		margin-top: 1rem;
	}

	.start-upload-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.modal-footer {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1rem 1.5rem;
		border-top: 1px solid var(--border-color, #e5e7eb);
		background: var(--bg-secondary, #f9fafb);
	}

	.selected-count {
		font-size: 0.875rem;
		color: var(--text-secondary, #6b7280);
	}

	.footer-actions {
		display: flex;
		gap: 0.75rem;
	}

	.cancel-btn,
	.insert-btn {
		padding: 0.625rem 1.25rem;
		border-radius: 0.375rem;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.cancel-btn {
		background: var(--bg-primary, #ffffff);
		border: 1px solid var(--border-color, #e5e7eb);
		color: var(--text-secondary, #6b7280);
	}

	.insert-btn {
		background: var(--primary, #3b82f6);
		border: none;
		color: white;
	}

	.insert-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	/* Edit Modal */
	.edit-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.6);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1100;
	}

	.edit-modal {
		width: 100%;
		max-width: 500px;
		background: var(--bg-primary, #ffffff);
		border-radius: 0.75rem;
		padding: 1.5rem;
		box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25);
	}

	.edit-modal h3 {
		margin: 0 0 1rem;
		font-size: 1.125rem;
		font-weight: 600;
	}

	.edit-preview {
		width: 100%;
		aspect-ratio: 16/9;
		background: #e5e7eb;
		border-radius: 0.5rem;
		overflow: hidden;
		display: flex;
		align-items: center;
		justify-content: center;
		margin-bottom: 1rem;
	}

	.edit-preview img {
		width: 100%;
		height: 100%;
		object-fit: contain;
	}

	.type-icon-large {
		font-size: 4rem;
	}

	.edit-form {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.field label {
		display: block;
		margin-bottom: 0.375rem;
		font-size: 0.8125rem;
		font-weight: 500;
		color: var(--text-secondary, #6b7280);
	}

	.field input,
	.field textarea {
		width: 100%;
		padding: 0.625rem 0.875rem;
		border: 1px solid var(--border-color, #e5e7eb);
		border-radius: 0.375rem;
		font-size: 0.875rem;
	}

	.field textarea {
		min-height: 80px;
		resize: vertical;
	}

	.edit-actions {
		display: flex;
		justify-content: flex-end;
		gap: 0.75rem;
		margin-top: 1.5rem;
	}

	.save-btn {
		padding: 0.625rem 1.25rem;
		background: var(--primary, #3b82f6);
		color: white;
		border: none;
		border-radius: 0.375rem;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
	}
</style>

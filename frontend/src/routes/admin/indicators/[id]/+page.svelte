<script lang="ts">
	/**
	 * Admin Indicator Editor Page
	 * Apple Principal Engineer ICT 7 Grade - February 2026
	 * Full API integration for files, videos, and license management
	 */

	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { adminFetch } from '$lib/utils/adminFetch';

	// ICT 7: Match actual backend schema
	interface Indicator {
		id: number;
		name: string;
		slug: string;
		description?: string;
		long_description?: string;
		price?: number;
		is_active?: boolean;
		thumbnail?: string;
		platform?: string;
		version?: string;
		download_url?: string;
		documentation_url?: string;
		features?: any;
		requirements?: any;
		screenshots?: any;
		meta_title?: string;
		meta_description?: string;
		created_at?: string;
		updated_at?: string;
	}

	interface IndicatorFile {
		id: number;
		indicator_id: number;
		platform: string;
		file_name: string;
		display_name?: string;
		file_path?: string;
		file_size_bytes?: number;
		version?: string;
		is_current_version?: boolean;
		is_active?: boolean;
		display_order?: number;
		download_count?: number;
		created_at?: string;
	}

	interface IndicatorVideo {
		id: number;
		indicator_id: number;
		title: string;
		description?: string;
		video_url?: string;
		embed_url?: string;
		thumbnail_url?: string;
		duration_seconds?: number;
		display_order?: number;
		is_active?: boolean;
		created_at?: string;
	}

	let indicator = $state<Indicator | null>(null);
	let loading = $state(true);
	let saving = $state(false);
	let activeTab = $state<'details' | 'files' | 'videos' | 'seo'>('details');
	let error = $state('');
	let success = $state('');

	// Files and Videos state - ICT 7 with real API integration
	let files = $state<IndicatorFile[]>([]);
	let videos = $state<IndicatorVideo[]>([]);
	let loadingFiles = $state(false);
	let loadingVideos = $state(false);

	// File upload modal state
	let showFileModal = $state(false);
	let newFile = $state({
		platform: 'tradingview',
		display_name: '',
		version: '1.0',
		file: null as File | null
	});
	let uploadingFile = $state(false);

	// Video add modal state
	let showVideoModal = $state(false);
	let newVideo = $state({
		title: '',
		description: '',
		video_url: '',
		embed_url: '',
		thumbnail_url: ''
	});
	let addingVideo = $state(false);

	// Platform options
	const platformOptions = [
		{ value: 'tradingview', label: 'TradingView' },
		{ value: 'thinkorswim', label: 'ThinkorSwim' },
		{ value: 'metatrader', label: 'MetaTrader' },
		{ value: 'mt4', label: 'MetaTrader 4' },
		{ value: 'mt5', label: 'MetaTrader 5' },
		{ value: 'ninjatrader', label: 'NinjaTrader' },
		{ value: 'tradestation', label: 'TradeStation' },
		{ value: 'sierrachart', label: 'Sierra Chart' }
	];

	// Extract indicator ID from URL pathname
	let indicatorId = $state('');

	onMount(() => {
		const pathParts = window.location.pathname.split('/');
		indicatorId = pathParts[pathParts.length - 1];
		fetchIndicator();
	});

	const fetchIndicator = async () => {
		loading = true;
		try {
			const indicatorData = await adminFetch(`/api/admin/indicators/${indicatorId}`);
			if (indicatorData.success) {
				indicator = indicatorData.data;
				// Fetch files and videos after indicator loads
				fetchFiles();
				fetchVideos();
			}
		} catch (e) {
			error = 'Failed to load indicator';
			console.error(e);
		} finally {
			loading = false;
		}
	};

	// ICT 7: Fetch indicator files from real API
	const fetchFiles = async () => {
		loadingFiles = true;
		try {
			const data = await adminFetch(`/api/admin/indicators/${indicatorId}/files`);
			if (data.success) {
				files = data.data || [];
			}
		} catch (e) {
			console.error('Failed to load files:', e);
		} finally {
			loadingFiles = false;
		}
	};

	// ICT 7: Fetch indicator videos from real API
	const fetchVideos = async () => {
		loadingVideos = true;
		try {
			const data = await adminFetch(`/api/admin/indicators/${indicatorId}/videos`);
			if (data.success) {
				videos = data.data || [];
			}
		} catch (e) {
			console.error('Failed to load videos:', e);
		} finally {
			loadingVideos = false;
		}
	};

	const saveIndicator = async () => {
		if (!indicator) return;
		saving = true;
		error = '';
		success = '';

		try {
			const data = await adminFetch(`/api/admin/indicators/${indicatorId}`, {
				method: 'PUT',
				body: JSON.stringify(indicator)
			});

			if (data.success) {
				indicator = data.data;
				success = 'Indicator saved successfully';
				setTimeout(() => (success = ''), 3000);
			} else {
				error = data.error || 'Failed to save';
			}
		} catch (e) {
			error = 'Failed to save indicator';
		} finally {
			saving = false;
		}
	};

	const toggleIndicator = async () => {
		if (!indicator) return;
		try {
			const data = await adminFetch(`/api/admin/indicators/${indicatorId}/toggle`, {
				method: 'POST'
			});
			if (data.success) {
				indicator = data.data;
				success = indicator?.is_active ? 'Indicator activated!' : 'Indicator deactivated!';
				setTimeout(() => (success = ''), 3000);
			}
		} catch (e) {
			error = 'Failed to toggle status';
		}
	};

	// ICT 7: Upload new file
	const uploadFile = async () => {
		if (!newFile.file || !newFile.display_name) {
			error = 'Please select a file and provide a display name';
			return;
		}

		uploadingFile = true;
		error = '';

		try {
			const formData = new FormData();
			formData.append('file', newFile.file);
			formData.append('platform', newFile.platform);
			formData.append('display_name', newFile.display_name);
			formData.append('version', newFile.version);

			const response = await fetch(`/api/admin/indicators/${indicatorId}/files`, {
				method: 'POST',
				body: formData,
				credentials: 'include'
			});

			const data = await response.json();

			if (data.success) {
				files = [...files, data.data];
				showFileModal = false;
				newFile = { platform: 'tradingview', display_name: '', version: '1.0', file: null };
				success = 'File uploaded successfully';
				setTimeout(() => (success = ''), 3000);
			} else {
				error = data.error || 'Failed to upload file';
			}
		} catch (e) {
			error = 'Failed to upload file';
			console.error(e);
		} finally {
			uploadingFile = false;
		}
	};

	// ICT 7: Delete file
	const deleteFile = async (fileId: number) => {
		if (!confirm('Are you sure you want to delete this file?')) return;

		try {
			const data = await adminFetch(`/api/admin/indicators/${indicatorId}/files/${fileId}`, {
				method: 'DELETE'
			});

			if (data.success) {
				files = files.filter((f) => f.id !== fileId);
				success = 'File deleted successfully';
				setTimeout(() => (success = ''), 3000);
			} else {
				error = data.error || 'Failed to delete file';
			}
		} catch (e) {
			error = 'Failed to delete file';
		}
	};

	// ICT 7: Toggle file active status
	const toggleFileStatus = async (fileId: number) => {
		try {
			const file = files.find((f) => f.id === fileId);
			if (!file) return;

			const data = await adminFetch(`/api/admin/indicators/${indicatorId}/files/${fileId}`, {
				method: 'PUT',
				body: JSON.stringify({ is_active: !file.is_active })
			});

			if (data.success) {
				files = files.map((f) => (f.id === fileId ? data.data : f));
			}
		} catch (e) {
			error = 'Failed to update file status';
		}
	};

	// ICT 7: Add new video
	const addVideo = async () => {
		if (!newVideo.title) {
			error = 'Please provide a video title';
			return;
		}

		addingVideo = true;
		error = '';

		try {
			const data = await adminFetch(`/api/admin/indicators/${indicatorId}/videos`, {
				method: 'POST',
				body: JSON.stringify(newVideo)
			});

			if (data.success) {
				videos = [...videos, data.data];
				showVideoModal = false;
				newVideo = { title: '', description: '', video_url: '', embed_url: '', thumbnail_url: '' };
				success = 'Video added successfully';
				setTimeout(() => (success = ''), 3000);
			} else {
				error = data.error || 'Failed to add video';
			}
		} catch (e) {
			error = 'Failed to add video';
			console.error(e);
		} finally {
			addingVideo = false;
		}
	};

	// ICT 7: Delete video
	const deleteVideo = async (videoId: number) => {
		if (!confirm('Are you sure you want to delete this video?')) return;

		try {
			const data = await adminFetch(`/api/admin/indicators/${indicatorId}/videos/${videoId}`, {
				method: 'DELETE'
			});

			if (data.success) {
				videos = videos.filter((v) => v.id !== videoId);
				success = 'Video deleted successfully';
				setTimeout(() => (success = ''), 3000);
			} else {
				error = data.error || 'Failed to delete video';
			}
		} catch (e) {
			error = 'Failed to delete video';
		}
	};

	// Format file size for display
	const formatFileSize = (bytes?: number): string => {
		if (!bytes) return 'Unknown';
		if (bytes < 1024) return `${bytes} B`;
		if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
		return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
	};

	// Format duration for display
	const formatDuration = (seconds?: number): string => {
		if (!seconds) return '';
		const mins = Math.floor(seconds / 60);
		const secs = seconds % 60;
		return `${mins}:${secs.toString().padStart(2, '0')}`;
	};

	// Handle file input change
	const handleFileSelect = (e: Event) => {
		const target = e.target as HTMLInputElement;
		if (target.files && target.files.length > 0) {
			newFile.file = target.files[0];
			if (!newFile.display_name) {
				newFile.display_name = target.files[0].name;
			}
		}
	};
</script>

<svelte:head>
	<title>{indicator?.name || 'Edit Indicator'} | Admin</title>
</svelte:head>

<div class="editor-page">
	{#if loading}
		<div class="loading">
			<div class="spinner"></div>
			<p>Loading indicator...</p>
		</div>
	{:else if !indicator}
		<div class="error-state">
			<p>Indicator not found</p>
			<a href="/admin/indicators" class="btn-secondary">Back to Indicators</a>
		</div>
	{:else}
		<header class="page-header">
			<div class="header-left">
				<a href="/admin/indicators" class="back-link">← Back</a>
				<h1>{indicator.name}</h1>
				<span
					class="status"
					class:status--published={indicator.is_active}
					class:status--draft={!indicator.is_active}
				>
					{indicator.is_active ? 'Active' : 'Inactive'}
				</span>
			</div>
			<div class="header-actions">
				<a href="/indicators/{indicator.slug}" target="_blank" class="btn-secondary">Preview</a>
				<button class="btn-success" onclick={toggleIndicator}>
					{indicator.is_active ? 'Deactivate' : 'Activate'}
				</button>
				<button class="btn-primary" onclick={saveIndicator} disabled={saving}>
					{saving ? 'Saving...' : 'Save Changes'}
				</button>
			</div>
		</header>

		{#if error}
			<div class="alert alert-error">{error}</div>
		{/if}
		{#if success}
			<div class="alert alert-success">{success}</div>
		{/if}

		<nav class="tabs">
			<button class:active={activeTab === 'details'} onclick={() => (activeTab = 'details')}
				>Details</button
			>
			<button class:active={activeTab === 'files'} onclick={() => (activeTab = 'files')}
				>Files <span class="badge">{files.length}</span></button
			>
			<button class:active={activeTab === 'videos'} onclick={() => (activeTab = 'videos')}
				>Videos <span class="badge">{videos.length}</span></button
			>
			<button class:active={activeTab === 'seo'} onclick={() => (activeTab = 'seo')}>SEO</button>
		</nav>

		<div class="tab-content">
			{#if activeTab === 'details'}
				<div class="form-section">
					<h2>Basic Information</h2>
					<div class="form-grid">
						<div class="form-group">
							<label for="name">Name *</label>
							<input type="text" id="name" name="name" bind:value={indicator.name} />
						</div>
						<div class="form-group">
							<label for="slug">Slug</label>
							<input type="text" id="slug" name="slug" bind:value={indicator.slug} />
						</div>
						<div class="form-group">
							<label for="price">Price (USD)</label>
							<input type="number" id="price" name="price" step="0.01" bind:value={indicator.price} />
						</div>
						<div class="form-group">
							<label for="platform">Platform</label>
							<select id="platform" bind:value={indicator.platform}>
								{#each platformOptions as opt}
									<option value={opt.value}>{opt.label}</option>
								{/each}
							</select>
						</div>
						<div class="form-group">
							<label for="version">Version</label>
							<input type="text" id="version" name="version" bind:value={indicator.version} placeholder="1.0" />
						</div>
						<div class="form-group">
							<label for="is_active">Status</label>
							<select id="is_active" bind:value={indicator.is_active}>
								<option value={true}>Active</option>
								<option value={false}>Inactive</option>
							</select>
						</div>
					</div>
				</div>

				<div class="form-section">
					<h2>Images & URLs</h2>
					<div class="form-grid">
						<div class="form-group">
							<label for="thumbnail">Thumbnail URL</label>
							<input
								type="url"
								id="thumbnail" name="thumbnail"
								bind:value={indicator.thumbnail}
								placeholder="https://..."
							/>
						</div>
						<div class="form-group">
							<label for="download_url">Download URL</label>
							<input
								type="url"
								id="download_url" name="download_url"
								bind:value={indicator.download_url}
								placeholder="https://..."
							/>
						</div>
						<div class="form-group full-width">
							<label for="documentation_url">Documentation URL</label>
							<input
								type="url"
								id="documentation_url" name="documentation_url"
								bind:value={indicator.documentation_url}
								placeholder="https://..."
							/>
						</div>
					</div>
				</div>

				<div class="form-section">
					<h2>Description</h2>
					<div class="form-group full-width">
						<label for="description">Short Description</label>
						<textarea id="description" rows="3" bind:value={indicator.description}></textarea>
					</div>
					<div class="form-group full-width">
						<label for="long_description">Long Description</label>
						<textarea id="long_description" rows="8" bind:value={indicator.long_description}
						></textarea>
					</div>
				</div>

			{:else if activeTab === 'files'}
				<div class="form-section">
					<div class="section-header">
						<h2>Indicator Files</h2>
						<button class="btn-primary" onclick={() => (showFileModal = true)}>Upload File</button>
					</div>

					{#if loadingFiles}
						<div class="loading-inline">
							<div class="spinner-small"></div>
							<span>Loading files...</span>
						</div>
					{:else if files.length === 0}
						<div class="empty-state">
							<p>No files uploaded yet</p>
							<p class="hint">Upload indicator files for different trading platforms</p>
						</div>
					{:else}
						<div class="files-table">
							<table>
								<thead>
									<tr>
										<th>Platform</th>
										<th>File</th>
										<th>Version</th>
										<th>Size</th>
										<th>Downloads</th>
										<th>Status</th>
										<th>Actions</th>
									</tr>
								</thead>
								<tbody>
									{#each files as file}
										<tr>
											<td class="platform">{file.platform}</td>
											<td class="file-name">{file.display_name || file.file_name}</td>
											<td>{file.version || '1.0'}</td>
											<td>{formatFileSize(file.file_size_bytes)}</td>
											<td>{file.download_count || 0}</td>
											<td>
												<button
													class="file-status"
													class:active={file.is_active}
													onclick={() => toggleFileStatus(file.id)}
												>
													{file.is_active ? 'Active' : 'Inactive'}
												</button>
											</td>
											<td>
												<button class="btn-icon btn-danger" onclick={() => deleteFile(file.id)} title="Delete file">
													X
												</button>
											</td>
										</tr>
									{/each}
								</tbody>
							</table>
						</div>
					{/if}
				</div>

			{:else if activeTab === 'videos'}
				<div class="form-section">
					<div class="section-header">
						<h2>Tutorial Videos</h2>
						<button class="btn-primary" onclick={() => (showVideoModal = true)}>Add Video</button>
					</div>

					{#if loadingVideos}
						<div class="loading-inline">
							<div class="spinner-small"></div>
							<span>Loading videos...</span>
						</div>
					{:else if videos.length === 0}
						<div class="empty-state">
							<p>No videos added yet</p>
							<p class="hint">Add tutorial videos to help users learn the indicator</p>
						</div>
					{:else}
						<div class="videos-grid">
							{#each videos as video}
								<div class="video-card">
									{#if video.thumbnail_url}
										<img src={video.thumbnail_url} alt={video.title} class="thumbnail" />
									{:else}
										<div class="thumbnail-placeholder">No Thumbnail</div>
									{/if}
									<div class="video-info">
										<h3>{video.title}</h3>
										<div class="video-meta">
											{#if video.duration_seconds}
												<span class="tag">{formatDuration(video.duration_seconds)}</span>
											{/if}
											<span class="tag" class:active={video.is_active}>
												{video.is_active !== false ? 'Active' : 'Inactive'}
											</span>
										</div>
									</div>
									<button class="btn-icon btn-danger" onclick={() => deleteVideo(video.id)} title="Delete video">X</button>
								</div>
							{/each}
						</div>
					{/if}
				</div>

			{:else if activeTab === 'seo'}
				<div class="form-section">
					<h2>SEO Settings</h2>
					<div class="form-group full-width">
						<label for="meta_title">Meta Title</label>
						<input
							type="text"
							id="meta_title" name="meta_title"
							bind:value={indicator.meta_title}
							placeholder="Page title for search engines"
						/>
					</div>
					<div class="form-group full-width">
						<label for="meta_description">Meta Description</label>
						<textarea
							id="meta_description"
							rows="3"
							bind:value={indicator.meta_description}
							placeholder="Description for search engines"
						></textarea>
					</div>
				</div>
			{/if}
		</div>
	{/if}
</div>

<!-- File Upload Modal -->
{#if showFileModal}
	<div class="modal-overlay" onclick={() => (showFileModal = false)}>
		<div class="modal" onclick={(e) => e.stopPropagation()}>
			<div class="modal-header">
				<h2>Upload Indicator File</h2>
				<button class="btn-close" onclick={() => (showFileModal = false)}>X</button>
			</div>
			<div class="modal-body">
				<div class="form-group">
					<label for="file-platform">Platform *</label>
					<select id="file-platform" bind:value={newFile.platform}>
						{#each platformOptions as opt}
							<option value={opt.value}>{opt.label}</option>
						{/each}
					</select>
				</div>
				<div class="form-group">
					<label for="file-name">Display Name *</label>
					<input type="text" id="file-name" bind:value={newFile.display_name} placeholder="e.g., Squeeze Pro v2.0" />
				</div>
				<div class="form-group">
					<label for="file-version">Version</label>
					<input type="text" id="file-version" bind:value={newFile.version} placeholder="1.0" />
				</div>
				<div class="form-group">
					<label for="file-input">File *</label>
					<input type="file" id="file-input" onchange={handleFileSelect} />
					{#if newFile.file}
						<p class="hint">Selected: {newFile.file.name} ({formatFileSize(newFile.file.size)})</p>
					{/if}
				</div>
			</div>
			<div class="modal-footer">
				<button class="btn-secondary" onclick={() => (showFileModal = false)}>Cancel</button>
				<button class="btn-primary" onclick={uploadFile} disabled={uploadingFile}>
					{uploadingFile ? 'Uploading...' : 'Upload File'}
				</button>
			</div>
		</div>
	</div>
{/if}

<!-- Video Add Modal -->
{#if showVideoModal}
	<div class="modal-overlay" onclick={() => (showVideoModal = false)}>
		<div class="modal" onclick={(e) => e.stopPropagation()}>
			<div class="modal-header">
				<h2>Add Tutorial Video</h2>
				<button class="btn-close" onclick={() => (showVideoModal = false)}>X</button>
			</div>
			<div class="modal-body">
				<div class="form-group">
					<label for="video-title">Title *</label>
					<input type="text" id="video-title" bind:value={newVideo.title} placeholder="e.g., Getting Started with Squeeze Pro" />
				</div>
				<div class="form-group">
					<label for="video-description">Description</label>
					<textarea id="video-description" rows="3" bind:value={newVideo.description} placeholder="Brief description of the video"></textarea>
				</div>
				<div class="form-group">
					<label for="video-url">Video URL (Direct)</label>
					<input type="url" id="video-url" bind:value={newVideo.video_url} placeholder="https://..." />
				</div>
				<div class="form-group">
					<label for="embed-url">Embed URL (YouTube/Vimeo)</label>
					<input type="url" id="embed-url" bind:value={newVideo.embed_url} placeholder="https://www.youtube.com/embed/..." />
				</div>
				<div class="form-group">
					<label for="thumbnail-url">Thumbnail URL</label>
					<input type="url" id="thumbnail-url" bind:value={newVideo.thumbnail_url} placeholder="https://..." />
				</div>
			</div>
			<div class="modal-footer">
				<button class="btn-secondary" onclick={() => (showVideoModal = false)}>Cancel</button>
				<button class="btn-primary" onclick={addVideo} disabled={addingVideo}>
					{addingVideo ? 'Adding...' : 'Add Video'}
				</button>
			</div>
		</div>
	</div>
{/if}

<style>
	.editor-page {
		padding: 24px;
		max-width: 1200px;
		margin: 0 auto;
	}

	.loading,
	.error-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 64px;
	}
	.spinner {
		width: 40px;
		height: 40px;
		border: 3px solid #e5e7eb;
		border-top-color: #143e59;
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin-bottom: 16px;
	}
	.spinner-small {
		width: 20px;
		height: 20px;
		border: 2px solid #e5e7eb;
		border-top-color: #143e59;
		border-radius: 50%;
		animation: spin 1s linear infinite;
	}
	.loading-inline {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 24px;
		color: #6b7280;
	}
	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 24px;
		flex-wrap: wrap;
		gap: 16px;
	}
	.header-left {
		display: flex;
		align-items: center;
		gap: 12px;
	}
	.back-link {
		color: #6b7280;
		text-decoration: none;
		font-size: 14px;
	}
	.back-link:hover {
		color: #143e59;
	}
	h1 {
		font-size: 24px;
		font-weight: 600;
		color: #1f2937;
		margin: 0;
	}

	.status {
		display: inline-block;
		padding: 4px 10px;
		border-radius: 20px;
		font-size: 12px;
		font-weight: 500;
		text-transform: capitalize;
	}
	.status--draft {
		background: #fef3c7;
		color: #92400e;
	}
	.status--published {
		background: #d1fae5;
		color: #065f46;
	}

	.header-actions {
		display: flex;
		gap: 12px;
	}
	.btn-primary,
	.btn-secondary,
	.btn-success {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		padding: 10px 20px;
		border: none;
		border-radius: 8px;
		font-size: 14px;
		font-weight: 500;
		cursor: pointer;
		text-decoration: none;
	}
	.btn-primary {
		background: #143e59;
		color: #fff;
	}
	.btn-primary:hover {
		background: #0f2d42;
	}
	.btn-primary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}
	.btn-secondary {
		background: #f3f4f6;
		color: #374151;
		border: 1px solid #e5e7eb;
	}
	.btn-secondary:hover {
		background: #e5e7eb;
	}
	.btn-success {
		background: #10b981;
		color: #fff;
	}
	.btn-success:hover {
		background: #059669;
	}
	.btn-close {
		background: none;
		border: none;
		font-size: 20px;
		cursor: pointer;
		color: #6b7280;
		padding: 4px 8px;
	}
	.btn-close:hover {
		color: #1f2937;
	}

	.alert {
		padding: 12px 16px;
		border-radius: 8px;
		margin-bottom: 16px;
		font-size: 14px;
	}
	.alert-error {
		background: #fee2e2;
		color: #dc2626;
	}
	.alert-success {
		background: #d1fae5;
		color: #065f46;
	}

	.tabs {
		display: flex;
		gap: 4px;
		border-bottom: 1px solid #e5e7eb;
		margin-bottom: 24px;
	}
	.tabs button {
		padding: 12px 20px;
		background: none;
		border: none;
		font-size: 14px;
		font-weight: 500;
		color: #6b7280;
		cursor: pointer;
		border-bottom: 2px solid transparent;
		margin-bottom: -1px;
		display: flex;
		align-items: center;
		gap: 8px;
	}
	.tabs button:hover {
		color: #1f2937;
	}
	.tabs button.active {
		color: #143e59;
		border-bottom-color: #143e59;
	}
	.badge {
		background: #e5e7eb;
		padding: 2px 8px;
		border-radius: 10px;
		font-size: 12px;
	}

	.form-section {
		background: #fff;
		border: 1px solid #e5e7eb;
		border-radius: 12px;
		padding: 24px;
		margin-bottom: 24px;
	}
	.form-section h2 {
		font-size: 16px;
		font-weight: 600;
		color: #1f2937;
		margin: 0 0 20px;
	}
	.section-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 20px;
	}
	.section-header h2 {
		margin: 0;
	}

	.form-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 16px;
	}
	.form-group {
		display: flex;
		flex-direction: column;
		gap: 6px;
	}
	.form-group.full-width {
		grid-column: 1 / -1;
	}
	label {
		font-size: 13px;
		font-weight: 500;
		color: #374151;
	}
	input,
	select,
	textarea {
		padding: 10px 12px;
		border: 1px solid #e5e7eb;
		border-radius: 6px;
		font-size: 14px;
	}
	input:focus,
	select:focus,
	textarea:focus {
		outline: none;
		border-color: #143e59;
	}
	textarea {
		resize: vertical;
	}

	.empty-state {
		text-align: center;
		padding: 40px;
		color: #6b7280;
	}
	.hint {
		font-size: 13px;
		color: #9ca3af;
	}

	.files-table {
		overflow-x: auto;
	}
	.files-table table {
		width: 100%;
		border-collapse: collapse;
		min-width: 600px;
	}
	.files-table th {
		text-align: left;
		padding: 12px;
		font-size: 12px;
		font-weight: 600;
		color: #6b7280;
		text-transform: uppercase;
		border-bottom: 1px solid #e5e7eb;
	}
	.files-table td {
		padding: 12px;
		border-bottom: 1px solid #f3f4f6;
	}
	.platform {
		font-weight: 500;
		text-transform: capitalize;
	}
	.file-name {
		color: #1f2937;
		font-size: 14px;
	}
	.file-status {
		font-size: 12px;
		padding: 4px 12px;
		border-radius: 4px;
		background: #f3f4f6;
		border: none;
		cursor: pointer;
	}
	.file-status.active {
		background: #d1fae5;
		color: #065f46;
	}
	.file-status:hover {
		opacity: 0.8;
	}

	.videos-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
		gap: 16px;
	}
	.video-card {
		background: #f9fafb;
		border-radius: 8px;
		overflow: hidden;
		position: relative;
	}
	.thumbnail,
	.thumbnail-placeholder {
		width: 100%;
		aspect-ratio: 16/9;
		object-fit: cover;
		background: #e5e7eb;
		display: flex;
		align-items: center;
		justify-content: center;
		color: #9ca3af;
	}
	.video-info {
		padding: 12px;
	}
	.video-info h3 {
		font-size: 14px;
		font-weight: 500;
		margin: 0 0 8px;
	}
	.video-meta {
		display: flex;
		gap: 6px;
	}
	.tag {
		font-size: 11px;
		padding: 2px 8px;
		background: #143e59;
		color: #fff;
		border-radius: 4px;
	}
	.tag.active {
		background: #10b981;
	}
	.video-card .btn-icon {
		position: absolute;
		top: 8px;
		right: 8px;
		background: rgba(0, 0, 0, 0.5);
		color: #fff;
	}

	.btn-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		border: none;
		background: #f3f4f6;
		border-radius: 6px;
		color: #6b7280;
		cursor: pointer;
		min-height: 44px;
		min-width: 44px;
		font-weight: bold;
	}
	.btn-danger:hover {
		background: #fee2e2;
		color: #dc2626;
	}

	/* Modal Styles */
	.modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.5);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
		padding: 20px;
	}
	.modal {
		background: #fff;
		border-radius: 12px;
		max-width: 500px;
		width: 100%;
		max-height: 90vh;
		overflow-y: auto;
	}
	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 20px 24px;
		border-bottom: 1px solid #e5e7eb;
	}
	.modal-header h2 {
		margin: 0;
		font-size: 18px;
		font-weight: 600;
	}
	.modal-body {
		padding: 24px;
		display: flex;
		flex-direction: column;
		gap: 16px;
	}
	.modal-footer {
		display: flex;
		justify-content: flex-end;
		gap: 12px;
		padding: 16px 24px;
		border-top: 1px solid #e5e7eb;
	}

	/* Responsive Design */
	@media (max-width: 639px) {
		.editor-page {
			padding: 16px;
			padding-left: max(16px, env(safe-area-inset-left));
			padding-right: max(16px, env(safe-area-inset-right));
		}

		.page-header {
			flex-direction: column;
			align-items: flex-start;
			gap: 12px;
		}

		.header-left {
			flex-wrap: wrap;
		}

		.header-actions {
			width: 100%;
			flex-wrap: wrap;
		}

		.header-actions button,
		.header-actions a {
			flex: 1;
			min-width: 100px;
			justify-content: center;
		}

		h1 {
			font-size: 20px;
		}

		.tabs {
			overflow-x: auto;
			-webkit-overflow-scrolling: touch;
			gap: 0;
		}

		.tabs button {
			padding: 12px 16px;
			white-space: nowrap;
			min-height: 44px;
		}

		.form-grid {
			grid-template-columns: 1fr;
		}

		.form-section {
			padding: 16px;
		}

		.videos-grid {
			grid-template-columns: 1fr;
		}

		.modal {
			margin: 10px;
			max-width: calc(100% - 20px);
		}
	}

	@media (min-width: 640px) {
		.header-actions {
			flex-wrap: nowrap;
		}
	}

	@media (min-width: 768px) {
		.editor-page {
			padding: 24px;
		}

		.form-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	@media (hover: none) and (pointer: coarse) {
		.btn-primary,
		.btn-secondary,
		.btn-success {
			padding: 12px 24px;
			min-height: 48px;
		}

		.tabs button {
			min-height: 48px;
		}

		input,
		select,
		textarea {
			font-size: 16px;
			min-height: 44px;
		}
	}

	@media (prefers-reduced-motion: reduce) {
		.spinner,
		.spinner-small {
			animation: none;
		}
	}

	@supports (padding: max(0px)) {
		.editor-page {
			padding-bottom: max(24px, env(safe-area-inset-bottom));
		}
	}
</style>

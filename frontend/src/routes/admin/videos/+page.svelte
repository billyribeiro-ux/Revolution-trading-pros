<script lang="ts">
	import { onMount } from 'svelte';
	import {
		IconVideo,
		IconUpload,
		IconSearch,
		IconFilter,
		IconPlayerPlay,
		IconEdit,
		IconTrash,
		IconEye,
		IconClock,
		IconCalendar,
		IconRefresh,
		IconPlus,
		IconLink,
		IconDownload,
		IconShare
	} from '@tabler/icons-svelte';
	import { api } from '$lib/api/config';

	interface Video {
		id: number;
		title: string;
		description?: string;
		thumbnail_url?: string;
		video_url: string;
		duration?: string;
		views?: number;
		status: 'published' | 'draft' | 'processing';
		created_at: string;
	}

	let videos: Video[] = [];
	let isLoading = true;
	let error = '';
	let searchQuery = '';
	let selectedStatus = 'all';
	let showUploadModal = false;

	// Stats
	let stats = {
		totalVideos: 0,
		totalViews: 0,
		totalDuration: '0h',
		publishedCount: 0
	};

	async function loadVideos() {
		isLoading = true;
		error = '';

		try {
			const response = await api.get('/api/admin/videos');
			const data = response?.data || response;
			
			videos = data?.videos || data || [];
			
			// Calculate stats
			stats.totalVideos = videos.length;
			stats.totalViews = videos.reduce((sum, v) => sum + (v.views || 0), 0);
			stats.publishedCount = videos.filter(v => v.status === 'published').length;
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load videos';
		} finally {
			isLoading = false;
		}
	}

	async function deleteVideo(id: number) {
		if (!confirm('Are you sure you want to delete this video?')) return;

		try {
			await api.delete(`/api/admin/videos/${id}`);
			await loadVideos();
		} catch (err) {
			alert(err instanceof Error ? err.message : 'Failed to delete video');
		}
	}

	function formatDate(dateString: string): string {
		if (!dateString) return 'N/A';
		return new Date(dateString).toLocaleDateString();
	}

	function formatViews(views: number): string {
		if (views >= 1000000) return (views / 1000000).toFixed(1) + 'M';
		if (views >= 1000) return (views / 1000).toFixed(1) + 'K';
		return views.toString();
	}

	$: filteredVideos = videos.filter(video => {
		const matchesSearch = !searchQuery || 
			video.title?.toLowerCase().includes(searchQuery.toLowerCase()) ||
			video.description?.toLowerCase().includes(searchQuery.toLowerCase());
		const matchesStatus = selectedStatus === 'all' || video.status === selectedStatus;
		return matchesSearch && matchesStatus;
	});

	onMount(() => {
		loadVideos();
	});
</script>

<svelte:head>
	<title>Videos - Admin Dashboard</title>
</svelte:head>

<div class="videos-page">
	<!-- Header -->
	<div class="page-header">
		<div>
			<h1>Video Management</h1>
			<p class="page-description">Upload, manage, and organize your video content</p>
		</div>
		<div class="header-actions">
			<button class="btn-refresh" on:click={loadVideos} disabled={isLoading}>
				<IconRefresh size={18} class={isLoading ? 'spinning' : ''} />
			</button>
			<button class="btn-primary" on:click={() => showUploadModal = true}>
				<IconUpload size={18} />
				Upload Video
			</button>
		</div>
	</div>

	<!-- Stats -->
	<div class="stats-grid">
		<div class="stat-card">
			<div class="stat-icon purple">
				<IconVideo size={24} />
			</div>
			<div class="stat-content">
				<span class="stat-value">{stats.totalVideos}</span>
				<span class="stat-label">Total Videos</span>
			</div>
		</div>
		<div class="stat-card">
			<div class="stat-icon blue">
				<IconEye size={24} />
			</div>
			<div class="stat-content">
				<span class="stat-value">{formatViews(stats.totalViews)}</span>
				<span class="stat-label">Total Views</span>
			</div>
		</div>
		<div class="stat-card">
			<div class="stat-icon green">
				<IconPlayerPlay size={24} />
			</div>
			<div class="stat-content">
				<span class="stat-value">{stats.publishedCount}</span>
				<span class="stat-label">Published</span>
			</div>
		</div>
		<div class="stat-card">
			<div class="stat-icon amber">
				<IconClock size={24} />
			</div>
			<div class="stat-content">
				<span class="stat-value">{stats.totalDuration}</span>
				<span class="stat-label">Total Duration</span>
			</div>
		</div>
	</div>

	<!-- Filters -->
	<div class="filters-bar">
		<div class="search-box">
			<IconSearch size={18} />
			<input type="text" placeholder="Search videos..." bind:value={searchQuery} />
		</div>
		<select class="filter-select" bind:value={selectedStatus}>
			<option value="all">All Videos</option>
			<option value="published">Published</option>
			<option value="draft">Drafts</option>
			<option value="processing">Processing</option>
		</select>
	</div>

	<!-- Content -->
	{#if isLoading}
		<div class="loading-state">
			<div class="spinner"></div>
			<p>Loading videos...</p>
		</div>
	{:else if error}
		<div class="error-state">
			<p>{error}</p>
			<button on:click={loadVideos}>Try Again</button>
		</div>
	{:else if filteredVideos.length === 0}
		<div class="empty-state">
			<IconVideo size={64} />
			<h3>No videos found</h3>
			<p>Upload your first video to get started</p>
			<button class="btn-primary" on:click={() => showUploadModal = true}>
				<IconUpload size={18} />
				Upload Video
			</button>
		</div>
	{:else}
		<div class="videos-grid">
			{#each filteredVideos as video}
				<div class="video-card">
					<div class="video-thumbnail">
						{#if video.thumbnail_url}
							<img src={video.thumbnail_url} alt={video.title} />
						{:else}
							<div class="thumbnail-placeholder">
								<IconVideo size={32} />
							</div>
						{/if}
						<div class="video-overlay">
							<button class="play-btn">
								<IconPlayerPlay size={24} />
							</button>
						</div>
						{#if video.duration}
							<span class="video-duration">{video.duration}</span>
						{/if}
						<span class="video-status {video.status}">{video.status}</span>
					</div>
					<div class="video-info">
						<h4 class="video-title">{video.title}</h4>
						{#if video.description}
							<p class="video-description">{video.description}</p>
						{/if}
						<div class="video-meta">
							<span class="meta-item">
								<IconEye size={14} />
								{formatViews(video.views || 0)} views
							</span>
							<span class="meta-item">
								<IconCalendar size={14} />
								{formatDate(video.created_at)}
							</span>
						</div>
					</div>
					<div class="video-actions">
						<button class="btn-icon" title="Edit">
							<IconEdit size={16} />
						</button>
						<button class="btn-icon" title="Copy Link">
							<IconLink size={16} />
						</button>
						<button class="btn-icon" title="Share">
							<IconShare size={16} />
						</button>
						<button class="btn-icon danger" title="Delete" on:click={() => deleteVideo(video.id)}>
							<IconTrash size={16} />
						</button>
					</div>
				</div>
			{/each}
		</div>
	{/if}
</div>

<!-- Upload Modal -->
{#if showUploadModal}
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div class="modal-overlay" on:click={() => showUploadModal = false} on:keydown={(e) => e.key === 'Escape' && (showUploadModal = false)}>
		<!-- svelte-ignore a11y_no_static_element_interactions -->
		<div class="modal" on:click|stopPropagation on:keydown|stopPropagation role="dialog" aria-modal="true" tabindex="-1">
			<div class="modal-header">
				<h2>Upload Video</h2>
				<button class="modal-close" on:click={() => showUploadModal = false} type="button" aria-label="Close">&times;</button>
			</div>
			<div class="modal-body">
				<div class="upload-zone">
					<IconUpload size={48} />
					<h3>Drag & drop your video here</h3>
					<p>or click to browse files</p>
					<p class="upload-hint">Supports MP4, WebM, MOV (max 500MB)</p>
				</div>
				<div class="upload-divider">
					<span>or</span>
				</div>
				<div class="url-input">
					<label for="video-url">Video URL (YouTube, Vimeo, etc.)</label>
					<input type="url" id="video-url" placeholder="https://..." />
				</div>
			</div>
			<div class="modal-footer">
				<button class="btn-secondary" on:click={() => showUploadModal = false} type="button">Cancel</button>
				<button class="btn-primary" type="button">Upload</button>
			</div>
		</div>
	</div>
{/if}

<style>
	.videos-page {
		max-width: 1600px;
	}

	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 2rem;
		flex-wrap: wrap;
		gap: 1rem;
	}

	.page-header h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0 0 0.25rem 0;
	}

	.page-description {
		color: #64748b;
		margin: 0;
	}

	.header-actions {
		display: flex;
		gap: 0.75rem;
	}

	.btn-refresh {
		width: 42px;
		height: 42px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(30, 41, 59, 0.8);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 10px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-refresh:hover {
		background: rgba(99, 102, 241, 0.2);
		color: #818cf8;
	}

	.btn-refresh :global(.spinning) {
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		from { transform: rotate(0deg); }
		to { transform: rotate(360deg); }
	}

	.btn-primary {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background: linear-gradient(135deg, #6366f1, #8b5cf6);
		color: white;
		border: none;
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-primary:hover {
		transform: translateY(-2px);
		box-shadow: 0 4px 15px rgba(99, 102, 241, 0.4);
	}

	.btn-secondary {
		padding: 0.75rem 1.25rem;
		background: rgba(99, 102, 241, 0.1);
		color: #94a3b8;
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-secondary:hover {
		background: rgba(99, 102, 241, 0.2);
		color: #e2e8f0;
	}

	/* Stats */
	.stats-grid {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 1rem;
		margin-bottom: 2rem;
	}

	@media (max-width: 1024px) {
		.stats-grid { grid-template-columns: repeat(2, 1fr); }
	}

	@media (max-width: 640px) {
		.stats-grid { grid-template-columns: 1fr; }
	}

	.stat-card {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1.25rem;
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 14px;
	}

	.stat-icon {
		width: 52px;
		height: 52px;
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.stat-icon.purple { background: rgba(139, 92, 246, 0.15); color: #a78bfa; }
	.stat-icon.blue { background: rgba(59, 130, 246, 0.15); color: #60a5fa; }
	.stat-icon.green { background: rgba(34, 197, 94, 0.15); color: #4ade80; }
	.stat-icon.amber { background: rgba(245, 158, 11, 0.15); color: #fbbf24; }

	.stat-content {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.stat-value {
		font-size: 1.5rem;
		font-weight: 700;
		color: #f1f5f9;
	}

	.stat-label {
		font-size: 0.8rem;
		color: #64748b;
	}

	/* Filters */
	.filters-bar {
		display: flex;
		gap: 1rem;
		margin-bottom: 1.5rem;
		flex-wrap: wrap;
	}

	.search-box {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0 1rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 10px;
		flex: 1;
		max-width: 400px;
	}

	.search-box :global(svg) {
		color: #64748b;
	}

	.search-box input {
		flex: 1;
		padding: 0.75rem 0;
		background: transparent;
		border: none;
		color: #e2e8f0;
		font-size: 0.9rem;
		outline: none;
	}

	.search-box input::placeholder {
		color: #64748b;
	}

	.filter-select {
		padding: 0.75rem 1rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 10px;
		color: #e2e8f0;
		font-size: 0.9rem;
		cursor: pointer;
	}

	/* Videos Grid */
	.videos-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
		gap: 1.5rem;
	}

	.video-card {
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 16px;
		overflow: hidden;
		transition: all 0.2s;
	}

	.video-card:hover {
		border-color: rgba(99, 102, 241, 0.3);
		transform: translateY(-4px);
		box-shadow: 0 12px 40px rgba(0, 0, 0, 0.3);
	}

	.video-thumbnail {
		position: relative;
		aspect-ratio: 16/9;
		background: #1e293b;
	}

	.video-thumbnail img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.thumbnail-placeholder {
		width: 100%;
		height: 100%;
		display: flex;
		align-items: center;
		justify-content: center;
		color: #475569;
	}

	.video-overlay {
		position: absolute;
		inset: 0;
		background: rgba(0, 0, 0, 0.4);
		display: flex;
		align-items: center;
		justify-content: center;
		opacity: 0;
		transition: opacity 0.2s;
	}

	.video-card:hover .video-overlay {
		opacity: 1;
	}

	.play-btn {
		width: 60px;
		height: 60px;
		border-radius: 50%;
		background: rgba(99, 102, 241, 0.9);
		border: none;
		color: white;
		display: flex;
		align-items: center;
		justify-content: center;
		cursor: pointer;
		transition: all 0.2s;
	}

	.play-btn:hover {
		transform: scale(1.1);
		background: #6366f1;
	}

	.video-duration {
		position: absolute;
		bottom: 0.5rem;
		right: 0.5rem;
		padding: 0.25rem 0.5rem;
		background: rgba(0, 0, 0, 0.8);
		border-radius: 4px;
		font-size: 0.75rem;
		color: white;
	}

	.video-status {
		position: absolute;
		top: 0.5rem;
		left: 0.5rem;
		padding: 0.25rem 0.75rem;
		border-radius: 20px;
		font-size: 0.7rem;
		font-weight: 600;
		text-transform: uppercase;
	}

	.video-status.published { background: rgba(34, 197, 94, 0.9); color: white; }
	.video-status.draft { background: rgba(245, 158, 11, 0.9); color: white; }
	.video-status.processing { background: rgba(59, 130, 246, 0.9); color: white; }

	.video-info {
		padding: 1.25rem;
	}

	.video-title {
		font-size: 1rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0 0 0.5rem 0;
		display: -webkit-box;
		-webkit-line-clamp: 2;
		line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	.video-description {
		font-size: 0.85rem;
		color: #64748b;
		margin: 0 0 0.75rem 0;
		display: -webkit-box;
		-webkit-line-clamp: 2;
		line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	.video-meta {
		display: flex;
		gap: 1rem;
	}

	.meta-item {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		font-size: 0.8rem;
		color: #64748b;
	}

	.video-actions {
		display: flex;
		gap: 0.5rem;
		padding: 0 1.25rem 1.25rem;
	}

	.btn-icon {
		width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(99, 102, 241, 0.1);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 8px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-icon:hover {
		background: rgba(99, 102, 241, 0.2);
		color: #818cf8;
	}

	.btn-icon.danger:hover {
		background: rgba(239, 68, 68, 0.1);
		color: #f87171;
		border-color: rgba(239, 68, 68, 0.3);
	}

	/* States */
	.loading-state, .error-state, .empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 2rem;
		text-align: center;
		color: #64748b;
	}

	.empty-state :global(svg) {
		color: #475569;
		margin-bottom: 1rem;
	}

	.empty-state h3 {
		color: #e2e8f0;
		margin: 0 0 0.5rem 0;
	}

	.empty-state p {
		margin: 0 0 1.5rem 0;
	}

	.spinner {
		width: 40px;
		height: 40px;
		border: 3px solid rgba(99, 102, 241, 0.2);
		border-top-color: #6366f1;
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin-bottom: 1rem;
	}

	/* Modal */
	.modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.7);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 100;
		padding: 1rem;
	}

	.modal {
		background: #1e293b;
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 16px;
		width: 100%;
		max-width: 500px;
		max-height: 90vh;
		overflow: auto;
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1.25rem;
		border-bottom: 1px solid rgba(99, 102, 241, 0.1);
	}

	.modal-header h2 {
		font-size: 1.25rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0;
	}

	.modal-close {
		width: 32px;
		height: 32px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: transparent;
		border: none;
		color: #64748b;
		font-size: 1.5rem;
		cursor: pointer;
		border-radius: 8px;
		transition: all 0.2s;
	}

	.modal-close:hover {
		background: rgba(99, 102, 241, 0.1);
		color: #e2e8f0;
	}

	.modal-body {
		padding: 1.25rem;
	}

	.upload-zone {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 3rem 2rem;
		border: 2px dashed rgba(99, 102, 241, 0.3);
		border-radius: 12px;
		background: rgba(99, 102, 241, 0.05);
		cursor: pointer;
		transition: all 0.2s;
	}

	.upload-zone:hover {
		border-color: rgba(99, 102, 241, 0.5);
		background: rgba(99, 102, 241, 0.1);
	}

	.upload-zone :global(svg) {
		color: #818cf8;
		margin-bottom: 1rem;
	}

	.upload-zone h3 {
		color: #e2e8f0;
		margin: 0 0 0.5rem 0;
	}

	.upload-zone p {
		color: #64748b;
		margin: 0;
	}

	.upload-hint {
		font-size: 0.8rem;
		margin-top: 1rem !important;
	}

	.upload-divider {
		display: flex;
		align-items: center;
		gap: 1rem;
		margin: 1.5rem 0;
		color: #64748b;
	}

	.upload-divider::before,
	.upload-divider::after {
		content: '';
		flex: 1;
		height: 1px;
		background: rgba(99, 102, 241, 0.2);
	}

	.url-input label {
		display: block;
		font-size: 0.85rem;
		color: #94a3b8;
		margin-bottom: 0.5rem;
	}

	.url-input input {
		width: 100%;
		padding: 0.75rem 1rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 10px;
		color: #e2e8f0;
		font-size: 0.9rem;
	}

	.url-input input:focus {
		outline: none;
		border-color: rgba(99, 102, 241, 0.5);
	}

	.modal-footer {
		display: flex;
		justify-content: flex-end;
		gap: 0.75rem;
		padding: 1.25rem;
		border-top: 1px solid rgba(99, 102, 241, 0.1);
	}
</style>

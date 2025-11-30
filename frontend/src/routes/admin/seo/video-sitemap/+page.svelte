<script lang="ts">
	import { onMount } from 'svelte';
	import {
		IconVideo,
		IconRefresh,
		IconExternalLink,
		IconBrandYoutube,
		IconPlayerPlay,
		IconPlus,
		IconTrash,
		IconEdit
	} from '@tabler/icons-svelte';

	// State using Svelte 5 runes
	let videos = $state<any[]>([]);
	let settings = $state({
		enabled: true,
		includeThumbnails: true,
		includeUploader: true,
		detectFromContent: true,
		platforms: ['youtube', 'vimeo', 'dailymotion', 'ted', 'wistia']
	});
	let loading = $state(false);
	let showAddModal = $state(false);
	let newVideoUrl = $state('');

	// Computed
	let sitemapUrl = $derived(`${typeof window !== 'undefined' ? window.location.origin : ''}/video-sitemap.xml`);
	let totalDuration = $derived(videos.reduce((acc, v) => acc + (v.duration || 0), 0));

	onMount(async () => {
		await loadVideos();
	});

	async function loadVideos() {
		loading = true;
		try {
			// In production, fetch from API
			videos = [
				{
					id: 1,
					title: 'Day Trading Masterclass - Complete Course Overview',
					pageUrl: '/courses/day-trading-masterclass',
					platform: 'youtube',
					videoId: 'dQw4w9WgXcQ',
					thumbnail: 'https://img.youtube.com/vi/dQw4w9WgXcQ/maxresdefault.jpg',
					duration: 3600,
					publishedAt: '2025-10-15T10:00:00Z',
					views: 15420
				},
				{
					id: 2,
					title: 'Swing Trading Pro - Master Multi-Day Positions',
					pageUrl: '/courses/swing-trading-pro',
					platform: 'vimeo',
					videoId: '123456789',
					thumbnail: 'https://vumbnail.com/123456789.jpg',
					duration: 5400,
					publishedAt: '2025-09-20T14:00:00Z',
					views: 8320
				},
				{
					id: 3,
					title: 'Options Trading Fundamentals',
					pageUrl: '/courses/options-trading',
					platform: 'youtube',
					videoId: 'OPTIONS123',
					thumbnail: 'https://img.youtube.com/vi/OPTIONS123/maxresdefault.jpg',
					duration: 7200,
					publishedAt: '2025-11-01T09:00:00Z',
					views: 22150
				},
				{
					id: 4,
					title: 'Live Day Trading Room',
					pageUrl: '/live-trading-rooms/day-trading',
					platform: 'youtube',
					videoId: 'LIVE123',
					thumbnail: 'https://img.youtube.com/vi/LIVE123/maxresdefault.jpg',
					duration: 0,
					publishedAt: '2025-11-25T13:30:00Z',
					views: 3240,
					isLive: true
				}
			];
		} finally {
			loading = false;
		}
	}

	async function regenerateSitemap() {
		loading = true;
		try {
			await new Promise((resolve) => setTimeout(resolve, 1000));
			alert('Video sitemap regenerated successfully!');
		} finally {
			loading = false;
		}
	}

	function formatDuration(seconds: number): string {
		if (seconds === 0) return 'Live';
		const hours = Math.floor(seconds / 3600);
		const minutes = Math.floor((seconds % 3600) / 60);
		if (hours > 0) return `${hours}h ${minutes}m`;
		return `${minutes}m`;
	}

	function formatTotalDuration(seconds: number): string {
		const hours = Math.floor(seconds / 3600);
		const minutes = Math.floor((seconds % 3600) / 60);
		return `${hours}h ${minutes}m`;
	}

	function getPlatformIcon(platform: string) {
		// Return appropriate icon based on platform
		return IconBrandYoutube;
	}

	function getPlatformColor(platform: string): string {
		const colors: Record<string, string> = {
			youtube: '#ff0000',
			vimeo: '#1ab7ea',
			dailymotion: '#00d2f3',
			ted: '#eb0028',
			wistia: '#54bbff'
		};
		return colors[platform] || '#666';
	}

	async function addVideo() {
		if (!newVideoUrl) return;
		// In production, validate URL and add to database
		alert('Video URL added: ' + newVideoUrl);
		newVideoUrl = '';
		showAddModal = false;
	}

	async function removeVideo(id: number) {
		if (confirm('Remove this video from the sitemap?')) {
			videos = videos.filter((v) => v.id !== id);
		}
	}
</script>

<svelte:head>
	<title>Video Sitemap | SEO</title>
</svelte:head>

<div class="video-sitemap-page">
	<header class="page-header">
		<div>
			<h1>
				<IconVideo size={28} />
				Google Video Sitemap
			</h1>
			<p>Manage your video sitemap for enhanced video search visibility</p>
		</div>
		<div class="header-actions">
			<a href={sitemapUrl} target="_blank" rel="noopener" class="btn-secondary">
				<IconExternalLink size={18} />
				View Sitemap
			</a>
			<button class="btn-primary" onclick={regenerateSitemap} disabled={loading}>
				<IconRefresh size={18} class={loading ? 'spinning' : ''} />
				{loading ? 'Regenerating...' : 'Regenerate'}
			</button>
		</div>
	</header>

	<div class="stats-grid">
		<div class="stat-card">
			<div class="stat-value">{videos.length}</div>
			<div class="stat-label">Total Videos</div>
		</div>
		<div class="stat-card">
			<div class="stat-value">{formatTotalDuration(totalDuration)}</div>
			<div class="stat-label">Total Duration</div>
		</div>
		<div class="stat-card">
			<div class="stat-value">{videos.filter((v) => v.isLive).length}</div>
			<div class="stat-label">Live Streams</div>
		</div>
		<div class="stat-card">
			<div class="stat-value">{new Set(videos.map((v) => v.platform)).size}</div>
			<div class="stat-label">Platforms</div>
		</div>
	</div>

	<div class="settings-card">
		<h2>Detection Settings</h2>
		<div class="settings-grid">
			<label class="checkbox-label">
				<input type="checkbox" bind:checked={settings.detectFromContent} />
				Auto-detect videos from content
			</label>
			<label class="checkbox-label">
				<input type="checkbox" bind:checked={settings.includeThumbnails} />
				Include video thumbnails
			</label>
			<label class="checkbox-label">
				<input type="checkbox" bind:checked={settings.includeUploader} />
				Include uploader info
			</label>
		</div>
		<div class="platforms-section">
			<h3>Supported Platforms</h3>
			<div class="platforms-grid">
				{#each ['youtube', 'vimeo', 'dailymotion', 'ted', 'wistia'] as platform}
					<label class="platform-checkbox" style="--platform-color: {getPlatformColor(platform)}">
						<input
							type="checkbox"
							checked={settings.platforms.includes(platform)}
							onchange={(e) => {
								const target = e.target as HTMLInputElement;
								if (target.checked) {
									settings.platforms = [...settings.platforms, platform];
								} else {
									settings.platforms = settings.platforms.filter((p) => p !== platform);
								}
							}}
						/>
						<span class="platform-name">{platform.charAt(0).toUpperCase() + platform.slice(1)}</span>
					</label>
				{/each}
			</div>
		</div>
	</div>

	<div class="videos-section">
		<div class="section-header">
			<h2>Videos in Sitemap ({videos.length})</h2>
			<button class="btn-add" onclick={() => (showAddModal = true)}>
				<IconPlus size={18} />
				Add Video
			</button>
		</div>

		{#if loading}
			<div class="loading">Loading videos...</div>
		{:else if videos.length === 0}
			<div class="empty-state">
				<IconVideo size={48} />
				<p>No videos found. Add videos manually or enable auto-detection.</p>
			</div>
		{:else}
			<div class="videos-list">
				{#each videos as video}
					<div class="video-item">
						<div class="video-thumbnail">
							<img src={video.thumbnail} alt={video.title} />
							<span class="duration-badge" class:live={video.isLive}>
								{#if video.isLive}
									<span class="live-dot"></span> LIVE
								{:else}
									{formatDuration(video.duration)}
								{/if}
							</span>
						</div>
						<div class="video-info">
							<h4>{video.title}</h4>
							<div class="video-meta">
								<span
									class="platform-badge"
									style="background: {getPlatformColor(video.platform)}20; color: {getPlatformColor(video.platform)}"
								>
									{video.platform}
								</span>
								<span class="video-url">{video.pageUrl}</span>
								{#if video.views}
									<span class="video-views">{video.views.toLocaleString()} views</span>
								{/if}
							</div>
						</div>
						<div class="video-actions">
							<button class="icon-btn" title="Edit">
								<IconEdit size={18} />
							</button>
							<button class="icon-btn danger" onclick={() => removeVideo(video.id)} title="Remove">
								<IconTrash size={18} />
							</button>
						</div>
					</div>
				{/each}
			</div>
		{/if}
	</div>

	<div class="info-section">
		<h3>Video Sitemap Features</h3>
		<div class="info-grid">
			<div class="info-card">
				<h4>Platform Auto-Detection</h4>
				<p>
					Automatically detects videos from YouTube, Vimeo, Dailymotion, TED, and Wistia embedded
					in your content.
				</p>
			</div>
			<div class="info-card">
				<h4>Thumbnail Extraction</h4>
				<p>
					Extracts high-quality thumbnails from video platforms for rich search results.
				</p>
			</div>
			<div class="info-card">
				<h4>Live Stream Support</h4>
				<p>
					Supports live video markup for real-time streams with start/end times.
				</p>
			</div>
		</div>
	</div>
</div>

{#if showAddModal}
	<div class="modal-overlay" onclick={() => (showAddModal = false)}>
		<div class="modal" onclick={(e) => e.stopPropagation()}>
			<h3>Add Video URL</h3>
			<p>Enter a video URL from YouTube, Vimeo, or other supported platforms.</p>
			<input
				type="url"
				bind:value={newVideoUrl}
				placeholder="https://youtube.com/watch?v=..."
				class="url-input"
			/>
			<div class="modal-actions">
				<button class="btn-secondary" onclick={() => (showAddModal = false)}>Cancel</button>
				<button class="btn-primary" onclick={addVideo}>Add Video</button>
			</div>
		</div>
	</div>
{/if}

<style>
	.video-sitemap-page {
		padding: 2rem;
		max-width: 1200px;
		margin: 0 auto;
	}

	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 2rem;
	}

	.page-header h1 {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		font-size: 1.75rem;
		font-weight: 700;
		color: #1a1a1a;
		margin-bottom: 0.5rem;
	}

	.page-header p {
		color: #666;
		font-size: 0.95rem;
	}

	.header-actions {
		display: flex;
		gap: 1rem;
	}

	.btn-primary,
	.btn-secondary {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		border-radius: 6px;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
		text-decoration: none;
		border: none;
	}

	.btn-primary {
		background: #3b82f6;
		color: white;
	}

	.btn-primary:hover:not(:disabled) {
		background: #2563eb;
	}

	.btn-primary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.btn-secondary {
		background: white;
		color: #374151;
		border: 1px solid #e5e5e5;
	}

	.btn-secondary:hover {
		background: #f9fafb;
	}

	.btn-add {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 1rem;
		background: #10b981;
		color: white;
		border: none;
		border-radius: 6px;
		font-weight: 500;
		cursor: pointer;
		transition: background 0.2s;
	}

	.btn-add:hover {
		background: #059669;
	}

	.stats-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
		gap: 1rem;
		margin-bottom: 2rem;
	}

	.stat-card {
		background: white;
		border: 1px solid #e5e5e5;
		border-radius: 8px;
		padding: 1.5rem;
		text-align: center;
	}

	.stat-value {
		font-size: 1.75rem;
		font-weight: 700;
		color: #1a1a1a;
		margin-bottom: 0.25rem;
	}

	.stat-label {
		color: #666;
		font-size: 0.85rem;
	}

	.settings-card {
		background: white;
		border: 1px solid #e5e5e5;
		border-radius: 12px;
		padding: 1.5rem;
		margin-bottom: 2rem;
	}

	.settings-card h2 {
		font-size: 1.25rem;
		font-weight: 600;
		color: #1a1a1a;
		margin-bottom: 1.5rem;
	}

	.settings-grid {
		display: flex;
		gap: 2rem;
		flex-wrap: wrap;
		margin-bottom: 1.5rem;
	}

	.checkbox-label {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		cursor: pointer;
	}

	.platforms-section h3 {
		font-size: 1rem;
		font-weight: 600;
		color: #374151;
		margin-bottom: 1rem;
	}

	.platforms-grid {
		display: flex;
		gap: 1rem;
		flex-wrap: wrap;
	}

	.platform-checkbox {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 1rem;
		background: #f9fafb;
		border-radius: 6px;
		cursor: pointer;
		transition: all 0.2s;
	}

	.platform-checkbox:hover {
		background: #f3f4f6;
	}

	.platform-checkbox input:checked + .platform-name {
		color: var(--platform-color);
		font-weight: 600;
	}

	.videos-section {
		background: white;
		border: 1px solid #e5e5e5;
		border-radius: 12px;
		padding: 1.5rem;
		margin-bottom: 2rem;
	}

	.section-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1.5rem;
	}

	.section-header h2 {
		font-size: 1.25rem;
		font-weight: 600;
		color: #1a1a1a;
		margin: 0;
	}

	.loading,
	.empty-state {
		text-align: center;
		padding: 3rem;
		color: #666;
	}

	.videos-list {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.video-item {
		display: flex;
		gap: 1rem;
		padding: 1rem;
		background: #f9fafb;
		border-radius: 8px;
		align-items: center;
	}

	.video-thumbnail {
		position: relative;
		width: 160px;
		height: 90px;
		flex-shrink: 0;
		border-radius: 6px;
		overflow: hidden;
	}

	.video-thumbnail img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.duration-badge {
		position: absolute;
		bottom: 4px;
		right: 4px;
		background: rgba(0, 0, 0, 0.8);
		color: white;
		padding: 2px 6px;
		border-radius: 4px;
		font-size: 0.75rem;
		font-weight: 600;
	}

	.duration-badge.live {
		background: #ef4444;
		display: flex;
		align-items: center;
		gap: 4px;
	}

	.live-dot {
		width: 6px;
		height: 6px;
		background: white;
		border-radius: 50%;
		animation: pulse 1.5s infinite;
	}

	@keyframes pulse {
		0%,
		100% {
			opacity: 1;
		}
		50% {
			opacity: 0.5;
		}
	}

	.video-info {
		flex: 1;
		min-width: 0;
	}

	.video-info h4 {
		font-size: 0.95rem;
		font-weight: 600;
		color: #1a1a1a;
		margin: 0 0 0.5rem;
	}

	.video-meta {
		display: flex;
		gap: 1rem;
		font-size: 0.85rem;
		color: #666;
		flex-wrap: wrap;
	}

	.platform-badge {
		padding: 2px 8px;
		border-radius: 4px;
		font-weight: 600;
		font-size: 0.75rem;
		text-transform: uppercase;
	}

	.video-url {
		color: #3b82f6;
	}

	.video-actions {
		display: flex;
		gap: 0.5rem;
	}

	.icon-btn {
		padding: 0.5rem;
		background: white;
		border: 1px solid #e5e5e5;
		border-radius: 4px;
		cursor: pointer;
		color: #666;
		transition: all 0.2s;
	}

	.icon-btn:hover {
		background: #f3f4f6;
	}

	.icon-btn.danger {
		color: #ef4444;
	}

	.icon-btn.danger:hover {
		background: #fee2e2;
	}

	.info-section h3 {
		font-size: 1.25rem;
		font-weight: 600;
		color: #1a1a1a;
		margin-bottom: 1rem;
	}

	.info-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
		gap: 1.5rem;
	}

	.info-card {
		background: white;
		border: 1px solid #e5e5e5;
		border-radius: 8px;
		padding: 1.5rem;
	}

	.info-card h4 {
		font-size: 1rem;
		font-weight: 600;
		color: #1a1a1a;
		margin-bottom: 0.5rem;
	}

	.info-card p {
		color: #666;
		line-height: 1.6;
		font-size: 0.9rem;
	}

	.modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.5);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
		padding: 1rem;
	}

	.modal {
		background: white;
		border-radius: 12px;
		padding: 2rem;
		max-width: 500px;
		width: 100%;
	}

	.modal h3 {
		font-size: 1.25rem;
		font-weight: 600;
		margin-bottom: 0.5rem;
	}

	.modal p {
		color: #666;
		margin-bottom: 1.5rem;
	}

	.url-input {
		width: 100%;
		padding: 0.75rem;
		border: 1px solid #e5e5e5;
		border-radius: 6px;
		font-size: 0.95rem;
		margin-bottom: 1.5rem;
	}

	.modal-actions {
		display: flex;
		gap: 1rem;
		justify-content: flex-end;
	}

	:global(.spinning) {
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

	@media (max-width: 768px) {
		.video-sitemap-page {
			padding: 1rem;
		}

		.page-header {
			flex-direction: column;
			gap: 1rem;
		}

		.header-actions {
			width: 100%;
			flex-direction: column;
		}

		.video-item {
			flex-direction: column;
			align-items: stretch;
		}

		.video-thumbnail {
			width: 100%;
			height: 180px;
		}

		.video-actions {
			justify-content: flex-end;
		}
	}
</style>

<script lang="ts">
	/**
	 * Trading Room Video Management - Admin Dashboard
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Professional video management with room selector tabs.
	 * Upload videos to specific trading rooms or alert services.
	 *
	 * Rooms:
	 * - Day Trading Room
	 * - Swing Trading Room
	 * - Small Account Mentorship
	 *
	 * Alert Services:
	 * - SPX Profit Pulse
	 * - Explosive Swing
	 *
	 * @version 2.0.0 - December 2025
	 */

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
		IconCheck,
		IconX,
		IconChartBar,
		IconUser,
		IconBuilding
	} from '@tabler/icons-svelte';

	// ═══════════════════════════════════════════════════════════════════════════
	// TYPES
	// ═══════════════════════════════════════════════════════════════════════════

	interface TradingRoom {
		id: number;
		name: string;
		slug: string;
		type: 'trading_room' | 'alert_service';
		icon?: string;
		color?: string;
		video_count?: number;
	}

	interface Trader {
		id: number;
		name: string;
		slug: string;
		photo_url?: string;
	}

	interface Video {
		id: number;
		trading_room_id: number;
		trader_id?: number;
		title: string;
		description?: string;
		video_url: string;
		video_platform: 'vimeo' | 'youtube' | 'bunny' | 'wistia' | 'direct';
		thumbnail_url?: string;
		duration?: number;
		video_date: string;
		is_featured: boolean;
		is_published: boolean;
		views_count: number;
		trader?: Trader;
		created_at: string;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE
	// ═══════════════════════════════════════════════════════════════════════════

	// Rooms and selection
	let rooms = $state<TradingRoom[]>([]);
	let selectedRoom = $state<TradingRoom | null>(null);
	let traders = $state<Trader[]>([]);

	// Videos
	let videos = $state<Video[]>([]);
	let isLoading = $state(true);
	let error = $state('');

	// Filters
	let searchQuery = $state('');
	let selectedTrader = $state('all');

	// Modal state
	let showUploadModal = $state(false);
	let showEditModal = $state(false);
	let editingVideo = $state<Video | null>(null);
	let isSaving = $state(false);

	// Form state
	let formData = $state({
		trading_room_id: 0,
		trader_id: null as number | null,
		title: '',
		description: '',
		video_url: '',
		video_platform: 'vimeo' as 'vimeo' | 'youtube' | 'bunny' | 'wistia' | 'direct',
		thumbnail_url: '',
		video_date: new Date().toISOString().split('T')[0],
		is_published: true,
		is_featured: false
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// MOCK DATA (Replace with API calls)
	// ═══════════════════════════════════════════════════════════════════════════

	function initializeMockData() {
		rooms = [
			{ id: 1, name: 'Day Trading Room', slug: 'day-trading-room', type: 'trading_room', color: '#0984ae', video_count: 45 },
			{ id: 2, name: 'Swing Trading Room', slug: 'swing-trading-room', type: 'trading_room', color: '#10b981', video_count: 32 },
			{ id: 3, name: 'Small Account Mentorship', slug: 'small-account-mentorship', type: 'trading_room', color: '#f59e0b', video_count: 28 },
			{ id: 4, name: 'SPX Profit Pulse', slug: 'spx-profit-pulse', type: 'alert_service', color: '#ef4444', video_count: 15 },
			{ id: 5, name: 'Explosive Swing', slug: 'explosive-swing', type: 'alert_service', color: '#8b5cf6', video_count: 12 }
		];

		traders = [
			{ id: 1, name: 'John Carter', slug: 'john-carter', photo_url: '/images/traders/john-carter.jpg' },
			{ id: 2, name: 'Henry Gambell', slug: 'henry-gambell', photo_url: '/images/traders/henry-gambell.jpg' },
			{ id: 3, name: 'Danielle Shay', slug: 'danielle-shay', photo_url: '/images/traders/danielle-shay.jpg' },
			{ id: 4, name: 'Bruce Marshall', slug: 'bruce-marshall', photo_url: '/images/traders/bruce-marshall.jpg' },
			{ id: 5, name: 'Sam Shames', slug: 'sam-shames', photo_url: '/images/traders/sam-shames.jpg' },
			{ id: 6, name: 'Allison Ostrander', slug: 'allison-ostrander', photo_url: '/images/traders/allison-ostrander.jpg' }
		];

		selectedRoom = rooms[0];
		loadVideos();
	}

	async function loadVideos() {
		if (!selectedRoom) return;

		isLoading = true;
		error = '';

		try {
			// TODO: Replace with actual API call
			// const response = await fetch(`/api/admin/trading-rooms/videos/${selectedRoom.slug}`);
			// const data = await response.json();
			// videos = data.data;

			// Mock data for now
			videos = [
				{
					id: 1,
					trading_room_id: selectedRoom.id,
					trader_id: 5,
					title: 'Weekend Market Review',
					description: 'Bulls held the gains from last week and in the process now have a "prize worth fighting for" into next week as we all wait for Powell and his purple tie.',
					video_url: 'https://vimeo.com/123456789',
					video_platform: 'vimeo',
					thumbnail_url: '/images/traders/sam-shames.jpg',
					duration: 1845,
					video_date: '2025-12-05',
					is_featured: true,
					is_published: true,
					views_count: 234,
					trader: traders.find(t => t.id === 5),
					created_at: '2025-12-05T10:00:00Z'
				},
				{
					id: 2,
					trading_room_id: selectedRoom.id,
					trader_id: 4,
					title: 'Santa Is Waiting For Powell',
					description: "Bruce likes to really think about the upcoming economic calendar, such as next week's FED meeting, and how this will affect the market.",
					video_url: 'https://vimeo.com/123456790',
					video_platform: 'vimeo',
					thumbnail_url: '/images/traders/bruce-marshall.jpg',
					duration: 2100,
					video_date: '2025-12-04',
					is_featured: false,
					is_published: true,
					views_count: 189,
					trader: traders.find(t => t.id === 4),
					created_at: '2025-12-04T10:00:00Z'
				},
				{
					id: 3,
					trading_room_id: selectedRoom.id,
					trader_id: 2,
					title: 'Ready For The Ripple',
					description: "It's said there can be a ripple effect in equities due to the impact of currencies. Henry thinks the stage is set for that.",
					video_url: 'https://vimeo.com/123456791',
					video_platform: 'vimeo',
					thumbnail_url: '/images/traders/henry-gambell.jpg',
					duration: 1560,
					video_date: '2025-12-03',
					is_featured: false,
					is_published: true,
					views_count: 312,
					trader: traders.find(t => t.id === 2),
					created_at: '2025-12-03T10:00:00Z'
				}
			];
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load videos';
		} finally {
			isLoading = false;
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// ACTIONS
	// ═══════════════════════════════════════════════════════════════════════════

	function selectRoom(room: TradingRoom) {
		selectedRoom = room;
		loadVideos();
	}

	function openUploadModal() {
		formData = {
			trading_room_id: selectedRoom?.id || 0,
			trader_id: null,
			title: '',
			description: '',
			video_url: '',
			video_platform: 'vimeo',
			thumbnail_url: '',
			video_date: new Date().toISOString().split('T')[0],
			is_published: true,
			is_featured: false
		};
		showUploadModal = true;
	}

	function openEditModal(video: Video) {
		editingVideo = video;
		formData = {
			trading_room_id: video.trading_room_id,
			trader_id: video.trader_id || null,
			title: video.title,
			description: video.description || '',
			video_url: video.video_url,
			video_platform: video.video_platform,
			thumbnail_url: video.thumbnail_url || '',
			video_date: video.video_date,
			is_published: video.is_published,
			is_featured: video.is_featured
		};
		showEditModal = true;
	}

	async function saveVideo() {
		isSaving = true;

		try {
			// TODO: Replace with actual API call
			// if (editingVideo) {
			//   await fetch(`/api/admin/trading-rooms/videos/${editingVideo.id}`, {
			//     method: 'PUT',
			//     headers: { 'Content-Type': 'application/json' },
			//     body: JSON.stringify(formData)
			//   });
			// } else {
			//   await fetch('/api/admin/trading-rooms/videos', {
			//     method: 'POST',
			//     headers: { 'Content-Type': 'application/json' },
			//     body: JSON.stringify(formData)
			//   });
			// }

			// Simulate success
			await new Promise(resolve => setTimeout(resolve, 1000));

			showUploadModal = false;
			showEditModal = false;
			editingVideo = null;
			await loadVideos();
		} catch (err) {
			alert(err instanceof Error ? err.message : 'Failed to save video');
		} finally {
			isSaving = false;
		}
	}

	async function deleteVideo(video: Video) {
		if (!confirm(`Are you sure you want to delete "${video.title}"?`)) return;

		try {
			// TODO: Replace with actual API call
			// await fetch(`/api/admin/trading-rooms/videos/${video.id}`, { method: 'DELETE' });

			await loadVideos();
		} catch (err) {
			alert(err instanceof Error ? err.message : 'Failed to delete video');
		}
	}

	async function togglePublished(video: Video) {
		try {
			// TODO: Replace with actual API call
			video.is_published = !video.is_published;
		} catch (err) {
			alert(err instanceof Error ? err.message : 'Failed to update video');
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// HELPERS
	// ═══════════════════════════════════════════════════════════════════════════

	function formatDuration(seconds?: number): string {
		if (!seconds) return '';
		const h = Math.floor(seconds / 3600);
		const m = Math.floor((seconds % 3600) / 60);
		const s = seconds % 60;
		return h > 0 ? `${h}:${m.toString().padStart(2, '0')}:${s.toString().padStart(2, '0')}` : `${m}:${s.toString().padStart(2, '0')}`;
	}

	function formatDate(dateString: string): string {
		return new Date(dateString).toLocaleDateString('en-US', { month: 'short', day: 'numeric', year: 'numeric' });
	}

	function formatViews(views: number): string {
		if (views >= 1000000) return (views / 1000000).toFixed(1) + 'M';
		if (views >= 1000) return (views / 1000).toFixed(1) + 'K';
		return views.toString();
	}

	function detectPlatform(url: string): typeof formData.video_platform {
		if (url.includes('vimeo.com')) return 'vimeo';
		if (url.includes('youtube.com') || url.includes('youtu.be')) return 'youtube';
		if (url.includes('bunny.net')) return 'bunny';
		if (url.includes('wistia.com')) return 'wistia';
		return 'direct';
	}

	// Auto-detect platform when URL changes
	$effect(() => {
		if (formData.video_url) {
			formData.video_platform = detectPlatform(formData.video_url);
		}
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED
	// ═══════════════════════════════════════════════════════════════════════════

	const tradingRooms = $derived(rooms.filter(r => r.type === 'trading_room'));
	const alertServices = $derived(rooms.filter(r => r.type === 'alert_service'));

	const filteredVideos = $derived(videos.filter(video => {
		const matchesSearch = !searchQuery ||
			video.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
			video.description?.toLowerCase().includes(searchQuery.toLowerCase());
		const matchesTrader = selectedTrader === 'all' || video.trader_id?.toString() === selectedTrader;
		return matchesSearch && matchesTrader;
	}));

	const totalViews = $derived(videos.reduce((sum, v) => sum + v.views_count, 0));
	const publishedCount = $derived(videos.filter(v => v.is_published).length);

	// ═══════════════════════════════════════════════════════════════════════════
	// LIFECYCLE
	// ═══════════════════════════════════════════════════════════════════════════

	onMount(() => {
		initializeMockData();
	});
</script>

<svelte:head>
	<title>Video Management - Trading Rooms | Admin</title>
</svelte:head>

<div class="videos-page">
	<!-- Header -->
	<div class="page-header">
		<div>
			<h1>Trading Room Videos</h1>
			<p class="page-description">Manage daily videos for each trading room and alert service</p>
		</div>
		<div class="header-actions">
			<button class="btn-refresh" onclick={() => loadVideos()} disabled={isLoading}>
				<IconRefresh size={18} class={isLoading ? 'spinning' : ''} />
			</button>
			<button class="btn-primary" onclick={openUploadModal}>
				<IconPlus size={18} />
				Add Video
			</button>
		</div>
	</div>

	<!-- Room Selector Tabs -->
	<div class="room-selector">
		<!-- Trading Rooms -->
		<div class="room-group">
			<h3 class="room-group-title">
				<IconBuilding size={16} />
				Trading Rooms
			</h3>
			<div class="room-tabs">
				{#each tradingRooms as room}
					<button
						class="room-tab"
						class:active={selectedRoom?.id === room.id}
						style:--room-color={room.color}
						onclick={() => selectRoom(room)}
					>
						<span class="room-name">{room.name}</span>
						<span class="room-count">{room.video_count}</span>
					</button>
				{/each}
			</div>
		</div>

		<!-- Alert Services -->
		<div class="room-group">
			<h3 class="room-group-title">
				<IconChartBar size={16} />
				Alert Services
			</h3>
			<div class="room-tabs">
				{#each alertServices as service}
					<button
						class="room-tab"
						class:active={selectedRoom?.id === service.id}
						style:--room-color={service.color}
						onclick={() => selectRoom(service)}
					>
						<span class="room-name">{service.name}</span>
						<span class="room-count">{service.video_count}</span>
					</button>
				{/each}
			</div>
		</div>
	</div>

	<!-- Selected Room Header -->
	{#if selectedRoom}
		<div class="selected-room-header" style:--room-color={selectedRoom.color}>
			<div class="selected-room-info">
				<h2>{selectedRoom.name}</h2>
				<span class="room-type-badge">{selectedRoom.type === 'trading_room' ? 'Trading Room' : 'Alert Service'}</span>
			</div>
			<div class="selected-room-stats">
				<div class="stat">
					<span class="stat-value">{videos.length}</span>
					<span class="stat-label">Videos</span>
				</div>
				<div class="stat">
					<span class="stat-value">{formatViews(totalViews)}</span>
					<span class="stat-label">Total Views</span>
				</div>
				<div class="stat">
					<span class="stat-value">{publishedCount}</span>
					<span class="stat-label">Published</span>
				</div>
			</div>
		</div>
	{/if}

	<!-- Filters -->
	<div class="filters-bar">
		<div class="search-box">
			<IconSearch size={18} />
			<label for="search-videos" class="sr-only">Search videos</label>
			<input type="text" id="search-videos" placeholder="Search videos..." bind:value={searchQuery} />
		</div>
		<label for="trader-filter" class="sr-only">Filter by trader</label>
		<select id="trader-filter" class="filter-select" bind:value={selectedTrader}>
			<option value="all">All Traders</option>
			{#each traders as trader}
				<option value={trader.id.toString()}>{trader.name}</option>
			{/each}
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
			<button onclick={() => loadVideos()}>Try Again</button>
		</div>
	{:else if filteredVideos.length === 0}
		<div class="empty-state">
			<IconVideo size={64} />
			<h3>No videos found</h3>
			<p>Add your first video to {selectedRoom?.name}</p>
			<button class="btn-primary" onclick={openUploadModal}>
				<IconPlus size={18} />
				Add Video
			</button>
		</div>
	{:else}
		<div class="videos-table-wrapper">
			<table class="videos-table">
				<thead>
					<tr>
						<th>Video</th>
						<th>Trader</th>
						<th>Date</th>
						<th>Views</th>
						<th>Status</th>
						<th>Actions</th>
					</tr>
				</thead>
				<tbody>
					{#each filteredVideos as video}
						<tr>
							<td class="video-cell">
								<div class="video-thumbnail-small">
									{#if video.thumbnail_url}
										<img src={video.thumbnail_url} alt="" />
									{:else}
										<div class="thumbnail-placeholder-small">
											<IconVideo size={20} />
										</div>
									{/if}
									{#if video.duration}
										<span class="duration-badge">{formatDuration(video.duration)}</span>
									{/if}
								</div>
								<div class="video-info">
									<span class="video-title">{video.title}</span>
									<span class="video-platform">{video.video_platform}</span>
								</div>
							</td>
							<td>
								<div class="trader-cell">
									{#if video.trader?.photo_url}
										<img src={video.trader.photo_url} alt="" class="trader-avatar" />
									{:else}
										<div class="trader-avatar-placeholder">
											<IconUser size={16} />
										</div>
									{/if}
									<span>{video.trader?.name || 'Unassigned'}</span>
								</div>
							</td>
							<td>{formatDate(video.video_date)}</td>
							<td>{formatViews(video.views_count)}</td>
							<td>
								<button
									class="status-toggle"
									class:published={video.is_published}
									onclick={() => togglePublished(video)}
								>
									{#if video.is_published}
										<IconCheck size={14} />
										Published
									{:else}
										<IconX size={14} />
										Draft
									{/if}
								</button>
							</td>
							<td>
								<div class="action-buttons">
									<button class="btn-icon" title="Edit" onclick={() => openEditModal(video)}>
										<IconEdit size={16} />
									</button>
									<button class="btn-icon" title="Preview">
										<IconPlayerPlay size={16} />
									</button>
									<button class="btn-icon danger" title="Delete" onclick={() => deleteVideo(video)}>
										<IconTrash size={16} />
									</button>
								</div>
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	{/if}
</div>

<!-- Upload/Edit Modal -->
{#if showUploadModal || showEditModal}
	<div
		class="modal-overlay"
		role="button"
		tabindex="0"
		onclick={() => { showUploadModal = false; showEditModal = false; }}
		onkeydown={(e) => e.key === 'Escape' && (showUploadModal = false, showEditModal = false)}
	>
		<div class="modal" onclick={(e) => e.stopPropagation()} role="dialog" aria-modal="true" tabindex="-1">
			<div class="modal-header">
				<h2>{showEditModal ? 'Edit Video' : 'Add New Video'}</h2>
				<button class="modal-close" onclick={() => { showUploadModal = false; showEditModal = false; }} type="button" aria-label="Close">&times;</button>
			</div>
			<div class="modal-body">
				<!-- Room Selector (for new videos) -->
				{#if !showEditModal}
					<div class="form-group">
						<label for="room-select">Upload to Room/Service</label>
						<select id="room-select" bind:value={formData.trading_room_id}>
							<optgroup label="Trading Rooms">
								{#each tradingRooms as room}
									<option value={room.id}>{room.name}</option>
								{/each}
							</optgroup>
							<optgroup label="Alert Services">
								{#each alertServices as service}
									<option value={service.id}>{service.name}</option>
								{/each}
							</optgroup>
						</select>
					</div>
				{/if}

				<!-- Video URL -->
				<div class="form-group">
					<label for="video-url">Video URL</label>
					<input type="url" id="video-url" placeholder="https://vimeo.com/..." bind:value={formData.video_url} />
					<span class="form-hint">
						Supports: Vimeo, YouTube, Bunny.net, Wistia, or direct URL
						{#if formData.video_platform !== 'direct'}
							<span class="detected-platform">Detected: {formData.video_platform}</span>
						{/if}
					</span>
				</div>

				<!-- Title -->
				<div class="form-group">
					<label for="video-title">Title</label>
					<input type="text" id="video-title" placeholder="Weekend Market Review" bind:value={formData.title} />
				</div>

				<!-- Description -->
				<div class="form-group">
					<label for="video-description">Description</label>
					<textarea id="video-description" rows="3" placeholder="Brief description of the video content..." bind:value={formData.description}></textarea>
				</div>

				<!-- Row: Trader + Date -->
				<div class="form-row">
					<div class="form-group">
						<label for="trader-select">Trader</label>
						<select id="trader-select" bind:value={formData.trader_id}>
							<option value={null}>Select trader...</option>
							{#each traders as trader}
								<option value={trader.id}>{trader.name}</option>
							{/each}
						</select>
					</div>
					<div class="form-group">
						<label for="video-date">Video Date</label>
						<input type="date" id="video-date" bind:value={formData.video_date} />
					</div>
				</div>

				<!-- Thumbnail URL -->
				<div class="form-group">
					<label for="thumbnail-url">Thumbnail URL (optional)</label>
					<input type="url" id="thumbnail-url" placeholder="https://..." bind:value={formData.thumbnail_url} />
				</div>

				<!-- Options -->
				<div class="form-options">
					<label class="checkbox-label">
						<input type="checkbox" bind:checked={formData.is_published} />
						<span>Publish immediately</span>
					</label>
					<label class="checkbox-label">
						<input type="checkbox" bind:checked={formData.is_featured} />
						<span>Feature this video</span>
					</label>
				</div>
			</div>
			<div class="modal-footer">
				<button class="btn-secondary" onclick={() => { showUploadModal = false; showEditModal = false; }} type="button">Cancel</button>
				<button class="btn-primary" onclick={saveVideo} disabled={isSaving || !formData.title || !formData.video_url}>
					{#if isSaving}
						<span class="btn-spinner"></span>
						Saving...
					{:else}
						{showEditModal ? 'Update Video' : 'Add Video'}
					{/if}
				</button>
			</div>
		</div>
	</div>
{/if}

<style>
	.videos-page {
		max-width: 1600px;
	}

	/* Header */
	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 1.5rem;
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

	.btn-primary:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow: 0 4px 15px rgba(99, 102, 241, 0.4);
	}

	.btn-primary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
		transform: none;
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

	/* Room Selector */
	.room-selector {
		display: flex;
		flex-direction: column;
		gap: 1rem;
		margin-bottom: 1.5rem;
		padding: 1.25rem;
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 14px;
	}

	.room-group-title {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.8rem;
		font-weight: 600;
		color: #64748b;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		margin: 0 0 0.75rem 0;
	}

	.room-tabs {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
	}

	.room-tab {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.625rem 1rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 8px;
		color: #94a3b8;
		font-size: 0.9rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.room-tab:hover {
		background: rgba(99, 102, 241, 0.1);
		border-color: rgba(99, 102, 241, 0.3);
		color: #e2e8f0;
	}

	.room-tab.active {
		background: var(--room-color, #6366f1);
		border-color: var(--room-color, #6366f1);
		color: white;
	}

	.room-count {
		padding: 0.125rem 0.5rem;
		background: rgba(255, 255, 255, 0.15);
		border-radius: 12px;
		font-size: 0.75rem;
		font-weight: 600;
	}

	.room-tab.active .room-count {
		background: rgba(255, 255, 255, 0.25);
	}

	/* Selected Room Header */
	.selected-room-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1.25rem;
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 14px;
		margin-bottom: 1.5rem;
		border-left: 4px solid var(--room-color, #6366f1);
	}

	.selected-room-info h2 {
		font-size: 1.25rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0 0 0.25rem 0;
	}

	.room-type-badge {
		display: inline-block;
		padding: 0.25rem 0.625rem;
		background: rgba(99, 102, 241, 0.15);
		border-radius: 20px;
		font-size: 0.75rem;
		color: #94a3b8;
	}

	.selected-room-stats {
		display: flex;
		gap: 2rem;
	}

	.selected-room-stats .stat {
		text-align: center;
	}

	.selected-room-stats .stat-value {
		display: block;
		font-size: 1.5rem;
		font-weight: 700;
		color: #f1f5f9;
	}

	.selected-room-stats .stat-label {
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

	/* Videos Table */
	.videos-table-wrapper {
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 14px;
		overflow: hidden;
	}

	.videos-table {
		width: 100%;
		border-collapse: collapse;
	}

	.videos-table th {
		text-align: left;
		padding: 1rem 1.25rem;
		background: rgba(30, 41, 59, 0.6);
		font-size: 0.8rem;
		font-weight: 600;
		color: #64748b;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		border-bottom: 1px solid rgba(99, 102, 241, 0.1);
	}

	.videos-table td {
		padding: 1rem 1.25rem;
		border-bottom: 1px solid rgba(99, 102, 241, 0.05);
		color: #e2e8f0;
	}

	.videos-table tr:last-child td {
		border-bottom: none;
	}

	.videos-table tr:hover td {
		background: rgba(99, 102, 241, 0.05);
	}

	.video-cell {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.video-thumbnail-small {
		position: relative;
		width: 80px;
		height: 45px;
		border-radius: 6px;
		overflow: hidden;
		background: #1e293b;
		flex-shrink: 0;
	}

	.video-thumbnail-small img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.thumbnail-placeholder-small {
		width: 100%;
		height: 100%;
		display: flex;
		align-items: center;
		justify-content: center;
		color: #475569;
	}

	.duration-badge {
		position: absolute;
		bottom: 2px;
		right: 2px;
		padding: 1px 4px;
		background: rgba(0, 0, 0, 0.8);
		border-radius: 3px;
		font-size: 0.65rem;
		color: white;
	}

	.video-info {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.video-info .video-title {
		font-weight: 500;
		color: #f1f5f9;
	}

	.video-info .video-platform {
		font-size: 0.75rem;
		color: #64748b;
		text-transform: capitalize;
	}

	.trader-cell {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.trader-avatar {
		width: 28px;
		height: 28px;
		border-radius: 50%;
		object-fit: cover;
	}

	.trader-avatar-placeholder {
		width: 28px;
		height: 28px;
		border-radius: 50%;
		background: rgba(99, 102, 241, 0.15);
		display: flex;
		align-items: center;
		justify-content: center;
		color: #64748b;
	}

	.status-toggle {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.375rem 0.75rem;
		background: rgba(245, 158, 11, 0.15);
		border: 1px solid rgba(245, 158, 11, 0.3);
		border-radius: 20px;
		font-size: 0.75rem;
		font-weight: 600;
		color: #fbbf24;
		cursor: pointer;
		transition: all 0.2s;
	}

	.status-toggle.published {
		background: rgba(34, 197, 94, 0.15);
		border-color: rgba(34, 197, 94, 0.3);
		color: #4ade80;
	}

	.action-buttons {
		display: flex;
		gap: 0.5rem;
	}

	.btn-icon {
		width: 32px;
		height: 32px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(99, 102, 241, 0.1);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 6px;
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
		max-width: 600px;
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

	.form-group {
		margin-bottom: 1rem;
	}

	.form-group label {
		display: block;
		font-size: 0.85rem;
		font-weight: 500;
		color: #94a3b8;
		margin-bottom: 0.5rem;
	}

	.form-group input,
	.form-group select,
	.form-group textarea {
		width: 100%;
		padding: 0.75rem 1rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 10px;
		color: #e2e8f0;
		font-size: 0.9rem;
	}

	.form-group input:focus,
	.form-group select:focus,
	.form-group textarea:focus {
		outline: none;
		border-color: rgba(99, 102, 241, 0.5);
	}

	.form-hint {
		display: block;
		font-size: 0.75rem;
		color: #64748b;
		margin-top: 0.375rem;
	}

	.detected-platform {
		color: #4ade80;
		margin-left: 0.5rem;
	}

	.form-row {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1rem;
	}

	.form-options {
		display: flex;
		gap: 1.5rem;
		padding-top: 0.5rem;
	}

	.checkbox-label {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.9rem;
		color: #94a3b8;
		cursor: pointer;
	}

	.checkbox-label input {
		width: 18px;
		height: 18px;
		accent-color: #6366f1;
	}

	.modal-footer {
		display: flex;
		justify-content: flex-end;
		gap: 0.75rem;
		padding: 1.25rem;
		border-top: 1px solid rgba(99, 102, 241, 0.1);
	}

	.btn-spinner {
		width: 16px;
		height: 16px;
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-top-color: white;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	.sr-only {
		position: absolute;
		width: 1px;
		height: 1px;
		padding: 0;
		margin: -1px;
		overflow: hidden;
		clip: rect(0, 0, 0, 0);
		white-space: nowrap;
		border-width: 0;
	}

	@media (max-width: 768px) {
		.room-tabs {
			flex-direction: column;
		}

		.room-tab {
			justify-content: space-between;
		}

		.selected-room-header {
			flex-direction: column;
			gap: 1rem;
			text-align: center;
		}

		.selected-room-stats {
			width: 100%;
			justify-content: space-around;
		}

		.form-row {
			grid-template-columns: 1fr;
		}

		.videos-table-wrapper {
			overflow-x: auto;
		}

		.videos-table {
			min-width: 700px;
		}
	}
</style>

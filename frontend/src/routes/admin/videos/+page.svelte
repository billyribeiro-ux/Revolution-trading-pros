<script lang="ts">
	/**
	 * Trading Room Video Management - Admin Dashboard
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Professional video management with room selector tabs and category tagging.
	 * Upload videos to specific trading rooms or alert services with topic tags.
	 *
	 * Connected to real backend API:
	 * - GET /api/admin/trading-rooms - List rooms
	 * - GET /api/admin/trading-rooms/traders - List traders
	 * - GET /api/admin/trading-rooms/videos/{slug} - List videos
	 * - POST /api/admin/trading-rooms/videos - Create video
	 * - PUT /api/admin/trading-rooms/videos/{id} - Update video
	 * - DELETE /api/admin/trading-rooms/videos/{id} - Delete video
	 *
	 * @version 4.0.0 - December 2025 - Real API Integration
	 */

	import { onMount } from 'svelte';
	import IconVideo from '@tabler/icons-svelte/icons/video';
	import IconUpload from '@tabler/icons-svelte/icons/upload';
	import IconSearch from '@tabler/icons-svelte/icons/search';
	import IconFilter from '@tabler/icons-svelte/icons/filter';
	import IconPlayerPlay from '@tabler/icons-svelte/icons/player-play';
	import IconEdit from '@tabler/icons-svelte/icons/edit';
	import IconTrash from '@tabler/icons-svelte/icons/trash';
	import IconEye from '@tabler/icons-svelte/icons/eye';
	import IconClock from '@tabler/icons-svelte/icons/clock';
	import IconCalendar from '@tabler/icons-svelte/icons/calendar';
	import IconRefresh from '@tabler/icons-svelte/icons/refresh';
	import IconPlus from '@tabler/icons-svelte/icons/plus';
	import IconLink from '@tabler/icons-svelte/icons/link';
	import IconCheck from '@tabler/icons-svelte/icons/check';
	import IconX from '@tabler/icons-svelte/icons/x';
	import IconChartBar from '@tabler/icons-svelte/icons/chart-bar';
	import IconUser from '@tabler/icons-svelte/icons/user';
	import IconBuilding from '@tabler/icons-svelte/icons/building';
	import IconTag from '@tabler/icons-svelte/icons/tag';
	import IconTags from '@tabler/icons-svelte/icons/tags';
	import IconAlertCircle from '@tabler/icons-svelte/icons/alert-circle';
	import { tradingRoomApi, type TradingRoom, type Trader, type DailyVideo } from '$lib/api/trading-rooms';

	// ═══════════════════════════════════════════════════════════════════════════
	// LOCAL TYPES (extending API types)
	// ═══════════════════════════════════════════════════════════════════════════

	interface VideoCategory {
		id: string;
		name: string;
		color: string;
	}

	// Extended video type with categories mapped from tags
	interface Video extends Omit<DailyVideo, 'tags'> {
		categories: string[]; // Mapped from backend 'tags' field
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// PREDEFINED CATEGORIES
	// ═══════════════════════════════════════════════════════════════════════════

	const availableCategories: VideoCategory[] = [
		{ id: 'risk-management', name: 'Risk Management', color: '#ef4444' },
		{ id: 'options-strategies', name: 'Options Strategies', color: '#f59e0b' },
		{ id: 'macro-structure', name: 'Macro Structure', color: '#10b981' },
		{ id: 'micro-structure', name: 'Micro Structure', color: '#E6B800' },
		{ id: 'psychology', name: 'Psychology', color: '#E6B800' },
		{ id: 'technical-analysis', name: 'Technical Analysis', color: '#FFD11A' },
		{ id: 'fundamentals', name: 'Fundamentals', color: '#E6B800' },
		{ id: 'trade-setups', name: 'Trade Setups', color: '#14b8a6' },
		{ id: 'market-review', name: 'Market Review', color: '#E6B800' },
		{ id: 'earnings', name: 'Earnings', color: '#f97316' },
		{ id: 'futures', name: 'Futures', color: '#84cc16' },
		{ id: 'forex', name: 'Forex', color: '#22c55e' },
		{ id: 'crypto', name: 'Crypto', color: '#B38F00' },
		{ id: 'small-accounts', name: 'Small Accounts', color: '#eab308' },
		{ id: 'position-sizing', name: 'Position Sizing', color: '#FFD11A' },
		{ id: 'entry-exit', name: 'Entry & Exit', color: '#FFD11A' },
		{ id: 'scanner-setups', name: 'Scanner Setups', color: '#64748b' },
		{ id: 'indicators', name: 'Indicators', color: '#B38F00' }
	];

	// Helper to get category by ID
	function getCategoryById(id: string): VideoCategory | undefined {
		return availableCategories.find(c => c.id === id);
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
	let isLoadingRooms = $state(true);
	let error = $state('');
	let successMessage = $state('');

	// Pagination
	let currentPage = $state(1);
	let totalPages = $state(1);
	let totalVideos = $state(0);

	// Filters
	let searchQuery = $state('');
	let selectedTrader = $state('all');
	let selectedCategory = $state('all');

	// Modal state
	let showUploadModal = $state(false);
	let showEditModal = $state(false);
	let showReplaceModal = $state(false);
	let editingVideo = $state<Video | null>(null);
	let replacingVideo = $state<Video | null>(null);
	let isSaving = $state(false);
	let isDeleting = $state(false);
	let newVideoUrl = $state('');

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
		is_featured: false,
		categories: [] as string[] // Mapped to 'tags' when sending to API
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// CATEGORY SELECTION
	// ═══════════════════════════════════════════════════════════════════════════

	function toggleCategory(categoryId: string) {
		const index = formData.categories.indexOf(categoryId);
		if (index === -1) {
			formData.categories = [...formData.categories, categoryId];
		} else {
			formData.categories = formData.categories.filter(c => c !== categoryId);
		}
	}

	function isCategorySelected(categoryId: string): boolean {
		return formData.categories.includes(categoryId);
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// API FUNCTIONS
	// ═══════════════════════════════════════════════════════════════════════════

	/**
	 * Load trading rooms and traders from API
	 */
	async function loadRoomsAndTraders() {
		isLoadingRooms = true;
		error = '';

		try {
			// Load rooms and traders in parallel
			const [roomsResponse, tradersResponse] = await Promise.all([
				tradingRoomApi.rooms.list({ with_counts: true }),
				tradingRoomApi.traders.list({ active_only: true })
			]);

			if (roomsResponse.success) {
				rooms = roomsResponse.data;
				// Select first room by default
				if (rooms.length > 0 && !selectedRoom) {
					selectedRoom = rooms[0];
				}
			}

			if (tradersResponse.success) {
				traders = tradersResponse.data;
			}
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load rooms and traders';
			console.error('Error loading rooms/traders:', err);
		} finally {
			isLoadingRooms = false;
		}
	}

	/**
	 * Load videos for the selected room from API
	 */
	async function loadVideos() {
		if (!selectedRoom) return;

		isLoading = true;
		error = '';

		try {
			const response = await tradingRoomApi.videos.adminListByRoom(selectedRoom.slug, {
				published_only: false,
				per_page: 50,
				page: currentPage
			});

			if (response.success && response.data) {
				// Map backend 'tags' to frontend 'categories'
				videos = response.data.data.map(video => ({
					...video,
					categories: video.tags || []
				}));
				totalPages = response.data.last_page;
				totalVideos = response.data.total;
			}
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load videos';
			console.error('Error loading videos:', err);
		} finally {
			isLoading = false;
		}
	}

	/**
	 * Show success message temporarily
	 */
	function showSuccess(message: string) {
		successMessage = message;
		setTimeout(() => {
			successMessage = '';
		}, 3000);
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// ACTIONS
	// ═══════════════════════════════════════════════════════════════════════════

	function selectRoom(room: TradingRoom) {
		selectedRoom = room;
		currentPage = 1;
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
			is_featured: false,
			categories: []
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
			video_date: typeof video.video_date === 'string' ? video.video_date.split('T')[0] : video.video_date,
			is_published: video.is_published,
			is_featured: video.is_featured,
			categories: video.categories || []
		};
		showEditModal = true;
	}

	function openReplaceModal(video: Video) {
		replacingVideo = video;
		newVideoUrl = video.video_url;
		showReplaceModal = true;
	}

	async function replaceVideo() {
		if (!replacingVideo || !newVideoUrl) return;

		isSaving = true;
		error = '';

		try {
			const response = await tradingRoomApi.videos.update(replacingVideo.id, {
				video_url: newVideoUrl,
				video_platform: detectPlatform(newVideoUrl)
			});

			if (response.success) {
				showSuccess('Video replaced successfully');
				showReplaceModal = false;
				replacingVideo = null;
				newVideoUrl = '';
				await loadVideos();
			}
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to replace video';
			console.error('Error replacing video:', err);
		} finally {
			isSaving = false;
		}
	}

	async function saveVideo() {
		isSaving = true;
		error = '';

		try {
			// Map frontend 'categories' to backend 'tags'
			const apiData = {
				trading_room_id: formData.trading_room_id,
				trader_id: formData.trader_id,
				title: formData.title,
				description: formData.description,
				video_url: formData.video_url,
				video_platform: formData.video_platform,
				thumbnail_url: formData.thumbnail_url || undefined,
				video_date: formData.video_date,
				is_published: formData.is_published,
				is_featured: formData.is_featured,
				tags: formData.categories // Map categories to tags for backend
			};

			if (editingVideo) {
				const response = await tradingRoomApi.videos.update(editingVideo.id, apiData);
				if (response.success) {
					showSuccess('Video updated successfully');
				}
			} else {
				const response = await tradingRoomApi.videos.create(apiData);
				if (response.success) {
					showSuccess('Video created successfully');
				}
			}

			showUploadModal = false;
			showEditModal = false;
			editingVideo = null;
			await loadVideos();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to save video';
			console.error('Error saving video:', err);
		} finally {
			isSaving = false;
		}
	}

	async function deleteVideo(video: Video) {
		if (!confirm(`Are you sure you want to delete "${video.title}"?`)) return;

		isDeleting = true;
		error = '';

		try {
			const response = await tradingRoomApi.videos.delete(video.id);
			if (response.success) {
				showSuccess('Video deleted successfully');
				await loadVideos();
			}
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to delete video';
			console.error('Error deleting video:', err);
		} finally {
			isDeleting = false;
		}
	}

	async function togglePublished(video: Video) {
		try {
			const newStatus = !video.is_published;
			const response = await tradingRoomApi.videos.update(video.id, {
				is_published: newStatus
			});

			if (response.success) {
				video.is_published = newStatus;
				showSuccess(`Video ${newStatus ? 'published' : 'unpublished'}`);
			}
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to update video';
			console.error('Error toggling publish status:', err);
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

	// Filter videos by search, trader, and category
	const filteredVideos = $derived(videos.filter(video => {
		// Search matches title, description, or category names
		const searchLower = searchQuery.toLowerCase();
		const categoryNames = video.categories.map(c => getCategoryById(c)?.name.toLowerCase() || '');
		const matchesSearch = !searchQuery ||
			video.title.toLowerCase().includes(searchLower) ||
			video.description?.toLowerCase().includes(searchLower) ||
			categoryNames.some(name => name.includes(searchLower));

		// Trader filter
		const matchesTrader = selectedTrader === 'all' || video.trader_id?.toString() === selectedTrader;

		// Category filter
		const matchesCategory = selectedCategory === 'all' || video.categories.includes(selectedCategory);

		return matchesSearch && matchesTrader && matchesCategory;
	}));

	const totalViews = $derived(videos.reduce((sum, v) => sum + v.views_count, 0));
	const publishedCount = $derived(videos.filter(v => v.is_published).length);

	// Get unique categories used in current videos
	const usedCategories = $derived(() => {
		const categoryIds = new Set<string>();
		videos.forEach(v => v.categories.forEach(c => categoryIds.add(c)));
		return Array.from(categoryIds).map(id => getCategoryById(id)).filter(Boolean) as VideoCategory[];
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// LIFECYCLE
	// ═══════════════════════════════════════════════════════════════════════════

	onMount(async () => {
		// Load rooms and traders first
		await loadRoomsAndTraders();
		// Then load videos for selected room
		if (selectedRoom) {
			await loadVideos();
		}
	});

	// Load videos when selected room changes
	$effect(() => {
		if (selectedRoom && !isLoadingRooms) {
			loadVideos();
		}
	});
</script>

<svelte:head>
	<title>Video Management - Trading Rooms | Admin</title>
</svelte:head>

<div class="videos-page">
	<!-- Success/Error Messages -->
	{#if successMessage}
		<div class="alert alert-success">
			<IconCheck size={18} />
			{successMessage}
		</div>
	{/if}

	{#if error}
		<div class="alert alert-error">
			<IconAlertCircle size={18} />
			{error}
			<button class="alert-close" onclick={() => error = ''}>
				<IconX size={16} />
			</button>
		</div>
	{/if}

	<!-- Header -->
	<div class="page-header">
		<div>
			<h1>Trading Room Videos</h1>
			<p class="page-description">Manage daily videos with category tags for each room and service</p>
		</div>
		<div class="header-actions">
			<button class="btn-refresh" onclick={() => loadVideos()} disabled={isLoading}>
				<IconRefresh size={18} class={isLoading ? 'spinning' : ''} />
			</button>
			<button class="btn-primary" onclick={openUploadModal} disabled={!selectedRoom}>
				<IconPlus size={18} />
				Add Video
			</button>
		</div>
	</div>

	<!-- Room Selector Tabs -->
	{#if isLoadingRooms}
		<div class="room-selector loading">
			<div class="spinner"></div>
			<p>Loading rooms...</p>
		</div>
	{:else if rooms.length === 0}
		<div class="room-selector empty">
			<IconBuilding size={32} />
			<p>No trading rooms found. Create rooms in the Trading Rooms admin.</p>
		</div>
	{:else}
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
							style:--room-color={room.color || '#E6B800'}
							onclick={() => selectRoom(room)}
						>
							<span class="room-name">{room.name}</span>
							<span class="room-count">{room.daily_videos_count || 0}</span>
						</button>
					{/each}
					{#if tradingRooms.length === 0}
						<p class="no-rooms">No trading rooms</p>
					{/if}
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
							style:--room-color={service.color || '#B38F00'}
							onclick={() => selectRoom(service)}
						>
							<span class="room-name">{service.name}</span>
							<span class="room-count">{service.daily_videos_count || 0}</span>
						</button>
					{/each}
					{#if alertServices.length === 0}
						<p class="no-rooms">No alert services</p>
					{/if}
				</div>
			</div>
		</div>
	{/if}

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
			<label for="search-videos" class="sr-only">Search videos or categories</label>
			<input type="text" id="search-videos" placeholder="Search videos, categories..." bind:value={searchQuery} />
		</div>
		<div class="filter-group">
			<label for="category-filter" class="sr-only">Filter by category</label>
			<select id="category-filter" class="filter-select" bind:value={selectedCategory}>
				<option value="all">All Categories</option>
				{#each availableCategories as category}
					<option value={category.id}>{category.name}</option>
				{/each}
			</select>
		</div>
		<div class="filter-group">
			<label for="trader-filter" class="sr-only">Filter by trader</label>
			<select id="trader-filter" class="filter-select" bind:value={selectedTrader}>
				<option value="all">All Traders</option>
				{#each traders as trader}
					<option value={trader.id.toString()}>{trader.name}</option>
				{/each}
			</select>
		</div>
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
			<p>
				{#if searchQuery || selectedCategory !== 'all' || selectedTrader !== 'all'}
					No videos match your filters. Try adjusting your search.
				{:else}
					Add your first video to {selectedRoom?.name}
				{/if}
			</p>
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
						<th>Categories</th>
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
							<td class="categories-cell">
								<div class="category-tags">
									{#each video.categories.slice(0, 3) as categoryId}
										{@const category = getCategoryById(categoryId)}
										{#if category}
											<span class="category-tag" style:--tag-color={category.color}>
												{category.name}
											</span>
										{/if}
									{/each}
									{#if video.categories.length > 3}
										<span class="category-more">+{video.categories.length - 3}</span>
									{/if}
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
									<button class="btn-icon" title="Replace Video" onclick={() => openReplaceModal(video)}>
										<IconLink size={16} />
									</button>
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
		onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && (showUploadModal = false, showEditModal = false)}
	>
		<div class="modal modal-large" onclick={(e: MouseEvent) => e.stopPropagation()} onkeydown={(e: KeyboardEvent) => e.stopPropagation()} role="dialog" aria-modal="true" tabindex="-1">
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
					<input type="text" id="video-title" placeholder="Risk Management Fundamentals" bind:value={formData.title} />
				</div>

				<!-- Description -->
				<div class="form-group">
					<label for="video-description">Description</label>
					<textarea id="video-description" rows="3" placeholder="Brief description of the video content..." bind:value={formData.description}></textarea>
				</div>

				<!-- Categories Section -->
				<div class="form-group">
					<label>
						<IconTags size={16} style="display: inline; vertical-align: middle; margin-right: 4px;" />
						Categories (select all that apply)
					</label>
					<div class="categories-grid">
						{#each availableCategories as category}
							<button
								type="button"
								class="category-btn"
								class:selected={isCategorySelected(category.id)}
								style:--tag-color={category.color}
								onclick={() => toggleCategory(category.id)}
							>
								{#if isCategorySelected(category.id)}
									<IconCheck size={14} />
								{/if}
								{category.name}
							</button>
						{/each}
					</div>
					{#if formData.categories.length > 0}
						<div class="selected-categories">
							<span class="selected-count">{formData.categories.length} selected:</span>
							{#each formData.categories as categoryId}
								{@const category = getCategoryById(categoryId)}
								{#if category}
									<span class="selected-tag" style:--tag-color={category.color}>
										{category.name}
										<button type="button" onclick={() => toggleCategory(categoryId)} aria-label="Remove {category.name}">
											<IconX size={12} />
										</button>
									</span>
								{/if}
							{/each}
						</div>
					{/if}
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

<!-- Replace Video Modal -->
{#if showReplaceModal && replacingVideo}
	<div
		class="modal-overlay"
		role="button"
		tabindex="0"
		onclick={() => { showReplaceModal = false; replacingVideo = null; newVideoUrl = ''; }}
		onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && (showReplaceModal = false, replacingVideo = null, newVideoUrl = '')}
	>
		<div class="modal" onclick={(e: MouseEvent) => e.stopPropagation()} onkeydown={(e: KeyboardEvent) => e.stopPropagation()} role="dialog" aria-modal="true" tabindex="-1">
			<div class="modal-header">
				<h2>Replace Video</h2>
				<button class="modal-close" onclick={() => { showReplaceModal = false; replacingVideo = null; newVideoUrl = ''; }} type="button" aria-label="Close">&times;</button>
			</div>
			<div class="modal-body">
				<div class="replace-info">
					<p><strong>Current video:</strong> {replacingVideo.title}</p>
					<p class="text-muted">All metadata (title, description, categories, featured status) will be kept. Only the video URL will be replaced.</p>
				</div>

				<div class="form-group">
					<label for="new-video-url">New Video URL</label>
					<input
						type="url"
						id="new-video-url"
						placeholder="https://your-video-url.com/video.mp4"
						bind:value={newVideoUrl}
						autofocus
					/>
					<small class="form-hint">Platform will be auto-detected from URL</small>
				</div>

				{#if newVideoUrl}
					<div class="platform-preview">
						<span>Detected platform: <strong>{detectPlatform(newVideoUrl)}</strong></span>
					</div>
				{/if}
			</div>
			<div class="modal-footer">
				<button class="btn-secondary" onclick={() => { showReplaceModal = false; replacingVideo = null; newVideoUrl = ''; }} type="button">Cancel</button>
				<button class="btn-primary" onclick={replaceVideo} disabled={isSaving || !newVideoUrl}>
					{#if isSaving}
						<span class="btn-spinner"></span>
						Replacing...
					{:else}
						<IconLink size={16} />
						Replace Video
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
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 10px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-refresh:hover {
		background: rgba(230, 184, 0, 0.2);
		color: #E6B800;
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
		background: linear-gradient(135deg, #E6B800 0%, #B38F00 100%);
		color: #0D1117;
		border: none;
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-primary:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow: 0 4px 15px rgba(230, 184, 0, 0.4);
	}

	.btn-primary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
		transform: none;
	}

	.btn-secondary {
		padding: 0.75rem 1.25rem;
		background: rgba(230, 184, 0, 0.1);
		color: #94a3b8;
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-secondary:hover {
		background: rgba(230, 184, 0, 0.2);
		color: #e2e8f0;
	}

	/* Replace Modal Specific */
	.replace-info {
		background: rgba(230, 184, 0, 0.1);
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 8px;
		padding: 1rem;
		margin-bottom: 1.5rem;
	}

	.replace-info p {
		margin: 0.5rem 0;
		color: #cbd5e1;
	}

	.replace-info strong {
		color: #f1f5f9;
	}

	.text-muted {
		color: #64748b !important;
		font-size: 0.875rem;
	}

	.form-hint {
		display: block;
		margin-top: 0.5rem;
		color: #64748b;
		font-size: 0.875rem;
	}

	.platform-preview {
		background: rgba(34, 197, 94, 0.1);
		border: 1px solid rgba(34, 197, 94, 0.2);
		border-radius: 6px;
		padding: 0.75rem;
		margin-top: 1rem;
		color: #4ade80;
		font-size: 0.875rem;
	}

	.platform-preview strong {
		text-transform: uppercase;
	}

	/* Alert Messages */
	.alert {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem 1.25rem;
		border-radius: 10px;
		margin-bottom: 1.5rem;
		font-size: 0.9rem;
	}

	.alert-success {
		background: rgba(34, 197, 94, 0.15);
		border: 1px solid rgba(34, 197, 94, 0.3);
		color: #4ade80;
	}

	.alert-error {
		background: rgba(239, 68, 68, 0.15);
		border: 1px solid rgba(239, 68, 68, 0.3);
		color: #f87171;
	}

	.alert-close {
		margin-left: auto;
		background: transparent;
		border: none;
		color: inherit;
		cursor: pointer;
		opacity: 0.7;
		transition: opacity 0.2s;
	}

	.alert-close:hover {
		opacity: 1;
	}

	/* Room Selector */
	.room-selector {
		display: flex;
		flex-direction: column;
		gap: 1rem;
		margin-bottom: 1.5rem;
		padding: 1.25rem;
		background: rgba(30, 41, 59, 0.6);
	}

	.room-selector.loading,
	.room-selector.empty {
		align-items: center;
		justify-content: center;
		padding: 3rem;
		text-align: center;
		color: #64748b;
	}

	.room-selector.loading p,
	.room-selector.empty p {
		margin: 1rem 0 0;
	}

	.no-rooms {
		color: #64748b;
		font-size: 0.85rem;
		padding: 0.5rem;
	}

	.room-selector:not(.loading):not(.empty) {
		border: 1px solid rgba(230, 184, 0, 0.1);
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
		border: 1px solid rgba(230, 184, 0, 0.1);
		border-radius: 8px;
		color: #94a3b8;
		font-size: 0.9rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.room-tab:hover {
		background: rgba(230, 184, 0, 0.1);
		border-color: rgba(230, 184, 0, 0.3);
		color: #e2e8f0;
	}

	.room-tab.active {
		background: var(--room-color, #E6B800);
		border-color: var(--room-color, #E6B800);
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
		border: 1px solid rgba(230, 184, 0, 0.1);
		border-radius: 14px;
		margin-bottom: 1.5rem;
		border-left: 4px solid var(--room-color, #E6B800);
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
		background: rgba(230, 184, 0, 0.15);
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
		border: 1px solid rgba(230, 184, 0, 0.1);
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
		border: 1px solid rgba(230, 184, 0, 0.1);
		border-radius: 10px;
		color: #e2e8f0;
		font-size: 0.9rem;
		cursor: pointer;
	}

	/* Videos Table */
	.videos-table-wrapper {
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(230, 184, 0, 0.1);
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
		border-bottom: 1px solid rgba(230, 184, 0, 0.1);
	}

	.videos-table td {
		padding: 1rem 1.25rem;
		border-bottom: 1px solid rgba(230, 184, 0, 0.05);
		color: #e2e8f0;
	}

	.videos-table tr:last-child td {
		border-bottom: none;
	}

	.videos-table tr:hover td {
		background: rgba(230, 184, 0, 0.05);
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

	/* Category Tags in Table */
	.categories-cell {
		min-width: 200px;
	}

	.category-tags {
		display: flex;
		flex-wrap: wrap;
		gap: 0.375rem;
	}

	.category-tag {
		display: inline-flex;
		align-items: center;
		padding: 0.25rem 0.5rem;
		background: color-mix(in srgb, var(--tag-color, #E6B800) 15%, transparent);
		border: 1px solid color-mix(in srgb, var(--tag-color, #E6B800) 30%, transparent);
		border-radius: 4px;
		font-size: 0.7rem;
		font-weight: 500;
		color: var(--tag-color, #E6B800);
		white-space: nowrap;
	}

	.category-more {
		display: inline-flex;
		align-items: center;
		padding: 0.25rem 0.5rem;
		background: rgba(100, 116, 139, 0.15);
		border-radius: 4px;
		font-size: 0.7rem;
		font-weight: 500;
		color: #94a3b8;
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
		background: rgba(230, 184, 0, 0.15);
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
		background: rgba(230, 184, 0, 0.1);
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 6px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-icon:hover {
		background: rgba(230, 184, 0, 0.2);
		color: #E6B800;
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
		border: 3px solid rgba(230, 184, 0, 0.2);
		border-top-color: #E6B800;
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin-bottom: 1rem;
	}

	.checkbox-label input {
		width: 18px;
		height: 18px;
		accent-color: #E6B800;
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
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 16px;
		width: 100%;
		max-width: 600px;
		max-height: 90vh;
		overflow: auto;
	}

	.modal-large {
		max-width: 800px;
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1.25rem;
		border-bottom: 1px solid rgba(230, 184, 0, 0.1);
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
		background: rgba(230, 184, 0, 0.1);
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
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 10px;
		color: #e2e8f0;
		font-size: 0.9rem;
	}

	.form-group input:focus,
	.form-group select:focus,
	.form-group textarea:focus {
		outline: none;
		border-color: rgba(230, 184, 0, 0.5);
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

	/* Categories Grid in Modal */
	.categories-grid {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
		padding: 0.75rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 10px;
	}

	.category-btn {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.5rem 0.75rem;
		background: rgba(100, 116, 139, 0.1);
		border: 1px solid rgba(100, 116, 139, 0.2);
		border-radius: 6px;
		font-size: 0.8rem;
		font-weight: 500;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.15s;
	}

	.category-btn:hover {
		background: color-mix(in srgb, var(--tag-color, #E6B800) 15%, transparent);
		border-color: color-mix(in srgb, var(--tag-color, #E6B800) 30%, transparent);
		color: var(--tag-color, #E6B800);
	}

	.category-btn.selected {
		background: color-mix(in srgb, var(--tag-color, #E6B800) 20%, transparent);
		border-color: var(--tag-color, #E6B800);
		color: var(--tag-color, #E6B800);
	}

	.selected-categories {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
		margin-top: 0.75rem;
		padding: 0.75rem;
		background: rgba(230, 184, 0, 0.05);
		border-radius: 8px;
	}

	.selected-count {
		font-size: 0.75rem;
		color: #64748b;
		margin-right: 0.5rem;
	}

	.selected-tag {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.375rem 0.625rem;
		background: color-mix(in srgb, var(--tag-color, #E6B800) 15%, transparent);
		border: 1px solid color-mix(in srgb, var(--tag-color, #E6B800) 30%, transparent);
		border-radius: 4px;
		font-size: 0.75rem;
		font-weight: 500;
		color: var(--tag-color, #E6B800);
	}

	.selected-tag button {
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 0;
		background: transparent;
		border: none;
		color: inherit;
		cursor: pointer;
		opacity: 0.7;
		transition: opacity 0.15s;
	}

	.selected-tag button:hover {
		opacity: 1;
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
		accent-color: #E6B800;
	}

	.modal-footer {
		display: flex;
		justify-content: flex-end;
		gap: 0.75rem;
		padding: 1.25rem;
		border-top: 1px solid rgba(230, 184, 0, 0.1);
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
			min-width: 900px;
		}

		.categories-grid {
			max-height: 200px;
			overflow-y: auto;
		}
	}
</style>

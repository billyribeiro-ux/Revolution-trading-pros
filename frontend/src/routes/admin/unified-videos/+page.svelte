<script lang="ts">
	/**
	 * Unified Video Management - Admin Dashboard
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Apple ICT 11+ Principal Engineer Grade - January 2026
	 *
	 * Complete video management with:
	 * - All content types (Daily Video, Weekly Watchlist, Learning Center, Archives)
	 * - Multi-room assignment with "Upload to All" checkbox
	 * - Bunny.net direct upload integration
	 * - Bulk operations (assign, publish, delete)
	 * - Tag-based filtering
	 *
	 * @version 5.0.0 - January 2026 - Unified Video System
	 */

	import { onMount } from 'svelte';
	import IconVideo from '@tabler/icons-svelte/icons/video';
	import IconUpload from '@tabler/icons-svelte/icons/upload';
	import IconSearch from '@tabler/icons-svelte/icons/search';
	import IconPlayerPlay from '@tabler/icons-svelte/icons/player-play';
	import IconEdit from '@tabler/icons-svelte/icons/edit';
	import IconTrash from '@tabler/icons-svelte/icons/trash';
	import IconEye from '@tabler/icons-svelte/icons/eye';
	import IconCalendar from '@tabler/icons-svelte/icons/calendar';
	import IconRefresh from '@tabler/icons-svelte/icons/refresh';
	import IconPlus from '@tabler/icons-svelte/icons/plus';
	import IconCheck from '@tabler/icons-svelte/icons/check';
	import IconX from '@tabler/icons-svelte/icons/x';
	import IconChartBar from '@tabler/icons-svelte/icons/chart-bar';
	import IconUser from '@tabler/icons-svelte/icons/user';
	import IconBuilding from '@tabler/icons-svelte/icons/building';
	import IconTags from '@tabler/icons-svelte/icons/tags';
	import IconAlertCircle from '@tabler/icons-svelte/icons/alert-circle';
	import IconListCheck from '@tabler/icons-svelte/icons/list-check';
	import IconSchool from '@tabler/icons-svelte/icons/school';
	import IconArchive from '@tabler/icons-svelte/icons/archive';
	import IconSquareCheck from '@tabler/icons-svelte/icons/square-check';
	import IconSquare from '@tabler/icons-svelte/icons/square';
	import IconDotsVertical from '@tabler/icons-svelte/icons/dots-vertical';
	import IconCloudUpload from '@tabler/icons-svelte/icons/cloud-upload';
	import IconLink from '@tabler/icons-svelte/icons/link';
	import BunnyVideoUploader from '$lib/components/admin/BunnyVideoUploader.svelte';
	import { 
		unifiedVideoApi, 
		CONTENT_TYPES, 
		AVAILABLE_TAGS,
		getTagBySlug,
		getContentTypeLabel,
		getContentTypeColor,
		type UnifiedVideo, 
		type RoomInfo, 
		type TraderInfo,
		type TagDetail
	} from '$lib/api/unified-videos';

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE
	// ═══════════════════════════════════════════════════════════════════════════

	// Data
	let videos = $state<UnifiedVideo[]>([]);
	let rooms = $state<RoomInfo[]>([]);
	let traders = $state<TraderInfo[]>([]);
	let stats = $state({ total: 0, published: 0, by_type: { daily_video: 0, weekly_watchlist: 0, learning_center: 0, room_archive: 0 }, total_views: 0 });

	// Loading states
	let isLoading = $state(true);
	let isLoadingOptions = $state(true);
	let isSaving = $state(false);

	// Messages
	let error = $state('');
	let successMessage = $state('');

	// Pagination
	let currentPage = $state(1);
	let totalPages = $state(1);
	let totalVideos = $state(0);
	let perPage = $state(20);

	// Filters
	let searchQuery = $state('');
	let selectedContentType = $state('all');
	let selectedRoom = $state('all');
	let selectedTrader = $state('all');
	let selectedTag = $state('all');

	// Selection for bulk actions
	let selectedVideoIds = $state<number[]>([]);
	let showBulkMenu = $state(false);

	// Modal state
	let showUploadModal = $state(false);
	let showEditModal = $state(false);
	let editingVideo = $state<UnifiedVideo | null>(null);

	// Form state
	// Upload mode: 'url' for pasting URL, 'upload' for direct Bunny upload
	let uploadMode = $state<'url' | 'upload'>('upload');

	let formData = $state({
		title: '',
		description: '',
		video_url: '',
		video_platform: 'bunny' as string,
		content_type: 'daily_video' as string,
		video_date: new Date().toISOString().split('T')[0],
		trader_id: null as number | null,
		is_published: true,
		is_featured: false,
		tags: [] as string[],
		room_ids: [] as number[],
		upload_to_all: false,
		thumbnail_url: '',
		difficulty_level: '',
		duration: null as number | null
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// API FUNCTIONS
	// ═══════════════════════════════════════════════════════════════════════════

	async function loadOptions() {
		isLoadingOptions = true;
		try {
			const [optionsRes, statsRes] = await Promise.all([
				unifiedVideoApi.options(),
				unifiedVideoApi.stats()
			]);

			if (optionsRes.success) {
				rooms = optionsRes.data.trading_rooms;
				traders = optionsRes.data.traders;
			}

			if (statsRes.success) {
				stats = statsRes.data;
			}
		} catch (err) {
			console.error('Failed to load options:', err);
		} finally {
			isLoadingOptions = false;
		}
	}

	async function loadVideos() {
		isLoading = true;
		error = '';

		try {
			const params: Record<string, any> = {
				page: currentPage,
				per_page: perPage
			};

			if (selectedContentType !== 'all') params.content_type = selectedContentType;
			if (selectedRoom !== 'all') params.room_id = parseInt(selectedRoom);
			if (selectedTrader !== 'all') params.trader_id = parseInt(selectedTrader);
			if (selectedTag !== 'all') params.tags = selectedTag;
			if (searchQuery) params.search = searchQuery;

			const response = await unifiedVideoApi.list(params);

			if (response.success) {
				videos = response.data;
				totalPages = response.meta.last_page;
				totalVideos = response.meta.total;
			}
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load videos';
			console.error('Error loading videos:', err);
		} finally {
			isLoading = false;
		}
	}

	function showSuccess(message: string) {
		successMessage = message;
		setTimeout(() => { successMessage = ''; }, 3000);
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// MODAL ACTIONS
	// ═══════════════════════════════════════════════════════════════════════════

	function openUploadModal() {
		formData = {
			title: '',
			description: '',
			video_url: '',
			video_platform: 'bunny',
			content_type: 'daily_video',
			video_date: new Date().toISOString().split('T')[0],
			trader_id: null,
			is_published: true,
			is_featured: false,
			tags: [],
			room_ids: [],
			upload_to_all: false,
			thumbnail_url: '',
			difficulty_level: '',
			duration: null
		};
		showUploadModal = true;
	}

	function openEditModal(video: UnifiedVideo) {
		editingVideo = video;
		formData = {
			title: video.title,
			description: video.description || '',
			video_url: video.video_url,
			video_platform: video.video_platform,
			content_type: video.content_type,
			video_date: video.video_date,
			trader_id: video.trader?.id || null,
			is_published: video.is_published,
			is_featured: video.is_featured,
			tags: video.tags,
			room_ids: video.rooms.map(r => r.id),
			upload_to_all: false,
			thumbnail_url: video.thumbnail_url || '',
			difficulty_level: '',
			duration: video.duration
		};
		showEditModal = true;
	}

	function closeModals() {
		showUploadModal = false;
		showEditModal = false;
		editingVideo = null;
	}

	async function saveVideo() {
		isSaving = true;
		error = '';

		try {
			const data = {
				title: formData.title,
				description: formData.description || undefined,
				video_url: formData.video_url,
				video_platform: formData.video_platform,
				content_type: formData.content_type,
				video_date: formData.video_date,
				trader_id: formData.trader_id,
				is_published: formData.is_published,
				is_featured: formData.is_featured,
				tags: formData.tags,
				room_ids: formData.room_ids,
				upload_to_all: formData.upload_to_all,
				thumbnail_url: formData.thumbnail_url || undefined,
				duration: formData.duration || undefined
			};

			if (editingVideo) {
				const response = await unifiedVideoApi.update(editingVideo.id, data);
				if (response.success) {
					showSuccess('Video updated successfully');
				}
			} else {
				const response = await unifiedVideoApi.create(data);
				if (response.success) {
					showSuccess('Video created successfully');
				}
			}

			closeModals();
			await loadVideos();
			await loadOptions(); // Refresh stats
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to save video';
		} finally {
			isSaving = false;
		}
	}

	async function deleteVideo(video: UnifiedVideo) {
		if (!confirm(`Delete "${video.title}"? This cannot be undone.`)) return;

		try {
			const response = await unifiedVideoApi.delete(video.id);
			if (response.success) {
				showSuccess('Video deleted');
				await loadVideos();
				await loadOptions();
			}
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to delete video';
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// BULK ACTIONS
	// ═══════════════════════════════════════════════════════════════════════════

	function toggleSelectAll() {
		if (selectedVideoIds.length === videos.length) {
			selectedVideoIds = [];
		} else {
			selectedVideoIds = videos.map(v => v.id);
		}
	}

	function toggleSelectVideo(id: number) {
		if (selectedVideoIds.includes(id)) {
			selectedVideoIds = selectedVideoIds.filter(vid => vid !== id);
		} else {
			selectedVideoIds = [...selectedVideoIds, id];
		}
	}

	async function bulkPublish(publish: boolean) {
		if (selectedVideoIds.length === 0) return;

		try {
			const response = await unifiedVideoApi.bulkPublish({ video_ids: selectedVideoIds, publish });
			if (response.success) {
				showSuccess(`${response.count} videos ${publish ? 'published' : 'unpublished'}`);
				selectedVideoIds = [];
				await loadVideos();
			}
		} catch (err) {
			error = err instanceof Error ? err.message : 'Bulk action failed';
		}
		showBulkMenu = false;
	}

	async function bulkDelete() {
		if (selectedVideoIds.length === 0) return;
		if (!confirm(`Delete ${selectedVideoIds.length} videos? This cannot be undone.`)) return;

		try {
			const response = await unifiedVideoApi.bulkDelete({ video_ids: selectedVideoIds });
			if (response.success) {
				showSuccess(`${response.count} videos deleted`);
				selectedVideoIds = [];
				await loadVideos();
				await loadOptions();
			}
		} catch (err) {
			error = err instanceof Error ? err.message : 'Bulk delete failed';
		}
		showBulkMenu = false;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// HELPERS
	// ═══════════════════════════════════════════════════════════════════════════

	function toggleTag(slug: string) {
		if (formData.tags.includes(slug)) {
			formData.tags = formData.tags.filter(t => t !== slug);
		} else {
			formData.tags = [...formData.tags, slug];
		}
	}

	function toggleRoom(id: number) {
		if (formData.room_ids.includes(id)) {
			formData.room_ids = formData.room_ids.filter(r => r !== id);
		} else {
			formData.room_ids = [...formData.room_ids, id];
		}
	}

	function formatViews(views: number): string {
		if (views >= 1000000) return (views / 1000000).toFixed(1) + 'M';
		if (views >= 1000) return (views / 1000).toFixed(1) + 'K';
		return views.toString();
	}

	function formatDate(dateString: string): string {
		return new Date(dateString).toLocaleDateString('en-US', { month: 'short', day: 'numeric', year: 'numeric' });
	}

	function getContentTypeIcon(type: string) {
		switch (type) {
			case 'daily_video': return IconVideo;
			case 'weekly_watchlist': return IconListCheck;
			case 'learning_center': return IconSchool;
			case 'room_archive': return IconArchive;
			default: return IconVideo;
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// EFFECTS & LIFECYCLE
	// ═══════════════════════════════════════════════════════════════════════════

	$effect(() => {
		// Reload when filters change
		if (!isLoadingOptions) {
			loadVideos();
		}
	});

	onMount(async () => {
		await loadOptions();
		await loadVideos();
	});
</script>

<svelte:head>
	<title>Unified Video Management | Admin</title>
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
			<h1>Video Management</h1>
			<p class="page-description">Manage all video content across rooms and services</p>
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

	<!-- Stats Cards -->
	<div class="stats-grid">
		<div class="stat-card">
			<div class="stat-icon" style="background: rgba(99, 102, 241, 0.15); color: #818cf8;">
				<IconVideo size={24} />
			</div>
			<div class="stat-info">
				<span class="stat-value">{stats.total}</span>
				<span class="stat-label">Total Videos</span>
			</div>
		</div>
		<div class="stat-card">
			<div class="stat-icon" style="background: rgba(34, 197, 94, 0.15); color: #4ade80;">
				<IconCheck size={24} />
			</div>
			<div class="stat-info">
				<span class="stat-value">{stats.published}</span>
				<span class="stat-label">Published</span>
			</div>
		</div>
		<div class="stat-card">
			<div class="stat-icon" style="background: rgba(245, 158, 11, 0.15); color: #fbbf24;">
				<IconEye size={24} />
			</div>
			<div class="stat-info">
				<span class="stat-value">{formatViews(stats.total_views)}</span>
				<span class="stat-label">Total Views</span>
			</div>
		</div>
		<div class="stat-card">
			<div class="stat-icon" style="background: rgba(139, 92, 246, 0.15); color: #a78bfa;">
				<IconChartBar size={24} />
			</div>
			<div class="stat-info">
				<span class="stat-value">{stats.by_type.daily_video}</span>
				<span class="stat-label">Daily Videos</span>
			</div>
		</div>
	</div>

	<!-- Content Type Tabs -->
	<div class="content-type-tabs">
		<button class="type-tab" class:active={selectedContentType === 'all'} onclick={() => { selectedContentType = 'all'; currentPage = 1; }}>
			<IconVideo size={18} />
			All Videos
			<span class="tab-count">{stats.total}</span>
		</button>
		{#each CONTENT_TYPES as ct}
			<button 
				class="type-tab" 
				class:active={selectedContentType === ct.value}
				style:--type-color={ct.color}
				onclick={() => { selectedContentType = ct.value; currentPage = 1; }}
			>
				{#if ct.value === 'daily_video'}
					<IconVideo size={18} />
				{:else if ct.value === 'weekly_watchlist'}
					<IconListCheck size={18} />
				{:else if ct.value === 'learning_center'}
					<IconSchool size={18} />
				{:else if ct.value === 'room_archive'}
					<IconArchive size={18} />
				{/if}
				{ct.label}
				<span class="tab-count">{stats.by_type[ct.value as keyof typeof stats.by_type] || 0}</span>
			</button>
		{/each}
	</div>

	<!-- Filters Bar -->
	<div class="filters-bar">
		<div class="search-box">
			<IconSearch size={18} />
			<input type="text" placeholder="Search videos..." bind:value={searchQuery} />
		</div>
		<select class="filter-select" bind:value={selectedRoom}>
			<option value="all">All Rooms</option>
			{#each rooms as room}
				<option value={room.id.toString()}>{room.name}</option>
			{/each}
		</select>
		<select class="filter-select" bind:value={selectedTrader}>
			<option value="all">All Traders</option>
			{#each traders as trader}
				<option value={trader.id.toString()}>{trader.name}</option>
			{/each}
		</select>
		<select class="filter-select" bind:value={selectedTag}>
			<option value="all">All Tags</option>
			{#each AVAILABLE_TAGS as tag}
				<option value={tag.slug}>{tag.name}</option>
			{/each}
		</select>
	</div>

	<!-- Bulk Actions Bar -->
	{#if selectedVideoIds.length > 0}
		<div class="bulk-actions-bar">
			<span class="bulk-count">{selectedVideoIds.length} selected</span>
			<button class="bulk-btn" onclick={() => bulkPublish(true)}>
				<IconCheck size={16} /> Publish
			</button>
			<button class="bulk-btn" onclick={() => bulkPublish(false)}>
				<IconX size={16} /> Unpublish
			</button>
			<button class="bulk-btn danger" onclick={bulkDelete}>
				<IconTrash size={16} /> Delete
			</button>
			<button class="bulk-btn secondary" onclick={() => selectedVideoIds = []}>
				Clear Selection
			</button>
		</div>
	{/if}

	<!-- Videos Table -->
	{#if isLoading}
		<div class="loading-state">
			<div class="spinner"></div>
			<p>Loading videos...</p>
		</div>
	{:else if videos.length === 0}
		<div class="empty-state">
			<IconVideo size={64} />
			<h3>No videos found</h3>
			<p>Add your first video or adjust your filters</p>
			<button class="btn-primary" onclick={openUploadModal}>
				<IconPlus size={18} /> Add Video
			</button>
		</div>
	{:else}
		<div class="videos-table-wrapper">
			<table class="videos-table">
				<thead>
					<tr>
						<th class="checkbox-col">
							<button class="checkbox-btn" onclick={toggleSelectAll}>
								{#if selectedVideoIds.length === videos.length}
									<IconSquareCheck size={20} />
								{:else}
									<IconSquare size={20} />
								{/if}
							</button>
						</th>
						<th>Video</th>
						<th>Type</th>
						<th>Rooms</th>
						<th>Tags</th>
						<th>Date</th>
						<th>Views</th>
						<th>Status</th>
						<th>Actions</th>
					</tr>
				</thead>
				<tbody>
					{#each videos as video}
						<tr class:selected={selectedVideoIds.includes(video.id)}>
							<td class="checkbox-col">
								<button class="checkbox-btn" onclick={() => toggleSelectVideo(video.id)}>
									{#if selectedVideoIds.includes(video.id)}
										<IconSquareCheck size={20} />
									{:else}
										<IconSquare size={20} />
									{/if}
								</button>
							</td>
							<td class="video-cell">
								<div class="video-thumbnail">
									{#if video.thumbnail_url}
										<img src={video.thumbnail_url} alt="" />
									{:else}
										<div class="thumbnail-placeholder">
											<IconVideo size={20} />
										</div>
									{/if}
									{#if video.formatted_duration}
										<span class="duration-badge">{video.formatted_duration}</span>
									{/if}
								</div>
								<div class="video-info">
									<span class="video-title">{video.title}</span>
									<span class="video-platform">{video.video_platform}</span>
								</div>
							</td>
							<td>
								<span class="content-type-badge" style:--type-color={getContentTypeColor(video.content_type)}>
									{#if video.content_type === 'daily_video'}
										<IconVideo size={14} />
									{:else if video.content_type === 'weekly_watchlist'}
										<IconListCheck size={14} />
									{:else if video.content_type === 'learning_center'}
										<IconSchool size={14} />
									{:else if video.content_type === 'room_archive'}
										<IconArchive size={14} />
									{/if}
									{getContentTypeLabel(video.content_type)}
								</span>
							</td>
							<td>
								<div class="rooms-list">
									{#each video.rooms.slice(0, 2) as room}
										<span class="room-badge">{room.name}</span>
									{/each}
									{#if video.rooms.length > 2}
										<span class="room-more">+{video.rooms.length - 2}</span>
									{/if}
								</div>
							</td>
							<td>
								<div class="tags-list">
									{#each video.tag_details.slice(0, 2) as tag}
										<span class="tag-badge" style:--tag-color={tag.color}>{tag.name}</span>
									{/each}
									{#if video.tag_details.length > 2}
										<span class="tag-more">+{video.tag_details.length - 2}</span>
									{/if}
								</div>
							</td>
							<td>{formatDate(video.video_date)}</td>
							<td>{formatViews(video.views_count)}</td>
							<td>
								<span class="status-badge" class:published={video.is_published}>
									{video.is_published ? 'Published' : 'Draft'}
								</span>
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

		<!-- Pagination -->
		{#if totalPages > 1}
			<div class="pagination">
				<button disabled={currentPage === 1} onclick={() => { currentPage--; loadVideos(); }}>Previous</button>
				<span>Page {currentPage} of {totalPages}</span>
				<button disabled={currentPage === totalPages} onclick={() => { currentPage++; loadVideos(); }}>Next</button>
			</div>
		{/if}
	{/if}
</div>

<!-- Upload/Edit Modal -->
{#if showUploadModal || showEditModal}
	<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
	<div 
		class="modal-overlay" 
		role="button"
		tabindex="0"
		aria-label="Close modal"
		onclick={closeModals}
		onkeydown={(e) => e.key === 'Escape' && closeModals()}
	>
		<!-- svelte-ignore a11y_no_static_element_interactions -->
		<div class="modal modal-large" role="dialog" aria-modal="true" aria-labelledby="modal-title" tabindex="-1" onclick={(e) => e.stopPropagation()} onkeydown={(e) => e.stopPropagation()}>
			<div class="modal-header">
				<h2 id="modal-title">{showEditModal ? 'Edit Video' : 'Add New Video'}</h2>
				<button class="modal-close" onclick={closeModals}>&times;</button>
			</div>
			<div class="modal-body">
				<!-- Content Type -->
				<div class="form-group">
					<span class="form-label">Content Type</span>
					<div class="content-type-selector">
						{#each CONTENT_TYPES as ct}
						<button
							type="button"
							class="type-option"
							class:selected={formData.content_type === ct.value}
							style:--type-color={ct.color}
							onclick={() => formData.content_type = ct.value}
						>
							{#if ct.value === 'daily_video'}
								<IconVideo size={20} />
							{:else if ct.value === 'weekly_watchlist'}
								<IconListCheck size={20} />
							{:else if ct.value === 'learning_center'}
								<IconSchool size={20} />
							{:else if ct.value === 'room_archive'}
								<IconArchive size={20} />
							{/if}
							{ct.label}
						</button>
					{/each}
					</div>
				</div>

				<!-- Video Source Mode Toggle (only for new videos) -->
				{#if !showEditModal}
					<div class="form-group">
						<span class="form-label">Video Source</span>
						<div class="source-mode-toggle">
							<button
								type="button"
								class="mode-btn"
								class:active={uploadMode === 'upload'}
								onclick={() => uploadMode = 'upload'}
							>
								<IconCloudUpload size={18} />
								Direct Upload
							</button>
							<button
								type="button"
								class="mode-btn"
								class:active={uploadMode === 'url'}
								onclick={() => uploadMode = 'url'}
							>
								<IconLink size={18} />
								Paste URL
							</button>
						</div>
					</div>

					{#if uploadMode === 'upload'}
						<div class="form-group">
							<BunnyVideoUploader
								onUploadComplete={(data) => {
									formData.video_url = data.video_url;
									formData.video_platform = 'bunny';
								}}
								onError={(err) => { error = err; }}
							/>
						</div>
					{:else}
						<div class="form-group">
							<label for="video-url">Video URL</label>
							<input type="url" id="video-url" placeholder="https://..." bind:value={formData.video_url} />
							<span class="form-hint">Supports: Bunny.net, Vimeo, YouTube, Wistia, or direct URL</span>
						</div>
					{/if}
				{:else}
					<!-- Edit mode: just show URL field -->
					<div class="form-group">
						<label for="video-url">Video URL</label>
						<input type="url" id="video-url" placeholder="https://..." bind:value={formData.video_url} />
						<span class="form-hint">Supports: Bunny.net, Vimeo, YouTube, Wistia, or direct URL</span>
					</div>
				{/if}

				<!-- Title -->
				<div class="form-group">
					<label for="video-title">Title</label>
					<input type="text" id="video-title" placeholder="Video title" bind:value={formData.title} />
				</div>

				<!-- Description -->
				<div class="form-group">
					<label for="video-description">Description</label>
					<textarea id="video-description" rows="3" placeholder="Brief description..." bind:value={formData.description}></textarea>
				</div>

				<!-- Room Assignment -->
				<div class="form-group">
					<label>
						<IconBuilding size={16} style="display: inline; vertical-align: middle; margin-right: 4px;" />
						Assign to Rooms
					</label>
					<div class="checkbox-option">
						<label class="checkbox-label highlight">
							<input type="checkbox" bind:checked={formData.upload_to_all} />
							<span>Upload to ALL Rooms</span>
						</label>
					</div>
					{#if !formData.upload_to_all}
						<div class="rooms-grid">
							{#each rooms as room}
								<button
									type="button"
									class="room-option"
									class:selected={formData.room_ids.includes(room.id)}
									onclick={() => toggleRoom(room.id)}
								>
									{#if formData.room_ids.includes(room.id)}
										<IconCheck size={14} />
									{/if}
									{room.name}
								</button>
							{/each}
						</div>
					{/if}
				</div>

				<!-- Tags -->
				<div class="form-group">
					<label>
						<IconTags size={16} style="display: inline; vertical-align: middle; margin-right: 4px;" />
						Tags (select all that apply)
					</label>
					<div class="tags-grid">
						{#each AVAILABLE_TAGS as tag}
							<button
								type="button"
								class="tag-option"
								class:selected={formData.tags.includes(tag.slug)}
								style:--tag-color={tag.color}
								onclick={() => toggleTag(tag.slug)}
							>
								{#if formData.tags.includes(tag.slug)}
									<IconCheck size={14} />
								{/if}
								{tag.name}
							</button>
						{/each}
					</div>
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
				<button class="btn-secondary" onclick={closeModals}>Cancel</button>
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

	.btn-refresh:hover { background: rgba(99, 102, 241, 0.2); color: #818cf8; }
	.btn-refresh :global(.spinning) { animation: spin 1s linear infinite; }

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

	.btn-primary:hover:not(:disabled) { transform: translateY(-2px); box-shadow: 0 4px 15px rgba(99, 102, 241, 0.4); }
	.btn-primary:disabled { opacity: 0.6; cursor: not-allowed; }

	.btn-secondary {
		padding: 0.75rem 1.25rem;
		background: rgba(99, 102, 241, 0.1);
		color: #94a3b8;
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
	}

	.btn-secondary:hover { background: rgba(99, 102, 241, 0.2); color: #e2e8f0; }

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

	.alert-success { background: rgba(34, 197, 94, 0.15); border: 1px solid rgba(34, 197, 94, 0.3); color: #4ade80; }
	.alert-error { background: rgba(239, 68, 68, 0.15); border: 1px solid rgba(239, 68, 68, 0.3); color: #f87171; }
	.alert-close { margin-left: auto; background: transparent; border: none; color: inherit; cursor: pointer; }

	/* Stats Grid */
	.stats-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
		gap: 1rem;
		margin-bottom: 1.5rem;
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
		width: 48px;
		height: 48px;
		display: flex;
		align-items: center;
		justify-content: center;
		border-radius: 12px;
	}

	.stat-info { display: flex; flex-direction: column; }
	.stat-value { font-size: 1.5rem; font-weight: 700; color: #f1f5f9; }
	.stat-label { font-size: 0.8rem; color: #64748b; }

	/* Content Type Tabs */
	.content-type-tabs {
		display: flex;
		gap: 0.5rem;
		margin-bottom: 1.5rem;
		flex-wrap: wrap;
	}

	.type-tab {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1rem;
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 10px;
		color: #94a3b8;
		font-size: 0.9rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.type-tab:hover { background: rgba(99, 102, 241, 0.1); color: #e2e8f0; }
	.type-tab.active { background: var(--type-color, #6366f1); border-color: var(--type-color, #6366f1); color: white; }
	.tab-count { padding: 0.125rem 0.5rem; background: rgba(255, 255, 255, 0.15); border-radius: 12px; font-size: 0.75rem; font-weight: 600; }

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

	.search-box :global(svg) { color: #64748b; }
	.search-box input { flex: 1; padding: 0.75rem 0; background: transparent; border: none; color: #e2e8f0; font-size: 0.9rem; outline: none; }
	.search-box input::placeholder { color: #64748b; }

	.filter-select {
		padding: 0.75rem 1rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 10px;
		color: #e2e8f0;
		font-size: 0.9rem;
		cursor: pointer;
	}

	/* Bulk Actions Bar */
	.bulk-actions-bar {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1rem 1.25rem;
		background: rgba(99, 102, 241, 0.1);
		border: 1px solid rgba(99, 102, 241, 0.3);
		border-radius: 10px;
		margin-bottom: 1.5rem;
	}

	.bulk-count { font-weight: 600; color: #818cf8; }
	.bulk-btn { display: flex; align-items: center; gap: 0.375rem; padding: 0.5rem 1rem; background: rgba(99, 102, 241, 0.2); border: none; border-radius: 6px; color: #e2e8f0; font-size: 0.85rem; cursor: pointer; }
	.bulk-btn:hover { background: rgba(99, 102, 241, 0.3); }
	.bulk-btn.danger { background: rgba(239, 68, 68, 0.2); color: #f87171; }
	.bulk-btn.danger:hover { background: rgba(239, 68, 68, 0.3); }
	.bulk-btn.secondary { background: transparent; color: #94a3b8; }

	/* Table */
	.videos-table-wrapper {
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 14px;
		overflow: hidden;
	}

	.videos-table { width: 100%; border-collapse: collapse; }
	.videos-table th { text-align: left; padding: 1rem 1.25rem; background: rgba(30, 41, 59, 0.6); font-size: 0.8rem; font-weight: 600; color: #64748b; text-transform: uppercase; letter-spacing: 0.05em; border-bottom: 1px solid rgba(99, 102, 241, 0.1); }
	.videos-table td { padding: 1rem 1.25rem; border-bottom: 1px solid rgba(99, 102, 241, 0.05); color: #e2e8f0; }
	.videos-table tr:last-child td { border-bottom: none; }
	.videos-table tr:hover td { background: rgba(99, 102, 241, 0.05); }
	.videos-table tr.selected td { background: rgba(99, 102, 241, 0.1); }

	.checkbox-col { width: 50px; }
	.checkbox-btn { background: transparent; border: none; color: #64748b; cursor: pointer; display: flex; align-items: center; justify-content: center; }
	.checkbox-btn:hover { color: #818cf8; }

	.video-cell { display: flex; align-items: center; gap: 1rem; }
	.video-thumbnail { position: relative; width: 80px; height: 45px; border-radius: 6px; overflow: hidden; background: #1e293b; flex-shrink: 0; }
	.video-thumbnail img { width: 100%; height: 100%; object-fit: cover; }
	.thumbnail-placeholder { width: 100%; height: 100%; display: flex; align-items: center; justify-content: center; color: #475569; }
	.duration-badge { position: absolute; bottom: 2px; right: 2px; padding: 1px 4px; background: rgba(0, 0, 0, 0.8); border-radius: 3px; font-size: 0.65rem; color: white; }
	.video-info { display: flex; flex-direction: column; gap: 0.25rem; }
	.video-title { font-weight: 500; color: #f1f5f9; }
	.video-platform { font-size: 0.75rem; color: #64748b; text-transform: capitalize; }

	.content-type-badge { display: inline-flex; align-items: center; gap: 0.375rem; padding: 0.375rem 0.625rem; background: color-mix(in srgb, var(--type-color) 15%, transparent); border: 1px solid color-mix(in srgb, var(--type-color) 30%, transparent); border-radius: 6px; font-size: 0.75rem; font-weight: 500; color: var(--type-color); white-space: nowrap; }

	.rooms-list, .tags-list { display: flex; flex-wrap: wrap; gap: 0.25rem; }
	.room-badge { padding: 0.25rem 0.5rem; background: rgba(99, 102, 241, 0.15); border-radius: 4px; font-size: 0.7rem; color: #94a3b8; }
	.room-more, .tag-more { padding: 0.25rem 0.5rem; background: rgba(100, 116, 139, 0.15); border-radius: 4px; font-size: 0.7rem; color: #64748b; }
	.tag-badge { padding: 0.25rem 0.5rem; background: color-mix(in srgb, var(--tag-color) 15%, transparent); border: 1px solid color-mix(in srgb, var(--tag-color) 30%, transparent); border-radius: 4px; font-size: 0.7rem; color: var(--tag-color); }

	.status-badge { display: inline-block; padding: 0.375rem 0.75rem; background: rgba(245, 158, 11, 0.15); border: 1px solid rgba(245, 158, 11, 0.3); border-radius: 20px; font-size: 0.75rem; font-weight: 600; color: #fbbf24; }
	.status-badge.published { background: rgba(34, 197, 94, 0.15); border-color: rgba(34, 197, 94, 0.3); color: #4ade80; }

	.action-buttons { display: flex; gap: 0.5rem; }
	.btn-icon { width: 32px; height: 32px; display: flex; align-items: center; justify-content: center; background: rgba(99, 102, 241, 0.1); border: 1px solid rgba(99, 102, 241, 0.2); border-radius: 6px; color: #94a3b8; cursor: pointer; transition: all 0.2s; }
	.btn-icon:hover { background: rgba(99, 102, 241, 0.2); color: #818cf8; }
	.btn-icon.danger:hover { background: rgba(239, 68, 68, 0.1); color: #f87171; border-color: rgba(239, 68, 68, 0.3); }

	/* States */
	.loading-state, .empty-state { display: flex; flex-direction: column; align-items: center; justify-content: center; padding: 4rem 2rem; text-align: center; color: #64748b; }
	.empty-state :global(svg) { color: #475569; margin-bottom: 1rem; }
	.empty-state h3 { color: #e2e8f0; margin: 0 0 0.5rem 0; }
	.empty-state p { margin: 0 0 1.5rem 0; }
	.spinner { width: 40px; height: 40px; border: 3px solid rgba(99, 102, 241, 0.2); border-top-color: #6366f1; border-radius: 50%; animation: spin 1s linear infinite; margin-bottom: 1rem; }

	/* Pagination */
	.pagination { display: flex; align-items: center; justify-content: center; gap: 1rem; margin-top: 1.5rem; }
	.pagination button { padding: 0.5rem 1rem; background: rgba(99, 102, 241, 0.1); border: 1px solid rgba(99, 102, 241, 0.2); border-radius: 8px; color: #94a3b8; cursor: pointer; }
	.pagination button:hover:not(:disabled) { background: rgba(99, 102, 241, 0.2); color: #e2e8f0; }
	.pagination button:disabled { opacity: 0.5; cursor: not-allowed; }
	.pagination span { color: #64748b; }

	/* Modal */
	.modal-overlay { position: fixed; inset: 0; background: rgba(0, 0, 0, 0.7); display: flex; align-items: center; justify-content: center; z-index: 100; padding: 1rem; overflow-y: auto; }
	.modal { background: #1e293b; border: 1px solid rgba(99, 102, 241, 0.2); border-radius: 16px; width: 100%; max-width: 700px; max-height: 90vh; overflow: auto; }
	.modal-large { max-width: 800px; }
	.modal-header { display: flex; justify-content: space-between; align-items: center; padding: 1.25rem; border-bottom: 1px solid rgba(99, 102, 241, 0.1); }
	.modal-header h2 { font-size: 1.25rem; font-weight: 600; color: #f1f5f9; margin: 0; }
	.modal-close { width: 32px; height: 32px; display: flex; align-items: center; justify-content: center; background: transparent; border: none; color: #64748b; font-size: 1.5rem; cursor: pointer; border-radius: 8px; }
	.modal-close:hover { background: rgba(99, 102, 241, 0.1); color: #e2e8f0; }
	.modal-body { padding: 1.25rem; }
	.modal-footer { display: flex; justify-content: flex-end; gap: 0.75rem; padding: 1.25rem; border-top: 1px solid rgba(99, 102, 241, 0.1); }

	/* Form Elements */
	.form-group { margin-bottom: 1rem; }
	.form-group label, .form-label { display: block; font-size: 0.85rem; font-weight: 500; color: #94a3b8; margin-bottom: 0.5rem; }
	.form-group input, .form-group select, .form-group textarea { width: 100%; padding: 0.75rem 1rem; background: rgba(15, 23, 42, 0.6); border: 1px solid rgba(99, 102, 241, 0.2); border-radius: 10px; color: #e2e8f0; font-size: 0.9rem; }
	.form-group input:focus, .form-group select:focus, .form-group textarea:focus { outline: none; border-color: rgba(99, 102, 241, 0.5); }
	.form-hint { display: block; font-size: 0.75rem; color: #64748b; margin-top: 0.375rem; }
	.form-row { display: grid; grid-template-columns: 1fr 1fr; gap: 1rem; }

	.content-type-selector { display: flex; gap: 0.5rem; flex-wrap: wrap; }
	.type-option { display: flex; align-items: center; gap: 0.5rem; padding: 0.75rem 1rem; background: rgba(15, 23, 42, 0.6); border: 1px solid rgba(99, 102, 241, 0.2); border-radius: 10px; color: #94a3b8; font-size: 0.9rem; cursor: pointer; transition: all 0.2s; }
	.type-option:hover { border-color: var(--type-color, rgba(99, 102, 241, 0.5)); color: #e2e8f0; }
	.type-option.selected { background: color-mix(in srgb, var(--type-color) 20%, transparent); border-color: var(--type-color); color: var(--type-color); }

	.source-mode-toggle { display: flex; gap: 0.5rem; }
	.mode-btn { display: flex; align-items: center; gap: 0.5rem; padding: 0.75rem 1.25rem; background: rgba(15, 23, 42, 0.6); border: 1px solid rgba(99, 102, 241, 0.2); border-radius: 10px; color: #94a3b8; font-size: 0.9rem; cursor: pointer; transition: all 0.2s; flex: 1; justify-content: center; }
	.mode-btn:hover { background: rgba(99, 102, 241, 0.1); border-color: rgba(99, 102, 241, 0.4); color: #e2e8f0; }
	.mode-btn.active { background: rgba(99, 102, 241, 0.2); border-color: #6366f1; color: #818cf8; }

	.checkbox-option { margin-bottom: 0.75rem; }
	.checkbox-label { display: flex; align-items: center; gap: 0.5rem; font-size: 0.9rem; color: #94a3b8; cursor: pointer; }
	.checkbox-label.highlight { padding: 0.75rem; background: rgba(99, 102, 241, 0.1); border-radius: 8px; }
	.checkbox-label input { width: 18px; height: 18px; accent-color: #6366f1; }

	.rooms-grid, .tags-grid { display: flex; flex-wrap: wrap; gap: 0.5rem; padding: 0.75rem; background: rgba(15, 23, 42, 0.6); border: 1px solid rgba(99, 102, 241, 0.2); border-radius: 10px; max-height: 200px; overflow-y: auto; }
	.room-option, .tag-option { display: inline-flex; align-items: center; gap: 0.375rem; padding: 0.5rem 0.75rem; background: rgba(100, 116, 139, 0.1); border: 1px solid rgba(100, 116, 139, 0.2); border-radius: 6px; font-size: 0.8rem; font-weight: 500; color: #94a3b8; cursor: pointer; transition: all 0.15s; }
	.room-option:hover { background: rgba(99, 102, 241, 0.15); border-color: rgba(99, 102, 241, 0.3); color: #e2e8f0; }
	.room-option.selected { background: rgba(99, 102, 241, 0.2); border-color: #6366f1; color: #818cf8; }
	.tag-option:hover { background: color-mix(in srgb, var(--tag-color) 15%, transparent); border-color: color-mix(in srgb, var(--tag-color) 30%, transparent); color: var(--tag-color); }
	.tag-option.selected { background: color-mix(in srgb, var(--tag-color) 20%, transparent); border-color: var(--tag-color); color: var(--tag-color); }

	.form-options { display: flex; gap: 1.5rem; padding-top: 0.5rem; }

	.btn-spinner { width: 16px; height: 16px; border: 2px solid rgba(255, 255, 255, 0.3); border-top-color: white; border-radius: 50%; animation: spin 0.8s linear infinite; }

	@media (max-width: 768px) {
		.form-row { grid-template-columns: 1fr; }
		.videos-table-wrapper { overflow-x: auto; }
		.videos-table { min-width: 1000px; }
		.stats-grid { grid-template-columns: repeat(2, 1fr); }
		.content-type-tabs { flex-direction: column; }
	}
</style>

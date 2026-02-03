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

	import { browser } from '$app/environment';
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
		IconBuilding,
		IconTag,
		IconTags,
		IconAlertCircle,
		IconCloudUpload,
		IconProgressCheck
	} from '$lib/icons';
	import {
		tradingRoomApi,
		type TradingRoom,
		type Trader,
		type DailyVideo
	} from '$lib/api/trading-rooms';
	import {
		bulkUploadApi,
		analyticsApi,
		videoOpsApi,
		bulkOpsApi,
		embedApi,
		transcodingApi,
		type AnalyticsDashboard,
		type BatchStatus
	} from '$lib/api/video-advanced';
	import IconCode from '@tabler/icons-svelte-runes/icons/code';
	import IconCheckbox from '@tabler/icons-svelte-runes/icons/checkbox';
	import IconSquare from '@tabler/icons-svelte-runes/icons/square';
	import IconStar from '@tabler/icons-svelte-runes/icons/star';
	import IconStarOff from '@tabler/icons-svelte-runes/icons/star-off';

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
		{ id: 'micro-structure', name: 'Micro Structure', color: '#06b6d4' },
		{ id: 'psychology', name: 'Psychology', color: '#B38F00' },
		{ id: 'technical-analysis', name: 'Technical Analysis', color: '#3b82f6' },
		{ id: 'fundamentals', name: 'Fundamentals', color: '#ec4899' },
		{ id: 'trade-setups', name: 'Trade Setups', color: '#14b8a6' },
		{ id: 'market-review', name: 'Market Review', color: '#E6B800' },
		{ id: 'earnings', name: 'Earnings', color: '#f97316' },
		{ id: 'futures', name: 'Futures', color: '#84cc16' },
		{ id: 'forex', name: 'Forex', color: '#22c55e' },
		{ id: 'crypto', name: 'Crypto', color: '#a855f7' },
		{ id: 'small-accounts', name: 'Small Accounts', color: '#eab308' },
		{ id: 'position-sizing', name: 'Position Sizing', color: '#0ea5e9' },
		{ id: 'entry-exit', name: 'Entry & Exit', color: '#d946ef' },
		{ id: 'scanner-setups', name: 'Scanner Setups', color: '#64748b' },
		{ id: 'indicators', name: 'Indicators', color: '#fb7185' }
	];

	// Helper to get category by ID
	function getCategoryById(id: string): VideoCategory | undefined {
		return availableCategories.find((c) => c.id === id);
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

	// Bunny.net Direct Upload State
	let showBunnyUploadModal = $state(false);
	let bunnyUploadFiles = $state<File[]>([]);
	let bunnyUploadBatchId = $state<string | null>(null);
	let bunnyUploadStatus = $state<BatchStatus | null>(null);
	let isUploadingToBunny = $state(false);
	let bunnyUploadProgress = $state(0);

	// Analytics State
	let showAnalyticsPanel = $state(false);
	let analyticsData = $state<AnalyticsDashboard | null>(null);
	let isLoadingAnalytics = $state(false);
	let analyticsPeriod = $state<'7d' | '30d' | '90d'>('30d');

	// ICT 7 ADDITION: Bulk Operations State
	let selectedVideoIds = $state<Set<number>>(new Set());
	let isBulkActionLoading = $state(false);
	let showBulkTagsModal = $state(false);
	let bulkTagsToAdd = $state<string[]>([]);
	let bulkTagsToRemove = $state<string[]>([]);
	let showEmbedModal = $state(false);
	let embedCodeData = $state<{ video_id: number; title: string; embed_html: string } | null>(null);

	// ═══════════════════════════════════════════════════════════════════════════
	// CATEGORY SELECTION
	// ═══════════════════════════════════════════════════════════════════════════

	function toggleCategory(categoryId: string) {
		const index = formData.categories.indexOf(categoryId);
		if (index === -1) {
			formData.categories = [...formData.categories, categoryId];
		} else {
			formData.categories = formData.categories.filter((c) => c !== categoryId);
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
				videos = response.data.data.map((video) => ({
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
			video_date:
				typeof video.video_date === 'string' ? video.video_date.split('T')[0] : video.video_date,
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
	// BUNNY.NET DIRECT UPLOAD
	// ═══════════════════════════════════════════════════════════════════════════

	function openBunnyUploadModal() {
		bunnyUploadFiles = [];
		bunnyUploadBatchId = null;
		bunnyUploadStatus = null;
		bunnyUploadProgress = 0;
		showBunnyUploadModal = true;
	}

	function handleBunnyFileSelect(event: Event) {
		const input = event.target as HTMLInputElement;
		if (input.files) {
			bunnyUploadFiles = Array.from(input.files).filter(
				(f) =>
					f.type.startsWith('video/') ||
					f.name.endsWith('.mp4') ||
					f.name.endsWith('.mov') ||
					f.name.endsWith('.webm')
			);
		}
	}

	function handleBunnyFileDrop(event: DragEvent) {
		event.preventDefault();
		if (event.dataTransfer?.files) {
			bunnyUploadFiles = Array.from(event.dataTransfer.files).filter(
				(f) =>
					f.type.startsWith('video/') ||
					f.name.endsWith('.mp4') ||
					f.name.endsWith('.mov') ||
					f.name.endsWith('.webm')
			);
		}
	}

	function removeBunnyFile(index: number) {
		bunnyUploadFiles = bunnyUploadFiles.filter((_, i) => i !== index);
	}

	async function startBunnyUpload() {
		if (bunnyUploadFiles.length === 0 || !selectedRoom) return;

		isUploadingToBunny = true;
		error = '';

		try {
			// Initialize bulk upload with Bunny.net
			const response = await bulkUploadApi.init({
				files: bunnyUploadFiles.map((f) => ({
					filename: f.name,
					file_size_bytes: f.size,
					content_type: f.type,
					title: f.name.replace(/\.[^/.]+$/, '').replace(/[-_]/g, ' ')
				})),
				default_metadata: {
					content_type: 'daily_video',
					video_date: new Date().toISOString().split('T')[0],
					room_ids: [selectedRoom.id],
					is_published: false,
					tags: []
				}
			});

			if (response.success && response.data) {
				bunnyUploadBatchId = response.data.batch_id;

				// Upload each file to its respective Bunny upload URL
				for (let i = 0; i < response.data.uploads.length; i++) {
					const uploadItem = response.data.uploads[i];
					const file = bunnyUploadFiles[i];

					try {
						// Upload to Bunny CDN
						await uploadFileToBunny(uploadItem.upload_url, file, uploadItem.id);
						bunnyUploadProgress = Math.round(((i + 1) / response.data.uploads.length) * 100);
					} catch (uploadErr) {
						console.error(`Failed to upload ${file.name}:`, uploadErr);
						await bulkUploadApi.updateItemStatus(uploadItem.id, {
							status: 'failed',
							error_message: uploadErr instanceof Error ? uploadErr.message : 'Upload failed'
						});
					}
				}

				// Poll for batch status
				await pollBatchStatus(response.data.batch_id);
				showSuccess(`Successfully uploaded ${bunnyUploadFiles.length} video(s) to Bunny.net`);
				await loadVideos();
			}
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to initialize Bunny upload';
			console.error('Bunny upload error:', err);
		} finally {
			isUploadingToBunny = false;
		}
	}

	async function uploadFileToBunny(uploadUrl: string, file: File, itemId: number): Promise<void> {
		return new Promise((resolve, reject) => {
			const xhr = new XMLHttpRequest();

			xhr.upload.addEventListener('progress', async (e) => {
				if (e.lengthComputable) {
					const progress = Math.round((e.loaded / e.total) * 100);
					await bulkUploadApi.updateItemStatus(itemId, {
						status: 'uploading',
						progress_percent: progress
					});
				}
			});

			xhr.addEventListener('load', async () => {
				if (xhr.status >= 200 && xhr.status < 300) {
					await bulkUploadApi.updateItemStatus(itemId, {
						status: 'completed',
						progress_percent: 100
					});
					resolve();
				} else {
					reject(new Error(`Upload failed with status ${xhr.status}`));
				}
			});

			xhr.addEventListener('error', () => reject(new Error('Network error during upload')));
			xhr.addEventListener('abort', () => reject(new Error('Upload aborted')));

			xhr.open('PUT', uploadUrl);
			xhr.setRequestHeader('Content-Type', file.type || 'video/mp4');
			xhr.send(file);
		});
	}

	async function pollBatchStatus(batchId: string): Promise<void> {
		let attempts = 0;
		const maxAttempts = 60; // 5 minutes with 5-second intervals

		while (attempts < maxAttempts) {
			const response = await bulkUploadApi.getBatchStatus(batchId);
			if (response.success && response.data) {
				bunnyUploadStatus = response.data;

				if (response.data.pending === 0 && response.data.in_progress === 0) {
					return; // All uploads complete
				}
			}

			await new Promise((resolve) => setTimeout(resolve, 5000));
			attempts++;
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// VIDEO ANALYTICS
	// ═══════════════════════════════════════════════════════════════════════════

	async function loadAnalytics() {
		isLoadingAnalytics = true;
		error = '';

		try {
			const response = await analyticsApi.getDashboard({
				period: analyticsPeriod,
				room_id: selectedRoom?.id
			});

			if (response.success && response.data) {
				analyticsData = response.data;
			}
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load analytics';
			console.error('Analytics error:', err);
		} finally {
			isLoadingAnalytics = false;
		}
	}

	function toggleAnalyticsPanel() {
		showAnalyticsPanel = !showAnalyticsPanel;
		if (showAnalyticsPanel && !analyticsData) {
			loadAnalytics();
		}
	}

	async function fetchVideoDuration(videoId: number) {
		try {
			const response = await videoOpsApi.fetchDuration(videoId);
			if (response.success && response.data) {
				showSuccess(`Duration fetched: ${response.data.formatted_duration}`);
				await loadVideos();
			}
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to fetch duration';
		}
	}

	// Reload analytics when period changes
	$effect(() => {
		if (showAnalyticsPanel && analyticsPeriod) {
			loadAnalytics();
		}
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// ICT 7 ADDITION: BULK OPERATIONS
	// ═══════════════════════════════════════════════════════════════════════════

	function toggleVideoSelection(videoId: number) {
		const newSet = new Set(selectedVideoIds);
		if (newSet.has(videoId)) {
			newSet.delete(videoId);
		} else {
			newSet.add(videoId);
		}
		selectedVideoIds = newSet;
	}

	function toggleSelectAll() {
		if (selectedVideoIds.size === filteredVideos.length) {
			selectedVideoIds = new Set();
		} else {
			selectedVideoIds = new Set(filteredVideos.map((v) => v.id));
		}
	}

	function clearSelection() {
		selectedVideoIds = new Set();
	}

	async function bulkPublish(publish: boolean) {
		if (selectedVideoIds.size === 0) return;
		isBulkActionLoading = true;
		error = '';

		try {
			const response = await bulkOpsApi.bulkPublish(Array.from(selectedVideoIds), publish);
			if (response.success) {
				showSuccess(response.message || `${selectedVideoIds.size} videos ${publish ? 'published' : 'unpublished'}`);
				clearSelection();
				await loadVideos();
			} else {
				error = response.error || 'Failed to update videos';
			}
		} catch (err) {
			error = err instanceof Error ? err.message : 'Bulk operation failed';
		} finally {
			isBulkActionLoading = false;
		}
	}

	async function bulkFeature(feature: boolean) {
		if (selectedVideoIds.size === 0) return;
		isBulkActionLoading = true;
		error = '';

		try {
			const response = await bulkOpsApi.bulkFeature(Array.from(selectedVideoIds), feature);
			if (response.success) {
				showSuccess(response.message || `${selectedVideoIds.size} videos ${feature ? 'featured' : 'unfeatured'}`);
				clearSelection();
				await loadVideos();
			} else {
				error = response.error || 'Failed to update videos';
			}
		} catch (err) {
			error = err instanceof Error ? err.message : 'Bulk operation failed';
		} finally {
			isBulkActionLoading = false;
		}
	}

	async function bulkDelete() {
		if (selectedVideoIds.size === 0) return;
		if (!confirm(`Are you sure you want to delete ${selectedVideoIds.size} video(s)? This cannot be undone.`)) return;

		isBulkActionLoading = true;
		error = '';

		try {
			const response = await bulkOpsApi.bulkDelete(Array.from(selectedVideoIds), false);
			if (response.success) {
				showSuccess(response.message || `${selectedVideoIds.size} videos deleted`);
				clearSelection();
				await loadVideos();
			} else {
				error = response.error || 'Failed to delete videos';
			}
		} catch (err) {
			error = err instanceof Error ? err.message : 'Bulk delete failed';
		} finally {
			isBulkActionLoading = false;
		}
	}

	async function bulkUpdateTags() {
		if (selectedVideoIds.size === 0) return;
		if (bulkTagsToAdd.length === 0 && bulkTagsToRemove.length === 0) {
			error = 'Please select tags to add or remove';
			return;
		}

		isBulkActionLoading = true;
		error = '';

		try {
			const response = await bulkOpsApi.bulkUpdateTags({
				video_ids: Array.from(selectedVideoIds),
				add_tags: bulkTagsToAdd.length > 0 ? bulkTagsToAdd : undefined,
				remove_tags: bulkTagsToRemove.length > 0 ? bulkTagsToRemove : undefined
			});

			if (response.success) {
				showSuccess(response.message || `Tags updated for ${response.updated_count} videos`);
				closeBulkTagsModal();
				clearSelection();
				await loadVideos();
			} else {
				error = response.error || 'Failed to update tags';
			}
		} catch (err) {
			error = err instanceof Error ? err.message : 'Bulk tag update failed';
		} finally {
			isBulkActionLoading = false;
		}
	}

	function openBulkTagsModal() {
		bulkTagsToAdd = [];
		bulkTagsToRemove = [];
		showBulkTagsModal = true;
	}

	function closeBulkTagsModal() {
		showBulkTagsModal = false;
		bulkTagsToAdd = [];
		bulkTagsToRemove = [];
	}

	function toggleBulkTagToAdd(tagId: string) {
		if (bulkTagsToAdd.includes(tagId)) {
			bulkTagsToAdd = bulkTagsToAdd.filter((t) => t !== tagId);
		} else {
			bulkTagsToAdd = [...bulkTagsToAdd, tagId];
			// Remove from remove list if present
			bulkTagsToRemove = bulkTagsToRemove.filter((t) => t !== tagId);
		}
	}

	function toggleBulkTagToRemove(tagId: string) {
		if (bulkTagsToRemove.includes(tagId)) {
			bulkTagsToRemove = bulkTagsToRemove.filter((t) => t !== tagId);
		} else {
			bulkTagsToRemove = [...bulkTagsToRemove, tagId];
			// Remove from add list if present
			bulkTagsToAdd = bulkTagsToAdd.filter((t) => t !== tagId);
		}
	}

	async function showEmbedCode(video: Video) {
		try {
			const response = await embedApi.getEmbedCode(video.id, { responsive: true });
			if (response.success && response.data) {
				embedCodeData = {
					video_id: video.id,
					title: response.data.title,
					embed_html: response.data.embed_html
				};
				showEmbedModal = true;
			} else {
				error = response.error || 'Failed to get embed code';
			}
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to get embed code';
		}
	}

	function copyEmbedCode() {
		if (embedCodeData?.embed_html) {
			navigator.clipboard.writeText(embedCodeData.embed_html);
			showSuccess('Embed code copied to clipboard');
		}
	}

	// Clear selection when videos change
	$effect(() => {
		if (videos.length) {
			// Keep only valid selections
			const validIds = new Set(videos.map((v) => v.id));
			const newSelection = new Set<number>();
			for (const id of selectedVideoIds) {
				if (validIds.has(id)) {
					newSelection.add(id);
				}
			}
			if (newSelection.size !== selectedVideoIds.size) {
				selectedVideoIds = newSelection;
			}
		}
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// HELPERS
	// ═══════════════════════════════════════════════════════════════════════════

	function formatDuration(seconds?: number): string {
		if (!seconds) return '';
		const h = Math.floor(seconds / 3600);
		const m = Math.floor((seconds % 3600) / 60);
		const s = seconds % 60;
		return h > 0
			? `${h}:${m.toString().padStart(2, '0')}:${s.toString().padStart(2, '0')}`
			: `${m}:${s.toString().padStart(2, '0')}`;
	}

	function formatDate(dateString: string): string {
		return new Date(dateString).toLocaleDateString('en-US', {
			month: 'short',
			day: 'numeric',
			year: 'numeric'
		});
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

	const tradingRooms = $derived(rooms.filter((r) => r.type === 'trading_room'));
	const alertServices = $derived(rooms.filter((r) => r.type === 'alert_service'));

	// Filter videos by search, trader, and category
	const filteredVideos = $derived(
		videos.filter((video) => {
			// Search matches title, description, or category names
			const searchLower = searchQuery.toLowerCase();
			const categoryNames = video.categories.map(
				(c) => getCategoryById(c)?.name.toLowerCase() || ''
			);
			const matchesSearch =
				!searchQuery ||
				video.title.toLowerCase().includes(searchLower) ||
				video.description?.toLowerCase().includes(searchLower) ||
				categoryNames.some((name) => name.includes(searchLower));

			// Trader filter
			const matchesTrader =
				selectedTrader === 'all' || video.trader_id?.toString() === selectedTrader;

			// Category filter
			const matchesCategory =
				selectedCategory === 'all' || video.categories.includes(selectedCategory);

			return matchesSearch && matchesTrader && matchesCategory;
		})
	);

	const totalViews = $derived(videos.reduce((sum, v) => sum + v.views_count, 0));
	const publishedCount = $derived(videos.filter((v) => v.is_published).length);

	// Get unique categories used in current videos
	const usedCategories = $derived.by(() => {
		const categoryIds = new Set<string>();
		videos.forEach((v) => v.categories.forEach((c) => categoryIds.add(c)));
		return Array.from(categoryIds)
			.map((id) => getCategoryById(id))
			.filter(Boolean) as VideoCategory[];
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// LIFECYCLE
	// ═══════════════════════════════════════════════════════════════════════════

	// Svelte 5: Initialize on mount
	$effect(() => {
		if (!browser) return;

		const init = async () => {
			// Load rooms and traders first
			await loadRoomsAndTraders();
			// Then load videos for selected room
			if (selectedRoom) {
				await loadVideos();
			}
		};
		init();
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

<div class="admin-videos">
	<div class="admin-page-container">
		<!-- Animated Background -->
		<div class="bg-effects">
			<div class="bg-blob bg-blob-1"></div>
			<div class="bg-blob bg-blob-2"></div>
			<div class="bg-blob bg-blob-3"></div>
		</div>

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
				<button class="alert-close" onclick={() => (error = '')}>
					<IconX size={16} />
				</button>
			</div>
		{/if}

		<!-- Header -->
		<div class="page-header">
			<h1>Trading Room Videos</h1>
			<p class="subtitle">Manage daily videos with category tags for each room and service</p>
			<div class="header-actions">
				<button class="btn-secondary" onclick={toggleAnalyticsPanel} title="View Analytics">
					<IconChartBar size={18} />
					Analytics
				</button>
				<button
					class="btn-bunny"
					onclick={openBunnyUploadModal}
					disabled={!selectedRoom}
					title="Upload to Bunny.net"
				>
					<IconCloudUpload size={18} />
					Bunny Upload
				</button>
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
					<span class="room-type-badge"
						>{selectedRoom.type === 'trading_room' ? 'Trading Room' : 'Alert Service'}</span
					>
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

		<!-- Analytics Panel -->
		{#if showAnalyticsPanel}
			<div class="analytics-panel">
				<div class="analytics-header">
					<h3><IconChartBar size={20} /> Video Analytics</h3>
					<div class="analytics-controls">
						<label for="analytics-period" class="sr-only">Analytics period</label>
						<select id="analytics-period" class="filter-select" bind:value={analyticsPeriod}>
							<option value="7d">Last 7 days</option>
							<option value="30d">Last 30 days</option>
							<option value="90d">Last 90 days</option>
						</select>
						<button class="btn-icon" onclick={() => (showAnalyticsPanel = false)} title="Close">
							<IconX size={18} />
						</button>
					</div>
				</div>

				{#if isLoadingAnalytics}
					<div class="analytics-loading">
						<div class="spinner"></div>
						<p>Loading analytics...</p>
					</div>
				{:else if analyticsData}
					<div class="analytics-grid">
						<div class="analytics-stat">
							<span class="stat-value">{formatViews(analyticsData.total_views)}</span>
							<span class="stat-label">Total Views</span>
						</div>
						<div class="analytics-stat">
							<span class="stat-value">{analyticsData.unique_viewers.toLocaleString()}</span>
							<span class="stat-label">Unique Viewers</span>
						</div>
						<div class="analytics-stat">
							<span class="stat-value">{analyticsData.total_watch_time_hours.toFixed(1)}h</span>
							<span class="stat-label">Watch Time</span>
						</div>
						<div class="analytics-stat">
							<span class="stat-value">{(analyticsData.avg_completion_rate * 100).toFixed(0)}%</span
							>
							<span class="stat-label">Avg Completion</span>
						</div>
					</div>

					{#if analyticsData.top_videos && analyticsData.top_videos.length > 0}
						<div class="top-videos-section">
							<h4>Top Performing Videos</h4>
							<div class="top-videos-list">
								{#each analyticsData.top_videos.slice(0, 5) as topVideo, index}
									<div class="top-video-item">
										<span class="rank">#{index + 1}</span>
										<span class="title">{topVideo.title}</span>
										<span class="views">{formatViews(topVideo.views)} views</span>
									</div>
								{/each}
							</div>
						</div>
					{/if}

					<!-- Used Categories Quick Stats -->
					{#if usedCategories.length > 0}
						<div class="categories-stats-section">
							<h4>Categories in Use</h4>
							<div class="used-categories-tags">
								{#each usedCategories as category}
									<span class="category-tag" style:--tag-color={category.color}>
										{category.name}
									</span>
								{/each}
							</div>
						</div>
					{/if}
				{:else}
					<div class="analytics-empty">
						<p>No analytics data available yet.</p>
					</div>
				{/if}
			</div>
		{/if}

		<!-- Filters -->
		<div class="filters-bar">
			<div class="search-box">
				<IconSearch size={18} />
				<label for="search-videos" class="sr-only">Search videos or categories</label>
				<input
					type="text"
					id="search-videos" name="search-videos"
					placeholder="Search videos, categories..."
					bind:value={searchQuery}
				/>
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
			<!-- ICT 7 ADDITION: Bulk Action Bar -->
			{#if selectedVideoIds.size > 0}
				<div class="bulk-action-bar">
					<div class="bulk-selection-info">
						<button class="btn-icon" onclick={clearSelection} title="Clear selection">
							<IconX size={16} />
						</button>
						<span>{selectedVideoIds.size} video(s) selected</span>
					</div>
					<div class="bulk-actions">
						<button
							class="btn-bulk"
							onclick={() => bulkPublish(true)}
							disabled={isBulkActionLoading}
							title="Publish selected"
						>
							<IconCheck size={16} />
							Publish
						</button>
						<button
							class="btn-bulk"
							onclick={() => bulkPublish(false)}
							disabled={isBulkActionLoading}
							title="Unpublish selected"
						>
							<IconX size={16} />
							Unpublish
						</button>
						<button
							class="btn-bulk"
							onclick={() => bulkFeature(true)}
							disabled={isBulkActionLoading}
							title="Feature selected"
						>
							<IconStar size={16} />
							Feature
						</button>
						<button
							class="btn-bulk"
							onclick={() => bulkFeature(false)}
							disabled={isBulkActionLoading}
							title="Unfeature selected"
						>
							<IconStarOff size={16} />
							Unfeature
						</button>
						<button
							class="btn-bulk"
							onclick={openBulkTagsModal}
							disabled={isBulkActionLoading}
							title="Manage tags"
						>
							<IconTags size={16} />
							Tags
						</button>
						<button
							class="btn-bulk danger"
							onclick={bulkDelete}
							disabled={isBulkActionLoading}
							title="Delete selected"
						>
							<IconTrash size={16} />
							Delete
						</button>
						{#if isBulkActionLoading}
							<span class="bulk-spinner"></span>
						{/if}
					</div>
				</div>
			{/if}

			<div class="videos-table-wrapper">
				<table class="videos-table">
					<thead>
						<tr>
							<th class="checkbox-col">
								<button
									class="select-all-btn"
									onclick={toggleSelectAll}
									title={selectedVideoIds.size === filteredVideos.length ? 'Deselect all' : 'Select all'}
								>
									{#if selectedVideoIds.size === filteredVideos.length && filteredVideos.length > 0}
										<IconCheckbox size={18} />
									{:else if selectedVideoIds.size > 0}
										<IconSquare size={18} class="partial" />
									{:else}
										<IconSquare size={18} />
									{/if}
								</button>
							</th>
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
							<tr class:selected={selectedVideoIds.has(video.id)}>
								<td class="checkbox-col">
									<button
										class="select-row-btn"
										onclick={() => toggleVideoSelection(video.id)}
										title={selectedVideoIds.has(video.id) ? 'Deselect' : 'Select'}
									>
										{#if selectedVideoIds.has(video.id)}
											<IconCheckbox size={18} />
										{:else}
											<IconSquare size={18} />
										{/if}
									</button>
								</td>
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
										{#each (video.categories || []).slice(0, 3) as categoryId}
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
										<button
											class="btn-icon"
											title="Embed Code"
											onclick={() => showEmbedCode(video)}
										>
											<IconCode size={16} />
										</button>
										<button
											class="btn-icon"
											title="Replace Video"
											onclick={() => openReplaceModal(video)}
										>
											<IconLink size={16} />
										</button>
										<button class="btn-icon" title="Edit" onclick={() => openEditModal(video)}>
											<IconEdit size={16} />
										</button>
										<button class="btn-icon" title="Preview">
											<IconPlayerPlay size={16} />
										</button>
										<button
											class="btn-icon danger"
											title="Delete"
											onclick={() => deleteVideo(video)}
										>
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
	<!-- End admin-page-container -->
</div>

<!-- Upload/Edit Modal -->
{#if showUploadModal || showEditModal}
	<div
		class="modal-overlay"
		role="button"
		tabindex="0"
		onclick={() => {
			showUploadModal = false;
			showEditModal = false;
		}}
		onkeydown={(e: KeyboardEvent) =>
			e.key === 'Escape' && ((showUploadModal = false), (showEditModal = false))}
	>
		<div
			class="modal modal-large"
			onclick={(e: MouseEvent) => e.stopPropagation()}
			onkeydown={(e: KeyboardEvent) => e.stopPropagation()}
			role="dialog"
			aria-modal="true"
			tabindex="-1"
		>
			<div class="modal-header">
				<h2>{showEditModal ? 'Edit Video' : 'Add New Video'}</h2>
				<button
					class="modal-close"
					onclick={() => {
						showUploadModal = false;
						showEditModal = false;
					}}
					type="button"
					aria-label="Close">&times;</button
				>
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
					<input
						type="url"
						id="video-url" name="video-url"
						placeholder="https://vimeo.com/..."
						bind:value={formData.video_url}
					/>
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
					<input
						type="text"
						id="video-title" name="video-title"
						placeholder="Risk Management Fundamentals"
						bind:value={formData.title}
					/>
				</div>

				<!-- Description -->
				<div class="form-group">
					<label for="video-description">Description</label>
					<textarea
						id="video-description"
						rows="3"
						placeholder="Brief description of the video content..."
						bind:value={formData.description}
					></textarea>
				</div>

				<!-- Categories Section -->
				<div class="form-group">
					<label>
						<IconTags
							size={16}
							style="display: inline; vertical-align: middle; margin-right: 4px;"
						/>
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
										<button
											type="button"
											onclick={() => toggleCategory(categoryId)}
											aria-label="Remove {category.name}"
										>
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
						<input type="date" id="video-date" name="video-date" bind:value={formData.video_date} />
					</div>
				</div>

				<!-- Thumbnail URL -->
				<div class="form-group">
					<label for="thumbnail-url">Thumbnail URL (optional)</label>
					<input
						type="url"
						id="thumbnail-url" name="thumbnail-url"
						placeholder="https://..."
						bind:value={formData.thumbnail_url}
					/>
				</div>

				<!-- Options -->
				<div class="form-options">
					<label class="checkbox-label">
						<input id="videos-formdata-is-published" name="videos-formdata-is-published" type="checkbox" bind:checked={formData.is_published} />
						<span>Publish immediately</span>
					</label>
					<label class="checkbox-label">
						<input id="videos-formdata-is-featured" name="videos-formdata-is-featured" type="checkbox" bind:checked={formData.is_featured} />
						<span>Feature this video</span>
					</label>
				</div>
			</div>
			<div class="modal-footer">
				<button
					class="btn-secondary"
					onclick={() => {
						showUploadModal = false;
						showEditModal = false;
					}}
					type="button">Cancel</button
				>
				<button
					class="btn-primary"
					onclick={saveVideo}
					disabled={isSaving || !formData.title || !formData.video_url}
				>
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
		onclick={() => {
			showReplaceModal = false;
			replacingVideo = null;
			newVideoUrl = '';
		}}
		onkeydown={(e: KeyboardEvent) =>
			e.key === 'Escape' &&
			((showReplaceModal = false), (replacingVideo = null), (newVideoUrl = ''))}
	>
		<div
			class="modal"
			onclick={(e: MouseEvent) => e.stopPropagation()}
			onkeydown={(e: KeyboardEvent) => e.stopPropagation()}
			role="dialog"
			aria-modal="true"
			tabindex="-1"
		>
			<div class="modal-header">
				<h2>Replace Video</h2>
				<button
					class="modal-close"
					onclick={() => {
						showReplaceModal = false;
						replacingVideo = null;
						newVideoUrl = '';
					}}
					type="button"
					aria-label="Close">&times;</button
				>
			</div>
			<div class="modal-body">
				<div class="replace-info">
					<p><strong>Current video:</strong> {replacingVideo.title}</p>
					<p class="text-muted">
						All metadata (title, description, categories, featured status) will be kept. Only the
						video URL will be replaced.
					</p>
				</div>

				<div class="form-group">
					<label for="new-video-url">New Video URL</label>
					<input
						type="url"
						id="new-video-url" name="new-video-url"
						placeholder="https://your-video-url.com/video.mp4"
						bind:value={newVideoUrl}
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
				<button
					class="btn-secondary"
					onclick={() => {
						showReplaceModal = false;
						replacingVideo = null;
						newVideoUrl = '';
					}}
					type="button">Cancel</button
				>
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

<!-- Bunny.net Direct Upload Modal -->
{#if showBunnyUploadModal}
	<div
		class="modal-overlay"
		role="button"
		tabindex="0"
		onclick={() => {
			if (!isUploadingToBunny) {
				showBunnyUploadModal = false;
				bunnyUploadFiles = [];
			}
		}}
		onkeydown={(e: KeyboardEvent) =>
			e.key === 'Escape' &&
			!isUploadingToBunny &&
			((showBunnyUploadModal = false), (bunnyUploadFiles = []))}
	>
		<div
			class="modal modal-large"
			onclick={(e: MouseEvent) => e.stopPropagation()}
			onkeydown={(e: KeyboardEvent) => e.stopPropagation()}
			role="dialog"
			aria-modal="true"
			tabindex="-1"
		>
			<div class="modal-header bunny-header">
				<h2>
					<IconCloudUpload size={24} />
					Upload to Bunny.net
				</h2>
				<button
					class="modal-close"
					onclick={() => {
						if (!isUploadingToBunny) {
							showBunnyUploadModal = false;
							bunnyUploadFiles = [];
						}
					}}
					type="button"
					aria-label="Close"
					disabled={isUploadingToBunny}>&times;</button
				>
			</div>
			<div class="modal-body">
				<div class="bunny-info">
					<p>Upload video files directly to Bunny.net CDN for fast, reliable streaming.</p>
					<p class="text-muted">
						Videos will be uploaded to: <strong>{selectedRoom?.name}</strong>
					</p>
				</div>

				<!-- Drop Zone -->
				<div
					class="bunny-dropzone"
					class:has-files={bunnyUploadFiles.length > 0}
					ondragover={(e: DragEvent) => e.preventDefault()}
					ondrop={handleBunnyFileDrop}
					role="button"
					tabindex="0"
					aria-label="Drop video files here or click to select"
				>
					{#if bunnyUploadFiles.length === 0}
						<IconFileUpload size={48} />
						<p>Drag and drop video files here</p>
						<p class="text-muted">or</p>
						<label class="btn-secondary">
							Browse Files
							<input
								type="file"
								accept="video/*,.mp4,.mov,.webm"
								multiple
								onchange={handleBunnyFileSelect}
								hidden
							/>
						</label>
						<p class="file-types">Supported: MP4, MOV, WebM</p>
					{:else}
						<div class="selected-files">
							<h4><IconProgressCheck size={20} /> {bunnyUploadFiles.length} file(s) selected</h4>
							<ul class="file-list">
								{#each bunnyUploadFiles as file, index}
									<li>
										<span class="file-name">{file.name}</span>
										<span class="file-size">{(file.size / (1024 * 1024)).toFixed(1)} MB</span>
										{#if !isUploadingToBunny}
											<button
												class="btn-remove"
												onclick={() => removeBunnyFile(index)}
												aria-label="Remove {file.name}"
											>
												<IconX size={14} />
											</button>
										{/if}
									</li>
								{/each}
							</ul>
						</div>
					{/if}
				</div>

				<!-- Upload Progress -->
				{#if isUploadingToBunny}
					<div class="upload-progress">
						<div class="progress-bar">
							<div class="progress-fill" style:width="{bunnyUploadProgress}%"></div>
						</div>
						<p class="progress-text">Uploading... {bunnyUploadProgress}%</p>

						{#if bunnyUploadStatus}
							<div class="batch-status">
								<span class="status-item completed">Completed: {bunnyUploadStatus.completed}</span>
								<span class="status-item in-progress"
									>In Progress: {bunnyUploadStatus.in_progress}</span
								>
								<span class="status-item pending">Pending: {bunnyUploadStatus.pending}</span>
								{#if bunnyUploadStatus.failed > 0}
									<span class="status-item failed">Failed: {bunnyUploadStatus.failed}</span>
								{/if}
							</div>
						{/if}
					</div>
				{/if}
			</div>
			<div class="modal-footer">
				<button
					class="btn-secondary"
					onclick={() => {
						showBunnyUploadModal = false;
						bunnyUploadFiles = [];
					}}
					type="button"
					disabled={isUploadingToBunny}>Cancel</button
				>
				<button
					class="btn-bunny"
					onclick={startBunnyUpload}
					disabled={isUploadingToBunny || bunnyUploadFiles.length === 0}
				>
					{#if isUploadingToBunny}
						<span class="btn-spinner"></span>
						Uploading...
					{:else}
						<IconCloudUpload size={16} />
						Upload to Bunny.net
					{/if}
				</button>
			</div>
		</div>
	</div>
{/if}

<!-- ICT 7 ADDITION: Bulk Tags Modal -->
{#if showBulkTagsModal}
	<div
		class="modal-overlay"
		role="button"
		tabindex="0"
		onclick={closeBulkTagsModal}
		onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && closeBulkTagsModal()}
	>
		<div
			class="modal modal-large"
			onclick={(e: MouseEvent) => e.stopPropagation()}
			onkeydown={(e: KeyboardEvent) => e.stopPropagation()}
			role="dialog"
			aria-modal="true"
			tabindex="-1"
		>
			<div class="modal-header">
				<h2>
					<IconTags size={24} />
					Bulk Update Tags
				</h2>
				<button
					class="modal-close"
					onclick={closeBulkTagsModal}
					type="button"
					aria-label="Close"
				>&times;</button>
			</div>
			<div class="modal-body">
				<p class="bulk-info">Update tags for <strong>{selectedVideoIds.size}</strong> selected video(s)</p>

				<!-- Tags to Add -->
				<div class="form-group">
					<label>
						<IconPlus size={16} style="display:inline;vertical-align:middle;color:#22c55e;margin-right:4px;" />
						Tags to Add
					</label>
					<div class="bulk-tags-grid">
						{#each availableCategories as category}
							<button
								type="button"
								class="bulk-tag-btn add"
								class:selected={bulkTagsToAdd.includes(category.id)}
								style:--tag-color={category.color}
								onclick={() => toggleBulkTagToAdd(category.id)}
							>
								{#if bulkTagsToAdd.includes(category.id)}
									<IconCheck size={14} />
								{/if}
								{category.name}
							</button>
						{/each}
					</div>
				</div>

				<!-- Tags to Remove -->
				<div class="form-group">
					<label>
						<IconX size={16} style="display:inline;vertical-align:middle;color:#ef4444;margin-right:4px;" />
						Tags to Remove
					</label>
					<div class="bulk-tags-grid">
						{#each availableCategories as category}
							<button
								type="button"
								class="bulk-tag-btn remove"
								class:selected={bulkTagsToRemove.includes(category.id)}
								style:--tag-color={category.color}
								onclick={() => toggleBulkTagToRemove(category.id)}
							>
								{#if bulkTagsToRemove.includes(category.id)}
									<IconTrash size={14} />
								{/if}
								{category.name}
							</button>
						{/each}
					</div>
				</div>

				{#if bulkTagsToAdd.length > 0 || bulkTagsToRemove.length > 0}
					<div class="bulk-tags-summary">
						{#if bulkTagsToAdd.length > 0}
							<p><strong>Adding:</strong> {bulkTagsToAdd.map(id => getCategoryById(id)?.name).join(', ')}</p>
						{/if}
						{#if bulkTagsToRemove.length > 0}
							<p><strong>Removing:</strong> {bulkTagsToRemove.map(id => getCategoryById(id)?.name).join(', ')}</p>
						{/if}
					</div>
				{/if}
			</div>
			<div class="modal-footer">
				<button class="btn-secondary" onclick={closeBulkTagsModal} type="button">Cancel</button>
				<button
					class="btn-primary"
					onclick={bulkUpdateTags}
					disabled={isBulkActionLoading || (bulkTagsToAdd.length === 0 && bulkTagsToRemove.length === 0)}
				>
					{#if isBulkActionLoading}
						<span class="btn-spinner"></span>
						Updating...
					{:else}
						<IconTags size={16} />
						Update Tags
					{/if}
				</button>
			</div>
		</div>
	</div>
{/if}

<!-- ICT 7 ADDITION: Embed Code Modal -->
{#if showEmbedModal && embedCodeData}
	<div
		class="modal-overlay"
		role="button"
		tabindex="0"
		onclick={() => { showEmbedModal = false; embedCodeData = null; }}
		onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && (showEmbedModal = false, embedCodeData = null)}
	>
		<div
			class="modal"
			onclick={(e: MouseEvent) => e.stopPropagation()}
			onkeydown={(e: KeyboardEvent) => e.stopPropagation()}
			role="dialog"
			aria-modal="true"
			tabindex="-1"
		>
			<div class="modal-header">
				<h2>
					<IconCode size={24} />
					Embed Code
				</h2>
				<button
					class="modal-close"
					onclick={() => { showEmbedModal = false; embedCodeData = null; }}
					type="button"
					aria-label="Close"
				>&times;</button>
			</div>
			<div class="modal-body">
				<p class="embed-title"><strong>{embedCodeData.title}</strong></p>
				<div class="embed-code-box">
					<pre><code>{embedCodeData.embed_html}</code></pre>
				</div>
				<p class="embed-hint">Copy this code and paste it into your website to embed this video.</p>
			</div>
			<div class="modal-footer">
				<button class="btn-secondary" onclick={() => { showEmbedModal = false; embedCodeData = null; }} type="button">Close</button>
				<button class="btn-primary" onclick={copyEmbedCode}>
					<IconCode size={16} />
					Copy to Clipboard
				</button>
			</div>
		</div>
	</div>
{/if}

<style>
	.admin-videos {
		max-width: 1400px;
		margin: 0 auto;
		padding: 2rem;
	}

	/* Header - Centered */
	.page-header {
		text-align: center;
		margin-bottom: 2rem;
	}

	.page-header h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: var(--text-primary);
		margin: 0 0 0.25rem 0;
	}

	.subtitle {
		color: var(--text-tertiary);
		font-size: 0.875rem;
		margin: 0.25rem 0 1rem 0;
	}

	.header-actions {
		display: flex;
		justify-content: center;
		gap: 0.75rem;
	}

	.btn-refresh {
		width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(22, 27, 34, 0.8);
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 10px;
		color: var(--text-secondary);
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-refresh:hover {
		background: rgba(230, 184, 0, 0.2);
		color: var(--primary-500);
	}

	.btn-refresh :global(.spinning) {
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

	.btn-primary {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background: linear-gradient(135deg, var(--primary-500) 0%, var(--primary-600) 100%);
		color: var(--bg-base);
		border: none;
		border-radius: 6px;
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
	}

	.btn-secondary {
		padding: 0.75rem 1.25rem;
		background: rgba(230, 184, 0, 0.1);
		color: var(--text-secondary);
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-secondary:hover {
		background: rgba(230, 184, 0, 0.2);
		color: var(--text-primary);
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
		color: var(--text-primary);
	}

	.replace-info strong {
		color: var(--text-primary);
	}

	.text-muted {
		color: var(--text-tertiary) !important;
		font-size: 0.875rem;
	}

	.form-hint {
		display: block;
		margin-top: 0.5rem;
		color: var(--text-tertiary);
		font-size: 0.875rem;
	}

	.platform-preview {
		background: rgba(34, 197, 94, 0.1);
		border: 1px solid rgba(34, 197, 94, 0.2);
		border-radius: 6px;
		padding: 0.75rem;
		margin-top: 1rem;
		color: var(--success-emphasis);
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
		color: var(--success-emphasis);
	}

	.alert-error {
		background: rgba(239, 68, 68, 0.15);
		border: 1px solid rgba(239, 68, 68, 0.3);
		color: var(--error-emphasis);
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
		background: rgba(22, 27, 34, 0.4);
		border-radius: 8px;
	}

	.room-selector.loading,
	.room-selector.empty {
		align-items: center;
		justify-content: center;
		padding: 3rem;
		text-align: center;
		color: var(--text-tertiary);
	}

	.room-selector.loading p,
	.room-selector.empty p {
		margin: 1rem 0 0;
	}

	.no-rooms {
		color: var(--text-tertiary);
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
		color: var(--text-tertiary);
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
		background: rgba(13, 17, 23, 0.6);
		border: 1px solid rgba(230, 184, 0, 0.1);
		border-radius: 8px;
		color: var(--text-secondary);
		font-size: 0.9rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.room-tab:hover {
		background: rgba(230, 184, 0, 0.1);
		border-color: rgba(230, 184, 0, 0.3);
		color: var(--text-primary);
	}

	.room-tab.active {
		background: var(--room-color, var(--primary-500));
		border-color: var(--room-color, var(--primary-500));
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
		background: rgba(22, 27, 34, 0.6);
		border: 1px solid rgba(230, 184, 0, 0.1);
		border-radius: 14px;
		margin-bottom: 1.5rem;
		border-left: 4px solid var(--room-color, var(--primary-500));
	}

	.selected-room-info h2 {
		font-size: 1.25rem;
		font-weight: 600;
		color: var(--text-primary);
		margin: 0 0 0.25rem 0;
	}

	.room-type-badge {
		display: inline-block;
		padding: 0.25rem 0.625rem;
		background: rgba(230, 184, 0, 0.15);
		border-radius: 20px;
		font-size: 0.75rem;
		color: var(--text-secondary);
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
		color: var(--text-primary);
	}

	.selected-room-stats .stat-label {
		font-size: 0.8rem;
		color: var(--text-tertiary);
	}

	/* Filters */
	.filters-bar {
		display: flex;
		justify-content: center;
		gap: 1rem;
		margin-bottom: 1.5rem;
		flex-wrap: wrap;
	}

	.search-box {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0 1rem;
		background: rgba(13, 17, 23, 0.6);
		border: 1px solid rgba(230, 184, 0, 0.1);
		border-radius: 10px;
		flex: 1;
		max-width: 400px;
	}

	.search-box :global(svg) {
		color: var(--text-tertiary);
	}

	.search-box input {
		flex: 1;
		padding: 0.625rem 0;
		background: transparent;
		border: none;
		color: var(--text-primary);
		font-size: 0.875rem;
		outline: none;
	}

	.search-box input::placeholder {
		color: var(--text-tertiary);
	}

	.search-box input:focus {
		outline: none;
	}

	.filter-select {
		padding: 0.75rem 1rem;
		background: rgba(13, 17, 23, 0.6);
		border: 1px solid rgba(230, 184, 0, 0.1);
		border-radius: 10px;
		color: var(--text-primary);
		font-size: 0.9rem;
		cursor: pointer;
	}

	.filter-select:focus {
		outline: none;
		border-color: rgba(99, 102, 241, 0.5);
	}

	/* Videos Table */
	.videos-table-wrapper {
		background: rgba(13, 17, 23, 0.6);
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
		padding: 0.75rem 1rem;
		background: rgba(13, 17, 23, 0.6);
		font-weight: 600;
		color: var(--text-tertiary);
		text-transform: uppercase;
		letter-spacing: 0.05em;
		border-bottom: 1px solid rgba(230, 184, 0, 0.1);
	}

	.videos-table td {
		padding: 1rem 1.25rem;
		border-bottom: 1px solid rgba(230, 184, 0, 0.05);
		color: var(--text-primary);
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
		background: var(--bg-elevated);
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
		color: var(--text-muted);
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
		color: var(--text-primary);
	}

	.video-info .video-platform {
		font-size: 0.75rem;
		color: var(--text-tertiary);
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
		background: color-mix(in srgb, var(--tag-color, var(--primary-500)) 15%, transparent);
		border: 1px solid color-mix(in srgb, var(--tag-color, var(--primary-500)) 30%, transparent);
		border-radius: 4px;
		font-size: 0.7rem;
		font-weight: 500;
		color: var(--tag-color, var(--primary-500));
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
		color: var(--text-secondary);
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
		color: var(--text-tertiary);
	}

	.status-toggle {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.375rem 0.75rem;
		background: rgba(148, 163, 184, 0.15);
		border: 1px solid var(--border-default);
		border-radius: 20px;
		font-size: 0.75rem;
		font-weight: 600;
		color: var(--text-secondary);
		cursor: pointer;
		transition: all 0.2s;
	}

	.status-toggle.published {
		background: rgba(34, 197, 94, 0.15);
		border-color: rgba(34, 197, 94, 0.3);
		color: var(--success-base);
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
		color: var(--text-secondary);
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-icon:hover {
		background: rgba(230, 184, 0, 0.2);
		color: var(--primary-500);
	}

	.btn-icon.danger:hover {
		background: rgba(239, 68, 68, 0.1);
		color: var(--error-emphasis);
		border-color: rgba(239, 68, 68, 0.3);
	}

	/* States */
	.loading-state,
	.error-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 2rem;
		text-align: center;
		color: var(--text-secondary);
	}

	.empty-state {
		text-align: center;
		padding: 4rem 2rem;
		background: rgba(22, 27, 34, 0.4);
		border-radius: 8px;
		border: 1px solid var(--border-muted);
	}

	.empty-state :global(svg) {
		color: var(--text-muted);
		margin-bottom: 1rem;
	}

	.empty-state h3 {
		color: var(--text-primary);
		margin: 0 0 0.5rem 0;
	}

	.empty-state p {
		color: var(--text-secondary);
		margin: 0 0 1.5rem 0;
	}

	.spinner {
		width: 40px;
		height: 40px;
		border: 3px solid rgba(230, 184, 0, 0.2);
		border-top-color: var(--primary-500);
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin-bottom: 1rem;
	}

	.checkbox-label input {
		width: 18px;
		height: 18px;
		accent-color: var(--primary-500);
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
		background: var(--bg-elevated);
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
		color: var(--text-primary);
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
		color: var(--text-tertiary);
		font-size: 1.5rem;
		cursor: pointer;
		border-radius: 8px;
		transition: all 0.2s;
	}

	.modal-close:hover {
		background: rgba(230, 184, 0, 0.1);
		color: var(--text-primary);
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
		color: var(--text-secondary);
		margin-bottom: 0.5rem;
	}

	.form-group input,
	.form-group select,
	.form-group textarea {
		width: 100%;
		padding: 0.75rem 1rem;
		background: rgba(13, 17, 23, 0.6);
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 10px;
		color: var(--text-primary);
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
		color: var(--text-tertiary);
		margin-top: 0.375rem;
	}

	.detected-platform {
		color: var(--success-emphasis);
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
		background: rgba(13, 17, 23, 0.6);
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
		color: var(--text-secondary);
		cursor: pointer;
		transition: all 0.15s;
	}

	.category-btn:hover {
		background: color-mix(in srgb, var(--tag-color, var(--primary-500)) 15%, transparent);
		border-color: color-mix(in srgb, var(--tag-color, var(--primary-500)) 30%, transparent);
		color: var(--tag-color, var(--primary-500));
	}

	.category-btn.selected {
		background: color-mix(in srgb, var(--tag-color, var(--primary-500)) 20%, transparent);
		border-color: var(--tag-color, var(--primary-500));
		color: var(--tag-color, var(--primary-500));
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
		color: var(--text-tertiary);
		margin-right: 0.5rem;
	}

	.selected-tag {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.375rem 0.625rem;
		background: color-mix(in srgb, var(--tag-color, var(--primary-500)) 15%, transparent);
		border: 1px solid color-mix(in srgb, var(--tag-color, var(--primary-500)) 30%, transparent);
		border-radius: 4px;
		font-size: 0.75rem;
		font-weight: 500;
		color: var(--tag-color, var(--primary-500));
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
		color: var(--text-secondary);
		cursor: pointer;
	}

	.checkbox-label input {
		width: 18px;
		height: 18px;
		accent-color: var(--primary-500);
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

	/* Bunny.net Button */
	.btn-bunny {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background: linear-gradient(135deg, #ff9500, #ff7b00);
		color: white;
		border: none;
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-bunny:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow: 0 4px 15px rgba(255, 149, 0, 0.4);
	}

	.btn-bunny:disabled {
		opacity: 0.6;
		cursor: not-allowed;
		transform: none;
	}

	/* Analytics Panel */
	.analytics-panel {
		background: rgba(22, 27, 34, 0.4);
		border-radius: 8px;
		padding: 1.5rem;
		margin-bottom: 1.5rem;
	}

	.analytics-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1.5rem;
	}

	.analytics-header h3 {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 1.1rem;
		font-weight: 600;
		color: var(--text-primary);
		margin: 0;
	}

	.analytics-controls {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.analytics-loading,
	.analytics-empty {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 2rem;
		color: var(--text-tertiary);
	}

	.analytics-grid {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 1rem;
		margin-bottom: 1.5rem;
	}

	.analytics-stat {
		text-align: center;
		padding: 1rem;
		background: rgba(99, 102, 241, 0.1);
		border-radius: 10px;
	}

	.analytics-stat .stat-value {
		display: block;
		font-size: 1.75rem;
		font-weight: 700;
		color: var(--text-primary);
		margin-bottom: 0.25rem;
	}

	.analytics-stat .stat-label {
		font-size: 0.8rem;
		color: var(--text-tertiary);
	}

	.top-videos-section,
	.categories-stats-section {
		margin-top: 1.5rem;
		padding-top: 1.5rem;
		border-top: 1px solid rgba(99, 102, 241, 0.1);
	}

	.top-videos-section h4,
	.categories-stats-section h4 {
		font-size: 0.9rem;
		font-weight: 600;
		color: var(--text-secondary);
		margin: 0 0 1rem 0;
	}

	.top-videos-list {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.top-video-item {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 0.75rem;
		background: rgba(13, 17, 23, 0.6);
		border-radius: 8px;
	}

	.top-video-item .rank {
		font-weight: 700;
		color: var(--warning-emphasis);
		min-width: 24px;
	}

	.top-video-item .title {
		flex: 1;
		color: var(--text-primary);
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.top-video-item .views {
		color: var(--text-tertiary);
		font-size: 0.85rem;
	}

	.used-categories-tags {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
	}

	/* Bunny Upload Modal */
	.bunny-header h2 {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.bunny-info {
		margin-bottom: 1.5rem;
	}

	.bunny-info p {
		margin: 0.5rem 0;
		color: var(--text-primary);
	}

	.bunny-dropzone {
		border: 2px dashed rgba(255, 149, 0, 0.3);
		border-radius: 14px;
		padding: 3rem 2rem;
		text-align: center;
		transition: all 0.2s;
		background: rgba(255, 149, 0, 0.05);
	}

	.bunny-dropzone:hover,
	.bunny-dropzone:focus {
		border-color: rgba(255, 149, 0, 0.5);
		background: rgba(255, 149, 0, 0.1);
	}

	.bunny-dropzone.has-files {
		padding: 1.5rem;
		border-style: solid;
	}

	.bunny-dropzone :global(svg) {
		color: #ff9500;
		margin-bottom: 1rem;
	}

	.bunny-dropzone p {
		color: var(--text-secondary);
		margin: 0.5rem 0;
	}

	.bunny-dropzone .file-types {
		font-size: 0.75rem;
		color: var(--text-tertiary);
		margin-top: 1rem;
	}

	.selected-files h4 {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		color: var(--success-emphasis);
		margin: 0 0 1rem 0;
	}

	.file-list {
		list-style: none;
		padding: 0;
		margin: 0;
		text-align: left;
	}

	.file-list li {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 0.75rem;
		background: rgba(13, 17, 23, 0.6);
		border-radius: 8px;
		margin-bottom: 0.5rem;
	}

	.file-list .file-name {
		flex: 1;
		color: var(--text-primary);
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.file-list .file-size {
		color: var(--text-tertiary);
		font-size: 0.85rem;
	}

	.btn-remove {
		padding: 0.375rem;
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.2);
		border-radius: 4px;
		color: var(--error-emphasis);
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-remove:hover {
		background: rgba(239, 68, 68, 0.2);
	}

	.upload-progress {
		margin-top: 1.5rem;
	}

	.progress-bar {
		height: 8px;
		background: rgba(255, 149, 0, 0.2);
		border-radius: 4px;
		overflow: hidden;
	}

	.progress-fill {
		height: 100%;
		background: linear-gradient(90deg, #ff9500, #ff7b00);
		border-radius: 4px;
		transition: width 0.3s ease;
	}

	.progress-text {
		text-align: center;
		color: #ff9500;
		font-weight: 600;
		margin: 0.75rem 0;
	}

	.batch-status {
		display: flex;
		justify-content: center;
		gap: 1.5rem;
		flex-wrap: wrap;
	}

	.status-item {
		font-size: 0.85rem;
	}

	.status-item.completed {
		color: var(--success-emphasis);
	}
	.status-item.in-progress {
		color: var(--warning-emphasis);
	}
	.status-item.pending {
		color: var(--text-secondary);
	}
	.status-item.failed {
		color: var(--error-emphasis);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ICT 7 ADDITION: BULK OPERATIONS STYLES
	   ═══════════════════════════════════════════════════════════════════════════ */

	.bulk-action-bar {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 0.75rem 1rem;
		background: linear-gradient(135deg, rgba(99, 102, 241, 0.15), rgba(230, 184, 0, 0.1));
		border: 1px solid rgba(99, 102, 241, 0.3);
		border-radius: 10px;
		margin-bottom: 1rem;
	}

	.bulk-selection-info {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		color: var(--text-primary);
		font-weight: 500;
	}

	.bulk-actions {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		flex-wrap: wrap;
	}

	.btn-bulk {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.5rem 0.75rem;
		background: rgba(22, 27, 34, 0.8);
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 6px;
		font-size: 0.8rem;
		font-weight: 500;
		color: var(--text-secondary);
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-bulk:hover:not(:disabled) {
		background: rgba(230, 184, 0, 0.15);
		border-color: rgba(230, 184, 0, 0.4);
		color: var(--primary-500);
	}

	.btn-bulk:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.btn-bulk.danger:hover:not(:disabled) {
		background: rgba(239, 68, 68, 0.15);
		border-color: rgba(239, 68, 68, 0.4);
		color: var(--error-emphasis);
	}

	.bulk-spinner {
		width: 16px;
		height: 16px;
		border: 2px solid rgba(230, 184, 0, 0.3);
		border-top-color: var(--primary-500);
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	/* Checkbox column */
	.checkbox-col {
		width: 48px;
		text-align: center;
	}

	.select-all-btn,
	.select-row-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 0.25rem;
		background: transparent;
		border: none;
		color: var(--text-tertiary);
		cursor: pointer;
		transition: color 0.15s;
	}

	.select-all-btn:hover,
	.select-row-btn:hover {
		color: var(--primary-500);
	}

	.select-all-btn :global(.partial) {
		opacity: 0.6;
	}

	tr.selected {
		background: rgba(99, 102, 241, 0.1) !important;
	}

	tr.selected td {
		border-color: rgba(99, 102, 241, 0.2);
	}

	/* Bulk Tags Modal */
	.bulk-info {
		margin-bottom: 1.5rem;
		padding: 0.75rem 1rem;
		background: rgba(99, 102, 241, 0.1);
		border-radius: 8px;
		color: var(--text-primary);
	}

	.bulk-tags-grid {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
		padding: 0.75rem;
		background: rgba(13, 17, 23, 0.6);
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 10px;
		max-height: 200px;
		overflow-y: auto;
	}

	.bulk-tag-btn {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.5rem 0.75rem;
		background: rgba(100, 116, 139, 0.1);
		border: 1px solid rgba(100, 116, 139, 0.2);
		border-radius: 6px;
		font-size: 0.8rem;
		font-weight: 500;
		color: var(--text-secondary);
		cursor: pointer;
		transition: all 0.15s;
	}

	.bulk-tag-btn.add:hover {
		background: rgba(34, 197, 94, 0.15);
		border-color: rgba(34, 197, 94, 0.3);
		color: #22c55e;
	}

	.bulk-tag-btn.add.selected {
		background: rgba(34, 197, 94, 0.2);
		border-color: #22c55e;
		color: #22c55e;
	}

	.bulk-tag-btn.remove:hover {
		background: rgba(239, 68, 68, 0.15);
		border-color: rgba(239, 68, 68, 0.3);
		color: #ef4444;
	}

	.bulk-tag-btn.remove.selected {
		background: rgba(239, 68, 68, 0.2);
		border-color: #ef4444;
		color: #ef4444;
	}

	.bulk-tags-summary {
		margin-top: 1rem;
		padding: 0.75rem;
		background: rgba(230, 184, 0, 0.05);
		border-radius: 8px;
		font-size: 0.85rem;
		color: var(--text-secondary);
	}

	.bulk-tags-summary p {
		margin: 0.25rem 0;
	}

	/* Embed Code Modal */
	.embed-title {
		margin-bottom: 1rem;
		color: var(--text-primary);
	}

	.embed-code-box {
		background: rgba(13, 17, 23, 0.8);
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 8px;
		padding: 1rem;
		overflow-x: auto;
	}

	.embed-code-box pre {
		margin: 0;
		white-space: pre-wrap;
		word-break: break-all;
	}

	.embed-code-box code {
		font-family: 'SF Mono', Monaco, 'Cascadia Code', 'Roboto Mono', Consolas, monospace;
		font-size: 0.8rem;
		color: var(--primary-500);
	}

	.embed-hint {
		margin-top: 1rem;
		font-size: 0.85rem;
		color: var(--text-tertiary);
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

		.analytics-grid {
			grid-template-columns: repeat(2, 1fr);
		}

		.header-actions {
			flex-wrap: wrap;
		}
	}
</style>

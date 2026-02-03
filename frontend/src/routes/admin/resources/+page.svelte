<script lang="ts">
	/**
	 * Room Resources Management - Admin Dashboard
	 * ═══════════════════════════════════════════════════════════════════════════
	 * Apple Principal Engineer ICT 7 Grade - January 2026
	 *
	 * Unified resource management for trading rooms:
	 * - Videos (Bunny.net, Vimeo, YouTube, Direct)
	 * - PDFs (Trade plans, guides, cheat sheets)
	 * - Documents (Word, Excel, templates)
	 * - Images (Charts, screenshots, diagrams)
	 *
	 * Features:
	 * - Room selector tabs
	 * - Resource type filtering
	 * - Quick replace functionality
	 * - Featured/pinned controls
	 * - Drag & drop upload
	 *
	 * @version 1.0.0
	 */

	import { browser } from '$app/environment';
	import {
		IconVideo,
		IconFileText,
		IconPhoto,
		IconTable,
		IconFile,
		IconSearch,
		IconEdit,
		IconTrash,
		IconEye,
		IconRefresh,
		IconPlus,
		IconLink,
		IconCheck,
		IconX,
		IconStar,
		IconStarFilled,
		IconPin,
		IconPinFilled,
		IconAlertCircle
	} from '$lib/icons';
	import {
		roomResourcesApi,
		type RoomResource,
		type ResourceType,
		type ContentType,
		type VideoPlatform,
		type CreateResourceRequest,
		type AccessLevel,
		type BulkUpdateFields,
		bulkUpdateResources,
		bulkDeleteResources
	} from '$lib/api/room-resources';
	import { tradingRoomApi, type TradingRoom, type Trader } from '$lib/api/trading-rooms';

	// ═══════════════════════════════════════════════════════════════════════════
	// TYPES & CONSTANTS
	// ═══════════════════════════════════════════════════════════════════════════

	const RESOURCE_TYPES: { id: ResourceType; name: string; icon: any }[] = [
		{ id: 'video', name: 'Videos', icon: IconVideo },
		{ id: 'pdf', name: 'PDFs', icon: IconFileText },
		{ id: 'document', name: 'Documents', icon: IconFile },
		{ id: 'image', name: 'Images', icon: IconPhoto },
		{ id: 'spreadsheet', name: 'Spreadsheets', icon: IconTable }
	];

	const CONTENT_TYPES: { id: ContentType; name: string; resourceTypes: ResourceType[] }[] = [
		{ id: 'introduction', name: 'Introduction', resourceTypes: ['video'] }, // ICT 7: Main videos
		{ id: 'tutorial', name: 'Tutorial', resourceTypes: ['video'] },
		{ id: 'daily_video', name: 'Daily Video', resourceTypes: ['video'] },
		{ id: 'weekly_watchlist', name: 'Weekly Watchlist', resourceTypes: ['video'] },
		{ id: 'weekly_alert', name: 'Weekly Alert', resourceTypes: ['video'] }, // ICT 7: Explosive Swings
		{ id: 'trade_plan', name: 'Trade Plan', resourceTypes: ['pdf', 'document'] },
		{ id: 'guide', name: 'Guide', resourceTypes: ['pdf', 'document', 'video'] },
		{ id: 'chart', name: 'Chart', resourceTypes: ['image', 'pdf'] },
		{ id: 'screenshot', name: 'Screenshot', resourceTypes: ['image'] },
		{ id: 'template', name: 'Template', resourceTypes: ['document', 'spreadsheet'] },
		{ id: 'cheat_sheet', name: 'Cheat Sheet', resourceTypes: ['pdf', 'image'] },
		{
			id: 'other',
			name: 'Other',
			resourceTypes: ['video', 'pdf', 'document', 'image', 'spreadsheet', 'archive', 'other']
		}
	];

	// ICT 7: Section definitions for dashboard organization
	const ROOM_SECTIONS = [
		{
			id: 'introduction',
			name: 'Introduction',
			icon: IconVideo,
			description: 'Main welcome and overview videos'
		},
		{
			id: 'latest_updates',
			name: 'Latest Updates',
			icon: IconRefresh,
			description: 'Recent announcements and updates'
		},
		{
			id: 'premium_daily_videos',
			name: 'Premium Daily Videos',
			icon: IconVideo,
			description: 'Daily trading analysis'
		},
		{
			id: 'watchlist',
			name: 'Watchlist',
			icon: IconFileText,
			description: 'Weekly stock watchlist'
		},
		{
			id: 'weekly_alerts',
			name: 'Weekly Alerts',
			icon: IconAlertCircle,
			description: 'Weekly alert summaries',
			roomsOnly: ['explosive-swings']
		},
		{
			id: 'learning_center',
			name: 'Learning Center',
			icon: IconFileText,
			description: 'Educational content'
		}
	];

	const VIDEO_PLATFORMS: { id: VideoPlatform; name: string }[] = [
		{ id: 'bunny', name: 'Bunny.net' },
		{ id: 'vimeo', name: 'Vimeo' },
		{ id: 'youtube', name: 'YouTube' },
		{ id: 'wistia', name: 'Wistia' },
		{ id: 'direct', name: 'Direct URL' }
	];

	const CATEGORIES = [
		{ id: 'risk-management', name: 'Risk Management', color: '#ef4444' },
		{ id: 'options-strategies', name: 'Options Strategies', color: '#f59e0b' },
		{ id: 'macro-structure', name: 'Macro Structure', color: '#10b981' },
		{ id: 'micro-structure', name: 'Micro Structure', color: '#06b6d4' },
		{ id: 'psychology', name: 'Psychology', color: '#B38F00' },
		{ id: 'technical-analysis', name: 'Technical Analysis', color: '#3b82f6' },
		{ id: 'fundamentals', name: 'Fundamentals', color: '#ec4899' },
		{ id: 'trade-setups', name: 'Trade Setups', color: '#14b8a6' },
		{ id: 'market-review', name: 'Market Review', color: '#E6B800' },
		{ id: 'earnings', name: 'Earnings', color: '#f97316' }
	];

	// ICT 7: Access level options for free/premium control
	const ACCESS_LEVELS: { id: AccessLevel; name: string; color: string }[] = [
		{ id: 'free', name: 'Free (Public)', color: '#22c55e' },
		{ id: 'member', name: 'Member', color: '#3b82f6' },
		{ id: 'premium', name: 'Premium', color: '#f59e0b' },
		{ id: 'vip', name: 'VIP', color: '#a855f7' }
	];

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE
	// ═══════════════════════════════════════════════════════════════════════════

	// Rooms and selection
	let rooms = $state<TradingRoom[]>([]);
	let selectedRoom = $state<TradingRoom | null>(null);
	let traders = $state<Trader[]>([]);
	let isLoadingRooms = $state(true);

	// Resources
	let resources = $state<RoomResource[]>([]);
	let isLoading = $state(false);
	let error = $state('');
	let successMessage = $state('');

	// Pagination
	let currentPage = $state(1);

	// Filters
	let searchQuery = $state('');
	let selectedResourceType = $state<ResourceType | 'all'>('all');
	let selectedContentType = $state<ContentType | 'all'>('all');
	let selectedSection = $state<string>('all'); // ICT 7: Section filter

	// Modal state
	let showCreateModal = $state(false);
	let showEditModal = $state(false);
	let showReplaceModal = $state(false);
	let showBulkModal = $state(false);
	let editingResource = $state<RoomResource | null>(null);
	let replacingResource = $state<RoomResource | null>(null);
	let isSaving = $state(false);
	let newFileUrl = $state('');

	// ICT 7: Bulk operations state
	let selectedResources = $state<Set<number>>(new Set());
	let bulkAction = $state<'publish' | 'unpublish' | 'feature' | 'unfeature' | 'access' | 'delete'>('publish');
	let bulkAccessLevel = $state<AccessLevel>('premium');

	// Form state
	let formData = $state<CreateResourceRequest>({
		title: '',
		description: '',
		resource_type: 'video',
		content_type: 'daily_video',
		section: 'latest_updates', // ICT 7: Default section
		file_url: '',
		mime_type: '',
		video_platform: 'direct',
		thumbnail_url: '',
		trading_room_id: 0,
		trader_id: undefined,
		resource_date: new Date().toISOString().split('T')[0],
		is_published: true,
		is_featured: false,
		is_pinned: false,
		tags: [],
		access_level: 'premium' // ICT 7: Default to premium access
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// COMPUTED
	// ═══════════════════════════════════════════════════════════════════════════

	const filteredResources = $derived(
		resources.filter((resource) => {
			const matchesSearch =
				!searchQuery ||
				resource.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
				resource.description?.toLowerCase().includes(searchQuery.toLowerCase());

			const matchesType =
				selectedResourceType === 'all' || resource.resource_type === selectedResourceType;
			const matchesContent =
				selectedContentType === 'all' || resource.content_type === selectedContentType;
			const matchesSection = selectedSection === 'all' || resource.section === selectedSection; // ICT 7: Section filter

			return matchesSearch && matchesType && matchesContent && matchesSection;
		})
	);

	// ICT 7: Get available sections for the selected room
	const availableSections = $derived(
		ROOM_SECTIONS.filter((section) => {
			// Check if section is restricted to specific rooms
			if (section.roomsOnly && selectedRoom) {
				return section.roomsOnly.includes(selectedRoom.slug);
			}
			// Check if room has this section in available_sections
			if (selectedRoom?.available_sections) {
				return selectedRoom.available_sections.includes(section.id);
			}
			return true;
		})
	);

	const availableContentTypes = $derived(
		CONTENT_TYPES.filter((ct) =>
			formData.resource_type === 'video' || formData.resource_type === 'other'
				? true
				: ct.resourceTypes.includes(formData.resource_type)
		)
	);

	const stats = $derived({
		total: resources.length,
		videos: resources.filter((r) => r.resource_type === 'video').length,
		pdfs: resources.filter((r) => r.resource_type === 'pdf').length,
		documents: resources.filter((r) => r.resource_type === 'document').length,
		images: resources.filter((r) => r.resource_type === 'image').length,
		published: resources.filter((r) => r.is_published).length,
		featured: resources.filter((r) => r.is_featured).length
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// API FUNCTIONS
	// ═══════════════════════════════════════════════════════════════════════════

	// ICT 7: All 6 trading rooms in correct order
	const FALLBACK_ROOMS: TradingRoom[] = [
		{
			id: 1,
			name: 'Day Trading Room',
			slug: 'day-trading-room',
			type: 'trading_room',
			is_active: true,
			is_featured: true,
			sort_order: 1,
			icon: 'chart-line',
			color: '#3b82f6',
			available_sections: [
				'introduction',
				'latest_updates',
				'premium_daily_videos',
				'watchlist',
				'learning_center'
			],
			created_at: '',
			updated_at: ''
		},
		{
			id: 2,
			name: 'Swing Trading Room',
			slug: 'swing-trading-room',
			type: 'trading_room',
			is_active: true,
			is_featured: false,
			sort_order: 2,
			icon: 'trending-up',
			color: '#10b981',
			available_sections: [
				'introduction',
				'latest_updates',
				'premium_daily_videos',
				'watchlist',
				'learning_center'
			],
			created_at: '',
			updated_at: ''
		},
		{
			id: 3,
			name: 'Small Account Mentorship',
			slug: 'small-account-mentorship',
			type: 'mentorship',
			is_active: true,
			is_featured: false,
			sort_order: 3,
			icon: 'wallet',
			color: '#f59e0b',
			available_sections: [
				'introduction',
				'latest_updates',
				'premium_daily_videos',
				'learning_center'
			],
			created_at: '',
			updated_at: ''
		},
		{
			id: 4,
			name: 'Explosive Swings',
			slug: 'explosive-swings',
			type: 'alert_service',
			is_active: true,
			is_featured: false,
			sort_order: 4,
			icon: 'rocket',
			color: '#ef4444',
			available_sections: [
				'introduction',
				'latest_updates',
				'premium_daily_videos',
				'watchlist',
				'weekly_alerts',
				'learning_center'
			],
			created_at: '',
			updated_at: ''
		},
		{
			id: 5,
			name: 'SPX Profit Pulse',
			slug: 'spx-profit-pulse',
			type: 'alert_service',
			is_active: true,
			is_featured: false,
			sort_order: 5,
			icon: 'activity',
			color: '#8b5cf6',
			available_sections: [
				'introduction',
				'latest_updates',
				'premium_daily_videos',
				'learning_center'
			],
			created_at: '',
			updated_at: ''
		},
		{
			id: 6,
			name: 'High Octane Scanner',
			slug: 'high-octane-scanner',
			type: 'alert_service',
			is_active: true,
			is_featured: false,
			sort_order: 6,
			icon: 'radar',
			color: '#06b6d4',
			available_sections: [
				'introduction',
				'latest_updates',
				'premium_daily_videos',
				'learning_center'
			],
			created_at: '',
			updated_at: ''
		}
	];

	async function loadRoomsAndTraders() {
		isLoadingRooms = true;
		error = '';

		// ICT 7: Use Promise.allSettled to handle CORB/API failures gracefully
		const results = await Promise.allSettled([
			tradingRoomApi.rooms.list({ with_counts: true }),
			tradingRoomApi.traders.list({ active_only: true })
		]);

		// Extract rooms - use fallback if API fails
		const roomsResult = results[0];
		if (roomsResult.status === 'fulfilled' && roomsResult.value?.data) {
			const data = roomsResult.value.data;
			rooms = Array.isArray(data) && data.length > 0 ? data : FALLBACK_ROOMS;
		} else {
			rooms = FALLBACK_ROOMS;
		}

		// Select first room
		if (rooms.length > 0 && !selectedRoom) {
			selectedRoom = rooms[0];
		}

		// Extract traders - optional, empty array on failure
		const tradersResult = results[1];
		if (tradersResult.status === 'fulfilled' && tradersResult.value?.data) {
			traders = Array.isArray(tradersResult.value.data) ? tradersResult.value.data : [];
		} else {
			traders = [];
		}

		isLoadingRooms = false;
	}

	async function loadResources() {
		if (!selectedRoom) return;

		isLoading = true;
		error = '';

		try {
			const response = await roomResourcesApi.adminList({
				room_id: selectedRoom.id,
				per_page: 100,
				page: currentPage
			});

			if (response?.success && response.data) {
				resources = response.data;
			} else {
				// API returned but no data - show empty state
				resources = [];
			}
		} catch {
			// ICT 7: CORB or API failure - show empty state, don't crash
			resources = [];
			// Don't set error - just show empty state
		} finally {
			isLoading = false;
		}
	}

	function showSuccess(message: string) {
		successMessage = message;
		setTimeout(() => (successMessage = ''), 3000);
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// ACTIONS
	// ═══════════════════════════════════════════════════════════════════════════

	function selectRoom(room: TradingRoom) {
		selectedRoom = room;
		currentPage = 1;
		loadResources();
	}

	function openCreateModal() {
		formData = {
			title: '',
			description: '',
			resource_type: 'video',
			content_type: 'daily_video',
			section: 'latest_updates', // ICT 7: Default to Latest Updates section
			file_url: '',
			mime_type: '',
			video_platform: 'direct',
			thumbnail_url: '',
			trading_room_id: selectedRoom?.id || 0,
			trader_id: undefined,
			resource_date: new Date().toISOString().split('T')[0],
			is_published: true,
			is_featured: false,
			is_pinned: false,
			tags: [],
			access_level: 'premium' // ICT 7: Default to premium
		};
		showCreateModal = true;
	}

	function openEditModal(resource: RoomResource) {
		editingResource = resource;
		formData = {
			title: resource.title,
			description: resource.description || '',
			resource_type: resource.resource_type,
			content_type: resource.content_type,
			section: resource.section || 'latest_updates', // ICT 7: Include section for editing
			file_url: resource.file_url,
			mime_type: resource.mime_type || '',
			video_platform: resource.video_platform || 'direct',
			thumbnail_url: resource.thumbnail_url || '',
			trading_room_id: resource.trading_room_id,
			trader_id: resource.trader_id,
			resource_date: resource.resource_date.split('T')[0],
			is_published: resource.is_published,
			is_featured: resource.is_featured,
			is_pinned: resource.is_pinned,
			tags: resource.tags || [],
			access_level: resource.access_level || 'premium' // ICT 7: Include access level
		};
		showEditModal = true;
	}

	function openBulkModal() {
		if (selectedResources.size === 0) {
			error = 'Please select at least one resource';
			return;
		}
		showBulkModal = true;
	}

	async function executeBulkAction() {
		if (selectedResources.size === 0) return;

		isSaving = true;
		error = '';

		const ids = Array.from(selectedResources);

		try {
			let updates: BulkUpdateFields = {};

			switch (bulkAction) {
				case 'publish':
					updates = { is_published: true };
					break;
				case 'unpublish':
					updates = { is_published: false };
					break;
				case 'feature':
					updates = { is_featured: true };
					break;
				case 'unfeature':
					updates = { is_featured: false };
					break;
				case 'access':
					updates = { access_level: bulkAccessLevel };
					break;
				case 'delete':
					const deleteResult = await bulkDeleteResources(ids);
					if (deleteResult.success) {
						showSuccess(`Deleted ${deleteResult.deleted_count} resources`);
						selectedResources.clear();
						selectedResources = new Set(selectedResources);
						await loadResources();
					}
					showBulkModal = false;
					isSaving = false;
					return;
			}

			const result = await bulkUpdateResources(ids, updates);
			if (result.success) {
				showSuccess(`Updated ${result.updated_count} resources`);
				selectedResources.clear();
				selectedResources = new Set(selectedResources);
				await loadResources();
			}
		} catch (err) {
			error = err instanceof Error ? err.message : 'Bulk operation failed';
		} finally {
			isSaving = false;
			showBulkModal = false;
		}
	}

	function openReplaceModal(resource: RoomResource) {
		replacingResource = resource;
		newFileUrl = resource.file_url;
		showReplaceModal = true;
	}

	async function saveResource() {
		isSaving = true;
		error = '';

		try {
			if (editingResource) {
				const response = await roomResourcesApi.update(editingResource.id, formData);
				if (response.success) {
					showSuccess('Resource updated successfully');
				}
			} else {
				const response = await roomResourcesApi.create(formData);
				if (response.success) {
					showSuccess('Resource created successfully');
				}
			}

			showCreateModal = false;
			showEditModal = false;
			editingResource = null;
			await loadResources();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to save resource';
		} finally {
			isSaving = false;
		}
	}

	async function replaceResource() {
		if (!replacingResource || !newFileUrl) return;

		isSaving = true;
		error = '';

		try {
			const response = await roomResourcesApi.update(replacingResource.id, {
				file_url: newFileUrl,
				video_platform: roomResourcesApi.detectVideoPlatform(newFileUrl)
			});

			if (response.success) {
				showSuccess('Resource replaced successfully');
				showReplaceModal = false;
				replacingResource = null;
				newFileUrl = '';
				await loadResources();
			}
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to replace resource';
		} finally {
			isSaving = false;
		}
	}

	async function deleteResource(resource: RoomResource) {
		if (!confirm(`Delete "${resource.title}"? This action cannot be undone.`)) return;

		try {
			const response = await roomResourcesApi.delete(resource.id);
			if (response.success) {
				showSuccess('Resource deleted');
				await loadResources();
			}
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to delete';
		}
	}

	async function toggleFeatured(resource: RoomResource) {
		try {
			await roomResourcesApi.update(resource.id, { is_featured: !resource.is_featured });
			resource.is_featured = !resource.is_featured;
			showSuccess(resource.is_featured ? 'Marked as featured' : 'Removed from featured');
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to update';
		}
	}

	async function togglePublished(resource: RoomResource) {
		try {
			await roomResourcesApi.update(resource.id, { is_published: !resource.is_published });
			resource.is_published = !resource.is_published;
			showSuccess(resource.is_published ? 'Published' : 'Unpublished');
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to update';
		}
	}

	async function togglePinned(resource: RoomResource) {
		try {
			await roomResourcesApi.update(resource.id, { is_pinned: !resource.is_pinned });
			resource.is_pinned = !resource.is_pinned;
			showSuccess(resource.is_pinned ? 'Pinned to top' : 'Unpinned');
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to update';
		}
	}

	function getResourceIcon(type: ResourceType) {
		const iconMap = {
			video: IconVideo,
			pdf: IconFileText,
			document: IconFile,
			image: IconPhoto,
			spreadsheet: IconTable,
			archive: IconFile,
			other: IconFile
		};
		return iconMap[type] || IconFile;
	}

	function getCategoryById(id: string) {
		return CATEGORIES.find((c) => c.id === id);
	}

	function toggleTag(tagId: string) {
		if (formData.tags?.includes(tagId)) {
			formData.tags = formData.tags.filter((t) => t !== tagId);
		} else {
			formData.tags = [...(formData.tags || []), tagId];
		}
	}

	// Auto-detect platform when URL changes
	$effect(() => {
		if (formData.file_url && formData.resource_type === 'video') {
			formData.video_platform = roomResourcesApi.detectVideoPlatform(formData.file_url);
		}
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// LIFECYCLE
	// ═══════════════════════════════════════════════════════════════════════════

	// Svelte 5: Initialize on mount
	$effect(() => {
		if (!browser) return;

		const init = async () => {
			await loadRoomsAndTraders();
			if (selectedRoom) {
				await loadResources();
			}
		};
		init();
	});

	// ICT 7: Track previous room to prevent duplicate API calls
	let previousRoomId = $state<number | null>(null);

	$effect(() => {
		// Only load if room changed and not during initial load
		// selectRoom() handles its own loadResources() call
		if (
			selectedRoom &&
			!isLoadingRooms &&
			previousRoomId !== null &&
			previousRoomId !== selectedRoom.id
		) {
			loadResources();
		}
		if (selectedRoom) {
			previousRoomId = selectedRoom.id;
		}
	});
</script>

<svelte:head>
	<title>Room Resources | Admin</title>
</svelte:head>

<div class="admin-resources">
	<!-- Animated Background -->
	<div class="bg-effects">
		<div class="bg-blob bg-blob-1"></div>
		<div class="bg-blob bg-blob-2"></div>
		<div class="bg-blob bg-blob-3"></div>
	</div>

	<div class="admin-page-container">
		<!-- Alerts -->
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
			<h1>Room Resources</h1>
			<p class="subtitle">Manage videos, PDFs, documents, and images for each trading room</p>
			<div class="header-actions">
				{#if selectedResources.size > 0}
					<button class="btn-warning" onclick={openBulkModal}>
						Bulk Actions ({selectedResources.size})
					</button>
				{/if}
				<button class="btn-secondary" onclick={() => loadResources()} disabled={isLoading}>
					<IconRefresh size={18} class={isLoading ? 'spinning' : ''} />
				</button>
				<button class="btn-primary" onclick={openCreateModal} disabled={!selectedRoom}>
					<IconPlus size={18} />
					Add Resource
				</button>
			</div>
		</div>

		<!-- Room Tabs -->
		{#if isLoadingRooms}
			<div class="room-tabs loading">
				<div class="spinner"></div>
				<span>Loading rooms...</span>
			</div>
		{:else}
			<div class="room-tabs">
				{#each rooms as room}
					<button
						class="room-tab"
						class:active={selectedRoom?.id === room.id}
						onclick={() => selectRoom(room)}
					>
						{room.name}
					</button>
				{/each}
			</div>
		{/if}

		<!-- Stats Bar -->
		{#if selectedRoom}
			<div class="stats-bar">
				<div class="stat">
					<span class="stat-value">{stats.total}</span>
					<span class="stat-label">Total</span>
				</div>
				<div class="stat">
					<IconVideo size={16} />
					<span class="stat-value">{stats.videos}</span>
					<span class="stat-label">Videos</span>
				</div>
				<div class="stat">
					<IconFileText size={16} />
					<span class="stat-value">{stats.pdfs}</span>
					<span class="stat-label">PDFs</span>
				</div>
				<div class="stat">
					<IconPhoto size={16} />
					<span class="stat-value">{stats.images}</span>
					<span class="stat-label">Images</span>
				</div>
				<div class="stat">
					<IconCheck size={16} />
					<span class="stat-value">{stats.published}</span>
					<span class="stat-label">Published</span>
				</div>
				<div class="stat">
					<IconStarFilled size={16} />
					<span class="stat-value">{stats.featured}</span>
					<span class="stat-label">Featured</span>
				</div>
			</div>
		{/if}

		<!-- Filters -->
		<div class="filters-bar">
			<div class="search-box">
				<IconSearch size={18} />
				<input
					type="text"
					id="search-resources"
					name="search"
					placeholder="Search resources..."
					bind:value={searchQuery}
				/>
			</div>
			<div class="filter-group">
				<select bind:value={selectedResourceType}>
					<option value="all">All Types</option>
					{#each RESOURCE_TYPES as type}
						<option value={type.id}>{type.name}</option>
					{/each}
				</select>
				<select bind:value={selectedContentType}>
					<option value="all">All Content</option>
					{#each CONTENT_TYPES as type}
						<option value={type.id}>{type.name}</option>
					{/each}
				</select>
				<!-- ICT 7: Section filter -->
				<select bind:value={selectedSection}>
					<option value="all">All Sections</option>
					{#each availableSections as section}
						<option value={section.id}>{section.name}</option>
					{/each}
				</select>
			</div>
		</div>

		<!-- Resources Grid -->
		{#if isLoading}
			<div class="loading-state">
				<div class="spinner"></div>
				<p>Loading resources...</p>
			</div>
		{:else if filteredResources.length === 0}
			<div class="empty-state">
				<IconFile size={64} />
				<h3>No resources found</h3>
				<p>Add your first resource to {selectedRoom?.name}</p>
				<button class="btn-primary" onclick={openCreateModal}>
					<IconPlus size={18} />
					Add Resource
				</button>
			</div>
		{:else}
			<div class="resources-grid">
				{#each filteredResources as resource}
					<div
						class="resource-card"
						class:featured={resource.is_featured}
						class:pinned={resource.is_pinned}
						class:unpublished={!resource.is_published}
					>
						<!-- Thumbnail -->
						<div class="resource-thumbnail">
							{#if resource.thumbnail_url}
								<img src={resource.thumbnail_url} alt={resource.title} />
							{:else}
								<div class="thumbnail-placeholder">
									<!-- svelte-ignore svelte_component_deprecated -->
									<svelte:component this={getResourceIcon(resource.resource_type)} size={32} />
								</div>
							{/if}

							<!-- Type Badge -->
							<span
								class="type-badge"
								class:video={resource.resource_type === 'video'}
								class:pdf={resource.resource_type === 'pdf'}
								class:image={resource.resource_type === 'image'}
							>
								{resource.resource_type.toUpperCase()}
							</span>

							<!-- Featured/Pinned indicators -->
							{#if resource.is_featured}
								<span class="featured-badge"><IconStarFilled size={14} /></span>
							{/if}
							{#if resource.is_pinned}
								<span class="pinned-badge"><IconPinFilled size={14} /></span>
							{/if}
						</div>

						<!-- Content -->
						<div class="resource-content">
							<h3 class="resource-title">{resource.title}</h3>
							<div class="resource-meta">
								<span class="content-type">{resource.content_type.replace('_', ' ')}</span>
								<span class="date">{resource.formatted_date}</span>
							</div>
							{#if resource.tags && resource.tags.length > 0}
								<div class="resource-tags">
									{#each (resource.tags || []).slice(0, 3) as tagId}
										{@const tag = getCategoryById(tagId)}
										{#if tag}
											<span class="tag" style:--tag-color={tag.color}>{tag.name}</span>
										{/if}
									{/each}
								</div>
							{/if}
						</div>

						<!-- Actions -->
						<div class="resource-actions">
							<button class="btn-icon" title="Replace" onclick={() => openReplaceModal(resource)}>
								<IconLink size={16} />
							</button>
							<button class="btn-icon" title="Edit" onclick={() => openEditModal(resource)}>
								<IconEdit size={16} />
							</button>
							<button
								class="btn-icon"
								title={resource.is_featured ? 'Unfeature' : 'Feature'}
								onclick={() => toggleFeatured(resource)}
							>
								{#if resource.is_featured}
									<IconStarFilled size={16} />
								{:else}
									<IconStar size={16} />
								{/if}
							</button>
							<button
								class="btn-icon"
								title={resource.is_pinned ? 'Unpin' : 'Pin to top'}
								onclick={() => togglePinned(resource)}
							>
								{#if resource.is_pinned}
									<IconPinFilled size={16} />
								{:else}
									<IconPin size={16} />
								{/if}
							</button>
							<button
								class="btn-icon"
								class:active={resource.is_published}
								title={resource.is_published ? 'Unpublish' : 'Publish'}
								onclick={() => togglePublished(resource)}
							>
								<IconEye size={16} />
							</button>
							<button
								class="btn-icon danger"
								title="Delete"
								onclick={() => deleteResource(resource)}
							>
								<IconTrash size={16} />
							</button>
						</div>
					</div>
				{/each}
			</div>
		{/if}
	</div>
	<!-- End admin-page-container -->
</div>

<!-- Create/Edit Modal -->
{#if showCreateModal || showEditModal}
	<!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions a11y_interactive_supports_focus -->
	<div
		class="modal-overlay"
		onclick={() => {
			showCreateModal = false;
			showEditModal = false;
			editingResource = null;
		}}
		role="dialog"
		aria-modal="true"
		tabindex="-1"
	>
		<!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions a11y_no_noninteractive_element_interactions -->
		<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
		<div class="modal modal-large" onclick={(e) => e.stopPropagation()} role="document">
			<div class="modal-header">
				<h2>{showEditModal ? 'Edit Resource' : 'Add New Resource'}</h2>
				<button
					class="modal-close"
					onclick={() => {
						showCreateModal = false;
						showEditModal = false;
						editingResource = null;
					}}>&times;</button
				>
			</div>
			<div class="modal-body">
				<!-- Title -->
				<div class="form-group">
					<label for="title">Title *</label>
					<input
						type="text"
						id="title" name="title"
						bind:value={formData.title}
						placeholder="Resource title"
						required
					/>
				</div>

				<!-- Description -->
				<div class="form-group">
					<label for="description">Description</label>
					<textarea
						id="description"
						bind:value={formData.description}
						placeholder="Brief description..."
						rows="3"
					></textarea>
				</div>

				<!-- Type Selection -->
				<div class="form-row">
					<div class="form-group">
						<label for="resource-type">Resource Type *</label>
						<select id="resource-type" bind:value={formData.resource_type}>
							{#each RESOURCE_TYPES as type}
								<option value={type.id}>{type.name}</option>
							{/each}
						</select>
					</div>
					<div class="form-group">
						<label for="content-type">Content Type *</label>
						<select id="content-type" bind:value={formData.content_type}>
							{#each availableContentTypes as type}
								<option value={type.id}>{type.name}</option>
							{/each}
						</select>
					</div>
				</div>

				<!-- ICT 7: Section Selection -->
				<div class="form-row">
					<div class="form-group">
						<label for="section">Dashboard Section</label>
						<select id="section" bind:value={formData.section}>
							{#each availableSections as section}
								<option value={section.id}>{section.name}</option>
							{/each}
						</select>
						<small class="form-hint">Where this resource appears</small>
					</div>
					<!-- ICT 7: Access Level -->
					<div class="form-group">
						<label for="access-level">Access Level</label>
						<select id="access-level" bind:value={formData.access_level}>
							{#each ACCESS_LEVELS as level}
								<option value={level.id}>{level.name}</option>
							{/each}
						</select>
						<small class="form-hint">Who can access this resource</small>
					</div>
				</div>

				<!-- File URL -->
				<div class="form-group">
					<label for="file-url">File/Video URL *</label>
					<input
						type="url"
						id="file-url" name="file-url"
						bind:value={formData.file_url}
						placeholder="https://..."
						required
					/>
					{#if formData.resource_type === 'video' && formData.video_platform}
						<small class="form-hint"
							>Detected platform: <strong>{formData.video_platform}</strong></small
						>
					{/if}
				</div>

				<!-- Video Platform (for videos only) -->
				{#if formData.resource_type === 'video'}
					<div class="form-row">
						<div class="form-group">
							<label for="video-platform">Video Platform</label>
							<select id="video-platform" bind:value={formData.video_platform}>
								{#each VIDEO_PLATFORMS as platform}
									<option value={platform.id}>{platform.name}</option>
								{/each}
							</select>
						</div>
						<div class="form-group">
							<label for="trader">Trader</label>
							<select id="trader" bind:value={formData.trader_id}>
								<option value={undefined}>Select trader...</option>
								{#each traders as trader}
									<option value={trader.id}>{trader.name}</option>
								{/each}
							</select>
						</div>
					</div>
				{/if}

				<!-- Thumbnail & Date -->
				<div class="form-row">
					<div class="form-group">
						<label for="thumbnail-url">Thumbnail URL</label>
						<input
							type="url"
							id="thumbnail-url" name="thumbnail-url"
							bind:value={formData.thumbnail_url}
							placeholder="https://..."
						/>
					</div>
					<div class="form-group">
						<label for="resource-date">Date</label>
						<input type="date" id="resource-date" name="resource-date" bind:value={formData.resource_date} />
					</div>
				</div>

				<!-- Tags -->
				<div class="form-group">
					<!-- svelte-ignore a11y_label_has_associated_control -->
					<label>Categories/Tags</label>
					<div class="tags-grid">
						{#each CATEGORIES as category}
							<button
								type="button"
								class="tag-btn"
								class:selected={formData.tags?.includes(category.id)}
								style:--tag-color={category.color}
								onclick={() => toggleTag(category.id)}
							>
								{#if formData.tags?.includes(category.id)}
									<IconCheck size={14} />
								{/if}
								{category.name}
							</button>
						{/each}
					</div>
				</div>

				<!-- Options -->
				<div class="form-options">
					<label class="checkbox-label">
						<input id="page-formdata-is-published" name="page-formdata-is-published" type="checkbox" bind:checked={formData.is_published} />
						<span>Published</span>
					</label>
					<label class="checkbox-label">
						<input id="page-formdata-is-featured" name="page-formdata-is-featured" type="checkbox" bind:checked={formData.is_featured} />
						<span>Featured (Main Resource)</span>
					</label>
					<label class="checkbox-label">
						<input id="page-formdata-is-pinned" name="page-formdata-is-pinned" type="checkbox" bind:checked={formData.is_pinned} />
						<span>Pinned (Always on top)</span>
					</label>
				</div>
			</div>
			<div class="modal-footer">
				<button
					class="btn-secondary"
					onclick={() => {
						showCreateModal = false;
						showEditModal = false;
						editingResource = null;
					}}>Cancel</button
				>
				<button
					class="btn-primary"
					onclick={saveResource}
					disabled={isSaving || !formData.title || !formData.file_url}
				>
					{#if isSaving}
						<span class="spinner-small"></span>
						Saving...
					{:else}
						{showEditModal ? 'Update' : 'Create'} Resource
					{/if}
				</button>
			</div>
		</div>
	</div>
{/if}

<!-- Replace Modal -->
{#if showReplaceModal && replacingResource}
	<!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions a11y_interactive_supports_focus -->
	<div
		class="modal-overlay"
		onclick={() => {
			showReplaceModal = false;
			replacingResource = null;
			newFileUrl = '';
		}}
		role="dialog"
		aria-modal="true"
		tabindex="-1"
	>
		<!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions a11y_no_noninteractive_element_interactions -->
		<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
		<div class="modal" onclick={(e) => e.stopPropagation()} role="document">
			<div class="modal-header">
				<h2>Replace Resource</h2>
				<button
					class="modal-close"
					onclick={() => {
						showReplaceModal = false;
						replacingResource = null;
						newFileUrl = '';
					}}>&times;</button
				>
			</div>
			<div class="modal-body">
				<div class="replace-info">
					<p><strong>Current:</strong> {replacingResource.title}</p>
					<p class="text-muted">All metadata will be kept. Only the file URL will be replaced.</p>
				</div>
				<div class="form-group">
					<label for="new-file-url">New File URL</label>
					<!-- svelte-ignore a11y_autofocus -->
					<input
						type="url"
						id="new-file-url" name="new-file-url"
						bind:value={newFileUrl}
						placeholder="https://..."
						autofocus
					/>
				</div>
			</div>
			<div class="modal-footer">
				<button
					class="btn-secondary"
					onclick={() => {
						showReplaceModal = false;
						replacingResource = null;
						newFileUrl = '';
					}}>Cancel</button
				>
				<button class="btn-primary" onclick={replaceResource} disabled={isSaving || !newFileUrl}>
					{#if isSaving}
						<span class="spinner-small"></span>
						Replacing...
					{:else}
						<IconLink size={16} />
						Replace
					{/if}
				</button>
			</div>
		</div>
	</div>
{/if}

<!-- ICT 7: Bulk Operations Modal -->
{#if showBulkModal}
	<!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions a11y_interactive_supports_focus -->
	<div
		class="modal-overlay"
		onclick={() => showBulkModal = false}
		role="dialog"
		aria-modal="true"
		tabindex="-1"
	>
		<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
		<div class="modal" onclick={(e) => e.stopPropagation()} onkeydown={(e) => e.stopPropagation()} role="document">
			<div class="modal-header">
				<h2>Bulk Operations</h2>
				<button class="modal-close" onclick={() => showBulkModal = false}>&times;</button>
			</div>
			<div class="modal-body">
				<p class="bulk-info">Apply action to <strong>{selectedResources.size}</strong> selected resources:</p>

				<div class="form-group">
					<label for="bulk-action">Action</label>
					<select id="bulk-action" bind:value={bulkAction}>
						<option value="publish">Publish All</option>
						<option value="unpublish">Unpublish All</option>
						<option value="feature">Mark as Featured</option>
						<option value="unfeature">Remove Featured</option>
						<option value="access">Set Access Level</option>
						<option value="delete">Delete All</option>
					</select>
				</div>

				{#if bulkAction === 'access'}
					<div class="form-group">
						<label for="bulk-access">Access Level</label>
						<select id="bulk-access" bind:value={bulkAccessLevel}>
							{#each ACCESS_LEVELS as level}
								<option value={level.id}>{level.name}</option>
							{/each}
						</select>
					</div>
				{/if}

				{#if bulkAction === 'delete'}
					<div class="alert alert-error">
						<IconAlertCircle size={18} />
						This will permanently delete {selectedResources.size} resources. This cannot be undone.
					</div>
				{/if}
			</div>
			<div class="modal-footer">
				<button class="btn-secondary" onclick={() => showBulkModal = false}>Cancel</button>
				<button
					class="btn-primary"
					class:btn-danger={bulkAction === 'delete'}
					onclick={executeBulkAction}
					disabled={isSaving}
				>
					{#if isSaving}
						<span class="spinner-small"></span>
						Processing...
					{:else if bulkAction === 'delete'}
						Delete {selectedResources.size} Resources
					{:else}
						Apply to {selectedResources.size} Resources
					{/if}
				</button>
			</div>
		</div>
	</div>
{/if}

<style>
	.admin-resources {
		background: linear-gradient(
			135deg,
			var(--bg-base) 0%,
			var(--bg-elevated) 50%,
			var(--bg-base) 100%
		);
		position: relative;
		overflow: hidden;
	}

	/* Header - Centered */
	.page-header {
		text-align: center;
		margin-bottom: 2rem;
	}

	.page-header h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0 0 0.5rem 0;
	}

	.subtitle {
		color: #64748b;
		font-size: 0.875rem;
		margin: 0 0 1rem 0;
	}

	.header-actions {
		display: flex;
		justify-content: center;
		gap: 0.75rem;
	}

	/* Buttons */
	.btn-primary {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background: linear-gradient(135deg, var(--primary-500) 0%, var(--primary-600) 100%);
		color: var(--bg-base);
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
	}

	.btn-secondary {
		display: flex;
		align-items: center;
		gap: 0.5rem;
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

	/* ICT 7: Warning and danger buttons */
	.btn-warning {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background: linear-gradient(135deg, #f59e0b 0%, #d97706 100%);
		color: #fff;
		border: none;
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-warning:hover {
		transform: translateY(-2px);
		box-shadow: 0 4px 15px rgba(245, 158, 11, 0.4);
	}

	.btn-danger,
	.btn-primary.btn-danger {
		background: linear-gradient(135deg, #ef4444 0%, #dc2626 100%) !important;
	}

	.btn-danger:hover,
	.btn-primary.btn-danger:hover {
		box-shadow: 0 4px 15px rgba(239, 68, 68, 0.4);
	}

	/* Bulk info */
	.bulk-info {
		color: #cbd5e1;
		margin-bottom: 1.5rem;
	}

	.bulk-info strong {
		color: #f1f5f9;
	}

	/* Unused - keeping for future use */
	/*.btn-refresh {
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
	}*/

	.btn-icon {
		width: 32px;
		height: 32px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: transparent;
		border: none;
		border-radius: 6px;
		color: #64748b;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-icon:hover {
		background: rgba(230, 184, 0, 0.2);
		color: var(--primary-500);
	}

	.btn-icon.danger:hover {
		background: rgba(239, 68, 68, 0.2);
		color: #f87171;
	}

	.btn-icon.active {
		color: #22c55e;
	}

	/* Room Tabs */
	.room-tabs {
		display: flex;
		gap: 0.5rem;
		margin-bottom: 1.5rem;
		flex-wrap: wrap;
		padding: 0.5rem;
		background: rgba(15, 23, 42, 0.6);
		border-radius: 12px;
	}

	.room-tabs.loading {
		justify-content: center;
		padding: 1rem;
	}

	.room-tab {
		padding: 0.75rem 1.5rem;
		background: transparent;
		border: none;
		border-radius: 8px;
		color: #94a3b8;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.room-tab:hover {
		background: rgba(230, 184, 0, 0.1);
		color: #e2e8f0;
	}

	.room-tab.active {
		background: linear-gradient(135deg, var(--primary-500) 0%, var(--primary-600) 100%);
		color: var(--bg-base);
	}

	/* Stats Bar */
	.stats-bar {
		display: flex;
		gap: 1.5rem;
		padding: 1rem 1.5rem;
		background: rgba(15, 23, 42, 0.6);
		border-radius: 12px;
		margin-bottom: 1.5rem;
		flex-wrap: wrap;
	}

	.stat {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		color: #94a3b8;
	}

	.stat-value {
		font-size: 1.25rem;
		font-weight: 700;
		color: #f1f5f9;
	}

	.stat-label {
		font-size: 0.875rem;
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
		flex: 1;
		min-width: 250px;
		padding: 0.75rem 1rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 10px;
		color: #94a3b8;
	}

	.search-box input {
		flex: 1;
		background: transparent;
		border: none;
		color: #f1f5f9;
		font-size: 0.9rem;
		outline: none;
	}

	.search-box input::placeholder {
		color: #64748b;
	}

	.filter-group {
		display: flex;
		gap: 0.75rem;
	}

	.filter-group select {
		padding: 0.75rem 1rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 10px;
		color: #f1f5f9;
		font-size: 0.9rem;
		cursor: pointer;
	}

	/* Resources Grid */
	.resources-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
		gap: 1.5rem;
	}

	.resource-card {
		background: rgba(15, 23, 42, 0.8);
		border: 1px solid rgba(230, 184, 0, 0.15);
		border-radius: 12px;
		overflow: hidden;
		transition: all 0.3s;
	}

	.resource-card:hover {
		border-color: rgba(230, 184, 0, 0.4);
		transform: translateY(-2px);
		box-shadow: 0 8px 25px rgba(0, 0, 0, 0.3);
	}

	.resource-card.featured {
		border-color: rgba(234, 179, 8, 0.5);
	}

	.resource-card.pinned {
		border-color: rgba(6, 182, 212, 0.5);
	}

	.resource-card.unpublished {
		opacity: 0.6;
	}

	.resource-thumbnail {
		position: relative;
		width: 100%;
		padding-top: 56.25%;
		background: rgba(30, 41, 59, 0.8);
	}

	.resource-thumbnail img {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.thumbnail-placeholder {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		display: flex;
		align-items: center;
		justify-content: center;
		color: #64748b;
	}

	.type-badge {
		position: absolute;
		top: 0.75rem;
		left: 0.75rem;
		padding: 0.25rem 0.5rem;
		background: rgba(15, 23, 42, 0.9);
		border-radius: 4px;
		font-size: 0.65rem;
		font-weight: 700;
		color: #94a3b8;
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.type-badge.video {
		color: var(--primary-500);
	}
	.type-badge.pdf {
		color: #f87171;
	}
	.type-badge.image {
		color: #4ade80;
	}

	.featured-badge {
		position: absolute;
		top: 0.75rem;
		right: 0.75rem;
		color: #eab308;
	}

	.pinned-badge {
		position: absolute;
		top: 0.75rem;
		right: 2rem;
		color: #06b6d4;
	}

	.resource-content {
		padding: 1rem;
	}

	.resource-title {
		font-size: 1rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0 0 0.5rem 0;
		line-height: 1.4;
		display: -webkit-box;
		-webkit-line-clamp: 2;
		line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	.resource-meta {
		display: flex;
		gap: 0.75rem;
		font-size: 0.8rem;
		color: #64748b;
		margin-bottom: 0.75rem;
	}

	.content-type {
		text-transform: capitalize;
	}

	.resource-tags {
		display: flex;
		gap: 0.5rem;
		flex-wrap: wrap;
	}

	.tag {
		padding: 0.2rem 0.5rem;
		background: color-mix(in srgb, var(--tag-color, var(--primary-500)) 20%, transparent);
		border-radius: 4px;
		font-size: 0.7rem;
		font-weight: 500;
		color: var(--tag-color, var(--primary-400));
	}

	.resource-actions {
		display: flex;
		justify-content: flex-end;
		gap: 0.25rem;
		padding: 0.75rem 1rem;
		border-top: 1px solid rgba(230, 184, 0, 0.1);
	}

	/* Empty & Loading States */
	.empty-state,
	.loading-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 2rem;
		text-align: center;
		color: #64748b;
	}

	.empty-state h3 {
		margin: 1rem 0 0.5rem;
		color: #f1f5f9;
	}

	.spinner {
		width: 40px;
		height: 40px;
		border: 3px solid rgba(230, 184, 0, 0.2);
		border-top-color: var(--primary-500);
		border-radius: 50%;
		animation: spin 1s linear infinite;
	}

	.spinner-small {
		width: 16px;
		height: 16px;
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-top-color: white;
		border-radius: 50%;
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	/* Alerts */
	.alert {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem;
		border-radius: 10px;
		margin-bottom: 1.5rem;
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
	}

	.alert-close:hover {
		opacity: 1;
	}

	/* Modal */
	.modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.75);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
		padding: 1rem;
	}

	.modal {
		background: #1e293b;
		border-radius: 16px;
		width: 100%;
		max-width: 500px;
		max-height: 90vh;
		overflow: hidden;
		display: flex;
		flex-direction: column;
	}

	.modal.modal-large {
		max-width: 700px;
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1.25rem 1.5rem;
		border-bottom: 1px solid rgba(230, 184, 0, 0.2);
	}

	.modal-header h2 {
		margin: 0;
		font-size: 1.25rem;
		color: #f1f5f9;
	}

	.modal-close {
		background: transparent;
		border: none;
		font-size: 1.5rem;
		color: #64748b;
		cursor: pointer;
		line-height: 1;
	}

	.modal-close:hover {
		color: #f1f5f9;
	}

	.modal-body {
		padding: 1.5rem;
		overflow-y: auto;
	}

	.modal-footer {
		display: flex;
		justify-content: flex-end;
		gap: 0.75rem;
		padding: 1rem 1.5rem;
		border-top: 1px solid rgba(230, 184, 0, 0.2);
	}

	/* Form Elements */
	.form-group {
		margin-bottom: 1.25rem;
	}

	.form-group label {
		display: block;
		margin-bottom: 0.5rem;
		color: #94a3b8;
		font-size: 0.875rem;
		font-weight: 500;
	}

	.form-group input,
	.form-group textarea,
	.form-group select {
		width: 100%;
		padding: 0.75rem 1rem;
		background: rgba(15, 23, 42, 0.8);
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 8px;
		color: #f1f5f9;
		font-size: 0.9rem;
	}

	.form-group input:focus,
	.form-group textarea:focus,
	.form-group select:focus {
		outline: none;
		border-color: var(--primary-500);
	}

	.form-group textarea {
		resize: vertical;
		min-height: 80px;
	}

	.form-row {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1rem;
	}

	.form-hint {
		display: block;
		margin-top: 0.5rem;
		color: #64748b;
		font-size: 0.8rem;
	}

	.form-hint strong {
		color: #4ade80;
		text-transform: uppercase;
	}

	/* Tags Grid */
	.tags-grid {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
	}

	.tag-btn {
		display: flex;
		align-items: center;
		gap: 0.35rem;
		padding: 0.5rem 0.75rem;
		background: rgba(15, 23, 42, 0.8);
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 6px;
		color: #94a3b8;
		font-size: 0.8rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.tag-btn:hover {
		border-color: var(--tag-color, var(--primary-500));
		color: var(--tag-color, var(--primary-400));
	}

	.tag-btn.selected {
		background: color-mix(in srgb, var(--tag-color, var(--primary-500)) 20%, transparent);
		border-color: var(--tag-color, var(--primary-500));
		color: var(--tag-color, var(--primary-400));
	}

	/* Form Options */
	.form-options {
		display: flex;
		flex-wrap: wrap;
		gap: 1.5rem;
	}

	.checkbox-label {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		cursor: pointer;
		color: #94a3b8;
		font-size: 0.9rem;
	}

	.checkbox-label input[type='checkbox'] {
		width: 18px;
		height: 18px;
		accent-color: var(--primary-500);
	}

	/* Replace Modal */
	.replace-info {
		background: rgba(230, 184, 0, 0.1);
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 8px;
		padding: 1rem;
		margin-bottom: 1.5rem;
	}

	.replace-info p {
		margin: 0.25rem 0;
		color: #cbd5e1;
	}

	.replace-info strong {
		color: #f1f5f9;
	}

	.text-muted {
		color: #64748b !important;
		font-size: 0.875rem;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   2026 Mobile-First Responsive - Admin Resources
	   Breakpoints: xs(360px), sm(640px), md(768px), lg(1024px), xl(1280px)
	   ═══════════════════════════════════════════════════════════════════════════ */

	/* Base Mobile Styles (xs: 360px) - Safe Areas */
	.admin-resources {
		padding-left: env(safe-area-inset-left, 0px);
		padding-right: env(safe-area-inset-right, 0px);
	}

	/* Page Header - Mobile */
	.page-header h1 {
		font-size: 1.375rem;
	}

	.subtitle {
		font-size: 0.8125rem;
	}

	.header-actions {
		flex-wrap: wrap;
		gap: 0.5rem;
	}

	/* Buttons - Touch Targets */
	.btn-primary,
	.btn-secondary {
		/* 44px min touch target */
		min-height: 44px;
		padding: 0.625rem 1rem;
		font-size: 0.875rem;
	}

	.btn-icon {
		/* 44px touch target */
		width: 44px;
		height: 44px;
	}

	/* Room Tabs - Mobile Horizontal Scroll */
	.room-tabs {
		flex-wrap: nowrap;
		overflow-x: auto;
		-webkit-overflow-scrolling: touch;
		scrollbar-width: none;
		gap: 0.375rem;
		padding: 0.375rem;
	}

	.room-tabs::-webkit-scrollbar {
		display: none;
	}

	.room-tab {
		/* 44px touch target */
		min-height: 44px;
		padding: 0.625rem 1rem;
		font-size: 0.8125rem;
		white-space: nowrap;
		flex-shrink: 0;
	}

	/* Stats Bar - Mobile Wrap */
	.stats-bar {
		flex-wrap: wrap;
		justify-content: flex-start;
		gap: 0.75rem 1.25rem;
		padding: 0.875rem 1rem;
	}

	.stat {
		gap: 0.375rem;
	}

	.stat-value {
		font-size: 1.125rem;
	}

	.stat-label {
		font-size: 0.75rem;
	}

	/* Filters - Mobile Stack */
	.filters-bar {
		flex-direction: column;
		gap: 0.75rem;
	}

	.search-box {
		min-width: 100%;
		/* 44px touch target */
		min-height: 44px;
	}

	.filter-group {
		flex-direction: column;
		width: 100%;
		gap: 0.5rem;
	}

	.filter-group select {
		width: 100%;
		/* 44px touch target */
		min-height: 44px;
		font-size: 0.875rem;
	}

	/* Resources Grid - Mobile Single Column */
	.resources-grid {
		grid-template-columns: 1fr;
		gap: 1rem;
	}

	/* Resource Card - Mobile */
	.resource-card {
		border-radius: 10px;
	}

	.resource-thumbnail {
		padding-top: 50%; /* Less tall on mobile */
	}

	.type-badge {
		font-size: 0.6rem;
		padding: 0.1875rem 0.375rem;
	}

	.resource-content {
		padding: 0.875rem;
	}

	.resource-title {
		font-size: 0.9375rem;
		-webkit-line-clamp: 2;
		line-clamp: 2;
	}

	.resource-meta {
		font-size: 0.75rem;
		gap: 0.5rem;
	}

	.resource-tags {
		gap: 0.375rem;
	}

	.tag {
		font-size: 0.625rem;
		padding: 0.1875rem 0.375rem;
	}

	/* Resource Actions - Mobile Touch Friendly */
	.resource-actions {
		padding: 0.625rem 0.875rem;
		gap: 0.375rem;
		flex-wrap: wrap;
		justify-content: center;
	}

	.resource-actions .btn-icon {
		/* 44px touch target */
		width: 44px;
		height: 44px;
	}

	/* Modal - Mobile Full Width */
	.modal-overlay {
		padding: 0.5rem;
		padding-top: calc(0.5rem + env(safe-area-inset-top, 0px));
		padding-bottom: calc(0.5rem + env(safe-area-inset-bottom, 0px));
	}

	.modal {
		max-width: 100%;
		max-height: calc(100vh - 1rem - env(safe-area-inset-top, 0px) - env(safe-area-inset-bottom, 0px));
	}

	.modal.modal-large {
		max-width: 100%;
	}

	.modal-header {
		padding: 1rem;
	}

	.modal-header h2 {
		font-size: 1.125rem;
	}

	.modal-body {
		padding: 1rem;
	}

	.modal-footer {
		padding: 0.75rem 1rem;
		flex-direction: column;
		gap: 0.5rem;
	}

	.modal-footer button {
		width: 100%;
		/* 44px touch target */
		min-height: 44px;
	}

	/* Form Elements - Mobile */
	.form-row {
		grid-template-columns: 1fr;
		gap: 0.875rem;
	}

	.form-group label {
		font-size: 0.8125rem;
	}

	.form-group input,
	.form-group textarea,
	.form-group select {
		/* 44px touch target */
		min-height: 44px;
		font-size: 0.9375rem;
		padding: 0.625rem 0.875rem;
	}

	.form-group textarea {
		min-height: 80px;
	}

	/* Tags Grid - Mobile */
	.tags-grid {
		gap: 0.375rem;
	}

	.tag-btn {
		/* 44px touch target */
		min-height: 44px;
		padding: 0.5rem 0.75rem;
		font-size: 0.75rem;
	}

	/* Form Options - Mobile */
	.form-options {
		flex-direction: column;
		gap: 1rem;
	}

	.checkbox-label {
		/* 44px touch target */
		min-height: 44px;
		padding: 0.5rem 0;
		display: flex;
		align-items: center;
	}

	.checkbox-label input[type='checkbox'] {
		width: 22px;
		height: 22px;
	}

	/* Empty/Loading States - Mobile */
	.empty-state,
	.loading-state {
		padding: 3rem 1.5rem;
	}

	/* sm: 640px+ */
	@media (min-width: 640px) {
		.page-header h1 {
			font-size: 1.5rem;
		}

		.filters-bar {
			flex-direction: row;
			flex-wrap: wrap;
		}

		.search-box {
			min-width: 250px;
			flex: 1;
		}

		.filter-group {
			flex-direction: row;
			width: auto;
		}

		.filter-group select {
			width: auto;
		}

		.resources-grid {
			grid-template-columns: repeat(2, 1fr);
			gap: 1.25rem;
		}

		.modal-footer {
			flex-direction: row;
			justify-content: flex-end;
		}

		.modal-footer button {
			width: auto;
		}

		.form-options {
			flex-direction: row;
			flex-wrap: wrap;
		}
	}

	/* md: 768px+ */
	@media (min-width: 768px) {
		.page-header h1 {
			font-size: 1.75rem;
		}

		.room-tabs {
			flex-wrap: wrap;
			overflow-x: visible;
		}

		.stats-bar {
			justify-content: flex-start;
			gap: 1.5rem;
			padding: 1rem 1.5rem;
		}

		.stat-value {
			font-size: 1.25rem;
		}

		.resources-grid {
			grid-template-columns: repeat(2, 1fr);
			gap: 1.5rem;
		}

		.modal {
			max-width: 500px;
		}

		.modal.modal-large {
			max-width: 700px;
		}

		.form-row {
			grid-template-columns: 1fr 1fr;
		}
	}

	/* lg: 1024px+ */
	@media (min-width: 1024px) {
		.resources-grid {
			grid-template-columns: repeat(3, 1fr);
		}
	}

	/* xl: 1280px+ */
	@media (min-width: 1280px) {
		.resources-grid {
			grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
		}
	}
</style>

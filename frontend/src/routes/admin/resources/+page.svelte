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
	// FIX-2026-04-26 (CLAUDE.md): init/cleanup belongs in onMount, not $effect.
	import { onMount, type Component, SvelteSet } from 'svelte';
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
	import ConfirmationModal from '$lib/components/admin/ConfirmationModal.svelte';
	import AlertsBanner from './_components/AlertsBanner.svelte';
	import PageHeader from './_components/PageHeader.svelte';
	import RoomTabs from './_components/RoomTabs.svelte';
	import StatsBar from './_components/StatsBar.svelte';
	import ReplaceModal from './_components/ReplaceModal.svelte';
	import BulkOperationsModal from './_components/BulkOperationsModal.svelte';
	import ResourceFormModal from './_components/ResourceFormModal.svelte';

	// ═══════════════════════════════════════════════════════════════════════════
	// TYPES & CONSTANTS
	// ═══════════════════════════════════════════════════════════════════════════

	const RESOURCE_TYPES: {
		id: ResourceType;
		name: string;
		icon: Component<{ size?: number | string }>;
	}[] = [
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
	let selectedResources = $state<SvelteSet<number>>(new SvelteSet());
	let bulkAction = $state<'publish' | 'unpublish' | 'feature' | 'unfeature' | 'access' | 'delete'>(
		'publish'
	);
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

	// ICT 7: All 6 trading rooms in correct order.
	// TODO(2026-04-26-audit P2-11): retained for offline-dev; not used by the
	// runtime path anymore. Underscore prefix suppresses unused-var lint while
	// keeping the data here for the planned dev-only flag.
	const _FALLBACK_ROOMS: TradingRoom[] = [
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

		// Extract rooms.
		// FIX-2026-04-26 (P2-11): when the rooms API truly fails, surface a clear
		// error and render an empty state rather than masquerading FALLBACK_ROOMS
		// as truth — the production DB may have different rooms entirely.
		const roomsResult = results[0];
		if (roomsResult.status === 'fulfilled' && roomsResult.value?.data) {
			const data = roomsResult.value.data;
			if (Array.isArray(data) && data.length > 0) {
				rooms = data;
			} else {
				rooms = [];
				error = 'No rooms returned from backend.';
			}
		} else {
			// TODO(2026-04-26-audit): keep FALLBACK_ROOMS available behind a flag
			// for offline dev only; production must show the empty state.
			rooms = [];
			error = 'Could not load trading rooms — please retry or contact support.';
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
		} catch (err) {
			// FIX-2026-04-26 (P2-8): surface backend failures so admins can tell a
			// genuine empty room from a 5xx. Previously every error masqueraded
			// as "no content found".
			resources = [];
			error =
				err instanceof Error
					? `Failed to load resources: ${err.message}`
					: 'Failed to load resources';
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
						// FIX-2026-04-26 (P3-9): drop legacy clear+self-reassign hack;
						// reassign with a fresh empty Set is equivalent and clearer.
						selectedResources.clear();
						await loadResources();
					}
					showBulkModal = false;
					isSaving = false;
					return;
			}

			const result = await bulkUpdateResources(ids, updates);
			if (result.success) {
				showSuccess(`Updated ${result.updated_count} resources`);
				// FIX-2026-04-26 (P3-9): clear+self-reassign is dead in Svelte 5;
				// just clear the SvelteSet.
				selectedResources.clear();
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

	// Delete confirmation modal state
	let showDeleteModal = $state(false);
	let pendingDeleteResource = $state<RoomResource | null>(null);

	function deleteResource(resource: RoomResource) {
		pendingDeleteResource = resource;
		showDeleteModal = true;
	}

	async function confirmDeleteResource() {
		if (!pendingDeleteResource) return;
		showDeleteModal = false;
		const resource = pendingDeleteResource;
		pendingDeleteResource = null;

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

	// FIX-2026-04-26 (CLAUDE.md / P1-7): init in onMount and remove the trailing
	// $effect that re-fired loadResources whenever selectedRoom changed —
	// selectRoom() already calls loadResources() explicitly. The previous
	// double-effect dance produced two in-flight requests per click.
	onMount(() => {
		if (!browser) return;
		void (async () => {
			await loadRoomsAndTraders();
			if (selectedRoom) {
				await loadResources();
			}
		})();
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
		<AlertsBanner {successMessage} {error} onClearError={() => (error = '')} />

		<!-- Header -->
		<PageHeader
			selectedCount={selectedResources.size}
			{isLoading}
			hasSelectedRoom={!!selectedRoom}
			onBulk={openBulkModal}
			onRefresh={() => loadResources()}
			onCreate={openCreateModal}
		/>

		<!-- Room Tabs -->
		<RoomTabs {rooms} {selectedRoom} {isLoadingRooms} onSelect={selectRoom} />

		<!-- Stats Bar -->
		{#if selectedRoom}
			<StatsBar {stats} />
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
					{#each RESOURCE_TYPES as type (type.id)}
						<option value={type.id}>{type.name}</option>
					{/each}
				</select>
				<select bind:value={selectedContentType}>
					<option value="all">All Content</option>
					{#each CONTENT_TYPES as type (type.id)}
						<option value={type.id}>{type.name}</option>
					{/each}
				</select>
				<!-- ICT 7: Section filter -->
				<select bind:value={selectedSection}>
					<option value="all">All Sections</option>
					{#each availableSections as section (section.id)}
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
				{#each filteredResources as resource (resource.id)}
					<div
						class="resource-card"
						class:featured={resource.is_featured}
						class:pinned={resource.is_pinned}
						class:unpublished={!resource.is_published}
					>
						<!-- Thumbnail -->
						<div class="resource-thumbnail">
							{#if resource.thumbnail_url}
								<img
									src={resource.thumbnail_url}
									alt={resource.title}
									width="320"
									height="180"
									loading="lazy"
								/>
							{:else}
								{@const ResourceIcon = getResourceIcon(resource.resource_type)}
								<div class="thumbnail-placeholder">
									<ResourceIcon size={32} />
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
									{#each (resource.tags || []).slice(0, 3) as tagId (tagId)}
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
	<ResourceFormModal
		isEdit={showEditModal}
		bind:formData
		resourceTypes={RESOURCE_TYPES}
		{availableContentTypes}
		{availableSections}
		accessLevels={ACCESS_LEVELS}
		videoPlatforms={VIDEO_PLATFORMS}
		categories={CATEGORIES}
		{traders}
		{isSaving}
		onClose={() => {
			showCreateModal = false;
			showEditModal = false;
			editingResource = null;
		}}
		onSave={saveResource}
		onToggleTag={toggleTag}
	/>
{/if}

<!-- Replace Modal -->
{#if showReplaceModal && replacingResource}
	<ReplaceModal
		resource={replacingResource}
		bind:newFileUrl
		{isSaving}
		onClose={() => {
			showReplaceModal = false;
			replacingResource = null;
			newFileUrl = '';
		}}
		onReplace={replaceResource}
	/>
{/if}

<!-- ICT 7: Bulk Operations Modal -->
{#if showBulkModal}
	<BulkOperationsModal
		selectedCount={selectedResources.size}
		bind:bulkAction
		bind:bulkAccessLevel
		accessLevels={ACCESS_LEVELS}
		{isSaving}
		onClose={() => (showBulkModal = false)}
		onConfirm={executeBulkAction}
	/>
{/if}

<ConfirmationModal
	isOpen={showDeleteModal}
	title="Delete Resource"
	message={pendingDeleteResource
		? `Delete "${pendingDeleteResource.title}"? This action cannot be undone.`
		: ''}
	confirmText="Delete"
	variant="danger"
	onConfirm={confirmDeleteResource}
	onCancel={() => {
		showDeleteModal = false;
		pendingDeleteResource = null;
	}}
/>

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

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
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

	/* Buttons - Touch Targets */
	.btn-primary {
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

	/* Empty/Loading States - Mobile */
	.empty-state,
	.loading-state {
		padding: 3rem 1.5rem;
	}

	/* sm: 640px+ */
	@media (min-width: 640px) {
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
	}

	/* md: 768px+ */
	@media (min-width: 768px) {
		.resources-grid {
			grid-template-columns: repeat(2, 1fr);
			gap: 1.5rem;
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

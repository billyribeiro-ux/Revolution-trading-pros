<script lang="ts">
	import { browser } from '$app/environment';
	import { goto } from '$app/navigation';
	// FIX-2026-04-26 (CLAUDE.md): init/cleanup belongs in onMount, not $effect.
	import { onMount } from 'svelte';
	import { slide, scale } from 'svelte/transition';
	import { adminFetch } from '$lib/utils/adminFetch';
	import {
		IconPlus,
		IconSearch,
		IconFilter,
		IconEdit,
		IconTrash,
		IconEye,
		IconCopy,
		IconCalendar,
		IconUser,
		IconClock,
		IconChartBar,
		IconDownload,
		IconUpload,
		IconList,
		IconTrendingUp,
		IconExternalLink,
		IconPlayerPlay,
		IconPlayerPause,
		IconStar,
		IconStarFilled,
		IconSortAscending,
		IconSortDescending,
		IconRefresh,
		IconMenu2,
		IconLayoutGrid
	} from '$lib/icons';
	import ConfirmationModal from '$lib/components/admin/ConfirmationModal.svelte';
	import { logger } from '$lib/utils/logger';
	import { predefinedCategories, type BlogCategory } from '$lib/data/predefined-categories';
	import PreviewModal from './_components/PreviewModal.svelte';
	import ExportModal from './_components/ExportModal.svelte';
	import AnalyticsModal from './_components/AnalyticsModal.svelte';
	import NotificationsList from './_components/NotificationsList.svelte';
	import KeyboardShortcutsHelper from './_components/KeyboardShortcutsHelper.svelte';
	import StatsGrid from './_components/StatsGrid.svelte';

	// ═══════════════════════════════════════════════════════════════════════════
	// State Management
	// ═══════════════════════════════════════════════════════════════════════════

	// Admin list-view post shape: the persisted post plus admin-only analytics
	// (seo_score/engagement_rate) and the row-selection flag. Index signature
	// covers the raw `...post` API spread and the dynamic updatePostMetric write.
	type AdminPostCategory = string | { id?: string; name?: string; color?: string };
	interface AdminBlogPost {
		id: number;
		title: string;
		slug: string;
		excerpt?: string | null;
		status?: string;
		featured?: boolean;
		featured_image?: string | null;
		author?: { name?: string } | null;
		categories?: AdminPostCategory[];
		tags?: unknown[];
		comments?: number;
		likes?: number;
		shares?: number;
		view_count?: number;
		word_count?: number;
		avg_read_time?: number;
		bounce_rate?: number;
		engagement_rate?: number;
		seo_score?: number;
		meta_description?: string | null;
		publish_at?: string | null;
		published_at?: string | null;
		selected?: boolean;
	}

	interface AppNotification {
		id: string;
		type: 'success' | 'error' | 'warning' | 'info';
		message: string;
	}

	let posts = $state<AdminBlogPost[]>([]);
	let stats = $state<import('svelte').ComponentProps<typeof StatsGrid>['stats']>(null);
	let loading = $state(false);
	let searchQuery = $state('');
	let statusFilter = $state('all');
	let categoryFilter = $state('all');

	// New state for improvements
	let selectedPosts = $state(new Set<number>());
	let selectAll = $state(false);
	let viewMode = $state<'grid' | 'list'>('grid');
	let sortBy = $state('created_at');
	let sortOrder = $state<'asc' | 'desc'>('desc');
	let dateRange = $state({ start: '', end: '' });
	let previewPost = $state<import('svelte').ComponentProps<typeof PreviewModal>['post']>(null);
	let activeActionMenu = $state<number | null>(null);
	let ws = $state<WebSocket | null>(null);
	let showExportModal = $state(false);
	let exportFormat = $state<'csv' | 'json' | 'wordpress'>('csv');
	// Schedule modal state removed 2026-04-27 with the modal itself; see TODO marker below.
	let showAnalyticsModal = $state(false);
	let analyticsPost = $state<import('svelte').ComponentProps<typeof AnalyticsModal>['post']>(null);
	let refreshInterval = $state<ReturnType<typeof setInterval> | undefined>(undefined);
	let notifications = $state<AppNotification[]>([]);

	// Delete confirmation modal state
	let showDeleteModal = $state(false);
	let showBulkDeleteModal = $state(false);
	let pendingDeleteId = $state<number | null>(null);

	const statusOptions = [
		{ value: 'all', label: 'All Status' },
		{ value: 'published', label: 'Published' },
		{ value: 'draft', label: 'Draft' },
		{ value: 'scheduled', label: 'Scheduled' },
		{ value: 'archived', label: 'Archived' }
	];

	const sortOptions = [
		{ value: 'created_at', label: 'Date Created' },
		{ value: 'published_at', label: 'Date Published' },
		{ value: 'view_count', label: 'Most Viewed' },
		{ value: 'engagement_rate', label: 'Engagement' },
		{ value: 'seo_score', label: 'SEO Score' },
		{ value: 'title', label: 'Alphabetical' }
	];

	// Helper function to get category by ID
	function getPredefinedCategoryById(id: string): BlogCategory | undefined {
		return predefinedCategories.find((cat) => cat.id === id);
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Lifecycle
	// ═══════════════════════════════════════════════════════════════════════════

	// FIX-2026-04-26 (CLAUDE.md / P1-8 / P2-7): init in onMount, not $effect.
	// Adds visibility gate so polling doesn't hammer expired sessions in
	// background tabs, plus an AbortController-style poll-stop on 401.
	let pollAuthBlocked = $state(false);
	onMount(() => {
		if (!browser) return;

		loadPosts();
		loadStats();
		setupWebSocket();
		setupKeyboardShortcuts();

		refreshInterval = setInterval(() => {
			// FIX-2026-04-26 (P1-8): only poll when tab is visible AND a previous
			// load hasn't disabled polling due to auth failure.
			if (pollAuthBlocked) return;
			if (typeof document !== 'undefined' && document.visibilityState !== 'visible') return;
			loadStats();
			if (viewMode === 'list') loadPosts();
		}, 30000);

		return () => {
			if (ws) ws.close();
			if (refreshInterval) clearInterval(refreshInterval);
			document.removeEventListener('keydown', handleKeyboard);
		};
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// Data Loading
	// ═══════════════════════════════════════════════════════════════════════════

	async function loadPosts() {
		loading = true;
		try {
			const params = new URLSearchParams();
			if (statusFilter !== 'all') params.append('status', statusFilter);
			if (categoryFilter !== 'all') params.append('category', categoryFilter);
			if (searchQuery) params.append('search', searchQuery);
			if (sortBy) params.append('sort', sortBy);
			if (sortOrder) params.append('order', sortOrder);
			if (dateRange.start) params.append('date_from', dateRange.start);
			if (dateRange.end) params.append('date_to', dateRange.end);

			const data = await adminFetch(`/api/admin/posts?${params}`);

			// Enhance posts with additional data
			posts = (data.data || []).map((post: AdminBlogPost) => ({
				...post,
				seo_score: calculateSeoScore(post),
				engagement_rate: calculateEngagementRate(post),
				selected: selectedPosts.has(post.id)
			}));
		} catch (error) {
			const err = error as { status?: number; message?: string };
			logger.error('Failed to load posts', { error });
			// FIX-2026-04-26 (P1-8): on 401, stop polling so we don't blast the
			// expired session every 30s.
			if (err?.status === 401 || /401|Unauthorized/i.test(String(err?.message || ''))) {
				pollAuthBlocked = true;
				showNotification('error', 'Session expired — please re-login');
			} else {
				showNotification('error', 'Failed to load posts');
			}
		} finally {
			loading = false;
		}
	}

	async function loadStats() {
		try {
			stats = await adminFetch('/api/admin/posts/stats');
		} catch (error) {
			logger.error('Failed to load stats', { error });
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// WebSocket & Real-time Updates
	// ═══════════════════════════════════════════════════════════════════════════

	function setupWebSocket() {
		// ICT 11+ FIX: Only attempt WebSocket if VITE_WS_URL is explicitly configured
		// Fly.io doesn't support WebSockets by default - this is optional functionality
		// FIX-2026-04-26 (P3-6): use dot-property form so Vite static replaces.
		const configuredWsUrl = import.meta.env.VITE_WS_URL;
		if (!configuredWsUrl) {
			// Silently skip - WebSocket is optional, use polling instead
			return;
		}

		try {
			ws = new WebSocket(`${configuredWsUrl}/posts`);

			ws.onmessage = (event) => {
				const update = JSON.parse(event.data);

				switch (update.type) {
					case 'view_count':
						updatePostMetric(update.postId, 'view_count', update.count);
						break;
					case 'engagement':
						updatePostMetric(update.postId, 'engagement_rate', update.rate);
						break;
					case 'status_change':
						updatePostStatus(update.postId, update.status);
						break;
					case 'new_post':
						loadPosts();
						loadStats();
						showNotification('info', 'New post created');
						break;
				}
			};

			ws.onerror = () => {
				// Silently handle - WebSocket is optional
			};
		} catch {
			// Silently handle - WebSocket is optional
		}
	}

	function updatePostMetric(postId: number, metric: string, value: unknown) {
		// FIX-2026-04-26 (P2-2): drop `posts = posts;` self-assignment — array
		// element mutation is reactive in Svelte 5 runes.
		const post = posts.find((p) => p.id === postId);
		if (post) {
			(post as unknown as Record<string, unknown>)[metric] = value;
		}
	}

	function updatePostStatus(postId: number, status: string) {
		// FIX-2026-04-26 (P2-2): drop `posts = posts;` self-assignment.
		const post = posts.find((p) => p.id === postId);
		if (post) {
			post.status = status;
			loadStats();
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Bulk Operations
	// ═══════════════════════════════════════════════════════════════════════════

	function toggleSelectAll() {
		selectAll = !selectAll;
		if (selectAll) {
			selectedPosts = new Set(posts.map((p) => p.id));
		} else {
			selectedPosts.clear();
		}
		posts = posts.map((p) => ({ ...p, selected: selectAll }));
	}

	function togglePostSelection(postId: number) {
		// FIX-2026-04-26 (P2-2): drop self-assignment hacks; Set/array mutations
		// are reactive in Svelte 5 runes. Reassigning the Set with new Set keeps
		// downstream `$derived` consumers consistent.
		if (selectedPosts.has(postId)) {
			selectedPosts.delete(postId);
		} else {
			selectedPosts.add(postId);
		}
		// Note:  mutations are reactive, no need to reassign

		const post = posts.find((p) => p.id === postId);
		if (post) post.selected = !post.selected;

		// FIX-2026-04-26 (P1-2): guard against the empty-list case where 0 === 0
		// would otherwise flip selectAll true with nothing actually selected.
		selectAll = posts.length > 0 && selectedPosts.size === posts.length;
	}

	function bulkDelete() {
		if (selectedPosts.size === 0) {
			showNotification('warning', 'No posts selected');
			return;
		}
		showBulkDeleteModal = true;
	}

	async function confirmBulkDelete() {
		showBulkDeleteModal = false;
		const count = selectedPosts.size;
		try {
			await adminFetch('/api/admin/posts/bulk-delete', {
				method: 'POST',
				body: JSON.stringify({ ids: [...selectedPosts] })
			});
			loadPosts();
			loadStats();
			selectedPosts.clear();
			selectAll = false;
			showNotification('success', `Deleted ${count} posts`);
		} catch (error) {
			logger.error('Failed to bulk delete', { error });
			showNotification('error', 'Failed to delete posts');
		}
	}

	async function bulkChangeStatus(newStatus: string) {
		if (selectedPosts.size === 0) {
			showNotification('warning', 'No posts selected');
			return;
		}

		// FIX-2026-04-26 (P1-3): capture the count BEFORE clearing the set; the
		// previous code read it after .clear() and always reported "Updated 0".
		const count = selectedPosts.size;
		try {
			await adminFetch('/api/admin/posts/bulk-status', {
				method: 'POST',
				body: JSON.stringify({ ids: [...selectedPosts], status: newStatus })
			});
			loadPosts();
			loadStats();
			selectedPosts.clear();
			selectAll = false;
			showNotification('success', `Updated ${count} posts to ${newStatus}`);
		} catch (error) {
			logger.error('Failed to bulk update status', { error });
			showNotification('error', 'Failed to update posts');
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Individual Post Actions
	// ═══════════════════════════════════════════════════════════════════════════

	function deletePost(id: number) {
		pendingDeleteId = id;
		showDeleteModal = true;
	}

	async function confirmDeletePost() {
		if (pendingDeleteId === null) return;
		showDeleteModal = false;
		const id = pendingDeleteId;
		pendingDeleteId = null;
		try {
			await adminFetch(`/api/admin/posts/${id}`, { method: 'DELETE' });
			loadPosts();
			loadStats();
			showNotification('success', 'Post deleted');
		} catch (error) {
			logger.error('Failed to delete post', { error });
			showNotification('error', 'Failed to delete post');
		}
	}

	async function duplicatePost(id: number) {
		try {
			await adminFetch(`/api/admin/posts/${id}/duplicate`, { method: 'POST' });
			loadPosts();
			showNotification('success', 'Post duplicated');
		} catch (error) {
			logger.error('Failed to duplicate post', { error });
			showNotification('error', 'Failed to duplicate post');
		}
	}

	async function toggleStatus(post: AdminBlogPost) {
		const newStatus = post.status === 'published' ? 'draft' : 'published';

		try {
			await adminFetch(`/api/admin/posts/${post.id}/status`, {
				method: 'PATCH',
				body: JSON.stringify({ status: newStatus })
			});
			// FIX-2026-04-26 (P2-2): drop self-assignment hack.
			post.status = newStatus;
			loadStats();
			showNotification('success', `Post ${newStatus}`);
		} catch (error) {
			logger.error('Failed to toggle status', { error });
			showNotification('error', 'Failed to update status');
		}
	}

	async function toggleFeatured(post: AdminBlogPost) {
		try {
			await adminFetch(`/api/admin/posts/${post.id}/featured`, {
				method: 'PATCH',
				body: JSON.stringify({ featured: !post.featured })
			});
			// FIX-2026-04-26 (P2-2): drop self-assignment hack.
			post.featured = !post.featured;
			showNotification('success', post.featured ? 'Post featured' : 'Post unfeatured');
		} catch (error) {
			logger.error('Failed to toggle featured', { error });
			showNotification('error', 'Failed to update featured status');
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Export & Import
	// ═══════════════════════════════════════════════════════════════════════════

	async function exportPosts() {
		try {
			const params = new URLSearchParams();
			params.append('format', exportFormat);
			if (selectedPosts.size > 0) {
				params.append('ids', [...selectedPosts].join(','));
			}

			const response = await adminFetch<Response>(`/api/admin/posts/export?${params}`, {
				rawResponse: true
			});
			const blob = await response.blob();

			const a = document.createElement('a');
			a.href = URL.createObjectURL(blob);
			a.download = `posts-export-${new Date().toISOString().split('T')[0]}.${exportFormat}`;
			a.click();

			showExportModal = false;
			showNotification('success', 'Posts exported successfully');
		} catch (error) {
			logger.error('Failed to export posts', { error });
			showNotification('error', 'Failed to export posts');
		}
	}

	async function importPosts(event: Event) {
		const input = event.target as HTMLInputElement;
		const file = input.files?.[0];
		if (!file) return;

		const formData = new FormData();
		formData.append('file', file);

		try {
			await adminFetch('/api/admin/posts/import', {
				method: 'POST',
				body: formData
			});
			loadPosts();
			loadStats();
			showNotification('success', 'Posts imported successfully');
		} catch (error) {
			logger.error('Failed to import posts', { error });
			showNotification('error', 'Failed to import posts');
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Analytics & SEO
	// ═══════════════════════════════════════════════════════════════════════════

	function calculateSeoScore(post: AdminBlogPost): number {
		let score = 0;

		// Title length (60-70 chars optimal)
		const titleLen = post.title?.length ?? 0;
		if (titleLen >= 30 && titleLen <= 70) score += 20;
		else if (titleLen > 0) score += 10;

		// Meta description
		const metaLen = post.meta_description?.length ?? 0;
		if (metaLen >= 120 && metaLen <= 160) score += 20;
		else if (metaLen > 0) score += 10;

		// Featured image
		if (post.featured_image) score += 15;

		// Categories and tags
		if ((post.categories?.length ?? 0) > 0) score += 10;
		if ((post.tags?.length ?? 0) > 0) score += 10;

		// URL slug
		if (post.slug && post.slug.length < 60) score += 10;

		// Content length (min 300 words assumed)
		const wordCount = post.word_count ?? 0;
		if (wordCount >= 1500) score += 15;
		else if (wordCount >= 500) score += 10;
		else if (wordCount >= 300) score += 5;

		return Math.min(score, 100);
	}

	function calculateEngagementRate(post: AdminBlogPost): number {
		const views = post.view_count ?? 0;
		if (views === 0) return 0;

		const interactions = (post.likes || 0) + (post.comments || 0) + (post.shares || 0);
		return Math.round((interactions / views) * 100);
	}

	async function loadPostAnalytics(post: AdminBlogPost) {
		analyticsPost = post;
		showAnalyticsModal = true;

		try {
			const data = await adminFetch(`/api/admin/posts/${post.id}/analytics`);
			analyticsPost = { ...analyticsPost, analytics: data };
		} catch (error) {
			logger.error('Failed to load analytics', { error });
			showNotification('error', 'Failed to load analytics');
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Keyboard Shortcuts
	// ═══════════════════════════════════════════════════════════════════════════

	function setupKeyboardShortcuts() {
		document.addEventListener('keydown', handleKeyboard);
	}

	function handleKeyboard(e: KeyboardEvent) {
		// New post (Ctrl/Cmd + N)
		if ((e.ctrlKey || e.metaKey) && e.key === 'n') {
			e.preventDefault();
			goto('/admin/blog/create');
		}

		// Search (Ctrl/Cmd + F)
		if ((e.ctrlKey || e.metaKey) && e.key === 'f') {
			e.preventDefault();
			document.querySelector<HTMLInputElement>('.search-box input')?.focus();
		}

		// Select all (Ctrl/Cmd + A)
		if (
			(e.ctrlKey || e.metaKey) &&
			e.key === 'a' &&
			!(e.target instanceof HTMLInputElement || e.target instanceof HTMLTextAreaElement)
		) {
			e.preventDefault();
			toggleSelectAll();
		}

		// Delete selected (Delete key)
		if (
			e.key === 'Delete' &&
			selectedPosts.size > 0 &&
			!(e.target instanceof HTMLInputElement || e.target instanceof HTMLTextAreaElement)
		) {
			e.preventDefault();
			bulkDelete();
		}

		// Toggle view (V key)
		if (
			e.key === 'v' &&
			!(e.target instanceof HTMLInputElement || e.target instanceof HTMLTextAreaElement)
		) {
			e.preventDefault();
			viewMode = viewMode === 'grid' ? 'list' : 'grid';
		}

		// Refresh (R key)
		if (
			e.key === 'r' &&
			!(e.target instanceof HTMLInputElement || e.target instanceof HTMLTextAreaElement)
		) {
			e.preventDefault();
			loadPosts();
			loadStats();
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Utility Functions
	// ═══════════════════════════════════════════════════════════════════════════

	function formatDate(dateString: string) {
		return new Date(dateString).toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'short',
			day: 'numeric'
		});
	}

	function formatDateTime(dateString: string) {
		return new Date(dateString).toLocaleString('en-US', {
			year: 'numeric',
			month: 'short',
			day: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	function formatNumber(num: number): string {
		if (num >= 1000000) return (num / 1000000).toFixed(1) + 'M';
		if (num >= 1000) return (num / 1000).toFixed(1) + 'K';
		return num.toString();
	}

	function getSeoScoreColor(score: number): string {
		if (score >= 80) return 'var(--admin-success, #10b981)';
		if (score >= 60) return 'var(--admin-warning, #f59e0b)';
		return 'var(--admin-error, #ef4444)';
	}

	function showNotification(type: 'success' | 'error' | 'warning' | 'info', message: string) {
		// FIX-2026-04-26 (P3-3): Date.now() collides when two notifications fire
		// in the same ms. Use crypto.randomUUID() for guaranteed uniqueness.
		const id =
			typeof crypto !== 'undefined' && 'randomUUID' in crypto
				? crypto.randomUUID()
				: `${Date.now()}-${Math.random()}`;
		notifications = [...notifications, { id, type, message }];

		setTimeout(() => {
			notifications = notifications.filter((n) => n.id !== id);
		}, 5000);
	}

	function toggleActionMenu(postId: number) {
		activeActionMenu = activeActionMenu === postId ? null : postId;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Reactive Statements (Svelte 5 Runes)
	// ═══════════════════════════════════════════════════════════════════════════

	// Derived state for bulk actions visibility
	const showBulkActions = $derived(selectedPosts.size > 0);

	// Track initial mount to prevent effect from running on first load
	let isInitialMount = $state(true);
	let filterDebounceTimer: ReturnType<typeof setTimeout> | undefined;

	// ICT11+ Fix: Debounced effect to reload posts when filters change
	$effect(() => {
		// Track all filter dependencies (void to suppress unused warnings)
		void dateRange.start;
		void dateRange.end;

		if (isInitialMount) {
			isInitialMount = false;
			return;
		}

		// Debounce filter changes to prevent rapid API calls
		clearTimeout(filterDebounceTimer);
		filterDebounceTimer = setTimeout(() => {
			loadPosts();
		}, 300);

		return () => {
			clearTimeout(filterDebounceTimer);
		};
	});
</script>

<svelte:head>
	<title>Blog Posts | Enterprise Admin</title>
</svelte:head>

<div class="admin-blog">
	<!-- Animated Background -->
	<div class="bg-effects">
		<div class="bg-blob bg-blob-1"></div>
		<div class="bg-blob bg-blob-2"></div>
		<div class="bg-blob bg-blob-3"></div>
	</div>

	<div class="admin-page-container">
		<!-- Notifications -->
		<NotificationsList {notifications} />

		<!-- Header -->
		<header class="page-header">
			<h1>Blog Posts</h1>
			<p class="subtitle">
				Create and manage blog posts • {posts.length} posts • Last updated: {new Date().toLocaleTimeString()}
			</p>
			<div class="header-actions">
				<button
					class="btn-icon"
					onclick={() => {
						loadPosts();
						loadStats();
					}}
					title="Refresh (R)"
				>
					<IconRefresh size={18} />
				</button>
				<button
					class="btn-primary"
					onclick={() => goto('/admin/blog/create')}
					title="New Post (Ctrl+N)"
				>
					<IconPlus size={18} />
					New Post
				</button>
			</div>
		</header>

		<!-- Stats -->
		<StatsGrid {stats} {formatNumber} />

		<!-- Controls Bar -->
		<div class="controls-bar">
			<div class="controls-left">
				{#if showBulkActions}
					<div class="bulk-actions" transition:slide={{ duration: 200 }}>
						<span class="bulk-count">{selectedPosts.size} selected</span>
						<button class="btn-secondary" onclick={() => bulkChangeStatus('published')}>
							Publish
						</button>
						<button class="btn-secondary" onclick={() => bulkChangeStatus('draft')}> Draft </button>
						<button class="btn-secondary danger" onclick={bulkDelete}>
							<IconTrash size={18} />
							Delete
						</button>
						<button
							class="btn-ghost"
							onclick={() => {
								selectedPosts.clear();
								selectAll = false;
							}}
						>
							Cancel
						</button>
					</div>
				{:else}
					<div class="search-box">
						<IconSearch size={20} />
						<input
							id="blog-search"
							name="blog-search"
							type="text"
							bind:value={searchQuery}
							placeholder="Search posts... (Ctrl+F)"
						/>
					</div>
				{/if}
			</div>

			<div class="controls-right">
				<!-- Date Range -->
				<div class="date-range">
					<input
						id="blog-date-start"
						name="blog-date-start"
						type="date"
						bind:value={dateRange.start}
						class="date-input"
					/>
					<span>to</span>
					<input
						id="blog-date-end"
						name="blog-date-end"
						type="date"
						bind:value={dateRange.end}
						class="date-input"
					/>
				</div>

				<!-- Filters -->
				<select class="filter-select" bind:value={statusFilter}>
					{#each statusOptions as option (option.value)}
						<option value={option.value}>{option.label}</option>
					{/each}
				</select>

				<select class="filter-select" bind:value={categoryFilter}>
					<option value="all">All Categories</option>
					{#each predefinedCategories as category (category.id)}
						<option value={category.id}>{category.name}</option>
					{/each}
				</select>

				<!-- Sort -->
				<div class="sort-controls">
					<select class="filter-select" bind:value={sortBy}>
						{#each sortOptions as option (option.value)}
							<option value={option.value}>{option.label}</option>
						{/each}
					</select>
					<button
						class="btn-icon"
						onclick={() => (sortOrder = sortOrder === 'asc' ? 'desc' : 'asc')}
					>
						{#if sortOrder === 'asc'}
							<IconSortAscending size={18} />
						{:else}
							<IconSortDescending size={18} />
						{/if}
					</button>
				</div>

				<!-- View Toggle -->
				<div class="view-toggle">
					<button
						class:active={viewMode === 'grid'}
						onclick={() => (viewMode = 'grid')}
						title="Grid View"
					>
						<IconLayoutGrid size={18} />
					</button>
					<button
						class:active={viewMode === 'list'}
						onclick={() => (viewMode = 'list')}
						title="List View"
					>
						<IconList size={18} />
					</button>
				</div>

				<!-- Actions -->
				<div class="action-buttons">
					<button class="btn-secondary" onclick={() => (showExportModal = true)}>
						<IconDownload size={18} />
						Export
					</button>
					<label class="btn-secondary">
						<IconUpload size={18} />
						Import
						<input type="file" accept=".csv,.json" onchange={importPosts} hidden />
					</label>
					<a href="/admin/blog/categories" class="btn-secondary">
						<IconFilter size={18} />
						Categories
					</a>
				</div>
			</div>
		</div>

		<!-- Posts View -->
		{#if viewMode === 'grid'}
			<div class="posts-grid">
				{#if loading}
					{#each Array(6) as _, i (i)}
						<div class="post-card skeleton">
							<div class="skeleton-image"></div>
							<div class="skeleton-content">
								<div class="skeleton-line"></div>
								<div class="skeleton-line short"></div>
							</div>
						</div>
					{/each}
				{:else if posts.length === 0}
					<div class="empty-state">
						<IconEdit size={48} />
						<h3>No posts found</h3>
						<p>{searchQuery ? 'Try a different search' : 'Create your first blog post'}</p>
						<button class="btn-primary" onclick={() => goto('/admin/blog/create')}>
							<IconPlus size={18} />
							Create Post
						</button>
					</div>
				{:else}
					<!-- Select All Card -->
					<div class="select-all-card">
						<label class="checkbox-label">
							<input
								id="select-all-posts"
								name="select-all-posts"
								type="checkbox"
								bind:checked={selectAll}
								onchange={toggleSelectAll}
							/>
							<span>Select All ({posts.length})</span>
						</label>
					</div>

					{#each posts as post (post.id)}
						<!-- FIX-2026-04-26 (P3-10): drop per-card transition:scale; running 50+
						     scale animations on every filter change is a perf hit. -->
						<div class="post-card" class:selected={selectedPosts.has(post.id)}>
							<!-- Selection Checkbox -->
							<div class="post-select">
								<input
									id="post-select-{post.id}"
									name="post-select-{post.id}"
									type="checkbox"
									checked={selectedPosts.has(post.id)}
									onchange={() => togglePostSelection(post.id)}
								/>
							</div>

							<!-- Featured Star -->
							<button
								class="featured-toggle"
								class:active={post.featured}
								onclick={() => toggleFeatured(post)}
							>
								{#if post.featured}
									<IconStarFilled size={20} />
								{:else}
									<IconStar size={20} />
								{/if}
							</button>

							<!-- Post Image -->
							{#if post.featured_image}
								<div class="post-image" style="background-image: url({post.featured_image})">
									{#if post.status === 'scheduled'}
										<div class="scheduled-overlay">
											<IconClock size={20} />
											{formatDateTime(post.publish_at ?? '')}
										</div>
									{/if}
								</div>
							{:else}
								<div class="post-image placeholder">
									<IconEdit size={32} />
								</div>
							{/if}

							<div class="post-content">
								<div class="post-header">
									<h3>{post.title}</h3>
									<div class="post-badges">
										<span class="status-badge status-{post.status}">
											{post.status}
										</span>
										{#if post.seo_score}
											<span
												class="seo-badge"
												style="background: {getSeoScoreColor(
													post.seo_score
												)}20; color: {getSeoScoreColor(post.seo_score)}"
											>
												SEO: {post.seo_score}
											</span>
										{/if}
									</div>
								</div>

								{#if post.excerpt}
									<p class="post-excerpt">{post.excerpt}</p>
								{/if}

								<!-- Metrics -->
								<div class="post-metrics">
									<div class="metric">
										<IconEye size={16} />
										{formatNumber(post.view_count || 0)}
									</div>
									<div class="metric">
										<IconChartBar size={16} />
										{post.engagement_rate || 0}%
									</div>
									{#if post.bounce_rate}
										<div class="metric">
											<IconTrendingUp size={16} />
											{post.bounce_rate}% bounce
										</div>
									{/if}
									{#if post.avg_read_time}
										<div class="metric">
											<IconClock size={16} />
											{post.avg_read_time}s read
										</div>
									{/if}
								</div>

								<div class="post-meta">
									{#if post.author}
										<div class="meta-item">
											<IconUser size={16} />
											{post.author.name}
										</div>
									{/if}

									{#if post.published_at}
										<div class="meta-item">
											<IconCalendar size={16} />
											{formatDate(post.published_at)}
										</div>
									{/if}
								</div>

								{#if post.categories && post.categories.length > 0}
									<div class="post-categories">
										{#each (post.categories || []).slice(0, 3) as categoryId, i (i)}
											{@const category =
												typeof categoryId === 'string'
													? getPredefinedCategoryById(categoryId)
													: categoryId}
											{#if category}
												<span
													class="category-badge"
													style:--tag-color={category.color || '#E6B800'}
												>
													{category.name}
												</span>
											{/if}
										{/each}
										{#if post.categories.length > 3}
											<span class="category-more">+{post.categories.length - 3}</span>
										{/if}
									</div>
								{/if}

								<!-- Quick Actions -->
								<div class="post-actions">
									<button
										class="action-btn primary"
										onclick={() => goto(`/admin/blog/edit/${post.id}`)}
										title="Edit"
									>
										<IconEdit size={18} />
										Edit
									</button>

									<button class="action-btn" onclick={() => (previewPost = post)} title="Preview">
										<IconEye size={18} />
									</button>

									<button
										class="action-btn"
										onclick={() => toggleStatus(post)}
										title="Toggle Status"
									>
										{#if post.status === 'published'}
											<IconPlayerPause size={18} />
										{:else}
											<IconPlayerPlay size={18} />
										{/if}
									</button>

									<!-- More Actions -->
									<div class="action-dropdown">
										<button class="action-btn" onclick={() => toggleActionMenu(post.id)}>
											<IconMenu2 size={18} />
										</button>

										{#if activeActionMenu === post.id}
											<div class="action-menu" transition:scale={{ duration: 150 }}>
												<button onclick={() => duplicatePost(post.id)}>
													<IconCopy size={16} />
													Duplicate
												</button>
												<!--
													TODO: Re-add schedule button + modal when a scheduler worker exists.
													See BLOG_SYSTEM_AUDIT.md §7. Needs a tokio::spawn loop in
													api/src/main.rs that polls posts WHERE status='scheduled'
													AND scheduled_publish_at <= NOW() and flips them to 'published'.
												-->

												<button onclick={() => loadPostAnalytics(post)}>
													<IconChartBar size={16} />
													Analytics
												</button>
												<button onclick={() => window.open(`/blog/${post.slug}`, '_blank')}>
													<IconExternalLink size={16} />
													View Live
												</button>
												<hr />
												<button class="danger" onclick={() => deletePost(post.id)}>
													<IconTrash size={16} />
													Delete
												</button>
											</div>
										{/if}
									</div>
								</div>
							</div>
						</div>
					{/each}
				{/if}
			</div>
		{:else}
			<!-- List View -->
			<div class="posts-table-container">
				<div class="table-scroll-wrapper">
					<table class="posts-table">
						<thead>
							<tr>
								<th class="th-checkbox">
									<input
										id="select-all-posts-list"
										name="select-all-posts-list"
										type="checkbox"
										bind:checked={selectAll}
										onchange={toggleSelectAll}
									/>
								</th>
								<th class="th-title">Title</th>
								<th class="th-author hidden-mobile">Author</th>
								<th class="th-status">Status</th>
								<th class="th-categories hidden-mobile hidden-tablet">Categories</th>
								<th class="th-views hidden-mobile">Views</th>
								<th class="th-seo hidden-mobile hidden-tablet">SEO</th>
								<th class="th-published hidden-mobile">Published</th>
								<th class="th-actions">Actions</th>
							</tr>
						</thead>
						<tbody>
							{#each posts as post (post.id)}
								<tr class:selected={selectedPosts.has(post.id)}>
									<td class="td-checkbox">
										<input
											id="post-select-list-{post.id}"
											name="post-select-list-{post.id}"
											type="checkbox"
											checked={selectedPosts.has(post.id)}
											onchange={() => togglePostSelection(post.id)}
										/>
									</td>
									<td class="td-title">
										<div class="table-title">
											{#if post.featured}
												<IconStarFilled size={16} class="featured-icon" />
											{/if}
											<a href="/admin/blog/edit/{post.id}">{post.title}</a>
										</div>
										<!-- Mobile-only status badge inline -->
										<span class="mobile-status-badge status-badge status-{post.status}">
											{post.status}
										</span>
									</td>
									<td class="hidden-mobile">{post.author?.name || '-'}</td>
									<td class="td-status">
										<span class="status-badge status-{post.status}">
											{post.status}
										</span>
									</td>
									<td class="hidden-mobile hidden-tablet">
										{#if (post.categories?.length ?? 0) > 0}
											<div class="table-category-tags">
												{#each (post.categories || []).slice(0, 2) as categoryId, i (i)}
													{@const category =
														typeof categoryId === 'string'
															? getPredefinedCategoryById(categoryId)
															: categoryId}
													{#if category}
														<span
															class="category-tag-table"
															style:--tag-color={category.color || '#E6B800'}>{category.name}</span
														>
													{/if}
												{/each}
												{#if (post.categories?.length ?? 0) > 2}
													<span class="more-tag">+{(post.categories?.length ?? 0) - 2}</span>
												{/if}
											</div>
										{:else}
											-
										{/if}
									</td>
									<td class="hidden-mobile">{formatNumber(post.view_count || 0)}</td>
									<td class="hidden-mobile hidden-tablet">
										<div class="seo-score-bar" style="--score: {post.seo_score}%">
											{post.seo_score}
										</div>
									</td>
									<td class="hidden-mobile"
										>{post.published_at ? formatDate(post.published_at) : '-'}</td
									>
									<td class="td-actions">
										<div class="table-actions">
											<button
												class="action-icon"
												onclick={() => goto(`/admin/blog/edit/${post.id}`)}
												title="Edit"
											>
												<IconEdit size={16} />
											</button>
											<button
												class="action-icon hidden-mobile"
												onclick={() => (previewPost = post)}
												title="Preview"
											>
												<IconEye size={16} />
											</button>
											<button
												class="action-icon hidden-mobile hidden-tablet"
												onclick={() => duplicatePost(post.id)}
												title="Duplicate"
											>
												<IconCopy size={16} />
											</button>
											<button
												class="action-icon danger"
												onclick={() => deletePost(post.id)}
												title="Delete"
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

				<!-- Mobile Pagination (Simplified) -->
				<div class="table-pagination">
					<div class="pagination-info">
						Showing {posts.length} posts
					</div>
					<div class="pagination-controls">
						<button class="pagination-btn" disabled> Previous </button>
						<span class="pagination-current">1</span>
						<button class="pagination-btn" disabled> Next </button>
					</div>
				</div>
			</div>
		{/if}

		<!-- Preview Modal -->
		<PreviewModal post={previewPost} onClose={() => (previewPost = null)} />

		<!-- Export Modal -->
		<ExportModal
			open={showExportModal}
			bind:format={exportFormat}
			selectedCount={selectedPosts.size}
			onClose={() => (showExportModal = false)}
			onExport={exportPosts}
		/>

		<!--
			TODO: Re-add schedule modal when scheduler worker exists.
			See BLOG_SYSTEM_AUDIT.md §7. Removed 2026-04-27: the previous
			modal here only showed a "coming soon" toast on submit.
		-->

		<!-- Analytics Modal -->
		<AnalyticsModal
			open={showAnalyticsModal}
			post={analyticsPost}
			{formatNumber}
			onClose={() => (showAnalyticsModal = false)}
		/>
	</div>
	<!-- End admin-page-container -->
</div>

<!-- Keyboard Shortcuts Helper -->
<KeyboardShortcutsHelper />

<ConfirmationModal
	isOpen={showDeleteModal}
	title="Delete Post"
	message="Are you sure you want to delete this post?"
	confirmText="Delete"
	variant="danger"
	onConfirm={confirmDeletePost}
	onCancel={() => {
		showDeleteModal = false;
		pendingDeleteId = null;
	}}
/>

<ConfirmationModal
	isOpen={showBulkDeleteModal}
	title="Delete Posts"
	message={`Delete ${selectedPosts.size} posts? This cannot be undone.`}
	confirmText="Delete All"
	variant="danger"
	onConfirm={confirmBulkDelete}
	onCancel={() => (showBulkDeleteModal = false)}
/>

<style>
	.admin-blog {
		background: linear-gradient(
			135deg,
			var(--bg-base) 0%,
			var(--bg-elevated) 50%,
			var(--bg-base) 100%
		);
		position: relative;
		overflow: hidden;
		color: var(--admin-text-primary);
	}

	/* Notifications */
	.notifications {
		position: fixed;
		top: 1rem;
		right: 1rem;
		z-index: 1000;
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.notification {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem 1.5rem;
		background: white;
		border-radius: 8px;
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
		font-weight: 500;
		min-width: 300px;
	}

	.notification.success {
		background: #dcfce7;
		color: #16a34a;
	}

	.notification.error {
		background: #fee2e2;
		color: #dc2626;
	}

	.notification.warning {
		background: #fef3c7;
		color: #d97706;
	}

	.notification.info {
		background: #dbeafe;
		color: #2563eb;
	}

	/* Header */
	.page-header {
		text-align: center;
		margin-bottom: 2rem;
	}

	.page-header h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: #f1f5f9;
		margin-bottom: 0.5rem;
	}

	.subtitle {
		color: #64748b;
		font-size: 0.875rem;
		margin-bottom: 1rem;
	}

	.header-actions {
		display: flex;
		justify-content: center;
		gap: 0.75rem;
	}

	/* Buttons - Email Templates Style */
	.btn-primary,
	.btn-secondary,
	.btn-ghost,
	.btn-icon {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		border-radius: 8px;
		font-weight: 500;
		border: none;
		cursor: pointer;
		transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1);
		text-decoration: none;
	}

	.btn-primary {
		background: linear-gradient(135deg, var(--primary-500), var(--primary-600));
		color: white;
		box-shadow: 0 2px 8px rgba(99, 102, 241, 0.25);
	}

	.btn-primary:hover {
		transform: translateY(-2px);
		box-shadow: 0 8px 20px rgba(99, 102, 241, 0.4);
	}

	.btn-secondary {
		background: rgba(100, 116, 139, 0.2);
		color: #cbd5e1;
		border: 1px solid rgba(100, 116, 139, 0.3);
	}

	.btn-secondary:hover {
		background: rgba(100, 116, 139, 0.3);
		border-color: rgba(100, 116, 139, 0.5);
	}

	.btn-ghost {
		background: transparent;
		color: var(--admin-text-secondary);
	}

	.btn-ghost:hover {
		background: var(--admin-btn-bg-hover);
		color: var(--admin-text-primary);
	}

	.btn-icon {
		padding: 0.5rem;
		background: var(--admin-btn-bg);
		color: var(--admin-text-secondary);
	}

	.btn-icon:hover {
		background: var(--admin-btn-bg-hover);
		color: var(--admin-accent-primary);
	}

	.btn-secondary.danger {
		background: var(--admin-error-bg);
		border-color: var(--admin-error-border);
		color: var(--admin-error-text);
	}

	.btn-secondary.danger:hover {
		background: rgba(218, 54, 51, 0.25);
	}

	/* Stats Grid */
	.stats-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
		gap: 1.5rem;
		margin-bottom: 2rem;
	}

	.stat-card {
		background: rgba(30, 41, 59, 0.4);
		border: 1px solid rgba(148, 163, 184, 0.2);
		padding: 1.5rem;
		border-radius: 8px;
		display: flex;
		gap: 1rem;
		transition: all 0.3s;
	}

	.stat-card:hover {
		transform: translateY(-2px);
		box-shadow: 0 8px 20px rgba(0, 0, 0, 0.2);
	}

	.stat-card.published {
		background: linear-gradient(135deg, rgba(16, 185, 129, 0.1), rgba(16, 185, 129, 0.05));
		border-color: rgba(16, 185, 129, 0.3);
	}

	.stat-card.draft {
		background: linear-gradient(135deg, rgba(245, 158, 11, 0.1), rgba(245, 158, 11, 0.05));
		border-color: rgba(245, 158, 11, 0.3);
	}

	.stat-card.views {
		background: linear-gradient(135deg, rgba(59, 130, 246, 0.1), rgba(230, 184, 0, 0.05));
		border-color: rgba(59, 130, 246, 0.3);
	}

	.stat-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 48px;
		height: 48px;
		background: rgba(255, 255, 255, 0.1);
		border-radius: 12px;
		color: #94a3b8;
	}

	.stat-content {
		flex: 1;
	}

	.stat-value {
		font-size: 2rem;
		font-weight: 700;
		color: var(--admin-text-primary);
		margin-bottom: 0.25rem;
	}

	.stat-label {
		color: var(--admin-text-muted);
		font-size: 0.9rem;
	}

	.stat-change {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		color: var(--admin-success);
		font-size: 0.85rem;
		margin-top: 0.5rem;
	}

	/* Controls Bar */
	.controls-bar {
		display: flex;
		justify-content: space-between;
		align-items: center;
		gap: 1rem;
		margin-bottom: 2rem;
		flex-wrap: wrap;
		padding: 1rem;
		background: rgba(30, 41, 59, 0.4);
		border-radius: 8px;
	}

	.controls-left {
		flex: 1;
		min-width: 300px;
	}

	.controls-right {
		display: flex;
		gap: 1rem;
		align-items: center;
		flex-wrap: wrap;
	}

	.search-box {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem 1rem;
		background: rgba(15, 23, 42, 0.8);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		flex: 1;
	}

	.search-box input {
		flex: 1;
		border: none;
		outline: none;
		background: transparent;
		color: #f1f5f9;
		font-size: 0.95rem;
	}

	/* Bulk Actions */
	.bulk-actions {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 0.75rem 1rem;
		background: rgba(59, 130, 246, 0.1);
		border: 1px solid rgba(59, 130, 246, 0.3);
		border-radius: 8px;
	}

	.bulk-count {
		font-weight: 600;
		color: #3b82f6;
	}

	/* Filters */
	.filter-select,
	.date-input {
		padding: 0.75rem 1rem;
		background: rgba(15, 23, 42, 0.8);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		color: #f1f5f9;
		cursor: pointer;
	}

	.date-range {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.sort-controls {
		display: flex;
		gap: 0.5rem;
	}

	.view-toggle {
		display: flex;
		gap: 0.25rem;
		padding: 0.25rem;
		background: rgba(15, 23, 42, 0.8);
		border-radius: 8px;
	}

	.view-toggle button {
		padding: 0.5rem;
		background: transparent;
		border: none;
		color: #94a3b8;
		cursor: pointer;
		border-radius: 6px;
		transition: all 0.2s;
	}

	.view-toggle button.active {
		background: rgba(59, 130, 246, 0.2);
		color: #3b82f6;
	}

	.action-buttons {
		display: flex;
		gap: 0.75rem;
	}

	/* Posts Grid */
	.posts-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(350px, 1fr));
		gap: 1.5rem;
	}

	.select-all-card {
		grid-column: 1 / -1;
		padding: 1rem;
		background: rgba(30, 41, 59, 0.4);
		border-radius: 8px;
	}

	.checkbox-label {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		cursor: pointer;
		color: #cbd5e1;
		font-weight: 500;
	}

	.post-card {
		background: rgba(30, 41, 59, 0.4);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		overflow: hidden;
		transition: all 0.3s;
		position: relative;
	}

	.post-card:hover {
		transform: translateY(-4px);
		box-shadow: 0 12px 32px rgba(0, 0, 0, 0.3);
		border-color: rgba(59, 130, 246, 0.3);
	}

	.post-card.selected {
		border-color: #3b82f6;
		background: rgba(59, 130, 246, 0.05);
	}

	.post-select {
		position: absolute;
		top: 1rem;
		left: 1rem;
		z-index: 10;
	}

	.featured-toggle {
		position: absolute;
		top: 1rem;
		right: 1rem;
		z-index: 10;
		background: rgba(0, 0, 0, 0.5);
		border: none;
		padding: 0.5rem;
		border-radius: 8px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
	}

	.featured-toggle.active {
		color: #fbbf24;
	}

	.post-image {
		width: 100%;
		height: 200px;
		background-size: cover;
		background-position: center;
		background-color: rgba(148, 163, 184, 0.1);
		position: relative;
	}

	.post-image.placeholder {
		display: flex;
		align-items: center;
		justify-content: center;
		background: linear-gradient(135deg, rgba(230, 184, 0, 0.2), rgba(179, 143, 0, 0.2));
		color: #94a3b8;
	}

	.scheduled-overlay {
		position: absolute;
		bottom: 0;
		left: 0;
		right: 0;
		padding: 0.75rem;
		background: rgba(0, 0, 0, 0.8);
		color: white;
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.875rem;
	}

	.post-content {
		padding: 1.5rem;
	}

	.post-header {
		margin-bottom: 0.75rem;
	}

	.post-header h3 {
		font-size: 1.25rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0 0 0.5rem 0;
		line-height: 1.4;
	}

	.post-badges {
		display: flex;
		gap: 0.5rem;
		flex-wrap: wrap;
	}

	.status-badge {
		padding: 0.25rem 0.75rem;
		border-radius: 12px;
		font-size: 0.75rem;
		font-weight: 600;
		text-transform: uppercase;
	}

	.status-badge.status-published {
		background: rgba(16, 185, 129, 0.2);
		color: #10b981;
	}

	.status-badge.status-draft {
		background: rgba(245, 158, 11, 0.2);
		color: #f59e0b;
	}

	.status-badge.status-scheduled {
		background: rgba(59, 130, 246, 0.2);
		color: #3b82f6;
	}

	.status-badge.status-archived {
		background: rgba(148, 163, 184, 0.2);
		color: #94a3b8;
	}

	.seo-badge {
		padding: 0.25rem 0.75rem;
		border-radius: 12px;
		font-size: 0.75rem;
		font-weight: 600;
	}

	.post-excerpt {
		color: #94a3b8;
		font-size: 0.9rem;
		line-height: 1.6;
		margin-bottom: 1rem;
		display: -webkit-box;
		-webkit-line-clamp: 2;
		line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	.post-metrics {
		display: flex;
		gap: 1rem;
		margin-bottom: 1rem;
		font-size: 0.85rem;
		color: #94a3b8;
	}

	.metric {
		display: flex;
		align-items: center;
		gap: 0.25rem;
	}

	.post-meta {
		display: flex;
		flex-wrap: wrap;
		gap: 1rem;
		margin-bottom: 1rem;
		font-size: 0.85rem;
		color: #94a3b8;
	}

	.meta-item {
		display: flex;
		align-items: center;
		gap: 0.375rem;
	}

	.post-categories {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
		margin-bottom: 1rem;
	}

	.category-badge {
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
		color: #94a3b8;
	}

	.post-actions {
		display: flex;
		gap: 0.5rem;
		padding-top: 1rem;
		border-top: 1px solid rgba(148, 163, 184, 0.1);
	}

	.action-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.375rem;
		padding: 0.5rem;
		background: rgba(148, 163, 184, 0.1);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 6px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
	}

	.action-btn:hover {
		background: rgba(148, 163, 184, 0.2);
		color: #f1f5f9;
	}

	.action-btn.primary {
		flex: 1;
		background: rgba(59, 130, 246, 0.1);
		border-color: rgba(59, 130, 246, 0.3);
		color: #3b82f6;
	}

	.action-btn.primary:hover {
		background: rgba(59, 130, 246, 0.2);
	}

	.action-dropdown {
		position: relative;
	}

	.action-menu {
		position: absolute;
		top: 100%;
		right: 0;
		margin-top: 0.5rem;
		background: rgba(30, 41, 59, 0.98);
		border: 1px solid rgba(148, 163, 184, 0.3);
		border-radius: 8px;
		overflow: hidden;
		min-width: 180px;
		z-index: 100;
		box-shadow: 0 8px 24px rgba(0, 0, 0, 0.3);
	}

	.action-menu button {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		width: 100%;
		padding: 0.75rem 1rem;
		background: none;
		border: none;
		color: #cbd5e1;
		font-size: 0.9rem;
		cursor: pointer;
		transition: all 0.2s;
		text-align: left;
	}

	.action-menu button:hover {
		background: rgba(59, 130, 246, 0.1);
		color: #3b82f6;
	}

	.action-menu button.danger {
		color: #f87171;
	}

	.action-menu button.danger:hover {
		background: rgba(239, 68, 68, 0.1);
	}

	.action-menu hr {
		margin: 0;
		border: none;
		border-top: 1px solid rgba(148, 163, 184, 0.2);
	}

	/* Table View */
	.posts-table-container {
		background: rgba(30, 41, 59, 0.4);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		overflow: hidden;
	}

	/* Responsive table scroll wrapper */
	.table-scroll-wrapper {
		overflow-x: auto;
		-webkit-overflow-scrolling: touch;
		scrollbar-width: thin;
		scrollbar-color: rgba(148, 163, 184, 0.3) transparent;
	}

	.table-scroll-wrapper::-webkit-scrollbar {
		height: 6px;
	}

	.table-scroll-wrapper::-webkit-scrollbar-track {
		background: transparent;
	}

	.table-scroll-wrapper::-webkit-scrollbar-thumb {
		background: rgba(148, 163, 184, 0.3);
		border-radius: 3px;
	}

	.posts-table {
		width: 100%;
		border-collapse: collapse;
		min-width: 600px;
	}

	/* Mobile status badge shown inline with title on mobile */
	.mobile-status-badge {
		display: none;
		margin-top: 0.5rem;
	}

	/* Responsive visibility classes */
	.hidden-mobile {
		display: table-cell;
	}

	.hidden-tablet {
		display: table-cell;
	}

	/* Table column specific styling */
	.th-checkbox,
	.td-checkbox {
		width: 48px;
		min-width: 48px;
	}

	.th-title {
		min-width: 200px;
	}

	.th-actions,
	.td-actions {
		width: 140px;
		min-width: 100px;
	}

	.posts-table th {
		background: rgba(15, 23, 42, 0.8);
		padding: 1rem;
		text-align: left;
		font-weight: 600;
		color: #cbd5e1;
		border-bottom: 1px solid rgba(148, 163, 184, 0.2);
	}

	.posts-table td {
		padding: 1rem;
		color: #94a3b8;
		border-bottom: 1px solid rgba(148, 163, 184, 0.1);
	}

	.posts-table tr:hover {
		background: rgba(59, 130, 246, 0.05);
	}

	.posts-table tr.selected {
		background: rgba(59, 130, 246, 0.1);
	}

	.table-title {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.table-title a {
		color: #f1f5f9;
		text-decoration: none;
		font-weight: 500;
	}

	.table-title a:hover {
		color: #3b82f6;
	}

	.featured-icon {
		color: #fbbf24;
	}

	.table-category-tags {
		display: flex;
		flex-wrap: wrap;
		gap: 0.25rem;
	}

	.category-tag-table {
		display: inline-flex;
		align-items: center;
		padding: 0.125rem 0.5rem;
		background: color-mix(in srgb, var(--tag-color, var(--primary-500)) 15%, transparent);
		border: 1px solid color-mix(in srgb, var(--tag-color, var(--primary-500)) 30%, transparent);
		border-radius: 4px;
		font-size: 0.7rem;
		font-weight: 500;
		color: var(--tag-color, var(--primary-500));
		white-space: nowrap;
	}

	.category-tag {
		display: inline-block;
		padding: 0.125rem 0.5rem;
		background: rgba(148, 163, 184, 0.2);
		border-radius: 4px;
		font-size: 0.75rem;
		margin-right: 0.25rem;
	}

	.more-tag {
		display: inline-block;
		padding: 0.125rem 0.5rem;
		background: rgba(59, 130, 246, 0.2);
		color: #3b82f6;
		border-radius: 4px;
		font-size: 0.75rem;
		font-weight: 600;
	}

	.seo-score-bar {
		position: relative;
		width: 60px;
		height: 20px;
		background: rgba(148, 163, 184, 0.2);
		border-radius: 10px;
		overflow: hidden;
		font-size: 0.75rem;
		display: flex;
		align-items: center;
		justify-content: center;
		color: white;
		font-weight: 600;
	}

	.seo-score-bar::before {
		content: '';
		position: absolute;
		left: 0;
		top: 0;
		height: 100%;
		width: var(--score);
		background: linear-gradient(90deg, #ef4444, #f59e0b, #10b981);
		z-index: -1;
	}

	.table-actions {
		display: flex;
		gap: 0.5rem;
	}

	.action-icon {
		padding: 0.375rem;
		background: rgba(148, 163, 184, 0.1);
		border: 1px solid transparent;
		border-radius: 6px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
	}

	.action-icon:hover {
		background: rgba(148, 163, 184, 0.2);
		color: #f1f5f9;
	}

	.action-icon.danger:hover {
		background: rgba(239, 68, 68, 0.1);
		border-color: rgba(239, 68, 68, 0.3);
		color: #f87171;
	}

	/* Table Pagination */
	.table-pagination {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1rem;
		border-top: 1px solid rgba(148, 163, 184, 0.1);
		background: rgba(15, 23, 42, 0.4);
	}

	.pagination-info {
		font-size: 0.875rem;
		color: #94a3b8;
	}

	.pagination-controls {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.pagination-btn {
		padding: 0.5rem 1rem;
		background: rgba(148, 163, 184, 0.1);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 6px;
		color: #cbd5e1;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.2s;
		min-height: 44px;
		min-width: 44px;
	}

	.pagination-btn:hover:not(:disabled) {
		background: rgba(59, 130, 246, 0.1);
		border-color: rgba(59, 130, 246, 0.3);
		color: #3b82f6;
	}

	.pagination-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.pagination-current {
		padding: 0.5rem 1rem;
		background: rgba(59, 130, 246, 0.2);
		border: 1px solid rgba(59, 130, 246, 0.3);
		border-radius: 6px;
		color: #3b82f6;
		font-weight: 600;
		font-size: 0.875rem;
	}

	/* Full pagination numbers - hidden on mobile */
	.pagination-numbers {
		display: flex;
		gap: 0.25rem;
	}

	.pagination-number {
		padding: 0.5rem 0.75rem;
		background: transparent;
		border: 1px solid transparent;
		border-radius: 6px;
		color: #94a3b8;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.2s;
		min-width: 40px;
		text-align: center;
	}

	.pagination-number:hover {
		background: rgba(148, 163, 184, 0.1);
		color: #f1f5f9;
	}

	.pagination-number.active {
		background: rgba(59, 130, 246, 0.2);
		border-color: rgba(59, 130, 246, 0.3);
		color: #3b82f6;
		font-weight: 600;
	}

	/* Empty State */
	.empty-state {
		grid-column: 1 / -1;
		text-align: center;
		padding: 4rem 2rem;
	}

	.empty-state h3 {
		color: #f1f5f9;
		margin: 1rem 0 0.5rem;
	}

	.empty-state p {
		color: #94a3b8;
		margin-bottom: 1.5rem;
	}

	/* Loading Skeleton */
	.skeleton {
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
	}

	.skeleton-image {
		height: 200px;
		background: linear-gradient(
			90deg,
			rgba(148, 163, 184, 0.1) 25%,
			rgba(148, 163, 184, 0.2) 50%,
			rgba(148, 163, 184, 0.1) 75%
		);
		background-size: 200% 100%;
		animation: loading 1.5s infinite;
	}

	.skeleton-content {
		padding: 1.5rem;
	}

	.skeleton-line {
		height: 20px;
		background: linear-gradient(
			90deg,
			rgba(148, 163, 184, 0.1) 25%,
			rgba(148, 163, 184, 0.2) 50%,
			rgba(148, 163, 184, 0.1) 75%
		);
		background-size: 200% 100%;
		animation: loading 1.5s infinite;
		border-radius: 4px;
		margin-bottom: 0.75rem;
	}

	.skeleton-line.short {
		width: 60%;
	}

	@keyframes loading {
		0% {
			background-position: 200% 0;
		}
		100% {
			background-position: -200% 0;
		}
	}

	/* Modal CSS (overlay/header/content/actions + analytics/export/preview variants) moved
	   to _components/{PreviewModal,ExportModal,AnalyticsModal}.svelte (extraction R5-C). */

	/* Schedule modal CSS removed 2026-04-27 along with the modal markup. */

	/* Keyboard Shortcuts */
	.keyboard-shortcuts {
		position: fixed;
		bottom: 1rem;
		left: 50%;
		transform: translateX(-50%);
		display: flex;
		gap: 1rem;
		padding: 0.75rem 1.25rem;
		background: rgba(30, 41, 59, 0.95);
		border: 1px solid rgba(148, 163, 184, 0.3);
		border-radius: 8px;
		font-size: 0.75rem;
		color: #94a3b8;
		z-index: 100;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * RESPONSIVE BREAKPOINTS - Apple ICT7 Principal Engineer Grade
	 * Mobile-first approach with progressive enhancement
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* Tablet Landscape (< 1200px) */
	@media (max-width: 1200px) {
		.controls-bar {
			flex-direction: column;
			align-items: stretch;
		}

		.controls-left,
		.controls-right {
			width: 100%;
		}

		.controls-right {
			flex-direction: column;
			align-items: stretch;
		}

		.action-buttons {
			flex-direction: column;
		}
	}

	/* Tablet Portrait (< 1024px) */
	@media (max-width: 1024px) {
		.posts-grid {
			grid-template-columns: repeat(2, 1fr);
		}

		.stats-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	/* Mobile Landscape / Tablet (< 768px) */
	@media (max-width: 768px) {
		.posts-grid {
			grid-template-columns: 1fr;
		}

		.stats-grid {
			grid-template-columns: 1fr;
		}

		.keyboard-shortcuts {
			display: none;
		}

		/* Table responsive - hide non-essential columns */
		:where(.hidden-mobile) {
			display: none;
		}

		/* Show mobile status badge inline with title */
		.mobile-status-badge {
			display: inline-block;
		}

		/* Hide the separate status column on mobile */
		.td-status {
			display: none;
		}

		.th-status {
			display: none;
		}

		/* Reduce table minimum width for mobile */
		.posts-table {
			min-width: 320px;
		}

		/* Stacked action buttons on mobile */
		.table-actions {
			flex-direction: column;
			gap: 0.375rem;
		}

		.action-icon {
			width: 100%;
			justify-content: center;
		}

		/* Pagination simplified */
		.pagination-numbers {
			display: none;
		}

		.table-pagination {
			flex-direction: column;
			gap: 0.75rem;
		}

		/* Search/filter controls full width */
		.controls-left {
			min-width: 100%;
		}

		.search-box {
			width: 100%;
		}

		/* Bulk actions stacked */
		.bulk-actions {
			flex-wrap: wrap;
			justify-content: center;
		}

		/* Date range stacked */
		.date-range {
			flex-direction: column;
			width: 100%;
		}

		.date-input {
			width: 100%;
		}

		/* Filter selects full width */
		.filter-select {
			width: 100%;
		}

		/* Action buttons stacked */
		.action-buttons {
			width: 100%;
		}

		.action-buttons .btn-secondary {
			flex: 1;
			justify-content: center;
		}
	}

	/* Tablet (< 1024px) - hide extra columns */
	@media (max-width: 1024px) {
		:where(.hidden-tablet) {
			display: none;
		}
	}

	/* Mobile Portrait (< 640px) */
	@media (max-width: 640px) {
		.page {
			padding: 0.75rem;
		}

		.page-header h1 {
			font-size: 1.5rem;
		}

		.post-card {
			padding: 1rem;
		}

		.stat-card {
			padding: 1rem;
		}

		.btn-primary,
		.btn-secondary {
			padding: 0.5rem 0.75rem;
			font-size: 0.8125rem;
			width: 100%;
			justify-content: center;
		}

		/* Controls bar tighter on mobile */
		.controls-bar {
			padding: 0.75rem;
			gap: 0.75rem;
		}

		/* Header actions stacked */
		.header-actions {
			flex-direction: column;
			width: 100%;
		}

		.header-actions .btn-primary,
		.header-actions .btn-icon {
			width: 100%;
		}

		/* View toggle centered */
		.view-toggle {
			width: 100%;
			justify-content: center;
		}

		/* Sort controls full width */
		.sort-controls {
			width: 100%;
		}

		.sort-controls .filter-select {
			flex: 1;
		}

		/* Table title truncation */
		.td-title {
			max-width: 180px;
		}

		.table-title a {
			white-space: nowrap;
			overflow: hidden;
			text-overflow: ellipsis;
			max-width: 150px;
			display: block;
		}
	}

	/* Extra Small Mobile (< 380px) - iPhone SE, Galaxy Fold */
	@media (max-width: 380px) {
		.page {
			padding: 0.5rem;
		}

		.page-header h1 {
			font-size: 1.25rem;
		}

		.page-header {
			flex-direction: column;
			align-items: flex-start;
			gap: 0.75rem;
		}

		.controls-bar {
			padding: 0.75rem;
		}

		.post-card {
			padding: 0.75rem;
		}

		.post-title {
			font-size: 0.9375rem;
		}
	}

	/* Touch Device Optimizations - Apple HIG 44pt minimum */
	@media (hover: none) and (pointer: coarse) {
		.btn-primary,
		.btn-secondary,
		.btn-icon {
			min-height: 44px;
			min-width: 44px;
		}

		.search-input,
		.filter-select {
			min-height: 48px;
			font-size: 16px; /* Prevents iOS zoom */
		}

		.post-card {
			min-height: 100px;
		}

		.stat-card {
			min-height: 80px;
		}

		/* Larger touch targets for actions */
		.post-actions button {
			min-width: 44px;
			min-height: 44px;
		}
	}

	/* Reduced Motion - Accessibility */
	@media (prefers-reduced-motion: reduce) {
		.btn-primary,
		.btn-secondary,
		.post-card,
		.stat-card {
			transition: none;
		}

		.post-card:hover,
		.stat-card:hover {
			transform: none;
		}
	}

	/* High Contrast Mode - Accessibility */
	@media (prefers-contrast: high) {
		.post-card,
		.controls-bar {
			border-width: 2px;
		}

		.page-header h1,
		.post-title {
			font-weight: 800;
		}
	}

	/* Print Styles */
	@media print {
		.page {
			background: white;
			color: black;
		}

		.controls-bar,
		.action-buttons,
		.keyboard-shortcuts {
			display: none;
		}

		.post-card,
		.stat-card {
			break-inside: avoid;
			box-shadow: none;
			border: 1px solid #ccc;
		}
	}

	/* Landscape Mode - Short viewport */
	@media (max-height: 500px) and (orientation: landscape) {
		.page {
			padding: 0.5rem 1rem;
		}

		.page-header {
			margin-bottom: 0.75rem;
		}

		.posts-grid,
		.stats-grid {
			gap: 0.5rem;
		}
	}
</style>

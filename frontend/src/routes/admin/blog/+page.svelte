<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { goto } from '$app/navigation';
	import { fly, slide, scale } from 'svelte/transition';
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
		IconAlertCircle,
		IconCheck,
		IconX,
		IconMenu2,
		IconLayoutGrid
	} from '$lib/icons';

	// ═══════════════════════════════════════════════════════════════════════════
	// PREDEFINED BLOG CATEGORIES (same system as videos)
	// ═══════════════════════════════════════════════════════════════════════════

	interface BlogCategory {
		id: string;
		name: string;
		color: string;
	}

	const predefinedCategories: BlogCategory[] = [
		{ id: 'market-analysis', name: 'Market Analysis', color: '#3b82f6' },
		{ id: 'trading-strategies', name: 'Trading Strategies', color: '#10b981' },
		{ id: 'risk-management', name: 'Risk Management', color: '#ef4444' },
		{ id: 'options-trading', name: 'Options Trading', color: '#f59e0b' },
		{ id: 'technical-analysis', name: 'Technical Analysis', color: '#6366f1' },
		{ id: 'fundamental-analysis', name: 'Fundamental Analysis', color: '#ec4899' },
		{ id: 'psychology', name: 'Psychology', color: '#8b5cf6' },
		{ id: 'education', name: 'Education', color: '#14b8a6' },
		{ id: 'news', name: 'News & Updates', color: '#06b6d4' },
		{ id: 'earnings', name: 'Earnings', color: '#f97316' },
		{ id: 'stocks', name: 'Stocks', color: '#84cc16' },
		{ id: 'futures', name: 'Futures', color: '#22c55e' },
		{ id: 'forex', name: 'Forex', color: '#0ea5e9' },
		{ id: 'crypto', name: 'Crypto', color: '#a855f7' },
		{ id: 'small-accounts', name: 'Small Accounts', color: '#eab308' },
		{ id: 'day-trading', name: 'Day Trading', color: '#d946ef' },
		{ id: 'swing-trading', name: 'Swing Trading', color: '#64748b' },
		{ id: 'beginners', name: 'Beginners Guide', color: '#fb7185' }
	];

	function getPredefinedCategoryById(id: string): BlogCategory | undefined {
		return predefinedCategories.find(c => c.id === id);
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// State Management
	// ═══════════════════════════════════════════════════════════════════════════

	let posts = $state<any[]>([]);
	let stats = $state<any>(null);
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
	let previewPost = $state<any>(null);
	let activeActionMenu = $state<number | null>(null);
	let ws = $state<WebSocket | null>(null);
	let showExportModal = $state(false);
	let exportFormat = $state<'csv' | 'json' | 'wordpress'>('csv');
	let showScheduleModal = $state(false);
	let schedulePost = $state<any>(null);
	let showAnalyticsModal = $state(false);
	let analyticsPost = $state<any>(null);
	let refreshInterval = $state<ReturnType<typeof setInterval> | undefined>(undefined);
	let notifications = $state<any[]>([]);

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

	// ═══════════════════════════════════════════════════════════════════════════
	// Lifecycle
	// ═══════════════════════════════════════════════════════════════════════════

	onMount(() => {
		loadPosts();
		loadStats();
		setupWebSocket();
		setupKeyboardShortcuts();

		// Auto-refresh every 30 seconds
		refreshInterval = setInterval(() => {
			loadStats();
			if (viewMode === 'list') loadPosts();
		}, 30000);
	});

	onDestroy(() => {
		if (ws) ws.close();
		if (refreshInterval) clearInterval(refreshInterval);
		document.removeEventListener('keydown', handleKeyboard);
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
			posts = (data.data || []).map((post: any) => ({
				...post,
				seo_score: calculateSeoScore(post),
				engagement_rate: calculateEngagementRate(post),
				selected: selectedPosts.has(post.id)
			}));
		} catch (error) {
			console.error('Failed to load posts:', error);
			showNotification('error', 'Failed to load posts');
		} finally {
			loading = false;
		}
	}

	async function loadStats() {
		try {
			stats = await adminFetch('/api/admin/posts/stats');
		} catch (error) {
			console.error('Failed to load stats:', error);
		}
	}

	
	// ═══════════════════════════════════════════════════════════════════════════
	// WebSocket & Real-time Updates
	// ═══════════════════════════════════════════════════════════════════════════

	function setupWebSocket() {
		try {
			ws = new WebSocket('wss://your-api/posts');

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

			ws.onerror = (error) => {
				console.error('WebSocket error:', error);
			};
		} catch (error) {
			console.error('Failed to setup WebSocket:', error);
		}
	}

	function updatePostMetric(postId: number, metric: string, value: any) {
		const post = posts.find((p) => p.id === postId);
		if (post) {
			post[metric] = value;
			posts = posts; // Trigger reactivity
		}
	}

	function updatePostStatus(postId: number, status: string) {
		const post = posts.find((p) => p.id === postId);
		if (post) {
			post.status = status;
			posts = posts;
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
		if (selectedPosts.has(postId)) {
			selectedPosts.delete(postId);
		} else {
			selectedPosts.add(postId);
		}
		selectedPosts = selectedPosts;

		const post = posts.find((p) => p.id === postId);
		if (post) post.selected = !post.selected;
		posts = posts;

		selectAll = selectedPosts.size === posts.length;
	}

	async function bulkDelete() {
		if (selectedPosts.size === 0) {
			showNotification('warning', 'No posts selected');
			return;
		}

		if (!confirm(`Delete ${selectedPosts.size} posts? This cannot be undone.`)) return;

		try {
			await adminFetch('/api/admin/posts/bulk-delete', {
				method: 'POST',
				body: JSON.stringify({ ids: [...selectedPosts] })
			});
			loadPosts();
			loadStats();
			selectedPosts.clear();
			selectAll = false;
			showNotification('success', `Deleted ${selectedPosts.size} posts`);
		} catch (error) {
			console.error('Failed to bulk delete:', error);
			showNotification('error', 'Failed to delete posts');
		}
	}

	async function bulkChangeStatus(newStatus: string) {
		if (selectedPosts.size === 0) {
			showNotification('warning', 'No posts selected');
			return;
		}

		try {
			await adminFetch('/api/admin/posts/bulk-status', {
				method: 'POST',
				body: JSON.stringify({ ids: [...selectedPosts], status: newStatus })
			});
			loadPosts();
			loadStats();
			selectedPosts.clear();
			selectAll = false;
			showNotification('success', `Updated ${selectedPosts.size} posts to ${newStatus}`);
		} catch (error) {
			console.error('Failed to bulk update status:', error);
			showNotification('error', 'Failed to update posts');
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Individual Post Actions
	// ═══════════════════════════════════════════════════════════════════════════

	async function deletePost(id: number) {
		if (!confirm('Are you sure you want to delete this post?')) return;

		try {
			await adminFetch(`/api/admin/posts/${id}`, { method: 'DELETE' });
			loadPosts();
			loadStats();
			showNotification('success', 'Post deleted');
		} catch (error) {
			console.error('Failed to delete post:', error);
			showNotification('error', 'Failed to delete post');
		}
	}

	async function duplicatePost(id: number) {
		try {
			await adminFetch(`/api/admin/posts/${id}/duplicate`, { method: 'POST' });
			loadPosts();
			showNotification('success', 'Post duplicated');
		} catch (error) {
			console.error('Failed to duplicate post:', error);
			showNotification('error', 'Failed to duplicate post');
		}
	}

	async function toggleStatus(post: any) {
		const newStatus = post.status === 'published' ? 'draft' : 'published';

		try {
			await adminFetch(`/api/admin/posts/${post.id}/status`, {
				method: 'PATCH',
				body: JSON.stringify({ status: newStatus })
			});
			post.status = newStatus;
			posts = posts;
			loadStats();
			showNotification('success', `Post ${newStatus}`);
		} catch (error) {
			console.error('Failed to toggle status:', error);
			showNotification('error', 'Failed to update status');
		}
	}

	async function toggleFeatured(post: any) {
		try {
			await adminFetch(`/api/admin/posts/${post.id}/featured`, {
				method: 'PATCH',
				body: JSON.stringify({ featured: !post.featured })
			});
			post.featured = !post.featured;
			posts = posts;
			showNotification('success', post.featured ? 'Post featured' : 'Post unfeatured');
		} catch (error) {
			console.error('Failed to toggle featured:', error);
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

			const response = await adminFetch<Response>(`/api/admin/posts/export?${params}`, { rawResponse: true });
			const blob = await response.blob();

			const a = document.createElement('a');
			a.href = URL.createObjectURL(blob);
			a.download = `posts-export-${new Date().toISOString().split('T')[0]}.${exportFormat}`;
			a.click();

			showExportModal = false;
			showNotification('success', 'Posts exported successfully');
		} catch (error) {
			console.error('Failed to export posts:', error);
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
			console.error('Failed to import posts:', error);
			showNotification('error', 'Failed to import posts');
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Analytics & SEO
	// ═══════════════════════════════════════════════════════════════════════════

	function calculateSeoScore(post: any): number {
		let score = 0;

		// Title length (60-70 chars optimal)
		if (post.title?.length >= 30 && post.title?.length <= 70) score += 20;
		else if (post.title?.length > 0) score += 10;

		// Meta description
		if (post.meta_description?.length >= 120 && post.meta_description?.length <= 160) score += 20;
		else if (post.meta_description?.length > 0) score += 10;

		// Featured image
		if (post.featured_image) score += 15;

		// Categories and tags
		if (post.categories?.length > 0) score += 10;
		if (post.tags?.length > 0) score += 10;

		// URL slug
		if (post.slug && post.slug.length < 60) score += 10;

		// Content length (min 300 words assumed)
		if (post.word_count >= 1500) score += 15;
		else if (post.word_count >= 500) score += 10;
		else if (post.word_count >= 300) score += 5;

		return Math.min(score, 100);
	}

	function calculateEngagementRate(post: any): number {
		if (!post.view_count || post.view_count === 0) return 0;

		const interactions = (post.likes || 0) + (post.comments || 0) + (post.shares || 0);
		return Math.round((interactions / post.view_count) * 100);
	}

	async function loadPostAnalytics(post: any) {
		analyticsPost = post;
		showAnalyticsModal = true;

		try {
			const data = await adminFetch(`/api/admin/posts/${post.id}/analytics`);
			analyticsPost = { ...analyticsPost, analytics: data };
		} catch (error) {
			console.error('Failed to load analytics:', error);
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
		const id = Date.now();
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

	// ICT11+ Fix: Debounced effect to reload posts when filters change
	// Previous implementation caused infinite loop by always evaluating to true
	let filterDebounceTimer: ReturnType<typeof setTimeout> | undefined;
	let isInitialMount = true;

	$effect(() => {
		// Track filter values to detect changes (this creates proper dependencies)
		const currentFilters = {
			search: searchQuery,
			status: statusFilter,
			category: categoryFilter,
			sort: sortBy,
			order: sortOrder,
			dateStart: dateRange.start,
			dateEnd: dateRange.end
		};

		// Skip initial mount (onMount already calls loadPosts)
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

<div class="blog-page">
	<!-- Notifications -->
	<div class="notifications">
		{#each notifications as notification (notification.id)}
			<div class="notification {notification.type}" transition:fly={{ y: -20, duration: 300 }}>
				{#if notification.type === 'success'}
					<IconCheck size={20} />
				{:else if notification.type === 'error'}
					<IconX size={20} />
				{:else}
					<IconAlertCircle size={20} />
				{/if}
				{notification.message}
			</div>
		{/each}
	</div>

	<!-- Header -->
	<header class="page-header">
		<div>
			<h1>Blog Posts</h1>
			<p>
				Create and manage blog posts • {posts.length} posts • Last updated: {new Date().toLocaleTimeString()}
			</p>
		</div>
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
	{#if stats}
		<div class="stats-grid">
			<div class="stat-card">
				<div class="stat-icon"><IconChartBar size={24} /></div>
				<div class="stat-content">
					<div class="stat-value">{stats.total}</div>
					<div class="stat-label">Total Posts</div>
					<div class="stat-change">+{stats.new_this_month || 0} this month</div>
				</div>
			</div>
			<div class="stat-card published">
				<div class="stat-icon"><IconCheck size={24} /></div>
				<div class="stat-content">
					<div class="stat-value">{stats.published}</div>
					<div class="stat-label">Published</div>
					<div class="stat-change">{Math.round((stats.published / stats.total) * 100)}%</div>
				</div>
			</div>
			<div class="stat-card draft">
				<div class="stat-icon"><IconEdit size={24} /></div>
				<div class="stat-content">
					<div class="stat-value">{stats.draft}</div>
					<div class="stat-label">Drafts</div>
					<div class="stat-change">{stats.scheduled || 0} scheduled</div>
				</div>
			</div>
			<div class="stat-card views">
				<div class="stat-icon"><IconEye size={24} /></div>
				<div class="stat-content">
					<div class="stat-value">{formatNumber(stats.total_views || 0)}</div>
					<div class="stat-label">Total Views</div>
					<div class="stat-change">
						<IconTrendingUp size={16} />
						{stats.views_growth || 0}%
					</div>
				</div>
			</div>
		</div>
	{/if}

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
					<input type="text" bind:value={searchQuery} placeholder="Search posts... (Ctrl+F)" />
				</div>
			{/if}
		</div>

		<div class="controls-right">
			<!-- Date Range -->
			<div class="date-range">
				<input type="date" bind:value={dateRange.start} class="date-input" />
				<span>to</span>
				<input type="date" bind:value={dateRange.end} class="date-input" />
			</div>

			<!-- Filters -->
			<select class="filter-select" bind:value={statusFilter}>
				{#each statusOptions as option}
					<option value={option.value}>{option.label}</option>
				{/each}
			</select>

			<select class="filter-select" bind:value={categoryFilter}>
				<option value="all">All Categories</option>
				{#each predefinedCategories as category}
					<option value={category.id}>{category.name}</option>
				{/each}
			</select>

			<!-- Sort -->
			<div class="sort-controls">
				<select class="filter-select" bind:value={sortBy}>
					{#each sortOptions as option}
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
				{#each Array(6) as _}
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
						<input type="checkbox" bind:checked={selectAll} onchange={toggleSelectAll} />
						<span>Select All ({posts.length})</span>
					</label>
				</div>

				{#each posts as post (post.id)}
					<div
						class="post-card"
						class:selected={selectedPosts.has(post.id)}
						transition:scale={{ duration: 200 }}
					>
						<!-- Selection Checkbox -->
						<div class="post-select">
							<input
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
										{formatDateTime(post.publish_at)}
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
									{#each post.categories.slice(0, 3) as categoryId}
										{@const category = typeof categoryId === 'string' ? getPredefinedCategoryById(categoryId) : categoryId}
										{#if category}
											<span
												class="category-badge"
												style:--tag-color={category.color || '#6366f1'}
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
											<button
												onclick={() => {
													schedulePost = post;
													showScheduleModal = true;
												}}
											>
												<IconClock size={16} />
												Schedule
											</button>
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
			<table class="posts-table">
				<thead>
					<tr>
						<th>
							<input type="checkbox" bind:checked={selectAll} onchange={toggleSelectAll} />
						</th>
						<th>Title</th>
						<th>Author</th>
						<th>Status</th>
						<th>Categories</th>
						<th>Views</th>
						<th>SEO</th>
						<th>Published</th>
						<th>Actions</th>
					</tr>
				</thead>
				<tbody>
					{#each posts as post (post.id)}
						<tr class:selected={selectedPosts.has(post.id)}>
							<td>
								<input
									type="checkbox"
									checked={selectedPosts.has(post.id)}
									onchange={() => togglePostSelection(post.id)}
								/>
							</td>
							<td>
								<div class="table-title">
									{#if post.featured}
										<IconStarFilled size={16} class="featured-icon" />
									{/if}
									<a href="/admin/blog/edit/{post.id}">{post.title}</a>
								</div>
							</td>
							<td>{post.author?.name || '-'}</td>
							<td>
								<span class="status-badge status-{post.status}">
									{post.status}
								</span>
							</td>
							<td>
								{#if post.categories?.length > 0}
									<div class="table-category-tags">
										{#each post.categories.slice(0, 2) as categoryId}
											{@const category = typeof categoryId === 'string' ? getPredefinedCategoryById(categoryId) : categoryId}
											{#if category}
												<span class="category-tag-table" style:--tag-color={category.color || '#6366f1'}>{category.name}</span>
											{/if}
										{/each}
										{#if post.categories.length > 2}
											<span class="more-tag">+{post.categories.length - 2}</span>
										{/if}
									</div>
								{:else}
									-
								{/if}
							</td>
							<td>{formatNumber(post.view_count || 0)}</td>
							<td>
								<div class="seo-score-bar" style="--score: {post.seo_score}%">
									{post.seo_score}
								</div>
							</td>
							<td>{post.published_at ? formatDate(post.published_at) : '-'}</td>
							<td>
								<div class="table-actions">
									<button
										class="action-icon"
										onclick={() => goto(`/admin/blog/edit/${post.id}`)}
										title="Edit"
									>
										<IconEdit size={16} />
									</button>
									<button class="action-icon" onclick={() => (previewPost = post)} title="Preview">
										<IconEye size={16} />
									</button>
									<button
										class="action-icon"
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
	{/if}

	<!-- Preview Modal -->
	{#if previewPost}
		<div
			class="modal-overlay"
			role="button"
			tabindex="0"
			onclick={() => (previewPost = null)}
			onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && (previewPost = null)}
		>
			<div
				class="modal preview-modal"
				role="dialog"
				aria-modal="true"
				tabindex="-1"
				onclick={(e: MouseEvent) => e.stopPropagation()}
				onkeydown={(e: KeyboardEvent) => e.stopPropagation()}
			>
				<div class="modal-header">
					<h2>Preview: {previewPost.title}</h2>
					<button class="btn-icon" onclick={() => (previewPost = null)}>
						<IconX size={20} />
					</button>
				</div>
				<div class="modal-content">
					<iframe src="/blog/{previewPost.slug}?preview=true" title="Post Preview"></iframe>
				</div>
			</div>
		</div>
	{/if}

	<!-- Export Modal -->
	{#if showExportModal}
		<div
			class="modal-overlay"
			role="button"
			tabindex="0"
			onclick={() => (showExportModal = false)}
			onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && (showExportModal = false)}
		>
			<div
				class="modal"
				role="dialog"
				aria-modal="true"
				tabindex="-1"
				onclick={(e: MouseEvent) => e.stopPropagation()}
				onkeydown={(e: KeyboardEvent) => e.stopPropagation()}
			>
				<div class="modal-header">
					<h2>Export Posts</h2>
					<button class="btn-icon" onclick={() => (showExportModal = false)}>
						<IconX size={20} />
					</button>
				</div>
				<div class="modal-content">
					<div class="export-options">
						<label>
							<input type="radio" bind:group={exportFormat} value="csv" />
							CSV Format
						</label>
						<label>
							<input type="radio" bind:group={exportFormat} value="json" />
							JSON Format
						</label>
						<label>
							<input type="radio" bind:group={exportFormat} value="wordpress" />
							WordPress XML
						</label>
					</div>
					<p class="export-info">
						{selectedPosts.size > 0
							? `Exporting ${selectedPosts.size} selected posts`
							: 'Exporting all posts matching current filters'}
					</p>
				</div>
				<div class="modal-actions">
					<button class="btn-secondary" onclick={() => (showExportModal = false)}> Cancel </button>
					<button class="btn-primary" onclick={exportPosts}>
						<IconDownload size={18} />
						Export
					</button>
				</div>
			</div>
		</div>
	{/if}

	<!-- Schedule Modal -->
	{#if showScheduleModal && schedulePost}
		<div
			class="modal-overlay"
			role="button"
			tabindex="0"
			onclick={() => (showScheduleModal = false)}
			onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && (showScheduleModal = false)}
		>
			<div
				class="modal"
				role="dialog"
				aria-modal="true"
				tabindex="-1"
				onclick={(e: MouseEvent) => e.stopPropagation()}
				onkeydown={(e: KeyboardEvent) => e.stopPropagation()}
			>
				<div class="modal-header">
					<h2>Schedule Post</h2>
					<button class="btn-icon" onclick={() => (showScheduleModal = false)}>
						<IconX size={20} />
					</button>
				</div>
				<div class="modal-content">
					<p class="schedule-info">Schedule "{schedulePost.title}" for publication</p>
					<div class="schedule-form">
						<label>
							<span>Publish Date & Time</span>
							<input 
								type="datetime-local" 
								class="schedule-input"
								min={new Date().toISOString().slice(0, 16)}
							/>
						</label>
					</div>
				</div>
				<div class="modal-actions">
					<button class="btn-secondary" onclick={() => (showScheduleModal = false)}>Cancel</button>
					<button class="btn-primary" onclick={() => {
						showNotification('info', 'Scheduling feature coming soon');
						showScheduleModal = false;
					}}>
						<IconCalendar size={18} />
						Schedule
					</button>
				</div>
			</div>
		</div>
	{/if}

	<!-- Analytics Modal -->
	{#if showAnalyticsModal && analyticsPost}
		<div
			class="modal-overlay"
			role="button"
			tabindex="0"
			onclick={() => (showAnalyticsModal = false)}
			onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && (showAnalyticsModal = false)}
		>
			<div
				class="modal analytics-modal"
				role="dialog"
				aria-modal="true"
				tabindex="-1"
				onclick={(e: MouseEvent) => e.stopPropagation()}
				onkeydown={(e: KeyboardEvent) => e.stopPropagation()}
			>
				<div class="modal-header">
					<h2>Analytics: {analyticsPost.title}</h2>
					<button class="btn-icon" onclick={() => (showAnalyticsModal = false)}>
						<IconX size={20} />
					</button>
				</div>
				<div class="modal-content">
					{#if analyticsPost.analytics}
						<div class="analytics-grid">
							<div class="analytics-card">
								<h3>Performance</h3>
								<div class="metric-row">
									<span>Total Views</span>
									<strong>{formatNumber(analyticsPost.analytics.views)}</strong>
								</div>
								<div class="metric-row">
									<span>Unique Visitors</span>
									<strong>{formatNumber(analyticsPost.analytics.unique_visitors)}</strong>
								</div>
								<div class="metric-row">
									<span>Avg. Time on Page</span>
									<strong>{analyticsPost.analytics.avg_time}s</strong>
								</div>
								<div class="metric-row">
									<span>Bounce Rate</span>
									<strong>{analyticsPost.analytics.bounce_rate}%</strong>
								</div>
							</div>

							<div class="analytics-card">
								<h3>Engagement</h3>
								<div class="metric-row">
									<span>Comments</span>
									<strong>{analyticsPost.analytics.comments}</strong>
								</div>
								<div class="metric-row">
									<span>Shares</span>
									<strong>{analyticsPost.analytics.shares}</strong>
								</div>
								<div class="metric-row">
									<span>Likes</span>
									<strong>{analyticsPost.analytics.likes}</strong>
								</div>
								<div class="metric-row">
									<span>CTR</span>
									<strong>{analyticsPost.analytics.ctr}%</strong>
								</div>
							</div>
						</div>
					{:else}
						<div class="loading">Loading analytics...</div>
					{/if}
				</div>
			</div>
		</div>
	{/if}
</div>

<!-- Keyboard Shortcuts Helper -->
<div class="keyboard-shortcuts">
	<span>⌘N New</span>
	<span>⌘F Search</span>
	<span>⌘A Select All</span>
	<span>V Toggle View</span>
	<span>R Refresh</span>
</div>

<style>
	.blog-page {
		padding: 2rem;
		max-width: 1600px;
		margin: 0 auto;
		background: var(--admin-bg);
		min-height: 100vh;
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
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 2rem;
	}

	.page-header h1 {
		font-size: 2rem;
		font-weight: 700;
		color: var(--admin-text-primary);
		margin-bottom: 0.5rem;
	}

	.page-header p {
		color: var(--admin-text-muted);
		font-size: 0.95rem;
	}

	.header-actions {
		display: flex;
		gap: 0.75rem;
	}

	/* Buttons - Using RTP Admin Color System */
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
		background: var(--admin-btn-primary-bg);
		color: var(--admin-btn-primary-text);
		box-shadow: 0 2px 8px rgba(230, 184, 0, 0.25);
	}

	.btn-primary:hover {
		transform: translateY(-2px);
		box-shadow: 0 8px 20px rgba(230, 184, 0, 0.4);
	}

	.btn-secondary {
		background: var(--admin-btn-bg);
		color: var(--admin-text-primary);
		border: 1px solid var(--admin-border);
	}

	.btn-secondary:hover {
		background: var(--admin-btn-bg-hover);
		border-color: var(--admin-border-interactive);
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
		background: var(--admin-card-bg);
		border: 1px solid var(--admin-card-border);
		padding: 1.5rem;
		border-radius: 12px;
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
		background: linear-gradient(135deg, rgba(59, 130, 246, 0.1), rgba(139, 92, 246, 0.05));
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
		border-radius: 12px;
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
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 12px;
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
		background: linear-gradient(135deg, rgba(59, 130, 246, 0.2), rgba(139, 92, 246, 0.2));
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
		background: color-mix(in srgb, var(--tag-color, #6366f1) 15%, transparent);
		border: 1px solid color-mix(in srgb, var(--tag-color, #6366f1) 30%, transparent);
		border-radius: 4px;
		font-size: 0.7rem;
		font-weight: 500;
		color: var(--tag-color, #6366f1);
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
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 12px;
		overflow: hidden;
	}

	.posts-table {
		width: 100%;
		border-collapse: collapse;
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
		background: color-mix(in srgb, var(--tag-color, #6366f1) 15%, transparent);
		border: 1px solid color-mix(in srgb, var(--tag-color, #6366f1) 30%, transparent);
		border-radius: 4px;
		font-size: 0.7rem;
		font-weight: 500;
		color: var(--tag-color, #6366f1);
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

	/* Modals */
	.modal-overlay {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background: rgba(0, 0, 0, 0.8);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
	}

	.modal {
		background: rgba(30, 41, 59, 0.98);
		border: 1px solid rgba(148, 163, 184, 0.3);
		border-radius: 12px;
		width: 90%;
		max-width: 600px;
		max-height: 90vh;
		overflow: hidden;
		display: flex;
		flex-direction: column;
	}

	.modal.preview-modal {
		max-width: 1200px;
		height: 80vh;
	}

	.modal.analytics-modal {
		max-width: 800px;
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1.5rem;
		border-bottom: 1px solid rgba(148, 163, 184, 0.2);
	}

	.modal-header h2 {
		font-size: 1.5rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0;
	}

	.modal-content {
		flex: 1;
		padding: 1.5rem;
		overflow-y: auto;
	}

	.modal-content iframe {
		width: 100%;
		height: 100%;
		border: none;
		border-radius: 8px;
	}

	.modal-actions {
		display: flex;
		justify-content: flex-end;
		gap: 1rem;
		padding: 1.5rem;
		border-top: 1px solid rgba(148, 163, 184, 0.2);
	}

	/* Export Modal */
	.export-options {
		display: flex;
		flex-direction: column;
		gap: 1rem;
		margin-bottom: 1.5rem;
	}

	.export-options label {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		cursor: pointer;
		color: #cbd5e1;
		transition: all 0.2s;
	}

	.export-options label:hover {
		border-color: rgba(59, 130, 246, 0.3);
	}

	.export-options label:has(input[type='radio']:checked) {
		background: rgba(59, 130, 246, 0.1);
		border-color: #3b82f6;
		color: #3b82f6;
	}

	.export-info {
		color: #94a3b8;
		font-size: 0.9rem;
		padding: 1rem;
		background: rgba(59, 130, 246, 0.1);
		border-radius: 8px;
	}

	/* Schedule Modal */
	.schedule-info {
		color: #f1f5f9;
		font-size: 1rem;
		margin-bottom: 1.5rem;
	}

	.schedule-form label {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.schedule-form label span {
		color: #94a3b8;
		font-size: 0.9rem;
	}

	.schedule-input {
		padding: 0.75rem 1rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.3);
		border-radius: 8px;
		color: #f1f5f9;
		font-size: 1rem;
	}

	.schedule-input:focus {
		outline: none;
		border-color: #3b82f6;
	}

	/* Analytics Modal */
	.analytics-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
		gap: 1.5rem;
	}

	.analytics-card {
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		padding: 1.5rem;
	}

	.analytics-card h3 {
		font-size: 1.1rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0 0 1rem 0;
	}

	.metric-row {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 0.75rem 0;
		border-bottom: 1px solid rgba(148, 163, 184, 0.1);
	}

	.metric-row:last-child {
		border-bottom: none;
	}

	.metric-row span {
		color: #94a3b8;
	}

	.metric-row strong {
		color: #f1f5f9;
		font-weight: 600;
	}

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

	.keyboard-shortcuts span {
		display: flex;
		align-items: center;
		gap: 0.25rem;
	}

	/* Responsive */
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

		.posts-table-container {
			overflow-x: auto;
		}
	}
</style>

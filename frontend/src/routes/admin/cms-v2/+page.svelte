<!--
	CMS v2 Dashboard
	═══════════════════════════════════════════════════════════════════════════════

	Overview dashboard for the CMS v2 system showing:
	- Content statistics
	- Recent activity
	- Quick actions
	- System health

	@version 1.0.0
	@author Revolution Trading Pros
	@since January 2026
-->

<script lang="ts">
	import { browser } from '$app/environment';
	import { fly, scale } from 'svelte/transition';
	import { backOut } from 'svelte/easing';
	import { cmsApi, type CmsStats } from '$lib/api/cms-v2';
	import {
		IconFileText,
		IconPhoto,
		IconTags,
		IconNavigation,
		IconArrowUpRight,
		IconRefresh,
		IconPlus,
		IconEdit,
		IconClock,
		IconCheck,
		IconAlertTriangle,
		IconActivity
	} from '$lib/icons';

	// ═══════════════════════════════════════════════════════════════════════════
	// State
	// ═══════════════════════════════════════════════════════════════════════════

	let isLoading = $state(true);
	let stats = $state<CmsStats | null>(null);
	let recentContent = $state<any[]>([]);
	let error = $state<string | null>(null);

	// ═══════════════════════════════════════════════════════════════════════════
	// Lifecycle
	// ═══════════════════════════════════════════════════════════════════════════

	$effect(() => {
		if (!browser) return;
		loadData();
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// Data Loading
	// ═══════════════════════════════════════════════════════════════════════════

	async function loadData() {
		isLoading = true;
		error = null;

		try {
			const [statsResponse, contentResponse] = await Promise.all([
				cmsApi.getStats(),
				cmsApi.listContent({ limit: 5, offset: 0 })
			]);

			stats = statsResponse;
			recentContent = contentResponse.data;
		} catch (e: any) {
			error = e.message || 'Failed to load dashboard data';
			console.error('Dashboard error:', e);
		} finally {
			isLoading = false;
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Helpers
	// ═══════════════════════════════════════════════════════════════════════════

	function formatDate(dateString: string): string {
		return new Date(dateString).toLocaleDateString('en-US', {
			month: 'short',
			day: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	function getStatusColor(status: string): string {
		switch (status) {
			case 'published':
				return 'green';
			case 'draft':
				return 'gray';
			case 'in_review':
				return 'yellow';
			case 'scheduled':
				return 'blue';
			default:
				return 'gray';
		}
	}
</script>

<svelte:head>
	<title>CMS v2 Dashboard | Admin</title>
</svelte:head>

<div class="cms-dashboard">
	<!-- Header -->
	<header class="dashboard-header" in:fly={{ y: -20, duration: 400 }}>
		<div class="header-content">
			<h1 class="page-title">CMS Dashboard</h1>
			<p class="page-subtitle">Manage your content, assets, and site structure</p>
		</div>
		<button class="btn-refresh" onclick={loadData} disabled={isLoading}>
			<IconRefresh size={18} class={isLoading ? 'spinning' : ''} />
		</button>
	</header>

	{#if error}
		<div class="error-banner" in:scale={{ duration: 200 }}>
			<IconAlertTriangle size={20} />
			<span>{error}</span>
			<button onclick={() => (error = null)}>Dismiss</button>
		</div>
	{/if}

	<!-- Stats Grid -->
	<section class="stats-section" in:fly={{ y: 20, duration: 400, delay: 100 }}>
		<div class="stats-grid">
			{#each [{ href: '/admin/cms-v2/content', icon: IconFileText, label: 'Content Items', value: stats?.total_content ?? 0, sublabel: `${stats?.published_content ?? 0} published`, color: 'indigo' }, { href: '/admin/cms-v2/assets', icon: IconPhoto, label: 'Assets', value: stats?.total_assets ?? 0, sublabel: `${stats?.total_folders ?? 0} folders`, color: 'violet' }, { href: '/admin/cms-v2/tags', icon: IconTags, label: 'Tags', value: stats?.total_tags ?? 0, sublabel: 'Organize content', color: 'amber' }, { href: '/admin/cms-v2/menus', icon: IconNavigation, label: 'Menus', value: stats?.total_menus ?? 0, sublabel: 'Site navigation', color: 'emerald' }] as item, i}
				{@const Icon = item.icon}
				<a
					href={item.href}
					class="stat-card stat-{item.color}"
					in:scale={{ duration: 400, delay: 150 + i * 50, start: 0.95, easing: backOut }}
				>
					<div class="stat-icon">
						<Icon size={24} />
					</div>
					<div class="stat-content">
						<span class="stat-value">
							{#if isLoading}
								<span class="skeleton">--</span>
							{:else}
								{item.value.toLocaleString()}
							{/if}
						</span>
						<span class="stat-label">{item.label}</span>
						<span class="stat-sublabel">{item.sublabel}</span>
					</div>
					<div class="stat-arrow">
						<IconArrowUpRight size={16} />
					</div>
				</a>
			{/each}
		</div>
	</section>

	<!-- Main Grid -->
	<div class="main-grid">
		<!-- Recent Content -->
		<section class="panel" in:fly={{ x: -20, duration: 400, delay: 200 }}>
			<div class="panel-header">
				<div class="panel-title-group">
					<IconActivity size={20} />
					<div>
						<h2 class="panel-title">Recent Content</h2>
						<p class="panel-subtitle">Latest updates</p>
					</div>
				</div>
				<a href="/admin/cms-v2/content" class="panel-action">
					View All <IconArrowUpRight size={14} />
				</a>
			</div>

			<div class="panel-content">
				{#if isLoading}
					<div class="loading-skeleton">
						{#each Array(4) as _, i}
							<div class="skeleton-row" style="animation-delay: {i * 100}ms"></div>
						{/each}
					</div>
				{:else if recentContent.length === 0}
					<div class="empty-state">
						<IconFileText size={40} />
						<h3>No content yet</h3>
						<p>Create your first content item to get started</p>
						<a href="/admin/cms-v2/content/create" class="btn-primary">
							<IconPlus size={16} />
							Create Content
						</a>
					</div>
				{:else}
					<ul class="content-list">
						{#each recentContent as content, i}
							<li in:fly={{ x: -10, duration: 300, delay: i * 50 }}>
								<a href="/admin/cms-v2/content/{content.id}" class="content-item">
									<div class="content-icon">
										<IconFileText size={18} />
									</div>
									<div class="content-details">
										<span class="content-title">{content.title}</span>
										<span class="content-meta">
											{content.content_type} · {formatDate(content.updated_at)}
										</span>
									</div>
									<span class="status-badge status-{getStatusColor(content.status)}">
										{content.status}
									</span>
								</a>
							</li>
						{/each}
					</ul>
				{/if}
			</div>
		</section>

		<!-- Quick Actions -->
		<section class="panel" in:fly={{ x: 20, duration: 400, delay: 200 }}>
			<div class="panel-header">
				<div class="panel-title-group">
					<IconPlus size={20} />
					<div>
						<h2 class="panel-title">Quick Actions</h2>
						<p class="panel-subtitle">Common tasks</p>
					</div>
				</div>
			</div>

			<div class="panel-content">
				<div class="actions-grid">
					{#each [{ href: '/admin/cms-v2/content/create', icon: IconPlus, label: 'New Page', desc: 'Create a new page' }, { href: '/admin/cms-v2/content/create?type=blog_post', icon: IconEdit, label: 'New Blog Post', desc: 'Write a blog post' }, { href: '/admin/cms-v2/assets', icon: IconPhoto, label: 'Upload Assets', desc: 'Add images & files' }, { href: '/admin/cms-v2/menus', icon: IconNavigation, label: 'Edit Menu', desc: 'Update navigation' }] as action, i}
						{@const ActionIcon = action.icon}
						<a
							href={action.href}
							class="action-card"
							in:scale={{ duration: 300, delay: 250 + i * 50, start: 0.95 }}
						>
							<div class="action-icon">
								<ActionIcon size={20} />
							</div>
							<div class="action-content">
								<span class="action-label">{action.label}</span>
								<span class="action-desc">{action.desc}</span>
							</div>
						</a>
					{/each}
				</div>
			</div>
		</section>
	</div>

	<!-- Workflow Status -->
	<section class="workflow-section" in:fly={{ y: 20, duration: 400, delay: 300 }}>
		<div class="section-header">
			<h2 class="section-title">Content Workflow</h2>
		</div>

		<div class="workflow-grid">
			{#each [{ status: 'draft', icon: IconEdit, label: 'Drafts', count: stats?.draft_content ?? 0, color: 'gray' }, { status: 'in_review', icon: IconClock, label: 'In Review', count: stats?.in_review_content ?? 0, color: 'yellow' }, { status: 'scheduled', icon: IconClock, label: 'Scheduled', count: stats?.scheduled_content ?? 0, color: 'blue' }, { status: 'published', icon: IconCheck, label: 'Published', count: stats?.published_content ?? 0, color: 'green' }] as item}
				{@const StatusIcon = item.icon}
				<div class="workflow-card workflow-{item.color}">
					<div class="workflow-icon">
						<StatusIcon size={20} />
					</div>
					<div class="workflow-content">
						<span class="workflow-count">
							{#if isLoading}
								--
							{:else}
								{item.count}
							{/if}
						</span>
						<span class="workflow-label">{item.label}</span>
					</div>
				</div>
			{/each}
		</div>
	</section>
</div>

<style>
	.cms-dashboard {
		padding: 1.5rem;
		max-width: 1400px;
		margin: 0 auto;
	}

	/* Header */
	.dashboard-header {
		display: flex;
		align-items: flex-start;
		justify-content: space-between;
		margin-bottom: 2rem;
	}

	.page-title {
		font-size: 1.75rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0 0 0.25rem 0;
	}

	.page-subtitle {
		font-size: 0.9375rem;
		color: #64748b;
		margin: 0;
	}

	.btn-refresh {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 40px;
		height: 40px;
		background: rgba(51, 65, 85, 0.3);
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-radius: 0.5rem;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.15s;
	}

	.btn-refresh:hover {
		background: rgba(51, 65, 85, 0.5);
		color: #f1f5f9;
	}

	.btn-refresh :global(.spinning) {
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	/* Error Banner */
	.error-banner {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem 1.25rem;
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		border-radius: 0.5rem;
		color: #f87171;
		margin-bottom: 1.5rem;
	}

	.error-banner button {
		margin-left: auto;
		padding: 0.375rem 0.75rem;
		background: transparent;
		border: 1px solid rgba(239, 68, 68, 0.3);
		border-radius: 0.375rem;
		color: #f87171;
		font-size: 0.8125rem;
		cursor: pointer;
		transition: all 0.15s;
	}

	.error-banner button:hover {
		background: rgba(239, 68, 68, 0.1);
	}

	/* Stats Section */
	.stats-section {
		margin-bottom: 2rem;
	}

	.stats-grid {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 1rem;
	}

	.stat-card {
		position: relative;
		display: flex;
		align-items: flex-start;
		gap: 1rem;
		padding: 1.25rem;
		background: rgba(30, 41, 59, 0.5);
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-radius: 0.75rem;
		text-decoration: none;
		transition: all 0.25s;
		overflow: hidden;
	}

	.stat-card:hover {
		transform: translateY(-2px);
		border-color: rgba(230, 184, 0, 0.3);
		box-shadow: 0 8px 24px rgba(0, 0, 0, 0.2);
	}

	.stat-icon {
		width: 48px;
		height: 48px;
		display: flex;
		align-items: center;
		justify-content: center;
		border-radius: 0.75rem;
		flex-shrink: 0;
	}

	.stat-indigo .stat-icon {
		background: rgba(99, 102, 241, 0.15);
		color: #818cf8;
	}

	.stat-violet .stat-icon {
		background: rgba(139, 92, 246, 0.15);
		color: #a78bfa;
	}

	.stat-amber .stat-icon {
		background: rgba(245, 158, 11, 0.15);
		color: #fbbf24;
	}

	.stat-emerald .stat-icon {
		background: rgba(16, 185, 129, 0.15);
		color: #34d399;
	}

	.stat-content {
		flex: 1;
		min-width: 0;
	}

	.stat-value {
		display: block;
		font-size: 1.75rem;
		font-weight: 700;
		color: #f1f5f9;
		line-height: 1;
		margin-bottom: 0.25rem;
	}

	.stat-label {
		display: block;
		font-size: 0.875rem;
		font-weight: 500;
		color: #cbd5e1;
	}

	.stat-sublabel {
		display: block;
		font-size: 0.75rem;
		color: #64748b;
		margin-top: 0.125rem;
	}

	.stat-arrow {
		position: absolute;
		top: 1rem;
		right: 1rem;
		color: #475569;
		opacity: 0;
		transform: translate(-4px, 4px);
		transition: all 0.25s;
	}

	.stat-card:hover .stat-arrow {
		opacity: 1;
		transform: translate(0, 0);
		color: #e6b800;
	}

	.skeleton {
		opacity: 0.5;
		animation: pulse 1.5s ease-in-out infinite;
	}

	@keyframes pulse {
		0%,
		100% {
			opacity: 0.5;
		}
		50% {
			opacity: 0.3;
		}
	}

	/* Main Grid */
	.main-grid {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1.5rem;
		margin-bottom: 2rem;
	}

	/* Panel */
	.panel {
		background: rgba(30, 41, 59, 0.5);
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-radius: 0.75rem;
		overflow: hidden;
	}

	.panel-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 1rem 1.25rem;
		border-bottom: 1px solid rgba(51, 65, 85, 0.5);
	}

	.panel-title-group {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		color: #e6b800;
	}

	.panel-title {
		font-size: 1rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0;
	}

	.panel-subtitle {
		font-size: 0.75rem;
		color: #64748b;
		margin: 0;
	}

	.panel-action {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		font-size: 0.8125rem;
		font-weight: 500;
		color: #e6b800;
		text-decoration: none;
		transition: opacity 0.15s;
	}

	.panel-action:hover {
		opacity: 0.8;
	}

	.panel-content {
		padding: 1.25rem;
	}

	/* Content List */
	.content-list {
		list-style: none;
		margin: 0;
		padding: 0;
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.content-item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem;
		background: rgba(0, 0, 0, 0.2);
		border-radius: 0.5rem;
		text-decoration: none;
		transition: all 0.15s;
	}

	.content-item:hover {
		background: rgba(51, 65, 85, 0.5);
	}

	.content-icon {
		width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(230, 184, 0, 0.1);
		border-radius: 0.375rem;
		color: #e6b800;
		flex-shrink: 0;
	}

	.content-details {
		flex: 1;
		min-width: 0;
	}

	.content-title {
		display: block;
		font-size: 0.875rem;
		font-weight: 500;
		color: #f1f5f9;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.content-meta {
		display: block;
		font-size: 0.75rem;
		color: #64748b;
	}

	.status-badge {
		font-size: 0.6875rem;
		font-weight: 600;
		text-transform: uppercase;
		padding: 0.25rem 0.5rem;
		border-radius: 0.25rem;
	}

	.status-green {
		background: rgba(16, 185, 129, 0.15);
		color: #34d399;
	}

	.status-gray {
		background: rgba(100, 116, 139, 0.15);
		color: #94a3b8;
	}

	.status-yellow {
		background: rgba(245, 158, 11, 0.15);
		color: #fbbf24;
	}

	.status-blue {
		background: rgba(59, 130, 246, 0.15);
		color: #60a5fa;
	}

	/* Actions Grid */
	.actions-grid {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 0.75rem;
	}

	.action-card {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem;
		background: rgba(0, 0, 0, 0.2);
		border: 1px solid transparent;
		border-radius: 0.5rem;
		text-decoration: none;
		transition: all 0.15s;
	}

	.action-card:hover {
		background: rgba(51, 65, 85, 0.5);
		border-color: rgba(230, 184, 0, 0.2);
	}

	.action-icon {
		width: 40px;
		height: 40px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(230, 184, 0, 0.1);
		border-radius: 0.5rem;
		color: #e6b800;
		flex-shrink: 0;
	}

	.action-content {
		flex: 1;
		min-width: 0;
	}

	.action-label {
		display: block;
		font-size: 0.875rem;
		font-weight: 500;
		color: #f1f5f9;
	}

	.action-desc {
		display: block;
		font-size: 0.75rem;
		color: #64748b;
	}

	/* Empty State */
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 2rem;
		text-align: center;
		color: #64748b;
	}

	.empty-state h3 {
		font-size: 1rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 1rem 0 0.25rem 0;
	}

	.empty-state p {
		font-size: 0.875rem;
		margin: 0 0 1rem 0;
	}

	.btn-primary {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1rem;
		background: linear-gradient(135deg, #e6b800, #d4a600);
		border: none;
		border-radius: 0.5rem;
		color: #0f172a;
		font-weight: 600;
		font-size: 0.875rem;
		text-decoration: none;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-primary:hover {
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(230, 184, 0, 0.3);
	}

	/* Loading Skeleton */
	.loading-skeleton {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.skeleton-row {
		height: 56px;
		background: linear-gradient(
			90deg,
			rgba(51, 65, 85, 0.3) 0%,
			rgba(51, 65, 85, 0.5) 50%,
			rgba(51, 65, 85, 0.3) 100%
		);
		background-size: 200% 100%;
		border-radius: 0.5rem;
		animation: shimmer 1.5s ease-in-out infinite;
	}

	@keyframes shimmer {
		0% {
			background-position: 200% 0;
		}
		100% {
			background-position: -200% 0;
		}
	}

	/* Workflow Section */
	.workflow-section {
		margin-bottom: 2rem;
	}

	.section-header {
		margin-bottom: 1rem;
	}

	.section-title {
		font-size: 1.0625rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0;
	}

	.workflow-grid {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 1rem;
	}

	.workflow-card {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1.25rem;
		background: rgba(30, 41, 59, 0.3);
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-radius: 0.75rem;
	}

	.workflow-icon {
		width: 40px;
		height: 40px;
		display: flex;
		align-items: center;
		justify-content: center;
		border-radius: 0.5rem;
	}

	.workflow-gray .workflow-icon {
		background: rgba(100, 116, 139, 0.15);
		color: #94a3b8;
	}

	.workflow-yellow .workflow-icon {
		background: rgba(245, 158, 11, 0.15);
		color: #fbbf24;
	}

	.workflow-blue .workflow-icon {
		background: rgba(59, 130, 246, 0.15);
		color: #60a5fa;
	}

	.workflow-green .workflow-icon {
		background: rgba(16, 185, 129, 0.15);
		color: #34d399;
	}

	.workflow-content {
		flex: 1;
	}

	.workflow-count {
		display: block;
		font-size: 1.5rem;
		font-weight: 700;
		color: #f1f5f9;
		line-height: 1;
	}

	.workflow-label {
		display: block;
		font-size: 0.8125rem;
		color: #94a3b8;
		margin-top: 0.25rem;
	}

	/* Responsive */
	@media (max-width: 1200px) {
		.stats-grid {
			grid-template-columns: repeat(2, 1fr);
		}

		.workflow-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	@media (max-width: 1024px) {
		.main-grid {
			grid-template-columns: 1fr;
		}
	}

	@media (max-width: 768px) {
		.cms-dashboard {
			padding: 1rem;
		}

		.stats-grid {
			grid-template-columns: 1fr;
		}

		.workflow-grid {
			grid-template-columns: 1fr 1fr;
		}

		.actions-grid {
			grid-template-columns: 1fr;
		}
	}

	@media (max-width: 480px) {
		.workflow-grid {
			grid-template-columns: 1fr;
		}
	}
</style>

<!--
	CMS Dashboard - Apple ICT 11+ Principal Engineer Grade
	January 2026

	Advanced CMS features:
	- Content versioning with rollback
	- Workflow management with approvals
	- Webhook configuration
	- Publish scheduling
	- i18n/Localization
	- Real-time updates via SSE
-->

<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { fly, scale, fade } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';
	import {
		IconHistory,
		IconGitBranch,
		IconWebhook,
		IconCalendarEvent,
		IconWorld,
		IconEye,
		IconChartBar,
		IconRefresh,
		IconArrowUpRight,
		IconActivity,
		IconBell,
		IconCheckCircle2,
		IconClock,
		IconAlertTriangle,
		IconUsers,
		IconFileText,
		IconSettings,
		IconPlug,
		IconLanguage,
		IconCalendar,
		IconGitCommit,
		IconSend
	} from '$lib/icons';

	let mounted = false;
	let isLoading = true;
	let error: string | null = null;
	let sseConnected = false;
	let eventSource: EventSource | null = null;
	let recentEvents: Array<{type: string; data: any; timestamp: Date}> = [];

	// CMS Stats
	let stats = {
		versions: { total: 0 },
		workflows: { pending: 0 },
		webhooks: { active: 0, pending_deliveries: 0 },
		scheduling: { upcoming: 0 },
		previews: { active: 0 },
		localization: { active_locales: 0, total_translations: 0 }
	};

	// Pending assignments for current user
	let myAssignments: any[] = [];

	async function fetchCmsStats() {
		isLoading = true;
		error = null;
		try {
			const response = await fetch('/api/admin/cms/stats', {
				credentials: 'include'
			});
			if (response.ok) {
				stats = await response.json();
			}
		} catch (e) {
			console.error('Failed to fetch CMS stats:', e);
			error = 'Failed to load CMS statistics';
		} finally {
			isLoading = false;
		}
	}

	async function fetchMyAssignments() {
		try {
			const response = await fetch('/api/admin/cms/workflow/my-assignments', {
				credentials: 'include'
			});
			if (response.ok) {
				myAssignments = await response.json();
			}
		} catch (e) {
			console.error('Failed to fetch assignments:', e);
		}
	}

	function connectToSSE() {
		if (typeof EventSource === 'undefined') return;

		eventSource = new EventSource('/api/realtime/events');

		eventSource.onopen = () => {
			sseConnected = true;
			console.log('SSE connected');
		};

		eventSource.onerror = () => {
			sseConnected = false;
			console.log('SSE disconnected, reconnecting...');
			setTimeout(connectToSSE, 5000);
		};

		eventSource.addEventListener('content.created', (e) => {
			addEvent('content.created', JSON.parse(e.data));
		});

		eventSource.addEventListener('content.updated', (e) => {
			addEvent('content.updated', JSON.parse(e.data));
		});

		eventSource.addEventListener('content.published', (e) => {
			addEvent('content.published', JSON.parse(e.data));
		});

		eventSource.addEventListener('workflow.transition', (e) => {
			addEvent('workflow.transition', JSON.parse(e.data));
			fetchCmsStats(); // Refresh stats
		});

		eventSource.addEventListener('notification', (e) => {
			addEvent('notification', JSON.parse(e.data));
		});
	}

	function addEvent(type: string, data: any) {
		recentEvents = [
			{ type, data, timestamp: new Date() },
			...recentEvents.slice(0, 9)
		];
	}

	function formatTimeAgo(date: Date): string {
		const seconds = Math.floor((new Date().getTime() - date.getTime()) / 1000);
		if (seconds < 60) return 'just now';
		if (seconds < 3600) return `${Math.floor(seconds / 60)}m ago`;
		if (seconds < 86400) return `${Math.floor(seconds / 3600)}h ago`;
		return `${Math.floor(seconds / 86400)}d ago`;
	}

	function getPriorityColor(priority: string): string {
		switch (priority) {
			case 'urgent': return 'red';
			case 'high': return 'orange';
			case 'normal': return 'blue';
			case 'low': return 'gray';
			default: return 'gray';
		}
	}

	onMount(() => {
		mounted = true;
		fetchCmsStats();
		fetchMyAssignments();
		connectToSSE();
	});

	onDestroy(() => {
		if (eventSource) {
			eventSource.close();
		}
	});
</script>

<div class="cms-dashboard">
	<!-- Header -->
	<header class="dashboard-header" in:fly={{ y: -20, duration: 400 }}>
		<div class="header-left">
			<h1 class="dashboard-title">Content Management</h1>
			<p class="dashboard-subtitle">
				Advanced CMS features & workflow management
				<span class="connection-status" class:connected={sseConnected}>
					<span class="status-dot"></span>
					{sseConnected ? 'Live' : 'Connecting...'}
				</span>
			</p>
		</div>
		<div class="header-right">
			<button class="refresh-btn" onclick={fetchCmsStats} disabled={isLoading} class:loading={isLoading}>
				<IconRefresh size={20} />
			</button>
		</div>
	</header>

	{#if error}
		<div class="error-banner" in:fly={{ y: -10, duration: 300 }}>
			<IconAlertTriangle size={20} />
			<span>{error}</span>
		</div>
	{/if}

	<!-- CMS Stats Overview -->
	<section class="stats-grid" in:fly={{ y: 20, duration: 500, delay: 100 }}>
		{#each [
			{ href: '/admin/cms/versions', icon: IconHistory, value: stats.versions.total, label: 'Content Versions', desc: 'Version history & rollback', color: 'indigo' },
			{ href: '/admin/cms/workflows', icon: IconGitBranch, value: stats.workflows.pending, label: 'Pending Reviews', desc: 'Workflow approvals', color: 'amber' },
			{ href: '/admin/cms/webhooks', icon: IconWebhook, value: stats.webhooks.active, label: 'Active Webhooks', desc: `${stats.webhooks.pending_deliveries} pending`, color: 'purple' },
			{ href: '/admin/cms/scheduled', icon: IconCalendarEvent, value: stats.scheduling.upcoming, label: 'Scheduled', desc: 'Upcoming publications', color: 'cyan' },
			{ href: '/admin/cms/previews', icon: IconEye, value: stats.previews.active, label: 'Active Previews', desc: 'Preview tokens', color: 'pink' },
			{ href: '/admin/cms/locales', icon: IconWorld, value: stats.localization.active_locales, label: 'Languages', desc: `${stats.localization.total_translations} translations`, color: 'emerald' }
		] as item, i}
			{@const StatIcon = item.icon}
			<a href={item.href} class="stat-card {item.color}" in:scale={{ duration: 400, delay: 150 + i * 50, easing: cubicOut }}>
				<div class="stat-icon {item.color}">
					<StatIcon size={24} />
				</div>
				<div class="stat-content">
					<span class="stat-value">
						{#if isLoading}...{:else}{item.value}{/if}
					</span>
					<span class="stat-label">{item.label}</span>
					<span class="stat-desc">{item.desc}</span>
				</div>
				<div class="stat-arrow">
					<IconArrowUpRight size={18} />
				</div>
			</a>
		{/each}
	</section>

	<!-- Two Column Layout -->
	<div class="two-column">
		<!-- My Assignments -->
		<section class="panel glass-panel" in:fly={{ x: -20, duration: 500, delay: 300 }}>
			<div class="panel-header">
				<div class="panel-title">
					<div class="panel-icon assignments-icon">
						<IconUsers size={22} />
					</div>
					<div>
						<h2>My Assignments</h2>
						<span class="panel-subtitle">Content awaiting your review</span>
					</div>
				</div>
				<a href="/admin/cms/workflows" class="panel-link">
					View All <IconArrowUpRight size={14} />
				</a>
			</div>

			{#if myAssignments.length === 0}
				<div class="empty-state">
					<IconCheckCircle2 size={48} />
					<p>No pending assignments</p>
				</div>
			{:else}
				<div class="assignments-list">
					{#each myAssignments.slice(0, 5) as assignment}
						<a href="/admin/cms/workflows/{assignment.content_type}/{assignment.content_id}" class="assignment-item">
							<div class="assignment-icon">
								<IconFileText size={18} />
							</div>
							<div class="assignment-info">
								<span class="assignment-type">{assignment.content_type}</span>
								<span class="assignment-id">#{assignment.content_id}</span>
							</div>
							<div class="assignment-meta">
								<span class="assignment-stage badge-{assignment.current_stage}">{assignment.current_stage}</span>
								{#if assignment.priority}
									<span class="assignment-priority priority-{getPriorityColor(assignment.priority)}">{assignment.priority}</span>
								{/if}
							</div>
							{#if assignment.due_date}
								<div class="assignment-due">
									<IconClock size={14} />
									<span>{new Date(assignment.due_date).toLocaleDateString()}</span>
								</div>
							{/if}
						</a>
					{/each}
				</div>
			{/if}
		</section>

		<!-- Recent Activity -->
		<section class="panel glass-panel" in:fly={{ x: 20, duration: 500, delay: 300 }}>
			<div class="panel-header">
				<div class="panel-title">
					<div class="panel-icon activity-icon">
						<IconActivity size={22} />
					</div>
					<div>
						<h2>Live Activity</h2>
						<span class="panel-subtitle">Real-time CMS events</span>
					</div>
				</div>
				<div class="live-indicator" class:active={sseConnected}>
					<span class="live-dot"></span>
					LIVE
				</div>
			</div>

			{#if recentEvents.length === 0}
				<div class="empty-state">
					<IconBell size={48} />
					<p>Waiting for events...</p>
				</div>
			{:else}
				<div class="activity-list">
					{#each recentEvents as event, i}
						<div class="activity-item" in:fly={{ x: 20, duration: 300, delay: i * 50 }}>
							<div class="activity-icon event-{event.type.split('.')[0]}">
								{#if event.type.startsWith('content')}
									<IconFileText size={16} />
								{:else if event.type.startsWith('workflow')}
									<IconGitBranch size={16} />
								{:else}
									<IconBell size={16} />
								{/if}
							</div>
							<div class="activity-info">
								<span class="activity-type">{event.type}</span>
								{#if event.data.content_type}
									<span class="activity-detail">{event.data.content_type} #{event.data.content_id}</span>
								{/if}
							</div>
							<span class="activity-time">{formatTimeAgo(event.timestamp)}</span>
						</div>
					{/each}
				</div>
			{/if}
		</section>
	</div>

	<!-- Quick Actions -->
	<section class="quick-actions" in:fly={{ y: 20, duration: 500, delay: 400 }}>
		<h3 class="section-title">Quick Actions</h3>
		<div class="actions-grid">
			{#each [
				{ href: '/admin/cms/versions', icon: IconGitCommit, label: 'Version History', desc: 'Browse content versions', color: 'indigo' },
				{ href: '/admin/cms/workflows', icon: IconGitBranch, label: 'Workflow Manager', desc: 'Manage approval workflows', color: 'amber' },
				{ href: '/admin/cms/webhooks', icon: IconSend, label: 'Webhooks', desc: 'Configure integrations', color: 'purple' },
				{ href: '/admin/cms/scheduled', icon: IconCalendar, label: 'Scheduler', desc: 'Schedule publications', color: 'cyan' },
				{ href: '/admin/cms/locales', icon: IconLanguage, label: 'Localization', desc: 'Manage translations', color: 'emerald' },
				{ href: '/admin/cms/audit', icon: IconActivity, label: 'Audit Logs', desc: 'View activity history', color: 'gray' }
			] as action}
				{@const ActionIcon = action.icon}
				<a href={action.href} class="action-card">
					<div class="action-icon {action.color}">
						<ActionIcon size={24} />
					</div>
					<div class="action-info">
						<span class="action-label">{action.label}</span>
						<span class="action-desc">{action.desc}</span>
					</div>
				</a>
			{/each}
		</div>
	</section>
</div>

<style>
	.cms-dashboard {
		max-width: 1600px;
		padding: 0 1rem;
	}

	/* Header */
	.dashboard-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 2rem;
		flex-wrap: wrap;
		gap: 1rem;
	}

	.dashboard-title {
		font-size: 2rem;
		font-weight: 800;
		background: linear-gradient(135deg, var(--admin-text-primary) 0%, var(--admin-text-secondary) 100%);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
		margin: 0 0 0.25rem 0;
	}

	.dashboard-subtitle {
		font-size: 0.95rem;
		color: var(--admin-text-muted);
		margin: 0;
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.connection-status {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.8rem;
		padding: 0.25rem 0.75rem;
		border-radius: 20px;
		background: rgba(148, 163, 184, 0.1);
		color: var(--admin-text-muted);
	}

	.connection-status.connected {
		background: rgba(34, 197, 94, 0.1);
		color: #4ade80;
	}

	.status-dot {
		width: 6px;
		height: 6px;
		border-radius: 50%;
		background: currentColor;
	}

	.connection-status.connected .status-dot {
		animation: pulse 2s infinite;
	}

	@keyframes pulse {
		0%, 100% { opacity: 1; }
		50% { opacity: 0.5; }
	}

	.refresh-btn {
		width: 44px;
		height: 44px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: var(--admin-surface);
		border: 1px solid var(--admin-border);
		border-radius: 12px;
		color: var(--admin-text-secondary);
		cursor: pointer;
		transition: all 0.25s;
	}

	.refresh-btn:hover {
		background: var(--admin-surface-elevated);
		color: var(--admin-accent-primary);
		border-color: var(--admin-accent-primary);
	}

	.refresh-btn.loading :global(svg) {
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		from { transform: rotate(0deg); }
		to { transform: rotate(360deg); }
	}

	.error-banner {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem 1.25rem;
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		border-radius: 12px;
		color: #f87171;
		margin-bottom: 1.5rem;
	}

	/* Stats Grid */
	.stats-grid {
		display: grid;
		grid-template-columns: repeat(6, 1fr);
		gap: 1rem;
		margin-bottom: 2rem;
	}

	@media (max-width: 1400px) {
		.stats-grid { grid-template-columns: repeat(3, 1fr); }
	}

	@media (max-width: 768px) {
		.stats-grid { grid-template-columns: repeat(2, 1fr); }
	}

	.stat-card {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1.25rem;
		background: var(--admin-surface);
		border: 1px solid var(--admin-border);
		border-radius: 16px;
		text-decoration: none;
		transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
		position: relative;
		overflow: hidden;
	}

	.stat-card:hover {
		transform: translateY(-4px);
		box-shadow: var(--admin-card-shadow-hover);
	}

	.stat-card.indigo:hover { border-color: rgba(99, 102, 241, 0.5); }
	.stat-card.amber:hover { border-color: rgba(245, 158, 11, 0.5); }
	.stat-card.purple:hover { border-color: rgba(139, 92, 246, 0.5); }
	.stat-card.cyan:hover { border-color: rgba(6, 182, 212, 0.5); }
	.stat-card.pink:hover { border-color: rgba(236, 72, 153, 0.5); }
	.stat-card.emerald:hover { border-color: rgba(16, 185, 129, 0.5); }

	.stat-icon {
		width: 48px;
		height: 48px;
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
	}

	.stat-icon.indigo { background: rgba(99, 102, 241, 0.15); color: #818cf8; }
	.stat-icon.amber { background: rgba(245, 158, 11, 0.15); color: #fbbf24; }
	.stat-icon.purple { background: rgba(139, 92, 246, 0.15); color: #a78bfa; }
	.stat-icon.cyan { background: rgba(6, 182, 212, 0.15); color: #22d3ee; }
	.stat-icon.pink { background: rgba(236, 72, 153, 0.15); color: #f472b6; }
	.stat-icon.emerald { background: rgba(16, 185, 129, 0.15); color: #34d399; }

	.stat-content {
		flex: 1;
		display: flex;
		flex-direction: column;
		gap: 0.125rem;
	}

	.stat-value {
		font-size: 1.5rem;
		font-weight: 800;
		color: var(--admin-text-primary);
	}

	.stat-label {
		font-size: 0.85rem;
		font-weight: 600;
		color: var(--admin-text-primary);
	}

	.stat-desc {
		font-size: 0.75rem;
		color: var(--admin-text-muted);
	}

	.stat-arrow {
		color: var(--admin-text-muted);
		opacity: 0;
		transform: translateX(-10px);
		transition: all 0.3s;
	}

	.stat-card:hover .stat-arrow {
		opacity: 1;
		transform: translateX(0);
		color: var(--admin-accent-primary);
	}

	/* Two Column Layout */
	.two-column {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1.5rem;
		margin-bottom: 2rem;
	}

	@media (max-width: 1024px) {
		.two-column { grid-template-columns: 1fr; }
	}

	/* Glass Panel */
	.glass-panel {
		background: var(--admin-surface);
		border: 1px solid var(--admin-border);
		border-radius: 20px;
		padding: 1.5rem;
		box-shadow: var(--admin-card-shadow);
	}

	.panel-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1.5rem;
	}

	.panel-title {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.panel-title h2 {
		font-size: 1.125rem;
		font-weight: 700;
		color: var(--admin-text-primary);
		margin: 0;
	}

	.panel-subtitle {
		font-size: 0.8rem;
		color: var(--admin-text-muted);
	}

	.panel-icon {
		width: 44px;
		height: 44px;
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.panel-icon.assignments-icon {
		background: rgba(99, 102, 241, 0.15);
		color: #818cf8;
	}

	.panel-icon.activity-icon {
		background: rgba(34, 197, 94, 0.15);
		color: #4ade80;
	}

	.panel-link {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.875rem;
		color: var(--admin-accent-primary);
		text-decoration: none;
		padding: 0.5rem 1rem;
		border-radius: 8px;
		transition: all 0.2s;
	}

	.panel-link:hover {
		background: rgba(99, 102, 241, 0.1);
	}

	.live-indicator {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.7rem;
		font-weight: 700;
		letter-spacing: 0.05em;
		padding: 0.35rem 0.75rem;
		border-radius: 20px;
		background: rgba(148, 163, 184, 0.1);
		color: var(--admin-text-muted);
	}

	.live-indicator.active {
		background: rgba(239, 68, 68, 0.15);
		color: #f87171;
	}

	.live-dot {
		width: 8px;
		height: 8px;
		border-radius: 50%;
		background: currentColor;
	}

	.live-indicator.active .live-dot {
		animation: pulse 1s infinite;
	}

	/* Empty State */
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 3rem;
		color: var(--admin-text-muted);
		text-align: center;
	}

	.empty-state p {
		margin-top: 1rem;
		font-size: 0.9rem;
	}

	/* Assignments List */
	.assignments-list {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.assignment-item {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 0.875rem 1rem;
		background: var(--admin-surface-elevated);
		border: 1px solid var(--admin-border-light);
		border-radius: 12px;
		text-decoration: none;
		transition: all 0.2s;
	}

	.assignment-item:hover {
		border-color: var(--admin-accent-primary);
		background: rgba(99, 102, 241, 0.05);
	}

	.assignment-icon {
		width: 36px;
		height: 36px;
		border-radius: 10px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(99, 102, 241, 0.1);
		color: var(--admin-accent-primary);
	}

	.assignment-info {
		flex: 1;
		display: flex;
		flex-direction: column;
		gap: 0.125rem;
	}

	.assignment-type {
		font-weight: 600;
		font-size: 0.9rem;
		color: var(--admin-text-primary);
		text-transform: capitalize;
	}

	.assignment-id {
		font-size: 0.75rem;
		color: var(--admin-text-muted);
	}

	.assignment-meta {
		display: flex;
		gap: 0.5rem;
	}

	.assignment-stage {
		font-size: 0.7rem;
		font-weight: 600;
		padding: 0.25rem 0.5rem;
		border-radius: 6px;
		text-transform: uppercase;
	}

	.badge-draft { background: rgba(148, 163, 184, 0.15); color: #94a3b8; }
	.badge-review { background: rgba(245, 158, 11, 0.15); color: #fbbf24; }
	.badge-pending_approval { background: rgba(139, 92, 246, 0.15); color: #a78bfa; }
	.badge-approved { background: rgba(34, 197, 94, 0.15); color: #4ade80; }
	.badge-published { background: rgba(99, 102, 241, 0.15); color: #818cf8; }

	.assignment-priority {
		font-size: 0.65rem;
		font-weight: 700;
		padding: 0.2rem 0.4rem;
		border-radius: 4px;
		text-transform: uppercase;
	}

	.priority-red { background: rgba(239, 68, 68, 0.15); color: #f87171; }
	.priority-orange { background: rgba(251, 146, 60, 0.15); color: #fb923c; }
	.priority-blue { background: rgba(59, 130, 246, 0.15); color: #60a5fa; }
	.priority-gray { background: rgba(148, 163, 184, 0.15); color: #94a3b8; }

	.assignment-due {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		font-size: 0.75rem;
		color: var(--admin-text-muted);
	}

	/* Activity List */
	.activity-list {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.activity-item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem;
		background: var(--admin-surface-elevated);
		border-radius: 10px;
	}

	.activity-icon {
		width: 32px;
		height: 32px;
		border-radius: 8px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.activity-icon.event-content {
		background: rgba(59, 130, 246, 0.15);
		color: #60a5fa;
	}

	.activity-icon.event-workflow {
		background: rgba(245, 158, 11, 0.15);
		color: #fbbf24;
	}

	.activity-icon.event-notification {
		background: rgba(236, 72, 153, 0.15);
		color: #f472b6;
	}

	.activity-info {
		flex: 1;
		display: flex;
		flex-direction: column;
		gap: 0.125rem;
	}

	.activity-type {
		font-size: 0.8rem;
		font-weight: 600;
		color: var(--admin-text-primary);
	}

	.activity-detail {
		font-size: 0.7rem;
		color: var(--admin-text-muted);
	}

	.activity-time {
		font-size: 0.7rem;
		color: var(--admin-text-muted);
	}

	/* Quick Actions */
	.quick-actions {
		margin-bottom: 2rem;
	}

	.section-title {
		font-size: 1.125rem;
		font-weight: 700;
		color: var(--admin-text-primary);
		margin-bottom: 1rem;
		padding-bottom: 0.75rem;
		border-bottom: 1px solid var(--admin-border);
	}

	.actions-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
		gap: 1rem;
	}

	.action-card {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1rem 1.25rem;
		background: var(--admin-surface);
		border: 1px solid var(--admin-border);
		border-radius: 14px;
		text-decoration: none;
		transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
	}

	.action-card:hover {
		background: var(--admin-surface-elevated);
		border-color: var(--admin-accent-primary);
		transform: translateX(4px);
	}

	.action-icon {
		width: 48px;
		height: 48px;
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
	}

	.action-icon.indigo { background: rgba(99, 102, 241, 0.15); color: #818cf8; }
	.action-icon.amber { background: rgba(245, 158, 11, 0.15); color: #fbbf24; }
	.action-icon.purple { background: rgba(139, 92, 246, 0.15); color: #a78bfa; }
	.action-icon.cyan { background: rgba(6, 182, 212, 0.15); color: #22d3ee; }
	.action-icon.emerald { background: rgba(16, 185, 129, 0.15); color: #34d399; }
	.action-icon.gray { background: rgba(148, 163, 184, 0.15); color: #94a3b8; }

	.action-info {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.action-label {
		font-weight: 600;
		font-size: 0.95rem;
		color: var(--admin-text-primary);
	}

	.action-desc {
		font-size: 0.8rem;
		color: var(--admin-text-muted);
	}
</style>

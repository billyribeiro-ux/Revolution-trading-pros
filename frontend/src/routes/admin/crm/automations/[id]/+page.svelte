<!--
	/admin/crm/automations/[id] - View Automation Details
	Apple Principal Engineer ICT 7 Grade - January 2026

	Features:
	- Automation overview with stats
	- Visual workflow display
	- Subscriber list with status
	- Activity log
	- Full Svelte 5 $state/$derived/$effect reactivity
-->

<script lang="ts">
	import { page } from '$app/state';
	import { onMount } from 'svelte';
	import IconShare from '@tabler/icons-svelte/icons/share';
	import IconArrowLeft from '@tabler/icons-svelte/icons/arrow-left';
	import IconEdit from '@tabler/icons-svelte/icons/edit';
	import IconPlayerPlay from '@tabler/icons-svelte/icons/player-play';
	import IconPlayerPause from '@tabler/icons-svelte/icons/player-pause';
	import IconUsers from '@tabler/icons-svelte/icons/users';
	import IconChartBar from '@tabler/icons-svelte/icons/chart-bar';
	import IconCheck from '@tabler/icons-svelte/icons/check';
	import IconX from '@tabler/icons-svelte/icons/x';
	import IconAlertCircle from '@tabler/icons-svelte/icons/alert-circle';
	import IconRefresh from '@tabler/icons-svelte/icons/refresh';
	import IconDownload from '@tabler/icons-svelte/icons/download';
	import IconMail from '@tabler/icons-svelte/icons/mail';
	import IconClock from '@tabler/icons-svelte/icons/clock';
	import IconTag from '@tabler/icons-svelte/icons/tag';
	import IconList from '@tabler/icons-svelte/icons/list';
	import IconBolt from '@tabler/icons-svelte/icons/bolt';
	import IconArrowRight from '@tabler/icons-svelte/icons/arrow-right';
	import IconLoader2 from '@tabler/icons-svelte/icons/loader-2';
	import { crmAPI } from '$lib/api/crm';
	import type {
		AutomationFunnel,
		FunnelAction,
		FunnelSubscriber,
		FunnelStats,
		FunnelStatus
	} from '$lib/crm/types';

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let funnel = $state<AutomationFunnel | null>(null);
	let actions = $state<FunnelAction[]>([]);
	let subscribers = $state<FunnelSubscriber[]>([]);
	let stats = $state<FunnelStats | null>(null);
	let triggerTypes = $state<Record<string, string>>({});
	let actionTypes = $state<Record<string, string>>({});

	let isLoading = $state(true);
	let isLoadingSubscribers = $state(false);
	let error = $state('');
	let successMessage = $state('');
	let actionInProgress = $state(false);

	let activeTab = $state<'overview' | 'workflow' | 'subscribers'>('overview');

	const funnelId = page.params.id ?? '';

	// ═══════════════════════════════════════════════════════════════════════════
	// API FUNCTIONS
	// ═══════════════════════════════════════════════════════════════════════════

	async function loadFunnel() {
		isLoading = true;
		error = '';

		try {
			const response = await crmAPI.getAutomationFunnel(funnelId);
			funnel = response.funnel;
			stats = response.stats;
			triggerTypes = response.trigger_types || {};
			actionTypes = response.action_types || {};

			// Load actions
			actions = await crmAPI.getFunnelActions(funnelId);
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load automation';
		} finally {
			isLoading = false;
		}
	}

	async function loadSubscribers() {
		isLoadingSubscribers = true;

		try {
			const response = await crmAPI.getFunnelSubscribers(funnelId, { per_page: 50 });
			subscribers = response.data || [];
		} catch (err) {
			console.error('Failed to load subscribers:', err);
		} finally {
			isLoadingSubscribers = false;
		}
	}

	async function toggleStatus() {
		if (!funnel) return;

		actionInProgress = true;
		error = '';

		try {
			const newStatus: FunnelStatus = funnel.status === 'active' ? 'paused' : 'active';
			await crmAPI.updateAutomationFunnel(funnelId, { status: newStatus });
			showSuccess(`Automation ${newStatus === 'active' ? 'activated' : 'paused'} successfully`);
			await loadFunnel();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to update status';
		} finally {
			actionInProgress = false;
		}
	}

	async function exportFunnel() {
		actionInProgress = true;
		error = '';

		try {
			const data = await crmAPI.exportAutomationFunnel(funnelId);
			const blob = new Blob([JSON.stringify(data, null, 2)], { type: 'application/json' });
			const url = URL.createObjectURL(blob);
			const a = document.createElement('a');
			a.href = url;
			a.download = `automation-${funnelId}-${Date.now()}.json`;
			a.click();
			URL.revokeObjectURL(url);
			showSuccess('Automation exported successfully');
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to export automation';
		} finally {
			actionInProgress = false;
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// HELPERS
	// ═══════════════════════════════════════════════════════════════════════════

	function showSuccess(message: string) {
		successMessage = message;
		setTimeout(() => {
			successMessage = '';
		}, 3000);
	}

	function formatNumber(num: number): string {
		return num.toLocaleString();
	}

	function formatDate(date: string): string {
		return new Date(date).toLocaleDateString('en-US', {
			month: 'short',
			day: 'numeric',
			year: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	function getStatusColor(status: FunnelStatus): string {
		const colors: Record<FunnelStatus, string> = {
			draft: 'bg-slate-500/20 text-slate-400',
			active: 'bg-emerald-500/20 text-emerald-400',
			paused: 'bg-amber-500/20 text-amber-400'
		};
		return colors[status];
	}

	function getSubscriberStatusColor(status: string): string {
		const colors: Record<string, string> = {
			active: 'bg-blue-500/20 text-blue-400',
			waiting: 'bg-amber-500/20 text-amber-400',
			completed: 'bg-emerald-500/20 text-emerald-400',
			cancelled: 'bg-red-500/20 text-red-400',
			failed: 'bg-red-500/20 text-red-400'
		};
		return colors[status] || 'bg-slate-500/20 text-slate-400';
	}

	function getActionIcon(actionType: string) {
		const iconMap: Record<string, typeof IconMail> = {
			send_email: IconMail,
			wait: IconClock,
			add_tag: IconTag,
			remove_tag: IconTag,
			add_to_list: IconList,
			remove_from_list: IconList
		};
		return iconMap[actionType] || IconBolt;
	}

	function getActionColor(actionType: string): string {
		const colorMap: Record<string, string> = {
			send_email: 'bg-blue-500/20 text-blue-400 border-blue-500/30',
			wait: 'bg-amber-500/20 text-amber-400 border-amber-500/30',
			add_tag: 'bg-emerald-500/20 text-emerald-400 border-emerald-500/30',
			remove_tag: 'bg-red-500/20 text-red-400 border-red-500/30',
			add_to_list: 'bg-purple-500/20 text-purple-400 border-purple-500/30',
			remove_from_list: 'bg-orange-500/20 text-orange-400 border-orange-500/30',
			condition: 'bg-cyan-500/20 text-cyan-400 border-cyan-500/30'
		};
		return colorMap[actionType] || 'bg-slate-500/20 text-slate-400 border-slate-500/30';
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let completionRate = $derived(
		stats && stats.total_subscribers > 0
			? ((stats.completed / stats.total_subscribers) * 100).toFixed(1)
			: '0'
	);

	// ═══════════════════════════════════════════════════════════════════════════
	// EFFECTS
	// ═══════════════════════════════════════════════════════════════════════════

	$effect(() => {
		if (activeTab === 'subscribers' && subscribers.length === 0) {
			loadSubscribers();
		}
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// LIFECYCLE
	// ═══════════════════════════════════════════════════════════════════════════

	$effect(() => {
		if (browser) {
			loadFunnel();
		}
	});
</script>

<svelte:head>
	<title>{funnel?.title || 'Automation'} - FluentCRM Pro</title>
</svelte:head>

<div class="automation-view-page">
	{#if isLoading}
		<div class="loading-state">
			<div class="spinner"></div>
			<p>Loading automation...</p>
		</div>
	{:else if error && !funnel}
		<div class="error-state">
			<IconAlertCircle size={48} />
			<h3>Failed to load automation</h3>
			<p>{error}</p>
			<button class="btn-primary" onclick={loadFunnel}>Try Again</button>
		</div>
	{:else if funnel}
		<!-- Header -->
		<div class="page-header">
			<div class="header-content">
				<a href="/admin/crm/automations" class="back-link">
					<IconArrowLeft size={18} />
					Back to Automations
				</a>
				<div class="title-row">
					<div class="funnel-icon">
						<IconShare size={24} />
					</div>
					<div>
						<h1>{funnel.title}</h1>
						{#if funnel.description}
							<p class="description">{funnel.description}</p>
						{/if}
					</div>
					<span class="status-badge {getStatusColor(funnel.status)}">
						{funnel.status}
					</span>
				</div>
			</div>
			<div class="header-actions">
				<button class="btn-icon" onclick={() => loadFunnel()} disabled={isLoading} title="Refresh">
					<IconRefresh size={18} class={isLoading ? 'spinning' : ''} />
				</button>
				<button class="btn-icon" onclick={exportFunnel} disabled={actionInProgress} title="Export">
					<IconDownload size={18} />
				</button>
				{#if funnel.status !== 'draft'}
					<button class="btn-secondary" onclick={toggleStatus} disabled={actionInProgress}>
						{#if funnel.status === 'active'}
							<IconPlayerPause size={18} />
							Pause
						{:else}
							<IconPlayerPlay size={18} />
							Activate
						{/if}
					</button>
				{/if}
				<a href="/admin/crm/automations/{funnelId}/edit" class="btn-primary">
					<IconEdit size={18} />
					Edit Workflow
				</a>
			</div>
		</div>

		<!-- Alerts -->
		{#if error}
			<div class="error-alert">
				<IconAlertCircle size={18} />
				<span>{error}</span>
				<button onclick={() => (error = '')}>
					<IconX size={16} />
				</button>
			</div>
		{/if}

		{#if successMessage}
			<div class="success-alert">
				<IconCheck size={18} />
				<span>{successMessage}</span>
			</div>
		{/if}

		<!-- Stats Cards -->
		{#if stats}
			<div class="stats-grid">
				<div class="stat-card">
					<div class="stat-icon blue">
						<IconUsers size={24} />
					</div>
					<div class="stat-content">
						<span class="stat-value">{formatNumber(stats.total_subscribers)}</span>
						<span class="stat-label">Total Contacts</span>
					</div>
				</div>
				<div class="stat-card">
					<div class="stat-icon green">
						<IconBolt size={24} />
					</div>
					<div class="stat-content">
						<span class="stat-value">{formatNumber(stats.active_subscribers)}</span>
						<span class="stat-label">Active</span>
					</div>
				</div>
				<div class="stat-card">
					<div class="stat-icon purple">
						<IconCheck size={24} />
					</div>
					<div class="stat-content">
						<span class="stat-value">{formatNumber(stats.completed)}</span>
						<span class="stat-label">Completed</span>
					</div>
				</div>
				<div class="stat-card">
					<div class="stat-icon amber">
						<IconChartBar size={24} />
					</div>
					<div class="stat-content">
						<span class="stat-value">{completionRate}%</span>
						<span class="stat-label">Completion Rate</span>
					</div>
				</div>
			</div>
		{/if}

		<!-- Tabs -->
		<div class="tabs">
			<button
				class="tab"
				class:active={activeTab === 'overview'}
				onclick={() => (activeTab = 'overview')}
			>
				Overview
			</button>
			<button
				class="tab"
				class:active={activeTab === 'workflow'}
				onclick={() => (activeTab = 'workflow')}
			>
				Workflow ({actions.length})
			</button>
			<button
				class="tab"
				class:active={activeTab === 'subscribers'}
				onclick={() => (activeTab = 'subscribers')}
			>
				Subscribers ({stats?.total_subscribers || 0})
			</button>
		</div>

		<!-- Tab Content - Layout Shift Free Pattern -->
		<div class="tab-content">
			<!-- Overview Panel -->
			<div 
				class="tab-panel" 
				class:active={activeTab === 'overview'}
				inert={activeTab !== 'overview' ? true : undefined}
			>
				<div class="overview-grid">
					<div class="info-card">
						<h3>Trigger</h3>
						<div class="trigger-info">
							<span class="trigger-label">
								{triggerTypes[funnel.trigger_type] || funnel.trigger_type}
							</span>
							{#if funnel.trigger_settings && Object.keys(funnel.trigger_settings).length > 0}
								<div class="trigger-settings">
									{#each Object.entries(funnel.trigger_settings) as [key, value]}
										<div class="setting-item">
											<span class="setting-key">{key.replace(/_/g, ' ')}:</span>
											<span class="setting-value">{value}</span>
										</div>
									{/each}
								</div>
							{/if}
						</div>
					</div>

					<div class="info-card">
						<h3>Details</h3>
						<div class="details-list">
							<div class="detail-item">
								<span class="detail-label">Created</span>
								<span class="detail-value">{formatDate(funnel.created_at)}</span>
							</div>
							<div class="detail-item">
								<span class="detail-label">Updated</span>
								<span class="detail-value">{formatDate(funnel.updated_at)}</span>
							</div>
							<div class="detail-item">
								<span class="detail-label">Actions</span>
								<span class="detail-value">{actions.length}</span>
							</div>
							<div class="detail-item">
								<span class="detail-label">Total Revenue</span>
								<span class="detail-value">${formatNumber(funnel.total_revenue || 0)}</span>
							</div>
						</div>
					</div>
				</div>
			</div>

			<!-- Workflow Panel -->
			<div 
				class="tab-panel" 
				class:active={activeTab === 'workflow'}
				inert={activeTab !== 'workflow' ? true : undefined}
			>
				{#if actions.length === 0}
					<div class="empty-state">
						<IconBolt size={48} />
						<h3>No actions yet</h3>
						<p>Add actions to build your automation workflow</p>
						<a href="/admin/crm/automations/{funnelId}/edit" class="btn-primary">
							<IconEdit size={18} />
							Edit Workflow
						</a>
					</div>
				{:else}
					<div class="workflow-visual">
						<!-- Trigger Node -->
						<div class="workflow-node trigger-node">
							<div class="node-icon bg-emerald-500/20 text-emerald-400">
								<IconBolt size={20} />
							</div>
							<div class="node-content">
								<span class="node-type">Trigger</span>
								<span class="node-title"
									>{triggerTypes[funnel.trigger_type] || funnel.trigger_type}</span
								>
							</div>
						</div>

						<!-- Action Nodes -->
						{#each actions as action, index}
							{@const ActionIcon = getActionIcon(action.action_type)}
							<div class="workflow-connector">
								<IconArrowRight size={16} />
							</div>
							<div class="workflow-node action-node {getActionColor(action.action_type)}">
								<div class="node-icon">
									<ActionIcon size={20} />
								</div>
								<div class="node-content">
									<span class="node-type"
										>{actionTypes[action.action_type] || action.action_type}</span
									>
									<span class="node-title">{action.title || `Step ${index + 1}`}</span>
									{#if action.delay_seconds > 0}
										<span class="node-delay">
											<IconClock size={12} />
											{action.delay_seconds >= 86400
												? `${Math.floor(action.delay_seconds / 86400)} day(s)`
												: action.delay_seconds >= 3600
													? `${Math.floor(action.delay_seconds / 3600)} hour(s)`
													: `${Math.floor(action.delay_seconds / 60)} minute(s)`}
										</span>
									{/if}
								</div>
								<div class="node-stats">
									<span>{formatNumber(action.execution_count)} executed</span>
								</div>
							</div>
						{/each}
					</div>
				{/if}
			</div>

			<!-- Subscribers Panel -->
			<div 
				class="tab-panel" 
				class:active={activeTab === 'subscribers'}
				inert={activeTab !== 'subscribers' ? true : undefined}
			>
				{#if isLoadingSubscribers}
					<div class="loading-state mini">
						<IconLoader2 size={24} class="spinning" />
						<p>Loading subscribers...</p>
					</div>
				{:else if subscribers.length === 0}
					<div class="empty-state">
						<IconUsers size={48} />
						<h3>No subscribers yet</h3>
						<p>Contacts will appear here when they enter the automation</p>
					</div>
				{:else}
					<div class="table-container">
						<table class="data-table">
							<thead>
								<tr>
									<th>Contact</th>
									<th>Status</th>
									<th>Progress</th>
									<th>Entered</th>
									<th>Next Action</th>
								</tr>
							</thead>
							<tbody>
								{#each subscribers as subscriber}
									<tr>
										<td>
											<div class="contact-cell">
												<span class="contact-name">
													{subscriber.contact?.full_name || subscriber.contact?.email || 'Unknown'}
												</span>
												<span class="contact-email">
													{subscriber.contact?.email || ''}
												</span>
											</div>
										</td>
										<td>
											<span class="status-badge {getSubscriberStatusColor(subscriber.status)}">
												{subscriber.status}
											</span>
										</td>
										<td>
											<div class="progress-bar">
												<div
													class="progress-fill"
													style="width: {subscriber.progress_percentage || 0}%"
												></div>
											</div>
											<span class="progress-text">{subscriber.progress_percentage || 0}%</span>
										</td>
										<td>{formatDate(subscriber.entered_at)}</td>
										<td>
											{#if subscriber.next_execution_at}
												{formatDate(subscriber.next_execution_at)}
											{:else}
												-
											{/if}
										</td>
									</tr>
								{/each}
							</tbody>
						</table>
					</div>
				{/if}
			</div>
		</div>
	{/if}
</div>

<style>
	.automation-view-page {
		max-width: 1400px;
		margin: 0 auto;
		padding: 24px;
	}

	/* Loading/Error States */
	.loading-state,
	.error-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 2rem;
		text-align: center;
		color: #64748b;
	}

	.loading-state.mini {
		padding: 2rem;
	}

	.spinner {
		width: 40px;
		height: 40px;
		border: 3px solid rgba(99, 102, 241, 0.2);
		border-top-color: var(--primary-500);
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin-bottom: 1rem;
	}

	.error-state :global(svg) {
		color: #f87171;
		margin-bottom: 1rem;
	}

	.error-state h3 {
		color: #e2e8f0;
		margin: 0 0 0.5rem 0;
	}

	/* Header */
	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 2rem;
		flex-wrap: wrap;
		gap: 1rem;
	}

	.back-link {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		color: #94a3b8;
		text-decoration: none;
		font-size: 0.875rem;
		margin-bottom: 0.75rem;
		transition: color 0.2s;
	}

	.back-link:hover {
		color: #e2e8f0;
	}

	.title-row {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.funnel-icon {
		width: 56px;
		height: 56px;
		border-radius: 14px;
		background: linear-gradient(135deg, #ec4899, #be185d);
		display: flex;
		align-items: center;
		justify-content: center;
		color: white;
	}

	.page-header h1 {
		font-size: 1.5rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0;
	}

	.description {
		color: #64748b;
		margin: 0.25rem 0 0 0;
		font-size: 0.875rem;
	}

	.header-actions {
		display: flex;
		gap: 0.75rem;
		align-items: center;
	}

	/* Status Badge */
	.status-badge {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.375rem 0.875rem;
		border-radius: 20px;
		font-size: 0.8rem;
		font-weight: 600;
		text-transform: capitalize;
	}

	/* Buttons */
	.btn-icon {
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

	.btn-icon:hover:not(:disabled) {
		background: rgba(99, 102, 241, 0.2);
		color: #818cf8;
	}

	.btn-icon:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.btn-primary {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background: linear-gradient(135deg, var(--primary-500), var(--primary-600));
		color: white;
		border: none;
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
		text-decoration: none;
	}

	.btn-primary:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow: 0 4px 15px rgba(99, 102, 241, 0.4);
	}

	.btn-secondary {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background: rgba(30, 41, 59, 0.8);
		border: 1px solid rgba(99, 102, 241, 0.2);
		color: #e2e8f0;
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-secondary:hover:not(:disabled) {
		background: rgba(99, 102, 241, 0.2);
	}

	.btn-secondary:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	/* Alerts */
	.error-alert,
	.success-alert {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem;
		border-radius: 10px;
		margin-bottom: 1.5rem;
	}

	.error-alert {
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		color: #f87171;
	}

	.success-alert {
		background: rgba(34, 197, 94, 0.1);
		border: 1px solid rgba(34, 197, 94, 0.3);
		color: #4ade80;
	}

	.error-alert span,
	.success-alert span {
		flex: 1;
	}

	.error-alert button {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 24px;
		height: 24px;
		background: transparent;
		border: none;
		color: inherit;
		cursor: pointer;
	}

	/* Stats Grid */
	.stats-grid {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 1rem;
		margin-bottom: 2rem;
	}

	@media (max-width: 1200px) {
		.stats-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	@media (max-width: 640px) {
		.stats-grid {
			grid-template-columns: 1fr;
		}
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
		width: 52px;
		height: 52px;
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.stat-icon.blue {
		background: rgba(59, 130, 246, 0.15);
		color: #60a5fa;
	}
	.stat-icon.green {
		background: rgba(34, 197, 94, 0.15);
		color: #4ade80;
	}
	.stat-icon.purple {
		background: rgba(139, 92, 246, 0.15);
		color: #a78bfa;
	}
	.stat-icon.amber {
		background: rgba(245, 158, 11, 0.15);
		color: #fbbf24;
	}

	.stat-content {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.stat-value {
		font-size: 1.5rem;
		font-weight: 700;
		color: #f1f5f9;
	}

	.stat-label {
		font-size: 0.8rem;
		color: #64748b;
	}

	/* Tabs */
	.tabs {
		display: flex;
		gap: 0.5rem;
		margin-bottom: 1.5rem;
		border-bottom: 1px solid rgba(99, 102, 241, 0.1);
		padding-bottom: 0.5rem;
	}

	.tab {
		padding: 0.75rem 1.25rem;
		background: transparent;
		border: none;
		color: #94a3b8;
		font-weight: 500;
		cursor: pointer;
		border-radius: 8px;
		transition: all 0.2s;
	}

	.tab:hover {
		color: #e2e8f0;
		background: rgba(99, 102, 241, 0.1);
	}

	.tab.active {
		color: #e2e8f0;
		background: rgba(99, 102, 241, 0.2);
	}

	/* Tab Content - Layout Shift Prevention */
	.tab-content {
		position: relative;
		min-height: 400px;
		contain: layout style;
		isolation: isolate;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 16px;
	}

	/* Tab Panels - CSS visibility toggling eliminates layout shift */
	.tab-panel {
		position: absolute;
		inset: 0;
		width: 100%;
		padding: 1.5rem;
		contain: content;
		opacity: 0;
		visibility: hidden;
		transform: translateY(8px);
		transition: 
			opacity 0.2s ease,
			visibility 0.2s ease,
			transform 0.2s ease;
		z-index: 0;
		pointer-events: none;
	}

	.tab-panel.active {
		position: relative;
		opacity: 1;
		visibility: visible;
		transform: translateY(0);
		z-index: 1;
		pointer-events: auto;
	}

	@media (prefers-reduced-motion: reduce) {
		.tab-panel {
			transition: none;
			transform: none;
		}
		.tab-panel:not(.active) {
			display: none;
		}
	}

	/* Overview Grid */
	.overview-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 1.5rem;
	}

	@media (max-width: 768px) {
		.overview-grid {
			grid-template-columns: 1fr;
		}
	}

	.info-card {
		background: rgba(15, 23, 42, 0.8);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 12px;
		padding: 1.25rem;
	}

	.info-card h3 {
		font-size: 0.875rem;
		font-weight: 600;
		color: #94a3b8;
		margin: 0 0 1rem 0;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.trigger-label {
		display: inline-block;
		padding: 0.5rem 1rem;
		background: rgba(99, 102, 241, 0.2);
		border-radius: 8px;
		color: #818cf8;
		font-weight: 600;
	}

	.trigger-settings {
		margin-top: 1rem;
		padding-top: 1rem;
		border-top: 1px solid rgba(99, 102, 241, 0.1);
	}

	.setting-item {
		display: flex;
		gap: 0.5rem;
		margin-bottom: 0.5rem;
		font-size: 0.875rem;
	}

	.setting-key {
		color: #64748b;
		text-transform: capitalize;
	}

	.setting-value {
		color: #e2e8f0;
	}

	.details-list {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.detail-item {
		display: flex;
		justify-content: space-between;
	}

	.detail-label {
		color: #64748b;
		font-size: 0.875rem;
	}

	.detail-value {
		color: #e2e8f0;
		font-size: 0.875rem;
		font-weight: 500;
	}

	/* Workflow Visual */
	.workflow-visual {
		display: flex;
		align-items: center;
		gap: 0;
		overflow-x: auto;
		padding: 1rem 0;
	}

	.workflow-node {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem 1.25rem;
		background: rgba(15, 23, 42, 0.8);
		border: 2px solid rgba(99, 102, 241, 0.2);
		border-radius: 12px;
		min-width: 200px;
		flex-shrink: 0;
	}

	.trigger-node {
		border-color: rgba(34, 197, 94, 0.3);
		background: rgba(34, 197, 94, 0.05);
	}

	.node-icon {
		width: 40px;
		height: 40px;
		border-radius: 10px;
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
	}

	.node-content {
		display: flex;
		flex-direction: column;
		gap: 0.125rem;
		flex: 1;
	}

	.node-type {
		font-size: 0.7rem;
		color: #64748b;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.node-title {
		font-size: 0.875rem;
		font-weight: 600;
		color: #e2e8f0;
	}

	.node-delay {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		font-size: 0.7rem;
		color: #94a3b8;
		margin-top: 0.25rem;
	}

	.node-stats {
		font-size: 0.7rem;
		color: #64748b;
	}

	.workflow-connector {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 40px;
		color: #64748b;
		flex-shrink: 0;
	}

	/* Empty State */
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 3rem 2rem;
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

	/* Table */
	.table-container {
		overflow-x: auto;
	}

	.data-table {
		width: 100%;
		border-collapse: collapse;
	}

	.data-table th {
		padding: 0.75rem 1rem;
		text-align: left;
		font-size: 0.75rem;
		font-weight: 600;
		color: #64748b;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		border-bottom: 1px solid rgba(99, 102, 241, 0.1);
	}

	.data-table td {
		padding: 0.75rem 1rem;
		font-size: 0.875rem;
		color: #e2e8f0;
		border-bottom: 1px solid rgba(99, 102, 241, 0.05);
	}

	.data-table tbody tr:hover {
		background: rgba(99, 102, 241, 0.05);
	}

	.contact-cell {
		display: flex;
		flex-direction: column;
		gap: 0.125rem;
	}

	.contact-name {
		font-weight: 500;
		color: #f1f5f9;
	}

	.contact-email {
		font-size: 0.75rem;
		color: #64748b;
	}

	/* Progress Bar */
	.progress-bar {
		width: 80px;
		height: 6px;
		background: rgba(99, 102, 241, 0.2);
		border-radius: 3px;
		overflow: hidden;
		display: inline-block;
		vertical-align: middle;
		margin-right: 0.5rem;
	}

	.progress-fill {
		height: 100%;
		background: var(--primary-500);
		border-radius: 3px;
		transition: width 0.3s;
	}

	.progress-text {
		font-size: 0.75rem;
		color: #94a3b8;
	}

	/* Animations */
	:global(.spinning) {
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

	/* Responsive */
	@media (max-width: 640px) {
		.automation-view-page {
			padding: 16px;
		}

		.page-header {
			flex-direction: column;
		}

		.header-actions {
			width: 100%;
			justify-content: flex-start;
		}

		.tab-content {
			padding: 1rem;
		}
	}
</style>

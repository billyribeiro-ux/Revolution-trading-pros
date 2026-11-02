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
	import IconShare from '@tabler/icons-svelte-runes/icons/share';
	import IconArrowLeft from '@tabler/icons-svelte-runes/icons/arrow-left';
	import IconEdit from '@tabler/icons-svelte-runes/icons/edit';
	import IconPlayerPlay from '@tabler/icons-svelte-runes/icons/player-play';
	import IconPlayerPause from '@tabler/icons-svelte-runes/icons/player-pause';
	import IconUsers from '@tabler/icons-svelte-runes/icons/users';
	import IconChartBar from '@tabler/icons-svelte-runes/icons/chart-bar';
	import IconCheck from '@tabler/icons-svelte-runes/icons/check';
	import IconX from '@tabler/icons-svelte-runes/icons/x';
	import IconAlertCircle from '@tabler/icons-svelte-runes/icons/alert-circle';
	import IconRefresh from '@tabler/icons-svelte-runes/icons/refresh';
	import IconDownload from '@tabler/icons-svelte-runes/icons/download';
	import IconMail from '@tabler/icons-svelte-runes/icons/mail';
	import IconClock from '@tabler/icons-svelte-runes/icons/clock';
	import IconTag from '@tabler/icons-svelte-runes/icons/tag';
	import IconList from '@tabler/icons-svelte-runes/icons/list';
	import IconBolt from '@tabler/icons-svelte-runes/icons/bolt';
	import IconArrowRight from '@tabler/icons-svelte-runes/icons/arrow-right';
	import IconLoader2 from '@tabler/icons-svelte-runes/icons/loader-2';
	import { crmAPI } from '#lib/api/crm.js';
	import type {
		AutomationFunnel,
		FunnelAction,
		FunnelSubscriber,
		FunnelStats,
		FunnelStatus
	} from '#lib/crm/types.js';

	// STATE

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

	// Audit P2 #14: this used to be `const funnelId = page.params.id ?? ''`,
	// which captured the route param at component-init time only. SvelteKit
	// reuses `+page.svelte` instances when navigating between
	// `/automations/A` and `/automations/B`, so subsequent
	// `crmAPI.updateAutomationFunnel(funnelId, …)` calls would silently
	// mutate the *original* funnel — a real data-integrity bug. Switching
	// to `$derived` rebinds on every param change.
	let funnelId = $derived(page.params.id ?? '');

	// API FUNCTIONS

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

	async function selectTab(tab: typeof activeTab) {
		activeTab = tab;

		if (tab === 'subscribers' && subscribers.length === 0 && !isLoadingSubscribers) {
			await loadSubscribers();
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

	// HELPERS

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

	function getFunnelStatusClass(status: FunnelStatus): string {
		const classes: Record<FunnelStatus, string> = {
			draft: 'status-badge--draft',
			active: 'status-badge--funnel-active',
			paused: 'status-badge--paused'
		};
		return classes[status];
	}

	function getSubscriberStatusClass(status: string): string {
		const classes: Record<string, string> = {
			active: 'status-badge--subscriber-active',
			waiting: 'status-badge--waiting',
			completed: 'status-badge--completed',
			cancelled: 'status-badge--cancelled',
			failed: 'status-badge--failed'
		};
		return classes[status] || 'status-badge--neutral';
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

	function getActionClass(actionType: string): string {
		const classMap: Record<string, string> = {
			send_email: 'workflow-node--send-email',
			wait: 'workflow-node--wait',
			add_tag: 'workflow-node--add-tag',
			remove_tag: 'workflow-node--remove-tag',
			add_to_list: 'workflow-node--add-to-list',
			remove_from_list: 'workflow-node--remove-from-list',
			condition: 'workflow-node--condition'
		};
		return classMap[actionType] || 'workflow-node--default';
	}

	function getProgressWidth(value: number | null | undefined): string {
		const percentage = Number(value ?? 0);
		if (!Number.isFinite(percentage)) return '0%';
		return `${Math.min(100, Math.max(0, percentage))}%`;
	}

	// DERIVED STATE

	let completionRate = $derived(
		stats && stats.total_subscribers > 0
			? ((stats.completed / stats.total_subscribers) * 100).toFixed(1)
			: '0'
	);

	// LIFECYCLE

	onMount(() => {
		loadFunnel();
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
					<span class={['status-badge', getFunnelStatusClass(funnel.status)]}>
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
				class={['tab', { active: activeTab === 'overview' }]}
				onclick={() => selectTab('overview')}
			>
				Overview
			</button>
			<button
				class={['tab', { active: activeTab === 'workflow' }]}
				onclick={() => selectTab('workflow')}
			>
				Workflow ({actions.length})
			</button>
			<button
				class={['tab', { active: activeTab === 'subscribers' }]}
				onclick={() => selectTab('subscribers')}
			>
				Subscribers ({stats?.total_subscribers || 0})
			</button>
		</div>

		<!-- Tab Content - Layout Shift Free Pattern -->
		<div class="tab-content">
			<!-- Overview Panel -->
			<div
				class={['tab-panel', { active: activeTab === 'overview' }]}
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
									{#each Object.entries(funnel.trigger_settings) as [key, value] (key)}
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
				class={['tab-panel', { active: activeTab === 'workflow' }]}
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
							<div class="node-icon node-icon--trigger">
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
						{#each actions as action, index (action.id)}
							{@const ActionIcon = getActionIcon(action.action_type)}
							<div class="workflow-connector">
								<IconArrowRight size={16} />
							</div>
							<div class={['workflow-node', 'action-node', getActionClass(action.action_type)]}>
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
				class={['tab-panel', { active: activeTab === 'subscribers' }]}
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
								{#each subscribers as subscriber (subscriber.id)}
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
											<span class={['status-badge', getSubscriberStatusClass(subscriber.status)]}>
												{subscriber.status}
											</span>
										</td>
										<td>
											<div class="progress-bar">
												<div
													class="progress-fill"
													style:width={getProgressWidth(subscriber.progress_percentage)}
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
		color: var(--text-tertiary, #6e7681);
	}

	.loading-state.mini {
		padding: 2rem;
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

	.error-state :global(svg) {
		color: var(--error-emphasis, #f85149);
		margin-bottom: 1rem;
	}

	.error-state h3 {
		color: var(--text-primary, #f0f6fc);
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
		color: var(--text-secondary, #8b949e);
		text-decoration: none;
		font-size: 0.875rem;
		margin-bottom: 0.75rem;
		transition: color 0.2s;
	}

	.back-link:hover {
		color: var(--text-primary, #f0f6fc);
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
		background: #ec4899;
		display: flex;
		align-items: center;
		justify-content: center;
		color: white;
	}

	.page-header h1 {
		font-size: 1.5rem;
		font-weight: 700;
		color: var(--text-primary, #f0f6fc);
		margin: 0;
	}

	.description {
		color: var(--text-tertiary, #6e7681);
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

	.status-badge--draft,
	.status-badge--neutral {
		background: var(--bg-hover, #252b33);
		color: var(--text-secondary, #8b949e);
	}

	.status-badge--funnel-active,
	.status-badge--completed {
		background: rgba(63, 185, 80, 0.2);
		color: var(--success-emphasis, #3fb950);
	}

	.status-badge--paused,
	.status-badge--waiting {
		background: rgba(210, 153, 34, 0.2);
		color: var(--warning-emphasis, #d29922);
	}

	.status-badge--subscriber-active {
		background: rgba(88, 166, 255, 0.2);
		color: var(--info-emphasis, #58a6ff);
	}

	.status-badge--cancelled,
	.status-badge--failed {
		background: rgba(248, 81, 73, 0.2);
		color: var(--error-emphasis, #f85149);
	}

	/* Buttons */
	.btn-icon {
		width: 42px;
		height: 42px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: var(--bg-elevated, #161b22);
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 10px;
		color: var(--text-secondary, #8b949e);
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-icon:hover:not(:disabled) {
		background: rgba(230, 184, 0, 0.2);
		color: var(--primary-400, #ffd11a);
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
		background: var(--primary-500);
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
		background: var(--bg-elevated, #161b22);
		border: 1px solid rgba(230, 184, 0, 0.2);
		color: var(--text-primary, #f0f6fc);
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-secondary:hover:not(:disabled) {
		background: rgba(230, 184, 0, 0.2);
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
		background: rgba(248, 81, 73, 0.1);
		border: 1px solid rgba(248, 81, 73, 0.3);
		color: var(--error-emphasis, #f85149);
	}

	.success-alert {
		background: rgba(63, 185, 80, 0.1);
		border: 1px solid rgba(63, 185, 80, 0.3);
		color: var(--success-emphasis, #3fb950);
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

	@media (max-width: 1199.98px) {
		.stats-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	@media (max-width: 639.98px) {
		.stats-grid {
			grid-template-columns: 1fr;
		}
	}

	.stat-card {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1.25rem;
		background: var(--bg-elevated, #161b22);
		border: 1px solid rgba(230, 184, 0, 0.1);
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
		background: rgba(88, 166, 255, 0.15);
		color: var(--info-emphasis, #58a6ff);
	}
	.stat-icon.green {
		background: rgba(63, 185, 80, 0.15);
		color: var(--success-emphasis, #3fb950);
	}
	.stat-icon.purple {
		background: rgba(230, 184, 0, 0.15);
		color: var(--primary-400, #ffd11a);
	}
	.stat-icon.amber {
		background: rgba(210, 153, 34, 0.15);
		color: var(--warning-emphasis, #d29922);
	}

	.stat-content {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.stat-value {
		font-size: 1.5rem;
		font-weight: 700;
		color: var(--text-primary, #f0f6fc);
	}

	.stat-label {
		font-size: 0.8rem;
		color: var(--text-tertiary, #6e7681);
	}

	/* Tabs */
	.tabs {
		display: flex;
		gap: 0.5rem;
		margin-bottom: 1.5rem;
		border-bottom: 1px solid rgba(230, 184, 0, 0.1);
		padding-bottom: 0.5rem;
	}

	.tab {
		padding: 0.75rem 1.25rem;
		background: transparent;
		border: none;
		color: var(--text-secondary, #8b949e);
		font-weight: 500;
		cursor: pointer;
		border-radius: 8px;
		transition: all 0.2s;
	}

	.tab:hover {
		color: var(--text-primary, #f0f6fc);
		background: rgba(230, 184, 0, 0.1);
	}

	.tab.active {
		color: var(--text-primary, #f0f6fc);
		background: rgba(230, 184, 0, 0.2);
	}

	/* Tab Content - Layout Shift Prevention */
	.tab-content {
		position: relative;
		min-height: 400px;
		contain: layout style;
		isolation: isolate;
		background: var(--bg-base, #0d1117);
		border: 1px solid rgba(230, 184, 0, 0.1);
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

	@media (max-width: 767.98px) {
		.overview-grid {
			grid-template-columns: 1fr;
		}
	}

	.info-card {
		background: var(--bg-base, #0d1117);
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 12px;
		padding: 1.25rem;
	}

	.info-card h3 {
		font-size: 0.875rem;
		font-weight: 600;
		color: var(--text-secondary, #8b949e);
		margin: 0 0 1rem 0;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.trigger-label {
		display: inline-block;
		padding: 0.5rem 1rem;
		background: rgba(230, 184, 0, 0.2);
		border-radius: 8px;
		color: var(--primary-400, #ffd11a);
		font-weight: 600;
	}

	.trigger-settings {
		margin-top: 1rem;
		padding-top: 1rem;
		border-top: 1px solid rgba(230, 184, 0, 0.1);
	}

	.setting-item {
		display: flex;
		gap: 0.5rem;
		margin-bottom: 0.5rem;
		font-size: 0.875rem;
	}

	.setting-key {
		color: var(--text-tertiary, #6e7681);
		text-transform: capitalize;
	}

	.setting-value {
		color: var(--text-primary, #f0f6fc);
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
		color: var(--text-tertiary, #6e7681);
		font-size: 0.875rem;
	}

	.detail-value {
		color: var(--text-primary, #f0f6fc);
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
		background: var(--bg-base, #0d1117);
		border: 2px solid rgba(230, 184, 0, 0.2);
		border-radius: 12px;
		min-width: 200px;
		flex-shrink: 0;
	}

	.workflow-node--send-email {
		background: rgba(88, 166, 255, 0.2);
		border-color: rgba(88, 166, 255, 0.3);
		color: var(--info-emphasis, #58a6ff);
	}

	.workflow-node--wait {
		background: rgba(210, 153, 34, 0.2);
		border-color: rgba(210, 153, 34, 0.3);
		color: var(--warning-emphasis, #d29922);
	}

	.workflow-node--add-tag {
		background: rgba(63, 185, 80, 0.2);
		border-color: rgba(63, 185, 80, 0.3);
		color: var(--success-emphasis, #3fb950);
	}

	.workflow-node--remove-tag {
		background: rgba(248, 81, 73, 0.2);
		border-color: rgba(248, 81, 73, 0.3);
		color: var(--error-emphasis, #f85149);
	}

	.workflow-node--add-to-list {
		background: rgba(230, 184, 0, 0.2);
		border-color: rgba(230, 184, 0, 0.3);
		color: #c084fc;
	}

	.workflow-node--remove-from-list {
		background: rgba(210, 153, 34, 0.2);
		border-color: rgba(210, 153, 34, 0.3);
		color: var(--warning-emphasis, #d29922);
	}

	.workflow-node--condition {
		background: rgba(88, 166, 255, 0.2);
		border-color: rgba(88, 166, 255, 0.3);
		color: var(--info-emphasis, #58a6ff);
	}

	.workflow-node--default {
		background: var(--bg-hover, #252b33);
		border-color: var(--border-default, #30363d);
		color: var(--text-secondary, #8b949e);
	}

	.trigger-node {
		border-color: rgba(63, 185, 80, 0.3);
		background: rgba(63, 185, 80, 0.05);
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

	.node-icon--trigger {
		background: rgba(63, 185, 80, 0.2);
		color: var(--success-emphasis, #3fb950);
	}

	.node-content {
		display: flex;
		flex-direction: column;
		gap: 0.125rem;
		flex: 1;
	}

	.node-type {
		font-size: 0.7rem;
		color: var(--text-tertiary, #6e7681);
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.node-title {
		font-size: 0.875rem;
		font-weight: 600;
		color: var(--text-primary, #f0f6fc);
	}

	.node-delay {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		font-size: 0.7rem;
		color: var(--text-secondary, #8b949e);
		margin-top: 0.25rem;
	}

	.node-stats {
		font-size: 0.7rem;
		color: var(--text-tertiary, #6e7681);
	}

	.workflow-connector {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 40px;
		color: var(--text-tertiary, #6e7681);
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
		color: var(--text-tertiary, #6e7681);
	}

	.empty-state :global(svg) {
		color: var(--text-tertiary, #6e7681);
		margin-bottom: 1rem;
	}

	.empty-state h3 {
		color: var(--text-primary, #f0f6fc);
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
		color: var(--text-tertiary, #6e7681);
		text-transform: uppercase;
		letter-spacing: 0.05em;
		border-bottom: 1px solid rgba(230, 184, 0, 0.1);
	}

	.data-table td {
		padding: 0.75rem 1rem;
		font-size: 0.875rem;
		color: var(--text-primary, #f0f6fc);
		border-bottom: 1px solid rgba(230, 184, 0, 0.05);
	}

	.data-table tbody tr:hover {
		background: rgba(230, 184, 0, 0.05);
	}

	.contact-cell {
		display: flex;
		flex-direction: column;
		gap: 0.125rem;
	}

	.contact-name {
		font-weight: 500;
		color: var(--text-primary, #f0f6fc);
	}

	.contact-email {
		font-size: 0.75rem;
		color: var(--text-tertiary, #6e7681);
	}

	/* Progress Bar */
	.progress-bar {
		width: 80px;
		height: 6px;
		background: rgba(230, 184, 0, 0.2);
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
		color: var(--text-secondary, #8b949e);
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
	@media (max-width: 639.98px) {
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

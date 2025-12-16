<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { toastStore } from '$lib/stores/toast';
	import { adminFetch } from '$lib/utils/adminFetch';
	import { connections, isEmailConnected } from '$lib/stores/connections';
	import ApiNotConnected from '$lib/components/ApiNotConnected.svelte';
	import SkeletonLoader from '$lib/components/SkeletonLoader.svelte';
	import {
		IconMail,
		IconPlus,
		IconCalendar,
		IconUsers,
		IconChartBar,
		IconSearch,
		IconFilter,
		IconRefresh,
		IconSend,
		IconClock,
		IconCheck,
		IconX,
		IconEdit,
		IconTrash,
		IconEye,
		IconCopy,
		IconTrendingUp,
		IconArrowLeft
	} from '$lib/icons';
	import {
		getCampaigns,
		getCampaignStats,
		createCampaign,
		deleteCampaign as apiDeleteCampaign,
		duplicateCampaign as apiDuplicateCampaign,
		sendCampaign,
		cancelCampaign,
		type Campaign as APICampaign,
		type CampaignStats
	} from '$lib/api/campaigns';

	// Connection-aware state
	let connectionLoading = true;

	// State
	let loading = true;
	let error = '';
	let activeTab: 'all' | 'scheduled' | 'sent' | 'drafts' = 'all';

	// Campaign data
	let campaigns: APICampaign[] = [];
	let stats: CampaignStats | null = null;

	// Create modal
	let showCreateModal = false;
	let creating = false;
	let newCampaign = {
		name: '',
		subject: '',
		subjectB: '',
		useABTest: false,
		abTestSplit: 50,
		scheduledFor: '',
		segmentId: '',
		templateId: ''
	};

	// Segments for targeting (will be loaded from API in future)
	let segments = [
		{ id: 1, name: 'All Members', count: 12847 },
		{ id: 2, name: 'Active Subscribers', count: 8420 },
		{ id: 3, name: 'Trial Users', count: 2690 },
		{ id: 4, name: 'Churned (Win-back)', count: 1737 },
		{ id: 5, name: 'High Value (>$500 LTV)', count: 847 },
		{ id: 6, name: 'Highly Engaged', count: 3250 },
		{ id: 7, name: 'At Risk of Churn', count: 620 }
	];

	// Templates (will be loaded from API)
	let templates: { id: number; name: string }[] = [];

	onMount(async () => {
		// Load connection status first
		await connections.load();
		connectionLoading = false;

		// Only load data if email is connected
		if ($isEmailConnected) {
			await Promise.all([loadCampaigns(), loadStats(), loadTemplates()]);
		} else {
			loading = false;
		}
	});

	async function loadCampaigns() {
		loading = true;
		error = '';
		try {
			const statusFilter = activeTab === 'all' ? undefined : 
				activeTab === 'drafts' ? 'draft' : activeTab;
			const response = await getCampaigns({ status: statusFilter });
			campaigns = response.data;
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load campaigns';
			console.error('Failed to load campaigns:', err);
		} finally {
			loading = false;
		}
	}

	async function loadStats() {
		try {
			stats = await getCampaignStats();
		} catch (err) {
			console.error('Failed to load stats:', err);
		}
	}

	async function loadTemplates() {
		try {
			const data = await adminFetch('/api/admin/email/templates');
			templates = (data.data || data || []).map((t: any) => ({ id: t.id, name: t.name }));
		} catch (err) {
			console.error('Failed to load templates:', err);
		}
	}

	function getFilteredCampaigns() {
		if (activeTab === 'all') return campaigns;
		if (activeTab === 'scheduled') return campaigns.filter((c) => c.status === 'scheduled');
		if (activeTab === 'sent') return campaigns.filter((c) => c.status === 'sent');
		if (activeTab === 'drafts') return campaigns.filter((c) => c.status === 'draft');
		return campaigns;
	}

	function formatDate(dateStr: string | null): string {
		if (!dateStr) return '-';
		return new Date(dateStr).toLocaleDateString('en-US', {
			month: 'short',
			day: 'numeric',
			year: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	function formatNumber(num: number): string {
		return num.toLocaleString();
	}

	function getStatusColor(status: string): string {
		switch (status) {
			case 'sent':
				return 'bg-emerald-500/20 text-emerald-400';
			case 'scheduled':
				return 'bg-blue-500/20 text-blue-400';
			case 'sending':
				return 'bg-yellow-500/20 text-yellow-400';
			default:
				return 'bg-slate-500/20 text-slate-400';
		}
	}

	async function handleCreateCampaign() {
		// Validation
		if (!newCampaign.name || !newCampaign.subject) {
			toastStore.error('Please fill in campaign name and subject');
			return;
		}
		if (!newCampaign.templateId) {
			toastStore.error('Please select an email template');
			return;
		}
		if (newCampaign.useABTest && !newCampaign.subjectB) {
			toastStore.error('Please enter Subject B for A/B test');
			return;
		}

		creating = true;
		try {
			const campaignData = {
				name: newCampaign.name,
				subject: newCampaign.subject,
				template_id: parseInt(newCampaign.templateId),
				segment_id: newCampaign.segmentId ? parseInt(newCampaign.segmentId) : null,
				scheduled_at: newCampaign.scheduledFor || null,
				ab_test_config: newCampaign.useABTest ? {
					enabled: true,
					subject_b: newCampaign.subjectB,
					split_percentage: newCampaign.abTestSplit
				} : null
			};

			await createCampaign(campaignData);
			showCreateModal = false;
			toastStore.success('Campaign created successfully');
			
			// Reset form
			newCampaign = {
				name: '',
				subject: '',
				subjectB: '',
				useABTest: false,
				abTestSplit: 50,
				scheduledFor: '',
				segmentId: '',
				templateId: ''
			};

			// Reload campaigns
			await loadCampaigns();
		} catch (err) {
			toastStore.error(err instanceof Error ? err.message : 'Failed to create campaign');
		} finally {
			creating = false;
		}
	}

	async function handleDeleteCampaign(id: number) {
		if (!confirm('Delete this campaign?')) return;
		try {
			await apiDeleteCampaign(id);
			toastStore.success('Campaign deleted');
			await loadCampaigns();
		} catch (err) {
			toastStore.error(err instanceof Error ? err.message : 'Failed to delete campaign');
		}
	}

	async function handleDuplicateCampaign(campaign: APICampaign) {
		try {
			await apiDuplicateCampaign(campaign.id);
			toastStore.success('Campaign duplicated');
			await loadCampaigns();
		} catch (err) {
			toastStore.error(err instanceof Error ? err.message : 'Failed to duplicate campaign');
		}
	}

	async function handleSendCampaign(id: number) {
		if (!confirm('Send this campaign now?')) return;
		try {
			await sendCampaign(id);
			toastStore.success('Campaign is being sent');
			await loadCampaigns();
		} catch (err) {
			toastStore.error(err instanceof Error ? err.message : 'Failed to send campaign');
		}
	}

	async function handleCancelCampaign(id: number) {
		if (!confirm('Cancel this scheduled campaign?')) return;
		try {
			await cancelCampaign(id);
			toastStore.success('Campaign cancelled');
			await loadCampaigns();
		} catch (err) {
			toastStore.error(err instanceof Error ? err.message : 'Failed to cancel campaign');
		}
	}

	// Helper to get segment name
	function getSegmentName(segmentId: number | null): string {
		if (!segmentId) return 'All Members';
		const segment = segments.find(s => s.id === segmentId);
		return segment?.name || 'Unknown';
	}
</script>

<svelte:head>
	<title>Email Campaigns | Revolution Trading Pros</title>
</svelte:head>

<div class="campaigns-page">
	<!-- Header -->
	<div class="page-header">
		<button class="back-btn" onclick={() => goto('/admin/email/templates')}>
			<IconArrowLeft size={20} />
			Back to Templates
		</button>

		<div class="header-content">
			<div class="header-title">
				<div class="title-icon">
					<IconMail size={28} />
				</div>
				<div>
					<h1>Email Campaigns</h1>
					<p class="subtitle">Create, schedule, and analyze email campaigns</p>
				</div>
			</div>

			{#if $isEmailConnected}
				<div class="header-actions">
					<button class="btn-secondary" onclick={loadCampaigns}>
						<IconRefresh size={18} />
						Refresh
					</button>
					<button class="btn-primary" onclick={() => (showCreateModal = true)}>
						<IconPlus size={18} />
						New Campaign
					</button>
				</div>
			{/if}
		</div>
	</div>

	<!-- Connection Check -->
	{#if connectionLoading}
		<SkeletonLoader variant="dashboard" />
	{:else if !$isEmailConnected}
		<ApiNotConnected
			serviceName="Email Marketing"
			description="Connect an email marketing service to create and manage email campaigns, track opens, clicks, and subscriber engagement."
			serviceKey="mailchimp"
			icon="📧"
			color="#6366f1"
			features={[
				'Create and schedule email campaigns',
				'A/B test subject lines',
				'Track opens, clicks, and conversions',
				'Segment your audience',
				'Automated email sequences'
			]}
		/>
	{:else if loading}
		<div class="loading-grid">
			{#each [1, 2, 3, 4] as _}
				<div class="skeleton skeleton-metric"></div>
			{/each}
		</div>
	{:else if error}
		<div class="error-state">
			<p>{error}</p>
			<button class="btn-primary" onclick={loadCampaigns}>Try Again</button>
		</div>
	{:else}
		<!-- Stats Overview -->
		<div class="stats-grid">
			<div class="stat-card">
				<div class="stat-icon purple">
					<IconSend size={24} />
				</div>
				<div class="stat-content">
					<div class="stat-value">{formatNumber(stats?.total_sent || 0)}</div>
					<div class="stat-label">Total Emails Sent</div>
				</div>
			</div>
			<div class="stat-card">
				<div class="stat-icon emerald">
					<IconEye size={24} />
				</div>
				<div class="stat-content">
					<div class="stat-value">{formatNumber(stats?.total_opened || 0)}</div>
					<div class="stat-label">Total Opens</div>
				</div>
			</div>
			<div class="stat-card">
				<div class="stat-icon blue">
					<IconTrendingUp size={24} />
				</div>
				<div class="stat-content">
					<div class="stat-value">{stats?.avg_open_rate || 0}%</div>
					<div class="stat-label">Avg Open Rate</div>
				</div>
			</div>
			<div class="stat-card">
				<div class="stat-icon yellow">
					<IconChartBar size={24} />
				</div>
				<div class="stat-content">
					<div class="stat-value">{stats?.avg_click_rate || 0}%</div>
					<div class="stat-label">Avg Click Rate</div>
				</div>
			</div>
		</div>

		<!-- Tabs -->
		<div class="tabs-container">
			<div class="tabs">
				<button class:active={activeTab === 'all'} onclick={() => (activeTab = 'all')}>
					All ({campaigns.length})
				</button>
				<button class:active={activeTab === 'scheduled'} onclick={() => (activeTab = 'scheduled')}>
					<IconClock size={16} />
					Scheduled ({campaigns.filter((c) => c.status === 'scheduled').length})
				</button>
				<button class:active={activeTab === 'sent'} onclick={() => (activeTab = 'sent')}>
					<IconCheck size={16} />
					Sent ({campaigns.filter((c) => c.status === 'sent').length})
				</button>
				<button class:active={activeTab === 'drafts'} onclick={() => (activeTab = 'drafts')}>
					<IconEdit size={16} />
					Drafts ({campaigns.filter((c) => c.status === 'draft').length})
				</button>
			</div>
		</div>

		<!-- Campaigns List -->
		<div class="campaigns-list">
			{#each getFilteredCampaigns() as campaign}
				<div class="campaign-card">
					<div class="campaign-header">
						<div class="campaign-info">
							<div class="campaign-name-row">
								<h3>{campaign.name}</h3>
								{#if campaign.ab_test_config?.enabled}
									<span class="badge ab-badge">A/B Test</span>
								{/if}
							</div>
							<p class="campaign-subject">{campaign.subject || '(No subject)'}</p>
						</div>
						<span class="status-badge {getStatusColor(campaign.status)}">
							{campaign.status}
						</span>
					</div>

					<div class="campaign-meta">
						<div class="meta-item">
							<IconUsers size={16} />
							<span>{getSegmentName(campaign.segment_id)}</span>
							<span class="meta-count">({formatNumber(campaign.total_recipients)})</span>
						</div>
						{#if campaign.scheduled_at}
							<div class="meta-item">
								<IconCalendar size={16} />
								<span>Scheduled: {formatDate(campaign.scheduled_at)}</span>
							</div>
						{/if}
						{#if campaign.sent_at}
							<div class="meta-item">
								<IconClock size={16} />
								<span>Sent: {formatDate(campaign.sent_at)}</span>
							</div>
						{/if}
					</div>

					{#if campaign.status === 'sent' && campaign.sent_count > 0}
						<div class="campaign-stats">
							<div class="stat-item">
								<span class="stat-label">Sent</span>
								<span class="stat-value">{formatNumber(campaign.sent_count)}</span>
							</div>
							<div class="stat-item">
								<span class="stat-label">Opened</span>
								<span class="stat-value highlight">
									{formatNumber(campaign.opened_count)}
									<small>({campaign.open_rate?.toFixed(1) || 0}%)</small>
								</span>
							</div>
							<div class="stat-item">
								<span class="stat-label">Clicked</span>
								<span class="stat-value highlight-blue">
									{formatNumber(campaign.clicked_count)}
									<small>({campaign.click_rate?.toFixed(1) || 0}%)</small>
								</span>
							</div>
							<div class="stat-item">
								<span class="stat-label">Bounced</span>
								<span class="stat-value">{campaign.bounced_count}</span>
							</div>
						</div>

						{#if campaign.ab_test_config?.enabled && campaign.ab_test_config.variant_a}
							<div class="ab-test-results">
								<span class="ab-label">A/B Test Results:</span>
								<div class="ab-variants">
									<div class="ab-variant" class:winner={campaign.ab_test_config.winner === 'a'}>
										<span class="variant-label">A:</span>
										<span class="variant-subject">{campaign.subject}</span>
										<span class="variant-rate">{campaign.ab_test_config.variant_a?.open_rate || 0}%</span>
									</div>
									<div class="ab-variant" class:winner={campaign.ab_test_config.winner === 'b'}>
										<span class="variant-label">B:</span>
										<span class="variant-subject">{campaign.ab_test_config.subject_b}</span>
										<span class="variant-rate">{campaign.ab_test_config.variant_b?.open_rate || 0}%</span>
									</div>
								</div>
							</div>
						{/if}
					{/if}

					<div class="campaign-actions">
						{#if campaign.status === 'draft'}
							<button class="btn-primary small" onclick={() => handleSendCampaign(campaign.id)}>
								<IconSend size={16} />
								Send Now
							</button>
							<button class="btn-secondary small" onclick={() => handleDuplicateCampaign(campaign)}>
								<IconCopy size={16} />
								Duplicate
							</button>
							<button class="btn-danger small" onclick={() => handleDeleteCampaign(campaign.id)}>
								<IconTrash size={16} />
							</button>
						{:else if campaign.status === 'scheduled'}
							<button class="btn-secondary small" onclick={() => handleCancelCampaign(campaign.id)}>
								<IconX size={16} />
								Cancel
							</button>
							<button class="btn-secondary small" onclick={() => handleDuplicateCampaign(campaign)}>
								<IconCopy size={16} />
								Duplicate
							</button>
						{:else}
							<button class="btn-secondary small">
								<IconChartBar size={16} />
								View Report
							</button>
							<button class="btn-secondary small" onclick={() => handleDuplicateCampaign(campaign)}>
								<IconCopy size={16} />
								Duplicate
							</button>
						{/if}
					</div>
				</div>
			{/each}

			{#if getFilteredCampaigns().length === 0}
				<div class="empty-state">
					<IconMail size={48} stroke={1} />
					<h3>No campaigns found</h3>
					<p>Create your first email campaign to get started</p>
					<button class="btn-primary" onclick={() => (showCreateModal = true)}>
						<IconPlus size={18} />
						Create Campaign
					</button>
				</div>
			{/if}
		</div>
	{/if}
</div>

<!-- Create Campaign Modal -->
{#if showCreateModal}
	<div
		class="modal-overlay"
		role="dialog"
		aria-modal="true"
		tabindex="-1"
		onclick={() => (showCreateModal = false)}
		onkeydown={(e) => e.key === 'Escape' && (showCreateModal = false)}
	>
		<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
		<div
			class="modal-content large"
			role="document"
			onclick={(e) => e.stopPropagation()}
			onkeydown={(e) => e.stopPropagation()}
		>
			<div class="modal-header">
				<h2>Create New Campaign</h2>
				<button class="close-btn" onclick={() => (showCreateModal = false)}>
					<IconX size={20} />
				</button>
			</div>

			<div class="modal-body">
				<div class="form-grid">
					<div class="form-group full-width">
						<label for="campaign-name">Campaign Name</label>
						<input
							id="campaign-name"
							type="text"
							bind:value={newCampaign.name}
							placeholder="e.g., December Newsletter"
						/>
					</div>

					<div class="form-group full-width">
						<label for="campaign-subject">Subject Line</label>
						<input
							id="campaign-subject"
							type="text"
							bind:value={newCampaign.subject}
							placeholder="e.g., Your Weekly Trading Insights"
						/>
					</div>

					<div class="form-group full-width ab-toggle">
						<label class="checkbox-label">
							<input type="checkbox" bind:checked={newCampaign.useABTest} />
							<span>Enable A/B Testing</span>
						</label>
					</div>

					{#if newCampaign.useABTest}
						<div class="form-group full-width">
							<label for="campaign-subject-b">Subject Line B</label>
							<input
								id="campaign-subject-b"
								type="text"
								bind:value={newCampaign.subjectB}
								placeholder="Alternative subject to test"
							/>
						</div>
						<div class="form-group">
							<label for="ab-split">A/B Split (%)</label>
							<input
								id="ab-split"
								type="range"
								min="10"
								max="90"
								bind:value={newCampaign.abTestSplit}
							/>
							<span class="split-label">{newCampaign.abTestSplit}% / {100 - newCampaign.abTestSplit}%</span>
						</div>
					{/if}

					<div class="form-group">
						<label for="campaign-segment">Target Segment</label>
						<select id="campaign-segment" bind:value={newCampaign.segmentId}>
							<option value="">Select segment...</option>
							{#each segments as segment}
								<option value={segment.id}>{segment.name} ({formatNumber(segment.count)})</option>
							{/each}
						</select>
					</div>

					<div class="form-group">
						<label for="campaign-template">Email Template</label>
						<select id="campaign-template" bind:value={newCampaign.templateId}>
							<option value="">Select template...</option>
							{#each templates as template}
								<option value={template.id}>{template.name}</option>
							{/each}
						</select>
					</div>

					<div class="form-group full-width">
						<label for="campaign-schedule">Schedule (optional)</label>
						<input
							id="campaign-schedule"
							type="datetime-local"
							bind:value={newCampaign.scheduledFor}
						/>
						<small>Leave empty to save as draft</small>
					</div>
				</div>
			</div>

			<div class="modal-footer">
				<button class="btn-secondary" onclick={() => (showCreateModal = false)}>Cancel</button>
				<button class="btn-primary" onclick={handleCreateCampaign}>
					{#if newCampaign.scheduledFor}
						<IconCalendar size={18} />
						Schedule Campaign
					{:else}
						<IconEdit size={18} />
						Save as Draft
					{/if}
				</button>
			</div>
		</div>
	</div>
{/if}

<style>
	.campaigns-page {
		padding: 2rem;
		max-width: 1400px;
		margin: 0 auto;
	}

	/* Header */
	.back-btn {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 0;
		color: #94a3b8;
		background: none;
		border: none;
		cursor: pointer;
		font-size: 0.875rem;
		margin-bottom: 1rem;
		transition: color 0.2s;
	}

	.back-btn:hover {
		color: #a5b4fc;
	}

	.header-content {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 2rem;
	}

	.header-title {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.title-icon {
		width: 56px;
		height: 56px;
		background: linear-gradient(135deg, #6366f1, #8b5cf6);
		border-radius: 16px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: white;
	}

	.header-title h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0;
	}

	.subtitle {
		color: #64748b;
		margin: 0.25rem 0 0;
	}

	.header-actions {
		display: flex;
		gap: 0.75rem;
	}

	/* Stats */
	.stats-grid {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 1rem;
		margin-bottom: 2rem;
	}

	.stat-card {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1.25rem;
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 16px;
	}

	.stat-icon {
		width: 48px;
		height: 48px;
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.stat-icon.purple { background: rgba(99, 102, 241, 0.15); color: #818cf8; }
	.stat-icon.emerald { background: rgba(16, 185, 129, 0.15); color: #34d399; }
	.stat-icon.blue { background: rgba(59, 130, 246, 0.15); color: #60a5fa; }
	.stat-icon.yellow { background: rgba(251, 191, 36, 0.15); color: #fbbf24; }

	.stat-content .stat-value {
		font-size: 1.5rem;
		font-weight: 700;
		color: #f1f5f9;
	}

	.stat-content .stat-label {
		font-size: 0.8125rem;
		color: #64748b;
	}

	/* Tabs */
	.tabs-container {
		margin-bottom: 1.5rem;
		border-bottom: 1px solid rgba(148, 163, 184, 0.1);
	}

	.tabs {
		display: flex;
		gap: 0.5rem;
	}

	.tabs button {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background: none;
		border: none;
		color: #64748b;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		border-bottom: 2px solid transparent;
		margin-bottom: -1px;
		transition: all 0.2s;
	}

	.tabs button:hover {
		color: #a5b4fc;
	}

	.tabs button.active {
		color: #a5b4fc;
		border-bottom-color: #6366f1;
	}

	/* Campaign List */
	.campaigns-list {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.campaign-card {
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 16px;
		padding: 1.5rem;
	}

	.campaign-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 1rem;
	}

	.campaign-name-row {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.campaign-name-row h3 {
		font-size: 1.125rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0;
	}

	.campaign-subject {
		color: #94a3b8;
		font-size: 0.875rem;
		margin: 0.25rem 0 0;
	}

	.badge {
		padding: 0.25rem 0.5rem;
		border-radius: 6px;
		font-size: 0.6875rem;
		font-weight: 600;
		text-transform: uppercase;
	}

	.ab-badge {
		background: rgba(251, 191, 36, 0.2);
		color: #fbbf24;
	}

	.status-badge {
		padding: 0.375rem 0.75rem;
		border-radius: 9999px;
		font-size: 0.75rem;
		font-weight: 600;
		text-transform: capitalize;
	}

	.campaign-meta {
		display: flex;
		flex-wrap: wrap;
		gap: 1.5rem;
		margin-bottom: 1rem;
	}

	.meta-item {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.8125rem;
		color: #94a3b8;
	}

	.meta-count {
		color: #64748b;
	}

	.campaign-stats {
		display: flex;
		gap: 2rem;
		padding: 1rem 0;
		border-top: 1px solid rgba(148, 163, 184, 0.1);
	}

	.campaign-stats .stat-item {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.campaign-stats .stat-label {
		font-size: 0.6875rem;
		color: #64748b;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.campaign-stats .stat-value {
		font-size: 1rem;
		font-weight: 600;
		color: #f1f5f9;
	}

	.campaign-stats .stat-value.highlight {
		color: #34d399;
	}

	.campaign-stats .stat-value.highlight-blue {
		color: #60a5fa;
	}

	.campaign-stats .stat-value small {
		font-size: 0.75rem;
		font-weight: 400;
		color: #64748b;
	}

	/* A/B Test Results */
	.ab-test-results {
		padding: 1rem;
		background: rgba(15, 23, 42, 0.4);
		border-radius: 10px;
		margin-top: 1rem;
	}

	.ab-label {
		font-size: 0.75rem;
		font-weight: 600;
		color: #94a3b8;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		display: block;
		margin-bottom: 0.75rem;
	}

	.ab-variants {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.ab-variant {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.5rem 0.75rem;
		background: rgba(148, 163, 184, 0.05);
		border-radius: 6px;
	}

	.ab-variant.winner {
		background: rgba(16, 185, 129, 0.1);
		border: 1px solid rgba(16, 185, 129, 0.3);
	}

	.variant-label {
		font-weight: 600;
		color: #a5b4fc;
	}

	.variant-subject {
		flex: 1;
		color: #cbd5e1;
		font-size: 0.875rem;
	}

	.variant-rate {
		font-weight: 600;
		color: #f1f5f9;
	}

	.ab-variant.winner .variant-rate {
		color: #34d399;
	}

	/* Campaign Actions */
	.campaign-actions {
		display: flex;
		gap: 0.5rem;
		padding-top: 1rem;
		border-top: 1px solid rgba(148, 163, 184, 0.1);
		margin-top: 1rem;
	}

	/* Buttons */
	.btn-primary,
	.btn-secondary,
	.btn-danger {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
		border: none;
		font-size: 0.875rem;
	}

	.btn-primary {
		background: linear-gradient(135deg, #6366f1, #8b5cf6);
		color: white;
	}

	.btn-secondary {
		background: rgba(148, 163, 184, 0.1);
		color: #94a3b8;
		border: 1px solid rgba(148, 163, 184, 0.2);
	}

	.btn-danger {
		background: rgba(239, 68, 68, 0.1);
		color: #f87171;
		border: 1px solid rgba(239, 68, 68, 0.2);
	}

	.btn-primary.small,
	.btn-secondary.small,
	.btn-danger.small {
		padding: 0.5rem 0.875rem;
		font-size: 0.8125rem;
	}

	/* Error State */
	.error-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 3rem 2rem;
		color: #f87171;
		text-align: center;
		background: rgba(239, 68, 68, 0.1);
		border-radius: 16px;
		border: 1px solid rgba(239, 68, 68, 0.2);
		gap: 1rem;
	}

	/* Empty State */
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 2rem;
		color: #64748b;
		text-align: center;
		background: rgba(30, 41, 59, 0.3);
		border-radius: 16px;
		border: 2px dashed rgba(148, 163, 184, 0.2);
	}

	.empty-state h3 {
		color: #f1f5f9;
		margin: 1rem 0 0.5rem;
	}

	.empty-state p {
		margin-bottom: 1.5rem;
	}

	/* Loading */
	.loading-grid {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 1rem;
		margin-bottom: 2rem;
	}

	.skeleton {
		background: linear-gradient(90deg, rgba(148, 163, 184, 0.1) 25%, rgba(148, 163, 184, 0.2) 50%, rgba(148, 163, 184, 0.1) 75%);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
		border-radius: 16px;
	}

	.skeleton-metric {
		height: 100px;
	}

	@keyframes shimmer {
		0% { background-position: 200% 0; }
		100% { background-position: -200% 0; }
	}

	/* Modal */
	.modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.8);
		backdrop-filter: blur(4px);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 50;
		padding: 2rem;
	}

	.modal-content {
		background: #1e293b;
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 20px;
		width: 100%;
		max-width: 600px;
		max-height: 90vh;
		overflow-y: auto;
	}

	.modal-content.large {
		max-width: 700px;
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1.5rem;
		border-bottom: 1px solid rgba(148, 163, 184, 0.1);
	}

	.modal-header h2 {
		font-size: 1.25rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0;
	}

	.close-btn {
		width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(148, 163, 184, 0.1);
		border: none;
		border-radius: 8px;
		color: #94a3b8;
		cursor: pointer;
	}

	.modal-body {
		padding: 1.5rem;
	}

	.modal-footer {
		display: flex;
		justify-content: flex-end;
		gap: 0.75rem;
		padding: 1.5rem;
		border-top: 1px solid rgba(148, 163, 184, 0.1);
	}

	/* Form */
	.form-grid {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1rem;
	}

	.form-group {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.form-group.full-width {
		grid-column: 1 / -1;
	}

	.form-group label {
		font-size: 0.8125rem;
		font-weight: 500;
		color: #94a3b8;
	}

	.form-group input,
	.form-group select {
		padding: 0.75rem 1rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 10px;
		color: #f1f5f9;
		font-size: 0.9375rem;
	}

	.form-group input:focus,
	.form-group select:focus {
		outline: none;
		border-color: rgba(99, 102, 241, 0.5);
	}

	.form-group small {
		color: #64748b;
		font-size: 0.75rem;
	}

	.ab-toggle {
		padding: 0.75rem 0;
	}

	.checkbox-label {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		cursor: pointer;
	}

	.checkbox-label input[type="checkbox"] {
		width: 18px;
		height: 18px;
		accent-color: #6366f1;
	}

	.checkbox-label span {
		color: #f1f5f9;
		font-weight: 500;
	}

	.split-label {
		display: block;
		text-align: center;
		color: #94a3b8;
		font-size: 0.875rem;
		margin-top: 0.25rem;
	}

	input[type="range"] {
		width: 100%;
		accent-color: #6366f1;
	}

	@media (max-width: 768px) {
		.stats-grid {
			grid-template-columns: repeat(2, 1fr);
		}

		.header-content {
			flex-direction: column;
			gap: 1rem;
			align-items: flex-start;
		}

		.form-grid {
			grid-template-columns: 1fr;
		}
	}
</style>

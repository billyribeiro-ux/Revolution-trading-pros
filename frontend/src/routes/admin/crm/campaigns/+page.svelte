<!--
	/admin/crm/campaigns - Email Campaign Management
	Apple Principal Engineer ICT 7 Grade - January 2026
	
	Features:
	- One-time email broadcasts with stats tracking
	- Status filtering (draft, scheduled, sending, sent, paused, failed)
	- Open/click rate analytics
	- Duplicate and delete functionality
	- Full Svelte 5 $state/$derived reactivity
-->

<script lang="ts">
	/**
	 * Email Campaigns - FluentCRM Pro Style
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * One-time email campaigns and broadcasts.
	 *
	 * @version 1.0.0 (December 2025)
	 */

	import { browser } from '$app/environment';
	import {
		IconMail,
		IconPlus,
		IconSearch,
		IconEdit,
		IconTrash,
		IconEye,
		IconRefresh,
		IconUsers,
		IconChartBar,
		IconCopy,
		IconSend,
		IconClock,
		IconCheck,
		IconX
	} from '$lib/icons';
	import { api } from '$lib/api/config';
	import { toastStore } from '$lib/stores/toast.svelte';

	// ═══════════════════════════════════════════════════════════════════════════
	// TYPES
	// ═══════════════════════════════════════════════════════════════════════════

	interface Campaign {
		id: string;
		title: string;
		subject: string;
		status: 'draft' | 'scheduled' | 'sending' | 'sent' | 'paused' | 'failed';
		recipients_count: number;
		sent_count: number;
		open_count: number;
		click_count: number;
		bounce_count: number;
		unsubscribe_count: number;
		scheduled_at?: string;
		sent_at?: string;
		created_at: string;
		updated_at: string;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let campaigns = $state<Campaign[]>([]);
	let isLoading = $state(true);
	let error = $state('');
	let searchQuery = $state('');
	let selectedStatus = $state<string>('all');

	let stats = $state({
		total: 0,
		sent: 0,
		scheduled: 0,
		totalSent: 0
	});

	const statusOptions = [
		{ value: 'all', label: 'All Campaigns' },
		{ value: 'draft', label: 'Draft' },
		{ value: 'scheduled', label: 'Scheduled' },
		{ value: 'sending', label: 'Sending' },
		{ value: 'sent', label: 'Sent' },
		{ value: 'paused', label: 'Paused' },
		{ value: 'failed', label: 'Failed' }
	];

	// ═══════════════════════════════════════════════════════════════════════════
	// API FUNCTIONS
	// ═══════════════════════════════════════════════════════════════════════════

	async function loadCampaigns() {
		isLoading = true;
		error = '';

		try {
			const response = await api.get('/api/admin/crm/campaigns', {
				params: {
					search: searchQuery || undefined,
					status: selectedStatus !== 'all' ? selectedStatus : undefined
				}
			});
			campaigns = response?.data || response || [];

			stats = {
				total: campaigns.length,
				sent: campaigns.filter((c) => c.status === 'sent').length,
				scheduled: campaigns.filter((c) => c.status === 'scheduled').length,
				totalSent: campaigns.reduce((sum, c) => sum + (c.sent_count || 0), 0)
			};
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load campaigns';
		} finally {
			isLoading = false;
		}
	}

	async function duplicateCampaign(id: string) {
		try {
			await api.post(`/api/admin/crm/campaigns/${id}/duplicate`);
			toastStore.success('Campaign duplicated successfully');
			await loadCampaigns();
		} catch (err) {
			const message = err instanceof Error ? err.message : 'Failed to duplicate campaign';
			error = message;
			toastStore.error(message);
		}
	}

	async function deleteCampaign(id: string) {
		if (!confirm('Are you sure you want to delete this campaign?')) return;

		try {
			await api.delete(`/api/admin/crm/campaigns/${id}`);
			toastStore.success('Campaign deleted successfully');
			await loadCampaigns();
		} catch (err) {
			const message = err instanceof Error ? err.message : 'Failed to delete campaign';
			error = message;
			toastStore.error(message);
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// HELPERS
	// ═══════════════════════════════════════════════════════════════════════════

	function formatNumber(num: number): string {
		return num.toLocaleString();
	}

	function formatDate(dateString: string | undefined): string {
		if (!dateString) return 'Not set';
		return new Date(dateString).toLocaleDateString('en-US', {
			month: 'short',
			day: 'numeric',
			year: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	function formatRate(sent: number, metric: number): string {
		if (sent === 0) return '0%';
		return ((metric / sent) * 100).toFixed(1) + '%';
	}

	function getStatusColor(status: string): string {
		const colors: Record<string, string> = {
			draft: 'bg-slate-500/20 text-slate-400',
			scheduled: 'bg-blue-500/20 text-blue-400',
			sending: 'bg-amber-500/20 text-amber-400',
			sent: 'bg-emerald-500/20 text-emerald-400',
			paused: 'bg-orange-500/20 text-orange-400',
			failed: 'bg-red-500/20 text-red-400'
		};
		return colors[status] || colors.draft;
	}

	function getStatusIcon(status: string) {
		const icons: Record<string, any> = {
			draft: IconEdit,
			scheduled: IconClock,
			sending: IconSend,
			sent: IconCheck,
			paused: IconClock,
			failed: IconX
		};
		return icons[status] || IconMail;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let filteredCampaigns = $derived(
		campaigns.filter((campaign) => {
			const matchesSearch =
				!searchQuery ||
				campaign.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
				campaign.subject?.toLowerCase().includes(searchQuery.toLowerCase());
			const matchesStatus = selectedStatus === 'all' || campaign.status === selectedStatus;
			return matchesSearch && matchesStatus;
		})
	);

	// ═══════════════════════════════════════════════════════════════════════════
	// LIFECYCLE
	// ═══════════════════════════════════════════════════════════════════════════

	$effect(() => {
		if (browser) {
			loadCampaigns();
		}
	});
</script>

<svelte:head>
	<title>Email Campaigns - FluentCRM Pro</title>
</svelte:head>

<div class="campaigns-page">
	<!-- Header -->
	<div class="page-header">
		<div>
			<h1>
				<IconMail size={28} class="header-icon" />
				Email Campaigns
			</h1>
			<p class="page-description">Create and send one-time email broadcasts to your contacts</p>
		</div>
		<div class="header-actions">
			<button class="btn-refresh" onclick={() => loadCampaigns()} disabled={isLoading}>
				<IconRefresh size={18} class={isLoading ? 'spinning' : ''} />
			</button>
			<a href="/admin/crm/campaigns/new" class="btn-primary">
				<IconPlus size={18} />
				New Campaign
			</a>
		</div>
	</div>

	<!-- Stats Cards -->
	<div class="stats-grid">
		<div class="stat-card">
			<div class="stat-icon blue">
				<IconMail size={24} />
			</div>
			<div class="stat-content">
				<span class="stat-value">{formatNumber(stats.total)}</span>
				<span class="stat-label">Total Campaigns</span>
			</div>
		</div>
		<div class="stat-card">
			<div class="stat-icon green">
				<IconCheck size={24} />
			</div>
			<div class="stat-content">
				<span class="stat-value">{formatNumber(stats.sent)}</span>
				<span class="stat-label">Sent</span>
			</div>
		</div>
		<div class="stat-card">
			<div class="stat-icon purple">
				<IconClock size={24} />
			</div>
			<div class="stat-content">
				<span class="stat-value">{formatNumber(stats.scheduled)}</span>
				<span class="stat-label">Scheduled</span>
			</div>
		</div>
		<div class="stat-card">
			<div class="stat-icon amber">
				<IconSend size={24} />
			</div>
			<div class="stat-content">
				<span class="stat-value">{formatNumber(stats.totalSent)}</span>
				<span class="stat-label">Emails Delivered</span>
			</div>
		</div>
	</div>

	<!-- Search & Filters -->
	<div class="filters-bar">
		<div class="search-box">
			<IconSearch size={18} />
			<input
				type="text"
				id="search-campaigns"
				name="search"
				placeholder="Search campaigns..."
				bind:value={searchQuery}
			/>
		</div>
		<select class="filter-select" bind:value={selectedStatus}>
			{#each statusOptions as option}
				<option value={option.value}>{option.label}</option>
			{/each}
		</select>
	</div>

	<!-- Campaigns Table -->
	{#if isLoading}
		<div class="loading-state">
			<div class="spinner"></div>
			<p>Loading campaigns...</p>
		</div>
	{:else if error}
		<div class="error-state">
			<p>{error}</p>
			<button onclick={() => loadCampaigns()}>Try Again</button>
		</div>
	{:else if filteredCampaigns.length === 0}
		<div class="empty-state">
			<IconMail size={48} />
			<h3>No campaigns found</h3>
			<p>Create your first email campaign to reach your contacts</p>
			<a href="/admin/crm/campaigns/new" class="btn-primary">
				<IconPlus size={18} />
				Create Campaign
			</a>
		</div>
	{:else}
		<div class="table-container">
			<table class="data-table">
				<thead>
					<tr>
						<th>Campaign</th>
						<th>Status</th>
						<th>Recipients</th>
						<th>Open Rate</th>
						<th>Click Rate</th>
						<th>Sent / Scheduled</th>
						<th>Actions</th>
					</tr>
				</thead>
				<tbody>
					{#each filteredCampaigns as campaign}
						{@const CampaignStatusIcon = getStatusIcon(campaign.status)}
						<tr>
							<td>
								<div class="campaign-cell">
									<div class="campaign-icon">
										<IconMail size={20} />
									</div>
									<div class="campaign-info">
										<span class="campaign-title">{campaign.title}</span>
										<span class="campaign-subject">{campaign.subject}</span>
									</div>
								</div>
							</td>
							<td>
								<span class="status-badge {getStatusColor(campaign.status)}">
									<CampaignStatusIcon size={12} />
									{campaign.status}
								</span>
							</td>
							<td>{formatNumber(campaign.recipients_count)}</td>
							<td>
								<span class="rate-value"
									>{formatRate(campaign.sent_count, campaign.open_count)}</span
								>
							</td>
							<td>
								<span class="rate-value"
									>{formatRate(campaign.sent_count, campaign.click_count)}</span
								>
							</td>
							<td>
								<span class="date-value">
									{campaign.status === 'sent'
										? formatDate(campaign.sent_at)
										: formatDate(campaign.scheduled_at)}
								</span>
							</td>
							<td>
								<div class="action-buttons">
									<a href="/admin/crm/campaigns/{campaign.id}" class="btn-icon" title="View Report">
										<IconChartBar size={16} />
									</a>
									<a href="/admin/crm/campaigns/{campaign.id}/edit" class="btn-icon" title="Edit">
										<IconEdit size={16} />
									</a>
									<button
										class="btn-icon"
										title="Duplicate"
										onclick={() => duplicateCampaign(campaign.id)}
									>
										<IconCopy size={16} />
									</button>
									<button
										class="btn-icon danger"
										title="Delete"
										onclick={() => deleteCampaign(campaign.id)}
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

<style>
	.campaigns-page {
		max-width: 1600px;
		padding: 24px;
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

	.page-header h1 {
		display: flex;
		align-items: center;
		gap: 12px;
		font-size: 1.75rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0 0 0.25rem 0;
	}

	.page-header h1 :global(.header-icon) {
		color: #f97316;
	}

	.page-description {
		color: #64748b;
		margin: 0;
	}

	.header-actions {
		display: flex;
		gap: 0.75rem;
	}

	.btn-refresh {
		width: 42px;
		height: 42px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(30, 41, 59, 0.8);
		border: 1px solid rgba(249, 115, 22, 0.2);
		border-radius: 10px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-refresh:hover {
		background: rgba(249, 115, 22, 0.2);
		color: #f97316;
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
		background: linear-gradient(135deg, #f97316, #ea580c);
		color: white;
		border: none;
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
		text-decoration: none;
	}

	.btn-primary:hover {
		transform: translateY(-2px);
		box-shadow: 0 4px 15px rgba(249, 115, 22, 0.4);
	}

	/* Stats Grid */
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
		border: 1px solid rgba(249, 115, 22, 0.1);
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
		background: rgba(230, 184, 0, 0.15);
		color: var(--primary-500);
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
		padding: 0 1rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(249, 115, 22, 0.1);
		border-radius: 10px;
		flex: 1;
		max-width: 400px;
	}

	.search-box :global(svg) {
		color: #64748b;
	}

	.search-box input {
		flex: 1;
		padding: 0.75rem 0;
		background: transparent;
		border: none;
		color: #e2e8f0;
		font-size: 0.9rem;
		outline: none;
	}

	.search-box input::placeholder {
		color: #64748b;
	}

	.filter-select {
		padding: 0.75rem 1rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(249, 115, 22, 0.1);
		border-radius: 10px;
		color: #e2e8f0;
		font-size: 0.9rem;
		cursor: pointer;
	}

	/* Table */
	.table-container {
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(249, 115, 22, 0.1);
		border-radius: 14px;
		overflow: hidden;
	}

	.data-table {
		width: 100%;
		border-collapse: collapse;
	}

	.data-table th {
		padding: 1rem;
		text-align: left;
		font-size: 0.75rem;
		font-weight: 600;
		color: #64748b;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		background: rgba(249, 115, 22, 0.05);
		border-bottom: 1px solid rgba(249, 115, 22, 0.1);
	}

	.data-table td {
		padding: 1rem;
		font-size: 0.9rem;
		color: #e2e8f0;
		border-bottom: 1px solid rgba(249, 115, 22, 0.05);
	}

	.data-table tbody tr:hover {
		background: rgba(249, 115, 22, 0.05);
	}

	.campaign-cell {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.campaign-icon {
		width: 40px;
		height: 40px;
		border-radius: 10px;
		background: linear-gradient(135deg, #f97316, #ea580c);
		display: flex;
		align-items: center;
		justify-content: center;
		color: white;
		flex-shrink: 0;
	}

	.campaign-info {
		display: flex;
		flex-direction: column;
		gap: 0.125rem;
	}

	.campaign-title {
		font-weight: 600;
		color: #f1f5f9;
	}

	.campaign-subject {
		font-size: 0.8rem;
		color: #64748b;
	}

	.status-badge {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.25rem 0.75rem;
		border-radius: 20px;
		font-size: 0.75rem;
		font-weight: 600;
		text-transform: capitalize;
	}

	.rate-value {
		font-weight: 600;
		color: #4ade80;
	}

	.date-value {
		font-size: 0.8rem;
		color: #94a3b8;
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
		background: transparent;
		border: 1px solid rgba(249, 115, 22, 0.2);
		border-radius: 8px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
		text-decoration: none;
	}

	.btn-icon:hover {
		background: rgba(249, 115, 22, 0.1);
		color: #f97316;
		border-color: rgba(249, 115, 22, 0.3);
	}

	.btn-icon.danger:hover {
		background: rgba(239, 68, 68, 0.1);
		color: #f87171;
		border-color: rgba(239, 68, 68, 0.3);
	}

	/* States */
	.loading-state,
	.error-state,
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 2rem;
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

	.spinner {
		width: 40px;
		height: 40px;
		border: 3px solid rgba(249, 115, 22, 0.2);
		border-top-color: #f97316;
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin-bottom: 1rem;
	}

	/* Responsive */
	@media (max-width: 1200px) {
		.stats-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	@media (max-width: 640px) {
		.campaigns-page {
			padding: 16px;
		}

		.stats-grid {
			grid-template-columns: 1fr;
		}

		.data-table {
			font-size: 0.8rem;
		}

		.data-table th,
		.data-table td {
			padding: 0.75rem 0.5rem;
		}
	}
</style>

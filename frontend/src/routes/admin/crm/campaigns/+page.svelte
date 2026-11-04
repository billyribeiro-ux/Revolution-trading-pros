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
	 *
	 * One-time email campaigns and broadcasts.
	 *
	 */

	import { onMount, type Component } from 'svelte';
	import {
		IconMail,
		IconPlus,
		IconSearch,
		IconEdit,
		IconTrash,
		IconRefresh,
		IconChartBar,
		IconCopy,
		IconSend,
		IconClock,
		IconCheck,
		IconX
	} from '#lib/icons/index.js';
	import { crmAPI } from '#lib/api/crm.js';
	import type { Campaign } from '#lib/crm/types.js';
	import { toastStore } from '#lib/stores/toast.svelte.js';
	import ConfirmationModal from '#lib/components/admin/ConfirmationModal.svelte';

	// TYPES

	// STATE

	let campaigns = $state<Campaign[]>([]);
	let isLoading = $state(true);
	let error = $state('');
	let searchQuery = $state('');
	let selectedStatus = $state<string>('all');
	let currentPage = $state(1),
		totalPages = $state(1),
		total = $state(0);

	// Delete confirmation modal state
	let showDeleteModal = $state(false);
	let pendingDeleteId = $state<string | null>(null);

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

	// API FUNCTIONS

	async function loadCampaigns() {
		isLoading = true;
		error = '';

		try {
			const response = await crmAPI.getCampaigns({
				page: currentPage,
				per_page: 25,
				search: searchQuery || undefined,
				status: selectedStatus === 'all' ? undefined : selectedStatus
			});
			campaigns = response.data;
			total = response.meta.total;
			totalPages = response.meta.total_pages ?? 1;
			if (currentPage > totalPages) {
				currentPage = totalPages;
				await loadCampaigns();
				return;
			}
			stats = {
				total: response.stats.total,
				sent: response.stats.sent,
				scheduled: response.stats.scheduled,
				totalSent: response.stats.total_emails_sent
			};
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load campaigns';
		} finally {
			isLoading = false;
		}
	}

	async function duplicateCampaign(id: string) {
		try {
			await crmAPI.duplicateCampaign(id);
			toastStore.success('Campaign duplicated successfully');
			await loadCampaigns();
		} catch (err) {
			const message = err instanceof Error ? err.message : 'Failed to duplicate campaign';
			error = message;
			toastStore.error(message);
		}
	}

	function deleteCampaign(id: string) {
		pendingDeleteId = id;
		showDeleteModal = true;
	}

	async function confirmDeleteCampaign() {
		if (!pendingDeleteId) return;
		showDeleteModal = false;
		const id = pendingDeleteId;
		pendingDeleteId = null;

		try {
			await crmAPI.deleteCampaign(id);
			toastStore.success('Campaign deleted successfully');
			await loadCampaigns();
		} catch (err) {
			const message = err instanceof Error ? err.message : 'Failed to delete campaign';
			error = message;
			toastStore.error(message);
		}
	}

	// HELPERS

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

	function getStatusClass(status: Campaign['status']): string {
		return `status-${status}`;
	}

	function getStatusIcon(status: string) {
		const icons: Record<string, Component<{ size?: number | string }>> = {
			draft: IconEdit,
			scheduled: IconClock,
			sending: IconSend,
			sent: IconCheck,
			paused: IconClock,
			failed: IconX
		};
		return icons[status] || IconMail;
	}

	// DERIVED STATE

	let filteredCampaigns = $derived(campaigns);
	function applyFilters() {
		currentPage = 1;
		void loadCampaigns();
	}
	function changePage(next: number) {
		currentPage = next;
		void loadCampaigns();
	}

	// LIFECYCLE

	onMount(() => {
		void loadCampaigns();
	});
</script>

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
				autocomplete="off"
				type="text"
				id="search-campaigns"
				name="search"
				placeholder="Search campaigns..."
				bind:value={searchQuery}
				onkeydown={(event) => {
					if (event.key === 'Enter') applyFilters();
				}}
			/>
		</div>
		<select
			id="campaigns-selectedstatus"
			name="campaigns_selectedstatus"
			class="filter-select"
			bind:value={selectedStatus}
			onchange={applyFilters}
		>
			{#each statusOptions as option (option.value)}
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
					{#each filteredCampaigns as campaign (campaign.id)}
						{@const CampaignStatusIcon = getStatusIcon(campaign.status)}
						<tr>
							<td>
								<div class="campaign-cell">
									<div class="campaign-icon">
										<IconMail size={20} />
									</div>
									<div class="campaign-info">
										<span class="campaign-title">{campaign.title}</span>
										<span class="campaign-subject">{campaign.email_subject}</span>
									</div>
								</div>
							</td>
							<td>
								<span class={['status-badge', getStatusClass(campaign.status)]}>
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

<nav class="campaign-pagination" aria-label="Campaign pages">
	<button disabled={isLoading || currentPage <= 1} onclick={() => changePage(currentPage - 1)}
		>Previous</button
	><span>Page {currentPage} of {totalPages} · {total} matching campaigns</span><button
		disabled={isLoading || currentPage >= totalPages}
		onclick={() => changePage(currentPage + 1)}>Next</button
	><button onclick={applyFilters}>Search campaigns</button>
</nav>

<ConfirmationModal
	isOpen={showDeleteModal}
	title="Delete Campaign"
	message="Are you sure you want to delete this campaign?"
	confirmText="Delete"
	variant="danger"
	onConfirm={confirmDeleteCampaign}
	onCancel={() => {
		showDeleteModal = false;
		pendingDeleteId = null;
	}}
/>

<style>
	.campaign-pagination {
		display: flex;
		align-items: center;
		gap: 1rem;
		flex-wrap: wrap;
		margin: 1rem 0;
		padding: 1rem 2rem;
	}
	.campaign-pagination button {
		padding: 0.65rem;
		border: 1px solid var(--border-color, #475569);
		border-radius: 0.5rem;
		background: var(--bg-secondary, #18181b);
		color: inherit;
		cursor: pointer;
	}
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
		color: var(--text-primary, #f0f6fc);
		margin: 0 0 0.25rem 0;
	}

	.page-header h1 :global(.header-icon) {
		color: var(--warning-emphasis, #d29922);
	}

	.page-description {
		color: var(--text-tertiary, #6e7681);
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
		background: var(--bg-elevated, #161b22);
		border: 1px solid rgba(210, 153, 34, 0.2);
		border-radius: 10px;
		color: var(--text-secondary, #8b949e);
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-refresh:hover {
		background: rgba(210, 153, 34, 0.2);
		color: var(--warning-emphasis, #d29922);
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
		background: var(--primary-500, #e6b800);
		color: var(--bg-base, #0d1117);
		border: none;
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
		text-decoration: none;
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
		background: var(--bg-elevated, #161b22);
		border: 1px solid rgba(210, 153, 34, 0.1);
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
		color: var(--primary-500);
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
		background: var(--bg-base, #0d1117);
		border: 1px solid rgba(210, 153, 34, 0.1);
		border-radius: 10px;
		flex: 1;
		max-width: 400px;
	}

	.search-box :global(svg) {
		color: var(--text-tertiary, #6e7681);
	}

	.search-box input {
		flex: 1;
		padding: 0.75rem 0;
		background: transparent;
		border: none;
		color: var(--text-primary, #f0f6fc);
		font-size: 0.9rem;
		outline: none;
	}

	.search-box input::placeholder {
		color: var(--text-tertiary, #6e7681);
	}

	.filter-select {
		padding: 0.75rem 1rem;
		background: var(--bg-base, #0d1117);
		border: 1px solid rgba(210, 153, 34, 0.1);
		border-radius: 10px;
		color: var(--text-primary, #f0f6fc);
		font-size: 0.9rem;
		cursor: pointer;
	}

	/* Table */
	.table-container {
		background: var(--bg-base, #0d1117);
		border: 1px solid rgba(210, 153, 34, 0.1);
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
		color: var(--text-tertiary, #6e7681);
		text-transform: uppercase;
		letter-spacing: 0.05em;
		background: rgba(210, 153, 34, 0.05);
		border-bottom: 1px solid rgba(210, 153, 34, 0.1);
	}

	.data-table td {
		padding: 1rem;
		font-size: 0.9rem;
		color: var(--text-primary, #f0f6fc);
		border-bottom: 1px solid rgba(210, 153, 34, 0.05);
	}

	.data-table tbody tr:hover {
		background: rgba(210, 153, 34, 0.05);
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
		background: var(--primary-500, #e6b800);
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--bg-base, #0d1117);
		flex-shrink: 0;
	}

	.campaign-info {
		display: flex;
		flex-direction: column;
		gap: 0.125rem;
	}

	.campaign-title {
		font-weight: 600;
		color: var(--text-primary, #f0f6fc);
	}

	.campaign-subject {
		font-size: 0.8rem;
		color: var(--text-tertiary, #6e7681);
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

	.status-badge.status-draft {
		background: var(--bg-hover, #252b33);
		color: var(--text-secondary, #8b949e);
	}

	.status-badge.status-scheduled {
		background: rgba(88, 166, 255, 0.2);
		color: var(--info-emphasis, #58a6ff);
	}

	.status-badge.status-sending {
		background: rgba(210, 153, 34, 0.2);
		color: var(--warning-emphasis, #d29922);
	}

	.status-badge.status-sent {
		background: rgba(63, 185, 80, 0.2);
		color: var(--success-emphasis, #3fb950);
	}

	.status-badge.status-paused {
		background: rgba(210, 153, 34, 0.2);
		color: var(--warning-emphasis, #d29922);
	}

	.status-badge.status-failed {
		background: rgba(248, 81, 73, 0.2);
		color: var(--error-emphasis, #f85149);
	}

	.rate-value {
		font-weight: 600;
		color: var(--success-emphasis, #3fb950);
	}

	.date-value {
		font-size: 0.8rem;
		color: var(--text-secondary, #8b949e);
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
		border: 1px solid rgba(210, 153, 34, 0.2);
		border-radius: 8px;
		color: var(--text-secondary, #8b949e);
		cursor: pointer;
		transition: all 0.2s;
		text-decoration: none;
	}

	.btn-icon:hover {
		background: rgba(210, 153, 34, 0.1);
		color: var(--warning-emphasis, #d29922);
		border-color: rgba(210, 153, 34, 0.3);
	}

	.btn-icon.danger:hover {
		background: rgba(248, 81, 73, 0.1);
		color: var(--error-emphasis, #f85149);
		border-color: rgba(248, 81, 73, 0.3);
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

	.spinner {
		width: 40px;
		height: 40px;
		border: 3px solid rgba(210, 153, 34, 0.2);
		border-top-color: var(--primary-500, #e6b800);
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin-bottom: 1rem;
	}

	/* Responsive */
	@media (max-width: 1199.98px) {
		.stats-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	@media (max-width: 639.98px) {
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

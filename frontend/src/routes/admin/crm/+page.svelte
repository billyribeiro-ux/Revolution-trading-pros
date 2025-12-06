<script lang="ts">
	import { onMount } from 'svelte';
	import {
		IconUsers,
		IconUserPlus,
		IconMail,
		IconPhone,
		IconBuilding,
		IconCalendar,
		IconSearch,
		IconFilter,
		IconDotsVertical,
		IconEdit,
		IconTrash,
		IconEye,
		IconTag,
		IconChartBar,
		IconTrendingUp,
		IconCurrencyDollar,
		IconRefresh
	} from '@tabler/icons-svelte';
	import { api } from '$lib/api/config';

	let contacts: any[] = [];
	let deals: any[] = [];
	let isLoading = true;
	let error = '';
	let searchQuery = '';
	let activeTab: 'contacts' | 'deals' | 'pipeline' = 'contacts';
	let selectedStatus = 'all';

	// Stats
	let stats = {
		totalContacts: 0,
		newThisMonth: 0,
		activeDeals: 0,
		dealValue: 0
	};

	const statusOptions = [
		{ value: 'all', label: 'All Contacts' },
		{ value: 'lead', label: 'Leads' },
		{ value: 'prospect', label: 'Prospects' },
		{ value: 'customer', label: 'Customers' },
		{ value: 'churned', label: 'Churned' }
	];

	const pipelineStages = [
		{ id: 'lead', name: 'Lead', color: '#94a3b8' },
		{ id: 'qualified', name: 'Qualified', color: '#60a5fa' },
		{ id: 'proposal', name: 'Proposal', color: '#a78bfa' },
		{ id: 'negotiation', name: 'Negotiation', color: '#fbbf24' },
		{ id: 'closed', name: 'Closed Won', color: '#4ade80' }
	];

	async function loadData() {
		isLoading = true;
		error = '';

		try {
			const [contactsRes, dealsRes, statsRes] = await Promise.allSettled([
				api.get('/api/admin/crm/contacts'),
				api.get('/api/admin/crm/deals'),
				api.get('/api/admin/crm/stats')
			]);

			if (contactsRes.status === 'fulfilled') {
				contacts = contactsRes.value?.data || contactsRes.value?.contacts || [];
			}

			if (dealsRes.status === 'fulfilled') {
				deals = dealsRes.value?.data || dealsRes.value?.deals || [];
			}

			if (statsRes.status === 'fulfilled') {
				const data = statsRes.value?.data || statsRes.value;
				stats = {
					totalContacts: data?.total_contacts || contacts.length,
					newThisMonth: data?.new_this_month || 0,
					activeDeals: data?.active_deals || deals.length,
					dealValue: data?.deal_value || 0
				};
			}
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load CRM data';
		} finally {
			isLoading = false;
		}
	}

	function formatCurrency(amount: number): string {
		return new Intl.NumberFormat('en-US', {
			style: 'currency',
			currency: 'USD',
			minimumFractionDigits: 0
		}).format(amount);
	}

	function formatDate(dateString: string): string {
		if (!dateString) return 'N/A';
		return new Date(dateString).toLocaleDateString();
	}

	$: filteredContacts = contacts.filter(contact => {
		const matchesSearch = !searchQuery || 
			contact.name?.toLowerCase().includes(searchQuery.toLowerCase()) ||
			contact.email?.toLowerCase().includes(searchQuery.toLowerCase()) ||
			contact.company?.toLowerCase().includes(searchQuery.toLowerCase());
		const matchesStatus = selectedStatus === 'all' || contact.status === selectedStatus;
		return matchesSearch && matchesStatus;
	});

	onMount(() => {
		loadData();
	});
</script>

<svelte:head>
	<title>CRM - Admin Dashboard</title>
</svelte:head>

<div class="crm-page" data-page="crm" data-module="customer-relationship-management" role="main" aria-label="CRM Dashboard">
	<!-- Header -->
	<header class="page-header" data-section="header" aria-label="Page Header">
		<div>
			<h1>Customer Relationship Management</h1>
			<p class="page-description">Manage contacts, deals, and customer relationships</p>
		</div>
		<div class="header-actions">
			<button class="btn-refresh" onclick={loadData} disabled={isLoading}>
				<IconRefresh size={18} class={isLoading ? 'spinning' : ''} />
			</button>
			<button class="btn-primary">
				<IconUserPlus size={18} />
				Add Contact
			</button>
		</div>
	</header>

	<!-- Stats Cards -->
	<section class="stats-grid" data-section="statistics" aria-label="CRM Statistics">
		<div class="stat-card" data-stat="total-contacts" aria-label="Total Contacts">
			<div class="stat-icon blue">
				<IconUsers size={24} />
			</div>
			<div class="stat-content">
				<span class="stat-value" data-value="{stats.totalContacts}">{stats.totalContacts.toLocaleString()}</span>
				<span class="stat-label">Total Contacts</span>
			</div>
		</div>
		<div class="stat-card" data-stat="new-this-month" aria-label="New Contacts This Month">
			<div class="stat-icon green">
				<IconUserPlus size={24} />
			</div>
			<div class="stat-content">
				<span class="stat-value" data-value="{stats.newThisMonth}">{stats.newThisMonth}</span>
				<span class="stat-label">New This Month</span>
			</div>
		</div>
		<div class="stat-card" data-stat="active-deals" aria-label="Active Deals">
			<div class="stat-icon purple">
				<IconTrendingUp size={24} />
			</div>
			<div class="stat-content">
				<span class="stat-value" data-value="{stats.activeDeals}">{stats.activeDeals}</span>
				<span class="stat-label">Active Deals</span>
			</div>
		</div>
		<div class="stat-card" data-stat="pipeline-value" aria-label="Pipeline Value">
			<div class="stat-icon amber">
				<IconCurrencyDollar size={24} />
			</div>
			<div class="stat-content">
				<span class="stat-value" data-value="{stats.dealValue}" data-currency="USD">{formatCurrency(stats.dealValue)}</span>
				<span class="stat-label">Pipeline Value</span>
			</div>
		</div>
	</section>

	<!-- Tabs -->
	<nav class="tabs" data-section="navigation" aria-label="CRM Navigation Tabs" role="tablist">
		<button class="tab" class:active={activeTab === 'contacts'} onclick={() => activeTab = 'contacts'} role="tab" aria-selected={activeTab === 'contacts'} data-tab="contacts">
			<IconUsers size={18} />
			Contacts
		</button>
		<button class="tab" class:active={activeTab === 'deals'} onclick={() => activeTab = 'deals'} role="tab" aria-selected={activeTab === 'deals'} data-tab="deals">
			<IconTrendingUp size={18} />
			Deals
		</button>
		<button class="tab" class:active={activeTab === 'pipeline'} onclick={() => activeTab = 'pipeline'} role="tab" aria-selected={activeTab === 'pipeline'} data-tab="pipeline">
			<IconChartBar size={18} />
			Pipeline
		</button>
	</nav>

	<!-- Content -->
	<section data-section="content" data-active-tab={activeTab} aria-label="CRM Content">
	{#if activeTab === 'contacts'}
		<!-- Search & Filters -->
		<div class="filters-bar" data-component="filters" aria-label="Contact Filters">
			<div class="search-box">
				<IconSearch size={18} />
				<input type="text" placeholder="Search contacts..." bind:value={searchQuery} aria-label="Search contacts" data-filter="search" />
			</div>
			<select class="filter-select" bind:value={selectedStatus} aria-label="Filter by status" data-filter="status">
				{#each statusOptions as option}
					<option value={option.value}>{option.label}</option>
				{/each}
			</select>
		</div>

		<!-- Contacts Table -->
		<div data-component="contacts-table" data-state={isLoading ? 'loading' : error ? 'error' : filteredContacts.length === 0 ? 'empty' : 'loaded'} data-count={filteredContacts.length}>
		{#if isLoading}
			<div class="loading-state">
				<div class="spinner"></div>
				<p>Loading contacts...</p>
			</div>
		{:else if error}
			<div class="error-state">
				<p>{error}</p>
				<button onclick={loadData}>Try Again</button>
			</div>
		{:else if filteredContacts.length === 0}
			<div class="empty-state">
				<IconUsers size={48} />
				<h3>No contacts found</h3>
				<p>Add your first contact to get started with CRM</p>
				<button class="btn-primary">
					<IconUserPlus size={18} />
					Add Contact
				</button>
			</div>
		{:else}
			<div class="table-container">
				<table class="data-table" data-entity="contacts" aria-label="Contacts List">
					<thead>
						<tr>
							<th>Contact</th>
							<th>Company</th>
							<th>Status</th>
							<th>Last Activity</th>
							<th>Value</th>
							<th>Actions</th>
						</tr>
					</thead>
					<tbody>
						{#each filteredContacts as contact (contact.id || contact.email)}
							<tr data-contact-id={contact.id} data-contact-email={contact.email} data-contact-status={contact.status || 'lead'}>
								<td>
									<div class="contact-cell">
										<div class="contact-avatar">
											{contact.name?.charAt(0).toUpperCase() || '?'}
										</div>
										<div class="contact-info">
											<span class="contact-name">{contact.name || 'Unknown'}</span>
											<span class="contact-email">{contact.email || 'No email'}</span>
										</div>
									</div>
								</td>
								<td>{contact.company || '-'}</td>
								<td>
									<span class="status-badge {contact.status || 'lead'}">
										{contact.status || 'Lead'}
									</span>
								</td>
								<td>{formatDate(contact.last_activity_at)}</td>
								<td>{contact.lifetime_value ? formatCurrency(contact.lifetime_value) : '-'}</td>
								<td>
									<div class="action-buttons">
										<button class="btn-icon" title="View">
											<IconEye size={16} />
										</button>
										<button class="btn-icon" title="Edit">
											<IconEdit size={16} />
										</button>
										<button class="btn-icon danger" title="Delete">
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
	{:else if activeTab === 'deals'}
		<div class="empty-state">
			<IconTrendingUp size={48} />
			<h3>Deals Management</h3>
			<p>Track and manage your sales deals</p>
		</div>
	{:else if activeTab === 'pipeline'}
		<!-- Pipeline View -->
		<div class="pipeline-view" data-component="pipeline" aria-label="Sales Pipeline">
			{#each pipelineStages as stage (stage.id)}
				<div class="pipeline-column" data-stage-id={stage.id} data-stage-name={stage.name}>
					<div class="pipeline-header" style="border-color: {stage.color}">
						<span class="pipeline-name">{stage.name}</span>
						<span class="pipeline-count">0</span>
					</div>
					<div class="pipeline-cards">
						<div class="pipeline-empty">No deals</div>
					</div>
				</div>
			{/each}
		</div>
	{/if}
	</section>
</div>

<style>
	.crm-page {
		max-width: 1600px;
	}

	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 2rem;
		flex-wrap: wrap;
		gap: 1rem;
	}

	.page-header h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0 0 0.25rem 0;
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
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 10px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-refresh:hover {
		background: rgba(99, 102, 241, 0.2);
		color: #818cf8;
	}

	.btn-refresh :global(.spinning) {
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		from { transform: rotate(0deg); }
		to { transform: rotate(360deg); }
	}

	.btn-primary {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background: linear-gradient(135deg, #6366f1, #8b5cf6);
		color: white;
		border: none;
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-primary:hover {
		transform: translateY(-2px);
		box-shadow: 0 4px 15px rgba(99, 102, 241, 0.4);
	}

	/* Stats Grid */
	.stats-grid {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 1rem;
		margin-bottom: 2rem;
	}

	@media (max-width: 1200px) {
		.stats-grid { grid-template-columns: repeat(2, 1fr); }
	}

	@media (max-width: 640px) {
		.stats-grid { grid-template-columns: 1fr; }
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

	.stat-icon.blue { background: rgba(59, 130, 246, 0.15); color: #60a5fa; }
	.stat-icon.green { background: rgba(34, 197, 94, 0.15); color: #4ade80; }
	.stat-icon.purple { background: rgba(139, 92, 246, 0.15); color: #a78bfa; }
	.stat-icon.amber { background: rgba(245, 158, 11, 0.15); color: #fbbf24; }

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
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background: transparent;
		border: none;
		color: #64748b;
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
		color: #818cf8;
		background: rgba(99, 102, 241, 0.15);
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
		border: 1px solid rgba(99, 102, 241, 0.1);
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
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 10px;
		color: #e2e8f0;
		font-size: 0.9rem;
		cursor: pointer;
	}

	/* Table */
	.table-container {
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(99, 102, 241, 0.1);
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
		background: rgba(99, 102, 241, 0.05);
		border-bottom: 1px solid rgba(99, 102, 241, 0.1);
	}

	.data-table td {
		padding: 1rem;
		font-size: 0.9rem;
		color: #e2e8f0;
		border-bottom: 1px solid rgba(99, 102, 241, 0.05);
	}

	.data-table tbody tr:hover {
		background: rgba(99, 102, 241, 0.05);
	}

	.contact-cell {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.contact-avatar {
		width: 40px;
		height: 40px;
		border-radius: 10px;
		background: linear-gradient(135deg, #6366f1, #8b5cf6);
		display: flex;
		align-items: center;
		justify-content: center;
		font-weight: 600;
		color: white;
	}

	.contact-info {
		display: flex;
		flex-direction: column;
		gap: 0.125rem;
	}

	.contact-name {
		font-weight: 600;
		color: #f1f5f9;
	}

	.contact-email {
		font-size: 0.8rem;
		color: #64748b;
	}

	.status-badge {
		display: inline-block;
		padding: 0.25rem 0.75rem;
		border-radius: 20px;
		font-size: 0.75rem;
		font-weight: 500;
		text-transform: capitalize;
	}

	.status-badge.lead { background: rgba(148, 163, 184, 0.2); color: #94a3b8; }
	.status-badge.prospect { background: rgba(59, 130, 246, 0.2); color: #60a5fa; }
	.status-badge.customer { background: rgba(34, 197, 94, 0.2); color: #4ade80; }
	.status-badge.churned { background: rgba(239, 68, 68, 0.2); color: #f87171; }

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
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 8px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-icon:hover {
		background: rgba(99, 102, 241, 0.1);
		color: #818cf8;
	}

	.btn-icon.danger:hover {
		background: rgba(239, 68, 68, 0.1);
		color: #f87171;
		border-color: rgba(239, 68, 68, 0.3);
	}

	/* Pipeline */
	.pipeline-view {
		display: grid;
		grid-template-columns: repeat(5, 1fr);
		gap: 1rem;
		overflow-x: auto;
	}

	@media (max-width: 1200px) {
		.pipeline-view {
			grid-template-columns: repeat(5, 250px);
		}
	}

	.pipeline-column {
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 12px;
		min-height: 400px;
	}

	.pipeline-header {
		padding: 1rem;
		border-bottom: 2px solid;
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.pipeline-name {
		font-weight: 600;
		color: #e2e8f0;
	}

	.pipeline-count {
		background: rgba(99, 102, 241, 0.2);
		padding: 0.25rem 0.5rem;
		border-radius: 6px;
		font-size: 0.75rem;
		color: #94a3b8;
	}

	.pipeline-cards {
		padding: 1rem;
	}

	.pipeline-empty {
		text-align: center;
		color: #64748b;
		font-size: 0.85rem;
		padding: 2rem 1rem;
	}

	/* States */
	.loading-state, .error-state, .empty-state {
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
		border: 3px solid rgba(99, 102, 241, 0.2);
		border-top-color: #6366f1;
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin-bottom: 1rem;
	}
</style>

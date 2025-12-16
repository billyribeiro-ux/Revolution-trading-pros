<script lang="ts">
	/**
	 * CRM Admin - FluentCRM Pro Identical Implementation
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Full CRM dashboard matching FluentCRM Pro functionality:
	 * - Search with Quick Links dropdown
	 * - Contact Segments, Email Sequences, Recurring Campaigns
	 * - Full contact management
	 *
	 * @version 2.0.0 (December 2025)
	 */

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
		IconRefresh,
		IconListDetails,
		IconMailForward,
		IconRepeat,
		IconBook,
		IconPlayerPlay,
		IconExternalLink,
		IconLink,
		IconRobot,
		IconClick
	} from '$lib/icons';
	import { api } from '$lib/api/config';
	import { connections, isCrmConnected } from '$lib/stores/connections';
	import ApiNotConnected from '$lib/components/ApiNotConnected.svelte';
	import SkeletonLoader from '$lib/components/SkeletonLoader.svelte';

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let contacts = $state<any[]>([]);
	let deals = $state<any[]>([]);
	let isLoading = $state(true);
	let connectionLoading = $state(true);
	let error = $state('');
	let searchQuery = $state('');
	let activeTab = $state<'contacts' | 'deals' | 'pipeline'>('contacts');
	let selectedStatus = $state('all');
	let showSearchDropdown = $state(false);
	let searchInputFocused = $state(false);

	// Stats
	let stats = $state({
		totalContacts: 0,
		newThisMonth: 0,
		activeDeals: 0,
		dealValue: 0,
		highScoreContacts: 0,
		atRiskContacts: 0
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// QUICK LINKS (FluentCRM Pro Style)
	// ═══════════════════════════════════════════════════════════════════════════

	const quickLinks = [
		{ name: 'Contact Segments', href: '/admin/crm/segments', icon: IconListDetails },
		{ name: 'Recurring Campaigns', href: '/admin/crm/recurring-campaigns', icon: IconRepeat },
		{ name: 'Email Sequences', href: '/admin/crm/sequences', icon: IconMailForward },
		{ name: 'Documentations', href: 'https://docs.revolutiontradingpros.com', icon: IconBook, external: true },
		{ name: 'Video Tutorials (Free)', href: '/tutorials', icon: IconPlayerPlay }
	];

	// ═══════════════════════════════════════════════════════════════════════════
	// SIDEBAR NAVIGATION (FluentCRM Pro Style)
	// ═══════════════════════════════════════════════════════════════════════════

	const sidebarNav = [
		{ name: 'Contacts', href: '/admin/crm', icon: IconUsers, active: true },
		{ name: 'Lists', href: '/admin/crm/lists', icon: IconListDetails },
		{ name: 'Tags', href: '/admin/crm/tags', icon: IconTag },
		{ name: 'Segments', href: '/admin/crm/segments', icon: IconFilter },
		{ name: 'Companies', href: '/admin/crm/companies', icon: IconBuilding },
		{ name: 'Campaigns', href: '/admin/crm/campaigns', icon: IconMail },
		{ name: 'Email Sequences', href: '/admin/crm/sequences', icon: IconMailForward },
		{ name: 'Recurring Campaigns', href: '/admin/crm/recurring-campaigns', icon: IconRepeat },
		{ name: 'Automations', href: '/admin/crm/automations', icon: IconRobot },
		{ name: 'Smart Links', href: '/admin/crm/smart-links', icon: IconLink },
		{ name: 'Email Templates', href: '/admin/crm/templates', icon: IconMail },
		{ name: 'Abandoned Carts', href: '/admin/crm/abandoned-carts', icon: IconCurrencyDollar },
		{ name: 'Webhooks', href: '/admin/crm/webhooks', icon: IconLink },
		{ name: 'Import / Export', href: '/admin/crm/import-export', icon: IconChartBar },
		{ name: 'System Logs', href: '/admin/crm/logs', icon: IconListDetails },
		{ name: 'Settings', href: '/admin/crm/settings', icon: IconDotsVertical }
	];

	const statusOptions = [
		{ value: 'all', label: 'All Contacts' },
		{ value: 'subscribed', label: 'Subscribed' },
		{ value: 'pending', label: 'Pending' },
		{ value: 'unsubscribed', label: 'Unsubscribed' },
		{ value: 'bounced', label: 'Bounced' },
		{ value: 'complained', label: 'Complained' }
	];

	const lifecycleStages = [
		{ value: 'all', label: 'All Stages' },
		{ value: 'subscriber', label: 'Subscriber' },
		{ value: 'lead', label: 'Lead' },
		{ value: 'mql', label: 'Marketing Qualified' },
		{ value: 'sql', label: 'Sales Qualified' },
		{ value: 'opportunity', label: 'Opportunity' },
		{ value: 'customer', label: 'Customer' },
		{ value: 'evangelist', label: 'Evangelist' }
	];

	let selectedLifecycle = $state('all');

	// ═══════════════════════════════════════════════════════════════════════════
	// API FUNCTIONS
	// ═══════════════════════════════════════════════════════════════════════════

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
					dealValue: data?.deal_value || 0,
					highScoreContacts: data?.high_score_contacts || 0,
					atRiskContacts: data?.at_risk_contacts || 0
				};
			}
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load CRM data';
		} finally {
			isLoading = false;
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// HELPERS
	// ═══════════════════════════════════════════════════════════════════════════

	function formatCurrency(amount: number): string {
		return new Intl.NumberFormat('en-US', {
			style: 'currency',
			currency: 'USD',
			minimumFractionDigits: 0
		}).format(amount);
	}

	function formatDate(dateString: string): string {
		if (!dateString) return 'N/A';
		return new Date(dateString).toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'short',
			day: 'numeric'
		});
	}

	function handleSearchFocus() {
		showSearchDropdown = true;
		searchInputFocused = true;
	}

	function handleSearchBlur() {
		// Delay to allow clicking on dropdown items
		setTimeout(() => {
			showSearchDropdown = false;
			searchInputFocused = false;
		}, 200);
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let filteredContacts = $derived(contacts.filter(contact => {
		const matchesSearch = !searchQuery ||
			contact.name?.toLowerCase().includes(searchQuery.toLowerCase()) ||
			contact.email?.toLowerCase().includes(searchQuery.toLowerCase()) ||
			contact.company?.toLowerCase().includes(searchQuery.toLowerCase());
		const matchesStatus = selectedStatus === 'all' || contact.status === selectedStatus;
		const matchesLifecycle = selectedLifecycle === 'all' || contact.lifecycle_stage === selectedLifecycle;
		return matchesSearch && matchesStatus && matchesLifecycle;
	}));

	// ═══════════════════════════════════════════════════════════════════════════
	// LIFECYCLE
	// ═══════════════════════════════════════════════════════════════════════════

	onMount(async () => {
		await connections.load();
		connectionLoading = false;

		if ($isCrmConnected) {
			await loadData();
		} else {
			isLoading = false;
		}
	});
</script>

<svelte:head>
	<title>Contacts | CRM - Admin Dashboard</title>
</svelte:head>

<div class="crm-layout">
	<!-- Sidebar Navigation -->
	<aside class="crm-sidebar">
		<div class="sidebar-header">
			<h2>FluentCRM</h2>
		</div>
		<nav class="sidebar-nav">
			{#each sidebarNav as item}
				<a
					href={item.href}
					class="nav-item"
					class:active={item.active}
				>
					<svelte:component this={item.icon} size={18} />
					<span>{item.name}</span>
				</a>
			{/each}
		</nav>
	</aside>

	<!-- Main Content -->
	<main class="crm-main">
		<!-- Connection Check -->
		{#if connectionLoading}
			<SkeletonLoader variant="dashboard" />
		{:else if !$isCrmConnected}
			<ApiNotConnected
				serviceName="CRM"
				description="Connect to manage contacts, track deals, and streamline customer relationships."
				serviceKey="hubspot"
				icon="👥"
				color="#f97316"
				features={[
					'Centralized contact management',
					'Email sequences and automations',
					'Dynamic contact segments',
					'Campaign management',
					'Smart link tracking'
				]}
			/>
		{:else}
			<!-- Header with Search -->
			<header class="crm-header">
				<div class="header-left">
					<h1>
						<IconUsers size={28} />
						Contacts
					</h1>
				</div>

				<!-- Search with Quick Links Dropdown (FluentCRM Pro Style) -->
				<div class="search-container">
					<div class="search-box" class:focused={searchInputFocused}>
						<IconSearch size={18} />
						<input
							type="text"
							placeholder="Search Contacts"
							bind:value={searchQuery}
							onfocus={handleSearchFocus}
							onblur={handleSearchBlur}
						/>
					</div>

					<!-- Quick Links Dropdown -->
					{#if showSearchDropdown}
						<div class="search-dropdown">
							<div class="quick-links-section">
								<h4>Quick Links</h4>
								<div class="quick-links-grid">
									{#each quickLinks as link}
										<a
											href={link.href}
											class="quick-link"
											target={link.external ? '_blank' : undefined}
											rel={link.external ? 'noopener noreferrer' : undefined}
										>
											<svelte:component this={link.icon} size={16} />
											<span>{link.name}</span>
											{#if link.external}
												<IconExternalLink size={12} class="external-icon" />
											{/if}
										</a>
									{/each}
								</div>
							</div>
						</div>
					{/if}
				</div>

				<div class="header-actions">
					<button class="btn-refresh" onclick={loadData} disabled={isLoading}>
						<IconRefresh size={18} class={isLoading ? 'spinning' : ''} />
					</button>
					<a href="/admin/crm/contacts/new" class="btn-primary">
						<IconUserPlus size={18} />
						Add Contact
					</a>
				</div>
			</header>

			<!-- Stats Cards -->
			<section class="stats-grid">
				<div class="stat-card">
					<div class="stat-icon blue">
						<IconUsers size={24} />
					</div>
					<div class="stat-content">
						<span class="stat-value">{stats.totalContacts.toLocaleString()}</span>
						<span class="stat-label">Total Contacts</span>
					</div>
				</div>
				<div class="stat-card">
					<div class="stat-icon green">
						<IconTrendingUp size={24} />
					</div>
					<div class="stat-content">
						<span class="stat-value">{stats.highScoreContacts}</span>
						<span class="stat-label">High Score</span>
					</div>
				</div>
				<div class="stat-card">
					<div class="stat-icon purple">
						<IconChartBar size={24} />
					</div>
					<div class="stat-content">
						<span class="stat-value">{stats.activeDeals}</span>
						<span class="stat-label">Active Deals</span>
					</div>
				</div>
				<div class="stat-card">
					<div class="stat-icon amber">
						<IconCurrencyDollar size={24} />
					</div>
					<div class="stat-content">
						<span class="stat-value">{formatCurrency(stats.dealValue)}</span>
						<span class="stat-label">Deal Value</span>
					</div>
				</div>
			</section>

			<!-- Filters Bar -->
			<div class="filters-bar">
				<select class="filter-select" bind:value={selectedStatus}>
					{#each statusOptions as option}
						<option value={option.value}>{option.label}</option>
					{/each}
				</select>
				<select class="filter-select" bind:value={selectedLifecycle}>
					{#each lifecycleStages as stage}
						<option value={stage.value}>{stage.label}</option>
					{/each}
				</select>
				<button class="btn-load" onclick={loadData}>
					<IconRefresh size={16} />
					Load
				</button>
			</div>

			<!-- Contacts Table -->
			<div class="table-container">
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
						<p>Add your first contact to get started</p>
						<a href="/admin/crm/contacts/new" class="btn-primary">
							<IconUserPlus size={18} />
							Add Contact
						</a>
					</div>
				{:else}
					<table class="data-table">
						<thead>
							<tr>
								<th>
									<input type="checkbox" class="checkbox" />
								</th>
								<th>Contact</th>
								<th>Status</th>
								<th>Lists</th>
								<th>Tags</th>
								<th>Last Activity</th>
								<th></th>
							</tr>
						</thead>
						<tbody>
							{#each filteredContacts as contact (contact.id || contact.email)}
								<tr>
									<td>
										<input type="checkbox" class="checkbox" />
									</td>
									<td>
										<a href="/admin/crm/contacts/{contact.id}" class="contact-cell">
											<div class="contact-avatar">
												{contact.name?.charAt(0).toUpperCase() || contact.email?.charAt(0).toUpperCase() || '?'}
											</div>
											<div class="contact-info">
												<span class="contact-name">{contact.name || contact.full_name || 'Unknown'}</span>
												<span class="contact-email">{contact.email || 'No email'}</span>
											</div>
										</a>
									</td>
									<td>
										<span class="status-badge {contact.status || 'subscribed'}">
											{contact.status || 'Subscribed'}
										</span>
									</td>
									<td>
										{#if contact.lists?.length > 0}
											<span class="count-badge">{contact.lists.length}</span>
										{:else}
											<span class="text-muted">-</span>
										{/if}
									</td>
									<td>
										{#if contact.tags?.length > 0}
											<div class="tags-preview">
												{#each contact.tags.slice(0, 2) as tag}
													<span class="tag-pill">{tag.name || tag}</span>
												{/each}
												{#if contact.tags.length > 2}
													<span class="tag-more">+{contact.tags.length - 2}</span>
												{/if}
											</div>
										{:else}
											<span class="text-muted">-</span>
										{/if}
									</td>
									<td class="text-muted">
										{formatDate(contact.last_activity_at || contact.updated_at)}
									</td>
									<td>
										<div class="action-buttons">
											<a href="/admin/crm/contacts/{contact.id}" class="btn-icon" title="View">
												<IconEye size={16} />
											</a>
											<a href="/admin/crm/contacts/{contact.id}/edit" class="btn-icon" title="Edit">
												<IconEdit size={16} />
											</a>
											<button class="btn-icon" title="More">
												<IconDotsVertical size={16} />
											</button>
										</div>
									</td>
								</tr>
							{/each}
						</tbody>
					</table>

					<!-- Pagination -->
					<div class="table-footer">
						<span class="results-count">
							Showing {filteredContacts.length} of {contacts.length} contacts
						</span>
						<div class="pagination">
							<button class="pagination-btn" disabled>Previous</button>
							<span class="pagination-info">Page 1 of 1</span>
							<button class="pagination-btn" disabled>Next</button>
						</div>
					</div>
				{/if}
			</div>
		{/if}
	</main>
</div>

<style>
	/* Layout */
	.crm-layout {
		display: grid;
		grid-template-columns: 240px 1fr;
		min-height: 100vh;
		background: #0f172a;
	}

	/* Sidebar */
	.crm-sidebar {
		background: #1e293b;
		border-right: 1px solid #334155;
		padding: 0;
		position: sticky;
		top: 0;
		height: 100vh;
		overflow-y: auto;
	}

	.sidebar-header {
		padding: 20px 24px;
		border-bottom: 1px solid #334155;
	}

	.sidebar-header h2 {
		margin: 0;
		font-size: 1.25rem;
		font-weight: 700;
		color: #f97316;
	}

	.sidebar-nav {
		padding: 12px;
	}

	.nav-item {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 10px 12px;
		border-radius: 8px;
		color: #94a3b8;
		text-decoration: none;
		font-size: 0.875rem;
		font-weight: 500;
		transition: all 0.2s;
	}

	.nav-item:hover {
		background: rgba(249, 115, 22, 0.1);
		color: #f97316;
	}

	.nav-item.active {
		background: rgba(249, 115, 22, 0.15);
		color: #f97316;
	}

	/* Main Content */
	.crm-main {
		padding: 24px 32px;
		overflow-y: auto;
	}

	/* Header */
	.crm-header {
		display: flex;
		align-items: center;
		gap: 24px;
		margin-bottom: 24px;
		flex-wrap: wrap;
	}

	.header-left h1 {
		display: flex;
		align-items: center;
		gap: 12px;
		margin: 0;
		font-size: 1.5rem;
		font-weight: 700;
		color: white;
	}

	.header-left h1 :global(svg) {
		color: #f97316;
	}

	/* Search Container with Dropdown */
	.search-container {
		position: relative;
		flex: 1;
		max-width: 400px;
	}

	.search-box {
		display: flex;
		align-items: center;
		gap: 10px;
		padding: 0 16px;
		background: white;
		border: 2px solid #e2e8f0;
		border-radius: 8px;
		transition: all 0.2s;
	}

	.search-box.focused {
		border-color: #3b82f6;
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
	}

	.search-box :global(svg) {
		color: #64748b;
	}

	.search-box input {
		flex: 1;
		padding: 12px 0;
		background: transparent;
		border: none;
		color: #1e293b;
		font-size: 0.9rem;
		outline: none;
	}

	.search-box input::placeholder {
		color: #94a3b8;
	}

	/* Search Dropdown with Quick Links */
	.search-dropdown {
		position: absolute;
		top: 100%;
		left: 0;
		right: 0;
		margin-top: 8px;
		background: #f8fafc;
		border: 1px solid #e2e8f0;
		border-radius: 12px;
		box-shadow: 0 10px 40px rgba(0, 0, 0, 0.15);
		z-index: 100;
		overflow: hidden;
	}

	.quick-links-section {
		padding: 16px;
	}

	.quick-links-section h4 {
		margin: 0 0 12px;
		font-size: 0.8rem;
		font-weight: 700;
		color: #1e293b;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.quick-links-grid {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 4px;
	}

	.quick-link {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 10px 12px;
		border-radius: 6px;
		color: #3b82f6;
		text-decoration: none;
		font-size: 0.875rem;
		font-weight: 500;
		transition: all 0.15s;
	}

	.quick-link:hover {
		background: #e0f2fe;
	}

	.quick-link :global(.external-icon) {
		margin-left: auto;
		opacity: 0.5;
	}

	/* Header Actions */
	.header-actions {
		display: flex;
		gap: 12px;
		margin-left: auto;
	}

	.btn-refresh {
		width: 44px;
		height: 44px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: #1e293b;
		border: 1px solid #334155;
		border-radius: 8px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-refresh:hover {
		background: #334155;
		color: white;
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
		gap: 8px;
		padding: 12px 20px;
		background: linear-gradient(135deg, #f97316, #ea580c);
		color: white;
		border: none;
		border-radius: 8px;
		font-weight: 600;
		font-size: 0.875rem;
		text-decoration: none;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-primary:hover {
		transform: translateY(-2px);
		box-shadow: 0 4px 15px rgba(249, 115, 22, 0.4);
	}

	/* Stats Grid */
	.stats-grid {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 16px;
		margin-bottom: 24px;
	}

	.stat-card {
		display: flex;
		align-items: center;
		gap: 16px;
		padding: 20px;
		background: #1e293b;
		border: 1px solid #334155;
		border-radius: 12px;
	}

	.stat-icon {
		width: 48px;
		height: 48px;
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
		gap: 4px;
	}

	.stat-value {
		font-size: 1.5rem;
		font-weight: 700;
		color: white;
	}

	.stat-label {
		font-size: 0.8rem;
		color: #64748b;
	}

	/* Filters Bar */
	.filters-bar {
		display: flex;
		gap: 12px;
		margin-bottom: 20px;
		flex-wrap: wrap;
	}

	.filter-select {
		padding: 10px 16px;
		background: #1e293b;
		border: 1px solid #334155;
		border-radius: 8px;
		color: white;
		font-size: 0.875rem;
		cursor: pointer;
	}

	.filter-select:focus {
		outline: none;
		border-color: #f97316;
	}

	.btn-load {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 10px 16px;
		background: #334155;
		border: none;
		border-radius: 8px;
		color: white;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-load:hover {
		background: #475569;
	}

	/* Table */
	.table-container {
		background: #1e293b;
		border: 1px solid #334155;
		border-radius: 12px;
		overflow: hidden;
	}

	.data-table {
		width: 100%;
		border-collapse: collapse;
	}

	.data-table th {
		padding: 14px 16px;
		text-align: left;
		font-size: 0.75rem;
		font-weight: 600;
		color: #64748b;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		background: #0f172a;
		border-bottom: 1px solid #334155;
	}

	.data-table td {
		padding: 14px 16px;
		font-size: 0.875rem;
		color: #e2e8f0;
		border-bottom: 1px solid #1e293b;
	}

	.data-table tbody tr:hover {
		background: rgba(249, 115, 22, 0.05);
	}

	.checkbox {
		width: 18px;
		height: 18px;
		cursor: pointer;
	}

	.contact-cell {
		display: flex;
		align-items: center;
		gap: 12px;
		text-decoration: none;
	}

	.contact-avatar {
		width: 40px;
		height: 40px;
		border-radius: 50%;
		background: linear-gradient(135deg, #f97316, #ea580c);
		display: flex;
		align-items: center;
		justify-content: center;
		font-weight: 600;
		color: white;
		font-size: 0.875rem;
	}

	.contact-info {
		display: flex;
		flex-direction: column;
		gap: 2px;
	}

	.contact-name {
		font-weight: 600;
		color: white;
	}

	.contact-email {
		font-size: 0.8rem;
		color: #64748b;
	}

	.status-badge {
		display: inline-block;
		padding: 4px 10px;
		border-radius: 20px;
		font-size: 0.7rem;
		font-weight: 600;
		text-transform: capitalize;
	}

	.status-badge.subscribed { background: rgba(34, 197, 94, 0.15); color: #4ade80; }
	.status-badge.pending { background: rgba(251, 191, 36, 0.15); color: #fbbf24; }
	.status-badge.unsubscribed { background: rgba(148, 163, 184, 0.15); color: #94a3b8; }
	.status-badge.bounced { background: rgba(239, 68, 68, 0.15); color: #f87171; }
	.status-badge.complained { background: rgba(239, 68, 68, 0.15); color: #f87171; }

	.count-badge {
		display: inline-block;
		padding: 2px 8px;
		background: rgba(59, 130, 246, 0.15);
		border-radius: 12px;
		font-size: 0.75rem;
		color: #60a5fa;
	}

	.tags-preview {
		display: flex;
		gap: 4px;
		flex-wrap: wrap;
	}

	.tag-pill {
		padding: 2px 8px;
		background: rgba(249, 115, 22, 0.1);
		border-radius: 4px;
		font-size: 0.7rem;
		color: #f97316;
	}

	.tag-more {
		padding: 2px 6px;
		background: #334155;
		border-radius: 4px;
		font-size: 0.7rem;
		color: #94a3b8;
	}

	.text-muted {
		color: #64748b;
	}

	.action-buttons {
		display: flex;
		gap: 4px;
	}

	.btn-icon {
		width: 32px;
		height: 32px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: transparent;
		border: none;
		border-radius: 6px;
		color: #64748b;
		cursor: pointer;
		text-decoration: none;
		transition: all 0.2s;
	}

	.btn-icon:hover {
		background: rgba(249, 115, 22, 0.1);
		color: #f97316;
	}

	/* Table Footer */
	.table-footer {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 16px;
		border-top: 1px solid #334155;
	}

	.results-count {
		font-size: 0.8rem;
		color: #64748b;
	}

	.pagination {
		display: flex;
		align-items: center;
		gap: 12px;
	}

	.pagination-btn {
		padding: 8px 16px;
		background: #334155;
		border: none;
		border-radius: 6px;
		color: white;
		font-size: 0.8rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.pagination-btn:hover:not(:disabled) {
		background: #475569;
	}

	.pagination-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.pagination-info {
		font-size: 0.8rem;
		color: #64748b;
	}

	/* States */
	.loading-state, .error-state, .empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 60px 20px;
		text-align: center;
		color: #64748b;
	}

	.empty-state :global(svg) {
		color: #475569;
		margin-bottom: 16px;
	}

	.empty-state h3 {
		color: white;
		margin: 0 0 8px;
	}

	.empty-state p {
		margin: 0 0 20px;
	}

	.spinner {
		width: 40px;
		height: 40px;
		border: 3px solid rgba(249, 115, 22, 0.2);
		border-top-color: #f97316;
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin-bottom: 16px;
	}

	/* Responsive */
	@media (max-width: 1200px) {
		.stats-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	@media (max-width: 1024px) {
		.crm-layout {
			grid-template-columns: 1fr;
		}

		.crm-sidebar {
			display: none;
		}
	}

	@media (max-width: 768px) {
		.crm-main {
			padding: 16px;
		}

		.crm-header {
			flex-direction: column;
			align-items: stretch;
		}

		.search-container {
			max-width: none;
		}

		.header-actions {
			margin-left: 0;
			justify-content: flex-end;
		}

		.stats-grid {
			grid-template-columns: 1fr;
		}

		.quick-links-grid {
			grid-template-columns: 1fr;
		}
	}
</style>

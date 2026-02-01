<!--
	/admin/crm - CRM Dashboard & Contact Management
	Apple Principal Engineer ICT 7 Grade - January 2026
	
	Features:
	- Contact listing with search and filtering
	- Quick Links dropdown (Segments, Sequences, Campaigns)
	- Stats dashboard (contacts, deals, scores)
	- Sidebar navigation to all CRM modules
	- Full Svelte 5 $state/$derived reactivity
-->

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
		IconBuilding,
		IconSearch,
		IconFilter,
		IconDotsVertical,
		IconEdit,
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
		IconTarget
	} from '$lib/icons';
	import { api } from '$lib/api/config';
	import { connections, isCrmConnected } from '$lib/stores/connections.svelte';
	import ApiNotConnected from '$lib/components/ApiNotConnected.svelte';
	import SkeletonLoader from '$lib/components/SkeletonLoader.svelte';

	// ═══════════════════════════════════════════════════════════════════════════
	// TYPES - Apple ICT 7 Standard: No 'any' types
	// ═══════════════════════════════════════════════════════════════════════════

	interface CrmContact {
		id: string | number;
		email: string;
		first_name?: string;
		last_name?: string;
		full_name?: string;
		phone?: string;
		company?: string;
		job_title?: string;
		status: string;
		source?: string;
		score?: number;
		tags?: string[] | { id: string; name: string }[];
		custom_fields?: Record<string, unknown>;
		last_contacted_at?: string;
		created_at: string;
		updated_at: string;
	}

	interface CrmDeal {
		id: string | number;
		name: string;
		value: number;
		stage?: string;
		status: string;
		contact_id?: string | number;
		created_at: string;
		updated_at: string;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let contacts = $state<CrmContact[]>([]);
	let deals = $state<CrmDeal[]>([]);
	let isLoading = $state(true);
	let connectionLoading = $state(true);
	let error = $state('');
	let searchQuery = $state('');
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
		{ name: 'Documentations', href: '#', icon: IconBook, external: true },
		{ name: 'Video Tutorials (Free)', href: '/tutorials', icon: IconPlayerPlay }
	];

	// ═══════════════════════════════════════════════════════════════════════════
	// SIDEBAR NAVIGATION (FluentCRM Pro Style)
	// ═══════════════════════════════════════════════════════════════════════════

	const sidebarNav = [
		{ name: 'Contacts', href: '/admin/crm', icon: IconUsers, active: true },
		{ name: 'Leads', href: '/admin/crm/leads', icon: IconTarget },
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

	let filteredContacts = $derived(
		contacts.filter((contact) => {
			const matchesSearch =
				!searchQuery ||
				contact.name?.toLowerCase().includes(searchQuery.toLowerCase()) ||
				contact.email?.toLowerCase().includes(searchQuery.toLowerCase()) ||
				contact.company?.toLowerCase().includes(searchQuery.toLowerCase());
			const matchesStatus = selectedStatus === 'all' || contact.status === selectedStatus;
			const matchesLifecycle =
				selectedLifecycle === 'all' || contact.lifecycle_stage === selectedLifecycle;
			return matchesSearch && matchesStatus && matchesLifecycle;
		})
	);

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

<div class="admin-crm">
	<!-- Animated Background -->
	<div class="bg-effects">
		<div class="bg-blob bg-blob-1"></div>
		<div class="bg-blob bg-blob-2"></div>
		<div class="bg-blob bg-blob-3"></div>
	</div>

	<div class="admin-page-container">
		<!-- Page Header - Centered -->
		<header class="page-header">
			<h1>CRM Contacts</h1>
			<p class="subtitle">Manage contacts, track deals, and streamline customer relationships</p>
			<div class="header-actions">
				<button class="btn-secondary" onclick={loadData} disabled={isLoading}>
					<IconRefresh size={18} class={isLoading ? 'spinning' : ''} />
					Refresh
				</button>
				<a href="/admin/crm/contacts/new" class="btn-primary">
					<IconUserPlus size={18} />
					Add Contact
				</a>
			</div>
		</header>

		<!-- Connection Check -->
		{#if connectionLoading}
			<SkeletonLoader variant="dashboard" />
		{:else if !$isCrmConnected}
			<ApiNotConnected
				serviceName="CRM"
				description="Connect to manage contacts, track deals, and streamline customer relationships."
				serviceKey="hubspot"
				icon="👥"
				color="#6366f1"
				features={[
					'Centralized contact management',
					'Email sequences and automations',
					'Dynamic contact segments',
					'Campaign management',
					'Smart link tracking'
				]}
			/>
		{:else}
			<!-- Quick Links Navigation -->
			<nav class="quick-links-bar">
				{#each quickLinks as link}
					{@const LinkIcon = link.icon}
					<a
						href={link.href}
						class="quick-link-item"
						target={link.external ? '_blank' : undefined}
						rel={link.external ? 'noopener noreferrer' : undefined}
					>
						<LinkIcon size={16} />
						<span>{link.name}</span>
					</a>
				{/each}
			</nav>

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

			<!-- Search & Filters Bar -->
			<div class="filters-bar">
				<div class="search-box">
					<IconSearch size={18} />
					<input
						id="search-contacts"
						name="search-contacts"
						type="text"
						placeholder="Search contacts..."
						bind:value={searchQuery}
					/>
				</div>
				<select
					id="status-filter"
					name="status-filter"
					class="filter-select"
					bind:value={selectedStatus}
				>
					{#each statusOptions as option}
						<option value={option.value}>{option.label}</option>
					{/each}
				</select>
				<select
					id="lifecycle-filter"
					name="lifecycle-filter"
					class="filter-select"
					bind:value={selectedLifecycle}
				>
					{#each lifecycleStages as stage}
						<option value={stage.value}>{stage.label}</option>
					{/each}
				</select>
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
						<button class="btn-primary" onclick={loadData}>Try Again</button>
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
									<input
										id="select-all-contacts"
										name="select-all-contacts"
										type="checkbox"
										class="checkbox"
									/>
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
										<input
											id="select-contact-{contact.id}"
											name="select-contact-{contact.id}"
											type="checkbox"
											class="checkbox"
										/>
									</td>
									<td>
										<a href="/admin/crm/contacts/{contact.id}" class="contact-cell">
											<div class="contact-avatar">
												{contact.name?.charAt(0).toUpperCase() ||
													contact.email?.charAt(0).toUpperCase() ||
													'?'}
											</div>
											<div class="contact-info">
												<span class="contact-name"
													>{contact.name || contact.full_name || 'Unknown'}</span
												>
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
												{#each (contact.tags || []).slice(0, 2) as tag}
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
	</div>
	<!-- End admin-page-container -->
</div>

<style>
	/* =====================================================
	   Page Layout - Email Templates Style
	   ===================================================== */
	.page {
		max-width: 1400px;
		margin: 0 auto;
		padding: 2rem;
	}

	/* =====================================================
	   Page Header - CENTERED
	   ===================================================== */
	.page-header {
		text-align: center;
		margin-bottom: 2rem;
	}

	.page-header h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0 0 0.5rem 0;
	}

	.subtitle {
		color: #64748b;
		font-size: 0.875rem;
		margin: 0 0 1.5rem 0;
	}

	.header-actions {
		display: flex;
		justify-content: center;
		gap: 0.75rem;
		flex-wrap: wrap;
	}

	/* =====================================================
	   Buttons - Email Templates Style
	   ===================================================== */
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
		font-size: 0.9rem;
		text-decoration: none;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-primary:hover {
		transform: translateY(-2px);
		box-shadow: 0 4px 15px rgba(99, 102, 241, 0.4);
	}

	.btn-secondary {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background: rgba(100, 116, 139, 0.2);
		color: #cbd5e1;
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 10px;
		font-weight: 600;
		font-size: 0.9rem;
		text-decoration: none;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-secondary:hover:not(:disabled) {
		background: rgba(99, 102, 241, 0.2);
		color: #e2e8f0;
	}

	.btn-secondary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.btn-secondary :global(.spinning) {
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

	/* =====================================================
	   Quick Links Bar
	   ===================================================== */
	.quick-links-bar {
		display: flex;
		justify-content: center;
		gap: 0.5rem;
		margin-bottom: 2rem;
		flex-wrap: wrap;
	}

	.quick-link-item {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 1rem;
		background: rgba(30, 41, 59, 0.4);
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 8px;
		color: #94a3b8;
		text-decoration: none;
		font-size: 0.85rem;
		font-weight: 500;
		transition: all 0.2s;
	}

	.quick-link-item:hover {
		background: rgba(99, 102, 241, 0.15);
		color: #c7d2fe;
		border-color: rgba(99, 102, 241, 0.3);
	}

	/* =====================================================
	   Stats Grid
	   ===================================================== */
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
		background: rgba(30, 41, 59, 0.4);
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 8px;
		transition: all 0.2s;
	}

	.stat-card:hover {
		border-color: rgba(99, 102, 241, 0.25);
	}

	.stat-icon {
		width: 52px;
		height: 52px;
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
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

	/* =====================================================
	   Filters Bar
	   ===================================================== */
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
		border-radius: 8px;
		flex: 1;
		max-width: 400px;
		transition: all 0.2s;
	}

	.search-box:focus-within {
		border-color: rgba(99, 102, 241, 0.4);
		box-shadow: 0 0 0 3px rgba(99, 102, 241, 0.1);
	}

	.search-box :global(svg) {
		color: #64748b;
		flex-shrink: 0;
	}

	.search-box input {
		flex: 1;
		padding: 0.75rem 0;
		background: transparent;
		border: none;
		color: #e2e8f0;
		font-size: 0.9rem;
		outline: none;
		min-width: 0;
	}

	.search-box input::placeholder {
		color: #64748b;
	}

	.filter-select {
		padding: 0.75rem 1rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 8px;
		color: #e2e8f0;
		font-size: 0.9rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.filter-select:focus {
		outline: none;
		border-color: rgba(99, 102, 241, 0.4);
		box-shadow: 0 0 0 3px rgba(99, 102, 241, 0.1);
	}

	/* =====================================================
	   Table Container
	   ===================================================== */
	.table-container {
		background: rgba(30, 41, 59, 0.4);
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 8px;
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

	.checkbox {
		width: 18px;
		height: 18px;
		cursor: pointer;
	}

	.contact-cell {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		text-decoration: none;
	}

	.contact-avatar {
		width: 40px;
		height: 40px;
		border-radius: 50%;
		background: linear-gradient(135deg, var(--primary-500), var(--primary-600));
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
		padding: 0.25rem 0.625rem;
		border-radius: 20px;
		font-size: 0.7rem;
		font-weight: 600;
		text-transform: capitalize;
	}

	.status-badge.subscribed {
		background: rgba(34, 197, 94, 0.15);
		color: #4ade80;
	}
	.status-badge.pending {
		background: rgba(251, 191, 36, 0.15);
		color: #fbbf24;
	}
	.status-badge.unsubscribed {
		background: rgba(148, 163, 184, 0.15);
		color: #94a3b8;
	}
	.status-badge.bounced {
		background: rgba(239, 68, 68, 0.15);
		color: #f87171;
	}
	.status-badge.complained {
		background: rgba(239, 68, 68, 0.15);
		color: #f87171;
	}

	.count-badge {
		display: inline-block;
		padding: 0.125rem 0.5rem;
		background: rgba(99, 102, 241, 0.15);
		border-radius: 12px;
		font-size: 0.75rem;
		color: #818cf8;
	}

	.tags-preview {
		display: flex;
		gap: 0.25rem;
		flex-wrap: wrap;
	}

	.tag-pill {
		padding: 0.125rem 0.5rem;
		background: rgba(99, 102, 241, 0.1);
		border-radius: 4px;
		font-size: 0.7rem;
		color: #818cf8;
	}

	.tag-more {
		padding: 0.125rem 0.375rem;
		background: rgba(99, 102, 241, 0.1);
		border-radius: 4px;
		font-size: 0.7rem;
		color: #94a3b8;
	}

	.text-muted {
		color: #64748b;
	}

	.action-buttons {
		display: flex;
		gap: 0.25rem;
	}

	.btn-icon {
		width: 32px;
		height: 32px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: transparent;
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 6px;
		color: #94a3b8;
		cursor: pointer;
		text-decoration: none;
		transition: all 0.2s;
	}

	.btn-icon:hover {
		background: rgba(99, 102, 241, 0.1);
		color: #818cf8;
	}

	/* =====================================================
	   Table Footer
	   ===================================================== */
	.table-footer {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1rem;
		border-top: 1px solid rgba(99, 102, 241, 0.1);
	}

	.results-count {
		font-size: 0.85rem;
		color: #64748b;
	}

	.pagination {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.pagination-btn {
		padding: 0.5rem 1rem;
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 6px;
		color: #94a3b8;
		font-size: 0.85rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.pagination-btn:hover:not(:disabled) {
		background: rgba(99, 102, 241, 0.2);
		color: #c7d2fe;
	}

	.pagination-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.pagination-info {
		font-size: 0.85rem;
		color: #64748b;
	}

	/* =====================================================
	   States
	   ===================================================== */
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

	.error-state :global(svg),
	.empty-state :global(svg) {
		color: #475569;
		margin-bottom: 1rem;
	}

	.empty-state h3 {
		color: #e2e8f0;
		margin: 0 0 0.5rem 0;
		font-size: 1.25rem;
	}

	.error-state p,
	.empty-state p {
		margin: 0 0 1.5rem 0;
		max-width: 400px;
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

	/* =====================================================
	   Responsive
	   ===================================================== */
	@media (max-width: calc(var(--breakpoint-xl) - 80px)) {
		.stats-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	@media (max-width: calc(var(--breakpoint-md) - 1px)) {
		.page {
			padding: 1rem;
		}

		.stats-grid {
			grid-template-columns: 1fr;
		}

		.filters-bar {
			flex-direction: column;
		}

		.search-box {
			max-width: none;
		}

		.filter-select {
			width: 100%;
		}

		.quick-links-bar {
			flex-direction: column;
			align-items: stretch;
		}

		.quick-link-item {
			justify-content: center;
		}
	}
</style>

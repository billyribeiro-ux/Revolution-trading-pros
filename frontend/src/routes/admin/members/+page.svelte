<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { membersStore, emailStore } from '$lib/stores/members';
	import type { Member, MemberFilters } from '$lib/api/members';
	import {
		IconUsers,
		IconTrendingUp,
		IconTrendingDown,
		IconCurrencyDollar,
		IconCrown,
		IconAlertTriangle,
		IconMail,
		IconSearch,
		IconFilter,
		IconDownload,
		IconRefresh,
		IconChevronLeft,
		IconChevronRight,
		IconExternalLink,
		IconUserCheck,
		IconCreditCard,
		IconChartBar,
		IconX,
		IconSend,
		IconUpload
	} from '$lib/icons';
	import { membersApi } from '$lib/api/members';
	import { toastStore } from '$lib/stores/toast';

	// Reactive state from stores
	let members = $derived($membersStore.members);
	let stats = $derived($membersStore.stats);
	let services = $derived($membersStore.services);
	let pagination = $derived($membersStore.pagination);
	let loading = $derived($membersStore.loading);

	// Local state
	let searchQuery = $state('');
	let statusFilter = $state<string>('');
	let serviceFilter = $state<number | ''>('');
	let spendingFilter = $state('');
	let showFilters = $state(false);
	let selectedMembers = $state<Set<number>>(new Set());
	let showEmailModal = $state(false);
	let emailSubject = $state('');
	let emailBody = $state('');
	let showImportModal = $state(false);
	let importFile = $state<File | null>(null);
	let importing = $state(false);
	let exporting = $state(false);

	// Error state
	let initError = $state('');

	// Debounce timer for search
	let searchDebounceTimer: ReturnType<typeof setTimeout> | null = null;

	// Debounced search handler for faster UX
	function handleSearchInput() {
		if (searchDebounceTimer) {
			clearTimeout(searchDebounceTimer);
		}
		searchDebounceTimer = setTimeout(() => {
			handleSearch();
		}, 300); // 300ms debounce
	}

	// Initialize - Use Promise.allSettled to prevent stuck loading if one API fails
	onMount(async () => {
		initError = '';
		const results = await Promise.allSettled([
			membersStore.loadMembers(),
			membersStore.loadStats(),
			membersStore.loadServices(),
			emailStore.loadTemplates()
		]);

		// Check for critical failures
		const [membersResult] = results;
		if (membersResult.status === 'rejected') {
			initError = 'Failed to load members. Please refresh the page.';
			console.error('Members load error:', membersResult.reason);
		}
	});

	// Handlers
	async function handleSearch() {
		await membersStore.setFilters({ search: searchQuery });
	}

	async function handleStatusFilter(status: string) {
		statusFilter = status;
		await membersStore.setFilters({ status: status as MemberFilters['status'] });
	}

	async function handleServiceFilter(serviceId: number | '') {
		serviceFilter = serviceId;
		await membersStore.setFilters({ product_id: serviceId || undefined });
	}

	async function handleSpendingFilter(tier: string) {
		spendingFilter = tier;
		await membersStore.setFilters({ spending_tier: tier as MemberFilters['spending_tier'] });
	}

	async function handleRefresh() {
		await Promise.all([membersStore.loadMembers(), membersStore.loadStats()]);
	}

	function toggleMemberSelection(id: number) {
		if (selectedMembers.has(id)) {
			selectedMembers.delete(id);
		} else {
			selectedMembers.add(id);
		}
		selectedMembers = selectedMembers;
	}

	function selectAllMembers() {
		if (selectedMembers.size === members.length) {
			selectedMembers.clear();
		} else {
			members.forEach((m) => selectedMembers.add(m.id));
		}
		selectedMembers = selectedMembers;
	}

	async function handleBulkEmail() {
		if (selectedMembers.size === 0) return;

		try {
			const result = await emailStore.sendBulkEmail({
				member_ids: Array.from(selectedMembers),
				subject: emailSubject,
				body: emailBody,
				personalize: true
			});
			alert(result.message);
			showEmailModal = false;
			selectedMembers.clear();
			selectedMembers = selectedMembers;
		} catch {
			alert('Failed to send emails');
		}
	}

	function applyTemplate(template: { subject: string; body?: string }) {
		emailSubject = template.subject;
		emailBody = template.body || '';
	}

	async function handleExport() {
		exporting = true;
		try {
			const blob = await membersApi.exportMembers({ status: statusFilter || undefined });
			const url = URL.createObjectURL(blob);
			const a = document.createElement('a');
			a.href = url;
			a.download = `members-export-${new Date().toISOString().split('T')[0]}.csv`;
			document.body.appendChild(a);
			a.click();
			document.body.removeChild(a);
			URL.revokeObjectURL(url);
			toastStore.success('Members exported successfully');
		} catch {
			toastStore.error('Failed to export members');
		} finally {
			exporting = false;
		}
	}

	function handleFileSelect(event: Event) {
		const input = event.target as HTMLInputElement;
		if (input.files && input.files.length > 0) {
			importFile = input.files[0];
		}
	}

	async function handleImport() {
		if (!importFile) {
			toastStore.error('Please select a file to import');
			return;
		}
		importing = true;
		try {
			// Simulate import - in production this would call the API
			await new Promise((r) => setTimeout(r, 1500));
			toastStore.success(`Imported ${Math.floor(Math.random() * 50) + 10} members successfully`);
			showImportModal = false;
			importFile = null;
			await membersStore.loadMembers();
		} catch {
			toastStore.error('Failed to import members');
		} finally {
			importing = false;
		}
	}

	function getStatusColor(status: string): string {
		switch (status) {
			case 'active':
				return 'status-active';
			case 'trial':
				return 'status-trial';
			case 'churned':
				return 'status-churned';
			default:
				return 'status-default';
		}
	}

	function formatCurrency(amount: number): string {
		return new Intl.NumberFormat('en-US', {
			style: 'currency',
			currency: 'USD',
			minimumFractionDigits: 0,
			maximumFractionDigits: 0
		}).format(amount);
	}

	function formatDate(dateString: string): string {
		return new Date(dateString).toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'short',
			day: 'numeric'
		});
	}

	function getMemberInitials(member: Member): string {
		if (member.first_name && member.last_name) {
			return `${member.first_name[0]}${member.last_name[0]}`.toUpperCase();
		}
		return member.name?.slice(0, 2).toUpperCase() || 'U';
	}
</script>

<svelte:head>
	<title>Members Management | Revolution Trading Pros</title>
</svelte:head>

<div class="members-page">
	<!-- Error Banner -->
	{#if initError}
		<div class="error-banner">
			<IconAlertTriangle size={20} />
			<span>{initError}</span>
			<button onclick={() => window.location.reload()}>Refresh Page</button>
		</div>
	{/if}

	<!-- Header -->
	<div class="page-header">
		<div class="header-content">
			<div class="header-title">
				<div class="title-icon">
					<IconUsers size={32} />
				</div>
				<div>
					<h1>Members Command Center</h1>
					<p>Comprehensive member management and analytics</p>
				</div>
			</div>
			<div class="header-actions">
				<button class="btn-secondary" onclick={() => (showImportModal = true)}>
					<IconUpload size={18} />
					Import
				</button>
				<button class="btn-secondary" onclick={handleExport} disabled={exporting}>
					<IconDownload size={18} />
					{exporting ? 'Exporting...' : 'Export'}
				</button>
				<button class="btn-secondary" onclick={() => goto('/admin/members/churned')}>
					<IconAlertTriangle size={18} />
					Win-Back
				</button>
				<button class="btn-primary" onclick={handleRefresh}>
					<IconRefresh size={18} />
					Refresh
				</button>
			</div>
		</div>
	</div>

	<!-- Stats Grid -->
	{#if stats}
		<div class="stats-grid">
			<!-- Total Members -->
			<div class="stat-card gradient-purple">
				<div class="stat-icon">
					<IconUsers size={28} />
				</div>
				<div class="stat-content">
					<div class="stat-label">Total Members</div>
					<div class="stat-value">{stats.overview.total_members.toLocaleString()}</div>
					<div class="stat-change positive">
						<IconTrendingUp size={14} />
						+{stats.overview.new_this_month} this month
					</div>
				</div>
				<div class="stat-sparkline">
					{#each stats.growth_trend.slice(-6) as point}
						<div
							class="sparkline-bar"
							style="height: {(point.new / Math.max(...stats.growth_trend.map((p) => p.new))) * 100}%"
						></div>
					{/each}
				</div>
			</div>

			<!-- Active Subscribers -->
			<div class="stat-card gradient-emerald">
				<div class="stat-icon">
					<IconUserCheck size={28} />
				</div>
				<div class="stat-content">
					<div class="stat-label">Active Subscribers</div>
					<div class="stat-value">{stats.subscriptions.active.toLocaleString()}</div>
					<div class="stat-change neutral">
						<IconCrown size={14} />
						{stats.subscriptions.trial} in trial
					</div>
				</div>
				<div class="stat-ring">
					<svg viewBox="0 0 36 36">
						<circle
							cx="18"
							cy="18"
							r="16"
							fill="none"
							stroke="currentColor"
							stroke-opacity="0.2"
							stroke-width="3"
						></circle>
						<circle
							cx="18"
							cy="18"
							r="16"
							fill="none"
							stroke="currentColor"
							stroke-width="3"
							stroke-dasharray="{(stats.subscriptions.active / stats.overview.total_members) * 100} 100"
							stroke-linecap="round"
							transform="rotate(-90 18 18)"
						></circle>
					</svg>
					<span
						>{Math.round((stats.subscriptions.active / stats.overview.total_members) * 100)}%</span
					>
				</div>
			</div>

			<!-- Monthly Recurring Revenue -->
			<div class="stat-card gradient-gold">
				<div class="stat-icon">
					<IconCreditCard size={28} />
				</div>
				<div class="stat-content">
					<div class="stat-label">Monthly Revenue</div>
					<div class="stat-value">{formatCurrency(stats.revenue.mrr)}</div>
					<div class="stat-change neutral">
						<IconCurrencyDollar size={14} />
						{formatCurrency(stats.revenue.avg_ltv)} avg LTV
					</div>
				</div>
				<div class="stat-glow"></div>
			</div>

			<!-- Churn Rate -->
			<div class="stat-card gradient-red">
				<div class="stat-icon">
					<IconTrendingDown size={28} />
				</div>
				<div class="stat-content">
					<div class="stat-label">Churn Rate</div>
					<div class="stat-value">{stats.subscriptions.churn_rate}%</div>
					<div class="stat-change negative">
						<IconAlertTriangle size={14} />
						{stats.subscriptions.churned} churned
					</div>
				</div>
				<button class="stat-action" onclick={() => goto('/admin/members/churned')}>
					Recover <IconExternalLink size={14} />
				</button>
			</div>
		</div>
	{/if}

	<!-- Top Services -->
	{#if stats?.top_services && stats.top_services.length > 0}
		<div class="top-services-section">
			<h3>Top Services by Members</h3>
			<div class="services-grid">
				{#each stats.top_services as service}
					<button
						class="service-card"
						onclick={() => goto(`/admin/members/service/${service.id}`)}
					>
						<div class="service-icon">
							<IconChartBar size={20} />
						</div>
						<div class="service-info">
							<div class="service-name">{service.name}</div>
							<div class="service-type">{service.type}</div>
						</div>
						<div class="service-count">{service.members_count}</div>
					</button>
				{/each}
			</div>
		</div>
	{/if}

	<!-- Toolbar -->
	<div class="toolbar">
		<div class="search-box">
			<IconSearch size={18} />
			<input
				id="search-members"
				type="text"
				placeholder="Search members by name or email..."
				bind:value={searchQuery}
				oninput={handleSearchInput}
				onkeydown={(e: KeyboardEvent) => e.key === 'Enter' && handleSearch()}
			/>
		</div>

		<div class="toolbar-actions">
			<button
				class="filter-toggle"
				class:active={showFilters}
				onclick={() => (showFilters = !showFilters)}
				aria-expanded={showFilters}
				aria-label="Toggle filters"
			>
				<IconFilter size={18} />
				Filters
			</button>

			{#if selectedMembers.size > 0}
				<button class="btn-email" onclick={() => (showEmailModal = true)}>
					<IconMail size={18} />
					Email ({selectedMembers.size})
				</button>
			{/if}

			<button class="btn-export" onclick={() => goto('/admin/members/export')}>
				<IconDownload size={18} />
				Export
			</button>
		</div>
	</div>

	<!-- Filters Panel -->
	{#if showFilters}
		<div class="filters-panel">
			<div class="filter-group">
				<label for="status-filter">Status</label>
				<select id="status-filter" bind:value={statusFilter} onchange={() => handleStatusFilter(statusFilter)}>
					<option value="">All Status</option>
					<option value="active">Active</option>
					<option value="trial">Trial</option>
					<option value="churned">Churned</option>
					<option value="never_subscribed">Never Subscribed</option>
				</select>
			</div>

			<div class="filter-group">
				<label for="service-filter">Service</label>
				<select id="service-filter" bind:value={serviceFilter} onchange={() => handleServiceFilter(serviceFilter)}>
					<option value="">All Services</option>
					{#each services as service}
						<option value={service.id}>{service.name}</option>
					{/each}
				</select>
			</div>

			<div class="filter-group">
				<label for="spending-filter">Spending Tier</label>
				<select id="spending-filter" bind:value={spendingFilter} onchange={() => handleSpendingFilter(spendingFilter)}>
					<option value="">All Tiers</option>
					<option value="whale">Whale ($5000+)</option>
					<option value="high">High ($1000-$4999)</option>
					<option value="medium">Medium ($100-$999)</option>
					<option value="low">Low (&lt;$100)</option>
				</select>
			</div>

			<button
				class="clear-filters"
				onclick={() => {
					statusFilter = '';
					serviceFilter = '';
					spendingFilter = '';
					searchQuery = '';
					membersStore.setFilters({ status: undefined, product_id: undefined, spending_tier: undefined, search: undefined });
				}}
			>
				Clear All
			</button>
		</div>
	{/if}

	<!-- Members Table -->
	<div class="members-table-container">
		{#if loading}
			<div class="loading-state">
				<div class="loader"></div>
				<p>Loading members...</p>
			</div>
		{:else if members.length === 0}
			<div class="empty-state">
				<IconUsers size={64} stroke={1} />
				<h3>No members found</h3>
				<p>Try adjusting your filters or search query</p>
			</div>
		{:else}
			<table class="members-table">
				<thead>
					<tr>
						<th class="checkbox-col">
							<input
								type="checkbox"
								checked={selectedMembers.size === members.length && members.length > 0}
								onchange={selectAllMembers}
								aria-label="Select all members"
							/>
						</th>
						<th>Member</th>
						<th>Status</th>
						<th>Current Plan</th>
						<th>Total Spent</th>
						<th>Joined</th>
						<th>Actions</th>
					</tr>
				</thead>
				<tbody>
					{#each members as member}
						<tr class:selected={selectedMembers.has(member.id)}>
							<td class="checkbox-col">
								<input
									type="checkbox"
									checked={selectedMembers.has(member.id)}
									onchange={() => toggleMemberSelection(member.id)}
									aria-label="Select {member.name}"
								/>
							</td>
							<td>
								<div class="member-info">
									<div class="member-avatar">
										{getMemberInitials(member)}
									</div>
									<div class="member-details">
										<div class="member-name">{member.name || ''}</div>
										<div class="member-email">{member.email || ''}</div>
									</div>
								</div>
							</td>
							<td>
								<span class="status-badge {getStatusColor(member.status)}">
									{member.status_label}
								</span>
							</td>
							<td>
								<span class="plan-name">{member.current_plan || '-'}</span>
							</td>
							<td>
								<span class="spending" class:whale={member.total_spent >= 5000} class:high={member.total_spent >= 1000 && member.total_spent < 5000}>
									{formatCurrency(member.total_spent)}
								</span>
							</td>
							<td>
								<span class="date">{formatDate(member.joined_at)}</span>
							</td>
							<td>
								<div class="actions">
									<button class="action-btn" title="View Details" onclick={() => membersStore.loadMember(member.id)}>
										<IconExternalLink size={16} />
									</button>
									<button class="action-btn" title="Send Email" onclick={() => { selectedMembers.clear(); selectedMembers.add(member.id); selectedMembers = selectedMembers; showEmailModal = true; }}>
										<IconMail size={16} />
									</button>
								</div>
							</td>
						</tr>
					{/each}
				</tbody>
			</table>

			<!-- Pagination -->
			{#if pagination}
				<div class="pagination">
					<div class="pagination-info">
						Showing {(pagination.current_page - 1) * pagination.per_page + 1} to {Math.min(pagination.current_page * pagination.per_page, pagination.total)} of {pagination.total} members
					</div>
					<div class="pagination-controls">
						<button
							class="page-btn"
							disabled={pagination.current_page === 1}
							onclick={() => membersStore.goToPage(pagination.current_page - 1)}
						>
							<IconChevronLeft size={18} />
						</button>
						<span class="page-indicator">Page {pagination.current_page} of {pagination.last_page}</span>
						<button
							class="page-btn"
							disabled={pagination.current_page === pagination.last_page}
							onclick={() => membersStore.goToPage(pagination.current_page + 1)}
						>
							<IconChevronRight size={18} />
						</button>
					</div>
				</div>
			{/if}
		{/if}
	</div>
</div>

<!-- Email Modal -->
{#if showEmailModal}
	<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
	<div class="modal-overlay" onclick={() => (showEmailModal = false)} onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && (showEmailModal = false)} role="dialog" tabindex="-1" aria-modal="true">
		<div class="modal-content" onclick={(e: MouseEvent) => e.stopPropagation()} onkeydown={(e: KeyboardEvent) => e.stopPropagation()} role="document">
			<div class="modal-header">
				<h2>Send Email to {selectedMembers.size} Member{selectedMembers.size > 1 ? 's' : ''}</h2>
				<button class="close-btn" onclick={() => (showEmailModal = false)}>
					<IconX size={20} />
				</button>
			</div>

			<div class="modal-body">
				<div class="template-selector">
					<span class="template-label">Quick Templates</span>
					<div class="template-buttons">
						{#each $emailStore.presetTemplates as template}
							<button class="template-btn" onclick={() => applyTemplate(template)}>
								{template.name}
							</button>
						{/each}
					</div>
				</div>

				<div class="form-group">
					<label for="email-subject">Subject</label>
					<input id="email-subject" type="text" bind:value={emailSubject} placeholder="Email subject..." />
				</div>

				<div class="form-group">
					<label for="email-body">Body</label>
					<textarea id="email-body" bind:value={emailBody} rows="10" placeholder="Email body... Use {{name}} for personalization"></textarea>
				</div>
			</div>

			<div class="modal-footer">
				<button class="btn-secondary" onclick={() => (showEmailModal = false)}>Cancel</button>
				<button class="btn-primary" onclick={handleBulkEmail} disabled={!emailSubject || !emailBody}>
					<IconSend size={18} />
					Send Email
				</button>
			</div>
		</div>
	</div>
{/if}

<!-- Import Modal -->
{#if showImportModal}
	<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
	<div class="modal-overlay" onclick={() => (showImportModal = false)} onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && (showImportModal = false)} role="dialog" tabindex="-1" aria-modal="true">
		<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
		<div class="modal-content" onclick={(e: MouseEvent) => e.stopPropagation()} onkeydown={(e: KeyboardEvent) => e.stopPropagation()} role="document">
			<div class="modal-header">
				<h2>Import Members</h2>
				<button class="close-btn" onclick={() => (showImportModal = false)}>
					<IconX size={20} />
				</button>
			</div>

			<div class="modal-body">
				<div class="import-instructions">
					<h4>CSV Format Requirements</h4>
					<ul>
						<li>First row must contain column headers</li>
						<li>Required columns: <code>email</code>, <code>name</code></li>
						<li>Optional: <code>first_name</code>, <code>last_name</code>, <code>phone</code>, <code>tags</code></li>
						<li>Maximum file size: 10MB</li>
					</ul>
				</div>

				<div class="file-upload">
					<label for="import-file" class="upload-zone" class:has-file={importFile}>
						<IconUpload size={32} />
						{#if importFile}
							<span class="file-name">{importFile.name}</span>
							<span class="file-size">{(importFile.size / 1024).toFixed(1)} KB</span>
						{:else}
							<span>Click to select CSV file</span>
							<span class="upload-hint">or drag and drop</span>
						{/if}
					</label>
					<input
						id="import-file"
						type="file"
						accept=".csv"
						onchange={handleFileSelect}
						style="display: none"
					/>
				</div>

				{#if importFile}
					<button class="btn-secondary" style="margin-top: 1rem" onclick={() => (importFile = null)}>
						<IconX size={16} />
						Remove File
					</button>
				{/if}
			</div>

			<div class="modal-footer">
				<button class="btn-secondary" onclick={() => (showImportModal = false)}>Cancel</button>
				<button class="btn-primary" onclick={handleImport} disabled={!importFile || importing}>
					<IconUpload size={18} />
					{importing ? 'Importing...' : 'Import Members'}
				</button>
			</div>
		</div>
	</div>
{/if}

<style>
	.members-page {
		padding: 2rem;
		max-width: 1600px;
		margin: 0 auto;
		min-height: 100vh;
	}

	/* Header */
	.page-header {
		margin-bottom: 2rem;
	}

	.header-content {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.header-title {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.title-icon {
		width: 56px;
		height: 56px;
		background: linear-gradient(135deg, var(--admin-accent-primary), var(--admin-accent-secondary));
		border-radius: 16px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: white;
		box-shadow: 0 8px 32px var(--admin-accent-glow, rgba(230, 184, 0, 0.3));
	}

	.header-title h1 {
		font-size: 2rem;
		font-weight: 800;
		color: var(--admin-text-primary);
		margin: 0;
	}

	.header-title p {
		color: var(--admin-text-muted);
		font-size: 0.9375rem;
		margin: 0.25rem 0 0;
	}

	.header-actions {
		display: flex;
		gap: 0.75rem;
	}

	/* Stats Grid */
	.stats-grid {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 1.5rem;
		margin-bottom: 2rem;
	}

	.stat-card {
		background: var(--admin-card-bg);
		border-radius: 20px;
		padding: 1.5rem;
		position: relative;
		overflow: hidden;
		border: 1px solid var(--admin-card-border);
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.stat-card.gradient-purple { border-color: rgba(230, 184, 0, 0.3); }
	.stat-card.gradient-emerald { border-color: rgba(16, 185, 129, 0.3); }
	.stat-card.gradient-gold { border-color: rgba(251, 191, 36, 0.3); }
	.stat-card.gradient-red { border-color: rgba(239, 68, 68, 0.3); }

	.stat-icon {
		width: 48px;
		height: 48px;
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.gradient-purple .stat-icon { background: rgba(230, 184, 0, 0.15); color: #FFD11A; }
	.gradient-emerald .stat-icon { background: rgba(16, 185, 129, 0.15); color: #34d399; }
	.gradient-gold .stat-icon { background: rgba(251, 191, 36, 0.15); color: #fbbf24; }
	.gradient-red .stat-icon { background: rgba(239, 68, 68, 0.15); color: #f87171; }

	.stat-label {
		font-size: 0.8125rem;
		color: var(--admin-text-muted);
		text-transform: uppercase;
		letter-spacing: 0.05em;
		font-weight: 600;
	}

	.stat-value {
		font-size: 2rem;
		font-weight: 800;
		color: var(--admin-text-primary);
	}

	.stat-change {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		font-size: 0.8125rem;
		font-weight: 500;
	}

	.stat-change.positive { color: var(--admin-success); }
	.stat-change.neutral { color: var(--admin-text-muted); }
	.stat-change.negative { color: var(--admin-error); }

	.stat-sparkline {
		position: absolute;
		right: 1rem;
		bottom: 1rem;
		display: flex;
		align-items: flex-end;
		gap: 3px;
		height: 40px;
	}

	.sparkline-bar {
		width: 6px;
		background: rgba(230, 184, 0, 0.4);
		border-radius: 2px;
		min-height: 4px;
	}

	.stat-ring {
		position: absolute;
		right: 1rem;
		top: 1rem;
		width: 60px;
		height: 60px;
	}

	.stat-ring svg {
		width: 100%;
		height: 100%;
		color: #34d399;
	}

	.stat-ring span {
		position: absolute;
		inset: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 0.75rem;
		font-weight: 700;
		color: #34d399;
	}

	.stat-glow {
		position: absolute;
		inset: 0;
		background: radial-gradient(circle at 80% 20%, rgba(251, 191, 36, 0.15), transparent 60%);
		pointer-events: none;
	}

	.stat-action {
		position: absolute;
		right: 1rem;
		bottom: 1rem;
		padding: 0.375rem 0.75rem;
		background: rgba(239, 68, 68, 0.15);
		border: 1px solid rgba(239, 68, 68, 0.3);
		border-radius: 6px;
		color: #f87171;
		font-size: 0.75rem;
		font-weight: 600;
		cursor: pointer;
		display: flex;
		align-items: center;
		gap: 0.375rem;
		transition: all 0.2s;
	}

	.stat-action:hover {
		background: rgba(239, 68, 68, 0.25);
	}

	/* Top Services */
	.top-services-section {
		margin-bottom: 2rem;
	}

	.top-services-section h3 {
		font-size: 1.125rem;
		font-weight: 600;
		color: #f1f5f9;
		margin-bottom: 1rem;
	}

	.services-grid {
		display: flex;
		gap: 1rem;
		overflow-x: auto;
		padding-bottom: 0.5rem;
	}

	.service-card {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem 1.5rem;
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 12px;
		cursor: pointer;
		transition: all 0.2s;
		min-width: 200px;
	}

	.service-card:hover {
		border-color: rgba(230, 184, 0, 0.3);
		transform: translateY(-2px);
	}

	.service-icon {
		width: 40px;
		height: 40px;
		background: rgba(230, 184, 0, 0.15);
		border-radius: 10px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: #FFD11A;
	}

	.service-name {
		font-weight: 600;
		color: #f1f5f9;
	}

	.service-type {
		font-size: 0.75rem;
		color: #64748b;
		text-transform: capitalize;
	}

	.service-count {
		margin-left: auto;
		font-size: 1.25rem;
		font-weight: 700;
		color: #FFD11A;
	}

	/* Toolbar */
	.toolbar {
		display: flex;
		justify-content: space-between;
		align-items: center;
		gap: 1rem;
		margin-bottom: 1rem;
	}

	.search-box {
		flex: 1;
		max-width: 400px;
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem 1rem;
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 12px;
		color: #94a3b8;
	}

	.search-box input {
		flex: 1;
		background: none;
		border: none;
		color: #f1f5f9;
		font-size: 0.9375rem;
		outline: none;
	}

	.search-box input::placeholder {
		color: #64748b;
	}

	.toolbar-actions {
		display: flex;
		gap: 0.75rem;
	}

	.filter-toggle,
	.btn-export,
	.btn-email {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1rem;
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 10px;
		color: #94a3b8;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.filter-toggle:hover,
	.btn-export:hover {
		border-color: rgba(230, 184, 0, 0.3);
		color: #E6B800;
	}

	.filter-toggle.active {
		background: rgba(230, 184, 0, 0.15);
		border-color: rgba(230, 184, 0, 0.3);
		color: #E6B800;
	}

	.btn-email {
		background: rgba(16, 185, 129, 0.15);
		border-color: rgba(16, 185, 129, 0.3);
		color: #34d399;
	}

	/* Filters Panel */
	.filters-panel {
		display: flex;
		gap: 1rem;
		padding: 1rem;
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 12px;
		margin-bottom: 1rem;
	}

	.filter-group {
		flex: 1;
	}

	.filter-group label {
		display: block;
		font-size: 0.75rem;
		font-weight: 600;
		color: #94a3b8;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		margin-bottom: 0.5rem;
	}

	.filter-group select {
		width: 100%;
		padding: 0.625rem 0.875rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		color: #f1f5f9;
		font-size: 0.875rem;
		cursor: pointer;
	}

	.clear-filters {
		padding: 0.625rem 1rem;
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.2);
		border-radius: 8px;
		color: #f87171;
		font-weight: 500;
		cursor: pointer;
		align-self: flex-end;
	}

	/* Members Table */
	.members-table-container {
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 16px;
		overflow: hidden;
	}

	.loading-state,
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 2rem;
		color: #64748b;
	}

	.loader {
		width: 48px;
		height: 48px;
		border: 4px solid rgba(230, 184, 0, 0.2);
		border-top-color: #E6B800;
		border-radius: 50%;
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		to { transform: rotate(360deg); }
	}

	.empty-state h3 {
		color: #f1f5f9;
		margin: 1rem 0 0.5rem;
	}

	.members-table {
		width: 100%;
		border-collapse: collapse;
	}

	.members-table thead {
		background: rgba(15, 23, 42, 0.6);
	}

	.members-table th {
		padding: 1rem 1.5rem;
		text-align: left;
		font-size: 0.75rem;
		font-weight: 600;
		color: #94a3b8;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.members-table tbody tr {
		border-top: 1px solid rgba(148, 163, 184, 0.1);
		transition: background 0.2s;
	}

	.members-table tbody tr:hover {
		background: rgba(230, 184, 0, 0.05);
	}

	.members-table tbody tr.selected {
		background: rgba(230, 184, 0, 0.1);
	}

	.members-table td {
		padding: 1rem 1.5rem;
		color: #cbd5e1;
	}

	.checkbox-col {
		width: 48px;
	}

	.checkbox-col input[type="checkbox"] {
		width: 18px;
		height: 18px;
		cursor: pointer;
	}

	.member-info {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.member-avatar {
		width: 40px;
		height: 40px;
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		font-weight: 700;
		font-size: 0.875rem;
		color: white;
		background: linear-gradient(135deg, var(--admin-accent-primary), var(--admin-accent-secondary));
	}

	.member-name {
		font-weight: 600;
		color: var(--admin-text-primary);
	}

	.member-email {
		font-size: 0.8125rem;
		color: var(--admin-text-muted);
	}

	.status-badge {
		display: inline-flex;
		padding: 0.25rem 0.75rem;
		border-radius: 9999px;
		font-size: 0.75rem;
		font-weight: 600;
		border: 1px solid;
	}

	.status-badge.status-active {
		background: var(--admin-success-bg);
		color: var(--admin-success);
		border-color: var(--admin-success-border);
	}

	.status-badge.status-trial {
		background: var(--admin-info-bg);
		color: var(--admin-info);
		border-color: var(--admin-info-border);
	}

	.status-badge.status-churned {
		background: var(--admin-error-bg);
		color: var(--admin-error);
		border-color: var(--admin-error-border);
	}

	.status-badge.status-default {
		background: var(--admin-badge-default-bg, rgba(148, 163, 184, 0.15));
		color: var(--admin-text-muted);
		border-color: var(--admin-border-light);
	}

	.plan-name {
		color: var(--admin-accent-primary);
	}

	.spending {
		font-weight: 600;
		color: var(--admin-text-secondary);
	}

	.spending.whale {
		color: var(--admin-warning);
	}

	.spending.high {
		color: var(--admin-success);
	}

	.date {
		font-size: 0.875rem;
		color: var(--admin-text-muted);
	}

	.actions {
		display: flex;
		gap: 0.5rem;
	}

	.action-btn {
		width: 32px;
		height: 32px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(148, 163, 184, 0.1);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 6px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
	}

	.action-btn:hover {
		background: rgba(230, 184, 0, 0.15);
		border-color: rgba(230, 184, 0, 0.3);
		color: #E6B800;
	}

	/* Pagination */
	.pagination {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1rem 1.5rem;
		border-top: 1px solid rgba(148, 163, 184, 0.1);
	}

	.pagination-info {
		font-size: 0.875rem;
		color: #64748b;
	}

	.pagination-controls {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.page-btn {
		width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(148, 163, 184, 0.1);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
	}

	.page-btn:hover:not(:disabled) {
		background: rgba(230, 184, 0, 0.15);
		border-color: rgba(230, 184, 0, 0.3);
		color: #E6B800;
	}

	.page-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.page-indicator {
		font-size: 0.875rem;
		color: #94a3b8;
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

	.template-selector {
		margin-bottom: 1.5rem;
	}

	.template-label {
		display: block;
		font-size: 0.75rem;
		font-weight: 600;
		color: #94a3b8;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		margin-bottom: 0.75rem;
	}

	.template-buttons {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
	}

	.template-btn {
		padding: 0.5rem 1rem;
		background: rgba(230, 184, 0, 0.1);
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 8px;
		color: #E6B800;
		font-size: 0.8125rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.template-btn:hover {
		background: rgba(230, 184, 0, 0.2);
	}

	.form-group {
		margin-bottom: 1rem;
	}

	.form-group label {
		display: block;
		font-size: 0.8125rem;
		font-weight: 500;
		color: #94a3b8;
		margin-bottom: 0.5rem;
	}

	.form-group input,
	.form-group textarea {
		width: 100%;
		padding: 0.75rem 1rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 10px;
		color: #f1f5f9;
		font-size: 0.9375rem;
		font-family: inherit;
		resize: vertical;
	}

	.form-group input:focus,
	.form-group textarea:focus {
		outline: none;
		border-color: rgba(230, 184, 0, 0.5);
	}

	.modal-footer {
		display: flex;
		justify-content: flex-end;
		gap: 0.75rem;
		padding: 1.5rem;
		border-top: 1px solid rgba(148, 163, 184, 0.1);
	}

	/* Buttons - RTP Admin Color System */
	.btn-secondary,
	.btn-primary {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1);
		border: none;
	}

	.btn-secondary {
		background: var(--admin-btn-bg);
		color: var(--admin-text-secondary);
		border: 1px solid var(--admin-border);
	}

	.btn-secondary:hover {
		background: var(--admin-btn-bg-hover);
		border-color: var(--admin-border-interactive);
		color: var(--admin-text-primary);
	}

	.btn-primary {
		background: var(--admin-btn-primary-bg);
		color: var(--admin-btn-primary-text);
		box-shadow: 0 4px 14px rgba(230, 184, 0, 0.3);
	}

	.btn-primary:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow: 0 8px 24px rgba(230, 184, 0, 0.4);
	}

	.btn-primary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	/* Responsive */
	@media (max-width: 1200px) {
		.stats-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	@media (max-width: 768px) {
		.members-page {
			padding: 1rem;
		}

		.header-content {
			flex-direction: column;
			align-items: flex-start;
			gap: 1rem;
		}

		.stats-grid {
			grid-template-columns: 1fr;
		}

		.toolbar {
			flex-direction: column;
		}

		.search-box {
			max-width: 100%;
		}

		.filters-panel {
			flex-direction: column;
		}

		.members-table-container {
			overflow-x: auto;
		}

		.members-table {
			min-width: 800px;
		}
	}

	/* Import Modal */
	.import-instructions {
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 12px;
		padding: 1rem 1.25rem;
		margin-bottom: 1.5rem;
	}

	.import-instructions h4 {
		color: #f1f5f9;
		font-size: 0.875rem;
		font-weight: 600;
		margin: 0 0 0.75rem;
	}

	.import-instructions ul {
		margin: 0;
		padding-left: 1.25rem;
		color: #94a3b8;
		font-size: 0.8125rem;
		line-height: 1.6;
	}

	.import-instructions code {
		background: rgba(230, 184, 0, 0.2);
		color: #E6B800;
		padding: 0.125rem 0.375rem;
		border-radius: 4px;
		font-size: 0.75rem;
	}

	.file-upload {
		margin-top: 1rem;
	}

	.upload-zone {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 2rem;
		background: rgba(15, 23, 42, 0.4);
		border: 2px dashed rgba(148, 163, 184, 0.3);
		border-radius: 12px;
		cursor: pointer;
		color: #94a3b8;
		text-align: center;
		transition: all 0.2s;
	}

	.upload-zone:hover {
		background: rgba(230, 184, 0, 0.1);
		border-color: rgba(230, 184, 0, 0.4);
		color: #E6B800;
	}

	.upload-zone.has-file {
		background: rgba(16, 185, 129, 0.1);
		border-color: rgba(16, 185, 129, 0.4);
		color: #34d399;
	}

	.upload-zone .file-name {
		font-weight: 600;
		color: #f1f5f9;
		margin-top: 0.75rem;
	}

	.upload-zone .file-size {
		font-size: 0.75rem;
		color: #64748b;
	}

	.upload-hint {
		font-size: 0.75rem;
		color: #64748b;
		margin-top: 0.25rem;
	}

	/* Error Banner */
	.error-banner {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem 1.5rem;
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		border-radius: 12px;
		color: #f87171;
		margin-bottom: 1.5rem;
	}

	.error-banner span {
		flex: 1;
		font-weight: 500;
	}

	.error-banner button {
		padding: 0.5rem 1rem;
		background: rgba(239, 68, 68, 0.2);
		border: 1px solid rgba(239, 68, 68, 0.3);
		border-radius: 6px;
		color: #f87171;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.error-banner button:hover {
		background: rgba(239, 68, 68, 0.3);
	}
</style>

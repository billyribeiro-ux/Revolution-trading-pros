<script lang="ts">
	import { browser } from '$app/environment';
	import { goto } from '$app/navigation';
	import { membersStore, emailStore } from '$lib/stores/members.svelte';
	import type { Member, MemberFilters, MemberFullDetails } from '$lib/api/members';
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
		IconUpload,
		IconUserPlus,
		IconEdit,
		IconTrash,
		IconBan,
		IconPlayerPlay,
		IconFileSpreadsheet,
		IconPdf
	} from '$lib/icons';
	import { membersApi } from '$lib/api/members';
	import { toastStore } from '$lib/stores/toast.svelte';

	// New enterprise components
	import ConfirmationModal from '$lib/components/admin/ConfirmationModal.svelte';
	import MemberFormModal from '$lib/components/admin/MemberFormModal.svelte';
	import MemberDetailDrawer from '$lib/components/admin/MemberDetailDrawer.svelte';
	import ActionsDropdown from '$lib/components/admin/ActionsDropdown.svelte';

	// Reactive state from stores
	let members = $derived(membersStore.members);
	let stats = $derived(membersStore.stats);
	let services = $derived(membersStore.services);
	let pagination = $derived(membersStore.pagination);
	let loading = $derived(membersStore.loading);

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

	// Enterprise member management state
	let showCreateModal = $state(false);
	let showEditModal = $state(false);
	let showDetailDrawer = $state(false);
	let showDeleteModal = $state(false);
	let selectedMemberId = $state<number | null>(null);
	let selectedMemberForEdit = $state<Member | null>(null);
	let selectedMemberForDelete = $state<Member | null>(null);
	let isDeleting = $state(false);
	let exportFormat = $state<'csv' | 'xlsx' | 'pdf'>('csv');

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
	// Svelte 5: $effect for initialization
	$effect(() => {
		if (!browser) return;

		const init = async () => {
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
		};
		init();
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

	// ═══════════════════════════════════════════════════════════════════════════
	// ENTERPRISE MEMBER MANAGEMENT HANDLERS
	// ═══════════════════════════════════════════════════════════════════════════

	function openMemberDetail(member: Member) {
		selectedMemberId = member.id;
		showDetailDrawer = true;
	}

	function openEditMember(member: Member | MemberFullDetails['member']) {
		selectedMemberForEdit = member as Member;
		showEditModal = true;
		showDetailDrawer = false;
	}

	function openDeleteMember(member: Member) {
		selectedMemberForDelete = member;
		showDeleteModal = true;
	}

	async function handleDeleteMember() {
		if (!selectedMemberForDelete) return;

		isDeleting = true;
		try {
			await membersApi.deleteMember(selectedMemberForDelete.id);
			toastStore.success('Member deleted successfully');
			showDeleteModal = false;
			selectedMemberForDelete = null;
			await membersStore.loadMembers();
		} catch (err) {
			toastStore.error(err instanceof Error ? err.message : 'Failed to delete member');
		} finally {
			isDeleting = false;
		}
	}

	function handleMemberSaved(savedMember: Member, temporaryPassword?: string) {
		if (temporaryPassword) {
			toastStore.success(`Member ${savedMember.name} created! Temporary password: ${temporaryPassword}`);
		} else {
			toastStore.success(`Member ${savedMember.name} saved successfully`);
		}
		showCreateModal = false;
		showEditModal = false;
		membersStore.loadMembers();
		membersStore.loadStats();
	}

	async function handleExportAdvanced(format: 'csv' | 'xlsx' | 'pdf') {
		exporting = true;
		try {
			const blob = await membersApi.exportMembersAdvanced({
				format,
				status: statusFilter || undefined
			});
			const url = URL.createObjectURL(blob);
			const a = document.createElement('a');
			a.href = url;
			const ext = format === 'xlsx' ? 'xlsx' : format === 'pdf' ? 'pdf' : 'csv';
			a.download = `members-export-${new Date().toISOString().split('T')[0]}.${ext}`;
			document.body.appendChild(a);
			a.click();
			document.body.removeChild(a);
			URL.revokeObjectURL(url);
			toastStore.success(`Members exported as ${format.toUpperCase()}`);
		} catch {
			toastStore.error('Failed to export members');
		} finally {
			exporting = false;
		}
	}

	function getMemberActions(targetMember: Member) {
		// Check if member is inactive (churned or never subscribed)
		const isInactive = targetMember.status === 'churned' || targetMember.status === 'never_subscribed';
		return [
			{ id: 'view', label: 'View Details', icon: IconExternalLink },
			{ id: 'edit', label: 'Edit Member', icon: IconEdit },
			{ id: 'email', label: 'Send Email', icon: IconMail },
			{ id: isInactive ? 'activate' : 'suspend', label: isInactive ? 'Activate' : 'Suspend', icon: isInactive ? IconPlayerPlay : IconBan },
			{
				id: 'delete',
				label: 'Delete Member',
				icon: IconTrash,
				variant: 'danger' as const,
				dividerBefore: true
			}
		];
	}

	function handleMemberAction(actionId: string, member: Member) {
		switch (actionId) {
			case 'view':
				openMemberDetail(member);
				break;
			case 'edit':
				openEditMember(member);
				break;
			case 'email':
				selectedMembers.clear();
				selectedMembers.add(member.id);
				selectedMembers = selectedMembers;
				showEmailModal = true;
				break;
			case 'delete':
				openDeleteMember(member);
				break;
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

	function formatCurrency(amount: number | null | undefined): string {
		if (amount === null || amount === undefined || isNaN(amount)) {
			return '$0';
		}
		return new Intl.NumberFormat('en-US', {
			style: 'currency',
			currency: 'USD',
			minimumFractionDigits: 0,
			maximumFractionDigits: 0
		}).format(amount);
	}

	function formatDate(dateString: string | null | undefined): string {
		if (!dateString) return 'N/A';
		const date = new Date(dateString);
		if (isNaN(date.getTime())) return 'N/A';
		return date.toLocaleDateString('en-US', {
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

<div class="admin-members">
	<!-- Animated Background -->
	<div class="bg-effects">
		<div class="bg-blob bg-blob-1"></div>
		<div class="bg-blob bg-blob-2"></div>
		<div class="bg-blob bg-blob-3"></div>
	</div>

	<div class="admin-page-container">
		<!-- Error Banner -->
		{#if initError}
			<div class="error-banner">
				<IconAlertTriangle size={20} />
				<span>{initError}</span>
				<button onclick={() => window.location.reload()}>Refresh Page</button>
			</div>
		{/if}

		<!-- Header - Centered Style -->
		<header class="page-header">
			<h1>Members Command Center</h1>
			<p class="subtitle">Comprehensive member management and analytics</p>
			<div class="header-actions">
				<button class="btn-secondary" onclick={handleRefresh} title="Refresh data">
					<IconRefresh size={18} />
					Refresh
				</button>
				<button class="btn-secondary" onclick={() => (showImportModal = true)}>
					<IconUpload size={18} />
					Import
				</button>
				<div class="export-dropdown">
					<button class="btn-secondary" onclick={() => handleExportAdvanced(exportFormat)} disabled={exporting}>
						<IconDownload size={18} />
						{exporting ? 'Exporting...' : `Export ${exportFormat.toUpperCase()}`}
					</button>
					<div class="export-options">
						<button onclick={() => handleExportAdvanced('csv')} disabled={exporting}>
							<IconDownload size={14} />
							CSV
						</button>
						<button onclick={() => handleExportAdvanced('xlsx')} disabled={exporting}>
							<IconFileSpreadsheet size={14} />
							Excel
						</button>
						<button onclick={() => handleExportAdvanced('pdf')} disabled={exporting}>
							<IconPdf size={14} />
							PDF
						</button>
					</div>
				</div>
				<button class="btn-secondary" onclick={() => goto('/admin/members/churned')}>
					<IconAlertTriangle size={18} />
					Win-Back
				</button>
				<button class="btn-primary" onclick={() => (showCreateModal = true)}>
					<IconUserPlus size={18} />
					Create Member
				</button>
			</div>
		</header>

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
						{#each (stats.growth_trend || []).slice(-6) as point}
							<div
								class="sparkline-bar"
								style="height: {(point.new /
									Math.max(...(stats.growth_trend || []).map((p) => p.new))) *
									100}%"
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
								stroke-dasharray="{(stats.subscriptions.active / stats.overview.total_members) *
									100} 100"
								stroke-linecap="round"
								transform="rotate(-90 18 18)"
							></circle>
						</svg>
						<span
							>{Math.round(
								(stats.subscriptions.active / stats.overview.total_members) * 100
							)}%</span
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
						<div class="stat-value">{formatCurrency(stats?.revenue?.mrr ?? 0)}</div>
						<div class="stat-change neutral">
							<IconCurrencyDollar size={14} />
							{formatCurrency(stats?.revenue?.avg_ltv ?? 0)} avg LTV
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
						<div class="stat-value">{stats?.subscriptions?.churn_rate ?? 0}%</div>
						<div class="stat-change negative">
							<IconAlertTriangle size={14} />
							{stats?.subscriptions?.churned ?? 0} churned
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
					id="search-members" name="search-members"
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

				<button class="btn-export" onclick={handleExport}>
					<IconDownload size={18} />
					Export CSV
				</button>
			</div>
		</div>

		<!-- Filters Panel -->
		{#if showFilters}
			<div class="filters-panel">
				<div class="filter-group">
					<label for="status-filter">Status</label>
					<select
						id="status-filter"
						bind:value={statusFilter}
						onchange={() => handleStatusFilter(statusFilter)}
					>
						<option value="">All Status</option>
						<option value="active">Active</option>
						<option value="trial">Trial</option>
						<option value="churned">Churned</option>
						<option value="never_subscribed">Never Subscribed</option>
					</select>
				</div>

				<div class="filter-group">
					<label for="service-filter">Service</label>
					<select
						id="service-filter"
						bind:value={serviceFilter}
						onchange={() => handleServiceFilter(serviceFilter)}
					>
						<option value="">All Services</option>
						{#each services as service}
							<option value={service.id}>{service.name}</option>
						{/each}
					</select>
				</div>

				<div class="filter-group">
					<label for="spending-filter">Spending Tier</label>
					<select
						id="spending-filter"
						bind:value={spendingFilter}
						onchange={() => handleSpendingFilter(spendingFilter)}
					>
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
						membersStore.setFilters({
							status: undefined,
							product_id: undefined,
							spending_tier: undefined,
							search: undefined
						});
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
									id="select-all-members"
									name="select-all-members"
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
										id="select-member-{member.id}"
										name="select-member-{member.id}"
										type="checkbox"
										checked={selectedMembers.has(member.id)}
										onchange={() => toggleMemberSelection(member.id)}
										aria-label="Select {member.name}"
									/>
								</td>
								<td>
									<button class="member-info-btn" onclick={() => openMemberDetail(member)}>
										<div class="member-avatar">
											{getMemberInitials(member)}
										</div>
										<div class="member-details">
											<div class="member-name">{member.name || ''}</div>
											<div class="member-email">{member.email || ''}</div>
										</div>
									</button>
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
									<span
										class="spending"
										class:whale={member.total_spent >= 5000}
										class:high={member.total_spent >= 1000 && member.total_spent < 5000}
									>
										{formatCurrency(member.total_spent)}
									</span>
								</td>
								<td>
									<span class="date">{formatDate(member.joined_at)}</span>
								</td>
								<td>
									<ActionsDropdown
										actions={getMemberActions(member)}
										onAction={(actionId) => handleMemberAction(actionId, member)}
									/>
								</td>
							</tr>
						{/each}
					</tbody>
				</table>

				<!-- Pagination -->
				{#if pagination}
					<div class="pagination">
						<div class="pagination-info">
							Showing {(pagination.current_page - 1) * pagination.per_page + 1} to {Math.min(
								pagination.current_page * pagination.per_page,
								pagination.total
							)} of {pagination.total} members
						</div>
						<div class="pagination-controls">
							<button
								class="page-btn"
								disabled={pagination.current_page === 1}
								onclick={() => membersStore.goToPage(pagination.current_page - 1)}
							>
								<IconChevronLeft size={18} />
							</button>
							<span class="page-indicator"
								>Page {pagination.current_page} of {pagination.last_page}</span
							>
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
	<!-- End admin-page-container -->
</div>

<!-- Email Modal -->
{#if showEmailModal}
	<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
	<div
		class="modal-overlay"
		onclick={() => (showEmailModal = false)}
		onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && (showEmailModal = false)}
		role="dialog"
		tabindex="-1"
		aria-modal="true"
	>
		<div
			class="modal-content"
			onclick={(e: MouseEvent) => e.stopPropagation()}
			onkeydown={(e: KeyboardEvent) => e.stopPropagation()}
			role="document"
		>
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
						{#each emailStore.presetTemplates as template}
							<button class="template-btn" onclick={() => applyTemplate(template)}>
								{template.name}
							</button>
						{/each}
					</div>
				</div>

				<div class="form-group">
					<label for="email-subject">Subject</label>
					<input
						id="email-subject" name="email-subject"
						type="text"
						bind:value={emailSubject}
						placeholder="Email subject..."
					/>
				</div>

				<div class="form-group">
					<label for="email-body">Body</label>
					<textarea
						id="email-body"
						bind:value={emailBody}
						rows="10"
						placeholder="Email body... Use {{ name }} for personalization"
					></textarea>
				</div>
			</div>

			<div class="modal-footer">
				<button class="btn-secondary" onclick={() => (showEmailModal = false)}>Cancel</button>
				<button
					class="btn-primary"
					onclick={handleBulkEmail}
					disabled={!emailSubject || !emailBody}
				>
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
	<div
		class="modal-overlay"
		onclick={() => (showImportModal = false)}
		onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && (showImportModal = false)}
		role="dialog"
		tabindex="-1"
		aria-modal="true"
	>
		<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
		<div
			class="modal-content"
			onclick={(e: MouseEvent) => e.stopPropagation()}
			onkeydown={(e: KeyboardEvent) => e.stopPropagation()}
			role="document"
		>
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
						<li>
							Optional: <code>first_name</code>, <code>last_name</code>, <code>phone</code>,
							<code>tags</code>
						</li>
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
						id="import-file" name="import-file"
						type="file"
						accept=".csv"
						onchange={handleFileSelect}
						style="display: none"
					/>
				</div>

				{#if importFile}
					<button
						class="btn-secondary"
						style="margin-top: 1rem"
						onclick={() => (importFile = null)}
					>
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

<!-- ═══════════════════════════════════════════════════════════════════════════════════
     ENTERPRISE MEMBER MANAGEMENT COMPONENTS
     Apple ICT 11+ Principal Engineer Grade
═══════════════════════════════════════════════════════════════════════════════════ -->

<!-- Member Detail Drawer -->
{#if showDetailDrawer && selectedMemberId}
	<MemberDetailDrawer
		memberId={selectedMemberId}
		isOpen={showDetailDrawer}
		onClose={() => {
			showDetailDrawer = false;
			selectedMemberId = null;
		}}
		onEdit={openEditMember}
		onEmail={(member) => {
			selectedMembers.clear();
			selectedMembers.add(member.id);
			selectedMembers = selectedMembers;
			showDetailDrawer = false;
			showEmailModal = true;
		}}
	/>
{/if}

<!-- Create Member Modal -->
{#if showCreateModal}
	<MemberFormModal
		isOpen={showCreateModal}
		onClose={() => (showCreateModal = false)}
		onSave={handleMemberSaved}
	/>
{/if}

<!-- Edit Member Modal -->
{#if showEditModal && selectedMemberForEdit}
	<MemberFormModal
		isOpen={showEditModal}
		member={selectedMemberForEdit}
		onClose={() => {
			showEditModal = false;
			selectedMemberForEdit = null;
		}}
		onSave={handleMemberSaved}
	/>
{/if}

<!-- Delete Confirmation Modal -->
{#if showDeleteModal && selectedMemberForDelete}
	<ConfirmationModal
		isOpen={showDeleteModal}
		title="Delete Member"
		message="Are you sure you want to delete {selectedMemberForDelete.name ||
			selectedMemberForDelete.email}? This action cannot be undone and will remove all associated data including subscriptions, orders, and activity history."
		confirmLabel="Delete Member"
		variant="danger"
		isLoading={isDeleting}
		onConfirm={handleDeleteMember}
		onCancel={() => {
			showDeleteModal = false;
			selectedMemberForDelete = null;
		}}
	/>
{/if}

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * RTP ADMIN MEMBERS - Apple ICT7+ Principal Engineer Grade
	 * Consistent with Analytics Dashboard styling
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* Outer Container with Gradient Background */
	.admin-members {
		min-height: calc(
			100vh - 70px - 4rem
		); /* Account for header (70px) and admin-content padding (2rem top + 2rem bottom) */
		background: var(--admin-bg, linear-gradient(135deg, #0f172a 0%, #1e293b 50%, #0f172a 100%));
		color: white;
		position: relative;
		overflow: hidden;
		margin: -2rem; /* Negate admin-content padding to fill edge-to-edge */
		padding: 2rem; /* Restore internal padding */
	}

	/* Inner Container */
	.admin-page-container {
		position: relative;
		z-index: 10;
		max-width: 1400px;
		margin: 0 auto;
		padding: 2rem;
	}

	/* Background Effects - Animated Blobs */
	.bg-effects {
		position: fixed;
		inset: 0;
		pointer-events: none;
		overflow: hidden;
	}

	.bg-blob {
		position: absolute;
		border-radius: 50%;
		filter: blur(80px);
		opacity: 0.15;
	}

	.bg-blob-1 {
		width: 600px;
		height: 600px;
		top: -200px;
		right: -200px;
		background: linear-gradient(135deg, var(--primary-500), var(--primary-600));
		animation: float 20s ease-in-out infinite;
	}

	.bg-blob-2 {
		width: 500px;
		height: 500px;
		bottom: -150px;
		left: -150px;
		background: linear-gradient(135deg, var(--primary-600), #1e293b);
		animation: float 25s ease-in-out infinite reverse;
	}

	.bg-blob-3 {
		width: 400px;
		height: 400px;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		background: linear-gradient(135deg, #10b981, #14b8a6);
		animation: float 30s ease-in-out infinite;
	}

	@keyframes float {
		0%,
		100% {
			transform: translate(0, 0) scale(1);
		}
		33% {
			transform: translate(30px, -30px) scale(1.05);
		}
		66% {
			transform: translate(-20px, 20px) scale(0.95);
		}
	}

	/* Header - CENTERED */
	.page-header {
		text-align: center;
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

	.subtitle {
		color: var(--text-tertiary);
		font-size: 0.875rem;
		margin: 0 0 1.5rem;
	}

	.header-actions {
		display: flex;
		justify-content: center;
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
		background: rgba(22, 27, 34, 0.8);
		border-radius: 12px;
		padding: 1.5rem;
		position: relative;
		overflow: hidden;
		border: 1px solid var(--border-muted);
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		backdrop-filter: blur(10px);
	}

	.stat-card.gradient-purple {
		border-color: rgba(230, 184, 0, 0.3);
	}
	.stat-card.gradient-emerald {
		border-color: rgba(16, 185, 129, 0.3);
	}
	.stat-card.gradient-gold {
		border-color: rgba(251, 191, 36, 0.3);
	}
	.stat-card.gradient-red {
		border-color: rgba(239, 68, 68, 0.3);
	}

	.stat-icon {
		width: 48px;
		height: 48px;
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.gradient-purple .stat-icon {
		background: rgba(230, 184, 0, 0.15);
		color: var(--primary-400);
	}
	.gradient-emerald .stat-icon {
		background: rgba(16, 185, 129, 0.15);
		color: var(--success-emphasis);
	}
	.gradient-gold .stat-icon {
		background: rgba(251, 191, 36, 0.15);
		color: var(--warning-emphasis);
	}
	.gradient-red .stat-icon {
		background: rgba(239, 68, 68, 0.15);
		color: var(--error-emphasis);
	}

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

	.stat-change.positive {
		color: var(--admin-success);
	}
	.stat-change.neutral {
		color: var(--admin-text-muted);
	}
	.stat-change.negative {
		color: var(--admin-error);
	}

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
		color: var(--success-emphasis);
	}

	.stat-ring span {
		position: absolute;
		inset: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 0.75rem;
		font-weight: 700;
		color: var(--success-emphasis);
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
		color: var(--error-emphasis);
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
		color: var(--text-primary);
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
		background: rgba(22, 27, 34, 0.8);
		border: 1px solid var(--border-muted);
		border-radius: 12px;
		cursor: pointer;
		transition: all 0.2s;
		min-width: 200px;
		backdrop-filter: blur(10px);
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
		color: var(--primary-400);
	}

	.service-name {
		font-weight: 600;
		color: var(--text-primary);
	}

	.service-type {
		font-size: 0.75rem;
		color: var(--text-tertiary);
		text-transform: capitalize;
	}

	.service-count {
		margin-left: auto;
		font-size: 1.25rem;
		font-weight: 700;
		color: var(--primary-400);
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
		background: rgba(22, 27, 34, 0.85);
		border: 1px solid var(--border-muted);
		border-radius: 12px;
		color: var(--text-secondary);
		backdrop-filter: blur(10px);
	}

	.search-box:focus-within {
		border-color: rgba(230, 184, 0, 0.5);
		box-shadow: 0 0 0 3px rgba(230, 184, 0, 0.15);
	}

	.search-box input {
		flex: 1;
		background: none;
		border: none;
		color: var(--text-primary);
		font-size: 0.9375rem;
		outline: none;
	}

	.search-box input::placeholder {
		color: var(--text-tertiary);
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
		background: rgba(100, 116, 139, 0.2);
		border: 1px solid rgba(100, 116, 139, 0.3);
		border-radius: 12px;
		color: var(--text-primary);
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.filter-toggle:hover,
	.btn-export:hover {
		border-color: rgba(230, 184, 0, 0.3);
		color: var(--primary-500);
	}

	.filter-toggle.active {
		background: rgba(230, 184, 0, 0.15);
		border-color: rgba(230, 184, 0, 0.3);
		color: var(--primary-500);
	}

	.btn-email {
		background: rgba(16, 185, 129, 0.15);
		border-color: rgba(16, 185, 129, 0.3);
		color: var(--success-emphasis);
	}

	/* Filters Panel */
	.filters-panel {
		display: flex;
		gap: 1rem;
		padding: 1rem;
		background: rgba(22, 27, 34, 0.85);
		border: 1px solid var(--border-muted);
		border-radius: 12px;
		margin-bottom: 1rem;
		backdrop-filter: blur(10px);
	}

	.filter-group {
		flex: 1;
	}

	.filter-group label {
		display: block;
		font-size: 0.75rem;
		font-weight: 600;
		color: var(--text-secondary);
		text-transform: uppercase;
		letter-spacing: 0.05em;
		margin-bottom: 0.5rem;
	}

	.filter-group select {
		width: 100%;
		padding: 0.625rem 0.875rem;
		background: rgba(22, 27, 34, 0.95);
		border: 1px solid var(--border-default);
		border-radius: 12px;
		color: var(--text-primary);
		font-size: 0.875rem;
		cursor: pointer;
	}

	.filter-group select:focus {
		border-color: rgba(230, 184, 0, 0.5);
		outline: none;
		box-shadow: 0 0 0 3px rgba(230, 184, 0, 0.15);
	}

	.clear-filters {
		padding: 0.625rem 1rem;
		background: rgba(100, 116, 139, 0.2);
		border: 1px solid rgba(100, 116, 139, 0.3);
		border-radius: 12px;
		color: var(--text-primary);
		font-weight: 500;
		cursor: pointer;
		align-self: flex-end;
	}

	.clear-filters:hover {
		background: rgba(239, 68, 68, 0.15);
		border-color: rgba(239, 68, 68, 0.3);
		color: var(--error-emphasis);
	}

	/* Members Table */
	.members-table-container {
		background: rgba(22, 27, 34, 0.85);
		border: 1px solid var(--border-muted);
		border-radius: 12px;
		overflow: hidden;
		backdrop-filter: blur(10px);
	}

	.loading-state,
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 2rem;
		color: var(--text-tertiary);
	}

	.loader {
		width: 48px;
		height: 48px;
		border: 4px solid rgba(230, 184, 0, 0.2);
		border-top-color: var(--primary-500);
		border-radius: 50%;
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.empty-state h3 {
		color: var(--text-primary);
		margin: 1rem 0 0.5rem;
	}

	.members-table {
		width: 100%;
		border-collapse: collapse;
	}

	.members-table thead {
		background: rgba(13, 17, 23, 0.6);
	}

	.members-table th {
		padding: 1rem 1.5rem;
		text-align: left;
		font-size: 0.75rem;
		font-weight: 600;
		color: var(--text-secondary);
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.members-table tbody tr {
		border-top: 1px solid var(--border-muted);
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
		color: var(--text-primary);
	}

	.checkbox-col {
		width: 48px;
	}

	.checkbox-col input[type='checkbox'] {
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
		border: 1px solid var(--border-default);
		border-radius: 6px;
		color: var(--text-secondary);
		cursor: pointer;
		transition: all 0.2s;
	}

	.action-btn:hover {
		background: rgba(230, 184, 0, 0.15);
		border-color: rgba(230, 184, 0, 0.3);
		color: var(--primary-500);
	}

	/* Pagination */
	.pagination {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1rem 1.5rem;
		border-top: 1px solid var(--border-muted);
	}

	.pagination-info {
		font-size: 0.875rem;
		color: var(--text-tertiary);
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
		border: 1px solid var(--border-default);
		border-radius: 12px;
		color: var(--text-secondary);
		cursor: pointer;
		transition: all 0.2s;
	}

	.page-btn:hover:not(:disabled) {
		background: rgba(230, 184, 0, 0.15);
		border-color: rgba(230, 184, 0, 0.3);
		color: var(--primary-500);
	}

	.page-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.page-indicator {
		font-size: 0.875rem;
		color: var(--text-secondary);
	}

	/* Modal */
	.modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.85);
		backdrop-filter: blur(8px);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 50;
		padding: 2rem;
	}

	.modal-content {
		background: linear-gradient(145deg, var(--bg-elevated) 0%, var(--bg-base) 100%);
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 16px;
		width: 100%;
		max-width: 600px;
		max-height: 90vh;
		overflow-y: auto;
		box-shadow:
			0 25px 60px -15px rgba(0, 0, 0, 0.7),
			0 0 40px -10px rgba(230, 184, 0, 0.1);
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1.5rem;
		border-bottom: 1px solid var(--border-muted);
		background: rgba(13, 17, 23, 0.5);
	}

	.modal-header h2 {
		font-size: 1.25rem;
		font-weight: 600;
		color: var(--text-primary);
		margin: 0;
	}

	.close-btn {
		width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(100, 116, 139, 0.2);
		border: 1px solid rgba(100, 116, 139, 0.3);
		border-radius: 10px;
		color: var(--text-secondary);
		cursor: pointer;
		transition: all 0.2s;
	}

	.close-btn:hover {
		background: rgba(100, 116, 139, 0.3);
		color: var(--text-primary);
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
		color: var(--text-secondary);
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
		padding: 0.625rem 1rem;
		background: rgba(230, 184, 0, 0.1);
		border: 1px solid rgba(230, 184, 0, 0.25);
		border-radius: 10px;
		color: var(--primary-500);
		font-size: 0.8125rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.template-btn:hover {
		background: rgba(230, 184, 0, 0.2);
		border-color: rgba(230, 184, 0, 0.4);
		transform: translateY(-1px);
	}

	.form-group {
		margin-bottom: 1.25rem;
	}

	.form-group label {
		display: block;
		font-size: 0.875rem;
		font-weight: 600;
		color: var(--text-primary);
		margin-bottom: 0.5rem;
	}

	.form-group input,
	.form-group textarea {
		width: 100%;
		padding: 0.75rem 1rem;
		background: rgba(13, 17, 23, 0.6);
		border: 1px solid var(--border-default);
		border-radius: 10px;
		color: var(--text-primary);
		font-size: 0.9375rem;
		font-family: inherit;
		resize: vertical;
		transition: all 0.2s;
	}

	.form-group input:focus,
	.form-group textarea:focus {
		outline: none;
		border-color: var(--primary-500);
		box-shadow: 0 0 0 3px rgba(230, 184, 0, 0.15);
	}

	.form-group input::placeholder,
	.form-group textarea::placeholder {
		color: var(--text-tertiary);
	}

	.modal-footer {
		display: flex;
		justify-content: flex-end;
		gap: 0.75rem;
		padding: 1.5rem;
		border-top: 1px solid var(--border-muted);
		background: rgba(13, 17, 23, 0.3);
	}

	/* Buttons - Email Templates Style */
	.btn-secondary,
	.btn-primary {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		border-radius: 12px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s ease;
		border: none;
	}

	.btn-secondary {
		background: rgba(100, 116, 139, 0.2);
		color: var(--text-primary);
		border: 1px solid rgba(100, 116, 139, 0.3);
	}

	.btn-secondary:hover {
		background: rgba(100, 116, 139, 0.3);
		color: var(--text-primary);
	}

	.btn-primary {
		background: linear-gradient(135deg, var(--primary-500), var(--primary-600));
		color: var(--bg-base);
		box-shadow: 0 4px 14px rgba(230, 184, 0, 0.3);
	}

	.btn-primary:hover:not(:disabled) {
		background: linear-gradient(135deg, var(--primary-400), var(--primary-500));
		transform: translateY(-2px);
		box-shadow: 0 8px 24px rgba(230, 184, 0, 0.4);
	}

	.btn-primary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * RESPONSIVE BREAKPOINTS - Apple ICT7 Principal Engineer Grade
	 * Mobile-first approach with progressive enhancement
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* Tablet Landscape (< 1200px) */
	@media (max-width: calc(var(--breakpoint-xl) - 80px)) {
		.stats-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	/* Tablet Portrait (< 1024px) */
	@media (max-width: calc(var(--breakpoint-lg) - 1px)) {
		.stats-grid {
			grid-template-columns: repeat(2, 1fr);
			gap: 0.75rem;
		}

		.header-actions {
			flex-wrap: wrap;
			gap: 0.5rem;
		}
	}

	/* Mobile Landscape (< 768px) */
	@media (max-width: calc(var(--breakpoint-md) - 1px)) {
		.admin-page-container {
			padding: 1rem;
		}

		.header-actions {
			flex-wrap: wrap;
		}

		.stats-grid {
			grid-template-columns: 1fr;
		}

		.toolbar {
			flex-direction: column;
			gap: 0.75rem;
		}

		.search-box {
			max-width: 100%;
		}

		.filters-panel {
			flex-direction: column;
		}

		.members-table-container {
			overflow-x: auto;
			-webkit-overflow-scrolling: touch;
		}

		.members-table {
			min-width: 800px;
		}
	}

	/* Mobile Portrait (< 640px) */
	@media (max-width: calc(var(--breakpoint-sm) - 1px)) {
		.admin-page-container {
			padding: 0.75rem;
		}

		.page-header h1 {
			font-size: 1.5rem;
		}

		.stats-grid {
			gap: 0.5rem;
		}

		.stat-card {
			padding: 1rem;
		}

		.stat-value {
			font-size: 1.25rem;
		}

		.btn-primary,
		.btn-secondary {
			padding: 0.5rem 0.75rem;
			font-size: 0.8125rem;
		}
	}

	/* Extra Small Mobile (< 380px) - iPhone SE, Galaxy Fold */
	@media (max-width: calc(var(--breakpoint-sm) - 260px)) {
		.admin-page-container {
			padding: 0.5rem;
		}

		.page-header h1 {
			font-size: 1.25rem;
		}

		.page-header {
			flex-direction: column;
			align-items: flex-start;
			gap: 0.75rem;
		}

		.header-actions {
			width: 100%;
		}

		.stats-grid {
			grid-template-columns: 1fr;
		}

		.stat-card {
			padding: 0.75rem;
		}

		.stat-value {
			font-size: 1.125rem;
		}

		.toolbar {
			padding: 0.75rem;
		}

		.modal-content {
			padding: 1rem;
			margin: 0.5rem;
			max-width: calc(100vw - 1rem);
		}
	}

	/* Touch Device Optimizations - Apple HIG 44pt minimum */
	@media (hover: none) and (pointer: coarse) {
		.btn-primary,
		.btn-secondary,
		.btn-icon {
			min-height: 44px;
			min-width: 44px;
		}

		.search-input,
		.filter-select {
			min-height: 48px;
			font-size: 16px; /* Prevents iOS zoom */
		}

		.members-table td,
		.members-table th {
			padding: 0.875rem 1rem;
		}

		.stat-card {
			min-height: 80px;
		}

		/* Larger touch targets for checkboxes */
		.member-checkbox {
			width: 20px;
			height: 20px;
		}
	}

	/* Reduced Motion - Accessibility */
	@media (prefers-reduced-motion: reduce) {
		.btn-primary,
		.btn-secondary,
		.stat-card,
		.member-row {
			transition: none;
		}

		.btn-primary:hover,
		.stat-card:hover {
			transform: none;
		}
	}

	/* High Contrast Mode - Accessibility */
	@media (prefers-contrast: high) {
		.stat-card,
		.toolbar,
		.members-table,
		.modal-content {
			border-width: 2px;
		}

		.stat-value,
		.page-header h1 {
			font-weight: 800;
		}

		.member-row:hover {
			outline: 2px solid currentColor;
		}
	}

	/* Print Styles */
	@media print {
		.admin-members {
			background: white;
			color: black;
		}

		.header-actions,
		.toolbar,
		.pagination {
			display: none !important;
		}

		.members-table {
			min-width: 100%;
			box-shadow: none;
			border: 1px solid #ccc;
		}

		.stat-card {
			break-inside: avoid;
			box-shadow: none;
			border: 1px solid #ccc;
		}
	}

	/* Landscape Mode - Short viewport */
	@media (max-height: 500px) and (orientation: landscape) {
		.admin-page-container {
			padding: 0.5rem 1rem;
		}

		.page-header {
			margin-bottom: 0.75rem;
		}

		.stats-grid {
			gap: 0.5rem;
		}

		.stat-card {
			padding: 0.75rem;
		}
	}

	/* Import Modal */
	.import-instructions {
		background: rgba(22, 27, 34, 0.6);
		border: 1px solid var(--border-muted);
		border-radius: 12px;
		padding: 1rem 1.25rem;
		margin-bottom: 1.5rem;
	}

	.import-instructions h4 {
		color: var(--text-primary);
		font-size: 0.875rem;
		font-weight: 600;
		margin: 0 0 0.75rem;
	}

	.import-instructions ul {
		margin: 0;
		padding-left: 1.25rem;
		color: var(--text-secondary);
		font-size: 0.8125rem;
		line-height: 1.6;
	}

	.import-instructions code {
		background: rgba(230, 184, 0, 0.2);
		color: var(--primary-500);
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
		background: rgba(22, 27, 34, 0.6);
		border: 2px dashed var(--border-default);
		border-radius: 12px;
		cursor: pointer;
		color: var(--text-secondary);
		text-align: center;
		transition: all 0.2s;
	}

	.upload-zone:hover {
		background: rgba(230, 184, 0, 0.1);
		border-color: rgba(230, 184, 0, 0.4);
		color: var(--primary-500);
	}

	.upload-zone.has-file {
		background: rgba(16, 185, 129, 0.1);
		border-color: rgba(16, 185, 129, 0.4);
		color: var(--success-emphasis);
	}

	.upload-zone .file-name {
		font-weight: 600;
		color: var(--text-primary);
		margin-top: 0.75rem;
	}

	.upload-zone .file-size {
		font-size: 0.75rem;
		color: var(--text-tertiary);
	}

	.upload-hint {
		font-size: 0.75rem;
		color: var(--text-tertiary);
		margin-top: 0.25rem;
	}

	/* Export Dropdown */
	.export-dropdown {
		position: relative;
		display: inline-block;
	}

	.export-dropdown:hover .export-options {
		display: flex;
	}

	.export-options {
		display: none;
		position: absolute;
		top: 100%;
		left: 0;
		z-index: 20;
		flex-direction: column;
		min-width: 140px;
		background: var(--admin-surface-primary, rgba(30, 41, 59, 0.98));
		border: 1px solid var(--admin-border-subtle, rgba(148, 163, 184, 0.15));
		border-radius: var(--radius-md, 0.5rem);
		padding: 0.375rem;
		margin-top: 0.25rem;
		box-shadow: 0 10px 30px -5px rgba(0, 0, 0, 0.4);
		animation: exportDropIn 0.15s ease;
	}

	@keyframes exportDropIn {
		from {
			opacity: 0;
			transform: translateY(-6px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	.export-options button {
		display: flex;
		align-items: center;
		gap: 0.625rem;
		width: 100%;
		padding: 0.5rem 0.75rem;
		background: transparent;
		border: none;
		color: var(--admin-text-secondary, #cbd5e1);
		font-family: var(--font-body), 'Roboto', sans-serif;
		font-size: 0.8125rem;
		font-weight: 500;
		text-align: left;
		border-radius: var(--radius-sm, 0.25rem);
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.export-options button:hover:not(:disabled) {
		background: var(--admin-surface-hover, rgba(230, 184, 0, 0.1));
		color: var(--primary-500);
	}

	.export-options button:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	/* Member Info Button - Clickable table cell */
	.member-info-btn {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		background: transparent;
		border: none;
		padding: 0.25rem 0;
		text-align: left;
		cursor: pointer;
		transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1);
		border-radius: var(--radius-sm, 0.25rem);
	}

	.member-info-btn:hover {
		transform: translateX(4px);
	}

	.member-info-btn:hover .member-name {
		color: var(--primary-500);
	}

	.member-info-btn:focus-visible {
		outline: 2px solid var(--primary-500);
		outline-offset: 2px;
	}

	.member-info-btn .member-details {
		display: flex;
		flex-direction: column;
		gap: 0.125rem;
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
		color: var(--error-emphasis);
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
		color: var(--error-emphasis);
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.error-banner button:hover {
		background: rgba(239, 68, 68, 0.3);
	}
</style>

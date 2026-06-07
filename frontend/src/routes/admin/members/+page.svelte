<script lang="ts">
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import { goto } from '$app/navigation';
	import { membersStore, emailStore } from '$lib/stores/members.svelte';
	import SkeletonLoader from '$lib/components/SkeletonLoader.svelte';
	import { SvelteSet } from 'svelte/reactivity';
	import type { Member, MemberFilters, MemberFullDetails } from '$lib/api/members';
	// FIX-2026-04-26: alert() calls replaced with existing toastStore import below.
	import {
		IconUsers,
		IconMail,
		IconSearch,
		IconFilter,
		IconDownload,
		IconExternalLink,
		IconEdit,
		IconTrash,
		IconBan,
		IconPlayerPlay
	} from '$lib/icons';
	import { membersApi } from '$lib/api/members';
	import { toastStore } from '$lib/stores/toast.svelte';

	// New enterprise components
	import ConfirmationModal from '$lib/components/admin/ConfirmationModal.svelte';
	import MemberFormModal from '$lib/components/admin/MemberFormModal.svelte';
	import MemberDetailDrawer from '$lib/components/admin/MemberDetailDrawer.svelte';
	import ActionsDropdown from '$lib/components/admin/ActionsDropdown.svelte';

	// Route-local extractions
	import ErrorBanner from './_components/ErrorBanner.svelte';
	import TempPasswordModal from './_components/TempPasswordModal.svelte';
	import ImportModal from './_components/ImportModal.svelte';
	import EmailModal from './_components/EmailModal.svelte';
	import TopServicesSection from './_components/TopServicesSection.svelte';
	import StatsGrid from './_components/StatsGrid.svelte';
	import MembersPagination from './_components/MembersPagination.svelte';
	import PageHeader from './_components/PageHeader.svelte';

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
	let selectedMembers = new SvelteSet<number>();
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

	// FIX-2026-04-26 (audit 02 §P2-8 / §P2-9 / CLAUDE.md commit 34a0bd070):
	// initialization is `onMount` (not `$effect`) and the search debounce
	// timer is cleared on destroy so a navigate-away mid-typing can't fire a
	// store update on a destroyed component.
	onMount(() => {
		if (!browser) return;

		const init = async () => {
			initError = '';
			const results = await Promise.allSettled([
				membersStore.loadMembers(),
				membersStore.loadStats(),
				membersStore.loadServices(),
				emailStore.loadTemplates()
			]);

			const [membersResult] = results;
			if (membersResult.status === 'rejected') {
				initError = 'Failed to load members. Please refresh the page.';
				console.error('Members load error:', membersResult.reason);
			}
		};
		init();

		return () => {
			if (searchDebounceTimer) {
				clearTimeout(searchDebounceTimer);
				searchDebounceTimer = null;
			}
		};
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
	}

	function selectAllMembers() {
		if (selectedMembers.size === members.length) {
			selectedMembers.clear();
		} else {
			members.forEach((m) => selectedMembers.add(m.id));
		}
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
			// FIX-2026-04-26: replaced native alert() with toastStore.success.
			toastStore.success(result.message);
			showEmailModal = false;
			selectedMembers.clear();
		} catch {
			// FIX-2026-04-26: replaced native alert() with toastStore.error.
			toastStore.error('Failed to send emails');
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
			// FIX-2026-04-26 (audit 02 §P1-8): the previous body was a 1.5s
			// `setTimeout` followed by `Math.floor(Math.random() * 50) + 10` —
			// admins were shown a fake import-success count for an upload that
			// never happened. Real CSV import wiring is tracked in
			// `02-members-subscriptions-DEFERRED.md` §D3 (multipart/form-data
			// upload + Rust handler). Until then, surface the missing wiring
			// honestly instead of lying.
			toastStore.warning(
				'Member CSV import is not yet wired to the backend. ' +
					'Upload a file with the JSON Grant API instead, or contact engineering. ' +
					'Tracked in audit 02 §P1-8.'
			);
			showImportModal = false;
			importFile = null;
		} catch {
			toastStore.error('Failed to import members');
		} finally {
			importing = false;
		}
	}

	// ENTERPRISE MEMBER MANAGEMENT HANDLERS

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

	// FIX-2026-04-26 (audit 02 §P1-9): the previous toast inlined the plaintext
	// `temporaryPassword`. Toasts can be auto-dismissed before an admin reads
	// them, are read aloud by screen-readers, and end up in browser
	// accessibility trees / support screenshots. We now stash the password
	// briefly and surface it through a one-time modal where the admin must
	// explicitly copy it before dismissing. Token is wiped when the modal
	// closes; we never log it.
	let temporaryPasswordToReveal = $state<string | null>(null);
	let temporaryPasswordMember = $state<string | null>(null);

	function handleMemberSaved(savedMember: Member, temporaryPassword?: string) {
		if (temporaryPassword) {
			temporaryPasswordToReveal = temporaryPassword;
			temporaryPasswordMember = savedMember.name;
			toastStore.success(`Member ${savedMember.name} created — see password modal.`);
		} else {
			toastStore.success(`Member ${savedMember.name} saved successfully`);
		}
		showCreateModal = false;
		showEditModal = false;
		membersStore.loadMembers();
		membersStore.loadStats();
	}

	async function copyTemporaryPassword() {
		if (!temporaryPasswordToReveal) return;
		try {
			await navigator.clipboard.writeText(temporaryPasswordToReveal);
			toastStore.success('Temporary password copied to clipboard');
		} catch {
			toastStore.error('Could not copy automatically — please select and copy manually.');
		}
	}

	function dismissTemporaryPassword() {
		temporaryPasswordToReveal = null;
		temporaryPasswordMember = null;
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
		const isInactive =
			targetMember.status === 'churned' || targetMember.status === 'never_subscribed';
		return [
			{ id: 'view', label: 'View Details', icon: IconExternalLink },
			{ id: 'edit', label: 'Edit Member', icon: IconEdit },
			{ id: 'email', label: 'Send Email', icon: IconMail },
			{
				id: isInactive ? 'activate' : 'suspend',
				label: isInactive ? 'Activate' : 'Suspend',
				icon: isInactive ? IconPlayerPlay : IconBan
			},
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

	// Discriminated dispatcher for PageHeader actions (R10-C pattern)
	import type { HeaderAction } from './_components/PageHeader.svelte';
	function handleHeaderAction(action: HeaderAction) {
		switch (action.type) {
			case 'refresh':
				handleRefresh();
				break;
			case 'import':
				showImportModal = true;
				break;
			case 'export':
				handleExportAdvanced(action.format);
				break;
			case 'win-back':
				goto('/admin/members/churned');
				break;
			case 'create':
				showCreateModal = true;
				break;
		}
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
			<ErrorBanner message={initError} onRefresh={() => window.location.reload()} />
		{/if}

		<!-- Header - Centered Style -->
		<PageHeader {exportFormat} {exporting} onAction={handleHeaderAction} />

		<!-- Stats Grid -->
		{#if stats}
			<StatsGrid {stats} {formatCurrency} onRecoverChurned={() => goto('/admin/members/churned')} />
		{/if}

		<!-- Top Services -->
		{#if stats?.top_services && stats.top_services.length > 0}
			<TopServicesSection
				services={stats.top_services}
				onSelectService={(id) => goto(`/admin/members/service/${id}`)}
			/>
		{/if}

		<!-- Toolbar -->
		<div class="toolbar">
			<div class="search-box">
				<IconSearch size={18} />
				<input
					id="search-members"
					name="search-members"
					type="text"
					placeholder="Search members by name or email..."
					bind:value={searchQuery}
					oninput={handleSearchInput}
					onkeydown={(e: KeyboardEvent) => e.key === 'Enter' && handleSearch()}
				/>
			</div>

			<div class="toolbar-actions">
				<button
					class={['filter-toggle', { active: showFilters }]}
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
						{#each services as service (service.id)}
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
				<!-- Skeleton rows reserve the members-table footprint to avoid CLS -->
				<SkeletonLoader variant="table-row" count={8} />
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
						{#each members as member (member.id)}
							<tr class={{ selected: selectedMembers.has(member.id) }}>
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
									<span class={['status-badge', getStatusColor(member.status)]}>
										{member.status_label}
									</span>
								</td>
								<td>
									<span class="plan-name">{member.current_plan || '-'}</span>
								</td>
								<td>
									<span
										class={[
											'spending',
											{
												whale: member.total_spent >= 5000,
												high: member.total_spent >= 1000 && member.total_spent < 5000
											}
										]}
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
					<MembersPagination {pagination} onGoToPage={(page) => membersStore.goToPage(page)} />
				{/if}
			{/if}
		</div>
	</div>
	<!-- End admin-page-container -->
</div>

{#if showEmailModal}
	<EmailModal
		recipientCount={selectedMembers.size}
		templates={emailStore.presetTemplates}
		bind:subject={emailSubject}
		bind:body={emailBody}
		onClose={() => (showEmailModal = false)}
		onApplyTemplate={applyTemplate}
		onSend={handleBulkEmail}
	/>
{/if}

{#if showImportModal}
	<ImportModal
		file={importFile}
		{importing}
		onClose={() => (showImportModal = false)}
		onFileSelect={handleFileSelect}
		onRemoveFile={() => (importFile = null)}
		onImport={handleImport}
	/>
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

{#if temporaryPasswordToReveal}
	<TempPasswordModal
		password={temporaryPasswordToReveal}
		memberName={temporaryPasswordMember}
		onCopy={copyTemporaryPassword}
		onDismiss={dismissTemporaryPassword}
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

	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 2rem;
		color: var(--text-tertiary);
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
	@media (max-width: 1200px) {
		.stats-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	/* Tablet Portrait (< 1024px) */
	@media (max-width: 1023px) {
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
	@media (max-width: 767px) {
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
	@media (max-width: 639px) {
		.admin-page-container {
			padding: 0.75rem;
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
	@media (max-width: 380px) {
		.admin-page-container {
			padding: 0.5rem;
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

		.stat-value {
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
			display: none;
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
</style>

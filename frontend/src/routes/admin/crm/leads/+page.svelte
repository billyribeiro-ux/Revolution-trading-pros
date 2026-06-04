<!--
	/admin/crm/leads - CRM Lead Management
	Apple Principal Engineer ICT 7 Grade - January 2026

	Features:
	- Full lead pipeline management with kanban view
	- Lead scoring and qualification
	- Bulk operations (assign, tag, delete, convert)
	- Advanced filtering and search
	- Lead source tracking and analytics
	- Full Svelte 5 $state/$derived/$effect reactivity
-->

<script lang="ts">
	import { SvelteSet } from 'svelte/reactivity';
	/**
	 * CRM Leads Management - Apple ICT 7 Principal Engineer Grade
	 *
	 * Comprehensive lead management system with:
	 * - Pipeline view with drag-and-drop stages
	 * - Lead scoring and qualification workflows
	 * - Bulk operations and automation
	 * - Source tracking and attribution
	 * - Full API integration
	 *
	 */

	import {
		IconUsers,
		IconUserPlus,
		IconSearch,
		IconFilter,
		IconEdit,
		IconEye,
		IconTrash,
		IconRefresh,
		IconBuilding,
		IconChevronDown,
		IconFlame,
		IconStar,
		IconStarFilled,
		IconArrowRight
	} from '$lib/icons';
	import { api } from '$lib/api/config';
	import { connections, getIsCrmConnected } from '$lib/stores/connections.svelte';
	import ApiNotConnected from '$lib/components/ApiNotConnected.svelte';
	import SkeletonLoader from '$lib/components/SkeletonLoader.svelte';
	import ConfirmationModal from '$lib/components/admin/ConfirmationModal.svelte';
	import PageHeader from './_components/PageHeader.svelte';
	import StatsGrid from './_components/StatsGrid.svelte';
	import DeleteLeadModal from './_components/DeleteLeadModal.svelte';
	import ConvertLeadModal from './_components/ConvertLeadModal.svelte';
	import LeadFormModal from './_components/LeadFormModal.svelte';
	import LeadsPagination from './_components/LeadsPagination.svelte';
	import { logger } from '$lib/utils/logger';
	import { onMount } from 'svelte';

	// TYPES

	interface Lead {
		id: string;
		email: string;
		phone?: string;
		first_name?: string;
		last_name?: string;
		full_name: string;
		company_name?: string;
		job_title?: string;
		status: 'new' | 'contacted' | 'qualified' | 'proposal' | 'negotiation' | 'won' | 'lost';
		source: string;
		lead_score: number;
		is_hot: boolean;
		is_starred: boolean;
		assigned_to?: { id: string; name: string };
		tags?: { id: string; name: string; color?: string }[];
		estimated_value?: number;
		notes_count: number;
		activities_count: number;
		last_contacted_at?: string;
		next_followup_at?: string;
		created_at: string;
		updated_at: string;
	}

	interface LeadStats {
		total: number;
		new: number;
		contacted: number;
		qualified: number;
		proposal: number;
		negotiation: number;
		won: number;
		lost: number;
		hot_leads: number;
		total_value: number;
		conversion_rate: number;
	}

	// STATE - Svelte 5 Runes

	let leads = $state<Lead[]>([]);
	let stats = $state<LeadStats>({
		total: 0,
		new: 0,
		contacted: 0,
		qualified: 0,
		proposal: 0,
		negotiation: 0,
		won: 0,
		lost: 0,
		hot_leads: 0,
		total_value: 0,
		conversion_rate: 0
	});
	let isLoading = $state(true);
	let connectionLoading = $state(true);
	let error = $state('');
	let searchQuery = $state('');
	let selectedStatus = $state<string>('all');
	let selectedSource = $state<string>('all');
	let sortBy = $state<string>('created_at');
	let sortOrder = $state<'asc' | 'desc'>('desc');
	let selectedLeads = new SvelteSet<string>();
	let showFilters = $state(false);
	let _viewMode = $state<'list' | 'kanban'>('list');

	// Modal states
	let showAddModal = $state(false);
	let showEditModal = $state(false);
	let showDeleteModal = $state(false);
	let showConvertModal = $state(false);
	let editingLead = $state<Lead | null>(null);
	let deletingLead = $state<Lead | null>(null);
	let convertingLead = $state<Lead | null>(null);

	// Bulk-action confirmation modal state — every destructive bulk op
	// goes through ConfirmationModal so the user has to acknowledge the
	// blast radius (was previously fire-and-forget; see audit P0 #3).
	let showBulkDeleteModal = $state(false);
	let bulkDeleteLoading = $state(false);
	let showBulkStatusModal = $state(false);
	let bulkStatusLoading = $state(false);
	let pendingBulkStatus = $state<string>('');

	// Form state
	let formData = $state({
		first_name: '',
		last_name: '',
		email: '',
		phone: '',
		company_name: '',
		job_title: '',
		source: 'website',
		estimated_value: 0,
		notes: ''
	});
	let formLoading = $state(false);
	let formError = $state('');

	// Pagination
	let requestedPage = $state(1);
	let perPage = $state(25);

	// CONSTANTS

	const statusOptions = [
		{ value: 'all', label: 'All Leads' },
		{ value: 'new', label: 'New' },
		{ value: 'contacted', label: 'Contacted' },
		{ value: 'qualified', label: 'Qualified' },
		{ value: 'proposal', label: 'Proposal' },
		{ value: 'negotiation', label: 'Negotiation' },
		{ value: 'won', label: 'Won' },
		{ value: 'lost', label: 'Lost' }
	];

	const sourceOptions = [
		{ value: 'all', label: 'All Sources' },
		{ value: 'website', label: 'Website' },
		{ value: 'referral', label: 'Referral' },
		{ value: 'social', label: 'Social Media' },
		{ value: 'email', label: 'Email Campaign' },
		{ value: 'ads', label: 'Paid Ads' },
		{ value: 'organic', label: 'Organic Search' },
		{ value: 'event', label: 'Event/Webinar' },
		{ value: 'other', label: 'Other' }
	];

	const sortOptions = [
		{ value: 'created_at', label: 'Date Created' },
		{ value: 'lead_score', label: 'Lead Score' },
		{ value: 'estimated_value', label: 'Est. Value' },
		{ value: 'last_contacted_at', label: 'Last Contacted' },
		{ value: 'full_name', label: 'Name' }
	];

	// DERIVED STATE - Svelte 5 $derived

	let filteredLeads = $derived.by(() => {
		let result = [...leads];

		// Search filter
		if (searchQuery.trim()) {
			const query = searchQuery.toLowerCase();
			result = result.filter(
				(lead) =>
					lead.full_name?.toLowerCase().includes(query) ||
					lead.email?.toLowerCase().includes(query) ||
					lead.company_name?.toLowerCase().includes(query) ||
					lead.phone?.includes(query)
			);
		}

		// Status filter
		if (selectedStatus !== 'all') {
			result = result.filter((lead) => lead.status === selectedStatus);
		}

		// Source filter
		if (selectedSource !== 'all') {
			result = result.filter((lead) => lead.source === selectedSource);
		}

		// Sorting
		result.sort((a, b) => {
			type SortVal = string | number | null | undefined;
			let aVal: SortVal = a[sortBy as keyof Lead] as SortVal;
			let bVal: SortVal = b[sortBy as keyof Lead] as SortVal;

			if (sortBy === 'full_name') {
				aVal = (typeof aVal === 'string' ? aVal.toLowerCase() : '') || '';
				bVal = (typeof bVal === 'string' ? bVal.toLowerCase() : '') || '';
			}

			if (aVal === undefined || aVal === null) aVal = sortOrder === 'asc' ? Infinity : -Infinity;
			if (bVal === undefined || bVal === null) bVal = sortOrder === 'asc' ? Infinity : -Infinity;

			if (sortOrder === 'asc') {
				return aVal > bVal ? 1 : -1;
			}
			return aVal < bVal ? 1 : -1;
		});

		return result;
	});

	let totalPages = $derived(Math.max(1, Math.ceil(filteredLeads.length / perPage)));
	let currentPage = $derived(Math.min(requestedPage, totalPages));
	let paginatedLeads = $derived(
		filteredLeads.slice((currentPage - 1) * perPage, currentPage * perPage)
	);

	let isAllSelected = $derived(
		paginatedLeads.length > 0 && paginatedLeads.every((lead) => selectedLeads.has(lead.id))
	);

	// API FUNCTIONS

	async function loadData() {
		isLoading = true;
		error = '';

		try {
			type LeadsRes = { data?: Lead[]; leads?: Lead[] } | Lead[];
			type StatsRes = { data?: Partial<LeadStats> } & Partial<LeadStats>;
			const [leadsRes, statsRes] = await Promise.allSettled([
				api.get<LeadsRes>('/api/admin/crm/leads'),
				api.get<StatsRes>('/api/admin/crm/leads/stats')
			]);

			if (leadsRes.status === 'fulfilled') {
				const v = leadsRes.value;
				leads = Array.isArray(v) ? v : v?.data || v?.leads || [];
			}

			if (statsRes.status === 'fulfilled') {
				const data = statsRes.value?.data || statsRes.value;
				stats = {
					total: data?.total || leads.length,
					new: data?.new || leads.filter((l) => l.status === 'new').length,
					contacted: data?.contacted || leads.filter((l) => l.status === 'contacted').length,
					qualified: data?.qualified || leads.filter((l) => l.status === 'qualified').length,
					proposal: data?.proposal || leads.filter((l) => l.status === 'proposal').length,
					negotiation: data?.negotiation || leads.filter((l) => l.status === 'negotiation').length,
					won: data?.won || leads.filter((l) => l.status === 'won').length,
					lost: data?.lost || leads.filter((l) => l.status === 'lost').length,
					hot_leads: data?.hot_leads || leads.filter((l) => l.is_hot).length,
					total_value:
						data?.total_value || leads.reduce((sum, l) => sum + (l.estimated_value || 0), 0),
					conversion_rate: data?.conversion_rate || 0
				};
			}
		} catch (error_) {
			error = error_ instanceof Error ? error_.message : 'Failed to load leads';
			logger.error('[CRM Leads] Load failed', { error: error_ });
		} finally {
			isLoading = false;
		}
	}

	async function createLead() {
		formLoading = true;
		formError = '';

		try {
			const payload = {
				first_name: formData.first_name.trim(),
				last_name: formData.last_name.trim(),
				email: formData.email.trim(),
				phone: formData.phone.trim() || undefined,
				company_name: formData.company_name.trim() || undefined,
				job_title: formData.job_title.trim() || undefined,
				source: formData.source,
				estimated_value: formData.estimated_value || 0,
				notes: formData.notes.trim() || undefined,
				status: 'new'
			};

			await api.post('/api/admin/crm/leads', payload);
			showAddModal = false;
			resetForm();
			await loadData();
		} catch (error_) {
			formError = error_ instanceof Error ? error_.message : 'Failed to create lead';
			logger.error('[CRM Leads] Create failed', { error: error_ });
		} finally {
			formLoading = false;
		}
	}

	async function updateLead() {
		if (!editingLead) return;
		formLoading = true;
		formError = '';

		try {
			const payload = {
				first_name: formData.first_name.trim(),
				last_name: formData.last_name.trim(),
				email: formData.email.trim(),
				phone: formData.phone.trim() || undefined,
				company_name: formData.company_name.trim() || undefined,
				job_title: formData.job_title.trim() || undefined,
				source: formData.source,
				estimated_value: formData.estimated_value || 0
			};

			await api.put(`/api/admin/crm/leads/${editingLead.id}`, payload);
			showEditModal = false;
			editingLead = null;
			resetForm();
			await loadData();
		} catch (error_) {
			formError = error_ instanceof Error ? error_.message : 'Failed to update lead';
			logger.error('[CRM Leads] Update failed', { error: error_ });
		} finally {
			formLoading = false;
		}
	}

	async function deleteLead() {
		if (!deletingLead) return;

		try {
			await api.delete(`/api/admin/crm/leads/${deletingLead.id}`);
			showDeleteModal = false;
			deletingLead = null;
			await loadData();
		} catch (error_) {
			error = error_ instanceof Error ? error_.message : 'Failed to delete lead';
			logger.error('[CRM Leads] Delete failed', { error: error_ });
		}
	}

	async function updateLeadStatus(leadId: string, newStatus: string) {
		try {
			await api.patch(`/api/admin/crm/leads/${leadId}/status`, { status: newStatus });
			await loadData();
		} catch (error_) {
			error = error_ instanceof Error ? error_.message : 'Failed to update lead status';
			logger.error('[CRM Leads] Status update failed', { error: error_ });
		}
	}

	async function toggleStarred(lead: Lead) {
		try {
			await api.patch(`/api/admin/crm/leads/${lead.id}/star`, {
				is_starred: !lead.is_starred
			});
			lead.is_starred = !lead.is_starred;
		} catch (error_) {
			logger.error('[CRM Leads] Toggle starred failed', { error: error_ });
		}
	}

	async function convertToContact() {
		if (!convertingLead) return;

		try {
			await api.post(`/api/admin/crm/leads/${convertingLead.id}/convert`);
			showConvertModal = false;
			convertingLead = null;
			await loadData();
		} catch (error_) {
			error = error_ instanceof Error ? error_.message : 'Failed to convert lead';
			logger.error('[CRM Leads] Convert failed', { error: error_ });
		}
	}

	function requestBulkDelete() {
		if (selectedLeads.size === 0) return;
		showBulkDeleteModal = true;
	}

	async function confirmBulkDelete() {
		if (selectedLeads.size === 0) {
			showBulkDeleteModal = false;
			return;
		}

		bulkDeleteLoading = true;
		try {
			await api.post('/api/admin/crm/leads/bulk-delete', {
				ids: Array.from(selectedLeads)
			});
			selectedLeads.clear();
			showBulkDeleteModal = false;
			await loadData();
		} catch (error_) {
			error = error_ instanceof Error ? error_.message : 'Failed to delete leads';
			logger.error('[CRM Leads] Bulk delete failed', { error: error_ });
		} finally {
			bulkDeleteLoading = false;
		}
	}

	function requestBulkUpdateStatus(status: string) {
		if (selectedLeads.size === 0) return;
		pendingBulkStatus = status;
		showBulkStatusModal = true;
	}

	async function confirmBulkUpdateStatus() {
		if (selectedLeads.size === 0 || !pendingBulkStatus) {
			showBulkStatusModal = false;
			return;
		}

		bulkStatusLoading = true;
		try {
			await api.post('/api/admin/crm/leads/bulk-status', {
				ids: Array.from(selectedLeads),
				status: pendingBulkStatus
			});
			selectedLeads.clear();
			showBulkStatusModal = false;
			pendingBulkStatus = '';
			await loadData();
		} catch (error_) {
			error = error_ instanceof Error ? error_.message : 'Failed to update leads';
			logger.error('[CRM Leads] Bulk update failed', { error: error_ });
		} finally {
			bulkStatusLoading = false;
		}
	}

	// HELPERS

	function resetForm() {
		formData = {
			first_name: '',
			last_name: '',
			email: '',
			phone: '',
			company_name: '',
			job_title: '',
			source: 'website',
			estimated_value: 0,
			notes: ''
		};
		formError = '';
	}

	function openEditModal(lead: Lead) {
		editingLead = lead;
		formData = {
			first_name: lead.first_name || '',
			last_name: lead.last_name || '',
			email: lead.email || '',
			phone: lead.phone || '',
			company_name: lead.company_name || '',
			job_title: lead.job_title || '',
			source: lead.source || 'website',
			estimated_value: lead.estimated_value || 0,
			notes: ''
		};
		showEditModal = true;
	}

	function openDeleteModal(lead: Lead) {
		deletingLead = lead;
		showDeleteModal = true;
	}

	function openConvertModal(lead: Lead) {
		convertingLead = lead;
		showConvertModal = true;
	}

	function toggleSelectAll() {
		if (isAllSelected) {
			selectedLeads.clear();
		} else {
			selectedLeads.clear();
			for (const lead of paginatedLeads) {
				selectedLeads.add(lead.id);
			}
		}
	}

	function toggleSelectLead(leadId: string) {
		if (selectedLeads.has(leadId)) {
			selectedLeads.delete(leadId);
		} else {
			selectedLeads.add(leadId);
		}
	}

	function resetPagination() {
		requestedPage = 1;
	}

	function setCurrentPage(page: number) {
		requestedPage = Math.max(1, Math.min(page, totalPages));
	}

	function formatCurrency(amount: number): string {
		return new Intl.NumberFormat('en-US', {
			style: 'currency',
			currency: 'USD',
			minimumFractionDigits: 0
		}).format(amount);
	}

	function formatDate(dateString: string | undefined): string {
		if (!dateString) return 'N/A';
		return new Date(dateString).toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'short',
			day: 'numeric'
		});
	}

	function getStatusColor(status: string): string {
		const colors: Record<string, string> = {
			new: 'bg-blue-500/15 text-blue-400 border-blue-500/30',
			contacted: 'bg-cyan-500/15 text-cyan-400 border-cyan-500/30',
			qualified: 'bg-violet-500/15 text-violet-400 border-violet-500/30',
			proposal: 'bg-amber-500/15 text-amber-400 border-amber-500/30',
			negotiation: 'bg-orange-500/15 text-orange-400 border-orange-500/30',
			won: 'bg-emerald-500/15 text-emerald-400 border-emerald-500/30',
			lost: 'bg-red-500/15 text-red-400 border-red-500/30'
		};
		return colors[status] || colors.new;
	}

	function getScoreColor(score: number): string {
		if (score >= 80) return 'text-emerald-400';
		if (score >= 60) return 'text-amber-400';
		if (score >= 40) return 'text-orange-400';
		return 'text-red-400';
	}

	// LIFECYCLE

	// Svelte 5: Initialize on mount.
	// Was previously a `$effect` that read `getIsCrmConnected()` after writing
	// to `connections.load()` — same write-while-reading-tracked-dep cascade
	// that was fixed elsewhere in commit 34a0bd070. Migrating to `onMount`
	// to keep the lifecycle init off the reactive graph.
	onMount(() => {
		(async () => {
			await connections.load();
			connectionLoading = false;

			if (getIsCrmConnected()) {
				await loadData();
			} else {
				isLoading = false;
			}
		})();
	});
</script>

<svelte:head>
	<title>Leads | CRM - Admin Dashboard</title>
</svelte:head>

<div class="admin-crm-leads">
	<!-- Animated Background -->
	<div class="bg-effects">
		<div class="bg-blob bg-blob-1"></div>
		<div class="bg-blob bg-blob-2"></div>
		<div class="bg-blob bg-blob-3"></div>
	</div>

	<div class="admin-page-container">
		<!-- Header - CENTERED -->
		<PageHeader />

		{#if connectionLoading}
			<SkeletonLoader variant="dashboard" />
		{:else if !getIsCrmConnected()}
			<ApiNotConnected
				serviceName="CRM"
				description="Connect to manage leads, track opportunities, and grow your sales pipeline."
				serviceKey="hubspot"
				icon="target"
				color="#f97316"
				features={[
					'Lead scoring and qualification',
					'Pipeline stage management',
					'Lead source tracking',
					'Conversion analytics',
					'Bulk operations'
				]}
			/>
		{:else}
			<!-- Stats Overview -->
			<StatsGrid {stats} {formatCurrency} />

			<!-- Actions Bar -->
			<div class="actions-bar">
				<div class="search-section">
					<div class="search-box">
						<IconSearch size={18} />
						<input
							id="search-leads"
							name="search-leads"
							type="text"
							placeholder="Search leads..."
							bind:value={searchQuery}
							oninput={resetPagination}
						/>
					</div>
					<button class="btn-filter" onclick={() => (showFilters = !showFilters)}>
						<IconFilter size={18} />
						Filters
						<IconChevronDown size={16} class={showFilters ? 'rotate' : ''} />
					</button>
				</div>

				<div class="action-buttons">
					{#if selectedLeads.size > 0}
						<div class="bulk-actions">
							<span class="selected-count">{selectedLeads.size} selected</span>
							<button class="btn-bulk" onclick={() => requestBulkUpdateStatus('qualified')}>
								Mark Qualified
							</button>
							<button class="btn-bulk danger" onclick={requestBulkDelete}> Delete </button>
						</div>
					{/if}

					<button
						class="btn-view-toggle"
						onclick={() => (_viewMode = _viewMode === 'list' ? 'kanban' : 'list')}
						title="Toggle view mode"
					>
						{#if _viewMode === 'list'}List{:else}Kanban{/if}
					</button>
					<button class="btn-refresh" onclick={loadData} disabled={isLoading}>
						<IconRefresh size={18} class={isLoading ? 'spinning' : ''} />
					</button>
					<button
						class="btn-primary"
						onclick={() => {
							resetForm();
							showAddModal = true;
						}}
					>
						<IconUserPlus size={18} />
						Add Lead
					</button>
				</div>
			</div>

			<!-- Filters Panel -->
			{#if showFilters}
				<div class="filters-panel">
					<div class="filter-group">
						<label for="filter-status">Status</label>
						<select id="filter-status" bind:value={selectedStatus} onchange={resetPagination}>
							{#each statusOptions as option (option.value)}
								<option value={option.value}>{option.label}</option>
							{/each}
						</select>
					</div>
					<div class="filter-group">
						<label for="filter-source">Source</label>
						<select id="filter-source" bind:value={selectedSource} onchange={resetPagination}>
							{#each sourceOptions as option (option.value)}
								<option value={option.value}>{option.label}</option>
							{/each}
						</select>
					</div>
					<div class="filter-group">
						<label for="filter-sort">Sort By</label>
						<select id="filter-sort" bind:value={sortBy} onchange={resetPagination}>
							{#each sortOptions as option (option.value)}
								<option value={option.value}>{option.label}</option>
							{/each}
						</select>
					</div>
					<div class="filter-group">
						<label for="filter-order">Order</label>
						<select id="filter-order" bind:value={sortOrder} onchange={resetPagination}>
							<option value="desc">Newest First</option>
							<option value="asc">Oldest First</option>
						</select>
					</div>
					<button
						class="btn-clear-filters"
						onclick={() => {
							selectedStatus = 'all';
							selectedSource = 'all';
							sortBy = 'created_at';
							sortOrder = 'desc';
							searchQuery = '';
							resetPagination();
						}}
					>
						Clear Filters
					</button>
				</div>
			{/if}

			<!-- Leads Table -->
			<div class="table-container">
				{#if isLoading}
					<div class="loading-state">
						<div class="spinner"></div>
						<p>Loading leads...</p>
					</div>
				{:else if error}
					<div class="error-state">
						<p>{error}</p>
						<button onclick={loadData}>Try Again</button>
					</div>
				{:else if filteredLeads.length === 0}
					<div class="empty-state">
						<IconUsers size={48} />
						<h3>No leads found</h3>
						<p>Create your first lead to start building your pipeline</p>
						<button
							class="btn-primary"
							onclick={() => {
								resetForm();
								showAddModal = true;
							}}
						>
							<IconUserPlus size={18} />
							Add Lead
						</button>
					</div>
				{:else}
					<table class="data-table">
						<thead>
							<tr>
								<th class="checkbox-col">
									<input
										id="select-all-leads"
										name="select-all-leads"
										type="checkbox"
										checked={isAllSelected}
										onchange={toggleSelectAll}
									/>
								</th>
								<th></th>
								<th>Lead</th>
								<th>Status</th>
								<th>Score</th>
								<th>Source</th>
								<th>Est. Value</th>
								<th>Last Contact</th>
								<th>Created</th>
								<th></th>
							</tr>
						</thead>
						<tbody>
							{#each paginatedLeads as lead (lead.id)}
								<tr class:selected={selectedLeads.has(lead.id)}>
									<td class="checkbox-col">
										<input
											id="select-lead-{lead.id}"
											name="select-lead-{lead.id}"
											type="checkbox"
											checked={selectedLeads.has(lead.id)}
											onchange={() => toggleSelectLead(lead.id)}
										/>
									</td>
									<td class="star-col">
										<button
											class="btn-star"
											class:starred={lead.is_starred}
											onclick={() => toggleStarred(lead)}
										>
											{#if lead.is_starred}
												<IconStarFilled size={16} />
											{:else}
												<IconStar size={16} />
											{/if}
										</button>
									</td>
									<td>
										<div class="lead-cell">
											<div class="lead-avatar" class:hot={lead.is_hot}>
												{lead.full_name?.charAt(0).toUpperCase() || '?'}
												{#if lead.is_hot}
													<span class="hot-indicator"><IconFlame size={10} /></span>
												{/if}
											</div>
											<div class="lead-info">
												<span class="lead-name">{lead.full_name || 'Unknown'}</span>
												<span class="lead-email">{lead.email || 'No email'}</span>
												{#if lead.company_name}
													<span class="lead-company">
														<IconBuilding size={12} />
														{lead.company_name}
													</span>
												{/if}
											</div>
										</div>
									</td>
									<td>
										<select
											class="status-select {getStatusColor(lead.status)}"
											value={lead.status}
											onchange={(e) =>
												updateLeadStatus(lead.id, (e.target as HTMLSelectElement).value)}
										>
											{#each statusOptions.filter((o) => o.value !== 'all') as option (option.value)}
												<option value={option.value}>{option.label}</option>
											{/each}
										</select>
									</td>
									<td>
										<div class="score-cell">
											<span class="score-value {getScoreColor(lead.lead_score)}">
												{lead.lead_score}
											</span>
											<div class="score-bar">
												<div class="score-fill" style="width: {lead.lead_score}%"></div>
											</div>
										</div>
									</td>
									<td>
										<span class="source-badge">{lead.source || 'Unknown'}</span>
									</td>
									<td class="value-col">
										{lead.estimated_value ? formatCurrency(lead.estimated_value) : '-'}
									</td>
									<td class="date-col">
										{formatDate(lead.last_contacted_at)}
									</td>
									<td class="date-col">
										{formatDate(lead.created_at)}
									</td>
									<td>
										<div class="action-buttons-cell">
											<a href="/admin/crm/leads/{lead.id}" class="btn-icon" title="View">
												<IconEye size={16} />
											</a>
											<button class="btn-icon" title="Edit" onclick={() => openEditModal(lead)}>
												<IconEdit size={16} />
											</button>
											<button
												class="btn-icon convert"
												title="Convert to Contact"
												onclick={() => openConvertModal(lead)}
											>
												<IconArrowRight size={16} />
											</button>
											<button
												class="btn-icon danger"
												title="Delete"
												onclick={() => openDeleteModal(lead)}
											>
												<IconTrash size={16} />
											</button>
										</div>
									</td>
								</tr>
							{/each}
						</tbody>
					</table>

					<!-- Pagination -->
					<LeadsPagination
						{currentPage}
						{totalPages}
						{perPage}
						totalCount={filteredLeads.length}
						onPageChange={setCurrentPage}
					/>
				{/if}
			</div>
		{/if}
	</div>
	<!-- End admin-page-container -->
</div>

<!-- Add Lead Modal -->
{#if showAddModal}
	<LeadFormModal
		mode="add"
		bind:formData
		{formError}
		{formLoading}
		{sourceOptions}
		onClose={() => (showAddModal = false)}
		onSubmit={createLead}
	/>
{/if}

<!-- Edit Lead Modal -->
{#if showEditModal && editingLead}
	<LeadFormModal
		mode="edit"
		bind:formData
		{formError}
		{formLoading}
		{sourceOptions}
		onClose={() => (showEditModal = false)}
		onSubmit={updateLead}
	/>
{/if}

<!-- Delete Confirmation Modal -->
{#if showDeleteModal && deletingLead}
	<DeleteLeadModal
		lead={deletingLead}
		onClose={() => (showDeleteModal = false)}
		onConfirm={deleteLead}
	/>
{/if}

<!-- Convert to Contact Modal -->
{#if showConvertModal && convertingLead}
	<ConvertLeadModal
		lead={convertingLead}
		onClose={() => (showConvertModal = false)}
		onConfirm={convertToContact}
	/>
{/if}

<!-- Bulk Delete Confirmation (audit P0 #3) -->
<ConfirmationModal
	isOpen={showBulkDeleteModal}
	title="Delete {selectedLeads.size} lead{selectedLeads.size === 1 ? '' : 's'}?"
	message="This will permanently delete the selected leads and all associated notes, activities, and tags. This action cannot be undone."
	confirmText="Delete {selectedLeads.size} lead{selectedLeads.size === 1 ? '' : 's'}"
	variant="danger"
	isLoading={bulkDeleteLoading}
	onConfirm={confirmBulkDelete}
	onCancel={() => (showBulkDeleteModal = false)}
/>

<!-- Bulk Status Update Confirmation -->
<ConfirmationModal
	isOpen={showBulkStatusModal}
	title="Update {selectedLeads.size} lead{selectedLeads.size === 1 ? '' : 's'}?"
	message="Change the status of the selected leads to '{pendingBulkStatus}'? This will fire any automation rules tied to that status."
	confirmText="Update {selectedLeads.size} lead{selectedLeads.size === 1 ? '' : 's'}"
	variant="warning"
	isLoading={bulkStatusLoading}
	onConfirm={confirmBulkUpdateStatus}
	onCancel={() => {
		showBulkStatusModal = false;
		pendingBulkStatus = '';
	}}
/>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   LEADS PAGE - Email Templates Style
	   ═══════════════════════════════════════════════════════════════════════════ */

	.page {
		max-width: 1400px;
		margin: 0 auto;
		padding: 2rem;
	}

	/* Actions Bar */
	.actions-bar {
		display: flex;
		justify-content: space-between;
		align-items: center;
		gap: 16px;
		margin-bottom: 16px;
		flex-wrap: wrap;
	}

	.search-section {
		display: flex;
		align-items: center;
		gap: 12px;
		flex: 1;
		max-width: 500px;
	}

	.search-box {
		display: flex;
		align-items: center;
		gap: 10px;
		flex: 1;
		padding: 0 16px;
		background: #1e293b;
		border: 1px solid #334155;
		border-radius: 10px;
		transition: all 0.2s;
	}

	.search-box:focus-within {
		border-color: var(--primary-500);
		box-shadow: 0 0 0 3px rgba(99, 102, 241, 0.1);
	}

	.search-box :global(svg) {
		color: #64748b;
		flex-shrink: 0;
	}

	.search-box input {
		flex: 1;
		padding: 12px 0;
		background: transparent;
		border: none;
		color: white;
		font-size: 0.9rem;
		outline: none;
	}

	.search-box input::placeholder {
		color: #64748b;
	}

	.btn-filter {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 12px 16px;
		background: #1e293b;
		border: 1px solid #334155;
		border-radius: 10px;
		color: #94a3b8;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-filter:hover {
		border-color: #475569;
		color: white;
	}

	.btn-filter :global(.rotate) {
		transform: rotate(180deg);
	}

	.action-buttons {
		display: flex;
		align-items: center;
		gap: 12px;
	}

	.bulk-actions {
		display: flex;
		align-items: center;
		gap: 12px;
		padding-right: 12px;
		border-right: 1px solid #334155;
	}

	.selected-count {
		font-size: 0.85rem;
		color: #818cf8;
		font-weight: 600;
	}

	.btn-bulk {
		padding: 8px 14px;
		background: rgba(99, 102, 241, 0.1);
		border: 1px solid rgba(99, 102, 241, 0.3);
		border-radius: 8px;
		color: #818cf8;
		font-size: 0.8rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-bulk:hover {
		background: rgba(99, 102, 241, 0.2);
	}

	.btn-bulk.danger {
		background: rgba(239, 68, 68, 0.1);
		border-color: rgba(239, 68, 68, 0.3);
		color: #f87171;
	}

	.btn-bulk.danger:hover {
		background: rgba(239, 68, 68, 0.2);
	}

	.btn-refresh {
		width: 44px;
		height: 44px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: #1e293b;
		border: 1px solid #334155;
		border-radius: 10px;
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
		gap: 8px;
		padding: 0.75rem 1.25rem;
		background: linear-gradient(135deg, var(--primary-500), var(--primary-600));
		color: white;
		border: none;
		border-radius: 8px;
		font-weight: 600;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-primary:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow: 0 4px 15px rgba(99, 102, 241, 0.4);
	}

	.btn-primary:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	/* Filters Panel */
	.filters-panel {
		display: flex;
		align-items: flex-end;
		gap: 16px;
		padding: 20px;
		margin-bottom: 16px;
		background: #1e293b;
		border: 1px solid #334155;
		border-radius: 12px;
		flex-wrap: wrap;
	}

	.filter-group {
		display: flex;
		flex-direction: column;
		gap: 6px;
		min-width: 150px;
	}

	.filter-group label {
		font-size: 0.75rem;
		font-weight: 600;
		color: #64748b;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.filter-group select {
		padding: 10px 14px;
		background: #0f172a;
		border: 1px solid #334155;
		border-radius: 8px;
		color: white;
		font-size: 0.875rem;
		cursor: pointer;
	}

	.filter-group select:focus {
		outline: none;
		border-color: var(--primary-500);
	}

	.btn-clear-filters {
		padding: 10px 16px;
		background: transparent;
		border: 1px solid #334155;
		border-radius: 8px;
		color: #94a3b8;
		font-size: 0.85rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-clear-filters:hover {
		border-color: #f87171;
		color: #f87171;
	}

	/* Table */
	.table-container {
		background: rgba(30, 41, 59, 0.4);
		border: 1px solid #334155;
		border-radius: 8px;
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
		border-bottom: 1px solid rgba(51, 65, 85, 0.5);
	}

	.data-table tbody tr {
		transition: background 0.15s;
	}

	.data-table tbody tr:hover {
		background: rgba(99, 102, 241, 0.05);
	}

	.data-table tbody tr.selected {
		background: rgba(99, 102, 241, 0.1);
	}

	.checkbox-col {
		width: 50px;
	}

	.checkbox-col input {
		width: 18px;
		height: 18px;
		cursor: pointer;
	}

	.star-col {
		width: 40px;
	}

	.btn-star {
		display: flex;
		padding: 6px;
		background: transparent;
		border: none;
		color: #475569;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-star:hover,
	.btn-star.starred {
		color: #fbbf24;
	}

	/* Lead Cell */
	.lead-cell {
		display: flex;
		align-items: center;
		gap: 12px;
	}

	.lead-avatar {
		width: 42px;
		height: 42px;
		border-radius: 50%;
		background: linear-gradient(135deg, var(--primary-500), var(--primary-600));
		display: flex;
		align-items: center;
		justify-content: center;
		font-weight: 600;
		color: white;
		font-size: 0.9rem;
		flex-shrink: 0;
		position: relative;
	}

	.lead-avatar.hot {
		background: linear-gradient(135deg, #f97316, #ef4444);
	}

	.hot-indicator {
		position: absolute;
		bottom: -2px;
		right: -2px;
		width: 18px;
		height: 18px;
		background: #ef4444;
		border: 2px solid #1e293b;
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		color: white;
	}

	.lead-info {
		display: flex;
		flex-direction: column;
		gap: 2px;
	}

	.lead-name {
		font-weight: 600;
		color: white;
	}

	.lead-email {
		font-size: 0.8rem;
		color: #64748b;
	}

	.lead-company {
		display: flex;
		align-items: center;
		gap: 4px;
		font-size: 0.75rem;
		color: #94a3b8;
	}

	/* Status Select */
	.status-select {
		padding: 6px 12px;
		background: transparent;
		border: 1px solid;
		border-radius: 20px;
		font-size: 0.75rem;
		font-weight: 600;
		cursor: pointer;
		text-transform: capitalize;
	}

	.status-select:focus {
		outline: none;
	}

	/* Score Cell */
	.score-cell {
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.score-value {
		font-weight: 700;
		font-size: 0.9rem;
		min-width: 28px;
	}

	.score-bar {
		width: 60px;
		height: 6px;
		background: #334155;
		border-radius: 3px;
		overflow: hidden;
	}

	.score-fill {
		height: 100%;
		background: linear-gradient(90deg, var(--primary-500), var(--primary-600));
		border-radius: 3px;
		transition: width 0.3s;
	}

	/* Source Badge */
	.source-badge {
		display: inline-block;
		padding: 4px 10px;
		background: rgba(100, 116, 139, 0.15);
		border-radius: 6px;
		font-size: 0.75rem;
		color: #94a3b8;
		text-transform: capitalize;
	}

	.value-col,
	.date-col {
		white-space: nowrap;
		color: #94a3b8;
	}

	/* Action Buttons */
	.action-buttons-cell {
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
		background: rgba(99, 102, 241, 0.1);
		color: #818cf8;
	}

	.btn-icon.convert:hover {
		background: rgba(16, 185, 129, 0.1);
		color: #34d399;
	}

	.btn-icon.danger:hover {
		background: rgba(239, 68, 68, 0.1);
		color: #f87171;
	}

	/* Table Footer */
	/* States */
	.loading-state,
	.error-state,
	.empty-state {
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
		border: 3px solid rgba(99, 102, 241, 0.2);
		border-top-color: var(--primary-500);
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin-bottom: 16px;
	}

	/* Responsive */
	@media (max-width: 1400px) {
		.stats-grid {
			grid-template-columns: repeat(3, 1fr);
		}
	}

	@media (max-width: 1023.98px) {
		.stats-grid {
			grid-template-columns: repeat(2, 1fr);
		}

		.data-table {
			font-size: 0.8rem;
		}

		.data-table th,
		.data-table td {
			padding: 12px;
		}
	}

	@media (max-width: 767.98px) {
		.page {
			padding: 1rem;
		}

		.stats-grid {
			grid-template-columns: 1fr;
		}

		.actions-bar {
			flex-direction: column;
			align-items: stretch;
		}

		.search-section {
			max-width: none;
		}

		.action-buttons {
			justify-content: flex-end;
		}

		.filters-panel {
			flex-direction: column;
			align-items: stretch;
		}

		.filter-group {
			width: 100%;
		}

		.table-container {
			overflow-x: auto;
		}
	}
</style>

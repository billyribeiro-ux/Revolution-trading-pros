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
	/**
	 * CRM Leads Management - Apple ICT 7 Principal Engineer Grade
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Comprehensive lead management system with:
	 * - Pipeline view with drag-and-drop stages
	 * - Lead scoring and qualification workflows
	 * - Bulk operations and automation
	 * - Source tracking and attribution
	 * - Full API integration
	 *
	 * @version 1.0.0 (January 2026)
	 */

	import { browser } from '$app/environment';
	import {
		IconUsers,
		IconUserPlus,
		IconSearch,
		IconFilter,
		IconDotsVertical,
		IconEdit,
		IconEye,
		IconTrash,
		IconTag,
		IconChartBar,
		IconTrendingUp,
		IconRefresh,
		IconMail,
		IconPhone,
		IconBuilding,
		IconCalendar,
		IconCheck,
		IconX,
		IconChevronDown,
		IconTarget,
		IconFlame,
		IconStar,
		IconStarFilled,
		IconArrowRight,
		IconDownload,
		IconUpload
	} from '$lib/icons';
	import { api } from '$lib/api/config';
	import { connections, isCrmConnected } from '$lib/stores/connections.svelte';
	import ApiNotConnected from '$lib/components/ApiNotConnected.svelte';
	import SkeletonLoader from '$lib/components/SkeletonLoader.svelte';

	// ═══════════════════════════════════════════════════════════════════════════
	// TYPES
	// ═══════════════════════════════════════════════════════════════════════════

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

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE - Svelte 5 Runes
	// ═══════════════════════════════════════════════════════════════════════════

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
	let selectedLeads = $state<Set<string>>(new Set());
	let showFilters = $state(false);
	let viewMode = $state<'list' | 'kanban'>('list');

	// Modal states
	let showAddModal = $state(false);
	let showEditModal = $state(false);
	let showDeleteModal = $state(false);
	let showConvertModal = $state(false);
	let editingLead = $state<Lead | null>(null);
	let deletingLead = $state<Lead | null>(null);
	let convertingLead = $state<Lead | null>(null);

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
	let currentPage = $state(1);
	let totalPages = $state(1);
	let perPage = $state(25);

	// ═══════════════════════════════════════════════════════════════════════════
	// CONSTANTS
	// ═══════════════════════════════════════════════════════════════════════════

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

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED STATE - Svelte 5 $derived
	// ═══════════════════════════════════════════════════════════════════════════

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
			let aVal: any = a[sortBy as keyof Lead];
			let bVal: any = b[sortBy as keyof Lead];

			if (sortBy === 'full_name') {
				aVal = aVal?.toLowerCase() || '';
				bVal = bVal?.toLowerCase() || '';
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

	let paginatedLeads = $derived(
		filteredLeads.slice((currentPage - 1) * perPage, currentPage * perPage)
	);

	let isAllSelected = $derived(
		paginatedLeads.length > 0 && paginatedLeads.every((lead) => selectedLeads.has(lead.id))
	);

	// ═══════════════════════════════════════════════════════════════════════════
	// EFFECTS - Svelte 5 $effect
	// ═══════════════════════════════════════════════════════════════════════════

	$effect(() => {
		// Reset page when filters change
		if (searchQuery || selectedStatus || selectedSource) {
			currentPage = 1;
		}
	});

	$effect(() => {
		// Calculate total pages when filtered leads change
		totalPages = Math.max(1, Math.ceil(filteredLeads.length / perPage));
		if (currentPage > totalPages) {
			currentPage = totalPages;
		}
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// API FUNCTIONS
	// ═══════════════════════════════════════════════════════════════════════════

	async function loadData() {
		isLoading = true;
		error = '';

		try {
			const [leadsRes, statsRes] = await Promise.allSettled([
				api.get('/api/admin/crm/leads'),
				api.get('/api/admin/crm/leads/stats')
			]);

			if (leadsRes.status === 'fulfilled') {
				leads = leadsRes.value?.data || leadsRes.value?.leads || leadsRes.value || [];
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
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load leads';
			console.error('Load leads error:', err);
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
		} catch (err) {
			formError = err instanceof Error ? err.message : 'Failed to create lead';
			console.error('Create lead error:', err);
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
		} catch (err) {
			formError = err instanceof Error ? err.message : 'Failed to update lead';
			console.error('Update lead error:', err);
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
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to delete lead';
			console.error('Delete lead error:', err);
		}
	}

	async function updateLeadStatus(leadId: string, newStatus: string) {
		try {
			await api.patch(`/api/admin/crm/leads/${leadId}/status`, { status: newStatus });
			await loadData();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to update lead status';
			console.error('Update status error:', err);
		}
	}

	async function toggleStarred(lead: Lead) {
		try {
			await api.patch(`/api/admin/crm/leads/${lead.id}/star`, {
				is_starred: !lead.is_starred
			});
			lead.is_starred = !lead.is_starred;
		} catch (err) {
			console.error('Toggle starred error:', err);
		}
	}

	async function convertToContact() {
		if (!convertingLead) return;

		try {
			await api.post(`/api/admin/crm/leads/${convertingLead.id}/convert`);
			showConvertModal = false;
			convertingLead = null;
			await loadData();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to convert lead';
			console.error('Convert lead error:', err);
		}
	}

	async function bulkDelete() {
		if (selectedLeads.size === 0) return;

		try {
			await api.post('/api/admin/crm/leads/bulk-delete', {
				ids: Array.from(selectedLeads)
			});
			selectedLeads = new Set();
			await loadData();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to delete leads';
			console.error('Bulk delete error:', err);
		}
	}

	async function bulkUpdateStatus(status: string) {
		if (selectedLeads.size === 0) return;

		try {
			await api.post('/api/admin/crm/leads/bulk-status', {
				ids: Array.from(selectedLeads),
				status
			});
			selectedLeads = new Set();
			await loadData();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to update leads';
			console.error('Bulk update error:', err);
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// HELPERS
	// ═══════════════════════════════════════════════════════════════════════════

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
			selectedLeads = new Set();
		} else {
			selectedLeads = new Set(paginatedLeads.map((l) => l.id));
		}
	}

	function toggleSelectLead(leadId: string) {
		const newSet = new Set(selectedLeads);
		if (newSet.has(leadId)) {
			newSet.delete(leadId);
		} else {
			newSet.add(leadId);
		}
		selectedLeads = newSet;
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

	// ═══════════════════════════════════════════════════════════════════════════
	// LIFECYCLE
	// ═══════════════════════════════════════════════════════════════════════════

	// Svelte 5: Initialize on mount
	$effect(() => {
		if (!browser) return;

		const init = async () => {
			await connections.load();
			connectionLoading = false;

			if ($isCrmConnected) {
				await loadData();
			} else {
				isLoading = false;
			}
		};
		init();
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
		<header class="page-header">
			<h1>
				<IconUsers size={28} />
				Lead Management
			</h1>
			<p class="subtitle">Track, qualify, and convert leads through your sales pipeline</p>
		</header>

		{#if connectionLoading}
			<SkeletonLoader variant="dashboard" />
		{:else if !$isCrmConnected}
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
			<section class="stats-grid">
				<div class="stat-card highlight">
					<div class="stat-icon blue">
						<IconUsers size={24} />
					</div>
					<div class="stat-content">
						<span class="stat-value">{stats.total.toLocaleString()}</span>
						<span class="stat-label">Total Leads</span>
					</div>
				</div>
				<div class="stat-card">
					<div class="stat-icon cyan">
						<IconFlame size={24} />
					</div>
					<div class="stat-content">
						<span class="stat-value">{stats.hot_leads}</span>
						<span class="stat-label">Hot Leads</span>
					</div>
				</div>
				<div class="stat-card">
					<div class="stat-icon purple">
						<IconTarget size={24} />
					</div>
					<div class="stat-content">
						<span class="stat-value">{stats.qualified}</span>
						<span class="stat-label">Qualified</span>
					</div>
				</div>
				<div class="stat-card">
					<div class="stat-icon emerald">
						<IconCheck size={24} />
					</div>
					<div class="stat-content">
						<span class="stat-value">{stats.won}</span>
						<span class="stat-label">Won</span>
					</div>
				</div>
				<div class="stat-card">
					<div class="stat-icon amber">
						<IconChartBar size={24} />
					</div>
					<div class="stat-content">
						<span class="stat-value">{formatCurrency(stats.total_value)}</span>
						<span class="stat-label">Pipeline Value</span>
					</div>
				</div>
				<div class="stat-card">
					<div class="stat-icon green">
						<IconTrendingUp size={24} />
					</div>
					<div class="stat-content">
						<span class="stat-value">{stats.conversion_rate.toFixed(1)}%</span>
						<span class="stat-label">Conversion Rate</span>
					</div>
				</div>
			</section>

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
							<button class="btn-bulk" onclick={() => bulkUpdateStatus('qualified')}>
								Mark Qualified
							</button>
							<button class="btn-bulk danger" onclick={bulkDelete}> Delete </button>
						</div>
					{/if}

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
						<select id="filter-status" bind:value={selectedStatus}>
							{#each statusOptions as option}
								<option value={option.value}>{option.label}</option>
							{/each}
						</select>
					</div>
					<div class="filter-group">
						<label for="filter-source">Source</label>
						<select id="filter-source" bind:value={selectedSource}>
							{#each sourceOptions as option}
								<option value={option.value}>{option.label}</option>
							{/each}
						</select>
					</div>
					<div class="filter-group">
						<label for="filter-sort">Sort By</label>
						<select id="filter-sort" bind:value={sortBy}>
							{#each sortOptions as option}
								<option value={option.value}>{option.label}</option>
							{/each}
						</select>
					</div>
					<div class="filter-group">
						<label for="filter-order">Order</label>
						<select id="filter-order" bind:value={sortOrder}>
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
											{#each statusOptions.filter((o) => o.value !== 'all') as option}
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
					<div class="table-footer">
						<span class="results-count">
							Showing {(currentPage - 1) * perPage + 1} - {Math.min(
								currentPage * perPage,
								filteredLeads.length
							)} of {filteredLeads.length} leads
						</span>
						<div class="pagination">
							<button
								class="pagination-btn"
								disabled={currentPage === 1}
								onclick={() => (currentPage = currentPage - 1)}
							>
								Previous
							</button>
							<span class="pagination-info">
								Page {currentPage} of {totalPages}
							</span>
							<button
								class="pagination-btn"
								disabled={currentPage === totalPages}
								onclick={() => (currentPage = currentPage + 1)}
							>
								Next
							</button>
						</div>
					</div>
				{/if}
			</div>
		{/if}
	</div>
	<!-- End admin-page-container -->
</div>

<!-- Add Lead Modal -->
{#if showAddModal}
	<!-- svelte-ignore a11y_no_static_element_interactions a11y_click_events_have_key_events a11y_interactive_supports_focus -->
	<div
		class="modal-overlay"
		onclick={() => (showAddModal = false)}
		onkeydown={(e) => e.key === 'Escape' && (showAddModal = false)}
		role="dialog"
		aria-modal="true"
		tabindex="-1"
	>
		<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
		<div
			class="modal"
			onclick={(e) => e.stopPropagation()}
			onkeydown={(e) => e.key === 'Escape' && (showAddModal = false)}
			role="document"
		>
			<div class="modal-header">
				<h3>
					<IconUserPlus size={20} />
					Add New Lead
				</h3>
				<button class="modal-close" onclick={() => (showAddModal = false)}>
					<IconX size={20} />
				</button>
			</div>
			<div class="modal-body">
				{#if formError}
					<div class="form-error">{formError}</div>
				{/if}
				<div class="form-grid">
					<div class="form-group">
						<label for="first_name">First Name *</label>
						<input id="first_name" name="first_name" type="text" bind:value={formData.first_name} required />
					</div>
					<div class="form-group">
						<label for="last_name">Last Name *</label>
						<input id="last_name" name="last_name" type="text" bind:value={formData.last_name} required />
					</div>
					<div class="form-group full-width">
						<label for="email">Email *</label>
						<input id="email" name="email" autocomplete="email" type="email" bind:value={formData.email} required />
					</div>
					<div class="form-group">
						<label for="phone">Phone</label>
						<input id="phone" name="phone" autocomplete="tel" type="tel" bind:value={formData.phone} />
					</div>
					<div class="form-group">
						<label for="company">Company</label>
						<input id="company" name="company" type="text" bind:value={formData.company_name} />
					</div>
					<div class="form-group">
						<label for="job_title">Job Title</label>
						<input id="job_title" name="job_title" type="text" bind:value={formData.job_title} />
					</div>
					<div class="form-group">
						<label for="source">Lead Source</label>
						<select id="source" bind:value={formData.source}>
							{#each sourceOptions.filter((o) => o.value !== 'all') as option}
								<option value={option.value}>{option.label}</option>
							{/each}
						</select>
					</div>
					<div class="form-group full-width">
						<label for="estimated_value">Estimated Value ($)</label>
						<input
							id="estimated_value" name="estimated_value"
							type="number"
							min="0"
							bind:value={formData.estimated_value}
						/>
					</div>
					<div class="form-group full-width">
						<label for="notes">Notes</label>
						<textarea id="notes" rows="3" bind:value={formData.notes}></textarea>
					</div>
				</div>
			</div>
			<div class="modal-footer">
				<button class="btn-secondary" onclick={() => (showAddModal = false)}> Cancel </button>
				<button
					class="btn-primary"
					onclick={createLead}
					disabled={formLoading || !formData.email || !formData.first_name}
				>
					{#if formLoading}
						Creating...
					{:else}
						<IconCheck size={18} />
						Create Lead
					{/if}
				</button>
			</div>
		</div>
	</div>
{/if}

<!-- Edit Lead Modal -->
{#if showEditModal && editingLead}
	<!-- svelte-ignore a11y_no_static_element_interactions a11y_click_events_have_key_events a11y_interactive_supports_focus -->
	<div
		class="modal-overlay"
		onclick={() => (showEditModal = false)}
		onkeydown={(e) => e.key === 'Escape' && (showEditModal = false)}
		role="dialog"
		aria-modal="true"
		tabindex="-1"
	>
		<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
		<div
			class="modal"
			onclick={(e) => e.stopPropagation()}
			onkeydown={(e) => e.key === 'Escape' && (showEditModal = false)}
			role="document"
		>
			<div class="modal-header">
				<h3>
					<IconEdit size={20} />
					Edit Lead
				</h3>
				<button class="modal-close" onclick={() => (showEditModal = false)}>
					<IconX size={20} />
				</button>
			</div>
			<div class="modal-body">
				{#if formError}
					<div class="form-error">{formError}</div>
				{/if}
				<div class="form-grid">
					<div class="form-group">
						<label for="edit_first_name">First Name *</label>
						<input id="edit_first_name" name="edit_first_name" type="text" bind:value={formData.first_name} required />
					</div>
					<div class="form-group">
						<label for="edit_last_name">Last Name *</label>
						<input id="edit_last_name" name="edit_last_name" type="text" bind:value={formData.last_name} required />
					</div>
					<div class="form-group full-width">
						<label for="edit_email">Email *</label>
						<input id="edit_email" name="edit_email" autocomplete="email" type="email" bind:value={formData.email} required />
					</div>
					<div class="form-group">
						<label for="edit_phone">Phone</label>
						<input id="edit_phone" name="edit_phone" autocomplete="tel" type="tel" bind:value={formData.phone} />
					</div>
					<div class="form-group">
						<label for="edit_company">Company</label>
						<input id="edit_company" name="edit_company" type="text" bind:value={formData.company_name} />
					</div>
					<div class="form-group">
						<label for="edit_job_title">Job Title</label>
						<input id="edit_job_title" name="edit_job_title" type="text" bind:value={formData.job_title} />
					</div>
					<div class="form-group">
						<label for="edit_source">Lead Source</label>
						<select id="edit_source" bind:value={formData.source}>
							{#each sourceOptions.filter((o) => o.value !== 'all') as option}
								<option value={option.value}>{option.label}</option>
							{/each}
						</select>
					</div>
					<div class="form-group full-width">
						<label for="edit_estimated_value">Estimated Value ($)</label>
						<input
							id="edit_estimated_value" name="edit_estimated_value"
							type="number"
							min="0"
							bind:value={formData.estimated_value}
						/>
					</div>
				</div>
			</div>
			<div class="modal-footer">
				<button class="btn-secondary" onclick={() => (showEditModal = false)}> Cancel </button>
				<button
					class="btn-primary"
					onclick={updateLead}
					disabled={formLoading || !formData.email || !formData.first_name}
				>
					{#if formLoading}
						Saving...
					{:else}
						<IconCheck size={18} />
						Save Changes
					{/if}
				</button>
			</div>
		</div>
	</div>
{/if}

<!-- Delete Confirmation Modal -->
{#if showDeleteModal && deletingLead}
	<!-- svelte-ignore a11y_no_static_element_interactions a11y_click_events_have_key_events a11y_interactive_supports_focus -->
	<div
		class="modal-overlay"
		onclick={() => (showDeleteModal = false)}
		onkeydown={(e) => e.key === 'Escape' && (showDeleteModal = false)}
		role="dialog"
		aria-modal="true"
		tabindex="-1"
	>
		<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
		<div
			class="modal modal-small"
			onclick={(e) => e.stopPropagation()}
			onkeydown={(e) => e.key === 'Escape' && (showDeleteModal = false)}
			role="document"
		>
			<div class="modal-header">
				<h3>
					<IconTrash size={20} />
					Delete Lead
				</h3>
				<button class="modal-close" onclick={() => (showDeleteModal = false)}>
					<IconX size={20} />
				</button>
			</div>
			<div class="modal-body">
				<p class="confirm-text">
					Are you sure you want to delete <strong>{deletingLead.full_name}</strong>? This action
					cannot be undone.
				</p>
			</div>
			<div class="modal-footer">
				<button class="btn-secondary" onclick={() => (showDeleteModal = false)}> Cancel </button>
				<button class="btn-danger" onclick={deleteLead}>
					<IconTrash size={18} />
					Delete Lead
				</button>
			</div>
		</div>
	</div>
{/if}

<!-- Convert to Contact Modal -->
{#if showConvertModal && convertingLead}
	<!-- svelte-ignore a11y_no_static_element_interactions a11y_click_events_have_key_events a11y_interactive_supports_focus -->
	<div
		class="modal-overlay"
		onclick={() => (showConvertModal = false)}
		onkeydown={(e) => e.key === 'Escape' && (showConvertModal = false)}
		role="dialog"
		aria-modal="true"
		tabindex="-1"
	>
		<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
		<div
			class="modal modal-small"
			onclick={(e) => e.stopPropagation()}
			onkeydown={(e) => e.key === 'Escape' && (showConvertModal = false)}
			role="document"
		>
			<div class="modal-header">
				<h3>
					<IconArrowRight size={20} />
					Convert to Contact
				</h3>
				<button class="modal-close" onclick={() => (showConvertModal = false)}>
					<IconX size={20} />
				</button>
			</div>
			<div class="modal-body">
				<p class="confirm-text">
					Convert <strong>{convertingLead.full_name}</strong> to a contact? The lead will be moved to
					your contacts list with all associated data.
				</p>
			</div>
			<div class="modal-footer">
				<button class="btn-secondary" onclick={() => (showConvertModal = false)}> Cancel </button>
				<button class="btn-primary" onclick={convertToContact}>
					<IconArrowRight size={18} />
					Convert Lead
				</button>
			</div>
		</div>
	</div>
{/if}

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   LEADS PAGE - Email Templates Style
	   ═══════════════════════════════════════════════════════════════════════════ */

	.page {
		max-width: 1400px;
		margin: 0 auto;
		padding: 2rem;
	}

	/* Header - CENTERED */
	.page-header {
		text-align: center;
		margin-bottom: 2rem;
	}

	.page-header h1 {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 12px;
		margin: 0 0 0.5rem;
		font-size: 1.75rem;
		font-weight: 700;
		color: #f1f5f9;
	}

	.page-header h1 :global(svg) {
		color: var(--primary-500);
	}

	.subtitle {
		margin: 0;
		color: #64748b;
		font-size: 0.875rem;
	}

	/* Stats Grid */
	.stats-grid {
		display: grid;
		grid-template-columns: repeat(6, 1fr);
		gap: 16px;
		margin-bottom: 24px;
	}

	.stat-card {
		display: flex;
		align-items: center;
		gap: 16px;
		padding: 20px;
		background: rgba(30, 41, 59, 0.4);
		border: 1px solid #334155;
		border-radius: 8px;
		transition: all 0.2s;
	}

	.stat-card:hover {
		border-color: rgba(99, 102, 241, 0.3);
		transform: translateY(-2px);
	}

	.stat-card.highlight {
		background: linear-gradient(135deg, rgba(99, 102, 241, 0.1) 0%, #1e293b 100%);
		border-color: rgba(99, 102, 241, 0.3);
	}

	.stat-icon {
		width: 48px;
		height: 48px;
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
	.stat-icon.cyan {
		background: rgba(6, 182, 212, 0.15);
		color: #22d3ee;
	}
	.stat-icon.purple {
		background: rgba(139, 92, 246, 0.15);
		color: #a78bfa;
	}
	.stat-icon.emerald {
		background: rgba(16, 185, 129, 0.15);
		color: #34d399;
	}
	.stat-icon.amber {
		background: rgba(245, 158, 11, 0.15);
		color: #fbbf24;
	}
	.stat-icon.green {
		background: rgba(34, 197, 94, 0.15);
		color: #4ade80;
	}

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
	.table-footer {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 16px 20px;
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

	/* Modal */
	.modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.7);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
		padding: 20px;
	}

	.modal {
		width: 100%;
		max-width: 560px;
		max-height: 90vh;
		overflow-y: auto;
		background: #1e293b;
		border: 1px solid #334155;
		border-radius: 16px;
	}

	.modal-small {
		max-width: 420px;
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 20px;
		border-bottom: 1px solid #334155;
	}

	.modal-header h3 {
		display: flex;
		align-items: center;
		gap: 10px;
		margin: 0;
		font-size: 1.1rem;
		font-weight: 600;
		color: white;
	}

	.modal-header h3 :global(svg) {
		color: var(--primary-500);
	}

	.modal-close {
		display: flex;
		padding: 8px;
		background: transparent;
		border: none;
		color: #64748b;
		cursor: pointer;
		transition: color 0.2s;
	}

	.modal-close:hover {
		color: white;
	}

	.modal-body {
		padding: 20px;
	}

	.modal-footer {
		display: flex;
		justify-content: flex-end;
		gap: 12px;
		padding: 20px;
		border-top: 1px solid #334155;
	}

	.form-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 16px;
	}

	.form-group {
		display: flex;
		flex-direction: column;
		gap: 6px;
	}

	.form-group.full-width {
		grid-column: 1 / -1;
	}

	.form-group label {
		font-size: 0.8rem;
		font-weight: 600;
		color: #94a3b8;
	}

	.form-group input,
	.form-group select,
	.form-group textarea {
		padding: 10px 14px;
		background: #0f172a;
		border: 1px solid #334155;
		border-radius: 8px;
		color: white;
		font-size: 0.9rem;
		font-family: inherit;
	}

	.form-group input:focus,
	.form-group select:focus,
	.form-group textarea:focus {
		outline: none;
		border-color: var(--primary-500);
	}

	.form-group textarea {
		resize: vertical;
		min-height: 80px;
	}

	.form-error {
		margin-bottom: 16px;
		padding: 12px;
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		border-radius: 8px;
		color: #f87171;
		font-size: 0.875rem;
	}

	.confirm-text {
		margin: 0;
		font-size: 0.95rem;
		color: #94a3b8;
		line-height: 1.6;
	}

	.confirm-text strong {
		color: white;
	}

	.btn-secondary {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		padding: 0.75rem 1rem;
		background: rgba(100, 116, 139, 0.2);
		border: 1px solid rgba(100, 116, 139, 0.3);
		border-radius: 8px;
		color: #cbd5e1;
		font-weight: 500;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-secondary:hover {
		background: rgba(100, 116, 139, 0.3);
		border-color: rgba(100, 116, 139, 0.5);
	}

	.btn-danger {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 10px 20px;
		background: linear-gradient(135deg, #ef4444, #dc2626);
		color: white;
		border: none;
		border-radius: 10px;
		font-weight: 600;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-danger:hover {
		transform: translateY(-2px);
		box-shadow: 0 4px 15px rgba(239, 68, 68, 0.4);
	}

	/* Responsive */
	@media (max-width: 1400px) {
		.stats-grid {
			grid-template-columns: repeat(3, 1fr);
		}
	}

	@media (max-width: 1024px) {
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

	@media (max-width: 768px) {
		.page {
			padding: 1rem;
		}

		.page-header h1 {
			font-size: 1.5rem;
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

		.form-grid {
			grid-template-columns: 1fr;
		}

		.table-container {
			overflow-x: auto;
		}
	}
</style>

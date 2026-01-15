<!--
	/admin/crm/companies - B2B Company Management
	Apple Principal Engineer ICT 7 Grade - January 2026
	
	Features:
	- Company CRUD with logo support
	- Industry and size filtering
	- Contact and deal tracking per company
	- Total deal value aggregation
	- Full Svelte 5 $state/$derived reactivity
-->

<script lang="ts">
	import { onMount } from 'svelte';
	import {
		IconBuilding,
		IconPlus,
		IconSearch,
		IconEdit,
		IconTrash,
		IconEye,
		IconRefresh,
		IconUsers,
		IconCurrencyDollar,
		IconBriefcase,
		IconWorld,
		IconCopy
	} from '$lib/icons';
	import { crmAPI } from '$lib/api/crm';
	import { toastStore } from '$lib/stores/toast';
	import type { CrmCompany, CompanyFilters, CompanyIndustry, CompanySize } from '$lib/crm/types';

	let companies = $state<CrmCompany[]>([]);
	let isLoading = $state(true);
	let error = $state('');
	let searchQuery = $state('');
	let selectedIndustry = $state<CompanyIndustry | 'all'>('all');
	let selectedSize = $state<CompanySize | 'all'>('all');

	let stats = $state({
		total: 0,
		totalContacts: 0,
		totalDeals: 0,
		totalDealValue: 0
	});

	const industryOptions = [
		{ value: 'all', label: 'All Industries' },
		{ value: 'technology', label: 'Technology' },
		{ value: 'finance', label: 'Finance' },
		{ value: 'healthcare', label: 'Healthcare' },
		{ value: 'education', label: 'Education' },
		{ value: 'retail', label: 'Retail' },
		{ value: 'manufacturing', label: 'Manufacturing' },
		{ value: 'consulting', label: 'Consulting' },
		{ value: 'other', label: 'Other' }
	];

	const sizeOptions = [
		{ value: 'all', label: 'All Sizes' },
		{ value: '1-10', label: '1-10 employees' },
		{ value: '11-50', label: '11-50 employees' },
		{ value: '51-200', label: '51-200 employees' },
		{ value: '201-500', label: '201-500 employees' },
		{ value: '501-1000', label: '501-1000 employees' },
		{ value: '1001-5000', label: '1001-5000 employees' },
		{ value: '5001+', label: '5001+ employees' }
	];

	async function loadCompanies() {
		isLoading = true;
		error = '';

		try {
			const filters: CompanyFilters = {
				search: searchQuery || undefined,
				industry: selectedIndustry !== 'all' ? selectedIndustry : undefined,
				size: selectedSize !== 'all' ? selectedSize : undefined
			};

			const response = await crmAPI.getCompanies(filters);
			companies = response.data || [];

			stats = {
				total: companies.length,
				totalContacts: companies.reduce((sum, c) => sum + c.contacts_count, 0),
				totalDeals: companies.reduce((sum, c) => sum + c.deals_count, 0),
				totalDealValue: companies.reduce((sum, c) => sum + c.total_deal_value, 0)
			};
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load companies';
		} finally {
			isLoading = false;
		}
	}

	async function deleteCompany(id: string) {
		if (!confirm('Are you sure you want to delete this company? Associated contacts will NOT be deleted.')) return;

		try {
			await crmAPI.deleteCompany(id);
			await loadCompanies();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to delete company';
		}
	}

	async function duplicateCompany(company: CrmCompany) {
		try {
			await crmAPI.duplicateCompany(company.id);
			toastStore.success(`Company "${company.name}" duplicated successfully`);
			await loadCompanies();
		} catch (err) {
			const message = err instanceof Error ? err.message : 'Failed to duplicate company';
			toastStore.error(message);
			error = message;
		}
	}

	function formatNumber(num: number): string {
		return num.toLocaleString();
	}

	function formatCurrency(amount: number): string {
		return new Intl.NumberFormat('en-US', {
			style: 'currency',
			currency: 'USD',
			minimumFractionDigits: 0,
			maximumFractionDigits: 0
		}).format(amount);
	}

	let filteredCompanies = $derived(
		companies.filter(company => {
			const matchesSearch = !searchQuery ||
				company.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
				company.website?.toLowerCase().includes(searchQuery.toLowerCase());
			const matchesIndustry = selectedIndustry === 'all' || company.industry === selectedIndustry;
			const matchesSize = selectedSize === 'all' || company.size === selectedSize;
			return matchesSearch && matchesIndustry && matchesSize;
		})
	);

	onMount(() => {
		loadCompanies();
	});
</script>

<svelte:head>
	<title>CRM Companies - FluentCRM Pro</title>
</svelte:head>

<div class="page">
	<!-- Page Header - CENTERED -->
	<header class="page-header">
		<h1>CRM Companies</h1>
		<p class="subtitle">Manage B2B company accounts and their contacts</p>
		<div class="header-actions">
			<button class="btn-secondary" onclick={() => loadCompanies()} disabled={isLoading}>
				<IconRefresh size={18} class={isLoading ? 'spinning' : ''} />
				Refresh
			</button>
			<a href="/admin/crm/companies/new" class="btn-primary">
				<IconPlus size={18} />
				New Company
			</a>
		</div>
	</header>

	<!-- Stats Cards -->
	<div class="stats-grid">
		<div class="stat-card">
			<div class="stat-icon blue">
				<IconBuilding size={24} />
			</div>
			<div class="stat-content">
				<span class="stat-value">{formatNumber(stats.total)}</span>
				<span class="stat-label">Total Companies</span>
			</div>
		</div>
		<div class="stat-card">
			<div class="stat-icon green">
				<IconUsers size={24} />
			</div>
			<div class="stat-content">
				<span class="stat-value">{formatNumber(stats.totalContacts)}</span>
				<span class="stat-label">Total Contacts</span>
			</div>
		</div>
		<div class="stat-card">
			<div class="stat-icon purple">
				<IconBriefcase size={24} />
			</div>
			<div class="stat-content">
				<span class="stat-value">{formatNumber(stats.totalDeals)}</span>
				<span class="stat-label">Total Deals</span>
			</div>
		</div>
		<div class="stat-card">
			<div class="stat-icon amber">
				<IconCurrencyDollar size={24} />
			</div>
			<div class="stat-content">
				<span class="stat-value">{formatCurrency(stats.totalDealValue)}</span>
				<span class="stat-label">Total Deal Value</span>
			</div>
		</div>
	</div>

	<!-- Search & Filters -->
	<div class="filters-bar">
		<div class="search-box">
			<IconSearch size={18} />
			<input type="text" placeholder="Search companies..." bind:value={searchQuery} />
		</div>
		<select class="filter-select" bind:value={selectedIndustry}>
			{#each industryOptions as option}
				<option value={option.value}>{option.label}</option>
			{/each}
		</select>
		<select class="filter-select" bind:value={selectedSize}>
			{#each sizeOptions as option}
				<option value={option.value}>{option.label}</option>
			{/each}
		</select>
	</div>

	<!-- Companies Table -->
	{#if isLoading}
		<div class="loading-state">
			<div class="spinner"></div>
			<p>Loading companies...</p>
		</div>
	{:else if error}
		<div class="error-state">
			<p>{error}</p>
			<button onclick={() => loadCompanies()}>Try Again</button>
		</div>
	{:else if filteredCompanies.length === 0}
		<div class="empty-state">
			<IconBuilding size={48} />
			<h3>No companies found</h3>
			<p>Add your first company to start managing B2B accounts</p>
			<a href="/admin/crm/companies/new" class="btn-primary">
				<IconPlus size={18} />
				Add Company
			</a>
		</div>
	{:else}
		<div class="table-container">
			<table class="data-table">
				<thead>
					<tr>
						<th>Company</th>
						<th>Industry</th>
						<th>Size</th>
						<th>Contacts</th>
						<th>Deals</th>
						<th>Deal Value</th>
						<th>Actions</th>
					</tr>
				</thead>
				<tbody>
					{#each filteredCompanies as company}
						<tr>
							<td>
								<div class="company-cell">
									<div class="company-icon">
										{#if company.logo_url}
											<img src={company.logo_url} alt={company.name} />
										{:else}
											<IconBuilding size={20} />
										{/if}
									</div>
									<div class="company-info">
										<span class="company-name">{company.name}</span>
										{#if company.website}
											<a href={company.website} target="_blank" class="company-website">
												<IconWorld size={12} />
												{company.website.replace(/^https?:\/\//, '')}
											</a>
										{/if}
									</div>
								</div>
							</td>
							<td>
								<span class="industry-badge">
									{company.industry || '-'}
								</span>
							</td>
							<td>{company.size || '-'}</td>
							<td>{formatNumber(company.contacts_count)}</td>
							<td>{formatNumber(company.deals_count)}</td>
							<td>{formatCurrency(company.total_deal_value)}</td>
							<td>
								<div class="action-buttons">
									<a href="/admin/crm/companies/{company.id}" class="btn-icon" title="View">
										<IconEye size={16} />
									</a>
									<a href="/admin/crm/companies/{company.id}/edit" class="btn-icon" title="Edit">
										<IconEdit size={16} />
									</a>
									<button class="btn-icon" title="Duplicate" onclick={() => duplicateCompany(company)}>
										<IconCopy size={16} />
									</button>
									<button class="btn-icon danger" title="Delete" onclick={() => deleteCompany(company.id)}>
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
		background: linear-gradient(135deg, #6366f1, #8b5cf6);
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
		from { transform: rotate(0deg); }
		to { transform: rotate(360deg); }
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

	.company-cell {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.company-icon {
		width: 40px;
		height: 40px;
		border-radius: 10px;
		background: linear-gradient(135deg, #6366f1, #8b5cf6);
		display: flex;
		align-items: center;
		justify-content: center;
		color: white;
		overflow: hidden;
	}

	.company-icon img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.company-info {
		display: flex;
		flex-direction: column;
		gap: 0.125rem;
	}

	.company-name {
		font-weight: 600;
		color: #f1f5f9;
	}

	.company-website {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		font-size: 0.75rem;
		color: #818cf8;
		text-decoration: none;
	}

	.company-website:hover {
		text-decoration: underline;
	}

	.industry-badge {
		display: inline-block;
		padding: 0.25rem 0.5rem;
		background: rgba(99, 102, 241, 0.1);
		border-radius: 4px;
		font-size: 0.75rem;
		color: #818cf8;
		text-transform: capitalize;
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
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 8px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
		text-decoration: none;
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

	/* =====================================================
	   States
	   ===================================================== */
	.loading-state, .error-state, .empty-state {
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

	.error-state h3,
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
		border-top-color: #6366f1;
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin-bottom: 1rem;
	}

	/* =====================================================
	   Responsive
	   ===================================================== */
	@media (max-width: 1200px) {
		.stats-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	@media (max-width: 768px) {
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
	}
</style>

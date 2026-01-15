<script lang="ts">
	/**
	 * Admin Indicator List Page
	 * Apple Principal Engineer ICT 7 Grade - January 2026
	 */

	import { onMount } from 'svelte';
	import { adminFetch } from '$lib/utils/adminFetch';

	// ICT 7 FIX: Match actual backend schema (admin_indicators.rs)
	interface IndicatorListItem {
		id: number; // Backend uses i64, not string
		name: string;
		slug: string;
		description?: string; // Backend uses 'description' not 'tagline'
		price?: number; // Backend uses price (float dollars), not price_cents
		is_active?: boolean; // Backend uses 'is_active' not 'status'
		thumbnail?: string; // Backend uses 'thumbnail' not 'logo_url'
		platform?: string;
		version?: string;
		created_at?: string;
	}

	let indicators = $state<IndicatorListItem[]>([]);
	let loading = $state(true);
	let total = $state(0);
	let page = $state(1);
	let perPage = $state(20);
	let search = $state('');
	let statusFilter = $state('');
	let deleting = $state<string | null>(null);

	const fetchIndicators = async () => {
		loading = true;
		try {
			const params = new URLSearchParams({
				page: page.toString(),
				per_page: perPage.toString()
			});
			if (search) params.set('search', search);
			// ICT 7 FIX: Backend uses 'is_active' not 'status'
			if (statusFilter) params.set('is_active', statusFilter);

			// ICT 11+ FIX: Use adminFetch for absolute URL on Pages.dev
			const data = await adminFetch(`/api/admin/indicators?${params}`);
			if (data.success) {
				indicators = data.data.indicators;
				total = data.data.total;
			}
		} catch (e) {
			console.error('Failed to fetch indicators:', e);
		} finally {
			loading = false;
		}
	};

	// ICT 7 FIX: id is number, not string
	const deleteIndicator = async (id: number, name: string) => {
		if (!confirm(`Delete "${name}"? This cannot be undone.`)) return;
		deleting = id.toString();
		try {
			// ICT 11+ FIX: Use adminFetch for absolute URL on Pages.dev
			const data = await adminFetch(`/api/admin/indicators/${id}`, { method: 'DELETE' });
			if (data.success) {
				indicators = indicators.filter(i => i.id !== id);
				total--;
			}
		} catch (e) {
			console.error('Failed to delete:', e);
		} finally {
			deleting = null;
		}
	};

	// ICT 7 FIX: Backend uses price in dollars, not cents
	const formatPrice = (price?: number) => {
		if (!price || price === 0) return 'Free';
		return `$${price.toFixed(2)}`;
	};

	const formatDate = (date?: string) => {
		if (!date) return '-';
		return new Date(date).toLocaleDateString();
	};

	onMount(() => {
		fetchIndicators();
	});

	$effect(() => {
		if (search !== undefined || statusFilter !== undefined) {
			page = 1;
		}
	});
</script>

<svelte:head>
	<title>Indicators | Admin</title>
</svelte:head>

<div class="admin-page">
	<header class="page-header">
		<div class="header-left">
			<h1>Indicators</h1>
			<span class="count">{total} total</span>
		</div>
		<a href="/admin/indicators/create" class="btn-primary">
			<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
				<line x1="12" x2="12" y1="5" y2="19"/><line x1="5" x2="19" y1="12" y2="12"/>
			</svg>
			New Indicator
		</a>
	</header>

	<div class="filters">
		<div class="search-box">
			<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
				<circle cx="11" cy="11" r="8"/><path d="m21 21-4.3-4.3"/>
			</svg>
			<input
				type="text"
				placeholder="Search indicators..."
				bind:value={search}
				onkeyup={(e) => e.key === 'Enter' && fetchIndicators()}
			/>
		</div>
			<!-- ICT 7 FIX: Backend uses is_active boolean, not status string -->
			<select bind:value={statusFilter} onchange={fetchIndicators}>
			<option value="">All Status</option>
			<option value="true">Active</option>
			<option value="false">Inactive</option>
		</select>
		<button class="btn-secondary" onclick={fetchIndicators}>
			<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
				<path d="M21 12a9 9 0 0 0-9-9 9.75 9.75 0 0 0-6.74 2.74L3 8"/><path d="M3 3v5h5"/><path d="M3 12a9 9 0 0 0 9 9 9.75 9.75 0 0 0 6.74-2.74L21 16"/><path d="M16 16h5v5"/>
			</svg>
			Refresh
		</button>
	</div>

	{#if loading}
		<div class="loading">
			<div class="spinner"></div>
			<p>Loading indicators...</p>
		</div>
	{:else if indicators.length === 0}
		<div class="empty-state">
			<svg xmlns="http://www.w3.org/2000/svg" width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1">
				<path d="M3 3v18h18"/><path d="m19 9-5 5-4-4-3 3"/>
			</svg>
			<h3>No indicators yet</h3>
			<p>Create your first trading indicator to get started.</p>
			<a href="/admin/indicators/create" class="btn-primary">Create Indicator</a>
		</div>
	{:else}
		<div class="table-container">
			<table>
				<thead>
					<tr>
						<th>Indicator</th>
						<th>Price</th>
						<th>Status</th>
						<th>Platform</th>
						<th>Created</th>
						<th>Actions</th>
					</tr>
				</thead>
				<tbody>
					{#each indicators as indicator}
						<tr>
							<td class="indicator-cell">
								{#if indicator.thumbnail}
									<img src={indicator.thumbnail} alt="" class="logo" />
								{:else}
									<div class="logo-placeholder">
										<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
											<path d="M3 3v18h18"/><path d="m19 9-5 5-4-4-3 3"/>
										</svg>
									</div>
								{/if}
								<div class="indicator-info">
									<a href="/admin/indicators/{indicator.id}" class="name">{indicator.name}</a>
									{#if indicator.description}
										<span class="tagline">{indicator.description.slice(0, 60)}{indicator.description.length > 60 ? '...' : ''}</span>
									{/if}
								</div>
							</td>
							<td class="price">{formatPrice(indicator.price)}</td>
							<td>
								<span class="status" class:status--published={indicator.is_active} class:status--draft={!indicator.is_active}>
									{indicator.is_active ? 'Active' : 'Inactive'}
								</span>
							</td>
							<td class="platform">{indicator.platform || '-'}</td>
							<td class="date">{formatDate(indicator.created_at)}</td>
							<td class="actions">
								<a href="/admin/indicators/{indicator.id}" class="btn-icon" title="Edit" aria-label="Edit {indicator.name}">
									<svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
										<path d="M17 3a2.85 2.83 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5Z"/>
									</svg>
								</a>
								<a href="/indicators/{indicator.slug}" target="_blank" class="btn-icon" title="View" aria-label="View {indicator.name}">
									<svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
										<path d="M2 12s3-7 10-7 10 7 10 7-3 7-10 7-10-7-10-7Z"/><circle cx="12" cy="12" r="3"/>
									</svg>
								</a>
								<button
									class="btn-icon btn-danger"
									onclick={() => deleteIndicator(indicator.id, indicator.name)}
									disabled={deleting === indicator.id.toString()}
									title="Delete"
									aria-label="Delete {indicator.name}"
								>
									{#if deleting === indicator.id.toString()}
										<div class="spinner-small"></div>
									{:else}
										<svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
											<path d="M3 6h18"/><path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6"/><path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2"/>
										</svg>
									{/if}
								</button>
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>

		{#if total > perPage}
			<div class="pagination">
				<button
					class="btn-page"
					disabled={page === 1}
					onclick={() => { page--; fetchIndicators(); }}
					aria-label="Previous page"
				>
					← Previous
				</button>
				<span class="page-info">Page {page} of {Math.ceil(total / perPage)}</span>
				<button
					class="btn-page"
					disabled={page >= Math.ceil(total / perPage)}
					onclick={() => { page++; fetchIndicators(); }}
					aria-label="Next page"
				>
					Next →
				</button>
			</div>
		{/if}
	{/if}
</div>

<style>
	.admin-page { padding: 24px; max-width: 1400px; margin: 0 auto; }

	.page-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 24px; }
	.header-left { display: flex; align-items: center; gap: 12px; }
	.header-left h1 { font-size: 24px; font-weight: 600; color: #1f2937; margin: 0; }
	.count { font-size: 14px; color: #6b7280; background: #f3f4f6; padding: 4px 12px; border-radius: 20px; }

	/* Buttons - RTP Admin Color System */
	.btn-primary { display: inline-flex; align-items: center; gap: 8px; padding: 10px 20px; background: var(--admin-btn-primary-bg); color: var(--admin-btn-primary-text); border: none; border-radius: 8px; font-size: 14px; font-weight: 500; text-decoration: none; cursor: pointer; transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1); box-shadow: 0 2px 8px rgba(230, 184, 0, 0.25); }
	.btn-primary:hover { transform: translateY(-1px); box-shadow: 0 4px 16px rgba(230, 184, 0, 0.4); }

	.btn-secondary { display: inline-flex; align-items: center; gap: 6px; padding: 8px 16px; background: var(--admin-btn-bg); color: var(--admin-text-secondary); border: 1px solid var(--admin-border); border-radius: 6px; font-size: 14px; cursor: pointer; transition: all 0.2s; }
	.btn-secondary:hover { background: var(--admin-btn-bg-hover); border-color: var(--admin-border-interactive); color: var(--admin-text-primary); }

	.filters { display: flex; gap: 12px; margin-bottom: 20px; flex-wrap: wrap; }
	.search-box { display: flex; align-items: center; gap: 8px; padding: 8px 12px; background: #fff; border: 1px solid #e5e7eb; border-radius: 8px; flex: 1; min-width: 200px; max-width: 400px; }
	.search-box svg { color: #9ca3af; flex-shrink: 0; }
	.search-box input { border: none; outline: none; flex: 1; font-size: 14px; }

	.filters select { padding: 8px 12px; border: 1px solid #e5e7eb; border-radius: 6px; font-size: 14px; background: #fff; }

	.loading, .empty-state { display: flex; flex-direction: column; align-items: center; justify-content: center; padding: 64px; text-align: center; }
	.empty-state svg { color: #d1d5db; margin-bottom: 16px; }
	.empty-state h3 { font-size: 18px; color: #1f2937; margin: 0 0 8px; }
	.empty-state p { color: #6b7280; margin: 0 0 20px; }

	.spinner { width: 40px; height: 40px; border: 3px solid #e5e7eb; border-top-color: #143e59; border-radius: 50%; animation: spin 1s linear infinite; margin-bottom: 16px; }
	.spinner-small { width: 16px; height: 16px; border: 2px solid #e5e7eb; border-top-color: #143e59; border-radius: 50%; animation: spin 1s linear infinite; }
	@keyframes spin { to { transform: rotate(360deg); } }

	.table-container { background: #fff; border-radius: 12px; border: 1px solid #e5e7eb; overflow: hidden; }
	table { width: 100%; border-collapse: collapse; }
	thead { background: #f9fafb; }
	th { padding: 12px 16px; text-align: left; font-size: 12px; font-weight: 600; color: #6b7280; text-transform: uppercase; letter-spacing: 0.05em; }
	td { padding: 16px; border-top: 1px solid #f3f4f6; vertical-align: middle; }
	tbody tr:hover { background: #f9fafb; }

	.indicator-cell { display: flex; align-items: center; gap: 12px; }
	.logo { width: 48px; height: 48px; border-radius: 8px; object-fit: cover; }
	.logo-placeholder { width: 48px; height: 48px; background: #f3f4f6; border-radius: 8px; display: flex; align-items: center; justify-content: center; color: #9ca3af; }
	.indicator-info { display: flex; flex-direction: column; gap: 2px; }
	.name { font-weight: 500; color: #1f2937; text-decoration: none; }
	.name:hover { color: #143e59; }
	.tagline { font-size: 13px; color: #6b7280; }

	.price { font-weight: 600; color: #143e59; }
	.downloads { color: #6b7280; }
	.date { color: #9ca3af; font-size: 13px; }

	.status { display: inline-block; padding: 4px 10px; border-radius: 20px; font-size: 12px; font-weight: 500; text-transform: capitalize; }
	.status--draft { background: #fef3c7; color: #92400e; }
	.status--published { background: #d1fae5; color: #065f46; }
	.status--archived { background: #f3f4f6; color: #6b7280; }

	.actions { display: flex; gap: 8px; }
	.btn-icon { display: flex; align-items: center; justify-content: center; width: 32px; height: 32px; border: none; background: #f3f4f6; border-radius: 6px; color: #6b7280; cursor: pointer; text-decoration: none; transition: all 0.2s; }
	.btn-icon:hover { background: #e5e7eb; color: #1f2937; }
	.btn-danger:hover { background: #fee2e2; color: #dc2626; }
	.btn-icon:disabled { opacity: 0.5; cursor: not-allowed; }

	.pagination { display: flex; justify-content: center; align-items: center; gap: 16px; margin-top: 24px; }
	.btn-page { padding: 8px 16px; background: #fff; border: 1px solid #e5e7eb; border-radius: 6px; font-size: 14px; cursor: pointer; }
	.btn-page:hover:not(:disabled) { background: #f3f4f6; }
	.btn-page:disabled { opacity: 0.5; cursor: not-allowed; }
	.page-info { font-size: 14px; color: #6b7280; }
</style>

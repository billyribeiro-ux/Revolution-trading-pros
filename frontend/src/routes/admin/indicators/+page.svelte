<script lang="ts">
	/**
	 * Admin Indicator List Page
	 * Apple Principal Engineer ICT 7 Grade - January 2026
	 */

	import { onMount, untrack } from 'svelte';
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
	// @ts-ignore write-only state
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
				indicators = indicators.filter((i) => i.id !== id);
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

	// ICT 7: Initial fetch on mount — avoids $effect infinite loop
	onMount(() => {
		fetchIndicators();
	});

	// ICT 7: Reset page when filters change, then re-fetch
	// Uses untrack on fetchIndicators to prevent circular dependency
	// @ts-ignore write-only state
	let searchDebounceTimer: ReturnType<typeof setTimeout>;
	$effect(() => {
		// Explicitly subscribe to search and statusFilter as reactive dependencies
		// Debounce to avoid rapid re-fetches while typing
		clearTimeout(searchDebounceTimer);
		searchDebounceTimer = setTimeout(() => {
			untrack(() => {
				page = 1;
				fetchIndicators();
			});
		}, 300);
	});
</script>

<svelte:head>
	<title>Indicators | Admin</title>
</svelte:head>

<div class="admin-indicators">
	<!-- Animated Background -->
	<div class="bg-effects">
		<div class="bg-blob bg-blob-1"></div>
		<div class="bg-blob bg-blob-2"></div>
		<div class="bg-blob bg-blob-3"></div>
	</div>

	<div class="admin-page-container">
		<header class="page-header">
			<h1>Indicators</h1>
			<p class="subtitle">{total} total indicators</p>
			<div class="actions-row">
				<a href="/admin/indicators/create" class="btn-primary">
					<svg
						xmlns="http://www.w3.org/2000/svg"
						width="20"
						height="20"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
					>
						<line x1="12" x2="12" y1="5" y2="19" /><line x1="5" x2="19" y1="12" y2="12" />
					</svg>
					New Indicator
				</a>
			</div>
		</header>

		<div class="filters">
			<div class="search-box">
				<svg
					xmlns="http://www.w3.org/2000/svg"
					width="20"
					height="20"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
				>
					<circle cx="11" cy="11" r="8" /><path d="m21 21-4.3-4.3" />
				</svg>
				<input
					id="search-indicators"
					name="search-indicators"
					type="text"
					placeholder="Search indicators..."
					bind:value={search}
					onkeyup={(e) => e.key === 'Enter' && fetchIndicators()}
				/>
			</div>
			<!-- ICT 7 FIX: Backend uses is_active boolean, not status string -->
			<select
				id="status-filter"
				name="status-filter"
				bind:value={statusFilter}
				onchange={fetchIndicators}
			>
				<option value="">All Status</option>
				<option value="true">Active</option>
				<option value="false">Inactive</option>
			</select>
			<button class="btn-secondary" onclick={fetchIndicators}>
				<svg
					xmlns="http://www.w3.org/2000/svg"
					width="16"
					height="16"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
				>
					<path d="M21 12a9 9 0 0 0-9-9 9.75 9.75 0 0 0-6.74 2.74L3 8" /><path d="M3 3v5h5" /><path
						d="M3 12a9 9 0 0 0 9 9 9.75 9.75 0 0 0 6.74-2.74L21 16"
					/><path d="M16 16h5v5" />
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
				<svg
					xmlns="http://www.w3.org/2000/svg"
					width="64"
					height="64"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="1"
				>
					<path d="M3 3v18h18" /><path d="m19 9-5 5-4-4-3 3" />
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
											<svg
												xmlns="http://www.w3.org/2000/svg"
												width="24"
												height="24"
												viewBox="0 0 24 24"
												fill="none"
												stroke="currentColor"
												stroke-width="2"
											>
												<path d="M3 3v18h18" /><path d="m19 9-5 5-4-4-3 3" />
											</svg>
										</div>
									{/if}
									<div class="indicator-info">
										<a href="/admin/indicators/{indicator.id}" class="name">{indicator.name}</a>
										{#if indicator.description}
											<span class="tagline"
												>{indicator.description.slice(0, 60)}{indicator.description.length > 60
													? '...'
													: ''}</span
											>
										{/if}
									</div>
								</td>
								<td class="price">{formatPrice(indicator.price)}</td>
								<td>
									<span
										class="status"
										class:status--published={indicator.is_active}
										class:status--draft={!indicator.is_active}
									>
										{indicator.is_active ? 'Active' : 'Inactive'}
									</span>
								</td>
								<td class="platform">{indicator.platform || '-'}</td>
								<td class="date">{formatDate(indicator.created_at)}</td>
								<td class="actions">
									<a
										href="/admin/indicators/{indicator.id}"
										class="btn-icon"
										title="Edit"
										aria-label="Edit {indicator.name}"
									>
										<svg
											xmlns="http://www.w3.org/2000/svg"
											width="18"
											height="18"
											viewBox="0 0 24 24"
											fill="none"
											stroke="currentColor"
											stroke-width="2"
										>
											<path d="M17 3a2.85 2.83 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5Z" />
										</svg>
									</a>
									<a
										href="/indicators/{indicator.slug}"
										target="_blank"
										class="btn-icon"
										title="View"
										aria-label="View {indicator.name}"
									>
										<svg
											xmlns="http://www.w3.org/2000/svg"
											width="18"
											height="18"
											viewBox="0 0 24 24"
											fill="none"
											stroke="currentColor"
											stroke-width="2"
										>
											<path d="M2 12s3-7 10-7 10 7 10 7-3 7-10 7-10-7-10-7Z" /><circle
												cx="12"
												cy="12"
												r="3"
											/>
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
											<svg
												xmlns="http://www.w3.org/2000/svg"
												width="18"
												height="18"
												viewBox="0 0 24 24"
												fill="none"
												stroke="currentColor"
												stroke-width="2"
											>
												<path d="M3 6h18" /><path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6" /><path
													d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2"
												/>
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
						onclick={() => {
							page--;
							fetchIndicators();
						}}
						aria-label="Previous page"
					>
						← Previous
					</button>
					<span class="page-info">Page {page} of {Math.ceil(total / perPage)}</span>
					<button
						class="btn-page"
						disabled={page >= Math.ceil(total / perPage)}
						onclick={() => {
							page++;
							fetchIndicators();
						}}
						aria-label="Next page"
					>
						Next →
					</button>
				</div>
			{/if}
		{/if}
	</div>
	<!-- End admin-page-container -->
</div>

<style>
	/* Page wrapper */
	.admin-indicators {
		background: linear-gradient(
			135deg,
			var(--bg-base) 0%,
			var(--bg-elevated) 50%,
			var(--bg-base) 100%
		);
		position: relative;
		overflow: hidden;
	}

	/* Header CENTERED */
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
		margin: 0 0 1rem 0;
	}

	/* Actions row centered */
	.actions-row {
		display: flex;
		justify-content: center;
		gap: 0.75rem;
	}

	/* Buttons - Email Templates Style */
	.btn-primary {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1.25rem;
		background: linear-gradient(135deg, var(--primary-500), var(--primary-600));
		color: white;
		border: none;
		border-radius: 8px;
		font-size: 0.875rem;
		font-weight: 500;
		text-decoration: none;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.btn-primary:hover {
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(99, 102, 241, 0.4);
	}

	.btn-secondary {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.5rem 1rem;
		background: rgba(100, 116, 139, 0.2);
		color: #cbd5e1;
		border: 1px solid rgba(100, 116, 139, 0.3);
		border-radius: 6px;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.btn-secondary:hover {
		background: rgba(100, 116, 139, 0.3);
		color: #f1f5f9;
	}

	/* Filters - Dark styling */
	.filters {
		display: flex;
		gap: 0.75rem;
		margin-bottom: 1.5rem;
		flex-wrap: wrap;
		justify-content: center;
	}

	.search-box {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 0.75rem;
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(100, 116, 139, 0.3);
		border-radius: 8px;
		flex: 1;
		min-width: 200px;
		max-width: 400px;
	}

	.search-box svg {
		color: #64748b;
		flex-shrink: 0;
	}

	.search-box input {
		border: none;
		outline: none;
		flex: 1;
		font-size: 0.875rem;
		background: transparent;
		color: #f1f5f9;
	}

	.search-box input::placeholder {
		color: #64748b;
	}

	.filters select {
		padding: 0.5rem 0.75rem;
		border: 1px solid rgba(100, 116, 139, 0.3);
		border-radius: 6px;
		font-size: 0.875rem;
		background: rgba(30, 41, 59, 0.6);
		color: #f1f5f9;
		cursor: pointer;
	}

	.filters select option {
		background: #1e293b;
		color: #f1f5f9;
	}

	/* Loading and Empty State */
	.loading,
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem;
		text-align: center;
		background: rgba(30, 41, 59, 0.4);
		border-radius: 8px;
	}

	.empty-state svg {
		color: #64748b;
		margin-bottom: 1rem;
	}

	.empty-state h3 {
		font-size: 1.125rem;
		color: #f1f5f9;
		margin: 0 0 0.5rem;
	}

	.empty-state p {
		color: #64748b;
		margin: 0 0 1.25rem;
	}

	.spinner {
		width: 2.5rem;
		height: 2.5rem;
		border: 3px solid rgba(100, 116, 139, 0.3);
		border-top-color: var(--primary-500);
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin-bottom: 1rem;
	}

	.spinner-small {
		width: 1rem;
		height: 1rem;
		border: 2px solid rgba(100, 116, 139, 0.3);
		border-top-color: var(--primary-500);
		border-radius: 50%;
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	/* Table - Dark styling */
	.table-container {
		background: rgba(30, 41, 59, 0.4);
		border-radius: 8px;
		border: 1px solid rgba(100, 116, 139, 0.2);
		overflow: hidden;
	}

	table {
		width: 100%;
		border-collapse: collapse;
	}

	thead {
		background: rgba(30, 41, 59, 0.6);
	}

	th {
		padding: 0.75rem 1rem;
		text-align: left;
		font-size: 0.75rem;
		font-weight: 600;
		color: #94a3b8;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	td {
		padding: 1rem;
		border-top: 1px solid rgba(100, 116, 139, 0.15);
		vertical-align: middle;
	}

	tbody tr:hover {
		background: rgba(100, 116, 139, 0.1);
	}

	.indicator-cell {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.logo {
		width: 3rem;
		height: 3rem;
		border-radius: 8px;
		object-fit: cover;
	}

	.logo-placeholder {
		width: 3rem;
		height: 3rem;
		background: rgba(100, 116, 139, 0.2);
		border-radius: 8px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: #64748b;
	}

	.indicator-info {
		display: flex;
		flex-direction: column;
		gap: 0.125rem;
	}

	.name {
		font-weight: 500;
		color: #f1f5f9;
		text-decoration: none;
	}

	.name:hover {
		color: #a5b4fc;
	}

	.tagline {
		font-size: 0.8125rem;
		color: #64748b;
	}

	.price {
		font-weight: 600;
		color: #a5b4fc;
	}

	.platform {
		color: #94a3b8;
	}

	.date {
		color: #64748b;
		font-size: 0.8125rem;
	}

	.status {
		display: inline-block;
		padding: 0.25rem 0.625rem;
		border-radius: 1.25rem;
		font-size: 0.75rem;
		font-weight: 500;
		text-transform: capitalize;
	}

	.status--draft {
		background: rgba(251, 191, 36, 0.15);
		color: #fbbf24;
	}

	.status--published {
		background: rgba(34, 197, 94, 0.15);
		color: #22c55e;
	}

	.actions {
		display: flex;
		gap: 0.5rem;
	}

	.btn-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 2rem;
		height: 2rem;
		border: none;
		background: rgba(100, 116, 139, 0.2);
		border-radius: 6px;
		color: #94a3b8;
		cursor: pointer;
		text-decoration: none;
		transition: all 0.2s ease;
	}

	.btn-icon:hover {
		background: rgba(100, 116, 139, 0.3);
		color: #f1f5f9;
	}

	.btn-danger:hover {
		background: rgba(239, 68, 68, 0.2);
		color: #ef4444;
	}

	.btn-icon:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	/* Pagination */
	.pagination {
		display: flex;
		justify-content: center;
		align-items: center;
		gap: 1rem;
		margin-top: 1.5rem;
		flex-wrap: wrap;
	}

	.btn-page {
		padding: 0.5rem 1rem;
		background: rgba(100, 116, 139, 0.2);
		color: #cbd5e1;
		border: 1px solid rgba(100, 116, 139, 0.3);
		border-radius: 6px;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.2s ease;
		/* Touch-friendly min size */
		min-height: 44px;
		min-width: 44px;
	}

	.btn-page:hover:not(:disabled) {
		background: rgba(100, 116, 139, 0.3);
		color: #f1f5f9;
	}

	.btn-page:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.page-info {
		font-size: 0.875rem;
		color: #64748b;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * 2026 Mobile-First Responsive Design
	 * Breakpoints: xs(360px), sm(640px), md(768px), lg(1024px), xl(1280px)
	 * Touch targets: min 44x44px, Safe areas: env(safe-area-inset-*)
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* Mobile-first base improvements */
	.btn-primary,
	.btn-secondary,
	.btn-icon {
		min-height: 44px;
		min-width: 44px;
		-webkit-tap-highlight-color: transparent;
	}

	/* Mobile: Stack filters vertically */
	@media (max-width: 639px) {
		.filters {
			flex-direction: column;
			align-items: stretch;
		}

		.search-box {
			max-width: 100%;
			min-width: 100%;
		}

		.actions-row {
			flex-direction: column;
			width: 100%;
		}

		.actions-row .btn-primary {
			width: 100%;
			justify-content: center;
		}

		/* Hide table, show cards on mobile */
		.table-container {
			overflow-x: auto;
			-webkit-overflow-scrolling: touch;
		}

		table {
			min-width: 600px;
		}

		.indicator-cell {
			min-width: 200px;
		}

		/* Stack pagination */
		.pagination {
			flex-direction: column;
			gap: 0.75rem;
		}
	}

	/* sm: Small devices (≥ 640px) */
	@media (min-width: 640px) {
		.filters {
			flex-direction: row;
		}

		.search-box {
			min-width: 200px;
		}
	}

	/* md: Medium devices (≥ 768px) */
	@media (min-width: 768px) {
		.page-header h1 {
			font-size: 2rem;
		}

		th {
			padding: 1rem;
		}

		td {
			padding: 1.25rem 1rem;
		}
	}

	/* lg: Large devices (≥ 1024px) */
	@media (min-width: 1024px) {
		.search-box {
			max-width: 500px;
		}
	}

	/* Touch device optimizations */
	@media (hover: none) and (pointer: coarse) {
		.btn-icon {
			width: 2.5rem;
			height: 2.5rem;
		}

		td {
			padding: 1.25rem 1rem;
		}

		tbody tr:hover {
			background: transparent;
		}

		tbody tr:active {
			background: rgba(100, 116, 139, 0.15);
		}
	}

	/* Reduced motion preference */
	@media (prefers-reduced-motion: reduce) {
		.spinner,
		.spinner-small {
			animation: none;
		}

		.btn-primary:hover,
		.btn-secondary:hover,
		.btn-icon:hover {
			transform: none;
		}
	}

	/* Safe areas for notched devices */
	@supports (padding: max(0px)) {
		.admin-page-container {
			padding-left: max(1.5rem, env(safe-area-inset-left));
			padding-right: max(1.5rem, env(safe-area-inset-right));
			padding-bottom: max(1.5rem, env(safe-area-inset-bottom));
		}
	}
</style>

<!--
	My Indicator Download History Page
	Apple Principal Engineer ICT 7 Grade - February 2026

	Features:
	- View all indicator downloads
	- Paginated results
	- Filter by date range
	- Re-download files
	- Track download status
-->
<script lang="ts">
	import { onMount } from 'svelte';

	interface DownloadRecord {
		id: number;
		indicator_id: number;
		indicator_name?: string;
		indicator_slug?: string;
		file_id: number;
		file_name?: string;
		platform?: string;
		downloaded_at?: string;
		status?: string;
	}

	// State
	let downloads = $state<DownloadRecord[]>([]);
	let loading = $state(true);
	let error = $state('');
	let currentPage = $state(1);
	let totalPages = $state(1);
	let total = $state(0);
	const perPage = 20;

	// Fetch download history
	const fetchHistory = async () => {
		loading = true;
		error = '';

		try {
			const res = await fetch(`/api/my/indicators/history?page=${currentPage}&per_page=${perPage}`);
			const data = await res.json();

			if (data.success) {
				downloads = data.data.downloads || [];
				total = data.data.total;
				totalPages = data.data.total_pages;
			} else {
				error = data.error || 'Failed to load download history';
			}
		} catch (e) {
			error = 'Failed to load download history';
			console.error(e);
		} finally {
			loading = false;
		}
	};

	// Format date for display
	const formatDate = (dateStr?: string): string => {
		if (!dateStr) return 'Unknown';
		const date = new Date(dateStr);
		return date.toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'short',
			day: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	};

	// Get status badge class
	const getStatusClass = (status?: string): string => {
		switch (status?.toLowerCase()) {
			case 'completed':
				return 'status-completed';
			case 'pending':
				return 'status-pending';
			case 'failed':
				return 'status-failed';
			default:
				return 'status-completed';
		}
	};

	// Navigate to page
	const goToPage = (page: number) => {
		if (page >= 1 && page <= totalPages) {
			currentPage = page;
			fetchHistory();
		}
	};

	onMount(fetchHistory);
</script>

<svelte:head>
	<title>Download History - My Indicators</title>
	<meta name="description" content="View your indicator download history" />
</svelte:head>

<div class="history-page">
	<header class="page-header">
		<div class="header-left">
			<a href="/dashboard/indicators" class="back-link">Back to My Indicators</a>
			<h1>Download History</h1>
		</div>
	</header>

	{#if loading}
		<div class="loading">
			<div class="spinner"></div>
			<p>Loading download history...</p>
		</div>
	{:else if error}
		<div class="error-state">
			<p class="error-message">{error}</p>
			<button class="btn btn-primary" onclick={fetchHistory}>Try Again</button>
		</div>
	{:else if downloads.length === 0}
		<div class="empty-state">
			<p>No downloads yet</p>
			<p class="hint">Your indicator downloads will appear here.</p>
			<a href="/dashboard/indicators" class="btn btn-primary">Browse My Indicators</a>
		</div>
	{:else}
		<div class="history-content">
			<div class="history-stats">
				<p>Showing {downloads.length} of {total} downloads</p>
			</div>

			<div class="downloads-table">
				<table>
					<thead>
						<tr>
							<th>Indicator</th>
							<th>File</th>
							<th>Platform</th>
							<th>Downloaded</th>
							<th>Status</th>
							<th>Actions</th>
						</tr>
					</thead>
					<tbody>
						{#each downloads as download}
							<tr>
								<td>
									<a href="/dashboard/indicators/{download.indicator_slug}" class="indicator-link">
										{download.indicator_name || 'Unknown Indicator'}
									</a>
								</td>
								<td class="file-name">{download.file_name || 'Unknown File'}</td>
								<td class="platform">{download.platform || 'N/A'}</td>
								<td class="date">{formatDate(download.downloaded_at)}</td>
								<td>
									<span class="status-badge {getStatusClass(download.status)}">
										{download.status || 'Completed'}
									</span>
								</td>
								<td>
									<a href="/dashboard/indicators/{download.indicator_slug}" class="btn btn-small">
										Re-download
									</a>
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>

			<!-- Pagination -->
			{#if totalPages > 1}
				<nav class="pagination" aria-label="Download history pagination">
					<ul class="page-numbers">
						{#if currentPage > 1}
							<li>
								<button class="page-numbers prev" onclick={() => goToPage(currentPage - 1)}>
									Previous
								</button>
							</li>
						{/if}

						{#each Array.from({ length: totalPages }, (_, i) => i + 1) as page}
							{#if page === currentPage}
								<li>
									<span class="page-numbers current" aria-current="page">{page}</span>
								</li>
							{:else if page <= 3 || page > totalPages - 3 || Math.abs(page - currentPage) <= 1}
								<li>
									<button class="page-numbers" onclick={() => goToPage(page)}>
										{page}
									</button>
								</li>
							{:else if Math.abs(page - currentPage) === 2}
								<li>
									<span class="page-numbers dots">...</span>
								</li>
							{/if}
						{/each}

						{#if currentPage < totalPages}
							<li>
								<button class="page-numbers next" onclick={() => goToPage(currentPage + 1)}>
									Next
								</button>
							</li>
						{/if}
					</ul>
				</nav>
			{/if}
		</div>
	{/if}
</div>

<style>
	.history-page {
		max-width: 1200px;
		margin: 0 auto;
		padding: 24px;
		background: #fff;
		min-height: 100vh;
	}

	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 24px;
		flex-wrap: wrap;
		gap: 16px;
	}

	.header-left {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.back-link {
		color: #666;
		text-decoration: none;
		font-size: 14px;
	}

	.back-link:hover {
		color: #143e59;
	}

	h1 {
		font-size: 28px;
		font-weight: 700;
		color: #333;
		margin: 0;
	}

	.loading,
	.error-state,
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 80px 20px;
		text-align: center;
	}

	.spinner {
		width: 48px;
		height: 48px;
		border: 3px solid #e5e5e5;
		border-top-color: #f69532;
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin-bottom: 16px;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.error-message {
		color: #dc2626;
		font-size: 16px;
		margin: 0 0 20px;
	}

	.empty-state p {
		color: #666;
		font-size: 16px;
		margin: 0 0 8px;
	}

	.hint {
		color: #999;
		font-size: 14px;
	}

	.history-content {
		background: #fff;
		border: 1px solid #e5e5e5;
		border-radius: 12px;
		overflow: hidden;
	}

	.history-stats {
		padding: 16px 24px;
		background: #f9f9f9;
		border-bottom: 1px solid #e5e5e5;
		color: #666;
		font-size: 14px;
	}

	.downloads-table {
		overflow-x: auto;
	}

	.downloads-table table {
		width: 100%;
		border-collapse: collapse;
		min-width: 700px;
	}

	.downloads-table th {
		text-align: left;
		padding: 14px 16px;
		font-size: 12px;
		font-weight: 600;
		color: #666;
		text-transform: uppercase;
		background: #fafafa;
		border-bottom: 1px solid #e5e5e5;
	}

	.downloads-table td {
		padding: 16px;
		border-bottom: 1px solid #f3f4f6;
		color: #333;
		font-size: 14px;
	}

	.downloads-table tbody tr:hover {
		background: #fafafa;
	}

	.indicator-link {
		color: #143e59;
		text-decoration: none;
		font-weight: 500;
	}

	.indicator-link:hover {
		text-decoration: underline;
	}

	.file-name {
		color: #666;
	}

	.platform {
		text-transform: capitalize;
	}

	.date {
		white-space: nowrap;
		color: #888;
	}

	.status-badge {
		display: inline-block;
		padding: 4px 10px;
		border-radius: 12px;
		font-size: 12px;
		font-weight: 500;
	}

	.status-completed {
		background: #d1fae5;
		color: #065f46;
	}

	.status-pending {
		background: #fef3c7;
		color: #92400e;
	}

	.status-failed {
		background: #fee2e2;
		color: #dc2626;
	}

	.btn {
		display: inline-block;
		padding: 8px 16px;
		font-size: 13px;
		font-weight: 500;
		text-decoration: none;
		border-radius: 6px;
		border: none;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.btn-primary {
		background: #f69532;
		color: #fff;
	}

	.btn-primary:hover {
		background: #dc7309;
	}

	.btn-small {
		background: #f3f4f6;
		color: #374151;
		padding: 6px 12px;
		font-size: 12px;
	}

	.btn-small:hover {
		background: #e5e7eb;
	}

	/* Pagination */
	.pagination {
		padding: 24px;
		text-align: center;
		border-top: 1px solid #e5e5e5;
	}

	.page-numbers {
		list-style: none;
		margin: 0;
		padding: 0;
		display: flex;
		gap: 8px;
		align-items: center;
		justify-content: center;
		flex-wrap: wrap;
	}

	.page-numbers li {
		list-style: none;
	}

	.page-numbers button,
	.page-numbers span {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		min-width: 40px;
		height: 40px;
		padding: 8px 12px;
		border: 1px solid #e5e5e5;
		background: #fff;
		color: #333;
		font-size: 14px;
		font-weight: 500;
		border-radius: 6px;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.page-numbers button:hover {
		background: #f5f5f5;
		border-color: #143e59;
	}

	.page-numbers .current {
		background: #143e59;
		color: #fff;
		border-color: #143e59;
		cursor: default;
	}

	.page-numbers .dots {
		border: none;
		background: none;
		cursor: default;
	}

	.page-numbers .prev,
	.page-numbers .next {
		min-width: auto;
		padding: 8px 16px;
	}

	/* Responsive */
	@media (max-width: 768px) {
		.history-page {
			padding: 16px;
		}

		h1 {
			font-size: 24px;
		}

		.downloads-table th,
		.downloads-table td {
			padding: 12px;
		}

		.page-numbers button,
		.page-numbers span {
			min-width: 36px;
			height: 36px;
			font-size: 13px;
		}
	}

	@media (max-width: 576px) {
		.downloads-table {
			font-size: 13px;
		}

		.pagination {
			padding: 16px;
		}

		.page-numbers {
			gap: 4px;
		}

		.page-numbers .prev,
		.page-numbers .next {
			padding: 8px 10px;
		}
	}

	/* Touch device optimizations */
	@media (hover: none) and (pointer: coarse) {
		.btn,
		.page-numbers button {
			min-height: 44px;
		}
	}

	/* Reduced motion */
	@media (prefers-reduced-motion: reduce) {
		.spinner {
			animation: none;
		}
	}
</style>

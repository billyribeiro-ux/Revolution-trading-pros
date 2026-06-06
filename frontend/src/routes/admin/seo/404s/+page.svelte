<script lang="ts">
	import { onMount } from 'svelte';
	import { Card, Button, Badge, Table } from '$lib/components/ui';
	import { addToast } from '$lib/utils/toast';
	import { seoApi, type Error404 } from '$lib/api/seo';
	import { IconAlertCircle, IconTrash } from '$lib/icons';
	import ConfirmationModal from '$lib/components/admin/ConfirmationModal.svelte';

	// State using Svelte 5 runes
	let errors = $state<Error404[]>([]);
	let loading = $state(true);
	let stats = $state({
		total: 0,
		resolved: 0,
		unresolved: 0,
		total_hits: 0
	});

	// Delete confirmation modal state
	let showDeleteModal = $state(false);
	let pendingDeleteResolvedOnly = $state(true);

	// Svelte 5: Initialize on mount
	onMount(() => {
		const init = async () => {
			await loadErrors();
			await loadStats();
		};
		void init();
	});

	async function loadErrors() {
		try {
			loading = true;
			// list404s returns Error404[] directly
			errors = (await seoApi.list404s()) || [];
		} catch (_error) {
			addToast({ type: 'error', message: 'Failed to load 404 errors' });
		} finally {
			loading = false;
		}
	}

	async function loadStats() {
		try {
			const response = (await seoApi.get404Stats()) as {
				total: number;
				resolved: number;
				unresolved: number;
				total_hits: number;
			};
			stats = response;
		} catch (error) {
			console.error('Failed to load stats:', error);
		}
	}

	function handleBulkDelete(resolvedOnly: boolean = true) {
		pendingDeleteResolvedOnly = resolvedOnly;
		showDeleteModal = true;
	}

	async function confirmBulkDelete() {
		showDeleteModal = false;
		const resolvedOnly = pendingDeleteResolvedOnly;

		try {
			await seoApi.bulkDelete404s(resolvedOnly);
			addToast({ type: 'success', message: '404 errors deleted successfully' });
			await loadErrors();
			await loadStats();
		} catch (_error) {
			addToast({ type: 'error', message: 'Failed to delete 404 errors' });
		}
	}
</script>

<svelte:head>
	<title>404 Errors | Revolution Admin</title>
</svelte:head>

<div class="errors-page">
	<!-- Header -->
	<div class="page-header">
		<div>
			<h1 class="page-title">404 Error Monitor</h1>
			<p class="page-subtitle">Track and fix broken links on your site</p>
		</div>
		<Button variant="danger" onclick={() => handleBulkDelete(true)}>
			<IconTrash size={20} />
			Delete Resolved
		</Button>
	</div>

	<!-- Stats -->
	<div class="stats-grid">
		<Card>
			<p class="stat-label">Total 404s</p>
			<p class="stat-value">{stats.total}</p>
		</Card>
		<Card>
			<p class="stat-label">Unresolved</p>
			<p class="stat-value stat-value--danger">{stats.unresolved}</p>
		</Card>
		<Card>
			<p class="stat-label">Resolved</p>
			<p class="stat-value stat-value--success">{stats.resolved}</p>
		</Card>
		<Card>
			<p class="stat-label">Total Hits</p>
			<p class="stat-value">{stats.total_hits}</p>
		</Card>
	</div>

	<!-- Errors List -->
	{#if loading}
		<Card>
			<div class="state-card">
				<div class="spinner" aria-label="Loading 404 errors"></div>
				<p class="state-text">Loading 404 errors...</p>
			</div>
		</Card>
	{:else if errors.length === 0}
		<Card>
			<div class="state-card">
				<span class="state-icon state-icon--success">
					<IconAlertCircle size={48} />
				</span>
				<p class="empty-text">No 404 errors found. Great job!</p>
			</div>
		</Card>
	{:else}
		<Card padding={false}>
			<Table headers={['URL', 'Hit Count', 'Status', 'First Seen', 'Last Seen']}>
				{#each errors as error (error.id)}
					<tr>
						<td class="url-cell">{error.url}</td>
						<td>
							<Badge variant={error.hit_count > 10 ? 'danger' : 'warning'}>
								{error.hit_count} hits
							</Badge>
						</td>
						<td>
							<Badge variant={error.is_resolved ? 'success' : 'danger'}>
								{error.is_resolved ? 'Resolved' : 'Active'}
							</Badge>
						</td>
						<td class="date-cell">
							{new Date(error.first_seen_at).toLocaleDateString()}
						</td>
						<td class="date-cell">
							{new Date(error.last_seen_at).toLocaleDateString()}
						</td>
					</tr>
				{/each}
			</Table>
		</Card>

		<!-- Suggested Actions -->
		<div class="actions-card">
			<Card>
				<h3 class="section-title">Recommended Actions</h3>
				<ul class="action-list">
					<li class="action-item">
						<span class="action-marker">→</span>
						<span>Review high-traffic 404s and create redirects to relevant content</span>
					</li>
					<li class="action-item">
						<span class="action-marker">→</span>
						<span>Check internal links that may be pointing to these broken URLs</span>
					</li>
					<li class="action-item">
						<span class="action-marker">→</span>
						<span>Consider creating content for frequently accessed missing pages</span>
					</li>
				</ul>
			</Card>
		</div>
	{/if}
</div>

<ConfirmationModal
	isOpen={showDeleteModal}
	title="Delete 404 Errors"
	message={pendingDeleteResolvedOnly
		? 'Delete all resolved 404 errors?'
		: 'Delete all 404 errors? This action cannot be undone.'}
	confirmText="Delete"
	variant="danger"
	onConfirm={confirmBulkDelete}
	onCancel={() => {
		showDeleteModal = false;
	}}
/>

<style>
	.errors-page {
		color: #111827;
	}

	.page-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		margin-bottom: 1.5rem;
	}

	.page-title {
		margin: 0;
		color: #111827;
		font-size: 1.875rem;
		font-weight: 700;
		line-height: 2.25rem;
	}

	.page-subtitle {
		margin: 0.25rem 0 0;
		color: #4b5563;
	}

	.stats-grid {
		display: grid;
		grid-template-columns: 1fr;
		gap: 1rem;
		margin-bottom: 1.5rem;
	}

	.stat-label {
		margin: 0;
		color: #4b5563;
		font-size: 0.875rem;
	}

	.stat-value {
		margin: 0.25rem 0 0;
		color: #111827;
		font-size: 1.5rem;
		font-weight: 700;
		line-height: 2rem;
	}

	.stat-value--danger {
		color: #dc2626;
	}

	.stat-value--success {
		color: #16a34a;
	}

	.state-card {
		padding-block: 3rem;
		text-align: center;
	}

	.spinner {
		width: 3rem;
		height: 3rem;
		margin: 0 auto;
		border: 2px solid rgba(37, 99, 235, 0.18);
		border-bottom-color: #2563eb;
		border-radius: 999px;
		animation: spin 800ms linear infinite;
	}

	.state-text {
		margin: 1rem 0 0;
		color: #4b5563;
	}

	.state-icon {
		display: block;
		margin: 0 auto 1rem;
	}

	.state-icon--success {
		color: #22c55e;
	}

	.empty-text {
		margin: 0;
		color: #6b7280;
	}

	.url-cell {
		max-width: 28rem;
		overflow: hidden;
		color: #111827;
		font-family:
			ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, 'Liberation Mono', 'Courier New',
			monospace;
		font-size: 0.875rem;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.date-cell {
		color: #4b5563;
		font-size: 0.875rem;
	}

	.actions-card {
		margin-top: 1.5rem;
	}

	.section-title {
		margin: 0 0 1rem;
		color: #111827;
		font-size: 1.125rem;
		font-weight: 700;
		line-height: 1.75rem;
	}

	.action-list {
		display: grid;
		gap: 0.5rem;
		margin: 0;
		padding: 0;
		color: #374151;
		list-style: none;
	}

	.action-item {
		display: flex;
		align-items: flex-start;
		gap: 0.5rem;
	}

	.action-marker {
		margin-top: 0.125rem;
		color: #2563eb;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	@media (min-width: 768px) {
		.stats-grid {
			grid-template-columns: repeat(4, minmax(0, 1fr));
		}
	}

	@media (max-width: 640px) {
		.page-header {
			align-items: stretch;
			flex-direction: column;
		}
	}
</style>

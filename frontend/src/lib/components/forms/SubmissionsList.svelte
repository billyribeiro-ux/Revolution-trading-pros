<script lang="ts">
	import { onMount } from 'svelte';
	import { SvelteSet } from 'svelte/reactivity';
	import type { FormSubmission } from '$lib/api/forms';
	import {
		getSubmissions,
		getSubmissionStats,
		updateSubmissionStatus,
		deleteSubmission,
		bulkUpdateSubmissionStatus,
		bulkDeleteSubmissions,
		exportSubmissions
	} from '$lib/api/forms';
	import ConfirmationModal from '$lib/components/admin/ConfirmationModal.svelte';
	import { toastStore } from '$lib/stores/toast.svelte';

	interface Props {
		formId: number;
	}

	let { formId }: Props = $props();

	// The backend's submission-stats endpoint returns these summary counts.
	// (The shared `FormAnalytics` type models a different, richer payload, so we
	// describe only the fields this template renders.)
	interface SubmissionStats {
		total_submissions?: number;
		unread_count?: number;
		starred_count?: number;
		recent_submissions?: number;
	}

	let submissions: FormSubmission[] = $state([]);
	let stats: SubmissionStats | null = $state(null);
	let loading = $state(true);
	let error = $state('');
	let currentPage = $state(1);
	let totalPages = $state(1);
	let statusFilter: string = $state('');
	let selectedSubmissions = new SvelteSet<string>();

	// Confirmation modal state (replaces native confirm())
	let showDeleteOneModal = $state(false);
	let pendingDeleteSubmission = $state<FormSubmission | null>(null);
	let showBulkDeleteModal = $state(false);

	onMount(() => {
		loadData();
	});

	async function loadData() {
		loading = true;
		error = '';

		try {
			// R8-A: `getSubmissions` filters are now `Record<string, string>`
			// (was `any`). Build the dict explicitly from `statusFilter`.
			const filters = statusFilter ? { status: statusFilter } : undefined;
			const [submissionsData, statsData] = await Promise.all([
				getSubmissions(formId, currentPage, 20, filters),
				getSubmissionStats(formId)
			]);

			submissions = submissionsData.submissions;
			totalPages = Math.ceil((submissionsData.total ?? 0) / (submissionsData.perPage ?? 10));
			stats = statsData as unknown as SubmissionStats;
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load submissions';
		} finally {
			loading = false;
		}
	}

	async function handleStatusChange(
		submission: FormSubmission,
		newStatus: FormSubmission['status']
	) {
		try {
			await updateSubmissionStatus(formId, Number(submission.submission_id), newStatus);
			await loadData();
		} catch (err) {
			toastStore.error(err instanceof Error ? err.message : 'Failed to update status');
		}
	}

	function handleDelete(submission: FormSubmission) {
		pendingDeleteSubmission = submission;
		showDeleteOneModal = true;
	}

	async function confirmDelete() {
		const submission = pendingDeleteSubmission;
		if (!submission) return;
		showDeleteOneModal = false;
		pendingDeleteSubmission = null;
		try {
			await deleteSubmission(formId, Number(submission.submission_id));
			await loadData();
		} catch (err) {
			toastStore.error(err instanceof Error ? err.message : 'Failed to delete submission');
		}
	}

	function cancelDelete() {
		showDeleteOneModal = false;
		pendingDeleteSubmission = null;
	}

	async function handleBulkStatusUpdate(newStatus: FormSubmission['status']) {
		if (selectedSubmissions.size === 0) return;

		try {
			await bulkUpdateSubmissionStatus(
				formId,
				Array.from(selectedSubmissions).map(Number),
				newStatus
			);
			clearSelections();
			await loadData();
		} catch (err) {
			toastStore.error(err instanceof Error ? err.message : 'Failed to update submissions');
		}
	}

	function handleBulkDelete() {
		if (selectedSubmissions.size === 0) return;
		showBulkDeleteModal = true;
	}

	async function confirmBulkDelete() {
		showBulkDeleteModal = false;
		try {
			await bulkDeleteSubmissions(formId, Array.from(selectedSubmissions).map(Number));
			clearSelections();
			await loadData();
		} catch (err) {
			toastStore.error(err instanceof Error ? err.message : 'Failed to delete submissions');
		}
	}

	function cancelBulkDelete() {
		showBulkDeleteModal = false;
	}

	async function handleExport() {
		try {
			const csv = await exportSubmissions(formId);
			const blob = new Blob([csv], { type: 'text/csv' });
			const url = window.URL.createObjectURL(blob);
			const a = document.createElement('a');
			a.href = url;
			a.download = `form-submissions-${formId}-${Date.now()}.csv`;
			a.click();
			window.URL.revokeObjectURL(url);
		} catch (err) {
			toastStore.error(err instanceof Error ? err.message : 'Failed to export submissions');
		}
	}

	function toggleSelection(submissionId: string) {
		if (selectedSubmissions.has(submissionId)) {
			selectedSubmissions.delete(submissionId);
		} else {
			selectedSubmissions.add(submissionId);
		}
	}

	function toggleSelectAll() {
		if (selectedSubmissions.size === submissions.length) {
			clearSelections();
		} else {
			clearSelections();
			submissions.forEach((submission) => selectedSubmissions.add(submission.submission_id));
		}
	}

	function clearSelections() {
		selectedSubmissions.clear();
	}

	function handleFilterChange(event: Event) {
		const target = event.target as HTMLSelectElement;
		statusFilter = target.value;
		currentPage = 1;
		loadData();
	}

	function handlePageChange(page: number) {
		currentPage = page;
		loadData();
	}

	function getStatusBadgeClass(status: string): string {
		switch (status) {
			case 'unread':
				return 'badge-unread';
			case 'read':
				return 'badge-read';
			case 'starred':
				return 'badge-starred';
			case 'archived':
				return 'badge-archived';
			case 'spam':
				return 'badge-spam';
			default:
				return 'badge-default';
		}
	}

	function submissionRowClass(submissionId: string) {
		return {
			selected: selectedSubmissions.has(submissionId)
		};
	}

	function statusBadgeClass(status: string) {
		return ['badge', getStatusBadgeClass(status)];
	}

	function formatDate(dateString: string | undefined): string {
		if (!dateString) return 'N/A';
		return new Date(dateString).toLocaleString();
	}

	let hasSelections = $derived(selectedSubmissions.size > 0);
	let allSelected = $derived(
		submissions.length > 0 && selectedSubmissions.size === submissions.length
	);
</script>

<div class="submissions-container">
	{#if stats}
		<div class="stats-grid">
			<div class="stat-card">
				<div class="stat-value">{stats.total_submissions}</div>
				<div class="stat-label">Total</div>
			</div>
			<div class="stat-card">
				<div class="stat-value">{stats.unread_count}</div>
				<div class="stat-label">Unread</div>
			</div>
			<div class="stat-card">
				<div class="stat-value">{stats.starred_count}</div>
				<div class="stat-label">Starred</div>
			</div>
			<div class="stat-card">
				<div class="stat-value">{stats.recent_submissions}</div>
				<div class="stat-label">Last 7 Days</div>
			</div>
		</div>
	{/if}

	<div class="submissions-header">
		<div class="header-left">
			<select class="filter-select" onchange={handleFilterChange} bind:value={statusFilter}>
				<option value="">All Submissions</option>
				<option value="unread">Unread</option>
				<option value="read">Read</option>
				<option value="starred">Starred</option>
				<option value="archived">Archived</option>
				<option value="spam">Spam</option>
			</select>

			{#if hasSelections}
				<div class="bulk-actions">
					<span class="selection-count">{selectedSubmissions.size} selected</span>
					<button class="btn-bulk" onclick={() => handleBulkStatusUpdate('read')}>Mark Read</button>
					<button class="btn-bulk" onclick={() => handleBulkStatusUpdate('starred')}>Star</button>
					<button class="btn-bulk" onclick={() => handleBulkStatusUpdate('archived')}
						>Archive</button
					>
					<button class="btn-bulk btn-danger" onclick={handleBulkDelete}>Delete</button>
				</div>
			{/if}
		</div>

		<button class="btn-export" onclick={handleExport}> Export CSV </button>
	</div>

	{#if loading}
		<div class="loading">Loading submissions...</div>
	{:else if error}
		<div class="error-message">{error}</div>
	{:else if submissions.length === 0}
		<div class="empty-state">
			<p>No submissions yet.</p>
		</div>
	{:else}
		<div class="table-container">
			<table class="submissions-table">
				<thead>
					<tr>
						<th>
							<input type="checkbox" checked={allSelected} onchange={toggleSelectAll} />
						</th>
						<th>Submission ID</th>
						<th>Status</th>
						<th>Date</th>
						<th>Fields</th>
						<th>Actions</th>
					</tr>
				</thead>
				<tbody>
					{#each submissions as submission (submission.id)}
						<tr class={submissionRowClass(submission.submission_id)}>
							<td>
								<input
									type="checkbox"
									checked={selectedSubmissions.has(submission.submission_id)}
									onchange={() => toggleSelection(submission.submission_id)}
								/>
							</td>
							<td class="submission-id">{submission.submission_id}</td>
							<td>
								<span class={statusBadgeClass(submission.status)}>
									{submission.status}
								</span>
							</td>
							<td>{formatDate(submission.created_at)}</td>
							<td>
								<div class="field-data">
									{#if submission.data}
										{#each submission.data.slice(0, 3) as data (data.field_name)}
											{@const preview =
												typeof data.value === 'string'
													? data.value
													: data.value === null || data.value === undefined
														? ''
														: JSON.stringify(data.value)}
											<div class="field-item">
												<strong>{data.field?.label || data.field_name}:</strong>
												{preview.substring(0, 50)}{preview.length > 50 ? '...' : ''}
											</div>
										{/each}
										{#if submission.data.length > 3}
											<div class="field-more">+{submission.data.length - 3} more</div>
										{/if}
									{/if}
								</div>
							</td>
							<td>
								<div class="action-buttons">
									<select
										class="status-select"
										value={submission.status}
										onchange={(e: Event) =>
											handleStatusChange(
												submission,
												(e.currentTarget as HTMLSelectElement).value as FormSubmission['status']
											)}
									>
										<option value="unread">Unread</option>
										<option value="read">Read</option>
										<option value="starred">Starred</option>
										<option value="archived">Archived</option>
										<option value="spam">Spam</option>
									</select>
									<button
										class="btn-icon btn-danger"
										onclick={() => handleDelete(submission)}
										title="Delete"
									>
										🗑️
									</button>
								</div>
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>

		{#if totalPages > 1}
			<div class="pagination">
				<button
					class="btn-page"
					disabled={currentPage === 1}
					onclick={() => handlePageChange(currentPage - 1)}
				>
					Previous
				</button>

				<span class="page-info">
					Page {currentPage} of {totalPages}
				</span>

				<button
					class="btn-page"
					disabled={currentPage === totalPages}
					onclick={() => handlePageChange(currentPage + 1)}
				>
					Next
				</button>
			</div>
		{/if}
	{/if}
</div>

<ConfirmationModal
	isOpen={showDeleteOneModal}
	title="Delete submission?"
	message="Are you sure you want to delete this submission? This action cannot be undone."
	confirmText="Delete"
	variant="danger"
	onConfirm={confirmDelete}
	onCancel={cancelDelete}
/>

<ConfirmationModal
	isOpen={showBulkDeleteModal}
	title="Delete submissions?"
	message={`Delete ${selectedSubmissions.size} submissions? This action cannot be undone.`}
	confirmText="Delete"
	variant="danger"
	onConfirm={confirmBulkDelete}
	onCancel={cancelBulkDelete}
/>

<style>
	.submissions-container {
		padding: 0;
	}

	.stats-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
		gap: 1rem;
		margin-bottom: 2rem;
	}

	.stat-card {
		background: rgba(99, 102, 241, 0.05);
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 10px;
		padding: 1.5rem;
		text-align: center;
	}

	.stat-value {
		font-size: 2rem;
		font-weight: 700;
		color: #a5b4fc;
		margin-bottom: 0.5rem;
	}

	.stat-label {
		font-size: 0.875rem;
		color: #94a3b8;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.submissions-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1.5rem;
		gap: 1rem;
		flex-wrap: wrap;
	}

	.header-left {
		display: flex;
		align-items: center;
		gap: 1rem;
		flex-wrap: wrap;
	}

	.filter-select {
		padding: 0.5rem 1rem;
		background: #1e293b;
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 8px;
		color: #e2e8f0;
		font-size: 0.875rem;
		cursor: pointer;
	}

	.bulk-actions {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.selection-count {
		font-size: 0.875rem;
		color: #94a3b8;
	}

	.btn-bulk {
		padding: 0.5rem 0.875rem;
		background: rgba(99, 102, 241, 0.1);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 6px;
		color: #a5b4fc;
		font-size: 0.75rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-bulk:hover {
		background: rgba(99, 102, 241, 0.2);
	}

	.btn-bulk.btn-danger {
		background: rgba(239, 68, 68, 0.1);
		border-color: rgba(239, 68, 68, 0.2);
		color: #f87171;
	}

	.btn-bulk.btn-danger:hover {
		background: rgba(239, 68, 68, 0.2);
	}

	.btn-export {
		padding: 0.625rem 1.25rem;
		background: linear-gradient(135deg, #6366f1, #8b5cf6);
		color: white;
		border: none;
		border-radius: 8px;
		font-weight: 600;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-export:hover {
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(99, 102, 241, 0.3);
	}

	.loading,
	.error-message,
	.empty-state {
		text-align: center;
		padding: 3rem 1rem;
		color: #94a3b8;
	}

	.error-message {
		color: #f87171;
	}

	.table-container {
		overflow-x: auto;
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 10px;
	}

	.submissions-table {
		width: 100%;
		border-collapse: collapse;
		background: #1e293b;
	}

	.submissions-table thead {
		background: rgba(99, 102, 241, 0.05);
	}

	.submissions-table th {
		padding: 0.75rem 1rem;
		text-align: left;
		font-size: 0.75rem;
		font-weight: 600;
		color: #94a3b8;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		border-bottom: 1px solid rgba(99, 102, 241, 0.1);
	}

	.submissions-table td {
		padding: 1rem;
		font-size: 0.875rem;
		color: #e2e8f0;
		border-bottom: 1px solid rgba(99, 102, 241, 0.05);
	}

	.submissions-table tbody tr:hover {
		background: rgba(99, 102, 241, 0.05);
	}

	.submissions-table tbody tr.selected {
		background: rgba(99, 102, 241, 0.1);
	}

	.submission-id {
		font-family: monospace;
		font-size: 0.75rem;
		color: #94a3b8;
	}

	.badge {
		display: inline-block;
		padding: 0.25rem 0.75rem;
		border-radius: 9999px;
		font-size: 0.75rem;
		font-weight: 500;
		text-transform: capitalize;
	}

	.badge-unread {
		background: rgba(59, 130, 246, 0.2);
		color: #60a5fa;
	}

	.badge-read {
		background: rgba(34, 197, 94, 0.2);
		color: #4ade80;
	}

	.badge-starred {
		background: rgba(251, 191, 36, 0.2);
		color: #fbbf24;
	}

	.badge-archived {
		background: rgba(107, 114, 128, 0.2);
		color: #9ca3af;
	}

	.badge-spam {
		background: rgba(239, 68, 68, 0.2);
		color: #f87171;
	}

	.field-data {
		max-width: 300px;
		font-size: 0.75rem;
	}

	.field-item {
		margin-bottom: 0.25rem;
		color: #94a3b8;
	}

	.field-item strong {
		color: #e2e8f0;
	}

	.field-more {
		color: #6366f1;
		font-size: 0.7rem;
	}

	.action-buttons {
		display: flex;
		gap: 0.5rem;
		align-items: center;
	}

	.status-select {
		padding: 0.375rem 0.625rem;
		background: #0f172a;
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 6px;
		color: #e2e8f0;
		font-size: 0.75rem;
		cursor: pointer;
	}

	.btn-icon {
		background: none;
		border: none;
		font-size: 1.125rem;
		cursor: pointer;
		padding: 0.25rem;
		opacity: 0.7;
		transition: opacity 0.2s;
	}

	.btn-icon:hover {
		opacity: 1;
	}

	.pagination {
		display: flex;
		justify-content: center;
		align-items: center;
		gap: 1rem;
		margin-top: 2rem;
	}

	.btn-page {
		padding: 0.5rem 1rem;
		background: #1e293b;
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 8px;
		color: #e2e8f0;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-page:hover:not(:disabled) {
		background: rgba(99, 102, 241, 0.1);
		border-color: rgba(99, 102, 241, 0.4);
	}

	.btn-page:disabled {
		opacity: 0.3;
		cursor: not-allowed;
	}

	.page-info {
		font-size: 0.875rem;
		color: #94a3b8;
	}
</style>

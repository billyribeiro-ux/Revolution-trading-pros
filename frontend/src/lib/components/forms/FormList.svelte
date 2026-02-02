<script lang="ts">
	import { onMount } from 'svelte';
	import type { Form } from '$lib/api/forms';
	import {
		getForms,
		deleteForm,
		publishForm,
		unpublishForm,
		archiveForm,
		duplicateForm
	} from '$lib/api/forms';

	interface Props {
		onEdit?: (form: Form) => void;
		onViewSubmissions?: (form: Form) => void;
		onViewAnalytics?: (form: Form) => void;
	}

	let props: Props = $props();

	let forms: Form[] = $state([]);
	let loading = $state(true);
	let error = $state('');
	let currentPage = $state(1);
	let totalPages = $state(1);
	let statusFilter: string = $state('');

	onMount(() => {
		loadForms();
	});

	async function loadForms() {
		loading = true;
		error = '';

		try {
			const filters = statusFilter ? { status: statusFilter } : undefined;
			const response = await getForms(currentPage, 20, filters);

			// Response is { forms: Form[], total: number, perPage: number }
			forms = response?.forms || [];
			totalPages = Math.ceil((response?.total || forms.length) / (response?.perPage || 20));
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load forms';
			forms = [];
		} finally {
			loading = false;
		}
	}

	async function handleDelete(form: Form) {
		if (!form.id) return;

		if (!confirm(`Are you sure you want to delete "${form.title}"?`)) {
			return;
		}

		try {
			await deleteForm(form.id);
			await loadForms();
		} catch (err) {
			alert(err instanceof Error ? err.message : 'Failed to delete form');
		}
	}

	async function handlePublish(form: Form) {
		if (!form.id) return;

		try {
			if (form.status === 'published') {
				await unpublishForm(form.id);
			} else {
				await publishForm(form.id);
			}
			await loadForms();
		} catch (err) {
			alert(err instanceof Error ? err.message : 'Failed to update form status');
		}
	}

	async function handleArchive(form: Form) {
		if (!form.id) return;

		if (!confirm(`Archive "${form.title}"?`)) {
			return;
		}

		try {
			await archiveForm(form.id);
			await loadForms();
		} catch (err) {
			alert(err instanceof Error ? err.message : 'Failed to archive form');
		}
	}

	async function handleDuplicate(form: Form) {
		if (!form.id) return;

		try {
			await duplicateForm(form.id);
			await loadForms();
		} catch (err) {
			alert(err instanceof Error ? err.message : 'Failed to duplicate form');
		}
	}

	function handleFilterChange(event: Event) {
		const target = event.target as HTMLSelectElement;
		statusFilter = target.value;
		currentPage = 1;
		loadForms();
	}

	function handlePageChange(page: number) {
		currentPage = page;
		loadForms();
	}

	function getStatusBadgeClass(status: string): string {
		switch (status) {
			case 'published':
				return 'badge-success';
			case 'draft':
				return 'badge-warning';
			case 'archived':
				return 'badge-secondary';
			default:
				return 'badge-default';
		}
	}

	function formatDate(dateString: string | undefined): string {
		if (!dateString) return 'N/A';
		return new Date(dateString).toLocaleDateString();
	}
</script>

<div class="form-list-container">
	<div class="list-header">
		<h2>Forms</h2>
		<div class="header-actions">
			<select class="filter-select" onchange={handleFilterChange} bind:value={statusFilter}>
				<option value="">All Forms</option>
				<option value="published">Published</option>
				<option value="draft">Drafts</option>
				<option value="archived">Archived</option>
			</select>
		</div>
	</div>

	{#if loading}
		<div class="loading">Loading forms...</div>
	{:else if error}
		<div class="error-message">{error}</div>
	{:else if forms.length === 0}
		<div class="empty-state">
			<p>No forms found.</p>
			<p class="empty-hint">Create your first form to get started!</p>
		</div>
	{:else}
		<div class="forms-table-container">
			<table class="forms-table">
				<thead>
					<tr>
						<th>Title</th>
						<th>Status</th>
						<th>Submissions</th>
						<th>Created</th>
						<th>Published</th>
						<th>Actions</th>
					</tr>
				</thead>
				<tbody>
					{#each forms as form (form.id)}
						<tr>
							<td class="form-title">
								<div class="title-cell">
									<strong>{form.title}</strong>
									{#if form.description}
										<p class="form-description-preview">{form.description}</p>
									{/if}
								</div>
							</td>
							<td>
								<span class="badge {getStatusBadgeClass(form.status)}">
									{form.status}
								</span>
							</td>
							<td class="text-center">{form.submission_count || 0}</td>
							<td>{formatDate(form.created_at)}</td>
							<td>{formatDate(form.published_at)}</td>
							<td>
								<div class="action-buttons">
									{#if props.onEdit}
										<button class="btn-icon" onclick={() => props.onEdit?.(form)} title="Edit">
											✏️
										</button>
									{/if}

									{#if props.onViewSubmissions}
										<button
											class="btn-icon"
											onclick={() => props.onViewSubmissions?.(form)}
											title="View Submissions"
										>
											📊
										</button>
									{/if}

									{#if props.onViewAnalytics}
										<button
											class="btn-icon"
											onclick={() => props.onViewAnalytics?.(form)}
											title="View Analytics"
										>
											📈
										</button>
									{/if}

									<button
										class="btn-icon"
										onclick={() => handlePublish(form)}
										title={form.status === 'published' ? 'Unpublish' : 'Publish'}
									>
										{form.status === 'published' ? '👁️' : '🚀'}
									</button>

									<button class="btn-icon" onclick={() => handleDuplicate(form)} title="Duplicate">
										📋
									</button>

									{#if form.status !== 'archived'}
										<button class="btn-icon" onclick={() => handleArchive(form)} title="Archive">
											📦
										</button>
									{/if}

									<button
										class="btn-icon btn-danger"
										onclick={() => handleDelete(form)}
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

<style>
	.form-list-container {
		padding: 0;
	}

	.list-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 2rem;
	}

	.list-header h2 {
		font-size: 1.5rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0;
	}

	.header-actions {
		display: flex;
		gap: 1rem;
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

	.empty-hint {
		font-size: 0.875rem;
		margin-top: 0.5rem;
	}

	.forms-table-container {
		overflow-x: auto;
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 10px;
	}

	.forms-table {
		width: 100%;
		border-collapse: collapse;
		background: #1e293b;
	}

	.forms-table thead {
		background: rgba(99, 102, 241, 0.05);
	}

	.forms-table th {
		padding: 0.75rem 1rem;
		text-align: left;
		font-size: 0.75rem;
		font-weight: 600;
		color: #94a3b8;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		border-bottom: 1px solid rgba(99, 102, 241, 0.1);
	}

	.forms-table td {
		padding: 1rem;
		font-size: 0.875rem;
		color: #e2e8f0;
		border-bottom: 1px solid rgba(99, 102, 241, 0.05);
	}

	.forms-table tbody tr:hover {
		background: rgba(99, 102, 241, 0.05);
	}

	.form-title {
		min-width: 250px;
	}

	.title-cell strong {
		display: block;
		margin-bottom: 0.25rem;
		color: #f1f5f9;
	}

	.form-description-preview {
		color: #6b7280;
		font-size: 0.875rem;
		margin: 0;
		display: -webkit-box;
		-webkit-line-clamp: 2;
		line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	.text-center {
		text-align: center;
	}

	.badge {
		display: inline-block;
		padding: 0.25rem 0.75rem;
		border-radius: 9999px;
		font-size: 0.75rem;
		font-weight: 500;
		text-transform: capitalize;
	}

	.badge-success {
		background: rgba(34, 197, 94, 0.2);
		color: #4ade80;
	}

	.badge-warning {
		background: rgba(251, 191, 36, 0.2);
		color: #fbbf24;
	}

	.badge-secondary {
		background: rgba(107, 114, 128, 0.2);
		color: #9ca3af;
	}

	.action-buttons {
		display: flex;
		gap: 0.5rem;
		flex-wrap: wrap;
	}

	.btn-icon {
		background: none;
		border: none;
		font-size: 1.25rem;
		cursor: pointer;
		padding: 0.25rem;
		opacity: 0.7;
		transition: opacity 0.2s;
	}

	.btn-icon:hover {
		opacity: 1;
	}

	.btn-danger:hover {
		opacity: 1;
		filter: brightness(0.9);
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

	@media (max-width: 768px) {
		.forms-table th,
		.forms-table td {
			padding: 0.5rem;
			font-size: 0.75rem;
		}

		.action-buttons {
			flex-direction: column;
		}
	}
</style>

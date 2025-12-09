<script lang="ts">
	import { onMount } from 'svelte';
	import { emailTemplatesApi, AdminApiError, type EmailTemplate } from '$lib/api/admin';
	import { goto } from '$app/navigation';
	import { IconEdit, IconTrash, IconEye, IconPlus } from '@tabler/icons-svelte';

	let templates: EmailTemplate[] = [];
	let loading = true;
	let error = '';

	async function loadTemplates() {
		loading = true;
		error = '';
		try {
			const response = await emailTemplatesApi.list();
			templates = (response.data as unknown as EmailTemplate[]) || [];
		} catch (e: any) {
			if (e instanceof AdminApiError) {
				if (e.status === 401) {
					goto('/login');
					return;
				}
				// Handle SQL errors with user-friendly messages
				if (e.message.includes('SQLSTATE') || e.message.includes('Column not found') || e.message.includes('deleted_at')) {
					error = 'Database configuration issue. Email templates feature is being set up. Please try again later or contact support.';
				} else {
					error = e.message;
				}
			} else if (e?.message?.includes('SQLSTATE') || e?.message?.includes('deleted_at')) {
				// Handle raw SQL errors
				error = 'Database configuration issue. Email templates feature is being set up. Please try again later or contact support.';
			} else {
				error = 'Failed to load templates. Please refresh the page or contact support.';
			}
			console.error('Failed to load templates:', e);
		} finally {
			loading = false;
		}
	}

	async function deleteTemplate(id: number) {
		if (!confirm('Delete this template? This action cannot be undone.')) return;
		try {
			await emailTemplatesApi.delete(id);
			templates = templates.filter((t) => t.id !== id);
		} catch (e) {
			alert('Failed to delete template');
			console.error(e);
		}
	}

	onMount(loadTemplates);
</script>

<svelte:head>
	<title>Email Templates | Admin | Revolution Trading Pros</title>
</svelte:head>

<div class="templates-page">
	<div class="page-header">
		<h1>Email Templates</h1>
		<button class="btn-primary" onclick={() => goto('/admin/email/templates/new')}
			>New Template</button
		>
	</div>

	{#if loading}
		<div class="loading-state">
			<div class="loader"></div>
			<p>Loading templates...</p>
		</div>
	{:else if error}
		<div class="alert alert-error">
			<div class="error-content">
				<span>{error}</span>
				<button class="btn-retry" onclick={loadTemplates}>Retry</button>
			</div>
		</div>
	{:else if templates.length === 0}
		<div class="empty-state">
			<p>No templates found. Create one to get started.</p>
			<button class="btn-primary" onclick={() => goto('/admin/email/templates/new')}>Create Template</button>
		</div>
	{:else}
		<table class="templates-table">
			<thead>
				<tr>
					<th>Name</th>
					<th>Slug</th>
					<th>Subject</th>
					<th>Type</th>
					<th>Status</th>
					<th>Actions</th>
				</tr>
			</thead>
			<tbody>
				{#each templates as tmpl}
					<tr>
						<td>{(tmpl as any).name}</td>
						<td>{(tmpl as any).slug}</td>
						<td>{(tmpl as any).subject}</td>
						<td>{(tmpl as any).email_type}</td>
						<td>{(tmpl as any).is_active ? 'Active' : 'Inactive'}</td>
						<td class="actions">
							<button
								class="btn-secondary"
								onclick={() => goto(`/admin/email/templates/preview/${(tmpl as any).id}`)}
								title="Preview"><IconEye size={18} /></button
							>
							<button
								class="btn-secondary"
								onclick={() => goto(`/admin/email/templates/edit/${(tmpl as any).id}`)}
								title="Edit"><IconEdit size={18} /></button
							>
							<button
								class="btn-danger"
								onclick={() => deleteTemplate((tmpl as any).id)}
								title="Delete"><IconTrash size={18} /></button
							>
						</td>
					</tr>
				{/each}
			</tbody>
		</table>
	{/if}
</div>

<style>
	.templates-page {
		max-width: 1200px;
		margin: 0 auto;
		padding: 2rem;
	}
	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1.5rem;
	}
	.templates-table {
		width: 100%;
		border-collapse: collapse;
		background: rgba(30, 41, 59, 0.4);
		border-radius: 8px;
		overflow: hidden;
	}
	.templates-table th,
	.templates-table td {
		padding: 0.75rem 1rem;
		text-align: left;
		color: #e2e8f0;
	}
	.templates-table th {
		background: rgba(15, 23, 42, 0.6);
		font-weight: 600;
	}
	.templates-table tr:nth-child(even) {
		background: rgba(15, 23, 42, 0.3);
	}
	.actions button {
		margin-right: 0.5rem;
	}
	.btn-primary {
		background: linear-gradient(135deg, #6366f1, #8b5cf6);
		color: white;
		border: none;
		padding: 0.5rem 1rem;
		border-radius: 6px;
		cursor: pointer;
	}
	.btn-secondary {
		background: rgba(100, 116, 139, 0.2);
		color: #cbd5e1;
		border: 1px solid rgba(100, 116, 139, 0.3);
		padding: 0.4rem 0.6rem;
		border-radius: 4px;
		cursor: pointer;
	}
	.btn-danger {
		background: rgba(239, 68, 68, 0.2);
		color: #f87171;
		border: 1px solid rgba(239, 68, 68, 0.3);
		padding: 0.4rem 0.6rem;
		border-radius: 4px;
		cursor: pointer;
	}
	.btn-primary:hover,
	.btn-secondary:hover,
	.btn-danger:hover {
		opacity: 0.9;
	}
	.alert-error {
		background: rgba(239, 68, 68, 0.1);
		color: #ef4444;
		padding: 0.75rem 1rem;
		border-radius: 6px;
		margin-bottom: 1rem;
		border: 1px solid rgba(239, 68, 68, 0.3);
	}
	.error-content {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
	}
	.btn-retry {
		background: rgba(239, 68, 68, 0.2);
		color: #f87171;
		border: 1px solid rgba(239, 68, 68, 0.3);
		padding: 0.4rem 1rem;
		border-radius: 6px;
		cursor: pointer;
		font-weight: 600;
		white-space: nowrap;
	}
	.btn-retry:hover {
		background: rgba(239, 68, 68, 0.3);
	}
	.loading-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 2rem;
		color: #94a3b8;
	}
	.loader {
		width: 40px;
		height: 40px;
		border: 3px solid rgba(99, 102, 241, 0.2);
		border-top-color: #6366f1;
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin-bottom: 1rem;
	}
	@keyframes spin {
		to { transform: rotate(360deg); }
	}
	.empty-state {
		text-align: center;
		padding: 4rem 2rem;
		background: rgba(30, 41, 59, 0.4);
		border-radius: 12px;
		border: 1px solid rgba(148, 163, 184, 0.1);
	}
	.empty-state p {
		color: #94a3b8;
		margin-bottom: 1.5rem;
	}
</style>

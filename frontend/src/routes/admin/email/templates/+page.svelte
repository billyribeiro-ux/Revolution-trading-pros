<script lang="ts">
import { logger } from '$lib/utils/logger';
	/**
	 * Email Templates - Apple ICT7 Principal Engineer Grade
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * Svelte 5 runes implementation with:
	 * - $state for reactive state management
	 * - $effect for lifecycle and data loading
	 * - Proper TypeScript types
	 * - Enhanced error handling
	 *
	 * @version 2.0.0 - Svelte 5 Migration (Dec 2025)
	 */

	import { emailTemplatesApi, AdminApiError, type EmailTemplate } from '$lib/api/admin';
	import { goto } from '$app/navigation';
	import { toastStore } from '$lib/stores/toast.svelte';
	import { IconEdit, IconTrash, IconEye, IconPlus, IconRefresh } from '$lib/icons';

	// ═══════════════════════════════════════════════════════════════════════════════
	// State - Svelte 5 Runes
	// ═══════════════════════════════════════════════════════════════════════════════

	let templates = $state<EmailTemplate[]>([]);
	let loading = $state(true);
	let error = $state('');
	let searchQuery = $state('');

	// ═══════════════════════════════════════════════════════════════════════════════
	// Derived State
	// ═══════════════════════════════════════════════════════════════════════════════

	let filteredTemplates = $derived(
		templates.filter((tmpl) => {
			if (!searchQuery) return true;
			const query = searchQuery.toLowerCase();
			return (
				tmpl.name?.toLowerCase().includes(query) ||
				tmpl.subject?.toLowerCase().includes(query) ||
				tmpl.slug?.toLowerCase().includes(query)
			);
		})
	);

	// ═══════════════════════════════════════════════════════════════════════════════
	// Lifecycle - Svelte 5 $effect
	// ═══════════════════════════════════════════════════════════════════════════════

	$effect(() => {
		loadTemplates();
	});

	// ═══════════════════════════════════════════════════════════════════════════════
	// Data Loading
	// ═══════════════════════════════════════════════════════════════════════════════

	async function loadTemplates() {
		loading = true;
		error = '';
		try {
			const response = await emailTemplatesApi.list();
			templates = (response.data as unknown as EmailTemplate[]) || [];
		} catch (e: unknown) {
			if (e instanceof AdminApiError) {
				if (e.status === 401) {
					goto('/login');
					return;
				}
				// Handle SQL errors with user-friendly messages
				if (
					e.message.includes('SQLSTATE') ||
					e.message.includes('Column not found') ||
					e.message.includes('deleted_at')
				) {
					error =
						'Database configuration issue. Email templates feature is being set up. Please try again later or contact support.';
				} else {
					error = e.message;
				}
			} else if (
				e instanceof Error &&
				(e.message?.includes('SQLSTATE') || e.message?.includes('deleted_at'))
			) {
				// Handle raw SQL errors
				error =
					'Database configuration issue. Email templates feature is being set up. Please try again later or contact support.';
			} else {
				error = 'Failed to load templates. Please refresh the page or contact support.';
			}
			logger.error('Failed to load templates:', e);
		} finally {
			loading = false;
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════════
	// Actions
	// ═══════════════════════════════════════════════════════════════════════════════

	async function deleteTemplate(id: number) {
		if (!confirm('Are you sure you want to delete this template?')) return;
		try {
			await emailTemplatesApi.delete(id);
			templates = templates.filter((t) => t.id !== id);
			toastStore.success('Template deleted successfully');
		} catch (e) {
			toastStore.error('Failed to delete template');
			logger.error(e);
		}
	}
</script>

<svelte:head>
	<title>Email Templates | Admin | Revolution Trading Pros</title>
</svelte:head>

<div class="templates-page">
	<div class="page-header">
		<div class="header-left">
			<h1>Email Templates</h1>
			<p class="subtitle">Manage your email templates for campaigns and automations</p>
		</div>
		<div class="header-actions">
			<div class="search-box">
				<input
					id="page-searchquery"
					name="page-searchquery"
					type="text"
					placeholder="Search templates..."
					bind:value={searchQuery}
					class="search-input"
				/>
			</div>
			<button class="btn-secondary" onclick={loadTemplates} disabled={loading}>
				<IconRefresh size={18} />
				Refresh
			</button>
			<button class="btn-primary" onclick={() => goto('/admin/email/templates/new')}>
				<IconPlus size={18} />
				New Template
			</button>
		</div>
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
	{:else if filteredTemplates.length === 0}
		<div class="empty-state">
			{#if searchQuery}
				<p>No templates match "{searchQuery}"</p>
				<button class="btn-secondary" onclick={() => (searchQuery = '')}>Clear Search</button>
			{:else}
				<p>No templates found. Create one to get started.</p>
				<button class="btn-primary" onclick={() => goto('/admin/email/templates/new')}
					>Create Template</button
				>
			{/if}
		</div>
	{:else}
		<div class="templates-count">
			Showing {filteredTemplates.length} of {templates.length} templates
		</div>
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
				{#each filteredTemplates as tmpl}
					<tr>
						<td class="name-cell">{tmpl.name}</td>
						<td class="slug-cell"><code>{tmpl.slug}</code></td>
						<td class="subject-cell">{tmpl.subject}</td>
						<td><span class="type-badge">{tmpl.email_type}</span></td>
						<td>
							<span class="status-badge" class:active={tmpl.is_active}>
								{tmpl.is_active ? 'Active' : 'Inactive'}
							</span>
						</td>
						<td class="actions">
							<button
								class="btn-icon"
								onclick={() => goto(`/admin/email/templates/preview/${tmpl.id}`)}
								title="Preview"
							>
								<IconEye size={18} />
							</button>
							<button
								class="btn-icon"
								onclick={() => goto(`/admin/email/templates/edit/${tmpl.id}`)}
								title="Edit"
							>
								<IconEdit size={18} />
							</button>
							<button
								class="btn-icon danger"
								onclick={() => deleteTemplate(tmpl.id)}
								title="Delete"
							>
								<IconTrash size={18} />
							</button>
						</td>
					</tr>
				{/each}
			</tbody>
		</table>
	{/if}
</div>

<style>
	.templates-page {
		max-width: 1400px;
		margin: 0 auto;
		padding: 2rem;
	}
	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 1.5rem;
		gap: 1rem;
	}
	.header-left h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0;
	}
	.subtitle {
		color: #64748b;
		font-size: 0.875rem;
		margin: 0.25rem 0 0;
	}
	.header-actions {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}
	.search-box {
		position: relative;
	}
	.search-input {
		width: 240px;
		padding: 0.625rem 1rem;
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		color: #f1f5f9;
		font-size: 0.875rem;
	}
	.search-input:focus {
		outline: none;
		border-color: rgba(99, 102, 241, 0.5);
	}
	.templates-count {
		font-size: 0.8125rem;
		color: #64748b;
		margin-bottom: 1rem;
	}
	.name-cell {
		font-weight: 600;
		color: #f1f5f9;
	}
	.slug-cell code {
		padding: 0.25rem 0.5rem;
		background: rgba(15, 23, 42, 0.6);
		border-radius: 4px;
		font-size: 0.75rem;
		color: #a5b4fc;
	}
	.type-badge {
		padding: 0.25rem 0.5rem;
		background: rgba(99, 102, 241, 0.15);
		color: #a5b4fc;
		border-radius: 6px;
		font-size: 0.75rem;
		font-weight: 500;
		text-transform: capitalize;
	}
	.status-badge {
		padding: 0.25rem 0.5rem;
		background: rgba(148, 163, 184, 0.15);
		color: #94a3b8;
		border-radius: 6px;
		font-size: 0.75rem;
		font-weight: 500;
	}
	.status-badge.active {
		background: rgba(34, 197, 94, 0.15);
		color: #22c55e;
	}
	.btn-icon {
		width: 32px;
		height: 32px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(148, 163, 184, 0.1);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 6px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
	}
	.btn-icon:hover {
		background: rgba(99, 102, 241, 0.15);
		border-color: rgba(99, 102, 241, 0.3);
		color: #a5b4fc;
	}
	.btn-icon.danger:hover {
		background: rgba(239, 68, 68, 0.15);
		border-color: rgba(239, 68, 68, 0.3);
		color: #f87171;
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
		background: linear-gradient(135deg, var(--primary-500) 0%, var(--primary-600) 100%);
		color: var(--bg-base);
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
	.btn-primary:hover,
	.btn-secondary:hover {
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
		border: 3px solid rgba(230, 184, 0, 0.2);
		border-top-color: var(--primary-500);
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin-bottom: 1rem;
	}
	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
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

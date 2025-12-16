<script lang="ts">
	import { onMount } from 'svelte';
	import { Card, Button, Badge, Table, Input, Select } from '$lib/components/ui';
	import { addToast } from '$lib/utils/toast';
	import { crmAPI } from '$lib/api/crm';
	import type { Contact, ContactStatus } from '$lib/crm/types';
	import { IconPlus, IconSearch, IconMail, IconPhone } from '$lib/icons';

	let contacts = $state<Contact[]>([]);
	let loading = $state(true);
	let searchQuery = $state('');
	let statusFilter = $state<ContactStatus | ''>('');
	let pagination = $state<{ current_page: number; last_page: number; total: number }>({
		current_page: 1,
		last_page: 1,
		total: 0
	});

	const statusOptions = [
		{ value: '', label: 'All Statuses' },
		{ value: 'lead', label: 'Lead' },
		{ value: 'prospect', label: 'Prospect' },
		{ value: 'customer', label: 'Customer' },
		{ value: 'churned', label: 'Churned' },
		{ value: 'unqualified', label: 'Unqualified' }
	];

	onMount(async () => {
		await loadContacts();
	});

	async function loadContacts() {
		try {
			loading = true;
			const response = await crmAPI.getContacts({
				search: searchQuery || undefined,
				status: statusFilter || undefined,
				per_page: 25
			});
			contacts = response.data || [];
			if (response.meta) {
				pagination = {
					current_page: response.meta.current_page,
					last_page: response.meta.last_page,
					total: response.meta.total
				};
			}
		} catch (error) {
			addToast({ type: 'error', message: 'Failed to load contacts' });
		} finally {
			loading = false;
		}
	}

	function getStatusColor(status: string): 'success' | 'info' | 'warning' | 'default' {
		switch (status) {
			case 'customer':
				return 'success';
			case 'prospect':
				return 'info';
			case 'lead':
				return 'warning';
			case 'churned':
				return 'default';
			default:
				return 'default';
		}
	}

	// Reload contacts when filter changes
	$effect(() => {
		if (statusFilter !== '') {
			loadContacts();
		}
	});

	// Debounced search
	let searchTimeout: ReturnType<typeof setTimeout>;
	function handleSearch() {
		clearTimeout(searchTimeout);
		searchTimeout = setTimeout(() => {
			loadContacts();
		}, 300);
	}
</script>

<svelte:head>
	<title>Contacts | Revolution Admin</title>
</svelte:head>

<div class="contacts-page">
	<!-- Header -->
	<div class="page-header">
		<div>
			<h1 class="page-title">Contacts</h1>
			<p class="page-subtitle">Manage your contact list and leads</p>
		</div>
		<Button>
			<a href="/admin/contacts/new" class="flex items-center gap-2">
				<IconPlus size={20} />
				Add Contact
			</a>
		</Button>
	</div>

	<!-- Filters -->
	<div class="filters-grid">
		<Input
			placeholder="Search contacts..."
			bind:value={searchQuery}
			oninput={handleSearch}
		/>
		<Select options={statusOptions} bind:value={statusFilter} placeholder="Filter by status" />
	</div>

	<!-- Contact Stats -->
	<div class="stats-grid">
		<Card>
			<p class="stat-label">Total Contacts</p>
			<p class="stat-value">{pagination.total}</p>
		</Card>
		<Card>
			<p class="stat-label">Customers</p>
			<p class="stat-value stat-success">
				{contacts.filter((c) => c.status === 'customer').length}
			</p>
		</Card>
		<Card>
			<p class="stat-label">Prospects</p>
			<p class="stat-value stat-info">
				{contacts.filter((c) => c.status === 'prospect').length}
			</p>
		</Card>
		<Card>
			<p class="stat-label">Leads</p>
			<p class="stat-value stat-warning">
				{contacts.filter((c) => c.status === 'lead').length}
			</p>
		</Card>
	</div>

	<!-- Contacts List -->
	{#if loading}
		<Card>
			<div class="loading-state">
				<div class="loading-spinner"></div>
				<p class="loading-text">Loading contacts...</p>
			</div>
		</Card>
	{:else if contacts.length === 0}
		<Card>
			<div class="empty-state">
				<p class="empty-title">No contacts found.</p>
				<p class="empty-description">Form submissions will automatically create contacts here.</p>
			</div>
		</Card>
	{:else}
		<Card padding={false}>
			<Table headers={['Name', 'Email', 'Phone', 'Job Title', 'Status', 'Lead Score', 'Last Activity']}>
				{#each contacts as contact}
					<tr>
						<td>
							<a
								href="/admin/contacts/{contact.id}"
								class="contact-name-link"
							>
								{contact.full_name || `${contact.first_name} ${contact.last_name}`}
							</a>
						</td>
						<td>
							<a
								href="mailto:{contact.email}"
								class="contact-link"
							>
								<IconMail size={16} />
								{contact.email}
							</a>
						</td>
						<td>
							{#if contact.phone}
								<a
									href="tel:{contact.phone}"
									class="contact-link"
								>
									<IconPhone size={16} />
									{contact.phone}
								</a>
							{:else}
								<span class="text-muted">—</span>
							{/if}
						</td>
						<td>{contact.job_title || '—'}</td>
						<td>
							<Badge variant={getStatusColor(contact.status)}>
								{contact.status}
							</Badge>
						</td>
						<td>
							<span class="lead-score" class:high={contact.lead_score >= 70} class:medium={contact.lead_score >= 40 && contact.lead_score < 70}>
								{contact.lead_score || 0}
							</span>
						</td>
						<td class="text-secondary">
							{contact.last_activity_at
								? new Date(contact.last_activity_at).toLocaleDateString()
								: '—'}
						</td>
					</tr>
				{/each}
			</Table>
		</Card>

		<!-- Pagination -->
		{#if pagination.last_page > 1}
			<div class="pagination-info">
				<p>
					Page {pagination.current_page} of {pagination.last_page} ({pagination.total} contacts)
				</p>
			</div>
		{/if}
	{/if}
</div>

<style>
	.contacts-page {
		color: var(--admin-text-primary);
	}

	/* Header */
	.page-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-bottom: 1.5rem;
	}

	.page-title {
		font-size: 1.875rem;
		font-weight: 700;
		color: var(--admin-text-primary);
	}

	.page-subtitle {
		color: var(--admin-text-muted);
		margin-top: 0.25rem;
	}

	/* Filters */
	.filters-grid {
		display: grid;
		grid-template-columns: repeat(1, 1fr);
		gap: 1rem;
		margin-bottom: 1.5rem;
	}

	@media (min-width: 768px) {
		.filters-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	/* Stats */
	.stats-grid {
		display: grid;
		grid-template-columns: repeat(1, 1fr);
		gap: 1rem;
		margin-bottom: 1.5rem;
	}

	@media (min-width: 768px) {
		.stats-grid {
			grid-template-columns: repeat(4, 1fr);
		}
	}

	.stat-label {
		font-size: 0.875rem;
		color: var(--admin-text-muted);
	}

	.stat-value {
		font-size: 1.5rem;
		font-weight: 700;
		margin-top: 0.25rem;
		color: var(--admin-text-primary);
	}

	.stat-success {
		color: var(--admin-success);
	}

	.stat-info {
		color: var(--admin-info);
	}

	.stat-warning {
		color: var(--admin-warning);
	}

	/* Loading State */
	.loading-state {
		text-align: center;
		padding: 3rem 0;
	}

	.loading-spinner {
		width: 3rem;
		height: 3rem;
		border-radius: 9999px;
		border: 2px solid var(--admin-border-light);
		border-bottom-color: var(--admin-accent-primary);
		animation: spin 1s linear infinite;
		margin: 0 auto;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.loading-text {
		margin-top: 1rem;
		color: var(--admin-text-muted);
	}

	/* Empty State */
	.empty-state {
		text-align: center;
		padding: 3rem 0;
	}

	.empty-title {
		color: var(--admin-text-secondary);
	}

	.empty-description {
		font-size: 0.875rem;
		color: var(--admin-text-muted);
		margin-top: 0.5rem;
	}

	/* Table Links */
	.contact-name-link {
		font-weight: 600;
		color: var(--admin-accent-primary);
		text-decoration: none;
		transition: color 0.2s;
	}

	.contact-name-link:hover {
		color: var(--admin-accent-secondary);
	}

	.contact-link {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		color: var(--admin-text-secondary);
		text-decoration: none;
		transition: color 0.2s;
	}

	.contact-link:hover {
		color: var(--admin-accent-primary);
	}

	.text-muted {
		color: var(--admin-text-muted);
	}

	.text-secondary {
		color: var(--admin-text-secondary);
	}

	/* Lead Score */
	.lead-score {
		font-weight: 500;
		color: var(--admin-text-muted);
	}

	.lead-score.high {
		color: var(--admin-success);
	}

	.lead-score.medium {
		color: var(--admin-warning);
	}

	/* Pagination */
	.pagination-info {
		display: flex;
		justify-content: center;
		gap: 0.5rem;
		margin-top: 1rem;
	}

	.pagination-info p {
		font-size: 0.875rem;
		color: var(--admin-text-muted);
	}
</style>

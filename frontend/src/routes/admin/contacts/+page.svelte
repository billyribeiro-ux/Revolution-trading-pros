<script lang="ts">
	import { onMount } from 'svelte';
	import { Card, Button, Badge, Table, Input, Select } from '$lib/components/ui';
	import { addToast } from '$lib/utils/toast';
	import { crmAPI } from '$lib/api/crm';
	import type { Contact } from '$lib/crm/types';
	import { IconPlus, IconSearch, IconMail, IconPhone } from '@tabler/icons-svelte';

	let contacts = $state<Contact[]>([]);
	let loading = $state(true);
	let searchQuery = $state('');
	let statusFilter = $state('');
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

<div>
	<!-- Header -->
	<div class="flex items-center justify-between mb-6">
		<div>
			<h1 class="text-3xl font-bold text-gray-900">Contacts</h1>
			<p class="text-gray-600 mt-1">Manage your contact list and leads</p>
		</div>
		<Button>
			<a href="/admin/contacts/new" class="flex items-center gap-2">
				<IconPlus size={20} />
				Add Contact
			</a>
		</Button>
	</div>

	<!-- Filters -->
	<div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-6">
		<Input
			placeholder="Search contacts..."
			bind:value={searchQuery}
			oninput={handleSearch}
		/>
		<Select options={statusOptions} bind:value={statusFilter} placeholder="Filter by status" />
	</div>

	<!-- Contact Stats -->
	<div class="grid grid-cols-1 md:grid-cols-4 gap-4 mb-6">
		<Card>
			<p class="text-sm text-gray-600">Total Contacts</p>
			<p class="text-2xl font-bold mt-1">{pagination.total}</p>
		</Card>
		<Card>
			<p class="text-sm text-gray-600">Customers</p>
			<p class="text-2xl font-bold mt-1 text-green-600">
				{contacts.filter((c) => c.status === 'customer').length}
			</p>
		</Card>
		<Card>
			<p class="text-sm text-gray-600">Prospects</p>
			<p class="text-2xl font-bold mt-1 text-blue-600">
				{contacts.filter((c) => c.status === 'prospect').length}
			</p>
		</Card>
		<Card>
			<p class="text-sm text-gray-600">Leads</p>
			<p class="text-2xl font-bold mt-1 text-yellow-600">
				{contacts.filter((c) => c.status === 'lead').length}
			</p>
		</Card>
	</div>

	<!-- Contacts List -->
	{#if loading}
		<Card>
			<div class="text-center py-12">
				<div class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600 mx-auto"></div>
				<p class="mt-4 text-gray-600">Loading contacts...</p>
			</div>
		</Card>
	{:else if contacts.length === 0}
		<Card>
			<div class="text-center py-12">
				<p class="text-gray-500">No contacts found.</p>
				<p class="text-sm text-gray-400 mt-2">Form submissions will automatically create contacts here.</p>
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
								class="font-semibold text-blue-600 hover:text-blue-700"
							>
								{contact.full_name || `${contact.first_name} ${contact.last_name}`}
							</a>
						</td>
						<td>
							<a
								href="mailto:{contact.email}"
								class="flex items-center gap-2 text-gray-600 hover:text-blue-600"
							>
								<IconMail size={16} />
								{contact.email}
							</a>
						</td>
						<td>
							{#if contact.phone}
								<a
									href="tel:{contact.phone}"
									class="flex items-center gap-2 text-gray-600 hover:text-blue-600"
								>
									<IconPhone size={16} />
									{contact.phone}
								</a>
							{:else}
								<span class="text-gray-400">—</span>
							{/if}
						</td>
						<td>{contact.job_title || '—'}</td>
						<td>
							<Badge variant={getStatusColor(contact.status)}>
								{contact.status}
							</Badge>
						</td>
						<td>
							<span class="font-medium {contact.lead_score >= 70 ? 'text-green-600' : contact.lead_score >= 40 ? 'text-yellow-600' : 'text-gray-500'}">
								{contact.lead_score || 0}
							</span>
						</td>
						<td class="text-gray-600">
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
			<div class="flex justify-center gap-2 mt-4">
				<p class="text-sm text-gray-600">
					Page {pagination.current_page} of {pagination.last_page} ({pagination.total} contacts)
				</p>
			</div>
		{/if}
	{/if}
</div>

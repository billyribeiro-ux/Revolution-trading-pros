<script lang="ts">
	import { onMount } from 'svelte';
	import { Card, Button, Badge, Table, Input, Select } from '$lib/components/ui';
	import { addToast } from '$lib/utils/toast';
	import { contactsApi, type Contact } from '$lib/api/forms';
	import { IconPlus, IconSearch, IconMail, IconPhone } from '@tabler/icons-svelte';

	let contacts: Contact[] = [];
	let loading = true;
	let searchQuery = '';
	let statusFilter = '';

	const statusOptions = [
		{ value: '', label: 'All Statuses' },
		{ value: 'lead', label: 'Lead' },
		{ value: 'subscriber', label: 'Subscriber' },
		{ value: 'customer', label: 'Customer' },
		{ value: 'inactive', label: 'Inactive' }
	];

	onMount(async () => {
		await loadContacts();
	});

	async function loadContacts() {
		try {
			loading = true;
			const response = await contactsApi.list();
			contacts = response.contacts || [];
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
			case 'subscriber':
				return 'info';
			case 'lead':
				return 'warning';
			default:
				return 'default';
		}
	}

	let filteredContacts = $derived(contacts.filter((contact) => {
		const matchesSearch =
			contact.email.toLowerCase().includes(searchQuery.toLowerCase()) ||
			contact.full_name?.toLowerCase().includes(searchQuery.toLowerCase()) ||
			contact.company?.toLowerCase().includes(searchQuery.toLowerCase());

		const matchesStatus = !statusFilter || contact.status === statusFilter;

		return matchesSearch && matchesStatus;
	}));
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
		<Input placeholder="Search contacts..." bind:value={searchQuery} />
		<Select options={statusOptions} bind:value={statusFilter} placeholder="Filter by status" />
	</div>

	<!-- Contact Stats -->
	<div class="grid grid-cols-1 md:grid-cols-4 gap-4 mb-6">
		<Card>
			<p class="text-sm text-gray-600">Total Contacts</p>
			<p class="text-2xl font-bold mt-1">{contacts.length}</p>
		</Card>
		<Card>
			<p class="text-sm text-gray-600">Customers</p>
			<p class="text-2xl font-bold mt-1 text-green-600">
				{contacts.filter((c) => c.status === 'customer').length}
			</p>
		</Card>
		<Card>
			<p class="text-sm text-gray-600">Subscribers</p>
			<p class="text-2xl font-bold mt-1 text-blue-600">
				{contacts.filter((c) => c.status === 'subscriber').length}
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
	{:else if filteredContacts.length === 0}
		<Card>
			<div class="text-center py-12">
				<p class="text-gray-500">No contacts found.</p>
			</div>
		</Card>
	{:else}
		<Card padding={false}>
			<Table headers={['Name', 'Email', 'Phone', 'Company', 'Status', 'Last Activity']}>
				{#each filteredContacts as contact}
					<tr>
						<td>
							<a
								href="/admin/contacts/{contact.id}"
								class="font-semibold text-blue-600 hover:text-blue-700"
							>
								{contact.full_name || 'N/A'}
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
								<span class="text-gray-400">N/A</span>
							{/if}
						</td>
						<td>{contact.company || 'N/A'}</td>
						<td>
							<Badge variant={getStatusColor(contact.status)}>
								{contact.status}
							</Badge>
						</td>
						<td class="text-gray-600">
							{contact.last_activity_at
								? new Date(contact.last_activity_at).toLocaleDateString()
								: 'N/A'}
						</td>
					</tr>
				{/each}
			</Table>
		</Card>
	{/if}
</div>

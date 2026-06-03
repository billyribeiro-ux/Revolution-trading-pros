<script lang="ts">
	import { SvelteSet } from 'svelte/reactivity';
	import { onMount } from 'svelte';
	import { toastStore } from '$lib/stores/toast.svelte';
	import { adminFetch } from '$lib/utils/adminFetch';
	import { emailApi, type EmailSubscriber } from '$lib/api/email';
	import ConfirmationModal from '$lib/components/admin/ConfirmationModal.svelte';

	import PageHeader from './_components/PageHeader.svelte';
	import StatsGrid from './_components/StatsGrid.svelte';
	import FiltersBar from './_components/FiltersBar.svelte';
	import EmptyState from './_components/EmptyState.svelte';
	import SubscribersTable, { type RowAction } from './_components/SubscribersTable.svelte';
	import Pagination from './_components/Pagination.svelte';
	import AddSubscriberModal from './_components/AddSubscriberModal.svelte';

	// State
	let loading = $state(true);
	let subscribers = $state<EmailSubscriber[]>([]);
	let total = $state(0);
	let page = $state(1);
	let perPage = $state(25);
	let searchQuery = $state('');
	let statusFilter = $state('all');
	let selectedTags = $state<string[]>([]);
	let selectedSubscribers = new SvelteSet<string>();

	// Modal states
	let showAddModal = $state(false);
	let showDeleteModal = $state(false);
	let showBulkDeleteModal = $state(false);
	let pendingDeleteId = $state<string | null>(null);

	// New subscriber form
	let newSubscriber = $state({
		email: '',
		first_name: '',
		last_name: '',
		tags: ''
	});

	// Stats
	let stats = $state({
		total: 0,
		subscribed: 0,
		unsubscribed: 0,
		bounced: 0
	});

	onMount(async () => {
		await loadSubscribers();
		await loadStats();
	});

	async function loadSubscribers() {
		loading = true;
		try {
			const params: {
				status?: string;
				search?: string;
				tags?: string[];
				page?: number;
				per_page?: number;
			} = { page, per_page: perPage };
			if (searchQuery) params.search = searchQuery;
			if (statusFilter !== 'all') params.status = statusFilter;
			if (selectedTags.length > 0) params.tags = selectedTags;

			const response = await emailApi.getSubscribers(params);
			subscribers = response.subscribers;
			total = response.total;
		} catch (err) {
			toastStore.error('Failed to load subscribers');
			console.error(err);
		} finally {
			loading = false;
		}
	}

	async function loadStats() {
		try {
			const data = await adminFetch('/api/admin/email/subscribers/stats');
			stats = data.data || data;
		} catch (err) {
			console.error('Failed to load stats:', err);
		}
	}

	async function handleAddSubscriber() {
		if (!newSubscriber.email) {
			toastStore.error('Email is required');
			return;
		}

		try {
			const tags = newSubscriber.tags
				.split(',')
				.map((t) => t.trim())
				.filter(Boolean);
			await emailApi.createSubscriber({
				email: newSubscriber.email,
				...(newSubscriber.first_name && { first_name: newSubscriber.first_name }),
				...(newSubscriber.last_name && { last_name: newSubscriber.last_name }),
				tags
			});

			toastStore.success('Subscriber added successfully');
			showAddModal = false;
			newSubscriber = { email: '', first_name: '', last_name: '', tags: '' };
			await loadSubscribers();
			await loadStats();
		} catch (err) {
			const message = err instanceof Error ? err.message : undefined;
			toastStore.error(message || 'Failed to add subscriber');
		}
	}

	function handleDeleteSubscriber(id: string) {
		pendingDeleteId = id;
		showDeleteModal = true;
	}

	async function confirmDeleteSubscriber() {
		if (!pendingDeleteId) return;
		showDeleteModal = false;
		const id = pendingDeleteId;
		pendingDeleteId = null;
		try {
			await emailApi.deleteSubscriber(id);
			toastStore.success('Subscriber deleted');
			await loadSubscribers();
			await loadStats();
		} catch (err) {
			const message = err instanceof Error ? err.message : undefined;
			toastStore.error(message || 'Failed to delete subscriber');
		}
	}

	async function handleUnsubscribe(id: string) {
		try {
			await emailApi.unsubscribe(id);
			toastStore.success('Subscriber unsubscribed');
			await loadSubscribers();
			await loadStats();
		} catch (err) {
			const message = err instanceof Error ? err.message : undefined;
			toastStore.error(message || 'Failed to unsubscribe');
		}
	}

	async function handleResubscribe(id: string) {
		try {
			await emailApi.resubscribe(id);
			toastStore.success('Subscriber resubscribed');
			await loadSubscribers();
			await loadStats();
		} catch (err) {
			const message = err instanceof Error ? err.message : undefined;
			toastStore.error(message || 'Failed to resubscribe');
		}
	}

	function handleRowAction(action: RowAction) {
		switch (action.type) {
			case 'unsubscribe':
				handleUnsubscribe(action.id);
				break;
			case 'resubscribe':
				handleResubscribe(action.id);
				break;
			case 'delete':
				handleDeleteSubscriber(action.id);
				break;
		}
	}

	function handleBulkDelete() {
		if (selectedSubscribers.size === 0) return;
		showBulkDeleteModal = true;
	}

	async function confirmBulkDelete() {
		showBulkDeleteModal = false;
		try {
			for (const id of selectedSubscribers) {
				await emailApi.deleteSubscriber(id);
			}
			toastStore.success(`Deleted ${selectedSubscribers.size} subscribers`);
			selectedSubscribers.clear();
			await loadSubscribers();
			await loadStats();
		} catch (err) {
			const message = err instanceof Error ? err.message : undefined;
			toastStore.error(message || 'Failed to delete subscribers');
		}
	}

	async function handleExport() {
		try {
			const response = await adminFetch<Response>('/api/admin/email/subscribers/export', {
				rawResponse: true
			});
			const blob = await response.blob();
			const url = window.URL.createObjectURL(blob);
			const a = document.createElement('a');
			a.href = url;
			a.download = `subscribers-${new Date().toISOString().split('T')[0]}.csv`;
			a.click();
			window.URL.revokeObjectURL(url);
		} catch (_err) {
			toastStore.error('Failed to export subscribers');
		}
	}

	function toggleSubscriber(id: string) {
		if (selectedSubscribers.has(id)) {
			selectedSubscribers.delete(id);
		} else {
			selectedSubscribers.add(id);
		}
	}

	function toggleAllSubscribers() {
		if (selectedSubscribers.size === subscribers.length) {
			selectedSubscribers.clear();
		} else {
			selectedSubscribers = new SvelteSet(subscribers.map((s) => s.id));
		}
	}

	function getStatusColor(status: string) {
		switch (status) {
			case 'subscribed':
				return 'bg-emerald-500/20 text-emerald-400';
			case 'unsubscribed':
				return 'bg-slate-500/20 text-slate-400';
			case 'bounced':
				return 'bg-red-500/20 text-red-400';
			case 'complained':
				return 'bg-yellow-500/20 text-yellow-400';
			default:
				return 'bg-slate-500/20 text-slate-400';
		}
	}

	function formatDate(dateStr: string | undefined): string {
		if (!dateStr) return '-';
		return new Date(dateStr).toLocaleDateString('en-US', {
			month: 'short',
			day: 'numeric',
			year: 'numeric'
		});
	}

	function handleSearch() {
		page = 1;
		loadSubscribers();
	}

	function handlePageChange(newPage: number) {
		page = newPage;
		loadSubscribers();
	}

	let totalPages = $derived(Math.ceil(total / perPage));
</script>

<svelte:head>
	<title>Email Subscribers | Admin</title>
</svelte:head>

<div class="subscribers-page">
	<PageHeader onexport={handleExport} onadd={() => (showAddModal = true)} />

	<StatsGrid {stats} />

	<FiltersBar
		bind:searchQuery
		bind:statusFilter
		selectedCount={selectedSubscribers.size}
		onsearch={handleSearch}
		onrefresh={loadSubscribers}
		onbulkdelete={handleBulkDelete}
	/>

	<!-- Subscribers Table -->
	<div class="table-container">
		{#if loading}
			<div class="loading-state">
				<div class="spinner"></div>
				<p>Loading subscribers...</p>
			</div>
		{:else if subscribers.length === 0}
			<EmptyState onadd={() => (showAddModal = true)} />
		{:else}
			<SubscribersTable
				{subscribers}
				bind:selectedSubscribers
				{getStatusColor}
				{formatDate}
				ontoggleAll={toggleAllSubscribers}
				ontoggleOne={toggleSubscriber}
				onaction={handleRowAction}
			/>

			{#if totalPages > 1}
				<Pagination {page} {totalPages} {total} onpageChange={handlePageChange} />
			{/if}
		{/if}
	</div>
</div>

<!-- Add Subscriber Modal -->
{#if showAddModal}
	<AddSubscriberModal
		bind:newSubscriber
		onclose={() => (showAddModal = false)}
		onsubmit={handleAddSubscriber}
	/>
{/if}

<ConfirmationModal
	isOpen={showDeleteModal}
	title="Delete Subscriber"
	message="Are you sure you want to delete this subscriber? This cannot be undone."
	confirmText="Delete"
	variant="danger"
	onConfirm={confirmDeleteSubscriber}
	onCancel={() => {
		showDeleteModal = false;
		pendingDeleteId = null;
	}}
/>

<ConfirmationModal
	isOpen={showBulkDeleteModal}
	title="Delete Subscribers"
	message={`Delete ${selectedSubscribers.size} selected subscribers? This cannot be undone.`}
	confirmText="Delete All"
	variant="danger"
	onConfirm={confirmBulkDelete}
	onCancel={() => (showBulkDeleteModal = false)}
/>

<style>
	.subscribers-page {
		padding: 2rem;
		max-width: 1400px;
		margin: 0 auto;
	}

	/* Table container (page chrome only) */
	.table-container {
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 16px;
		overflow: hidden;
	}

	/* Loading state (only rendered by parent) */
	.loading-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 2rem;
		color: #64748b;
		text-align: center;
	}

	.spinner {
		width: 40px;
		height: 40px;
		border: 3px solid rgba(148, 163, 184, 0.2);
		border-top-color: var(--primary-500);
		border-radius: 50%;
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}
</style>

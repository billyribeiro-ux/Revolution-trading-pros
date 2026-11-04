<script lang="ts">
	import { SvelteSet } from 'svelte/reactivity';
	import { onMount } from 'svelte';
	import { toastStore } from '#lib/stores/toast.svelte.js';
	import { adminFetch } from '#lib/utils/adminFetch.js';
	import {
		emailApi,
		type EmailSubscriber,
		type SubscriberStats,
		type ConfirmationReceipt
	} from '#lib/api/email.js';
	import ConfirmationModal from '#lib/components/admin/ConfirmationModal.svelte';

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
	let tagFilter = $state('');
	let loadError = $state('');
	let statsError = $state('');
	let requestSequence = 0;
	let mounted = false;
	let saving = $state(false);
	const selectedSubscribers = new SvelteSet<string>();

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

	let stats = $state<SubscriberStats | null>(null);
	onMount(() => {
		mounted = true;
		void refresh();
		return () => {
			mounted = false;
			requestSequence++;
		};
	});
	function filters() {
		return {
			page,
			per_page: perPage,
			...(searchQuery.trim() && { search: searchQuery.trim() }),
			...(statusFilter !== 'all' && { status: statusFilter }),
			...(tagFilter.trim() && {
				tags: tagFilter
					.split(',')
					.map((t) => t.trim())
					.filter(Boolean)
			})
		};
	}
	async function refresh() {
		await Promise.all([loadSubscribers(), loadStats()]);
	}
	async function loadSubscribers() {
		const sequence = ++requestSequence;
		loading = true;
		loadError = '';
		try {
			const response = await emailApi.getSubscribers(filters());
			if (!mounted || sequence !== requestSequence) return;
			subscribers = response.subscribers;
			total = response.total;
			selectedSubscribers.clear();
			if (page > 1 && subscribers.length === 0 && total > 0) {
				page = Math.max(1, Math.ceil(total / perPage));
				await loadSubscribers();
			}
		} catch (err) {
			if (mounted && sequence === requestSequence)
				loadError = err instanceof Error ? err.message : 'Failed to load subscribers';
		} finally {
			if (mounted && sequence === requestSequence) loading = false;
		}
	}
	async function loadStats() {
		try {
			const result = await emailApi.getSubscriberStats();
			if (mounted) {
				stats = result;
				statsError = '';
			}
		} catch (err) {
			if (mounted) {
				stats = null;
				statsError = err instanceof Error ? err.message : 'Subscriber statistics unavailable';
			}
		}
	}
	function showConfirmation(receipt: ConfirmationReceipt) {
		const message =
			receipt.message || 'Subscriber saved as pending; recipient confirmation is required';
		if (receipt.status === 'accepted') toastStore.info(message);
		else toastStore.error(message);
	}

	async function handleAddSubscriber() {
		if (!newSubscriber.email) {
			toastStore.error('Email is required');
			return;
		}

		if (saving) return;
		saving = true;
		try {
			const tags = newSubscriber.tags
				.split(',')
				.map((t) => t.trim())
				.filter(Boolean);
			const result = await emailApi.createSubscriber({
				email: newSubscriber.email,
				...(newSubscriber.first_name && { first_name: newSubscriber.first_name }),
				...(newSubscriber.last_name && { last_name: newSubscriber.last_name }),
				tags
			});

			showConfirmation(result.confirmation);
			showAddModal = false;
			newSubscriber = { email: '', first_name: '', last_name: '', tags: '' };
			await loadSubscribers();
			await loadStats();
		} catch (err) {
			const message = err instanceof Error ? err.message : undefined;
			toastStore.error(message || 'Failed to add subscriber');
		} finally {
			saving = false;
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
			const result = await emailApi.resubscribe(id);
			showConfirmation(result.confirmation);
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
				void handleUnsubscribe(action.id);
				break;
			case 'resubscribe':
				void handleResubscribe(action.id);
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
			const result = await emailApi.bulkDeleteSubscribers([...selectedSubscribers]);
			toastStore.success(`Deleted ${result.deleted} subscribers`);
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
			const { page: _page, per_page: _size, ...exportFilters } = filters();
			const query = new URLSearchParams(
				Object.entries(exportFilters).map(([key, value]) => [key, String(value)])
			);
			const response = await adminFetch<Response>(`/api/admin/email/subscribers/export?${query}`, {
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
		if (subscribers.every((s) => selectedSubscribers.has(s.id))) {
			selectedSubscribers.clear();
		} else {
			for (const subscriber of subscribers) selectedSubscribers.add(subscriber.id);
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

	function formatDate(dateStr: string | null | undefined): string {
		if (!dateStr) return '-';
		return new Date(dateStr).toLocaleDateString('en-US', {
			month: 'short',
			day: 'numeric',
			year: 'numeric'
		});
	}

	function handleSearch() {
		page = 1;
		void loadSubscribers();
	}

	function handlePageChange(newPage: number) {
		page = newPage;
		void loadSubscribers();
	}

	let totalPages = $derived(Math.ceil(total / perPage));
</script>

<div class="subscribers-page">
	<PageHeader onexport={handleExport} onadd={() => (showAddModal = true)} />

	{#if statsError}<p role="alert">
			{statsError} <button onclick={() => void loadStats()}>Retry statistics</button>
		</p>{/if}
	<StatsGrid {stats} />

	<FiltersBar
		bind:searchQuery
		bind:statusFilter
		bind:tagFilter
		selectedCount={selectedSubscribers.size}
		onsearch={handleSearch}
		onrefresh={() => void refresh()}
		onbulkdelete={handleBulkDelete}
	/>

	<!-- Subscribers Table -->
	<div class="table-container">
		{#if loading}
			<div class="loading-state">
				<div class="spinner"></div>
				<p>Loading subscribers...</p>
			</div>
		{:else if loadError}
			<div role="alert">
				<p>{loadError}</p>
				<button onclick={() => void loadSubscribers()}>Retry subscribers</button>
			</div>
		{:else if subscribers.length === 0}
			<EmptyState onadd={() => (showAddModal = true)} />
		{:else}
			<SubscribersTable
				{subscribers}
				{selectedSubscribers}
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
		{saving}
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
		background: var(--bg-elevated, #161b22);
		border: 1px solid var(--border-muted, #21262d);
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
		color: var(--text-tertiary, #6e7681);
		text-align: center;
	}

	.spinner {
		width: 40px;
		height: 40px;
		border: 3px solid var(--border-default, #30363d);
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

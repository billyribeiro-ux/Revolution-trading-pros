<script lang="ts">
	import { onMount } from 'svelte';
	import { toastStore } from '$lib/stores/toast.svelte';
	import { adminFetch } from '$lib/utils/adminFetch';
	import {
		IconUsers,
		IconSearch,
		IconRefresh,
		IconDownload,
		IconTrash,
		IconMail,
		IconCheck,
		IconX,
		IconUserPlus
	} from '$lib/icons';
	import { emailApi, type EmailSubscriber } from '$lib/api/email';
	import ConfirmationModal from '$lib/components/admin/ConfirmationModal.svelte';

	// State
	let loading = $state(true);
	let subscribers = $state<EmailSubscriber[]>([]);
	let total = $state(0);
	let page = $state(1);
	let perPage = $state(25);
	let searchQuery = $state('');
	let statusFilter = $state('all');
	let selectedTags = $state<string[]>([]);
	let selectedSubscribers = $state(new Set<string>());

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
			const params: any = { page, per_page: perPage };
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
		} catch (err: any) {
			toastStore.error(err.message || 'Failed to add subscriber');
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
		} catch (err: any) {
			toastStore.error(err.message || 'Failed to delete subscriber');
		}
	}

	async function handleUnsubscribe(id: string) {
		try {
			await emailApi.unsubscribe(id);
			toastStore.success('Subscriber unsubscribed');
			await loadSubscribers();
			await loadStats();
		} catch (err: any) {
			toastStore.error(err.message || 'Failed to unsubscribe');
		}
	}

	async function handleResubscribe(id: string) {
		try {
			await emailApi.resubscribe(id);
			toastStore.success('Subscriber resubscribed');
			await loadSubscribers();
			await loadStats();
		} catch (err: any) {
			toastStore.error(err.message || 'Failed to resubscribe');
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
			selectedSubscribers = new Set();
			await loadSubscribers();
			await loadStats();
		} catch (err: any) {
			toastStore.error(err.message || 'Failed to delete subscribers');
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
		selectedSubscribers = selectedSubscribers;
	}

	function toggleAllSubscribers() {
		if (selectedSubscribers.size === subscribers.length) {
			selectedSubscribers = new Set();
		} else {
			selectedSubscribers = new Set(subscribers.map((s) => s.id));
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
	<!-- Header -->
	<div class="page-header">
		<div class="header-content">
			<div class="header-title">
				<div class="title-icon">
					<IconUsers size={28} />
				</div>
				<div>
					<h1>Email Subscribers</h1>
					<p class="subtitle">Manage your email audience and subscriber lists</p>
				</div>
			</div>

			<div class="header-actions">
				<button class="btn-secondary" onclick={handleExport}>
					<IconDownload size={18} />
					Export
				</button>
				<button class="btn-primary" onclick={() => (showAddModal = true)}>
					<IconUserPlus size={18} />
					Add Subscriber
				</button>
			</div>
		</div>
	</div>

	<!-- Stats -->
	<div class="stats-grid">
		<div class="stat-card">
			<div class="stat-icon blue">
				<IconUsers size={24} />
			</div>
			<div class="stat-content">
				<div class="stat-value">{stats.total.toLocaleString()}</div>
				<div class="stat-label">Total Subscribers</div>
			</div>
		</div>
		<div class="stat-card">
			<div class="stat-icon emerald">
				<IconCheck size={24} />
			</div>
			<div class="stat-content">
				<div class="stat-value">{stats.subscribed.toLocaleString()}</div>
				<div class="stat-label">Active</div>
			</div>
		</div>
		<div class="stat-card">
			<div class="stat-icon slate">
				<IconX size={24} />
			</div>
			<div class="stat-content">
				<div class="stat-value">{stats.unsubscribed.toLocaleString()}</div>
				<div class="stat-label">Unsubscribed</div>
			</div>
		</div>
		<div class="stat-card">
			<div class="stat-icon red">
				<IconMail size={24} />
			</div>
			<div class="stat-content">
				<div class="stat-value">{stats.bounced.toLocaleString()}</div>
				<div class="stat-label">Bounced</div>
			</div>
		</div>
	</div>

	<!-- Filters -->
	<div class="filters-bar">
		<div class="search-box">
			<IconSearch size={18} />
			<input
				id="page-searchquery"
				name="page-searchquery"
				type="text"
				placeholder="Search by email or name..."
				bind:value={searchQuery}
				onkeydown={(e: KeyboardEvent) => e.key === 'Enter' && handleSearch()}
			/>
		</div>

		<div class="filter-group">
			<select bind:value={statusFilter} onchange={handleSearch}>
				<option value="all">All Status</option>
				<option value="subscribed">Subscribed</option>
				<option value="unsubscribed">Unsubscribed</option>
				<option value="bounced">Bounced</option>
				<option value="complained">Complained</option>
			</select>
		</div>

		<button class="btn-icon" onclick={loadSubscribers}>
			<IconRefresh size={18} />
		</button>

		{#if selectedSubscribers.size > 0}
			<div class="bulk-actions">
				<span>{selectedSubscribers.size} selected</span>
				<button class="btn-danger small" onclick={handleBulkDelete}>
					<IconTrash size={16} />
					Delete
				</button>
			</div>
		{/if}
	</div>

	<!-- Subscribers Table -->
	<div class="table-container">
		{#if loading}
			<div class="loading-state">
				<div class="spinner"></div>
				<p>Loading subscribers...</p>
			</div>
		{:else if subscribers.length === 0}
			<div class="empty-state">
				<IconUsers size={48} stroke={1} />
				<h3>No subscribers found</h3>
				<p>Add your first subscriber or import from a CSV file</p>
				<button class="btn-primary" onclick={() => (showAddModal = true)}>
					<IconUserPlus size={18} />
					Add Subscriber
				</button>
			</div>
		{:else}
			<table class="subscribers-table">
				<thead>
					<tr>
						<th class="checkbox-col">
							<input
								id="page-checkbox"
								name="page-checkbox"
								type="checkbox"
								checked={selectedSubscribers.size === subscribers.length}
								onchange={toggleAllSubscribers}
							/>
						</th>
						<th>Subscriber</th>
						<th>Status</th>
						<th>Tags</th>
						<th>Email Score</th>
						<th>Subscribed</th>
						<th>Last Activity</th>
						<th></th>
					</tr>
				</thead>
				<tbody>
					{#each subscribers as subscriber}
						<tr>
							<td class="checkbox-col">
								<input
									id="page-checkbox"
									name="page-checkbox"
									type="checkbox"
									checked={selectedSubscribers.has(subscriber.id)}
									onchange={() => toggleSubscriber(subscriber.id)}
								/>
							</td>
							<td>
								<div class="subscriber-info">
									<div class="subscriber-avatar">
										{(subscriber.first_name?.[0] || subscriber.email?.[0] || 'U').toUpperCase()}
									</div>
									<div>
										<div class="subscriber-name">
											{subscriber.first_name || ''}
											{subscriber.last_name || ''}
										</div>
										<div class="subscriber-email">{subscriber.email || ''}</div>
									</div>
								</div>
							</td>
							<td>
								<span class="status-badge {getStatusColor(subscriber.status)}">
									{subscriber.status}
								</span>
							</td>
							<td>
								<div class="tags-list">
									{#each (subscriber.tags || []).slice(0, 3) as tag}
										<span class="tag">{tag}</span>
									{/each}
									{#if (subscriber.tags || []).length > 3}
										<span class="tag more">+{(subscriber.tags || []).length - 3}</span>
									{/if}
								</div>
							</td>
							<td>
								<div class="score-bar">
									<div class="score-fill" style="width: {subscriber.email_score}%"></div>
									<span>{subscriber.email_score}</span>
								</div>
							</td>
							<td>{formatDate(subscriber.subscribed_at)}</td>
							<td>
								{formatDate(subscriber.last_opened_at || subscriber.last_clicked_at)}
							</td>
							<td>
								<div class="row-actions">
									{#if subscriber.status === 'subscribed'}
										<button
											class="btn-icon small"
											title="Unsubscribe"
											onclick={() => handleUnsubscribe(subscriber.id)}
										>
											<IconX size={16} />
										</button>
									{:else if subscriber.status === 'unsubscribed'}
										<button
											class="btn-icon small"
											title="Resubscribe"
											onclick={() => handleResubscribe(subscriber.id)}
										>
											<IconCheck size={16} />
										</button>
									{/if}
									<button
										class="btn-icon small danger"
										title="Delete"
										onclick={() => handleDeleteSubscriber(subscriber.id)}
									>
										<IconTrash size={16} />
									</button>
								</div>
							</td>
						</tr>
					{/each}
				</tbody>
			</table>

			<!-- Pagination -->
			{#if totalPages > 1}
				<div class="pagination">
					<button
						class="btn-secondary small"
						disabled={page === 1}
						onclick={() => handlePageChange(page - 1)}
					>
						Previous
					</button>
					<span class="page-info">
						Page {page} of {totalPages} ({total.toLocaleString()} total)
					</span>
					<button
						class="btn-secondary small"
						disabled={page === totalPages}
						onclick={() => handlePageChange(page + 1)}
					>
						Next
					</button>
				</div>
			{/if}
		{/if}
	</div>
</div>

<!-- Add Subscriber Modal -->
{#if showAddModal}
	<div
		class="modal-overlay"
		role="button"
		tabindex="0"
		aria-label="Close modal"
		onclick={() => (showAddModal = false)}
		onkeydown={(e: KeyboardEvent) => {
			if (e.key === 'Escape' || e.key === 'Enter' || e.key === ' ') showAddModal = false;
		}}
	>
		<div
			class="modal-content"
			role="dialog"
			aria-modal="true"
			tabindex="-1"
			onclick={(e: MouseEvent) => e.stopPropagation()}
			onkeydown={(e: KeyboardEvent) => e.stopPropagation()}
		>
			<div class="modal-header">
				<h2>Add Subscriber</h2>
				<button class="close-btn" onclick={() => (showAddModal = false)}>
					<IconX size={20} />
				</button>
			</div>

			<div class="modal-body">
				<div class="form-group">
					<label for="email">Email Address *</label>
					<input
						id="email"
						name="email"
						autocomplete="email"
						type="email"
						bind:value={newSubscriber.email}
						placeholder="subscriber@example.com"
					/>
				</div>

				<div class="form-row">
					<div class="form-group">
						<label for="first_name">First Name</label>
						<input
							id="first_name"
							name="first_name"
							type="text"
							bind:value={newSubscriber.first_name}
						/>
					</div>
					<div class="form-group">
						<label for="last_name">Last Name</label>
						<input
							id="last_name"
							name="last_name"
							type="text"
							bind:value={newSubscriber.last_name}
						/>
					</div>
				</div>

				<div class="form-group">
					<label for="tags">Tags (comma-separated)</label>
					<input
						id="tags"
						name="tags"
						type="text"
						bind:value={newSubscriber.tags}
						placeholder="newsletter, vip"
					/>
				</div>
			</div>

			<div class="modal-footer">
				<button class="btn-secondary" onclick={() => (showAddModal = false)}>Cancel</button>
				<button class="btn-primary" onclick={handleAddSubscriber}>
					<IconUserPlus size={18} />
					Add Subscriber
				</button>
			</div>
		</div>
	</div>
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

	.page-header {
		margin-bottom: 2rem;
	}

	.header-content {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.header-title {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.title-icon {
		width: 56px;
		height: 56px;
		background: linear-gradient(135deg, var(--primary-500) 0%, var(--primary-600) 100%);
		border-radius: 16px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: white;
	}

	.header-title h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0;
	}

	.subtitle {
		color: #64748b;
		margin: 0.25rem 0 0;
	}

	.header-actions {
		display: flex;
		gap: 0.75rem;
	}

	/* Stats */
	.stats-grid {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 1rem;
		margin-bottom: 1.5rem;
	}

	.stat-card {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1.25rem;
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 16px;
	}

	.stat-icon {
		width: 48px;
		height: 48px;
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.stat-icon.blue {
		background: rgba(59, 130, 246, 0.15);
		color: #60a5fa;
	}
	.stat-icon.emerald {
		background: rgba(16, 185, 129, 0.15);
		color: #34d399;
	}
	.stat-icon.slate {
		background: rgba(148, 163, 184, 0.15);
		color: #94a3b8;
	}
	.stat-icon.red {
		background: rgba(239, 68, 68, 0.15);
		color: #f87171;
	}

	.stat-content .stat-value {
		font-size: 1.5rem;
		font-weight: 700;
		color: #f1f5f9;
	}

	.stat-content .stat-label {
		font-size: 0.8125rem;
		color: #64748b;
	}

	/* Filters */
	.filters-bar {
		display: flex;
		align-items: center;
		gap: 1rem;
		margin-bottom: 1.5rem;
		padding: 1rem;
		background: rgba(30, 41, 59, 0.4);
		border-radius: 12px;
	}

	.search-box {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		flex: 1;
		max-width: 400px;
		padding: 0.5rem 1rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		color: #94a3b8;
	}

	.search-box input {
		flex: 1;
		background: none;
		border: none;
		color: #f1f5f9;
		font-size: 0.9375rem;
	}

	.search-box input:focus {
		outline: none;
	}

	.filter-group select {
		padding: 0.625rem 1rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		color: #f1f5f9;
		font-size: 0.875rem;
	}

	.bulk-actions {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		margin-left: auto;
		padding-left: 1rem;
		border-left: 1px solid rgba(148, 163, 184, 0.2);
		color: #94a3b8;
		font-size: 0.875rem;
	}

	/* Table */
	.table-container {
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 16px;
		overflow: hidden;
	}

	.subscribers-table {
		width: 100%;
		border-collapse: collapse;
	}

	.subscribers-table th,
	.subscribers-table td {
		padding: 1rem;
		text-align: left;
		border-bottom: 1px solid rgba(148, 163, 184, 0.1);
	}

	.subscribers-table th {
		font-size: 0.75rem;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		color: #64748b;
		background: rgba(15, 23, 42, 0.4);
	}

	.subscribers-table tbody tr:hover {
		background: rgba(148, 163, 184, 0.05);
	}

	.checkbox-col {
		width: 40px;
	}

	.subscriber-info {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.subscriber-avatar {
		width: 36px;
		height: 36px;
		background: linear-gradient(135deg, var(--primary-500) 0%, var(--primary-600) 100%);
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		color: white;
		font-weight: 600;
		font-size: 0.875rem;
	}

	.subscriber-name {
		font-weight: 500;
		color: #f1f5f9;
	}

	.subscriber-email {
		font-size: 0.8125rem;
		color: #64748b;
	}

	.status-badge {
		padding: 0.25rem 0.75rem;
		border-radius: 9999px;
		font-size: 0.75rem;
		font-weight: 600;
		text-transform: capitalize;
	}

	.tags-list {
		display: flex;
		flex-wrap: wrap;
		gap: 0.25rem;
	}

	.tag {
		padding: 0.125rem 0.5rem;
		background: rgba(230, 184, 0, 0.15);
		color: var(--primary-400);
		border-radius: 4px;
		font-size: 0.6875rem;
	}

	.tag.more {
		background: rgba(148, 163, 184, 0.15);
		color: #94a3b8;
	}

	.score-bar {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.score-bar .score-fill {
		height: 6px;
		width: 60px;
		background: rgba(148, 163, 184, 0.2);
		border-radius: 3px;
		position: relative;
	}

	.score-bar .score-fill::after {
		content: '';
		position: absolute;
		left: 0;
		top: 0;
		height: 100%;
		width: var(--width, 0%);
		background: linear-gradient(90deg, var(--primary-500), var(--primary-600));
		border-radius: 3px;
	}

	.score-bar span {
		font-size: 0.8125rem;
		color: #94a3b8;
	}

	.row-actions {
		display: flex;
		gap: 0.25rem;
	}

	/* Pagination */
	.pagination {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 1rem;
		padding: 1rem;
		border-top: 1px solid rgba(148, 163, 184, 0.1);
	}

	.page-info {
		color: #64748b;
		font-size: 0.875rem;
	}

	/* Empty & Loading states */
	.empty-state,
	.loading-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 2rem;
		color: #64748b;
		text-align: center;
	}

	.empty-state h3 {
		color: #f1f5f9;
		margin: 1rem 0 0.5rem;
	}

	.empty-state p {
		margin-bottom: 1.5rem;
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

	/* Buttons */
	.btn-primary,
	.btn-secondary,
	.btn-danger {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
		border: none;
		font-size: 0.875rem;
	}

	.btn-primary {
		background: linear-gradient(135deg, var(--primary-500) 0%, var(--primary-600) 100%);
		color: var(--bg-base);
	}

	.btn-secondary {
		background: rgba(148, 163, 184, 0.1);
		color: #94a3b8;
		border: 1px solid rgba(148, 163, 184, 0.2);
	}

	.btn-danger {
		background: rgba(239, 68, 68, 0.1);
		color: #f87171;
		border: 1px solid rgba(239, 68, 68, 0.2);
	}

	.btn-icon {
		width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(148, 163, 184, 0.1);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		color: #94a3b8;
		cursor: pointer;
	}

	.btn-icon.small {
		width: 28px;
		height: 28px;
	}

	.btn-icon.danger {
		color: #f87171;
	}

	.btn-primary.small,
	.btn-secondary.small,
	.btn-danger.small {
		padding: 0.5rem 0.875rem;
		font-size: 0.8125rem;
	}

	/* Modal */
	.modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.8);
		backdrop-filter: blur(4px);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 50;
		padding: 2rem;
	}

	.modal-content {
		background: #1e293b;
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 20px;
		width: 100%;
		max-width: 500px;
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1.5rem;
		border-bottom: 1px solid rgba(148, 163, 184, 0.1);
	}

	.modal-header h2 {
		font-size: 1.25rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0;
	}

	.close-btn {
		width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(148, 163, 184, 0.1);
		border: none;
		border-radius: 8px;
		color: #94a3b8;
		cursor: pointer;
	}

	.modal-body {
		padding: 1.5rem;
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.modal-footer {
		display: flex;
		justify-content: flex-end;
		gap: 0.75rem;
		padding: 1.5rem;
		border-top: 1px solid rgba(148, 163, 184, 0.1);
	}

	.form-group {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.form-group label {
		font-size: 0.8125rem;
		font-weight: 500;
		color: #94a3b8;
	}

	.form-group input {
		padding: 0.75rem 1rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 10px;
		color: #f1f5f9;
		font-size: 0.9375rem;
	}

	.form-group input:focus {
		outline: none;
		border-color: rgba(230, 184, 0, 0.5);
	}

	.form-row {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1rem;
	}

	@media (max-width: 768px) {
		.stats-grid {
			grid-template-columns: repeat(2, 1fr);
		}

		.header-content {
			flex-direction: column;
			gap: 1rem;
			align-items: flex-start;
		}

		.filters-bar {
			flex-wrap: wrap;
		}

		.search-box {
			width: 100%;
			max-width: none;
		}
	}
</style>

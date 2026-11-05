<script lang="ts">
	import { onMount } from 'svelte';
	import {
		IconMail,
		IconMailForward,
		IconPlus,
		IconSearch,
		IconEdit,
		IconTrash,
		IconPlayerPlay,
		IconPlayerPause,
		IconCopy,
		IconRefresh,
		IconUsers,
		IconChartBar,
		IconClock,
		IconX,
		IconSend,
		IconCheck,
		IconAlertCircle
	} from '#lib/icons/index.js';
	import { crmAPI } from '#lib/api/crm.js';
	import { api } from '#lib/api/config.js';
	import type { EmailSequence, SequenceFilters, SequenceStatus } from '#lib/crm/types.js';
	import ConfirmationModal from '#lib/components/admin/ConfirmationModal.svelte';

	// STATE

	let sequences = $state<EmailSequence[]>([]);
	let isLoading = $state(true);
	let error = $state('');
	let searchQuery = $state('');
	let selectedStatus = $state<SequenceStatus | 'all'>('all');
	let isInitialized = $state(false);
	let currentPage = $state(1);
	let totalPages = $state(1);
	let loadGeneration = 0;

	let stats = $state({
		total: 0,
		active: 0,
		totalSubscribers: 0,
		totalSent: 0
	});

	// Modal States
	let showSendEmailModal = $state(false);
	let selectedSequence = $state<EmailSequence | null>(null);
	let sendEmailForm = $state({
		testEmail: '',
		isLoading: false,
		success: false,
		error: ''
	});

	// Action States
	let actionInProgress = $state<string | null>(null);
	const showStats = $derived(!isLoading && !error);

	// Delete confirmation modal state
	let showDeleteModal = $state(false);
	let pendingDeleteId = $state<string | null>(null);

	const statusOptions = [
		{ value: 'all', label: 'All Sequences' },
		{ value: 'draft', label: 'Draft' },
		{ value: 'active', label: 'Active' },
		{ value: 'paused', label: 'Paused' },
		{ value: 'completed', label: 'Completed' }
	] as const;

	// API FUNCTIONS

	async function loadSequences() {
		const generation = ++loadGeneration;
		isLoading = true;
		error = '';

		try {
			const filters: SequenceFilters = {
				search: searchQuery || undefined, page: currentPage, per_page: 20,
				status: selectedStatus !== 'all' ? selectedStatus : undefined
			};

			const response = await crmAPI.getSequences(filters);
			if (generation !== loadGeneration) return;
			const lastPage = response.meta.total_pages ?? 1;
			if (currentPage > lastPage) {
				currentPage = lastPage;
				await loadSequences();
				return;
			}
			sequences = response.data;
			totalPages = lastPage;

			stats = {
				total: response.meta.total,
				active: sequences.filter((s) => s.status === 'active').length,
				totalSubscribers: sequences.reduce((sum, s) => sum + s.subscribers_count, 0),
				totalSent: sequences.reduce((sum, s) => sum + s.total_sent, 0)
			};
		} catch (err) {
			if (generation === loadGeneration) error = err instanceof Error ? err.message : 'Failed to load sequences';
		} finally {
			if (generation === loadGeneration) isLoading = false;
		}
	}

	function changePage(by: number) {
		const next = currentPage + by;
		if (isLoading || actionInProgress || next < 1 || next > totalPages) return;
		currentPage = next;
		void loadSequences();
	}

	function scheduleSequencesReload() {
		loadGeneration += 1; currentPage = 1; isLoading = true;
		if (searchTimeout) {
			clearTimeout(searchTimeout);
		}

		if (!isInitialized) return;

		searchTimeout = setTimeout(() => {
			void loadSequences();
		}, 300);
	}

	function handleSearchInput(value: string) {
		searchQuery = value;
		scheduleSequencesReload();
	}

	function clearSearch() {
		searchQuery = '';
		scheduleSequencesReload();
	}

	function handleStatusChange(value: SequenceStatus | 'all') {
		selectedStatus = value; currentPage = 1; loadGeneration += 1;

		if (searchTimeout) {
			clearTimeout(searchTimeout);
			searchTimeout = null;
		}

		if (isInitialized) {
			void loadSequences();
		}
	}

	async function duplicateSequence(id: string) {
		if (actionInProgress) return;
		actionInProgress = id;
		try {
			await crmAPI.duplicateSequence(id);
			await loadSequences();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to duplicate sequence';
		} finally {
			actionInProgress = null;
		}
	}

	function deleteSequence(id: string) {
		if (actionInProgress) return;
		pendingDeleteId = id;
		showDeleteModal = true;
	}

	async function confirmDeleteSequence() {
		if (!pendingDeleteId || actionInProgress) return;
		showDeleteModal = false;
		const id = pendingDeleteId;
		pendingDeleteId = null;

		actionInProgress = id;
		try {
			await crmAPI.deleteSequence(id);
			await loadSequences();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to delete sequence';
		} finally {
			actionInProgress = null;
		}
	}

	async function toggleSequenceStatus(sequence: EmailSequence) {
		if (actionInProgress) return;
		actionInProgress = sequence.id;
		try {
			const newStatus: SequenceStatus = sequence.status === 'active' ? 'paused' : 'active';
			await crmAPI.updateSequence(sequence.id, { status: newStatus, expected_version: sequence.authoring_version });
			await loadSequences();
		} catch (err) {
			error =
				err instanceof Error
					? err.message
					: `Failed to ${sequence.status === 'active' ? 'pause' : 'activate'} sequence`;
		} finally {
			actionInProgress = null;
		}
	}

	async function sendTestEmail() {
		if (!selectedSequence || !sendEmailForm.testEmail) return;

		sendEmailForm.isLoading = true;
		sendEmailForm.error = '';
		sendEmailForm.success = false;

		try {
			// Audit P1 #4: this used to fake a `setTimeout` "success" so the
			// user saw "Test email sent successfully" even though nothing was
			// dispatched. Now we actually POST to the test-email endpoint and
			// surface whatever the backend returns. If the endpoint isn't yet
			// wired up server-side, the user sees an honest error instead of
			// a green checkmark.
			const emails = await crmAPI.getSequenceEmails(selectedSequence.id);
			if (emails.length === 0) {
				sendEmailForm.error = 'This sequence has no emails to send';
				return;
			}

			await api.post(`/api/admin/crm/sequences/${selectedSequence.id}/test`, {
				email: sendEmailForm.testEmail,
				email_id: emails[0]?.id
			});

			sendEmailForm.success = true;
			setTimeout(() => {
				closeSendEmailModal();
			}, 2000);
		} catch (err) {
			sendEmailForm.error = err instanceof Error ? err.message : 'Failed to send test email';
		} finally {
			sendEmailForm.isLoading = false;
		}
	}

	// MODAL FUNCTIONS

	function openSendEmailModal(sequence: EmailSequence) {
		selectedSequence = sequence;
		sendEmailForm = {
			testEmail: '',
			isLoading: false,
			success: false,
			error: ''
		};
		showSendEmailModal = true;
	}

	function closeSendEmailModal() {
		showSendEmailModal = false;
		selectedSequence = null;
		sendEmailForm = {
			testEmail: '',
			isLoading: false,
			success: false,
			error: ''
		};
	}

	function handleModalKeydown(event: KeyboardEvent) {
		if (event.key === 'Escape') {
			closeSendEmailModal();
		}
	}

	function handleModalBackdropClick(event: MouseEvent) {
		if (event.target === event.currentTarget) {
			closeSendEmailModal();
		}
	}

	// HELPERS

	function formatNumber(num: number): string {
		return num.toLocaleString();
	}

	function formatRate(sent: number, opened: number): string {
		if (sent === 0) return 'Unavailable';
		return ((opened / sent) * 100).toFixed(1) + '%';
	}

	function getStatusClass(status: SequenceStatus): string {
		const statusClasses: Record<SequenceStatus, string> = {
			draft: 'status-draft',
			active: 'status-active',
			paused: 'status-paused',
			completed: 'status-completed'
		};
		return statusClasses[status];
	}

	function getStatusIcon(status: SequenceStatus) {
		const icons: Record<SequenceStatus, typeof IconEdit> = {
			draft: IconEdit,
			active: IconPlayerPlay,
			paused: IconPlayerPause,
			completed: IconCheck
		};
		return icons[status];
	}

	function isValidEmail(email: string): boolean {
		return /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email);
	}

	// DERIVED STATE


	let canSendTestEmail = $derived(
		sendEmailForm.testEmail.length > 0 &&
			isValidEmail(sendEmailForm.testEmail) &&
			!sendEmailForm.isLoading
	);

	// Debounced search timer. Cleared from the onMount cleanup below.
	let searchTimeout: ReturnType<typeof setTimeout> | null = null;

	// LIFECYCLE

	// Svelte 5: Initialize on mount.
	// Was previously a `$effect` — migrated to `onMount` per audit P2 #10
	// to keep one-shot init off the reactive graph.
	onMount(() => {
		void loadSequences();
		isInitialized = true;

		return () => {
			loadGeneration += 1; isInitialized = false;
			if (searchTimeout) {
				clearTimeout(searchTimeout);
			}
		};
	});
</script>

<!--
	/admin/crm/sequences - Email Drip Sequences
	Apple Principal Engineer ICT 7 Grade - January 2026

	Features:
	- Automated email sequence management
	- Status filtering (draft, active, paused, completed)
	- Open/click rate tracking
	- Subscriber count per sequence
	- Duplicate and delete functionality
	- Send test email modal
	- Pause/Resume sequence controls
	- Debounced reload on search/filter changes
	- Full Svelte 5 $state/$derived reactivity
-->

<!-- Handle escape key for modal -->

<svelte:window onkeydown={showSendEmailModal ? handleModalKeydown : undefined} />

<div class="admin-crm-sequences">
	

	<div class="admin-page-container">
		<!-- Header -->
		<header class="page-header">
			<h1>Email Sequences</h1>
			<p class="subtitle">Create automated drip campaigns for your contacts</p>
			<div class="header-actions">
				<button class="btn-refresh" onclick={() => loadSequences()} disabled={isLoading}>
					<IconRefresh size={18} class={isLoading ? 'spinning' : ''} />
				</button>
				<a href="/admin/crm/sequences/new" class="btn-primary">
					<IconPlus size={18} />
					New Sequence
				</a>
			</div>
		</header>

		<!-- Stats Cards -->
		<div class="stats-grid">
			<div class="stat-card">
				<div class="stat-icon blue">
					<IconMail size={24} />
				</div>
				<div class="stat-content">
					<span class="stat-value">{showStats ? formatNumber(stats.total) : '—'}</span>
					<span class="stat-label">Matching Sequences</span>
				</div>
			</div>
			<div class="stat-card">
				<div class="stat-icon green">
					<IconPlayerPlay size={24} />
				</div>
				<div class="stat-content">
					<span class="stat-value">{showStats ? formatNumber(stats.active) : '—'}</span>
					<span class="stat-label">Active on this page</span>
				</div>
			</div>
			<div class="stat-card">
				<div class="stat-icon purple">
					<IconUsers size={24} />
				</div>
				<div class="stat-content">
					<span class="stat-value">{showStats ? formatNumber(stats.totalSubscribers) : '—'}</span>
					<span class="stat-label">Enrollments on this page</span>
				</div>
			</div>
			<div class="stat-card">
				<div class="stat-icon amber">
					<IconMailForward size={24} />
				</div>
				<div class="stat-content">
					<span class="stat-value">{showStats ? formatNumber(stats.totalSent) : '—'}</span>
					<span class="stat-label">Accepted on this page</span>
				</div>
			</div>
		</div>

		<!-- Search & Filters -->
		<div class="filters-bar">
			<div class="search-box">
				<IconSearch size={18} />
				<input autocomplete="off"
					id="page-searchquery"
					name="page-searchquery"
					type="text"
					placeholder="Search sequences..."
					bind:value={searchQuery}
					oninput={(event) => handleSearchInput(event.currentTarget.value)}
					aria-label="Search sequences"
				/>
				{#if searchQuery}
					<button class="search-clear" onclick={clearSearch} aria-label="Clear search">
						<IconX size={14} />
					</button>
				{/if}
			</div>
			<select id="sequences-selectedstatus" name="sequences_selectedstatus"
				class="filter-select"
				bind:value={selectedStatus}
				onchange={(event) =>
					handleStatusChange(event.currentTarget.value as SequenceStatus | 'all')}
				aria-label="Filter by status"
			>
				{#each statusOptions as option (option.value)}
					<option value={option.value}>{option.label}</option>
				{/each}
			</select>
		</div>

		<!-- Error Alert -->
		{#if error}
			<div class="error-alert">
				<IconAlertCircle size={18} />
				<span>{error}</span>

				<button onclick={() => (error = '')} aria-label="Dismiss error"><IconX size={16} /></button>
			</div>
		{/if}

		<!-- Sequences Table -->
		{#if isLoading && !isInitialized}
			<div class="loading-state">
				<div class="spinner"></div>
				<p>Loading sequences...</p>
			</div>
		{:else if sequences.length === 0}
			<div class="empty-state">
				<IconMail size={48} />
				<h3>No sequences found</h3>
				<p>Create your first email sequence to start automating your campaigns</p>
				<a href="/admin/crm/sequences/new" class="btn-primary">
					<IconPlus size={18} />
					Create Sequence
				</a>
			</div>
		{:else}
			<div class={['table-container', { loading: isLoading }]}>
				<table class="data-table">
					<thead>
						<tr>
							<th>Sequence</th>
							<th>Status</th>
							<th>Emails</th>
							<th>Subscribers</th>
							<th>Open Rate</th>
							<th>Click Rate</th>
							<th>Actions</th>
						</tr>
					</thead>
					<tbody>
						{#each sequences as sequence (sequence.id)}
							{@const StatusIcon = getStatusIcon(sequence.status)}
							<tr class={{ 'action-in-progress': actionInProgress === sequence.id }}>
								<td>
									<div class="sequence-cell">
										<div class="sequence-icon">
											<IconMail size={20} />
										</div>
										<div class="sequence-info">
											<span class="sequence-title">{sequence.title}</span>
											<span class="sequence-meta">
												<IconClock size={12} />
												{sequence.emails_count} emails
											</span>
										</div>
									</div>
								</td>
								<td>
									<span class={['status-badge', getStatusClass(sequence.status)]}>
										<StatusIcon size={12} />
										{sequence.status}
									</span>
								</td>
								<td>{sequence.emails_count ?? 'Needs review'}</td>
								<td>{formatNumber(sequence.subscribers_count)}</td>
								<td
									><span class="rate-value"
										>{formatRate(sequence.total_sent, sequence.total_opened)}</span
									></td
								>
								<td
									><span class="rate-value"
										>{formatRate(sequence.total_sent, sequence.total_clicked)}</span
									></td
								>

								<td>
									<div class="action-buttons">
										{#if sequence.status === 'draft' || sequence.status === 'active' || sequence.status === 'paused'}
											<button
												class="btn-icon"
												title={sequence.status === 'active' ? 'Pause' : 'Activate'}
												onclick={() => toggleSequenceStatus(sequence)}
												disabled={actionInProgress !== null}
											>
												{#if sequence.status === 'active'}
													<IconPlayerPause size={16} />
												{:else}
													<IconPlayerPlay size={16} />
												{/if}
											</button>
										{/if}
										<a
											href="/admin/crm/sequences/{sequence.id}"
											class="btn-icon"
											title="View Analytics"
										>
											<IconChartBar size={16} />
										</a>
										<a
											href="/admin/crm/sequences/{sequence.id}/subscribers"
											class="btn-icon"
											title="View Subscribers"
										>
											<IconUsers size={16} />
										</a>
										<button
											class="btn-icon"
											title="Send Test Email"
											onclick={() => openSendEmailModal(sequence)}
										>
											<IconSend size={16} />
										</button>
										<a href="/admin/crm/sequences/{sequence.id}/edit" class="btn-icon" title="Edit">
											<IconEdit size={16} />
										</a>
										<button
											class="btn-icon"
											title="Duplicate"
											onclick={() => duplicateSequence(sequence.id)}
											disabled={actionInProgress !== null}
										>
											<IconCopy size={16} />
										</button>
										<button
											class="btn-icon danger"
											title="Delete"
											onclick={() => deleteSequence(sequence.id)}
											disabled={actionInProgress !== null}
										>
											<IconTrash size={16} />
										</button>
									</div>
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		{/if}
	<nav aria-label="Sequence pages"><button onclick={() => changePage(-1)} disabled={currentPage <= 1 || isLoading}>Previous page</button><span> Page {currentPage} of {totalPages} </span><button onclick={() => changePage(1)} disabled={currentPage >= totalPages || isLoading}>Next page</button></nav>
	</div>
	<!-- End admin-page-container -->
</div>

<!-- Send Test Email Modal -->
{#if showSendEmailModal}
	<div
		class="modal-backdrop"
		onclick={handleModalBackdropClick}
		onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && closeSendEmailModal()}
		role="dialog"
		aria-modal="true"
		aria-labelledby="modal-title"
		tabindex="-1"
	>
		<div class="modal-container">
			<div class="modal-header">
				<h2 id="modal-title">
					<IconSend size={20} />
					Send Test Email
				</h2>
				<button class="modal-close" onclick={closeSendEmailModal} aria-label="Close modal">
					<IconX size={20} />
				</button>
			</div>

			<div class="modal-body">
				{#if selectedSequence}
					<p class="modal-description">
						Send a test email from the sequence "<strong>{selectedSequence.title}</strong>" to
						verify your content and design.
					</p>

					{#if sendEmailForm.success}
						<div class="success-message">
							<IconCheck size={20} />
							<span>Test email sent successfully!</span>
						</div>
					{:else}
						<div class="form-group">
							<label for="test-email">Email Address</label>
							<input
								id="test-email"
								name="test-email"
								autocomplete="email"
								type="email"
								placeholder="Enter email address..."
								bind:value={sendEmailForm.testEmail}
								disabled={sendEmailForm.isLoading}
								class={{ error: !!sendEmailForm.error }}
							/>
							{#if sendEmailForm.error}
								<span class="error-message">{sendEmailForm.error}</span>
							{/if}
						</div>

						<div class="modal-info">
							<IconAlertCircle size={16} />
							<span>This will send the first email in the sequence as a test.</span>
						</div>
					{/if}
				{/if}
			</div>

			{#if !sendEmailForm.success}
				<div class="modal-footer">
					<button
						class="btn-secondary"
						onclick={closeSendEmailModal}
						disabled={sendEmailForm.isLoading}>Cancel</button
					>

					<button class="btn-primary" onclick={sendTestEmail} disabled={!canSendTestEmail}>
						{#if sendEmailForm.isLoading}
							<div class="btn-spinner"></div>
							Sending...
						{:else}
							<IconSend size={16} />
							Send Test
						{/if}
					</button>
				</div>
			{/if}
		</div>
	</div>
{/if}

<ConfirmationModal
	isOpen={showDeleteModal}
	title="Delete Sequence"
	message="Are you sure you want to delete this sequence? This action cannot be undone."
	confirmText="Delete"
	variant="danger"
	onConfirm={confirmDeleteSequence}
	onCancel={() => {
		showDeleteModal = false;
		pendingDeleteId = null;
	}}
/>

<style>
	/* =====================================================
	   Layout & Container
	   ===================================================== */
	.page {
		max-width: 1400px;
		margin: 0 auto;
		padding: 2rem;
	}

	/* =====================================================
	   Page Header - CENTERED
	   ===================================================== */
	.page-header {
		text-align: center;
		margin-bottom: 2rem;
	}

	.page-header h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: var(--text-primary, #f0f6fc);
		margin: 0 0 0.5rem 0;
	}

	.subtitle {
		color: var(--text-tertiary, #6e7681);
		font-size: 0.875rem;
		margin: 0 0 1.25rem 0;
	}

	.header-actions {
		display: flex;
		justify-content: center;
		gap: 0.75rem;
	}

	.btn-refresh {
		width: 42px;
		height: 42px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: var(--bg-elevated, #161b22);
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 10px;
		color: var(--text-secondary, #8b949e);
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-refresh:hover {
		background: rgba(230, 184, 0, 0.2);
		color: var(--primary-400);
	}

	.btn-refresh:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.btn-refresh :global(.spinning) {
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}

	.btn-primary {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background: var(--primary-500);
		color: var(--bg-base);
		border: none;
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
		text-decoration: none;
	}


	.btn-primary:disabled {
		opacity: 0.5;
		cursor: not-allowed;
		transform: none;
	}

	.btn-secondary {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background: var(--bg-hover, #252b33);
		border: 1px solid var(--border-default, #30363d);
		color: var(--text-secondary, #8b949e);
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-secondary:hover:not(:disabled) {
		background: var(--bg-hover, #252b33);
		border-color: var(--border-default, #30363d);
	}

	.btn-secondary:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	/* Stats Grid */
	.stats-grid {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 1rem;
		margin-bottom: 2rem;
	}

	@media (max-width: 1199.98px) {
		.stats-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	@media (max-width: 639.98px) {
		.stats-grid {
			grid-template-columns: 1fr;
		}
		.page {
			padding: 1rem;
		}
	}

	.stat-card {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1.25rem;
		background: var(--bg-elevated, #161b22);
		border: 1px solid rgba(230, 184, 0, 0.1);
		border-radius: 14px;
	}

	.stat-icon {
		width: 52px;
		height: 52px;
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.stat-icon.blue {
		background: rgba(88, 166, 255, 0.15);
		color: var(--info-emphasis, #58a6ff);
	}
	.stat-icon.green {
		background: rgba(63, 185, 80, 0.15);
		color: var(--success-emphasis, #3fb950);
	}
	.stat-icon.purple {
		background: rgba(230, 184, 0, 0.15);
		color: var(--primary-400);
	}
	.stat-icon.amber {
		background: rgba(210, 153, 34, 0.15);
		color: var(--warning-emphasis, #d29922);
	}

	.stat-content {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.stat-value {
		font-size: 1.5rem;
		font-weight: 700;
		color: var(--text-primary, #f0f6fc);
	}

	.stat-label {
		font-size: 0.8rem;
		color: var(--text-tertiary, #6e7681);
	}

	/* Filters */
	.filters-bar {
		display: flex;
		gap: 1rem;
		margin-bottom: 1.5rem;
		flex-wrap: wrap;
	}

	.search-box {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0 1rem;
		background: var(--bg-base, #0d1117);
		border: 1px solid rgba(230, 184, 0, 0.1);
		border-radius: 10px;
		flex: 1;
		max-width: 400px;
	}

	.search-box :global(svg) {
		color: var(--text-tertiary, #6e7681);
		flex-shrink: 0;
	}

	.search-box input {
		flex: 1;
		padding: 0.75rem 0;
		background: transparent;
		border: none;
		color: var(--text-primary, #f0f6fc);
		font-size: 0.9rem;
		outline: none;
	}

	.search-box input::placeholder {
		color: var(--text-tertiary, #6e7681);
	}

	.search-clear {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 20px;
		height: 20px;
		background: rgba(230, 184, 0, 0.2);
		border: none;
		border-radius: 4px;
		color: var(--text-secondary, #8b949e);
		cursor: pointer;
		transition: all 0.2s;
	}

	.search-clear:hover {
		background: rgba(230, 184, 0, 0.3);
		color: var(--text-primary, #f0f6fc);
	}

	.filter-select {
		padding: 0.75rem 1rem;
		background: var(--bg-base, #0d1117);
		border: 1px solid rgba(230, 184, 0, 0.1);
		border-radius: 10px;
		color: var(--text-primary, #f0f6fc);
		font-size: 0.9rem;
		cursor: pointer;
	}

	/* Error Alert */
	.error-alert {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem;
		background: rgba(248, 81, 73, 0.1);
		border: 1px solid rgba(248, 81, 73, 0.3);
		border-radius: 10px;
		color: var(--error-emphasis, #f85149);
		margin-bottom: 1.5rem;
	}

	.error-alert span {
		flex: 1;
	}

	.error-alert button {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 24px;
		height: 24px;
		background: transparent;
		border: none;
		color: var(--error-emphasis, #f85149);
		cursor: pointer;
		opacity: 0.7;
		transition: opacity 0.2s;
	}

	.error-alert button:hover {
		opacity: 1;
	}

	/* Table */
	.table-container {
		background: var(--bg-base, #0d1117);
		border: 1px solid rgba(230, 184, 0, 0.1);
		border-radius: 14px;
		overflow: hidden;
		transition: opacity 0.2s;
	}

	.table-container.loading {
		opacity: 0.6;
		pointer-events: none;
	}

	.data-table {
		width: 100%;
		border-collapse: collapse;
	}

	.data-table th {
		padding: 1rem;
		text-align: left;
		font-size: 0.75rem;
		font-weight: 600;
		color: var(--text-tertiary, #6e7681);
		text-transform: uppercase;
		letter-spacing: 0.05em;
		background: rgba(230, 184, 0, 0.05);
		border-bottom: 1px solid rgba(230, 184, 0, 0.1);
	}

	.data-table td {
		padding: 1rem;
		font-size: 0.9rem;
		color: var(--text-primary, #f0f6fc);
		border-bottom: 1px solid rgba(230, 184, 0, 0.05);
	}

	.data-table tbody tr:hover {
		background: rgba(230, 184, 0, 0.05);
	}

	.data-table tbody tr.action-in-progress {
		opacity: 0.5;
		pointer-events: none;
	}

	.sequence-cell {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.sequence-icon {
		width: 40px;
		height: 40px;
		border-radius: 10px;
		background: var(--primary-500);
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--bg-base, #0d1117);
		flex-shrink: 0;
	}

	.sequence-info {
		display: flex;
		flex-direction: column;
		gap: 0.125rem;
	}

	.sequence-title {
		font-weight: 600;
		color: var(--text-primary, #f0f6fc);
	}

	.sequence-meta {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		font-size: 0.75rem;
		color: var(--text-tertiary, #6e7681);
	}

	.status-badge {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.25rem 0.75rem;
		border-radius: 20px;
		font-size: 0.75rem;
		font-weight: 600;
		text-transform: capitalize;
	}

	.status-badge.status-draft {
		background: var(--bg-hover, #252b33);
		color: var(--text-secondary, #8b949e);
	}

	.status-badge.status-active {
		background: rgba(63, 185, 80, 0.2);
		color: var(--success-emphasis, #3fb950);
	}

	.status-badge.status-paused {
		background: rgba(210, 153, 34, 0.2);
		color: var(--warning-emphasis, #d29922);
	}

	.status-badge.status-completed {
		background: rgba(88, 166, 255, 0.2);
		color: var(--info-emphasis, #58a6ff);
	}

	.rate-value {
		font-weight: 600;
		color: var(--success-emphasis, #3fb950);
	}

	.action-buttons {
		display: flex;
		gap: 0.5rem;
		flex-wrap: wrap;
	}

	.btn-icon {
		width: 32px;
		height: 32px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: transparent;
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 8px;
		color: var(--text-secondary, #8b949e);
		cursor: pointer;
		transition: all 0.2s;
		text-decoration: none;
	}

	.btn-icon:hover:not(:disabled) {
		background: rgba(230, 184, 0, 0.1);
		color: var(--primary-400, #ffd11a);
	}

	.btn-icon:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.btn-icon.danger:hover:not(:disabled) {
		background: rgba(248, 81, 73, 0.1);
		color: var(--error-emphasis, #f85149);
		border-color: rgba(248, 81, 73, 0.3);
	}

	/* States */
	.loading-state,
	.error-state,
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 2rem;
		text-align: center;
		color: var(--text-tertiary, #6e7681);
	}

	.empty-state :global(svg) {
		color: var(--text-tertiary, #6e7681);
		margin-bottom: 1rem;
	}

	.empty-state h3 {
		color: var(--text-primary, #f0f6fc);
		margin: 0 0 0.5rem 0;
	}

	.empty-state p {
		margin: 0 0 1.5rem 0;
	}

	.spinner {
		width: 40px;
		height: 40px;
		border: 3px solid rgba(230, 184, 0, 0.2);
		border-top-color: var(--primary-500);
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin-bottom: 1rem;
	}

	/* Modal */
	.modal-backdrop {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.7);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
		padding: 1rem;
	}

	.modal-container {
		background: var(--bg-elevated, #161b22);
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 16px;
		width: 100%;
		max-width: 480px;
		max-height: 90vh;
		overflow: hidden;
		box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5);
	}

	.modal-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 1.25rem 1.5rem;
		border-bottom: 1px solid rgba(230, 184, 0, 0.1);
	}

	.modal-header h2 {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		margin: 0;
		font-size: 1.125rem;
		font-weight: 600;
		color: var(--text-primary, #f0f6fc);
	}

	.modal-header h2 :global(svg) {
		color: var(--primary-500);
	}

	.modal-close {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		background: transparent;
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 8px;
		color: var(--text-secondary, #8b949e);
		cursor: pointer;
		transition: all 0.2s;
	}

	.modal-close:hover {
		background: rgba(230, 184, 0, 0.1);
		color: var(--text-primary, #f0f6fc);
	}

	.modal-body {
		padding: 1.5rem;
	}

	.modal-description {
		color: var(--text-secondary, #8b949e);
		margin: 0 0 1.5rem 0;
		line-height: 1.6;
	}

	.modal-description strong {
		color: var(--text-primary, #f0f6fc);
	}

	.form-group {
		margin-bottom: 1rem;
	}

	.form-group label {
		display: block;
		margin-bottom: 0.5rem;
		font-size: 0.875rem;
		font-weight: 500;
		color: var(--text-primary, #f0f6fc);
	}

	.form-group input {
		width: 100%;
		padding: 0.75rem 1rem;
		background: var(--bg-base, #0d1117);
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 10px;
		color: var(--text-primary, #f0f6fc);
		font-size: 0.9rem;
		outline: none;
		transition: border-color 0.2s;
	}

	.form-group input:focus {
		border-color: rgba(230, 184, 0, 0.5);
	}

	.form-group input.error {
		border-color: rgba(248, 81, 73, 0.5);
	}

	.form-group input:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.form-group input::placeholder {
		color: var(--text-tertiary, #6e7681);
	}

	.error-message {
		display: block;
		margin-top: 0.5rem;
		font-size: 0.8rem;
		color: var(--error-emphasis, #f85149);
	}

	.modal-info {
		display: flex;
		align-items: flex-start;
		gap: 0.75rem;
		padding: 0.75rem 1rem;
		background: rgba(230, 184, 0, 0.1);
		border-radius: 8px;
		font-size: 0.8rem;
		color: var(--text-secondary, #8b949e);
	}

	.modal-info :global(svg) {
		color: var(--primary-500);
		flex-shrink: 0;
		margin-top: 0.125rem;
	}

	.success-message {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.75rem;
		padding: 1.5rem;
		background: rgba(63, 185, 80, 0.1);
		border: 1px solid rgba(63, 185, 80, 0.3);
		border-radius: 10px;
		color: var(--success-emphasis, #3fb950);
		font-weight: 500;
	}

	.modal-footer {
		display: flex;
		justify-content: flex-end;
		gap: 0.75rem;
		padding: 1rem 1.5rem;
		border-top: 1px solid rgba(230, 184, 0, 0.1);
		background: rgba(15, 23, 42, 0.3);
	}

	.btn-spinner {
		width: 16px;
		height: 16px;
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-top-color: white;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}
</style>

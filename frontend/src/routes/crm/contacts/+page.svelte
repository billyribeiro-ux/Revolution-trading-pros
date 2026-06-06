<!--
	URL: /crm/contacts
-->

<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/state';
	import { goto } from '$app/navigation';
	import { crmAPI } from '$lib/api/crm';
	import { crmStore } from '$lib/stores/crm.svelte';
	import type { Contact, ContactStatus, LifecycleStage } from '$lib/crm/types';
	import { IconUser, IconSearch, IconFilter, IconTrendingUp, IconAlertTriangle } from '$lib/icons';

	// Initialize filters from URL query params (ICT 7: URL-driven state)
	let localSearch = $state(page.url.searchParams.get('search') ?? '');
	let localStatus = $state<ContactStatus | 'all'>(
		(page.url.searchParams.get('status') as ContactStatus) ?? 'all'
	);
	let localStage = $state<LifecycleStage | 'all'>(
		(page.url.searchParams.get('stage') as LifecycleStage) ?? 'all'
	);

	const statusOptions: { value: ContactStatus | 'all'; label: string }[] = [
		{ value: 'all', label: 'All statuses' },
		{ value: 'lead', label: 'Leads' },
		{ value: 'prospect', label: 'Prospects' },
		{ value: 'customer', label: 'Customers' },
		{ value: 'churned', label: 'Churned' },
		{ value: 'unqualified', label: 'Unqualified' }
	];

	const stageOptions: { value: LifecycleStage | 'all'; label: string }[] = [
		{ value: 'all', label: 'All lifecycle stages' },
		{ value: 'subscriber', label: 'Subscribers' },
		{ value: 'lead', label: 'Leads' },
		{ value: 'mql', label: 'MQLs' },
		{ value: 'sql', label: 'SQLs' },
		{ value: 'opportunity', label: 'Opportunities' },
		{ value: 'customer', label: 'Customers' },
		{ value: 'evangelist', label: 'Evangelists' }
	];

	onMount(async () => {
		await loadContacts();
	});

	async function loadContacts() {
		crmStore.setLoading(true);
		crmStore.clearError();

		try {
			crmStore.contactFilters = {
				search: localSearch || undefined,
				status: localStatus !== 'all' ? localStatus : undefined,
				lifecycle_stage: localStage !== 'all' ? localStage : undefined,
				sort_by: 'created_at',
				sort_order: 'desc',
				per_page: 100
			};

			const response = await crmAPI.getContacts(crmStore.contactFilters);
			crmStore.setContacts(response.data);
		} catch (e) {
			console.error('Failed to load contacts', e);
			crmStore.setError('Failed to load contacts. Please try again.');
		} finally {
			crmStore.setLoading(false);
		}
	}

	function openContact(contact: Contact) {
		goto(`/crm/contacts/${contact.id}`);
	}

	function handleContactRowKeydown(event: KeyboardEvent, contact: Contact) {
		if (event.key !== 'Enter' && event.key !== ' ') return;
		event.preventDefault();
		openContact(contact);
	}

	let totalContacts = $derived(crmStore.contacts.length);
	let highScoreContacts = $derived(
		crmStore.contacts.filter((c: Contact) => c.lead_score >= 75).length
	);
	let atRiskCustomers = $derived(
		crmStore.contacts.filter((c: Contact) => c.status === 'customer' && c.health_score < 50).length
	);
</script>

<svelte:head>
	<title>CRM Contacts | Revolution Trading Pros</title>
	<meta name="description" content="Enterprise CRM contact management" />
</svelte:head>

<div class="crm-contacts-page">
	<div class="contacts-shell">
		<!-- Header -->
		<div class="page-header">
			<div class="title-group">
				<div class="title-icon">
					<IconUser size={22} />
				</div>
				<div>
					<h1 class="page-title">Contacts</h1>
					<p class="page-subtitle">RevolutionCRM-L8-System · 360° view of every relationship</p>
				</div>
			</div>

			<div class="signal-badges">
				<span class="signal-badge signal-badge--emerald">Lead Scoring L8</span>
				<span class="signal-badge signal-badge--sky">Behavior Linked</span>
			</div>
		</div>

		<!-- Filters -->
		<div class="filters-panel">
			<div class="search-field">
				<span class="filter-icon">
					<IconSearch size={18} />
				</span>
				<input
					id="contact-search"
					name="contact-search"
					class="search-input"
					placeholder="Search by name, email, title"
					bind:value={localSearch}
					onkeydown={(e: KeyboardEvent) => e.key === 'Enter' && loadContacts()}
				/>
			</div>

			<div class="select-group">
				<span class="filter-icon">
					<IconFilter size={16} />
				</span>
				<select class="filter-select" bind:value={localStatus} aria-label="Contact status">
					{#each statusOptions as opt (opt.value)}
						<option value={opt.value}>{opt.label}</option>
					{/each}
				</select>
				<select class="filter-select" bind:value={localStage} aria-label="Lifecycle stage">
					{#each stageOptions as opt (opt.value)}
						<option value={opt.value}>{opt.label}</option>
					{/each}
				</select>
			</div>

			<button class="apply-button" onclick={loadContacts} disabled={crmStore.isLoading}>
				{crmStore.isLoading ? 'Loading…' : 'Apply filters'}
			</button>
		</div>

		<!-- KPIs -->
		<div class="kpi-grid">
			<div class="kpi-card">
				<div class="kpi-content">
					<div>
						<p class="kpi-label">Total Contacts</p>
						<p class="kpi-value">{totalContacts}</p>
					</div>
					<div class="kpi-icon">
						<IconUser size={20} />
					</div>
				</div>
			</div>

			<div class="kpi-card">
				<div class="kpi-content">
					<div>
						<p class="kpi-label">High-Intent Leads</p>
						<p class="kpi-value">{highScoreContacts}</p>
					</div>
					<div class="kpi-icon kpi-icon--emerald">
						<IconTrendingUp size={20} />
					</div>
				</div>
			</div>

			<div class="kpi-card">
				<div class="kpi-content">
					<div>
						<p class="kpi-label">At-Risk Customers</p>
						<p class="kpi-value">{atRiskCustomers}</p>
					</div>
					<div class="kpi-icon kpi-icon--rose">
						<IconAlertTriangle size={20} />
					</div>
				</div>
			</div>
		</div>

		<!-- Error -->
		{#if crmStore.error}
			<div class="error-panel">
				{crmStore.error}
			</div>
		{/if}

		<!-- Table -->
		<div class="table-panel">
			<div class="table-scroll">
				<table class="contacts-table">
					<thead class="table-head">
						<tr>
							<th>Contact</th>
							<th>Status</th>
							<th>Lifecycle</th>
							<th>Lead Score</th>
							<th>Health</th>
							<th>Engagement</th>
							<th>MRR</th>
							<th>Owner</th>
							<th>Last Activity</th>
						</tr>
					</thead>
					<tbody>
						{#if crmStore.isLoading && !crmStore.contacts.length}
							<tr>
								<td colspan="9" class="empty-cell empty-cell--loading">Loading contacts…</td>
							</tr>
						{:else if !crmStore.contacts.length}
							<tr>
								<td colspan="9" class="empty-cell">No contacts found.</td>
							</tr>
						{:else}
							{#each crmStore.contacts as contact (contact.id)}
								<tr
									class="contact-row"
									onclick={() => openContact(contact)}
									onkeydown={(event) => handleContactRowKeydown(event, contact)}
									role="button"
									tabindex="0"
									aria-label={`Open contact ${contact.full_name}`}
								>
									<td>
										<div class="contact-cell">
											<span class="contact-name">{contact.full_name}</span>
											<span class="contact-email">{contact.email || ''}</span>
										</div>
									</td>
									<td class="compact-cell">
										<span class="status-pill">
											{contact.status}
										</span>
									</td>
									<td class="muted-cell compact-cell">{contact.lifecycle_stage}</td>
									<td>
										<div class="score-cell">
											<div class="score-track">
												<div class="score-fill" style:--lead-score={`${contact.lead_score}%`}></div>
											</div>
											<span class="score-value">{contact.lead_score}</span>
										</div>
									</td>
									<td class="compact-cell">{contact.health_score}</td>
									<td class="compact-cell">{contact.engagement_score}</td>
									<td class="compact-cell">
										{contact.subscription_mrr > 0 ? `$${contact.subscription_mrr.toFixed(0)}` : '—'}
									</td>
									<td class="muted-cell compact-cell">{contact.owner?.name ?? '—'}</td>
									<td class="subtle-cell compact-cell">
										{contact.last_activity_at
											? new Date(contact.last_activity_at).toLocaleDateString()
											: '—'}
									</td>
								</tr>
							{/each}
						{/if}
					</tbody>
				</table>
			</div>
		</div>
	</div>
</div>

<style>
	.crm-contacts-page {
		min-height: 100%;
		background: rgba(2, 6, 23, 0.95);
		color: #f8fafc;
	}

	.contacts-shell {
		width: min(100%, 80rem);
		margin: 0 auto;
		padding: 2rem 1.5rem;
	}

	.page-header,
	.title-group,
	.signal-badges,
	.filters-panel,
	.search-field,
	.select-group,
	.kpi-content,
	.kpi-icon,
	.score-cell {
		display: flex;
	}

	.page-header {
		flex-wrap: wrap;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		margin-bottom: 1.5rem;
	}

	.title-group {
		align-items: center;
		gap: 0.75rem;
	}

	.title-icon {
		display: flex;
		width: 2.5rem;
		height: 2.5rem;
		align-items: center;
		justify-content: center;
		border-radius: 0.75rem;
		background: rgba(99, 102, 241, 0.2);
		color: #818cf8;
	}

	.page-title {
		margin: 0;
		font-size: 1.5rem;
		font-weight: 600;
		letter-spacing: 0;
		line-height: 1.2;
	}

	.page-subtitle {
		margin: 0.25rem 0 0;
		color: #94a3b8;
		font-size: 0.875rem;
	}

	.signal-badges {
		align-items: center;
		gap: 0.5rem;
		color: #94a3b8;
		font-size: 0.75rem;
	}

	.signal-badge {
		border-radius: 999px;
		padding: 0.25rem 0.75rem;
	}

	.signal-badge--emerald {
		background: rgba(16, 185, 129, 0.1);
		color: #6ee7b7;
	}

	.signal-badge--sky {
		background: rgba(14, 165, 233, 0.1);
		color: #7dd3fc;
	}

	.filters-panel,
	.kpi-card,
	.table-panel {
		border: 1px solid rgba(30, 41, 59, 0.8);
		background: rgba(15, 23, 42, 0.7);
	}

	.filters-panel {
		flex-wrap: wrap;
		align-items: center;
		gap: 0.75rem;
		margin-bottom: 1.5rem;
		border-radius: 1rem;
		padding: 1rem;
	}

	.search-field {
		min-width: min(100%, 18rem);
		flex: 1;
		align-items: center;
		gap: 0.5rem;
		border: 1px solid #1e293b;
		border-radius: 0.75rem;
		background: rgba(15, 23, 42, 0.8);
		padding: 0.5rem 0.75rem;
	}

	.filter-icon {
		display: inline-flex;
		flex: 0 0 auto;
		color: #64748b;
	}

	.search-input {
		width: 100%;
		border: 0;
		background: transparent;
		color: #f1f5f9;
		font: inherit;
		font-size: 0.875rem;
		outline: none;
	}

	.search-input::placeholder {
		color: #64748b;
	}

	.select-group {
		align-items: center;
		gap: 0.5rem;
		color: #cbd5e1;
		font-size: 0.75rem;
	}

	.filter-select {
		border: 1px solid #1e293b;
		border-radius: 0.5rem;
		background: rgba(15, 23, 42, 0.8);
		color: #e2e8f0;
		padding: 0.25rem 0.5rem;
		font: inherit;
		font-size: 0.75rem;
	}

	.apply-button {
		border: 0;
		border-radius: 0.75rem;
		background: #6366f1;
		box-shadow: 0 1px 2px rgba(15, 23, 42, 0.3);
		color: #ffffff;
		padding: 0.5rem 1rem;
		font-size: 0.875rem;
		font-weight: 500;
		transition:
			background 0.2s ease,
			opacity 0.2s ease;
	}

	.apply-button:hover,
	.apply-button:focus-visible {
		background: #818cf8;
	}

	.apply-button:disabled {
		cursor: not-allowed;
		opacity: 0.6;
	}

	.kpi-grid {
		display: grid;
		gap: 1rem;
		margin-bottom: 1.5rem;
	}

	.kpi-card {
		border-radius: 1rem;
		padding: 1rem;
	}

	.kpi-content {
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
	}

	.kpi-label {
		margin: 0;
		color: #64748b;
		font-size: 0.75rem;
		letter-spacing: 0.04em;
		text-transform: uppercase;
	}

	.kpi-value {
		margin: 0.25rem 0 0;
		font-size: 1.5rem;
		font-weight: 600;
		line-height: 1.2;
	}

	.kpi-icon {
		align-items: center;
		justify-content: center;
		border-radius: 0.75rem;
		background: rgba(30, 41, 59, 0.8);
		color: #818cf8;
		padding: 0.5rem;
	}

	.kpi-icon--emerald {
		background: rgba(16, 185, 129, 0.1);
		color: #34d399;
	}

	.kpi-icon--rose {
		background: rgba(244, 63, 94, 0.1);
		color: #fb7185;
	}

	.error-panel {
		margin-bottom: 1rem;
		border: 1px solid rgba(190, 18, 60, 0.6);
		border-radius: 0.75rem;
		background: rgba(76, 5, 25, 0.4);
		color: #fecdd3;
		padding: 0.75rem 1rem;
		font-size: 0.875rem;
	}

	.table-panel {
		overflow: hidden;
		border-radius: 1rem;
	}

	.table-scroll {
		max-height: 70vh;
		overflow: auto;
	}

	.contacts-table {
		width: 100%;
		min-width: 68rem;
		border-collapse: collapse;
		font-size: 0.875rem;
	}

	.table-head {
		position: sticky;
		top: 0;
		z-index: 1;
		background: rgba(15, 23, 42, 0.95);
		color: #64748b;
		font-size: 0.75rem;
		letter-spacing: 0.04em;
		text-align: left;
		text-transform: uppercase;
	}

	th,
	td {
		padding: 0.75rem 1rem;
	}

	.contact-row {
		cursor: pointer;
		border-top: 1px solid rgba(30, 41, 59, 0.6);
		color: #e2e8f0;
		transition: background 0.2s ease;
	}

	.contact-row:hover,
	.contact-row:focus-visible {
		background: rgba(30, 41, 59, 0.4);
		outline: none;
	}

	.contact-row:focus-visible {
		box-shadow: inset 0 0 0 2px #818cf8;
	}

	.contact-cell {
		display: flex;
		flex-direction: column;
		gap: 0.125rem;
	}

	.contact-name {
		color: #f8fafc;
		font-weight: 500;
	}

	.contact-email,
	.subtle-cell {
		color: #94a3b8;
	}

	.contact-email,
	.compact-cell,
	.score-value {
		font-size: 0.75rem;
	}

	.status-pill {
		display: inline-flex;
		border-radius: 999px;
		background: rgba(30, 41, 59, 0.8);
		color: #e2e8f0;
		padding: 0.25rem 0.5rem;
	}

	.muted-cell {
		color: #cbd5e1;
	}

	.score-cell {
		align-items: center;
		gap: 0.5rem;
	}

	.score-track {
		width: 4rem;
		height: 0.375rem;
		overflow: hidden;
		border-radius: 999px;
		background: #1e293b;
	}

	.score-fill {
		width: var(--lead-score);
		height: 100%;
		border-radius: inherit;
		background: #34d399;
	}

	.score-value {
		color: #e2e8f0;
	}

	.empty-cell {
		padding: 2rem 1rem;
		color: #64748b;
		text-align: center;
	}

	.empty-cell--loading {
		color: #94a3b8;
	}

	@media (min-width: 768px) {
		.kpi-grid {
			grid-template-columns: repeat(3, minmax(0, 1fr));
		}
	}

	@media (max-width: 767px) {
		.contacts-shell {
			padding-right: 1rem;
			padding-left: 1rem;
		}

		.filters-panel,
		.select-group {
			align-items: stretch;
			flex-direction: column;
		}

		.select-group,
		.filter-select,
		.apply-button {
			width: 100%;
		}
	}
</style>

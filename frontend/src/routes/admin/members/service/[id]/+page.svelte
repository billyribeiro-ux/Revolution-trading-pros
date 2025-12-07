<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import { serviceMembersStore, emailStore } from '$lib/stores/members';
	import type { Member } from '$lib/api/members';
	import {
		IconArrowLeft,
		IconUsers,
		IconUserCheck,
		IconAlertTriangle,
		IconCurrencyDollar,
		IconSearch,
		IconMail,
		IconChevronLeft,
		IconChevronRight,
		IconExternalLink,
		IconFilter,
		IconX,
		IconSend,
		IconCrown,
		IconChartBar
	} from '@tabler/icons-svelte';

	let serviceId = $derived(Number($page.params.id));

	// Store state
	let service = $derived($serviceMembersStore.service);
	let stats = $derived($serviceMembersStore.stats);
	let members = $derived($serviceMembersStore.members);
	let pagination = $derived($serviceMembersStore.pagination);
	let loading = $derived($serviceMembersStore.loading);

	// Local state
	let searchQuery = '';
	let statusFilter = '';
	let selectedMembers: Set<number> = new Set();
	let showEmailModal = false;
	let emailSubject = '';
	let emailBody = '';

	onMount(async () => {
		await serviceMembersStore.loadServiceMembers(serviceId);
		await emailStore.loadTemplates();
	});

	async function handleSearch() {
		await serviceMembersStore.loadServiceMembers(serviceId, { search: searchQuery, status: statusFilter });
	}

	async function handleStatusFilter(status: string) {
		statusFilter = status;
		await serviceMembersStore.loadServiceMembers(serviceId, { search: searchQuery, status });
	}

	function toggleMemberSelection(id: number) {
		if (selectedMembers.has(id)) {
			selectedMembers.delete(id);
		} else {
			selectedMembers.add(id);
		}
		selectedMembers = selectedMembers;
	}

	function selectAllMembers() {
		if (selectedMembers.size === members.length) {
			selectedMembers.clear();
		} else {
			members.forEach((m) => selectedMembers.add(m.id));
		}
		selectedMembers = selectedMembers;
	}

	async function handleBulkEmail() {
		if (selectedMembers.size === 0) return;

		try {
			const result = await emailStore.sendBulkEmail({
				member_ids: Array.from(selectedMembers),
				subject: emailSubject,
				body: emailBody,
				personalize: true
			});
			alert(result.message);
			showEmailModal = false;
			selectedMembers.clear();
			selectedMembers = selectedMembers;
		} catch {
			alert('Failed to send emails');
		}
	}

	function applyTemplate(template: { subject: string; body?: string }) {
		emailSubject = template.subject;
		emailBody = template.body || '';
	}

	function getStatusColor(status: string): string {
		switch (status) {
			case 'active': return 'bg-emerald-500/20 text-emerald-400 border-emerald-500/30';
			case 'trial': return 'bg-blue-500/20 text-blue-400 border-blue-500/30';
			case 'churned': return 'bg-red-500/20 text-red-400 border-red-500/30';
			default: return 'bg-slate-500/20 text-slate-400 border-slate-500/30';
		}
	}

	function formatCurrency(amount: number): string {
		return new Intl.NumberFormat('en-US', {
			style: 'currency',
			currency: 'USD',
			minimumFractionDigits: 0,
			maximumFractionDigits: 0
		}).format(amount);
	}

	function formatDate(dateString: string): string {
		return new Date(dateString).toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'short',
			day: 'numeric'
		});
	}

	function getMemberInitials(member: Member): string {
		if (member.first_name && member.last_name) {
			return `${member.first_name[0]}${member.last_name[0]}`.toUpperCase();
		}
		return member.name?.slice(0, 2).toUpperCase() || 'U';
	}
</script>

<svelte:head>
	<title>{service?.name || 'Service'} Members | Revolution Trading Pros</title>
</svelte:head>

<div class="service-members-page">
	<!-- Header -->
	<div class="page-header">
		<button class="back-btn" onclick={() => goto('/admin/members')}>
			<IconArrowLeft size={20} />
			Back to Members
		</button>

		<div class="header-content">
			<div class="header-title">
				<div class="title-icon">
					<IconChartBar size={28} />
				</div>
				<div>
					<h1>{service?.name || 'Loading...'}</h1>
					<p class="service-type">{service?.type || ''}</p>
				</div>
			</div>
			{#if selectedMembers.size > 0}
				<button class="btn-email" onclick={() => (showEmailModal = true)}>
					<IconMail size={18} />
					Email ({selectedMembers.size})
				</button>
			{/if}
		</div>
	</div>

	<!-- Stats -->
	{#if stats}
		<div class="stats-grid">
			<div class="stat-card">
				<div class="stat-icon purple"><IconUsers size={24} /></div>
				<div class="stat-value">{stats.total_members}</div>
				<div class="stat-label">Total Members</div>
			</div>
			<div class="stat-card">
				<div class="stat-icon emerald"><IconUserCheck size={24} /></div>
				<div class="stat-value">{stats.active_members}</div>
				<div class="stat-label">Active</div>
			</div>
			<div class="stat-card">
				<div class="stat-icon blue"><IconCrown size={24} /></div>
				<div class="stat-value">{stats.trial_members}</div>
				<div class="stat-label">Trial</div>
			</div>
			<div class="stat-card">
				<div class="stat-icon red"><IconAlertTriangle size={24} /></div>
				<div class="stat-value">{stats.churned_members}</div>
				<div class="stat-label">Churned</div>
			</div>
			<div class="stat-card">
				<div class="stat-icon gold"><IconCurrencyDollar size={24} /></div>
				<div class="stat-value">{formatCurrency(stats.total_revenue)}</div>
				<div class="stat-label">Total Revenue</div>
			</div>
		</div>
	{/if}

	<!-- Toolbar -->
	<div class="toolbar">
		<div class="search-box">
			<IconSearch size={18} />
			<input
				type="text"
				placeholder="Search members..."
				bind:value={searchQuery}
				onkeydown={(e) => e.key === 'Enter' && handleSearch()}
			/>
		</div>

		<div class="filter-group">
			<IconFilter size={16} />
			<select bind:value={statusFilter} onchange={() => handleStatusFilter(statusFilter)}>
				<option value="">All Status</option>
				<option value="active">Active</option>
				<option value="trial">Trial</option>
				<option value="cancelled">Cancelled</option>
				<option value="expired">Expired</option>
			</select>
		</div>
	</div>

	<!-- Members Table -->
	<div class="members-table-container">
		{#if loading}
			<div class="loading-state">
				<div class="loader"></div>
				<p>Loading members...</p>
			</div>
		{:else if members.length === 0}
			<div class="empty-state">
				<IconUsers size={64} stroke={1} />
				<h3>No members found</h3>
				<p>No members have subscribed to this service yet</p>
			</div>
		{:else}
			<table class="members-table">
				<thead>
					<tr>
						<th class="checkbox-col">
							<input
								type="checkbox"
								checked={selectedMembers.size === members.length && members.length > 0}
								onchange={selectAllMembers}
							/>
						</th>
						<th>Member</th>
						<th>Status</th>
						<th>Plan</th>
						<th>Revenue</th>
						<th>Start Date</th>
						<th>Actions</th>
					</tr>
				</thead>
				<tbody>
					{#each members as member}
						<tr class:selected={selectedMembers.has(member.id)}>
							<td class="checkbox-col">
								<input
									type="checkbox"
									checked={selectedMembers.has(member.id)}
									onchange={() => toggleMemberSelection(member.id)}
								/>
							</td>
							<td>
								<div class="member-info">
									<div class="member-avatar">
										{getMemberInitials(member)}
									</div>
									<div class="member-details">
										<div class="member-name">{member.name}</div>
										<div class="member-email">{member.email}</div>
									</div>
								</div>
							</td>
							<td>
								<span class="status-badge {getStatusColor(member.status)}">
									{member.status_label}
								</span>
							</td>
							<td>
								<span class="plan-name">{member.current_plan || '-'}</span>
							</td>
							<td>
								<span class="revenue">{formatCurrency(member.total_spent)}</span>
							</td>
							<td>
								<span class="date">{formatDate(member.joined_at)}</span>
							</td>
							<td>
								<div class="actions">
									<button class="action-btn" title="View Details" onclick={() => goto(`/admin/members/${member.id}`)}>
										<IconExternalLink size={16} />
									</button>
									<button class="action-btn" title="Send Email" onclick={() => { selectedMembers.clear(); selectedMembers.add(member.id); selectedMembers = selectedMembers; showEmailModal = true; }}>
										<IconMail size={16} />
									</button>
								</div>
							</td>
						</tr>
					{/each}
				</tbody>
			</table>

			<!-- Pagination -->
			{#if pagination}
				<div class="pagination">
					<div class="pagination-info">
						Showing {(pagination.current_page - 1) * pagination.per_page + 1} to {Math.min(pagination.current_page * pagination.per_page, pagination.total)} of {pagination.total}
					</div>
					<div class="pagination-controls">
						<button
							class="page-btn"
							disabled={pagination.current_page === 1}
							onclick={() => serviceMembersStore.loadServiceMembers(serviceId, { page: pagination.current_page - 1 })}
						>
							<IconChevronLeft size={18} />
						</button>
						<span class="page-indicator">Page {pagination.current_page} of {pagination.last_page}</span>
						<button
							class="page-btn"
							disabled={pagination.current_page === pagination.last_page}
							onclick={() => serviceMembersStore.loadServiceMembers(serviceId, { page: pagination.current_page + 1 })}
						>
							<IconChevronRight size={18} />
						</button>
					</div>
				</div>
			{/if}
		{/if}
	</div>
</div>

<!-- Email Modal -->
{#if showEmailModal}
	<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
	<div class="modal-overlay" onclick={() => (showEmailModal = false)} onkeydown={(e) => e.key === 'Escape' && (showEmailModal = false)} role="dialog" tabindex="-1" aria-modal="true">
		<div class="modal-content" onclick={(e) => e.stopPropagation()} onkeydown={(e) => e.stopPropagation()} role="document">
			<div class="modal-header">
				<h2>Send Email to {selectedMembers.size} Member{selectedMembers.size > 1 ? 's' : ''}</h2>
				<button class="close-btn" onclick={() => (showEmailModal = false)}>
					<IconX size={20} />
				</button>
			</div>

			<div class="modal-body">
				<div class="template-selector">
					<span class="selector-label">Quick Templates</span>
					<div class="template-buttons">
						{#each $emailStore.presetTemplates as template}
							<button class="template-btn" onclick={() => applyTemplate(template)}>
								{template.name}
							</button>
						{/each}
					</div>
				</div>

				<div class="form-group">
					<label for="email-subject">Subject</label>
					<input id="email-subject" type="text" bind:value={emailSubject} placeholder="Email subject..." />
				</div>

				<div class="form-group">
					<label for="email-body">Body</label>
					<textarea id="email-body" bind:value={emailBody} rows="10" placeholder="Email body..."></textarea>
				</div>
			</div>

			<div class="modal-footer">
				<button class="btn-secondary" onclick={() => (showEmailModal = false)}>Cancel</button>
				<button class="btn-primary" onclick={handleBulkEmail} disabled={!emailSubject || !emailBody}>
					<IconSend size={18} />
					Send Email
				</button>
			</div>
		</div>
	</div>
{/if}

<style>
	.service-members-page {
		padding: 2rem;
		max-width: 1400px;
		margin: 0 auto;
	}

	.back-btn {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 0;
		color: #94a3b8;
		background: none;
		border: none;
		cursor: pointer;
		font-size: 0.875rem;
		margin-bottom: 1rem;
		transition: color 0.2s;
	}

	.back-btn:hover {
		color: #a5b4fc;
	}

	.header-content {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 2rem;
	}

	.header-title {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.title-icon {
		width: 56px;
		height: 56px;
		background: linear-gradient(135deg, #6366f1, #8b5cf6);
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

	.service-type {
		color: #64748b;
		text-transform: capitalize;
		margin-top: 0.25rem;
	}

	/* Stats Grid */
	.stats-grid {
		display: grid;
		grid-template-columns: repeat(5, 1fr);
		gap: 1rem;
		margin-bottom: 2rem;
	}

	.stat-card {
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 16px;
		padding: 1.25rem;
		text-align: center;
	}

	.stat-icon {
		width: 48px;
		height: 48px;
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
		margin: 0 auto 0.75rem;
	}

	.stat-icon.purple { background: rgba(99, 102, 241, 0.15); color: #818cf8; }
	.stat-icon.emerald { background: rgba(16, 185, 129, 0.15); color: #34d399; }
	.stat-icon.blue { background: rgba(59, 130, 246, 0.15); color: #60a5fa; }
	.stat-icon.red { background: rgba(239, 68, 68, 0.15); color: #f87171; }
	.stat-icon.gold { background: rgba(251, 191, 36, 0.15); color: #fbbf24; }

	.stat-value {
		font-size: 1.5rem;
		font-weight: 700;
		color: #f1f5f9;
	}

	.stat-label {
		font-size: 0.75rem;
		color: #64748b;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		margin-top: 0.25rem;
	}

	/* Toolbar */
	.toolbar {
		display: flex;
		gap: 1rem;
		margin-bottom: 1rem;
	}

	.search-box {
		flex: 1;
		max-width: 400px;
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem 1rem;
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 12px;
		color: #94a3b8;
	}

	.search-box input {
		flex: 1;
		background: none;
		border: none;
		color: #f1f5f9;
		font-size: 0.9375rem;
		outline: none;
	}

	.filter-group {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0 1rem;
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 12px;
		color: #94a3b8;
	}

	.filter-group select {
		background: none;
		border: none;
		color: #f1f5f9;
		font-size: 0.875rem;
		padding: 0.75rem 0;
		cursor: pointer;
	}

	/* Table */
	.members-table-container {
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 16px;
		overflow: hidden;
	}

	.loading-state,
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 2rem;
		color: #64748b;
	}

	.loader {
		width: 48px;
		height: 48px;
		border: 4px solid rgba(99, 102, 241, 0.2);
		border-top-color: #6366f1;
		border-radius: 50%;
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		to { transform: rotate(360deg); }
	}

	.empty-state h3 {
		color: #f1f5f9;
		margin: 1rem 0 0.5rem;
	}

	.members-table {
		width: 100%;
		border-collapse: collapse;
	}

	.members-table thead {
		background: rgba(15, 23, 42, 0.6);
	}

	.members-table th {
		padding: 1rem 1.5rem;
		text-align: left;
		font-size: 0.75rem;
		font-weight: 600;
		color: #94a3b8;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.members-table tbody tr {
		border-top: 1px solid rgba(148, 163, 184, 0.1);
		transition: background 0.2s;
	}

	.members-table tbody tr:hover {
		background: rgba(99, 102, 241, 0.05);
	}

	.members-table tbody tr.selected {
		background: rgba(99, 102, 241, 0.1);
	}

	.members-table td {
		padding: 1rem 1.5rem;
		color: #cbd5e1;
	}

	.checkbox-col {
		width: 48px;
	}

	.checkbox-col input {
		width: 18px;
		height: 18px;
		cursor: pointer;
	}

	.member-info {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.member-avatar {
		width: 40px;
		height: 40px;
		border-radius: 50%;
		background: linear-gradient(135deg, #6366f1, #8b5cf6);
		display: flex;
		align-items: center;
		justify-content: center;
		font-weight: 700;
		font-size: 0.875rem;
		color: white;
	}

	.member-name {
		font-weight: 600;
		color: #f1f5f9;
	}

	.member-email {
		font-size: 0.8125rem;
		color: #64748b;
	}

	.status-badge {
		display: inline-flex;
		padding: 0.25rem 0.75rem;
		border-radius: 9999px;
		font-size: 0.75rem;
		font-weight: 600;
		border: 1px solid;
	}

	.plan-name {
		color: #a5b4fc;
	}

	.revenue {
		font-weight: 600;
		color: #34d399;
	}

	.date {
		font-size: 0.875rem;
		color: #94a3b8;
	}

	.actions {
		display: flex;
		gap: 0.5rem;
	}

	.action-btn {
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

	.action-btn:hover {
		background: rgba(99, 102, 241, 0.15);
		border-color: rgba(99, 102, 241, 0.3);
		color: #a5b4fc;
	}

	/* Pagination */
	.pagination {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1rem 1.5rem;
		border-top: 1px solid rgba(148, 163, 184, 0.1);
	}

	.pagination-info {
		font-size: 0.875rem;
		color: #64748b;
	}

	.pagination-controls {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.page-btn {
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
		transition: all 0.2s;
	}

	.page-btn:hover:not(:disabled) {
		background: rgba(99, 102, 241, 0.15);
		border-color: rgba(99, 102, 241, 0.3);
		color: #a5b4fc;
	}

	.page-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.page-indicator {
		font-size: 0.875rem;
		color: #94a3b8;
	}

	/* Modal styles */
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
		max-width: 600px;
		max-height: 90vh;
		overflow-y: auto;
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
	}

	.template-selector {
		margin-bottom: 1.5rem;
	}

	.selector-label {
		display: block;
		font-size: 0.75rem;
		font-weight: 600;
		color: #94a3b8;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		margin-bottom: 0.75rem;
	}

	.template-buttons {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
	}

	.template-btn {
		padding: 0.5rem 1rem;
		background: rgba(99, 102, 241, 0.1);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 8px;
		color: #a5b4fc;
		font-size: 0.8125rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.template-btn:hover {
		background: rgba(99, 102, 241, 0.2);
	}

	.form-group {
		margin-bottom: 1rem;
	}

	.form-group label {
		display: block;
		font-size: 0.8125rem;
		font-weight: 500;
		color: #94a3b8;
		margin-bottom: 0.5rem;
	}

	.form-group input,
	.form-group textarea {
		width: 100%;
		padding: 0.75rem 1rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 10px;
		color: #f1f5f9;
		font-size: 0.9375rem;
		font-family: inherit;
		resize: vertical;
	}

	.modal-footer {
		display: flex;
		justify-content: flex-end;
		gap: 0.75rem;
		padding: 1.5rem;
		border-top: 1px solid rgba(148, 163, 184, 0.1);
	}

	.btn-secondary,
	.btn-primary,
	.btn-email {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
		border: none;
	}

	.btn-secondary {
		background: rgba(148, 163, 184, 0.1);
		color: #94a3b8;
		border: 1px solid rgba(148, 163, 184, 0.2);
	}

	.btn-primary {
		background: linear-gradient(135deg, #6366f1, #8b5cf6);
		color: white;
	}

	.btn-primary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.btn-email {
		background: rgba(16, 185, 129, 0.15);
		border: 1px solid rgba(16, 185, 129, 0.3);
		color: #34d399;
	}

	@media (max-width: 1024px) {
		.stats-grid {
			grid-template-columns: repeat(3, 1fr);
		}
	}

	@media (max-width: 768px) {
		.stats-grid {
			grid-template-columns: repeat(2, 1fr);
		}

		.toolbar {
			flex-direction: column;
		}

		.search-box {
			max-width: 100%;
		}
	}
</style>

<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { churnedStore, emailStore } from '$lib/stores/members.svelte';
	import type { Member } from '$lib/api/members';
	import {
		IconArrowLeft,
		IconHeart,
		IconMail,
		IconGift,
		IconCurrencyDollar,
		IconSearch,
		IconFilter,
		IconSend,
		IconX,
		IconChevronLeft,
		IconChevronRight,
		IconExternalLink,
		IconCalendar,
		IconAlertTriangle,
		IconTrendingDown,
		IconSparkles,
		IconRefresh
	} from '$lib/icons';

	// Store state
	let members = $derived($churnedStore.members);
	let stats = $derived($churnedStore.stats);
	let pagination = $derived($churnedStore.pagination);
	let loading = $derived($churnedStore.loading);

	// Local state
	let searchQuery = $state('');
	let winbackPotential = $state('');
	let churnedWithinDays = $state('');
	let selectedMembers = $state<Set<number>>(new Set());
	let showEmailModal = $state(false);
	let emailSubject = $state('');
	let emailBody = $state('');
	let campaignType = $state<'winback' | 'free_trial' | 'promo'>('winback');

	onMount(async () => {
		await churnedStore.loadChurnedMembers();
		await emailStore.loadTemplates();
	});

	async function handleSearch() {
		await churnedStore.setFilters({ search: searchQuery });
	}

	async function handlePotentialFilter(potential: string) {
		winbackPotential = potential;
		await churnedStore.setFilters({ winback_potential: potential as 'high' | 'medium' | 'low' | undefined });
	}

	async function handleDaysFilter(days: string) {
		churnedWithinDays = days;
		await churnedStore.setFilters({ churned_within_days: days ? Number(days) : undefined });
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
				campaign_type: campaignType,
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

	function startCampaign(type: 'winback' | 'free_trial' | 'promo') {
		campaignType = type;

		// Apply preset template based on campaign type
		const presets = $emailStore.presetTemplates;
		let template;

		switch (type) {
			case 'free_trial':
				template = presets.find(t => t.id === 'winback_30_free');
				break;
			case 'promo':
				template = presets.find(t => t.id === 'winback_discount');
				break;
			default:
				template = presets.find(t => t.id === 'winback_missed');
		}

		if (template) {
			emailSubject = template.subject;
			emailBody = template.body || '';
		}

		showEmailModal = true;
	}

	function formatCurrency(amount: number): string {
		return new Intl.NumberFormat('en-US', {
			style: 'currency',
			currency: 'USD',
			minimumFractionDigits: 0,
			maximumFractionDigits: 0
		}).format(amount);
	}

	function formatDate(dateString: string | undefined): string {
		if (!dateString) return '-';
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

	function getPotentialColor(spent: number): string {
		if (spent >= 500) return 'high';
		if (spent >= 100) return 'medium';
		return 'low';
	}

	function getPotentialLabel(spent: number): string {
		if (spent >= 500) return 'High Value';
		if (spent >= 100) return 'Medium Value';
		return 'Low Value';
	}
</script>

<svelte:head>
	<title>Win-Back Center | Revolution Trading Pros</title>
</svelte:head>

<div class="churned-page">
	<!-- Header -->
	<div class="page-header">
		<button class="back-btn" onclick={() => goto('/admin/members')}>
			<IconArrowLeft size={20} />
			Back to Members
		</button>

		<div class="header-content">
			<div class="header-title">
				<div class="title-icon">
					<IconHeart size={32} />
				</div>
				<div>
					<h1>Win-Back Center</h1>
					<p>Re-engage past members with targeted campaigns</p>
				</div>
			</div>
			<button class="btn-refresh" onclick={() => churnedStore.loadChurnedMembers()}>
				<IconRefresh size={18} />
				Refresh
			</button>
		</div>
	</div>

	<!-- Campaign Actions -->
	<div class="campaign-actions">
		<button class="campaign-card free-trial" onclick={() => { selectedMembers = new Set(members.map(m => m.id)); startCampaign('free_trial'); }}>
			<div class="campaign-icon">
				<IconGift size={28} />
			</div>
			<div class="campaign-content">
				<h3>30 Days Free</h3>
				<p>Win back with a free trial offer</p>
			</div>
			<span class="campaign-tag">Popular</span>
		</button>

		<button class="campaign-card promo" onclick={() => { selectedMembers = new Set(members.map(m => m.id)); startCampaign('promo'); }}>
			<div class="campaign-icon">
				<IconCurrencyDollar size={28} />
			</div>
			<div class="campaign-content">
				<h3>50% Discount</h3>
				<p>Exclusive comeback offer</p>
			</div>
		</button>

		<button class="campaign-card winback" onclick={() => { selectedMembers = new Set(members.map(m => m.id)); startCampaign('winback'); }}>
			<div class="campaign-icon">
				<IconMail size={28} />
			</div>
			<div class="campaign-content">
				<h3>We Miss You</h3>
				<p>Personal reconnection email</p>
			</div>
		</button>
	</div>

	<!-- Stats -->
	{#if stats}
		<div class="stats-grid">
			<div class="stat-card">
				<div class="stat-header">
					<IconAlertTriangle size={24} />
					<span class="stat-label">Total Churned</span>
				</div>
				<div class="stat-value">{stats.total_churned}</div>
				<div class="stat-sublabel">
					<IconTrendingDown size={14} />
					{stats.churned_this_month} this month
				</div>
			</div>

			<div class="stat-card recovery">
				<div class="stat-header">
					<IconCurrencyDollar size={24} />
					<span class="stat-label">Recovery Potential</span>
				</div>
				<div class="stat-value">{formatCurrency(stats.potential_recovery_revenue)}</div>
				<div class="stat-sublabel">
					<IconSparkles size={14} />
					Estimated monthly recovery
				</div>
			</div>

			<div class="stat-card">
				<div class="stat-header">
					<IconAlertTriangle size={24} />
					<span class="stat-label">Top Churn Reason</span>
				</div>
				<div class="stat-value churn-reason">
					{stats.top_churn_reasons?.[0]?.reason?.replace(/"/g, '') || 'Unknown'}
				</div>
				<div class="stat-sublabel">
					{stats.top_churn_reasons?.[0]?.count || 0} occurrences
				</div>
			</div>
		</div>
	{/if}

	<!-- Toolbar -->
	<div class="toolbar">
		<div class="search-box">
			<IconSearch size={18} />
			<input
				id="search-churned-members"
				type="text"
				placeholder="Search churned members..."
				bind:value={searchQuery}
				onkeydown={(e: KeyboardEvent) => e.key === 'Enter' && handleSearch()}
			/>
		</div>

		<div class="filters">
			<div class="filter-group">
				<IconFilter size={16} />
				<select bind:value={winbackPotential} onchange={() => handlePotentialFilter(winbackPotential)}>
					<option value="">All Value</option>
					<option value="high">High Value ($500+)</option>
					<option value="medium">Medium ($100-$500)</option>
					<option value="low">Low (&lt;$100)</option>
				</select>
			</div>

			<div class="filter-group">
				<IconCalendar size={16} />
				<select bind:value={churnedWithinDays} onchange={() => handleDaysFilter(churnedWithinDays)}>
					<option value="">Any Time</option>
					<option value="7">Last 7 days</option>
					<option value="30">Last 30 days</option>
					<option value="90">Last 90 days</option>
					<option value="180">Last 6 months</option>
				</select>
			</div>
		</div>

		{#if selectedMembers.size > 0}
			<button class="btn-email" onclick={() => (showEmailModal = true)}>
				<IconMail size={18} />
				Email Selected ({selectedMembers.size})
			</button>
		{/if}
	</div>

	<!-- Members Table -->
	<div class="members-table-container">
		{#if loading}
			<div class="loading-state">
				<div class="loader"></div>
				<p>Loading churned members...</p>
			</div>
		{:else if members.length === 0}
			<div class="empty-state">
				<IconHeart size={64} stroke={1} />
				<h3>No churned members</h3>
				<p>Great news! No members have churned matching your filters.</p>
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
								aria-label="Select all churned members"
							/>
						</th>
						<th>Member</th>
						<th>Win-Back Potential</th>
						<th>Last Product</th>
						<th>Total Spent</th>
						<th>Churned</th>
						<th>Reason</th>
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
									aria-label="Select {member.name}"
								/>
							</td>
							<td>
								<div class="member-info">
									<div class="member-avatar churned">
										{getMemberInitials(member)}
									</div>
									<div class="member-details">
										<div class="member-name">{member.name || ''}</div>
										<div class="member-email">{member.email || ''}</div>
									</div>
								</div>
							</td>
							<td>
								<span class="potential-badge {getPotentialColor(member.total_spent)}">
									{getPotentialLabel(member.total_spent)}
								</span>
							</td>
							<td>
								<span class="product-name">{member.last_product || '-'}</span>
							</td>
							<td>
								<span class="spending {getPotentialColor(member.total_spent)}">{formatCurrency(member.total_spent)}</span>
							</td>
							<td>
								<div class="churn-date">
									<span class="date">{formatDate(member.churned_at)}</span>
									{#if member.days_since_churn}
										<span class="days-ago">{member.days_since_churn}d ago</span>
									{/if}
								</div>
							</td>
							<td>
								<span class="churn-reason-text">{member.churn_reason || 'Unknown'}</span>
							</td>
							<td>
								<div class="actions">
									<button class="action-btn winback" title="Start Win-Back" onclick={() => { selectedMembers.clear(); selectedMembers.add(member.id); selectedMembers = selectedMembers; startCampaign('free_trial'); }}>
										<IconGift size={16} />
									</button>
									<button class="action-btn" title="Send Email" onclick={() => { selectedMembers.clear(); selectedMembers.add(member.id); selectedMembers = selectedMembers; showEmailModal = true; }}>
										<IconMail size={16} />
									</button>
									<button class="action-btn" title="View Details" onclick={() => goto(`/admin/members/${member.id}`)}>
										<IconExternalLink size={16} />
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
							onclick={() => churnedStore.goToPage(pagination.current_page - 1)}
						>
							<IconChevronLeft size={18} />
						</button>
						<span class="page-indicator">Page {pagination.current_page} of {pagination.last_page}</span>
						<button
							class="page-btn"
							disabled={pagination.current_page === pagination.last_page}
							onclick={() => churnedStore.goToPage(pagination.current_page + 1)}
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
	<div class="modal-overlay" onclick={() => (showEmailModal = false)} onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && (showEmailModal = false)} role="dialog" tabindex="-1" aria-modal="true">
		<div class="modal-content" onclick={(e: MouseEvent) => e.stopPropagation()} onkeydown={(e: KeyboardEvent) => e.stopPropagation()} role="document">
			<div class="modal-header">
				<div>
					<h2>Win-Back Campaign</h2>
					<p>Sending to {selectedMembers.size} member{selectedMembers.size > 1 ? 's' : ''}</p>
				</div>
				<button class="close-btn" onclick={() => (showEmailModal = false)}>
					<IconX size={20} />
				</button>
			</div>

			<div class="modal-body">
				<div class="campaign-type-selector">
					<span class="selector-label">Campaign Type</span>
					<div class="campaign-type-buttons">
						<button class:active={campaignType === 'free_trial'} onclick={() => { campaignType = 'free_trial'; const t = $emailStore.presetTemplates.find(t => t.id === 'winback_30_free'); if (t) applyTemplate(t); }}>
							<IconGift size={18} />
							30 Days Free
						</button>
						<button class:active={campaignType === 'promo'} onclick={() => { campaignType = 'promo'; const t = $emailStore.presetTemplates.find(t => t.id === 'winback_discount'); if (t) applyTemplate(t); }}>
							<IconCurrencyDollar size={18} />
							50% Off
						</button>
						<button class:active={campaignType === 'winback'} onclick={() => { campaignType = 'winback'; const t = $emailStore.presetTemplates.find(t => t.id === 'winback_missed'); if (t) applyTemplate(t); }}>
							<IconHeart size={18} />
							We Miss You
						</button>
					</div>
				</div>

				<div class="form-group">
					<label for="email-subject">Subject Line</label>
					<input id="email-subject" type="text" bind:value={emailSubject} placeholder="Email subject..." />
				</div>

				<div class="form-group">
					<label for="email-body">Email Body</label>
					<textarea id="email-body" bind:value={emailBody} rows="12" placeholder="Email body... Use {{name}} for personalization"></textarea>
					<p class="form-hint">Available variables: {"{{name}}"}, {"{{first_name}}"}, {"{{email}}"}</p>
				</div>
			</div>

			<div class="modal-footer">
				<button class="btn-secondary" onclick={() => (showEmailModal = false)}>Cancel</button>
				<button class="btn-primary" onclick={handleBulkEmail} disabled={!emailSubject || !emailBody}>
					<IconSend size={18} />
					Send Win-Back Email
				</button>
			</div>
		</div>
	</div>
{/if}

<style>
	.churned-page {
		padding: 2rem;
		max-width: 1600px;
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
		color: #E6B800;
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
		width: 64px;
		height: 64px;
		background: linear-gradient(135deg, #f43f5e, #fb7185);
		border-radius: 16px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: white;
		box-shadow: 0 8px 32px rgba(244, 63, 94, 0.3);
	}

	.header-title h1 {
		font-size: 2rem;
		font-weight: 800;
		background: linear-gradient(135deg, #f1f5f9, #cbd5e1);
		-webkit-background-clip: text;
		background-clip: text;
		-webkit-text-fill-color: transparent;
		margin: 0;
	}

	.header-title p {
		color: #64748b;
		margin-top: 0.25rem;
	}

	.btn-refresh {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background: rgba(148, 163, 184, 0.1);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 10px;
		color: #94a3b8;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-refresh:hover {
		border-color: rgba(230, 184, 0, 0.3);
		color: #E6B800;
	}

	/* Campaign Actions */
	.campaign-actions {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 1rem;
		margin-bottom: 2rem;
	}

	.campaign-card {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1.5rem;
		background: rgba(30, 41, 59, 0.6);
		border: 2px solid transparent;
		border-radius: 16px;
		cursor: pointer;
		transition: all 0.3s;
		position: relative;
		overflow: hidden;
		text-align: left;
	}

	.campaign-card::before {
		content: '';
		position: absolute;
		inset: 0;
		opacity: 0;
		transition: opacity 0.3s;
	}

	.campaign-card.free-trial::before {
		background: linear-gradient(135deg, rgba(16, 185, 129, 0.1), transparent);
	}

	.campaign-card.promo::before {
		background: linear-gradient(135deg, rgba(251, 191, 36, 0.1), transparent);
	}

	.campaign-card.winback::before {
		background: linear-gradient(135deg, rgba(244, 63, 94, 0.1), transparent);
	}

	.campaign-card:hover::before {
		opacity: 1;
	}

	.campaign-card.free-trial:hover {
		border-color: rgba(16, 185, 129, 0.5);
	}

	.campaign-card.promo:hover {
		border-color: rgba(251, 191, 36, 0.5);
	}

	.campaign-card.winback:hover {
		border-color: rgba(244, 63, 94, 0.5);
	}

	.campaign-icon {
		width: 56px;
		height: 56px;
		border-radius: 14px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.free-trial .campaign-icon {
		background: rgba(16, 185, 129, 0.15);
		color: #34d399;
	}

	.promo .campaign-icon {
		background: rgba(251, 191, 36, 0.15);
		color: #fbbf24;
	}

	.winback .campaign-icon {
		background: rgba(244, 63, 94, 0.15);
		color: #fb7185;
	}

	.campaign-content h3 {
		font-size: 1.125rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0;
	}

	.campaign-content p {
		font-size: 0.875rem;
		color: #64748b;
		margin: 0.25rem 0 0;
	}

	.campaign-tag {
		position: absolute;
		top: 0.75rem;
		right: 0.75rem;
		padding: 0.25rem 0.75rem;
		background: rgba(16, 185, 129, 0.2);
		border: 1px solid rgba(16, 185, 129, 0.3);
		border-radius: 9999px;
		font-size: 0.6875rem;
		font-weight: 700;
		color: #34d399;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	/* Stats */
	.stats-grid {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 1.5rem;
		margin-bottom: 2rem;
	}

	.stat-card {
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 16px;
		padding: 1.5rem;
	}

	.stat-card.recovery {
		border-color: rgba(16, 185, 129, 0.3);
		background: linear-gradient(135deg, rgba(30, 41, 59, 0.6), rgba(16, 185, 129, 0.05));
	}

	.stat-header {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		color: #64748b;
		margin-bottom: 0.75rem;
	}

	.stat-label {
		font-size: 0.8125rem;
		font-weight: 500;
	}

	.stat-value {
		font-size: 2rem;
		font-weight: 800;
		color: #f1f5f9;
	}

	.stat-value.churn-reason {
		font-size: 1.25rem;
		text-transform: capitalize;
	}

	.stat-sublabel {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		font-size: 0.8125rem;
		color: #64748b;
		margin-top: 0.5rem;
	}

	.recovery .stat-value {
		color: #34d399;
	}

	/* Toolbar */
	.toolbar {
		display: flex;
		align-items: center;
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

	.filters {
		display: flex;
		gap: 0.75rem;
	}

	.filter-group {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0 1rem;
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 10px;
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

	.btn-email {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background: linear-gradient(135deg, #f43f5e, #fb7185);
		border: none;
		border-radius: 10px;
		color: white;
		font-weight: 600;
		cursor: pointer;
		box-shadow: 0 4px 14px rgba(244, 63, 94, 0.3);
		transition: all 0.2s;
	}

	.btn-email:hover {
		transform: translateY(-2px);
		box-shadow: 0 8px 24px rgba(244, 63, 94, 0.4);
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
		border: 4px solid rgba(244, 63, 94, 0.2);
		border-top-color: #f43f5e;
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
		background: rgba(244, 63, 94, 0.03);
	}

	.members-table tbody tr.selected {
		background: rgba(244, 63, 94, 0.08);
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
		display: flex;
		align-items: center;
		justify-content: center;
		font-weight: 700;
		font-size: 0.875rem;
		color: white;
	}

	.member-avatar.churned {
		background: linear-gradient(135deg, #64748b, #94a3b8);
	}

	.member-name {
		font-weight: 600;
		color: #f1f5f9;
	}

	.member-email {
		font-size: 0.8125rem;
		color: #64748b;
	}

	.potential-badge {
		display: inline-flex;
		padding: 0.25rem 0.75rem;
		border-radius: 9999px;
		font-size: 0.75rem;
		font-weight: 600;
		border: 1px solid;
	}

	.potential-badge.high {
		background: rgba(16, 185, 129, 0.15);
		color: #34d399;
		border-color: rgba(16, 185, 129, 0.3);
	}

	.potential-badge.medium {
		background: rgba(251, 191, 36, 0.15);
		color: #fbbf24;
		border-color: rgba(251, 191, 36, 0.3);
	}

	.potential-badge.low {
		background: rgba(148, 163, 184, 0.15);
		color: #94a3b8;
		border-color: rgba(148, 163, 184, 0.3);
	}

	.product-name {
		color: #E6B800;
	}

	.spending {
		font-weight: 600;
	}

	.spending.high {
		color: #34d399;
	}

	.spending.medium {
		color: #fbbf24;
	}

	.churn-date {
		display: flex;
		flex-direction: column;
		gap: 0.125rem;
	}

	.date {
		font-size: 0.875rem;
		color: #94a3b8;
	}

	.days-ago {
		font-size: 0.75rem;
		color: #f87171;
	}

	.churn-reason-text {
		font-size: 0.8125rem;
		color: #94a3b8;
		text-transform: capitalize;
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
		background: rgba(230, 184, 0, 0.15);
		border-color: rgba(230, 184, 0, 0.3);
		color: #E6B800;
	}

	.action-btn.winback:hover {
		background: rgba(16, 185, 129, 0.15);
		border-color: rgba(16, 185, 129, 0.3);
		color: #34d399;
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
		background: rgba(230, 184, 0, 0.15);
		border-color: rgba(230, 184, 0, 0.3);
		color: #E6B800;
	}

	.page-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.page-indicator {
		font-size: 0.875rem;
		color: #94a3b8;
	}

	/* Modal */
	.modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.85);
		backdrop-filter: blur(8px);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 50;
		padding: 2rem;
	}

	.modal-content {
		background: linear-gradient(135deg, #1e293b, #0f172a);
		border: 1px solid rgba(244, 63, 94, 0.2);
		border-radius: 24px;
		width: 100%;
		max-width: 640px;
		max-height: 90vh;
		overflow-y: auto;
		box-shadow: 0 24px 48px rgba(0, 0, 0, 0.4);
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		padding: 1.5rem;
		border-bottom: 1px solid rgba(148, 163, 184, 0.1);
	}

	.modal-header h2 {
		font-size: 1.5rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0;
	}

	.modal-header p {
		font-size: 0.875rem;
		color: #64748b;
		margin: 0.25rem 0 0;
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

	.campaign-type-selector {
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

	.campaign-type-buttons {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 0.75rem;
	}

	.campaign-type-buttons button {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 0.875rem 1rem;
		background: rgba(30, 41, 59, 0.6);
		border: 2px solid rgba(148, 163, 184, 0.1);
		border-radius: 12px;
		color: #94a3b8;
		font-weight: 600;
		font-size: 0.8125rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.campaign-type-buttons button:hover {
		border-color: rgba(148, 163, 184, 0.3);
	}

	.campaign-type-buttons button.active {
		background: rgba(244, 63, 94, 0.15);
		border-color: rgba(244, 63, 94, 0.5);
		color: #fb7185;
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
		padding: 0.875rem 1rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 12px;
		color: #f1f5f9;
		font-size: 0.9375rem;
		font-family: inherit;
		resize: vertical;
		transition: border-color 0.2s;
	}

	.form-group input:focus,
	.form-group textarea:focus {
		outline: none;
		border-color: rgba(244, 63, 94, 0.5);
	}

	.form-hint {
		font-size: 0.75rem;
		color: #64748b;
		margin-top: 0.5rem;
	}

	.modal-footer {
		display: flex;
		justify-content: flex-end;
		gap: 0.75rem;
		padding: 1.5rem;
		border-top: 1px solid rgba(148, 163, 184, 0.1);
	}

	.btn-secondary,
	.btn-primary {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.875rem 1.5rem;
		border-radius: 12px;
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
		background: linear-gradient(135deg, #f43f5e, #fb7185);
		color: white;
		box-shadow: 0 4px 14px rgba(244, 63, 94, 0.3);
	}

	.btn-primary:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow: 0 8px 24px rgba(244, 63, 94, 0.4);
	}

	.btn-primary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
		transform: none;
	}

	@media (max-width: 1024px) {
		.campaign-actions {
			grid-template-columns: 1fr;
		}

		.stats-grid {
			grid-template-columns: 1fr;
		}
	}

	@media (max-width: 768px) {
		.toolbar {
			flex-direction: column;
		}

		.search-box {
			max-width: 100%;
		}

		.filters {
			flex-wrap: wrap;
		}

		.campaign-type-buttons {
			grid-template-columns: 1fr;
		}
	}
</style>

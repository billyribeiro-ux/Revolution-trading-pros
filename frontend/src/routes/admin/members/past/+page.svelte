<script lang="ts">
	/**
	 * Past Members Dashboard - Svelte 5 / SvelteKit Implementation
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Enterprise-grade admin dashboard with:
	 * - Svelte 5 runes ($state, $derived, $effect)
	 * - SvelteKit data streaming from load function
	 * - Multi-tier caching integration
	 * - Optimistic UI updates
	 * - Skeleton loading states
	 *
	 * @version 3.0.0 (Svelte 5 / December 2025)
	 */

	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import { browser } from '$app/environment';
	import pastMembersApi, {
		type TimePeriod,
		type PastMember,
		type DashboardOverview,
		type ChurnReason,
		type PeriodStats,
		type PaginatedResponse,
		TIME_PERIOD_LABELS,
		invalidateDashboardCache
	} from '$lib/api/past-members-dashboard';
	import {
		IconArrowLeft,
		IconUsers,
		IconClock,
		IconMail,
		IconSend,
		IconSearch,
		IconRefresh,
		IconChevronLeft,
		IconChevronRight,
		IconX,
		IconGift,
		IconCurrencyDollar,
		IconTrendingUp,
		IconChartPie,
		IconCalendar,
		IconFileAnalytics,
		IconSparkles,
		IconCheck,
		IconAlertTriangle
	} from '$lib/icons';
	import { toastStore } from '$lib/stores/toast';
	import type { PastMembersPageData } from './+page';

	// ═══════════════════════════════════════════════════════════════════════════
	// PAGE DATA
	// ═══════════════════════════════════════════════════════════════════════════

	const data = $derived($page.data as PastMembersPageData);

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE (Svelte 5 Runes)
	// ═══════════════════════════════════════════════════════════════════════════

	// Dashboard state
	let overview = $state<DashboardOverview | null>(null);
	let churnReasons = $state<ChurnReason[]>([]);
	let selectedPeriod = $state<TimePeriod>('30d');
	let members = $state<PastMember[]>([]);
	let searchQuery = $state('');
	let selectedMembers = $state<Set<number>>(new Set());

	// Loading states
	let isLoading = $state(true);
	let isRefreshing = $state(false);
	let isLoadingMembers = $state(false);
	let error = $state<string | null>(null);

	// Pagination
	let pagination = $state({
		current_page: 1,
		last_page: 1,
		per_page: 20,
		total: 0
	});

	// Win-back email modal
	let showEmailModal = $state(false);
	let emailTemplate = $state<'30_free' | 'discount' | 'missed' | 'custom'>('30_free');
	let customSubject = $state('');
	let customBody = $state('');
	let offerCode = $state('');
	let discountPercent = $state(20);
	let sending = $state(false);

	// Survey modal
	let showSurveyModal = $state(false);
	let surveyIncentive = $state('');

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED STATE
	// ═══════════════════════════════════════════════════════════════════════════

	const periodStats = $derived(overview?.periods[selectedPeriod] as PeriodStats | undefined);
	const hasMembers = $derived(members.length > 0);
	const hasSelectedMembers = $derived(selectedMembers.size > 0);
	const allSelected = $derived(selectedMembers.size === members.length && members.length > 0);
	const periodLabel = $derived(TIME_PERIOD_LABELS[selectedPeriod]);

	// ═══════════════════════════════════════════════════════════════════════════
	// EFFECTS
	// ═══════════════════════════════════════════════════════════════════════════

	// Initialize from load function data
	$effect(() => {
		if (!browser) return;

		// Handle auth errors
		if (data?.authError) {
			error = 'Session expired or access denied.';
			isLoading = false;
			return;
		}

		// Use data from load function if available
		if (data?.overview) {
			overview = data.overview;
			churnReasons = data.churnReasons ?? [];
			isLoading = false;
			loadPeriodMembers();
		} else {
			// Fallback to client-side fetch
			loadDashboard();
		}
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// FUNCTIONS - Data Loading
	// ═══════════════════════════════════════════════════════════════════════════

	async function loadDashboard(): Promise<void> {
		isLoading = true;
		error = null;

		try {
			const [overviewData, reasonsData] = await Promise.all([
				pastMembersApi.getDashboardOverview(),
				pastMembersApi.getChurnReasons()
			]);
			overview = overviewData;
			churnReasons = reasonsData;
			await loadPeriodMembers();
		} catch (e) {
			console.error('[PastMembers] Failed to load dashboard:', e);
			error = e instanceof Error ? e.message : 'Failed to load dashboard';
			toastStore.error('Failed to load dashboard data');
		} finally {
			isLoading = false;
		}
	}

	async function refreshDashboard(): Promise<void> {
		isRefreshing = true;
		error = null;

		try {
			// Invalidate cache and fetch fresh data
			await invalidateDashboardCache();

			const [overviewData, reasonsData] = await Promise.all([
				pastMembersApi.getDashboardOverview({ skipCache: true }),
				pastMembersApi.getChurnReasons(selectedPeriod)
			]);

			overview = overviewData;
			churnReasons = reasonsData;
			await loadPeriodMembers(1, true);

			toastStore.success('Dashboard refreshed');
		} catch (e) {
			console.error('[PastMembers] Failed to refresh:', e);
			toastStore.error('Failed to refresh dashboard');
		} finally {
			isRefreshing = false;
		}
	}

	async function loadPeriodMembers(page: number = 1, skipCache: boolean = false): Promise<void> {
		isLoadingMembers = true;

		try {
			const result = await pastMembersApi.getPastMembersByPeriod(
				selectedPeriod,
				page,
				pagination.per_page,
				searchQuery || undefined,
				{ skipCache }
			);

			members = result.members;
			pagination = {
				current_page: result.pagination.current_page,
				last_page: result.pagination.last_page,
				per_page: result.pagination.per_page,
				total: result.pagination.total
			};
		} catch (e) {
			console.error('[PastMembers] Failed to load members:', e);
		} finally {
			isLoadingMembers = false;
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// FUNCTIONS - User Actions
	// ═══════════════════════════════════════════════════════════════════════════

	async function handlePeriodChange(period: TimePeriod): Promise<void> {
		selectedPeriod = period;
		selectedMembers = new Set();
		await loadPeriodMembers();
	}

	async function handleSearch(): Promise<void> {
		await loadPeriodMembers();
	}

	function toggleMemberSelection(id: number): void {
		const newSet = new Set(selectedMembers);
		if (newSet.has(id)) {
			newSet.delete(id);
		} else {
			newSet.add(id);
		}
		selectedMembers = newSet;
	}

	function selectAllMembers(): void {
		if (allSelected) {
			selectedMembers = new Set();
		} else {
			selectedMembers = new Set(members.map((m) => m.id));
		}
	}

	async function handleSendBulkWinBack(): Promise<void> {
		sending = true;

		try {
			const options: import('$lib/api/past-members-dashboard').BulkEmailOptions = {
				period: selectedPeriod,
				template: emailTemplate
			};

			if (emailTemplate === 'custom' && customSubject) options.custom_subject = customSubject;
			if (emailTemplate === 'custom' && customBody) options.custom_body = customBody;
			if (offerCode) options.offer_code = offerCode;
			if (emailTemplate === 'discount') options.discount_percent = discountPercent;

			const result = await pastMembersApi.sendBulkWinBack(options);

			toastStore.success(result.message);
			showEmailModal = false;
			selectedMembers = new Set();

			// Refresh dashboard after campaign
			await refreshDashboard();
		} catch (e) {
			toastStore.error(e instanceof Error ? e.message : 'Failed to send emails');
		} finally {
			sending = false;
		}
	}

	async function handleSendBulkSurvey(): Promise<void> {
		sending = true;

		try {
			const result = await pastMembersApi.sendBulkSurvey(
				selectedPeriod,
				surveyIncentive || undefined
			);

			toastStore.success(result.message);
			showSurveyModal = false;

			// Refresh dashboard after campaign
			await refreshDashboard();
		} catch (e) {
			toastStore.error(e instanceof Error ? e.message : 'Failed to send surveys');
		} finally {
			sending = false;
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// FUNCTIONS - Utilities
	// ═══════════════════════════════════════════════════════════════════════════

	function formatCurrency(amount: number): string {
		return new Intl.NumberFormat('en-US', {
			style: 'currency',
			currency: 'USD',
			minimumFractionDigits: 0,
			maximumFractionDigits: 0
		}).format(amount);
	}

	function formatDate(dateString: string | null): string {
		if (!dateString) return '-';
		return new Date(dateString).toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'short',
			day: 'numeric'
		});
	}

	function getMemberInitials(member: PastMember): string {
		return member.name?.slice(0, 2).toUpperCase() || 'U';
	}

	function getTemplatePreview(template: string): { subject: string; description: string } {
		switch (template) {
			case '30_free':
				return {
					subject: 'Get 30 Days FREE - We Want You Back!',
					description: 'Offer a free month to entice members back'
				};
			case 'discount':
				return {
					subject: `Exclusive ${discountPercent}% Off - Just for You!`,
					description: 'Provide a percentage discount on their next subscription'
				};
			case 'missed':
				return {
					subject: "We've Missed You at Revolution Trading Pros",
					description: 'Personal reconnection email highlighting new features'
				};
			default:
				return {
					subject: customSubject || 'Custom Email',
					description: 'Your custom win-back message'
				};
		}
	}

	function openEmailModal(template: '30_free' | 'discount' | 'missed' | 'custom'): void {
		emailTemplate = template;
		showEmailModal = true;
	}

	function handleKeydown(event: KeyboardEvent): void {
		if (event.key === 'Escape') {
			showEmailModal = false;
			showSurveyModal = false;
		}
	}
</script>

<!-- ═══════════════════════════════════════════════════════════════════════════
     HEAD
     ═══════════════════════════════════════════════════════════════════════════ -->

<svelte:head>
	<title>Past Members Dashboard | Revolution Trading Pros</title>
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<svelte:window onkeydown={handleKeydown} />

<!-- ═══════════════════════════════════════════════════════════════════════════
     TEMPLATE
     ═══════════════════════════════════════════════════════════════════════════ -->

<div class="past-members-page">
	<!-- Header -->
	<div class="page-header">
		<button class="back-btn" onclick={() => goto('/admin/members')}>
			<IconArrowLeft size={20} />
			Back to Members
		</button>

		<div class="header-content">
			<div class="header-title">
				<div class="title-icon">
					<IconClock size={32} />
				</div>
				<div>
					<h1>Past Members Dashboard</h1>
					<p>Time-based win-back campaigns at your fingertips</p>
				</div>
			</div>
			<div class="header-actions">
				<button class="btn-secondary" onclick={() => (showSurveyModal = true)}>
					<IconFileAnalytics size={18} />
					Send Survey
				</button>
				<button class="btn-refresh" onclick={refreshDashboard} disabled={isRefreshing}>
					<IconRefresh size={18} class={isRefreshing ? 'spin' : ''} />
					{isRefreshing ? 'Refreshing...' : 'Refresh'}
				</button>
			</div>
		</div>
	</div>

	{#if isLoading}
		<!-- Skeleton Loading State -->
		<div class="loading-state">
			<div class="loader"></div>
			<p>Loading dashboard...</p>
		</div>
	{:else if error}
		<!-- Error State -->
		<div class="error-state">
			<div class="error-icon">
				<IconAlertTriangle size={48} />
			</div>
			<h3>Failed to load dashboard</h3>
			<p>{error}</p>
			<button class="btn-primary" onclick={loadDashboard}>
				Try Again
			</button>
		</div>
	{:else}
		<!-- Period Selector Cards -->
		<div class="period-selector">
			{#each Object.entries(TIME_PERIOD_LABELS) as [period, label]}
				{@const stats = overview?.periods[period as TimePeriod]}
				<button
					class="period-card"
					class:active={selectedPeriod === period}
					onclick={() => handlePeriodChange(period as TimePeriod)}
				>
					<div class="period-label">{label}</div>
					<div class="period-count">{stats?.total_count?.toLocaleString() || 0}</div>
					<div class="period-revenue">{formatCurrency(stats?.potential_revenue || 0)}</div>
					{#if selectedPeriod === period}
						<div class="period-indicator">
							<IconCheck size={16} />
						</div>
					{/if}
				</button>
			{/each}
		</div>

		<!-- Stats Row -->
		{#if periodStats}
			<div class="stats-row">
				<div class="stat-card primary">
					<div class="stat-icon">
						<IconUsers size={24} />
					</div>
					<div class="stat-content">
						<div class="stat-label">Past Members</div>
						<div class="stat-value">{periodStats.total_count.toLocaleString()}</div>
					</div>
				</div>

				<div class="stat-card success">
					<div class="stat-icon">
						<IconCurrencyDollar size={24} />
					</div>
					<div class="stat-content">
						<div class="stat-label">Potential Revenue</div>
						<div class="stat-value">{formatCurrency(periodStats.potential_revenue)}</div>
					</div>
				</div>

				<div class="stat-card warning">
					<div class="stat-icon">
						<IconCalendar size={24} />
					</div>
					<div class="stat-content">
						<div class="stat-label">Avg Days Since Expired</div>
						<div class="stat-value">{Math.round(periodStats.avg_days_since_expired)}</div>
					</div>
				</div>

				<div class="stat-card info">
					<div class="stat-icon">
						<IconTrendingUp size={24} />
					</div>
					<div class="stat-content">
						<div class="stat-label">Top Plan</div>
						<div class="stat-value plan-name">
							{periodStats.top_plans?.[0]?.name || 'N/A'}
						</div>
					</div>
				</div>
			</div>
		{/if}

		<!-- Churn Reasons Chart -->
		{#if churnReasons.length > 0}
			<div class="churn-reasons-section">
				<div class="section-header">
					<IconChartPie size={20} />
					<h3>Top Cancellation Reasons</h3>
				</div>
				<div class="reasons-grid">
					{#each churnReasons.slice(0, 5) as reason, i (reason.reason)}
						<div class="reason-card">
							<div class="reason-bar" style="--width: {reason.percentage}%"></div>
							<div class="reason-content">
								<span class="reason-rank">#{i + 1}</span>
								<span class="reason-text">{reason.reason || 'Unknown'}</span>
								<span class="reason-count">{reason.count} ({reason.percentage.toFixed(1)}%)</span>
							</div>
						</div>
					{/each}
				</div>
			</div>
		{/if}

		<!-- Campaign Action Bar -->
		<div class="campaign-bar">
			<div class="campaign-info">
				<IconSparkles size={20} />
				<span>Ready to win back <strong>{periodStats?.total_count || 0}</strong> members from {periodLabel}?</span>
			</div>
			<div class="campaign-actions">
				<button class="campaign-btn free" onclick={() => openEmailModal('30_free')}>
					<IconGift size={18} />
					30 Days Free
				</button>
				<button class="campaign-btn discount" onclick={() => openEmailModal('discount')}>
					<IconCurrencyDollar size={18} />
					Discount Offer
				</button>
				<button class="campaign-btn personal" onclick={() => openEmailModal('missed')}>
					<IconMail size={18} />
					Personal Email
				</button>
			</div>
		</div>

		<!-- Toolbar -->
		<div class="toolbar">
			<div class="search-box">
				<IconSearch size={18} />
				<input
					type="text"
					placeholder="Search by name or email..."
					bind:value={searchQuery}
					onkeydown={(e: KeyboardEvent) => e.key === 'Enter' && handleSearch()}
				/>
			</div>

			<div class="toolbar-actions">
				{#if hasSelectedMembers}
					<button class="btn-selected" onclick={() => (showEmailModal = true)}>
						<IconMail size={18} />
						Email {selectedMembers.size} Selected
					</button>
				{/if}
			</div>
		</div>

		<!-- Members Table -->
		<div class="members-table-container">
			{#if isLoadingMembers}
				<div class="table-loading">
					<div class="loader small"></div>
					<p>Loading members...</p>
				</div>
			{:else if !hasMembers}
				<div class="empty-state">
					<IconUsers size={64} stroke={1} />
					<h3>No past members found</h3>
					<p>No members have expired in this time period</p>
				</div>
			{:else}
				<table class="members-table">
					<thead>
						<tr>
							<th class="checkbox-col">
								<input
									type="checkbox"
									checked={allSelected}
									onchange={selectAllMembers}
									aria-label="Select all members"
								/>
							</th>
							<th>Member</th>
							<th>Last Plan</th>
							<th>Total Spent</th>
							<th>Expired</th>
							<th>Days Ago</th>
							<th>Reason</th>
						</tr>
					</thead>
					<tbody>
						{#each members as member (member.id)}
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
										<div class="member-avatar">
											{getMemberInitials(member)}
										</div>
										<div class="member-details">
											<div class="member-name">{member.name || ''}</div>
											<div class="member-email">{member.email || ''}</div>
										</div>
									</div>
								</td>
								<td>
									<span class="plan-badge">
										{member.last_membership?.plan_name || '-'}
									</span>
								</td>
								<td>
									<span class="spending">{formatCurrency(member.total_spent || 0)}</span>
								</td>
								<td>
									<span class="date">{formatDate(member.last_membership?.expired_at ?? null)}</span>
								</td>
								<td>
									<span class="days-badge">
										{member.last_membership?.days_since_expired || 0}d
									</span>
								</td>
								<td>
									<span class="reason-text-cell">
										{member.last_membership?.cancellation_reason || 'Unknown'}
									</span>
								</td>
							</tr>
						{/each}
					</tbody>
				</table>

				<!-- Pagination -->
				<div class="pagination">
					<div class="pagination-info">
						Showing {(pagination.current_page - 1) * pagination.per_page + 1} to {Math.min(pagination.current_page * pagination.per_page, pagination.total)} of {pagination.total}
					</div>
					<div class="pagination-controls">
						<button
							class="page-btn"
							disabled={pagination.current_page === 1}
							onclick={() => loadPeriodMembers(pagination.current_page - 1)}
							aria-label="Previous page"
						>
							<IconChevronLeft size={18} />
						</button>
						<span class="page-indicator">Page {pagination.current_page} of {pagination.last_page}</span>
						<button
							class="page-btn"
							disabled={pagination.current_page === pagination.last_page}
							onclick={() => loadPeriodMembers(pagination.current_page + 1)}
							aria-label="Next page"
						>
							<IconChevronRight size={18} />
						</button>
					</div>
				</div>
			{/if}
		</div>
	{/if}
</div>

<!-- Win-Back Email Modal -->
{#if showEmailModal}
	<div
		class="modal-overlay"
		role="button"
		tabindex="0"
		aria-label="Close modal"
		onclick={() => (showEmailModal = false)}
		onkeydown={(e: KeyboardEvent) => { if (e.key === 'Escape' || e.key === 'Enter' || e.key === ' ') showEmailModal = false; }}
	>
		<div
			class="modal-content"
			role="dialog"
			aria-modal="true"
			aria-labelledby="email-modal-title"
			tabindex="-1"
			onclick={(e: MouseEvent) => e.stopPropagation()}
			onkeydown={(e: KeyboardEvent) => e.stopPropagation()}
		>
			<div class="modal-header">
				<div>
					<h2 id="email-modal-title">Win-Back Campaign</h2>
					<p>Sending to all {periodStats?.total_count || 0} members from {periodLabel}</p>
				</div>
				<button class="close-btn" onclick={() => (showEmailModal = false)} aria-label="Close modal">
					<IconX size={20} />
				</button>
			</div>

			<div class="modal-body">
				<!-- Template Selection -->
				<div class="template-selector">
					<span class="selector-label">Choose Template</span>
					<div class="template-grid">
						<button
							class="template-option"
							class:active={emailTemplate === '30_free'}
							onclick={() => (emailTemplate = '30_free')}
						>
							<IconGift size={24} />
							<span class="template-name">30 Days Free</span>
							<span class="template-desc">Free trial offer</span>
						</button>
						<button
							class="template-option"
							class:active={emailTemplate === 'discount'}
							onclick={() => (emailTemplate = 'discount')}
						>
							<IconCurrencyDollar size={24} />
							<span class="template-name">Discount</span>
							<span class="template-desc">Percentage off</span>
						</button>
						<button
							class="template-option"
							class:active={emailTemplate === 'missed'}
							onclick={() => (emailTemplate = 'missed')}
						>
							<IconMail size={24} />
							<span class="template-name">We Miss You</span>
							<span class="template-desc">Personal touch</span>
						</button>
						<button
							class="template-option"
							class:active={emailTemplate === 'custom'}
							onclick={() => (emailTemplate = 'custom')}
						>
							<IconSparkles size={24} />
							<span class="template-name">Custom</span>
							<span class="template-desc">Your message</span>
						</button>
					</div>
				</div>

				<!-- Template Preview -->
				<div class="template-preview">
					<div class="preview-header">Preview</div>
					<div class="preview-subject">{getTemplatePreview(emailTemplate).subject}</div>
					<div class="preview-desc">{getTemplatePreview(emailTemplate).description}</div>
				</div>

				<!-- Discount Settings -->
				{#if emailTemplate === 'discount'}
					<div class="form-group">
						<label for="discount-percent">Discount Percentage</label>
						<div class="discount-slider">
							<input
								id="discount-percent"
								type="range"
								min="10"
								max="50"
								step="5"
								bind:value={discountPercent}
							/>
							<span class="discount-value">{discountPercent}%</span>
						</div>
					</div>
				{/if}

				<!-- Offer Code -->
				<div class="form-group">
					<label for="offer-code">Offer Code (Optional)</label>
					<input
						id="offer-code"
						type="text"
						placeholder="e.g., COMEBACK20"
						bind:value={offerCode}
					/>
				</div>

				<!-- Custom Template Fields -->
				{#if emailTemplate === 'custom'}
					<div class="form-group">
						<label for="custom-subject">Subject Line</label>
						<input
							id="custom-subject"
							type="text"
							placeholder="Enter email subject..."
							bind:value={customSubject}
						/>
					</div>
					<div class="form-group">
						<label for="custom-body">Email Body</label>
						<textarea
							id="custom-body"
							rows="8"
							placeholder="Enter email body... Use {{name}} for personalization"
							bind:value={customBody}
						></textarea>
					</div>
				{/if}
			</div>

			<div class="modal-footer">
				<button class="btn-secondary" onclick={() => (showEmailModal = false)}>Cancel</button>
				<button
					class="btn-primary"
					onclick={handleSendBulkWinBack}
					disabled={sending || (emailTemplate === 'custom' && (!customSubject || !customBody))}
				>
					<IconSend size={18} />
					{sending ? 'Sending...' : `Send to ${periodStats?.total_count || 0} Members`}
				</button>
			</div>
		</div>
	</div>
{/if}

<!-- Survey Modal -->
{#if showSurveyModal}
	<div
		class="modal-overlay"
		role="button"
		tabindex="0"
		aria-label="Close modal"
		onclick={() => (showSurveyModal = false)}
		onkeydown={(e: KeyboardEvent) => { if (e.key === 'Escape' || e.key === 'Enter' || e.key === ' ') showSurveyModal = false; }}
	>
		<div
			class="modal-content survey-modal"
			role="dialog"
			aria-modal="true"
			aria-labelledby="survey-modal-title"
			tabindex="-1"
			onclick={(e: MouseEvent) => e.stopPropagation()}
			onkeydown={(e: KeyboardEvent) => e.stopPropagation()}
		>
			<div class="modal-header">
				<div>
					<h2 id="survey-modal-title">Send Feedback Survey</h2>
					<p>Gather insights from {periodStats?.total_count || 0} past members</p>
				</div>
				<button class="close-btn" onclick={() => (showSurveyModal = false)} aria-label="Close modal">
					<IconX size={20} />
				</button>
			</div>

			<div class="modal-body">
				<div class="survey-info">
					<IconFileAnalytics size={48} />
					<p>Send a feedback survey to understand why members canceled and what would bring them back.</p>
				</div>

				<div class="form-group">
					<label for="survey-incentive">Incentive (Optional)</label>
					<input
						id="survey-incentive"
						type="text"
						placeholder="e.g., Complete the survey for 10% off your next month"
						bind:value={surveyIncentive}
					/>
					<span class="form-hint">Offering an incentive can increase response rates by up to 30%</span>
				</div>
			</div>

			<div class="modal-footer">
				<button class="btn-secondary" onclick={() => (showSurveyModal = false)}>Cancel</button>
				<button class="btn-primary" onclick={handleSendBulkSurvey} disabled={sending}>
					<IconSend size={18} />
					{sending ? 'Sending...' : 'Send Survey'}
				</button>
			</div>
		</div>
	</div>
{/if}

<!-- ═══════════════════════════════════════════════════════════════════════════
     STYLES
     ═══════════════════════════════════════════════════════════════════════════ -->

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   CSS CUSTOM PROPERTIES
	   ═══════════════════════════════════════════════════════════════════════════ */

	:root {
		--pm-bg: #0f172a;
		--pm-card-bg: rgba(30, 41, 59, 0.6);
		--pm-border: rgba(148, 163, 184, 0.1);
		--pm-text: #f1f5f9;
		--pm-text-muted: #94a3b8;
		--pm-text-dim: #64748b;
		--pm-primary: #6366f1;
		--pm-primary-light: #a5b4fc;
		--pm-success: #34d399;
		--pm-warning: #fbbf24;
		--pm-error: #f87171;
		--pm-transition: all 0.2s ease;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   PAGE LAYOUT
	   ═══════════════════════════════════════════════════════════════════════════ */

	.past-members-page {
		padding: 2rem;
		max-width: 1600px;
		margin: 0 auto;
		min-height: 100vh;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   HEADER
	   ═══════════════════════════════════════════════════════════════════════════ */

	.back-btn {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 0;
		color: var(--pm-text-muted);
		background: none;
		border: none;
		cursor: pointer;
		font-size: 0.875rem;
		margin-bottom: 1rem;
		transition: var(--pm-transition);
	}

	.back-btn:hover {
		color: var(--pm-primary-light);
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
		background: linear-gradient(135deg, #6366f1, #8b5cf6);
		border-radius: 20px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: white;
		box-shadow: 0 8px 32px rgba(99, 102, 241, 0.3);
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
		color: var(--pm-text-dim);
		margin: 0.25rem 0 0;
	}

	.header-actions {
		display: flex;
		gap: 0.75rem;
	}

	.btn-secondary,
	.btn-refresh {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background: rgba(148, 163, 184, 0.1);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 12px;
		color: var(--pm-text-muted);
		font-weight: 500;
		cursor: pointer;
		transition: var(--pm-transition);
	}

	.btn-secondary:hover,
	.btn-refresh:hover:not(:disabled) {
		border-color: rgba(99, 102, 241, 0.3);
		color: var(--pm-primary-light);
	}

	.btn-refresh:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	:global(.btn-refresh .spin) {
		animation: spin 1s linear infinite;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   LOADING & ERROR STATES
	   ═══════════════════════════════════════════════════════════════════════════ */

	.loading-state,
	.error-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 6rem 2rem;
		color: var(--pm-text-dim);
	}

	.error-state {
		color: var(--pm-text);
	}

	.error-icon {
		width: 80px;
		height: 80px;
		border-radius: 50%;
		background: rgba(248, 113, 113, 0.15);
		display: flex;
		align-items: center;
		justify-content: center;
		margin-bottom: 24px;
		color: var(--pm-error);
	}

	.error-state h3 {
		margin: 0 0 0.5rem;
	}

	.error-state p {
		color: var(--pm-text-muted);
		margin: 0 0 1.5rem;
	}

	.loader {
		width: 48px;
		height: 48px;
		border: 4px solid rgba(99, 102, 241, 0.2);
		border-top-color: var(--pm-primary);
		border-radius: 50%;
		animation: spin 1s linear infinite;
	}

	.loader.small {
		width: 32px;
		height: 32px;
		border-width: 3px;
	}

	.table-loading {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 2rem;
		color: var(--pm-text-dim);
		gap: 1rem;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   PERIOD SELECTOR
	   ═══════════════════════════════════════════════════════════════════════════ */

	.period-selector {
		display: grid;
		grid-template-columns: repeat(6, 1fr);
		gap: 1rem;
		margin-bottom: 2rem;
	}

	.period-card {
		position: relative;
		padding: 1.25rem;
		background: var(--pm-card-bg);
		border: 2px solid var(--pm-border);
		border-radius: 16px;
		cursor: pointer;
		transition: var(--pm-transition);
		text-align: left;
	}

	.period-card:hover {
		border-color: rgba(99, 102, 241, 0.3);
		transform: translateY(-2px);
	}

	.period-card.active {
		border-color: var(--pm-primary);
		background: linear-gradient(135deg, rgba(99, 102, 241, 0.15), rgba(139, 92, 246, 0.1));
	}

	.period-label {
		font-size: 0.8125rem;
		color: var(--pm-text-muted);
		font-weight: 500;
	}

	.period-count {
		font-size: 1.75rem;
		font-weight: 800;
		color: var(--pm-text);
		margin: 0.25rem 0;
	}

	.period-revenue {
		font-size: 0.875rem;
		color: var(--pm-success);
		font-weight: 600;
	}

	.period-indicator {
		position: absolute;
		top: 0.75rem;
		right: 0.75rem;
		width: 24px;
		height: 24px;
		background: var(--pm-primary);
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		color: white;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   STATS ROW
	   ═══════════════════════════════════════════════════════════════════════════ */

	.stats-row {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 1.5rem;
		margin-bottom: 2rem;
	}

	.stat-card {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1.25rem 1.5rem;
		background: var(--pm-card-bg);
		border: 1px solid var(--pm-border);
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

	.stat-card.primary .stat-icon {
		background: rgba(99, 102, 241, 0.15);
		color: #818cf8;
	}

	.stat-card.success .stat-icon {
		background: rgba(16, 185, 129, 0.15);
		color: var(--pm-success);
	}

	.stat-card.warning .stat-icon {
		background: rgba(251, 191, 36, 0.15);
		color: var(--pm-warning);
	}

	.stat-card.info .stat-icon {
		background: rgba(56, 189, 248, 0.15);
		color: #38bdf8;
	}

	.stat-label {
		font-size: 0.75rem;
		color: var(--pm-text-dim);
		text-transform: uppercase;
		letter-spacing: 0.05em;
		font-weight: 600;
	}

	.stat-value {
		font-size: 1.5rem;
		font-weight: 800;
		color: var(--pm-text);
	}

	.stat-value.plan-name {
		font-size: 1rem;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   CHURN REASONS
	   ═══════════════════════════════════════════════════════════════════════════ */

	.churn-reasons-section {
		background: var(--pm-card-bg);
		border: 1px solid var(--pm-border);
		border-radius: 20px;
		padding: 1.5rem;
		margin-bottom: 2rem;
	}

	.section-header {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		color: var(--pm-text);
		margin-bottom: 1.25rem;
	}

	.section-header h3 {
		font-size: 1.125rem;
		font-weight: 600;
		margin: 0;
	}

	.reasons-grid {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.reason-card {
		position: relative;
		background: rgba(15, 23, 42, 0.4);
		border-radius: 12px;
		overflow: hidden;
	}

	.reason-bar {
		position: absolute;
		left: 0;
		top: 0;
		bottom: 0;
		width: var(--width);
		background: linear-gradient(90deg, rgba(99, 102, 241, 0.2), rgba(99, 102, 241, 0.05));
		transition: width 0.5s ease-out;
	}

	.reason-content {
		position: relative;
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1rem 1.25rem;
	}

	.reason-rank {
		width: 28px;
		height: 28px;
		background: rgba(99, 102, 241, 0.2);
		border-radius: 8px;
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 0.75rem;
		font-weight: 700;
		color: var(--pm-primary-light);
	}

	.reason-text {
		flex: 1;
		color: var(--pm-text);
		font-weight: 500;
		text-transform: capitalize;
	}

	.reason-count {
		color: var(--pm-text-muted);
		font-size: 0.875rem;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   CAMPAIGN BAR
	   ═══════════════════════════════════════════════════════════════════════════ */

	.campaign-bar {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 1.25rem 1.5rem;
		background: linear-gradient(135deg, rgba(99, 102, 241, 0.1), rgba(139, 92, 246, 0.05));
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 16px;
		margin-bottom: 1.5rem;
	}

	.campaign-info {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		color: var(--pm-primary-light);
	}

	.campaign-info strong {
		color: var(--pm-text);
	}

	.campaign-actions {
		display: flex;
		gap: 0.75rem;
	}

	.campaign-btn {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1rem;
		border-radius: 10px;
		font-weight: 600;
		font-size: 0.875rem;
		cursor: pointer;
		transition: var(--pm-transition);
		border: 1px solid;
	}

	.campaign-btn.free {
		background: rgba(16, 185, 129, 0.15);
		border-color: rgba(16, 185, 129, 0.3);
		color: var(--pm-success);
	}

	.campaign-btn.free:hover {
		background: rgba(16, 185, 129, 0.25);
	}

	.campaign-btn.discount {
		background: rgba(251, 191, 36, 0.15);
		border-color: rgba(251, 191, 36, 0.3);
		color: var(--pm-warning);
	}

	.campaign-btn.discount:hover {
		background: rgba(251, 191, 36, 0.25);
	}

	.campaign-btn.personal {
		background: rgba(244, 63, 94, 0.15);
		border-color: rgba(244, 63, 94, 0.3);
		color: #fb7185;
	}

	.campaign-btn.personal:hover {
		background: rgba(244, 63, 94, 0.25);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   TOOLBAR
	   ═══════════════════════════════════════════════════════════════════════════ */

	.toolbar {
		display: flex;
		justify-content: space-between;
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
		background: var(--pm-card-bg);
		border: 1px solid var(--pm-border);
		border-radius: 12px;
		color: var(--pm-text-muted);
	}

	.search-box input {
		flex: 1;
		background: none;
		border: none;
		color: var(--pm-text);
		font-size: 0.9375rem;
		outline: none;
	}

	.search-box input::placeholder {
		color: var(--pm-text-dim);
	}

	.toolbar-actions {
		display: flex;
		gap: 0.75rem;
	}

	.btn-selected {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background: linear-gradient(135deg, #6366f1, #8b5cf6);
		border: none;
		border-radius: 10px;
		color: white;
		font-weight: 600;
		cursor: pointer;
		box-shadow: 0 4px 14px rgba(99, 102, 241, 0.3);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   TABLE
	   ═══════════════════════════════════════════════════════════════════════════ */

	.members-table-container {
		background: var(--pm-card-bg);
		border: 1px solid var(--pm-border);
		border-radius: 16px;
		overflow: hidden;
	}

	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 2rem;
		color: var(--pm-text-dim);
	}

	.empty-state h3 {
		color: var(--pm-text);
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
		color: var(--pm-text-muted);
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.members-table tbody tr {
		border-top: 1px solid var(--pm-border);
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
		accent-color: var(--pm-primary);
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
		background: linear-gradient(135deg, #64748b, var(--pm-text-muted));
		display: flex;
		align-items: center;
		justify-content: center;
		font-weight: 700;
		font-size: 0.875rem;
		color: white;
	}

	.member-name {
		font-weight: 600;
		color: var(--pm-text);
	}

	.member-email {
		font-size: 0.8125rem;
		color: var(--pm-text-dim);
	}

	.plan-badge {
		display: inline-flex;
		padding: 0.25rem 0.75rem;
		background: rgba(99, 102, 241, 0.15);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 9999px;
		font-size: 0.75rem;
		font-weight: 600;
		color: var(--pm-primary-light);
	}

	.spending {
		font-weight: 600;
		color: var(--pm-success);
	}

	.date {
		font-size: 0.875rem;
		color: var(--pm-text-muted);
	}

	.days-badge {
		display: inline-flex;
		padding: 0.25rem 0.5rem;
		background: rgba(251, 191, 36, 0.15);
		border: 1px solid rgba(251, 191, 36, 0.2);
		border-radius: 6px;
		font-size: 0.75rem;
		font-weight: 600;
		color: var(--pm-warning);
	}

	.reason-text-cell {
		font-size: 0.8125rem;
		color: var(--pm-text-muted);
		text-transform: capitalize;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   PAGINATION
	   ═══════════════════════════════════════════════════════════════════════════ */

	.pagination {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1rem 1.5rem;
		border-top: 1px solid var(--pm-border);
	}

	.pagination-info {
		font-size: 0.875rem;
		color: var(--pm-text-dim);
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
		color: var(--pm-text-muted);
		cursor: pointer;
		transition: var(--pm-transition);
	}

	.page-btn:hover:not(:disabled) {
		background: rgba(99, 102, 241, 0.15);
		border-color: rgba(99, 102, 241, 0.3);
		color: var(--pm-primary-light);
	}

	.page-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.page-indicator {
		font-size: 0.875rem;
		color: var(--pm-text-muted);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   MODAL
	   ═══════════════════════════════════════════════════════════════════════════ */

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
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 24px;
		width: 100%;
		max-width: 600px;
		max-height: 90vh;
		overflow-y: auto;
		box-shadow: 0 24px 48px rgba(0, 0, 0, 0.4);
	}

	.modal-content.survey-modal {
		max-width: 500px;
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		padding: 1.5rem;
		border-bottom: 1px solid var(--pm-border);
	}

	.modal-header h2 {
		font-size: 1.5rem;
		font-weight: 700;
		color: var(--pm-text);
		margin: 0;
	}

	.modal-header p {
		font-size: 0.875rem;
		color: var(--pm-text-dim);
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
		color: var(--pm-text-muted);
		cursor: pointer;
		transition: var(--pm-transition);
	}

	.close-btn:hover {
		background: rgba(148, 163, 184, 0.2);
	}

	.modal-body {
		padding: 1.5rem;
	}

	/* Template Selector */
	.template-selector {
		margin-bottom: 1.5rem;
	}

	.selector-label {
		display: block;
		font-size: 0.75rem;
		font-weight: 600;
		color: var(--pm-text-muted);
		text-transform: uppercase;
		letter-spacing: 0.05em;
		margin-bottom: 0.75rem;
	}

	.template-grid {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 0.75rem;
	}

	.template-option {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.5rem;
		padding: 1rem;
		background: var(--pm-card-bg);
		border: 2px solid var(--pm-border);
		border-radius: 12px;
		cursor: pointer;
		transition: var(--pm-transition);
		color: var(--pm-text-muted);
	}

	.template-option:hover {
		border-color: rgba(99, 102, 241, 0.3);
	}

	.template-option.active {
		border-color: var(--pm-primary);
		background: rgba(99, 102, 241, 0.1);
		color: var(--pm-primary-light);
	}

	.template-name {
		font-weight: 600;
		font-size: 0.8125rem;
		color: var(--pm-text);
	}

	.template-desc {
		font-size: 0.6875rem;
		color: var(--pm-text-dim);
	}

	/* Template Preview */
	.template-preview {
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid var(--pm-border);
		border-radius: 12px;
		padding: 1rem 1.25rem;
		margin-bottom: 1.5rem;
	}

	.preview-header {
		font-size: 0.6875rem;
		font-weight: 600;
		color: var(--pm-text-dim);
		text-transform: uppercase;
		letter-spacing: 0.1em;
		margin-bottom: 0.5rem;
	}

	.preview-subject {
		font-weight: 600;
		color: var(--pm-text);
		margin-bottom: 0.25rem;
	}

	.preview-desc {
		font-size: 0.8125rem;
		color: var(--pm-text-muted);
	}

	/* Form Elements */
	.form-group {
		margin-bottom: 1rem;
	}

	.form-group label {
		display: block;
		font-size: 0.8125rem;
		font-weight: 500;
		color: var(--pm-text-muted);
		margin-bottom: 0.5rem;
	}

	.form-group input,
	.form-group textarea {
		width: 100%;
		padding: 0.875rem 1rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 12px;
		color: var(--pm-text);
		font-size: 0.9375rem;
		font-family: inherit;
		resize: vertical;
		transition: border-color 0.2s;
	}

	.form-group input:focus,
	.form-group textarea:focus {
		outline: none;
		border-color: rgba(99, 102, 241, 0.5);
	}

	.form-hint {
		display: block;
		font-size: 0.75rem;
		color: var(--pm-text-dim);
		margin-top: 0.5rem;
	}

	.discount-slider {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.discount-slider input[type='range'] {
		flex: 1;
		height: 8px;
		background: rgba(99, 102, 241, 0.2);
		border-radius: 4px;
		-webkit-appearance: none;
		appearance: none;
	}

	.discount-slider input[type='range']::-webkit-slider-thumb {
		-webkit-appearance: none;
		width: 20px;
		height: 20px;
		background: var(--pm-primary);
		border-radius: 50%;
		cursor: pointer;
	}

	.discount-value {
		min-width: 48px;
		padding: 0.5rem 0.75rem;
		background: rgba(99, 102, 241, 0.15);
		border-radius: 8px;
		font-weight: 700;
		color: var(--pm-primary-light);
		text-align: center;
	}

	/* Survey Modal */
	.survey-info {
		text-align: center;
		padding: 1.5rem;
		margin-bottom: 1.5rem;
		color: var(--pm-text-muted);
	}

	.survey-info p {
		margin: 1rem 0 0;
		line-height: 1.6;
	}

	.modal-footer {
		display: flex;
		justify-content: flex-end;
		gap: 0.75rem;
		padding: 1.5rem;
		border-top: 1px solid var(--pm-border);
	}

	.btn-primary {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.875rem 1.5rem;
		background: linear-gradient(135deg, #6366f1, #8b5cf6);
		border: none;
		border-radius: 12px;
		color: white;
		font-weight: 600;
		cursor: pointer;
		box-shadow: 0 4px 14px rgba(99, 102, 241, 0.3);
		transition: var(--pm-transition);
	}

	.btn-primary:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow: 0 8px 24px rgba(99, 102, 241, 0.4);
	}

	.btn-primary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
		transform: none;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (max-width: 1200px) {
		.period-selector {
			grid-template-columns: repeat(3, 1fr);
		}

		.stats-row {
			grid-template-columns: repeat(2, 1fr);
		}

		.template-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	@media (max-width: 768px) {
		.past-members-page {
			padding: 1rem;
		}

		.header-content {
			flex-direction: column;
			align-items: flex-start;
			gap: 1rem;
		}

		.period-selector {
			grid-template-columns: repeat(2, 1fr);
		}

		.stats-row {
			grid-template-columns: 1fr;
		}

		.campaign-bar {
			flex-direction: column;
			gap: 1rem;
		}

		.toolbar {
			flex-direction: column;
		}

		.search-box {
			max-width: 100%;
		}

		.members-table-container {
			overflow-x: auto;
		}

		.members-table {
			min-width: 700px;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   REDUCED MOTION
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (prefers-reduced-motion: reduce) {
		.loader,
		:global(.btn-refresh .spin) {
			animation: none;
		}

		.period-card:hover,
		.btn-primary:hover:not(:disabled) {
			transform: none;
		}
	}
</style>

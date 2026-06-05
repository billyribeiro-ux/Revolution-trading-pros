<script lang="ts">
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import { logger } from '$lib/utils/logger';
	import {
		getSubscriptions,
		getSubscriptionStats,
		pauseSubscription,
		resumeSubscription,
		cancelSubscription,
		reactivateSubscription,
		retryPayment,
		getUpcomingRenewals,
		getFailedPayments
	} from '$lib/api/subscriptions';
	import type {
		Subscription,
		SubscriptionStatus,
		SubscriptionStats
	} from '$lib/stores/subscriptions.svelte';
	import { connections, getIsPaymentConnected } from '$lib/stores/connections.svelte';
	import ServiceConnectionStatus from '$lib/components/admin/ServiceConnectionStatus.svelte';
	import SubscriptionDetailDrawer from '$lib/components/admin/SubscriptionDetailDrawer.svelte';
	import SubscriptionFormModal from '$lib/components/admin/SubscriptionFormModal.svelte';
	// FIX-2026-04-26: Tabler icons replace raw inline <svg> blocks.
	import IconPlus from '@tabler/icons-svelte-runes/icons/plus';
	import IconCircleXFilled from '@tabler/icons-svelte-runes/icons/circle-x-filled';
	import IconClockHour3 from '@tabler/icons-svelte-runes/icons/clock-hour-3';
	import IconCircleCheck from '@tabler/icons-svelte-runes/icons/circle-check';
	import IconCurrencyDollar from '@tabler/icons-svelte-runes/icons/currency-dollar';
	import IconTrendingDown from '@tabler/icons-svelte-runes/icons/trending-down';
	import IconChartBar from '@tabler/icons-svelte-runes/icons/chart-bar';

	// State
	let connectionLoading = $state(true);
	let subscriptions = $state<Subscription[]>([]);
	let stats = $state<SubscriptionStats | null>(null);
	let upcomingRenewals = $state<Subscription[]>([]);
	let failedPayments = $state<ReturnType<typeof getFailedPayments>>([]);
	let loading = $state(true);
	let error = $state('');

	// Filters
	let statusFilter = $state<SubscriptionStatus | 'all'>('all');
	let searchQuery = $state('');
	let intervalFilter = $state<'all' | 'monthly' | 'quarterly' | 'yearly'>('all');
	let sortBy = $state<'date' | 'price' | 'status'>('date');

	// Modal state
	let selectedSubscription = $state<Subscription | null>(null);
	let showCancelModal = $state(false);
	let showPauseModal = $state(false);
	let cancelReason = $state('');
	let pauseReason = $state('');
	let cancelImmediate = $state(false);

	// Drawer and Form Modal state
	let showDetailDrawer = $state(false);
	let showFormModal = $state(false);
	let formModalMode = $state<'create' | 'edit'>('create');

	// Pagination (reserved for future implementation)
	// let currentPage = $state(1);
	// let perPage = $state(20);

	// Debounce timer for search
	let searchDebounceTimer: ReturnType<typeof setTimeout> | null = null;

	// Svelte 5: Initialize on mount with cleanup
	onMount(() => {
		if (!browser) return;

		const init = async () => {
			// Load connection status first
			await connections.load();
			connectionLoading = false;

			// Only load data if payment is connected
			if (getIsPaymentConnected()) {
				await loadData();
			} else {
				loading = false;
			}
		};
		init();

		// Cleanup on destroy
		return () => {
			if (searchDebounceTimer) {
				clearTimeout(searchDebounceTimer);
			}
		};
	});

	// Debounced search handler
	function handleSearchInput() {
		if (searchDebounceTimer) {
			clearTimeout(searchDebounceTimer);
		}
		searchDebounceTimer = setTimeout(() => {
			loadData();
		}, 300); // 300ms debounce
	}

	async function loadData() {
		loading = true;
		error = '';

		try {
			// Use Promise.allSettled for robust error handling - prevents freezing if one API fails
			const results = await Promise.allSettled([
				getSubscriptions(
					{
						...(statusFilter !== 'all' && { status: [statusFilter] }),
						...(intervalFilter !== 'all' && { interval: [intervalFilter] }),
						...(searchQuery && { searchQuery })
					},
					true
				), // isAdmin = true for admin page
				getSubscriptionStats(),
				getUpcomingRenewals(),
				getFailedPayments()
			]);

			// Extract results with fallbacks for failed requests
			const [subsResult, statsResult, renewalsResult, failedResult] = results;

			subscriptions = subsResult.status === 'fulfilled' ? subsResult.value : [];
			stats =
				statsResult.status === 'fulfilled'
					? statsResult.value
					: {
							total: 0,
							active: 0,
							totalActive: 0,
							newThisMonth: 0,
							onHold: 0,
							cancelled: 0,
							expired: 0,
							pendingCancel: 0,
							monthlyRecurringRevenue: 0,
							churnRate: 0,
							averageLifetimeValue: 0
						};
			upcomingRenewals = renewalsResult.status === 'fulfilled' ? renewalsResult.value : [];
			failedPayments = failedResult.status === 'fulfilled' ? failedResult.value : [];

			// Check if main subscriptions call failed
			if (subsResult.status === 'rejected') {
				error = 'Failed to load subscriptions. Some data may be unavailable.';
				logger.error('Subscriptions load error', { error: subsResult.reason });
			}
		} catch (err) {
			error = 'Failed to load subscription data';
			logger.error('Subscriptions load failed', { error: err });
		} finally {
			loading = false;
		}
	}

	// Filter and sort subscriptions
	let getFilteredSubscriptions = $derived(
		subscriptions
			.filter((sub) => {
				if (statusFilter !== 'all' && sub.status !== statusFilter) return false;
				if (intervalFilter !== 'all' && sub.interval !== intervalFilter) return false;
				if (searchQuery) {
					const query = searchQuery.toLowerCase();
					return (
						sub.productName.toLowerCase().includes(query) ||
						sub.userId.toLowerCase().includes(query) ||
						sub.id.toLowerCase().includes(query)
					);
				}
				return true;
			})
			.sort((a, b) => {
				if (sortBy === 'date') {
					return new Date(b.startDate).getTime() - new Date(a.startDate).getTime();
				} else if (sortBy === 'price') {
					return b.price - a.price;
				} else {
					return a.status.localeCompare(b.status);
				}
			})
	);

	// Actions
	async function handlePause(subscription: Subscription) {
		selectedSubscription = subscription;
		showPauseModal = true;
	}

	async function confirmPause() {
		if (!selectedSubscription || !pauseReason.trim()) return;

		try {
			await pauseSubscription(selectedSubscription.id, pauseReason);
			showPauseModal = false;
			pauseReason = '';
			selectedSubscription = null;
			await loadData();
		} catch (err) {
			logger.error('Failed to pause subscription', { error: err });
		}
	}

	async function handleResume(subscription: Subscription) {
		try {
			await resumeSubscription(subscription.id);
			await loadData();
		} catch (err) {
			logger.error('Failed to resume subscription', { error: err });
		}
	}

	async function handleCancel(subscription: Subscription) {
		selectedSubscription = subscription;
		showCancelModal = true;
	}

	async function confirmCancel() {
		if (!selectedSubscription || !cancelReason.trim()) return;

		try {
			await cancelSubscription(selectedSubscription.id, cancelReason, cancelImmediate);
			showCancelModal = false;
			cancelReason = '';
			cancelImmediate = false;
			selectedSubscription = null;
			await loadData();
		} catch (err) {
			logger.error('Failed to cancel subscription', { error: err });
		}
	}

	async function handleReactivate(subscription: Subscription) {
		try {
			await reactivateSubscription(subscription.id);
			await loadData();
		} catch (err) {
			logger.error('Failed to reactivate subscription', { error: err });
		}
	}

	async function handleRetryPayment(subscriptionId: string, paymentId: string) {
		try {
			await retryPayment(subscriptionId, paymentId);
			await loadData();
		} catch (err) {
			logger.error('Failed to retry payment', { error: err });
		}
	}

	function formatCurrency(amount: number): string {
		return new Intl.NumberFormat('en-US', {
			style: 'currency',
			currency: 'USD'
		}).format(amount);
	}

	function formatDate(dateString: string): string {
		return new Date(dateString).toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'short',
			day: 'numeric'
		});
	}

	function getIntervalLabel(interval: string): string {
		return interval.charAt(0).toUpperCase() + interval.slice(1);
	}

	function openSubscriptionDetail(subscription: Subscription) {
		selectedSubscription = subscription;
		showDetailDrawer = true;
	}

	function openCreateModal() {
		selectedSubscription = null;
		formModalMode = 'create';
		showFormModal = true;
	}

	function openEditModal(subscription: Subscription) {
		selectedSubscription = subscription;
		formModalMode = 'edit';
		showFormModal = true;
		showDetailDrawer = false;
	}

	function handleSubscriptionSaved() {
		loadData();
	}
</script>

<svelte:head>
	<title>Subscription Management - Admin</title>
</svelte:head>

<div class="admin-subscriptions">
	<div class="admin-page-container">
		<!-- Animated Background -->
		<div class="background-effects">
			<div class="background-blob background-blob--one"></div>
			<div class="background-blob background-blob--two"></div>
			<div class="background-blob background-blob--three"></div>
		</div>
		<!-- Header - Centered Style -->
		<header class="page-header">
			<h1>Subscription Management</h1>
			<p class="subtitle">Monitor and manage all customer subscriptions</p>
			{#if getIsPaymentConnected()}
				<div class="header-actions">
					<button class="btn-primary" onclick={openCreateModal}>
						<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: plus -->
						<IconPlus size={20} aria-hidden="true" />
						Create Subscription
					</button>
				</div>
			{/if}
		</header>

		<!-- Connection Check -->
		{#if connectionLoading}
			<div class="loading">
				<div class="spinner"></div>
				<p>Checking connection...</p>
			</div>
		{:else if !getIsPaymentConnected()}
			<ServiceConnectionStatus feature="payment" variant="card" showFeatures={true} />
		{:else}
			{#if error}
				<div class="error-banner">
					<span>{error}</span>
				</div>
			{/if}

			<!-- Stats Overview -->
			{#if stats}
				<div class="stats-grid">
					<div class="stat-card">
						<div class="stat-card__header">
							<h3>Active Subscriptions</h3>
							<div class="stat-card__icon" data-tone="emerald">
								<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: circle-check (active subs stat) -->
								<IconCircleCheck size={20} aria-hidden="true" />
							</div>
						</div>
						<p class="stat-card__value">{stats.totalActive}</p>
						<p class="stat-card__note stat-card__note--positive">
							{stats.newThisMonth > 0 ? '+' : ''}{stats.newThisMonth} this month
						</p>
					</div>

					<div class="stat-card">
						<div class="stat-card__header">
							<h3>Monthly Revenue</h3>
							<div class="stat-card__icon" data-tone="blue">
								<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: currency-dollar (revenue stat) -->
								<IconCurrencyDollar size={20} aria-hidden="true" />
							</div>
						</div>
						<p class="stat-card__value">
							{formatCurrency(stats.monthlyRecurringRevenue)}
						</p>
						<p class="stat-card__note">MRR</p>
					</div>

					<div class="stat-card">
						<div class="stat-card__header">
							<h3>Churn Rate</h3>
							<div class="stat-card__icon" data-tone="orange">
								<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: trending-down (churn stat) -->
								<IconTrendingDown size={20} aria-hidden="true" />
							</div>
						</div>
						<p class="stat-card__value">{stats.churnRate.toFixed(1)}%</p>
						<p class="stat-card__note">Last 30 days</p>
					</div>

					<div class="stat-card">
						<div class="stat-card__header">
							<h3>Avg Lifetime Value</h3>
							<div class="stat-card__icon" data-tone="purple">
								<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: chart-bar (LTV stat) -->
								<IconChartBar size={20} aria-hidden="true" />
							</div>
						</div>
						<p class="stat-card__value">
							{formatCurrency(stats.averageLifetimeValue)}
						</p>
						<p class="stat-card__note">Per customer</p>
					</div>
				</div>
			{/if}

			<!-- Alerts -->
			{#if failedPayments.length > 0}
				<div class="alert-card" data-tone="red">
					<h3 class="alert-card__title">
						<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: circle-x error -->
						<IconCircleXFilled size={20} aria-hidden="true" />
						{failedPayments.length} Failed Payment{failedPayments.length > 1 ? 's' : ''}
					</h3>
					<div class="alert-card__list">
						{#each failedPayments.slice(0, 3) as payment (payment.id)}
							<div class="alert-card__row">
								<span>{payment.id}</span>
								<button
									onclick={() => handleRetryPayment(payment.id, payment.id)}
									class="link-action link-action--success"
								>
									Retry Payment
								</button>
							</div>
						{/each}
					</div>
				</div>
			{/if}

			{#if upcomingRenewals.length > 0}
				<div class="alert-card" data-tone="blue">
					<h3 class="alert-card__title">
						<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: clock-hour (pending) -->
						<IconClockHour3 size={20} aria-hidden="true" />
						{upcomingRenewals.length} Renewal{upcomingRenewals.length > 1 ? 's' : ''} in Next 7 Days
					</h3>
					<div class="alert-card__list">
						{#each upcomingRenewals.slice(0, 3) as renewal (renewal.id)}
							<div class="alert-card__row">
								<span>{renewal.productName}</span>
								<span class="alert-card__date">{formatDate(renewal.nextPaymentDate)}</span>
							</div>
						{/each}
					</div>
				</div>
			{/if}

			<!-- Filters -->
			<div class="filters-panel">
				<div class="filters-grid">
					<div class="form-field">
						<label for="search" class="form-label">Search</label>
						<input
							type="text"
							id="search"
							name="search"
							bind:value={searchQuery}
							oninput={handleSearchInput}
							placeholder="Search subscriptions..."
							class="form-control"
						/>
					</div>

					<div class="form-field">
						<label for="status" class="form-label">Status</label>
						<select id="status" bind:value={statusFilter} onchange={loadData} class="form-control">
							<option value="all">All Statuses</option>
							<option value="active">Active</option>
							<option value="pending">Pending</option>
							<option value="on-hold">On Hold</option>
							<option value="cancelled">Cancelled</option>
							<option value="expired">Expired</option>
							<option value="pending-cancel">Pending Cancel</option>
						</select>
					</div>

					<div class="form-field">
						<label for="interval" class="form-label">Interval</label>
						<select
							id="interval"
							bind:value={intervalFilter}
							onchange={loadData}
							class="form-control"
						>
							<option value="all">All Intervals</option>
							<option value="monthly">Monthly</option>
							<option value="quarterly">Quarterly</option>
							<option value="yearly">Yearly</option>
						</select>
					</div>

					<div class="form-field">
						<label for="sort" class="form-label">Sort By</label>
						<select id="sort" bind:value={sortBy} class="form-control">
							<option value="date">Date</option>
							<option value="price">Price</option>
							<option value="status">Status</option>
						</select>
					</div>
				</div>
			</div>

			<!-- Subscriptions Table -->
			<div class="table-card">
				<div class="table-scroll">
					<table class="subscriptions-table">
						<thead>
							<tr>
								<th>Product</th>
								<th>Customer</th>
								<th>Status</th>
								<th>Interval</th>
								<th>Price</th>
								<th>Next Payment</th>
								<th>Actions</th>
							</tr>
						</thead>
						<tbody>
							{#if loading}
								<tr>
									<td colspan="7" class="table-state">
										<div class="table-state__content">
											<div class="table-spinner"></div>
											Loading subscriptions...
										</div>
									</td>
								</tr>
							{:else if getFilteredSubscriptions.length === 0}
								<tr>
									<td colspan="7" class="table-state">No subscriptions found</td>
								</tr>
							{:else}
								{#each getFilteredSubscriptions as subscription (subscription.id)}
									<tr class="subscription-row" onclick={() => openSubscriptionDetail(subscription)}>
										<td>
											<div class="table-primary">{subscription.productName}</div>
											<div class="table-muted">{subscription.id.slice(0, 8)}...</div>
										</td>
										<td>
											<div class="table-secondary">{subscription.userId.slice(0, 8)}...</div>
										</td>
										<td>
											<span class="status-badge" data-status={subscription.status}>
												{subscription.status}
											</span>
										</td>
										<td class="table-secondary">
											{getIntervalLabel(subscription.interval)}
										</td>
										<td class="table-primary">
											{formatCurrency(subscription.price)}
										</td>
										<td class="table-secondary">
											{formatDate(subscription.nextPaymentDate)}
										</td>
										<td>
											<!-- FIX-2026-04-26 (audit 02 §P2-3): action buttons used to bubble
											     into the row's openSubscriptionDetail handler — clicking Cancel
											     opened both the cancel modal and the detail drawer. Each handler
											     now stopPropagation()s. -->
											<div class="row-actions">
												<button
													onclick={(e) => {
														e.stopPropagation();
														openSubscriptionDetail(subscription);
													}}
													class="link-action link-action--info"
													title="View Details"
												>
													View
												</button>
												{#if subscription.status === 'active'}
													<button
														onclick={(e) => {
															e.stopPropagation();
															handlePause(subscription);
														}}
														class="link-action link-action--warning"
														title="Pause"
													>
														Pause
													</button>
													<button
														onclick={(e) => {
															e.stopPropagation();
															handleCancel(subscription);
														}}
														class="link-action link-action--danger"
														title="Cancel"
													>
														Cancel
													</button>
												{:else if subscription.status === 'on-hold'}
													<button
														onclick={(e) => {
															e.stopPropagation();
															handleResume(subscription);
														}}
														class="link-action link-action--success"
														title="Resume"
													>
														Resume
													</button>
													<button
														onclick={(e) => {
															e.stopPropagation();
															handleCancel(subscription);
														}}
														class="link-action link-action--danger"
														title="Cancel"
													>
														Cancel
													</button>
												{:else if subscription.status === 'cancelled'}
													<button
														onclick={(e) => {
															e.stopPropagation();
															handleReactivate(subscription);
														}}
														class="link-action link-action--success"
														title="Reactivate"
													>
														Reactivate
													</button>
												{/if}
											</div>
										</td>
									</tr>
								{/each}
							{/if}
						</tbody>
					</table>
				</div>
			</div>
		{/if}
	</div>
</div>

<!-- Pause Modal -->
{#if showPauseModal && selectedSubscription}
	<div class="modal-backdrop">
		<div class="modal-card" role="dialog" aria-modal="true" aria-labelledby="pause-modal-title">
			<h3 id="pause-modal-title">Pause Subscription</h3>
			<p class="modal-copy">
				Are you sure you want to pause {selectedSubscription.productName}?
			</p>

			<div class="modal-field">
				<label for="pauseReason" class="form-label">
					Reason for pausing <span class="required">*</span>
				</label>
				<textarea
					id="pauseReason"
					bind:value={pauseReason}
					rows="3"
					placeholder="Enter reason..."
					class="form-control form-control--textarea form-control--warning"
				></textarea>
			</div>

			<div class="modal-actions">
				<button
					onclick={() => {
						showPauseModal = false;
						pauseReason = '';
						selectedSubscription = null;
					}}
					class="modal-button modal-button--secondary"
				>
					Cancel
				</button>
				<button
					onclick={confirmPause}
					disabled={!pauseReason.trim()}
					class="modal-button modal-button--warning"
				>
					Pause Subscription
				</button>
			</div>
		</div>
	</div>
{/if}

<!-- Cancel Modal -->
{#if showCancelModal && selectedSubscription}
	<div class="modal-backdrop">
		<div class="modal-card" role="dialog" aria-modal="true" aria-labelledby="cancel-modal-title">
			<h3 id="cancel-modal-title">Cancel Subscription</h3>
			<p class="modal-copy">
				Are you sure you want to cancel {selectedSubscription.productName}?
			</p>

			<div class="modal-field">
				<label for="cancelReason" class="form-label">
					Cancellation reason <span class="required">*</span>
				</label>
				<textarea
					id="cancelReason"
					bind:value={cancelReason}
					rows="3"
					placeholder="Enter reason..."
					class="form-control form-control--textarea form-control--danger"
				></textarea>
			</div>

			<div class="modal-field">
				<label class="checkbox-field">
					<input
						id="page-cancelimmediate"
						name="page-cancelimmediate"
						type="checkbox"
						bind:checked={cancelImmediate}
						class="checkbox-input"
					/>
					<span>Cancel immediately (don't wait until period end)</span>
				</label>
			</div>

			<div class="modal-actions">
				<button
					onclick={() => {
						showCancelModal = false;
						cancelReason = '';
						cancelImmediate = false;
						selectedSubscription = null;
					}}
					class="modal-button modal-button--secondary"
				>
					Go Back
				</button>
				<button
					onclick={confirmCancel}
					disabled={!cancelReason.trim()}
					class="modal-button modal-button--danger"
				>
					Cancel Subscription
				</button>
			</div>
		</div>
	</div>
{/if}

<!-- Subscription Detail Drawer -->
<SubscriptionDetailDrawer
	isOpen={showDetailDrawer}
	subscription={selectedSubscription}
	onClose={() => {
		showDetailDrawer = false;
		selectedSubscription = null;
	}}
	onEdit={openEditModal}
	onRefresh={loadData}
/>

<!-- Subscription Form Modal -->
<!-- {#if} gates the mount so the modal seeds a fresh form per editing
     session. The child seeds its fields once at mount (via untrack)
     instead of a wasOpen-gated reset $effect, so it MUST remount per open.
     The modal has no exit transition, so unmount-on-close is visually
     identical to its internal {#if isOpen}. -->
{#if showFormModal}
	<SubscriptionFormModal
		isOpen={showFormModal}
		mode={formModalMode}
		subscription={selectedSubscription}
		onClose={() => {
			showFormModal = false;
			selectedSubscription = null;
		}}
		onSaved={handleSubscriptionSaved}
	/>
{/if}

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * RTP ADMIN SUBSCRIPTIONS - Apple ICT7+ Principal Engineer Grade
	 * Consistent with Analytics Dashboard styling
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* Outer Container with Gradient Background */
	.admin-subscriptions {
		background: linear-gradient(135deg, #0f172a 0%, #1e293b 50%, #0f172a 100%);
		color: white;
		position: relative;
		overflow: hidden;
	}

	/* Inner Container */
	.admin-page-container {
		position: relative;
		z-index: 10;
		max-width: 1400px;
		margin: 0 auto;
		padding: 3rem 1rem;
	}

	/* Background Effects - Animated Blobs */
	.background-effects {
		position: fixed;
		inset: 0;
		pointer-events: none;
		overflow: hidden;
	}

	.background-blob {
		position: absolute;
		border-radius: 50%;
		filter: blur(80px);
		opacity: 0.15;
	}

	.background-blob--one {
		width: 600px;
		height: 600px;
		top: -200px;
		right: -200px;
		background: linear-gradient(135deg, var(--primary-500), var(--primary-600));
		animation: float 20s ease-in-out infinite;
	}

	.background-blob--two {
		width: 500px;
		height: 500px;
		bottom: -150px;
		left: -150px;
		background: linear-gradient(135deg, #3b82f6, var(--primary-600));
		animation: float 25s ease-in-out infinite reverse;
	}

	.background-blob--three {
		width: 400px;
		height: 400px;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		background: linear-gradient(135deg, #10b981, #14b8a6);
		animation: float 30s ease-in-out infinite;
	}

	@keyframes float {
		0%,
		100% {
			transform: translate(0, 0) scale(1);
		}
		33% {
			transform: translate(30px, -30px) scale(1.05);
		}
		66% {
			transform: translate(-20px, 20px) scale(0.95);
		}
	}

	/* Header - Centered Style */
	.page-header {
		text-align: center;
		margin-bottom: 2rem;
	}

	.page-header h1 {
		font-size: 2rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0 0 0.5rem 0;
	}

	.subtitle {
		color: #64748b;
		font-size: 1rem;
		margin: 0 0 1.5rem 0;
	}

	.header-actions {
		display: flex;
		justify-content: center;
		gap: 1rem;
	}

	.btn-primary {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		background: linear-gradient(135deg, var(--primary-500) 0%, var(--primary-600) 100%);
		color: var(--bg-base);
		border: none;
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.3s;
	}

	.btn-primary:hover {
		transform: translateY(-2px);
		box-shadow: 0 8px 20px rgba(230, 184, 0, 0.3);
	}

	.stats-grid {
		display: grid;
		grid-template-columns: 1fr;
		gap: 1.5rem;
		margin-bottom: 2rem;
	}

	.stat-card,
	.filters-panel,
	.table-card {
		border: 1px solid rgb(51 65 85);
		background: rgb(30 41 59 / 0.5);
		backdrop-filter: blur(8px);
	}

	.stat-card,
	.filters-panel {
		border-radius: 0.75rem;
		padding: 1.5rem;
	}

	.stat-card__header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		margin-bottom: 0.5rem;
	}

	.stat-card h3 {
		margin: 0;
		color: #94a3b8;
		font-size: 0.875rem;
		font-weight: 500;
		line-height: 1.25rem;
	}

	.stat-card__icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 2.5rem;
		height: 2.5rem;
		border-radius: 0.5rem;
	}

	.stat-card__icon[data-tone='emerald'] {
		background: rgb(16 185 129 / 0.1);
		color: #34d399;
	}

	.stat-card__icon[data-tone='blue'] {
		background: rgb(59 130 246 / 0.1);
		color: #60a5fa;
	}

	.stat-card__icon[data-tone='orange'] {
		background: rgb(249 115 22 / 0.1);
		color: #fb923c;
	}

	.stat-card__icon[data-tone='purple'] {
		background: rgb(168 85 247 / 0.1);
		color: #c084fc;
	}

	.stat-card__value {
		margin: 0;
		color: #ffffff;
		font-size: 1.875rem;
		font-weight: 700;
		line-height: 2.25rem;
	}

	.stat-card__note {
		margin: 0.5rem 0 0;
		color: #94a3b8;
		font-size: 0.875rem;
		line-height: 1.25rem;
	}

	.stat-card__note--positive {
		color: #34d399;
	}

	.alert-card {
		border: 1px solid;
		border-radius: 0.75rem;
		margin-bottom: 1.5rem;
		padding: 1.5rem;
	}

	.alert-card[data-tone='red'] {
		border-color: rgb(239 68 68 / 0.2);
		background: rgb(239 68 68 / 0.1);
		color: #f87171;
	}

	.alert-card[data-tone='blue'] {
		border-color: rgb(59 130 246 / 0.2);
		background: rgb(59 130 246 / 0.1);
		color: #60a5fa;
	}

	.alert-card__title {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		margin: 0 0 0.75rem;
		color: inherit;
		font-size: 1rem;
		font-weight: 600;
		line-height: 1.5rem;
	}

	.alert-card__list {
		display: grid;
		gap: 0.5rem;
	}

	.alert-card__row {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		color: #cbd5e1;
		font-size: 0.875rem;
		line-height: 1.25rem;
	}

	.alert-card__date {
		color: #60a5fa;
	}

	.filters-panel {
		margin-bottom: 1.5rem;
	}

	.filters-grid {
		display: grid;
		grid-template-columns: 1fr;
		gap: 1rem;
	}

	.form-field,
	.modal-field {
		display: grid;
		gap: 0.5rem;
	}

	.form-label {
		color: #94a3b8;
		font-size: 0.875rem;
		font-weight: 500;
		line-height: 1.25rem;
	}

	.form-control {
		width: 100%;
		border: 1px solid #475569;
		border-radius: 0.5rem;
		background: #0f172a;
		color: #ffffff;
		font: inherit;
		padding: 0.5rem 1rem;
	}

	.form-control::placeholder {
		color: #64748b;
	}

	.form-control:focus {
		border-color: #10b981;
		outline: none;
	}

	.form-control--textarea {
		min-height: 7rem;
		border-radius: 0.75rem;
		padding-block: 0.75rem;
		resize: vertical;
	}

	.form-control--warning:focus {
		border-color: #f97316;
	}

	.form-control--danger:focus {
		border-color: #ef4444;
	}

	.table-card {
		overflow: hidden;
		border-radius: 0.75rem;
	}

	.table-scroll {
		overflow-x: auto;
	}

	.subscriptions-table {
		width: 100%;
		border-collapse: collapse;
	}

	.subscriptions-table thead {
		background: rgb(15 23 42 / 0.5);
	}

	.subscriptions-table th {
		padding: 1rem 1.5rem;
		color: #94a3b8;
		font-size: 0.75rem;
		font-weight: 500;
		letter-spacing: 0.05em;
		line-height: 1rem;
		text-align: left;
		text-transform: uppercase;
		white-space: nowrap;
	}

	.subscriptions-table td {
		padding: 1rem 1.5rem;
		vertical-align: middle;
	}

	.subscriptions-table tbody tr + tr {
		border-top: 1px solid #334155;
	}

	.subscription-row {
		cursor: pointer;
		transition: background 150ms ease;
	}

	.subscription-row:hover {
		background: rgb(51 65 85 / 0.3);
	}

	.table-state {
		color: #94a3b8;
		padding-block: 3rem;
		text-align: center;
	}

	.table-state__content {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 0.75rem;
	}

	.table-spinner {
		width: 1.25rem;
		height: 1.25rem;
		border: 2px solid #10b981;
		border-top-color: transparent;
		border-radius: 999px;
		animation: spin 700ms linear infinite;
	}

	.table-primary {
		color: #ffffff;
		font-weight: 500;
	}

	.table-secondary {
		color: #cbd5e1;
	}

	.table-muted {
		color: #94a3b8;
		font-size: 0.875rem;
		line-height: 1.25rem;
	}

	.status-badge {
		display: inline-flex;
		align-items: center;
		border: 1px solid;
		border-radius: 999px;
		font-size: 0.75rem;
		font-weight: 500;
		line-height: 1rem;
		padding: 0.25rem 0.75rem;
	}

	.status-badge[data-status='active'] {
		border-color: rgb(16 185 129 / 0.2);
		background: rgb(16 185 129 / 0.1);
		color: #34d399;
	}

	.status-badge[data-status='pending'] {
		border-color: rgb(234 179 8 / 0.2);
		background: rgb(234 179 8 / 0.1);
		color: #facc15;
	}

	.status-badge[data-status='on-hold'] {
		border-color: rgb(249 115 22 / 0.2);
		background: rgb(249 115 22 / 0.1);
		color: #fb923c;
	}

	.status-badge[data-status='cancelled'] {
		border-color: rgb(239 68 68 / 0.2);
		background: rgb(239 68 68 / 0.1);
		color: #f87171;
	}

	.status-badge[data-status='expired'] {
		border-color: rgb(100 116 139 / 0.2);
		background: rgb(100 116 139 / 0.1);
		color: #94a3b8;
	}

	.status-badge[data-status='pending-cancel'] {
		border-color: rgb(236 72 153 / 0.2);
		background: rgb(236 72 153 / 0.1);
		color: #f472b6;
	}

	.row-actions {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.link-action {
		border: 0;
		background: transparent;
		cursor: pointer;
		font: inherit;
		font-size: 0.875rem;
		font-weight: 500;
		line-height: 1.25rem;
		padding: 0;
		transition: color 150ms ease;
	}

	.link-action--info {
		color: #60a5fa;
	}

	.link-action--info:hover {
		color: #93c5fd;
	}

	.link-action--success {
		color: #34d399;
	}

	.link-action--success:hover {
		color: #6ee7b7;
	}

	.link-action--warning {
		color: #fb923c;
	}

	.link-action--warning:hover {
		color: #fdba74;
	}

	.link-action--danger {
		color: #f87171;
	}

	.link-action--danger:hover {
		color: #fca5a5;
	}

	.modal-backdrop {
		position: fixed;
		inset: 0;
		z-index: 50;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgb(0 0 0 / 0.8);
		backdrop-filter: blur(8px);
		padding: 1rem;
	}

	.modal-card {
		width: 100%;
		max-width: 28rem;
		border: 1px solid #334155;
		border-radius: 1rem;
		background: #1e293b;
		padding: 2rem;
	}

	.modal-card h3 {
		margin: 0 0 1rem;
		color: #ffffff;
		font-size: 1.5rem;
		font-weight: 700;
		line-height: 2rem;
	}

	.modal-copy {
		margin: 0 0 1.5rem;
		color: #94a3b8;
	}

	.modal-field {
		margin-bottom: 1.5rem;
	}

	.required {
		color: #f87171;
	}

	.checkbox-field {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		color: #cbd5e1;
		cursor: pointer;
		font-size: 0.875rem;
		line-height: 1.25rem;
	}

	.checkbox-input {
		width: 1rem;
		height: 1rem;
		accent-color: #ef4444;
	}

	.modal-actions {
		display: flex;
		gap: 0.75rem;
	}

	.modal-button {
		flex: 1 1 0;
		border: 0;
		border-radius: 0.75rem;
		color: #ffffff;
		cursor: pointer;
		font: inherit;
		font-weight: 600;
		padding: 0.75rem 1.5rem;
		transition:
			background 150ms ease,
			opacity 150ms ease;
	}

	.modal-button:disabled {
		cursor: not-allowed;
		opacity: 0.5;
	}

	.modal-button--secondary {
		background: #334155;
	}

	.modal-button--secondary:hover:not(:disabled) {
		background: #475569;
	}

	.modal-button--warning {
		background: #f97316;
	}

	.modal-button--warning:hover:not(:disabled) {
		background: #ea580c;
	}

	.modal-button--danger {
		background: #ef4444;
	}

	.modal-button--danger:hover:not(:disabled) {
		background: #dc2626;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	/* Custom scrollbar for table */
	.table-scroll::-webkit-scrollbar {
		height: 8px;
	}

	.table-scroll::-webkit-scrollbar-track {
		background: rgb(15 23 42);
		border-radius: 4px;
	}

	.table-scroll::-webkit-scrollbar-thumb {
		background: rgb(71 85 105);
		border-radius: 4px;
	}

	.table-scroll::-webkit-scrollbar-thumb:hover {
		background: rgb(100 116 139);
	}

	@media (min-width: 768px) {
		.admin-page-container {
			padding-inline: 2rem;
		}

		.stats-grid,
		.filters-grid {
			grid-template-columns: repeat(2, minmax(0, 1fr));
		}
	}

	@media (min-width: 1024px) {
		.stats-grid,
		.filters-grid {
			grid-template-columns: repeat(4, minmax(0, 1fr));
		}
	}

	@media (max-width: 640px) {
		.modal-card {
			padding: 1.5rem;
		}

		.modal-actions {
			flex-direction: column;
		}
	}
</style>

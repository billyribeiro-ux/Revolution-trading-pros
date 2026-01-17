<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
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
	import { connections, isPaymentConnected } from '$lib/stores/connections.svelte';
	import ServiceConnectionStatus from '$lib/components/admin/ServiceConnectionStatus.svelte';
	import SubscriptionDetailDrawer from '$lib/components/admin/SubscriptionDetailDrawer.svelte';
	import SubscriptionFormModal from '$lib/components/admin/SubscriptionFormModal.svelte';

	// State
	let connectionLoading = $state(true);
	let subscriptions = $state<Subscription[]>([]);
	let stats = $state<SubscriptionStats | null>(null);
	let upcomingRenewals = $state<Subscription[]>([]);
	let failedPayments = $state<any[]>([]);
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

	onMount(async () => {
		// Load connection status first
		await connections.load();
		connectionLoading = false;

		// Only load data if payment is connected
		if ($isPaymentConnected) {
			await loadData();
		} else {
			loading = false;
		}
	});

	onDestroy(() => {
		if (searchDebounceTimer) {
			clearTimeout(searchDebounceTimer);
		}
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
				getSubscriptions({
					...(statusFilter !== 'all' && { status: [statusFilter] }),
					...(intervalFilter !== 'all' && { interval: [intervalFilter] }),
					...(searchQuery && { searchQuery })
				}, true), // isAdmin = true for admin page
				getSubscriptionStats(),
				getUpcomingRenewals(),
				getFailedPayments()
			]);

			// Extract results with fallbacks for failed requests
			const [subsResult, statsResult, renewalsResult, failedResult] = results;

			subscriptions = subsResult.status === 'fulfilled' ? subsResult.value : [];
			stats = statsResult.status === 'fulfilled' ? statsResult.value : {
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
				console.error('Subscriptions load error:', subsResult.reason);
			}
		} catch (err) {
			error = 'Failed to load subscription data';
			console.error(err);
		} finally {
			loading = false;
		}
	}

	// Filter and sort subscriptions
	let filteredSubscriptions = $derived(subscriptions
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
		}));

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
			console.error('Failed to pause subscription:', err);
		}
	}

	async function handleResume(subscription: Subscription) {
		try {
			await resumeSubscription(subscription.id);
			await loadData();
		} catch (err) {
			console.error('Failed to resume subscription:', err);
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
			console.error('Failed to cancel subscription:', err);
		}
	}

	async function handleReactivate(subscription: Subscription) {
		try {
			await reactivateSubscription(subscription.id);
			await loadData();
		} catch (err) {
			console.error('Failed to reactivate subscription:', err);
		}
	}

	async function handleRetryPayment(subscriptionId: string, paymentId: string) {
		try {
			await retryPayment(subscriptionId, paymentId);
			await loadData();
		} catch (err) {
			console.error('Failed to retry payment:', err);
		}
	}

	function getStatusColor(status: SubscriptionStatus): string {
		switch (status) {
			case 'active':
				return 'bg-emerald-500/10 text-emerald-400 border-emerald-500/20';
			case 'pending':
				return 'bg-yellow-500/10 text-yellow-400 border-yellow-500/20';
			case 'on-hold':
				return 'bg-orange-500/10 text-orange-400 border-orange-500/20';
			case 'cancelled':
				return 'bg-red-500/10 text-red-400 border-red-500/20';
			case 'expired':
				return 'bg-slate-500/10 text-slate-400 border-slate-500/20';
			case 'pending-cancel':
				return 'bg-pink-500/10 text-pink-400 border-pink-500/20';
			default:
				return 'bg-slate-500/10 text-slate-400 border-slate-500/20';
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

<div class="min-h-screen bg-gradient-to-br from-slate-950 via-slate-900 to-slate-950 py-12 px-4">
	<div class="max-w-7xl mx-auto">
		<!-- Header -->
		<div class="flex items-start justify-between mb-8">
			<div>
				<h1 class="text-4xl font-bold text-white mb-2">Subscription Management</h1>
				<p class="text-slate-400">Monitor and manage all customer subscriptions</p>
			</div>
			{#if $isPaymentConnected}
				<button
					onclick={openCreateModal}
					class="flex items-center gap-2 px-4 py-2.5 bg-emerald-500 hover:bg-emerald-600 text-white font-semibold rounded-xl transition-colors"
				>
					<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
					</svg>
					Create Subscription
				</button>
			{/if}
		</div>

		<!-- Connection Check -->
		{#if connectionLoading}
			<div class="flex items-center justify-center py-20">
				<div class="relative">
					<div class="w-12 h-12 border-4 border-purple-500/20 rounded-full"></div>
					<div class="absolute top-0 left-0 w-12 h-12 border-4 border-purple-500 rounded-full animate-spin border-t-transparent"></div>
				</div>
			</div>
		{:else if !$isPaymentConnected}
			<ServiceConnectionStatus feature="payment" variant="card" showFeatures={true} />
		{:else}
			{#if error}
				<div class="bg-red-500/10 border border-red-500/20 rounded-xl p-4 mb-6">
					<p class="text-red-400">{error}</p>
				</div>
			{/if}

			<!-- Stats Overview -->
			{#if stats}
			<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
				<div class="bg-slate-800/50 backdrop-blur-sm border border-slate-700 rounded-xl p-6">
					<div class="flex items-center justify-between mb-2">
						<h3 class="text-slate-400 text-sm font-medium">Active Subscriptions</h3>
						<div class="w-10 h-10 bg-emerald-500/10 rounded-lg flex items-center justify-center">
							<svg
								class="w-5 h-5 text-emerald-400"
								fill="none"
								stroke="currentColor"
								viewBox="0 0 24 24"
							>
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
								/>
							</svg>
						</div>
					</div>
					<p class="text-3xl font-bold text-white">{stats.totalActive}</p>
					<p class="text-sm text-emerald-400 mt-2">
						{stats.newThisMonth > 0 ? '+' : ''}{stats.newThisMonth} this month
					</p>
				</div>

				<div class="bg-slate-800/50 backdrop-blur-sm border border-slate-700 rounded-xl p-6">
					<div class="flex items-center justify-between mb-2">
						<h3 class="text-slate-400 text-sm font-medium">Monthly Revenue</h3>
						<div class="w-10 h-10 bg-blue-500/10 rounded-lg flex items-center justify-center">
							<svg
								class="w-5 h-5 text-blue-400"
								fill="none"
								stroke="currentColor"
								viewBox="0 0 24 24"
							>
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
								/>
							</svg>
						</div>
					</div>
					<p class="text-3xl font-bold text-white">
						{formatCurrency(stats.monthlyRecurringRevenue)}
					</p>
					<p class="text-sm text-slate-400 mt-2">MRR</p>
				</div>

				<div class="bg-slate-800/50 backdrop-blur-sm border border-slate-700 rounded-xl p-6">
					<div class="flex items-center justify-between mb-2">
						<h3 class="text-slate-400 text-sm font-medium">Churn Rate</h3>
						<div class="w-10 h-10 bg-orange-500/10 rounded-lg flex items-center justify-center">
							<svg
								class="w-5 h-5 text-orange-400"
								fill="none"
								stroke="currentColor"
								viewBox="0 0 24 24"
							>
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M13 17h8m0 0V9m0 8l-8-8-4 4-6-6"
								/>
							</svg>
						</div>
					</div>
					<p class="text-3xl font-bold text-white">{stats.churnRate.toFixed(1)}%</p>
					<p class="text-sm text-slate-400 mt-2">Last 30 days</p>
				</div>

				<div class="bg-slate-800/50 backdrop-blur-sm border border-slate-700 rounded-xl p-6">
					<div class="flex items-center justify-between mb-2">
						<h3 class="text-slate-400 text-sm font-medium">Avg Lifetime Value</h3>
						<div class="w-10 h-10 bg-purple-500/10 rounded-lg flex items-center justify-center">
							<svg
								class="w-5 h-5 text-purple-400"
								fill="none"
								stroke="currentColor"
								viewBox="0 0 24 24"
							>
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M7 12l3-3 3 3 4-4M8 21l4-4 4 4M3 4h18M4 4h16v12a1 1 0 01-1 1H5a1 1 0 01-1-1V4z"
								/>
							</svg>
						</div>
					</div>
					<p class="text-3xl font-bold text-white">{formatCurrency(stats.averageLifetimeValue)}</p>
					<p class="text-sm text-slate-400 mt-2">Per customer</p>
				</div>
			</div>
		{/if}

		<!-- Alerts -->
		{#if failedPayments.length > 0}
			<div class="bg-red-500/10 border border-red-500/20 rounded-xl p-6 mb-6">
				<h3 class="text-red-400 font-semibold mb-3 flex items-center gap-2">
					<svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
						<path
							fill-rule="evenodd"
							d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z"
							clip-rule="evenodd"
						/>
					</svg>
					{failedPayments.length} Failed Payment{failedPayments.length > 1 ? 's' : ''}
				</h3>
				<div class="space-y-2">
					{#each failedPayments.slice(0, 3) as payment}
						<div class="flex items-center justify-between text-sm">
							<span class="text-slate-300">{payment.subscriptionId}</span>
							<button
								onclick={() => handleRetryPayment(payment.subscriptionId, payment.id)}
								class="text-emerald-400 hover:text-emerald-300 font-medium"
							>
								Retry Payment
							</button>
						</div>
					{/each}
				</div>
			</div>
		{/if}

		{#if upcomingRenewals.length > 0}
			<div class="bg-blue-500/10 border border-blue-500/20 rounded-xl p-6 mb-6">
				<h3 class="text-blue-400 font-semibold mb-3 flex items-center gap-2">
					<svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
						<path
							fill-rule="evenodd"
							d="M10 18a8 8 0 100-16 8 8 0 000 16zm1-12a1 1 0 10-2 0v4a1 1 0 00.293.707l2.828 2.829a1 1 0 101.415-1.415L11 9.586V6z"
							clip-rule="evenodd"
						/>
					</svg>
					{upcomingRenewals.length} Renewal{upcomingRenewals.length > 1 ? 's' : ''} in Next 7 Days
				</h3>
				<div class="space-y-2">
					{#each upcomingRenewals.slice(0, 3) as renewal}
						<div class="flex items-center justify-between text-sm">
							<span class="text-slate-300">{renewal.productName}</span>
							<span class="text-blue-400">{formatDate(renewal.nextPaymentDate)}</span>
						</div>
					{/each}
				</div>
			</div>
		{/if}

		<!-- Filters -->
		<div class="bg-slate-800/50 backdrop-blur-sm border border-slate-700 rounded-xl p-6 mb-6">
			<div class="grid grid-cols-1 md:grid-cols-4 gap-4">
				<div>
					<label for="search" class="block text-sm font-medium text-slate-400 mb-2">Search</label>
					<input
						type="text"
						id="search"
						bind:value={searchQuery}
						oninput={handleSearchInput}
						placeholder="Search subscriptions..."
						class="w-full px-4 py-2 bg-slate-900 border border-slate-600 rounded-lg text-white placeholder-slate-500 focus:outline-none focus:border-emerald-500"
					/>
				</div>

				<div>
					<label for="status" class="block text-sm font-medium text-slate-400 mb-2">Status</label>
					<select
						id="status"
						bind:value={statusFilter}
						onchange={loadData}
						class="w-full px-4 py-2 bg-slate-900 border border-slate-600 rounded-lg text-white focus:outline-none focus:border-emerald-500"
					>
						<option value="all">All Statuses</option>
						<option value="active">Active</option>
						<option value="pending">Pending</option>
						<option value="on-hold">On Hold</option>
						<option value="cancelled">Cancelled</option>
						<option value="expired">Expired</option>
						<option value="pending-cancel">Pending Cancel</option>
					</select>
				</div>

				<div>
					<label for="interval" class="block text-sm font-medium text-slate-400 mb-2"
						>Interval</label
					>
					<select
						id="interval"
						bind:value={intervalFilter}
						onchange={loadData}
						class="w-full px-4 py-2 bg-slate-900 border border-slate-600 rounded-lg text-white focus:outline-none focus:border-emerald-500"
					>
						<option value="all">All Intervals</option>
						<option value="monthly">Monthly</option>
						<option value="quarterly">Quarterly</option>
						<option value="yearly">Yearly</option>
					</select>
				</div>

				<div>
					<label for="sort" class="block text-sm font-medium text-slate-400 mb-2">Sort By</label>
					<select
						id="sort"
						bind:value={sortBy}
						class="w-full px-4 py-2 bg-slate-900 border border-slate-600 rounded-lg text-white focus:outline-none focus:border-emerald-500"
					>
						<option value="date">Date</option>
						<option value="price">Price</option>
						<option value="status">Status</option>
					</select>
				</div>
			</div>
		</div>

		<!-- Subscriptions Table -->
		<div
			class="bg-slate-800/50 backdrop-blur-sm border border-slate-700 rounded-xl overflow-hidden"
		>
			<div class="overflow-x-auto">
				<table class="w-full">
					<thead class="bg-slate-900/50">
						<tr>
							<th
								class="px-6 py-4 text-left text-xs font-medium text-slate-400 uppercase tracking-wider"
								>Product</th
							>
							<th
								class="px-6 py-4 text-left text-xs font-medium text-slate-400 uppercase tracking-wider"
								>Customer</th
							>
							<th
								class="px-6 py-4 text-left text-xs font-medium text-slate-400 uppercase tracking-wider"
								>Status</th
							>
							<th
								class="px-6 py-4 text-left text-xs font-medium text-slate-400 uppercase tracking-wider"
								>Interval</th
							>
							<th
								class="px-6 py-4 text-left text-xs font-medium text-slate-400 uppercase tracking-wider"
								>Price</th
							>
							<th
								class="px-6 py-4 text-left text-xs font-medium text-slate-400 uppercase tracking-wider"
								>Next Payment</th
							>
							<th
								class="px-6 py-4 text-left text-xs font-medium text-slate-400 uppercase tracking-wider"
								>Actions</th
							>
						</tr>
					</thead>
					<tbody class="divide-y divide-slate-700">
						{#if loading}
							<tr>
								<td colspan="7" class="px-6 py-12 text-center text-slate-400">
									<div class="flex items-center justify-center gap-3">
										<div
											class="w-5 h-5 border-2 border-emerald-500 border-t-transparent rounded-full animate-spin"
										></div>
										Loading subscriptions...
									</div>
								</td>
							</tr>
						{:else if filteredSubscriptions.length === 0}
							<tr>
								<td colspan="7" class="px-6 py-12 text-center text-slate-400">
									No subscriptions found
								</td>
							</tr>
						{:else}
							{#each filteredSubscriptions as subscription}
								<tr 
									class="hover:bg-slate-700/30 transition-colors cursor-pointer"
									onclick={() => openSubscriptionDetail(subscription)}
								>
									<td class="px-6 py-4">
										<div class="text-white font-medium">{subscription.productName}</div>
										<div class="text-sm text-slate-400">{subscription.id.slice(0, 8)}...</div>
									</td>
									<td class="px-6 py-4">
										<div class="text-slate-300">{subscription.userId.slice(0, 8)}...</div>
									</td>
									<td class="px-6 py-4">
										<span
											class="inline-flex items-center px-3 py-1 rounded-full text-xs font-medium border {getStatusColor(
												subscription.status
											)}"
										>
											{subscription.status}
										</span>
									</td>
									<td class="px-6 py-4 text-slate-300">
										{getIntervalLabel(subscription.interval)}
									</td>
									<td class="px-6 py-4 text-white font-medium">
										{formatCurrency(subscription.price)}
									</td>
									<td class="px-6 py-4 text-slate-300">
										{formatDate(subscription.nextPaymentDate)}
									</td>
									<td class="px-6 py-4">
										<div class="flex items-center gap-2" onclick={(e) => e.stopPropagation()}>
											<button
												onclick={() => openSubscriptionDetail(subscription)}
												class="text-blue-400 hover:text-blue-300 text-sm font-medium"
												title="View Details"
											>
												View
											</button>
											{#if subscription.status === 'active'}
												<button
													onclick={() => handlePause(subscription)}
													class="text-orange-400 hover:text-orange-300 text-sm font-medium"
													title="Pause"
												>
													Pause
												</button>
												<button
													onclick={() => handleCancel(subscription)}
													class="text-red-400 hover:text-red-300 text-sm font-medium"
													title="Cancel"
												>
													Cancel
												</button>
											{:else if subscription.status === 'on-hold'}
												<button
													onclick={() => handleResume(subscription)}
													class="text-emerald-400 hover:text-emerald-300 text-sm font-medium"
													title="Resume"
												>
													Resume
												</button>
												<button
													onclick={() => handleCancel(subscription)}
													class="text-red-400 hover:text-red-300 text-sm font-medium"
													title="Cancel"
												>
													Cancel
												</button>
											{:else if subscription.status === 'cancelled'}
												<button
													onclick={() => handleReactivate(subscription)}
													class="text-emerald-400 hover:text-emerald-300 text-sm font-medium"
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
	<div class="fixed inset-0 bg-black/80 backdrop-blur-sm flex items-center justify-center z-50 p-4">
		<div class="bg-slate-800 border border-slate-700 rounded-2xl p-8 max-w-md w-full">
			<h3 class="text-2xl font-bold text-white mb-4">Pause Subscription</h3>
			<p class="text-slate-400 mb-6">
				Are you sure you want to pause {selectedSubscription.productName}?
			</p>

			<div class="mb-6">
				<label for="pauseReason" class="block text-sm font-medium text-slate-400 mb-2">
					Reason for pausing <span class="text-red-400">*</span>
				</label>
				<textarea
					id="pauseReason"
					bind:value={pauseReason}
					rows="3"
					placeholder="Enter reason..."
					class="w-full px-4 py-3 bg-slate-900 border border-slate-600 rounded-xl text-white placeholder-slate-500 focus:outline-none focus:border-orange-500"
				></textarea>
			</div>

			<div class="flex gap-3">
				<button
					onclick={() => {
						showPauseModal = false;
						pauseReason = '';
						selectedSubscription = null;
					}}
					class="flex-1 px-6 py-3 bg-slate-700 hover:bg-slate-600 text-white font-semibold rounded-xl transition-colors"
				>
					Cancel
				</button>
				<button
					onclick={confirmPause}
					disabled={!pauseReason.trim()}
					class="flex-1 px-6 py-3 bg-orange-500 hover:bg-orange-600 text-white font-semibold rounded-xl transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
				>
					Pause Subscription
				</button>
			</div>
		</div>
	</div>
{/if}

<!-- Cancel Modal -->
{#if showCancelModal && selectedSubscription}
	<div class="fixed inset-0 bg-black/80 backdrop-blur-sm flex items-center justify-center z-50 p-4">
		<div class="bg-slate-800 border border-slate-700 rounded-2xl p-8 max-w-md w-full">
			<h3 class="text-2xl font-bold text-white mb-4">Cancel Subscription</h3>
			<p class="text-slate-400 mb-6">
				Are you sure you want to cancel {selectedSubscription.productName}?
			</p>

			<div class="mb-6">
				<label for="cancelReason" class="block text-sm font-medium text-slate-400 mb-2">
					Cancellation reason <span class="text-red-400">*</span>
				</label>
				<textarea
					id="cancelReason"
					bind:value={cancelReason}
					rows="3"
					placeholder="Enter reason..."
					class="w-full px-4 py-3 bg-slate-900 border border-slate-600 rounded-xl text-white placeholder-slate-500 focus:outline-none focus:border-red-500"
				></textarea>
			</div>

			<div class="mb-6">
				<label class="flex items-center gap-3 cursor-pointer">
					<input
						type="checkbox"
						bind:checked={cancelImmediate}
						class="w-4 h-4 bg-slate-900 border-slate-600 rounded text-red-500 focus:ring-red-500"
					/>
					<span class="text-slate-300 text-sm"
						>Cancel immediately (don't wait until period end)</span
					>
				</label>
			</div>

			<div class="flex gap-3">
				<button
					onclick={() => {
						showCancelModal = false;
						cancelReason = '';
						cancelImmediate = false;
						selectedSubscription = null;
					}}
					class="flex-1 px-6 py-3 bg-slate-700 hover:bg-slate-600 text-white font-semibold rounded-xl transition-colors"
				>
					Go Back
				</button>
				<button
					onclick={confirmCancel}
					disabled={!cancelReason.trim()}
					class="flex-1 px-6 py-3 bg-red-500 hover:bg-red-600 text-white font-semibold rounded-xl transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
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
	onClose={() => { showDetailDrawer = false; selectedSubscription = null; }}
	onEdit={openEditModal}
	onRefresh={loadData}
/>

<!-- Subscription Form Modal -->
<SubscriptionFormModal
	isOpen={showFormModal}
	mode={formModalMode}
	subscription={selectedSubscription}
	onClose={() => { showFormModal = false; selectedSubscription = null; }}
	onSaved={handleSubscriptionSaved}
/>

<style>
	/* Custom scrollbar for table */
	.overflow-x-auto::-webkit-scrollbar {
		height: 8px;
	}

	.overflow-x-auto::-webkit-scrollbar-track {
		background: rgb(15 23 42);
		border-radius: 4px;
	}

	.overflow-x-auto::-webkit-scrollbar-thumb {
		background: rgb(71 85 105);
		border-radius: 4px;
	}

	.overflow-x-auto::-webkit-scrollbar-thumb:hover {
		background: rgb(100 116 139);
	}
</style>

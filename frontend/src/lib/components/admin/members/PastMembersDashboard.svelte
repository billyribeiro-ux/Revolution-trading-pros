<script lang="ts">
	/**
	 * PastMembersDashboard - Beautiful admin dashboard for managing churned members
	 * Features: Stats, member list, win-back campaigns, feedback surveys
	 *
	 * @version 1.0.0
	 * @author Revolution Trading Pros
	 */
	import {
		IconUsers,
		IconMail,
		IconTrendingDown,
		IconTrendingUp,
		IconRefresh,
		IconSend,
		IconClipboardList,
		IconFilter,
		IconSearch,
		IconChevronLeft,
		IconChevronRight,
		IconCheck,
		IconX,
		IconLoader
	} from '@tabler/icons-svelte';
	import { onMount } from 'svelte';
	import { addToast } from '$lib/stores/toast';

	interface PastMember {
		id: number;
		name: string;
		email: string;
		last_membership: {
			plan_name: string;
			status: string;
			started_at: string;
			expired_at: string;
			days_since_expired: number;
			cancellation_reason: string | null;
		} | null;
		total_memberships: number;
		created_at: string;
	}

	interface Stats {
		total_past_members: number;
		expired_last_30_days: number;
		expired_last_60_days: number;
		expired_last_90_days: number;
		churn_by_plan: Record<string, number>;
		reactivated_last_6_months: number;
		win_back_rate: number;
	}

	let stats: Stats | null = null;
	let members: PastMember[] = [];
	let loading = true;
	let selectedMembers: number[] = [];
	let searchQuery = '';
	let currentPage = 1;
	let totalPages = 1;
	let perPage = 20;

	// Modal states
	let showWinBackModal = false;
	let showSurveyModal = false;
	let winBackTarget: PastMember | null = null;
	let surveyTarget: PastMember | null = null;
	let bulkAction = false;

	// Win-back form
	let offerCode = '';
	let discountPercent = 20;
	let discountMonths = 3;
	let expiresInDays = 7;
	let sendingEmail = false;

	// Survey form
	let incentiveDescription = '';

	// Filters
	let filterDays = '';
	let filterPlan = '';

	async function fetchStats() {
		try {
			const response = await fetch('/api/admin/past-members/stats', {
				credentials: 'include'
			});
			if (response.ok) {
				stats = await response.json();
			}
		} catch (error) {
			console.error('Failed to fetch stats:', error);
		}
	}

	async function fetchMembers() {
		loading = true;
		try {
			const params = new URLSearchParams({
				page: currentPage.toString(),
				per_page: perPage.toString()
			});

			if (searchQuery) params.append('search', searchQuery);
			if (filterDays) params.append('days_since_expired', filterDays);
			if (filterPlan) params.append('plan_id', filterPlan);

			const response = await fetch(`/api/admin/past-members?${params}`, {
				credentials: 'include'
			});

			if (response.ok) {
				const data = await response.json();
				members = data.data;
				totalPages = data.last_page;
			}
		} catch (error) {
			console.error('Failed to fetch members:', error);
		} finally {
			loading = false;
		}
	}

	async function sendWinBackEmail() {
		if (!winBackTarget && selectedMembers.length === 0) return;
		sendingEmail = true;

		try {
			const url = bulkAction
				? '/api/admin/past-members/bulk-win-back'
				: `/api/admin/past-members/${winBackTarget?.id}/win-back`;

			const body = bulkAction
				? {
						user_ids: selectedMembers,
						offer_code: offerCode || undefined,
						discount_percent: discountPercent,
						discount_months: discountMonths,
						expires_in_days: expiresInDays
					}
				: {
						offer_code: offerCode || undefined,
						discount_percent: discountPercent,
						discount_months: discountMonths,
						expires_in_days: expiresInDays
					};

			const response = await fetch(url, {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				credentials: 'include',
				body: JSON.stringify(body)
			});

			if (response.ok) {
				const result = await response.json();
				addToast(result.message, 'success');
				showWinBackModal = false;
				resetWinBackForm();
				selectedMembers = [];
			} else {
				addToast('Failed to send win-back email', 'error');
			}
		} catch (error) {
			addToast('Error sending win-back email', 'error');
		} finally {
			sendingEmail = false;
		}
	}

	async function sendSurvey() {
		if (!surveyTarget && selectedMembers.length === 0) return;
		sendingEmail = true;

		try {
			const url = bulkAction
				? '/api/admin/past-members/bulk-survey'
				: `/api/admin/past-members/${surveyTarget?.id}/survey`;

			const body = bulkAction
				? {
						user_ids: selectedMembers,
						incentive_description: incentiveDescription || undefined
					}
				: {
						incentive_description: incentiveDescription || undefined
					};

			const response = await fetch(url, {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				credentials: 'include',
				body: JSON.stringify(body)
			});

			if (response.ok) {
				const result = await response.json();
				addToast(result.message, 'success');
				showSurveyModal = false;
				incentiveDescription = '';
				selectedMembers = [];
			} else {
				addToast('Failed to send survey', 'error');
			}
		} catch (error) {
			addToast('Error sending survey', 'error');
		} finally {
			sendingEmail = false;
		}
	}

	function resetWinBackForm() {
		offerCode = '';
		discountPercent = 20;
		discountMonths = 3;
		expiresInDays = 7;
	}

	function openWinBackModal(member?: PastMember) {
		if (member) {
			winBackTarget = member;
			bulkAction = false;
		} else {
			winBackTarget = null;
			bulkAction = true;
		}
		showWinBackModal = true;
	}

	function openSurveyModal(member?: PastMember) {
		if (member) {
			surveyTarget = member;
			bulkAction = false;
		} else {
			surveyTarget = null;
			bulkAction = true;
		}
		showSurveyModal = true;
	}

	function toggleSelectAll() {
		if (selectedMembers.length === members.length) {
			selectedMembers = [];
		} else {
			selectedMembers = members.map((m) => m.id);
		}
	}

	function toggleSelect(id: number) {
		if (selectedMembers.includes(id)) {
			selectedMembers = selectedMembers.filter((m) => m !== id);
		} else {
			selectedMembers = [...selectedMembers, id];
		}
	}

	function formatDate(date: string) {
		return new Date(date).toLocaleDateString('en-US', {
			month: 'short',
			day: 'numeric',
			year: 'numeric'
		});
	}

	function getDaysBadgeColor(days: number): string {
		if (days <= 30) return 'bg-green-100 text-green-800';
		if (days <= 60) return 'bg-yellow-100 text-yellow-800';
		return 'bg-red-100 text-red-800';
	}

	onMount(() => {
		fetchStats();
		fetchMembers();
	});

	$: if (searchQuery !== undefined || filterDays || filterPlan) {
		currentPage = 1;
	}
</script>

<div class="min-h-screen bg-gray-50 p-6">
	<!-- Header -->
	<div class="mb-8">
		<h1 class="text-3xl font-bold text-gray-900">Past Members</h1>
		<p class="mt-2 text-gray-600">
			Manage churned members and run win-back campaigns to re-engage them.
		</p>
	</div>

	<!-- Stats Cards -->
	{#if stats}
		<div class="mb-8 grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-4">
			<!-- Total Past Members -->
			<div class="rounded-xl bg-white p-6 shadow-sm">
				<div class="flex items-center justify-between">
					<div>
						<p class="text-sm font-medium text-gray-500">Total Past Members</p>
						<p class="mt-2 text-3xl font-bold text-gray-900">{stats.total_past_members}</p>
					</div>
					<div class="rounded-full bg-blue-100 p-3">
						<IconUsers class="h-6 w-6 text-blue-600" />
					</div>
				</div>
			</div>

			<!-- Last 30 Days -->
			<div class="rounded-xl bg-white p-6 shadow-sm">
				<div class="flex items-center justify-between">
					<div>
						<p class="text-sm font-medium text-gray-500">Left in Last 30 Days</p>
						<p class="mt-2 text-3xl font-bold text-red-600">{stats.expired_last_30_days}</p>
					</div>
					<div class="rounded-full bg-red-100 p-3">
						<IconTrendingDown class="h-6 w-6 text-red-600" />
					</div>
				</div>
			</div>

			<!-- Win-Back Rate -->
			<div class="rounded-xl bg-white p-6 shadow-sm">
				<div class="flex items-center justify-between">
					<div>
						<p class="text-sm font-medium text-gray-500">Win-Back Rate</p>
						<p class="mt-2 text-3xl font-bold text-green-600">{stats.win_back_rate}%</p>
					</div>
					<div class="rounded-full bg-green-100 p-3">
						<IconTrendingUp class="h-6 w-6 text-green-600" />
					</div>
				</div>
			</div>

			<!-- Reactivated -->
			<div class="rounded-xl bg-white p-6 shadow-sm">
				<div class="flex items-center justify-between">
					<div>
						<p class="text-sm font-medium text-gray-500">Reactivated (6 mo)</p>
						<p class="mt-2 text-3xl font-bold text-purple-600">
							{stats.reactivated_last_6_months}
						</p>
					</div>
					<div class="rounded-full bg-purple-100 p-3">
						<IconRefresh class="h-6 w-6 text-purple-600" />
					</div>
				</div>
			</div>
		</div>
	{/if}

	<!-- Actions Bar -->
	<div class="mb-6 flex flex-wrap items-center justify-between gap-4 rounded-xl bg-white p-4 shadow-sm">
		<div class="flex flex-wrap items-center gap-4">
			<!-- Search -->
			<div class="relative">
				<IconSearch class="absolute left-3 top-1/2 h-5 w-5 -translate-y-1/2 text-gray-400" />
				<input
					type="text"
					bind:value={searchQuery}
					on:input={() => fetchMembers()}
					placeholder="Search by name or email..."
					class="rounded-lg border border-gray-300 py-2 pl-10 pr-4 focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
				/>
			</div>

			<!-- Filter by days -->
			<select
				bind:value={filterDays}
				on:change={() => fetchMembers()}
				class="rounded-lg border border-gray-300 px-4 py-2 focus:border-blue-500 focus:outline-none"
			>
				<option value="">All time</option>
				<option value="30">Left 0-30 days ago</option>
				<option value="60">Left 30-60 days ago</option>
				<option value="90">Left 60-90 days ago</option>
			</select>
		</div>

		<!-- Bulk Actions -->
		{#if selectedMembers.length > 0}
			<div class="flex items-center gap-2">
				<span class="text-sm text-gray-600">{selectedMembers.length} selected</span>
				<button
					onclick={() => openWinBackModal()}
					class="flex items-center gap-2 rounded-lg bg-blue-600 px-4 py-2 text-white transition hover:bg-blue-700"
				>
					<IconSend class="h-4 w-4" />
					Send Win-Back
				</button>
				<button
					onclick={() => openSurveyModal()}
					class="flex items-center gap-2 rounded-lg bg-purple-600 px-4 py-2 text-white transition hover:bg-purple-700"
				>
					<IconClipboardList class="h-4 w-4" />
					Send Survey
				</button>
			</div>
		{/if}
	</div>

	<!-- Members Table -->
	<div class="overflow-hidden rounded-xl bg-white shadow-sm">
		{#if loading}
			<div class="flex items-center justify-center py-20">
				<IconLoader class="h-8 w-8 animate-spin text-blue-600" />
			</div>
		{:else if members.length === 0}
			<div class="py-20 text-center">
				<IconUsers class="mx-auto h-12 w-12 text-gray-400" />
				<p class="mt-4 text-gray-500">No past members found</p>
			</div>
		{:else}
			<div class="overflow-x-auto">
				<table class="min-w-full divide-y divide-gray-200">
					<thead class="bg-gray-50">
						<tr>
							<th class="w-12 px-6 py-3">
								<input
									type="checkbox"
									checked={selectedMembers.length === members.length}
									onchange={toggleSelectAll}
									class="h-4 w-4 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
								/>
							</th>
							<th
								class="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500"
							>
								Member
							</th>
							<th
								class="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500"
							>
								Last Plan
							</th>
							<th
								class="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500"
							>
								Days Gone
							</th>
							<th
								class="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500"
							>
								Reason
							</th>
							<th
								class="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500"
							>
								Actions
							</th>
						</tr>
					</thead>
					<tbody class="divide-y divide-gray-200 bg-white">
						{#each members as member}
							<tr class="hover:bg-gray-50">
								<td class="px-6 py-4">
									<input
										type="checkbox"
										checked={selectedMembers.includes(member.id)}
										onchange={() => toggleSelect(member.id)}
										class="h-4 w-4 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
									/>
								</td>
								<td class="px-6 py-4">
									<div class="flex items-center">
										<div
											class="flex h-10 w-10 flex-shrink-0 items-center justify-center rounded-full bg-gradient-to-br from-blue-500 to-purple-600 font-medium text-white"
										>
											{member.name.charAt(0).toUpperCase()}
										</div>
										<div class="ml-4">
											<div class="font-medium text-gray-900">{member.name}</div>
											<div class="text-sm text-gray-500">{member.email}</div>
										</div>
									</div>
								</td>
								<td class="px-6 py-4">
									<span class="inline-flex rounded-full bg-gray-100 px-3 py-1 text-sm font-medium text-gray-800">
										{member.last_membership?.plan_name ?? 'Unknown'}
									</span>
								</td>
								<td class="px-6 py-4">
									{#if member.last_membership?.days_since_expired !== undefined}
										<span
											class="inline-flex rounded-full px-3 py-1 text-sm font-medium {getDaysBadgeColor(member.last_membership.days_since_expired)}"
										>
											{member.last_membership.days_since_expired} days
										</span>
									{:else}
										<span class="text-gray-400">-</span>
									{/if}
								</td>
								<td class="max-w-xs truncate px-6 py-4 text-sm text-gray-500">
									{member.last_membership?.cancellation_reason ?? 'Not specified'}
								</td>
								<td class="px-6 py-4">
									<div class="flex items-center gap-2">
										<button
											onclick={() => openWinBackModal(member)}
											class="rounded-lg bg-blue-100 p-2 text-blue-600 transition hover:bg-blue-200"
											title="Send Win-Back Email"
										>
											<IconMail class="h-4 w-4" />
										</button>
										<button
											onclick={() => openSurveyModal(member)}
											class="rounded-lg bg-purple-100 p-2 text-purple-600 transition hover:bg-purple-200"
											title="Send Feedback Survey"
										>
											<IconClipboardList class="h-4 w-4" />
										</button>
									</div>
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>

			<!-- Pagination -->
			<div class="flex items-center justify-between border-t border-gray-200 bg-gray-50 px-6 py-3">
				<p class="text-sm text-gray-500">
					Page {currentPage} of {totalPages}
				</p>
				<div class="flex items-center gap-2">
					<button
						onclick={() => { currentPage = Math.max(1, currentPage - 1); fetchMembers(); }}
						disabled={currentPage === 1}
						class="rounded-lg border border-gray-300 p-2 transition hover:bg-gray-100 disabled:cursor-not-allowed disabled:opacity-50"
					>
						<IconChevronLeft class="h-4 w-4" />
					</button>
					<button
						onclick={() => { currentPage = Math.min(totalPages, currentPage + 1); fetchMembers(); }}
						disabled={currentPage === totalPages}
						class="rounded-lg border border-gray-300 p-2 transition hover:bg-gray-100 disabled:cursor-not-allowed disabled:opacity-50"
					>
						<IconChevronRight class="h-4 w-4" />
					</button>
				</div>
			</div>
		{/if}
	</div>
</div>

<!-- Win-Back Modal -->
{#if showWinBackModal}
	<div
		class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 p-4"
		onclick={(e) => e.target === e.currentTarget && (showWinBackModal = false)}
	>
		<div class="w-full max-w-md rounded-2xl bg-white p-6 shadow-xl">
			<div class="mb-6 flex items-center justify-between">
				<h2 class="text-xl font-bold text-gray-900">
					{bulkAction ? `Send Win-Back to ${selectedMembers.length} Members` : 'Send Win-Back Email'}
				</h2>
				<button onclick={() => (showWinBackModal = false)} class="text-gray-400 hover:text-gray-600">
					<IconX class="h-5 w-5" />
				</button>
			</div>

			{#if !bulkAction && winBackTarget}
				<div class="mb-6 flex items-center gap-3 rounded-lg bg-gray-50 p-3">
					<div
						class="flex h-10 w-10 items-center justify-center rounded-full bg-gradient-to-br from-blue-500 to-purple-600 font-medium text-white"
					>
						{winBackTarget.name.charAt(0).toUpperCase()}
					</div>
					<div>
						<p class="font-medium text-gray-900">{winBackTarget.name}</p>
						<p class="text-sm text-gray-500">{winBackTarget.email}</p>
					</div>
				</div>
			{/if}

			<div class="space-y-4">
				<div>
					<label class="block text-sm font-medium text-gray-700">Discount Percentage</label>
					<input
						type="number"
						bind:value={discountPercent}
						min="1"
						max="100"
						class="mt-1 w-full rounded-lg border border-gray-300 px-4 py-2 focus:border-blue-500 focus:outline-none"
					/>
				</div>

				<div>
					<label class="block text-sm font-medium text-gray-700">Discount Duration (months)</label>
					<input
						type="number"
						bind:value={discountMonths}
						min="1"
						max="12"
						class="mt-1 w-full rounded-lg border border-gray-300 px-4 py-2 focus:border-blue-500 focus:outline-none"
					/>
				</div>

				<div>
					<label class="block text-sm font-medium text-gray-700">Offer Expires In (days)</label>
					<input
						type="number"
						bind:value={expiresInDays}
						min="1"
						max="30"
						class="mt-1 w-full rounded-lg border border-gray-300 px-4 py-2 focus:border-blue-500 focus:outline-none"
					/>
				</div>

				<div>
					<label class="block text-sm font-medium text-gray-700">Custom Offer Code (optional)</label>
					<input
						type="text"
						bind:value={offerCode}
						placeholder="Auto-generated if empty"
						class="mt-1 w-full rounded-lg border border-gray-300 px-4 py-2 focus:border-blue-500 focus:outline-none"
					/>
				</div>
			</div>

			<div class="mt-6 flex justify-end gap-3">
				<button
					onclick={() => (showWinBackModal = false)}
					class="rounded-lg border border-gray-300 px-4 py-2 text-gray-700 transition hover:bg-gray-50"
				>
					Cancel
				</button>
				<button
					onclick={sendWinBackEmail}
					disabled={sendingEmail}
					class="flex items-center gap-2 rounded-lg bg-blue-600 px-4 py-2 text-white transition hover:bg-blue-700 disabled:opacity-50"
				>
					{#if sendingEmail}
						<IconLoader class="h-4 w-4 animate-spin" />
					{:else}
						<IconSend class="h-4 w-4" />
					{/if}
					Send Email
				</button>
			</div>
		</div>
	</div>
{/if}

<!-- Survey Modal -->
{#if showSurveyModal}
	<div
		class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 p-4"
		onclick={(e) => e.target === e.currentTarget && (showSurveyModal = false)}
	>
		<div class="w-full max-w-md rounded-2xl bg-white p-6 shadow-xl">
			<div class="mb-6 flex items-center justify-between">
				<h2 class="text-xl font-bold text-gray-900">
					{bulkAction
						? `Send Survey to ${selectedMembers.length} Members`
						: 'Send Feedback Survey'}
				</h2>
				<button onclick={() => (showSurveyModal = false)} class="text-gray-400 hover:text-gray-600">
					<IconX class="h-5 w-5" />
				</button>
			</div>

			{#if !bulkAction && surveyTarget}
				<div class="mb-6 flex items-center gap-3 rounded-lg bg-gray-50 p-3">
					<div
						class="flex h-10 w-10 items-center justify-center rounded-full bg-gradient-to-br from-purple-500 to-pink-600 font-medium text-white"
					>
						{surveyTarget.name.charAt(0).toUpperCase()}
					</div>
					<div>
						<p class="font-medium text-gray-900">{surveyTarget.name}</p>
						<p class="text-sm text-gray-500">{surveyTarget.email}</p>
					</div>
				</div>
			{/if}

			<div class="space-y-4">
				<div>
					<label class="block text-sm font-medium text-gray-700">
						Incentive (optional)
					</label>
					<input
						type="text"
						bind:value={incentiveDescription}
						placeholder="e.g., 10% off your next subscription"
						class="mt-1 w-full rounded-lg border border-gray-300 px-4 py-2 focus:border-blue-500 focus:outline-none"
					/>
					<p class="mt-1 text-xs text-gray-500">
						Offer something in exchange for completing the survey
					</p>
				</div>
			</div>

			<div class="mt-6 flex justify-end gap-3">
				<button
					onclick={() => (showSurveyModal = false)}
					class="rounded-lg border border-gray-300 px-4 py-2 text-gray-700 transition hover:bg-gray-50"
				>
					Cancel
				</button>
				<button
					onclick={sendSurvey}
					disabled={sendingEmail}
					class="flex items-center gap-2 rounded-lg bg-purple-600 px-4 py-2 text-white transition hover:bg-purple-700 disabled:opacity-50"
				>
					{#if sendingEmail}
						<IconLoader class="h-4 w-4 animate-spin" />
					{:else}
						<IconClipboardList class="h-4 w-4" />
					{/if}
					Send Survey
				</button>
			</div>
		</div>
	</div>
{/if}

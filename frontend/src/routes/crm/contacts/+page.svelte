<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import { crmAPI } from '$lib/api/crm';
	import { contacts, contactFilters, isLoading, error } from '$lib/stores/crm';
	import type { Contact, ContactStatus, LifecycleStage } from '$lib/crm/types';
	import {
		IconUser,
		IconSearch,
		IconFilter,
		IconTrendingUp,
		IconAlertTriangle
	} from '$lib/icons';

	let localSearch = $state('');
	let localStatus = $state<ContactStatus | 'all'>('all');
	let localStage = $state<LifecycleStage | 'all'>('all');

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
		$isLoading = true;
		$error = null;

		try {
			$contactFilters = {
				search: localSearch || undefined,
				status: localStatus !== 'all' ? localStatus : undefined,
				lifecycle_stage: localStage !== 'all' ? localStage : undefined,
				sort_by: 'created_at',
				sort_order: 'desc',
				per_page: 100
			};

			const response = await crmAPI.getContacts($contactFilters);
			contacts.set(response.data);
		} catch (e) {
			console.error('Failed to load contacts', e);
			$error = 'Failed to load contacts. Please try again.';
		} finally {
			$isLoading = false;
		}
	}

	function openContact(contact: Contact) {
		goto(`/crm/contacts/${contact.id}`);
	}

	let totalContacts = $derived($contacts.length);
	let highScoreContacts = $derived($contacts.filter((c) => c.lead_score >= 75).length);
	let atRiskCustomers = $derived($contacts.filter(
		(c) => c.status === 'customer' && c.health_score < 50
	).length);
</script>

<svelte:head>
	<title>CRM Contacts | Revolution Trading Pros</title>
	<meta name="description" content="Enterprise CRM contact management" />
</svelte:head>

<div class="min-h-screen bg-slate-950/95 text-slate-50">
	<div class="mx-auto max-w-7xl px-6 py-8">
		<!-- Header -->
		<div class="mb-6 flex flex-wrap items-center justify-between gap-4">
			<div class="flex items-center gap-3">
				<div
					class="flex h-10 w-10 items-center justify-center rounded-xl bg-indigo-500/20 text-indigo-400"
				>
					<IconUser size={22} />
				</div>
				<div>
					<h1 class="text-2xl font-semibold tracking-tight">Contacts</h1>
					<p class="text-sm text-slate-400">
						RevolutionCRM-L8-System · 360° view of every relationship
					</p>
				</div>
			</div>

			<div class="flex items-center gap-2 text-xs text-slate-400">
				<span class="rounded-full bg-emerald-500/10 px-3 py-1 text-emerald-300"
					>Lead Scoring L8</span
				>
				<span class="rounded-full bg-sky-500/10 px-3 py-1 text-sky-300">Behavior Linked</span>
			</div>
		</div>

		<!-- Filters -->
		<div
			class="mb-6 flex flex-wrap items-center gap-3 rounded-2xl border border-slate-800/80 bg-slate-900/70 p-4"
		>
			<div
				class="flex flex-1 items-center gap-2 rounded-xl border border-slate-800 bg-slate-900/80 px-3 py-2"
			>
				<IconSearch size={18} class="text-slate-500" />
				<input
					id="contact-search"
					class="w-full bg-transparent text-sm text-slate-100 outline-none placeholder:text-slate-500"
					placeholder="Search by name, email, title"
					bind:value={localSearch}
					onkeydown={(e) => e.key === 'Enter' && loadContacts()}
				/>
			</div>

			<div class="flex items-center gap-2 text-xs text-slate-300">
				<IconFilter size={16} class="text-slate-500" />
				<select
					class="rounded-lg border border-slate-800 bg-slate-900/80 px-2 py-1 text-xs"
					bind:value={localStatus}
				>
					{#each statusOptions as opt}
						<option value={opt.value}>{opt.label}</option>
					{/each}
				</select>
				<select
					class="rounded-lg border border-slate-800 bg-slate-900/80 px-2 py-1 text-xs"
					bind:value={localStage}
				>
					{#each stageOptions as opt}
						<option value={opt.value}>{opt.label}</option>
					{/each}
				</select>
			</div>

			<button
				class="rounded-xl bg-indigo-500 px-4 py-2 text-sm font-medium text-white shadow-sm hover:bg-indigo-400 disabled:opacity-60"
				onclick={loadContacts}
				disabled={$isLoading}
			>
				{$isLoading ? 'Loading…' : 'Apply filters'}
			</button>
		</div>

		<!-- KPIs -->
		<div class="mb-6 grid gap-4 md:grid-cols-3">
			<div class="rounded-2xl border border-slate-800 bg-slate-900/70 p-4">
				<div class="flex items-center justify-between">
					<div>
						<p class="text-xs uppercase tracking-wide text-slate-500">Total Contacts</p>
						<p class="mt-1 text-2xl font-semibold">{totalContacts}</p>
					</div>
					<div class="rounded-xl bg-slate-800/80 p-2">
						<IconUser size={20} class="text-indigo-400" />
					</div>
				</div>
			</div>

			<div class="rounded-2xl border border-slate-800 bg-slate-900/70 p-4">
				<div class="flex items-center justify-between">
					<div>
						<p class="text-xs uppercase tracking-wide text-slate-500">High-Intent Leads</p>
						<p class="mt-1 text-2xl font-semibold">{highScoreContacts}</p>
					</div>
					<div class="rounded-xl bg-emerald-500/10 p-2">
						<IconTrendingUp size={20} class="text-emerald-400" />
					</div>
				</div>
			</div>

			<div class="rounded-2xl border border-slate-800 bg-slate-900/70 p-4">
				<div class="flex items-center justify-between">
					<div>
						<p class="text-xs uppercase tracking-wide text-slate-500">At-Risk Customers</p>
						<p class="mt-1 text-2xl font-semibold">{atRiskCustomers}</p>
					</div>
					<div class="rounded-xl bg-rose-500/10 p-2">
						<IconAlertTriangle size={20} class="text-rose-400" />
					</div>
				</div>
			</div>
		</div>

		<!-- Error -->
		{#if $error}
			<div
				class="mb-4 rounded-xl border border-rose-700/60 bg-rose-950/40 px-4 py-3 text-sm text-rose-200"
			>
				{$error}
			</div>
		{/if}

		<!-- Table -->
		<div class="overflow-hidden rounded-2xl border border-slate-800/80 bg-slate-900/70">
			<div class="max-h-[70vh] overflow-auto">
				<table class="min-w-full text-sm">
					<thead
						class="sticky top-0 bg-slate-900/95 text-left text-xs uppercase tracking-wide text-slate-500"
					>
						<tr>
							<th class="px-4 py-3">Contact</th>
							<th class="px-4 py-3">Status</th>
							<th class="px-4 py-3">Lifecycle</th>
							<th class="px-4 py-3">Lead Score</th>
							<th class="px-4 py-3">Health</th>
							<th class="px-4 py-3">Engagement</th>
							<th class="px-4 py-3">MRR</th>
							<th class="px-4 py-3">Owner</th>
							<th class="px-4 py-3">Last Activity</th>
						</tr>
					</thead>
					<tbody>
						{#if $isLoading && !$contacts.length}
							<tr>
								<td colspan="9" class="px-4 py-8 text-center text-slate-400">Loading contacts…</td>
							</tr>
						{:else if !$contacts.length}
							<tr>
								<td colspan="9" class="px-4 py-8 text-center text-slate-500">No contacts found.</td>
							</tr>
						{:else}
							{#each $contacts as contact (contact.id)}
								<tr
									class="cursor-pointer border-t border-slate-800/60 hover:bg-slate-800/40"
									onclick={() => openContact(contact)}
								>
									<td class="px-4 py-3">
										<div class="flex flex-col">
											<span class="font-medium text-slate-50">{contact.full_name}</span>
											<span class="text-xs text-slate-400">{contact.email}</span>
										</div>
									</td>
									<td class="px-4 py-3 text-xs">
										<span class="rounded-full bg-slate-800/80 px-2 py-1 text-slate-200">
											{contact.status}
										</span>
									</td>
									<td class="px-4 py-3 text-xs text-slate-300">{contact.lifecycle_stage}</td>
									<td class="px-4 py-3">
										<div class="flex items-center gap-2">
											<div class="h-1.5 w-16 overflow-hidden rounded-full bg-slate-800">
												<div
													class="h-full rounded-full bg-emerald-400"
													style={`width: ${contact.lead_score}%`}
												></div>
											</div>
											<span class="text-xs text-slate-200">{contact.lead_score}</span>
										</div>
									</td>
									<td class="px-4 py-3 text-xs text-slate-200">{contact.health_score}</td>
									<td class="px-4 py-3 text-xs text-slate-200">{contact.engagement_score}</td>
									<td class="px-4 py-3 text-xs text-slate-200">
										{contact.subscription_mrr > 0 ? `$${contact.subscription_mrr.toFixed(0)}` : '—'}
									</td>
									<td class="px-4 py-3 text-xs text-slate-300">{contact.owner?.name ?? '—'}</td>
									<td class="px-4 py-3 text-xs text-slate-400">
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

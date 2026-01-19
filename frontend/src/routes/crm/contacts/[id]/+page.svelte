<!--
	URL: /crm/contacts/[id]
-->

<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/state';
	import { goto } from '$app/navigation';
	import { crmAPI } from '$lib/api/crm';
	import type { Contact, TimelineEvent } from '$lib/crm/types';
	import { IconArrowLeft, IconMail, IconPhone, IconActivity, IconUserCircle } from '$lib/icons';

	let contact = $state<Contact | null>(null);
	let timeline = $state<TimelineEvent[]>([]);
	let loading = $state(true);
	let error = $state<string | null>(null);

	let contactId = $derived(page.params.id as string);

	onMount(async () => {
		await loadContact();
	});

	async function loadContact() {
		loading = true;
		error = null;
		try {
			contact = await crmAPI.getContact(contactId);
			timeline = await crmAPI.getContactTimeline(contactId);
		} catch (e) {
			console.error('Failed to load contact', e);
			error = 'Failed to load contact. Please try again.';
		} finally {
			loading = false;
		}
	}

	function goBack() {
		goto('/crm/contacts');
	}
</script>

<svelte:head>
	<title>{contact ? `${contact.full_name} | CRM Contact` : 'Contact | CRM'}</title>
</svelte:head>

<div class="bg-slate-950/95 text-slate-50">
	<div class="mx-auto max-w-6xl px-6 py-6">
		<button
			class="mb-4 inline-flex items-center gap-2 text-xs text-slate-400 hover:text-slate-200"
			onclick={goBack}
		>
			<IconArrowLeft size={16} />
			Back to contacts
		</button>

		{#if loading && !contact}
			<div class="flex h-64 items-center justify-center text-slate-400">Loading contact…</div>
		{:else if error}
			<div
				class="rounded-xl border border-rose-700/60 bg-rose-950/40 px-4 py-3 text-sm text-rose-200"
			>
				{error}
			</div>
		{:else if contact}
			<div class="grid gap-6 md:grid-cols-[minmax(0,2fr)_minmax(0,1.5fr)]">
				<!-- Left: Profile & Timeline -->
				<div class="space-y-4">
					<!-- Profile Card -->
					<div class="rounded-2xl border border-slate-800/80 bg-slate-900/70 p-5">
						<div class="flex items-start gap-4">
							<div class="flex h-14 w-14 items-center justify-center rounded-full bg-slate-800/80">
								<IconUserCircle size={32} class="text-indigo-300" />
							</div>
							<div class="flex-1">
								<h1 class="text-xl font-semibold leading-tight">{contact.full_name}</h1>
								<p class="text-xs text-slate-400">{contact.job_title}</p>
								<div class="mt-2 flex flex-wrap items-center gap-3 text-xs text-slate-300">
									<span class="rounded-full bg-slate-800/80 px-2 py-1">{contact.status}</span>
									<span class="rounded-full bg-slate-800/80 px-2 py-1"
										>{contact.lifecycle_stage}</span
									>
									{#if contact.subscription_status !== 'none'}
										<span class="rounded-full bg-emerald-500/10 px-2 py-1 text-emerald-300">
											{contact.subscription_status} · ${contact.subscription_mrr.toFixed(0)} MRR
										</span>
									{/if}
								</div>
							</div>
						</div>

						<div class="mt-4 grid gap-3 text-xs text-slate-300 md:grid-cols-2">
							<div class="flex items-center gap-2">
								<IconMail size={14} class="text-slate-400" />
								<a href={`mailto:${contact.email || ''}`} class="hover:underline"
									>{contact.email || ''}</a
								>
							</div>
							{#if contact.phone}
								<div class="flex items-center gap-2">
									<IconPhone size={14} class="text-slate-400" />
									<a href={`tel:${contact.phone}`} class="hover:underline">{contact.phone}</a>
								</div>
							{/if}
						</div>

						<div class="mt-4 grid gap-3 text-xs text-slate-400 md:grid-cols-3">
							<div>
								<p class="text-[11px] uppercase tracking-wide text-slate-500">Lead Score</p>
								<p class="mt-1 text-lg font-semibold text-slate-50">{contact.lead_score}</p>
							</div>
							<div>
								<p class="text-[11px] uppercase tracking-wide text-slate-500">Health</p>
								<p class="mt-1 text-lg font-semibold text-slate-50">{contact.health_score}</p>
							</div>
							<div>
								<p class="text-[11px] uppercase tracking-wide text-slate-500">Engagement</p>
								<p class="mt-1 text-lg font-semibold text-slate-50">{contact.engagement_score}</p>
							</div>
						</div>
					</div>

					<!-- Timeline -->
					<div class="rounded-2xl border border-slate-800/80 bg-slate-900/70 p-5">
						<div class="mb-3 flex items-center gap-2 text-sm font-medium text-slate-100">
							<IconActivity size={18} class="text-sky-400" />
							Activity Timeline
						</div>

						{#if !timeline.length}
							<p class="py-8 text-center text-xs text-slate-500">No timeline events yet.</p>
						{:else}
							<ol class="space-y-3 text-xs">
								{#each timeline as event}
									<li class="flex gap-3">
										<div class="mt-1 h-2 w-2 flex-shrink-0 rounded-full bg-sky-400"></div>
										<div class="flex-1 border-l border-slate-800 pl-3">
											<div class="flex items-center justify-between gap-2">
												<p class="text-[13px] font-medium text-slate-100">{event.title}</p>
												<span class="text-[11px] text-slate-500">
													{new Date(event.occurred_at).toLocaleString()}
												</span>
											</div>
											{#if event.description}
												<p class="mt-1 text-[11px] text-slate-300">{event.description}</p>
											{/if}
											{#if event.created_by}
												<p class="mt-1 text-[11px] text-slate-500">
													by {event.created_by.name}
												</p>
											{/if}
										</div>
									</li>
								{/each}
							</ol>
						{/if}
					</div>
				</div>

				<!-- Right: Metrics -->
				<div class="space-y-4">
					<div class="rounded-2xl border border-slate-800/80 bg-slate-900/70 p-4 text-xs">
						<p class="text-[11px] uppercase tracking-wide text-slate-500">Email Engagement</p>
						<div class="mt-3 grid grid-cols-2 gap-3">
							<div>
								<p class="text-[11px] text-slate-400">Opens</p>
								<p class="text-lg font-semibold text-slate-50">{contact.email_opens}</p>
							</div>
							<div>
								<p class="text-[11px] text-slate-400">Clicks</p>
								<p class="text-lg font-semibold text-slate-50">{contact.email_clicks}</p>
							</div>
						</div>
					</div>

					<div class="rounded-2xl border border-slate-800/80 bg-slate-900/70 p-4 text-xs">
						<p class="text-[11px] uppercase tracking-wide text-slate-500">Behavior Signals</p>
						<div class="mt-3 space-y-2 text-[11px] text-slate-300">
							<p>Avg Engagement Score: {contact.avg_engagement_score.toFixed(1)}</p>
							<p>Avg Intent Score: {contact.avg_intent_score.toFixed(1)}</p>
							<p>Friction Events: {contact.friction_events_count}</p>
							<p>Total Sessions: {contact.total_sessions}</p>
						</div>
					</div>

					<div class="rounded-2xl border border-slate-800/80 bg-slate-900/70 p-4 text-xs">
						<p class="text-[11px] uppercase tracking-wide text-slate-500">Value & Revenue</p>
						<div class="mt-3 space-y-2 text-[11px] text-slate-300">
							<p>MRR: ${contact.subscription_mrr.toFixed(0)}</p>
							<p>LTV: ${contact.lifetime_value.toFixed(0)}</p>
							<p>Deals: {contact.deals_count}</p>
						</div>
					</div>
				</div>
			</div>
		{:else}
			<div class="flex h-64 items-center justify-center text-slate-400">Contact not found.</div>
		{/if}
	</div>
</div>

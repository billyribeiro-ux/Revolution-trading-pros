<script lang="ts">
	import { useCrmAccess } from '#lib/crm/access.svelte.js';
	const access = useCrmAccess();
	import { onMount } from 'svelte';
	import { crmAPI } from '#lib/api/crm.js';
	import type {
		EmailSequence,
		SequenceStats,
		SequenceTracker,
		TrackerStatus
	} from '#lib/crm/types.js';
	import { apiErrorMessage } from '#lib/utils/api-error-message.js';
	import SequenceTestDialog from './SequenceTestDialog.svelte';
	import {
		getSequenceTriggerHistory,
		type SequenceTriggerRecord
	} from '#lib/api/sequence-trigger.js';
	let { id, showSubscribers = false }: { id: string; showSubscribers?: boolean } = $props();
	let sequence = $state<EmailSequence | null>(null);
	let stats = $state<SequenceStats | null>(null);
	let subscribers = $state<SequenceTracker[]>([]);
	let loading = $state(true);
	let ready = $state(false);
	let busy = $state(false);
	let error = $state('');
	let notice = $state('');
	let showTest = $state(false);
	let search = $state('');
	let status = $state<TrackerStatus | ''>('');
	let page = $state(1);
	let pages = $state(1);
	let total = $state(0);
	let selected = $state<string[]>([]);
	let contactIds = $state('');
	let startPosition = $state(0);
	let reason = $state('');
	let readGeneration = 0;
	let triggerRecords = $state<SequenceTriggerRecord[]>([]);
	let triggerLoading = $state(true),
		triggerError = $state('');
	let triggerPage = $state(1),
		triggerPages = $state(1),
		triggerTotal = $state(0);
	let triggerGeneration = 0;
	async function loadTriggers(targetPage = 1) {
		if (!access.can('GET', '/sequences/{id}/triggers')) {
			triggerRecords = [];
			triggerLoading = false;
			triggerError =
				'Recorded trigger activity requires its sequence and contact read permissions.';
			return;
		}
		const generation = ++triggerGeneration;
		triggerLoading = true;
		triggerError = '';
		try {
			const result = await getSequenceTriggerHistory(id, targetPage);
			if (generation !== triggerGeneration) return;
			if (result.page > result.pages) {
				await loadTriggers(result.pages);
				return;
			}
			triggerRecords = result.data;
			triggerPage = result.page;
			triggerPages = result.pages;
			triggerTotal = result.total;
		} catch (cause) {
			if (generation === triggerGeneration) {
				triggerRecords = [];
				triggerError = apiErrorMessage(cause, 'Recorded trigger activity is unavailable');
			}
		} finally {
			if (generation === triggerGeneration) triggerLoading = false;
		}
	}
	async function load() {
		const generation = ++readGeneration;
		loading = true;
		ready = false;
		error = '';
		void loadTriggers(triggerPage);
		try {
			const [saved, enrollments] = await Promise.all([
				crmAPI.getSequence(id),
				showSubscribers
					? crmAPI.getSequenceSubscribers(id, {
							page,
							per_page: 20,
							status: status || undefined,
							search: search || undefined
						})
					: Promise.resolve(null)
			]);
			if (generation !== readGeneration) return;
			sequence = saved.sequence;
			stats = saved.stats;
			if (enrollments) {
				subscribers = enrollments.data;
				total = enrollments.meta.total;
				pages = enrollments.meta.total_pages ?? 1;
			}
			selected = [];
			ready = true;
		} catch (cause) {
			if (generation === readGeneration)
				error = apiErrorMessage(cause, 'Unable to load saved sequence activity');
		} finally {
			if (generation === readGeneration) loading = false;
		}
	}
	async function enroll(event: SubmitEvent) {
		event.preventDefault();
		if (busy || !sequence) return;
		if (!access.can('POST', '/sequences/{id}/subscribe')) {
			error = 'Your role cannot enroll contacts.';
			return;
		}
		busy = true;
		error = '';
		notice = '';
		try {
			const ids = contactIds.split(/[\s,]+/).filter(Boolean);
			if (
				!ids.length ||
				ids.length > 1000 ||
				ids.some((id) => !/^[1-9]\d*$/.test(id)) ||
				new Set(ids).size !== ids.length
			)
				throw new Error('Enter 1–1000 distinct contact IDs');
			const result = await crmAPI.subscribeToSequence(
				id,
				ids,
				startPosition,
				sequence.authoring_version
			);
			notice = `${result.subscribed_count} contacts enrolled; ${ids.length - result.subscribed_count} already enrolled. ${sequence.status === 'paused' ? 'Execution remains paused.' : 'The worker will process due emails.'}`;
			await load();
		} catch (cause) {
			error = apiErrorMessage(cause, 'Enrollment was not confirmed; reload before retrying');
		} finally {
			busy = false;
		}
	}
	async function remove() {
		if (busy || !sequence || !selected.length) return;
		if (!access.can('POST', '/sequences/{id}/unsubscribe')) {
			error = 'Your role cannot cancel these enrollments.';
			return;
		}
		busy = true;
		error = '';
		notice = '';
		try {
			const result = await crmAPI.unsubscribeFromSequence(
				id,
				selected,
				reason,
				sequence.authoring_version
			);
			notice = `${result.unsubscribed_count} enrollments cancelled. An in-flight submission retains its actual receipt.`;
			await load();
		} catch (cause) {
			error = apiErrorMessage(cause, 'Removal was not confirmed; reload before retrying');
		} finally {
			busy = false;
		}
	}
	function filter(event: SubmitEvent) {
		event.preventDefault();
		page = 1;
		void load();
	}
	function changePage(by: number) {
		page += by;
		void load();
	}
	function rate(value: number | null) {
		return value === null ? 'Unavailable' : `${value.toFixed(1)}%`;
	}
	onMount(() => {
		void load();
		return () => {
			readGeneration += 1;
			triggerGeneration += 1;
		};
	});
</script>

<section class="sequence-detail">
	<a href="/admin/crm/sequences">← Sequences</a>
	<header>
		<h1>{sequence?.title ?? 'Sequence'} — {showSubscribers ? 'Subscribers' : 'Analytics'}</h1>
		<button onclick={() => void load()} disabled={loading || busy}>Refresh</button>
	</header>
	{#if error}<p role="alert">{error}</p>{/if}
	{#if notice}<p role="status">{notice}</p>{/if}
	{#if loading}<p role="status">
			Loading saved activity…
		</p>{:else if sequence && stats && ready && access.can('GET', '/sequences/{id}')}
		<nav aria-label="Sequence">
			{#if access.page(`/admin/crm/sequences/${id}/edit`)}<a href="/admin/crm/sequences/{id}/edit"
					>Edit sequence</a
				>{/if}<a href="/admin/crm/sequences/{id}">Analytics</a
			>{#if access.page(`/admin/crm/sequences/${id}/subscribers`)}<a
					href="/admin/crm/sequences/{id}/subscribers">Subscribers</a
				>{/if}<button
				type="button"
				disabled={busy ||
					!access.can('POST', '/sequences/{id}/test') ||
					!access.can('GET', '/sequences/{id}/tests')}
				onclick={() => {
					showTest = true;
				}}>Test saved email</button
			>
		</nav>
		<p>
			Saved status: <strong>{sequence.status}</strong> · Authoring version {sequence.authoring_version}.
			Existing enrollments retain their original content revision.
		</p>
		{#if sequence.last_error}<p role="alert">{sequence.last_error}</p>{/if}
		{#if sequence.trigger_error}<p role="alert">
				Trigger needs review: {sequence.trigger_error}.{#if sequence.trigger_type === 'legacy'}
					Stored trigger: <code>{sequence.historical_trigger_type}</code>.{/if} Open the editor to review
				the trigger settings.
			</p>{/if}
		{#if sequence.definition_error}<p role="alert">
				Stored emails need review: {sequence.definition_error}
			</p>{/if}
		{#if showSubscribers}
			<form onsubmit={(event) => void enroll(event)}>
				<fieldset disabled={busy || !access.can('POST', '/sequences/{id}/subscribe')}>
					<legend>Enroll confirmed contacts</legend><label for="enroll-ids"
						>Contact IDs (separated by commas or spaces)</label
					><textarea id="enroll-ids" bind:value={contactIds} required rows={3}></textarea><a
						href="/admin/crm/contacts">Find contacts</a
					><label for="enroll-start">Starting email</label><select
						id="enroll-start"
						bind:value={startPosition}
						>{#each sequence.emails?.filter((mail) => mail.status === 'active') ?? [] as mail (mail.id)}<option
								value={mail.position}>Email {mail.position + 1}: {mail.title}</option
							>{/each}</select
					>
					<p>
						Contacts must match the saved filters and have current newsletter confirmation. Repeated
						enrollment keeps its existing progress.
					</p>
					<button type="submit">Enroll contacts</button>
				</fieldset>
			</form>
			<form class="filters" onsubmit={filter}>
				<label for="enrollment-search">Search subscribers</label><input
					id="enrollment-search"
					bind:value={search}
					maxlength={255}
				/><label for="enrollment-status">Status</label><select
					id="enrollment-status"
					bind:value={status}
					><option value="">All statuses</option
					>{#each ['active', 'paused', 'completed', 'cancelled', 'failed'] as state (state)}<option
							value={state}>{state}</option
						>{/each}</select
				><button type="submit" disabled={busy}>Apply filters</button>
			</form>
			<p>{total} matching enrollments. Page {page} of {pages}.</p>
			{#if subscribers.length === 0}<p>No enrollments match these filters.</p>{:else}
				<div class="table-scroll">
					<table>
						<thead
							><tr
								><th>Select</th><th>Contact</th><th>Status</th><th>Accepted / opened / clicked</th
								><th>Next execution</th><th>Revision and history</th></tr
							></thead
						><tbody
							>{#each subscribers as tracker (tracker.id)}<tr
									><td
										>{#if tracker.contact_id && ['active', 'paused', 'failed'].includes(tracker.status)}<input
												type="checkbox"
												aria-label="Select {tracker.contact_email}"
												value={tracker.contact_id}
												bind:group={selected}
												disabled={busy}
											/>{/if}</td
									><td
										>{tracker.contact_email}{#if tracker.contact_id}<br /><a
												href="/admin/crm/contacts/{tracker.contact_id}"
												>Contact {tracker.contact_id}</a
											>{:else}<br />Contact deleted{/if}</td
									><td
										>{tracker.status}{#if tracker.last_error}<p>{tracker.last_error}</p>{/if}</td
									><td
										>{tracker.emails_sent} / {tracker.emails_opened} / {tracker.emails_clicked}</td
									><td
										>{tracker.next_execution_at
											? new Date(tracker.next_execution_at).toLocaleString()
											: 'None'}</td
									><td
										><details>
											<summary>Enrollment {tracker.id}</summary>
											<p>Saved revision: {tracker.revision_id}</p>
											<ul>
												{#each tracker.notes ?? [] as note, index (index)}<li>
														{note.action} — {new Date(note.at).toLocaleString()}
													</li>{/each}
											</ul>
										</details></td
									></tr
								>{/each}</tbody
						>
					</table>
				</div>
			{/if}
			<div class="actions">
				<button onclick={() => changePage(-1)} disabled={page <= 1 || busy}>Previous page</button
				><button onclick={() => changePage(1)} disabled={page >= pages || busy}>Next page</button>
			</div>
			<label for="remove-reason">Removal reason</label><input
				id="remove-reason"
				bind:value={reason}
				maxlength={4000}
			/><button
				onclick={() => void remove()}
				disabled={!selected.length || busy || !access.can('POST', '/sequences/{id}/unsubscribe')}
				>Cancel {selected.length} selected enrollments</button
			>
		{:else}
			<dl>
				<dt>Recorded enrollments</dt>
				<dd>{stats.subscribers}</dd>
				<dt>Active enrollments</dt>
				<dd>{stats.active_subscribers}</dd>
				<dt>Completed enrollments</dt>
				<dd>{stats.completed}</dd>
				<dt>Accepted by provider</dt>
				<dd>{stats.total_sent}</dd>
				<dt>Delivered</dt>
				<dd>{stats.total_delivered}</dd>
				<dt>Opened</dt>
				<dd>{stats.total_opened} ({rate(stats.open_rate)})</dd>
				<dt>Clicked</dt>
				<dd>{stats.total_clicked} ({rate(stats.click_rate)})</dd>
				<dt>Bounced</dt>
				<dd>{stats.total_bounced}</dd>
				<dt>Complaints</dt>
				<dd>{stats.total_complained}</dd>
				<dt>Unsubscribed</dt>
				<dd>{stats.total_unsubscribed}</dd>
				<dt>Failed submissions</dt>
				<dd>{stats.failed}</dd>
				<dt>Awaiting reconciliation</dt>
				<dd>{stats.unknown}</dd>
				<dt>Queued / processing</dt>
				<dd>{stats.queued} / {stats.processing}</dd>
				<dt>Attributed revenue</dt>
				<dd>Unavailable</dd>
			</dl>
			<p>
				Metrics come from recorded outbox receipts and authenticated provider events across saved
				revisions. Acceptance and recorded engagement do not establish inbox placement or human
				activity.
			</p>
		{/if}
		{#if access.can('GET', '/sequences/{id}/triggers')}<section
				aria-label="Recorded trigger activity"
			>
				<header>
					<h2>Recorded trigger activity</h2>
					<button
						type="button"
						disabled={triggerLoading || busy}
						onclick={() => void loadTriggers(triggerPage)}>Refresh trigger activity</button
					>
				</header>
				<p>
					Each record links a captured source event to its saved sequence revision and actual
					enrollment outcome. A recorded event or enrollment does not establish email submission or
					delivery.
				</p>
				{#if triggerLoading}<p role="status">Loading recorded trigger activity…</p>
				{:else if triggerError}<p role="alert">{triggerError}</p>
					<button type="button" onclick={() => void loadTriggers(triggerPage)}
						>Retry trigger activity</button
					>
				{:else}<p>{triggerTotal} recorded trigger events · Page {triggerPage} of {triggerPages}.</p>
					{#if !triggerRecords.length}<p>No trigger events have been recorded for this sequence.</p>
					{:else}<div class="table-scroll">
							<table>
								<thead
									><tr
										><th>Source event</th><th>Enrollment result</th><th>Contact</th><th
											>Saved revision</th
										><th>Processing</th></tr
									></thead
								><tbody
									>{#each triggerRecords as record (record.id)}<tr
											><td
												>{record.kind} · Event {record.event_id}<br /><time
													datetime={record.happened_at}
													>{new Date(record.happened_at).toLocaleString()}</time
												></td
											><td
												>{record.status}{#if record.tracker_id}<br />Enrollment {record.tracker_id}{/if}{#if record.last_error}<p
													>
														{record.last_error}
													</p>{/if}</td
											><td
												>{#if record.contact_id}<a href="/admin/crm/contacts/{record.contact_id}"
														>Contact {record.contact_id}</a
													>{:else}Contact unavailable{/if}</td
											><td><code>{record.revision_id}</code></td><td
												>{record.attempts} recorded attempts{#if record.processed_at}<br /><time
														datetime={record.processed_at}
														>{new Date(record.processed_at).toLocaleString()}</time
													>{/if}</td
											></tr
										>{/each}</tbody
								>
							</table>
						</div>{/if}
					<nav aria-label="Trigger history pages">
						<button
							type="button"
							disabled={triggerPage <= 1 || busy}
							onclick={() => void loadTriggers(triggerPage - 1)}>Previous trigger page</button
						><button
							type="button"
							disabled={triggerPage >= triggerPages || busy}
							onclick={() => void loadTriggers(triggerPage + 1)}>Next trigger page</button
						>
					</nav>
				{/if}
			</section>{:else}<p>
				Recorded trigger activity requires sequence and contact read permissions.
			</p>{/if}
	{/if}
</section>
{#if showTest}{#key id}<SequenceTestDialog
			sequenceId={id}
			onclose={() => {
				showTest = false;
			}}
		/>{/key}{/if}

<style>
	.sequence-detail {
		padding: 2rem;
		color: #e2e8f0;
		max-width: 90rem;
	}
	header,
	nav,
	.actions,
	.filters {
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		gap: 1rem;
		margin: 1rem 0;
	}
	header {
		justify-content: space-between;
	}
	fieldset {
		display: grid;
		gap: 0.75rem;
		padding: 1.25rem;
		border: 1px solid #475569;
		border-radius: 0.75rem;
	}
	input,
	select,
	textarea,
	button {
		padding: 0.75rem;
		border: 1px solid #475569;
		border-radius: 0.5rem;
		background: #0f172a;
		color: inherit;
	}
	dl {
		display: grid;
		grid-template-columns: minmax(0, 1fr) minmax(0, 1fr);
		gap: 1rem;
		max-width: 50rem;
	}
	dd {
		margin: 0;
	}
	.table-scroll {
		overflow-x: auto;
	}
	table {
		width: 100%;
		border-collapse: collapse;
	}
	th,
	td {
		text-align: left;
		border-bottom: 1px solid #475569;
		padding: 0.75rem;
		vertical-align: top;
	}
	[role='alert'] {
		color: #fca5a5;
	}
	[role='status'] {
		color: #86efac;
	}
	a {
		color: #7dd3fc;
	}
	button:disabled {
		opacity: 0.55;
	}
</style>

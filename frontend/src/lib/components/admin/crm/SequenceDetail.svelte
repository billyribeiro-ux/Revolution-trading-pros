<script lang="ts">
	import { onMount } from 'svelte';
	import { crmAPI } from '#lib/api/crm.js';
	import type { EmailSequence, SequenceStats, SequenceTracker, TrackerStatus } from '#lib/crm/types.js';
	import { apiErrorMessage } from '#lib/utils/api-error-message.js';
	let { id, showSubscribers = false }: { id: string; showSubscribers?: boolean } = $props();
	let sequence = $state<EmailSequence | null>(null);
	let stats = $state<SequenceStats | null>(null);
	let subscribers = $state<SequenceTracker[]>([]);
	let loading = $state(true);
	let ready = $state(false);
	let busy = $state(false);
	let error = $state('');
	let notice = $state('');
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
	async function load() {
		const generation = ++readGeneration;
		loading = true; ready = false; error = '';
		try {
			const [saved, enrollments] = await Promise.all([
				crmAPI.getSequence(id),
				showSubscribers ? crmAPI.getSequenceSubscribers(id, { page, per_page: 20, status: status || undefined, search: search || undefined }) : Promise.resolve(null)
			]);
			if (generation !== readGeneration) return;
			sequence = saved.sequence; stats = saved.stats;
			if (enrollments) { subscribers = enrollments.data; total = enrollments.meta.total; pages = enrollments.meta.total_pages ?? 1; }
			selected = [];
			ready = true;
		} catch (cause) { if (generation === readGeneration) error = apiErrorMessage(cause, 'Unable to load saved sequence activity'); }
		finally { if (generation === readGeneration) loading = false; }
	}
	async function enroll(event: SubmitEvent) {
		event.preventDefault();
		if (busy || !sequence) return;
		busy = true; error = ''; notice = '';
		try {
			const ids = contactIds.split(/[\s,]+/).filter(Boolean);
			if (!ids.length || ids.length > 1000 || ids.some(id => !/^[1-9]\d*$/.test(id)) || new Set(ids).size !== ids.length) throw new Error('Enter 1–1000 distinct contact IDs');
			const result = await crmAPI.subscribeToSequence(id, ids, startPosition, sequence.authoring_version);
			notice = `${result.subscribed_count} contacts enrolled; ${ids.length - result.subscribed_count} already enrolled. ${sequence.status === 'paused' ? 'Execution remains paused.' : 'The worker will process due emails.'}`;
			await load();
		} catch (cause) { error = apiErrorMessage(cause, 'Enrollment was not confirmed; reload before retrying'); }
		finally { busy = false; }
	}
	async function remove() {
		if (busy || !sequence || !selected.length) return;
		busy = true; error = ''; notice = '';
		try {
			const result = await crmAPI.unsubscribeFromSequence(id, selected, reason, sequence.authoring_version);
			notice = `${result.unsubscribed_count} enrollments cancelled. An in-flight submission retains its actual receipt.`;
			await load();
		} catch (cause) { error = apiErrorMessage(cause, 'Removal was not confirmed; reload before retrying'); }
		finally { busy = false; }
	}
	function filter(event: SubmitEvent) { event.preventDefault(); page = 1; void load(); }
	function changePage(by: number) { page += by; void load(); }
	function rate(value: number | null) { return value === null ? 'Unavailable' : `${value.toFixed(1)}%`; }
	onMount(() => { void load(); return () => { readGeneration += 1; }; });
</script>

<section class="sequence-detail">
	<a href="/admin/crm/sequences">← Sequences</a>
	<header><h1>{sequence?.title ?? 'Sequence'} — {showSubscribers ? 'Subscribers' : 'Analytics'}</h1><button onclick={() => void load()} disabled={loading || busy}>Refresh</button></header>
	{#if error}<p role="alert">{error}</p>{/if}
	{#if notice}<p role="status">{notice}</p>{/if}
	{#if loading}<p role="status">Loading saved activity…</p>{:else if sequence && stats && ready}
		<nav aria-label="Sequence"><a href="/admin/crm/sequences/{id}/edit">Edit sequence</a><a href="/admin/crm/sequences/{id}">Analytics</a><a href="/admin/crm/sequences/{id}/subscribers">Subscribers</a></nav>
		<p>Saved status: <strong>{sequence.status}</strong> · Authoring version {sequence.authoring_version}. Existing enrollments retain their original content revision.</p>
		{#if sequence.last_error}<p role="alert">{sequence.last_error}</p>{/if}
		{#if sequence.definition_error}<p role="alert">Stored emails need review: {sequence.definition_error}</p>{/if}
		{#if showSubscribers}
			<form onsubmit={(event) => void enroll(event)}><fieldset disabled={busy}><legend>Enroll confirmed contacts</legend><label for="enroll-ids">Contact IDs (separated by commas or spaces)</label><textarea id="enroll-ids" bind:value={contactIds} required rows={3}></textarea><a href="/admin/crm/contacts">Find contacts</a><label for="enroll-start">Starting email</label><select id="enroll-start" bind:value={startPosition}>{#each sequence.emails?.filter(mail => mail.status === 'active') ?? [] as mail (mail.id)}<option value={mail.position}>Email {mail.position + 1}: {mail.title}</option>{/each}</select><p>Contacts must match the saved filters and have current newsletter confirmation. Repeated enrollment keeps its existing progress.</p><button type="submit">Enroll contacts</button></fieldset></form>
			<form class="filters" onsubmit={filter}><label for="enrollment-search">Search subscribers</label><input id="enrollment-search" bind:value={search} maxlength={255} /><label for="enrollment-status">Status</label><select id="enrollment-status" bind:value={status}><option value="">All statuses</option>{#each ['active', 'paused', 'completed', 'cancelled', 'failed'] as state (state)}<option value={state}>{state}</option>{/each}</select><button type="submit" disabled={busy}>Apply filters</button></form>
			<p>{total} matching enrollments. Page {page} of {pages}.</p>
			{#if subscribers.length === 0}<p>No enrollments match these filters.</p>{:else}
				<div class="table-scroll"><table><thead><tr><th>Select</th><th>Contact</th><th>Status</th><th>Accepted / opened / clicked</th><th>Next execution</th><th>Revision and history</th></tr></thead><tbody>{#each subscribers as tracker (tracker.id)}<tr><td>{#if tracker.contact_id && ['active', 'paused', 'failed'].includes(tracker.status)}<input type="checkbox" aria-label="Select {tracker.contact_email}" value={tracker.contact_id} bind:group={selected} disabled={busy} />{/if}</td><td>{tracker.contact_email}{#if tracker.contact_id}<br /><a href="/admin/crm/contacts/{tracker.contact_id}">Contact {tracker.contact_id}</a>{:else}<br />Contact deleted{/if}</td><td>{tracker.status}{#if tracker.last_error}<p>{tracker.last_error}</p>{/if}</td><td>{tracker.emails_sent} / {tracker.emails_opened} / {tracker.emails_clicked}</td><td>{tracker.next_execution_at ? new Date(tracker.next_execution_at).toLocaleString() : 'None'}</td><td><details><summary>Enrollment {tracker.id}</summary><p>Saved revision: {tracker.revision_id}</p><ul>{#each tracker.notes ?? [] as note, index (index)}<li>{note.action} — {new Date(note.at).toLocaleString()}</li>{/each}</ul></details></td></tr>{/each}</tbody></table></div>
			{/if}
			<div class="actions"><button onclick={() => changePage(-1)} disabled={page <= 1 || busy}>Previous page</button><button onclick={() => changePage(1)} disabled={page >= pages || busy}>Next page</button></div>
			<label for="remove-reason">Removal reason</label><input id="remove-reason" bind:value={reason} maxlength={4000} /><button onclick={() => void remove()} disabled={!selected.length || busy}>Cancel {selected.length} selected enrollments</button>
		{:else}
			<dl><dt>Recorded enrollments</dt><dd>{stats.subscribers}</dd><dt>Active enrollments</dt><dd>{stats.active_subscribers}</dd><dt>Completed enrollments</dt><dd>{stats.completed}</dd><dt>Accepted by provider</dt><dd>{stats.total_sent}</dd><dt>Delivered</dt><dd>{stats.total_delivered}</dd><dt>Opened</dt><dd>{stats.total_opened} ({rate(stats.open_rate)})</dd><dt>Clicked</dt><dd>{stats.total_clicked} ({rate(stats.click_rate)})</dd><dt>Bounced</dt><dd>{stats.total_bounced}</dd><dt>Complaints</dt><dd>{stats.total_complained}</dd><dt>Unsubscribed</dt><dd>{stats.total_unsubscribed}</dd><dt>Failed submissions</dt><dd>{stats.failed}</dd><dt>Awaiting reconciliation</dt><dd>{stats.unknown}</dd><dt>Queued / processing</dt><dd>{stats.queued} / {stats.processing}</dd><dt>Attributed revenue</dt><dd>Unavailable</dd></dl>
			<p>Metrics come from recorded outbox receipts and authenticated provider events across saved revisions. Acceptance and recorded engagement do not establish inbox placement or human activity.</p>
		{/if}
	{/if}
</section>

<style>
	.sequence-detail { padding: 2rem; color: #e2e8f0; max-width: 90rem; }
	header, nav, .actions, .filters { display: flex; flex-wrap: wrap; align-items: center; gap: 1rem; margin: 1rem 0; }
	header { justify-content: space-between; }
	fieldset { display: grid; gap: .75rem; padding: 1.25rem; border: 1px solid #475569; border-radius: .75rem; }
	input, select, textarea, button { padding: .75rem; border: 1px solid #475569; border-radius: .5rem; background: #0f172a; color: inherit; }
	dl { display: grid; grid-template-columns: minmax(0, 1fr) minmax(0, 1fr); gap: 1rem; max-width: 50rem; }
	dd { margin: 0; }
	.table-scroll { overflow-x: auto; }
	table { width: 100%; border-collapse: collapse; }
	th, td { text-align: left; border-bottom: 1px solid #475569; padding: .75rem; vertical-align: top; }
	[role='alert'] { color: #fca5a5; }
	[role='status'] { color: #86efac; }
	a { color: #7dd3fc; }
	button:disabled { opacity: .55; }
</style>

<script lang="ts">
	import { onMount } from 'svelte';
	import PeriodSelector from '#lib/components/analytics/PeriodSelector.svelte';

	interface Session { session_id: string; duration: number; pages_viewed: number; events_count: number; started_at: string; has_errors: boolean; has_rage_clicks: boolean }
	interface Activity { event_type: string; page_url: string; timestamp: string; event_value: number | null }
	let sessions = $state<Session[]>([]);
	let selected = $state<Session | null>(null);
	let activity = $state<Activity[]>([]);
	let activityTruncated = $state(false);
	let period = $state('7d');
	let filter = $state<'all' | 'with_errors' | 'rage_clicks' | 'long'>('all');
	let page = $state(1);
	let totalPages = $state(1);
	let loading = $state(true);
	let activityLoading = $state(false);
	let error = $state<string | null>(null);
	let requestId = 0;

	async function loadSessions() {
		const current = ++requestId;
		loading = true;
		error = null;
		try {
			const query = new URLSearchParams({ period, filter, page: String(page) });
			const response = await fetch(`/api/admin/analytics/behavior/sessions?${query}`);
			if (!response.ok) throw new Error(`Session request failed (HTTP ${response.status})`);
			const data: { sessions: Session[]; pagination: { total_pages: number } } = await response.json();
			if (current !== requestId) return;
			sessions = data.sessions;
			totalPages = data.pagination.total_pages;
			selected = null;
			activity = [];
		} catch (cause) {
			if (current === requestId) error = cause instanceof Error ? cause.message : 'Session request failed';
		} finally {
			if (current === requestId) loading = false;
		}
	}

	async function selectSession(session: Session) {
		selected = session;
		activity = [];
		activityLoading = true;
		const current = requestId;
		try {
			const response = await fetch(`/api/admin/analytics/behavior/sessions/${encodeURIComponent(session.session_id)}`);
			if (!response.ok) throw new Error(`Activity request failed (HTTP ${response.status})`);
			const data: { events: Activity[]; truncated: boolean } = await response.json();
			if (current === requestId && selected?.session_id === session.session_id) {
				activity = data.events;
				activityTruncated = data.truncated;
			}
		} catch (cause) {
			if (current === requestId) error = cause instanceof Error ? cause.message : 'Activity request failed';
		} finally {
			if (current === requestId) activityLoading = false;
		}
	}

	function formatTime(value: string): string { return new Date(value).toLocaleString(); }
	function changePeriod(value: string) { period = value; page = 1; void loadSessions(); }
	function changeFilter(value: typeof filter) { filter = value; page = 1; void loadSessions(); }
	function changePage(value: number) { page = value; void loadSessions(); }
	onMount(() => { void loadSessions(); return () => { requestId++; }; });
</script>

<main class="sessions-page">
	<header class="page-header"><div><h1>Session activity</h1><p>Time ordered interaction events from consenting visitors. Page content, form values, and DOM replay are not collected.</p></div><PeriodSelector value={period} onchange={changePeriod} /></header>
	<div class="filters" aria-label="Session filters">
		{#each [{ value: 'all', label: 'All' }, { value: 'with_errors', label: 'Form errors' }, { value: 'rage_clicks', label: 'Rage clicks' }, { value: 'long', label: '60 seconds or longer' }] as option (option.value)}
			<button class={{ active: filter === option.value }} onclick={() => changeFilter(option.value as typeof filter)}>{option.label}</button>
		{/each}
	</div>
	{#if error}<p role="alert" class="error">{error} <button onclick={loadSessions}>Retry</button></p>{/if}
	{#if loading}<p>Loading sessions…</p>
	{:else if sessions.length === 0}<p>No consented sessions match this period and filter.</p>
	{:else}
		<div class="layout">
			<section aria-label="Sessions" class="list">
				{#each sessions as session (session.session_id)}
					<button data-session-id={session.session_id} class={{ active: selected?.session_id === session.session_id }} onclick={() => selectSession(session)}>
						<strong>{formatTime(session.started_at)}</strong>
						<span>{session.pages_viewed} pages · {session.events_count} events · {session.duration}s</span>
						{#if session.has_rage_clicks}<small>Rage clicks</small>{/if}
						{#if session.has_errors}<small>Form error</small>{/if}
					</button>
				{/each}
				<div class="pagination"><button disabled={page <= 1} onclick={() => changePage(page - 1)}>Previous</button><span>Page {page} of {totalPages}</span><button disabled={page >= totalPages} onclick={() => changePage(page + 1)}>Next</button></div>
			</section>
			<section class="detail" aria-label="Session event timeline">
				{#if selected}
					<h2>Activity timeline</h2><p>Session {selected.session_id.slice(0, 12)}…</p>
					{#if activityLoading}<p>Loading events…</p>
					{:else if activity.length === 0}<p>No events are retained for this session.</p>
					{:else}{#if activityTruncated}<p>Showing the first 500 retained events.</p>{/if}<ol>{#each activity as event, index (`${event.timestamp}-${index}`)}<li><time datetime={event.timestamp}>{formatTime(event.timestamp)}</time><strong>{event.event_type.replaceAll('_', ' ')}</strong><span>{event.page_url}</span>{#if event.event_value !== null}<span>{event.event_value}</span>{/if}</li>{/each}</ol>{/if}
				{:else}<p>Select a session to inspect its event timeline.</p>{/if}
			</section>
		</div>
	{/if}
</main>

<style>
	.sessions-page{max-width:80rem;margin:auto;padding:2rem;color:#f8fafc}.page-header{display:flex;justify-content:space-between;align-items:start;gap:1rem;margin-bottom:1.5rem}.page-header h1{margin:0 0 .4rem}.page-header p,.detail p{color:#94a3b8}.filters{display:flex;flex-wrap:wrap;gap:.5rem;margin-bottom:1.5rem}.filters button,.pagination button{padding:.55rem .85rem;border:1px solid #475569;border-radius:.5rem;background:#1e293b;color:inherit;cursor:pointer}.filters button.active{border-color:#facc15}.layout{display:grid;grid-template-columns:minmax(18rem,24rem) 1fr;gap:1.5rem}.list{display:flex;flex-direction:column;gap:.5rem}.list>button{background:#1e293b;border:1px solid #334155;border-radius:.5rem;color:inherit;text-align:left;padding:.85rem;cursor:pointer}.list>button.active{border-color:#facc15}.list>button strong,.list>button span{display:block}.list>button span{color:#94a3b8;margin-top:.35rem}.list>button small{display:inline-block;color:#fcd34d;margin:.4rem .5rem 0 0}.pagination{display:flex;align-items:center;justify-content:space-between;gap:.5rem;margin-top:.6rem}.pagination button:disabled{opacity:.4;cursor:default}.detail{background:#111827;border:1px solid #334155;border-radius:.75rem;padding:1.25rem;min-width:0}.detail ol{padding-left:1.4rem}.detail li{padding:.7rem 0;border-bottom:1px solid #334155;overflow-wrap:anywhere}.detail li time,.detail li strong,.detail li span{display:block}.detail li time{font-size:.85rem;color:#94a3b8}.detail li strong{text-transform:capitalize;margin:.2rem 0}.error{color:#fecaca}@media(max-width:700px){.page-header{flex-direction:column}.layout{grid-template-columns:1fr}}
</style>

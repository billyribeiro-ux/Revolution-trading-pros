<script lang="ts">
	import { onMount } from 'svelte';
	import PeriodSelector from '#lib/components/analytics/PeriodSelector.svelte';

	interface PageSummary { id: string; url: string; clicks: number; sessions: number; scroll_depth: number }
	interface Point { x: number; y: number; count: number }
	interface ScrollDepth { depth: number; sessions: number }
	interface PageDetail { page_url: string; points: Point[]; scroll_depth: ScrollDepth[]; sessions: number }

	let pages = $state<PageSummary[]>([]);
	let selected = $state<PageSummary | null>(null);
	let detail = $state<PageDetail | null>(null);
	let period = $state('7d');
	let loading = $state(true);
	let detailLoading = $state(false);
	let error = $state<string | null>(null);
	let requestId = 0;

	async function loadPages() {
		const current = ++requestId;
		loading = true;
		error = null;
		try {
			const response = await fetch(`/api/admin/analytics/behavior/heatmaps?period=${period}`);
			if (!response.ok) throw new Error(`Heatmap request failed (HTTP ${response.status})`);
			const data: { pages: PageSummary[] } = await response.json();
			if (current !== requestId) return;
			pages = data.pages;
			selected = data.pages.find((page) => page.url === selected?.url) ?? data.pages[0] ?? null;
			if (selected) await loadDetail(selected);
			else detail = null;
		} catch (cause) {
			if (current === requestId) error = cause instanceof Error ? cause.message : 'Heatmap request failed';
		} finally {
			if (current === requestId) loading = false;
		}
	}

	async function loadDetail(page: PageSummary) {
		selected = page;
		detail = null;
		detailLoading = true;
		const current = requestId;
		try {
			const query = new URLSearchParams({ period, page_url: page.url });
			const response = await fetch(`/api/admin/analytics/behavior/heatmaps?${query}`);
			if (!response.ok) throw new Error(`Click data request failed (HTTP ${response.status})`);
			const result: PageDetail = await response.json();
			if (current === requestId && selected?.url === page.url) detail = result;
		} catch (cause) {
			if (current === requestId) error = cause instanceof Error ? cause.message : 'Click data request failed';
		} finally {
			if (current === requestId) detailLoading = false;
		}
	}

	function changePeriod(value: string) { period = value; void loadPages(); }
	onMount(() => { void loadPages(); return () => { requestId++; }; });
	let maxCount = $derived(Math.max(1, ...(detail?.points.map((point) => point.count) ?? [])));
</script>

<main class="behavior-page">
	<header class="page-header">
		<div><h1>Click heatmaps</h1><p>Aggregated clicks from visitors who consented to analytics. Positions are relative to the viewport.</p></div>
		<PeriodSelector value={period} onchange={changePeriod} />
	</header>
	{#if error}<p role="alert" class="error">{error} <button onclick={loadPages}>Retry</button></p>{/if}
	{#if loading}<p>Loading click data…</p>
	{:else if pages.length === 0}<p>No consented activity is available for this period.</p>
	{:else}
		<div class="layout">
			<nav aria-label="Tracked pages" class="pages">
				{#each pages as page (page.id)}
					<button class={{ active: selected?.url === page.url }} onclick={() => loadDetail(page)}>
						<strong>{page.url}</strong><span>{page.clicks.toLocaleString()} clicks · {page.sessions.toLocaleString()} sessions</span>
					</button>
				{/each}
			</nav>
			<section class="detail" aria-label="Page click density">
				<h2>{selected?.url}</h2>
				{#if detailLoading}<p>Loading page detail…</p>
				{:else if detail}
					<p>{selected?.clicks.toLocaleString()} clicks · {detail.sessions.toLocaleString()} sessions with a page view</p>
					<div class="viewport" aria-label="Click density by viewport position">
						{#each detail.points as point (`${point.x}-${point.y}`)}
							<span class="point" style:left={`${point.x}%`} style:top={`${point.y}%`} style:--intensity={point.count / maxCount} title={`${point.count} clicks near ${point.x}%, ${point.y}%`}></span>
						{/each}
					</div>
					<p class="note">Each dot is a 5% viewport cell. Historical clicks without normalized coordinates are excluded.</p>
					<h3>Scroll depth</h3>
					{#each detail.scroll_depth as row (row.depth)}
						<div class="scroll-row"><span>{row.depth}%</span><progress value={row.sessions} max={Math.max(1, detail.sessions)}></progress><span>{row.sessions} sessions</span></div>
					{/each}
					{#if detail.scroll_depth.length === 0}<p>No scroll events for this page.</p>{/if}
				{/if}
			</section>
		</div>
	{/if}
</main>

<style>
	.behavior-page{max-width:80rem;margin:auto;padding:2rem;color:#f8fafc}.page-header{display:flex;justify-content:space-between;align-items:start;gap:1rem;margin-bottom:2rem}.page-header h1{margin:0 0 .4rem}.page-header p,.note{color:#94a3b8}.layout{display:grid;grid-template-columns:minmax(15rem,20rem) 1fr;gap:1.5rem}.pages{display:flex;flex-direction:column;gap:.5rem;max-height:42rem;overflow:auto}.pages button{color:inherit;background:#1e293b;border:1px solid #334155;border-radius:.5rem;text-align:left;padding:.8rem;cursor:pointer}.pages button.active{border-color:#facc15}.pages strong,.pages span{display:block;overflow-wrap:anywhere}.pages span{color:#94a3b8;font-size:.85rem;margin-top:.35rem}.detail{background:#111827;border:1px solid #334155;border-radius:.75rem;padding:1.25rem;min-width:0}.detail h2{overflow-wrap:anywhere}.viewport{height:28rem;position:relative;overflow:hidden;border:1px solid #475569;border-radius:.5rem;background:linear-gradient(90deg,transparent 49.8%,#334155 50%,transparent 50.2%),linear-gradient(transparent 49.8%,#334155 50%,transparent 50.2%),#0f172a}.point{position:absolute;width:1.6rem;height:1.6rem;transform:translate(-50%,-50%);border-radius:50%;background:rgba(239,68,68,var(--intensity));box-shadow:0 0 1.4rem #ef4444}.scroll-row{display:grid;grid-template-columns:3rem 1fr 5rem;align-items:center;gap:.7rem;margin:.6rem 0}.scroll-row progress{width:100%}.error{color:#fecaca}.error button{margin-left:.6rem}@media(max-width:700px){.page-header{flex-direction:column}.layout{grid-template-columns:1fr}.pages{max-height:14rem}}
</style>

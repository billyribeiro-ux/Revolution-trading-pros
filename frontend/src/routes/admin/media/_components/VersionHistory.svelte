<script lang="ts">
	import { onDestroy, onMount } from 'svelte';
	import { mediaApi, type MediaFile, type MediaVersion } from '#lib/api/media.js';
	let {
		mediaId,
		revision,
		onrestore
	}: { mediaId: string; revision?: number; onrestore: (file: MediaFile) => void } = $props();
	let error = $state('');
	let page = $state(1);
	let restoring = $state<string | null>(null);
	let reload = $state(0);
	let disposed = false;
	let mounted = $state(false);
	onMount(() => {
		mounted = true;
	});
	onDestroy(() => {
		disposed = true;
	});
	// Revision and retry are dependency keys for the derived request.
	const history = $derived(fetchVersions(mediaId, page, revision, reload));
	function fetchVersions(
		id: string,
		selectedPage: number,
		_revision: number | undefined,
		_retry: number
	) {
		return mediaApi.getVersions(id, { page: selectedPage, per_page: 10 });
	}
	async function restore(version: MediaVersion) {
		const id = mediaId;
		restoring = version.id;
		error = '';
		try {
			const result = await mediaApi.restoreVersion(id, version.id);
			if (!disposed && mediaId === id) {
				page = 1;
				onrestore(result.file);
				reload++;
			}
		} catch (caught) {
			if (!disposed && mediaId === id)
				error = caught instanceof Error ? caught.message : 'Unable to restore version';
		} finally {
			restoring = null;
		}
	}
</script>

<section class="version-history" aria-label="Version history">
	<h3>Version history</h3>
	<p class="hint">Restoring keeps the current file in history and creates a new revision.</p>
	{#if error}<p role="alert">{error}</p>
		<button onclick={() => reload++}>Retry</button>{/if}
	{#if mounted}
		{#await history}
			<p aria-busy="true">Loading versions…</p>
		{:then result}
			{#if result.versions.length === 0}<p>No versions recorded.</p>{/if}
			{#each result.versions as version (version.id)}
				<div class="version-row">
					<div>
						<strong>Version {version.version_number}</strong>
						<p>{version.changes}</p>
						<time datetime={version.created_at}
							>{new Date(version.created_at).toLocaleString()}</time
						>
					</div>
					{#if version.version_number === revision}<span>Current</span>
					{:else}<button disabled={restoring !== null} onclick={() => void restore(version)}
							>{restoring === version.id ? 'Restoring…' : 'Restore'}</button
						>{/if}
				</div>
			{/each}
			{#if result.meta.last_page > 1}<nav aria-label="Version pages">
					<button disabled={page <= 1} onclick={() => page--}>Previous</button><span
						>{page} / {result.meta.last_page}</span
					><button disabled={page >= result.meta.last_page} onclick={() => page++}>Next</button>
				</nav>{/if}
		{:catch caught}
			<p role="alert">{caught instanceof Error ? caught.message : 'Unable to load versions'}</p>
			<button onclick={() => reload++}>Retry</button>
		{/await}
	{:else}<p>Loading versions…</p>{/if}
</section>

<style>
	.version-history {
		padding: 1.25rem;
		border-top: 1px solid var(--border-default, #30363d);
		font-size: 0.8125rem;
	}
	h3 {
		margin: 0 0 0.5rem;
		font-size: 0.9rem;
	}
	.hint,
	time {
		color: var(--text-secondary, #8b949e);
	}
	.version-row {
		display: flex;
		justify-content: space-between;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem 0;
		border-bottom: 1px solid var(--border-default, #30363d);
	}
	.version-row p {
		margin: 0.25rem 0;
	}
	button {
		padding: 0.35rem 0.6rem;
		border: 1px solid var(--border-default, #30363d);
		border-radius: 4px;
		background: var(--bg-surface, #1c2128);
		color: inherit;
		cursor: pointer;
	}
	button:disabled {
		opacity: 0.5;
		cursor: wait;
	}
	nav {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-top: 0.75rem;
	}
	[role='alert'] {
		color: var(--admin-error, #ff6666);
	}
</style>

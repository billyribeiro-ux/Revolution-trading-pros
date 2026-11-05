<script lang="ts">
	type Pagination = { total: number; current_page: number; per_page: number; last_page: number };
	let {
		label,
		pagination,
		loading,
		error,
		busy = false,
		onPage,
		onPerPage,
		onRefresh
	}: {
		label: string;
		pagination: Pagination | null;
		loading: boolean;
		error: string;
		busy?: boolean;
		onPage: (page: number) => void;
		onPerPage: (size: number) => void;
		onRefresh: () => void;
	} = $props();
</script>

<footer class="history-controls">
	<div class="refresh">
		<button onclick={onRefresh} disabled={loading || busy}>Refresh {label}</button>{#if loading}<p
				role="status"
			>
				Loading {label}…
			</p>{:else if error}<p role="alert">{error}</p>
			<button onclick={onRefresh} disabled={busy}>Retry {label}</button>{:else if !pagination}<p>
				{label} is unavailable. Refresh to read its saved state.
			</p>{/if}
	</div>
	{#if pagination && !error}<nav aria-label={`${label} pages`}>
			<button
				disabled={loading || busy || pagination.current_page <= 1}
				onclick={() => onPage(pagination.current_page - 1)}>Previous {label}</button
			><span
				>Page {pagination.current_page} of {pagination.last_page} · {pagination.total} recorded rows</span
			><button
				disabled={loading || busy || pagination.current_page >= pagination.last_page}
				onclick={() => onPage(pagination.current_page + 1)}>Next {label}</button
			><label
				>Rows per page<select
					value={pagination.per_page}
					disabled={loading || busy}
					onchange={(event) => onPerPage(Number(event.currentTarget.value))}
					>{#each [10, 25, 50, 100] as size (size)}<option value={size}>{size}</option
						>{/each}</select
				></label
			>
		</nav>{/if}
</footer>

<style>
	.history-controls {
		padding: 1rem 1.5rem;
		border-top: 1px solid var(--border-muted, #21262d);
		color: var(--text-secondary, #8b949e);
	}
	.refresh,
	nav {
		display: flex;
		gap: 0.75rem;
		align-items: center;
		flex-wrap: wrap;
	}
	nav {
		margin-top: 0.75rem;
	}
	p {
		margin: 0.5rem 0;
	}
	button,
	select {
		background: var(--bg-surface, #1c2128);
		color: var(--text-primary, #f0f6fc);
		padding: 0.5rem 0.75rem;
		border: 1px solid var(--border-default, #30363d);
		border-radius: 8px;
	}
	button {
		cursor: pointer;
	}
	button:disabled {
		opacity: 0.5;
		cursor: default;
	}
	label {
		display: flex;
		gap: 0.5rem;
		align-items: center;
	}
	[role='alert'] {
		color: var(--error-emphasis, #f85149);
	}
</style>

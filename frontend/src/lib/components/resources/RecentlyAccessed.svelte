<!--
  RecentlyAccessed.svelte
  Apple Principal Engineer ICT 7 Grade - February 2026

  Display user's recently accessed resources:
  - Horizontal scrolling carousel
  - Thumbnail previews
  - Quick access links
  - Empty state handling
-->
<script lang="ts">
	import type { RecentlyAccessed } from '$lib/api/room-resources';
	import { getRecentlyAccessed } from '$lib/api/room-resources';

	interface Props {
		limit?: number;
		showTitle?: boolean;
		compact?: boolean;
		initialData?: RecentlyAccessed[];
		onSelect?: (item: RecentlyAccessed) => void;
	}

	let {
		limit = 10,
		showTitle = true,
		compact = false,
		initialData = undefined,
		onSelect
	}: Props = $props();

	let items = $state<RecentlyAccessed[]>([]);
	let loading = $state(true);
	let error = $state('');

	$effect(() => {
		if (initialData) {
			items = initialData;
			loading = false;
		} else {
			loadRecentlyAccessed();
		}
	});

	async function loadRecentlyAccessed() {
		loading = true;
		error = '';

		try {
			const response = await getRecentlyAccessed(limit);
			items = response.data;
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load recently accessed';
		} finally {
			loading = false;
		}
	}

	function handleItemClick(item: RecentlyAccessed) {
		onSelect?.(item);
	}

	function getResourceIcon(type: string): string {
		switch (type) {
			case 'video':
				return 'M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z';
			case 'pdf':
				return 'M7 21h10a2 2 0 002-2V9.414a1 1 0 00-.293-.707l-5.414-5.414A1 1 0 0012.586 3H7a2 2 0 00-2 2v14a2 2 0 002 2z';
			case 'image':
				return 'M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z';
			case 'document':
				return 'M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z';
			case 'spreadsheet':
				return 'M3 10h18M3 14h18m-9-4v8m-7 0h14a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z';
			default:
				return 'M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z';
		}
	}

	function formatTimeAgo(dateString: string): string {
		const date = new Date(dateString);
		const now = new Date();
		const diffMs = now.getTime() - date.getTime();
		const diffMins = Math.floor(diffMs / 60000);
		const diffHours = Math.floor(diffMs / 3600000);
		const diffDays = Math.floor(diffMs / 86400000);

		if (diffMins < 1) return 'Just now';
		if (diffMins < 60) return `${diffMins}m ago`;
		if (diffHours < 24) return `${diffHours}h ago`;
		if (diffDays < 7) return `${diffDays}d ago`;
		return date.toLocaleDateString();
	}
</script>

<div class="ra-root">
	{#if showTitle}
		<div class="ra-header">
			<h3 class="ra-title">Recently Accessed</h3>
			{#if items.length > 0}
				<button class="ra-refresh" onclick={loadRecentlyAccessed}>Refresh</button>
			{/if}
		</div>
	{/if}

	{#if loading}
		<div class="ra-scroll">
			{#each Array(4) as _, i (i)}
				<div class="ra-item-skel" data-compact={compact || undefined}>
					<div class="ra-skel-thumb"></div>
					<div class="ra-skel-line ra-skel-w34"></div>
					<div class="ra-skel-line ra-skel-w12"></div>
				</div>
			{/each}
		</div>
	{:else if error}
		<div class="ra-error">
			<p class="ra-error-msg">{error}</p>
			<button class="ra-error-retry" onclick={loadRecentlyAccessed}>Try again</button>
		</div>
	{:else if items.length === 0}
		<div class="ra-empty">
			<svg class="ra-empty-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
				<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
			</svg>
			<p class="ra-empty-text">No recently accessed resources</p>
			<p class="ra-empty-sub">Resources you view will appear here</p>
		</div>
	{:else}
		<div class="ra-scroll">
			{#each items as item (item.id)}
				<button class="ra-item" data-compact={compact || undefined} onclick={() => handleItemClick(item)}>
					<div class="ra-thumb">
						{#if item.resource_thumbnail}
							<img src={item.resource_thumbnail} alt={item.resource_title} class="ra-thumb-img" loading="lazy" />
						{:else}
							<div class="ra-thumb-placeholder">
								<svg class="ra-icon-md" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d={getResourceIcon(item.resource_type)} />
								</svg>
							</div>
						{/if}
						<span class="ra-type-badge">{item.resource_type}</span>
					</div>
					<div class="ra-item-text">
						<p class="ra-item-title">{item.resource_title}</p>
						<p class="ra-item-time">{formatTimeAgo(item.accessed_at)}</p>
					</div>
				</button>
			{/each}
		</div>
	{/if}
</div>

<style>
	.ra-root { display: flex; flex-direction: column; }

	.ra-header {
		margin-block-end: var(--space-4);
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	.ra-title { font-size: var(--text-lg); font-weight: var(--weight-semibold); color: oklch(0.15 0.01 265); }

	.ra-refresh {
		background: none;
		border: none;
		cursor: pointer;
		font-size: var(--text-sm);
		color: oklch(0.5 0.2 260);
		transition: color 200ms var(--ease-default);

		&:hover { color: oklch(0.45 0.2 260); }
	}

	/* ─── Scroll container ─── */
	.ra-scroll {
		display: flex;
		gap: var(--space-4);
		overflow-x: auto;
		padding-block-end: var(--space-2);

		&::-webkit-scrollbar { block-size: 6px; }
		&::-webkit-scrollbar-thumb { background-color: oklch(0.82 0.005 265); border-radius: 3px; }
		&::-webkit-scrollbar-track { background: transparent; }
	}

	/* ─── Item card ─── */
	.ra-item {
		inline-size: 12rem;
		flex-shrink: 0;
		text-align: start;
		background: none;
		border: none;
		cursor: pointer;
		transition: transform 200ms var(--ease-default);

		&:hover { transform: scale(1.02); }
		&:not([data-compact]) { inline-size: 12rem; }
	}

	.ra-item:not([data-compact]) { inline-size: 12rem; }

	/* ─── Thumbnail ─── */
	.ra-thumb {
		position: relative;
		aspect-ratio: 16 / 9;
		overflow: hidden;
		border-radius: var(--radius-lg);
		background-color: oklch(0.9 0.005 265);
	}

	.ra-thumb-img {
		inline-size: 100%;
		block-size: 100%;
		object-fit: cover;
		transition: transform 200ms var(--ease-default);
	}

	.ra-item:hover .ra-thumb-img { transform: scale(1.05); }

	.ra-thumb-placeholder {
		display: flex;
		align-items: center;
		justify-content: center;
		inline-size: 100%;
		block-size: 100%;
		background: linear-gradient(to bottom right, oklch(0.95 0.002 265), oklch(0.9 0.005 265));
	}

	.ra-icon-md { inline-size: 2rem; block-size: 2rem; color: oklch(0.65 0.01 265); }

	.ra-type-badge {
		position: absolute;
		inset-block-end: 0.25rem;
		inset-inline-start: 0.25rem;
		border-radius: var(--radius-sm);
		background-color: oklch(0 0 0 / 60%);
		padding-inline: 0.375rem;
		padding-block: 0.125rem;
		font-size: var(--text-xs);
		font-weight: var(--weight-medium);
		text-transform: capitalize;
		color: oklch(1 0 0);
	}

	/* ─── Text ─── */
	.ra-item-text { margin-block-start: var(--space-2); }

	.ra-item-title {
		font-size: var(--text-sm);
		font-weight: var(--weight-medium);
		color: oklch(0.15 0.01 265);
		display: -webkit-box;
		-webkit-line-clamp: 2;
		line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
		transition: color 200ms var(--ease-default);
	}

	.ra-item:hover .ra-item-title { color: oklch(0.5 0.2 260); }

	.ra-item-time {
		margin-block-start: 0.125rem;
		font-size: var(--text-xs);
		color: oklch(0.55 0.01 265);
	}

	/* ─── Error ─── */
	.ra-error {
		border-radius: var(--radius-lg);
		border: 1px solid oklch(0.75 0.15 25);
		background-color: oklch(0.97 0.02 25);
		padding: var(--space-4);
		text-align: center;
	}

	.ra-error-msg { font-size: var(--text-sm); color: oklch(0.5 0.2 25); }

	.ra-error-retry {
		margin-block-start: var(--space-2);
		background: none;
		border: none;
		cursor: pointer;
		font-size: var(--text-sm);
		color: oklch(0.45 0.2 25);
		text-decoration: underline;

		&:hover { text-decoration: none; }
	}

	/* ─── Empty ─── */
	.ra-empty {
		border-radius: var(--radius-lg);
		border: 1px solid oklch(0.9 0.005 265);
		background-color: oklch(0.97 0.002 265);
		padding: var(--space-6);
		text-align: center;
	}

	.ra-empty-icon { margin-inline: auto; inline-size: 2.5rem; block-size: 2.5rem; color: oklch(0.65 0.01 265); }
	.ra-empty-text { margin-block-start: var(--space-2); font-size: var(--text-sm); color: oklch(0.55 0.01 265); }
	.ra-empty-sub { font-size: var(--text-xs); color: oklch(0.65 0.01 265); }

	/* ─── Skeleton ─── */
	.ra-item-skel {
		inline-size: 12rem;
		flex-shrink: 0;
		animation: pulse 2s ease-in-out infinite;
	}

	.ra-skel-thumb {
		aspect-ratio: 16 / 9;
		border-radius: var(--radius-lg);
		background-color: oklch(0.9 0.005 265);
	}

	.ra-skel-line {
		border-radius: var(--radius-sm);
		background-color: oklch(0.9 0.005 265);
	}

	.ra-skel-w34 { block-size: 1rem; inline-size: 75%; margin-block-start: var(--space-2); }
	.ra-skel-w12 { block-size: 0.75rem; inline-size: 50%; margin-block-start: 0.25rem; }

	@keyframes pulse {
		0%, 100% { opacity: 1; }
		50% { opacity: 0.5; }
	}
</style>

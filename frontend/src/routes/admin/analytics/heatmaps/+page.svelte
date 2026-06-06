<script lang="ts">
	/**
	 * Click Heatmaps - Visual User Interaction Analysis
	 * Apple ICT7 Grade Implementation
	 *
	 * Visualize where users click on your pages
	 * with interactive heatmap overlays.
	 */
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import { connections, getIsAnalyticsConnected } from '$lib/stores/connections.svelte';
	import ServiceConnectionStatus from '$lib/components/admin/ServiceConnectionStatus.svelte';
	import PeriodSelector from '$lib/components/analytics/PeriodSelector.svelte';
	// FIX-2026-04-26: Tabler icons replace raw inline <svg> blocks.
	import IconPointer from '@tabler/icons-svelte-runes/icons/pointer';
	import IconAlertCircle from '@tabler/icons-svelte-runes/icons/alert-circle';
	import IconPhoto from '@tabler/icons-svelte-runes/icons/photo';

	interface HeatmapPage {
		id: string;
		url: string;
		title: string;
		clicks: number;
		unique_visitors: number;
		scroll_depth: number;
		last_updated: string;
		thumbnail?: string;
	}

	// Svelte 5 Runes - State
	let pages = $state<HeatmapPage[]>([]);
	let selectedPage = $state<HeatmapPage | null>(null);
	let loading = $state(true);
	let connectionLoading = $state(true);
	let error = $state<string | null>(null);
	let selectedPeriod = $state('7d');
	let heatmapType = $state<'click' | 'scroll' | 'move'>('click');

	async function loadPages() {
		loading = true;
		error = null;
		try {
			const params = new URLSearchParams({ period: selectedPeriod });
			const response = await fetch(`/api/admin/analytics/heatmaps?${params.toString()}`);
			if (!response.ok) {
				// FIX-2026-04-26 (audit 08-analytics §P1-4): surface real upstream
				// status instead of silently zeroing the list.
				throw new Error(`Failed to load heatmaps (HTTP ${response.status})`);
			}
			const data = await response.json();
			pages = data.pages || [];
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load heatmaps';
			pages = [];
		} finally {
			loading = false;
		}
	}

	async function loadHeatmapData(page: HeatmapPage) {
		selectedPage = page;
		try {
			const response = await fetch(
				`/api/admin/analytics/heatmaps/${encodeURIComponent(page.url)}?period=${selectedPeriod}&type=${heatmapType}`
			);
			if (response.ok) {
				await response.json();
			}
		} catch (_e) {
			// Handle gracefully
		}
	}

	function handlePeriodChange(value: string) {
		selectedPeriod = value;
		loadPages();
		if (selectedPage) {
			loadHeatmapData(selectedPage);
		}
	}

	// FIX-2026-04-26 (P1-3): $derived restores reactivity past helper's `untrack`.
	let isAnalyticsConnected = $derived(getIsAnalyticsConnected());

	// FIX-2026-04-26 (P1-1): onMount replaces the $effect cascade pattern.
	onMount(() => {
		if (!browser) return;

		(async () => {
			try {
				await connections.load();
			} catch (e) {
				if (import.meta.env.DEV) {
					console.error('[Heatmaps] Failed to load connection status:', e);
				}
			} finally {
				connectionLoading = false;
			}

			if (getIsAnalyticsConnected()) {
				await loadPages();
			} else {
				loading = false;
			}
		})();
	});

	// Derived stats
	const stats = $derived({
		totalPages: pages.length,
		totalClicks: pages.reduce((sum, p) => sum + p.clicks, 0),
		avgScrollDepth:
			pages.length > 0
				? Math.round(pages.reduce((sum, p) => sum + p.scroll_depth, 0) / pages.length)
				: 0,
		totalVisitors: pages.reduce((sum, p) => sum + p.unique_visitors, 0)
	});
</script>

<svelte:head>
	<title>Click Heatmaps | Analytics</title>
</svelte:head>

<div class="heatmaps-page">
	<div class="heatmaps-container">
		<!-- Apple ICT7 Grade Header -->
		<header class="heatmaps-header">
			<div class="header-title-group">
				<div class="header-icon">
					<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: pointer/cursor (heatmap) -->
					<IconPointer size={24} aria-hidden="true" />
				</div>
				<div>
					<h1 class="page-title">Click Heatmaps</h1>
					<p class="page-subtitle">Visualize where users click on your pages</p>
				</div>
			</div>
			{#if isAnalyticsConnected}
				<div class="header-controls">
					<PeriodSelector value={selectedPeriod} onchange={handlePeriodChange} />
					<div class="type-switcher">
						{#each [{ value: 'click', label: 'Clicks' }, { value: 'scroll', label: 'Scroll' }, { value: 'move', label: 'Movement' }] as type (type.value)}
							<button
								onclick={() => (heatmapType = type.value as typeof heatmapType)}
								class={{
									'type-button': true,
									'type-button--active': heatmapType === type.value
								}}
							>
								{type.label}
							</button>
						{/each}
					</div>
				</div>
			{/if}
		</header>

		<!-- Connection Check -->
		{#if connectionLoading}
			<div class="loading-state">
				<div class="spinner spinner--large"></div>
			</div>
		{:else if !isAnalyticsConnected}
			<ServiceConnectionStatus feature="analytics" variant="card" showFeatures={true} />
		{:else}
			<!-- Stats Grid -->
			<div class="stats-grid">
				<div class="metric-card">
					<div class="metric-value">{stats.totalPages}</div>
					<div class="metric-label">Tracked Pages</div>
				</div>
				<div class="metric-card">
					<div class="metric-value metric-value--rose">
						{stats.totalClicks.toLocaleString()}
					</div>
					<div class="metric-label">Total Clicks</div>
				</div>
				<div class="metric-card">
					<div class="metric-value metric-value--amber">{stats.avgScrollDepth}%</div>
					<div class="metric-label">Avg Scroll Depth</div>
				</div>
				<div class="metric-card">
					<div class="metric-value metric-value--blue">
						{stats.totalVisitors.toLocaleString()}
					</div>
					<div class="metric-label">Unique Visitors</div>
				</div>
			</div>

			{#if loading}
				<div class="loading-state">
					<div class="spinner"></div>
				</div>
			{:else if error}
				<div class="panel-state panel-state--error">
					<div class="state-icon state-icon--error">
						<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: alert-circle error -->
						<IconAlertCircle size={32} aria-hidden="true" />
					</div>
					<p class="error-message">{error}</p>
					<button onclick={loadPages} class="retry-button"> Retry </button>
				</div>
			{:else if pages.length === 0}
				<div class="panel-state">
					<div class="state-icon state-icon--rose">
						<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: pointer (no heatmap data empty state) -->
						<IconPointer size={32} aria-hidden="true" />
					</div>
					<h3 class="state-title">No Heatmap Data Yet</h3>
					<p class="state-copy">Heatmaps will appear once you have visitor activity on your site</p>
					<a href="/admin/settings/tracking" class="setup-link"> Setup Tracking </a>
				</div>
			{:else}
				<div class="heatmap-layout">
					<!-- Page List -->
					<div class="page-list">
						<h3 class="list-heading">Tracked Pages</h3>
						{#each pages as page (page.id)}
							<button
								onclick={() => loadHeatmapData(page)}
								class={{
									'page-card': true,
									'page-card--selected': selectedPage?.id === page.id
								}}
							>
								<div class="page-card-title">{page.title || page.url}</div>
								<div class="page-card-url">{page.url}</div>
								<div class="page-card-meta">
									<span class="page-clicks">{page.clicks.toLocaleString()} clicks</span>
									<span>{page.scroll_depth}% scroll</span>
								</div>
							</button>
						{/each}
					</div>

					<!-- Heatmap Preview -->
					<div class="preview-column">
						{#if selectedPage}
							<div class="preview-panel">
								<div class="preview-header">
									<div>
										<h3 class="preview-title">
											{selectedPage.title || selectedPage.url}
										</h3>
										<p class="preview-url">{selectedPage.url}</p>
									</div>
									<div class="preview-badges">
										<span class="preview-badge preview-badge--rose">
											{selectedPage.clicks.toLocaleString()} clicks
										</span>
										<span class="preview-badge preview-badge--blue">
											{selectedPage.unique_visitors.toLocaleString()} visitors
										</span>
									</div>
								</div>

								<!-- Heatmap Visualization Placeholder -->
								<div class="visualization-placeholder">
									<div class="placeholder-content">
										<div class="state-icon state-icon--rose">
											<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: photo (heatmap placeholder) -->
											<IconPhoto size={32} aria-hidden="true" />
										</div>
										<p class="placeholder-copy">Heatmap visualization will be rendered here</p>
										<p class="placeholder-hint">Requires heatmap rendering library integration</p>
									</div>
								</div>

								<!-- Scroll Depth Visualization -->
								<div class="scroll-depth-panel">
									<h4 class="scroll-depth-title">Scroll Depth Distribution</h4>
									<div class="scroll-depth-list">
										{#each [{ depth: '25%', percentage: 85 }, { depth: '50%', percentage: 62 }, { depth: '75%', percentage: 38 }, { depth: '100%', percentage: 15 }] as scroll (scroll.depth)}
											<div class="scroll-depth-row">
												<span class="scroll-depth-label">{scroll.depth}</span>
												<div class="scroll-depth-track">
													<div
														class="scroll-depth-fill"
														style:--scroll-percent={`${scroll.percentage}%`}
													></div>
												</div>
												<span class="scroll-depth-value">{scroll.percentage}%</span>
											</div>
										{/each}
									</div>
								</div>
							</div>
						{:else}
							<div class="preview-empty">
								<div>
									<div class="state-icon state-icon--muted">
										<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: pointer (select page placeholder) -->
										<IconPointer size={32} aria-hidden="true" />
									</div>
									<p class="state-copy state-copy--compact">Select a page to view its heatmap</p>
								</div>
							</div>
						{/if}
					</div>
				</div>
			{/if}
		{/if}
	</div>
</div>

<style>
	.heatmaps-page {
		min-height: 100%;
		background: linear-gradient(135deg, #020617 0%, #0f172a 50%, #020617 100%);
	}

	.heatmaps-container {
		width: min(100%, 80rem);
		margin: 0 auto;
		padding: 2rem 1rem;
	}

	.heatmaps-header,
	.header-title-group,
	.header-controls,
	.type-switcher,
	.loading-state,
	.page-card-meta,
	.preview-header,
	.preview-badges,
	.visualization-placeholder,
	.scroll-depth-row,
	.preview-empty,
	.state-icon {
		display: flex;
	}

	.heatmaps-header {
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		margin-bottom: 2rem;
	}

	.header-title-group,
	.header-controls,
	.preview-badges {
		align-items: center;
		gap: 1rem;
	}

	.header-icon {
		display: flex;
		width: 3rem;
		height: 3rem;
		align-items: center;
		justify-content: center;
		border-radius: 1rem;
		background: linear-gradient(135deg, #f43f5e, #db2777);
		box-shadow: 0 12px 26px rgba(244, 63, 94, 0.2);
		color: #ffffff;
	}

	.page-title {
		margin: 0;
		color: #ffffff;
		font-size: 1.5rem;
		font-weight: 700;
		letter-spacing: 0;
	}

	.page-subtitle,
	.metric-label,
	.page-card-meta,
	.preview-url,
	.placeholder-copy,
	.scroll-depth-label,
	.scroll-depth-value,
	.state-copy {
		color: #94a3b8;
	}

	.page-subtitle {
		margin: 0;
		font-size: 0.875rem;
	}

	.type-switcher {
		align-items: center;
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 0.75rem;
		background: rgba(30, 41, 59, 0.5);
		padding: 0.25rem;
	}

	.type-button {
		border: 0;
		border-radius: 0.5rem;
		background: transparent;
		color: #94a3b8;
		padding: 0.5rem 1rem;
		font-size: 0.875rem;
		font-weight: 500;
		transition:
			background 0.2s ease,
			box-shadow 0.2s ease,
			color 0.2s ease;
	}

	.type-button:hover,
	.type-button:focus-visible,
	.type-button--active {
		color: #ffffff;
	}

	.type-button--active {
		background: #f43f5e;
		box-shadow: 0 10px 24px rgba(244, 63, 94, 0.25);
	}

	.loading-state {
		align-items: center;
		justify-content: center;
		padding: 5rem 0;
	}

	.spinner {
		width: 2.5rem;
		height: 2.5rem;
		border: 4px solid rgba(244, 63, 94, 0.2);
		border-top-color: transparent;
		border-right-color: #f43f5e;
		border-radius: 999px;
		animation: spin 0.8s linear infinite;
	}

	.spinner--large {
		width: 3rem;
		height: 3rem;
	}

	.stats-grid {
		display: grid;
		grid-template-columns: 1fr;
		gap: 1rem;
		margin-bottom: 2rem;
	}

	.metric-card,
	.panel-state,
	.page-card,
	.preview-panel,
	.preview-empty {
		border: 1px solid rgba(255, 255, 255, 0.1);
		background: rgba(255, 255, 255, 0.05);
		backdrop-filter: blur(24px);
		-webkit-backdrop-filter: blur(24px);
	}

	.metric-card {
		border-radius: 1rem;
		padding: 1.25rem;
	}

	.metric-value {
		margin-bottom: 0.25rem;
		color: #ffffff;
		font-size: 1.875rem;
		font-weight: 700;
		line-height: 1.1;
	}

	.metric-value--rose {
		color: #fb7185;
	}

	.metric-value--amber {
		color: #fbbf24;
	}

	.metric-value--blue {
		color: #60a5fa;
	}

	.metric-label {
		font-size: 0.875rem;
	}

	.panel-state,
	.preview-empty {
		border-radius: 1rem;
		padding: 3rem 1.5rem;
		text-align: center;
	}

	.panel-state--error {
		border-color: rgba(239, 68, 68, 0.2);
		background: rgba(239, 68, 68, 0.1);
	}

	.state-icon {
		width: 4rem;
		height: 4rem;
		align-items: center;
		justify-content: center;
		margin: 0 auto 1rem;
		border-radius: 1rem;
	}

	.state-icon--error {
		background: rgba(239, 68, 68, 0.1);
		color: #f87171;
	}

	.state-icon--rose {
		background: rgba(244, 63, 94, 0.1);
		color: #fb7185;
	}

	.state-icon--muted {
		background: rgba(51, 65, 85, 0.5);
		color: #94a3b8;
	}

	.error-message {
		margin: 0 0 1rem;
		color: #f87171;
	}

	.retry-button,
	.setup-link {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		border-radius: 0.75rem;
		font-weight: 600;
		text-decoration: none;
		transition:
			background 0.2s ease,
			border-color 0.2s ease,
			box-shadow 0.2s ease;
	}

	.retry-button {
		border: 1px solid rgba(239, 68, 68, 0.3);
		background: rgba(239, 68, 68, 0.2);
		color: #f87171;
		padding: 0.625rem 1.25rem;
	}

	.retry-button:hover,
	.retry-button:focus-visible {
		background: rgba(239, 68, 68, 0.3);
	}

	.state-title {
		margin: 0 0 0.5rem;
		color: #ffffff;
		font-size: 1.125rem;
		font-weight: 500;
	}

	.state-copy {
		margin: 0 0 1.5rem;
	}

	.state-copy--compact {
		margin-bottom: 0;
	}

	.setup-link {
		background: linear-gradient(90deg, #f43f5e, #db2777);
		box-shadow: 0 12px 26px rgba(244, 63, 94, 0.25);
		color: #ffffff;
		padding: 0.75rem 1.5rem;
	}

	.setup-link:hover,
	.setup-link:focus-visible {
		background: linear-gradient(90deg, #fb7185, #ec4899);
	}

	.heatmap-layout {
		display: grid;
		grid-template-columns: 1fr;
		gap: 1.5rem;
	}

	.page-list {
		display: grid;
		align-content: start;
		gap: 1rem;
	}

	.list-heading {
		margin: 0;
		color: #94a3b8;
		font-size: 0.875rem;
		font-weight: 500;
		letter-spacing: 0.04em;
		text-transform: uppercase;
	}

	.page-card {
		width: 100%;
		border-radius: 1rem;
		padding: 1rem;
		color: inherit;
		text-align: left;
		transition:
			background 0.2s ease,
			border-color 0.2s ease,
			box-shadow 0.2s ease;
	}

	.page-card:hover,
	.page-card:focus-visible {
		background: rgba(255, 255, 255, 0.1);
	}

	.page-card--selected {
		border-color: rgba(244, 63, 94, 0.5);
		box-shadow: 0 0 0 2px #f43f5e;
	}

	.page-card-title,
	.preview-title {
		color: #ffffff;
		font-weight: 600;
	}

	.page-card-title,
	.page-card-url {
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.page-card-title {
		margin-bottom: 0.25rem;
	}

	.page-card-url {
		margin-bottom: 0.75rem;
		color: #64748b;
		font-size: 0.75rem;
	}

	.page-card-meta {
		align-items: center;
		justify-content: space-between;
		font-size: 0.75rem;
	}

	.page-clicks {
		color: #fb7185;
	}

	.preview-column {
		min-width: 0;
	}

	.preview-panel {
		overflow: hidden;
		border-radius: 1rem;
	}

	.preview-header {
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
		padding: 1.25rem;
	}

	.preview-title {
		margin: 0;
	}

	.preview-url {
		margin: 0.25rem 0 0;
		font-size: 0.75rem;
	}

	.preview-badge {
		border-radius: 0.5rem;
		padding: 0.375rem 0.75rem;
		font-size: 0.75rem;
		font-weight: 500;
	}

	.preview-badge--rose {
		background: rgba(244, 63, 94, 0.2);
		color: #fb7185;
	}

	.preview-badge--blue {
		background: rgba(59, 130, 246, 0.2);
		color: #60a5fa;
	}

	.visualization-placeholder {
		aspect-ratio: 16 / 9;
		align-items: center;
		justify-content: center;
		background: rgba(30, 41, 59, 0.5);
	}

	.placeholder-content {
		text-align: center;
	}

	.placeholder-copy {
		margin: 0;
		font-size: 0.875rem;
	}

	.placeholder-hint {
		margin: 0.25rem 0 0;
		color: #64748b;
		font-size: 0.75rem;
	}

	.scroll-depth-panel {
		border-top: 1px solid rgba(255, 255, 255, 0.1);
		padding: 1.25rem;
	}

	.scroll-depth-title {
		margin: 0 0 1rem;
		color: #cbd5e1;
		font-size: 0.875rem;
		font-weight: 500;
	}

	.scroll-depth-list {
		display: grid;
		gap: 0.5rem;
	}

	.scroll-depth-row {
		align-items: center;
		gap: 0.75rem;
	}

	.scroll-depth-label {
		width: 3rem;
		font-size: 0.75rem;
	}

	.scroll-depth-track {
		height: 0.5rem;
		flex: 1;
		overflow: hidden;
		border-radius: 999px;
		background: #334155;
	}

	.scroll-depth-fill {
		width: var(--scroll-percent);
		height: 100%;
		border-radius: 999px;
		background: linear-gradient(90deg, #f43f5e, #ec4899);
	}

	.scroll-depth-value {
		width: 2.5rem;
		font-size: 0.75rem;
		text-align: right;
	}

	.preview-empty {
		height: 100%;
		align-items: center;
		justify-content: center;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	@media (min-width: 640px) {
		.heatmaps-container {
			padding-right: 1.5rem;
			padding-left: 1.5rem;
		}
	}

	@media (min-width: 768px) {
		.stats-grid {
			grid-template-columns: repeat(4, minmax(0, 1fr));
		}
	}

	@media (min-width: 1024px) {
		.heatmaps-container {
			padding-right: 2rem;
			padding-left: 2rem;
		}

		.heatmap-layout {
			grid-template-columns: minmax(0, 1fr) minmax(0, 2fr);
		}
	}

	@media (max-width: 767px) {
		.heatmaps-header,
		.header-controls,
		.preview-header {
			align-items: flex-start;
			flex-direction: column;
		}

		.type-switcher {
			flex-wrap: wrap;
		}
	}
</style>

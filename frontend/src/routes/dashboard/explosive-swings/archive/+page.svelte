<script lang="ts">
	/**
	 * Weekly Archive - Browse Past Weeks
	 * @version 1.0.0
	 * @standards Apple ICT 7+ | Svelte 5 January 2026 | WCAG 2.1 AA
	 *
	 * Layout matches watchlist card design for consistency
	 */
	import { browser } from '$app/environment';
	import TradingRoomHeader from '$lib/components/dashboard/TradingRoomHeader.svelte';
	import { ROOM_SLUG } from '../constants';

	interface ArchivedWeek {
		id: number;
		weekOf: string;
		weekTitle: string;
		videoTitle: string;
		videoUrl: string;
		thumbnailUrl: string | null;
		duration: string | null;
		alertCount: number;
		tradeCount: number;
		winRate: number | null;
	}

	let weeks = $state<ArchivedWeek[]>([]);
	let isLoading = $state(true);
	let error = $state<string | null>(null);
	let selectedYear = $state(new Date().getFullYear());

	const currentYear = new Date().getFullYear();
	const availableYears = [currentYear, currentYear - 1];

	async function fetchArchivedWeeks() {
		isLoading = true;
		error = null;

		try {
			const response = await fetch(
				`/api/room-content/weekly-video/${ROOM_SLUG}/archive?year=${selectedYear}`,
				{ credentials: 'include' }
			);

			if (!response.ok) {
				throw new Error(`HTTP ${response.status}`);
			}

			const result = await response.json();

			// API returns { success: true, data: [...] }
			if (!result.success) {
				throw new Error(result.error || 'Failed to load archive');
			}

			weeks = (result.data || []).map((w: any) => ({
				id: w.id,
				weekOf: w.week_of,
				weekTitle: w.week_title,
				videoTitle: w.video_title,
				videoUrl: w.video_url,
				thumbnailUrl: w.thumbnail_url,
				duration: w.duration,
				alertCount: w.alert_count || 0,
				tradeCount: w.trade_count || 0,
				winRate: w.win_rate
			}));
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load';
			console.error('Failed to fetch archive:', err);
		} finally {
			isLoading = false;
		}
	}

	// Fetch on mount and refetch when year changes
	$effect(() => {
		if (browser) {
			selectedYear; // track dependency
			fetchArchivedWeeks();
		}
	});

	function formatWeekDate(dateStr: string): string {
		const date = new Date(dateStr);
		return date.toLocaleDateString('en-US', {
			month: 'long',
			day: 'numeric',
			year: 'numeric'
		});
	}

	function getMonthGroup(dateStr: string): string {
		const date = new Date(dateStr);
		return date.toLocaleDateString('en-US', { month: 'long', year: 'numeric' });
	}

	// Group weeks by month
	const groupedWeeks = $derived.by(() => {
		const groups: Record<string, ArchivedWeek[]> = {};

		for (const week of weeks) {
			const month = getMonthGroup(week.weekOf);
			if (!groups[month]) {
				groups[month] = [];
			}
			groups[month].push(week);
		}

		return groups;
	});

	const monthKeys = $derived(
		Object.keys(groupedWeeks).sort((a, b) => {
			return new Date(b).getTime() - new Date(a).getTime();
		})
	);
</script>

<svelte:head>
	<title>Weekly Archive | Explosive Swings</title>
</svelte:head>

<TradingRoomHeader
	roomName="Explosive Swings"
	startHereUrl="/dashboard/explosive-swings/start-here"
	showTradingRoomControls={false}
/>

<main class="archive-page">
	<header class="page-header">
		<div class="header-content">
			<a href="/dashboard/explosive-swings" class="back-link">
				<svg
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
					width="20"
					height="20"
				>
					<path d="M19 12H5M12 19l-7-7 7-7" />
				</svg>
				Back to Dashboard
			</a>
			<h1>Weekly Archive</h1>
			<p>Browse past weeks with video breakdowns, alerts, and trade results</p>
		</div>

		<!-- Year Filter -->
		<div class="year-filter">
			{#each availableYears as year}
				<button
					type="button"
					class="year-btn"
					class:active={selectedYear === year}
					onclick={() => (selectedYear = year)}
				>
					{year}
				</button>
			{/each}
		</div>
	</header>

	<section class="content-section">
		{#if isLoading}
			<div class="loading-state">
				<div class="spinner"></div>
				<p>Loading archive...</p>
			</div>
		{:else if error}
			<div class="error-state" role="alert">
				<svg
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="1.5"
					width="48"
					height="48"
				>
					<circle cx="12" cy="12" r="10" />
					<path d="M12 8v4m0 4h.01" />
				</svg>
				<h3>Unable to load archive</h3>
				<p>{error}</p>
				<button type="button" class="retry-btn" onclick={fetchArchivedWeeks}> Try Again </button>
			</div>
		{:else if weeks.length === 0}
			<div class="empty-state">
				<svg
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="1.5"
					width="48"
					height="48"
				>
					<path
						d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"
					/>
				</svg>
				<h3>No archived weeks for {selectedYear}</h3>
				<p>Check back later or select a different year.</p>
			</div>
		{:else}
			{#each monthKeys as month}
				<div class="month-group">
					<h2 class="month-header">{month}</h2>

					<div class="weeks-grid">
						{#each groupedWeeks[month] as week (week.id)}
							<article class="week-card">
								<!-- Thumbnail -->
								<div class="card-thumbnail">
									{#if week.thumbnailUrl}
										<img src={week.thumbnailUrl} alt="{week.weekTitle} thumbnail" loading="lazy" />
									{:else}
										<div class="thumbnail-placeholder">
											<svg
												viewBox="0 0 24 24"
												fill="none"
												stroke="currentColor"
												stroke-width="1.5"
												width="32"
												height="32"
											>
												<path
													d="M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z"
												/>
											</svg>
										</div>
									{/if}

									{#if week.duration}
										<span class="duration-badge">{week.duration}</span>
									{/if}

									<!-- Play overlay -->
									<a
										href="/dashboard/explosive-swings/video/{week.id}"
										class="play-overlay"
										aria-label="Watch {week.weekTitle}"
									>
										<svg viewBox="0 0 24 24" fill="currentColor" width="48" height="48">
											<path d="M8 5v14l11-7z" />
										</svg>
									</a>
								</div>

								<!-- Content -->
								<div class="card-content">
									<span class="week-date">Week of {formatWeekDate(week.weekOf)}</span>
									<h3 class="week-title">{week.weekTitle}</h3>
									<p class="video-title">{week.videoTitle}</p>

									<!-- Stats -->
									<div class="week-stats">
										<div class="stat">
											<span class="stat-value">{week.alertCount}</span>
											<span class="stat-label">Alerts</span>
										</div>
										<div class="stat">
											<span class="stat-value">{week.tradeCount}</span>
											<span class="stat-label">Trades</span>
										</div>
										{#if week.winRate !== null}
											<div class="stat">
												<span class="stat-value win-rate">{week.winRate}%</span>
												<span class="stat-label">Win Rate</span>
											</div>
										{/if}
									</div>
								</div>

								<!-- Actions -->
								<div class="card-actions">
									<a href="/dashboard/explosive-swings/video/{week.id}" class="action-btn primary">
										Watch Video
									</a>
									<a
										href="/dashboard/explosive-swings/alerts?week={week.weekOf}"
										class="action-btn secondary"
									>
										View Alerts
									</a>
								</div>
							</article>
						{/each}
					</div>
				</div>
			{/each}
		{/if}
	</section>
</main>

<style>
	/* Mobile-first base styles */
	.archive-page {
		background: var(--color-bg-page);
		min-height: 100vh;
		padding: var(--space-4);
	}

	.page-header {
		max-width: 1200px;
		margin: 0 auto var(--space-6);
	}

	.header-content {
		text-align: center;
		margin-bottom: var(--space-4);
	}

	.back-link {
		display: inline-flex;
		align-items: center;
		gap: var(--space-2);
		color: var(--color-text-tertiary);
		text-decoration: none;
		font-size: var(--text-sm);
		font-weight: var(--font-medium);
		margin-bottom: var(--space-4);
		transition: color 0.15s;
	}

	.back-link:hover {
		color: var(--color-brand-primary);
	}

	.page-header h1 {
		font-size: var(--text-2xl);
		font-weight: var(--font-bold);
		color: var(--color-text-primary);
		margin: 0 0 var(--space-2);
		font-family: var(--font-display);
	}

	.page-header p {
		font-size: var(--text-base);
		color: var(--color-text-tertiary);
		margin: 0;
	}

	/* Year Filter */
	.year-filter {
		display: flex;
		justify-content: center;
		gap: var(--space-2);
	}

	.year-btn {
		padding: var(--space-2) var(--space-4);
		background: var(--color-bg-card);
		border: 2px solid var(--color-border-default);
		border-radius: var(--radius-full);
		font-size: var(--text-sm);
		font-weight: var(--font-semibold);
		color: var(--color-text-tertiary);
		cursor: pointer;
		transition: all 0.15s;
	}

	.year-btn:hover {
		border-color: var(--color-brand-primary);
		color: var(--color-brand-primary);
	}

	.year-btn.active {
		background: var(--color-brand-primary);
		border-color: var(--color-brand-primary);
		color: white;
	}

	.content-section {
		max-width: 1200px;
		margin: 0 auto;
	}

	/* Month Groups */
	.month-group {
		margin-bottom: var(--space-8);
	}

	.month-header {
		font-size: var(--text-lg);
		font-weight: var(--font-bold);
		color: var(--color-text-secondary);
		margin: 0 0 var(--space-4);
		padding-bottom: var(--space-2);
		border-bottom: 2px solid var(--color-border-default);
	}

	/* Weeks Grid - Mobile first (1 column) */
	.weeks-grid {
		display: grid;
		grid-template-columns: 1fr;
		gap: var(--space-4);
	}

	/* Week Card - Matches watchlist card layout */
	.week-card {
		background: var(--color-bg-card);
		border: 1px solid var(--color-border-default);
		border-radius: var(--radius-xl);
		overflow: hidden;
		transition:
			box-shadow 0.2s,
			transform 0.2s;
	}

	.week-card:hover {
		box-shadow: var(--shadow-lg);
		transform: translateY(-2px);
	}

	.card-thumbnail {
		position: relative;
		aspect-ratio: 16 / 9;
		background: var(--color-bg-subtle);
		overflow: hidden;
	}

	.card-thumbnail img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.thumbnail-placeholder {
		width: 100%;
		height: 100%;
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--color-text-muted);
	}

	.duration-badge {
		position: absolute;
		bottom: var(--space-2);
		right: var(--space-2);
		background: rgba(0, 0, 0, 0.8);
		color: white;
		font-size: var(--text-xs);
		font-weight: var(--font-semibold);
		padding: 2px 6px;
		border-radius: var(--radius-sm);
	}

	.play-overlay {
		position: absolute;
		inset: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(0, 0, 0, 0.3);
		color: white;
		opacity: 0;
		transition: opacity 0.2s;
	}

	.week-card:hover .play-overlay {
		opacity: 1;
	}

	.card-content {
		padding: var(--space-4);
	}

	.week-date {
		font-size: var(--text-xs);
		font-weight: var(--font-semibold);
		color: var(--color-brand-primary);
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.week-title {
		font-size: var(--text-lg);
		font-weight: var(--font-bold);
		color: var(--color-text-primary);
		margin: var(--space-1) 0;
		font-family: var(--font-display);
	}

	.video-title {
		font-size: var(--text-sm);
		color: var(--color-text-tertiary);
		margin: 0 0 var(--space-3);
		display: -webkit-box;
		-webkit-line-clamp: 2;
		line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	/* Week Stats */
	.week-stats {
		display: flex;
		gap: var(--space-4);
	}

	.stat {
		display: flex;
		flex-direction: column;
	}

	.stat-value {
		font-size: var(--text-lg);
		font-weight: var(--font-bold);
		color: var(--color-text-primary);
		font-variant-numeric: tabular-nums;
	}

	.stat-value.win-rate {
		color: var(--color-profit);
	}

	.stat-label {
		font-size: var(--text-xs);
		color: var(--color-text-tertiary);
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	/* Card Actions */
	.card-actions {
		display: flex;
		gap: var(--space-2);
		padding: var(--space-4);
		padding-top: 0;
	}

	.action-btn {
		flex: 1;
		padding: var(--space-2) var(--space-3);
		border-radius: var(--radius-md);
		font-size: var(--text-sm);
		font-weight: var(--font-semibold);
		text-align: center;
		text-decoration: none;
		transition: all 0.15s;
	}

	.action-btn.primary {
		background: var(--color-brand-primary);
		color: white;
	}

	.action-btn.primary:hover {
		background: var(--color-brand-primary-hover);
	}

	.action-btn.secondary {
		background: var(--color-bg-subtle);
		color: var(--color-text-secondary);
		border: 1px solid var(--color-border-default);
	}

	.action-btn.secondary:hover {
		background: var(--color-bg-muted);
		color: var(--color-text-primary);
	}

	/* Loading/Error/Empty States */
	.loading-state,
	.error-state,
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: var(--space-12);
		text-align: center;
		background: var(--color-bg-card);
		border-radius: var(--radius-xl);
		border: 1px solid var(--color-border-default);
	}

	.spinner {
		width: 40px;
		height: 40px;
		border: 3px solid var(--color-border-default);
		border-top-color: var(--color-brand-primary);
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
		margin-bottom: var(--space-4);
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.loading-state p,
	.empty-state p {
		color: var(--color-text-tertiary);
		margin: 0;
	}

	.error-state svg {
		color: var(--color-loss);
		margin-bottom: var(--space-4);
	}

	.empty-state svg {
		color: var(--color-text-muted);
		margin-bottom: var(--space-4);
	}

	.error-state h3,
	.empty-state h3 {
		font-size: var(--text-lg);
		font-weight: var(--font-semibold);
		color: var(--color-text-primary);
		margin: 0 0 var(--space-2);
	}

	.error-state p {
		color: var(--color-text-tertiary);
		margin: 0 0 var(--space-4);
	}

	.retry-btn {
		background: var(--color-loss);
		color: white;
		border: none;
		padding: var(--space-2) var(--space-4);
		border-radius: var(--radius-md);
		font-size: var(--text-sm);
		font-weight: var(--font-semibold);
		cursor: pointer;
	}

	/* Tablet: 2 columns */
	@media (min-width: 640px) {
		.archive-page {
			padding: var(--space-6);
		}

		.page-header h1 {
			font-size: var(--text-3xl);
		}

		.weeks-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	/* Desktop: 3 columns */
	@media (min-width: 1024px) {
		.archive-page {
			padding: var(--space-8);
		}

		.weeks-grid {
			grid-template-columns: repeat(3, 1fr);
		}
	}
</style>

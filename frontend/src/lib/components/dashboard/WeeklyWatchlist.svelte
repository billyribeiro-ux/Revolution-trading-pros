<!--
	Weekly Watchlist Component
	═══════════════════════════════════════════════════════════════════════════
	
	Apple ICT 11+ Principal Engineer Grade - January 2026
	
	SSR-FIRST: Pre-fetched data for 0ms loading delay.
	Reusable across all dashboard pages with room-aware filtering.
	
	Usage:
	  SSR (recommended - 0ms delay):
	    Pass data prop from +page.server.ts load function
	  
	  Client fallback (auto-fetch if no data prop):
	    Pass roomSlug for room-specific filtering
	
	@version 2.0.0 - January 2026 - SSR-first with auto-fetch fallback
-->
<script lang="ts">
	import { browser } from '$app/environment';
	import { watchlistApi, type WatchlistItem } from '$lib/api/watchlist';

	interface WatchlistData {
		id: number;
		slug: string;
		title: string;
		trader: string;
		weekOf: string;
		video: { src: string; poster: string; title: string };
		description?: string;
	}

	interface Props {
		/** SSR pre-fetched data - pass this for 0ms loading */
		data?: WatchlistData | null;
		/** Room slug for filtering (used if no data prop) */
		roomSlug?: string;
		/** Custom href override */
		href?: string;
		/** Additional CSS class */
		className?: string;
	}

	let { data = null, roomSlug, href, className = '' }: Props = $props();

	// Client-side state (only used if no SSR data)
	let clientData = $state<WatchlistData | null>(null);
	let isLoading = $state(false);
	let hasError = $state(false);

	// Resolved watchlist data: SSR data takes priority, then client fetch
	const watchlist = $derived(data || clientData);

	// Computed display values
	const displayTitle = $derived(watchlist?.title || 'Weekly Watchlist');
	const displayTrader = $derived(watchlist?.trader || 'Trading Team');
	const displayWeekOf = $derived(formatWeekOf(watchlist?.weekOf));
	const displayImage = $derived(
		watchlist?.video?.poster ||
			'https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/TG-Watchlist-Rundown.jpg'
	);
	const displayHref = $derived(
		href || (watchlist?.slug ? `/watchlist/${watchlist.slug}` : '/watchlist/latest')
	);

	// Format week_of date
	function formatWeekOf(dateStr?: string): string {
		if (!dateStr) return 'This Week';
		try {
			const date = new Date(dateStr);
			return date.toLocaleDateString('en-US', {
				month: 'long',
				day: 'numeric',
				year: 'numeric'
			});
		} catch {
			return dateStr;
		}
	}

	// Client-side fetch fallback (only if no SSR data)
	$effect(() => {
		if (!browser || data) return; // Skip if SSR data exists

		isLoading = true;
		hasError = false;

		watchlistApi
			.getLatest(roomSlug)
			.then((response) => {
				if (response.success && response.data) {
					clientData = response.data;
				}
			})
			.catch((err) => {
				console.warn('[WeeklyWatchlist] Failed to fetch:', err);
				hasError = true;
			})
			.finally(() => {
				isLoading = false;
			});
	});
</script>

{#if isLoading}
	<!-- Loading skeleton for client-side fetch -->
	<div class="weekly-watchlist-section {className}">
		<div class="row">
			<div class="col-left">
				<div class="skeleton-title"></div>
				<div class="skeleton-subtitle"></div>
				<div class="skeleton-text"></div>
				<div class="skeleton-button"></div>
			</div>
			<div class="col-right desktop-only">
				<div class="skeleton-image"></div>
			</div>
		</div>
	</div>
{:else if hasError && !watchlist}
	<!-- Error state - hide section gracefully -->
{:else}
	<!-- Main content -->
	<div class="weekly-watchlist-section {className}">
		<div class="row">
			<div class="col-left">
				<h2 class="section-title-alt section-title-alt--underline">Weekly Watchlist</h2>
				<div class="mobile-image">
					<a href={displayHref}>
						<img
							src={displayImage}
							alt="Weekly Watchlist"
							class="u--border-radius"
							loading="eager"
						/>
					</a>
				</div>
				<h4 class="h5 u--font-weight-bold">{displayTitle}</h4>
				<div class="u--hide-read-more">
					<p>Week of {displayWeekOf}.</p>
				</div>
				<a href={displayHref} class="btn btn-tiny btn-default">Watch Now</a>
			</div>
			<div class="col-right desktop-only">
				<a href={displayHref}>
					<img src={displayImage} alt="Weekly Watchlist" class="u--border-radius" loading="eager" />
				</a>
			</div>
		</div>
	</div>
{/if}

<style>
	/* Weekly Watchlist Section */
	.weekly-watchlist-section {
		margin-top: 40px;
		padding-top: 40px;
		border-top: 1px solid #e6e6e6;
	}

	.row {
		display: flex;
		flex-wrap: wrap;
		margin: 0 -15px;
	}

	.col-left {
		flex: 0 0 100%;
		max-width: 100%;
		padding: 0 15px;
		margin-bottom: 20px;
	}

	.col-right {
		flex: 0 0 100%;
		max-width: 100%;
		padding: 0 15px;
		display: none;
	}

	.section-title-alt {
		font-size: 24px;
		font-weight: 700;
		color: #333;
		margin: 0 0 20px;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
	}

	.section-title-alt--underline {
		padding-bottom: 15px;
		border-bottom: 3px solid #f69532;
	}

	.h5 {
		font-size: 18px;
		font-weight: 700;
		color: #333;
		margin: 0 0 10px;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
	}

	.u--font-weight-bold {
		font-weight: 700;
	}

	.u--border-radius {
		border-radius: 8px;
		width: 100%;
		height: auto;
	}

	.u--hide-read-more p {
		color: #666;
		font-size: 14px;
		margin: 0 0 15px;
	}

	.mobile-image {
		display: block;
		margin-bottom: 15px;
	}

	.desktop-only {
		display: none;
	}

	.btn {
		display: inline-block;
		padding: 10px 20px;
		font-size: 14px;
		font-weight: 600;
		text-align: center;
		text-decoration: none;
		border-radius: 4px;
		transition: all 0.2s ease;
		cursor: pointer;
	}

	.btn-default {
		background: #143e59;
		color: #fff;
		border: 1px solid #143e59;
	}

	.btn-default:hover {
		background: #0c2638;
		border-color: #0c2638;
	}

	.btn-tiny {
		padding: 8px 16px;
		font-size: 13px;
		height: 40px;
		display: inline-flex;
		align-items: center;
		justify-content: center;
	}

	/* Tablet breakpoint */
	@media (min-width: 768px) {
		.col-left {
			flex: 0 0 50%;
			max-width: 50%;
			margin-bottom: 0;
		}

		.col-right {
			flex: 0 0 50%;
			max-width: 50%;
		}

		.mobile-image {
			display: none;
		}

		.desktop-only {
			display: block;
		}
	}

	/* Desktop breakpoint */
	@media (min-width: 992px) {
		.col-left {
			flex: 0 0 41.666667%;
			max-width: 41.666667%;
		}

		.col-right {
			flex: 0 0 58.333333%;
			max-width: 58.333333%;
		}
	}

	/* Skeleton Loading States */
	.skeleton-title {
		height: 32px;
		width: 200px;
		background: linear-gradient(90deg, #f0f0f0 25%, #e0e0e0 50%, #f0f0f0 75%);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
		border-radius: 4px;
		margin-bottom: 20px;
	}

	.skeleton-subtitle {
		height: 22px;
		width: 280px;
		background: linear-gradient(90deg, #f0f0f0 25%, #e0e0e0 50%, #f0f0f0 75%);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
		border-radius: 4px;
		margin-bottom: 10px;
	}

	.skeleton-text {
		height: 18px;
		width: 180px;
		background: linear-gradient(90deg, #f0f0f0 25%, #e0e0e0 50%, #f0f0f0 75%);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
		border-radius: 4px;
		margin-bottom: 15px;
	}

	.skeleton-button {
		height: 36px;
		width: 120px;
		background: linear-gradient(90deg, #f0f0f0 25%, #e0e0e0 50%, #f0f0f0 75%);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
		border-radius: 4px;
	}

	.skeleton-image {
		width: 100%;
		padding-top: 56.25%;
		background: linear-gradient(90deg, #f0f0f0 25%, #e0e0e0 50%, #f0f0f0 75%);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
		border-radius: 8px;
	}

	@keyframes shimmer {
		0% {
			background-position: -200% 0;
		}
		100% {
			background-position: 200% 0;
		}
	}
</style>

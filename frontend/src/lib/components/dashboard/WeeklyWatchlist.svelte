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

	let props: Props = $props();

	// Derived props with defaults
	let data = $derived(props.data ?? null);
	let roomSlug = $derived(props.roomSlug);
	let href = $derived(props.href);
	let className = $derived(props.className ?? '');

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
	/* ═══════════════════════════════════════════════════════════════════════════
	 * WEEKLY WATCHLIST - 2026 Mobile-First Responsive Design
	 * ═══════════════════════════════════════════════════════════════════════════
	 * Breakpoints: xs(360px), sm(640px), md(768px), lg(1024px), xl(1280px)
	 * Touch Targets: 44x44px minimum
	 * Safe Areas: env(safe-area-inset-*) for notched devices
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* Mobile-First Base */
	.weekly-watchlist-section {
		margin-top: 24px;
		padding-top: 24px;
		border-top: 1px solid #e6e6e6;
	}

	.row {
		display: flex;
		flex-direction: column;
		gap: 16px;
	}

	.col-left {
		width: 100%;
	}

	.col-right {
		width: 100%;
		display: none;
	}

	.section-title-alt {
		font-size: 20px;
		font-weight: 700;
		color: #333;
		margin: 0 0 16px;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
		line-height: 1.3;
	}

	.section-title-alt--underline {
		padding-bottom: 12px;
		border-bottom: 3px solid #f69532;
	}

	.h5 {
		font-size: 16px;
		font-weight: 700;
		color: #333;
		margin: 0 0 8px;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
		line-height: 1.3;
	}

	.u--font-weight-bold {
		font-weight: 700;
	}

	.u--border-radius {
		border-radius: 8px;
		width: 100%;
		height: auto;
		aspect-ratio: 16 / 9;
		object-fit: cover;
	}

	.u--hide-read-more p {
		color: #666;
		font-size: 14px;
		margin: 0 0 12px;
		line-height: 1.5;
	}

	.mobile-image {
		display: block;
		margin-bottom: 12px;
	}

	.mobile-image a {
		display: block;
	}

	.desktop-only {
		display: none;
	}

	/* Touch-Friendly Button (44px minimum) */
	.btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		min-height: 44px;
		min-width: 44px;
		padding: 12px 20px;
		font-size: 14px;
		font-weight: 600;
		text-align: center;
		text-decoration: none;
		border-radius: 8px;
		transition: all 0.2s ease;
		cursor: pointer;
		-webkit-tap-highlight-color: transparent;
		touch-action: manipulation;
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

	.btn-default:focus-visible {
		outline: 2px solid #143e59;
		outline-offset: 2px;
	}

	.btn-tiny {
		padding: 10px 18px;
		font-size: 13px;
		min-height: 44px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * RESPONSIVE BREAKPOINTS - Progressive Enhancement
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* xs: 360px+ - Small phones */
	@media (min-width: 360px) {
		.weekly-watchlist-section {
			margin-top: 28px;
			padding-top: 28px;
		}

		.row {
			gap: 20px;
		}
	}

	/* sm: 640px+ - Large phones / small tablets */
	@media (min-width: 640px) {
		.weekly-watchlist-section {
			margin-top: 32px;
			padding-top: 32px;
		}

		.section-title-alt {
			font-size: 22px;
		}

		.h5 {
			font-size: 17px;
		}
	}

	/* md: 768px+ - Tablets - Two column layout */
	@media (min-width: 768px) {
		.weekly-watchlist-section {
			margin-top: 36px;
			padding-top: 36px;
		}

		.row {
			flex-direction: row;
			flex-wrap: nowrap;
			gap: 24px;
		}

		.col-left {
			flex: 0 0 50%;
			max-width: 50%;
		}

		.col-right {
			flex: 0 0 50%;
			max-width: 50%;
			display: block;
		}

		.mobile-image {
			display: none;
		}

		.desktop-only {
			display: block;
		}

		.section-title-alt {
			font-size: 24px;
			margin-bottom: 20px;
		}

		.h5 {
			font-size: 18px;
			margin-bottom: 10px;
		}
	}

	/* lg: 1024px+ - Desktop */
	@media (min-width: 1024px) {
		.weekly-watchlist-section {
			margin-top: 40px;
			padding-top: 40px;
		}

		.col-left {
			flex: 0 0 41.666667%;
			max-width: 41.666667%;
		}

		.col-right {
			flex: 0 0 58.333333%;
			max-width: 58.333333%;
		}

		.row {
			gap: 30px;
		}
	}

	/* High contrast / reduced motion preferences */
	@media (prefers-reduced-motion: reduce) {
		.btn {
			transition: none;
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

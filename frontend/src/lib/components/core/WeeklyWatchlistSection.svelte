<!--
═══════════════════════════════════════════════════════════════════════════════════
WEEKLY WATCHLIST SECTION COMPONENT - Svelte 5 / SvelteKit (Nov/Dec 2025)
═══════════════════════════════════════════════════════════════════════════════════

PURPOSE:
This component renders the featured Weekly Watchlist section on the dashboard.
It displays a responsive two-column layout with video thumbnail and description.
The design matches the Simpler Trading reference (core file lines 2951-2975).

SVELTE 5 PATTERNS USED:
- $props() rune: Type-safe prop declaration with defaults
- $derived rune: Computed date formatting
- Responsive design: CSS Grid with mobile-first approach
- Conditional rendering: {#if} blocks for loading states

LAYOUT:
┌─────────────────────────────────────────────────────────────────┐
│ ┌─────────────────────┐ ┌─────────────────────────────────────┐ │
│ │ WEEKLY WATCHLIST    │ │                                     │ │
│ │ ─────────────────   │ │         [VIDEO THUMBNAIL]           │ │
│ │ Title               │ │                                     │ │
│ │ Description...      │ │                                     │ │
│ │ [Watch Now]         │ └─────────────────────────────────────┘ │
│ └─────────────────────┘                                         │
└─────────────────────────────────────────────────────────────────┘

@version 3.0.0 - Svelte 5 Runes
@updated December 2025
═══════════════════════════════════════════════════════════════════════════════════
-->
<script lang="ts">
	/**
	 * ─────────────────────────────────────────────────────────────────────────────
	 * IMPORTS
	 * ─────────────────────────────────────────────────────────────────────────────
	 */
	import type { WeeklyWatchlist, WeeklyWatchlistSectionProps } from './types';

	/**
	 * ─────────────────────────────────────────────────────────────────────────────
	 * PROPS - Svelte 5 $props() Rune
	 * ─────────────────────────────────────────────────────────────────────────────
	 *
	 * This component can receive watchlist data from its parent,
	 * or display default placeholder content when data isn't available.
	 */
	let {
		// Optional: Watchlist data from API
		watchlist = undefined,
		// Optional: Loading state indicator
		isLoading = false
	}: WeeklyWatchlistSectionProps = $props();

	/**
	 * ─────────────────────────────────────────────────────────────────────────────
	 * DEFAULT DATA - Fallback when no watchlist is provided
	 * ─────────────────────────────────────────────────────────────────────────────
	 *
	 * Uses static data matching the WordPress reference.
	 * In production, this would be replaced with API data.
	 */
	const defaultWatchlist: WeeklyWatchlist = {
		id: 'default',
		title: 'Weekly Watchlist with TG Watkins',
		subtitle: 'TG Watkins',
		description: 'Week of December 22, 2025.',
		imageUrl: 'https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/TG-Watchlist-Rundown.jpg',
		videoUrl: '/watchlist/latest',
		publishedAt: new Date().toISOString()
	};

	/**
	 * ─────────────────────────────────────────────────────────────────────────────
	 * DERIVED STATE - Svelte 5 $derived Rune
	 * ─────────────────────────────────────────────────────────────────────────────
	 *
	 * Compute the active watchlist data (from props or default)
	 */

	// Use provided watchlist or fall back to default
	let activeWatchlist = $derived(watchlist || defaultWatchlist);

	// Format the video URL for linking
	let videoLink = $derived(activeWatchlist.videoUrl);

	// Ensure image has alt text for accessibility
	let imageAlt = $derived(`${activeWatchlist.title} thumbnail`);
</script>

<!--
═══════════════════════════════════════════════════════════════════════════════════
TEMPLATE - Weekly Watchlist Section
═══════════════════════════════════════════════════════════════════════════════════

Matches WordPress structure from core file:
<div class="dashboard__content-section u--background-color-white">
  <section>
    <div class="row">...</div>
  </section>
</div>
-->
<div class="dashboard__content-section u--background-color-white">
	<section class="weekly-watchlist">
		<!--
		─────────────────────────────────────────────────────────────────────────────
		LOADING STATE
		Shows skeleton loader while data is being fetched
		─────────────────────────────────────────────────────────────────────────────
		-->
		{#if isLoading}
			<div class="loading-state">
				<div class="skeleton skeleton--title"></div>
				<div class="skeleton skeleton--text"></div>
				<div class="skeleton skeleton--button"></div>
			</div>
		{:else}
			<!--
			─────────────────────────────────────────────────────────────────────────────
			TWO-COLUMN LAYOUT
			Left: Text content | Right: Video thumbnail (visible on desktop)
			─────────────────────────────────────────────────────────────────────────────
			-->
			<div class="row">
				<!--
				LEFT COLUMN - Text Content
				Contains title, description, and CTA button
				-->
				<div class="col-sm-6 col-lg-5">
					<!-- Section Title with Underline -->
					<h2 class="section-title-alt section-title-alt--underline">
						Weekly Watchlist
					</h2>

					<!--
					Mobile Image - Only visible on small screens
					Uses d-lg-none utility class pattern from Bootstrap
					-->
					<div class="mobile-image">
						<a href={videoLink}>
							<img
								src={activeWatchlist.imageUrl}
								alt={imageAlt}
								class="u--border-radius"
								loading="lazy"
							/>
						</a>
					</div>

					<!-- Video Title -->
					<h4 class="h5 u--font-weight-bold">
						{activeWatchlist.title}
					</h4>

					<!-- Description / Week Info -->
					<div class="watchlist-description">
						<p>{activeWatchlist.description}</p>
					</div>

					<!-- Call-to-Action Button -->
					<a href={videoLink} class="btn btn-tiny btn-default">
						Watch Now
					</a>
				</div>

				<!--
				RIGHT COLUMN - Desktop Video Thumbnail
				Only visible on large screens (lg breakpoint and up)
				-->
				<div class="col-sm-6 col-lg-7 desktop-image">
					<a href={videoLink}>
						<img
							src={activeWatchlist.imageUrl}
							alt={imageAlt}
							class="u--border-radius"
							loading="lazy"
						/>
					</a>
				</div>
			</div>
		{/if}
	</section>
</div>

<!--
═══════════════════════════════════════════════════════════════════════════════════
STYLES - Weekly Watchlist Section CSS
═══════════════════════════════════════════════════════════════════════════════════
-->
<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   SECTION CONTAINER
	   White background with consistent padding
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__content-section {
		padding: 30px 40px;
		background-color: #fff;
		margin-bottom: 20px;
		overflow-x: auto;
		overflow-y: hidden;
	}

	.u--background-color-white {
		background-color: #fff !important;
	}

	.weekly-watchlist {
		margin: 0;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   SECTION TITLE - Blue uppercase with underline
	   Matches WordPress exactly (core file lines 3524-3548)
	   ═══════════════════════════════════════════════════════════════════════════ */

	.section-title-alt {
		color: #0984ae;
		font-weight: 700;
		font-size: 11px;
		letter-spacing: 1px;
		margin-bottom: 30px;
		text-transform: uppercase;
		font-family: 'Open Sans', sans-serif;
	}

	.section-title-alt--underline {
		padding-bottom: 30px;
		position: relative;
	}

	/* Decorative underline using ::after pseudo-element */
	.section-title-alt--underline::after {
		background-color: #e8e8e8;
		bottom: 2px;
		content: '';
		display: block;
		height: 2px;
		position: absolute;
		left: 0;
		width: 50px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ROW LAYOUT - Flexbox grid with negative margins
	   Matches Bootstrap 4/5 grid system
	   ═══════════════════════════════════════════════════════════════════════════ */

	.row {
		display: flex;
		flex-wrap: wrap;
		margin-right: -15px;
		margin-left: -15px;
	}

	/* Column base styles */
	.col-sm-6,
	.col-lg-5,
	.col-lg-7 {
		position: relative;
		width: 100%;
		padding-right: 15px;
		padding-left: 15px;
	}

	/* Small screens and up - 50% width */
	@media (min-width: 641px) {
		.col-sm-6 {
			flex: 0 0 50%;
			max-width: 50%;
		}
	}

	/* Large screens - 5/12 and 7/12 split */
	@media (min-width: 992px) {
		.col-lg-5 {
			flex: 0 0 41.666667%;
			max-width: 41.666667%;
		}

		.col-lg-7 {
			flex: 0 0 58.333333%;
			max-width: 58.333333%;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE IMAGE HANDLING
	   Mobile: Show inline image above text
	   Desktop: Show image in right column
	   ═══════════════════════════════════════════════════════════════════════════ */

	/* Mobile image - visible by default, hidden on large screens */
	.mobile-image {
		display: block;
		padding-bottom: 0.5rem;
	}

	@media (min-width: 992px) {
		.mobile-image {
			display: none;
		}
	}

	/* Desktop image - hidden by default, visible on large screens */
	.desktop-image {
		display: none;
	}

	@media (min-width: 992px) {
		.desktop-image {
			display: block;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   IMAGE STYLING
	   ═══════════════════════════════════════════════════════════════════════════ */

	.weekly-watchlist img {
		width: 100%;
		height: auto;
		display: block;
	}

	.u--border-radius {
		border-radius: 8px !important;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   TYPOGRAPHY
	   ═══════════════════════════════════════════════════════════════════════════ */

	.h5 {
		font-size: 18px;
		font-weight: 600;
		font-family: 'Open Sans', sans-serif;
		line-height: 1.4;
		margin: 0 0 15px 0;
		color: #333;
	}

	.u--font-weight-bold {
		font-weight: 700 !important;
	}

	.watchlist-description {
		margin-bottom: 20px;
	}

	.watchlist-description p {
		margin: 0;
		color: #666;
		font-size: 14px;
		line-height: 1.6;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   BUTTON STYLES
	   ═══════════════════════════════════════════════════════════════════════════ */

	.btn {
		display: inline-block;
		padding: 10px 20px;
		text-decoration: none;
		border: none;
		border-radius: 5px;
		font-weight: 700;
		font-family: 'Open Sans', sans-serif;
		cursor: pointer;
		transition: all 0.2s ease-in-out;
		font-size: 14px;
		text-align: center;
	}

	.btn-tiny {
		padding: 5px 10px;
		font-size: 11px;
	}

	.btn-default {
		background: #fff;
		color: #333;
		border: 1px solid #ccc;
		box-shadow: none;
	}

	.btn-default:hover {
		background: #e8e8e8;
		border-color: #ccc;
	}

	.btn-default:focus {
		outline: 2px solid #0984ae;
		outline-offset: 2px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   LOADING STATE - Skeleton loaders
	   ═══════════════════════════════════════════════════════════════════════════ */

	.loading-state {
		padding: 20px 0;
	}

	.skeleton {
		background: linear-gradient(90deg, #f0f0f0 25%, #e0e0e0 50%, #f0f0f0 75%);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
		border-radius: 4px;
	}

	.skeleton--title {
		width: 200px;
		height: 20px;
		margin-bottom: 30px;
	}

	.skeleton--text {
		width: 100%;
		max-width: 300px;
		height: 60px;
		margin-bottom: 20px;
	}

	.skeleton--button {
		width: 100px;
		height: 36px;
	}

	@keyframes shimmer {
		0% {
			background-position: 200% 0;
		}
		100% {
			background-position: -200% 0;
		}
	}
</style>

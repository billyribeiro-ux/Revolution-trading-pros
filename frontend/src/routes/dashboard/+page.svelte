<!--
═══════════════════════════════════════════════════════════════════════════════════
MEMBER DASHBOARD PAGE - Svelte 5 / SvelteKit (Nov/Dec 2025)
═══════════════════════════════════════════════════════════════════════════════════

PURPOSE:
This is the main Member Dashboard page that displays after user login.
It shows the user's active memberships, trading rooms, tools, and weekly watchlist.
All data is fetched from the API and rendered dynamically.

PRINCIPAL ENGINEER ICT 11+ PATTERN:
- Auth guard is handled by the parent +layout.svelte
- This page can safely assume authentication is complete
- Uses composition of reusable core components
- Data-driven rendering - no hardcoded membership values

SVELTE 5 RUNES USED:
- $state(): Reactive state management for loading, data, and errors
- $derived(): Computed values that auto-update
- $effect(): Side effects for data fetching
- $props(): Not used here (page component has no props)

COMPONENT COMPOSITION:
┌─────────────────────────────────────────────────────────────────────────────────┐
│ DashboardHeader                                                                 │
├─────────────────────────────────────────────────────────────────────────────────┤
│ MembershipCardsGrid (memberships)                                               │
├─────────────────────────────────────────────────────────────────────────────────┤
│ Tools Section (ToolCards)                                                       │
├─────────────────────────────────────────────────────────────────────────────────┤
│ WeeklyWatchlistSection                                                          │
├─────────────────────────────────────────────────────────────────────────────────┤
│ Content Sidebar (empty on main dashboard)                                       │
└─────────────────────────────────────────────────────────────────────────────────┘

@version 3.0.0 - Svelte 5 Runes with Core Components
@updated December 2025
═══════════════════════════════════════════════════════════════════════════════════
-->
<script lang="ts">
	/**
	 * ─────────────────────────────────────────────────────────────────────────────
	 * IMPORTS
	 * ─────────────────────────────────────────────────────────────────────────────
	 *
	 * Svelte 5 lifecycle import (onMount is still used for initial data fetch)
	 */
	import { onMount } from 'svelte';

	/**
	 * API import for fetching user membership data
	 * This function returns all memberships, trading rooms, courses, etc.
	 */
	import {
		getUserMemberships,
		type UserMembership,
		type UserMembershipsResponse
	} from '$lib/api/user-memberships';

	/**
	 * Core component imports using barrel export
	 * This is the recommended pattern for importing multiple components
	 */
	import {
		DashboardHeader,
		MembershipCardsGrid,
		ToolCard,
		WeeklyWatchlistSection,
		type Membership,
		type TradingRoom
	} from '$lib/components/core';

	/**
	 * ─────────────────────────────────────────────────────────────────────────────
	 * STATE - Svelte 5 $state() Rune
	 * ─────────────────────────────────────────────────────────────────────────────
	 *
	 * The $state() rune creates reactive state variables. When these values
	 * change, Svelte automatically updates the DOM.
	 *
	 * IMPORTANT: $state() replaces `let variable = value` for reactive state
	 * in Svelte 5. Variables declared with plain `let` are NOT reactive.
	 *
	 * @see https://svelte.dev/docs/svelte/$state
	 */

	// Loading state indicator - true while fetching data
	let loading = $state(true);

	// Error message to display if API call fails
	let error = $state<string | null>(null);

	// Main memberships data from API
	let membershipsData = $state<UserMembershipsResponse | null>(null);

	// Trading room dropdown open/close state
	let dropdownOpen = $state(false);

	/**
	 * ─────────────────────────────────────────────────────────────────────────────
	 * DERIVED STATE - Svelte 5 $derived() Rune
	 * ─────────────────────────────────────────────────────────────────────────────
	 *
	 * The $derived() rune creates computed values that automatically update
	 * when their dependencies change. This replaces the `$: derived = ...`
	 * reactive statement syntax from Svelte 4.
	 *
	 * Benefits:
	 * - Cleaner, more explicit syntax
	 * - Better TypeScript inference
	 * - Dependency tracking is automatic
	 *
	 * @see https://svelte.dev/docs/svelte/$derived
	 */

	// Extract memberships array, defaulting to empty array
	let memberships = $derived<Membership[]>(
		membershipsData?.memberships?.map(m => ({
			id: m.id,
			name: m.name,
			slug: m.slug,
			type: m.type as any,
			icon: m.icon,
			accessUrl: m.accessUrl,
			roomLabel: m.roomLabel,
			dashboardUrl: `/dashboard/${m.slug}`,
			isActive: true
		})) || []
	);

	// Extract trading rooms for the dropdown
	let tradingRooms = $derived<TradingRoom[]>(
		membershipsData?.tradingRooms?.map(r => ({
			id: r.id,
			name: r.name,
			slug: r.slug,
			ssoUrl: r.accessUrl || `/dashboard/${r.slug}`,
			roomLabel: r.roomLabel || r.name,
			icon: r.icon
		})) || []
	);

	// Check if user has any memberships
	let hasMemberships = $derived(memberships.length > 0);

	// Check if user has trading rooms (for dropdown display)
	let hasTradingRooms = $derived(tradingRooms.length > 0);

	/**
	 * ─────────────────────────────────────────────────────────────────────────────
	 * LIFECYCLE - Data Fetching
	 * ─────────────────────────────────────────────────────────────────────────────
	 *
	 * onMount runs after the component is first rendered to the DOM.
	 * We use it here for initial data fetching.
	 *
	 * Note: In Svelte 5, you could also use $effect() for data fetching,
	 * but onMount is still preferred for one-time initialization.
	 *
	 * PRINCIPAL ENGINEER PATTERN:
	 * The parent +layout.svelte guarantees auth is initialized before
	 * this component mounts, so we don't need to wait for auth here.
	 */
	onMount(async () => {
		console.log('[Dashboard] 🚀 Page mounted, fetching memberships...');

		try {
			// Fetch user memberships from API
			membershipsData = await getUserMemberships();

			// Log successful fetch for debugging
			console.log('[Dashboard] ✅ Memberships loaded:', {
				total: membershipsData?.memberships?.length || 0,
				tradingRooms: membershipsData?.tradingRooms?.length || 0,
				courses: membershipsData?.courses?.length || 0,
				indicators: membershipsData?.indicators?.length || 0,
				weeklyWatchlist: membershipsData?.weeklyWatchlist?.length || 0,
				premiumReports: membershipsData?.premiumReports?.length || 0
			});

			// Log trading rooms specifically
			if (membershipsData?.tradingRooms?.length) {
				console.log('[Dashboard] 🎯 Trading Rooms:',
					membershipsData.tradingRooms.map(r => r.name)
				);
			}

		} catch (err) {
			// Handle API errors gracefully
			console.error('[Dashboard] ❌ Failed to load memberships:', err);
			error = 'Failed to load memberships. Please try again.';
		} finally {
			// Always set loading to false when done
			loading = false;
			console.log('[Dashboard] ✅ Initialization complete');
		}
	});

	/**
	 * ─────────────────────────────────────────────────────────────────────────────
	 * EVENT HANDLERS
	 * ─────────────────────────────────────────────────────────────────────────────
	 *
	 * Pure functions for handling user interactions.
	 * Svelte 5 uses standard JavaScript event syntax (onclick, not on:click).
	 */

	/**
	 * Toggle the trading room dropdown
	 */
	function handleDropdownToggle(): void {
		dropdownOpen = !dropdownOpen;
	}

	/**
	 * Close dropdown when clicking outside
	 * Called from window click handler
	 */
	function handleWindowClick(): void {
		if (dropdownOpen) {
			dropdownOpen = false;
		}
	}

	/**
	 * Handle retry button click (in error state)
	 * Clears error and reloads the page
	 */
	async function handleRetry(): Promise<void> {
		console.log('[Dashboard] 🔄 Retry triggered...');
		loading = true;
		error = null;

		try {
			// Force cache bypass on retry
			membershipsData = await getUserMemberships({ skipCache: true });
		} catch (err) {
			console.error('[Dashboard] ❌ Retry failed:', err);
			error = 'Failed to load memberships. Please try again.';
		} finally {
			loading = false;
		}
	}

	/**
	 * Handle membership card dashboard click
	 * Logs for analytics and allows default navigation
	 */
	function handleDashboardClick(membership: Membership): void {
		console.log('[Dashboard] 📊 Dashboard clicked:', membership.name);
		// Analytics tracking could go here
	}

	/**
	 * Handle membership card access click (trading room, etc.)
	 * Logs for analytics and allows default navigation
	 */
	function handleAccessClick(membership: Membership): void {
		console.log('[Dashboard] 🚀 Access clicked:', membership.name);
		// Analytics tracking could go here
	}
</script>

<!--
═══════════════════════════════════════════════════════════════════════════════════
TEMPLATE - Dashboard Page Structure
═══════════════════════════════════════════════════════════════════════════════════

The page follows the Simpler Trading layout:
1. Header with title and trading room dropdown
2. Memberships grid (if any)
3. Latest Updates (if no memberships)
4. Tools section
5. Weekly Watchlist feature section
6. Content sidebar (empty on main dashboard)
-->

<!-- Window click handler for closing dropdown -->
<svelte:window onclick={handleWindowClick} />

<!--
─────────────────────────────────────────────────────────────────────────────────
SECTION 1: DASHBOARD HEADER
Contains page title, Trading Room Rules link, and Enter Trading Room dropdown
─────────────────────────────────────────────────────────────────────────────────
-->
<DashboardHeader
	title="Member Dashboard"
	tradingRooms={tradingRooms}
	showRulesLink={hasTradingRooms}
/>

<!--
─────────────────────────────────────────────────────────────────────────────────
DASHBOARD CONTENT WRAPPER
Contains main content area and optional sidebar
─────────────────────────────────────────────────────────────────────────────────
-->
<div class="dashboard__content">
	<div class="dashboard__content-main">

		<!--
		─────────────────────────────────────────────────────────────────────────
		SECTION 2: MEMBERSHIPS GRID
		Shows user's active memberships as cards
		Handles loading, error, and empty states internally
		─────────────────────────────────────────────────────────────────────────
		-->
		<MembershipCardsGrid
			{memberships}
			isLoading={loading}
			{error}
			title="Memberships"
			showActions={true}
			onRetry={handleRetry}
			onDashboardClick={handleDashboardClick}
			onAccessClick={handleAccessClick}
		/>

		<!--
		─────────────────────────────────────────────────────────────────────────
		SECTION 3: LATEST UPDATES (Conditional)
		Only shown when user has no memberships
		Displays placeholder content encouraging signup
		─────────────────────────────────────────────────────────────────────────
		-->
		{#if !loading && !error && !hasMemberships}
			<section class="dashboard__content-section">
				<h2 class="section-title u--margin-top-20">Latest Updates</h2>
				<div class="article-cards row flex-grid">
					<!--
					Placeholder article cards
					In production, these would be fetched from a blog/content API
					-->
					<div class="col-xs-12 col-sm-6 col-md-6 col-xl-4 flex-grid-item">
						<article class="article-card">
							<figure
								class="article-card__image"
								style="background-image: url('https://images.unsplash.com/photo-1611974789855-9c2a0a7236a3?w=800&h=600&fit=crop');"
							>
								<img
									src="https://images.unsplash.com/photo-1611974789855-9c2a0a7236a3?w=800&h=600&fit=crop"
									alt="Trading Update"
									loading="lazy"
								/>
							</figure>
							<div class="article-card__type">
								<span class="label label--info">Daily Video</span>
							</div>
							<h4 class="h5 article-card__title">
								<a href="/blog">Welcome to Revolution Trading Pros</a>
							</h4>
							<span class="article-card__meta">
								<small>Latest market insights and trading education</small>
							</span>
							<a href="/pricing" class="btn btn-tiny btn-default">View Plans</a>
						</article>
					</div>

					<div class="col-xs-12 col-sm-6 col-md-6 col-xl-4 flex-grid-item">
						<article class="article-card">
							<figure
								class="article-card__image"
								style="background-image: url('https://images.unsplash.com/photo-1590283603385-17ffb3a7f29f?w=800&h=600&fit=crop');"
							>
								<img
									src="https://images.unsplash.com/photo-1590283603385-17ffb3a7f29f?w=800&h=600&fit=crop"
									alt="Market Analysis"
									loading="lazy"
								/>
							</figure>
							<div class="article-card__type">
								<span class="label label--info">Daily Video</span>
							</div>
							<h4 class="h5 article-card__title">
								<a href="/blog">Market Analysis & Trading Strategies</a>
							</h4>
							<span class="article-card__meta">
								<small>Expert insights from our trading team</small>
							</span>
							<a href="/pricing" class="btn btn-tiny btn-default">View Plans</a>
						</article>
					</div>

					<div class="col-xs-12 col-sm-6 col-md-6 col-xl-4 flex-grid-item">
						<article class="article-card">
							<figure
								class="article-card__image"
								style="background-image: url('https://images.unsplash.com/photo-1642790106117-e829e14a795f?w=800&h=600&fit=crop');"
							>
								<img
									src="https://images.unsplash.com/photo-1642790106117-e829e14a795f?w=800&h=600&fit=crop"
									alt="Trading Education"
									loading="lazy"
								/>
							</figure>
							<div class="article-card__type">
								<span class="label label--info">Daily Video</span>
							</div>
							<h4 class="h5 article-card__title">
								<a href="/blog">Learn Advanced Trading Techniques</a>
							</h4>
							<span class="article-card__meta">
								<small>Professional trading education</small>
							</span>
							<a href="/pricing" class="btn btn-tiny btn-default">View Plans</a>
						</article>
					</div>
				</div>
			</section>
		{/if}

		<!--
		─────────────────────────────────────────────────────────────────────────
		SECTION 4: TOOLS
		Shows available tools like Weekly Watchlist, Scanner, etc.
		─────────────────────────────────────────────────────────────────────────
		-->
		<section class="dashboard__content-section">
			<h2 class="section-title">Tools</h2>
			<div class="membership-cards row">
				<div class="col-sm-6 col-xl-4">
					<ToolCard
						name="Weekly Watchlist"
						slug="ww"
						icon="st-icon-trade-of-the-week"
						href="/dashboard/ww/"
					/>
				</div>
				<!-- Additional tools can be added here -->
			</div>
		</section>

		<!--
		─────────────────────────────────────────────────────────────────────────
		SECTION 5: WEEKLY WATCHLIST FEATURE
		Featured content section with video thumbnail
		─────────────────────────────────────────────────────────────────────────
		-->
		<WeeklyWatchlistSection isLoading={loading} />

	</div>

	<!--
	─────────────────────────────────────────────────────────────────────────────
	PANEL 2: SECONDARY SIDEBAR (Content Sidebar)
	Empty on main dashboard, shown on membership-specific pages
	─────────────────────────────────────────────────────────────────────────────
	-->
	<aside class="dashboard__content-sidebar">
		<section class="content-sidebar__section">
			<!-- Sidebar content would go here on membership pages -->
		</section>
	</aside>
</div>

<!--
═══════════════════════════════════════════════════════════════════════════════════
STYLES - Page-specific CSS
═══════════════════════════════════════════════════════════════════════════════════

Most styling is inherited from +layout.svelte and core components.
These are page-specific overrides and additions.
-->
<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD CONTENT LAYOUT
	   Two-column layout: main content + optional sidebar
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__content {
		display: flex;
		flex-flow: row nowrap;
	}

	.dashboard__content-main {
		border-right: 1px solid #dbdbdb;
		flex: 1 1 auto;
		background-color: #efefef;
		min-width: 0;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   CONTENT SECTIONS
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__content-section {
		padding: 30px 40px;
		background-color: #fff;
		margin-bottom: 20px;
		overflow-x: auto;
		overflow-y: hidden;
	}

	@media screen and (min-width: 1280px) {
		.dashboard__content-section {
			padding: 30px;
		}
	}

	@media screen and (min-width: 1440px) {
		.dashboard__content-section {
			padding: 40px;
		}
	}

	.section-title {
		color: #333;
		font-size: 28px;
		font-weight: 700;
		margin: 0 0 30px;
		font-family: 'Open Sans', sans-serif;
		line-height: 1.2;
	}

	.u--margin-top-20 {
		margin-top: 20px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   GRID LAYOUT
	   Bootstrap-style flexbox grid
	   ═══════════════════════════════════════════════════════════════════════════ */

	.membership-cards.row {
		display: flex;
		flex-wrap: wrap;
		margin: -30px -15px 0;
	}

	.row {
		display: flex;
		flex-wrap: wrap;
		margin-right: -15px;
		margin-left: -15px;
	}

	.col-sm-6,
	.col-xl-4 {
		position: relative;
		width: 100%;
		padding-right: 15px;
		padding-left: 15px;
		margin-top: 30px;
	}

	@media (min-width: 641px) {
		.col-sm-6 {
			flex: 0 0 50%;
			max-width: 50%;
		}
	}

	@media (min-width: 1200px) {
		.col-xl-4 {
			flex: 0 0 33.333%;
			max-width: 33.333%;
		}
	}

	@media (max-width: 992px) {
		.col-xl-4 {
			flex: 0 0 50%;
			max-width: 50%;
		}
	}

	@media (max-width: 641px) {
		.col-sm-6,
		.col-xl-4 {
			flex: 0 0 100%;
			max-width: 100%;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ARTICLE CARDS - Latest Updates Section
	   ═══════════════════════════════════════════════════════════════════════════ */

	.article-cards.row {
		margin: 0 -10px;
	}

	.flex-grid {
		display: flex;
		flex-wrap: wrap;
	}

	.flex-grid-item {
		display: flex;
	}

	.col-xs-12 {
		width: 100%;
		padding: 0 10px;
		box-sizing: border-box;
		margin-bottom: 20px;
	}

	.col-md-6 {
		flex: 0 0 50%;
		max-width: 50%;
	}

	.article-card {
		background: #fff;
		border-radius: 5px;
		overflow: hidden;
		box-shadow: 0 5px 30px rgba(0, 0, 0, 0.1);
		transition: all 0.2s ease-in-out;
		display: flex;
		flex-direction: column;
		width: 100%;
	}

	.article-card:hover {
		box-shadow: 0 8px 35px rgba(0, 0, 0, 0.12);
	}

	.article-card__image {
		position: relative;
		width: 100%;
		height: 200px;
		background-size: cover;
		background-position: center;
		margin: 0;
	}

	.article-card__image img {
		width: 100%;
		height: 100%;
		object-fit: cover;
		opacity: 0;
	}

	.article-card__type {
		padding: 12px 20px 0;
	}

	.label {
		display: inline-block;
		padding: 4px 12px;
		border-radius: 3px;
		font-size: 11px;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.label--info {
		background: #0984ae;
		color: #fff;
	}

	.article-card__title {
		padding: 12px 20px 0;
		margin: 0;
	}

	.article-card__title a {
		color: #333;
		text-decoration: none;
		font-size: 18px;
		font-weight: 700;
		font-family: 'Open Sans', sans-serif;
		line-height: 1.4;
		transition: color 0.2s;
	}

	.article-card__title a:hover {
		color: #0984ae;
	}

	.h5 {
		font-size: 18px;
		font-weight: 600;
	}

	.article-card__meta {
		display: block;
		padding: 8px 20px 0;
		color: #999;
		font-size: 13px;
	}

	.article-card .btn {
		margin: auto 20px 20px;
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
		padding: 8px 16px;
		font-size: 13px;
	}

	.btn-default {
		background: #f5f5f5;
		color: #333;
		border: 1px solid #ddd;
		box-shadow: none;
	}

	.btn-default:hover {
		background: #e8e8e8;
		border-color: #ccc;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   SIDEBAR (Hidden on main dashboard)
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__content-sidebar {
		display: none;
		width: 260px;
		flex: 0 0 auto;
		margin-top: -1px;
		background: #fff;
		border-right: 1px solid #dbdbdb;
		border-top: 1px solid #dbdbdb;
	}

	.content-sidebar__section {
		padding: 20px 30px 20px 20px;
		border-bottom: 1px solid #dbdbdb;
	}
</style>

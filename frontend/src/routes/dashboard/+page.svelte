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
		font-size: 32px;
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

	/* Trading Room Rules - Exact WordPress Match (ultradingroom)
	   Note: In reference, JS removes .btn class. We override here instead. */
	.ultradingroom {
		text-align: right;
		list-style: none;
		margin: 0;
		padding: 0;
		display: block !important;  /* Override global display:none */
		max-width: none;
	}

	.ultradingroom .litradingroom {
		display: block;
		padding: 0;
		border: none;
		background: transparent;
	}

	.ultradingroom .litradingroom a {
		font-weight: 700 !important;
		font-size: 12px;
		color: #0984ae;
		text-decoration: none;
		padding: 0;
		background: transparent;
		border: none;
		box-shadow: none;
	}

	.ultradingroom .litradingroom a:hover {
		text-decoration: underline;
		color: #065a75;
	}

	.ultradingroom .litradingroomhind {
		font-size: 11px;
		color: #666;
		display: block;
		padding: 0;
		border: none;
		background: transparent;
		box-shadow: none;
		text-align: right;
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
		border: 1px solid #dbdbdb;
		border-radius: 8px;
		overflow: hidden;
		box-shadow: 0 5px 30px rgba(0, 0, 0, 0.1);
		transition: all 0.2s ease-in-out;
		display: flex;
		flex-direction: column;
		width: 100%;
	}

	.article-card:hover {
		box-shadow: 0 5px 30px rgba(0, 0, 0, 0.15);
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

	.btn-orange,
	.btn-tradingroom {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		background: #F69532;
		border: none;
		color: #fff;
		border-radius: 5px;
		font-size: 14px;
		font-weight: 700;
		font-family: 'Open Sans', sans-serif;
		cursor: pointer;
		transition: all 0.2s ease-in-out;
		box-shadow: 0 2px 5px rgba(0, 0, 0, 0.16);
		line-height: 1.4;
	}

	/* EXACT from Simpler Trading style.css - btn-tradingroom override */
	.btn.btn-xs.btn-orange.btn-tradingroom {
		width: 280px;
		padding: 12px 18px;
	}

	.btn-orange:hover,
	.btn-tradingroom:hover {
		background: #dc7309;
		box-shadow: 0 2px 5px rgba(0, 0, 0, 0.16);
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

	.content-sidebar__heading {
		padding: 15px 20px;
		margin: -20px -30px 20px -20px;
		font-size: 14px;
		font-weight: 700;
		font-family: 'Open Sans', sans-serif;
		letter-spacing: 0.1em;
		text-transform: uppercase;
		color: #333;
		background: #ededed;
		border-bottom: 1px solid #dbdbdb;
		line-height: 1.4;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ADDITIONAL GLOBAL STYLES FROM SIMPLER TRADING
	   ═══════════════════════════════════════════════════════════════════════════ */
	
	/* Empty paragraph cleanup */
	:global(p:empty) {
		display: none;
	}

	/* Scanner image cleanup */
	:global(.scanner-load-content img[src*="/public/images/space.gif"]) {
		display: none;
	}

	/* Typography overrides */
	:global(h2) {
		font-size: 32px;
	}

	:global(h3) {
		font-size: 26px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   BUTTON SHAPE VARIANTS - Simpler Trading Brand Buttons
	   ═══════════════════════════════════════════════════════════════════════════ */
	
	/* Shaped Button - Pill-shaped with extra bold text */
	:global(.shaped-btn) {
		display: block;
		border-radius: 25px;
		width: 100%;
		font-weight: 800;
		font-size: 18px;
		text-transform: uppercase;
		padding: 10px 20px;
		letter-spacing: 1.125px;
		transition: all 0.2s ease-in-out;
	}

	/* Squared Button - Subtle rounded corners */
	:global(.squared-btn) {
		display: block;
		border-radius: 4px;
		width: 100%;
		font-weight: 800;
		font-size: 14px;
		text-transform: uppercase;
		padding: 10px 20px;
		letter-spacing: 1.125px;
		transition: all 0.2s ease-in-out;
	}

	/* Primary Button Color - Brand Orange */
	:global(.primary-btn) {
		background-color: #F69532;
		color: #fff;
		box-shadow: 0 2px 5px rgba(0, 0, 0, 0.16);
	}

	:global(.primary-btn:hover) {
		background-color: #dc7309;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD-SPECIFIC UTILITY STYLES
	   ═══════════════════════════════════════════════════════════════════════════ */
	
	/* Dashboard navigation submenu z-index fix */
	:global(.dashboard__nav-secondary .dashboard__nav-submenu) {
		z-index: 110 !important;
	}

	/* Trading room image wrapper padding reset */
	:global(.tr_img_wrap) {
		padding: 0 !important;
	}

	/* Weekly watchlist title padding reset */
	:global(figure.weekly_watchlist .article-card__title) {
		padding: 0 !important;
	}

	/* Trading room layout controls - ultradingroom visibility controlled by component styles */
	:global(.dashboard__header) {
		justify-content: space-between;
	}

	/* EXACT CSS FROM FILE 1 - Lines 3924-4072 */
	:global(p:empty) {
		display: none;
	}

	:global(.scanner-load-content img[src*="/public/images/space.gif"]) {
		display: none;
	}

	:global(h2) {
		font-size: 32px;
	}

	:global(h3) {
		font-size: 26px;
	}

	/* Button Shapes - EXACT from file 1 */
	:global(.shaped-btn) {
		display: block;
		border-radius: 25px;
		width: 100%;
		font-weight: 800;
		font-size: 18px;
		text-transform: uppercase;
		padding: 10px 20px;
		letter-spacing: 1.125px;
		transition: all .2s ease-in-out;
	}

	:global(.squared-btn) {
		display: block;
		border-radius: 4px;
		width: 100%;
		font-weight: 800;
		font-size: 14px;
		text-transform: uppercase;
		padding: 10px 20px;
		letter-spacing: 1.125px;
		transition: all .2s ease-in-out;
	}

	/* Button Colors - EXACT from file 1 */
	:global(.primary-btn) {
		background-color: #F69532;
		color: #fff;
		box-shadow: 0 2px 5px rgba(0, 0, 0, 0.16);
	}

	:global(.primary-btn:hover) {
		background-color: #dc7309;
	}

	/* Dashboard-specific styles - EXACT from file 1 */
	:global(.dashboard__nav-secondary .dashboard__nav-submenu) {
		z-index: 110 !important;
	}

	:global(.tr_img_wrap) {
		padding: 0 !important;
	}

	:global(figure.weekly_watchlist .article-card__title) {
		padding: 0 !important;
	}

	/* Mobile navigation override - EXACT from file 1 */
	@media (max-width: 641px) {
		:global(.main-navigation .main-nav ul li[class*="current-menu-"] > a),
		:global(.main-navigation .main-nav ul li > a) {
			color: #191717 !important;
		}
	}

	/* Icon styles - EXACT from file 1 */
	:global(.st-icon-this-week) {
		font-size: 28px;
	}

	:global(.simpler-showcase-icon) {
		background: black !important;
		color: orange !important;
	}

	/* EXACT INLINE STYLES FROM FILE 1 - Lines 5776-5785 */
	:global(.mem_icon),
	:global(.mem_div) {
		display: inline-block;
		vertical-align: middle;
	}

	:global(.mem_div) {
		white-space: normal;
		width: calc(100% - 43px);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   MISSING BUTTON CLASSES - btn-xs, btn-link
	   ═══════════════════════════════════════════════════════════════════════════ */
	:global(.btn) {
		display: inline-block;
		padding: 6px 12px;
		margin-bottom: 0;
		font-size: 14px;
		font-weight: 400;
		line-height: 1.42857143;
		text-align: center;
		white-space: nowrap;
		vertical-align: middle;
		cursor: pointer;
		border: 1px solid transparent;
		border-radius: 4px;
		text-decoration: none;
		transition: all 0.15s ease-in-out;
	}

	:global(.btn-xs) {
		padding: 1px 5px;
		font-size: 12px;
		line-height: 1.5;
		border-radius: 3px;
	}

	:global(.btn-link) {
		color: #0984ae;
		font-weight: 400;
		border-radius: 0;
		background-color: transparent;
		border-color: transparent;
		box-shadow: none;
	}

	:global(.btn-link:hover),
	:global(.btn-link:focus) {
		color: #065a75;
		text-decoration: underline;
		background-color: transparent;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD ICON SIZING - Missing from your dashboard
	   ═══════════════════════════════════════════════════════════════════════════ */
	:global(.dashboard__nav-item-icon) {
		display: inline-block;
		width: 24px;
		height: 24px;
		margin-right: 10px;
		font-size: 24px;
		color: #0984ae;
		vertical-align: middle;
	}

	:global(.dashboard__nav-item-icon.st-icon-training-room) {
		font-size: 26px !important;
	}

	:global(.dashboard__nav-secondary .dashboard__nav-item-icon.st-icon-training-room) {
		font-size: 20px !important;
	}

	:global(.dashboard__nav-item-icon.st-icon-stacked-profits) {
		font-size: 40px !important;
	}

	:global(.st-icon-this-week) {
		font-size: 28px;
	}

	:global(.st-icon-big-market-scorecard:before) {
		margin-left: 4px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ICON SIZE CLASSES - icon--lg, icon--md
	   ═══════════════════════════════════════════════════════════════════════════ */
	:global(.icon) {
		display: inline-block;
		vertical-align: middle;
	}

	:global(.icon--lg) {
		font-size: 32px;
		width: 32px;
		height: 32px;
	}

	:global(.icon--md) {
		font-size: 24px;
		width: 24px;
		height: 24px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   MEMBERSHIP CARD ICON STYLING
	   ═══════════════════════════════════════════════════════════════════════════ */
	:global(.membership-card__icon) {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 40px;
		height: 40px;
		margin-right: 10px;
		color: #0984ae;
	}

	:global(.simpler-showcase-icon) {
		background: black !important;
		color: orange !important;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   FONT WEIGHT & SIZE FIXES - Match reference exactly
	   ═══════════════════════════════════════════════════════════════════════════ */
	:global(.dashboard__nav-item-text) {
		font-size: 14px;
		font-weight: 400;
		color: #fff;
	}

	/* Bold white text for My Classes and My Indicators */
	:global(.dashboard__nav-item-text[style*="bold"]) {
		font-weight: 700 !important;
		color: white !important;
	}
</style>

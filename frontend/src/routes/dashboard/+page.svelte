<!--
	URL: /dashboard
	
	Dashboard Home Page - Member Dashboard Landing
	═══════════════════════════════════════════════════════════════════════════
	Apple ICT 11+ Principal Engineer Implementation

	CLIENT-SIDE AUTH PATTERN:
	- User data from auth store (not server props)
	- Memberships fetched client-side
	- Reactive updates on auth changes

	Svelte 5 Features:
	- $state() for component state
	- $derived() for computed values from auth store
	- $effect() for reactive side effects

	@version 3.1.0 - RtpIcon Integration
	@author Revolution Trading Pros
-->
<script lang="ts">
	import { user, isAuthenticated, isInitializing } from '$lib/stores/auth';
	import { getUserMemberships, type UserMembershipsResponse } from '$lib/api/user-memberships';
	import RtpIcon from '$lib/components/icons/RtpIcon.svelte';

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE
	// ═══════════════════════════════════════════════════════════════════════════

	// Dropdown state
	let isDropdownOpen = $state(false);

	// Memberships data
	let membershipsData = $state<UserMembershipsResponse | null>(null);
	let isLoading = $state(true);

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED STATE
	// ═══════════════════════════════════════════════════════════════════════════

	// User display name
	const userName = $derived(
		$user?.name ?? $user?.email?.split('@')[0] ?? 'Member'
	);

	// ═══════════════════════════════════════════════════════════════════════════
	// DYNAMIC MEMBERSHIP RENDERING
	// Render cards from actual API data instead of hardcoded slugs
	// ═══════════════════════════════════════════════════════════════════════════

	// Trading rooms for the dropdown button - Only show Day Trading Room, Swing Trading Room, and Small Accounts
	const tradingRooms = $derived.by(() => {
		const rooms: { name: string; href: string; icon: string; variant: string }[] = [
			{
				name: 'Day Trading Room',
				href: '#', // TODO: Provide URL
				icon: 'chart-line',
				variant: 'day-trading'
			},
			{
				name: 'Swing Trading Room',
				href: '#', // TODO: Provide URL
				icon: 'trending-up',
				variant: 'swing-trading'
			},
			{
				name: 'Small Accounts Mentorship',
				href: '#', // TODO: Provide URL
				icon: 'piggy-bank',
				variant: 'small-accounts'
			}
		];

		return rooms;
	});

	// Membership cards - Trading Rooms and Alert Services
	const membershipCards = $derived.by(() => {
		const cards: {
			name: string;
			href: string;
			icon: string;
			variant: string;
			tradingRoom?: string;
		}[] = [];

		// Add trading rooms
		const tradingRoomData = membershipsData?.tradingRooms ?? [];
		for (const membership of tradingRoomData) {
			if (membership.status === 'active') {
				cards.push({
					name: membership.name,
					href: `/dashboard/${membership.slug}`,
					icon: membership.icon ?? 'chart-line',
					variant: getVariantFromSlug(membership.slug),
					tradingRoom: membership.accessUrl ?? `/live-trading-rooms/${membership.slug}`
				});
			}
		}

		// Add alert services
		const alertServiceData = membershipsData?.alertServices ?? [];
		for (const service of alertServiceData) {
			if (service.status === 'active') {
				cards.push({
					name: service.name,
					href: `/dashboard/${service.slug}`,
					icon: service.icon ?? 'bell',
					variant: getVariantFromSlug(service.slug)
				});
			}
		}

		return cards;
	});

	// Mentorship cards - All courses (main education section)
	const mentorshipCards = $derived.by(() => {
		const cards: {
			name: string;
			href: string;
			icon: string;
			variant: string;
			tradingRoom?: string;
		}[] = [];

		const courseData = membershipsData?.courses ?? [];
		for (const course of courseData) {
			// Show all active courses in Mentorship section
			if (course.status === 'active') {
				cards.push({
					name: course.name,
					href: `/dashboard/${course.slug}`,
					icon: course.icon ?? 'book',
					variant: course.slug === 'small-account-mentorship' ? 'training' : 'growth',
					tradingRoom: course.accessUrl
				});
			}
		}

		return cards;
	});

	// Scanner cards - Scanners and High Octane indicator (between Mentorship and Tools)
	const scannerCards = $derived.by(() => {
		const cards: {
			name: string;
			href: string;
			icon: string;
			variant: string;
		}[] = [];

		// Add scanners
		const scannerData = membershipsData?.scanners ?? [];
		for (const scanner of scannerData) {
			if (scanner.status === 'active') {
				cards.push({
					name: scanner.name,
					href: `/dashboard/${scanner.slug}`,
					icon: scanner.icon ?? 'chart-candle',
					variant: 'scanner'
				});
			}
		}

		// Add indicators (High Octane Scanner specifically)
		const indicatorData = membershipsData?.indicators ?? [];
		for (const indicator of indicatorData) {
			// Only show High Octane Scanner in this section (other indicators go elsewhere)
			if (indicator.status === 'active' && indicator.slug === 'high-octane-scanner') {
				cards.push({
					name: indicator.name,
					href: `/dashboard/${indicator.slug}`,
					icon: indicator.icon ?? 'chart-candle',
					variant: 'high-octane'
				});
			}
		}

		return cards;
	});

	// Tools cards (always shown - static)
	const toolsCards = [
		{
			name: 'Weekly Watchlist',
			href: '/dashboard/ww',
			icon: 'weekly-watchlist',
			variant: 'ww'
		}
	];

	// Helper function to get variant class based on slug
	function getVariantFromSlug(slug: string): string {
		const variantMap: Record<string, string> = {
			'day-trading-room': 'options',
			'swing-trading-room': 'foundation',
			'small-account-mentorship': 'training',
			'spx-profit-pulse': 'tr3ndy',
			'explosive-swing': 'growth',
			'mastering-the-trade': 'options',
			'simpler-showcase': 'foundation',
			'tr3ndy-spx-alerts': 'tr3ndy',
			'compounding-growth-mastery': 'growth'
		};
		return variantMap[slug] ?? 'options';
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// DATA LOADING - Reactive pattern to handle auth race condition
	// ═══════════════════════════════════════════════════════════════════════════

	// Use $effect to reactively load memberships when auth becomes ready
	// This fixes the race condition where onMount runs before auth is initialized
	// ICT 11+ Fix: Removed hasAttemptedLoad flag to ensure data loads on every page refresh
	$effect(() => {
		// Wait for auth to finish initializing
		if ($isInitializing) {
			isLoading = true;
			return;
		}

		// If not authenticated, stop loading
		if (!$isAuthenticated) {
			isLoading = false;
			membershipsData = null;
			return;
		}

		// Load memberships when authenticated
		// This runs on every page load/refresh when user is authenticated
		loadMemberships();
	});

	async function loadMemberships(): Promise<void> {
		isLoading = true;
		try {
			console.log('[Dashboard] Loading memberships for user:', $user?.email);
			membershipsData = await getUserMemberships();
			console.log('[Dashboard] Memberships loaded:', membershipsData);
		} catch (error) {
			console.error('[Dashboard] Failed to load memberships:', error);
			membershipsData = null;
		} finally {
			isLoading = false;
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// DROPDOWN HANDLERS
	// ═══════════════════════════════════════════════════════════════════════════

	function toggleDropdown(event: Event): void {
		event.stopPropagation();
		isDropdownOpen = !isDropdownOpen;
	}

	function closeDropdown(): void {
		isDropdownOpen = false;
	}

	// Close dropdown when clicking outside
	$effect(() => {
		if (isDropdownOpen && typeof window !== 'undefined') {
			const handleClickOutside = (e: MouseEvent) => {
				const target = e.target as HTMLElement;
				if (!target.closest('.dropdown')) {
					closeDropdown();
				}
			};
			const handleEscape = (e: KeyboardEvent) => {
				if (e.key === 'Escape') {
					closeDropdown();
				}
			};
			document.addEventListener('click', handleClickOutside);
			document.addEventListener('keydown', handleEscape);
			return () => {
				document.removeEventListener('click', handleClickOutside);
				document.removeEventListener('keydown', handleEscape);
			};
		}
		return undefined;
	});
</script>

<!-- Dashboard Header -->
<header class="dashboard__header">
	<div class="dashboard__header-left">
		<h1 class="dashboard__page-title">Member Dashboard</h1>
		<a href="https://www.simplertrading.com/dashboard/start-here" class="btn btn-xs btn-default">
			New? Start Here
		</a>
	</div>
	<div class="dashboard__header-right">
		{#if tradingRooms.length > 0}
			<div class="dropdown" class:is-open={isDropdownOpen}>
				<button
					class="btn btn-orange btn-tradingroom"
					onclick={toggleDropdown}
					aria-expanded={isDropdownOpen}
					aria-haspopup="true"
					type="button"
				>
					<strong>Enter the Trading Room</strong>
					<span class="dropdown-arrow">
						<RtpIcon name="chevron-down" size={14} />
					</span>
				</button>

				{#if isDropdownOpen}
					<div class="dropdown-menu" role="menu">
						{#each tradingRooms as room}
						<a 
							href={room.href} 
							class="dropdown-item dropdown-item--{room.variant}" 
							onclick={closeDropdown}
							role="menuitem"
						>
							<span class="dropdown-item__icon">
								<RtpIcon name={room.icon} size={20} />
							</span>
							<span class="dropdown-item__text">{room.name}</span>
						</a>
					{/each}
					</div>
				{/if}
			</div>

			<!-- Trading Room Rules - Legal Compliance -->
			<div class="trading-room-rules">
				<a
					href="/trading-room-rules.pdf"
					target="_blank"
					rel="noopener noreferrer"
					class="trading-room-rules__link"
				>
					Trading Room Rules
				</a>
				<p class="trading-room-rules__disclaimer">
					By logging into any of our Live Trading Rooms, You are agreeing to our Rules of the Room.
				</p>
			</div>
		{/if}
	</div>
</header>

<!-- Dashboard Content -->
{#if !isLoading}
	<!-- Memberships Section -->
	{#if membershipCards.length > 0}
		<section class="dashboard__content-section">
			<h2 class="section-title">Memberships</h2>
			<div class="membership-cards">
				{#each membershipCards as card}
					<div class="membership-card-col">
						<article class="membership-card membership-card--{card.variant}">
							<a href={card.href} class="membership-card__header">
								<span class="mem_icon">
									<span class="membership-card__icon">
										<RtpIcon name={card.icon} size={32} />
									</span>
								</span>
								<span class="mem_div">{card.name}</span>
							</a>
							<div class="membership-card__actions">
								<a href={card.href}>Dashboard</a>
								{#if card.tradingRoom}
									<a href={card.tradingRoom} target="_blank">Trading Room</a>
								{/if}
							</div>
						</article>
					</div>
				{/each}
			</div>
			<!-- Divider -->
			<div class="section-divider">
				<div class="section-divider__line"></div>
			</div>
		</section>
	{/if}

	<!-- Mentorship Section -->
	{#if mentorshipCards.length > 0}
		<section class="dashboard__content-section">
			<h2 class="section-title">Mentorship</h2>
			<div class="membership-cards">
				{#each mentorshipCards as card}
					<div class="membership-card-col">
						<article class="membership-card membership-card--{card.variant}">
							<a href={card.href} class="membership-card__header">
								<span class="mem_icon">
									<span class="membership-card__icon">
										<RtpIcon name={card.icon} size={32} />
									</span>
								</span>
								<span class="mem_div">{card.name}</span>
							</a>
							<div class="membership-card__actions">
								<a href={card.href}>Dashboard</a>
								{#if card.tradingRoom}
									<a href={card.tradingRoom} target="_blank">Trading Room</a>
								{/if}
							</div>
						</article>
					</div>
				{/each}
			</div>
			<!-- Divider -->
			<div class="section-divider">
				<div class="section-divider__line"></div>
			</div>
		</section>
	{/if}

	<!-- Scanners Section (High Octane Scanner) -->
	{#if scannerCards.length > 0}
		<section class="dashboard__content-section">
			<h2 class="section-title">Scanners</h2>
			<div class="membership-cards">
				{#each scannerCards as card}
					<div class="membership-card-col">
						<article class="membership-card membership-card--{card.variant}">
							<a href={card.href} class="membership-card__header">
								<span class="mem_icon">
									<span class="membership-card__icon">
										<RtpIcon name={card.icon} size={32} />
									</span>
								</span>
								<span class="mem_div">{card.name}</span>
							</a>
							<div class="membership-card__actions">
								<a href={card.href}>Dashboard</a>
							</div>
						</article>
					</div>
				{/each}
			</div>
			<!-- Divider -->
			<div class="section-divider">
				<div class="section-divider__line"></div>
			</div>
		</section>
	{/if}

	<!-- Tools Section -->
	<section class="dashboard__content-section">
		<h2 class="section-title">Tools</h2>
		<div class="membership-cards">
			{#each toolsCards as card}
				<div class="membership-card-col">
					<article class="membership-card membership-card--{card.variant}">
						<a href={card.href} class="membership-card__header">
							<span class="mem_icon">
								<span class="membership-card__icon">
									<RtpIcon name={card.icon} size={24} />
								</span>
							</span>
							<span class="mem_div">{card.name}</span>
						</a>
						<div class="membership-card__actions">
							<a href={card.href}>Dashboard</a>
						</div>
					</article>
				</div>
			{/each}
		</div>
		<!-- Divider -->
		<div class="section-divider">
			<div class="section-divider__line"></div>
		</div>
	</section>

	<!-- Weekly Watchlist Section -->
	<section class="dashboard__content-section">
		<div class="watchlist-row">
			<div class="watchlist-col-left">
				<h2 class="section-title-alt section-title-alt--underline">Weekly Watchlist</h2>
				<div class="watchlist-mobile-image">
					<a href="/watchlist/current">
						<img src="https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/David-Watchlist-Rundown.jpg" alt="Weekly Watchlist" class="watchlist-image" />
					</a>
				</div>
				<h4 class="watchlist-subtitle">Weekly Watchlist with David Starr</h4>
				<p class="watchlist-description">Week of December 29, 2025.</p>
				<a href="/watchlist/current" class="btn btn-tiny btn-default">Watch Now</a>
			</div>
			<div class="watchlist-col-right">
				<a href="/watchlist/current">
					<img src="https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/David-Watchlist-Rundown.jpg" alt="Weekly Watchlist" class="watchlist-image" />
				</a>
			</div>
		</div>
		<!-- Divider -->
		<div class="section-divider">
			<div class="section-divider__line"></div>
		</div>
	</section>

	<!-- Empty State - No Memberships -->
	{#if membershipCards.length === 0 && mentorshipCards.length === 0}
		<section class="dashboard__content-section">
			<div class="empty-state">
				<h2 class="empty-state__title">Welcome, {userName}!</h2>
				<p class="empty-state__text">
					You don't have any active memberships yet. Explore our trading rooms and courses to get started.
				</p>
				<a href="/courses" class="btn btn-orange">Browse Courses</a>
			</div>
		</section>
	{/if}
{/if}

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * DASHBOARD HEADER - WordPress Exact Match
	 * Source: DASHBOARD_DESIGN_SPECIFICATIONS.md
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__header {
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		background-color: #fff;
		border-bottom: 1px solid #dbdbdb;
		border-right: 1px solid #dbdbdb;
		padding: 20px;
	}
    
	@media (min-width: 1280px) {
		.dashboard__header {
			padding: 30px;
		}
	}

	@media (min-width: 1440px) {
		.dashboard__header {
			padding: 30px 40px;
		}
	}

	.dashboard__header-left {
		display: flex;
		align-items: center;
		justify-content: flex-start;
		flex: 1;
		gap: 15px;
	}

	.dashboard__page-title {
		margin: 0;
		color: #333;
		font-size: 36px;
		font-weight: 400;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
	}

	/* Button Styles */
	.btn {
		display: inline-block;
		padding: 10px 20px;
		font-size: 14px;
		font-weight: 600;
		text-align: center;
		text-decoration: none;
		border-radius: 4px;
		transition: all 0.15s ease-in-out;
		cursor: pointer;
		border: none;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
	}

	.btn-xs {
		padding: 6px 12px;
		font-size: 12px;
	}

	.dashboard__header-right {
		display: flex;
		flex-direction: column;
		align-items: flex-end;
		margin-top: 10px;
	}

	@media (min-width: 820px) {
		.dashboard__header-right {
			flex-direction: column;
			align-items: flex-end;
			gap: 0;
			margin-top: 0;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * TRADING ROOM RULES - Legal Compliance (WordPress Match)
	 * Button on top, text below in vertical stack
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.trading-room-rules {
		text-align: center;
		margin-top: 10px;
		width: 100%;
		max-width: 300px;
		margin-left: auto;
		margin-right: auto;
	}

	.trading-room-rules__link {
		display: block;
		margin-bottom: 8px;
		font-size: 18px;
		font-weight: 700;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
		color: #1e73be;
		text-decoration: none;
		transition: color 0.15s ease-in-out;
		text-align: center;
	}

	.trading-room-rules__link:hover {
		color: #0984ae;
		text-decoration: underline;
	}

	.trading-room-rules__disclaimer {
		margin: 0;
		font-size: 13px;
		font-weight: 400;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
		color: #666;
		line-height: 1.4;
		text-align: center;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * SECTION DIVIDER - WordPress Match
	 * Full width across content area, not crossing sidebar
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.section-divider {
		margin-top: 30px;
		width: 100%;
		margin-left: 0;
		margin-right: 0;
	}

	.section-divider__line {
		border-top-width: 1px;
		border-top-style: solid;
		border-top-color: #cccccc;
		width: 100%;
		margin: 0;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * WEEKLY WATCHLIST SECTION - WordPress Exact Match
	 * Bootstrap: col-sm-6 col-lg-5 / col-sm-6 col-lg-7
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.watchlist-row {
		display: flex;
		flex-wrap: wrap;
		margin-left: -15px;
		margin-right: -15px;
	}

	.watchlist-col-left {
		flex: 0 0 100%;
		max-width: 100%;
		padding-left: 15px;
		padding-right: 15px;
		text-align: center;
	}

	.watchlist-col-right {
		display: none;
	}

	@media (min-width: 576px) {
		.watchlist-col-left {
			flex: 0 0 50%;
			max-width: 50%;
			text-align: left;
		}

		.watchlist-col-right {
			display: none;
		}
	}

	@media (min-width: 992px) {
		.watchlist-col-left {
			flex: 0 0 41.666667%;
			max-width: 41.666667%;
			text-align: left;
		}

		.watchlist-col-right {
			display: block;
			flex: 0 0 58.333333%;
			max-width: 58.333333%;
			padding-left: 15px;
			padding-right: 15px;
		}

		.watchlist-mobile-image {
			display: none;
		}
	}

	.section-title-alt {
		font-size: 26px;
		font-weight: 800;
		color: #333;
		margin: 0 0 20px;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
	}

	.section-title-alt--underline {
		padding-bottom: 10px;
		border-bottom: 3px solid #ff8c00;
		display: inline-block;
	}

	.watchlist-mobile-image {
		margin-bottom: 15px;
	}

	.watchlist-mobile-image img {
		width: 100%;
		border-radius: 5px;
	}

	.watchlist-subtitle {
		font-size: 18px;
		font-weight: 800;
		color: #333;
		margin: 0 0 10px;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
	}

	.watchlist-description {
		font-size: 14px;
		color: #666;
		margin: 0 0 15px;
	}

	.watchlist-image {
		width: 100%;
		border-radius: 5px;
	}

	.btn-tiny {
		display: inline-block;
		padding: 8px 16px;
		font-size: 13px;
		font-weight: 700;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
		text-decoration: none;
		border-radius: 4px;
		transition: all 0.15s ease-in-out;
		cursor: pointer;
		text-align: center;
		white-space: nowrap;
	}

	.btn-default {
		background-color: #143E59;
		color: #fff;
		border: 1px solid #143E59;
	}

	.btn-default:hover {
		background-color: #0f2d41;
		border-color: #0f2d41;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * EMPTY STATE
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.empty-state {
		text-align: center;
		padding: 60px 20px;
	}

	.empty-state__title {
		font-size: 28px;
		font-weight: 700;
		color: #333;
		margin: 0 0 16px;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
	}

	.empty-state__text {
		font-size: 16px;
		color: #666;
		margin: 0 0 24px;
		max-width: 400px;
		margin-left: auto;
		margin-right: auto;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * BUTTONS - WordPress Exact Match
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.btn-orange {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		padding: 10px 20px;
		background-color: #f69532;
		color: #fff;
		font-size: 14px;
		font-weight: 600;
		text-decoration: none;
		border: none;
		border-radius: 4px;
		cursor: pointer;
		transition: all 0.15s ease-in-out;
	}

	.btn-orange:hover {
		background-color: #dc7309;
	}

	.btn-orange strong {
		font-weight: 700;
	}

	.btn-tradingroom {
		text-transform: none;
		width: 280px;
		padding: 12px 18px;
	}

	.dropdown-arrow {
		font-size: 10px;
		transition: transform 0.15s ease-in-out;
		display: flex;
		align-items: center;
	}

	.dropdown.is-open .dropdown-arrow {
		transform: rotate(180deg);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * DROPDOWN MENU - WordPress Exact Match
	 * Source: DASHBOARD_DESIGN_SPECIFICATIONS.md
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dropdown {
		position: relative;
		display: inline-block;
	}

	.dropdown-menu {
		position: absolute;
		top: 100%;
		right: 0;
		padding: 15px;
		min-width: 260px;
		max-width: 280px;
		margin: 5px 0 0;
		font-size: 14px;
		background-color: #ffffff;
		border: none;
		border-radius: 5px;
		box-shadow: 0 10px 30px rgba(0, 0, 0, 0.15);
		z-index: 1000;
		opacity: 1;
		visibility: visible;
		transform: translateY(0);
		transition: all 0.15s ease-in-out;
	}

	.dropdown:not(.is-open) .dropdown-menu {
		opacity: 0;
		visibility: hidden;
		transform: translateY(-5px);
		pointer-events: none;
	}

	.dropdown-menu__heading {
		margin: -20px -20px 20px;
		padding: 15px 20px;
		color: #fff;
		background: #0984ae;
		font-size: 17px;
		font-weight: 700;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
		border-radius: 5px 5px 0 0;
	}

	.dropdown-item {
		display: flex;
		align-items: center;
		gap: 6px;
		padding: 15px 20px;
		color: #666;
		font-size: 14px;
		font-weight: 400;
		text-decoration: none;
		transition: background-color 0.15s ease-in-out;
		border-radius: 5px;
		white-space: nowrap;
	}

	.dropdown-item:hover {
		background-color: #f4f4f4;
	}

	/* Trading Room Dropdown Icon Hover Effects */
	.dropdown-item__icon {
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
		color: #143E59;
		transition: color 0.15s ease-in-out;
	}

	/* Day Trading Room - Blue */
	.dropdown-item--day-trading:hover .dropdown-item__icon {
		color: #0984ae;
	}

	/* Swing Trading Room - Teal */
	.dropdown-item--swing-trading:hover .dropdown-item__icon {
		color: #00abaf;
	}

	/* Small Accounts Mentorship - Purple */
	.dropdown-item--small-accounts:hover .dropdown-item__icon {
		color: #3c22f1;
	}

	.dropdown-item__text {
		flex: 1;
	}


	/* ═══════════════════════════════════════════════════════════════════════════
	 * DASHBOARD CONTENT SECTION
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__content-section {
		padding: 30px 20px;
		overflow-x: auto;
		overflow-y: hidden;
		background-color: #ffffff;
	}

	@media (min-width: 1280px) {
		.dashboard__content-section {
			padding: 30px;
		}
	}

	@media (min-width: 1440px) {
		.dashboard__content-section {
			padding: 40px;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * SECTION TITLES - WordPress Exact Match
	 * Source: DASHBOARD_DESIGN_SPECIFICATIONS.md
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.section-title {
		color: #333;
		font-size: 20px;
		font-weight: 800;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
		margin: 0 0 30px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * MEMBERSHIP CARDS - WordPress Exact Match
	 * Source: DASHBOARD_DESIGN_SPECIFICATIONS.md
	 * Grid: col-sm-6 col-xl-4 (Bootstrap breakpoints)
	 * - Mobile (< 576px): 1 card per row
	 * - Small-Large (576px - 1199px): 2 cards per row
	 * - XL+ (≥ 1200px): 3 cards per row
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.membership-cards {
		display: flex;
		flex-wrap: wrap;
		margin-left: -15px;
		margin-right: -15px;
	}

	/* Grid column wrapper - handles responsive layout */
	.membership-card-col {
		flex: 0 0 100%;
		max-width: 100%;
		padding-left: 15px;
		padding-right: 15px;
		margin-bottom: 30px;
		box-sizing: border-box;
	}

	/* Small screens and up (≥576px): 2 columns */
	@media (min-width: 576px) {
		.membership-card-col {
			flex: 0 0 50%;
			max-width: 50%;
		}
	}

	/* Extra large screens (≥1200px): 3 columns */
	@media (min-width: 1200px) {
		.membership-card-col {
			flex: 0 0 33.333333%;
			max-width: 33.333333%;
		}
	}

	/* Card styling - visual appearance */
	.membership-card {
		background: #fff;
		border-radius: 5px;
		box-shadow: 0 5px 30px rgba(0, 0, 0, 0.1);
		height: 100%;
		display: flex;
		flex-direction: column;
	}

	.membership-card__header {
		display: block;
		padding: 20px;
		color: #333;
		font-weight: 700;
		white-space: nowrap;
		text-decoration: none;
		transition: all 0.15s ease-in-out;
	}

	.membership-card__header:hover {
		color: #0984ae;
	}

	.mem_icon {
		display: inline-block;
		vertical-align: middle;
		margin-right: 9px;
	}

	.membership-card__icon {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 50px;
		height: 50px;
		color: #fff;
		border-radius: 50%;
		transition: all 0.15s ease-in-out;
	}

	.mem_div {
		display: inline-block;
		vertical-align: middle;
		font-size: 14px;
		font-weight: 800;
		color: #333;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
		white-space: normal;
		width: calc(100% - 65px); /* Account for icon width + margin */
	}

	.icon {
		display: inline-flex;
		align-items: center;
		justify-content: center;
	}

	.icon--lg {
		font-size: 32px;
	}

	.icon--md {
		font-size: 24px;
	}

	/* Icons are now rendered via RtpIcon component using Tabler Icons */

	/* ═══════════════════════════════════════════════════════════════════════════
	 * MEMBERSHIP CARD ACTIONS - WordPress Exact Match
	 * Source: DASHBOARD_DESIGN_SPECIFICATIONS.md
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.membership-card__actions {
		display: flex;
		font-size: 14px;
		border-top: 1px solid #ededed;
		justify-content: center;
	}

	.membership-card__actions a {
		display: block;
		flex: 0 0 auto;
		flex-basis: 50%;
		width: 50%;
		height: 100%;
		padding: 15px;
		text-align: center;
		font-size: 14px;
		font-weight: 800;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
		text-decoration: none;
		color: #1e73be;
		transition: all 0.15s ease-in-out;
	}

	.membership-card__actions a:hover {
		color: #0a5a75;
	}

	.membership-card__actions a + a {
		border-left: 1px solid #ededed;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * CARD VARIANTS - WordPress Exact Match
	 * Source: DASHBOARD_DESIGN_SPECIFICATIONS.md
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.membership-card--options .membership-card__icon {
		background-color: #0984ae;
		box-shadow: 0 5px 15px rgba(9, 132, 174, 0.25);
	}

	.membership-card--options:hover .membership-card__icon {
		background-color: #076787;
	}

	.membership-card--foundation .membership-card__icon {
		background-color: #00abaf;
		box-shadow: 0 5px 15px rgba(0, 171, 175, 0.25);
	}

	.membership-card--foundation:hover .membership-card__icon {
		background-color: #008386;
	}

	.membership-card--tr3ndy .membership-card__icon {
		background-color: #fe8900;
		box-shadow: 0 5px 15px rgba(254, 137, 0, 0.25);
	}

	.membership-card--tr3ndy:hover .membership-card__icon {
		background-color: #d57300;
	}

	.membership-card--growth .membership-card__icon {
		background-color: #005695;
		box-shadow: 0 5px 15px rgba(0, 86, 149, 0.25);
	}

	.membership-card--ww .membership-card__icon {
		background-color: #0c2434;
		box-shadow: 0 5px 15px rgba(12, 36, 52, 0.25);
	}

	.membership-card--ww:hover .membership-card__icon {
		background-color: #040d13;
	}

	.membership-card--training .membership-card__icon {
		background-color: #3c22f1;
		box-shadow: 0 5px 15px rgba(60, 34, 241, 0.25);
	}

	.membership-card--training:hover .membership-card__icon {
		background-color: #280edc;
	}

	/* Scanner Card Variant */
	.membership-card--scanner .membership-card__icon {
		background-color: #7c3aed;
		box-shadow: 0 5px 15px rgba(124, 58, 237, 0.25);
	}

	.membership-card--scanner:hover .membership-card__icon {
		background-color: #6d28d9;
	}

	/* High Octane Scanner Card Variant */
	.membership-card--high-octane .membership-card__icon {
		background-color: #dc2626;
		box-shadow: 0 5px 15px rgba(220, 38, 38, 0.25);
	}

	.membership-card--high-octane:hover .membership-card__icon {
		background-color: #b91c1c;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * RESPONSIVE
	 * ═══════════════════════════════════════════════════════════════════════════ */

	@media (max-width: 768px) {
		.dashboard__header {
			padding: 20px;
			flex-direction: column;
			align-items: flex-start;
			gap: 15px;
		}

		.dashboard__page-title {
			font-size: 24px;
		}

		.dashboard__header-right {
			width: 100%;
		}

		.btn-orange {
			width: 100%;
			justify-content: center;
		}

		.dropdown {
			width: 100%;
		}

		.dropdown-menu {
			left: 0;
			right: 0;
			min-width: auto;
			max-width: none;
		}

		.dashboard__content-section {
			padding: 20px 15px;
		}

		.section-title {
			font-size: 18px;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * REDUCED MOTION
	 * ═══════════════════════════════════════════════════════════════════════════ */

	@media (prefers-reduced-motion: reduce) {
		.membership-card,
		.btn-orange,
		.btn-dashboard,
		.btn-room,
		.dropdown-arrow,
		.dropdown-menu {
			transition: none;
			animation: none;
		}
	}
</style>

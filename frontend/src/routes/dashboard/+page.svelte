<!--
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
	import { user, isAuthenticated } from '$lib/stores/auth';
	import { getUserMemberships, type UserMembershipsResponse } from '$lib/api/user-memberships';
	import { onMount } from 'svelte';
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

	// User's active membership slugs
	const membershipSlugs = $derived(
		membershipsData?.memberships
			?.filter((m: { status: string }) => m.status === 'active')
			?.map((m: { slug: string }) => m.slug) ?? []
	);

	// User display name
	const userName = $derived(
		$user?.name ?? $user?.email?.split('@')[0] ?? 'Member'
	);

	// Trading rooms based on memberships
	const tradingRooms = $derived.by(() => {
		const rooms: { name: string; href: string }[] = [];

		if (membershipSlugs.includes('mastering_the_trade')) {
			rooms.push({ name: 'Mastering the Trade', href: '/trading-room/mastering-the-trade' });
		}
		if (membershipSlugs.includes('simpler_showcase')) {
			rooms.push({ name: 'Simpler Showcase Breakout Room', href: '/trading-room/simpler-showcase' });
		}
		if (membershipSlugs.includes('tr3ndy_spx_alerts')) {
			rooms.push({ name: 'Tr3ndy SPX Trading Room', href: '/trading-room/tr3ndy-spx' });
		}
		if (membershipSlugs.includes('compounding_growth_mastery')) {
			rooms.push({ name: 'Compounding Growth Mastery', href: '/trading-room/cgm' });
		}

		return rooms;
	});

	// Membership cards data
	const membershipCards = $derived.by(() => {
		const cards: {
			name: string;
			href: string;
			icon: string;
			variant: string;
			tradingRoom?: string;
		}[] = [];

		if (membershipSlugs.includes('mastering_the_trade')) {
			cards.push({
				name: 'Mastering the Trade',
				href: '/dashboard/mastering-the-trade',
				icon: 'mastering-the-trade',
				variant: 'options',
				tradingRoom: '/trading-room/mastering-the-trade'
			});
		}
		if (membershipSlugs.includes('simpler_showcase')) {
			cards.push({
				name: 'Simpler Showcase',
				href: '/dashboard/simpler-showcase',
				icon: 'simpler-showcase',
				variant: 'foundation',
				tradingRoom: '/trading-room/simpler-showcase'
			});
		}
		if (membershipSlugs.includes('tr3ndy_spx_alerts')) {
			cards.push({
				name: 'Tr3ndy SPX Alerts Service',
				href: '/dashboard/tr3ndy-spx-alerts',
				icon: 'tr3ndy-spx-alerts-circle',
				variant: 'tr3ndy',
				tradingRoom: '/trading-room/tr3ndy-spx'
			});
		}

		return cards;
	});

	// Mastery cards
	const masteryCards = $derived.by(() => {
		const cards: {
			name: string;
			href: string;
			icon: string;
			variant: string;
			tradingRoom?: string;
		}[] = [];

		if (membershipSlugs.includes('compounding_growth_mastery')) {
			cards.push({
				name: 'Compounding Growth Mastery',
				href: '/dashboard/cgm',
				icon: 'consistent-growth',
				variant: 'growth',
				tradingRoom: '/trading-room/cgm'
			});
		}

		return cards;
	});

	// Tools cards (always shown)
	const toolsCards = [
		{
			name: 'Weekly Watchlist',
			href: '/dashboard/ww',
			icon: 'weekly-watchlist',
			variant: 'ww'
		}
	];

	// ═══════════════════════════════════════════════════════════════════════════
	// DATA LOADING
	// ═══════════════════════════════════════════════════════════════════════════

	onMount(async () => {
		if ($isAuthenticated) {
			await loadMemberships();
		}
	});

	async function loadMemberships(): Promise<void> {
		isLoading = true;
		try {
			membershipsData = await getUserMemberships();
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

	function toggleDropdown(): void {
		isDropdownOpen = !isDropdownOpen;
	}

	function closeDropdown(): void {
		isDropdownOpen = false;
	}
</script>

<!-- Dashboard Header -->
<header class="dashboard__header">
	<div class="dashboard__header-left">
		<h1 class="dashboard__page-title">Member Dashboard</h1>
	</div>
	<div class="dashboard__header-right">
		{#if tradingRooms.length > 0}
			<div class="dropdown" class:is-open={isDropdownOpen}>
				<button
					class="btn btn-orange btn-tradingroom"
					onclick={toggleDropdown}
					aria-expanded={isDropdownOpen}
					aria-haspopup="true"
				>
					<strong>Enter a Trading Room</strong>
					<span class="dropdown-arrow">
						<RtpIcon name="chevron-down" size={14} />
					</span>
				</button>

				{#if isDropdownOpen}
					<div class="dropdown-menu">
						{#each tradingRooms as room}
							<a href={room.href} class="dropdown-item" onclick={closeDropdown}>
								{room.name}
							</a>
						{/each}
					</div>
					<!-- Backdrop to close dropdown -->
					<div
						class="dropdown-backdrop"
						onclick={closeDropdown}
						onkeydown={(e) => e.key === 'Escape' && closeDropdown()}
						role="button"
						tabindex="-1"
						aria-label="Close dropdown"
					></div>
				{/if}
			</div>
		{/if}
	</div>
</header>

<!-- Dashboard Content -->
<div class="dashboard__content">
	<div class="dashboard__content-main">

		{#if isLoading}
			<!-- Loading State -->
			<section class="dashboard__content-section">
				<div class="loading-skeleton">
					<div class="skeleton skeleton--title"></div>
					<div class="skeleton-grid">
						<div class="skeleton skeleton--card"></div>
						<div class="skeleton skeleton--card"></div>
						<div class="skeleton skeleton--card"></div>
					</div>
				</div>
			</section>
		{:else}
			<!-- Memberships Section -->
			{#if membershipCards.length > 0}
				<section class="dashboard__content-section">
					<h2 class="section-title">Memberships</h2>
					<div class="membership-cards">
						{#each membershipCards as card}
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
									<a href={card.href} class="btn-dashboard">Dashboard</a>
									{#if card.tradingRoom}
										<a href={card.tradingRoom} class="btn-room" target="_blank">Trading Room</a>
									{/if}
								</div>
							</article>
						{/each}
					</div>
				</section>
			{/if}

			<!-- Mastery Section -->
			{#if masteryCards.length > 0}
				<section class="dashboard__content-section">
					<h2 class="section-title">Mastery</h2>
					<div class="membership-cards">
						{#each masteryCards as card}
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
									<a href={card.href} class="btn-dashboard">Dashboard</a>
									{#if card.tradingRoom}
										<a href={card.tradingRoom} class="btn-room" target="_blank">Trading Room</a>
									{/if}
								</div>
							</article>
						{/each}
					</div>
				</section>
			{/if}

			<!-- Tools Section -->
			<section class="dashboard__content-section">
				<h2 class="section-title">Tools</h2>
				<div class="membership-cards">
					{#each toolsCards as card}
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
								<a href={card.href} class="btn-dashboard">Dashboard</a>
							</div>
						</article>
					{/each}
				</div>
			</section>

			<!-- Empty State - No Memberships -->
			{#if membershipCards.length === 0 && masteryCards.length === 0}
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

	</div>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * DASHBOARD HEADER - WordPress Exact Match
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__header {
		background-color: #ffffff;
		padding: 30px 40px;
		display: flex;
		justify-content: space-between;
		align-items: center;
		border-bottom: 1px solid #e0e0e0;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
	}

	.dashboard__header-left {
		flex: 1;
	}

	.dashboard__page-title {
		font-size: 32px;
		font-weight: 700;
		color: #333333;
		margin: 0;
		font-family: 'Open Sans', sans-serif;
		line-height: 1.2;
	}

	.dashboard__header-right {
		display: flex;
		align-items: center;
		gap: 15px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * LOADING SKELETON
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.loading-skeleton {
		padding: 20px 0;
	}

	.skeleton {
		background: linear-gradient(90deg, #e0e0e0 25%, #f0f0f0 50%, #e0e0e0 75%);
		background-size: 200% 100%;
		animation: skeleton-loading 1.5s infinite;
		border-radius: 8px;
	}

	.skeleton--title {
		height: 32px;
		width: 200px;
		margin-bottom: 30px;
	}

	.skeleton--card {
		height: 150px;
	}

	.skeleton-grid {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 30px;
	}

	@keyframes skeleton-loading {
		0% { background-position: 200% 0; }
		100% { background-position: -200% 0; }
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
		color: #333333;
		margin: 0 0 16px;
		font-family: 'Open Sans', sans-serif;
	}

	.empty-state__text {
		font-size: 16px;
		color: #666666;
		margin: 0 0 24px;
		max-width: 400px;
		margin-left: auto;
		margin-right: auto;
		font-family: 'Open Sans', sans-serif;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * BUTTONS - WordPress Exact Match
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.btn-orange {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		padding: 10px 20px;
		background-color: #dc7309;
		color: #ffffff;
		font-size: 14px;
		font-weight: 600;
		font-family: 'Open Sans', sans-serif;
		text-decoration: none;
		border: none;
		border-radius: 4px;
		cursor: pointer;
		transition: background-color 0.2s ease;
	}

	.btn-orange:hover {
		background-color: #c56508;
	}

	.btn-orange strong {
		font-weight: 600;
	}

	.btn-tradingroom {
		text-transform: none;
	}

	.dropdown-arrow {
		font-size: 10px;
		transition: transform 0.2s ease;
	}

	.dropdown.is-open .dropdown-arrow {
		transform: rotate(180deg);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * DROPDOWN MENU
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dropdown {
		position: relative;
		display: inline-block;
	}

	.dropdown-menu {
		position: absolute;
		top: 100%;
		right: 0;
		margin-top: 8px;
		min-width: 220px;
		background-color: #ffffff;
		border-radius: 6px;
		box-shadow: 0 4px 20px rgba(0, 0, 0, 0.15);
		z-index: 1000;
		overflow: hidden;
		animation: dropdownFadeIn 0.2s ease;
	}

	@keyframes dropdownFadeIn {
		from {
			opacity: 0;
			transform: translateY(-10px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	.dropdown-item {
		display: block;
		padding: 12px 16px;
		color: #333333;
		font-size: 14px;
		font-weight: 400;
		font-family: 'Open Sans', sans-serif;
		text-decoration: none;
		transition: background-color 0.2s ease;
	}

	.dropdown-item:hover {
		background-color: #f5f5f5;
	}

	.dropdown-item:not(:last-child) {
		border-bottom: 1px solid #eeeeee;
	}

	.dropdown-backdrop {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		z-index: 999;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * DASHBOARD CONTENT
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__content {
		padding: 0;
	}

	.dashboard__content-main {
		background-color: #efefef;
		padding: 0;
	}

	.dashboard__content-section {
		padding: 30px 40px;
		background-color: #ffffff;
		margin-bottom: 20px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * SECTION TITLES
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.section-title {
		color: #333333;
		font-size: 32px;
		font-weight: 700;
		font-family: 'Open Sans', sans-serif;
		line-height: 1.2;
		margin: 0 0 30px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * MEMBERSHIP CARDS
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.membership-cards {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 30px;
	}

	@media (max-width: 1200px) {
		.membership-cards {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	@media (max-width: 768px) {
		.membership-cards {
			grid-template-columns: 1fr;
		}

		.skeleton-grid {
			grid-template-columns: 1fr;
		}
	}

	.membership-card {
		background-color: #ffffff;
		border-radius: 8px;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
		overflow: hidden;
		transition: transform 0.3s ease, box-shadow 0.3s ease;
		display: flex;
		flex-direction: column;
	}

	.membership-card:hover {
		transform: translateY(-4px);
		box-shadow: 0 4px 16px rgba(0, 0, 0, 0.15);
	}

	.membership-card__header {
		display: flex;
		align-items: center;
		padding: 25px 20px;
		text-decoration: none;
		color: #333333;
		border-bottom: 1px solid #f0f0f0;
		transition: background-color 0.2s ease;
	}

	.membership-card__header:hover {
		background-color: #f9f9f9;
	}

	.mem_icon {
		margin-right: 15px;
	}

	.membership-card__icon {
		width: 48px;
		height: 48px;
		display: flex;
		align-items: center;
		justify-content: center;
		border-radius: 8px;
		background-color: #f5f5f5;
	}

	.mem_div {
		font-size: 18px;
		font-weight: 700;
		color: #333333;
		line-height: 1.3;
		font-family: 'Open Sans', sans-serif;
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
	 * MEMBERSHIP CARD ACTIONS
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.membership-card__actions {
		display: flex;
		padding: 20px;
		gap: 10px;
	}

	.membership-card__actions a {
		flex: 1;
		text-align: center;
		padding: 10px 15px;
		font-size: 14px;
		font-weight: 600;
		font-family: 'Open Sans', sans-serif;
		text-decoration: none;
		border-radius: 4px;
		transition: all 0.2s ease;
	}

	.btn-dashboard {
		background-color: #0984ae;
		color: #ffffff;
	}

	.btn-dashboard:hover {
		background-color: #076d8f;
	}

	.btn-room {
		background-color: #f0f0f0;
		color: #333333;
	}

	.btn-room:hover {
		background-color: #e0e0e0;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * CARD VARIANTS
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.membership-card--options .membership-card__icon {
		background-color: #fff3e6;
		color: #dc7309;
	}

	.membership-card--foundation .membership-card__icon {
		background-color: #e6f7ff;
		color: #0984ae;
	}

	.membership-card--tr3ndy .membership-card__icon {
		background-color: #f0e6ff;
		color: #7b3ff2;
	}

	.membership-card--growth .membership-card__icon {
		background-color: #e6ffe6;
		color: #28a745;
	}

	.membership-card--ww .membership-card__icon {
		background-color: #fff0f0;
		color: #dc3545;
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
		}

		.dashboard__content-section {
			padding: 20px;
		}

		.section-title {
			font-size: 24px;
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
		.dropdown-menu,
		.skeleton {
			transition: none;
			animation: none;
		}
	}
</style>

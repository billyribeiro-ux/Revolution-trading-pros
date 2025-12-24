<script lang="ts">
	import { onMount } from 'svelte';
	import { getUserMemberships, type UserMembership, type UserMembershipsResponse } from '$lib/api/user-memberships';

	// Icons for different membership types
	import IconChartLine from '@tabler/icons-svelte/icons/chart-line';
	import IconTrendingUp from '@tabler/icons-svelte/icons/trending-up';
	import IconWallet from '@tabler/icons-svelte/icons/wallet';
	import IconBell from '@tabler/icons-svelte/icons/bell';
	import IconRocket from '@tabler/icons-svelte/icons/rocket';
	import IconActivity from '@tabler/icons-svelte/icons/activity';
	import IconChevronDown from '@tabler/icons-svelte/icons/chevron-down';

	let dropdownOpen = $state(false);
	let membershipsData = $state<UserMembershipsResponse | null>(null);
	let loading = $state(true);

	// Fetch memberships on mount
	onMount(async () => {
		try {
			membershipsData = await getUserMemberships();
		} catch (error) {
			console.error('Failed to load memberships:', error);
		} finally {
			loading = false;
		}
	});

	function toggleDropdown() {
		dropdownOpen = !dropdownOpen;
	}

	function closeDropdown() {
		dropdownOpen = false;
	}

	// Get icon component based on membership slug
	function getIconForMembership(slug: string) {
		switch (slug) {
			case 'day-trading-room':
				return IconChartLine;
			case 'swing-trading-room':
				return IconTrendingUp;
			case 'small-account-mentorship':
				return IconWallet;
			case 'alerts-only':
				return IconBell;
			case 'explosive-swing':
				return IconRocket;
			case 'spx-profit-pulse':
				return IconActivity;
			default:
				return IconChartLine;
		}
	}

	// Get dashboard URL for a membership
	function getDashboardUrl(membership: UserMembership): string {
		return `/dashboard/${membership.slug}`;
	}

	// Get trading room URL for a membership
	function getTradingRoomUrl(membership: UserMembership): string {
		return `/trading-room/${membership.slug}`;
	}
</script>

<svelte:window on:click={closeDropdown} />

<!-- DASHBOARD HEADER -->
<header class="dashboard__header">
	<div class="dashboard__header-left">
		<h1 class="dashboard__page-title">Member Dashboard</h1>
	</div>
	<div class="dashboard__header-right">
		<ul class="trading-room-rules">
			<li><a href="/trading-room-rules.pdf" target="_blank" class="rules-link">Trading Room Rules</a></li>
			<li class="rules-disclaimer">By logging into any of our Live Trading Rooms, You are agreeing to our Rules of the Room.</li>
		</ul>

		<div class="dropdown" class:is-open={dropdownOpen}>
			<!-- svelte-ignore a11y_click_events_have_key_events -->
			<!-- svelte-ignore a11y_no_static_element_interactions -->
			<button
				class="btn btn-orange btn-tradingroom"
				onclick={(e) => { e.stopPropagation(); toggleDropdown(); }}
			>
				<strong>Enter a Trading Room</strong>
				<IconChevronDown size={16} />
			</button>

			{#if dropdownOpen}
				<nav class="dropdown-menu">
					<ul>
						{#if membershipsData?.tradingRooms}
							{#each membershipsData.tradingRooms as room}
								{@const Icon = getIconForMembership(room.slug)}
								<li>
									<a href={getTradingRoomUrl(room)} target="_blank">
										<span class="room-icon"><Icon size={20} /></span>
										{room.name}
									</a>
								</li>
							{/each}
						{:else}
							<li>
								<a href="/trading-room/day-trading" target="_blank">
									<span class="room-icon"><IconChartLine size={20} /></span>
									Day Trading Room
								</a>
							</li>
						{/if}
					</ul>
				</nav>
			{/if}
		</div>
	</div>
</header>

<!-- DASHBOARD CONTENT -->
<div class="dashboard__content">
	<div class="dashboard__content-main">

		<!-- MEMBERSHIPS SECTION -->
		<section class="dashboard__content-section">
			<h2 class="section-title">Memberships</h2>
			<div class="membership-cards row">

				{#if loading}
					<div class="loading-state">Loading memberships...</div>
				{:else if membershipsData?.memberships && membershipsData.memberships.length > 0}
					{#each membershipsData.memberships as membership}
						{@const Icon = getIconForMembership(membership.slug)}
						<div class="col-sm-6 col-xl-4">
							<article class="membership-card membership-card--{membership.slug}">
								<a href={getDashboardUrl(membership)} class="membership-card__header">
									<span class="mem_icon">
										<span class="membership-card__icon">
											<Icon size={32} />
										</span>
									</span>
									<span class="mem_div">{membership.name}</span>
								</a>
								<div class="membership-card__actions">
									<a href={getDashboardUrl(membership)}>Dashboard</a>
									{#if membership.type === 'trading-room'}
										<a href={getTradingRoomUrl(membership)} target="_blank">Trading Room</a>
									{:else}
										<a href={getDashboardUrl(membership)}>View Alerts</a>
									{/if}
								</div>
							</article>
						</div>
					{/each}
				{:else}
					<div class="no-memberships">
						<p>You don't have any active memberships yet.</p>
						<a href="/pricing" class="btn btn-primary">View Plans</a>
					</div>
				{/if}

			</div>
		</section>

	</div>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD HEADER
	   ═══════════════════════════════════════════════════════════════════════════ */
	.dashboard__header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		padding: 30px;
		flex-wrap: wrap;
		gap: 20px;
	}

	.dashboard__header-left {
		flex: 1;
	}

	.dashboard__page-title {
		font-size: 24px;
		font-weight: 600;
		color: #333;
		margin: 0;
		font-family: 'Open Sans', sans-serif;
	}

	.dashboard__header-right {
		display: flex;
		align-items: center;
		gap: 20px;
		flex-wrap: wrap;
	}

	/* Trading Room Rules */
	.trading-room-rules {
		list-style: none;
		margin: 0;
		padding: 0;
		text-align: right;
	}

	.rules-link {
		font-size: 14px;
		font-weight: 700;
		color: #1e73be;
		text-decoration: none;
	}

	.rules-link:hover {
		text-decoration: underline;
	}

	.rules-disclaimer {
		font-size: 11px;
		color: #666;
		margin-top: 4px;
	}

	/* Dropdown */
	.dropdown {
		position: relative;
		display: inline-block;
	}

	.btn-orange {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		background: #f99e31;
		border: 1px solid #f99e31;
		color: #fff;
		padding: 8px 16px;
		border-radius: 4px;
		font-size: 14px;
		font-weight: 700;
		cursor: pointer;
		transition: all 0.15s ease-in-out;
	}

	.btn-orange:hover {
		background: #e8901d;
		border-color: #e8901d;
	}

	.dropdown-menu {
		position: absolute;
		top: 100%;
		right: 0;
		margin-top: 4px;
		background: #fff;
		border: 1px solid #e5e7eb;
		border-radius: 4px;
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
		min-width: 220px;
		z-index: 1000;
	}

	.dropdown-menu ul {
		list-style: none;
		margin: 0;
		padding: 8px 0;
	}

	.dropdown-menu li a {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 10px 16px;
		color: #333;
		text-decoration: none;
		font-size: 14px;
		transition: background 0.15s;
	}

	.dropdown-menu li a:hover {
		background: #f5f5f5;
	}

	.room-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		background: #0984ae;
		border-radius: 50%;
		color: #fff;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD CONTENT
	   ═══════════════════════════════════════════════════════════════════════════ */
	.dashboard__content {
		padding: 0 30px 30px;
	}

	.dashboard__content-section {
		margin-bottom: 30px;
	}

	.section-title {
		font-size: 18px;
		font-weight: 600;
		color: #333;
		margin: 0 0 20px 0;
		font-family: 'Open Sans', sans-serif;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   MEMBERSHIP CARDS
	   ═══════════════════════════════════════════════════════════════════════════ */
	.membership-cards {
		display: flex;
		flex-wrap: wrap;
		gap: 20px;
	}

	.membership-cards.row {
		margin: 0 -10px;
	}

	.col-sm-6 {
		flex: 0 0 50%;
		max-width: 50%;
		padding: 0 10px;
		box-sizing: border-box;
		margin-bottom: 20px;
	}

	.col-xl-4 {
		flex: 0 0 33.333%;
		max-width: 33.333%;
	}

	@media (max-width: 991px) {
		.col-xl-4 {
			flex: 0 0 50%;
			max-width: 50%;
		}
	}

	@media (max-width: 575px) {
		.col-sm-6,
		.col-xl-4 {
			flex: 0 0 100%;
			max-width: 100%;
		}
	}

	.membership-card {
		background: #fff;
		border-radius: 5px;
		overflow: hidden;
		box-shadow: 0 5px 30px rgba(0, 0, 0, 0.1);
		transition: all 0.15s ease-in-out;
	}

	.membership-card:hover {
		box-shadow: 0 8px 40px rgba(0, 0, 0, 0.15);
	}

	.membership-card__header {
		display: flex;
		align-items: center;
		padding: 20px;
		color: #333;
		text-decoration: none;
		font-size: 17px;
		font-weight: 600;
		transition: all 0.15s ease-in-out;
	}

	.membership-card__header:hover {
		color: #0e6ac4;
	}

	.mem_icon {
		display: inline-block;
		vertical-align: middle;
		margin-right: 12px;
	}

	.membership-card__icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 50px;
		height: 50px;
		background-color: #0984ae;
		border-radius: 50%;
		color: #fff;
		transition: all 0.15s ease-in-out;
	}

	.membership-card__header:hover .membership-card__icon {
		background-color: #076787;
	}

	.mem_div {
		display: inline-block;
		vertical-align: middle;
		white-space: normal;
		width: calc(100% - 62px);
	}

	.membership-card__actions {
		display: flex;
		border-top: 1px solid #ededed;
	}

	.membership-card__actions a {
		display: block;
		flex: 1 1 auto;
		padding: 12px 15px;
		color: #1e73be;
		font-size: 14px;
		font-weight: 400;
		text-align: center;
		text-decoration: none;
		transition: all 0.15s ease-in-out;
	}

	.membership-card__actions a:hover {
		background: #f5f5f5;
		color: #0984ae;
	}

	.membership-card__actions a + a {
		border-left: 1px solid #ededed;
	}

	/* Loading & Empty States */
	.loading-state {
		padding: 40px;
		text-align: center;
		color: #666;
		font-size: 14px;
	}

	.no-memberships {
		padding: 40px;
		text-align: center;
		background: #fff;
		border-radius: 5px;
		box-shadow: 0 5px 30px rgba(0, 0, 0, 0.1);
		width: 100%;
	}

	.no-memberships p {
		margin: 0 0 20px 0;
		color: #666;
	}

	.btn-primary {
		display: inline-block;
		padding: 10px 24px;
		background: #0984ae;
		color: #fff;
		text-decoration: none;
		border-radius: 4px;
		font-weight: 600;
		transition: background 0.15s;
	}

	.btn-primary:hover {
		background: #076787;
	}
</style>

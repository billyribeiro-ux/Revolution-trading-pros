<script lang="ts">
	/**
	 * User Dashboard Page
	 * Matches WordPress WooCommerce My Account dashboard structure
	 * Shows user's active memberships organized by category
	 */
	import { onMount } from 'svelte';
	import { IconUsers } from '@tabler/icons-svelte';
	import MembershipCard from '$lib/components/dashboard/MembershipCard.svelte';
	import { getUserMemberships, type UserMembership } from '$lib/api/user-memberships';

	// State for memberships
	let tradingRooms = $state<UserMembership[]>([]);
	let alertServices = $state<UserMembership[]>([]);
	let courses = $state<UserMembership[]>([]);
	let indicators = $state<UserMembership[]>([]);
	let allMemberships = $state<UserMembership[]>([]);
	let isLoading = $state(true);
	let error = $state<string | null>(null);

	// Load memberships on mount
	onMount(async () => {
		try {
			const memberships = await getUserMemberships();
			tradingRooms = memberships.tradingRooms;
			alertServices = memberships.alertServices;
			courses = memberships.courses || [];
			indicators = memberships.indicators || [];
			allMemberships = memberships.memberships;
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load memberships';
			console.error('Error loading memberships:', e);
		} finally {
			isLoading = false;
		}
	});

	const hasMemberships = $derived(allMemberships.length > 0);
</script>

<!-- Dashboard Header -->
<div class="wc-content-sction">
	<div class="dashb_headr">
		<div class="dashb_headr-left">
			<h1 class="dashb_pg-titl">My Account</h1>
		</div>
		<div class="dashb_headr-right">
			<a href="/live-trading-rooms" class="btn btn-xs btn-link start-here-btn">
				Start Here
			</a>
		</div>
	</div>
</div>

<!-- Dashboard Content -->
<div class="wc-accontent-inner">
	{#if isLoading}
		<!-- Loading State -->
		<div class="loading-state">
			<div class="loading-spinner"></div>
			<p>Loading your memberships...</p>
		</div>
	{:else if error}
		<!-- Error State -->
		<div class="error-state">
			<p class="error-message">{error}</p>
			<button class="btn btn-xs" onclick={() => location.reload()}>
				Try Again
			</button>
		</div>
	{:else if hasMemberships}
		<!-- Memberships Grid - Matches WordPress cselt-row structure -->
		<div class="cselt-row">
			<!-- Trading Rooms Section -->
			{#if tradingRooms.length > 0}
				<h3 class="custom-head-0">Trading Rooms</h3>
				{#each tradingRooms as room (room.id)}
					<div class="col-sm4">
						<MembershipCard
							id={room.id}
							name={room.name}
							type={room.type}
							slug={room.slug}
							icon={room.icon}
						/>
					</div>
				{/each}
				<hr class="custom-hr-0" />
			{/if}

			<!-- Alert Services Section -->
			{#if alertServices.length > 0}
				<h3 class="custom-head-1">Live Alerts</h3>
				{#each alertServices as alert (alert.id)}
					<div class="col-sm4">
						<MembershipCard
							id={alert.id}
							name={alert.name}
							type={alert.type}
							slug={alert.slug}
							icon={alert.icon}
						/>
					</div>
				{/each}
				<hr class="custom-hr-1" />
			{/if}

			<!-- Courses Section -->
			{#if courses.length > 0}
				<h3 class="custom-head-2">Courses</h3>
				{#each courses as course (course.id)}
					<div class="col-sm4">
						<MembershipCard
							id={course.id}
							name={course.name}
							type={course.type}
							slug={course.slug}
							icon={course.icon}
						/>
					</div>
				{/each}
			{/if}

			<!-- Indicators Section -->
			{#if indicators.length > 0}
				<h3 class="custom-head-3">My Indicators</h3>
				<div id="my-indicators-cards">
					{#each indicators as indicator (indicator.id)}
						<div class="col-sm4">
							<MembershipCard
								id={indicator.id}
								name={indicator.name}
								type={indicator.type}
								slug={indicator.slug}
								icon={indicator.icon}
							/>
						</div>
					{/each}
				</div>
			{/if}
		</div>
	{:else}
		<!-- Empty State -->
		<div class="empty-state">
			<div class="empty-icon">
				<IconUsers size={48} />
			</div>
			<h2>No Active Memberships</h2>
			<p>You don't have any active memberships yet.</p>
			<a href="/live-trading-rooms" class="btn btn-orange">
				Explore Trading Rooms
			</a>
		</div>
	{/if}
</div>

<style>
	/* Content Section */
	.wc-content-sction {
		width: 100%;
		margin: auto;
		padding: 20px;
	}

	/* Dashboard Header */
	.dashb_headr {
		background-color: #fff;
		border-bottom: 1px solid #dbdbdb;
		max-width: 100%;
		padding: 20px;
		display: flex;
		flex-wrap: wrap;
		justify-content: space-between;
	}

	.dashb_headr-left,
	.dashb_headr-right {
		align-items: center;
		display: flex;
		flex-direction: row;
	}

	.dashb_headr-right {
		flex-direction: row-reverse;
		justify-content: flex-end;
	}

	.dashb_pg-titl {
		color: #333;
		font-family: 'Open Sans Condensed', sans-serif;
		font-size: 36px;
		font-weight: 700;
		margin: 0;
	}

	/* Start Here Button */
	.start-here-btn {
		font-size: 14px;
		line-height: 18px;
		padding: 8px 14px;
		font-weight: 600;
		color: #0984ae;
		background: #f4f4f4;
		border-color: transparent;
		text-decoration: none;
		border-radius: 5px;
		transition: all 0.15s ease-in-out;
	}

	.start-here-btn:hover {
		color: #0984ae;
		background: #e7e7e7;
	}

	/* Inner Content */
	.wc-accontent-inner {
		padding: 4% 2%;
		background: #fff;
		border-radius: 5px;
		box-shadow: 0 1px 2px rgb(0 0 0 / 15%);
		position: relative;
		margin: 20px;
	}

	/* Loading State */
	.loading-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		min-height: 200px;
		color: #666;
	}

	.loading-spinner {
		width: 40px;
		height: 40px;
		border: 3px solid #e5e7eb;
		border-top-color: #0984ae;
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin-bottom: 16px;
	}

	@keyframes spin {
		to { transform: rotate(360deg); }
	}

	/* Membership Grid - cselt-row */
	.cselt-row {
		display: flex;
		flex-flow: row wrap;
		align-items: flex-end;
		column-gap: 30px;
		row-gap: 40px;
	}

	.cselt-row .col-sm4 {
		flex: 0 0 auto;
		width: 100%;
		max-width: 320px;
	}

	/* My Indicators Cards Section */
	#my-indicators-cards {
		display: flex;
		flex-flow: row wrap;
		gap: 30px;
		width: 100%;
	}

	#my-indicators-cards .col-sm4 {
		flex: 0 0 auto;
		width: 100%;
		max-width: 320px;
	}

	/* Section Headers */
	.custom-head-0,
	.custom-head-1,
	.custom-head-2,
	.custom-head-3 {
		color: #333;
		font-weight: 700;
		font-size: 20px;
		font-family: 'Open Sans', sans-serif;
		width: 100%;
	}

	.custom-head-1,
	.custom-head-2,
	.custom-head-3 {
		margin-top: 30px;
	}

	/* Section Dividers */
	.custom-hr-0,
	.custom-hr-1 {
		border-top: 1px solid #dbdbdb;
		width: 100%;
		margin: 20px 0;
	}

	/* Error State */
	.error-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		min-height: 200px;
		text-align: center;
	}

	.error-message {
		color: #eb5757;
		margin-bottom: 16px;
	}

	/* Empty State */
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		min-height: 300px;
		text-align: center;
		color: #666;
	}

	.empty-icon {
		width: 80px;
		height: 80px;
		border-radius: 50%;
		background: #f4f4f4;
		display: flex;
		align-items: center;
		justify-content: center;
		margin-bottom: 24px;
		color: #0984ae;
	}

	.empty-state h2 {
		font-size: 1.5rem;
		color: #333;
		margin-bottom: 8px;
	}

	.empty-state p {
		margin-bottom: 24px;
		color: #666;
	}

	.btn-orange {
		background: #f99e31;
		border-color: #f99e31;
		color: #fff;
		font-weight: 700;
		font-family: 'Open Sans', sans-serif;
		padding: 10px 20px;
		border-radius: 5px;
		text-decoration: none;
		transition: all 0.15s ease-in-out;
	}

	.btn-orange:hover {
		background: #f88b09;
		border-color: #f88b09;
	}

	/* Responsive */
	@media screen and (min-width: 577px) {
		.dashb_headr-left,
		.dashb_headr-right {
			flex-direction: row;
		}

		.dashb_headr-right {
			flex-direction: row-reverse;
			justify-content: flex-end;
		}
	}

	@media screen and (min-width: 468px) {
		.cselt-row .col-sm4,
		#my-indicators-cards .col-sm4 {
			width: 48%;
		}
	}

	@media screen and (min-width: 768px) {
		.cselt-row .col-sm4,
		#my-indicators-cards .col-sm4 {
			width: 31%;
		}
	}

	@media screen and (min-width: 1280px) {
		.dashb_headr {
			padding: 30px;
		}
	}
</style>

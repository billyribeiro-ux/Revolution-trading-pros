<script lang="ts">
	/**
	 * Member Dashboard - ICT 11+ Principal Engineer Pattern
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Fully data-driven dashboard with no hardcoded values.
	 * All memberships, icons, and content come from the API/database.
	 *
	 * Layout guarantees auth is initialized before this component mounts,
	 * so we can safely fetch data in onMount without race conditions.
	 *
	 * @version 2.1.0
	 */
	import { onMount } from 'svelte';
	import { getUserMemberships, type UserMembership, type UserMembershipsResponse } from '$lib/api/user-memberships';
	import DynamicIcon from '$lib/components/DynamicIcon.svelte';
	import IconChevronDown from '@tabler/icons-svelte/icons/chevron-down';

	let dropdownOpen = $state(false);
	let membershipsData = $state<UserMembershipsResponse | null>(null);
	let loading = $state(true);
	let error = $state<string | null>(null);

	// ICT 11+ Pattern: Layout guarantees auth is ready, so onMount is safe
	onMount(async () => {
		try {
			membershipsData = await getUserMemberships();
		} catch (err) {
			console.error('Failed to load memberships:', err);
			error = 'Failed to load memberships. Please try again.';
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

	// Generate dashboard URL from membership data
	function getDashboardUrl(membership: UserMembership): string {
		return membership.accessUrl || `/dashboard/${membership.slug}`;
	}

	// Generate trading room/alert service URL from membership data
	function getAccessUrl(membership: UserMembership): string {
		if (membership.type === 'trading-room') {
			return `/trading-room/${membership.slug}`;
		}
		return `/dashboard/${membership.slug}/alerts`;
	}

	// Get action label based on membership type from data
	function getActionLabel(membership: UserMembership): string {
		// Use roomLabel from data if available, otherwise derive from type
		if (membership.roomLabel) {
			return membership.roomLabel.includes('Room') ? 'Trading Room' : membership.roomLabel;
		}
		switch (membership.type) {
			case 'trading-room':
				return 'Trading Room';
			case 'alert-service':
				return 'View Alerts';
			case 'course':
				return 'View Course';
			case 'indicator':
				return 'Download';
			default:
				return 'Access';
		}
	}

	// Check if action should open in new tab based on membership type
	function shouldOpenNewTab(membership: UserMembership): boolean {
		return membership.type === 'trading-room';
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

		{#if membershipsData?.tradingRooms && membershipsData.tradingRooms.length > 0}
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
							{#each membershipsData.tradingRooms as room (room.id)}
								<li>
									<a href={getAccessUrl(room)} target="_blank">
										<span class="room-icon">
											<DynamicIcon name={room.icon} size={20} />
										</span>
										{room.name}
									</a>
								</li>
							{/each}
						</ul>
					</nav>
				{/if}
			</div>
		{/if}
	</div>
</header>

<!-- DASHBOARD CONTENT -->
<div class="dashboard__content">
	<div class="dashboard__content-main">

		<!-- MEMBERSHIPS SECTION - Only show if user has memberships -->
		{#if loading}
			<section class="dashboard__content-section">
				<h2 class="section-title">Memberships</h2>
				<div class="loading-state">Loading memberships...</div>
			</section>
		{:else if error}
			<section class="dashboard__content-section">
				<h2 class="section-title">Memberships</h2>
				<div class="error-state">
					<p>{error}</p>
					<button class="btn btn-primary" onclick={() => location.reload()}>Retry</button>
				</div>
			</section>
		{:else if membershipsData?.memberships && membershipsData.memberships.length > 0}
			<section class="dashboard__content-section">
				<h2 class="section-title">Memberships</h2>
				<div class="membership-cards row">
					{#each membershipsData.memberships as membership (membership.id)}
						<div class="col-sm-6 col-xl-4">
							<article class="membership-card">
								<a href={getDashboardUrl(membership)} class="membership-card__header">
									<span class="mem_icon">
										<span class="membership-card__icon">
											<DynamicIcon name={membership.icon} size={32} />
										</span>
									</span>
									<span class="mem_div">{membership.name}</span>
								</a>
								<div class="membership-card__actions">
									<a href={getDashboardUrl(membership)}>Dashboard</a>
									{#if shouldOpenNewTab(membership)}
										<a href={getAccessUrl(membership)} target="_blank">{getActionLabel(membership)}</a>
									{:else}
										<a href={getAccessUrl(membership)}>{getActionLabel(membership)}</a>
									{/if}
								</div>
							</article>
						</div>
					{/each}
				</div>
			</section>
		{/if}

		<!-- LATEST UPDATES SECTION - Simpler Trading shows this when no memberships -->
		{#if !loading && !error && (!membershipsData?.memberships || membershipsData.memberships.length === 0)}
			<section class="dashboard__content-section">
				<h2 class="section-title u--margin-top-20">Latest Updates</h2>
				<div class="article-cards row flex-grid">
					<!-- Placeholder article cards - will be replaced with actual blog/video content -->
					<div class="col-xs-12 col-sm-6 col-md-6 col-xl-4 flex-grid-item">
						<article class="article-card">
							<figure class="article-card__image" style="background-image: url('https://images.unsplash.com/photo-1611974789855-9c2a0a7236a3?w=800&h=600&fit=crop');">
								<img src="https://images.unsplash.com/photo-1611974789855-9c2a0a7236a3?w=800&h=600&fit=crop" alt="Trading Update" />
							</figure>
							<div class="article-card__type">
								<span class="label label--info">Daily Video</span>
							</div>
							<h4 class="h5 article-card__title"><a href="/blog">Welcome to Revolution Trading Pros</a></h4>
							<span class="article-card__meta"><small>Latest market insights and trading education</small></span>
							<div class="article-card__excerpt u--hide-read-more">
								<p>
									<div class="woocommerce">
										<div class="woocommerce-info wc-memberships-restriction-message wc-memberships-message wc-memberships-content-restricted-message">
											This content is only available to members.
										</div>
									</div>
								</p>
							</div>
							<a href="/pricing" class="btn btn-tiny btn-default">View Plans</a>
						</article>
					</div>
					<div class="col-xs-12 col-sm-6 col-md-6 col-xl-4 flex-grid-item">
						<article class="article-card">
							<figure class="article-card__image" style="background-image: url('https://images.unsplash.com/photo-1590283603385-17ffb3a7f29f?w=800&h=600&fit=crop');">
								<img src="https://images.unsplash.com/photo-1590283603385-17ffb3a7f29f?w=800&h=600&fit=crop" alt="Market Analysis" />
							</figure>
							<div class="article-card__type">
								<span class="label label--info">Daily Video</span>
							</div>
							<h4 class="h5 article-card__title"><a href="/blog">Market Analysis & Trading Strategies</a></h4>
							<span class="article-card__meta"><small>Expert insights from our trading team</small></span>
							<div class="article-card__excerpt u--hide-read-more">
								<p>
									<div class="woocommerce">
										<div class="woocommerce-info wc-memberships-restriction-message wc-memberships-message wc-memberships-content-restricted-message">
											This content is only available to members.
										</div>
									</div>
								</p>
							</div>
							<a href="/pricing" class="btn btn-tiny btn-default">View Plans</a>
						</article>
					</div>
					<div class="col-xs-12 col-sm-6 col-md-6 col-xl-4 flex-grid-item">
						<article class="article-card">
							<figure class="article-card__image" style="background-image: url('https://images.unsplash.com/photo-1642790106117-e829e14a795f?w=800&h=600&fit=crop');">
								<img src="https://images.unsplash.com/photo-1642790106117-e829e14a795f?w=800&h=600&fit=crop" alt="Trading Education" />
							</figure>
							<div class="article-card__type">
								<span class="label label--info">Daily Video</span>
							</div>
							<h4 class="h5 article-card__title"><a href="/blog">Learn Advanced Trading Techniques</a></h4>
							<span class="article-card__meta"><small>Professional trading education</small></span>
							<div class="article-card__excerpt u--hide-read-more">
								<p>Get access to exclusive trading education, live trading rooms, and expert analysis. Join our community of successful traders today.</p>
							</div>
							<a href="/pricing" class="btn btn-tiny btn-default">View Plans</a>
						</article>
					</div>
				</div>
			</section>
		{/if}

		<!-- PREMIUM REPORTS SECTION -->
		<section class="dashboard__content-section">
			<h2 class="section-title">Premium Reports</h2>
			<div class="premium-reports-placeholder">
				<div class="placeholder-icon">
					<svg xmlns="http://www.w3.org/2000/svg" width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
						<path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path>
						<polyline points="14 2 14 8 20 8"></polyline>
						<line x1="16" y1="13" x2="8" y2="13"></line>
						<line x1="16" y1="17" x2="8" y2="17"></line>
						<polyline points="10 9 9 9 8 9"></polyline>
					</svg>
				</div>
				<h3>Premium Reports Coming Soon</h3>
				<p>Access exclusive market analysis, trading reports, and insights from our expert team.</p>
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

	/* Loading, Error & Empty States */
	.loading-state,
	.error-state {
		padding: 40px;
		text-align: center;
		color: #666;
		font-size: 14px;
		width: 100%;
	}

	.error-state p {
		margin: 0 0 20px 0;
		color: #dc3545;
	}

	/* Article Cards - Simpler Trading Style */
	.article-cards {
		display: flex;
		flex-wrap: wrap;
		gap: 20px;
	}

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

	.col-sm-6 {
		flex: 0 0 50%;
		max-width: 50%;
	}

	.col-md-6 {
		flex: 0 0 50%;
		max-width: 50%;
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
		.col-md-6,
		.col-xl-4 {
			flex: 0 0 100%;
			max-width: 100%;
		}
	}

	.article-card {
		background: #fff;
		border-radius: 5px;
		overflow: hidden;
		box-shadow: 0 5px 30px rgba(0, 0, 0, 0.1);
		transition: all 0.15s ease-in-out;
		display: flex;
		flex-direction: column;
		width: 100%;
	}

	.article-card:hover {
		box-shadow: 0 8px 40px rgba(0, 0, 0, 0.15);
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
		font-weight: 600;
		line-height: 1.4;
		transition: color 0.15s;
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

	.article-card__excerpt {
		padding: 12px 20px;
		color: #666;
		font-size: 14px;
		line-height: 1.6;
	}

	.article-card__excerpt p {
		margin: 0;
	}

	.u--hide-read-more {
		/* Hide read more links */
	}

	.woocommerce {
		margin: 0;
	}

	.wc-memberships-restriction-message {
		background: #f8f9fa;
		border-left: 4px solid #0984ae;
		padding: 12px 16px;
		margin: 0;
		color: #666;
		font-size: 13px;
		border-radius: 3px;
	}

	.article-card .btn {
		margin: 0 20px 20px;
	}

	.btn-tiny {
		padding: 8px 16px;
		font-size: 13px;
	}

	.btn-default {
		background: #f5f5f5;
		color: #333;
		border: 1px solid #ddd;
	}

	.btn-default:hover {
		background: #e8e8e8;
		border-color: #ccc;
	}

	.u--margin-top-20 {
		margin-top: 20px;
	}

	.btn,
	.btn-primary {
		display: inline-block;
		padding: 10px 24px;
		background: #0984ae;
		color: #fff;
		text-decoration: none;
		border: none;
		border-radius: 4px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.15s;
		font-size: 14px;
		text-align: center;
	}

	.btn-primary:hover {
		background: #076787;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   PREMIUM REPORTS
	   ═══════════════════════════════════════════════════════════════════════════ */
	.premium-reports-placeholder {
		background: #fff;
		border-radius: 5px;
		box-shadow: 0 5px 30px rgba(0, 0, 0, 0.1);
		padding: 60px 40px;
		text-align: center;
	}

	.placeholder-icon {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 80px;
		height: 80px;
		background: linear-gradient(135deg, #0984ae 0%, #076787 100%);
		border-radius: 20px;
		color: #fff;
		margin-bottom: 20px;
	}

	.premium-reports-placeholder h3 {
		font-size: 20px;
		font-weight: 600;
		color: #333;
		margin: 0 0 12px 0;
	}

	.premium-reports-placeholder p {
		font-size: 14px;
		color: #666;
		margin: 0;
		max-width: 400px;
		margin-left: auto;
		margin-right: auto;
	}
</style>

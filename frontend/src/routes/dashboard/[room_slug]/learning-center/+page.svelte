<!--
	URL: /dashboard/[room_slug]/learning-center

	Learning Center Page - Dynamic Room
	═══════════════════════════════════════════════════════════════════════════
	Apple ICT 11+ Principal Engineer Implementation

	Educational resources and training materials for any trading room.
	Uses reusable components.

	@version 2.0.0
	@author Revolution Trading Pros
-->
<script lang="ts">
	import TradingRoomHeader from '$lib/components/dashboard/TradingRoomHeader.svelte';
	import LearningCenterFilter from '$lib/components/dashboard/LearningCenterFilter.svelte';
	import VideoGrid from '$lib/components/dashboard/VideoGrid.svelte';
	import type { PageData } from './$types';

	// Server-side data
	let { data }: { data: PageData } = $props();

	// Reactive state from server data
	let videos = $derived(data.videos || []);
	let meta = $derived(data.meta || { current_page: 1, per_page: 9, total: 0, last_page: 1 });
	let activeFilter = $derived(data.activeFilter || 'all');
	let error = $derived(data.error);
	let roomName = $derived(data.roomName || 'Trading Room');
	let roomSlug = $derived(data.roomSlug || '');

	// Pagination derived values
	let currentPage = $derived(meta.current_page);
	let totalPages = $derived(meta.last_page);

	// Category options matching WordPress exactly
	const categories = [
		{ id: '529', label: 'Trade Setups & Strategies' },
		{ id: '528', label: 'Methodology' },
		{ id: '329', label: 'Member Webinar' },
		{ id: '2932', label: 'Trade & Money Management/Trading Plan' },
		{ id: '531', label: 'Indicators' },
		{ id: '3260', label: 'Options' },
		{ id: '469', label: 'foundation' },
		{ id: '527', label: 'Fundamentals' },
		{ id: '522', label: 'Simpler Tech' },
		{ id: '2929', label: 'Charting/Indicators/Tools' },
		{ id: '530', label: 'Charting' },
		{ id: '3515', label: 'Drama Free Daytrades' },
		{ id: '3516', label: 'Quick Hits Daytrades' },
		{ id: '537', label: 'Psychology' },
		{ id: '775', label: 'Trading Platform' },
		{ id: '3055', label: 'Calls' },
		{ id: '447', label: 'ThinkorSwim' },
		{ id: '446', label: 'TradeStation' },
		{ id: '776', label: 'Charting Software' },
		{ id: '772', label: 'Trading Computer' },
		{ id: '3057', label: 'Calls Puts Credit Spreads' },
		{ id: '3056', label: 'Puts' },
		{ id: '3514', label: 'Profit Recycling' },
		{ id: '791', label: 'Trade Strategies' },
		{ id: '774', label: 'Website Support' },
		{ id: '2927', label: 'Options Strategies (Level 2 & 3)' },
		{ id: '457', label: 'Crypto' },
		{ id: '2931', label: 'Fibonacci & Options Trading' },
		{ id: '2928', label: 'Pricing/Volatility' },
		{ id: '459', label: 'Crypto Indicators & Trading' },
		{ id: '771', label: 'Browser Support' },
		{ id: '2930', label: 'Earnings & Options Expiration' }
	];
</script>

<svelte:head>
	<title>Learning Center | {roomName} | Revolution Trading Pros</title>
	<meta
		name="description"
		content="Access educational resources, training materials, and courses for {roomName}."
	/>
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<TradingRoomHeader {roomName} startHereUrl="/dashboard/{roomSlug}/start-here" />

<div class="dashboard__content">
	<div class="dashboard__content-main">
		<!-- Category Filter -->
		<LearningCenterFilter {categories} {activeFilter} />

		<!-- Section Title -->
		<section class="dashboard__content-section">
			<h2 class="section-title">{roomName} Learning Center<span> | </span><span>Overview</span></h2>
			<p></p>
		</section>

		<!-- Error State -->
		{#if error}
			<div class="error-message">
				<p>Unable to load videos. Please try again later.</p>
				<p class="error-detail">{error}</p>
			</div>
		{/if}

		<!-- Video Grid with Pagination -->
		<VideoGrid
			{videos}
			{currentPage}
			{totalPages}
			basePath="/dashboard/{roomSlug}/learning-center"
			emptyMessage="No learning resources found"
		/>
	</div>
</div>

<style>
	/* Dashboard Content */
	.dashboard__content {
		padding: 0;
	}

	.dashboard__content-main {
		padding-left: 10px;
		padding-right: 10px;
	}

	/* Section Title */
	.dashboard__content-section {
		padding: 30px 0 20px;
	}

	.section-title {
		font-size: 24px;
		font-weight: 700;
		color: #333;
		margin: 0;
	}

	.section-title span {
		color: #666;
		font-weight: 400;
	}

	/* Error State */
	.error-message {
		background: #fef2f2;
		border: 1px solid #fecaca;
		border-radius: 8px;
		padding: 20px;
		margin-bottom: 20px;
		text-align: center;
	}

	.error-message p {
		margin: 0;
		color: #dc2626;
	}

	.error-detail {
		font-size: 12px;
		color: #6b7280;
		margin-top: 8px !important;
	}
</style>

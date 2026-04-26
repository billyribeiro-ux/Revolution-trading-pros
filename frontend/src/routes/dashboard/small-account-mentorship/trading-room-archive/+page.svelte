<!--
	URL: /dashboard/day-trading-room/trading-room-archive
	
	Day Trading Room Archive Page
	═══════════════════════════════════════════════════════════════════════════
	Apple ICT 11+ Principal Engineer Implementation - Svelte 5
	
	Uses reusable TradingRoomArchive component.
	
	@version 5.0.0
	@author Revolution Trading Pros
-->
<script lang="ts">
	import TradingRoomHeader from '$lib/components/dashboard/TradingRoomHeader.svelte';
	import TradingRoomArchive from '$lib/components/dashboard/TradingRoomArchive.svelte';
	import type { ArchivePageData } from './+page.server';

	// Server data - extends parent layout data
	interface Props {
		data: ArchivePageData;
	}

	let props: Props = $props();
	let data = $derived(props.data);
</script>

<svelte:head>
	<title>Trading Room Archives | Small Account Mentorship | Revolution Trading Pros</title>
	<meta
		name="description"
		content="Access recordings of past live trading sessions and chat logs."
	/>
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<TradingRoomHeader
	roomName="Small Account Mentorship"
	pageTitle="Trading Room Archives"
	startHereUrl="/dashboard/small-account-mentorship/start-here"
/>

<!-- FIX-2026-04-26: data-unavailable banner so empty grid isn't mistaken for "no content". -->
{#if data.dataUnavailable}
	<div class="data-unavailable" role="status" aria-live="polite">
		<p>Video data temporarily unavailable. Check back soon.</p>
		{#if data.reason}
			<p class="data-unavailable__reason">({data.reason})</p>
		{/if}
	</div>
{/if}

<TradingRoomArchive
	roomSlug="small-account-mentorship"
	roomName="Small Account Mentorship"
	videos={data.videos || []}
	meta={data.meta || { current_page: 1, per_page: 50, total: 0, last_page: 1 }}
	search={data.search}
	error={data.error}
/>

<style>
	/* FIX-2026-04-26: info-style banner for backend-unavailable state. */
	.data-unavailable {
		background: #eff6ff;
		border: 1px solid #bfdbfe;
		border-radius: 8px;
		padding: 16px 20px;
		margin: 1rem 1.5rem;
		text-align: center;
	}

	.data-unavailable p {
		margin: 0;
		color: #1e40af;
	}

	.data-unavailable__reason {
		font-size: 12px;
		color: #6b7280;
		margin-top: 6px;
	}
</style>

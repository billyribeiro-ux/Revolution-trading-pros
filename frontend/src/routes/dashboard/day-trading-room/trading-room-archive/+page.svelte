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
	<title>Trading Room Archives | Day Trading Room | Revolution Trading Pros</title>
	<meta
		name="description"
		content="Access recordings of past live trading sessions and chat logs."
	/>
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<TradingRoomHeader
	roomName="Day Trading Room"
	pageTitle="Trading Room Archives"
	startHereUrl="/dashboard/day-trading-room/start-here"
/>

<TradingRoomArchive
	roomSlug="day-trading-room"
	roomName="Day Trading Room"
	videos={data.videos || []}
	meta={data.meta || { current_page: 1, per_page: 50, total: 0, last_page: 1 }}
	search={data.search}
	error={data.error}
/>

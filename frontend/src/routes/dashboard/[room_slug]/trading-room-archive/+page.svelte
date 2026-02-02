<!--
	URL: /dashboard/[room_slug]/trading-room-archive
	
	Dynamic Trading Room Archive Page
	═══════════════════════════════════════════════════════════════════════════
	Apple ICT 11+ Principal Engineer Implementation - Svelte 5
	
	Uses reusable TradingRoomArchive component for all trading rooms:
	- Day Trading Room
	- Swing Trading Room
	- Small Account Mentorship
	
	@version 2.0.0
	@author Revolution Trading Pros
-->
<script lang="ts">
	import TradingRoomHeader from '$lib/components/dashboard/TradingRoomHeader.svelte';
	import TradingRoomArchive from '$lib/components/dashboard/TradingRoomArchive.svelte';
	import type { DynamicArchivePageData } from './+page.server';

	// Server data - extends parent layout data
	interface Props {
		data: DynamicArchivePageData;
	}

	let props: Props = $props();
	let data = $derived(props.data);
</script>

<svelte:head>
	<title>Trading Room Archives | {data.roomName} | Revolution Trading Pros</title>
	<meta
		name="description"
		content="Access recordings of past live trading sessions and chat logs from {data.roomName}."
	/>
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<TradingRoomHeader
	roomName={data.roomName}
	pageTitle="Trading Room Archives"
	startHereUrl={data.startHereUrl}
/>

<TradingRoomArchive
	roomSlug={data.roomSlug}
	roomName={data.roomName}
	videos={data.videos || []}
	meta={data.meta || { current_page: 1, per_page: 50, total: 0, last_page: 1 }}
	search={data.search}
	error={data.error}
/>

<!--
	URL: /dashboard/swing-trading-room/trading-room-archive

	Swing Trading Room Archive Page
	===============================================================================
	Apple ICT 11+ Principal Engineer Implementation - Svelte 5

	Uses reusable TradingRoomArchive component.

	@version 1.0.0
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

	let { data }: Props = $props();
</script>

<svelte:head>
	<title>Trading Room Archives | Swing Trading Room | Revolution Trading Pros</title>
	<meta
		name="description"
		content="Access recordings of past live swing trading sessions and chat logs."
	/>
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<TradingRoomHeader
	roomName="Swing Trading Room"
	pageTitle="Trading Room Archives"
	startHereUrl="/dashboard/swing-trading-room/start-here"
/>

<TradingRoomArchive
	roomSlug="swing-trading-room"
	roomName="Swing Trading Room"
	videos={data.videos || []}
	meta={data.meta || { current_page: 1, per_page: 50, total: 0, last_page: 1 }}
	search={data.search}
	error={data.error}
/>

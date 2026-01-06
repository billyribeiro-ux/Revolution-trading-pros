<!--
	URL: /dashboard/day-trading-room/trading-room-archive
	
	Trading Room Archive Page
	═══════════════════════════════════════════════════════════════════════════
	Apple ICT 11+ Principal Engineer Implementation
	
	Archive of past live trading sessions and recordings.
	
	@version 2.0.0
	@author Revolution Trading Pros
-->
<script lang="ts">
	import TradingRoomHeader from '$lib/components/dashboard/TradingRoomHeader.svelte';
</script>

<svelte:head>
	<title>Trading Room Archives | Day Trading Room | Revolution Trading Pros</title>
	<meta name="description" content="Access recordings of past live trading sessions and chat logs." />
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<TradingRoomHeader 
	roomName="Day Trading Room" 
	startHereUrl="/dashboard/day-trading-room/start-here" 
/>

<div class="dashboard__content">
	<div class="dashboard__content-main">
		<section class="dashboard__content-section">
			<h2 class="section-title">Session Recordings</h2>
			<p>Review past live trading sessions, chat logs, and trade analysis from our expert traders.</p>
		</section>
	</div>
</div>

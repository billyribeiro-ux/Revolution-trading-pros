<script lang="ts">
	import Hero from '$lib/components/Hero.svelte';
	import WhySection from '$lib/components/sections/WhySection.svelte';
	import TradingRoomsSection from '$lib/components/sections/TradingRoomsSection.svelte';
	import AlertServicesSection from '$lib/components/sections/AlertServicesSection.svelte';
	import MentorshipSection from '$lib/components/sections/MentorshipSection.svelte';
	import LatestBlogsSection from '$lib/components/sections/LatestBlogsSection.svelte';
	import CTASection from '$lib/components/sections/CTASection.svelte';
	import type { PageData } from './$types';

	export let data: PageData;
</script>

<svelte:head>
	<title>Revolution Trading Pros | Live Trading Rooms, Alerts & Pro Tools</title>
	<meta
		name="description"
		content="Professional trading education and tools. Live trading rooms, SPX alerts, courses, and indicators built for disciplined traders."
	/>
	<link rel="canonical" href="https://revolutiontradingpros.com/" />
</svelte:head>

<Hero />

<WhySection />

<TradingRoomsSection />

<AlertServicesSection />

<MentorshipSection />

<LatestBlogsSection posts={data.posts} />

<CTASection />

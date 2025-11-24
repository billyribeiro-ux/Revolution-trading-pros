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

<!-- ⭐⭐⭐ THIS IS THE ONLY PLACE THE SEO GOES ⭐⭐⭐ -->
<svelte:head>
	<title>Revolution Trading Pros | Live Trading Rooms, Alerts & Pro Tools</title>

	<meta
		name="description"
		content="Professional trading education and tools. Live trading rooms, SPX alerts, courses, and indicators built for disciplined traders."
	/>
	<link rel="canonical" href="https://revolutiontradingpros.com/" />

	<!-- ⭐ ADDED BELOW (nothing removed!) ⭐ -->

	<!-- Open Graph -->
	<meta property="og:title" content="Revolution Trading Pros | Live Trading Rooms, Alerts & Pro Tools" />
	<meta property="og:description" content="Professional trading education and tools. Live trading rooms, SPX alerts, courses, and indicators built for disciplined traders." />
	<meta property="og:type" content="website" />
	<meta property="og:url" content="https://revolutiontradingpros.com/" />
	<meta property="og:image" content="https://revolutiontradingpros.com/og-image.webp" />

	<!-- Twitter -->
	<meta name="twitter:card" content="summary_large_image" />
	<meta name="twitter:title" content="Revolution Trading Pros | Live Trading Rooms, Alerts & Pro Tools" />
	<meta name="twitter:description" content="Professional trading rooms, SPX alerts, courses, and indicators designed for disciplined traders." />
	<meta name="twitter:image" content="https://revolutiontradingpros.com/og-image.webp" />

	<!-- Robots + Viewport -->
	<meta name="robots" content="index, follow" />
	<meta name="viewport" content="width=device-width, initial-scale=1" />

	<!-- JSON-LD Schema -->
	<script type="application/ld+json">
		{
			"@context": "https://schema.org",
			"@type": "Organization",
			"name": "Revolution Trading Pros",
			"url": "https://revolutiontradingpros.com",
			"logo": "https://revolutiontradingpros.com/logo.webp",
			"sameAs": [
				"https://www.instagram.com/revolutiontradingpros",
				"https://twitter.com/revtradingpros"
			]
		}
	</script>
</svelte:head>
<!-- ⭐⭐⭐ END OF SEO AREA ⭐⭐⭐ -->

<!-- ⭐ NOTHING BELOW WAS TOUCHED ⭐ -->

<Hero />

<WhySection />

<TradingRoomsSection />

<AlertServicesSection />

<MentorshipSection />

<LatestBlogsSection posts={data.posts} />

<CTASection />

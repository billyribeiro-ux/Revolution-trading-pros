<script lang="ts">
	/**
	 * Homepage - Enterprise Performance Optimized
	 * Zero blocking operations, instant render
	 */
	// Critical - Load immediately
	import Hero from '$lib/components/sections/Hero.svelte';
	import LazySection from '$lib/components/LazySection.svelte';
	
	// Below-the-fold - Lazy load
	import TradingRoomsSection from '$lib/components/sections/TradingRoomsSection.svelte';
	import AlertServicesSection from '$lib/components/sections/AlertServicesSection.svelte';
	import WhySection from '$lib/components/sections/WhySection.svelte';
	import MentorshipSection from '$lib/components/sections/MentorshipSection.svelte';
	import TestimonialsSection from '$lib/components/sections/TestimonialsSection.svelte';
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

	<!-- Canonical -->
	<link rel="canonical" href="https://revolutiontradingpros.com/" />

	<!-- Open Graph -->
	<meta
		property="og:title"
		content="Revolution Trading Pros | Live Trading Rooms, Alerts & Pro Tools"
	/>
	<meta
		property="og:description"
		content="Professional trading rooms, SPX alerts, courses, and indicators designed for disciplined traders."
	/>
	<meta property="og:type" content="website" />
	<meta property="og:url" content="https://revolutiontradingpros.com/" />
	<meta property="og:image" content="https://revolutiontradingpros.com/og-image.webp" />

	<!-- Twitter -->
	<meta name="twitter:card" content="summary_large_image" />
	<meta
		name="twitter:title"
		content="Revolution Trading Pros | Live Trading Rooms, Alerts & Pro Tools"
	/>
	<meta
		name="twitter:description"
		content="Professional trading education, live rooms, alerts, and premium indicators."
	/>
	<meta name="twitter:image" content="https://revolutiontradingpros.com/og-image.webp" />

	<!-- Viewport + Robots -->
	<meta name="viewport" content="width=device-width, initial-scale=1" />
	<meta name="robots" content="index, follow" />
	
	<!-- Critical Resource Preloading (L11+ Optimization) -->
	<link rel="modulepreload" href="/src/lib/components/sections/Hero.svelte" />
	<link rel="prefetch" href="/src/lib/components/sections/TradingRoomsSection.svelte" as="script" />

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

<!-- Hero Section - Critical, loads immediately -->
<Hero />

<!-- Below-the-fold sections - Lazy loaded for performance -->
<LazySection rootMargin="300px">
	<TradingRoomsSection />
</LazySection>

<LazySection rootMargin="300px">
	<AlertServicesSection />
</LazySection>

<LazySection rootMargin="300px">
	<WhySection />
</LazySection>

<LazySection rootMargin="300px">
	<MentorshipSection />
</LazySection>

<LazySection rootMargin="300px">
	<TestimonialsSection />
</LazySection>

<LazySection rootMargin="300px">
	<LatestBlogsSection posts={data.posts} />
</LazySection>

<LazySection rootMargin="300px">
	<CTASection />
</LazySection>

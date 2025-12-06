<script lang="ts">
	/**
	 * Homepage - Enterprise Performance Optimized + SEO Enhanced
	 */
	import Hero from '$lib/components/sections/Hero.svelte';
	import LazySection from '$lib/components/LazySection.svelte';
	import TradingRoomsSection from '$lib/components/sections/TradingRoomsSection.svelte';
	import AlertServicesSection from '$lib/components/sections/AlertServicesSection.svelte';
	import WhySection from '$lib/components/sections/WhySection.svelte';
	import MentorshipSection from '$lib/components/sections/MentorshipSection.svelte';
	import TestimonialsSection from '$lib/components/sections/TestimonialsSection.svelte';
	import LatestBlogsSection from '$lib/components/sections/LatestBlogsSection.svelte';
	import CTASection from '$lib/components/sections/CTASection.svelte';
	import SocialMediaSection from '$lib/components/sections/SocialMediaSection.svelte';
	import SEOHead from '$lib/components/SEOHead.svelte';
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();

	const homepageSchema = [
		{
			'@context': 'https://schema.org',
			'@type': 'FinancialService',
			name: 'Revolution Trading Pros',
			description: 'Professional trading education platform',
			url: 'https://revolutiontradingpros.com'
		}
	];
</script>

<SEOHead
	title="Live Trading Rooms, Alerts & Pro Tools"
	description="Professional trading education and tools."
	canonical="/"
	ogType="website"
	schema={homepageSchema}
/>

<Hero />

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

<!-- Blog section renders immediately for SEO -->
<LatestBlogsSection posts={data.posts} />

<LazySection rootMargin="300px">
	<CTASection />
</LazySection>

<LazySection rootMargin="300px">
	<SocialMediaSection />
</LazySection>

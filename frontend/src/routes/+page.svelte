<script lang="ts">
	/**
	 * Homepage - Enterprise Performance Optimized + SEO Enhanced
	 * ICT11+ Fix: Removed LazySection - render all sections directly for reliability
	 */
	import Hero from '$lib/components/sections/Hero.svelte';
	import TradingRoomsSection from '$lib/components/sections/TradingRoomsSection.svelte';
	import AlertServicesSection from '$lib/components/sections/AlertServicesSection.svelte';
	import IndicatorsSection from '$lib/components/sections/IndicatorsSection.svelte';
	import CoursesSection from '$lib/components/sections/CoursesSection.svelte';
	import WhySection from '$lib/components/sections/WhySection.svelte';
	import MentorshipSection from '$lib/components/sections/MentorshipSection.svelte';
	import TestimonialsSection from '$lib/components/sections/TestimonialsSection.svelte';
	import LatestBlogsSection from '$lib/components/sections/LatestBlogsSection.svelte';
	import CTASection from '$lib/components/sections/CTASection.svelte';
	import SocialMediaSection from '$lib/components/sections/SocialMediaSection.svelte';
	import SEOHead from '$lib/components/SEOHead.svelte';
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();

	// Simple posts access
	let posts = $derived(data.posts || []);

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
<TradingRoomsSection />
<AlertServicesSection />
<IndicatorsSection />
<CoursesSection />
<WhySection />
<MentorshipSection />
<TestimonialsSection />
<LatestBlogsSection posts={posts} />
<CTASection />
<SocialMediaSection />

<script lang="ts">
	/**
	 * Homepage - Enterprise Performance Optimized + SEO Enhanced
	 * ICT11+ Performance: Uses streamed data for blog posts to eliminate TTFB blocking
	 */
	import Hero from '$lib/components/sections/Hero.svelte';
	import LazySection from '$lib/components/LazySection.svelte';
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

	// ICT11+ Performance: Handle streamed posts data
	let streamedPosts = $state<any[]>([]);
	
	// Update posts when streamed data arrives
	$effect(() => {
		if (data.streamed?.posts) {
			data.streamed.posts.then((resolved: any[]) => {
				if (resolved?.length > 0) {
					streamedPosts = resolved;
				}
			});
		}
	});

	// Derive posts from either streamed data or initial data
	let posts = $derived(streamedPosts.length > 0 ? streamedPosts : (data.posts || []));

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

<!-- Professional Trading Indicators Section -->
<LazySection rootMargin="300px">
	<IndicatorsSection />
</LazySection>

<!-- Expert-Led Trading Courses Section -->
<LazySection rootMargin="300px">
	<CoursesSection />
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
<LatestBlogsSection posts={posts} />

<LazySection rootMargin="300px">
	<CTASection />
</LazySection>

<LazySection rootMargin="300px">
	<SocialMediaSection />
</LazySection>

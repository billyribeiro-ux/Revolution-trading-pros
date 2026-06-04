<!--
	URL: /
-->

<script lang="ts">
	/**
	 * Homepage - Enterprise Performance Optimized + SEO Enhanced
	 * Posts are loaded server-side. If the backend is unavailable, the loader
	 * returns an empty list so the public page stays quiet and resilient.
	 */
	import type { PageData } from './$types';
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

	interface Props {
		data: PageData;
	}
	let props: Props = $props();

	const posts = $derived(props.data?.posts ?? []);
</script>

<Hero />
<TradingRoomsSection />
<AlertServicesSection />
<IndicatorsSection />
<CoursesSection />
<WhySection />
<MentorshipSection />
<TestimonialsSection />
<LatestBlogsSection {posts} />
<CTASection />
<SocialMediaSection />

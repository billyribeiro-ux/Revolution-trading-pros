<!--
	URL: /
-->

<script lang="ts">
	/**
	 * Homepage - Enterprise Performance Optimized + SEO Enhanced
	 * ICT11+ Fix: Client-side posts fetch fallback for production
	 */
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import type { PageData } from './$types';
	import type { Post } from '$lib/types/post';
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

	// ICT 11+ CORB Fix: Use same-origin endpoints
	const API_URL = '';

	interface Props {
		data: PageData;
	}
	let props: Props = $props();

	// Client-side fallback posts (only used when SSR returned empty)
	let clientPosts = $state<Post[]>([]);

	// Prefer SSR data; fall back to the client-side fetch result when SSR returned empty.
	const posts = $derived(props.data?.posts?.length ? props.data.posts : clientPosts);

	// Client-side fallback fetch if SSR returned empty.
	onMount(async () => {
		if (!browser) return;
		if (props.data?.posts?.length) return;
		try {
			const res = await fetch(`${API_URL}/api/posts?per_page=6`);
			if (res.ok) {
				const json = await res.json();
				clientPosts = json.data || [];
			}
		} catch {
			// Network errors handled gracefully - posts remain empty.
		}
	});
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

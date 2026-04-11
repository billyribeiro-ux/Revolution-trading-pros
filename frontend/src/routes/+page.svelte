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
	import type { PageProps } from './$types';

	// ICT 11+ CORB Fix: Use same-origin endpoints
	const API_URL = '';

	// Use the SvelteKit-generated `PageProps` so `data` is typed from
	// the matching `+page.server.ts` `load` return without manual interfaces.
	let { data }: PageProps = $props();

	// Client-side fallback (only populated if SSR returned no posts). Kept as
	// a separate `$state` so the displayed `posts` can `$derived` from server
	// data — never copying it locally — and only fall through to the client
	// fetch when the server source is genuinely empty. This preserves
	// reactivity to `invalidate()` / `invalidateAll()` against the server load.
	let clientFallbackPosts = $state<typeof data.posts>([]);

	const posts = $derived(
		data.posts && data.posts.length > 0 ? data.posts : clientFallbackPosts
	);

	// Client-side fallback fetch if SSR returned empty (production safety net).
	onMount(async () => {
		if (!browser) return;
		if (data.posts && data.posts.length > 0) return;
		try {
			const res = await fetch(`${API_URL}/api/posts?per_page=6`);
			if (res.ok) {
				const json = await res.json();
				clientFallbackPosts = json.data || [];
			}
		} catch {
			// Network errors handled gracefully — `posts` stays empty so the
			// `<LatestBlogsSection>` shows its empty state.
		}
	});

	// NOTE: SEO (meta tags + JSON-LD) is owned by the unified SEO layer via
	// `+page.server.ts` → `page.data.seo` → `+layout.svelte` <Seo>. Do NOT emit
	// head tags or structured data from this component.
</script>

<!-- SEO: Visually-hidden canonical H1 guarantees exactly one h1 per page
	 (the Hero component uses marketing typography and does not own the h1). -->
<h1 class="sr-only">
	Revolution Trading Pros — Professional Trading Education, Live Trading Rooms,
	and Institutional-Grade Indicators
</h1>

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

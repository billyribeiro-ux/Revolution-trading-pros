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
	import SEOHead from '$lib/components/SEOHead.svelte';
	import MarketingFooter from '$lib/components/sections/MarketingFooter.svelte';

	// ICT 11+ CORB Fix: Use same-origin endpoints
	const API_URL = '';

	interface Props {
		data: any;
	}
	let props: Props = $props();

	// Posts state - use SSR data or fetch client-side
	let posts = $state<any[]>([]);

	// Sync posts with data.posts reactively
	$effect(() => {
		if (props.data.posts && props.data.posts.length > 0) {
			posts = props.data.posts;
		}
	});

	// Client-side fallback fetch if SSR returned empty
	// ICT 7: SSR should always fetch from production API - client fetch is backup only
	onMount(async () => {
		if (posts.length === 0 && browser) {
			try {
				const res = await fetch(`${API_URL}/api/posts?per_page=6`);
				if (res.ok) {
					const json = await res.json();
					posts = json.data || [];
				}
			} catch {
				// ICT 7: Network errors handled gracefully - posts remain empty
				// Component renders without posts (shows empty state or placeholder)
			}
		}
	});

	const siteUrl = import.meta.env['VITE_SITE_URL'] || 'https://revolution-trading-pros.pages.dev';

	const homepageSchema = [
		{
			'@context': 'https://schema.org',
			'@type': 'FinancialService',
			name: 'Revolution Trading Pros',
			description: 'Professional trading education platform',
			url: siteUrl
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
<LatestBlogsSection {posts} />
<CTASection />
<SocialMediaSection />

<MarketingFooter />

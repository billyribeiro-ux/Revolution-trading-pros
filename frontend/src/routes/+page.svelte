<script lang="ts">
	import Hero from '$lib/components/Hero.svelte';
	import TradingRoomsSection from '$lib/components/sections/TradingRoomsSection.svelte';
	import AlertServicesSection from '$lib/components/sections/AlertServicesSection.svelte';
	import WhySection from '$lib/components/sections/WhySection.svelte';
	import MentorshipSection from '$lib/components/sections/MentorshipSection.svelte';
	import LatestBlogsSection from '$lib/components/sections/LatestBlogsSection.svelte';
	import CTASection from '$lib/components/sections/CTASection.svelte';
	import SEOHead from '$lib/components/SEOHead.svelte';
	import type { PageData } from './$types';

	export let data: PageData;

	// Homepage structured data for rich snippets
	const homepageSchema = [
		{
			'@context': 'https://schema.org',
			'@type': 'FinancialService',
			'@id': 'https://revolutiontradingpros.com/#financialservice',
			name: 'Revolution Trading Pros',
			description:
				'Professional trading education platform offering live trading rooms, SPX alerts, courses, and institutional-grade indicators for disciplined traders.',
			url: 'https://revolutiontradingpros.com',
			logo: 'https://revolutiontradingpros.com/logo.webp',
			image: 'https://revolutiontradingpros.com/og-image.webp',
			telephone: '+1-800-RTP-TRADE',
			priceRange: '$$',
			areaServed: {
				'@type': 'Country',
				name: 'United States'
			},
			serviceType: ['Trading Education', 'Live Trading Rooms', 'Trading Alerts', 'Trading Courses'],
			sameAs: [
				'https://www.instagram.com/revolutiontradingpros',
				'https://twitter.com/revtradingpros',
				'https://www.youtube.com/@RevolutionTradingPros',
				'https://www.linkedin.com/company/revolution-trading-pros'
			],
			aggregateRating: {
				'@type': 'AggregateRating',
				ratingValue: '4.9',
				reviewCount: '2847',
				bestRating: '5',
				worstRating: '1'
			}
		},
		{
			'@context': 'https://schema.org',
			'@type': 'ItemList',
			'@id': 'https://revolutiontradingpros.com/#services',
			name: 'Trading Services',
			description: 'Professional trading services offered by Revolution Trading Pros',
			itemListElement: [
				{
					'@type': 'ListItem',
					position: 1,
					name: 'Live Trading Rooms',
					url: 'https://revolutiontradingpros.com/live-trading-rooms/day-trading'
				},
				{
					'@type': 'ListItem',
					position: 2,
					name: 'SPX Profit Pulse Alerts',
					url: 'https://revolutiontradingpros.com/alerts/spx-profit-pulse'
				},
				{
					'@type': 'ListItem',
					position: 3,
					name: 'Trading Courses',
					url: 'https://revolutiontradingpros.com/courses'
				},
				{
					'@type': 'ListItem',
					position: 4,
					name: 'Trading Indicators',
					url: 'https://revolutiontradingpros.com/indicators'
				}
			]
		}
	];
</script>

<SEOHead
	title="Live Trading Rooms, Alerts & Pro Tools"
	description="Professional trading education and tools. Live trading rooms, SPX alerts, courses, and indicators built for disciplined traders. Join 10,000+ successful traders."
	canonical="/"
	ogType="website"
	ogImage="/og-image.webp"
	ogImageAlt="Revolution Trading Pros - Professional Trading Education Platform"
	keywords={[
		'trading education',
		'live trading rooms',
		'SPX alerts',
		'day trading',
		'swing trading',
		'options trading',
		'trading courses',
		'trading indicators',
		'stock market education',
		'professional trading'
	]}
	schema={homepageSchema}
	preconnect={['https://fonts.googleapis.com', 'https://fonts.gstatic.com']}
	dnsPrefetch={['https://www.google-analytics.com', 'https://www.googletagmanager.com']}
/>

<!-- Hero Section with animated chart background -->
<Hero />

<!-- Trading Rooms Section -->
<TradingRoomsSection />

<!-- Alert Services Section -->
<AlertServicesSection />

<!-- Why Choose Us Section -->
<WhySection />

<!-- Mentorship Section -->
<MentorshipSection />

<!-- Latest Blog Posts Section -->
<LatestBlogsSection posts={data.posts} />

<!-- Call to Action Section -->
<CTASection />

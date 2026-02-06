<!--
	URL: /resources
-->

<script lang="ts">
	import SEOHead from '$lib/components/SEOHead.svelte';
	import MarketingFooter from '$lib/components/sections/MarketingFooter.svelte';

	// Resources page structured data
	const resourcesSchema = [
		{
			'@context': 'https://schema.org',
			'@type': 'CollectionPage',
			'@id': 'https://revolution-trading-pros.pages.dev/resources/#collection',
			name: 'Free Trading Resources',
			description:
				'Free trading resources, guides, calculators, and educational tools for traders.',
			url: 'https://revolution-trading-pros.pages.dev/resources',
			mainEntity: {
				'@type': 'ItemList',
				itemListElement: [
					{
						'@type': 'ListItem',
						position: 1,
						name: 'Beginner Trading Guides',
						description: 'Complete guides for new traders including account setup and basics.'
					},
					{
						'@type': 'ListItem',
						position: 2,
						name: 'Free Trading Tools',
						description:
							'Position size calculator, risk/reward calculator, and trading journal templates.'
					},
					{
						'@type': 'ListItem',
						position: 3,
						name: 'Video Tutorials',
						description: 'Platform setup, chart reading, and live trade walkthroughs.'
					},
					{
						'@type': 'ListItem',
						position: 4,
						name: 'Market Analysis',
						description: 'Weekly recaps, economic calendar, and key levels to watch.'
					},
					{
						'@type': 'ListItem',
						position: 5,
						name: 'Trading Psychology',
						description: 'Managing emotions, developing discipline, and building confidence.'
					}
				]
			}
		}
	];
</script>

<SEOHead
	title="Free Trading Resources & Education Tools"
	description="Free trading resources, guides, calculators, and tools. Start your trading education with our comprehensive resource library. Position size calculators, trading journal templates, and more."
	canonical="/resources"
	ogType="website"
	ogImage="/og-image.webp"
	ogImageAlt="Revolution Trading Pros Free Trading Resources"
	keywords={[
		'free trading resources',
		'trading guides',
		'trading calculators',
		'position size calculator',
		'trading journal',
		'trading education',
		'beginner trading',
		'trading tools',
		'market analysis',
		'trading psychology'
	]}
	schema={resourcesSchema}
/>

<!-- Hero - 2026 Mobile-First Responsive -->
<section
	class="resources-hero relative bg-gradient-to-br from-rtp-blue via-rtp-primary to-rtp-emerald text-white"
>
	<div class="absolute inset-0 bg-black/20"></div>
	<div class="resources-hero__content relative max-w-7xl mx-auto text-center">
		<h1 class="resources-hero__title font-heading font-bold">Free Trading Resources</h1>
		<p class="resources-hero__subtitle text-white/90 max-w-3xl mx-auto">
			Everything you need to start your trading journey. Guides, tools, and educational content.
		</p>
	</div>
</section>

<!-- Resource Categories - 2026 Mobile-First Responsive -->
<section class="resources-section bg-rtp-bg">
	<div class="resources-container max-w-7xl mx-auto">
		<div class="resources-grid">
			<!-- Beginner Guides -->
			<div
				class="resource-card bg-rtp-surface rounded-2xl shadow-lg hover:shadow-xl transition-all"
			>
				<div
					class="resource-card__icon bg-rtp-emerald/10 rounded-xl flex items-center justify-center"
				>
					<svg
						class="w-8 h-8 text-rtp-emerald"
						fill="none"
						stroke="currentColor"
						viewBox="0 0 24 24"
					>
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"
						/>
					</svg>
				</div>
				<h3 class="resource-card__title font-heading font-bold text-rtp-text">Beginner Guides</h3>
				<ul class="resource-card__list text-rtp-muted">
					<li class="resource-card__item flex items-start gap-2">
						<svg
							class="resource-card__check text-rtp-emerald flex-shrink-0"
							fill="currentColor"
							viewBox="0 0 20 20"
						>
							<path
								fill-rule="evenodd"
								d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
								clip-rule="evenodd"
							/>
						</svg>
						<span>Complete Beginner's Guide to Trading</span>
					</li>
					<li class="resource-card__item flex items-start gap-2">
						<svg
							class="resource-card__check text-rtp-emerald flex-shrink-0"
							fill="currentColor"
							viewBox="0 0 20 20"
						>
							<path
								fill-rule="evenodd"
								d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
								clip-rule="evenodd"
							/>
						</svg>
						<span>How to Set Up Your Trading Account</span>
					</li>
					<li class="resource-card__item flex items-start gap-2">
						<svg
							class="resource-card__check text-rtp-emerald flex-shrink-0"
							fill="currentColor"
							viewBox="0 0 20 20"
						>
							<path
								fill-rule="evenodd"
								d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
								clip-rule="evenodd"
							/>
						</svg>
						<span>Understanding Options Basics</span>
					</li>
					<li class="resource-card__item flex items-start gap-2">
						<svg
							class="resource-card__check text-rtp-emerald flex-shrink-0"
							fill="currentColor"
							viewBox="0 0 20 20"
						>
							<path
								fill-rule="evenodd"
								d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
								clip-rule="evenodd"
							/>
						</svg>
						<span>Risk Management for New Traders</span>
					</li>
				</ul>
			</div>

			<!-- Free Tools -->
			<div
				class="resource-card bg-rtp-surface rounded-2xl shadow-lg hover:shadow-xl transition-all"
			>
				<div
					class="resource-card__icon bg-rtp-primary/10 rounded-xl flex items-center justify-center"
				>
					<svg
						class="w-8 h-8 text-rtp-primary"
						fill="none"
						stroke="currentColor"
						viewBox="0 0 24 24"
					>
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M9 7h6m0 10v-3m-3 3h.01M9 17h.01M9 14h.01M12 14h.01M15 11h.01M12 11h.01M9 11h.01M7 21h10a2 2 0 002-2V5a2 2 0 00-2-2H7a2 2 0 00-2 2v14a2 2 0 002 2z"
						/>
					</svg>
				</div>
				<h3 class="text-2xl font-heading font-bold text-rtp-text mb-4">Free Tools</h3>
				<ul class="space-y-3 text-rtp-muted">
					<li class="flex items-start gap-2">
						<svg
							class="w-5 h-5 text-rtp-primary flex-shrink-0 mt-0.5"
							fill="currentColor"
							viewBox="0 0 20 20"
						>
							<path
								fill-rule="evenodd"
								d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
								clip-rule="evenodd"
							/>
						</svg>
						<span>Position Size Calculator</span>
					</li>
					<li class="flex items-start gap-2">
						<svg
							class="w-5 h-5 text-rtp-primary flex-shrink-0 mt-0.5"
							fill="currentColor"
							viewBox="0 0 20 20"
						>
							<path
								fill-rule="evenodd"
								d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
								clip-rule="evenodd"
							/>
						</svg>
						<span>Risk/Reward Calculator</span>
					</li>
					<li class="flex items-start gap-2">
						<svg
							class="w-5 h-5 text-rtp-primary flex-shrink-0 mt-0.5"
							fill="currentColor"
							viewBox="0 0 20 20"
						>
							<path
								fill-rule="evenodd"
								d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
								clip-rule="evenodd"
							/>
						</svg>
						<span>Options Profit Calculator</span>
					</li>
					<li class="flex items-start gap-2">
						<svg
							class="w-5 h-5 text-rtp-primary flex-shrink-0 mt-0.5"
							fill="currentColor"
							viewBox="0 0 20 20"
						>
							<path
								fill-rule="evenodd"
								d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
								clip-rule="evenodd"
							/>
						</svg>
						<span>Trading Journal Template</span>
					</li>
				</ul>
			</div>

			<!-- Video Tutorials -->
			<div class="bg-rtp-surface rounded-2xl p-8 shadow-lg hover:shadow-xl transition-all">
				<div class="w-16 h-16 bg-rtp-indigo/10 rounded-xl flex items-center justify-center mb-6">
					<svg
						class="w-8 h-8 text-rtp-indigo"
						fill="none"
						stroke="currentColor"
						viewBox="0 0 24 24"
					>
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z"
						/>
					</svg>
				</div>
				<h3 class="text-2xl font-heading font-bold text-rtp-text mb-4">Video Tutorials</h3>
				<ul class="space-y-3 text-rtp-muted">
					<li class="flex items-start gap-2">
						<svg
							class="w-5 h-5 text-rtp-indigo flex-shrink-0 mt-0.5"
							fill="currentColor"
							viewBox="0 0 20 20"
						>
							<path
								fill-rule="evenodd"
								d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
								clip-rule="evenodd"
							/>
						</svg>
						<span>Platform Setup Walkthrough</span>
					</li>
					<li class="flex items-start gap-2">
						<svg
							class="w-5 h-5 text-rtp-indigo flex-shrink-0 mt-0.5"
							fill="currentColor"
							viewBox="0 0 20 20"
						>
							<path
								fill-rule="evenodd"
								d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
								clip-rule="evenodd"
							/>
						</svg>
						<span>Reading Charts for Beginners</span>
					</li>
					<li class="flex items-start gap-2">
						<svg
							class="w-5 h-5 text-rtp-indigo flex-shrink-0 mt-0.5"
							fill="currentColor"
							viewBox="0 0 20 20"
						>
							<path
								fill-rule="evenodd"
								d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
								clip-rule="evenodd"
							/>
						</svg>
						<span>Your First Option Trade</span>
					</li>
					<li class="flex items-start gap-2">
						<svg
							class="w-5 h-5 text-rtp-indigo flex-shrink-0 mt-0.5"
							fill="currentColor"
							viewBox="0 0 20 20"
						>
							<path
								fill-rule="evenodd"
								d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
								clip-rule="evenodd"
							/>
						</svg>
						<span>Live Trade Walkthrough</span>
					</li>
				</ul>
			</div>

			<!-- Market Analysis -->
			<div class="bg-rtp-surface rounded-2xl p-8 shadow-lg hover:shadow-xl transition-all">
				<div class="w-16 h-16 bg-rtp-blue/10 rounded-xl flex items-center justify-center mb-6">
					<svg class="w-8 h-8 text-rtp-blue" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"
						/>
					</svg>
				</div>
				<h3 class="text-2xl font-heading font-bold text-rtp-text mb-4">Market Analysis</h3>
				<ul class="space-y-3 text-rtp-muted">
					<li class="flex items-start gap-2">
						<svg
							class="w-5 h-5 text-rtp-blue flex-shrink-0 mt-0.5"
							fill="currentColor"
							viewBox="0 0 20 20"
						>
							<path
								fill-rule="evenodd"
								d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
								clip-rule="evenodd"
							/>
						</svg>
						<span>Weekly Market Recap</span>
					</li>
					<li class="flex items-start gap-2">
						<svg
							class="w-5 h-5 text-rtp-blue flex-shrink-0 mt-0.5"
							fill="currentColor"
							viewBox="0 0 20 20"
						>
							<path
								fill-rule="evenodd"
								d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
								clip-rule="evenodd"
							/>
						</svg>
						<span>Economic Calendar Explained</span>
					</li>
					<li class="flex items-start gap-2">
						<svg
							class="w-5 h-5 text-rtp-blue flex-shrink-0 mt-0.5"
							fill="currentColor"
							viewBox="0 0 20 20"
						>
							<path
								fill-rule="evenodd"
								d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
								clip-rule="evenodd"
							/>
						</svg>
						<span>Key Levels to Watch</span>
					</li>
					<li class="flex items-start gap-2">
						<svg
							class="w-5 h-5 text-rtp-blue flex-shrink-0 mt-0.5"
							fill="currentColor"
							viewBox="0 0 20 20"
						>
							<path
								fill-rule="evenodd"
								d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
								clip-rule="evenodd"
							/>
						</svg>
						<span>Sector Rotation Analysis</span>
					</li>
				</ul>
			</div>

			<!-- Trading Psychology -->
			<div class="bg-rtp-surface rounded-2xl p-8 shadow-lg hover:shadow-xl transition-all">
				<div class="w-16 h-16 bg-rtp-emerald/10 rounded-xl flex items-center justify-center mb-6">
					<svg
						class="w-8 h-8 text-rtp-emerald"
						fill="none"
						stroke="currentColor"
						viewBox="0 0 24 24"
					>
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M9.663 17h4.673M12 3v1m6.364 1.636l-.707.707M21 12h-1M4 12H3m3.343-5.657l-.707-.707m2.828 9.9a5 5 0 117.072 0l-.548.547A3.374 3.374 0 0014 18.469V19a2 2 0 11-4 0v-.531c0-.895-.356-1.754-.988-2.386l-.548-.547z"
						/>
					</svg>
				</div>
				<h3 class="text-2xl font-heading font-bold text-rtp-text mb-4">Trading Psychology</h3>
				<ul class="space-y-3 text-rtp-muted">
					<li class="flex items-start gap-2">
						<svg
							class="w-5 h-5 text-rtp-emerald flex-shrink-0 mt-0.5"
							fill="currentColor"
							viewBox="0 0 20 20"
						>
							<path
								fill-rule="evenodd"
								d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
								clip-rule="evenodd"
							/>
						</svg>
						<span>Managing Trading Emotions</span>
					</li>
					<li class="flex items-start gap-2">
						<svg
							class="w-5 h-5 text-rtp-emerald flex-shrink-0 mt-0.5"
							fill="currentColor"
							viewBox="0 0 20 20"
						>
							<path
								fill-rule="evenodd"
								d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
								clip-rule="evenodd"
							/>
						</svg>
						<span>Developing Discipline</span>
					</li>
					<li class="flex items-start gap-2">
						<svg
							class="w-5 h-5 text-rtp-emerald flex-shrink-0 mt-0.5"
							fill="currentColor"
							viewBox="0 0 20 20"
						>
							<path
								fill-rule="evenodd"
								d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
								clip-rule="evenodd"
							/>
						</svg>
						<span>Dealing with Losing Streaks</span>
					</li>
					<li class="flex items-start gap-2">
						<svg
							class="w-5 h-5 text-rtp-emerald flex-shrink-0 mt-0.5"
							fill="currentColor"
							viewBox="0 0 20 20"
						>
							<path
								fill-rule="evenodd"
								d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
								clip-rule="evenodd"
							/>
						</svg>
						<span>Building Confidence</span>
					</li>
				</ul>
			</div>

			<!-- FAQ -->
			<div class="bg-rtp-surface rounded-2xl p-8 shadow-lg hover:shadow-xl transition-all">
				<div class="w-16 h-16 bg-rtp-primary/10 rounded-xl flex items-center justify-center mb-6">
					<svg
						class="w-8 h-8 text-rtp-primary"
						fill="none"
						stroke="currentColor"
						viewBox="0 0 24 24"
					>
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M8.228 9c.549-1.165 2.03-2 3.772-2 2.21 0 4 1.343 4 3 0 1.4-1.278 2.575-3.006 2.907-.542.104-.994.54-.994 1.093m0 3h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
						/>
					</svg>
				</div>
				<h3 class="text-2xl font-heading font-bold text-rtp-text mb-4">FAQ & Support</h3>
				<ul class="space-y-3 text-rtp-muted">
					<li class="flex items-start gap-2">
						<svg
							class="w-5 h-5 text-rtp-primary flex-shrink-0 mt-0.5"
							fill="currentColor"
							viewBox="0 0 20 20"
						>
							<path
								fill-rule="evenodd"
								d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
								clip-rule="evenodd"
							/>
						</svg>
						<span>Frequently Asked Questions</span>
					</li>
					<li class="flex items-start gap-2">
						<svg
							class="w-5 h-5 text-rtp-primary flex-shrink-0 mt-0.5"
							fill="currentColor"
							viewBox="0 0 20 20"
						>
							<path
								fill-rule="evenodd"
								d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
								clip-rule="evenodd"
							/>
						</svg>
						<span>Glossary of Trading Terms</span>
					</li>
					<li class="flex items-start gap-2">
						<svg
							class="w-5 h-5 text-rtp-primary flex-shrink-0 mt-0.5"
							fill="currentColor"
							viewBox="0 0 20 20"
						>
							<path
								fill-rule="evenodd"
								d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
								clip-rule="evenodd"
							/>
						</svg>
						<span>Community Guidelines</span>
					</li>
					<li class="flex items-start gap-2">
						<svg
							class="w-5 h-5 text-rtp-primary flex-shrink-0 mt-0.5"
							fill="currentColor"
							viewBox="0 0 20 20"
						>
							<path
								fill-rule="evenodd"
								d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
								clip-rule="evenodd"
							/>
						</svg>
						<span>Contact Support</span>
					</li>
				</ul>
			</div>
		</div>
	</div>
</section>

<!-- CTA - 2026 Mobile-First Responsive -->
<section class="resources-cta bg-gradient-to-br from-rtp-blue to-rtp-primary text-white">
	<div class="resources-cta__content max-w-4xl mx-auto text-center">
		<h2 class="resources-cta__title font-heading font-bold">Ready to Take the Next Step?</h2>
		<p class="resources-cta__text text-white/90 max-w-2xl mx-auto">
			Join our community and get access to professional trading rooms, courses, and indicators.
		</p>
		<a
			href="/signup"
			class="resources-cta__btn inline-block bg-white text-rtp-primary rounded-lg font-semibold hover:bg-rtp-bg transition-all hover:scale-105 shadow-lg"
		>
			Get Started Free
		</a>
	</div>
</section>

<MarketingFooter />

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   2026 Mobile-First Responsive Styles - Resources Page
	   Breakpoints: xs(360px), sm(640px), md(768px), lg(1024px), xl(1280px)
	   ═══════════════════════════════════════════════════════════════════════════ */

	/* Hero Section - Mobile First */
	:global(.resources-hero) {
		padding: 5rem 1rem 3rem;
		padding-top: calc(5rem + env(safe-area-inset-top, 0px));
		padding-left: calc(1rem + env(safe-area-inset-left, 0px));
		padding-right: calc(1rem + env(safe-area-inset-right, 0px));
	}

	:global(.resources-hero__content) {
		padding: 0 1rem;
	}

	:global(.resources-hero__title) {
		font-size: clamp(1.75rem, 6vw, 3.5rem);
		margin-bottom: 1rem;
		line-height: 1.15;
	}

	:global(.resources-hero__subtitle) {
		font-size: clamp(1rem, 3vw, 1.5rem);
		line-height: 1.5;
	}

	/* Tablet and up */
	@media (min-width: 640px) {
		:global(.resources-hero) {
			padding: 6rem 1.5rem 4rem;
		}

		:global(.resources-hero__title) {
			margin-bottom: 1.5rem;
		}
	}

	/* Desktop */
	@media (min-width: 768px) {
		:global(.resources-hero) {
			padding: 8rem 2rem 6rem;
		}
	}

	@media (min-width: 1024px) {
		:global(.resources-hero) {
			padding: 10rem 2rem 8rem;
		}
	}

	/* Resources Section - Mobile First */
	:global(.resources-section) {
		padding: 3rem 1rem;
		padding-left: calc(1rem + env(safe-area-inset-left, 0px));
		padding-right: calc(1rem + env(safe-area-inset-right, 0px));
	}

	:global(.resources-container) {
		padding: 0;
	}

	/* Resources Grid - Mobile First (single column) */
	:global(.resources-grid) {
		display: grid;
		grid-template-columns: 1fr;
		gap: 1.25rem;
	}

	/* sm: 640px - 2 columns */
	@media (min-width: 640px) {
		:global(.resources-section) {
			padding: 4rem 1.5rem;
		}

		:global(.resources-grid) {
			grid-template-columns: repeat(2, 1fr);
			gap: 1.5rem;
		}
	}

	/* md: 768px - Still 2 columns but larger gap */
	@media (min-width: 768px) {
		:global(.resources-section) {
			padding: 5rem 2rem;
		}

		:global(.resources-grid) {
			gap: 2rem;
		}
	}

	/* lg: 1024px - 3 columns */
	@media (min-width: 1024px) {
		:global(.resources-grid) {
			grid-template-columns: repeat(3, 1fr);
		}
	}

	/* Resource Card - Mobile First */
	:global(.resource-card) {
		padding: 1.25rem;
	}

	:global(.resource-card__icon) {
		width: 3rem;
		height: 3rem;
		margin-bottom: 1rem;
	}

	:global(.resource-card__icon svg) {
		width: 1.5rem;
		height: 1.5rem;
	}

	:global(.resource-card__title) {
		font-size: 1.25rem;
		margin-bottom: 0.75rem;
	}

	:global(.resource-card__list) {
		display: flex;
		flex-direction: column;
		gap: 0.625rem;
	}

	:global(.resource-card__item) {
		gap: 0.5rem;
	}

	:global(.resource-card__check) {
		width: 1.125rem;
		height: 1.125rem;
		margin-top: 0.125rem;
	}

	/* sm: 640px+ */
	@media (min-width: 640px) {
		:global(.resource-card) {
			padding: 1.5rem;
		}

		:global(.resource-card__icon) {
			width: 3.5rem;
			height: 3.5rem;
			margin-bottom: 1.25rem;
		}

		:global(.resource-card__icon svg) {
			width: 1.75rem;
			height: 1.75rem;
		}

		:global(.resource-card__title) {
			font-size: 1.375rem;
			margin-bottom: 1rem;
		}

		:global(.resource-card__list) {
			gap: 0.75rem;
		}

		:global(.resource-card__check) {
			width: 1.25rem;
			height: 1.25rem;
		}
	}

	/* md: 768px+ */
	@media (min-width: 768px) {
		:global(.resource-card) {
			padding: 2rem;
		}

		:global(.resource-card__icon) {
			width: 4rem;
			height: 4rem;
			margin-bottom: 1.5rem;
		}

		:global(.resource-card__icon svg) {
			width: 2rem;
			height: 2rem;
		}

		:global(.resource-card__title) {
			font-size: 1.5rem;
		}
	}

	/* CTA Section - Mobile First */
	:global(.resources-cta) {
		padding: 3rem 1rem;
		padding-bottom: calc(3rem + env(safe-area-inset-bottom, 0px));
	}

	:global(.resources-cta__content) {
		padding: 0 1rem;
	}

	:global(.resources-cta__title) {
		font-size: clamp(1.5rem, 5vw, 3rem);
		margin-bottom: 1rem;
		line-height: 1.2;
	}

	:global(.resources-cta__text) {
		font-size: 1rem;
		margin-bottom: 1.5rem;
		line-height: 1.6;
	}

	:global(.resources-cta__btn) {
		padding: 0.875rem 2rem;
		font-size: 1rem;
		/* 44px min touch target */
		min-height: 44px;
		min-width: 44px;
		display: inline-flex;
		align-items: center;
		justify-content: center;
	}

	/* sm: 640px+ */
	@media (min-width: 640px) {
		:global(.resources-cta) {
			padding: 4rem 1.5rem;
		}

		:global(.resources-cta__text) {
			font-size: 1.125rem;
			margin-bottom: 2rem;
		}

		:global(.resources-cta__btn) {
			padding: 1rem 2.5rem;
			font-size: 1.125rem;
		}
	}

	/* md: 768px+ */
	@media (min-width: 768px) {
		:global(.resources-cta) {
			padding: 5rem 2rem;
		}

		:global(.resources-cta__title) {
			margin-bottom: 1.5rem;
		}

		:global(.resources-cta__btn) {
			padding: 1rem 3rem;
		}
	}

	/* Touch targets - All interactive elements 44x44px minimum */
	:global(.resource-card a),
	:global(.resource-card button) {
		min-height: 44px;
		min-width: 44px;
		display: inline-flex;
		align-items: center;
	}
</style>

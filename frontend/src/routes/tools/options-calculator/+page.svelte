<script lang="ts">
	import Calculator from '$lib/options-calculator/components/Calculator.svelte';
	import '$lib/options-calculator/styles/calculator-theme.css';
	import {
		generateStructuredData,
		generateFAQStructuredData,
		generateOGMeta
	} from '$lib/options-calculator/utils/seo.js';

	const structuredData = generateStructuredData();
	const faqData = generateFAQStructuredData();
	const ogMeta = generateOGMeta();
</script>

<svelte:head>
	<title>Black-Scholes Options Calculator | Revolution Trading Pros</title>
	<meta
		name="description"
		content="The most advanced free Black-Scholes options calculator. Real-time Greeks (Delta, Gamma, Theta, Vega, Rho + second-order), payoff diagrams, strategy builder, Monte Carlo simulation, and live market data."
	/>
	<link rel="canonical" href="https://revolutiontradingpros.com/tools/options-calculator" />

	<!-- Open Graph -->
	<meta property="og:title" content={ogMeta['og:title']} />
	<meta property="og:description" content={ogMeta['og:description']} />
	<meta property="og:type" content={ogMeta['og:type']} />
	<meta property="og:url" content={ogMeta['og:url']} />
	<meta property="og:image" content={ogMeta['og:image']} />
	<meta property="og:site_name" content={ogMeta['og:site_name']} />

	<!-- Twitter Card -->
	<meta name="twitter:card" content={ogMeta['twitter:card']} />
	<meta name="twitter:title" content={ogMeta['twitter:title']} />
	<meta name="twitter:description" content={ogMeta['twitter:description']} />
	<meta name="twitter:image" content={ogMeta['twitter:image']} />

	<!-- Structured Data -->
	{@html `<script type="application/ld+json">${JSON.stringify(structuredData)}</script>`}
	{@html `<script type="application/ld+json">${JSON.stringify(faqData)}</script>`}

	<!-- Fonts -->
	<link rel="preconnect" href="https://fonts.googleapis.com" />
	<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous" />
	<link
		href="https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@400;500;600;700&family=Plus+Jakarta+Sans:wght@400;500;600;700;800&family=Inter:wght@400;500;600&display=swap"
		rel="stylesheet"
	/>
</svelte:head>

<Calculator />

<script lang="ts">
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
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

	let HeroSection: any = $state(null);
	let heroLoaded = $state(false);
	let calculatorVisible = $state(false);

	onMount(async () => {
		if (!browser) return;

		// Lazy-load the hero section
		try {
			const mod = await import('$lib/options-calculator/components/hero/HeroSection.svelte');
			HeroSection = mod.default;
			heroLoaded = true;
		} catch {
			// Fallback: skip hero if it fails to load
			heroLoaded = true;
			calculatorVisible = true;
		}

		// Reveal calculator after a brief delay for smooth entrance
		setTimeout(() => {
			calculatorVisible = true;
		}, 300);

	});

</script>

<svelte:head>
	<title>Black-Scholes Options Calculator | Revolution Trading Pros</title>
	<meta
		name="description"
		content="Free institutional-grade Black-Scholes options calculator with real-time data, full Greeks suite, Monte Carlo simulation, and multi-leg strategy builder."
	/>
	<link
		rel="preload"
		href="https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@400;600&family=Plus+Jakarta+Sans:wght@600;700;800&display=swap"
		as="style"
	/>
	<link
		rel="stylesheet"
		href="https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@400;600&family=Plus+Jakarta+Sans:wght@600;700;800&display=swap"
	/>
	{@html `<script type="application/ld+json">${JSON.stringify(structuredData)}</script>`}
	{@html `<script type="application/ld+json">${JSON.stringify(faqData)}</script>`}
	{#each Object.entries(ogMeta) as [property, content]}
		<meta {property} {content} />
	{/each}
</svelte:head>

<!-- Hero Section -->
{#if HeroSection}
	{@const Hero = HeroSection}
	<Hero />
{:else if !heroLoaded}
	<!-- Minimal hero placeholder while loading -->
	<div class="calc-hero">
		<div class="calc-hero__content">
			<h1 class="calc-hero__title">Black-Scholes Options Calculator</h1>
			<p class="calc-hero__subtitle">Institutional-Grade Options Pricing Engine</p>
		</div>
	</div>
{/if}

<!-- Calculator Section -->
<div
	id="calculator"
	class="calculator-wrapper"
	class:calculator-visible={calculatorVisible}
>
	<Calculator />
</div>

<style>
	.calculator-wrapper {
		opacity: 0;
		transform: translateY(20px);
		transition:
			opacity 0.8s cubic-bezier(0.4, 0, 0.2, 1),
			transform 0.8s cubic-bezier(0.4, 0, 0.2, 1);
	}

	.calculator-visible {
		opacity: 1;
		transform: translateY(0);
	}

	@media (prefers-reduced-motion: reduce) {
		.calculator-wrapper {
			opacity: 1;
			transform: none;
			transition: none;
		}
	}
</style>

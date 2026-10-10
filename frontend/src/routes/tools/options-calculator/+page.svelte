<script lang="ts">
	import Calculator from '#lib/options-calculator/components/Calculator.svelte';
	import HeroSection from '#lib/options-calculator/components/hero/HeroSection.svelte';
	import '#lib/options-calculator/styles/calculator-theme.css';
</script>

<!-- Title, description, canonical, Open Graph and JSON-LD come from +page.ts (`data.seo`). -->
<svelte:head>
	<link rel="preconnect" href="https://fonts.googleapis.com" />
	<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous" />
	<link
		href="https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@400;500;600;700&family=Plus+Jakarta+Sans:wght@400;500;600;700;800&family=Inter:wght@400;500;600&display=swap"
		rel="stylesheet"
	/>
</svelte:head>

<HeroSection targetId="calculator" />

<div id="calculator" class="calculator-anchor">
	<!-- The hero's title is the page's <h1>. -->
	<Calculator headingLevel={2} />
</div>

<style>
	/* Clear the sticky site navbar (plus the admin toolbar when it shows) when the hero's
	   CTA scrolls here. The heights mirror --nav-height in NavBar.svelte, which grows to
	   104px on wide screens, so a fixed 5rem left the calculator header under the nav. */
	.calculator-anchor {
		--calc-anchor-nav-height: 80px;
		scroll-margin-top: calc(var(--calc-anchor-nav-height) + var(--admin-toolbar-height, 0px));
	}

	@media (max-width: 767px) {
		.calculator-anchor {
			--calc-anchor-nav-height: 74px;
		}
	}

	@media (min-width: 768px) and (max-width: 1023px) {
		.calculator-anchor {
			--calc-anchor-nav-height: 84px;
		}
	}

	@media (min-width: 1152px) {
		.calculator-anchor {
			--calc-anchor-nav-height: 88px;
		}
	}

	@media (min-width: 1440px) {
		.calculator-anchor {
			--calc-anchor-nav-height: 94px;
		}
	}

	@media (min-width: 1536px) {
		.calculator-anchor {
			--calc-anchor-nav-height: 104px;
		}
	}
</style>

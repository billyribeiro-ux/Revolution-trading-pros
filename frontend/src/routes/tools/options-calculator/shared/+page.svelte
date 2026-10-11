<script lang="ts">
	import Calculator from '#lib/options-calculator/components/Calculator.svelte';
	import '#lib/options-calculator/styles/calculator-theme.css';
</script>

<!-- Title, description and Open Graph tags come from +page.ts (`data.seo`). -->
<svelte:head>
	<link rel="preconnect" href="https://fonts.googleapis.com" />
	<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous" />
	<link
		href="https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@400;500;600;700&family=Plus+Jakarta+Sans:wght@400;500;600;700;800&family=Inter:wght@400;500;600&display=swap"
		rel="stylesheet"
	/>
</svelte:head>

<!-- Shared Banner -->
<div
	class="w-full py-2.5 px-4 text-center text-xs"
	style="
		background: linear-gradient(135deg, rgba(99,102,241,0.08), rgba(0,212,170,0.08));
		border-bottom: 1px solid rgba(99,102,241,0.15);
		color: var(--calc-text-secondary, #a0a0b8);
	"
>
	Shared by a Revolution Trading Pros user &bull;
	<a
		href="/tools/options-calculator"
		class="font-semibold underline underline-offset-2"
		style="color: var(--calc-accent, #6366f1);">Start your own analysis</a
	>
</div>

<Calculator />

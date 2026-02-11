<script lang="ts">
	import { generateOGMeta } from '$lib/options-calculator/utils/seo.js';
	import Calculator from '$lib/options-calculator/components/Calculator.svelte';

	const ogMeta = generateOGMeta();
</script>

<svelte:head>
	<title>Shared Options Analysis | Revolution Trading Pros</title>
	<meta name="description" content={ogMeta['og:description']} />
	<meta property="og:title" content="Shared Options Analysis | Revolution Trading Pros" />
	<meta property="og:description" content={ogMeta['og:description']} />
	<meta property="og:type" content="website" />
	<meta property="og:image" content={ogMeta['og:image']} />
	<meta name="twitter:card" content="summary_large_image" />
	<meta name="twitter:title" content="Shared Options Analysis" />
	<meta name="twitter:description" content={ogMeta['og:description']} />
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

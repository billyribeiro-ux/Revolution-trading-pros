<script lang="ts">
	import { page } from '$app/state';
	import { CalculatorState } from '#lib/options-calculator/state/calculator.svelte.js';
	import { setCalculator } from '#lib/options-calculator/state/context.js';
	import { decodeState } from '#lib/options-calculator/utils/share-utils.js';
	import InputPanel from '#lib/options-calculator/components/InputPanel.svelte';
	import ResultsBar from '#lib/options-calculator/components/ResultsBar.svelte';
	import '#lib/options-calculator/styles/calculator-theme.css';

	// The widget owns its calculator; InputPanel and ResultsBar read it from context.
	const calc = setCalculator(new CalculatorState());

	// `ssr = false` (+page.ts): this script runs once, in the browser, before the first
	// render, so the embed URL's state is in place when the widget first paints.
	applyURLState(page.url.searchParams);

	function applyURLState(params: Pick<URLSearchParams, 'get'>): void {
		const theme = params.get('theme');
		if ((theme === 'light' || theme === 'dark') && theme !== calc.theme) calc.toggleTheme();
		calc.applyShareableState(decodeState(params));
	}
</script>

<svelte:head>
	<link rel="preconnect" href="https://fonts.googleapis.com" />
	<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous" />
	<link
		href="https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@400;500;600;700&family=Plus+Jakarta+Sans:wght@400;500;600;700;800&family=Inter:wght@400;500;600&display=swap"
		rel="stylesheet"
	/>
</svelte:head>

<!-- calculator-theme.css keys its light tokens off [data-calc-theme]. -->
<div
	class="min-h-screen flex flex-col"
	style="background: var(--calc-bg); color: var(--calc-text);"
	data-calc-theme={calc.theme}
>
	<!-- Compact Calculator -->
	<div class="flex-1 p-3 flex flex-col gap-3 max-w-4xl mx-auto w-full">
		<InputPanel />
		<ResultsBar />
	</div>

	<!-- Powered By Footer -->
	<div
		class="flex items-center justify-center py-2 px-4"
		style="border-top: 1px solid var(--calc-border);"
	>
		<a
			href="https://revolutiontradingpros.com/tools/options-calculator"
			target="_blank"
			rel="noopener"
			class="text-[10px] transition-colors"
			style="color: var(--calc-text-muted);"
		>
			Powered by <span class="font-semibold" style="color: var(--calc-accent);"
				>Revolution Trading Pros</span
			>
		</a>
	</div>
</div>

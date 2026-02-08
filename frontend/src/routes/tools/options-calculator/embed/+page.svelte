<script lang="ts">
	import { page } from '$app/state';
	import { onMount } from 'svelte';
	import { createCalculatorState } from '$lib/options-calculator/state/calculator.svelte.js';
	import { decodeState } from '$lib/options-calculator/utils/share-utils.js';
	import InputPanel from '$lib/options-calculator/components/InputPanel.svelte';
	import ResultsBar from '$lib/options-calculator/components/ResultsBar.svelte';

	const calc = $state(createCalculatorState());

	onMount(() => {
		const params = page.url.searchParams;

		// Apply theme from URL
		const theme = params.get('theme');
		if (theme === 'light' && calc.theme === 'dark') calc.toggleTheme();
		if (theme === 'dark' && calc.theme === 'light') calc.toggleTheme();

		// Apply shared state from URL
		const shared = decodeState(params);
		if (shared.inputs) {
			calc.updateInput('spotPrice', shared.inputs.spotPrice);
			calc.updateInput('strikePrice', shared.inputs.strikePrice);
			calc.updateInput('volatility', shared.inputs.volatility);
			calc.updateInput('timeToExpiry', shared.inputs.timeToExpiry);
			calc.updateInput('riskFreeRate', shared.inputs.riskFreeRate);
			calc.updateInput('dividendYield', shared.inputs.dividendYield);
		}
		if (shared.optionType) calc.optionType = shared.optionType;
	});
</script>

<svelte:head>
	<title>Options Calculator | Revolution Trading Pros</title>
	<meta name="robots" content="noindex" />
</svelte:head>

<div
	class="min-h-screen flex flex-col"
	style="background: var(--calc-bg); color: var(--calc-text);"
	data-theme={calc.theme}
>
	<!-- Compact Calculator -->
	<div class="flex-1 p-3 flex flex-col gap-3 max-w-4xl mx-auto w-full">
		<InputPanel {calc} />
		<ResultsBar {calc} />
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
			Powered by <span class="font-semibold" style="color: var(--calc-accent);">Revolution Trading Pros</span>
		</a>
	</div>
</div>

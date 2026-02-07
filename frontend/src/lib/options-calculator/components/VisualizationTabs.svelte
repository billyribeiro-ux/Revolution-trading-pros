<script lang="ts">
	import PayoffDiagram from './charts/PayoffDiagram.svelte';
	import GreeksHeatmap from './charts/GreeksHeatmap.svelte';
	import Surface3D from './charts/Surface3D.svelte';
	import MonteCarloPaths from './charts/MonteCarloPaths.svelte';
	import VolSmile from './charts/VolSmile.svelte';
	import ThetaDecay from './charts/ThetaDecay.svelte';
	import SensitivityRadar from './charts/SensitivityRadar.svelte';
	import OptionsChainViewer from './OptionsChainViewer.svelte';
	import type { CalculatorState } from '../state/calculator.svelte.js';
	import type { MarketDataService } from '../data/market-data-service.svelte.js';
	import type { ChartTab } from '../engine/types.js';

	interface Props {
		calc: CalculatorState;
		marketData: MarketDataService;
	}

	let { calc, marketData }: Props = $props();

	const TABS: { id: ChartTab; label: string }[] = [
		{ id: 'payoff', label: 'Payoff' },
		{ id: 'heatmap', label: 'Heatmap' },
		{ id: 'surface', label: '3D Surface' },
		{ id: 'montecarlo', label: 'Monte Carlo' },
		{ id: 'volsmile', label: 'Vol Smile' },
		{ id: 'theta', label: 'Theta Decay' },
		{ id: 'sensitivity', label: 'Sensitivity' },
		{ id: 'chain', label: 'Options Chain' },
	];
</script>

<div class="flex flex-col gap-0">
	<!-- Tab bar -->
	<div
		class="relative flex border-b"
		style="border-color: var(--calc-border);"
	>
		{#each TABS as tab (tab.id)}
			<button
				onclick={() => (calc.activeTab = tab.id)}
				class="relative px-3 py-2.5 text-xs font-medium transition-colors cursor-pointer whitespace-nowrap"
				style="color: {calc.activeTab === tab.id ? 'var(--calc-text)' : 'var(--calc-text-muted)'};"
			>
				{tab.label}
				{#if calc.activeTab === tab.id}
					<div
						class="absolute bottom-0 left-0 right-0 h-0.5 rounded-full"
						style="background: var(--calc-accent); box-shadow: 0 0 8px var(--calc-accent-glow);"
					></div>
				{/if}
			</button>
		{/each}
	</div>

	<!-- Tab content -->
	<div class="pt-4">
		{#if calc.activeTab === 'payoff'}
			<PayoffDiagram {calc} />
		{:else if calc.activeTab === 'heatmap'}
			<GreeksHeatmap {calc} />
		{:else if calc.activeTab === 'surface'}
			<Surface3D {calc} />
		{:else if calc.activeTab === 'montecarlo'}
			<MonteCarloPaths {calc} />
		{:else if calc.activeTab === 'volsmile'}
			<VolSmile {calc} />
		{:else if calc.activeTab === 'theta'}
			<ThetaDecay {calc} />
		{:else if calc.activeTab === 'sensitivity'}
			<SensitivityRadar {calc} />
		{:else if calc.activeTab === 'chain'}
			<OptionsChainViewer {marketData} {calc} />
		{/if}
	</div>
</div>

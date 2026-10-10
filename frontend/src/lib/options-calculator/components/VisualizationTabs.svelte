<script lang="ts">
	import PayoffDiagram from './charts/PayoffDiagram.svelte';
	import GreeksHeatmap from './charts/GreeksHeatmap.svelte';
	import MonteCarloPaths from './charts/MonteCarloPaths.svelte';
	import VolSmile from './charts/VolSmile.svelte';
	import ThetaDecay from './charts/ThetaDecay.svelte';
	import SensitivityRadar from './charts/SensitivityRadar.svelte';
	import OptionsChainViewer from './OptionsChainViewer.svelte';
	import { untrack } from 'svelte';
	import { prefersReducedMotion } from 'svelte/motion';
	import { fade } from 'svelte/transition';
	import type { Attachment } from 'svelte/attachments';
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
		{ id: 'chain', label: 'Options Chain' }
	];

	const surface3DModule = $derived(
		calc.activeTab === 'surface' ? import('./charts/Surface3D.svelte') : null
	);

	// Sliding active-tab indicator. Until the tab bar has been measured (SSR,
	// first paint) the active tab draws its own underline instead.
	let indicator = $state<{ left: number; width: number } | null>(null);

	const trackActiveTab: Attachment<HTMLDivElement> = (tabBar) => {
		const measure = () => {
			const active = tabBar.querySelector<HTMLButtonElement>(`[data-tab-id="${calc.activeTab}"]`);
			indicator = active ? { left: active.offsetLeft, width: active.offsetWidth } : null;
		};

		$effect(measure);

		// Web-font swaps and viewport changes resize tabs without a tab change.
		const observer = new ResizeObserver(() => untrack(measure));
		for (const button of tabBar.querySelectorAll('button')) observer.observe(button);
		return () => observer.disconnect();
	};
</script>

<div class="flex flex-col gap-0">
	<!-- Tab bar -->
	<div
		{@attach trackActiveTab}
		class="relative flex border-b"
		style="border-color: var(--calc-border);"
	>
		{#each TABS as tab (tab.id)}
			<button
				data-tab-id={tab.id}
				onclick={() => (calc.activeTab = tab.id)}
				class="relative px-3 py-2.5 text-xs font-medium transition-colors cursor-pointer whitespace-nowrap"
				style:color={calc.activeTab === tab.id ? 'var(--calc-text)' : 'var(--calc-text-muted)'}
			>
				{tab.label}
				{#if calc.activeTab === tab.id && !indicator}
					<div
						class="absolute bottom-0 left-0 right-0 h-0.5 rounded-full"
						style="background: var(--calc-accent); box-shadow: 0 0 8px var(--calc-accent-glow);"
					></div>
				{/if}
			</button>
		{/each}
		{#if indicator}
			<div
				class="tab-indicator"
				style:width="{indicator.width}px"
				style:transform="translateX({indicator.left}px)"
				aria-hidden="true"
			></div>
		{/if}
	</div>

	<!-- Tab content: each newly selected chart fades in -->
	<div class="pt-4">
		{#key calc.activeTab}
			<div in:fade={{ duration: prefersReducedMotion.current ? 0 : 200 }}>
				{#if calc.activeTab === 'payoff'}
					<PayoffDiagram {calc} />
				{:else if calc.activeTab === 'heatmap'}
					<GreeksHeatmap {calc} />
				{:else if calc.activeTab === 'surface'}
					{#await surface3DModule}
						<div
							class="flex min-h-80 items-center justify-center rounded-xl border text-xs"
							style="border-color: var(--calc-border); color: var(--calc-text-muted);"
						>
							Loading 3D surface...
						</div>
					{:then module}
						{const Surface3D = module?.default}
						<Surface3D {calc} />
					{:catch}
						<div
							class="rounded-xl border p-6 text-center text-xs"
							style="border-color: var(--calc-border); color: var(--calc-text-muted);"
						>
							3D surface failed to load.
						</div>
					{/await}
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
		{/key}
	</div>
</div>

<style>
	.tab-indicator {
		position: absolute;
		bottom: 0;
		left: 0;
		height: 2px;
		border-radius: 9999px;
		background: var(--calc-accent);
		box-shadow: 0 0 8px var(--calc-accent-glow);
		transition:
			transform 0.3s cubic-bezier(0.4, 0, 0.2, 1),
			width 0.3s cubic-bezier(0.4, 0, 0.2, 1);
	}

	@media (prefers-reduced-motion: reduce) {
		.tab-indicator {
			transition: none;
		}
	}
</style>

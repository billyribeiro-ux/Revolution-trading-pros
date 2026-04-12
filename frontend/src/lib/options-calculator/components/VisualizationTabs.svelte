<script lang="ts">
	import { onMount } from 'svelte';
	import { fade } from 'svelte/transition';
	import { SvelteMap } from 'svelte/reactivity';
	import OptionsChainViewer from './OptionsChainViewer.svelte';
	import type { CalculatorState } from '../state/calculator.svelte.js';
	import type { MarketDataService } from '../data/market-data-service.svelte.js';
	import type { ChartTab } from '../engine/types.js';
	import type { Component } from 'svelte';

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

	// --- Dynamic chart imports with cache ---
	const chartCache = new SvelteMap<string, Component>();
	let activeChartComponent: Component | null = $state(null);

	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	const chartImports: Record<string, () => Promise<{ default: any }>> = {
		payoff: () => import('./charts/PayoffDiagram.svelte'),
		theta: () => import('./charts/ThetaDecay.svelte'),
		surface: () => import('./charts/Surface3D.svelte'),
		heatmap: () => import('./charts/GreeksHeatmap.svelte'),
		volsmile: () => import('./charts/VolSmile.svelte'),
		sensitivity: () => import('./charts/SensitivityRadar.svelte'),
		montecarlo: () => import('./charts/MonteCarloPaths.svelte')
	};

	$effect(() => {
		const tab = calc.activeTab;
		// 'chain' uses OptionsChainViewer which is statically imported
		if (tab === 'chain') {
			activeChartComponent = null;
			return;
		}
		const cached = chartCache.get(tab);
		if (cached) {
			activeChartComponent = cached;
			return;
		}
		const loader = chartImports[tab];
		if (loader) {
			loader().then((m) => {
				chartCache.set(tab, m.default);
				activeChartComponent = m.default;
			});
		}
	});

	// --- Animated tab slider (GSAP) ---
	let gsapLib: typeof import('gsap').default | null = null;
	let sliderEl: HTMLDivElement | undefined = $state();
	let tabBarEl: HTMLDivElement | undefined = $state();

	function updateSlider() {
		if (!sliderEl || !tabBarEl || !gsapLib) return;
		const activeBtn = tabBarEl.querySelector<HTMLButtonElement>(
			`[data-tab-id="${calc.activeTab}"]`
		);
		if (!activeBtn) return;
		const targetLeft = activeBtn.offsetLeft;
		const targetWidth = activeBtn.offsetWidth;
		gsapLib.to(sliderEl, {
			x: targetLeft,
			width: targetWidth,
			duration: 0.3,
			ease: 'power2.out'
		});
	}

	onMount(async () => {
		if (typeof window !== 'undefined') {
			const mod = await import('gsap');
			gsapLib = mod.default || mod.gsap;
			updateSlider();
		}
	});

	$effect(() => {
		void calc.activeTab;
		if (gsapLib) updateSlider();
	});
</script>

<div class="flex flex-col gap-0">
	<!-- Tab bar -->
	<div
		bind:this={tabBarEl}
		class="relative flex border-b"
		style="border-color: var(--calc-border);"
	>
		{#each TABS as tab (tab.id)}
			<button
				data-tab-id={tab.id}
				onclick={() => (calc.activeTab = tab.id)}
				class="relative px-3 py-2.5 text-xs font-medium transition-colors cursor-pointer whitespace-nowrap"
				style="color: {calc.activeTab === tab.id ? 'var(--calc-text)' : 'var(--calc-text-muted)'};"
			>
				{tab.label}
			</button>
		{/each}
		<!-- Animated sliding indicator -->
		<div
			bind:this={sliderEl}
			class="tab-slider"
			style="height: 2px; background: var(--calc-accent); position: absolute; bottom: 0; left: 0; width: 0; box-shadow: 0 0 8px var(--calc-accent-glow);"
		></div>
	</div>

	<!-- Tab content with cross-fade -->
	<div class="pt-4">
		{#key calc.activeTab}
			<div in:fade={{ duration: 200, delay: 100 }} out:fade={{ duration: 150 }}>
				{#if calc.activeTab === 'chain'}
					<OptionsChainViewer {marketData} {calc} />
				{:else if activeChartComponent}
					{@const ChartComponent = activeChartComponent}
					<ChartComponent {calc} />
				{/if}
			</div>
		{/key}
	</div>
</div>

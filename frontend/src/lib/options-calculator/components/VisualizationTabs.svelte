<script lang="ts">
	import PayoffDiagram from './charts/PayoffDiagram.svelte';
	import GreeksHeatmap from './charts/GreeksHeatmap.svelte';
	import MonteCarloPaths from './charts/MonteCarloPaths.svelte';
	import VolSmile from './charts/VolSmile.svelte';
	import ThetaDecay from './charts/ThetaDecay.svelte';
	import SensitivityRadar from './charts/SensitivityRadar.svelte';
	import OptionsChainViewer from './OptionsChainViewer.svelte';
	import { prefersReducedMotion } from 'svelte/motion';
	import { fade } from 'svelte/transition';
	import { getCalculator } from '../state/context.js';
	import type { ChartTab } from '../engine/types.js';

	const calc = getCalculator();

	const uid = $props.id();
	const panelId = `${uid}-panel`;
	const tabId = (id: ChartTab) => `${uid}-tab-${id}`;

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

	/** Each tab's rendered (sub-pixel) width, kept current by its dimension binding. */
	const tabWidths = $state<Partial<Record<ChartTab, number>>>({});

	/** Setter for a tab's `bind:borderBoxSize` function binding (readonly, so no getter). */
	const measureTab = (id: ChartTab) => (size: ResizeObserverSize[]) => {
		tabWidths[id] = size[0]?.inlineSize;
	};

	// Sliding active-tab indicator. The tabs sit edge to edge, so the active tab starts where
	// the widths before it end. Until those tabs have been measured (SSR, first paint) the
	// active tab draws its own underline instead.
	const indicator = $derived.by(() => {
		let left = 0;
		for (const { id } of TABS) {
			const width = tabWidths[id];
			if (width === undefined) return null;
			if (id === calc.activeTab) return { left, width };
			left += width;
		}
		return null;
	});

	/** WAI-ARIA tabs keyboard model (automatic activation): arrows wrap, Home/End jump. */
	function handleTabKeydown(event: KeyboardEvent, index: number) {
		let next: number;
		switch (event.key) {
			case 'ArrowRight':
				next = (index + 1) % TABS.length;
				break;
			case 'ArrowLeft':
				next = (index - 1 + TABS.length) % TABS.length;
				break;
			case 'Home':
				next = 0;
				break;
			case 'End':
				next = TABS.length - 1;
				break;
			default:
				return;
		}
		event.preventDefault();
		const { id } = TABS[next];
		calc.activeTab = id;
		document.getElementById(tabId(id))?.focus();
	}
</script>

<div class="flex flex-col gap-0">
	<!-- Tab bar -->
	<div
		class="relative flex border-b"
		style="border-color: var(--calc-border);"
		role="tablist"
		aria-label="Charts"
	>
		{#each TABS as tab, index (tab.id)}
			{const selected = $derived(calc.activeTab === tab.id)}
			<button
				type="button"
				role="tab"
				id={tabId(tab.id)}
				aria-selected={selected}
				aria-controls={panelId}
				tabindex={selected ? 0 : -1}
				onclick={() => (calc.activeTab = tab.id)}
				onkeydown={(event) => handleTabKeydown(event, index)}
				bind:borderBoxSize={null, measureTab(tab.id)}
				class="relative px-3 py-2.5 text-xs font-medium transition-colors motion-reduce:transition-none cursor-pointer whitespace-nowrap"
				style:color={selected ? 'var(--calc-text)' : 'var(--calc-text-muted)'}
			>
				{tab.label}
				{#if selected && !indicator}
					<span
						class="absolute bottom-0 left-0 right-0 h-0.5 rounded-full"
						style="background: var(--calc-accent); box-shadow: 0 0 8px var(--calc-accent-glow);"
					></span>
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
	<div
		class="pt-4"
		role="tabpanel"
		id={panelId}
		aria-labelledby={tabId(calc.activeTab)}
		tabindex="0"
	>
		{#key calc.activeTab}
			<div in:fade={{ duration: prefersReducedMotion.current ? 0 : 200 }}>
				<!-- One chart failing (a NaN domain, a WebGL error) stays inside this panel;
				     switching tabs re-creates the boundary through the {#key} above. -->
				<svelte:boundary>
					{#if calc.activeTab === 'payoff'}
						<PayoffDiagram />
					{:else if calc.activeTab === 'heatmap'}
						<GreeksHeatmap />
					{:else if calc.activeTab === 'surface'}
						<!-- Three.js is only fetched the first time the 3D surface is opened. -->
						{#await import('./charts/Surface3D.svelte')}
							<div
								class="flex min-h-80 items-center justify-center rounded-xl border text-xs"
								style="border-color: var(--calc-border); color: var(--calc-text-muted);"
							>
								Loading 3D surface...
							</div>
						{:then { default: Surface3D }}
							<Surface3D />
						{:catch}
							<div
								class="rounded-xl border p-6 text-center text-xs"
								style="border-color: var(--calc-border); color: var(--calc-text-muted);"
							>
								3D surface failed to load.
							</div>
						{/await}
					{:else if calc.activeTab === 'montecarlo'}
						<MonteCarloPaths />
					{:else if calc.activeTab === 'volsmile'}
						<VolSmile />
					{:else if calc.activeTab === 'theta'}
						<ThetaDecay />
					{:else if calc.activeTab === 'sensitivity'}
						<SensitivityRadar />
					{:else if calc.activeTab === 'chain'}
						<OptionsChainViewer />
					{/if}
					{#snippet failed(_error, reset)}
						<div
							class="flex flex-col items-center gap-2 rounded-xl border p-6 text-center text-xs"
							style="border-color: var(--calc-border); color: var(--calc-text-muted);"
						>
							This chart failed to render.
							<button
								type="button"
								onclick={reset}
								class="rounded-lg px-3 py-1 font-medium cursor-pointer"
								style="background: var(--calc-surface); color: var(--calc-text); border: 1px solid var(--calc-border);"
							>
								Try again
							</button>
						</div>
					{/snippet}
				</svelte:boundary>
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

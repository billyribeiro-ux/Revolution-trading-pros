<script lang="ts">
	import IconChevronDown from '@tabler/icons-svelte-runes/icons/chevron-down';
	import MetricCard from './ui/MetricCard.svelte';
	import { getCalculator } from '../state/context.js';

	const calc = getCalculator();
	const secondOrderId = $props.id();

	let is3DView = $state(false);

	const FIRST_ORDER = [
		{
			key: 'delta',
			label: 'Delta (Δ)',
			desc: 'Price sensitivity to $1 move in underlying',
			decimals: 4
		},
		{ key: 'gamma', label: 'Gamma (Γ)', desc: 'Rate of change of delta per $1 move', decimals: 6 },
		{ key: 'theta', label: 'Theta (Θ)', desc: 'Daily time decay in dollars', decimals: 4 },
		{ key: 'vega', label: 'Vega (ν)', desc: 'Price change per 1% IV move', decimals: 4 },
		{ key: 'rho', label: 'Rho (ρ)', desc: 'Price change per 1% rate move', decimals: 4 }
	] as const;

	const SECOND_ORDER = [
		{ key: 'charm', label: 'Charm', desc: 'Delta decay per day', decimals: 6 },
		{ key: 'vanna', label: 'Vanna', desc: 'Delta sensitivity to volatility', decimals: 6 },
		{ key: 'volga', label: 'Volga', desc: 'Vega sensitivity to volatility', decimals: 4 },
		{ key: 'veta', label: 'Veta', desc: 'Vega decay per day', decimals: 6 },
		{ key: 'speed', label: 'Speed', desc: 'Gamma sensitivity to underlying', decimals: 8 },
		{ key: 'color', label: 'Color', desc: 'Gamma decay per day', decimals: 8 },
		{ key: 'zomma', label: 'Zomma', desc: 'Gamma sensitivity to volatility', decimals: 6 }
	] as const;
</script>

<div class="flex flex-col gap-3">
	<!-- First Order Greeks -->
	<div class="flex items-center justify-between mb-1">
		<h3
			class="text-xs font-semibold uppercase tracking-wider"
			style="color: var(--calc-text-secondary); font-family: var(--calc-font-display);"
		>
			First-Order Greeks
		</h3>
		<!-- One pill, two buttons: each one's accessible name is its visible label. -->
		<div
			class="flex items-center text-[10px] font-semibold uppercase tracking-wider rounded-md"
			style="background: var(--calc-surface); border: 1px solid var(--calc-border); color: var(--calc-text-muted);"
			role="group"
			aria-label="Greeks view"
		>
			<button
				type="button"
				onclick={() => (is3DView = false)}
				aria-pressed={!is3DView}
				class="pl-2 pr-1 py-1 cursor-pointer"
				style:opacity={is3DView ? 0.5 : 1}
			>
				2D
			</button>
			<span style="opacity: 0.3;" aria-hidden="true">|</span>
			<button
				type="button"
				onclick={() => (is3DView = true)}
				aria-pressed={is3DView}
				class="pl-1 pr-2 py-1 cursor-pointer"
				style:opacity={is3DView ? 1 : 0.5}
			>
				3D
			</button>
		</div>
	</div>

	{#if is3DView}
		<!-- Three.js is only fetched the first time the 3D view is opened. -->
		{#await import('./charts/GreeksOrbit3D.svelte')}
			<div
				class="flex h-[300px] items-center justify-center rounded-xl border text-xs"
				style="border-color: var(--calc-border); color: var(--calc-text-muted);"
			>
				Loading 3D view...
			</div>
		{:then { default: GreeksOrbit3D }}
			<!-- A WebGL/runtime failure stays inside the panel instead of failing the page. -->
			<svelte:boundary>
				<GreeksOrbit3D
					delta={calc.currentGreeks.first.delta}
					gamma={calc.currentGreeks.first.gamma}
					theta={calc.currentGreeks.first.theta}
					vega={calc.currentGreeks.first.vega}
					rho={calc.currentGreeks.first.rho}
					theme={calc.theme}
				/>
				{#snippet failed()}
					<div
						class="rounded-xl border p-6 text-center text-xs"
						style="border-color: var(--calc-border); color: var(--calc-text-muted);"
					>
						3D view failed to render.
					</div>
				{/snippet}
			</svelte:boundary>
		{:catch}
			<div
				class="rounded-xl border p-6 text-center text-xs"
				style="border-color: var(--calc-border); color: var(--calc-text-muted);"
			>
				3D view failed to load.
			</div>
		{/await}
	{:else}
		<div class="grid grid-cols-5 gap-2">
			{#each FIRST_ORDER as greek (greek.key)}
				<MetricCard
					label={greek.label}
					value={calc.currentGreeks.first[greek.key]}
					decimals={greek.decimals}
					tooltip={greek.desc}
					colorize
				/>
			{/each}
		</div>
	{/if}

	<!-- Toggle Advanced -->
	<button
		type="button"
		onclick={() => (calc.showAdvancedGreeks = !calc.showAdvancedGreeks)}
		aria-expanded={calc.showAdvancedGreeks}
		aria-controls={secondOrderId}
		class="flex items-center justify-center gap-1.5 text-xs py-2 rounded-lg transition-colors motion-reduce:transition-none cursor-pointer"
		style="color: var(--calc-text-muted); background: var(--calc-surface); border: 1px solid var(--calc-border);"
	>
		{calc.showAdvancedGreeks ? 'Hide' : 'Show'} Second-Order Greeks
		<span
			class="inline-flex transition-transform motion-reduce:transition-none"
			style:transform={calc.showAdvancedGreeks ? 'rotate(180deg)' : 'rotate(0deg)'}
		>
			<IconChevronDown size={14} aria-hidden="true" />
		</span>
	</button>

	<!-- Second Order Greeks (expandable). The cards stay mounted while collapsed so their
	     sparklines keep tracking the trend; `inert` takes the closed section out of the
	     focus order and the accessibility tree. -->
	<div
		id={secondOrderId}
		class={['second-order', calc.showAdvancedGreeks && 'second-order--open']}
		inert={!calc.showAdvancedGreeks}
	>
		<div class="second-order-inner">
			<div>
				<div class="flex flex-col gap-3 pt-1">
					<h3
						class="text-xs font-semibold uppercase tracking-wider"
						style="color: var(--calc-text-secondary); font-family: var(--calc-font-display);"
					>
						Second-Order Greeks
					</h3>
					<div class="grid grid-cols-4 gap-2">
						{#each SECOND_ORDER as greek (greek.key)}
							<MetricCard
								label={greek.label}
								value={calc.currentGreeks.second[greek.key]}
								decimals={greek.decimals}
								tooltip={greek.desc}
								colorize
							/>
						{/each}
					</div>
				</div>
			</div>
		</div>
	</div>
</div>

<style>
	/* Height animates from 0 to the content's size via grid rows; opacity fades with it. */
	.second-order {
		display: grid;
		grid-template-rows: 0fr;
		opacity: 0;
		transition:
			grid-template-rows 0.4s cubic-bezier(0.165, 0.84, 0.44, 1),
			opacity 0.4s cubic-bezier(0.165, 0.84, 0.44, 1);
	}

	.second-order--open {
		grid-template-rows: 1fr;
		opacity: 1;
	}

	.second-order-inner {
		min-height: 0;
		overflow: hidden;
	}

	@media (prefers-reduced-motion: reduce) {
		.second-order {
			transition: none;
		}
	}
</style>

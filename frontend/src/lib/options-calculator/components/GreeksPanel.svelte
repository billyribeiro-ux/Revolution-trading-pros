<script lang="ts">
	import { CaretDown as ChevronDown } from 'phosphor-svelte';
	import MetricCard from './ui/MetricCard.svelte';
	import gsap from 'gsap';
	import type { CalculatorState } from '../state/calculator.svelte.js';
	import type { Component } from 'svelte';

	interface Props {
		calc: CalculatorState;
	}

	let { calc }: Props = $props();

	let expandEl: HTMLDivElement | undefined = $state();
	let is3DView = $state(false);
	let GreeksOrbit3D: Component<{ delta: number; gamma: number; theta: number; vega: number; rho: number }> | null = $state(null);

	$effect(() => {
		if (is3DView && !GreeksOrbit3D) {
			import('./charts/GreeksOrbit3D.svelte').then((m) => (GreeksOrbit3D = m.default));
		}
	});

	$effect(() => {
		const show = calc.showAdvancedGreeks;
		if (expandEl) {
			if (show) {
				gsap.fromTo(
					expandEl,
					{ height: 0, opacity: 0 },
					{ height: 'auto', opacity: 1, duration: 0.4, ease: 'power3.out' }
				);
			} else {
				gsap.to(expandEl, { height: 0, opacity: 0, duration: 0.3, ease: 'power3.in' });
			}
		}
	});

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
		<button
			onclick={() => (is3DView = !is3DView)}
			class="flex items-center gap-1 text-[10px] font-semibold uppercase tracking-wider rounded-md px-2 py-1 transition-colors cursor-pointer"
			style="background: var(--calc-surface); border: 1px solid var(--calc-border); color: var(--calc-text-muted);"
		>
			<span style:opacity={is3DView ? '0.5' : '1'}>2D</span>
			<span style="opacity: 0.3;">|</span>
			<span style:opacity={is3DView ? '1' : '0.5'}>3D</span>
		</button>
	</div>

	{#if is3DView && GreeksOrbit3D}
		<GreeksOrbit3D
			delta={calc.currentGreeks.first.delta}
			gamma={calc.currentGreeks.first.gamma}
			theta={calc.currentGreeks.first.theta}
			vega={calc.currentGreeks.first.vega}
			rho={calc.currentGreeks.first.rho}
		/>
	{:else if is3DView && !GreeksOrbit3D}
		<div class="flex items-center justify-center h-[300px] text-sm opacity-50" style="color: var(--calc-text-muted);">
			Loading 3D view...
		</div>
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
		onclick={() => (calc.showAdvancedGreeks = !calc.showAdvancedGreeks)}
		class="flex items-center justify-center gap-1.5 text-xs py-2 rounded-lg transition-colors cursor-pointer"
		style="color: var(--calc-text-muted); background: var(--calc-surface); border: 1px solid var(--calc-border);"
	>
		{calc.showAdvancedGreeks ? 'Hide' : 'Show'} Second-Order Greeks
		<span
			class="inline-flex transition-transform"
			style:transform={calc.showAdvancedGreeks ? 'rotate(180deg)' : 'rotate(0deg)'}
		>
			<ChevronDown size={14} />
		</span>
	</button>

	<!-- Second Order Greeks (expandable) -->
	<div bind:this={expandEl} class="overflow-hidden" style="height: 0; opacity: 0;">
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

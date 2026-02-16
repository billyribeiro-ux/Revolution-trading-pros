<script lang="ts">
	import { Zap, X, Plus } from '@lucide/svelte';
	import gsap from 'gsap';
	import { formatCurrency } from '../utils/formatters.js';
	import type { CalculatorState } from '../state/calculator.svelte.js';
	import type { Scenario } from '../engine/types.js';

	interface Props {
		calc: CalculatorState;
	}

	let { calc }: Props = $props();

	let gridEl: HTMLDivElement | undefined = $state();
	let showCustom = $state(false);
	let customSpotPct = $state(0);
	let customVolPct = $state(0);
	let customTimeDays = $state(0);
	let customRateBp = $state(0);

	$effect(() => {
		if (gridEl) {
			const cards = gridEl.querySelectorAll('.scenario-card');
			gsap.fromTo(
				cards,
				{ y: 15, opacity: 0 },
				{ y: 0, opacity: 1, duration: 0.25, stagger: 0.03, ease: 'power2.out' }
			);
		}
	});

	function isActive(id: string): boolean {
		return calc.activeScenarios.some((s) => s.id === id);
	}

	function addCustomScenario() {
		const custom: Scenario = {
			id: `custom-${Date.now()}`,
			name: 'Custom',
			description: `Spot ${customSpotPct >= 0 ? '+' : ''}${customSpotPct}%, Vol ${customVolPct >= 0 ? '+' : ''}${customVolPct}%, Time ${customTimeDays}d, Rate ${customRateBp}bp`,
			adjustments: {
				spotPriceChangePct: customSpotPct || undefined,
				volatilityChange: customVolPct ? customVolPct / 100 : undefined,
				timeChange: customTimeDays || undefined,
				rateChange: customRateBp ? customRateBp / 10000 : undefined
			},
			color: 'var(--color-brand-secondary)'
		};
		calc.toggleScenario(custom);
		showCustom = false;
	}
</script>

<div class="flex flex-col gap-4">
	<!-- Header -->
	<div class="flex items-center justify-between">
		<div class="flex items-center gap-2">
			<Zap size={16} style="color: var(--calc-warning);" />
			<h3
				class="text-sm font-semibold"
				style="color: var(--calc-text); font-family: var(--calc-font-display);"
			>
				Scenario Engine
			</h3>
			{#if calc.activeScenarios.length > 0}
				<span
					class="text-[10px] px-1.5 py-0.5 rounded"
					style="background: var(--calc-warning); color: white;"
				>
					{calc.activeScenarios.length} active
				</span>
			{/if}
		</div>
		<div class="flex items-center gap-2">
			<button
				onclick={() => (showCustom = !showCustom)}
				class="flex items-center gap-1 text-xs px-2 py-1.5 rounded-lg transition-colors cursor-pointer"
				style="background: var(--calc-surface); color: var(--calc-text-secondary); border: 1px solid var(--calc-border);"
			>
				<Plus size={10} />
				Custom
			</button>
			{#if calc.activeScenarios.length > 0}
				<button
					onclick={() => calc.clearScenarios()}
					class="flex items-center gap-1 text-xs px-2 py-1.5 rounded-lg transition-colors cursor-pointer"
					style="color: var(--calc-put); background: var(--calc-put-bg);"
				>
					<X size={10} />
					Clear
				</button>
			{/if}
		</div>
	</div>

	<!-- Custom Scenario Builder -->
	{#if showCustom}
		<div
			class="grid grid-cols-2 sm:grid-cols-4 gap-2 rounded-xl p-3"
			style="background: var(--calc-surface); border: 1px solid var(--calc-accent);"
		>
			<div class="flex flex-col gap-1">
				<span class="text-[10px]" style="color: var(--calc-text-muted);">Spot %</span>
				<input
					type="number"
					bind:value={customSpotPct}
					step="1"
					class="text-xs px-2 py-1 rounded outline-none"
					style="background: var(--calc-surface-hover); color: var(--calc-text); border: 1px solid var(--calc-border); font-family: var(--calc-font-mono);"
				/>
			</div>
			<div class="flex flex-col gap-1">
				<span class="text-[10px]" style="color: var(--calc-text-muted);">Vol %</span>
				<input
					type="number"
					bind:value={customVolPct}
					step="1"
					class="text-xs px-2 py-1 rounded outline-none"
					style="background: var(--calc-surface-hover); color: var(--calc-text); border: 1px solid var(--calc-border); font-family: var(--calc-font-mono);"
				/>
			</div>
			<div class="flex flex-col gap-1">
				<span class="text-[10px]" style="color: var(--calc-text-muted);">Days</span>
				<input
					type="number"
					bind:value={customTimeDays}
					step="1"
					class="text-xs px-2 py-1 rounded outline-none"
					style="background: var(--calc-surface-hover); color: var(--calc-text); border: 1px solid var(--calc-border); font-family: var(--calc-font-mono);"
				/>
			</div>
			<div class="flex flex-col gap-1">
				<span class="text-[10px]" style="color: var(--calc-text-muted);">Rate (bp)</span>
				<div class="flex gap-1">
					<input
						type="number"
						bind:value={customRateBp}
						step="1"
						class="flex-1 text-xs px-2 py-1 rounded outline-none"
						style="background: var(--calc-surface-hover); color: var(--calc-text); border: 1px solid var(--calc-border); font-family: var(--calc-font-mono);"
					/>
					<button
						onclick={addCustomScenario}
						class="text-xs px-2 py-1 rounded cursor-pointer"
						style="background: var(--calc-accent); color: white;">Go</button
					>
				</div>
			</div>
		</div>
	{/if}

	<!-- Preset Cards -->
	<div bind:this={gridEl} class="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 gap-2">
		{#each calc.scenarioPresets as preset (preset.id)}
			<button
				onclick={() => calc.toggleScenario(preset)}
				class="scenario-card flex flex-col gap-1 rounded-xl px-3 py-2 text-left transition-all cursor-pointer"
				style="background: var(--calc-surface); border: 1px solid {isActive(preset.id)
					? preset.color
					: 'var(--calc-border)'}; {isActive(preset.id)
					? `box-shadow: 0 0 12px ${preset.color}33;`
					: ''}"
			>
				<span
					class="text-xs font-semibold"
					style="color: {isActive(preset.id) ? preset.color : 'var(--calc-text)'};"
				>
					{preset.name}
				</span>
				<span class="text-[9px] leading-tight" style="color: var(--calc-text-muted);">
					{preset.description}
				</span>
			</button>
		{/each}
	</div>

	<!-- Results Table -->
	{#if calc.scenarioResults.length > 0}
		<div class="rounded-xl overflow-hidden" style="border: 1px solid var(--calc-border);">
			<table class="w-full text-xs" style="font-family: var(--calc-font-mono);">
				<thead>
					<tr style="background: var(--calc-surface);">
						<th class="text-left px-3 py-2 font-medium" style="color: var(--calc-text-muted);"
							>Scenario</th
						>
						<th class="text-right px-3 py-2 font-medium" style="color: var(--calc-text-muted);"
							>Call</th
						>
						<th class="text-right px-3 py-2 font-medium" style="color: var(--calc-text-muted);"
							>Put</th
						>
						<th class="text-right px-3 py-2 font-medium" style="color: var(--calc-text-muted);"
							>Call Δ$</th
						>
						<th class="text-right px-3 py-2 font-medium" style="color: var(--calc-text-muted);"
							>Call Δ%</th
						>
					</tr>
				</thead>
				<tbody>
					{#each calc.scenarioResults as result (result.scenario.id)}
						<tr style="border-top: 1px solid var(--calc-border);">
							<td class="px-3 py-2">
								<span
									class="inline-block w-2 h-2 rounded-full mr-1.5"
									style="background: {result.scenario.color};"
								></span>
								<span style="color: var(--calc-text-secondary);">{result.scenario.name}</span>
							</td>
							<td class="text-right px-3 py-2" style="color: var(--calc-text);"
								>{formatCurrency(result.callPrice)}</td
							>
							<td class="text-right px-3 py-2" style="color: var(--calc-text);"
								>{formatCurrency(result.putPrice)}</td
							>
							<td
								class="text-right px-3 py-2"
								style="color: {result.callPriceChange >= 0
									? 'var(--calc-call)'
									: 'var(--calc-put)'};"
							>
								{result.callPriceChange >= 0 ? '+' : ''}{formatCurrency(result.callPriceChange)}
							</td>
							<td
								class="text-right px-3 py-2"
								style="color: {result.callPriceChangePct >= 0
									? 'var(--calc-call)'
									: 'var(--calc-put)'};"
							>
								{result.callPriceChangePct >= 0 ? '+' : ''}{result.callPriceChangePct.toFixed(1)}%
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	{/if}
</div>

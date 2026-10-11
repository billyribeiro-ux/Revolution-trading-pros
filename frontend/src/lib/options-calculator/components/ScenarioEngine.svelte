<script lang="ts">
	import IconBolt from '@tabler/icons-svelte-runes/icons/bolt';
	import IconX from '@tabler/icons-svelte-runes/icons/x';
	import IconPlus from '@tabler/icons-svelte-runes/icons/plus';
	import { cubicOut } from 'svelte/easing';
	import { prefersReducedMotion } from 'svelte/motion';
	import { fly, type FlyParams } from 'svelte/transition';
	import { formatCurrency } from '../utils/formatters.js';
	import { getCalculator } from '../state/context.js';
	import type { Scenario } from '../engine/types.js';

	const calc = getCalculator();
	const uid = $props.id();

	const RESULT_COLUMNS = ['Call', 'Put', 'Call Δ$', 'Call Δ%'];

	let showCustom = $state(false);
	// `bind:value` on a number input yields `undefined` while the field is empty.
	let customSpotPct = $state<number | undefined>(0);
	let customVolPct = $state<number | undefined>(0);
	let customTimeDays = $state<number | undefined>(0);
	let customRateBp = $state<number | undefined>(0);

	const activeIds = $derived(new Set(calc.activeScenarios.map((s) => s.id)));

	/** Staggered entrance for the preset cards; instant under reduced motion. */
	function cardIntro(index: number): FlyParams {
		if (prefersReducedMotion.current) return { duration: 0 };
		return { y: 15, duration: 250, delay: index * 30, easing: cubicOut };
	}

	function signed(value: number): string {
		return `${value >= 0 ? '+' : ''}${value}`;
	}

	function addCustomScenario(event: SubmitEvent) {
		event.preventDefault();
		const spotPct = customSpotPct || 0;
		const volPct = customVolPct || 0;
		const timeDays = customTimeDays || 0;
		const rateBp = customRateBp || 0;
		const custom: Scenario = {
			id: `custom-${Date.now()}`,
			name: 'Custom',
			description: `Spot ${signed(spotPct)}%, Vol ${signed(volPct)}%, Time ${timeDays}d, Rate ${rateBp}bp`,
			adjustments: {
				spotPriceChangePct: spotPct || undefined,
				volatilityChange: volPct ? volPct / 100 : undefined,
				timeChange: timeDays || undefined,
				rateChange: rateBp ? rateBp / 10000 : undefined
			},
			color: '#8b5cf6'
		};
		calc.toggleScenario(custom);
		showCustom = false;
	}
</script>

<div class="flex flex-col gap-4">
	<!-- Header -->
	<div class="flex items-center justify-between">
		<div class="flex items-center gap-2">
			<IconBolt size={16} style="color: var(--calc-warning);" aria-hidden="true" />
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
				type="button"
				onclick={() => (showCustom = !showCustom)}
				aria-expanded={showCustom}
				class="flex items-center gap-1 text-xs px-2 py-1.5 rounded-lg transition-colors motion-reduce:transition-none cursor-pointer"
				style="background: var(--calc-surface); color: var(--calc-text-secondary); border: 1px solid var(--calc-border);"
			>
				<IconPlus size={10} aria-hidden="true" />
				Custom
			</button>
			{#if calc.activeScenarios.length > 0}
				<button
					type="button"
					onclick={calc.clearScenarios}
					class="flex items-center gap-1 text-xs px-2 py-1.5 rounded-lg transition-colors motion-reduce:transition-none cursor-pointer"
					style="color: var(--calc-put); background: var(--calc-put-bg);"
				>
					<IconX size={10} aria-hidden="true" />
					Clear
				</button>
			{/if}
		</div>
	</div>

	<!-- Custom Scenario Builder -->
	{#if showCustom}
		<form
			onsubmit={addCustomScenario}
			class="grid grid-cols-2 sm:grid-cols-4 gap-2 rounded-xl p-3"
			style="background: var(--calc-surface); border: 1px solid var(--calc-accent);"
		>
			<div class="flex flex-col gap-1">
				<label for="{uid}-spot" class="text-[10px]" style="color: var(--calc-text-muted);"
					>Spot %</label
				>
				<input
					id="{uid}-spot"
					type="number"
					bind:value={customSpotPct}
					step="any"
					class="text-xs px-2 py-1 rounded outline-none"
					style="background: var(--calc-surface-hover); color: var(--calc-text); border: 1px solid var(--calc-border); font-family: var(--calc-font-mono);"
				/>
			</div>
			<div class="flex flex-col gap-1">
				<label for="{uid}-vol" class="text-[10px]" style="color: var(--calc-text-muted);"
					>Vol %</label
				>
				<input
					id="{uid}-vol"
					type="number"
					bind:value={customVolPct}
					step="any"
					class="text-xs px-2 py-1 rounded outline-none"
					style="background: var(--calc-surface-hover); color: var(--calc-text); border: 1px solid var(--calc-border); font-family: var(--calc-font-mono);"
				/>
			</div>
			<div class="flex flex-col gap-1">
				<label for="{uid}-days" class="text-[10px]" style="color: var(--calc-text-muted);"
					>Days</label
				>
				<input
					id="{uid}-days"
					type="number"
					bind:value={customTimeDays}
					step="any"
					class="text-xs px-2 py-1 rounded outline-none"
					style="background: var(--calc-surface-hover); color: var(--calc-text); border: 1px solid var(--calc-border); font-family: var(--calc-font-mono);"
				/>
			</div>
			<div class="flex flex-col gap-1">
				<label for="{uid}-rate" class="text-[10px]" style="color: var(--calc-text-muted);"
					>Rate (bp)</label
				>
				<div class="flex gap-1">
					<input
						id="{uid}-rate"
						type="number"
						bind:value={customRateBp}
						step="any"
						class="flex-1 text-xs px-2 py-1 rounded outline-none"
						style="background: var(--calc-surface-hover); color: var(--calc-text); border: 1px solid var(--calc-border); font-family: var(--calc-font-mono);"
					/>
					<button
						type="submit"
						class="text-xs px-2 py-1 rounded cursor-pointer"
						style="background: var(--calc-accent); color: white;">Go</button
					>
				</div>
			</div>
		</form>
	{/if}

	<!-- Preset Cards -->
	<div class="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 gap-2">
		{#each calc.scenarioPresets as preset, i (preset.id)}
			{const active = $derived(activeIds.has(preset.id))}
			<button
				type="button"
				in:fly|global={cardIntro(i)}
				onclick={() => calc.toggleScenario(preset)}
				aria-pressed={active}
				class="scenario-card flex flex-col gap-1 rounded-xl px-3 py-2 text-left transition-all motion-reduce:transition-none cursor-pointer"
				style="background: var(--calc-surface); border: 1px solid var(--calc-border);"
				style:border-color={active ? preset.color : 'var(--calc-border)'}
				style:box-shadow={active ? `0 0 12px ${preset.color}33` : undefined}
			>
				<span
					class="text-xs font-semibold"
					style:color={active ? preset.color : 'var(--calc-text)'}
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
						<th
							scope="col"
							class="text-left px-3 py-2 font-medium"
							style="color: var(--calc-text-muted);">Scenario</th
						>
						{#each RESULT_COLUMNS as heading (heading)}
							<th
								scope="col"
								class="text-right px-3 py-2 font-medium"
								style="color: var(--calc-text-muted);">{heading}</th
							>
						{/each}
					</tr>
				</thead>
				<tbody>
					{#each calc.scenarioResults as result (result.scenario.id)}
						{const changeColor = $derived(
							result.callPriceChange >= 0 ? 'var(--calc-call)' : 'var(--calc-put)'
						)}
						{const changePctColor = $derived(
							result.callPriceChangePct >= 0 ? 'var(--calc-call)' : 'var(--calc-put)'
						)}
						<tr style="border-top: 1px solid var(--calc-border);">
							<td class="px-3 py-2">
								<span
									class="inline-block w-2 h-2 rounded-full mr-1.5"
									style:background={result.scenario.color}
									aria-hidden="true"
								></span>
								<span style="color: var(--calc-text-secondary);">{result.scenario.name}</span>
							</td>
							<td class="text-right px-3 py-2" style="color: var(--calc-text);"
								>{formatCurrency(result.callPrice)}</td
							>
							<td class="text-right px-3 py-2" style="color: var(--calc-text);"
								>{formatCurrency(result.putPrice)}</td
							>
							<td class="text-right px-3 py-2" style:color={changeColor}>
								{result.callPriceChange >= 0 ? '+' : ''}{formatCurrency(result.callPriceChange)}
							</td>
							<td class="text-right px-3 py-2" style:color={changePctColor}>
								{result.callPriceChangePct >= 0 ? '+' : ''}{result.callPriceChangePct.toFixed(1)}%
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	{/if}
</div>

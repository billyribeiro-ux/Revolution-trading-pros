<script lang="ts">
	import { Plus, Trash2, BarChart3 } from '@lucide/svelte';
	import gsap from 'gsap';
	import StrategyLegRow from './ui/StrategyLegRow.svelte';
	import StrategyPresets from './StrategyPresets.svelte';
	import MetricCard from './ui/MetricCard.svelte';
	import { formatCurrency } from '../utils/formatters.js';
	import type { CalculatorState } from '../state/calculator.svelte.js';
	import type { StrategyLeg } from '../engine/types.js';

	interface Props {
		calc: CalculatorState;
	}

	let { calc }: Props = $props();

	let showPresets = $state(false);
	let addBtnEl: HTMLButtonElement | undefined = $state();

	const LEG_COLORS = ['#6366f1', '#8b5cf6', '#ec4899', '#f59e0b', '#10b981', '#3b82f6'];

	function addLeg() {
		calc.addStrategyLeg({
			type: 'call',
			strike: calc.strikePrice,
			expiry: calc.timeToExpiry,
			position: 1,
			quantity: 1,
			premium: 0
		});
		if (addBtnEl) {
			gsap.fromTo(addBtnEl, { scale: 0.9 }, { scale: 1, duration: 0.3, ease: 'back.out(2)' });
		}
	}

	function updateLeg(updated: StrategyLeg) {
		calc.removeStrategyLeg(updated.id);
		calc.addStrategyLeg({
			type: updated.type,
			strike: updated.strike,
			expiry: updated.expiry,
			position: updated.position,
			quantity: updated.quantity,
			premium: updated.premium
		});
	}

	function removeLeg(id: string) {
		calc.removeStrategyLeg(id);
	}

	let netPremium = $derived.by(() => {
		const legs = calc.strategyLegs;
		if (legs.length === 0) return 0;
		return legs.reduce((sum, l) => sum + l.premium, 0);
	});

	let netDelta = $derived.by(() => {
		return calc.currentGreeks.first.delta;
	});
</script>

<div class="flex flex-col gap-4">
	<!-- Header -->
	<div class="flex items-center justify-between">
		<div class="flex items-center gap-2">
			<BarChart3 size={16} style="color: var(--calc-accent);" />
			<h3
				class="text-sm font-semibold"
				style="color: var(--calc-text); font-family: var(--calc-font-display);"
			>
				Strategy Builder
			</h3>
			<span
				class="text-[10px] px-1.5 py-0.5 rounded"
				style="background: var(--calc-surface); color: var(--calc-text-muted); border: 1px solid var(--calc-border);"
			>
				{calc.strategyLegs.length} legs
			</span>
		</div>
		<div class="flex items-center gap-2">
			<button
				onclick={() => (showPresets = !showPresets)}
				class="text-xs px-2.5 py-1.5 rounded-lg transition-colors cursor-pointer"
				style="background: var(--calc-surface); color: var(--calc-text-secondary); border: 1px solid var(--calc-border);"
			>
				{showPresets ? 'Hide' : 'Show'} Presets
			</button>
			{#if calc.strategyLegs.length > 0}
				<button
					onclick={() => calc.clearStrategy()}
					class="flex items-center gap-1 text-xs px-2 py-1.5 rounded-lg transition-colors cursor-pointer"
					style="color: var(--calc-put); background: var(--calc-put-bg); border: 1px solid transparent;"
				>
					<Trash2 size={10} />
					Clear
				</button>
			{/if}
		</div>
	</div>

	<!-- Presets Panel -->
	{#if showPresets}
		<StrategyPresets {calc} onSelect={() => (showPresets = false)} />
	{/if}

	<!-- Legs -->
	<div class="flex flex-col gap-2">
		{#each calc.strategyLegs as leg, i (leg.id)}
			<StrategyLegRow
				{leg}
				spotPrice={calc.spotPrice}
				volatility={calc.volatility}
				riskFreeRate={calc.riskFreeRate}
				dividendYield={calc.dividendYield}
				color={LEG_COLORS[i % LEG_COLORS.length]}
				onUpdate={updateLeg}
				onRemove={removeLeg}
			/>
		{/each}
	</div>

	<!-- Add Leg Button -->
	<button
		bind:this={addBtnEl}
		onclick={addLeg}
		class="flex items-center justify-center gap-1.5 text-xs font-medium py-2.5 rounded-xl transition-colors cursor-pointer"
		style="background: var(--calc-surface); color: var(--calc-accent); border: 1px dashed var(--calc-accent); border-width: 1.5px;"
	>
		<Plus size={14} />
		Add Leg
	</button>

	<!-- Strategy Summary -->
	{#if calc.strategyLegs.length > 0}
		<div
			class="flex flex-col gap-3 rounded-xl p-3"
			style="background: var(--calc-surface); border: 1px solid var(--calc-border);"
		>
			<div class="flex items-center justify-between">
				<span class="text-xs font-medium" style="color: var(--calc-text-secondary);"
					>Strategy Summary</span
				>
				<span
					class="text-xs font-bold px-2 py-0.5 rounded"
					style="font-family: var(--calc-font-mono); {netPremium >= 0
						? 'color: var(--calc-call); background: var(--calc-call-bg);'
						: 'color: var(--calc-put); background: var(--calc-put-bg);'}"
				>
					{netPremium >= 0 ? 'Credit' : 'Debit'}
					{formatCurrency(Math.abs(netPremium))}
				</span>
			</div>

			<div class="grid grid-cols-4 gap-2">
				<MetricCard
					label="Max Profit"
					value={typeof calc.strategyMaxProfitLoss.maxProfit === 'number'
						? calc.strategyMaxProfitLoss.maxProfit
						: 99999}
					decimals={2}
					prefix="$"
					colorize
				/>
				<MetricCard
					label="Max Loss"
					value={typeof calc.strategyMaxProfitLoss.maxLoss === 'number'
						? -calc.strategyMaxProfitLoss.maxLoss
						: -99999}
					decimals={2}
					prefix="$"
					colorize
				/>
				<MetricCard
					label="Breakevens"
					value={calc.strategyBreakevens.length}
					decimals={0}
					colorize={false}
				/>
				<MetricCard label="Net Delta" value={netDelta} decimals={3} colorize />
			</div>

			{#if calc.strategyBreakevens.length > 0}
				<div class="flex items-center gap-2 flex-wrap">
					<span class="text-[10px]" style="color: var(--calc-text-muted);">B/E:</span>
					{#each calc.strategyBreakevens as be}
						<span
							class="text-xs px-1.5 py-0.5 rounded"
							style="background: var(--calc-surface-hover); color: var(--calc-warning); font-family: var(--calc-font-mono);"
						>
							{formatCurrency(be)}
						</span>
					{/each}
				</div>
			{/if}
		</div>
	{/if}
</div>

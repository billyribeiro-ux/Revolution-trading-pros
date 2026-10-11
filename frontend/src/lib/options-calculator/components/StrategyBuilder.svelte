<script lang="ts">
	import IconPlus from '@tabler/icons-svelte-runes/icons/plus';
	import IconTrash from '@tabler/icons-svelte-runes/icons/trash';
	import IconChartBar from '@tabler/icons-svelte-runes/icons/chart-bar';
	import { backOut } from 'svelte/easing';
	import { Tween, prefersReducedMotion } from 'svelte/motion';
	import StrategyLegRow from './ui/StrategyLegRow.svelte';
	import StrategyPresets from './StrategyPresets.svelte';
	import MetricCard from './ui/MetricCard.svelte';
	import { formatCurrency } from '../utils/formatters.js';
	import { getCalculator } from '../state/context.js';

	const calc = getCalculator();

	const LEG_COLORS = ['#6366f1', '#8b5cf6', '#ec4899', '#f59e0b', '#10b981', '#3b82f6'];
	/** Shown in place of an unbounded max profit / max loss. */
	const UNLIMITED = 99999;

	let showPresets = $state(false);

	/** Press feedback for the Add Leg button: snaps to 0.9 and springs back to 1. */
	const addButtonScale = new Tween(1, { duration: 300, easing: backOut });

	function addLeg() {
		calc.addStrategyLeg({
			type: 'call',
			strike: calc.strikePrice,
			expiry: calc.timeToExpiry,
			position: 1,
			quantity: 1,
			premium: 0
		});
		if (prefersReducedMotion.current) return;
		addButtonScale.set(0.9, { duration: 0 });
		addButtonScale.target = 1;
	}

	const maxProfit = $derived.by(() => {
		const profit = calc.strategyMaxProfitLoss.maxProfit;
		return typeof profit === 'number' ? profit : UNLIMITED;
	});

	// Signed like Max Profit (the lowest P&L on the payoff curve), so a loss reads negative
	// and red whether it is bounded or not.
	const maxLoss = $derived.by(() => {
		const loss = calc.strategyMaxProfitLoss.maxLoss;
		return typeof loss === 'number' ? loss : -UNLIMITED;
	});

	// A payoff that touches zero at a grid point yields the same breakeven twice;
	// de-duplicate so the keyed list below never sees a repeated key.
	const breakevens = $derived([...new Set(calc.strategyBreakevens)]);
</script>

<div class="strategy-builder">
	<!-- Header -->
	<div class="builder-header">
		<div class="title-group">
			<IconChartBar size={16} style="color: var(--calc-accent);" aria-hidden="true" />
			<h3 class="builder-title">Strategy Builder</h3>
			<span class="leg-count">
				{calc.strategyLegs.length} legs
			</span>
		</div>
		<div class="header-actions">
			<button
				type="button"
				onclick={() => (showPresets = !showPresets)}
				aria-expanded={showPresets}
				class="preset-toggle"
			>
				{showPresets ? 'Hide' : 'Show'} Presets
			</button>
			{#if calc.strategyLegs.length > 0}
				<button type="button" onclick={calc.clearStrategy} class="clear-button">
					<IconTrash size={10} aria-hidden="true" />
					Clear
				</button>
			{/if}
		</div>
	</div>

	<!-- Presets Panel -->
	{#if showPresets}
		<StrategyPresets onSelect={() => (showPresets = false)} />
	{/if}

	<!-- Legs -->
	<div class="legs-list">
		{#each calc.strategyLegs as leg, i (leg.id)}
			<StrategyLegRow
				{leg}
				spotPrice={calc.spotPrice}
				volatility={calc.volatility}
				riskFreeRate={calc.riskFreeRate}
				dividendYield={calc.dividendYield}
				color={LEG_COLORS[i % LEG_COLORS.length]}
				onUpdate={calc.updateStrategyLeg}
				onRemove={calc.removeStrategyLeg}
			/>
		{/each}
	</div>

	<!-- Add Leg Button -->
	<button
		type="button"
		onclick={addLeg}
		class="add-leg-button"
		style:scale={addButtonScale.current}
	>
		<IconPlus size={14} aria-hidden="true" />
		Add Leg
	</button>

	<!-- Strategy Summary -->
	{#if calc.strategyLegs.length > 0}
		<div class="summary-panel">
			<div class="summary-header">
				<span class="summary-title">Strategy Summary</span>
				<span class={['premium-badge', calc.strategyNetPremium >= 0 ? 'credit' : 'debit']}>
					{calc.strategyNetPremium >= 0 ? 'Credit' : 'Debit'}
					{formatCurrency(Math.abs(calc.strategyNetPremium))}
				</span>
			</div>

			<div class="metrics-grid">
				<MetricCard label="Max Profit" value={maxProfit} decimals={2} prefix="$" colorize />
				<MetricCard label="Max Loss" value={maxLoss} decimals={2} prefix="$" colorize />
				<MetricCard label="Breakevens" value={breakevens.length} decimals={0} colorize={false} />
				<MetricCard label="Net Delta" value={calc.strategyGreeks.netDelta} decimals={3} colorize />
			</div>

			{#if breakevens.length > 0}
				<div class="breakeven-list">
					<span class="breakeven-label">B/E:</span>
					{#each breakevens as be (be)}
						<span class="breakeven-value">
							{formatCurrency(be)}
						</span>
					{/each}
				</div>
			{/if}
		</div>
	{/if}
</div>

<style>
	.strategy-builder {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.builder-header,
	.title-group,
	.header-actions,
	.summary-header,
	.breakeven-list {
		display: flex;
		align-items: center;
	}

	.builder-header,
	.summary-header {
		justify-content: space-between;
	}

	.title-group,
	.header-actions,
	.breakeven-list {
		gap: 0.5rem;
	}

	.builder-title {
		margin: 0;
		color: var(--calc-text);
		font-family: var(--calc-font-display);
		font-size: 0.875rem;
		font-weight: 600;
		line-height: 1.25;
	}

	.leg-count,
	.breakeven-value {
		border-radius: 0.25rem;
		padding: 0.125rem 0.375rem;
	}

	.leg-count {
		border: 1px solid var(--calc-border);
		background: var(--calc-surface);
		color: var(--calc-text-muted);
		font-size: 0.625rem;
		line-height: 1.2;
	}

	button {
		border: 0;
		font: inherit;
	}

	.preset-toggle,
	.clear-button,
	.add-leg-button {
		cursor: pointer;
		transition:
			background-color 160ms ease,
			border-color 160ms ease,
			color 160ms ease,
			transform 160ms ease;
	}

	.preset-toggle,
	.clear-button {
		border-radius: 0.5rem;
		font-size: 0.75rem;
		line-height: 1.2;
	}

	.preset-toggle {
		border: 1px solid var(--calc-border);
		background: var(--calc-surface);
		color: var(--calc-text-secondary);
		padding: 0.375rem 0.625rem;
	}

	.clear-button {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		border: 1px solid transparent;
		background: var(--calc-put-bg);
		color: var(--calc-put);
		padding: 0.375rem 0.5rem;
	}

	.preset-toggle:hover,
	.clear-button:hover,
	.add-leg-button:hover {
		background: var(--calc-surface-hover);
	}

	.legs-list,
	.summary-panel {
		display: flex;
		flex-direction: column;
	}

	.legs-list {
		gap: 0.5rem;
	}

	.add-leg-button {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.375rem;
		border: 1.5px dashed var(--calc-accent);
		border-radius: 0.75rem;
		background: var(--calc-surface);
		color: var(--calc-accent);
		font-size: 0.75rem;
		font-weight: 500;
		padding-block: 0.625rem;
	}

	.summary-panel {
		gap: 0.75rem;
		border: 1px solid var(--calc-border);
		border-radius: 0.75rem;
		background: var(--calc-surface);
		padding: 0.75rem;
	}

	.summary-title {
		color: var(--calc-text-secondary);
		font-size: 0.75rem;
		font-weight: 500;
		line-height: 1.2;
	}

	.premium-badge {
		border-radius: 0.25rem;
		font-family: var(--calc-font-mono);
		font-size: 0.75rem;
		font-weight: 700;
		line-height: 1.2;
		padding: 0.125rem 0.5rem;
	}

	.premium-badge.credit {
		background: var(--calc-call-bg);
		color: var(--calc-call);
	}

	.premium-badge.debit {
		background: var(--calc-put-bg);
		color: var(--calc-put);
	}

	.metrics-grid {
		display: grid;
		grid-template-columns: repeat(4, minmax(0, 1fr));
		gap: 0.5rem;
	}

	.breakeven-list {
		flex-wrap: wrap;
	}

	.breakeven-label {
		color: var(--calc-text-muted);
		font-size: 0.625rem;
		line-height: 1.2;
	}

	.breakeven-value {
		background: var(--calc-surface-hover);
		color: var(--calc-warning);
		font-family: var(--calc-font-mono);
		font-size: 0.75rem;
		line-height: 1.2;
	}

	@media (prefers-reduced-motion: reduce) {
		.preset-toggle,
		.clear-button,
		.add-leg-button {
			transition: none;
		}
	}
</style>

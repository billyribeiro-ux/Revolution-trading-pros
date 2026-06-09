<script lang="ts">
	import IconPlus from '@tabler/icons-svelte-runes/icons/plus';
	import IconTrash from '@tabler/icons-svelte-runes/icons/trash';
	import IconChartBar from '@tabler/icons-svelte-runes/icons/chart-bar';
	import gsap from 'gsap';
	import type { Attachment } from 'svelte/attachments';
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
	let addButtonElement: HTMLButtonElement | undefined = $state();

	const LEG_COLORS = ['#6366f1', '#8b5cf6', '#ec4899', '#f59e0b', '#10b981', '#3b82f6'];

	const attachAddButton: Attachment<HTMLButtonElement> = (element) => {
		addButtonElement = element;

		return () => {
			if (addButtonElement === element) {
				addButtonElement = undefined;
			}
		};
	};

	function addLeg() {
		calc.addStrategyLeg({
			type: 'call',
			strike: calc.strikePrice,
			expiry: calc.timeToExpiry,
			position: 1,
			quantity: 1,
			premium: 0
		});
		if (addButtonElement) {
			gsap.fromTo(
				addButtonElement,
				{ scale: 0.9 },
				{ scale: 1, duration: 0.3, ease: 'back.out(2)' }
			);
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

	const premiumVariant = $derived(netPremium >= 0 ? 'credit' : 'debit');
	const premiumTextColor = $derived(
		premiumVariant === 'credit' ? 'var(--calc-call)' : 'var(--calc-put)'
	);
	const premiumBackground = $derived(
		premiumVariant === 'credit' ? 'var(--calc-call-bg)' : 'var(--calc-put-bg)'
	);

	let netDelta = $derived.by(() => {
		return calc.currentGreeks.first.delta;
	});
</script>

<div class="strategy-builder">
	<!-- Header -->
	<div class="builder-header">
		<div class="title-group">
			<IconChartBar size={16} style="color: var(--calc-accent);" />
			<h3 class="builder-title">Strategy Builder</h3>
			<span class="leg-count">
				{calc.strategyLegs.length} legs
			</span>
		</div>
		<div class="header-actions">
			<button onclick={() => (showPresets = !showPresets)} class="preset-toggle">
				{showPresets ? 'Hide' : 'Show'} Presets
			</button>
			{#if calc.strategyLegs.length > 0}
				<button onclick={() => calc.clearStrategy()} class="clear-button">
					<IconTrash size={10} />
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
	<div class="legs-list">
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
	<button {@attach attachAddButton} onclick={addLeg} class="add-leg-button">
		<IconPlus size={14} />
		Add Leg
	</button>

	<!-- Strategy Summary -->
	{#if calc.strategyLegs.length > 0}
		<div class="summary-panel">
			<div class="summary-header">
				<span class="summary-title">Strategy Summary</span>
				<span
					class="premium-badge"
					style:color={premiumTextColor}
					style:background={premiumBackground}
				>
					{netPremium >= 0 ? 'Credit' : 'Debit'}
					{formatCurrency(Math.abs(netPremium))}
				</span>
			</div>

			<div class="metrics-grid">
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
				<div class="breakeven-list">
					<span class="breakeven-label">B/E:</span>
					{#each calc.strategyBreakevens as be (be)}
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
</style>

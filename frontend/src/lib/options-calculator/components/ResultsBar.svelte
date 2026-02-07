<script lang="ts">
	import AnimatedNumber from './ui/AnimatedNumber.svelte';
	import MetricCard from './ui/MetricCard.svelte';
	import { formatCurrency, formatPercent } from '../utils/formatters.js';
	import type { CalculatorState } from '../state/calculator.svelte.js';

	interface Props {
		calc: CalculatorState;
	}

	let { calc }: Props = $props();

	let moneynessColor = $derived(
		calc.currentMoneyness === 'ITM'
			? 'var(--calc-call)'
			: calc.currentMoneyness === 'OTM'
				? 'var(--calc-put)'
				: 'var(--calc-accent)'
	);
</script>

<div class="flex flex-col gap-4">
	<!-- Price Display -->
	<div class="grid grid-cols-2 gap-3">
		<!-- Call Price -->
		<div
			class="flex flex-col gap-1 rounded-xl p-4 transition-all"
			style="background: var(--calc-call-bg); border: 1px solid {calc.optionType === 'call' ? 'var(--calc-call)' : 'transparent'};"
		>
			<span class="text-[10px] font-medium uppercase tracking-wider" style="color: var(--calc-text-muted);">
				Call Price
			</span>
			<AnimatedNumber
				value={calc.pricing.callPrice}
				decimals={4}
				prefix="$"
				class="text-2xl font-bold"
				colorize={false}
			/>
		</div>

		<!-- Put Price -->
		<div
			class="flex flex-col gap-1 rounded-xl p-4 transition-all"
			style="background: var(--calc-put-bg); border: 1px solid {calc.optionType === 'put' ? 'var(--calc-put)' : 'transparent'};"
		>
			<span class="text-[10px] font-medium uppercase tracking-wider" style="color: var(--calc-text-muted);">
				Put Price
			</span>
			<AnimatedNumber
				value={calc.pricing.putPrice}
				decimals={4}
				prefix="$"
				class="text-2xl font-bold"
				colorize={false}
			/>
		</div>
	</div>

	<!-- Key Metrics Row -->
	<div class="flex flex-wrap items-center gap-2">
		<!-- Moneyness Badge -->
		<span
			class="inline-flex items-center px-2.5 py-1 rounded-full text-xs font-bold"
			style="background: {moneynessColor}22; color: {moneynessColor}; border: 1px solid {moneynessColor}44;"
		>
			{calc.currentMoneyness}
		</span>

		<!-- Intrinsic / Extrinsic -->
		<span class="text-xs" style="color: var(--calc-text-secondary); font-family: var(--calc-font-mono);">
			Intrinsic: {formatCurrency(calc.currentIntrinsic)}
		</span>
		<span class="text-xs" style="color: var(--calc-text-muted);">|</span>
		<span class="text-xs" style="color: var(--calc-text-secondary); font-family: var(--calc-font-mono);">
			Extrinsic: {formatCurrency(calc.currentExtrinsic)}
		</span>
		<span class="text-xs" style="color: var(--calc-text-muted);">|</span>
		<span class="text-xs" style="color: var(--calc-text-secondary); font-family: var(--calc-font-mono);">
			B/E: {formatCurrency(calc.currentBreakeven)}
		</span>
	</div>

	<!-- Greeks Row -->
	<div class="grid grid-cols-5 gap-2">
		<MetricCard label="Delta" value={calc.currentGreeks.first.delta} decimals={4} colorize />
		<MetricCard label="Gamma" value={calc.currentGreeks.first.gamma} decimals={6} colorize />
		<MetricCard label="Theta" value={calc.currentGreeks.first.theta} decimals={4} colorize />
		<MetricCard label="Vega" value={calc.currentGreeks.first.vega} decimals={4} colorize />
		<MetricCard label="Rho" value={calc.currentGreeks.first.rho} decimals={4} colorize />
	</div>

	<!-- Probability Row -->
	<div class="grid grid-cols-3 gap-2">
		<div
			class="flex flex-col gap-0.5 rounded-lg px-3 py-2"
			style="background: var(--calc-surface); border: 1px solid var(--calc-border);"
		>
			<span class="text-[10px] uppercase tracking-wider" style="color: var(--calc-text-muted);">P(ITM)</span>
			<span class="text-sm font-semibold" style="color: var(--calc-call); font-family: var(--calc-font-mono);">
				{formatPercent(calc.probabilities.probabilityITM)}
			</span>
		</div>
		<div
			class="flex flex-col gap-0.5 rounded-lg px-3 py-2"
			style="background: var(--calc-surface); border: 1px solid var(--calc-border);"
		>
			<span class="text-[10px] uppercase tracking-wider" style="color: var(--calc-text-muted);">Expected Move</span>
			<span class="text-sm font-semibold" style="color: var(--calc-accent); font-family: var(--calc-font-mono);">
				±{formatCurrency(calc.probabilities.expectedMove)}
			</span>
		</div>
		<div
			class="flex flex-col gap-0.5 rounded-lg px-3 py-2"
			style="background: var(--calc-surface); border: 1px solid var(--calc-border);"
		>
			<span class="text-[10px] uppercase tracking-wider" style="color: var(--calc-text-muted);">1σ Range</span>
			<span class="text-xs font-semibold" style="color: var(--calc-text-secondary); font-family: var(--calc-font-mono);">
				{formatCurrency(calc.probabilities.oneSDRange[0])} – {formatCurrency(calc.probabilities.oneSDRange[1])}
			</span>
		</div>
	</div>
</div>

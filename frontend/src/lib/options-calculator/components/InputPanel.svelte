<script lang="ts">
	import { RotateCcw } from '@lucide/svelte';
	import AnimatedSlider from './ui/AnimatedSlider.svelte';
	import InfoTooltip from './ui/InfoTooltip.svelte';
	import { INPUT_FIELDS } from '../engine/constants.js';
	import { solveImpliedVolatility } from '../engine/implied-volatility.js';
	import { formatDaysToExpiry } from '../utils/formatters.js';
	import type { CalculatorState } from '../state/calculator.svelte.js';
	import type { OptionType, BSInputs } from '../engine/types.js';

	interface Props {
		calc: CalculatorState;
	}

	let { calc }: Props = $props();

	let marketPriceInput = $state('');
	let ivResult = $state<{ iv: number; converged: boolean } | null>(null);

	const SYMBOLS: Record<keyof BSInputs, string> = {
		spotPrice: 'S',
		strikePrice: 'K',
		volatility: 'σ',
		timeToExpiry: 'T',
		riskFreeRate: 'r',
		dividendYield: 'q',
	};

	function handleOptionTypeChange(type: OptionType) {
		calc.optionType = type;
	}

	function handleInputChange(key: keyof BSInputs, value: number) {
		calc.updateInput(key, value);
	}

	function solveIV() {
		const marketPrice = parseFloat(marketPriceInput);
		if (isNaN(marketPrice) || marketPrice <= 0) return;

		const result = solveImpliedVolatility(marketPrice, calc.inputs, calc.optionType);
		ivResult = { iv: result.impliedVolatility, converged: result.converged };

		if (result.converged) {
			calc.volatility = result.impliedVolatility;
		}
	}

	let daysDisplay = $derived(formatDaysToExpiry(calc.timeToExpiry));
</script>

<div class="flex flex-col gap-4 h-full overflow-y-auto calc-scrollbar">
	<!-- Header -->
	<div class="flex items-center justify-between">
		<h2
			class="text-sm font-semibold tracking-wide uppercase"
			style="color: var(--calc-text); font-family: var(--calc-font-display);"
		>
			Parameters
		</h2>
		<button
			onclick={() => calc.resetInputs()}
			class="flex items-center gap-1.5 text-xs px-2.5 py-1.5 rounded-lg transition-colors cursor-pointer"
			style="background: var(--calc-surface-hover); color: var(--calc-text-secondary); border: 1px solid var(--calc-border);"
			aria-label="Reset all inputs"
		>
			<RotateCcw size={12} />
			Reset
		</button>
	</div>

	<!-- Option Type Toggle -->
	<div
		class="relative flex rounded-xl p-1"
		style="background: var(--calc-surface); border: 1px solid var(--calc-border);"
	>
		<button
			onclick={() => handleOptionTypeChange('call')}
			class="flex-1 text-center text-sm font-semibold py-2 rounded-lg transition-all cursor-pointer"
			style={calc.optionType === 'call'
				? 'background: var(--calc-call-bg); color: var(--calc-call); box-shadow: 0 0 12px var(--calc-call-glow);'
				: 'color: var(--calc-text-muted);'}
		>
			CALL
		</button>
		<button
			onclick={() => handleOptionTypeChange('put')}
			class="flex-1 text-center text-sm font-semibold py-2 rounded-lg transition-all cursor-pointer"
			style={calc.optionType === 'put'
				? 'background: var(--calc-put-bg); color: var(--calc-put); box-shadow: 0 0 12px var(--calc-put-glow);'
				: 'color: var(--calc-text-muted);'}
		>
			PUT
		</button>
	</div>

	<!-- Input Sliders -->
	<div class="flex flex-col gap-4">
		{#each INPUT_FIELDS as field (field.key)}
			<div class="relative">
				{#if calc.isLivePopulated(field.key)}
					<span
						class="absolute -top-1 right-0 text-[8px] font-bold px-1 py-0.5 rounded z-10"
						style="background: var(--calc-call-bg); color: var(--calc-call);"
					>LIVE</span>
				{/if}
				<AnimatedSlider
					value={calc[field.key] as number}
					min={field.min}
					max={field.max}
					step={field.step}
					label={field.label}
					unit={field.unit}
					displayMultiplier={field.displayMultiplier ?? 1}
					displayDecimals={field.displayDecimals ?? 2}
					symbol={SYMBOLS[field.key]}
					onchange={(v) => {
						handleInputChange(field.key, v);
						if (calc.isLivePopulated(field.key)) {
							calc.clearLiveOverride(field.key);
						}
					}}
				/>
				{#if field.key === 'timeToExpiry'}
					<div
						class="mt-1 text-[10px] text-center"
						style="color: var(--calc-text-muted); font-family: var(--calc-font-mono);"
					>
						≈ {daysDisplay}
					</div>
				{/if}
			</div>
		{/each}
	</div>

	<!-- IV Solver -->
	<div
		class="flex flex-col gap-2 rounded-xl p-3"
		style="background: var(--calc-surface); border: 1px solid var(--calc-border);"
	>
		<div class="flex items-center gap-1.5">
			<span
				class="text-xs font-medium"
				style="color: var(--calc-text-secondary); font-family: var(--calc-font-body);"
			>
				IV Solver
			</span>
			<InfoTooltip content="Enter a market price to reverse-engineer the implied volatility using Newton-Raphson iteration." />
		</div>
		<div class="flex gap-2">
			<input
				type="number"
				bind:value={marketPriceInput}
				placeholder="Market price"
				class="flex-1 text-xs px-2.5 py-1.5 rounded-lg outline-none"
				style="background: var(--calc-surface-hover); color: var(--calc-text); border: 1px solid var(--calc-border); font-family: var(--calc-font-mono);"
			/>
			<button
				onclick={solveIV}
				class="text-xs font-medium px-3 py-1.5 rounded-lg transition-colors cursor-pointer"
				style="background: var(--calc-accent); color: white;"
			>
				Solve
			</button>
		</div>
		{#if ivResult}
			<div
				class="text-xs"
				style="color: {ivResult.converged ? 'var(--calc-call)' : 'var(--calc-warning)'}; font-family: var(--calc-font-mono);"
			>
				{#if ivResult.converged}
					IV = {(ivResult.iv * 100).toFixed(2)}% ✓
				{:else}
					Did not converge — try a different price
				{/if}
			</div>
		{/if}
	</div>
</div>

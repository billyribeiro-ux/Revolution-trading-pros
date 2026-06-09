<script lang="ts">
	import IconRotate2 from '@tabler/icons-svelte-runes/icons/rotate-2';
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
		dividendYield: 'q'
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
	const ivResultClass = $derived(ivResult?.converged ? 'iv-result--success' : 'iv-result--warning');

	function optionButtonClass(type: OptionType) {
		return [
			'option-button',
			`option-button--${type}`,
			calc.optionType === type && 'option-button--active'
		];
	}
</script>

<div class="input-panel calc-scrollbar">
	<!-- Header -->
	<div class="panel-header">
		<h2 class="panel-title">Parameters</h2>
		<button onclick={() => calc.resetInputs()} class="reset-button" aria-label="Reset all inputs">
			<IconRotate2 size={12} />
			Reset
		</button>
	</div>

	<!-- Option Type Toggle -->
	<div class="option-toggle">
		<button onclick={() => handleOptionTypeChange('call')} class={optionButtonClass('call')}>
			CALL
		</button>
		<button onclick={() => handleOptionTypeChange('put')} class={optionButtonClass('put')}>
			PUT
		</button>
	</div>

	<!-- Input Sliders -->
	<div class="slider-list">
		{#each INPUT_FIELDS as field (field.key)}
			<div class="slider-row">
				{#if calc.isLivePopulated(field.key)}
					<span class="live-badge">LIVE</span>
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
					<div class="days-display">
						≈ {daysDisplay}
					</div>
				{/if}
			</div>
		{/each}
	</div>

	<!-- IV Solver -->
	<div class="iv-solver">
		<div class="iv-heading">
			<span class="iv-title">IV Solver</span>
			<InfoTooltip
				content="Enter a market price to reverse-engineer the implied volatility using Newton-Raphson iteration."
			/>
		</div>
		<div class="iv-controls">
			<input
				type="number"
				bind:value={marketPriceInput}
				placeholder="Market price"
				class="market-price-input"
			/>
			<button onclick={solveIV} class="solve-button"> Solve </button>
		</div>
		{#if ivResult}
			<div class={['iv-result', ivResultClass]}>
				{#if ivResult.converged}
					IV = {(ivResult.iv * 100).toFixed(2)}% ✓
				{:else}
					Did not converge — try a different price
				{/if}
			</div>
		{/if}
	</div>
</div>

<style>
	.input-panel {
		display: flex;
		flex-direction: column;
		gap: 1rem;
		height: 100%;
		overflow-y: auto;
	}

	.panel-header,
	.iv-heading,
	.iv-controls {
		display: flex;
		align-items: center;
	}

	.panel-header {
		justify-content: space-between;
	}

	.iv-heading {
		gap: 0.375rem;
	}

	.iv-controls {
		gap: 0.5rem;
	}

	.panel-title {
		margin: 0;
		color: var(--calc-text);
		font-family: var(--calc-font-display);
		font-size: 0.875rem;
		font-weight: 600;
		letter-spacing: 0.025em;
		line-height: 1.25;
		text-transform: uppercase;
	}

	button,
	input {
		font: inherit;
	}

	button {
		border: 0;
	}

	.reset-button,
	.option-button,
	.solve-button {
		cursor: pointer;
		transition:
			background-color 160ms ease,
			border-color 160ms ease,
			box-shadow 160ms ease,
			color 160ms ease;
	}

	.reset-button {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		border: 1px solid var(--calc-border);
		border-radius: 0.5rem;
		background: var(--calc-surface-hover);
		color: var(--calc-text-secondary);
		font-size: 0.75rem;
		line-height: 1.2;
		padding: 0.375rem 0.625rem;
	}

	.reset-button:hover {
		color: var(--calc-text);
		border-color: var(--calc-accent);
	}

	.option-toggle {
		position: relative;
		display: flex;
		border: 1px solid var(--calc-border);
		border-radius: 0.75rem;
		background: var(--calc-surface);
		padding: 0.25rem;
	}

	.option-button {
		flex: 1;
		border-radius: 0.5rem;
		background: transparent;
		color: var(--calc-text-muted);
		font-size: 0.875rem;
		font-weight: 600;
		line-height: 1.25;
		padding-block: 0.5rem;
		text-align: center;
	}

	.option-button--call.option-button--active {
		background: var(--calc-call-bg);
		color: var(--calc-call);
		box-shadow: 0 0 12px var(--calc-call-glow);
	}

	.option-button--put.option-button--active {
		background: var(--calc-put-bg);
		color: var(--calc-put);
		box-shadow: 0 0 12px var(--calc-put-glow);
	}

	.slider-list,
	.iv-solver {
		display: flex;
		flex-direction: column;
	}

	.slider-list {
		gap: 1rem;
	}

	.slider-row {
		position: relative;
	}

	.live-badge {
		position: absolute;
		top: -0.25rem;
		right: 0;
		z-index: 10;
		border-radius: 0.25rem;
		background: var(--calc-call-bg);
		color: var(--calc-call);
		font-size: 0.5rem;
		font-weight: 700;
		line-height: 1.2;
		padding: 0.125rem 0.25rem;
	}

	.days-display {
		margin-top: 0.25rem;
		color: var(--calc-text-muted);
		font-family: var(--calc-font-mono);
		font-size: 0.625rem;
		line-height: 1.2;
		text-align: center;
	}

	.iv-solver {
		gap: 0.5rem;
		border: 1px solid var(--calc-border);
		border-radius: 0.75rem;
		background: var(--calc-surface);
		padding: 0.75rem;
	}

	.iv-title {
		color: var(--calc-text-secondary);
		font-family: var(--calc-font-body);
		font-size: 0.75rem;
		font-weight: 500;
		line-height: 1.2;
	}

	.market-price-input {
		flex: 1;
		min-width: 0;
		border: 1px solid var(--calc-border);
		border-radius: 0.5rem;
		background: var(--calc-surface-hover);
		color: var(--calc-text);
		font-family: var(--calc-font-mono);
		font-size: 0.75rem;
		line-height: 1.2;
		outline: none;
		padding: 0.375rem 0.625rem;
	}

	.market-price-input:focus {
		border-color: var(--calc-accent);
	}

	.solve-button {
		border-radius: 0.5rem;
		background: var(--calc-accent);
		color: white;
		font-size: 0.75rem;
		font-weight: 500;
		line-height: 1.2;
		padding: 0.375rem 0.75rem;
	}

	.solve-button:hover {
		filter: brightness(1.08);
	}

	.iv-result {
		font-family: var(--calc-font-mono);
		font-size: 0.75rem;
		line-height: 1.2;
	}

	.iv-result--success {
		color: var(--calc-call);
	}

	.iv-result--warning {
		color: var(--calc-warning);
	}
</style>

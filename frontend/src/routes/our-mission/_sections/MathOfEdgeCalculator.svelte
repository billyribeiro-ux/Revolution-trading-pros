<script lang="ts">
	/**
	 * Mathematics of Edge — Interactive Risk Calculator
	 * Self-contained component: all state, derivations, and inputs are scoped here.
	 * Extracted from /our-mission/+page.svelte (Cascade audit, May 2026).
	 */
	let simAccount = $state(25000);
	let simRisk = $state(2);
	let simWinRate = $state(55);
	let simRR = $state(2);

	const riskAmount = $derived(Math.round(simAccount * (simRisk / 100)));
	const winAmount = $derived(Math.round(riskAmount * simRR));
	const expectedValue = $derived(
		(simWinRate / 100) * winAmount - (1 - simWinRate / 100) * riskAmount
	);
	const tradesToDouble = $derived(Math.ceil(simAccount / expectedValue));
	const riskOfRuin = $derived(
		simRisk > 5
			? 'HIGH (Casino Mode)'
			: simRisk > 2
				? 'MODERATE (Aggressive)'
				: 'LOW (Institutional)'
	);
	const riskToneClass = $derived(
		simRisk > 5 ? 'tone-danger' : simRisk > 2 ? 'tone-warning' : 'tone-success'
	);
	const institutionalViabilityClass = $derived(simRisk <= 2 ? 'tone-success' : 'tone-danger');
	const expectedValueClass = $derived(expectedValue > 0 ? 'tone-neutral' : 'tone-danger');
</script>

<section class="edge-calculator">
	<div class="calculator-glow"></div>

	<div class="calculator-container">
		<div class="section-heading">
			<h2>The Mathematics of Edge</h2>
			<p>
				Trading is not magic. It is statistics. Use this institutional risk calculator to see how
				small changes in risk management define your survival.
			</p>
		</div>

		<div class="calculator-card">
			<div class="controls-column">
				<div class="control-group">
					<label for="sim-account" class="field-label">Account Size ($)</label>
					<div class="input-wrap input-wrap--currency">
						<span class="currency-prefix">$</span>
						<input
							id="sim-account"
							name="sim-account"
							type="number"
							bind:value={simAccount}
							class="numeric-input numeric-input--with-prefix"
							autocomplete="off"
						/>
					</div>
				</div>
				<div class="control-group">
					<label for="sim-risk" class="field-label field-label--inline">
						<span>Risk Per Trade (%)</span>
						<span class={riskToneClass}>{simRisk}%</span>
					</label>
					<input
						id="sim-risk"
						name="sim-risk"
						type="range"
						min="0.5"
						max="10"
						step="0.5"
						bind:value={simRisk}
						class="risk-slider"
						autocomplete="off"
					/>
					<div class="slider-scale">
						<span>Conservative (0.5%)</span>
						<span>Gambler (10%)</span>
					</div>
				</div>
				<div class="compact-fields">
					<div class="control-group">
						<label for="sim-winrate" class="field-label field-label--compact">Win Rate (%)</label>
						<input
							id="sim-winrate"
							name="sim-winrate"
							type="number"
							bind:value={simWinRate}
							class="numeric-input numeric-input--compact"
							autocomplete="off"
						/>
					</div>
					<div class="control-group">
						<label for="sim-rr" class="field-label field-label--compact">R:R Ratio</label>
						<input
							id="sim-rr"
							name="sim-rr"
							type="number"
							step="0.1"
							bind:value={simRR}
							class="numeric-input numeric-input--compact"
							autocomplete="off"
						/>
					</div>
				</div>
			</div>

			<div class="results-column">
				<div class="metrics-grid">
					<div class="metric-tile">
						<div class="metric-label">Dollar Risk (1R)</div>
						<div class="metric-value tone-neutral">${riskAmount}</div>
					</div>
					<div class="metric-tile">
						<div class="metric-label">Potential Reward</div>
						<div class="metric-value tone-success">${winAmount}</div>
					</div>
					<div class="metric-tile">
						<div class="metric-label">Trades to Double</div>
						<div class="metric-value tone-primary">
							{tradesToDouble > 0 ? tradesToDouble : '∞'}
						</div>
					</div>
					<div class="metric-tile">
						<div class="metric-label">Expectancy (per trade)</div>
						<div class={['metric-value', expectedValueClass]}>
							${expectedValue.toFixed(2)}
						</div>
					</div>
				</div>

				<div class="analysis-panel">
					<div class="analysis-label">Analysis Output</div>
					<div class="analysis-rows">
						<div class="analysis-row">
							<span class="analysis-name">Risk Classification</span>
							<span class={['analysis-value', riskToneClass]}>{riskOfRuin}</span>
						</div>
						<div class="analysis-row">
							<span class="analysis-name">Institutional Viability</span>
							<span class={['analysis-value', institutionalViabilityClass]}>
								{simRisk <= 2 ? 'PASS' : 'FAIL'}
							</span>
						</div>
						<p class="analysis-copy">
							{#if simRisk > 2}
								<span class="tone-danger-soft">WARNING:</span> Your risk per trade is too high for
								institutional standards. A standard drawdown sequence (4-5 losses) will cause
								significant emotional damage (-{simRisk * 5}% equity).
							{:else}
								<span class="tone-success">OPTIMAL:</span> Your sizing allows you to weather
								statistical variance. You can survive a 10-trade losing streak with only {simRisk *
									10}% drawdown.
							{/if}
						</p>
					</div>
				</div>
			</div>
		</div>
	</div>
</section>

<style>
	.edge-calculator {
		position: relative;
		overflow: hidden;
		border-bottom: 1px solid rgb(255 255 255 / 0.05);
		background: #080808;
		padding-block: 8rem;
	}

	.calculator-glow {
		position: absolute;
		top: 5rem;
		left: -10rem;
		width: 24rem;
		height: 24rem;
		pointer-events: none;
		border-radius: 999px;
		background: rgb(16 185 129 / 0.05);
		filter: blur(100px);
	}

	.calculator-container {
		position: relative;
		z-index: 1;
		width: min(100% - 2rem, 72rem);
		margin-inline: auto;
	}

	.section-heading {
		max-width: 42rem;
		margin: 0 auto 4rem;
		text-align: center;
	}

	.section-heading h2 {
		margin: 0 0 1rem;
		color: #fff;
		font-family: var(--font-heading);
		font-size: clamp(1.875rem, 4vw, 2.25rem);
		font-weight: 700;
		line-height: 1.12;
	}

	.section-heading p {
		margin: 0;
		color: rgb(148 163 184);
		line-height: 1.65;
	}

	.calculator-card {
		display: grid;
		grid-template-columns: minmax(0, 1fr);
		gap: 2rem;
		border: 1px solid rgb(255 255 255 / 0.1);
		border-radius: 1.5rem;
		background: #050505;
		box-shadow: 0 25px 50px -12px rgb(0 0 0 / 0.25);
		padding: 2rem;
	}

	.controls-column {
		display: flex;
		flex-direction: column;
		gap: 2rem;
	}

	.control-group {
		min-width: 0;
	}

	.field-label {
		display: block;
		margin-bottom: 1rem;
		color: rgb(100 116 139);
		font-size: 0.75rem;
		font-weight: 700;
		letter-spacing: 0.14em;
		line-height: 1.2;
		text-transform: uppercase;
	}

	.field-label--compact {
		margin-bottom: 0.5rem;
	}

	.field-label--inline {
		display: flex;
		justify-content: space-between;
		gap: 1rem;
	}

	.input-wrap {
		position: relative;
	}

	.currency-prefix {
		position: absolute;
		top: 0.75rem;
		left: 1rem;
		color: rgb(100 116 139);
		font-family:
			ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, 'Liberation Mono', 'Courier New',
			monospace;
		line-height: 1.5;
	}

	.numeric-input {
		width: 100%;
		border: 1px solid rgb(255 255 255 / 0.1);
		border-radius: 0.5rem;
		background: rgb(255 255 255 / 0.05);
		color: #fff;
		font-family:
			ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, 'Liberation Mono', 'Courier New',
			monospace;
		outline: none;
		transition: border-color 160ms ease;
	}

	.numeric-input:focus {
		border-color: var(--rtp-primary);
	}

	.numeric-input--with-prefix {
		padding: 0.75rem 1rem 0.75rem 2rem;
	}

	.numeric-input--compact {
		padding: 0.5rem 0.75rem;
	}

	.risk-slider {
		width: 100%;
		cursor: pointer;
		accent-color: var(--rtp-primary);
	}

	.slider-scale {
		display: flex;
		justify-content: space-between;
		gap: 1rem;
		margin-top: 0.5rem;
		color: rgb(71 85 105);
		font-family:
			ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, 'Liberation Mono', 'Courier New',
			monospace;
		font-size: 0.625rem;
		line-height: 1.2;
	}

	.compact-fields {
		display: grid;
		grid-template-columns: repeat(2, minmax(0, 1fr));
		gap: 1rem;
	}

	.results-column {
		display: flex;
		flex-direction: column;
		justify-content: center;
	}

	.metrics-grid {
		display: grid;
		grid-template-columns: repeat(2, minmax(0, 1fr));
		gap: 1.5rem;
		margin-bottom: 2rem;
	}

	.metric-tile {
		border: 1px solid rgb(255 255 255 / 0.05);
		border-radius: 0.5rem;
		background: rgb(255 255 255 / 0.02);
		padding: 1rem;
	}

	.metric-label {
		margin-bottom: 0.25rem;
		color: rgb(100 116 139);
		font-size: 0.75rem;
		line-height: 1.2;
		text-transform: uppercase;
	}

	.metric-value,
	.analysis-value {
		font-family:
			ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, 'Liberation Mono', 'Courier New',
			monospace;
		font-weight: 700;
	}

	.metric-value {
		font-size: 1.5rem;
		line-height: 1.25;
	}

	.analysis-panel {
		position: relative;
		overflow: hidden;
		border: 1px solid rgb(255 255 255 / 0.1);
		border-radius: 0.75rem;
		background: rgb(0 0 0 / 0.4);
		padding: 1.5rem;
	}

	.analysis-label {
		position: absolute;
		top: 0.5rem;
		right: 1rem;
		color: rgb(71 85 105);
		font-family:
			ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, 'Liberation Mono', 'Courier New',
			monospace;
		font-size: 0.625rem;
		letter-spacing: 0.14em;
		line-height: 1.2;
		text-transform: uppercase;
	}

	.analysis-rows {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.analysis-row {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		border-bottom: 1px solid rgb(255 255 255 / 0.05);
		padding-bottom: 0.5rem;
	}

	.analysis-name {
		color: rgb(148 163 184);
		font-size: 0.875rem;
		line-height: 1.4;
	}

	.analysis-copy {
		margin: 0.5rem 0 0;
		color: rgb(100 116 139);
		font-size: 0.75rem;
		line-height: 1.65;
	}

	.analysis-copy span {
		font-weight: 700;
	}

	.tone-neutral {
		color: #fff;
	}

	.tone-primary {
		color: var(--rtp-primary);
	}

	.tone-success {
		color: rgb(52 211 153);
	}

	.tone-warning {
		color: rgb(251 146 60);
	}

	.tone-danger {
		color: rgb(239 68 68);
	}

	.tone-danger-soft {
		color: rgb(248 113 113);
	}

	@media (min-width: 640px) {
		.calculator-container {
			width: min(100% - 3rem, 72rem);
		}
	}

	@media (min-width: 768px) {
		.calculator-card {
			grid-template-columns: repeat(12, minmax(0, 1fr));
			padding: 3rem;
		}

		.controls-column {
			grid-column: span 4;
		}

		.results-column {
			grid-column: span 8;
			border-left: 1px solid rgb(255 255 255 / 0.05);
			padding-left: 3rem;
		}
	}

	@media (max-width: 640px) {
		.edge-calculator {
			padding-block: 5rem;
		}

		.calculator-card {
			padding: 1.25rem;
		}

		.compact-fields,
		.metrics-grid {
			grid-template-columns: 1fr;
		}

		.analysis-row {
			align-items: flex-start;
			flex-direction: column;
			gap: 0.25rem;
		}
	}
</style>

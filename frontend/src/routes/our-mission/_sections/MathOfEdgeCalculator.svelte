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
	const riskColor = $derived(
		simRisk > 5 ? 'text-red-500' : simRisk > 2 ? 'text-orange-400' : 'text-emerald-400'
	);
</script>

<section class="py-32 bg-[#080808] border-b border-white/5 relative overflow-hidden">
	<div
		class="absolute -left-40 top-20 w-96 h-96 bg-emerald-500/5 blur-[100px] rounded-full pointer-events-none"
	></div>

	<div class="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8 relative z-10">
		<div class="text-center mb-16">
			<h2 class="text-3xl md:text-4xl font-heading font-bold text-white mb-4">
				The Mathematics of Edge
			</h2>
			<p class="text-slate-400 max-w-2xl mx-auto">
				Trading is not magic. It is statistics. Use this institutional risk calculator to see how
				small changes in risk management define your survival.
			</p>
		</div>

		<div
			class="grid md:grid-cols-12 gap-8 bg-[#050505] border border-white/10 rounded-3xl p-8 md:p-12 shadow-2xl"
		>
			<div class="md:col-span-4 space-y-8">
				<div>
					<label
						for="sim-account"
						class="text-xs uppercase tracking-widest text-slate-500 font-bold mb-4 block"
						>Account Size ($)</label
					>
					<div class="relative">
						<span class="absolute left-4 top-3 text-slate-500">$</span>
						<input
							id="sim-account"
							name="sim-account"
							type="number"
							bind:value={simAccount}
							class="w-full bg-white/5 border border-white/10 rounded-lg py-3 pl-8 pr-4 text-white font-mono focus:border-rtp-primary focus:outline-none transition-colors"
							autocomplete="off"
						/>
					</div>
				</div>
				<div>
					<label
						for="sim-risk"
						class="text-xs uppercase tracking-widest text-slate-500 font-bold mb-4 flex justify-between"
					>
						<span>Risk Per Trade (%)</span>
						<span class={riskColor}>{simRisk}%</span>
					</label>
					<input
						id="sim-risk"
						name="sim-risk"
						type="range"
						min="0.5"
						max="10"
						step="0.5"
						bind:value={simRisk}
						class="w-full accent-rtp-primary cursor-pointer"
						autocomplete="off"
					/>
					<div class="flex justify-between text-[10px] text-slate-600 mt-2 font-mono">
						<span>Conservative (0.5%)</span>
						<span>Gambler (10%)</span>
					</div>
				</div>
				<div class="grid grid-cols-2 gap-4">
					<div>
						<label
							for="sim-winrate"
							class="text-xs uppercase tracking-widest text-slate-500 font-bold mb-2 block"
							>Win Rate (%)</label
						>
						<input
							id="sim-winrate"
							name="sim-winrate"
							type="number"
							bind:value={simWinRate}
							class="w-full bg-white/5 border border-white/10 rounded-lg py-2 px-3 text-white font-mono focus:border-rtp-primary focus:outline-none"
							autocomplete="off"
						/>
					</div>
					<div>
						<label
							for="sim-rr"
							class="text-xs uppercase tracking-widest text-slate-500 font-bold mb-2 block"
							>R:R Ratio</label
						>
						<input
							id="sim-rr"
							name="sim-rr"
							type="number"
							step="0.1"
							bind:value={simRR}
							class="w-full bg-white/5 border border-white/10 rounded-lg py-2 px-3 text-white font-mono focus:border-rtp-primary focus:outline-none"
							autocomplete="off"
						/>
					</div>
				</div>
			</div>

			<div class="md:col-span-8 flex flex-col justify-center border-l border-white/5 md:pl-12">
				<div class="grid grid-cols-2 gap-6 mb-8">
					<div class="p-4 bg-white/2 rounded-lg border border-white/5">
						<div class="text-xs text-slate-500 uppercase mb-1">Dollar Risk (1R)</div>
						<div class="text-2xl text-white font-mono font-bold">${riskAmount}</div>
					</div>
					<div class="p-4 bg-white/2 rounded-lg border border-white/5">
						<div class="text-xs text-slate-500 uppercase mb-1">Potential Reward</div>
						<div class="text-2xl text-emerald-400 font-mono font-bold">${winAmount}</div>
					</div>
					<div class="p-4 bg-white/2 rounded-lg border border-white/5">
						<div class="text-xs text-slate-500 uppercase mb-1">Trades to Double</div>
						<div class="text-2xl text-rtp-primary font-mono font-bold">
							{tradesToDouble > 0 ? tradesToDouble : '∞'}
						</div>
					</div>
					<div class="p-4 bg-white/2 rounded-lg border border-white/5">
						<div class="text-xs text-slate-500 uppercase mb-1">Expectancy (per trade)</div>
						<div
							class="text-2xl font-mono font-bold {expectedValue > 0
								? 'text-white'
								: 'text-red-500'}"
						>
							${expectedValue.toFixed(2)}
						</div>
					</div>
				</div>

				<div class="bg-black/40 rounded-xl p-6 border border-white/10 relative overflow-hidden">
					<div
						class="absolute top-2 right-4 text-[10px] font-mono text-slate-600 uppercase tracking-widest"
					>
						Analysis Output
					</div>
					<div class="space-y-4">
						<div class="flex justify-between items-center border-b border-white/5 pb-2">
							<span class="text-slate-400 text-sm">Risk Classification</span>
							<span class={['font-bold', 'font-mono', riskColor]}>{riskOfRuin}</span>
						</div>
						<div class="flex justify-between items-center border-b border-white/5 pb-2">
							<span class="text-slate-400 text-sm">Institutional Viability</span>
							<span
								class={[
									'font-bold',
									'font-mono',
									simRisk <= 2 ? 'text-emerald-400' : 'text-red-500'
								]}
							>
								{simRisk <= 2 ? 'PASS' : 'FAIL'}
							</span>
						</div>
						<p class="text-xs text-slate-500 leading-relaxed mt-2">
							{#if simRisk > 2}
								<span class="text-red-400 font-bold">WARNING:</span> Your risk per trade is too high
								for institutional standards. A standard drawdown sequence (4-5 losses) will cause
								significant emotional damage (-{simRisk * 5}% equity).
							{:else}
								<span class="text-emerald-400 font-bold">OPTIMAL:</span> Your sizing allows you to
								weather statistical variance. You can survive a 10-trade losing streak with only {simRisk *
									10}% drawdown.
							{/if}
						</p>
					</div>
				</div>
			</div>
		</div>
	</div>
</section>

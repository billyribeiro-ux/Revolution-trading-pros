<script lang="ts">
	import { X, TrendingUp, TrendingDown } from '@lucide/svelte';
	import gsap from 'gsap';
	import { price as bsPrice } from '../engine/black-scholes.js';
	import type { MarketDataService } from '../data/market-data-service.svelte.js';
	import type { CalculatorState } from '../state/calculator.svelte.js';
	import type { MispricingResult } from '../data/types.js';
	import type { BSInputs } from '../engine/types.js';

	interface Props {
		marketData: MarketDataService;
		calc: CalculatorState;
	}

	let { marketData, calc }: Props = $props();

	let dismissed = $state<Set<string>>(new Set());
	let alertEl: HTMLDivElement | undefined = $state();

	let mispricings = $derived.by<MispricingResult[]>(() => {
		const chain = marketData.currentChain;
		if (!chain) return [];

		const results: MispricingResult[] = [];
		const allOptions = [...chain.calls, ...chain.puts];

		for (const option of allOptions) {
			if (option.mid <= 0.01 || dismissed.has(option.symbol)) continue;

			const today = new Date();
			const expiryDate = new Date(option.expiration);
			const daysToExpiry = Math.max(1, Math.round((expiryDate.getTime() - today.getTime()) / 86_400_000));

			const inputs: BSInputs = {
				spotPrice: chain.underlyingPrice,
				strikePrice: option.strike,
				volatility: option.impliedVolatility > 0 ? option.impliedVolatility : calc.volatility,
				timeToExpiry: daysToExpiry / 365,
				riskFreeRate: calc.riskFreeRate,
				dividendYield: calc.dividendYield,
			};

			const pricing = bsPrice(inputs);
			const theoretical = option.type === 'call' ? pricing.callPrice : pricing.putPrice;
			if (theoretical <= 0.01) continue;

			const diff = option.mid - theoretical;
			const diffPct = (diff / theoretical) * 100;

			if (Math.abs(diffPct) >= 5) {
				results.push({
					option,
					theoreticalPrice: theoretical,
					marketPrice: option.mid,
					priceDifference: diff,
					priceDifferencePct: diffPct,
					direction: diff > 0 ? 'overpriced' : 'underpriced',
					significance: Math.abs(diffPct) > 15 ? 'significant' : 'moderate',
				});
			}
		}

		return results.sort((a, b) => Math.abs(b.priceDifferencePct) - Math.abs(a.priceDifferencePct)).slice(0, 5);
	});

	function dismiss(symbol: string) {
		dismissed = new Set([...dismissed, symbol]);
	}

	$effect(() => {
		if (alertEl && mispricings.length > 0) {
			const cards = alertEl.querySelectorAll('.mispricing-card');
			gsap.fromTo(cards, { x: 30, opacity: 0 }, { x: 0, opacity: 1, duration: 0.3, stagger: 0.06, ease: 'power2.out' });
		}
	});
</script>

{#if mispricings.length > 0}
	<div bind:this={alertEl} class="flex flex-col gap-1.5">
		<span class="text-[10px] font-semibold uppercase tracking-wider" style="color: var(--calc-warning);">
			Mispricing Detected ({mispricings.length})
		</span>
		{#each mispricings as mp (mp.option.symbol)}
			<div
				class="mispricing-card flex items-center gap-2 rounded-lg px-2.5 py-1.5"
				style="background: {mp.direction === 'underpriced' ? 'rgba(0,212,170,0.08)' : 'rgba(255,68,119,0.08)'}; border: 1px solid {mp.direction === 'underpriced' ? 'rgba(0,212,170,0.2)' : 'rgba(255,68,119,0.2)'};"
			>
				{#if mp.direction === 'underpriced'}
					<TrendingUp size={12} style="color: var(--calc-call);" />
				{:else}
					<TrendingDown size={12} style="color: var(--calc-put);" />
				{/if}

				<div class="flex-1 flex items-center gap-2 min-w-0">
					<span class="text-[10px] font-bold truncate" style="color: var(--calc-text); font-family: var(--calc-font-mono);">
						{mp.option.type === 'call' ? 'C' : 'P'}{mp.option.strike}
					</span>
					<span class="text-[9px]" style="color: var(--calc-text-muted);">
						Mkt: ${mp.marketPrice.toFixed(2)} · Theo: ${mp.theoreticalPrice.toFixed(2)}
					</span>
					<span
						class="text-[10px] font-bold"
						style="color: {mp.direction === 'underpriced' ? 'var(--calc-call)' : 'var(--calc-put)'}; font-family: var(--calc-font-mono);"
					>
						{mp.priceDifferencePct > 0 ? '+' : ''}{mp.priceDifferencePct.toFixed(1)}%
					</span>
					{#if mp.significance === 'significant'}
						<span class="text-[8px] px-1 py-0.5 rounded-full font-bold" style="background: var(--calc-warning); color: white;">!</span>
					{/if}
				</div>

				<button
					onclick={() => dismiss(mp.option.symbol)}
					class="cursor-pointer flex-shrink-0"
					style="color: var(--calc-text-muted);"
					aria-label="Dismiss"
				>
					<X size={10} />
				</button>
			</div>
		{/each}
	</div>
{/if}

<script lang="ts">
	import IconX from '@tabler/icons-svelte-runes/icons/x';
	import IconTrendingUp from '@tabler/icons-svelte-runes/icons/trending-up';
	import IconTrendingDown from '@tabler/icons-svelte-runes/icons/trending-down';
	import gsap from 'gsap';
	import { SvelteSet } from 'svelte/reactivity';
	import type { Attachment } from 'svelte/attachments';
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

	let dismissed = new SvelteSet<string>();

	let mispricings = $derived.by<MispricingResult[]>(() => {
		const chain = marketData.currentChain;
		if (!chain) return [];

		const results: MispricingResult[] = [];
		const allOptions = [...chain.calls, ...chain.puts];

		for (const option of allOptions) {
			if (option.mid <= 0.01 || dismissed.has(option.symbol)) continue;

			const today = new Date();
			const expiryDate = new Date(option.expiration);
			const daysToExpiry = Math.max(
				1,
				Math.round((expiryDate.getTime() - today.getTime()) / 86_400_000)
			);

			const inputs: BSInputs = {
				spotPrice: chain.underlyingPrice,
				strikePrice: option.strike,
				volatility: option.impliedVolatility > 0 ? option.impliedVolatility : calc.volatility,
				timeToExpiry: daysToExpiry / 365,
				riskFreeRate: calc.riskFreeRate,
				dividendYield: calc.dividendYield
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
					significance: Math.abs(diffPct) > 15 ? 'significant' : 'moderate'
				});
			}
		}

		return results
			.sort((a, b) => Math.abs(b.priceDifferencePct) - Math.abs(a.priceDifferencePct))
			.slice(0, 5);
	});

	function dismiss(symbol: string) {
		dismissed.add(symbol);
	}

	function mispricingCardClass(direction: MispricingResult['direction']) {
		return ['mispricing-card', `mispricing-card--${direction}`];
	}

	function differenceClass(direction: MispricingResult['direction']) {
		return ['difference-value', `difference-value--${direction}`];
	}

	function animateAlerts(count: number): Attachment<HTMLDivElement> {
		return (node) => {
			if (count === 0) return;

			const cards = node.querySelectorAll('.mispricing-card');
			gsap.fromTo(
				cards,
				{ x: 30, opacity: 0 },
				{ x: 0, opacity: 1, duration: 0.3, stagger: 0.06, ease: 'power2.out' }
			);
		};
	}
</script>

{#if mispricings.length > 0}
	<div {@attach animateAlerts(mispricings.length)} class="mispricing-list">
		<span class="mispricing-heading">
			Mispricing Detected ({mispricings.length})
		</span>
		{#each mispricings as mp (mp.option.symbol)}
			<div class={mispricingCardClass(mp.direction)}>
				{#if mp.direction === 'underpriced'}
					<IconTrendingUp size={12} style="color: var(--calc-call);" />
				{:else}
					<IconTrendingDown size={12} style="color: var(--calc-put);" />
				{/if}

				<div class="mispricing-content">
					<span class="option-symbol">
						{mp.option.type === 'call' ? 'C' : 'P'}{mp.option.strike}
					</span>
					<span class="price-comparison">
						Mkt: ${mp.marketPrice.toFixed(2)} · Theo: ${mp.theoreticalPrice.toFixed(2)}
					</span>
					<span class={differenceClass(mp.direction)}>
						{mp.priceDifferencePct > 0 ? '+' : ''}{mp.priceDifferencePct.toFixed(1)}%
					</span>
					{#if mp.significance === 'significant'}
						<span class="significance-badge">!</span>
					{/if}
				</div>

				<button
					onclick={() => dismiss(mp.option.symbol)}
					class="dismiss-button"
					aria-label="Dismiss"
				>
					<IconX size={10} />
				</button>
			</div>
		{/each}
	</div>
{/if}

<style>
	.mispricing-list {
		display: flex;
		flex-direction: column;
		gap: 0.375rem;
	}

	.mispricing-heading {
		color: var(--calc-warning);
		font-size: 0.625rem;
		font-weight: 600;
		letter-spacing: 0.05em;
		line-height: 1.2;
		text-transform: uppercase;
	}

	.mispricing-card {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		border: 1px solid transparent;
		border-radius: 0.5rem;
		padding: 0.375rem 0.625rem;
	}

	.mispricing-card--underpriced {
		border-color: rgb(0 212 170 / 0.2);
		background: rgb(0 212 170 / 0.08);
	}

	.mispricing-card--overpriced {
		border-color: rgb(255 68 119 / 0.2);
		background: rgb(255 68 119 / 0.08);
	}

	.mispricing-content {
		display: flex;
		flex: 1;
		align-items: center;
		gap: 0.5rem;
		min-width: 0;
	}

	.option-symbol,
	.difference-value {
		font-family: var(--calc-font-mono);
		font-size: 0.625rem;
		font-weight: 700;
		line-height: 1.2;
	}

	.option-symbol {
		overflow: hidden;
		color: var(--calc-text);
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.price-comparison {
		color: var(--calc-text-muted);
		font-size: 0.5625rem;
		line-height: 1.2;
	}

	.difference-value--underpriced {
		color: var(--calc-call);
	}

	.difference-value--overpriced {
		color: var(--calc-put);
	}

	.significance-badge {
		border-radius: 999px;
		background: var(--calc-warning);
		color: white;
		font-size: 0.5rem;
		font-weight: 700;
		line-height: 1.2;
		padding: 0.125rem 0.25rem;
	}

	.dismiss-button {
		flex-shrink: 0;
		border: 0;
		background: transparent;
		color: var(--calc-text-muted);
		cursor: pointer;
		font: inherit;
		line-height: 0;
		padding: 0;
	}

	.dismiss-button:hover {
		color: var(--calc-text);
	}
</style>

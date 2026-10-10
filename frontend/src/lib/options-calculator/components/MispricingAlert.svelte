<script lang="ts">
	import IconX from '@tabler/icons-svelte-runes/icons/x';
	import IconTrendingUp from '@tabler/icons-svelte-runes/icons/trending-up';
	import IconTrendingDown from '@tabler/icons-svelte-runes/icons/trending-down';
	import { cubicOut } from 'svelte/easing';
	import { prefersReducedMotion } from 'svelte/motion';
	import { SvelteSet } from 'svelte/reactivity';
	import { fly, type FlyParams } from 'svelte/transition';
	import { SIGNIFICANT_MISPRICING_PCT, detectMispricing } from '../engine/mispricing.js';
	import { getCalculator, getMarketData } from '../state/context.js';
	import type { MispricingResult } from '../data/types.js';

	const calc = getCalculator();
	const marketData = getMarketData();

	const MAX_ALERTS = 5;

	/** Contracts the user dismissed; reactive, so dismissing recomputes the list. */
	const dismissed = new SvelteSet<string>();

	const mispricings = $derived.by((): MispricingResult[] => {
		const chain = marketData.currentChain;
		if (!chain) return [];

		const now = Date.now();
		const market = {
			spotPrice: chain.underlyingPrice,
			volatility: calc.volatility,
			riskFreeRate: calc.riskFreeRate,
			dividendYield: calc.dividendYield
		};

		return [...chain.calls, ...chain.puts]
			.filter((option) => !dismissed.has(option.symbol))
			.flatMap((option) => detectMispricing(option, market, now) ?? [])
			.sort((a, b) => Math.abs(b.priceDifferencePct) - Math.abs(a.priceDifferencePct))
			.slice(0, MAX_ALERTS);
	});

	/** Staggered slide-in for each alert as it appears; instant under reduced motion. */
	function alertIntro(index: number): FlyParams {
		if (prefersReducedMotion.current) return { duration: 0 };
		return { x: 30, duration: 300, delay: index * 60, easing: cubicOut };
	}
</script>

{#if mispricings.length > 0}
	<div class="mispricing-list">
		<span class="mispricing-heading">
			Mispricing Detected ({mispricings.length})
		</span>
		{#each mispricings as mp, i (mp.option.symbol)}
			{const contract = $derived(`${mp.option.type === 'call' ? 'C' : 'P'}${mp.option.strike}`)}
			<div
				in:fly|global={alertIntro(i)}
				class={['mispricing-card', `mispricing-card--${mp.direction}`]}
			>
				{#if mp.direction === 'underpriced'}
					<IconTrendingUp size={12} style="color: var(--calc-call);" aria-hidden="true" />
				{:else}
					<IconTrendingDown size={12} style="color: var(--calc-put);" aria-hidden="true" />
				{/if}

				<div class="mispricing-content">
					<span class="option-symbol">{contract}</span>
					<span class="price-comparison">
						Mkt: ${mp.marketPrice.toFixed(2)} · Theo: ${mp.theoreticalPrice.toFixed(2)}
					</span>
					<span class={['difference-value', `difference-value--${mp.direction}`]}>
						{mp.priceDifferencePct > 0 ? '+' : ''}{mp.priceDifferencePct.toFixed(1)}%
					</span>
					{#if mp.significance === 'significant'}
						<span
							class="significance-badge"
							title="Significant (over {SIGNIFICANT_MISPRICING_PCT}%)">!</span
						>
					{/if}
				</div>

				<button
					type="button"
					onclick={() => dismissed.add(mp.option.symbol)}
					class="dismiss-button"
					aria-label="Dismiss {contract} alert"
				>
					<IconX size={10} aria-hidden="true" />
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

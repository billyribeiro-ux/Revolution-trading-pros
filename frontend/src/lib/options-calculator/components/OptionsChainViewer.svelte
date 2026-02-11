<script lang="ts">
	import { Loader2 } from '@lucide/svelte';
	import gsap from 'gsap';
	import { price as bsPrice } from '../engine/black-scholes.js';
	import type { MarketDataService } from '../data/market-data-service.svelte.js';
	import type { CalculatorState } from '../state/calculator.svelte.js';
	import type { OptionQuote, MispricingResult } from '../data/types.js';
	import type { BSInputs } from '../engine/types.js';

	interface Props {
		marketData: MarketDataService;
		calc: CalculatorState;
		onSelectContract?: (option: OptionQuote) => void;
	}

	let { marketData, calc, onSelectContract }: Props = $props();

	let selectedExpiration = $state('');
	let isLoadingChain = $state(false);
	let tableEl: HTMLDivElement | undefined = $state();

	let chain = $derived(marketData.currentChain);
	let expirations = $derived(marketData.currentExpirations?.expirations ?? []);
	let ticker = $derived(marketData.activeTicker);

	$effect(() => {
		if (expirations.length > 0 && !selectedExpiration) {
			selectedExpiration = expirations[0];
		}
	});

	$effect(() => {
		if (tableEl && chain) {
			const rows = tableEl.querySelectorAll('tr');
			gsap.fromTo(
				rows,
				{ opacity: 0, y: 4 },
				{ opacity: 1, y: 0, duration: 0.2, stagger: 0.008, ease: 'power2.out' }
			);
		}
	});

	async function loadChain(expiration: string) {
		if (!ticker || !expiration) return;
		selectedExpiration = expiration;
		isLoadingChain = true;
		try {
			await marketData.fetchOptionsChain(ticker, expiration);
		} finally {
			isLoadingChain = false;
		}
	}

	function computeTheoreticalPrice(option: OptionQuote): number {
		const today = new Date();
		const expiryDate = new Date(option.expiration);
		const daysToExpiry = Math.max(
			1,
			Math.round((expiryDate.getTime() - today.getTime()) / 86_400_000)
		);
		const inputs: BSInputs = {
			spotPrice: chain?.underlyingPrice ?? calc.spotPrice,
			strikePrice: option.strike,
			volatility: option.impliedVolatility > 0 ? option.impliedVolatility : calc.volatility,
			timeToExpiry: daysToExpiry / 365,
			riskFreeRate: calc.riskFreeRate,
			dividendYield: calc.dividendYield
		};
		const pricing = bsPrice(inputs);
		return option.type === 'call' ? pricing.callPrice : pricing.putPrice;
	}

	function detectMispricing(option: OptionQuote): MispricingResult | null {
		if (option.mid <= 0.01) return null;
		const theoretical = computeTheoreticalPrice(option);
		if (theoretical <= 0.01) return null;
		const diff = option.mid - theoretical;
		const diffPct = (diff / theoretical) * 100;
		if (Math.abs(diffPct) < 5) return null;
		return {
			option,
			theoreticalPrice: theoretical,
			marketPrice: option.mid,
			priceDifference: diff,
			priceDifferencePct: diffPct,
			direction: diff > 0 ? 'overpriced' : 'underpriced',
			significance: Math.abs(diffPct) > 15 ? 'significant' : 'moderate'
		};
	}

	function handleSelectContract(option: OptionQuote) {
		if (onSelectContract) onSelectContract(option);
	}

	let strikes = $derived.by(() => {
		if (!chain) return [];
		const strikeSet = new Set<number>();
		for (const c of chain.calls) strikeSet.add(c.strike);
		for (const p of chain.puts) strikeSet.add(p.strike);
		return [...strikeSet].sort((a, b) => a - b);
	});

	let callsByStrike = $derived.by(() => {
		const map = new Map<number, OptionQuote>();
		if (chain) for (const c of chain.calls) map.set(c.strike, c);
		return map;
	});

	let putsByStrike = $derived.by(() => {
		const map = new Map<number, OptionQuote>();
		if (chain) for (const p of chain.puts) map.set(p.strike, p);
		return map;
	});

	let atmStrike = $derived.by(() => {
		if (!chain || strikes.length === 0) return 0;
		const spot = chain.underlyingPrice;
		return strikes.reduce((closest, s) =>
			Math.abs(s - spot) < Math.abs(closest - spot) ? s : closest
		);
	});

	function mispricingColor(option: OptionQuote): string {
		const mp = detectMispricing(option);
		if (!mp) return '';
		if (mp.direction === 'underpriced') return 'rgba(0,212,170,0.08)';
		return 'rgba(255,68,119,0.08)';
	}

	function mispricingBadge(option: OptionQuote): string {
		const mp = detectMispricing(option);
		if (!mp) return '';
		const sign = mp.direction === 'underpriced' ? '' : '+';
		return `${sign}${mp.priceDifferencePct.toFixed(1)}%`;
	}
</script>

<div class="flex flex-col gap-3">
	<!-- Header -->
	<div class="flex items-center justify-between flex-wrap gap-2">
		<div class="flex items-center gap-2">
			<h3
				class="text-sm font-semibold"
				style="color: var(--calc-text); font-family: var(--calc-font-display);"
			>
				Options Chain
			</h3>
			{#if ticker}
				<span
					class="text-xs font-bold px-1.5 py-0.5 rounded"
					style="background: var(--calc-accent-glow); color: var(--calc-accent); font-family: var(--calc-font-mono);"
				>
					{ticker}
				</span>
			{/if}
			{#if chain}
				<span class="text-[10px]" style="color: var(--calc-text-muted);">
					{chain.calls.length + chain.puts.length} contracts
				</span>
			{/if}
		</div>

		{#if expirations.length > 0}
			<div class="flex items-center gap-1.5 overflow-x-auto">
				{#each expirations.slice(0, 8) as exp (exp)}
					<button
						onclick={() => loadChain(exp)}
						class="text-[10px] px-2 py-1 rounded-lg whitespace-nowrap transition-colors cursor-pointer"
						style={selectedExpiration === exp
							? 'background: var(--calc-accent-glow); color: var(--calc-accent); border: 1px solid var(--calc-accent);'
							: 'background: var(--calc-surface); color: var(--calc-text-muted); border: 1px solid var(--calc-border);'}
					>
						{new Date(exp + 'T12:00:00').toLocaleDateString('en-US', {
							month: 'short',
							day: 'numeric'
						})}
					</button>
				{/each}
			</div>
		{/if}
	</div>

	<!-- States -->
	{#if !ticker}
		<div class="flex items-center justify-center py-12" style="color: var(--calc-text-muted);">
			<p class="text-sm">Search for a ticker to view its options chain</p>
		</div>
	{:else if isLoadingChain}
		<div class="flex items-center justify-center py-12">
			<Loader2 size={20} class="animate-spin" style="color: var(--calc-accent);" />
			<span class="ml-2 text-xs" style="color: var(--calc-text-muted);">Loading chain...</span>
		</div>
	{:else if chain && strikes.length > 0}
		<!-- Chain Table -->
		<div
			bind:this={tableEl}
			class="overflow-x-auto rounded-xl"
			style="border: 1px solid var(--calc-border);"
		>
			<table class="w-full text-[10px]" style="font-family: var(--calc-font-mono);">
				<thead>
					<tr style="background: var(--calc-surface);">
						<th
							colspan="6"
							class="text-center py-1.5 font-semibold border-b border-r"
							style="color: var(--calc-call); border-color: var(--calc-border);">CALLS</th
						>
						<th
							class="text-center py-1.5 font-semibold border-b px-3"
							style="color: var(--calc-text);">STRIKE</th
						>
						<th
							colspan="6"
							class="text-center py-1.5 font-semibold border-b border-l"
							style="color: var(--calc-put); border-color: var(--calc-border);">PUTS</th
						>
					</tr>
					<tr style="background: var(--calc-surface);">
						<th class="text-right px-2 py-1 font-medium" style="color: var(--calc-text-muted);"
							>Bid</th
						>
						<th class="text-right px-2 py-1 font-medium" style="color: var(--calc-text-muted);"
							>Ask</th
						>
						<th class="text-right px-2 py-1 font-medium" style="color: var(--calc-text-muted);"
							>Last</th
						>
						<th class="text-right px-2 py-1 font-medium" style="color: var(--calc-text-muted);"
							>Vol</th
						>
						<th class="text-right px-2 py-1 font-medium" style="color: var(--calc-text-muted);"
							>OI</th
						>
						<th
							class="text-right px-2 py-1 font-medium border-r"
							style="color: var(--calc-text-muted); border-color: var(--calc-border);">IV</th
						>
						<th class="text-center px-3 py-1 font-bold" style="color: var(--calc-text);">K</th>
						<th
							class="text-right px-2 py-1 font-medium border-l"
							style="color: var(--calc-text-muted); border-color: var(--calc-border);">Bid</th
						>
						<th class="text-right px-2 py-1 font-medium" style="color: var(--calc-text-muted);"
							>Ask</th
						>
						<th class="text-right px-2 py-1 font-medium" style="color: var(--calc-text-muted);"
							>Last</th
						>
						<th class="text-right px-2 py-1 font-medium" style="color: var(--calc-text-muted);"
							>Vol</th
						>
						<th class="text-right px-2 py-1 font-medium" style="color: var(--calc-text-muted);"
							>OI</th
						>
						<th class="text-right px-2 py-1 font-medium" style="color: var(--calc-text-muted);"
							>IV</th
						>
					</tr>
				</thead>
				<tbody>
					{#each strikes as strike (strike)}
						{@const call = callsByStrike.get(strike)}
						{@const put = putsByStrike.get(strike)}
						{@const isATM = strike === atmStrike}
						{@const callBg = call ? mispricingColor(call) : ''}
						{@const putBg = put ? mispricingColor(put) : ''}
						<tr
							class="transition-colors"
							style="border-top: 1px solid var(--calc-border); {isATM
								? 'background: var(--calc-accent-glow);'
								: ''}"
						>
							<!-- Call side -->
							{#if call}
								<td
									class="text-right px-2 py-1 cursor-pointer"
									style="background: {callBg}; {call.inTheMoney
										? 'color: var(--calc-call);'
										: 'color: var(--calc-text-secondary);'}"
									onclick={() => handleSelectContract(call)}>{call.bid.toFixed(2)}</td
								>
								<td
									class="text-right px-2 py-1 cursor-pointer"
									style="background: {callBg}; {call.inTheMoney
										? 'color: var(--calc-call);'
										: 'color: var(--calc-text-secondary);'}"
									onclick={() => handleSelectContract(call)}>{call.ask.toFixed(2)}</td
								>
								<td
									class="text-right px-2 py-1 cursor-pointer"
									style="background: {callBg}; color: var(--calc-text);"
									onclick={() => handleSelectContract(call)}>{call.lastPrice.toFixed(2)}</td
								>
								<td class="text-right px-2 py-1" style="color: var(--calc-text-muted);"
									>{call.volume.toLocaleString()}</td
								>
								<td class="text-right px-2 py-1" style="color: var(--calc-text-muted);"
									>{call.openInterest.toLocaleString()}</td
								>
								<td
									class="text-right px-2 py-1 border-r"
									style="color: var(--calc-accent); border-color: var(--calc-border);"
								>
									{(call.impliedVolatility * 100).toFixed(1)}%
									{#if mispricingBadge(call)}
										<span
											class="ml-0.5 text-[8px]"
											style="color: {detectMispricing(call)?.direction === 'underpriced'
												? 'var(--calc-call)'
												: 'var(--calc-put)'};"
										>
											{mispricingBadge(call)}
										</span>
									{/if}
								</td>
							{:else}
								<td colspan="6" class="border-r" style="border-color: var(--calc-border);"></td>
							{/if}

							<!-- Strike -->
							<td
								class="text-center px-3 py-1 font-bold"
								style="color: {isATM ? 'var(--calc-accent)' : 'var(--calc-text)'};"
							>
								{strike.toFixed(strike % 1 === 0 ? 0 : 2)}
							</td>

							<!-- Put side -->
							{#if put}
								<td
									class="text-right px-2 py-1 border-l cursor-pointer"
									style="background: {putBg}; border-color: var(--calc-border); {put.inTheMoney
										? 'color: var(--calc-put);'
										: 'color: var(--calc-text-secondary);'}"
									onclick={() => handleSelectContract(put)}>{put.bid.toFixed(2)}</td
								>
								<td
									class="text-right px-2 py-1 cursor-pointer"
									style="background: {putBg}; {put.inTheMoney
										? 'color: var(--calc-put);'
										: 'color: var(--calc-text-secondary);'}"
									onclick={() => handleSelectContract(put)}>{put.ask.toFixed(2)}</td
								>
								<td
									class="text-right px-2 py-1 cursor-pointer"
									style="background: {putBg}; color: var(--calc-text);"
									onclick={() => handleSelectContract(put)}>{put.lastPrice.toFixed(2)}</td
								>
								<td class="text-right px-2 py-1" style="color: var(--calc-text-muted);"
									>{put.volume.toLocaleString()}</td
								>
								<td class="text-right px-2 py-1" style="color: var(--calc-text-muted);"
									>{put.openInterest.toLocaleString()}</td
								>
								<td class="text-right px-2 py-1" style="color: var(--calc-accent);">
									{(put.impliedVolatility * 100).toFixed(1)}%
									{#if mispricingBadge(put)}
										<span
											class="ml-0.5 text-[8px]"
											style="color: {detectMispricing(put)?.direction === 'underpriced'
												? 'var(--calc-call)'
												: 'var(--calc-put)'};"
										>
											{mispricingBadge(put)}
										</span>
									{/if}
								</td>
							{:else}
								<td colspan="6" class="border-l" style="border-color: var(--calc-border);"></td>
							{/if}
						</tr>
					{/each}
				</tbody>
			</table>
		</div>

		<!-- Legend -->
		<div class="flex items-center gap-4 text-[9px]" style="color: var(--calc-text-muted);">
			<div class="flex items-center gap-1">
				<div class="w-2.5 h-2.5 rounded-sm" style="background: rgba(0,212,170,0.15);"></div>
				Underpriced (&gt;5%)
			</div>
			<div class="flex items-center gap-1">
				<div class="w-2.5 h-2.5 rounded-sm" style="background: rgba(255,68,119,0.15);"></div>
				Overpriced (&gt;5%)
			</div>
			<div class="flex items-center gap-1">
				<div class="w-2.5 h-2.5 rounded-sm" style="background: var(--calc-accent-glow);"></div>
				ATM Strike
			</div>
			<span>Click any row to populate calculator</span>
		</div>
	{/if}
</div>

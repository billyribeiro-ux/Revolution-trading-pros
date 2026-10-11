<script lang="ts">
	import IconLoader2 from '@tabler/icons-svelte-runes/icons/loader-2';
	import { cubicOut } from 'svelte/easing';
	import { prefersReducedMotion } from 'svelte/motion';
	import { fly, type FlyParams } from 'svelte/transition';
	import { logger } from '#lib/utils/logger.js';
	import {
		MISPRICING_THRESHOLD_PCT,
		daysUntilExpiration,
		detectMispricing
	} from '../engine/mispricing.js';
	import { getCalculator, getMarketData } from '../state/context.js';
	import type { OptionQuote, MispricingResult } from '../data/types.js';

	interface Props {
		/**
		 * Called when a bid / ask / last cell is chosen. Without it, the contract is
		 * loaded into the calculator (the behaviour the legend promises).
		 */
		onSelectContract?: (option: OptionQuote) => void;
	}

	let { onSelectContract }: Props = $props();

	const calc = getCalculator();
	const marketData = getMarketData();

	interface ChainSide {
		quote: OptionQuote;
		mispricing: MispricingResult | null;
	}

	interface ChainRow {
		strike: number;
		call?: ChainSide;
		put?: ChainSide;
	}

	/** Expiration chips shown in the header. */
	const MAX_EXPIRATIONS = 8;
	const QUOTE_COLUMNS = ['Bid', 'Ask', 'Last', 'Vol', 'OI', 'IV'];
	const MISPRICING_BACKGROUND: Record<MispricingResult['direction'], string> = {
		underpriced: 'rgba(0,212,170,0.08)',
		overpriced: 'rgba(255,68,119,0.08)'
	};

	const expiryFormat = new Intl.DateTimeFormat('en-US', { month: 'short', day: 'numeric' });

	let isLoadingChain = $state(false);
	/** Id of the newest `loadChain` call, so a superseded load cannot hide the spinner early. */
	let latestLoad = 0;

	const chain = $derived(marketData.currentChain);
	const expirations = $derived(marketData.currentExpirations?.expirations ?? []);
	const visibleExpirations = $derived(expirations.slice(0, MAX_EXPIRATIONS));
	const ticker = $derived(marketData.activeTicker);
	// Derived from the chain on screen (the source of truth) and overridden optimistically
	// while a newly picked expiration loads, so the highlighted chip always matches the
	// table — including after a remount or a failed load.
	let selectedExpiration = $derived(chain?.expiration ?? '');

	function formatExpiry(expiration: string): string {
		return expiryFormat.format(new Date(`${expiration}T12:00:00`));
	}

	function formatStrike(strike: number): string {
		return strike.toFixed(strike % 1 === 0 ? 0 : 2);
	}

	/** One row per strike, calls and puts side by side, each priced once against Black-Scholes. */
	const rows = $derived.by((): ChainRow[] => {
		if (!chain) return [];
		const now = Date.now();
		const market = {
			spotPrice: chain.underlyingPrice,
			volatility: calc.volatility,
			riskFreeRate: calc.riskFreeRate,
			dividendYield: calc.dividendYield
		};
		const side = (quote: OptionQuote | undefined): ChainSide | undefined =>
			quote && { quote, mispricing: detectMispricing(quote, market, now) };
		const calls = new Map<number, OptionQuote>(chain.calls.map((quote) => [quote.strike, quote]));
		const puts = new Map<number, OptionQuote>(chain.puts.map((quote) => [quote.strike, quote]));
		const strikes = [...new Set([...calls.keys(), ...puts.keys()])].sort((a, b) => a - b);
		return strikes.map((strike) => ({
			strike,
			call: side(calls.get(strike)),
			put: side(puts.get(strike))
		}));
	});

	const atmStrike = $derived.by(() => {
		if (!chain || rows.length === 0) return 0;
		const spot = chain.underlyingPrice;
		return rows.reduce(
			(closest, { strike }) =>
				Math.abs(strike - spot) < Math.abs(closest - spot) ? strike : closest,
			rows[0].strike
		);
	});

	function isAbortError(error: unknown): boolean {
		return error instanceof Error && error.name === 'AbortError';
	}

	async function loadChain(expiration: string) {
		if (!ticker || !expiration) return;
		const previous = selectedExpiration;
		selectedExpiration = expiration;
		const load = ++latestLoad;
		isLoadingChain = true;
		try {
			await marketData.fetchOptionsChain(ticker, expiration);
		} catch (error) {
			// A newer chain (or ticker) request aborts this one: that is not a failure.
			if (!isAbortError(error)) {
				logger.error('[OptionsChain] Failed to load chain:', error);
				if (load === latestLoad) selectedExpiration = previous;
			}
		} finally {
			if (load === latestLoad) isLoadingChain = false;
		}
	}

	function loadIntoCalculator(option: OptionQuote) {
		calc.optionType = option.type;
		calc.strikePrice = option.strike;
		calc.timeToExpiry = daysUntilExpiration(option.expiration, Date.now()) / 365;
		if (option.impliedVolatility > 0) calc.volatility = option.impliedVolatility;
		if (chain) calc.spotPrice = chain.underlyingPrice;
		calc.addToast(
			'success',
			`Loaded the ${formatStrike(option.strike)} ${option.type} (${formatExpiry(option.expiration)}) into the calculator`
		);
	}

	function selectContract(option: OptionQuote) {
		if (onSelectContract) onSelectContract(option);
		else loadIntoCalculator(option);
	}

	function mispricingBadge(mispricing: MispricingResult): string {
		const sign = mispricing.direction === 'underpriced' ? '' : '+';
		return `${sign}${mispricing.priceDifferencePct.toFixed(1)}%`;
	}

	/** Staggered fade-up for the chain rows; instant under reduced motion. */
	function rowIntro(index: number): FlyParams {
		if (prefersReducedMotion.current) return { duration: 0 };
		return { y: 4, duration: 200, delay: index * 8, easing: cubicOut };
	}
</script>

{#snippet priceCell(
	quote: OptionQuote,
	field: string,
	value: number,
	color: string,
	background: string | undefined,
	borderLeft = false
)}
	<td
		class={['p-0', borderLeft && 'border-l']}
		style="border-color: var(--calc-border);"
		style:background
		style:color
	>
		<button
			type="button"
			class="quote-button"
			aria-label="{value.toFixed(2)} {field}, {formatStrike(quote.strike)} {quote.type}"
			onclick={() => selectContract(quote)}>{value.toFixed(2)}</button
		>
	</td>
{/snippet}

{#snippet chainSide(side: ChainSide | undefined, kind: 'call' | 'put')}
	{#if side}
		{const itmColor = $derived(kind === 'call' ? 'var(--calc-call)' : 'var(--calc-put)')}
		{const priceColor = $derived(side.quote.inTheMoney ? itmColor : 'var(--calc-text-secondary)')}
		{const background = $derived(
			side.mispricing ? MISPRICING_BACKGROUND[side.mispricing.direction] : undefined
		)}
		{@render priceCell(side.quote, 'bid', side.quote.bid, priceColor, background, kind === 'put')}
		{@render priceCell(side.quote, 'ask', side.quote.ask, priceColor, background)}
		{@render priceCell(side.quote, 'last', side.quote.lastPrice, 'var(--calc-text)', background)}
		<td class="text-right px-2 py-1" style="color: var(--calc-text-muted);"
			>{side.quote.volume.toLocaleString()}</td
		>
		<td class="text-right px-2 py-1" style="color: var(--calc-text-muted);"
			>{side.quote.openInterest.toLocaleString()}</td
		>
		<td
			class={['text-right px-2 py-1', kind === 'call' && 'border-r']}
			style="color: var(--calc-accent); border-color: var(--calc-border);"
		>
			{(side.quote.impliedVolatility * 100).toFixed(1)}%
			{#if side.mispricing}
				<span
					class="ml-0.5 text-[8px]"
					style:color={side.mispricing.direction === 'underpriced'
						? 'var(--calc-call)'
						: 'var(--calc-put)'}
				>
					{mispricingBadge(side.mispricing)}
				</span>
			{/if}
		</td>
	{:else}
		<td
			colspan="6"
			class={kind === 'call' ? 'border-r' : 'border-l'}
			style="border-color: var(--calc-border);"
		></td>
	{/if}
{/snippet}

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

		{#if visibleExpirations.length > 0}
			<div class="flex items-center gap-1.5 overflow-x-auto" role="group" aria-label="Expiration">
				{#each visibleExpirations as exp (exp)}
					{const selected = $derived(selectedExpiration === exp)}
					<button
						type="button"
						onclick={() => loadChain(exp)}
						aria-pressed={selected}
						class="text-[10px] px-2 py-1 rounded-lg whitespace-nowrap transition-colors motion-reduce:transition-none cursor-pointer"
						style:background={selected ? 'var(--calc-accent-glow)' : 'var(--calc-surface)'}
						style:color={selected ? 'var(--calc-accent)' : 'var(--calc-text-muted)'}
						style:border="1px solid {selected ? 'var(--calc-accent)' : 'var(--calc-border)'}"
					>
						{formatExpiry(exp)}
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
		<div class="flex items-center justify-center py-12" role="status">
			<IconLoader2
				size={20}
				class="animate-spin motion-reduce:animate-none"
				style="color: var(--calc-accent);"
				aria-hidden="true"
			/>
			<span class="ml-2 text-xs" style="color: var(--calc-text-muted);">Loading chain...</span>
		</div>
	{:else if chain && rows.length > 0}
		<!-- Chain Table -->
		<div class="overflow-x-auto rounded-xl" style="border: 1px solid var(--calc-border);">
			<table class="w-full text-[10px]" style="font-family: var(--calc-font-mono);">
				<caption class="sr-only">
					{ticker} options chain, expiring {formatExpiry(chain.expiration)}
				</caption>
				<thead>
					<tr style="background: var(--calc-surface);">
						<th
							scope="col"
							colspan="6"
							class="text-center py-1.5 font-semibold border-b border-r"
							style="color: var(--calc-call); border-color: var(--calc-border);">CALLS</th
						>
						<th
							scope="col"
							class="text-center py-1.5 font-semibold border-b px-3"
							style="color: var(--calc-text);">STRIKE</th
						>
						<th
							scope="col"
							colspan="6"
							class="text-center py-1.5 font-semibold border-b border-l"
							style="color: var(--calc-put); border-color: var(--calc-border);">PUTS</th
						>
					</tr>
					<tr style="background: var(--calc-surface);">
						{#each QUOTE_COLUMNS as column, i (column)}
							<th
								scope="col"
								class={[
									'text-right px-2 py-1 font-medium',
									i === QUOTE_COLUMNS.length - 1 && 'border-r'
								]}
								style="color: var(--calc-text-muted); border-color: var(--calc-border);"
								>{column}</th
							>
						{/each}
						<th scope="col" class="text-center px-3 py-1 font-bold" style="color: var(--calc-text);"
							>K</th
						>
						{#each QUOTE_COLUMNS as column, i (column)}
							<th
								scope="col"
								class={['text-right px-2 py-1 font-medium', i === 0 && 'border-l']}
								style="color: var(--calc-text-muted); border-color: var(--calc-border);"
								>{column}</th
							>
						{/each}
					</tr>
				</thead>
				<tbody>
					{#each rows as row, i (row.strike)}
						{const isATM = $derived(row.strike === atmStrike)}
						<tr
							in:fly|global={rowIntro(i)}
							class="transition-colors motion-reduce:transition-none"
							style="border-top: 1px solid var(--calc-border);"
							style:background={isATM ? 'var(--calc-accent-glow)' : undefined}
						>
							{@render chainSide(row.call, 'call')}

							<th
								scope="row"
								class="text-center px-3 py-1 font-bold"
								style:color={isATM ? 'var(--calc-accent)' : 'var(--calc-text)'}
							>
								{formatStrike(row.strike)}
							</th>

							{@render chainSide(row.put, 'put')}
						</tr>
					{/each}
				</tbody>
			</table>
		</div>

		<!-- Legend -->
		<div class="flex items-center gap-4 text-[9px]" style="color: var(--calc-text-muted);">
			<div class="flex items-center gap-1">
				<div
					class="w-2.5 h-2.5 rounded-sm"
					style="background: rgba(0,212,170,0.15);"
					aria-hidden="true"
				></div>
				Underpriced (&gt;{MISPRICING_THRESHOLD_PCT}%)
			</div>
			<div class="flex items-center gap-1">
				<div
					class="w-2.5 h-2.5 rounded-sm"
					style="background: rgba(255,68,119,0.15);"
					aria-hidden="true"
				></div>
				Overpriced (&gt;{MISPRICING_THRESHOLD_PCT}%)
			</div>
			<div class="flex items-center gap-1">
				<div
					class="w-2.5 h-2.5 rounded-sm"
					style="background: var(--calc-accent-glow);"
					aria-hidden="true"
				></div>
				ATM Strike
			</div>
			<span>Click any row to populate calculator</span>
		</div>
	{:else if visibleExpirations.length > 0}
		<div class="flex items-center justify-center py-12" style="color: var(--calc-text-muted);">
			<p class="text-sm">Pick an expiration to load its options chain</p>
		</div>
	{/if}
</div>

<style>
	/* Bid / ask / last cells: a full-cell button keeps the table semantics and gives
	   native keyboard activation (Enter / Space). */
	.quote-button {
		display: block;
		width: 100%;
		border: 0;
		background: transparent;
		color: inherit;
		cursor: pointer;
		font: inherit;
		padding: 0.25rem 0.5rem;
		text-align: right;
	}

	.quote-button:focus-visible {
		outline: 2px solid var(--calc-accent, #6366f1);
		outline-offset: -2px;
	}
</style>

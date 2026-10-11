<script lang="ts">
	import IconSearch from '@tabler/icons-svelte-runes/icons/search';
	import IconX from '@tabler/icons-svelte-runes/icons/x';
	import IconLoader2 from '@tabler/icons-svelte-runes/icons/loader-2';
	import { prefersReducedMotion } from 'svelte/motion';
	import { fly } from 'svelte/transition';
	import { formatCurrency } from '../utils/formatters.js';
	import { getMarketData } from '../state/context.js';
	import type { MarketSnapshot } from '../data/types.js';

	interface Props {
		onSnapshot?: (snapshot: MarketSnapshot) => void;
	}

	let { onSnapshot }: Props = $props();

	const marketData = getMarketData();

	const uid = $props.id();
	const listboxId = `${uid}-listbox`;
	const optionId = (index: number) => `${uid}-option-${index}`;

	/** Only used from event handlers, so it needs no reactivity. */
	let inputEl: HTMLInputElement | undefined;
	let query = $state('');
	let isOpen = $state(false);
	let selectedIndex = $state(-1);

	const results = $derived(marketData.searchResults);
	const quote = $derived(marketData.currentQuote);
	const showDropdown = $derived(isOpen && (results.length > 0 || marketData.isSearching));
	/** The listbox itself renders only once there are results (the spinner shows before). */
	const listboxShown = $derived(showDropdown && results.length > 0);

	function handleInput(event: Event & { currentTarget: HTMLInputElement }) {
		query = event.currentTarget.value;
		selectedIndex = -1;
		isOpen = query.length > 0;
		// An empty query cancels any pending search and clears the results.
		void marketData.searchTicker(query);
	}

	function handleKeydown(event: KeyboardEvent & { currentTarget: HTMLInputElement }) {
		if (event.key === 'Escape') {
			isOpen = false;
			event.currentTarget.blur();
			return;
		}
		if (!isOpen || results.length === 0) return;

		switch (event.key) {
			case 'ArrowDown':
				event.preventDefault();
				selectedIndex = Math.min(selectedIndex + 1, results.length - 1);
				break;
			case 'ArrowUp':
				event.preventDefault();
				selectedIndex = Math.max(selectedIndex - 1, -1);
				break;
			case 'Enter':
				event.preventDefault();
				if (selectedIndex >= 0 && selectedIndex < results.length) {
					void selectResult(results[selectedIndex].ticker);
				} else if (query.length > 0) {
					void selectResult(query.toUpperCase());
				}
				break;
		}
	}

	async function selectResult(ticker: string) {
		isOpen = false;
		query = ticker;
		selectedIndex = -1;

		const snapshot = await marketData.selectTicker(ticker);
		if (snapshot) onSnapshot?.(snapshot);
	}

	function clearTicker() {
		query = '';
		isOpen = false;
		marketData.clearData();
		inputEl?.focus();
	}

	function handleFocus() {
		if (query.length > 0 && results.length > 0) {
			isOpen = true;
		}
	}

	/** Keeps focus in the combobox while an option is clicked, so blur cannot close it first. */
	function keepInputFocus(event: MouseEvent) {
		event.preventDefault();
	}
</script>

<div class="relative w-full max-w-xs">
	<!-- Input -->
	<div class="relative flex items-center">
		<IconSearch
			size={14}
			class="absolute left-2.5 pointer-events-none"
			style="color: var(--calc-text-muted);"
			aria-hidden="true"
		/>
		<input
			bind:this={inputEl}
			type="text"
			value={query}
			oninput={handleInput}
			onkeydown={handleKeydown}
			onfocus={handleFocus}
			onblur={() => (isOpen = false)}
			placeholder="Search ticker..."
			aria-label="Search ticker"
			class="w-full text-xs pl-8 pr-8 py-1.5 rounded-lg outline-none transition-all motion-reduce:transition-none"
			style:background="var(--calc-surface)"
			style:color="var(--calc-text)"
			style:border="1px solid {isOpen ? 'var(--calc-accent)' : 'var(--calc-border)'}"
			style:font-family="var(--calc-font-mono)"
			autocomplete="off"
			spellcheck="false"
			role="combobox"
			aria-expanded={listboxShown}
			aria-haspopup="listbox"
			aria-autocomplete="list"
			aria-controls={listboxId}
			aria-activedescendant={listboxShown && selectedIndex >= 0
				? optionId(selectedIndex)
				: undefined}
		/>

		{#if marketData.isSearching}
			<IconLoader2
				size={12}
				class="absolute right-2.5 motion-safe:animate-spin"
				style="color: var(--calc-accent);"
				aria-hidden="true"
			/>
		{:else if marketData.activeTicker}
			<button
				type="button"
				onclick={clearTicker}
				class="absolute right-2.5 cursor-pointer"
				style="color: var(--calc-text-muted);"
				aria-label="Clear ticker"
			>
				<IconX size={12} aria-hidden="true" />
			</button>
		{/if}
	</div>

	<!-- Active Ticker Display -->
	{#if marketData.activeTicker && quote && !isOpen}
		<div class="flex items-center gap-2 mt-1.5 px-1">
			<span
				class="text-xs font-bold"
				style="color: var(--calc-text); font-family: var(--calc-font-mono);"
			>
				{marketData.activeTicker}
			</span>
			<span
				class="text-xs"
				style="color: var(--calc-text-secondary); font-family: var(--calc-font-mono);"
			>
				{formatCurrency(quote.price)}
			</span>
			<span
				class="text-[10px] font-medium"
				style:color={quote.change >= 0 ? 'var(--calc-call)' : 'var(--calc-put)'}
				style:font-family="var(--calc-font-mono)"
			>
				{quote.change >= 0 ? '+' : ''}{quote.change.toFixed(2)}
				({quote.changePercent >= 0 ? '+' : ''}{quote.changePercent.toFixed(2)}%)
			</span>
		</div>
	{/if}

	<!-- Dropdown -->
	{#if showDropdown}
		<div
			in:fly={{ y: -8, duration: prefersReducedMotion.current ? 0 : 200 }}
			class="absolute top-full left-0 right-0 mt-1 rounded-xl overflow-hidden z-50"
			style="background: var(--calc-surface); border: 1px solid var(--calc-accent); box-shadow: 0 8px 32px rgba(0,0,0,0.3); backdrop-filter: blur(16px);"
		>
			{#if marketData.isSearching && results.length === 0}
				<div class="flex items-center justify-center py-4">
					<IconLoader2
						size={16}
						class="motion-safe:animate-spin"
						style="color: var(--calc-accent);"
						aria-hidden="true"
					/>
					<span class="ml-2 text-xs" style="color: var(--calc-text-muted);">Searching...</span>
				</div>
			{:else}
				<div id={listboxId} role="listbox" aria-label="Matching tickers">
					{#each results as result, index (result.ticker)}
						{const selected = $derived(index === selectedIndex)}
						<button
							type="button"
							id={optionId(index)}
							role="option"
							aria-selected={selected}
							tabindex="-1"
							onmousedown={keepInputFocus}
							onclick={() => selectResult(result.ticker)}
							class="w-full flex items-center gap-3 px-3 py-2 text-left transition-colors motion-reduce:transition-none cursor-pointer"
							style:background={selected ? 'var(--calc-surface-hover)' : 'transparent'}
						>
							<span
								class="text-xs font-bold min-w-[3.5rem]"
								style="color: var(--calc-accent); font-family: var(--calc-font-mono);"
							>
								{result.ticker}
							</span>
							<span class="text-xs truncate flex-1" style="color: var(--calc-text-secondary);">
								{result.name}
							</span>
							<span
								class="text-[9px] px-1.5 py-0.5 rounded-full uppercase font-medium"
								style="background: var(--calc-surface-hover); color: var(--calc-text-muted);"
							>
								{result.type}
							</span>
						</button>
					{/each}
				</div>
			{/if}

			<!-- Provider attribution -->
			<div
				class="px-3 py-1.5 text-[9px] border-t"
				style="color: var(--calc-text-muted); border-color: var(--calc-border);"
			>
				Powered by {marketData.currentQuote?.source ?? 'Mock Data'}
			</div>
		</div>
	{/if}
</div>

<script lang="ts">
	import IconSearch from '@tabler/icons-svelte-runes/icons/search';
	import IconX from '@tabler/icons-svelte-runes/icons/x';
	import IconLoader2 from '@tabler/icons-svelte-runes/icons/loader-2';
	import gsap from 'gsap';
	import { formatCurrency } from '../utils/formatters.js';
	import type { MarketDataService } from '../data/market-data-service.svelte.js';
	import type { MarketSnapshot } from '../data/types.js';

	interface Props {
		marketData: MarketDataService;
		onSnapshot?: (snapshot: MarketSnapshot) => void;
	}

	let { marketData, onSnapshot }: Props = $props();

	let inputEl: HTMLInputElement | undefined;
	let dropdownEl: HTMLDivElement | undefined;
	let query = $state('');
	let isOpen = $state(false);
	let selectedIndex = $state(-1);

	let results = $derived(marketData.searchResults);
	let searching = $derived(marketData.isSearching);
	let activeQuote = $derived(marketData.currentQuote);
	let activeTicker = $derived(marketData.activeTicker);
	let inputBorderColor = $derived(isOpen ? 'var(--calc-accent)' : 'var(--calc-border)');
	let quoteChangeColor = $derived(
		activeQuote && activeQuote.change >= 0 ? 'var(--calc-call)' : 'var(--calc-put)'
	);

	function handleInput(e: Event) {
		const val = (e.currentTarget as HTMLInputElement).value;
		query = val;
		selectedIndex = -1;
		if (val.length > 0) {
			isOpen = true;
			marketData.searchTicker(val);
		} else {
			isOpen = false;
		}
	}

	function handleKeydown(e: KeyboardEvent) {
		if (!isOpen || results.length === 0) {
			if (e.key === 'Escape') {
				isOpen = false;
				inputEl?.blur();
			}
			return;
		}

		switch (e.key) {
			case 'ArrowDown':
				e.preventDefault();
				selectedIndex = Math.min(selectedIndex + 1, results.length - 1);
				break;
			case 'ArrowUp':
				e.preventDefault();
				selectedIndex = Math.max(selectedIndex - 1, -1);
				break;
			case 'Enter':
				e.preventDefault();
				if (selectedIndex >= 0 && selectedIndex < results.length) {
					selectResult(results[selectedIndex].ticker);
				} else if (query.length > 0) {
					selectResult(query.toUpperCase());
				}
				break;
			case 'Escape':
				isOpen = false;
				inputEl?.blur();
				break;
		}
	}

	async function selectResult(ticker: string) {
		isOpen = false;
		query = ticker;
		selectedIndex = -1;

		const snapshot = await marketData.selectTicker(ticker);
		if (snapshot && onSnapshot) {
			onSnapshot(snapshot);
		}
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

	function handleBlur() {
		setTimeout(() => {
			isOpen = false;
		}, 200);
	}

	function trackInput(element: HTMLInputElement) {
		inputEl = element;

		return () => {
			if (inputEl === element) {
				inputEl = undefined;
			}
		};
	}

	function animateDropdown(element: HTMLDivElement) {
		dropdownEl = element;
		gsap.fromTo(
			element,
			{ y: -8, opacity: 0 },
			{ y: 0, opacity: 1, duration: 0.2, ease: 'power2.out' }
		);

		return () => {
			if (dropdownEl === element) {
				dropdownEl = undefined;
			}
		};
	}
</script>

<div class="relative w-full max-w-xs">
	<!-- Input -->
	<div class="relative flex items-center">
		<IconSearch
			size={14}
			class="absolute left-2.5 pointer-events-none"
			style="color: var(--calc-text-muted);"
		/>
		<input
			{@attach trackInput}
			type="text"
			value={query}
			oninput={handleInput}
			onkeydown={handleKeydown}
			onfocus={handleFocus}
			onblur={handleBlur}
			placeholder="Search ticker..."
			class="w-full text-xs pl-8 pr-8 py-1.5 rounded-lg outline-none transition-all"
			style:background="var(--calc-surface)"
			style:color="var(--calc-text)"
			style:border={`1px solid ${inputBorderColor}`}
			style:font-family="var(--calc-font-mono)"
			autocomplete="off"
			spellcheck="false"
			role="combobox"
			aria-expanded={isOpen}
			aria-haspopup="listbox"
			aria-autocomplete="list"
			aria-controls="ticker-search-listbox"
		/>

		{#if searching}
			<IconLoader2
				size={12}
				class="absolute right-2.5 animate-spin"
				style="color: var(--calc-accent);"
			/>
		{:else if activeTicker}
			<button
				onclick={clearTicker}
				class="absolute right-2.5 cursor-pointer"
				style="color: var(--calc-text-muted);"
				aria-label="Clear ticker"
			>
				<IconX size={12} />
			</button>
		{/if}
	</div>

	<!-- Active Ticker Display -->
	{#if activeTicker && activeQuote && !isOpen}
		<div class="flex items-center gap-2 mt-1.5 px-1">
			<span
				class="text-xs font-bold"
				style="color: var(--calc-text); font-family: var(--calc-font-mono);"
			>
				{activeTicker}
			</span>
			<span
				class="text-xs"
				style="color: var(--calc-text-secondary); font-family: var(--calc-font-mono);"
			>
				{formatCurrency(activeQuote.price)}
			</span>
			<span
				class="text-[10px] font-medium"
				style:color={quoteChangeColor}
				style:font-family="var(--calc-font-mono)"
			>
				{activeQuote.change >= 0 ? '+' : ''}{activeQuote.change.toFixed(2)}
				({activeQuote.changePercent >= 0 ? '+' : ''}{activeQuote.changePercent.toFixed(2)}%)
			</span>
		</div>
	{/if}

	<!-- Dropdown -->
	{#if isOpen && (results.length > 0 || searching)}
		<div
			{@attach animateDropdown}
			class="absolute top-full left-0 right-0 mt-1 rounded-xl overflow-hidden z-50"
			style="background: var(--calc-surface); border: 1px solid var(--calc-accent); box-shadow: 0 8px 32px rgba(0,0,0,0.3); backdrop-filter: blur(16px);"
			role="listbox"
		>
			{#if searching && results.length === 0}
				<div class="flex items-center justify-center py-4">
					<IconLoader2 size={16} class="animate-spin" style="color: var(--calc-accent);" />
					<span class="ml-2 text-xs" style="color: var(--calc-text-muted);">Searching...</span>
				</div>
			{:else}
				{#each results as result, i (result.ticker)}
					<button
						onclick={() => selectResult(result.ticker)}
						class="w-full flex items-center gap-3 px-3 py-2 text-left transition-colors cursor-pointer"
						style:background={i === selectedIndex ? 'var(--calc-surface-hover)' : 'transparent'}
						role="option"
						aria-selected={i === selectedIndex}
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

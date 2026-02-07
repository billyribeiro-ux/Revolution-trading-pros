<script lang="ts">
	import { Wifi, WifiOff, Database } from '@lucide/svelte';
	import type { MarketDataService } from '../data/market-data-service.svelte.js';

	interface Props {
		marketData: MarketDataService;
	}

	let { marketData }: Props = $props();

	let expanded = $state(false);

	let source = $derived(marketData.currentQuote?.source ?? null);
	let isLive = $derived(source !== null && source !== 'mock');
	let label = $derived.by(() => {
		if (!source) return 'No Data';
		switch (source) {
			case 'polygon': return 'Polygon.io';
			case 'theta-data': return 'Theta Data';
			case 'tradier': return 'Tradier';
			case 'fred': return 'FRED';
			case 'yahoo': return 'Yahoo';
			case 'mock': return 'Mock Data';
			default: return source;
		}
	});

	let dotColor = $derived.by(() => {
		if (!source) return 'var(--calc-text-muted)';
		if (marketData.lastError) return 'var(--calc-put)';
		if (source === 'mock') return 'var(--calc-warning)';
		return 'var(--calc-call)';
	});

	let cacheStats = $derived(marketData.cache.getStats());
</script>

<div class="relative">
	<button
		onclick={() => (expanded = !expanded)}
		class="flex items-center gap-1.5 text-[10px] px-2 py-1 rounded-lg transition-colors cursor-pointer"
		style="background: var(--calc-surface); color: var(--calc-text-muted); border: 1px solid var(--calc-border);"
	>
		<span class="w-1.5 h-1.5 rounded-full" style="background: {dotColor};"></span>
		{#if isLive}
			<Wifi size={10} />
		{:else if source === 'mock'}
			<Database size={10} />
		{:else}
			<WifiOff size={10} />
		{/if}
		<span style="font-family: var(--calc-font-mono);">{label}</span>
	</button>

	{#if expanded}
		<div
			class="absolute top-full right-0 mt-1 w-56 rounded-xl p-3 z-50 flex flex-col gap-2"
			style="background: var(--calc-surface); border: 1px solid var(--calc-border); box-shadow: 0 8px 32px rgba(0,0,0,0.3);"
		>
			<div class="text-[10px] font-semibold" style="color: var(--calc-text);">Data Source</div>

			<div class="flex items-center justify-between">
				<span class="text-[10px]" style="color: var(--calc-text-muted);">Provider:</span>
				<span class="text-[10px] font-medium" style="color: var(--calc-text-secondary); font-family: var(--calc-font-mono);">{label}</span>
			</div>

			<div class="flex items-center justify-between">
				<span class="text-[10px]" style="color: var(--calc-text-muted);">Status:</span>
				<span class="flex items-center gap-1 text-[10px]" style="color: {dotColor};">
					<span class="w-1.5 h-1.5 rounded-full" style="background: {dotColor};"></span>
					{marketData.lastError ? 'Error' : isLive ? 'Live' : source === 'mock' ? 'Mock' : 'Offline'}
				</span>
			</div>

			<div class="flex items-center justify-between">
				<span class="text-[10px]" style="color: var(--calc-text-muted);">Cache entries:</span>
				<span class="text-[10px]" style="color: var(--calc-text-secondary); font-family: var(--calc-font-mono);">{cacheStats.memoryEntries}</span>
			</div>

			<div class="flex items-center justify-between">
				<span class="text-[10px]" style="color: var(--calc-text-muted);">Cache size:</span>
				<span class="text-[10px]" style="color: var(--calc-text-secondary); font-family: var(--calc-font-mono);">{(cacheStats.memoryBytes / 1024).toFixed(1)} KB</span>
			</div>

			{#if marketData.lastError}
				<div class="text-[9px] px-2 py-1 rounded" style="background: var(--calc-put-bg); color: var(--calc-put);">
					{marketData.lastError}
				</div>
			{/if}

			<button
				onclick={() => { marketData.cache.clearAll(); expanded = false; }}
				class="text-[10px] px-2 py-1 rounded-lg cursor-pointer mt-1"
				style="background: var(--calc-surface-hover); color: var(--calc-text-muted); border: 1px solid var(--calc-border);"
			>
				Clear Cache
			</button>
		</div>
	{/if}
</div>

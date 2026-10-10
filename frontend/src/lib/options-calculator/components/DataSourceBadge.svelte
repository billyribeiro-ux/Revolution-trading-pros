<script lang="ts">
	import IconWifi from '@tabler/icons-svelte-runes/icons/wifi';
	import IconWifiOff from '@tabler/icons-svelte-runes/icons/wifi-off';
	import IconDatabase from '@tabler/icons-svelte-runes/icons/database';
	import { getMarketData } from '../state/context.js';
	import type { DataProviderName } from '../data/types.js';

	const marketData = getMarketData();
	const panelId = $props.id();

	const PROVIDER_LABELS = {
		polygon: 'Polygon.io',
		'theta-data': 'Theta Data',
		tradier: 'Tradier',
		fred: 'FRED',
		yahoo: 'Yahoo',
		mock: 'Mock Data'
	} satisfies Record<DataProviderName, string>;

	let expanded = $state(false);

	const source = $derived(marketData.currentQuote?.source ?? null);
	const isLive = $derived(source !== null && source !== 'mock');
	const label = $derived(source ? PROVIDER_LABELS[source] : 'No Data');

	const dotColor = $derived.by(() => {
		if (!source) return 'var(--calc-text-muted)';
		if (marketData.lastError) return 'var(--calc-put)';
		if (source === 'mock') return 'var(--calc-warning)';
		return 'var(--calc-call)';
	});

	const status = $derived.by(() => {
		if (marketData.lastError) return 'Error';
		if (isLive) return 'Live';
		if (source === 'mock') return 'Mock';
		return 'Offline';
	});

	// Reactive: the cache notifies readers whenever entries are written or removed.
	const cacheStats = $derived(marketData.cache.getStats());

	function clearCache() {
		marketData.cache.clearAll();
		expanded = false;
	}
</script>

<div class="relative">
	<button
		type="button"
		onclick={() => (expanded = !expanded)}
		aria-expanded={expanded}
		aria-controls={panelId}
		class="flex items-center gap-1.5 text-[10px] px-2 py-1 rounded-lg transition-colors cursor-pointer"
		style="background: var(--calc-surface); color: var(--calc-text-muted); border: 1px solid var(--calc-border);"
	>
		<span class="w-1.5 h-1.5 rounded-full" style:background={dotColor} aria-hidden="true"></span>
		{#if isLive}
			<IconWifi size={10} />
		{:else if source === 'mock'}
			<IconDatabase size={10} />
		{:else}
			<IconWifiOff size={10} />
		{/if}
		<span class="sr-only">Data source:</span>
		<span style="font-family: var(--calc-font-mono);">{label}</span>
	</button>

	{#if expanded}
		<div
			id={panelId}
			class="absolute top-full right-0 mt-1 w-56 rounded-xl p-3 z-50 flex flex-col gap-2"
			style="background: var(--calc-surface); border: 1px solid var(--calc-border); box-shadow: 0 8px 32px rgba(0,0,0,0.3);"
		>
			<div class="text-[10px] font-semibold" style="color: var(--calc-text);">Data Source</div>

			<div class="flex items-center justify-between">
				<span class="text-[10px]" style="color: var(--calc-text-muted);">Provider:</span>
				<span
					class="text-[10px] font-medium"
					style="color: var(--calc-text-secondary); font-family: var(--calc-font-mono);"
					>{label}</span
				>
			</div>

			<div class="flex items-center justify-between">
				<span class="text-[10px]" style="color: var(--calc-text-muted);">Status:</span>
				<span class="flex items-center gap-1 text-[10px]" style:color={dotColor}>
					<span class="w-1.5 h-1.5 rounded-full" style:background={dotColor} aria-hidden="true"
					></span>
					{status}
				</span>
			</div>

			<div class="flex items-center justify-between">
				<span class="text-[10px]" style="color: var(--calc-text-muted);">Cache entries:</span>
				<span
					class="text-[10px]"
					style="color: var(--calc-text-secondary); font-family: var(--calc-font-mono);"
					>{cacheStats.memoryEntries}</span
				>
			</div>

			<div class="flex items-center justify-between">
				<span class="text-[10px]" style="color: var(--calc-text-muted);">Cache size:</span>
				<span
					class="text-[10px]"
					style="color: var(--calc-text-secondary); font-family: var(--calc-font-mono);"
					>{(cacheStats.memoryBytes / 1024).toFixed(1)} KB</span
				>
			</div>

			{#if marketData.lastError}
				<div
					class="text-[9px] px-2 py-1 rounded"
					style="background: var(--calc-put-bg); color: var(--calc-put);"
				>
					{marketData.lastError}
				</div>
			{/if}

			<button
				type="button"
				onclick={clearCache}
				class="text-[10px] px-2 py-1 rounded-lg cursor-pointer mt-1"
				style="background: var(--calc-surface-hover); color: var(--calc-text-muted); border: 1px solid var(--calc-border);"
			>
				Clear Cache
			</button>
		</div>
	{/if}
</div>

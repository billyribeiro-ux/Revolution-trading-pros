<script lang="ts">
	import gsap from 'gsap';
	import type { MarketDataService } from '../data/market-data-service.svelte.js';
	import type { DataMode } from '../data/types.js';

	interface Props {
		marketData: MarketDataService;
	}

	let { marketData }: Props = $props();

	let indicatorEl: HTMLDivElement | undefined = $state();

	function setMode(mode: DataMode) {
		marketData.setDataMode(mode);
		if (indicatorEl) {
			gsap.to(indicatorEl, {
				x: mode === 'live' ? '100%' : '0%',
				duration: 0.25,
				ease: 'power2.out',
			});
		}
	}

	let isLive = $derived(marketData.dataMode === 'live');
</script>

<div class="flex items-center gap-1.5">
	<div
		class="relative flex rounded-lg overflow-hidden"
		style="border: 1px solid var(--calc-border);"
	>
		<!-- Sliding indicator -->
		<div
			bind:this={indicatorEl}
			class="absolute top-0 left-0 w-1/2 h-full rounded-lg pointer-events-none transition-transform"
			style="background: {isLive ? 'var(--calc-accent-glow)' : 'var(--calc-surface-hover)'}; transform: translateX({isLive ? '100%' : '0%'});"
		></div>

		<button
			onclick={() => setMode('manual')}
			class="relative z-10 text-[10px] font-medium px-3 py-1 transition-colors cursor-pointer"
			style="color: {!isLive ? 'var(--calc-text)' : 'var(--calc-text-muted)'};"
		>
			Manual
		</button>
		<button
			onclick={() => setMode('live')}
			class="relative z-10 text-[10px] font-medium px-3 py-1 transition-colors cursor-pointer"
			style="color: {isLive ? 'var(--calc-accent)' : 'var(--calc-text-muted)'};"
		>
			Live Data
		</button>
	</div>

	{#if isLive}
		<span class="w-1.5 h-1.5 rounded-full animate-pulse" style="background: var(--calc-call);"></span>
	{/if}
</div>

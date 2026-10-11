<script lang="ts">
	import { getCalculator, getMarketData } from '../state/context.js';
	import type { DataMode } from '../data/types.js';

	const calc = getCalculator();
	const marketData = getMarketData();

	const isLive = $derived(marketData.dataMode === 'live');

	function setMode(mode: DataMode) {
		marketData.setDataMode(mode);
		// The calculator keeps its own copy: it gates the LIVE badges on market-filled inputs.
		calc.dataMode = mode;
	}
</script>

<div class="flex items-center gap-1.5">
	<div
		class="relative flex rounded-lg overflow-hidden"
		style="border: 1px solid var(--calc-border);"
		role="group"
		aria-label="Data mode"
	>
		<!-- Sliding indicator -->
		<div
			class="absolute top-0 left-0 w-1/2 h-full rounded-lg pointer-events-none transition-transform duration-250 ease-out motion-reduce:transition-none"
			style:background={isLive ? 'var(--calc-accent-glow)' : 'var(--calc-surface-hover)'}
			style:transform="translateX({isLive ? '100%' : '0%'})"
			aria-hidden="true"
		></div>

		<button
			type="button"
			onclick={() => setMode('manual')}
			aria-pressed={!isLive}
			class="relative z-10 text-[10px] font-medium px-3 py-1 transition-colors motion-reduce:transition-none cursor-pointer"
			style:color={isLive ? 'var(--calc-text-muted)' : 'var(--calc-text)'}
		>
			Manual
		</button>
		<button
			type="button"
			onclick={() => setMode('live')}
			aria-pressed={isLive}
			class="relative z-10 text-[10px] font-medium px-3 py-1 transition-colors motion-reduce:transition-none cursor-pointer"
			style:color={isLive ? 'var(--calc-accent)' : 'var(--calc-text-muted)'}
		>
			Live Data
		</button>
	</div>

	{#if isLive}
		<span
			class="w-1.5 h-1.5 rounded-full motion-safe:animate-pulse"
			style="background: var(--calc-call);"
			aria-hidden="true"
		></span>
	{/if}
</div>

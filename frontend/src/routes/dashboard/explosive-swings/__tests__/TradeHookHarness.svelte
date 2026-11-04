<script lang="ts">
	import { untrack } from 'svelte';
	import { useTrades } from '../hooks/useTrades.svelte';
	const { autoRefresh = false }: { autoRefresh?: boolean } = $props();
	const trades = useTrades({ autoRefresh: untrack(() => autoRefresh), refreshInterval: 100 });
</script>

<button onclick={trades.refresh}>Refresh</button>
<p data-testid="positions">{trades.openCount}</p>
{#each trades.activePositions as position (position.id)}
	<p data-testid="quote">{position.ticker}: {position.currentPrice}</p>
{/each}

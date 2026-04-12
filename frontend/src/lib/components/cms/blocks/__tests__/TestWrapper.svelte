<!--
/**
 * Test Wrapper Component
 * ===============================================================================
 * Provides BlockStateManager context for testing CMS block components
 */
-->

<script lang="ts">
	import { setBlockStateManager, BlockStateManager } from '$lib/stores/blockState.svelte';
	import type { Snippet } from 'svelte';

	interface Props {
		children: Snippet;
		stateManager?: BlockStateManager;
	}

	let { children, stateManager }: Props = $props();

	// Set context during component init (createContext pattern)
	setBlockStateManager(stateManager ?? new BlockStateManager());
</script>

{@render children()}

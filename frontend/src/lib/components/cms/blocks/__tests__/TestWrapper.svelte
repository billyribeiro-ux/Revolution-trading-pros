<!--
/**
 * Test Wrapper Component
 * ===============================================================================
 * Provides BlockStateManager context for testing CMS block components
 */
-->

<script lang="ts">
	import { setBlockStateManager, BlockStateManager } from '$lib/stores/blockState.svelte';
	import { onMount, onDestroy } from 'svelte';
	import type { Snippet } from 'svelte';

	interface Props {
		children: Snippet;
		stateManager?: BlockStateManager;
	}

	let { children, stateManager = new BlockStateManager() }: Props = $props();

	// Set the context when the component mounts
	setBlockStateManager(stateManager);
</script>

{@render children()}

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

	// Use provided stateManager or create a default one
	let resolvedStateManager = $derived(stateManager ?? new BlockStateManager());

	// Set the context reactively when the stateManager changes
	$effect(() => {
		setBlockStateManager(resolvedStateManager);
	});
</script>

{@render children()}

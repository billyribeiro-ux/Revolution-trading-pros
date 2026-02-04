<script lang="ts">
	/**
	 * ClientOnly Component
	 * Only renders children on the client side (after hydration)
	 *
	 * @version 1.0.0
	 * @author Revolution Trading Pros
	 */
	import { browser } from '$app/environment';
	import type { Snippet } from 'svelte';

	interface Props {
		children: Snippet;
		placeholder?: Snippet;
	}

	const { children, placeholder }: Props = $props();

	let mounted = $state(false);

	$effect(() => {
		if (browser) {
			mounted = true;
		}
	});
</script>

{#if mounted}
	{@render children()}
{:else if placeholder}
	{@render placeholder()}
{/if}

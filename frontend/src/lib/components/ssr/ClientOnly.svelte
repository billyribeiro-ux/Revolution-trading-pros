<script lang="ts">
	/**
	 * ClientOnly - SSR-Safe Component Wrapper
	 * Ensures children only render on the client, preventing hydration mismatches
	 *
	 * @version 1.0.0
	 * @author Revolution Trading Pros
	 * @level L8 Principal Engineer
	 */
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import SkeletonLoader from '$lib/components/ui/SkeletonLoader.svelte';

	interface Props {
		showSkeleton?: boolean;
		skeletonVariant?: 'text' | 'circular' | 'rectangular' | 'card' | 'stat';
		skeletonHeight?: string;
		skeletonWidth?: string;
		children?: import('svelte').Snippet;
		fallback?: import('svelte').Snippet;
	}

	let {
		showSkeleton = true,
		skeletonVariant = 'rectangular',
		skeletonHeight = '200px',
		skeletonWidth = '100%',
		children,
		fallback
	}: Props = $props();

	let mounted = $state(false);

	onMount(() => {
		mounted = true;
	});
</script>

{#if mounted || browser}
	{@render children?.()}
{:else if showSkeleton}
	<SkeletonLoader
		variant={skeletonVariant}
		height={skeletonHeight}
		width={skeletonWidth}
	/>
{:else}
	{@render fallback?.()}
{/if}

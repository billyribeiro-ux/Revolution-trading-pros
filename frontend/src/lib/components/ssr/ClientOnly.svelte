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

	export let showSkeleton: boolean = true;
	export let skeletonVariant: 'text' | 'circular' | 'rectangular' | 'card' | 'stat' = 'rectangular';
	export let skeletonHeight: string = '200px';
	export let skeletonWidth: string = '100%';

	let mounted = false;

	onMount(() => {
		mounted = true;
	});
</script>

{#if mounted || browser}
	<slot />
{:else if showSkeleton}
	<SkeletonLoader
		variant={skeletonVariant}
		height={skeletonHeight}
		width={skeletonWidth}
	/>
{:else}
	<slot name="fallback">
		<!-- Empty fallback by default -->
	</slot>
{/if}

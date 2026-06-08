<!--
  FavoriteButton.svelte
  Apple Principal Engineer ICT 7 Grade - February 2026

  Favorite/bookmark button for resources with:
  - Toggle favorite state
  - Optimistic UI updates
  - Loading states
  - Accessible design
-->
<script lang="ts">
	import { addFavorite, removeFavorite, checkFavorite } from '$lib/api/room-resources';
	import Icon from '$lib/components/Icon.svelte';
	import { onMount } from 'svelte';

	interface Props {
		resourceId: number;
		initialFavorited?: boolean;
		size?: 'sm' | 'md' | 'lg';
		showLabel?: boolean;
		onChange?: (favorited: boolean) => void;
	}

	let {
		resourceId,
		initialFavorited = undefined,
		size = 'md',
		showLabel = false,
		onChange
	}: Props = $props();

	let isFavorited = $derived<boolean>(initialFavorited ?? false);
	let loading = $derived<boolean>(initialFavorited === undefined);
	let updating = $state(false);

	onMount(() => {
		if (initialFavorited !== undefined) {
			isFavorited = initialFavorited;
			loading = false;
			return;
		}

		void loadFavoriteState();
	});

	async function loadFavoriteState() {
		loading = true;
		try {
			const response = await checkFavorite(resourceId);
			isFavorited = response.is_favorited ?? false;
		} catch (e) {
			console.error('Failed to check favorite state:', e);
		} finally {
			loading = false;
		}
	}

	async function toggleFavorite(event: MouseEvent) {
		event.stopPropagation();
		if (updating) return;

		updating = true;
		const previousState = isFavorited;

		// Optimistic update
		isFavorited = !isFavorited;

		try {
			if (isFavorited) {
				await addFavorite(resourceId);
			} else {
				await removeFavorite(resourceId);
			}
			onChange?.(isFavorited);
		} catch (e) {
			// Revert on error
			isFavorited = previousState;
			console.error('Failed to update favorite:', e);
		} finally {
			updating = false;
		}
	}

	// Size classes
	let sizeClasses = $derived(
		{
			sm: 'h-6 w-6 p-1',
			md: 'h-8 w-8 p-1.5',
			lg: 'h-10 w-10 p-2'
		}[size]
	);

	let iconSize = $derived(
		{
			sm: 'h-4 w-4',
			md: 'h-5 w-5',
			lg: 'h-6 w-6'
		}[size]
	);

	let loaderIconClass = $derived(`animate-spin ${iconSize}`);
	let favoritedIconClass = $derived(`${iconSize} fill-current`);
</script>

<button
	type="button"
	class={[
		'favorite-button inline-flex items-center gap-1.5 rounded-lg transition-all duration-200 focus:outline-none focus:ring-2 focus:ring-amber-500/50 disabled:cursor-not-allowed disabled:opacity-50',
		sizeClasses,
		isFavorited ? 'text-amber-500 hover:text-amber-600' : 'text-gray-400 hover:text-amber-500'
	]}
	onclick={toggleFavorite}
	disabled={loading || updating}
	aria-label={isFavorited ? 'Remove from favorites' : 'Add to favorites'}
	aria-pressed={isFavorited}
>
	{#if loading || updating}
		<Icon name="IconLoader2" class={loaderIconClass} />
	{:else if isFavorited}
		<Icon name="IconHeart" class={favoritedIconClass} />
	{:else}
		<Icon name="IconHeart" class={iconSize} />
	{/if}

	{#if showLabel}
		<span class="text-sm font-medium">
			{isFavorited ? 'Favorited' : 'Favorite'}
		</span>
	{/if}
</button>

<style>
	.favorite-button:hover :global(svg) {
		transform: scale(1.1);
	}

	.favorite-button:active :global(svg) {
		transform: scale(0.95);
	}
</style>

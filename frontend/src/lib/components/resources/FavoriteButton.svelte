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

	let isFavorited = $state(initialFavorited ?? false);
	let loading = $state(initialFavorited === undefined);
	let updating = $state(false);

	// Check initial state if not provided
	$effect(() => {
		if (initialFavorited === undefined && typeof window !== 'undefined') {
			loadFavoriteState();
		}
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
	let sizeClasses = $derived({
		sm: 'h-6 w-6 p-1',
		md: 'h-8 w-8 p-1.5',
		lg: 'h-10 w-10 p-2'
	}[size]);

	let iconSize = $derived({
		sm: 'h-4 w-4',
		md: 'h-5 w-5',
		lg: 'h-6 w-6'
	}[size]);
</script>

<button
	type="button"
	class="favorite-button inline-flex items-center gap-1.5 rounded-lg transition-all duration-200 focus:outline-none focus:ring-2 focus:ring-amber-500/50 disabled:cursor-not-allowed disabled:opacity-50 {sizeClasses} {isFavorited
		? 'text-amber-500 hover:text-amber-600'
		: 'text-gray-400 hover:text-amber-500'}"
	onclick={toggleFavorite}
	disabled={loading || updating}
	aria-label={isFavorited ? 'Remove from favorites' : 'Add to favorites'}
	aria-pressed={isFavorited}
>
	{#if loading || updating}
		<svg class="animate-spin {iconSize}" fill="none" viewBox="0 0 24 24">
			<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
			<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
		</svg>
	{:else if isFavorited}
		<svg class="{iconSize} fill-current" viewBox="0 0 24 24">
			<path d="M12 21.35l-1.45-1.32C5.4 15.36 2 12.28 2 8.5 2 5.42 4.42 3 7.5 3c1.74 0 3.41.81 4.5 2.09C13.09 3.81 14.76 3 16.5 3 19.58 3 22 5.42 22 8.5c0 3.78-3.4 6.86-8.55 11.54L12 21.35z" />
		</svg>
	{:else}
		<svg class="{iconSize}" fill="none" stroke="currentColor" viewBox="0 0 24 24">
			<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z" />
		</svg>
	{/if}

	{#if showLabel}
		<span class="text-sm font-medium">
			{isFavorited ? 'Favorited' : 'Favorite'}
		</span>
	{/if}
</button>

<style>
	.favorite-button:hover svg {
		transform: scale(1.1);
	}

	.favorite-button:active svg {
		transform: scale(0.95);
	}
</style>

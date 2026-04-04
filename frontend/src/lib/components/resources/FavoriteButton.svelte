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
import { logger } from '$lib/utils/logger';
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

	let isFavorited = $state(false);
	let loading = $state(true);

	// Sync with prop when provided
	$effect(() => {
		if (initialFavorited !== undefined) {
			isFavorited = initialFavorited;
			loading = false;
		}
	});
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
			logger.error('Failed to check favorite state:', e);
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
			logger.error('Failed to update favorite:', e);
		} finally {
			updating = false;
		}
	}

</script>

<button
	type="button"
	class="fav-btn"
	data-size={size}
	data-favorited={isFavorited || undefined}
	onclick={toggleFavorite}
	disabled={loading || updating}
	aria-label={isFavorited ? 'Remove from favorites' : 'Add to favorites'}
	aria-pressed={isFavorited}
>
	{#if loading || updating}
		<svg class="fav-icon fav-spin" fill="none" viewBox="0 0 24 24">
			<circle class="fav-spinner-track" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
			<path class="fav-spinner-fill" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
		</svg>
	{:else if isFavorited}
		<svg class="fav-icon" fill="currentColor" viewBox="0 0 24 24">
			<path d="M12 21.35l-1.45-1.32C5.4 15.36 2 12.28 2 8.5 2 5.42 4.42 3 7.5 3c1.74 0 3.41.81 4.5 2.09C13.09 3.81 14.76 3 16.5 3 19.58 3 22 5.42 22 8.5c0 3.78-3.4 6.86-8.55 11.54L12 21.35z" />
		</svg>
	{:else}
		<svg class="fav-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
			<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z" />
		</svg>
	{/if}

	{#if showLabel}
		<span class="fav-label">{isFavorited ? 'Favorited' : 'Favorite'}</span>
	{/if}
</button>

<style>
	.fav-btn {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		border: none;
		background: none;
		border-radius: var(--radius-lg);
		cursor: pointer;
		transition: all 200ms var(--ease-default);
		color: oklch(0.65 0.01 265);

		&:hover { color: oklch(0.7 0.18 80); }
		&:focus-visible { outline: 2px solid oklch(0.7 0.18 80 / 50%); outline-offset: 2px; }
		&:disabled { cursor: not-allowed; opacity: 0.5; }

		&[data-favorited] { color: oklch(0.7 0.18 80); }
		&[data-favorited]:hover { color: oklch(0.6 0.2 80); }

		&[data-size='sm'] { inline-size: 1.5rem; block-size: 1.5rem; padding: 0.25rem; }
		&[data-size='md'] { inline-size: 2rem; block-size: 2rem; padding: 0.375rem; }
		&[data-size='lg'] { inline-size: 2.5rem; block-size: 2.5rem; padding: 0.5rem; }

		&:hover svg { transform: scale(1.1); }
		&:active svg { transform: scale(0.95); }
	}

	.fav-icon {
		inline-size: 100%;
		block-size: 100%;
		transition: transform 200ms var(--ease-default);
	}

	.fav-spin { animation: spin 1s linear infinite; }
	.fav-spinner-track { opacity: 0.25; }
	.fav-spinner-fill { opacity: 0.75; }

	.fav-label { font-size: var(--text-sm); font-weight: var(--weight-medium); }

	@keyframes spin {
		from { transform: rotate(0deg); }
		to { transform: rotate(360deg); }
	}
</style>

<script lang="ts">
	/**
	 * FavoriteButton - Persistent bookmark toggle
	 * ═══════════════════════════════════════════════════════════════════════════
	 * Toggles favorite status for any item (alert, video, trade plan)
	 *
	 * @version 2.0.0 - ICT 11 Principal Engineer Grade
	 * @requires Svelte 5.0+ / SvelteKit 2.0+
	 */

	interface Props {
		roomSlug: string;
		itemType: 'alert' | 'video' | 'trade_plan' | 'resource';
		itemId: number;
		title?: string;
		excerpt?: string;
		href?: string;
		thumbnailUrl?: string;
		size?: 'sm' | 'md' | 'lg';
		showLabel?: boolean;
	}

	let {
		roomSlug,
		itemType,
		itemId,
		title = '',
		excerpt = '',
		href = '',
		thumbnailUrl = '',
		size = 'md',
		showLabel = false
	}: Props = $props();

	let isFavorited = $state(false);
	let isLoading = $state(false);
	let favoriteId = $state<number | null>(null);

	// Check if item is favorited on mount
	async function checkFavoriteStatus() {
		try {
			const res = await fetch(`/api/favorites/check?item_type=${itemType}&item_id=${itemId}`);
			if (res.ok) {
				const data = await res.json();
				isFavorited = data.is_favorited;
				if (data.data) {
					favoriteId = data.data.id;
				}
			}
		} catch (err) {
			console.error('Failed to check favorite status:', err);
		}
	}

	// Toggle favorite
	async function toggleFavorite(e: MouseEvent) {
		e.preventDefault();
		e.stopPropagation();

		if (isLoading) return;
		isLoading = true;

		try {
			if (isFavorited && favoriteId) {
				// Remove from favorites
				const res = await fetch(`/api/favorites/${favoriteId}`, { method: 'DELETE' });
				if (res.ok) {
					isFavorited = false;
					favoriteId = null;
				}
			} else {
				// Add to favorites
				const res = await fetch('/api/favorites', {
					method: 'POST',
					headers: { 'Content-Type': 'application/json' },
					body: JSON.stringify({
						room_slug: roomSlug,
						item_type: itemType,
						item_id: itemId,
						title,
						excerpt,
						href,
						thumbnail_url: thumbnailUrl
					})
				});
				if (res.ok) {
					const data = await res.json();
					isFavorited = true;
					if (data.data) {
						favoriteId = data.data.id;
					}
				}
			}
		} catch (err) {
			console.error('Failed to toggle favorite:', err);
		} finally {
			isLoading = false;
		}
	}

	// Check status on mount
	$effect(() => {
		checkFavoriteStatus();
	});

	const sizeClasses = {
		sm: 'w-5 h-5',
		md: 'w-6 h-6',
		lg: 'w-8 h-8'
	};
</script>

<button
	class="favorite-btn"
	class:favorited={isFavorited}
	class:loading={isLoading}
	class:size-sm={size === 'sm'}
	class:size-lg={size === 'lg'}
	onclick={toggleFavorite}
	title={isFavorited ? 'Remove from favorites' : 'Add to favorites'}
	aria-label={isFavorited ? 'Remove from favorites' : 'Add to favorites'}
>
	<svg
		viewBox="0 0 24 24"
		fill={isFavorited ? 'currentColor' : 'none'}
		stroke="currentColor"
		stroke-width="2"
		class={sizeClasses[size]}
	>
		<path
			d="M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z"
		/>
	</svg>
	{#if showLabel}
		<span class="label">{isFavorited ? 'Saved' : 'Save'}</span>
	{/if}
</button>

<style>
	.favorite-btn {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		padding: 8px;
		background: transparent;
		border: none;
		cursor: pointer;
		color: #9ca3af;
		transition: all 0.2s ease;
		border-radius: 8px;
	}

	.favorite-btn:hover {
		color: #f59e0b;
		background: rgba(245, 158, 11, 0.1);
	}

	.favorite-btn.favorited {
		color: #f59e0b;
	}

	.favorite-btn.favorited:hover {
		color: #d97706;
	}

	.favorite-btn.loading {
		opacity: 0.5;
		pointer-events: none;
	}

	.favorite-btn svg {
		flex-shrink: 0;
	}

	.size-sm {
		padding: 4px;
	}

	.size-lg {
		padding: 10px;
	}

	.label {
		font-size: 13px;
		font-weight: 600;
	}

	.size-sm .label {
		font-size: 11px;
	}
</style>

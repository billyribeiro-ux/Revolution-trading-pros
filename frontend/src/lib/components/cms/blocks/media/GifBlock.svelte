<!--
/**
 * GIF Block Component
 * ═══════════════════════════════════════════════════════════════════════════
 * Animated GIF with Giphy search integration
 */
-->

<script lang="ts">
	import { IconGif, IconSearch, IconLoader2, IconX } from '$lib/icons';
	import { sanitizeURL } from '$lib/utils/sanitization';
	import { getBlockStateManager, type BlockId } from '$lib/stores/blockState.svelte';
	import type { Block, BlockContent } from '../types';

	interface Props {
		block: Block;
		blockId: BlockId;
		isSelected: boolean;
		isEditing: boolean;
		onUpdate: (updates: Partial<Block>) => void;
		onError?: (error: Error) => void;
	}

	const props: Props = $props();
	const stateManager = getBlockStateManager();

	let searchQuery = $state('');
	let searchResults = $state<Array<{ id: string; url: string; preview: string; title: string }>>([]);
	let isSearching = $state(false);

	let gifUrl = $derived(props.block.content.mediaUrl || '');
	let gifAlt = $derived(props.block.content.mediaAlt || 'Animated GIF');

	function updateContent(updates: Partial<BlockContent>): void {
		props.onUpdate({ content: { ...props.block.content, ...updates } });
	}

	async function searchGifs(): Promise<void> {
		if (!searchQuery.trim()) return;

		isSearching = true;
		
		try {
			// Simulated Giphy API response - replace with actual Giphy API integration
			await new Promise(resolve => setTimeout(resolve, 800));
			
			// Mock results for demonstration
			searchResults = [
				{ id: '1', url: `https://media.giphy.com/media/placeholder1/giphy.gif`, preview: `https://media.giphy.com/media/placeholder1/200w.gif`, title: `${searchQuery} GIF 1` },
				{ id: '2', url: `https://media.giphy.com/media/placeholder2/giphy.gif`, preview: `https://media.giphy.com/media/placeholder2/200w.gif`, title: `${searchQuery} GIF 2` },
				{ id: '3', url: `https://media.giphy.com/media/placeholder3/giphy.gif`, preview: `https://media.giphy.com/media/placeholder3/200w.gif`, title: `${searchQuery} GIF 3` },
				{ id: '4', url: `https://media.giphy.com/media/placeholder4/giphy.gif`, preview: `https://media.giphy.com/media/placeholder4/200w.gif`, title: `${searchQuery} GIF 4` },
			];
		} catch (error) {
			props.onError?.(error instanceof Error ? error : new Error('Failed to search GIFs'));
			searchResults = [];
		} finally {
			isSearching = false;
		}
	}

	function selectGif(gif: { url: string; title: string }): void {
		updateContent({ mediaUrl: gif.url, mediaAlt: gif.title });
		searchResults = [];
		searchQuery = '';
	}

	function clearGif(): void {
		updateContent({ mediaUrl: '', mediaAlt: '' });
	}

	function handleKeyDown(e: KeyboardEvent): void {
		if (e.key === 'Enter') {
			e.preventDefault();
			searchGifs();
		}
	}
</script>

<div class="gif-block" role="figure" aria-label={gifAlt}>
	{#if props.isEditing}
		{#if gifUrl}
			<div class="gif-preview">
				<img src={sanitizeURL(gifUrl)} alt={gifAlt} />
				<button type="button" class="gif-remove" onclick={clearGif} aria-label="Remove GIF">
					<IconX size={16} />
				</button>
			</div>
			<div class="gif-meta">
				<input
					type="text"
					placeholder="Alt text for accessibility"
					value={gifAlt}
					oninput={(e) => updateContent({ mediaAlt: (e.target as HTMLInputElement).value })}
				/>
			</div>
		{:else}
			<div class="gif-search">
				<div class="search-header">
					<IconGif size={24} aria-hidden="true" />
					<span>Search for GIFs</span>
				</div>
				<div class="search-input">
					<IconSearch size={18} aria-hidden="true" />
					<input
						type="text"
						placeholder="Search Giphy..."
						bind:value={searchQuery}
						onkeydown={handleKeyDown}
					/>
					<button type="button" onclick={searchGifs} disabled={isSearching || !searchQuery.trim()}>
						{#if isSearching}
							<IconLoader2 size={18} class="spinning" />
						{:else}
							Search
						{/if}
					</button>
				</div>

				{#if searchResults.length > 0}
					<div class="search-results">
						{#each searchResults as gif (gif.id)}
							<button
								type="button"
								class="gif-result"
								onclick={() => selectGif(gif)}
								aria-label="Select {gif.title}"
							>
								<div class="gif-placeholder">
									<IconGif size={24} />
									<span>{gif.title}</span>
								</div>
							</button>
						{/each}
					</div>
				{/if}

				<div class="gif-url-input">
					<span>Or paste a GIF URL directly:</span>
					<input
						type="url"
						placeholder="https://media.giphy.com/..."
						oninput={(e) => {
							const url = (e.target as HTMLInputElement).value;
							if (url) updateContent({ mediaUrl: url, mediaAlt: 'Animated GIF' });
						}}
					/>
				</div>
			</div>
		{/if}
	{:else if gifUrl}
		<img src={sanitizeURL(gifUrl)} alt={gifAlt} loading="lazy" />
	{:else}
		<div class="gif-empty">
			<IconGif size={48} aria-hidden="true" />
			<p>No GIF selected</p>
		</div>
	{/if}
</div>

<style>
	.gif-block {
		border: 1px solid #e5e7eb;
		border-radius: 12px;
		overflow: hidden;
	}

	.gif-block > img {
		display: block;
		width: 100%;
		height: auto;
	}

	.gif-preview {
		position: relative;
	}

	.gif-preview img {
		display: block;
		width: 100%;
		height: auto;
	}

	.gif-remove {
		position: absolute;
		top: 0.75rem;
		right: 0.75rem;
		width: 32px;
		height: 32px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(0, 0, 0, 0.7);
		border: none;
		border-radius: 8px;
		color: white;
		cursor: pointer;
		transition: background 0.15s;
	}

	.gif-remove:hover { background: #dc2626; }

	.gif-meta {
		padding: 0.75rem;
		background: #f8fafc;
		border-top: 1px solid #e5e7eb;
	}

	.gif-meta input {
		width: 100%;
		padding: 0.5rem 0.75rem;
		border: 1px solid #d1d5db;
		border-radius: 6px;
		font-size: 0.875rem;
	}

	.gif-search {
		padding: 1.5rem;
		background: #f8fafc;
	}

	.search-header {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		margin-bottom: 1rem;
		color: #1e293b;
		font-weight: 600;
	}

	.search-input {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.5rem 0.75rem;
		background: white;
		border: 1px solid #d1d5db;
		border-radius: 8px;
		color: #94a3b8;
	}

	.search-input input {
		flex: 1;
		border: none;
		font-size: 0.9375rem;
		outline: none;
	}

	.search-input button {
		padding: 0.5rem 1rem;
		background: #3b82f6;
		border: none;
		border-radius: 6px;
		color: white;
		font-weight: 500;
		cursor: pointer;
		display: flex;
		align-items: center;
		gap: 0.375rem;
	}

	.search-input button:disabled { opacity: 0.6; cursor: not-allowed; }
	.search-input button :global(.spinning) { animation: spin 1s linear infinite; }
	@keyframes spin { to { transform: rotate(360deg); } }

	.search-results {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 0.75rem;
		margin-top: 1rem;
	}

	.gif-result {
		aspect-ratio: 16/9;
		border: 2px solid #e5e7eb;
		border-radius: 8px;
		overflow: hidden;
		cursor: pointer;
		transition: border-color 0.15s;
		background: #e2e8f0;
	}

	.gif-result:hover { border-color: #3b82f6; }

	.gif-placeholder {
		width: 100%;
		height: 100%;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		color: #64748b;
		font-size: 0.75rem;
	}

	.gif-url-input {
		margin-top: 1rem;
		padding-top: 1rem;
		border-top: 1px solid #e5e7eb;
	}

	.gif-url-input span {
		display: block;
		font-size: 0.8125rem;
		color: #64748b;
		margin-bottom: 0.5rem;
	}

	.gif-url-input input {
		width: 100%;
		padding: 0.625rem 0.75rem;
		border: 1px solid #d1d5db;
		border-radius: 6px;
		font-size: 0.875rem;
	}

	.gif-empty {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.75rem;
		padding: 3rem;
		background: #f8fafc;
		color: #94a3b8;
	}

	.gif-empty p { margin: 0; }

	:global(.dark) .gif-block { border-color: #334155; }
	:global(.dark) .gif-meta { background: #1e293b; border-color: #334155; }
	:global(.dark) .gif-meta input { background: #0f172a; border-color: #475569; color: #e2e8f0; }
	:global(.dark) .gif-search { background: #1e293b; }
	:global(.dark) .search-header { color: #f1f5f9; }
	:global(.dark) .search-input { background: #0f172a; border-color: #475569; }
	:global(.dark) .search-input input { background: transparent; color: #e2e8f0; }
	:global(.dark) .gif-result { border-color: #334155; background: #0f172a; }
	:global(.dark) .gif-url-input { border-color: #334155; }
	:global(.dark) .gif-url-input input { background: #0f172a; border-color: #475569; color: #e2e8f0; }
	:global(.dark) .gif-empty { background: #1e293b; color: #64748b; }
</style>

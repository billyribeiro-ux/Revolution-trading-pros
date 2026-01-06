<!--
	VideoGrid Component
	═══════════════════════════════════════════════════════════════════════════
	
	Apple ICT 11+ Principal Engineer Grade - January 2026
	
	Reusable video grid component with pagination for learning center, 
	daily videos, and archives pages.
	
	@version 1.0.0
-->
<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import VideoCard from './VideoCard.svelte';

	interface VideoData {
		id: number | string;
		title: string;
		slug: string;
		description?: string | null;
		thumbnail_url?: string | null;
		tag_details?: Array<{ slug: string; name: string; color: string }>;
		trader?: { id: number; name: string; slug: string } | null;
		formatted_date?: string;
		duration?: number | null;
		formatted_duration?: string;
		[key: string]: unknown; // Allow additional properties
	}

	interface Props {
		videos: VideoData[];
		currentPage: number;
		totalPages: number;
		basePath?: string;
		showDate?: boolean;
		showDuration?: boolean;
		emptyMessage?: string;
	}

	let { 
		videos, 
		currentPage,
		totalPages,
		basePath = '/learning-center',
		showDate = false,
		showDuration = false,
		emptyMessage = 'No videos found'
	}: Props = $props();

	// Navigate to page
	function goToPage(pageNum: number) {
		if (pageNum >= 1 && pageNum <= totalPages) {
			const url = new URL($page.url);
			if (pageNum === 1) {
				url.searchParams.delete('page');
			} else {
				url.searchParams.set('page', pageNum.toString());
			}
			goto(url.toString(), { replaceState: true });
			window.scrollTo({ top: 0, behavior: 'smooth' });
		}
	}

	function getPaginationRange(): (number | string)[] {
		const range: (number | string)[] = [];
		const delta = 1;

		if (totalPages <= 7) {
			for (let i = 1; i <= totalPages; i++) {
				range.push(i);
			}
		} else {
			range.push(1);
			if (currentPage > 3) {
				range.push('...');
			}
			for (let i = Math.max(2, currentPage - delta); i <= Math.min(totalPages - 1, currentPage + delta); i++) {
				if (!range.includes(i)) {
					range.push(i);
				}
			}
			if (currentPage < totalPages - 2) {
				range.push('...');
			}
			if (!range.includes(totalPages)) {
				range.push(totalPages);
			}
		}
		return range;
	}
</script>

<div id="response">
	{#if videos.length === 0}
		<div class="empty-state">
			<h3>{emptyMessage}</h3>
			<p>Try adjusting your filter or check back later for new content.</p>
		</div>
	{:else}
		<div class="article-cards row flex-grid">
			{#each videos as video (video.id)}
				<VideoCard {video} {basePath} {showDate} {showDuration} />
			{/each}
		</div>
	{/if}
</div>

<!-- Pagination -->
{#if totalPages > 1}
	<div class="fl-builder-pagination">
		<ul class="page-numbers">
			{#if currentPage > 1}
				<li>
					<button 
						class="page-numbers" 
						onclick={() => goToPage(currentPage - 1)}
						type="button"
						aria-label="Previous page"
					>
						&laquo;
					</button>
				</li>
			{/if}
			
			{#each getPaginationRange() as pageNum}
				<li>
					{#if pageNum === '...'}
						<span class="page-numbers dots">…</span>
					{:else if pageNum === currentPage}
						<span class="page-numbers current" aria-current="page">{pageNum}</span>
					{:else}
						<button 
							class="page-numbers" 
							onclick={() => goToPage(pageNum as number)}
							type="button"
						>
							{pageNum}
						</button>
					{/if}
				</li>
			{/each}
			
			{#if currentPage < totalPages}
				<li>
					<button 
						class="page-numbers" 
						onclick={() => goToPage(currentPage + 1)}
						type="button"
						aria-label="Next page"
					>
						&raquo;
					</button>
				</li>
			{/if}
		</ul>
	</div>
{/if}

<style>
	/* Empty State */
	.empty-state {
		text-align: center;
		padding: 60px 20px;
		color: #666;
	}

	.empty-state h3 {
		margin: 0 0 10px;
		font-size: 18px;
	}

	/* Grid Layout */
	.article-cards {
		display: flex;
		flex-wrap: wrap;
		margin: 0 -15px;
	}

	/* Pagination */
	.fl-builder-pagination {
		text-align: center;
		padding: 30px 0;
	}

	.page-numbers {
		list-style: none;
		display: flex;
		justify-content: center;
		gap: 5px;
		padding: 0;
		margin: 0;
	}

	.page-numbers li {
		display: inline-block;
	}

	button.page-numbers,
	span.page-numbers {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		min-width: 36px;
		height: 36px;
		padding: 0 12px;
		border: 1px solid #ddd;
		background: #fff;
		color: #333;
		font-size: 14px;
		text-decoration: none;
		border-radius: 4px;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	button.page-numbers:hover {
		background: #0984ae;
		border-color: #0984ae;
		color: #fff;
	}

	span.page-numbers.current {
		background: #0984ae;
		border-color: #0984ae;
		color: #fff;
		cursor: default;
	}

	span.page-numbers.dots {
		border: none;
		background: transparent;
		cursor: default;
	}
</style>

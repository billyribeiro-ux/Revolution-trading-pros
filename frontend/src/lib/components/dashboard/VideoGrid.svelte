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
	import { page } from '$app/state';
	import VideoCard from './VideoCard.svelte';

	interface VideoData {
		// Core required properties
		id: number | string;
		title: string;
		slug: string;

		// Optional display properties
		description?: string | null;
		thumbnail_url?: string | null;
		tag_details?: Array<{ slug: string; name: string; color: string }>;
		trader?: { id: number; name: string; slug: string } | null;
		formatted_date?: string;
		duration?: number | null;
		formatted_duration?: string;

		// Allow any additional properties from API responses
		video_url?: string;
		embed_url?: string;
		video_platform?: string;
		content_type?: string;
		video_date?: string;
		is_published?: boolean;
		is_featured?: boolean;
		tags?: string[];
		views_count?: number;
		rooms?: Array<{ id: number; name: string; slug: string }>;
		created_at?: string;

		// Catch-all for any other properties
		[key: string]: unknown;
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

	let props: Props = $props();

	// Derived props with defaults
	let videos = $derived(props.videos);
	let currentPage = $derived(props.currentPage);
	let totalPages = $derived(props.totalPages);
	let basePath = $derived(props.basePath ?? '/learning-center');
	let showDate = $derived(props.showDate ?? false);
	let showDuration = $derived(props.showDuration ?? false);
	let emptyMessage = $derived(props.emptyMessage ?? 'No videos found');

	// Navigate to page
	function goToPage(pageNum: number) {
		if (pageNum >= 1 && pageNum <= totalPages) {
			const url = new URL(page.url);
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
			for (
				let i = Math.max(2, currentPage - delta);
				i <= Math.min(totalPages - 1, currentPage + delta);
				i++
			) {
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
		<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
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
						<button class="page-numbers" onclick={() => goToPage(pageNum as number)} type="button">
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
	/* ═══════════════════════════════════════════════════════════════════════════
	   VIDEO GRID (Dashboard) - 2026 Mobile-First Responsive Design
	   ═══════════════════════════════════════════════════════════════════════════
	   Breakpoints: xs(360px), sm(640px), md(768px), lg(1024px), xl(1280px)
	   Grid: 1 col (mobile) → 2 cols (sm) → 3 cols (lg) → 4 cols (xl)
	   Touch targets: 44x44px minimum for pagination buttons
	   ═══════════════════════════════════════════════════════════════════════════ */

	/* Empty State - Mobile first */
	.empty-state {
		text-align: center;
		padding: 40px 16px;
		color: #666;
	}

	.empty-state h3 {
		margin: 0 0 8px;
		font-size: 16px;
	}

	.empty-state p {
		font-size: 14px;
		margin: 0;
	}

	/* Pagination - Mobile first with 44px touch targets */
	.fl-builder-pagination {
		text-align: center;
		padding: 24px 16px;
	}

	.page-numbers {
		list-style: none;
		display: flex;
		justify-content: center;
		flex-wrap: wrap;
		gap: 6px;
		padding: 0;
		margin: 0;
	}

	.page-numbers li {
		display: inline-block;
	}

	/* 44px minimum touch targets for mobile */
	button.page-numbers,
	span.page-numbers {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		min-width: 44px;
		min-height: 44px;
		padding: 0 12px;
		border: 1px solid #ddd;
		background: #fff;
		color: #333;
		font-size: 14px;
		font-weight: 500;
		text-decoration: none;
		border-radius: 8px;
		cursor: pointer;
		transition: all 0.2s ease;
		-webkit-tap-highlight-color: transparent;
		touch-action: manipulation;
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
		min-width: 32px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE BREAKPOINTS - Mobile First (min-width queries)
	   ═══════════════════════════════════════════════════════════════════════════ */

	/* xs: 360px+ */
	@media (min-width: 360px) {
		.empty-state {
			padding: 48px 20px;
		}
	}

	/* sm: 640px+ */
	@media (min-width: 640px) {
		.empty-state {
			padding: 60px 24px;
		}

		.empty-state h3 {
			font-size: 18px;
			margin-bottom: 10px;
		}

		.fl-builder-pagination {
			padding: 30px 0;
		}

		.page-numbers {
			gap: 8px;
		}

		button.page-numbers,
		span.page-numbers {
			min-width: 40px;
			min-height: 40px;
		}

		button.page-numbers:hover {
			background: #0984ae;
			border-color: #0984ae;
			color: #fff;
		}
	}

	/* md: 768px+ */
	@media (min-width: 768px) {
		button.page-numbers,
		span.page-numbers {
			min-width: 36px;
			height: 36px;
			min-height: 36px;
			border-radius: 6px;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ACCESSIBILITY
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (prefers-reduced-motion: reduce) {
		button.page-numbers,
		span.page-numbers {
			transition: none;
		}
	}

	button.page-numbers:focus-visible {
		outline: 2px solid #0984ae;
		outline-offset: 2px;
	}
</style>

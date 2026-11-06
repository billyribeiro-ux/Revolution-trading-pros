<script lang="ts">
	import type { readPublicVideos } from '#lib/server/seo-publications.js';
	let {
		videos,
		title = 'Public Video Learning Center'
	}: { videos: Awaited<ReturnType<typeof readPublicVideos>>; title?: string } = $props();
</script>

<section class="video-hub">
	<h1>{title}</h1>
	<p>
		Educational trading videos from our published public library. Trading involves a risk of loss;
		this material is not personalized investment advice.
	</p>
	{#if !videos.data.length}<p>No public learning videos are currently published.</p>{:else}<div
			class="cards"
		>
			{#each videos.data as video (video.id)}<article>
					<a href={video.path}
						><img
							src={video.thumbnail_url}
							alt="Thumbnail for {video.title}"
							width="640"
							height="360"
							loading="lazy"
						/>
						<h2>{video.title}</h2></a
					>
					<p>{video.description.slice(0, 160)}</p>
					<p>
						By <a href="/authors/{video.author_slug}">{video.author_name}</a> ·
						<time datetime={video.published_at}>{video.published_at.slice(0, 10)}</time>
						· {video.duration} seconds
					</p>
				</article>{/each}
		</div>{/if}
	<nav aria-label="Public video pages">
		{#if videos.meta.current_page > 1}<a href="?page={videos.meta.current_page - 1}"
				>Previous videos</a
			>{/if}<span>Page {videos.meta.current_page} of {videos.meta.last_page}</span
		>{#if videos.meta.current_page < videos.meta.last_page}<a
				href="?page={videos.meta.current_page + 1}">Next videos</a
			>{/if}
	</nav>
</section>

<style>
	.video-hub {
		max-width: 1200px;
		margin: 2rem auto;
		padding: 1.5rem;
	}
	.cards {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(min(100%, 280px), 1fr));
		gap: 1.5rem;
	}
	article {
		border: 1px solid #64748b;
		border-radius: 0.75rem;
		padding: 1rem;
	}
	img {
		width: 100%;
		height: auto;
		aspect-ratio: 16 / 9;
		object-fit: cover;
	}
	a {
		color: #60a5fa;
	}
	nav {
		display: flex;
		justify-content: space-between;
		flex-wrap: wrap;
		gap: 1rem;
		margin-top: 2rem;
	}
</style>

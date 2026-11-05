<script lang="ts">
	import type { PageProps } from './$types';
	let { data }: PageProps = $props();
	const video = $derived(data.video);
	function date(value: string) {
		return new Date(value).toLocaleString('en-US', {
			timeZone: 'UTC',
			dateStyle: 'long',
			timeStyle: 'short'
		});
	}
</script>

<article class="watch-page">
	<h1>{video.title}</h1>
	<p>
		By <a href="/authors/{video.author_slug}">{video.author_name}</a> · Published
		<time datetime={video.published_at}>{date(video.published_at)} UTC</time>
		· {Math.floor(video.duration / 60)}m {video.duration % 60}s
	</p>
	<div class="player">
		{#if video.playback_kind === 'video'}<video
				controls
				muted
				preload="metadata"
				poster={video.thumbnail_url}
				width="1280"
				height="720"
				><source src={video.playback_url} />
				<p>
					<a href={video.playback_url} rel="noopener noreferrer">Open {video.title} video</a>
				</p></video
			>{:else}<iframe
				src={video.playback_url}
				title={video.title}
				allow="fullscreen; picture-in-picture"
				allowfullscreen
				width="1280"
				height="720"
			></iframe>{/if}
	</div>
	<p class="description">{video.description}</p>
	<details>
		<summary>Video thumbnail</summary><img
			src={video.thumbnail_url}
			alt="Thumbnail for {video.title}"
			width="1280"
			height="720"
			loading="lazy"
		/>
	</details>
	<aside>
		<h2>Financial risk disclosure</h2>
		<p>
			This material is for education. Trading involves a risk of loss. It does not constitute
			personalized investment advice or a guarantee of future results.
		</p>
	</aside>
	<a href="/learning-center">Browse public learning videos</a>
</article>

<style>
	.watch-page {
		max-width: 1000px;
		margin: 2rem auto;
		padding: 1.5rem;
	}
	h1 {
		font-size: clamp(2rem, 5vw, 3.5rem);
	}
	.player {
		aspect-ratio: 16 / 9;
		margin: 2rem 0;
		background: #090e1b;
	}
	video,
	iframe {
		width: 100%;
		height: 100%;
		border: 0;
	}
	img {
		max-width: 100%;
		height: auto;
	}
	.description {
		white-space: pre-wrap;
		line-height: 1.7;
	}
	aside {
		border: 1px solid #64748b;
		padding: 1rem;
		margin: 2rem 0;
	}
	a {
		color: #60a5fa;
	}
</style>

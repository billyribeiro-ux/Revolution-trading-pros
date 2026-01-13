<!--
	Watchlist Rundown Archive - Pixel-Perfect WordPress Match
	═══════════════════════════════════════════════════════════════════════════
	
	Apple ICT 11+ Principal Engineer Grade - January 2026
	Matches: frontend/Implementation/wwl-Rundown-Archive
	
	Svelte 5 / SvelteKit 2.0 Best Practices:
	- $props() rune for component props
	- $derived() rune for reactive computed values
	- Type imports from server load function
	- Proper empty state handling
	
	@version 1.1.0 - January 2026
-->
<script lang="ts">
	import type { WatchlistVideo } from './+page.server';

	interface Props {
		data: {
			videos: WatchlistVideo[];
		};
	}

	let { data }: Props = $props();

	let videos = $derived(data.videos ?? []);
	let hasVideos = $derived(videos.length > 0);
</script>

<svelte:head>
	<title>Watchlist Rundown Archive - Revolution Trading Pros</title>
</svelte:head>

<div class="dashboard__content-section">
	<section>
		<h2 class="section-title">Watchlist Rundown Archive</h2>
		{#if hasVideos}
		<div class="video-grid">
			{#each videos as { id, title, date, description, image, href } (id)}
				<div class="video-grid__item">
					<article class="video-card">
						<figure class="video-card__figure">
							<div class="video-card__thumbnail" style="background-image: url({image});">
								<a {href}>
									<img 
										src="https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg" 
										alt={title}
									/>
								</a>
							</div>
							<div class="video-card__content">
								<div>
									<h4 class="video-card__title">
										<a {href}>{title}</a>
									</h4>
									<div class="video-card__date">
										<span><i>{date}</i></span><br />
									</div>
									<p class="video-card__description">{description}</p>
								</div>
								<a {href} class="video-card__link">Watch Now</a>
							</div>
						</figure>
					</article>
				</div>
			{/each}
		</div>
		{:else}
		<div class="empty-state">
			<p>No watchlist rundown videos available at this time.</p>
		</div>
		{/if}
	</section>
</div>

<style>
	/* Video Grid - Semantic class names */
	.section-title {
		font-size: 1.75rem;
		font-weight: 600;
		margin-bottom: 2rem;
		color: #1a1a1a;
	}

	.video-grid {
		display: flex;
		flex-wrap: wrap;
		margin-left: -15px;
		margin-right: -15px;
	}

	.video-grid__item {
		padding-left: 15px;
		padding-right: 15px;
		margin-bottom: 30px;
		width: 100%;
	}

	@media (min-width: 576px) {
		.video-grid__item {
			width: 50%;
		}
	}

	@media (min-width: 1200px) {
		.video-grid__item {
			width: 33.333333%;
		}
	}

	.video-card {
		display: flex;
		flex-direction: column;
		height: 100%;
		background: #fff;
		border-radius: 8px;
		overflow: hidden;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
		transition: box-shadow 0.3s ease;
	}

	.video-card:hover {
		box-shadow: 0 4px 16px rgba(0, 0, 0, 0.15);
	}

	.video-card__figure {
		margin: 0;
		display: flex;
		flex-direction: column;
		height: 100%;
	}

	.video-card__thumbnail {
		position: relative;
		width: 100%;
		padding-bottom: 56.25%;
		background-size: cover;
		background-position: center;
		overflow: hidden;
	}

	.video-card__thumbnail a {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		display: block;
	}

	.video-card__thumbnail img {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		object-fit: cover;
		opacity: 0;
		transition: opacity 0.3s ease;
	}

	.video-card__thumbnail:hover img {
		opacity: 0.1;
	}

	.video-card__content {
		padding: 20px;
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		flex: 1;
	}

	.video-card__title {
		font-size: 1.125rem;
		font-weight: 600;
		margin: 0 0 10px 0;
		line-height: 1.4;
	}

	.video-card__title a {
		color: #1a1a1a;
		text-decoration: none;
		transition: color 0.2s ease;
	}

	.video-card__title a:hover {
		color: #143E59;
	}

	.video-card__date {
		margin-bottom: 20px;
	}

	.video-card__date span i {
		font-style: italic;
		color: #666;
		font-size: 0.875rem;
	}

	.video-card__description {
		font-size: 0.9375rem;
		line-height: 1.6;
		color: #666;
		margin: 0 0 20px;
	}

	.video-card__link {
		font-weight: 600;
		text-decoration: none;
		font-size: 0.9375rem;
		color: #f7941d;
		transition: color 0.2s ease;
		align-self: flex-start;
	}

	.video-card__link:hover {
		color: #c67000;
	}

	.empty-state {
		padding: 40px;
		text-align: center;
		color: #666;
	}
</style>

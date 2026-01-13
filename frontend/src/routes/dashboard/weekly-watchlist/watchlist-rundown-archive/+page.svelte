<!--
	Watchlist Rundown Archive - Pixel-Perfect WordPress Match
	═══════════════════════════════════════════════════════════════════════════
	
	Apple ICT 11+ Principal Engineer Grade - January 2026
	Matches: frontend/Implementation/wwl-Rundown-Archive
	
	@version 1.0.0 - January 2026
-->
<script lang="ts">
	interface WatchlistVideo {
		id: string;
		slug: string;
		title: string;
		date: string;
		weekOf: string;
		image: string;
		href: string;
		description: string;
	}

	interface PageData {
		videos: WatchlistVideo[];
	}

	let { data }: { data: PageData } = $props();

	let videos = $derived(data.videos || []);
</script>

<svelte:head>
	<title>Watchlist Rundown Archive - Revolution Trading Pros</title>
</svelte:head>

<div class="dashboard__content-section">
	<div class="">
		<section>
			<h2 class="section-title">Watchlist Rundown Archive</h2>
			<div class="article-cards row flex-grid">
				{#each videos as video}
					<div class="col-xs-12 col-sm-6 col-md-6 col-xl-4 flex-grid-item">
						<article class="article-card">
							<figure class="weekly_watchlist">
								<div class="article-card__image" style="background-image: url({video.image});">
									<a href={video.href}>
										<img 
											src="https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg" 
											alt={video.title}
										/>
									</a>
								</div>

								<div class="card_content u--margin-top-0">
									<div>
										<h4 class="h5 article-card__title">
											<a href={video.href}>{video.title}</a>
										</h4>
										<div class="u--margin-top-0 u--margin-bottom-20">
											<span class=""><i>{video.date}</i></span><br />
										</div>
										<p class="u--margin-bottom-20">{video.description}</p>
									</div>
									<a class="" href={video.href} style="color: #f7941d;">Watch Now</a>
								</div>
							</figure>
						</article>
					</div>
				{/each}
			</div>
		</section>
	</div>
</div>

<style>
	/* Watchlist Rundown Archive Styles - Matching WordPress Reference */
	
	.section-title {
		font-size: 1.75rem;
		font-weight: 600;
		margin-bottom: 2rem;
		color: #1a1a1a;
	}

	.article-cards {
		display: flex;
		flex-wrap: wrap;
		margin-left: -15px;
		margin-right: -15px;
	}

	.flex-grid {
		display: flex;
		flex-wrap: wrap;
	}

	.flex-grid-item {
		padding-left: 15px;
		padding-right: 15px;
		margin-bottom: 30px;
	}

	.col-xs-12 {
		width: 100%;
	}

	@media (min-width: 576px) {
		.col-sm-6 {
			width: 50%;
		}
	}

	@media (min-width: 768px) {
		.col-md-6 {
			width: 50%;
		}
	}

	@media (min-width: 1200px) {
		.col-xl-4 {
			width: 33.333333%;
		}
	}

	.article-card {
		display: flex;
		flex-direction: column;
		height: 100%;
		background: #fff;
		border-radius: 8px;
		overflow: hidden;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
		transition: box-shadow 0.3s ease;
	}

	.article-card:hover {
		box-shadow: 0 4px 16px rgba(0, 0, 0, 0.15);
	}

	.weekly_watchlist {
		margin: 0;
		display: flex;
		flex-direction: column;
		height: 100%;
	}

	.article-card__image {
		position: relative;
		width: 100%;
		padding-bottom: 56.25%; /* 16:9 aspect ratio */
		background-size: cover;
		background-position: center;
		overflow: hidden;
	}

	.article-card__image a {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		display: block;
	}

	.article-card__image img {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		object-fit: cover;
		opacity: 0;
		transition: opacity 0.3s ease;
	}

	.article-card__image:hover img {
		opacity: 0.1;
	}

	.card_content {
		padding: 20px;
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		flex: 1;
	}

	.u--margin-top-0 {
		margin-top: 0 !important;
	}

	.u--margin-bottom-20 {
		margin-bottom: 20px !important;
	}

	.article-card__title {
		font-size: 1.125rem;
		font-weight: 600;
		margin: 0 0 10px 0;
		line-height: 1.4;
	}

	.article-card__title a {
		color: #1a1a1a;
		text-decoration: none;
		transition: color 0.2s ease;
	}

	.article-card__title a:hover {
		color: #f7941d;
	}

	.card_content p {
		font-size: 0.9375rem;
		line-height: 1.6;
		color: #666;
		margin: 0;
	}

	.card_content > a {
		font-weight: 600;
		text-decoration: none;
		font-size: 0.9375rem;
		transition: opacity 0.2s ease;
		align-self: flex-start;
	}

	.card_content > a:hover {
		opacity: 0.8;
	}

	.card_content span i {
		font-style: italic;
		color: #666;
		font-size: 0.875rem;
	}
</style>

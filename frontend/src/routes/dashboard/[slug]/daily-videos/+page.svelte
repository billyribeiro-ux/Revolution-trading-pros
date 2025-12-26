<script lang="ts">
	let { data }: { data: any } = $props();
	const room = $derived(data.room);
	const slug = $derived(data.slug);

	let searchQuery = $state('');
	let currentPage = $state(1);
	const itemsPerPage = 12;

	const allVideos = [
		{ id: 1, title: 'A Strong End Of Year', date: 'December 24, 2025', trader: 'TG', thumbnail: 'https://cdn.simplertrading.com/2025/09/29170752/MTT-TG.jpg', description: 'Christmas week started with the strong turn around.', videoSlug: 'a-strong-end-of-year' },
		{ id: 2, title: "Santa's On His Way", date: 'December 23, 2025', trader: 'HG', thumbnail: 'https://cdn.simplertrading.com/2025/05/07134745/SimplerCentral_HG.jpg', description: "Santa's on his way based on Tuesday's close.", videoSlug: 'santas-on-his-way' },
		{ id: 3, title: 'Setting Up for the Santa Rally', date: 'December 22, 2025', trader: 'Danielle Shay', thumbnail: 'https://cdn.simplertrading.com/2025/05/07134911/SimplerCentral_DShay.jpg', description: 'Everything looks good for a potential rally.', videoSlug: 'setting-up-santa-rally' },
		{ id: 4, title: 'Holiday Weekend Market Review', date: 'December 19, 2025', trader: 'Sam', thumbnail: 'https://cdn.simplertrading.com/2025/05/07134553/SimplerCentral_SS.jpg', description: 'Indexes continue to churn sideways.', videoSlug: 'holiday-weekend-market-review' },
		{ id: 5, title: 'Ho Ho Whoa!', date: 'December 18, 2025', trader: 'Bruce Marshall', thumbnail: 'https://cdn.simplertrading.com/2025/04/07135027/SimplerCentral_BM.jpg', description: 'Market action and outlook for end of year.', videoSlug: 'ho-ho-whoa' },
		{ id: 6, title: 'A Moment For The VIX', date: 'December 17, 2025', trader: 'HG', thumbnail: 'https://cdn.simplertrading.com/2025/05/07134745/SimplerCentral_HG.jpg', description: 'VIX expiration and market weakness.', videoSlug: 'a-moment-for-the-vix' }
	];

	let filteredVideos = $derived(searchQuery ? allVideos.filter(v => v.title.toLowerCase().includes(searchQuery.toLowerCase()) || v.trader.toLowerCase().includes(searchQuery.toLowerCase())) : allVideos);
	let totalPages = $derived(Math.ceil(filteredVideos.length / itemsPerPage));
	let paginatedVideos = $derived(filteredVideos.slice((currentPage - 1) * itemsPerPage, currentPage * itemsPerPage));
</script>

<svelte:head><title>Premium Daily Videos - {room.name} | Revolution Trading Pros</title></svelte:head>

<div class="dashboard__content">
	<div class="dashboard__content-main">
		<section class="dashboard__content-section">
			<h2 class="section-title">{room.name} Premium Daily Videos</h2>
			<div class="dashboard-filters">
				<div class="dashboard-filters__count">Showing <span class="facetwp-counts">{filteredVideos.length} results</span></div>
				<div class="dashboard-filters__search">
					<input type="text" placeholder="Search..." bind:value={searchQuery} oninput={() => currentPage = 1} />
				</div>
			</div>
			<div class="card-grid flex-grid row">
				{#each paginatedVideos as video (video.id)}
					<article class="card-grid-spacer flex-grid-item col-xs-12 col-sm-6 col-md-6 col-lg-4">
						<div class="card flex-grid-panel">
							<figure class="card-media card-media--video">
								<a href="/daily/{slug}/{video.videoSlug}" class="card-image" style="background-image: url({video.thumbnail});"><img alt={video.title} /></a>
							</figure>
							<section class="card-body">
								<h4 class="h5 card-title"><a href="/daily/{slug}/{video.videoSlug}">{video.title}</a></h4>
								<span class="article-card__meta"><small>{video.date} with {video.trader}</small></span>
								<div class="card-description"><p>{video.description}</p></div>
							</section>
							<footer class="card-footer"><a class="btn btn-tiny btn-default" href="/daily/{slug}/{video.videoSlug}">Watch Now</a></footer>
						</div>
					</article>
				{/each}
			</div>
			{#if totalPages > 1}
				<div class="facetwp-pagination"><div class="facetwp-pager">
					{#each Array(totalPages) as _, i}<button class="facetwp-page" class:active={currentPage === i + 1} onclick={() => currentPage = i + 1}>{i + 1}</button>{/each}
				</div></div>
			{/if}
		</section>
	</div>
</div>

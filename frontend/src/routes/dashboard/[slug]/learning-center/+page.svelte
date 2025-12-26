<script lang="ts">
	let { data }: { data: any } = $props();
	const room = $derived(data.room);
	const slug = $derived(data.slug);

	let selectedCategory = $state('0');
	const categories = [
		{ id: '529', label: 'Trade Setups & Strategies' },
		{ id: '528', label: 'Methodology' },
		{ id: '329', label: 'Member Webinar' },
		{ id: '531', label: 'Indicators' },
		{ id: '3260', label: 'Options' },
		{ id: '537', label: 'Psychology' }
	];

	const allVideos = [
		{ id: 1, title: 'Q3 Market Cycles Update', trader: 'John Carter', category: '529', categoryLabel: 'Trade Setups & Strategies', thumbnail: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27111943/MemberWebinar-John.jpg', description: 'Key cycle dates and trading plan.', videoSlug: 'q3-market-cycles' },
		{ id: 2, title: 'Understanding Market Indicators', trader: 'Henry Gambell', category: '531', categoryLabel: 'Indicators', thumbnail: 'https://cdn.simplertrading.com/2025/05/07134745/SimplerCentral_HG.jpg', description: 'Deep dive into market indicators.', videoSlug: 'market-indicators' },
		{ id: 3, title: 'Options Trading Fundamentals', trader: 'Danielle Shay', category: '3260', categoryLabel: 'Options', thumbnail: 'https://cdn.simplertrading.com/2025/05/07134911/SimplerCentral_DShay.jpg', description: 'Learn options trading fundamentals.', videoSlug: 'options-fundamentals' },
		{ id: 4, title: 'Psychology of Trading', trader: 'Bruce Marshall', category: '537', categoryLabel: 'Psychology', thumbnail: 'https://cdn.simplertrading.com/2025/04/07135027/SimplerCentral_BM.jpg', description: 'Master trading psychology.', videoSlug: 'trading-psychology' }
	];

	let filteredVideos = $derived(selectedCategory === '0' ? allVideos : allVideos.filter(v => v.category === selectedCategory));
</script>

<svelte:head><title>Learning Center - {room.name} | Revolution Trading Pros</title></svelte:head>

<div class="dashboard__content">
	<div class="dashboard__content-main" style="min-width: 100%">
		<form class="term_filter">
			<div class="reset_filter"><input type="radio" id="0" value="0" name="cat" checked={selectedCategory === '0'} onchange={() => selectedCategory = '0'} /><label for="0">All</label></div>
			{#each categories as cat}<div class="filter_btn"><input type="radio" id={cat.id} value={cat.id} name="cat" checked={selectedCategory === cat.id} onchange={() => selectedCategory = cat.id} /><label for={cat.id}>{cat.label}</label></div>{/each}
		</form>
		<section class="dashboard__content-section">
			<h2 class="section-title">{room.name} Learning Center</h2>
			<div class="article-cards row flex-grid">
				{#each filteredVideos as video (video.id)}
					<div class="col-xs-12 col-sm-6 col-md-6 col-xl-4 flex-grid-item">
						<article class="article-card">
							<figure class="article-card__image" style="background-image: url({video.thumbnail});"><img alt={video.title} /></figure>
							<div class="article-card__type"><span class="label label--info">{video.categoryLabel}</span></div>
							<h4 class="h5 article-card__title"><a href="/learning-center/{video.videoSlug}">{video.title}</a></h4>
							<div class="u--margin-top-0"><span class="trader_name"><i>With {video.trader}</i></span></div>
							<div class="article-card__excerpt"><p>{video.description}</p></div>
							<a href="/learning-center/{video.videoSlug}" class="btn btn-tiny btn-default">Watch Now</a>
						</article>
					</div>
				{/each}
			</div>
		</section>
	</div>
</div>

<style>
	.term_filter { display: flex; flex-wrap: wrap; gap: 8px; padding: 20px; background: #f9f9f9; border-radius: 8px; margin-bottom: 20px; }
	.reset_filter, .filter_btn { display: inline-block; }
	.reset_filter input, .filter_btn input { display: none; }
	.reset_filter label, .filter_btn label { display: inline-block; padding: 8px 16px; font-size: 12px; font-weight: 600; color: #666; background: #fff; border: 1px solid #ddd; border-radius: 4px; cursor: pointer; }
	.reset_filter input:checked + label, .filter_btn input:checked + label { background: #0984ae; color: #fff; border-color: #0984ae; }
</style>

<script lang="ts">
	import type { PageData } from '@sveltejs/kit';
	let { data }: { data: PageData } = $props();
	const { room, slug } = data;

	const archivesByDate = [
		{ date: 'December 5, 2025', sessions: [
			{ id: 1, title: 'Nail the Open', trader: 'John F. Carter', sessionSlug: '12052025' },
			{ id: 2, title: 'Macro Moves', trader: 'Sam Shames', sessionSlug: '12052025' },
			{ id: 3, title: 'Cash In On The Close', trader: 'Allison Ostrander', sessionSlug: '12052025' }
		]},
		{ date: 'December 4, 2025', sessions: [
			{ id: 1, title: 'Nail the Open', trader: 'John F. Carter', sessionSlug: '12042025' },
			{ id: 2, title: 'Swing Trading Options', trader: 'Taylor Horton', sessionSlug: '12042025' },
			{ id: 3, title: 'Cash In On The Close', trader: 'Henry Gambell', sessionSlug: '12042025' }
		]}
	];
</script>

<svelte:head><title>Trading Room Archives - {room.name} | Revolution Trading Pros</title></svelte:head>

<div class="dashboard__content">
	<div class="dashboard__content-main">
		<section class="dashboard__content-section">
			<h2 class="section-title">{room.name} Trading Room Archives</h2>
			{#each archivesByDate as dateGroup}
				<p class="date-header"><strong>{dateGroup.date}</strong></p>
				<div class="card-grid flex-grid row">
					{#each dateGroup.sessions as session, index}
						<article class="flex-grid-item col-xs-12 col-sm-6 col-md-4 col-xl-3">
							<div class="card flex-grid-panel">
								<section class="card-body">
									<h4><strong>{session.title}</strong></h4>
									<p class="trader-name"><i>With {session.trader}</i></p>
									<a class="btn btn-tiny btn-default" href="/chatroom-archive/{slug}/{session.sessionSlug}">Watch Now</a>
								</section>
							</div>
						</article>
					{/each}
				</div>
			{/each}
		</section>
	</div>
</div>

<style>
	.date-header { margin: 20px 0 15px; font-size: 16px; }
	.card-grid { display: flex; flex-wrap: wrap; margin: 0 -10px 20px; }
	.flex-grid-item { padding: 0 10px; margin-bottom: 20px; }
	.col-xs-12 { flex: 0 0 100%; max-width: 100%; }
	@media (min-width: 576px) { .col-sm-6 { flex: 0 0 50%; max-width: 50%; } }
	@media (min-width: 768px) { .col-md-4 { flex: 0 0 33.333%; max-width: 33.333%; } }
	@media (min-width: 1200px) { .col-xl-3 { flex: 0 0 25%; max-width: 25%; } }
	.card { background: #fff; border: 1px solid #dbdbdb; border-radius: 8px; box-shadow: 0 5px 30px rgba(0,0,0,0.1); height: 100%; }
	.card-body { padding: 15px; display: flex; flex-direction: column; gap: 8px; }
	.card-body h4 { margin: 0; font-size: 16px; }
	.trader-name { font-size: 13px; color: #666; margin: 0; }
</style>

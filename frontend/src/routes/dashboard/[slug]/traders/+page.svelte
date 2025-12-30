<script lang="ts">
	import type { PageData } from '@sveltejs/kit';
	import IconArrowLeft from '@tabler/icons-svelte/icons/arrow-left';
	import IconUsers from '@tabler/icons-svelte/icons/users';
	let { data }: { data: PageData } = $props();
	const { room, slug } = data;
</script>

<svelte:head><title>Meet the Traders - {room.name} | Revolution Trading Pros</title></svelte:head>

<div class="traders">
	<a href="/dashboard/{slug}" class="back-link"><IconArrowLeft size={18} /> Back to {room.name}</a>
	<header class="page-header">
		<div class="header-icon"><IconUsers size={32} /></div>
		<div><h1 class="page-title">Meet the Traders</h1><p class="page-subtitle">Learn from our expert traders</p></div>
	</header>
	<div class="traders-grid">
		{#each room.traders as trader}
			<article class="trader-card">
				<div class="trader-image" style="background-image: url({trader.image})"></div>
				<div class="trader-info">
					<h3 class="trader-name">{trader.name}</h3>
					<p class="trader-title">{trader.title}</p>
					<p class="trader-bio">{trader.bio}</p>
					<div class="trader-specialties">{#each trader.specialties as s}<span class="specialty-tag">{s}</span>{/each}</div>
					<a href="/traders/{trader.id}" class="view-profile-btn">View Profile</a>
				</div>
			</article>
		{/each}
	</div>
</div>

<style>
	.traders { padding: 30px; max-width: 1400px; margin: 0 auto; }
	.back-link { display: inline-flex; align-items: center; gap: 8px; color: #0984ae; text-decoration: none; font-size: 14px; margin-bottom: 24px; }
	.page-header { display: flex; align-items: center; gap: 20px; margin-bottom: 40px; }
	.header-icon { display: flex; align-items: center; justify-content: center; width: 64px; height: 64px; background: linear-gradient(135deg, #0984ae 0%, #076787 100%); border-radius: 16px; color: #fff; }
	.page-title { font-size: 28px; font-weight: 700; color: #333; margin: 0 0 4px; }
	.page-subtitle { font-size: 14px; color: #666; margin: 0; }
	.traders-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(380px, 1fr)); gap: 24px; }
	.trader-card { background: #fff; border-radius: 12px; overflow: hidden; box-shadow: 0 2px 5px rgba(0, 0, 0, 0.16); }
	.trader-image { width: 100%; height: 200px; background-size: cover; background-position: center top; background-color: #0984ae; }
	.trader-info { padding: 24px; }
	.trader-name { font-size: 22px; font-weight: 700; color: #333; margin: 0 0 4px; }
	.trader-title { font-size: 14px; color: #0984ae; margin: 0 0 16px; font-weight: 600; }
	.trader-bio { font-size: 14px; color: #666; line-height: 1.6; margin: 0 0 16px; }
	.trader-specialties { display: flex; flex-wrap: wrap; gap: 8px; margin-bottom: 20px; }
	.specialty-tag { padding: 4px 12px; background: #f0f7fa; color: #0984ae; border-radius: 20px; font-size: 12px; font-weight: 600; }
	.view-profile-btn { display: inline-block; padding: 10px 20px; background: #0984ae; color: #fff; text-decoration: none; border-radius: 4px; font-size: 14px; font-weight: 600; }
</style>

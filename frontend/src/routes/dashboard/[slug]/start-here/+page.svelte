<script lang="ts">
	import IconRocket from '@tabler/icons-svelte/icons/rocket';
	
	let { data }: { data: any } = $props();
	const room = $derived(data.room);
	const slug = $derived(data.slug);
</script>

<svelte:head><title>Start Here - {room.name} | Revolution Trading Pros</title></svelte:head>

<div class="start-here">
	<header class="start-header">
		<div class="header-icon"><IconRocket size={40} /></div>
		<h1 class="page-title">Welcome to {room.name}</h1>
		<p class="page-subtitle">New here? Let's get you started on your trading journey.</p>
	</header>
	<div class="steps-grid">
		<div class="step-card"><div class="step-number">1</div><h3>Watch the Tutorial</h3><p>Start with our comprehensive tutorial video.</p><a href="/dashboard/{slug}">Watch Tutorial &rarr;</a></div>
		<div class="step-card"><div class="step-number">2</div><h3>Meet Your Traders</h3><p>Get to know our expert traders.</p><a href="/dashboard/{slug}/traders">Meet the Team &rarr;</a></div>
		<div class="step-card"><div class="step-number">3</div><h3>Explore Learning Center</h3><p>Browse our educational content.</p><a href="/dashboard/{slug}/learning-center">Start Learning &rarr;</a></div>
		<div class="step-card"><div class="step-number">4</div><h3>Join the Community</h3><p>Connect with fellow traders.</p><a href="#community">Join Chat &rarr;</a></div>
	</div>
</div>

<style>
	.start-here { padding: 40px; max-width: 1000px; margin: 0 auto; }
	.start-header { text-align: center; margin-bottom: 50px; }
	.header-icon { display: inline-flex; align-items: center; justify-content: center; width: 80px; height: 80px; background: linear-gradient(135deg, #F69532 0%, #dc7309 100%); border-radius: 20px; color: #fff; margin-bottom: 20px; }
	.page-title { font-size: 32px; font-weight: 700; color: #333; margin: 0 0 12px; }
	.page-subtitle { font-size: 16px; color: #666; max-width: 600px; margin: 0 auto; }
	.steps-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(280px, 1fr)); gap: 24px; }
	.step-card { background: #fff; border-radius: 12px; padding: 30px; box-shadow: 0 2px 5px rgba(0, 0, 0, 0.16); position: relative; }
	.step-number { position: absolute; top: -10px; left: -10px; width: 32px; height: 32px; background: #0984ae; color: #fff; border-radius: 50%; display: flex; align-items: center; justify-content: center; font-size: 14px; font-weight: 700; }
	.step-card h3 { font-size: 18px; font-weight: 700; color: #333; margin: 0 0 12px; }
	.step-card p { font-size: 14px; color: #666; margin: 0 0 16px; }
	.step-card a { color: #0984ae; font-size: 14px; font-weight: 600; text-decoration: none; }
</style>

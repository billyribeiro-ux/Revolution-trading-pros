<script lang="ts">
	import type { PageData } from './$types';
	let { data }: { data: PageData } = $props();
	const indicator = $derived(data.indicator);
</script>

<article class="indicator-overview">
	<header>
		<p class="category">{indicator.category} · {indicator.difficulty}</p>
		<h1>{indicator.name}</h1>
		<p class="description">{indicator.description}</p>
	</header>
	<section aria-labelledby="indicator-use">
		<h2 id="indicator-use">How traders use this indicator</h2>
		<p>{indicator.useCase}</p>
		<ul>
			{#each indicator.features as feature (feature)}
				<li>{feature}</li>
			{/each}
		</ul>
	</section>
	<nav aria-label="Indicator resources">
		<a href="/indicators">Compare all trading indicators</a>
		<a href="/contact">Contact the team about indicators</a>
	</nav>
</article>

<style>
	.indicator-overview {
		width: 100%;
		max-width: var(--rtp-content-max);
		margin-inline: auto;
		padding: 4rem 1.5rem;
		color: var(--rtp-text);
	}
	header {
		max-width: 50rem;
		margin-bottom: 3rem;
	}
	.category {
		color: var(--rtp-text-muted);
		margin-bottom: 1rem;
	}
	h1 {
		font-size: clamp(2rem, 5vw, 3.5rem);
		line-height: 1.15;
		margin-bottom: 1.5rem;
	}
	h2 {
		font-size: 1.75rem;
		margin-bottom: 1rem;
	}
	p,
	li {
		line-height: 1.7;
	}
	.description {
		font-size: 1.125rem;
		color: var(--rtp-text-soft);
	}
	ul {
		padding-inline-start: 1.5rem;
		margin-block: 1.5rem;
		list-style: disc;
	}
	nav {
		display: flex;
		flex-wrap: wrap;
		gap: 1.5rem;
		margin-top: 3rem;
	}
	a {
		color: var(--rtp-blue-bright);
		text-decoration: underline;
		text-underline-offset: 0.2em;
	}
</style>

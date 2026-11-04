<script lang="ts">
	import type { PageProps } from './$types';
	let { data }: PageProps = $props();
</script>

<section class="newsletter-result" aria-labelledby="newsletter-result-title">
	<div class="result-icon" aria-hidden="true">✓</div>
	<h1 id="newsletter-result-title">Newsletter subscription confirmed</h1>
	<p>{data.message}</p>
	<a href="/">Return to the home page</a>
</section>

<style>
	.newsletter-result {
		max-width: 42rem;
		margin: 6rem auto;
		padding: 2rem;
		border: 1px solid #334155;
		border-radius: 1rem;
		background: #0f172a;
		color: #f8fafc;
		text-align: center;
	}
	.result-icon {
		font-size: 2.5rem;
		color: #34d399;
	}
	h1 {
		font-size: 1.75rem;
		margin: 1rem 0;
	}
	p {
		line-height: 1.7;
		color: #cbd5e1;
	}
	a {
		display: inline-block;
		margin-top: 1.5rem;
		color: #7dd3fc;
		text-decoration: underline;
	}
</style>

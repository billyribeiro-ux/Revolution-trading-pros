<script lang="ts">
	import { page } from '$app/state';
</script>

<section class="newsletter-error" aria-labelledby="newsletter-error-title">
	<h1 id="newsletter-error-title">Unable to update your newsletter subscription</h1>
	<p>{page.error?.message ?? 'The subscription service could not verify this link.'}</p>
	<a href="/">Return to the home page</a>
</section>

<style>
	.newsletter-error {
		max-width: 42rem;
		margin: 6rem auto;
		padding: 2rem;
		border: 1px solid #334155;
		border-radius: 1rem;
		background: #0f172a;
		color: #f8fafc;
		text-align: center;
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

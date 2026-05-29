<script lang="ts">
	import type { PageData } from './$types';

	interface Props {
		data: PageData;
	}
	let { data }: Props = $props();
</script>

<article class="author-page">
	<header>
		<h1>{data.author.name}</h1>
		{#if data.author.jobTitle}
			<p class="role">{data.author.jobTitle}</p>
		{/if}
	</header>

	<section class="bio">
		{#if data.author.bio}
			<p>{data.author.bio}</p>
		{:else}
			<p class="bio--placeholder">
				<strong>TODO — bio for {data.author.name}.</strong>
				Replace this placeholder with first-person credentials, track record, and teaching philosophy.
				The bio is the E-E-A-T anchor for every article this author byline appears on.
			</p>
		{/if}
	</section>

	{#if data.author.knowsAbout && data.author.knowsAbout.length}
		<section class="knows-about">
			<h2>Specialties</h2>
			<ul>
				{#each data.author.knowsAbout as topic (topic)}
					<li>{topic}</li>
				{/each}
			</ul>
		</section>
	{/if}
</article>

<style>
	.author-page {
		max-width: 720px;
		margin: 0 auto;
		padding: 4rem 1.5rem;
		line-height: 1.7;
	}
	.author-page h1 {
		font-size: 2.25rem;
		margin-bottom: 0.25rem;
	}
	.role {
		opacity: 0.8;
		margin: 0 0 2rem;
	}
	.bio--placeholder {
		opacity: 0.7;
	}
	.knows-about {
		margin-top: 2.5rem;
	}
	.knows-about ul {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
		list-style: none;
		padding: 0;
	}
	.knows-about li {
		border: 1px solid rgba(255, 255, 255, 0.15);
		border-radius: 999px;
		padding: 0.25rem 0.75rem;
		font-size: 0.875rem;
	}
</style>

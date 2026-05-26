<script lang="ts">
	import type { PageData } from './$types';

	interface Props {
		data: PageData;
	}
	let { data }: Props = $props();
</script>

<article class="team-page">
	<h1>The Team</h1>
	<p class="lead">
		<strong>TODO — human copy.</strong> Replace this lede with your team
		positioning: a sentence about how the team is structured and why it's worth
		trusting with one's trading education.
	</p>

	<ul class="team-grid">
		{#each data.authors as author (author.slug)}
			<li class="team-card">
				<a href="/authors/{author.slug}">
					<h2>{author.name}</h2>
					{#if author.jobTitle}
						<p class="role">{author.jobTitle}</p>
					{/if}
					{#if author.bio}
						<p class="bio">{author.bio}</p>
					{:else}
						<p class="bio bio--placeholder">
							<em>TODO — bio for {author.name}.</em>
						</p>
					{/if}
				</a>
			</li>
		{/each}
	</ul>
</article>

<style>
	.team-page {
		max-width: 960px;
		margin: 0 auto;
		padding: 4rem 1.5rem;
		line-height: 1.6;
	}
	.team-page h1 {
		font-size: 2.25rem;
		margin-bottom: 0.5rem;
	}
	.lead {
		opacity: 0.85;
		margin-bottom: 2.5rem;
	}
	.team-grid {
		list-style: none;
		padding: 0;
		display: grid;
		gap: 1.5rem;
		grid-template-columns: repeat(auto-fit, minmax(260px, 1fr));
	}
	.team-card {
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 0.75rem;
		padding: 1.5rem;
		transition: border-color 200ms ease;
	}
	.team-card:hover {
		border-color: rgba(255, 255, 255, 0.25);
	}
	.team-card a {
		display: block;
		color: inherit;
		text-decoration: none;
	}
	.team-card h2 {
		font-size: 1.25rem;
		margin: 0 0 0.25rem;
	}
	.role {
		opacity: 0.75;
		font-size: 0.95rem;
		margin: 0 0 0.75rem;
	}
	.bio--placeholder {
		opacity: 0.5;
	}
</style>

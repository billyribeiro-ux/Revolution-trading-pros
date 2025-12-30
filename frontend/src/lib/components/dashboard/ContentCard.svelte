<script lang="ts">
	/**
	 * ContentCard - Simple card for courses, classes, indicators
	 * Svelte 5 with $props()
	 */
	interface Props {
		title: string;
		href: string;
		meta?: string;
		buttonText?: string;
	}

	let { title, href, meta, buttonText = 'Watch Now' }: Props = $props();
</script>

<article class="content-card">
	<div class="content-card__body">
		<h4 class="content-card__title">
			<a href={href}>{title}</a>
		</h4>
		{#if meta}
			<p class="content-card__meta"><small>{meta}</small></p>
		{/if}
	</div>
	<footer class="content-card__footer">
		<a class="content-card__button" href={href}>{buttonText}</a>
	</footer>
</article>

<style>
	.content-card {
		display: flex;
		flex-direction: column;
		width: 100%;
		height: 100%;
		background-color: #fff;
		border: 1px solid #dbdbdb;
		border-radius: 4px;
		box-shadow: 0 2px 5px rgba(0, 0, 0, 0.16);
		transition: box-shadow 0.2s ease-in-out;
		overflow: hidden;
	}

	.content-card:hover {
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
	}

	.content-card__body {
		padding: 20px 20px 10px;
		flex: 1;
	}

	.content-card__title {
		font-size: 16px;
		font-weight: 700;
		font-family: 'Open Sans', sans-serif;
		margin: 0 0 8px;
	}

	.content-card__title a {
		color: #333;
		text-decoration: none;
		transition: color 0.15s ease-in-out;
	}

	.content-card__title a:hover {
		color: #0984ae;
	}

	.content-card__meta {
		margin: 0;
		font-size: 14px;
		color: #666;
		font-family: 'Open Sans', sans-serif;
	}

	.content-card__meta small {
		font-size: 13px;
	}

	.content-card__footer {
		padding: 15px 20px;
		border-top: 1px solid #ededed;
	}

	.content-card__button {
		display: inline-block;
		padding: 8px 16px;
		font-size: 13px;
		font-weight: 600;
		font-family: 'Open Sans', sans-serif;
		border-radius: 4px;
		text-decoration: none;
		transition: all 0.15s ease-in-out;
		background: #f4f4f4;
		color: #333;
		border: 1px solid #dbdbdb;
	}

	.content-card__button:hover {
		color: #333;
		background-color: #e6e6e6;
		border-color: #adadad;
	}
</style>

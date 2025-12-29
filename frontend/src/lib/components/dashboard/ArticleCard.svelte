<script lang="ts">
	/**
	 * ArticleCard - Simpler Trading Style
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Svelte 5 component for article/video cards:
	 * - 16:9 aspect ratio image
	 * - Blue label badge (#0984ae)
	 * - Hover lift effect
	 * - Orange "Watch Now" button
	 *
	 * @version 1.0.0 - Svelte 5 with $props()
	 */

	interface Props {
		/** Article title */
		title: string;
		/** URL to article/video */
		href: string;
		/** Image URL */
		image?: string;
		/** Label text (e.g., "Daily Video") */
		label?: string;
		/** Label type for styling */
		labelType?: 'info' | 'success' | 'warning';
		/** Meta text (e.g., date or author) */
		meta?: string;
		/** Excerpt/description */
		excerpt?: string;
		/** Button text */
		buttonText?: string;
		/** Whether content is restricted */
		restricted?: boolean;
	}

	let {
		title,
		href,
		image,
		label,
		labelType = 'info',
		meta,
		excerpt,
		buttonText = 'Watch Now',
		restricted = false
	}: Props = $props();
</script>

<article class="article-card">
	<figure
		class="article-card__image"
		style="background-image: url({image || ''});"
	>
		{#if image}
			<img src={image} alt={title} loading="lazy" />
		{/if}
		{#if label}
			<div class="article-card__type">
				<span class="label label--{labelType}">{label}</span>
			</div>
		{/if}
	</figure>

	<h4 class="article-card__title">
		<a href={href}>{title}</a>
	</h4>

	{#if meta}
		<span class="article-card__meta">
			<small>{meta}</small>
		</span>
	{/if}

	{#if excerpt || restricted}
		<div class="article-card__excerpt">
			{#if restricted}
				<div class="article-card__restricted">
					This content is only available to members.
				</div>
			{:else if excerpt}
				<p>{excerpt}</p>
			{/if}
		</div>
	{/if}

	<a href={href} class="article-card__button">{buttonText}</a>
</article>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   ARTICLE CARD - Exact Simpler Trading Match
	   ═══════════════════════════════════════════════════════════════════════════ */
	.article-card {
		position: relative;
		background-color: #fff;
		border: 1px solid #dbdbdb;
		border-radius: 8px;
		box-shadow: 0 5px 30px rgba(0, 0, 0, 0.1);
		transition: box-shadow 0.2s ease-in-out, transform 0.2s ease-in-out;
		overflow: hidden;
		display: flex;
		flex-direction: column;
		height: 100%;
	}

	.article-card:hover {
		box-shadow: 0 5px 30px rgba(0, 0, 0, 0.15);
		transform: translateY(-2px);
	}

	/* Image - 16:9 aspect ratio */
	.article-card__image {
		position: relative;
		width: 100%;
		padding-top: 56.25%;
		background-size: cover;
		background-position: center;
		background-color: #0984ae;
		margin: 0;
	}

	.article-card__image img {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		object-fit: cover;
		opacity: 0;
	}

	/* Label Badge */
	.article-card__type {
		position: absolute;
		top: 10px;
		left: 10px;
		z-index: 2;
	}

	.label {
		display: inline-block;
		padding: 4px 10px;
		font-size: 10px;
		font-weight: 700;
		font-family: 'Open Sans', sans-serif;
		text-transform: uppercase;
		letter-spacing: 0.5px;
		border-radius: 25px;
	}

	.label--info {
		background-color: #0984ae;
		color: #fff;
	}

	.label--success {
		background-color: #28a745;
		color: #fff;
	}

	.label--warning {
		background-color: #ffc107;
		color: #000;
	}

	/* Title */
	.article-card__title {
		margin: 0;
		padding: 15px 15px 10px;
		font-size: 16px;
		font-weight: 700;
		font-family: 'Open Sans', sans-serif;
		line-height: 1.3;
	}

	.article-card__title a {
		color: #333;
		text-decoration: none;
		transition: color 0.15s ease-in-out;
	}

	.article-card__title a:hover {
		color: #0984ae;
	}

	/* Meta */
	.article-card__meta {
		display: block;
		padding: 0 15px;
		color: #999;
		font-size: 12px;
		font-family: 'Open Sans', sans-serif;
	}

	/* Excerpt */
	.article-card__excerpt {
		padding: 10px 15px;
		font-size: 14px;
		line-height: 1.5;
		color: #666;
		font-family: 'Open Sans', sans-serif;
		flex: 1;
	}

	.article-card__excerpt p {
		margin: 0;
	}

	/* Restricted Message */
	.article-card__restricted {
		background: #f8f9fa;
		border-left: 4px solid #0984ae;
		padding: 12px 16px;
		color: #666;
		font-size: 13px;
		border-radius: 3px;
	}

	/* Button */
	.article-card__button {
		display: inline-block;
		margin: 0 15px 15px;
		background: transparent;
		color: #F3911B;
		font-size: 17px;
		font-weight: 700;
		font-family: 'Open Sans', sans-serif;
		text-decoration: none;
		padding: 5px 0;
		border: none;
		transition: all 0.15s ease-in-out;
	}

	.article-card__button:hover {
		background: #e7e7e7;
		padding-left: 8px;
		padding-right: 8px;
	}
</style>

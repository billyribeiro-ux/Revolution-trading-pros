<script lang="ts">
	/**
	 * WeeklyWatchlistSection - Simpler Trading Style
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Svelte 5 component for the featured Weekly Watchlist section:
	 * - Blue section title (#0984ae)
	 * - Two-column layout (text + image)
	 * - Orange "Watch Now" button
	 *
	 * @version 1.0.0 - Svelte 5 with $props()
	 */

	interface Props {
		/** Section title */
		title?: string;
		/** Featured title */
		featuredTitle?: string;
		/** Description text */
		description?: string;
		/** Image URL */
		image?: string;
		/** Image alt text */
		imageAlt?: string;
		/** Link URL */
		href?: string;
		/** Button text */
		buttonText?: string;
	}

	let {
		title = 'Weekly Watchlist',
		featuredTitle = 'Weekly Watchlist with TG Watkins',
		description = '',
		image = 'https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/TG-Watchlist-Rundown.jpg',
		imageAlt = 'Weekly Watchlist',
		href = '/watchlist/latest',
		buttonText = 'Watch Now'
	}: Props = $props();
</script>

<section class="watchlist-section">
	<div class="watchlist-section__row">
		<div class="watchlist-section__content">
			<h2 class="watchlist-section__title">{title}</h2>

			<!-- Mobile Image -->
			<div class="watchlist-section__image-mobile">
				<a href={href}>
					<img src={image} alt={imageAlt} loading="lazy" />
				</a>
			</div>

			<h4 class="watchlist-section__featured-title">{featuredTitle}</h4>

			{#if description}
				<p class="watchlist-section__description">{description}</p>
			{/if}

			<a href={href} class="watchlist-section__button">{buttonText}</a>
		</div>

		<div class="watchlist-section__image-desktop">
			<a href={href}>
				<img src={image} alt={imageAlt} loading="lazy" />
			</a>
		</div>
	</div>
</section>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   WEEKLY WATCHLIST SECTION - Exact Simpler Trading Match
	   ═══════════════════════════════════════════════════════════════════════════ */
	.watchlist-section {
		padding: 40px;
		background-color: #fff;
	}

	.watchlist-section__row {
		display: flex;
		flex-wrap: wrap;
		margin: -15px;
	}

	.watchlist-section__content {
		flex: 0 0 100%;
		padding: 15px;
	}

	@media (min-width: 576px) {
		.watchlist-section__content {
			flex: 0 0 50%;
		}
	}

	@media (min-width: 992px) {
		.watchlist-section__content {
			flex: 0 0 41.666667%;
		}
	}

	/* Blue Section Title with Underline */
	.watchlist-section__title {
		color: #0984ae;
		font-size: 11px;
		font-weight: 700;
		font-family: 'Open Sans', sans-serif;
		text-transform: uppercase;
		letter-spacing: 1px;
		margin: 0 0 15px;
		padding-bottom: 30px;
		position: relative;
	}

	.watchlist-section__title::after {
		content: "";
		position: absolute;
		bottom: 2px;
		left: 0;
		width: 50px;
		height: 2px;
		background-color: #e8e8e8;
	}

	/* Featured Title */
	.watchlist-section__featured-title {
		font-size: 16px;
		font-weight: 700;
		font-family: 'Open Sans', sans-serif;
		color: #333;
		margin: 0 0 10px;
	}

	/* Description */
	.watchlist-section__description {
		font-size: 14px;
		color: #666;
		font-family: 'Open Sans', sans-serif;
		margin: 0 0 15px;
		line-height: 1.5;
	}

	/* Button - Orange "Watch Now" */
	.watchlist-section__button {
		display: inline-block;
		background: transparent;
		color: #F3911B;
		font-size: 17px;
		font-weight: 700;
		font-family: 'Open Sans', sans-serif;
		text-decoration: none;
		padding: 5px 0;
		transition: all 0.15s ease-in-out;
	}

	.watchlist-section__button:hover {
		background: #e7e7e7;
		padding-left: 8px;
		padding-right: 8px;
	}

	/* Desktop Image */
	.watchlist-section__image-desktop {
		display: none;
		padding: 15px;
	}

	@media (min-width: 992px) {
		.watchlist-section__image-desktop {
			display: block;
			flex: 0 0 58.333333%;
		}
	}

	.watchlist-section__image-desktop img,
	.watchlist-section__image-mobile img {
		width: 100%;
		height: auto;
		border-radius: 8px;
		display: block;
	}

	/* Mobile Image */
	.watchlist-section__image-mobile {
		display: block;
		margin-bottom: 15px;
	}

	@media (min-width: 992px) {
		.watchlist-section__image-mobile {
			display: none;
		}
	}
</style>

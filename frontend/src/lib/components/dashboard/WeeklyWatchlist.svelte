<!--
	Weekly Watchlist Component
	═══════════════════════════════════════════════════════════════════════════
	
	Apple ICT 11+ Principal Engineer Grade - January 2026
	
	Reusable Weekly Watchlist section that appears across multiple dashboard pages.
	Matches WordPress styling exactly with proper color inheritance.
	
	@version 1.0.0
-->
<script lang="ts">
	interface Props {
		href?: string;
		imageUrl?: string;
		weekOf?: string;
		className?: string;
	}

	let { 
		href = '/dashboard/day-trading-room/weekly-watchlist',
		imageUrl = 'https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/David-Watchlist-Rundown.jpg',
		weekOf = 'December 29, 2025',
		className = ''
	}: Props = $props();
</script>

<div class="weekly-watchlist-section {className}">
	<div class="row">
		<div class="col-left">
			<h2 class="section-title-alt section-title-alt--underline">Weekly Watchlist</h2>
			<div class="mobile-image">
				<a href={href}>
					<img src={imageUrl} alt="Weekly Watchlist" class="u--border-radius" />
				</a>
			</div>
			<h4 class="h5 u--font-weight-bold">Weekly Watchlist with David Starr</h4>
			<div class="u--hide-read-more">
				<p>Week of {weekOf}.</p>
			</div>
			<a href={href} class="btn btn-tiny btn-default">Watch Now</a>
		</div>
		<div class="col-right desktop-only">
			<a href={href}>
				<img src={imageUrl} alt="Weekly Watchlist" class="u--border-radius" />
			</a>
		</div>
	</div>
</div>

<style>
	/* Weekly Watchlist Section */
	.weekly-watchlist-section {
		margin-top: 40px;
		padding-top: 40px;
		border-top: 1px solid #e6e6e6;
	}

	.row {
		display: flex;
		flex-wrap: wrap;
		margin: 0 -15px;
	}

	.col-left {
		flex: 0 0 100%;
		max-width: 100%;
		padding: 0 15px;
		margin-bottom: 20px;
	}

	.col-right {
		flex: 0 0 100%;
		max-width: 100%;
		padding: 0 15px;
		display: none;
	}

	.section-title-alt {
		font-size: 24px;
		font-weight: 700;
		color: #333;
		margin: 0 0 20px;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
	}

	.section-title-alt--underline {
		padding-bottom: 15px;
		border-bottom: 3px solid #F69532;
	}

	.h5 {
		font-size: 18px;
		font-weight: 700;
		color: #333;
		margin: 0 0 10px;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
	}

	.u--font-weight-bold {
		font-weight: 700;
	}

	.u--border-radius {
		border-radius: 8px;
		width: 100%;
		height: auto;
	}

	.u--hide-read-more p {
		color: #666;
		font-size: 14px;
		margin: 0 0 15px;
	}

	.mobile-image {
		display: block;
		margin-bottom: 15px;
	}

	.desktop-only {
		display: none;
	}

	.btn {
		display: inline-block;
		padding: 10px 20px;
		font-size: 14px;
		font-weight: 600;
		text-align: center;
		text-decoration: none;
		border-radius: 4px;
		transition: all 0.2s ease;
		cursor: pointer;
	}

	.btn-default {
		background: #143E59;
		color: #fff;
		border: 1px solid #143E59;
	}

	.btn-default:hover {
		background: #0c2638;
		border-color: #0c2638;
	}

	.btn-tiny {
		padding: 8px 16px;
		font-size: 13px;
	}

	/* Tablet breakpoint */
	@media (min-width: 768px) {
		.col-left {
			flex: 0 0 50%;
			max-width: 50%;
			margin-bottom: 0;
		}

		.col-right {
			flex: 0 0 50%;
			max-width: 50%;
		}

		.mobile-image {
			display: none;
		}

		.desktop-only {
			display: block;
		}
	}

	/* Desktop breakpoint */
	@media (min-width: 992px) {
		.col-left {
			flex: 0 0 41.666667%;
			max-width: 41.666667%;
		}

		.col-right {
			flex: 0 0 58.333333%;
			max-width: 58.333333%;
		}
	}
</style>

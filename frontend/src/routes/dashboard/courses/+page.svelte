<script lang="ts">
	/**
	 * Dashboard - My Classes Page - Simpler Trading EXACT
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Shows user's enrolled classes in a simple card grid.
	 * Matches Simpler Trading's My Classes page exactly.
	 *
	 * @version 4.0.0 (Simpler Trading Exact / December 2025)
	 */

	// ═══════════════════════════════════════════════════════════════════════════
	// ENROLLED CLASSES (Mock data - will be fetched from API)
	// ═══════════════════════════════════════════════════════════════════════════

	interface EnrolledClass {
		id: number;
		title: string;
		instructor: string;
		date: string;
		slug: string;
	}

	const enrolledClasses: EnrolledClass[] = [
		{
			id: 1,
			title: 'Quickstart to Precision Trading E-Learning Module',
			instructor: 'TG Watkins',
			date: 'February 2022',
			slug: 'quickstart-precision-trading-elearning-c'
		},
		{
			id: 2,
			title: 'Quickstart To Precision Trading',
			instructor: 'TG Watkins',
			date: 'April 2020',
			slug: 'quickstart-precision-trading-c'
		}
	];
</script>

<!-- ═══════════════════════════════════════════════════════════════════════════
     HEAD
     ═══════════════════════════════════════════════════════════════════════════ -->

<svelte:head>
	<title>My Classes | Dashboard | Revolution Trading Pros</title>
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<!-- ═══════════════════════════════════════════════════════════════════════════
     DASHBOARD HEADER - WordPress: .dashboard__header
     ═══════════════════════════════════════════════════════════════════════════ -->

<header class="dashboard__header">
	<div class="dashboard__header-left">
		<h1 class="dashboard__page-title">My Classes</h1>
	</div>
</header>

<!-- ═══════════════════════════════════════════════════════════════════════════
     DASHBOARD CONTENT - WordPress: .dashboard__content
     ═══════════════════════════════════════════════════════════════════════════ -->

<div class="dashboard__content">
	<div class="dashboard__content-main">
		<section class="dashboard__content-section">
			{#if enrolledClasses.length > 0}
				<!-- Card Grid - WordPress: .card-grid.flex-grid.row -->
				<div class="card-grid flex-grid row">
					{#each enrolledClasses as classItem (classItem.id)}
						<article class="card-grid-spacer flex-grid-item col-xs-12 col-sm-6 col-md-6 col-lg-4">
							<div class="card flex-grid-panel">
								<section class="card-body u--squash">
									<h4 class="h5 card-title pb-1">
										<a href="/classes/{classItem.slug}">
											{classItem.title}
										</a>
									</h4>
									<p class="article-card__meta">
										<small>{classItem.date} with {classItem.instructor}</small>
									</p>
								</section>
								<footer class="card-footer">
									<a class="btn btn-tiny btn-default" href="/classes/{classItem.slug}">Watch Now</a>
								</footer>
							</div>
						</article>
					{/each}
				</div>
			{:else}
				<!-- Empty State -->
				<div class="empty-state">
					<p>You don't have any Classes.</p>
					<a href="/courses" class="btn btn-orange">See All Classes</a>
				</div>
			{/if}
		</section>
	</div>
</div>

<!-- ═══════════════════════════════════════════════════════════════════════════
     STYLES - Simpler Trading EXACT CSS
     ═══════════════════════════════════════════════════════════════════════════ -->

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD HEADER - WordPress: .dashboard__header
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__header {
		background-color: #fff;
		border-bottom: 1px solid #dbdbdb;
		padding: 20px 30px;
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.dashboard__header-left {
		display: flex;
		align-items: center;
	}

	.dashboard__page-title {
		color: #333;
		font-family: 'Open Sans Condensed', 'Open Sans', sans-serif;
		font-size: 36px;
		font-weight: 700;
		margin: 0;
		line-height: 1.2;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD CONTENT - WordPress: .dashboard__content
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__content {
		padding: 30px;
		background: #fff;
		min-height: calc(100vh - 80px);
	}

	.dashboard__content-main {
		width: 100%;
	}

	.dashboard__content-section {
		/* No additional styling needed */
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   CARD GRID - WordPress/Bootstrap Grid
	   ═══════════════════════════════════════════════════════════════════════════ */

	.card-grid {
		display: flex;
		flex-wrap: wrap;
		margin-right: -15px;
		margin-left: -15px;
	}

	.card-grid-spacer {
		position: relative;
		width: 100%;
		padding-right: 15px;
		padding-left: 15px;
		margin-bottom: 30px;
	}

	@media (min-width: 576px) {
		.col-sm-6 {
			flex: 0 0 50%;
			max-width: 50%;
		}
	}

	@media (min-width: 992px) {
		.col-lg-4 {
			flex: 0 0 33.333333%;
			max-width: 33.333333%;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   CARD - Simpler Trading Style
	   ═══════════════════════════════════════════════════════════════════════════ */

	.card {
		background: #fff;
		border: 1px solid #e5e5e5;
		border-radius: 4px;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.08);
		height: 100%;
		display: flex;
		flex-direction: column;
		transition: box-shadow 0.15s ease;
	}

	.card:hover {
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.12);
	}

	.card-body {
		padding: 20px;
		flex: 1;
	}

	.u--squash {
		padding-bottom: 10px;
	}

	.card-title {
		font-size: 16px;
		font-weight: 600;
		margin: 0 0 8px;
		line-height: 1.4;
	}

	.card-title a {
		color: #333;
		text-decoration: none;
		transition: color 0.15s ease;
	}

	.card-title a:hover {
		color: #0984ae;
	}

	.pb-1 {
		padding-bottom: 4px;
	}

	.article-card__meta {
		margin: 0;
		color: #666;
	}

	.article-card__meta small {
		font-size: 13px;
	}

	/* Card Footer */
	.card-footer {
		padding: 15px 20px;
		border-top: 1px solid #e5e5e5;
		background: #fafafa;
	}

	/* Buttons */
	.btn {
		display: inline-block;
		padding: 8px 16px;
		font-size: 13px;
		font-weight: 600;
		text-decoration: none;
		border-radius: 4px;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.btn-tiny {
		padding: 6px 12px;
		font-size: 12px;
	}

	.btn-default {
		background: transparent;
		border: 1px solid #0984ae;
		color: #0984ae;
	}

	.btn-default:hover {
		background: #0984ae;
		color: #fff;
	}

	.btn-orange {
		background: #f99e31;
		border: 1px solid #f99e31;
		color: #fff;
	}

	.btn-orange:hover {
		background: #dc7309;
		border-color: #dc7309;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   EMPTY STATE
	   ═══════════════════════════════════════════════════════════════════════════ */

	.empty-state {
		text-align: center;
		padding: 60px 20px;
		color: #666;
	}

	.empty-state p {
		font-size: 16px;
		margin: 0 0 20px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (max-width: 768px) {
		.dashboard__header {
			padding: 16px 20px;
		}

		.dashboard__page-title {
			font-size: 28px;
		}

		.dashboard__content {
			padding: 20px;
		}

		.card-grid {
			margin-right: -10px;
			margin-left: -10px;
		}

		.card-grid-spacer {
			padding-right: 10px;
			padding-left: 10px;
			margin-bottom: 20px;
		}
	}
</style>

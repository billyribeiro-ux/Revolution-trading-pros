<!--
	My Classes List Page
	═══════════════════════════════════════════════════════════════════════════
	Pixel-perfect match to WordPress implementation
	Reference: MyClassesY HTML file
	
	Features:
	- Grid layout of available classes
	- Breadcrumb: Home / Member Dashboard / My Classes
	- Page title with dashboard__header wrapper
	- Matches original WordPress structure exactly
-->
<script lang="ts">
	import DashboardBreadcrumbs from '$lib/components/dashboard/DashboardBreadcrumbs.svelte';
	
	interface ClassItem {
		id: number;
		title: string;
		slug: string;
		description?: string;
		thumbnail?: string;
		duration?: string;
		instructor?: string;
		date?: string;
	}
	
	// Mock data - replace with actual API call
	// Set to empty array to show empty state, or populate with classes
	const classes: ClassItem[] = [
		{
			id: 1,
			title: 'Tax Loss Harvest',
			slug: 'tax-loss-harvest-c',
			date: 'December 2025',
			instructor: 'John Carter'
		},
		{
			id: 2,
			title: 'Mastering the Trade University',
			slug: 'mtt-university-c',
			date: 'December 2025',
			instructor: 'John Carter'
		},
		{
			id: 3,
			title: 'Quickstart to Precision Trading E-Learning Module',
			slug: 'quickstart-precision-trading-elearning-c',
			date: 'February 2022',
			instructor: 'TG Watkins'
		},
		{
			id: 4,
			title: 'Quickstart To Precision Trading',
			slug: 'quickstart-precision-trading-c',
			date: 'April 2020',
			instructor: 'TG Watkins'
		}
	];
</script>

<svelte:head>
	<title>My Classes - Simpler Trading</title>
	<meta name="description" content="Access your trading classes and educational content" />
</svelte:head>

<!-- Breadcrumbs -->
<DashboardBreadcrumbs />

<!-- Main Content -->
<div id="page" class="hfeed site grid-parent">
	<div id="content" class="site-content">
		<aside class="dashboard__sidebar">
			<!-- Sidebar content if needed -->
		</aside>
		
		<main class="dashboard__main">
			<!-- Page Header with dashboard__header wrapper (matches WordPress) -->
			<header class="dashboard__header">
				<div class="dashboard__header-left">
					<h1 class="dashboard__page-title">My Classes</h1>
				</div>
			</header>
			
			<!-- Classes Grid or Empty State -->
		{#if classes.length === 0}
			<!-- Empty State -->
			<div class="dashboard__content">
				<div class="dashboard__content-main">
					<section class="dashboard__content-section">
						<div class="empty-state">
							<p class="empty-state__message">You don't have any Classes.</p>
							<a href="/courses" class="btn btn-orange">See All Courses</a>
						</div>
					</section>
				</div>
			</div>
		{:else}
			<!-- Classes Grid -->
			<div class="dashboard__content">
				<div class="dashboard__content-main">
					<section class="dashboard__content-section">
						<div class="card-grid flex-grid row">
							{#each classes as classItem}
								<article class="card-grid-spacer flex-grid-item col-xs-12 col-sm-6 col-md-6 col-lg-4">
									<div class="card flex-grid-panel">
										<section class="card-body u--squash">
											<h4 class="h5 card-title pb-1">
												<a href="/classes/{classItem.slug}">
													{classItem.title}
												</a>
											</h4>
											{#if classItem.date && classItem.instructor}
												<p class="article-card__meta">
													<small>{classItem.date} with {classItem.instructor}</small>
												</p>
											{/if}
										</section>
										<footer class="card-footer">
											<a class="btn btn-tiny btn-default" href="/classes/{classItem.slug}">Watch Now</a>
										</footer>
									</div>
								</article>
							{/each}
						</div>
					</section>
				</div>
			</div>
		{/if}
		</main>
	</div>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * My Classes Page Styles
	 * Matches WordPress implementation exactly
	 * ═══════════════════════════════════════════════════════════════════════════ */
	
	.dashboard__header {
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		justify-content: space-between;
		background: #fff;
		border-bottom: 1px solid #dbdbdb;
		border-right: 1px solid #dbdbdb;
		padding: 20px;
	}
	
	@media (min-width: 1280px) {
		.dashboard__header {
			padding: 30px;
		}
	}

	@media (min-width: 1440px) {
		.dashboard__header {
			padding: 30px 40px;
		}
	}
	
	.dashboard__header-left {
		flex: 1;
	}
	
	.dashboard__page-title {
		font-size: 28px;
		font-weight: 700;
		color: #333;
		margin: 0;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
	}

	/* Empty State - Matches screenshot */
	.empty-state {
		background: #f5f5f5;
		padding: 40px 20px;
		text-align: left;
		min-height: 200px;
	}

	.empty-state__message {
		font-size: 16px;
		color: #333;
		margin: 0 0 20px;
		font-family: 'Open Sans', sans-serif;
	}

	.btn-orange {
		background-color: #ff8c00;
		color: #fff;
		padding: 12px 24px;
		border-radius: 4px;
		text-decoration: none;
		display: inline-block;
		font-weight: 700;
		font-size: 14px;
		border: none;
		cursor: pointer;
		transition: background-color 0.2s ease;
	}

	.btn-orange:hover {
		background-color: #e67e00;
		text-decoration: none;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * Card Grid - WordPress Exact Match
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.card-grid {
		display: flex;
		flex-wrap: wrap;
		margin-left: -15px;
		margin-right: -15px;
	}

	.card-grid-spacer {
		padding-left: 15px;
		padding-right: 15px;
		margin-bottom: 30px;
	}

	.flex-grid-item {
		display: flex;
	}

	.card {
		background: #fff;
		border: 1px solid #e5e5e5;
		border-radius: 5px;
		box-shadow: 0 5px 30px rgba(0, 0, 0, 0.1);
		display: flex;
		flex-direction: column;
		width: 100%;
		transition: all 0.2s ease-in-out;
	}

	.card:hover {
		box-shadow: 0 8px 35px rgba(0, 0, 0, 0.15);
		transform: translateY(-2px);
	}

	.flex-grid-panel {
		display: flex;
		flex-direction: column;
		height: 100%;
	}

	.card-body {
		padding: 20px;
		flex: 1 1 auto;
	}

	.u--squash {
		padding-bottom: 10px;
	}

	.card-title {
		font-size: 18px;
		font-weight: 700;
		color: #333;
		margin: 0;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
		line-height: 1.4;
	}

	.card-title a {
		color: #333;
		text-decoration: none;
		transition: color 0.2s;
	}

	.card-title a:hover {
		color: #143E59;
	}

	.h5 {
		font-size: 18px;
		font-weight: 600;
	}

	.pb-1 {
		padding-bottom: 0.5rem;
	}

	.article-card__meta {
		color: #999;
		font-size: 13px;
		margin: 8px 0 0;
	}

	.article-card__meta small {
		font-size: 13px;
	}

	.card-footer {
		padding: 0 20px 20px;
		margin-top: auto;
	}

	.btn {
		display: inline-block;
		text-decoration: none;
		border-radius: 5px;
		font-weight: 700;
		font-family: 'Open Sans', sans-serif;
		transition: all 0.2s ease-in-out;
		text-align: center;
		cursor: pointer;
		border: none;
	}

	.btn-tiny {
		padding: 10px 24px;
		font-size: 13px;
		min-width: 120px;
	}

	.btn-default {
		background-color: #143E59;
		color: #fff;
		border: 1px solid #143E59;
		box-shadow: none;
	}

	.btn-default:hover {
		background-color: #0f2d41;
		border-color: #0f2d41;
		box-shadow: none;
		text-decoration: none;
	}

	/* Grid System - Bootstrap-like */
	.row {
		display: flex;
		flex-wrap: wrap;
		margin-left: -15px;
		margin-right: -15px;
	}

	.col-xs-12 {
		width: 100%;
	}

	@media (min-width: 576px) {
		.col-sm-6 {
			width: 50%;
		}
	}

	@media (min-width: 768px) {
		.col-md-6 {
			width: 50%;
		}
	}

	@media (min-width: 992px) {
		.col-lg-4 {
			width: 33.333333%;
		}
	}
	
	/* Responsive */
	@media (max-width: 768px) {
		.dashboard__page-title {
			font-size: 24px;
		}

		.card-grid-spacer {
			margin-bottom: 20px;
		}
	}
</style>

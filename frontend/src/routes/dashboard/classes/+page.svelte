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
	const classes: ClassItem[] = [];
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
		justify-content: space-between;
		padding: 20px 0;
		margin-bottom: 30px;
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
	
	/* Responsive */
	@media (max-width: 768px) {
		.dashboard__page-title {
			font-size: 24px;
		}
	}
</style>

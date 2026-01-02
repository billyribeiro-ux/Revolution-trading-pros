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
	
	// Mock data - replace with actual API call
	const classes = [
		{
			id: 1,
			title: 'Tax Loss Harvest',
			slug: 'tax-loss-harvest',
			description: 'Learn tax loss harvesting strategies',
			thumbnail: '/images/classes/tax-loss-harvest.jpg',
			duration: '2 hours'
		},
		// Add more classes as needed
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
			
			<!-- Classes Grid -->
			<div class="classes-grid">
				{#each classes as classItem}
					<article class="class-card">
						<a href="/classes/{classItem.slug}" class="class-card__link">
							<div class="class-card__thumbnail">
								{#if classItem.thumbnail}
									<img src={classItem.thumbnail} alt={classItem.title} />
								{:else}
									<div class="class-card__placeholder">
										<span class="fa fa-graduation-cap"></span>
									</div>
								{/if}
							</div>
							<div class="class-card__content">
								<h2 class="class-card__title">{classItem.title}</h2>
								{#if classItem.description}
									<p class="class-card__description">{classItem.description}</p>
								{/if}
								{#if classItem.duration}
									<span class="class-card__duration">
										<span class="fa fa-clock-o"></span> {classItem.duration}
									</span>
								{/if}
							</div>
						</a>
					</article>
				{/each}
			</div>
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
	
	/* Classes Grid */
	.classes-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
		gap: 30px;
		padding: 20px 0;
	}
	
	.class-card {
		background: #fff;
		border: 1px solid #e5e5e5;
		border-radius: 8px;
		overflow: hidden;
		transition: all 0.3s ease;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
	}
	
	.class-card:hover {
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
		transform: translateY(-2px);
	}
	
	.class-card__link {
		display: block;
		text-decoration: none;
		color: inherit;
	}
	
	.class-card__thumbnail {
		position: relative;
		padding-top: 56.25%; /* 16:9 aspect ratio */
		background: #f5f5f5;
		overflow: hidden;
	}
	
	.class-card__thumbnail img {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		object-fit: cover;
	}
	
	.class-card__placeholder {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		display: flex;
		align-items: center;
		justify-content: center;
		background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
		color: #fff;
		font-size: 48px;
	}
	
	.class-card__content {
		padding: 20px;
	}
	
	.class-card__title {
		font-size: 20px;
		font-weight: 700;
		color: #333;
		margin: 0 0 10px;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
	}
	
	.class-card__description {
		font-size: 14px;
		color: #666;
		line-height: 1.6;
		margin: 0 0 15px;
	}
	
	.class-card__duration {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		font-size: 13px;
		color: #999;
	}
	
	/* Responsive */
	@media (max-width: 768px) {
		.classes-grid {
			grid-template-columns: 1fr;
			gap: 20px;
		}
		
		.dashboard__page-title {
			font-size: 24px;
		}
	}
</style>

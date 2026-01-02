<!--
	Class Detail Page
	═══════════════════════════════════════════════════════════════════════════
	Pixel-perfect match to WordPress implementation
	Reference: MyClassesClicked HTML file
	
	Features:
	- Single class view with video player
	- Breadcrumb: Home / Classes / [Class Name]
	- Page title WITHOUT dashboard__header wrapper (key difference!)
	- Matches original WordPress structure exactly
-->
<script lang="ts">
	import { onMount } from 'svelte';
	import DashboardBreadcrumbs from '$lib/components/dashboard/DashboardBreadcrumbs.svelte';
	
	interface Props {
		data: {
			classData: {
				id: number;
				title: string;
				slug: string;
				description: string;
				videoUrl: string;
				sections: Array<{
					title: string;
					lessons: Array<{
						title: string;
						duration: string;
						videoId: string;
					}>;
				}>;
				metadata: {
					pageType: string;
					contentTitle: string;
				};
			};
		};
	}
	
	let { data }: Props = $props();
	let classData = $derived(data.classData);
	
	// Track page view on mount (matches WordPress richpanel tracking)
	onMount(() => {
		// Track article view event
		if (typeof window !== 'undefined' && (window as any).richpanel) {
			(window as any).richpanel.track('view_article', {
				id: classData.id,
				name: classData.title,
				url: window.location.href
			});
		}
		
		// Track with Google Analytics if available
		if (typeof window !== 'undefined' && (window as any).gtag) {
			(window as any).gtag('event', 'page_view', {
				page_title: classData.title,
				page_location: window.location.href,
				page_path: window.location.pathname,
				content_type: 'article'
			});
		}
	});
</script>

<svelte:head>
	<title>{classData.title} - Simpler Trading</title>
	<meta name="description" content={classData.description} />
	<meta property="og:title" content={classData.title} />
	<meta property="og:url" content="https://www.simplertrading.com/classes/{classData.slug}" />
	<meta property="og:type" content="article" />
</svelte:head>

<!-- Breadcrumbs -->
<DashboardBreadcrumbs />

<!-- Main Content -->
<div id="page" class="hfeed site grid-parent">
	<div id="content" class="site-content">
		<main class="dashboard__main class-detail">
			<!-- Page Title WITHOUT dashboard__header wrapper (key difference from list page!) -->
			<h1>{classData.title}</h1>
			
			<!-- Class Content -->
			<div class="class-content">
				<!-- Video Player Section -->
				<div class="class-player-section">
					<h2 class="class-player-header">{classData.sections[0].title}</h2>
					<div class="class-player-wrapper">
						<!-- Video player placeholder - integrate with actual video player -->
						<div class="video-placeholder">
							<span class="fa fa-play-circle"></span>
							<p>Video Player</p>
						</div>
					</div>
				</div>
				
				<!-- Class Description -->
				{#if classData.description}
					<div class="class-description">
						<p>{classData.description}</p>
					</div>
				{/if}
				
				<!-- Lessons/Modules List -->
				<div class="class-sections">
					{#each classData.sections as section}
						<div class="class-section">
							<h3 class="section-title">{section.title}</h3>
							<ul class="lessons-list">
								{#each section.lessons as lesson}
									<li class="lesson-item">
										<button class="lesson-button" type="button">
											<span class="lesson-icon fa fa-play-circle"></span>
											<span class="lesson-title">{lesson.title}</span>
											<span class="lesson-duration">{lesson.duration}</span>
										</button>
									</li>
								{/each}
							</ul>
						</div>
					{/each}
				</div>
			</div>
		</main>
	</div>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * Class Detail Page Styles
	 * Matches WordPress implementation exactly
	 * ═══════════════════════════════════════════════════════════════════════════ */
	
	.class-detail {
		max-width: 1200px;
		margin: 0 auto;
		padding: 20px;
	}
	
	/* Page Title - NO dashboard__header wrapper on detail pages! */
	.class-detail h1 {
		font-size: 32px;
		font-weight: 700;
		color: #333;
		margin: 0 0 30px;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
		line-height: 1.2;
	}
	
	/* Class Content */
	.class-content {
		background: #fff;
		border-radius: 8px;
		overflow: hidden;
	}
	
	/* Video Player Section */
	.class-player-section {
		margin-bottom: 30px;
	}
	
	.class-player-header {
		font-size: 24px;
		font-weight: 700;
		color: #333;
		margin: 0 0 20px;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
	}
	
	.class-player-wrapper {
		position: relative;
		padding-top: 56.25%; /* 16:9 aspect ratio */
		background: #000;
		border-radius: 8px;
		overflow: hidden;
	}
	
	.video-placeholder {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
		color: #fff;
	}
	
	.video-placeholder .fa {
		font-size: 64px;
		margin-bottom: 10px;
	}
	
	.video-placeholder p {
		font-size: 18px;
		margin: 0;
	}
	
	/* Class Description */
	.class-description {
		padding: 20px 0;
		border-bottom: 1px solid #e5e5e5;
		margin-bottom: 30px;
	}
	
	.class-description p {
		font-size: 16px;
		line-height: 1.6;
		color: #666;
		margin: 0;
	}
	
	/* Lessons/Sections */
	.class-sections {
		margin-top: 30px;
	}
	
	.class-section {
		margin-bottom: 30px;
	}
	
	.section-title {
		font-size: 20px;
		font-weight: 700;
		color: #333;
		margin: 0 0 15px;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
	}
	
	.lessons-list {
		list-style: none;
		margin: 0;
		padding: 0;
	}
	
	.lesson-item {
		border: 1px solid #e5e5e5;
		border-radius: 6px;
		margin-bottom: 10px;
		overflow: hidden;
		transition: all 0.2s ease;
	}
	
	.lesson-item:hover {
		border-color: #667eea;
		box-shadow: 0 2px 8px rgba(102, 126, 234, 0.15);
	}
	
	.lesson-button {
		width: 100%;
		display: flex;
		align-items: center;
		gap: 15px;
		padding: 15px 20px;
		background: #fff;
		border: none;
		cursor: pointer;
		text-align: left;
		transition: background 0.2s ease;
	}
	
	.lesson-button:hover {
		background: #f8f9ff;
	}
	
	.lesson-icon {
		font-size: 24px;
		color: #667eea;
		flex-shrink: 0;
	}
	
	.lesson-title {
		flex: 1;
		font-size: 16px;
		font-weight: 600;
		color: #333;
	}
	
	.lesson-duration {
		font-size: 14px;
		color: #999;
		flex-shrink: 0;
	}
	
	/* Responsive */
	@media (max-width: 768px) {
		.class-detail {
			padding: 15px;
		}
		
		.class-detail h1 {
			font-size: 24px;
			margin-bottom: 20px;
		}
		
		.class-player-header {
			font-size: 20px;
		}
		
		.lesson-button {
			padding: 12px 15px;
			gap: 10px;
		}
		
		.lesson-icon {
			font-size: 20px;
		}
		
		.lesson-title {
			font-size: 14px;
		}
		
		.lesson-duration {
			font-size: 12px;
		}
	}
</style>

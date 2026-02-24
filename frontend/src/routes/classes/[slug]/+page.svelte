<!--
	Class Detail Page
	═══════════════════════════════════════════════════════════════════════════
	Apple ICT 11+ Principal Engineer Grade - January 2026
	
	Features:
	- SSR with SEO meta tags from server load
	- ClassVideos component with Bunny.net integration
	- ClassDownloads component with API integration
	- Hydration-safe patterns
	- Mobile-first responsive design
-->
<script lang="ts">
	import { onMount } from 'svelte';
	import DashboardBreadcrumbs from '$lib/components/dashboard/DashboardBreadcrumbs.svelte';
	import ClassVideos from '$lib/components/ClassVideos.svelte';
	import ClassDownloads from '$lib/components/ClassDownloads.svelte';

	interface Props {
		data: {
			slug: string;
			courseData: unknown;
			seo: {
				title: string;
				description: string;
				ogImage: string;
				ogType: string;
				canonical: string;
			};
			classData: {
				id: number | string;
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
	let seo = $derived(data.seo);
	let slug = $derived(data.slug);
	let mounted = $state(false);

	onMount(() => {
		mounted = true;

		// Track article view event
		if (
			typeof window !== 'undefined' &&
			(window as unknown as { richpanel?: { track: (event: string, data: unknown) => void } })
				.richpanel
		) {
			(
				window as unknown as { richpanel: { track: (event: string, data: unknown) => void } }
			).richpanel.track('view_article', {
				id: classData.id,
				name: classData.title,
				url: window.location.href
			});
		}

		// Track with Google Analytics if available
		if (
			typeof window !== 'undefined' &&
			(window as unknown as { gtag?: (...args: unknown[]) => void }).gtag
		) {
			(window as unknown as { gtag: (...args: unknown[]) => void }).gtag('event', 'page_view', {
				page_title: classData.title,
				page_location: window.location.href,
				page_path: window.location.pathname,
				content_type: 'article'
			});
		}
	});
</script>

<!-- Breadcrumbs -->
<DashboardBreadcrumbs />

<!-- Main Content -->
<div id="page" class="hfeed site grid-parent">
	<div id="content" class="site-content">
		<!-- ICT11+ Fix: Changed from <main> to <div> - root layout provides <main> -->
		<div class="dashboard__main class-detail">
			<!-- Page Title -->
			<h1>{classData.title}</h1>

			<!-- Class Content -->
			<div class="class-content">
				<!-- Video Player Section - API-driven with Bunny.net -->
				<section class="class-videos-section">
					{#if mounted}
						<ClassVideos courseSlug={slug} title="Course Videos" />
					{:else}
						<div class="video-loading-placeholder">
							<div class="spinner"></div>
							<span>Loading videos...</span>
						</div>
					{/if}
				</section>

				<!-- Class Description -->
				{#if classData.description}
					<div class="class-description">
						<p>{classData.description}</p>
					</div>
				{/if}

				<!-- Downloads Section - API-driven -->
				<section class="class-downloads-section">
					<ClassDownloads courseSlug={slug} title="Class Downloads" />
				</section>
			</div>
		</div>
	</div>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * Class Detail Page Styles
	 * Apple ICT 11+ Principal Engineer Grade - January 2026
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.class-detail {
		max-width: 1200px;
		margin: 0 auto;
		padding: 20px;
	}

	.class-detail h1 {
		font-size: 32px;
		font-weight: 700;
		color: #333;
		margin: 0 0 30px;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
		line-height: 1.2;
	}

	.class-content {
		display: flex;
		flex-direction: column;
		gap: 30px;
	}

	/* Sections */
	.class-videos-section,
	.class-downloads-section {
		width: 100%;
	}

	/* Class Description */
	.class-description {
		padding: 20px;
		background: #fff;
		border-radius: 8px;
		border: 1px solid #e0e0e0;
	}

	.class-description p {
		font-size: 16px;
		line-height: 1.6;
		color: #666;
		margin: 0;
	}

	/* Loading placeholder */
	.video-loading-placeholder {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		min-height: 400px;
		background: #1a1a1a;
		border-radius: 8px;
		color: #888;
		gap: 16px;
	}

	.spinner {
		width: 32px;
		height: 32px;
		border: 3px solid rgba(255, 255, 255, 0.2);
		border-top-color: #fff;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
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

		.class-content {
			gap: 20px;
		}
	}
</style>

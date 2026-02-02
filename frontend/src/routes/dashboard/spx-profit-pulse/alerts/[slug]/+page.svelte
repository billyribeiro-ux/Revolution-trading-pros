<script lang="ts">
	/**
	 * SPX Profit Pulse Alert Detail Page
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Apple ICT Level 7 Principal Engineer Implementation
	 * Individual video/alert detail with related content
	 *
	 * @version 1.0.0 - January 2026
	 */
	import DashboardBreadcrumbs from '$lib/components/dashboard/DashboardBreadcrumbs.svelte';
	import RtpIcon from '$lib/components/icons/RtpIcon.svelte';

	// SSR data from +page.server.ts
	let props = $props();
	let data = $derived(props.data);

	const alert = $derived(data.alert);

	// Format duration from seconds to MM:SS
	function formatDuration(seconds: number): string {
		const mins = Math.floor(seconds / 60);
		const secs = seconds % 60;
		return `${mins}:${secs.toString().padStart(2, '0')}`;
	}
</script>

<svelte:head>
	<title>{alert?.title || 'Alert'} | SPX Profit Pulse | Revolution Trading Pros</title>
	<meta
		name="description"
		content={alert?.excerpt || 'SPX Profit Pulse daily video and trade alerts'}
	/>
</svelte:head>

<div class="alert-detail">
	<!-- Breadcrumbs - auto-generated from URL path -->
	<DashboardBreadcrumbs />

	<!-- Main Content -->
	<div class="alert-detail__content">
		<!-- Video Section -->
		<section class="alert-detail__video-section">
			<div class="video-container">
				{#if alert?.video?.url}
					<video controls poster={alert.video.thumbnail} class="video-player" preload="metadata">
						<source src={alert.video.url} type="video/mp4" />
						Your browser doesn't support HTML5 video.
					</video>
				{:else}
					<div class="video-placeholder">
						<RtpIcon name="video" size={48} />
						<p>Video unavailable</p>
					</div>
				{/if}
			</div>

			<!-- Video Info -->
			<div class="video-info">
				<h1 class="video-title">{alert?.title}</h1>
				<div class="video-meta">
					<span class="video-date">
						<RtpIcon name="calendar" size={16} />
						{alert?.date}
					</span>
					{#if alert?.video?.duration}
						<span class="video-duration">
							<RtpIcon name="clock" size={16} />
							{formatDuration(alert.video.duration)}
						</span>
					{/if}
				</div>
			</div>
		</section>

		<!-- Trader Info -->
		{#if alert?.trader}
			<section class="alert-detail__trader">
				<div class="trader-card">
					<div class="trader-photo">
						{#if alert.trader.photo_url}
							<img src={alert.trader.photo_url} alt={alert.trader.name} />
						{:else}
							<div class="trader-avatar">
								{alert.trader.name[0]}
							</div>
						{/if}
					</div>
					<div class="trader-info">
						<h3 class="trader-name">{alert.trader.name}</h3>
						<p class="trader-title">{alert.trader.title}</p>
						<p class="trader-bio">{alert.trader.bio}</p>
						<a href="/dashboard/spx-profit-pulse/{alert.trader.slug}" class="trader-link">
							View All Videos by {alert.trader.name}
							<RtpIcon name="arrow-right" size={14} />
						</a>
					</div>
				</div>
			</section>
		{/if}

		<!-- Description -->
		{#if alert?.description}
			<section class="alert-detail__description">
				<h2>About This Session</h2>
				<p>{alert.description}</p>
			</section>
		{/if}

		<!-- Related Alerts -->
		{#if alert?.related_alerts && alert.related_alerts.length > 0}
			<section class="alert-detail__related">
				<h2>Previous Sessions</h2>
				<div class="related-grid">
					{#each alert.related_alerts as related}
						<a href={related.href} class="related-card">
							<div class="related-image" style="background-image: url({related.image})">
								<div class="related-overlay">
									<RtpIcon name="play" size={32} />
								</div>
							</div>
							<div class="related-info">
								<h4>{related.title}</h4>
								<span class="related-date">{related.date}</span>
							</div>
						</a>
					{/each}
				</div>
			</section>
		{/if}

		<!-- Back Link -->
		<div class="alert-detail__back">
			<a href="/dashboard/spx-profit-pulse" class="back-link">
				<RtpIcon name="arrow-left" size={16} />
				Back to SPX Profit Pulse Dashboard
			</a>
		</div>
	</div>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * ALERT DETAIL PAGE - Apple ICT Level 7 Implementation
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.alert-detail {
		width: 100%;
		max-width: 1200px;
		margin: 0 auto;
		padding: 0;
	}

	.alert-detail__content {
		padding: 30px;
	}

	@media (min-width: 768px) {
		.alert-detail__content {
			padding: 40px;
		}
	}

	/* Video Section */
	.alert-detail__video-section {
		margin-bottom: 40px;
	}

	.video-container {
		width: 100%;
		max-width: 1024px;
		aspect-ratio: 16 / 9;
		background: #000;
		border-radius: 12px;
		overflow: hidden;
		margin-bottom: 20px;
	}

	.video-player {
		width: 100%;
		height: 100%;
		object-fit: contain;
		background: #000;
	}

	.video-placeholder {
		width: 100%;
		height: 100%;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		color: #666;
		gap: 10px;
	}

	.video-info {
		max-width: 1024px;
	}

	.video-title {
		font-size: 32px;
		font-weight: 700;
		color: #333;
		margin: 0 0 15px;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
	}

	.video-meta {
		display: flex;
		gap: 20px;
		color: #666;
		font-size: 14px;
	}

	.video-meta span {
		display: flex;
		align-items: center;
		gap: 6px;
	}

	/* Trader Section */
	.alert-detail__trader {
		margin-bottom: 40px;
		padding: 30px;
		background: #f8f9fa;
		border-radius: 12px;
		max-width: 1024px;
	}

	.trader-card {
		display: flex;
		gap: 24px;
		align-items: flex-start;
	}

	.trader-photo {
		flex-shrink: 0;
	}

	.trader-photo img {
		width: 100px;
		height: 100px;
		border-radius: 50%;
		object-fit: cover;
	}

	.trader-avatar {
		width: 100px;
		height: 100px;
		border-radius: 50%;
		background: linear-gradient(135deg, #143e59, #0984ae);
		display: flex;
		align-items: center;
		justify-content: center;
		color: white;
		font-size: 36px;
		font-weight: 700;
	}

	.trader-info {
		flex: 1;
	}

	.trader-name {
		font-size: 24px;
		font-weight: 700;
		color: #333;
		margin: 0 0 5px;
	}

	.trader-title {
		font-size: 14px;
		color: #0984ae;
		margin: 0 0 15px;
		font-weight: 600;
	}

	.trader-bio {
		font-size: 14px;
		color: #666;
		line-height: 1.6;
		margin: 0 0 15px;
	}

	.trader-link {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		color: #143e59;
		font-weight: 600;
		font-size: 14px;
		text-decoration: none;
		transition: color 0.2s;
	}

	.trader-link:hover {
		color: #0984ae;
	}

	/* Description Section */
	.alert-detail__description {
		margin-bottom: 40px;
		max-width: 1024px;
	}

	.alert-detail__description h2 {
		font-size: 20px;
		font-weight: 700;
		color: #333;
		margin: 0 0 15px;
	}

	.alert-detail__description p {
		font-size: 16px;
		color: #555;
		line-height: 1.7;
		margin: 0;
	}

	/* Related Alerts */
	.alert-detail__related {
		margin-bottom: 40px;
	}

	.alert-detail__related h2 {
		font-size: 20px;
		font-weight: 700;
		color: #333;
		margin: 0 0 20px;
	}

	.related-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
		gap: 20px;
	}

	.related-card {
		background: #fff;
		border-radius: 12px;
		overflow: hidden;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
		text-decoration: none;
		transition:
			transform 0.2s,
			box-shadow 0.2s;
	}

	.related-card:hover {
		transform: translateY(-4px);
		box-shadow: 0 8px 24px rgba(0, 0, 0, 0.12);
	}

	.related-image {
		width: 100%;
		aspect-ratio: 16 / 9;
		background-size: cover;
		background-position: center;
		position: relative;
	}

	.related-overlay {
		position: absolute;
		inset: 0;
		background: rgba(0, 0, 0, 0.4);
		display: flex;
		align-items: center;
		justify-content: center;
		color: white;
		opacity: 0;
		transition: opacity 0.2s;
	}

	.related-card:hover .related-overlay {
		opacity: 1;
	}

	.related-info {
		padding: 15px;
	}

	.related-info h4 {
		font-size: 16px;
		font-weight: 600;
		color: #333;
		margin: 0 0 5px;
	}

	.related-date {
		font-size: 13px;
		color: #666;
	}

	/* Back Link */
	.alert-detail__back {
		padding-top: 20px;
		border-top: 1px solid #e5e5e5;
	}

	.back-link {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		color: #143e59;
		font-weight: 600;
		font-size: 14px;
		text-decoration: none;
		transition: color 0.2s;
	}

	.back-link:hover {
		color: #0984ae;
	}

	/* Mobile Responsive */
	@media (max-width: 768px) {
		.alert-detail__content {
			padding: 20px;
		}

		.video-title {
			font-size: 24px;
		}

		.trader-card {
			flex-direction: column;
			align-items: center;
			text-align: center;
		}

		.trader-photo img,
		.trader-avatar {
			width: 80px;
			height: 80px;
		}

		.trader-avatar {
			font-size: 28px;
		}
	}
</style>

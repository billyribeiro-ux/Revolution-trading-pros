<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * Sidebar Component - Dashboard Sidebar Container
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @description Sticky sidebar containing performance, video, resources, and updates
	 * @version 4.2.0 - Svelte 5 January 2026 Refactor
	 * @requires Svelte 5.0+ (January 2026 syntax)
	 * @standards Apple Principal Engineer ICT 7+ Standards
	 *
	 * ARCHITECTURE:
	 * - Each sidebar section is a self-contained component
	 * - Sidebar orchestrates composition and spacing
	 * - Props flow down, events bubble up via callbacks
	 * - CSS containment for performance
	 */
	import type { ThirtyDayPerformance, WeeklyVideo, Video } from '../types';
	import PerformanceCard from './PerformanceCard.svelte';
	import WeeklyVideoCard from './WeeklyVideoCard.svelte';
	import ResourceLinks from './ResourceLinks.svelte';
	import LatestUpdatesCard from './LatestUpdatesCard.svelte';

	interface Props {
		thirtyDayPerformance: ThirtyDayPerformance;
		weeklyVideo: WeeklyVideo;
		latestUpdates?: Video[];
		isLoading?: boolean;
		onPlayVideo?: () => void;
	}

	const { 
		thirtyDayPerformance, 
		weeklyVideo, 
		latestUpdates = [],
		isLoading = false, 
		onPlayVideo 
	}: Props = $props();
</script>

<aside class="sidebar" aria-label="Dashboard sidebar">
	<!-- Weekly Video - Primary content, first position -->
	<WeeklyVideoCard video={weeklyVideo} {isLoading} onPlay={onPlayVideo} />

	<!-- 30-Day Performance - Key metric -->
	<PerformanceCard performance={thirtyDayPerformance} {isLoading} />

	<!-- Quick Resources - Navigation -->
	<ResourceLinks variant="compact" />

	<!-- Archive Link -->
	<a href="/dashboard/explosive-swings/archive" class="archive-link">
		<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="18" height="18">
			<path d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"/>
		</svg>
		Weekly Archive
	</a>

	<!-- Latest Updates - Compact 3-item grid -->
	{#if latestUpdates.length > 0}
		<LatestUpdatesCard updates={latestUpdates} {isLoading} maxItems={3} />
	{/if}

	<!-- Need Help Card -->
	<div class="help-card">
		<h4 class="help-title">Need Help?</h4>
		<p class="help-text">Questions about a trade or setup?</p>
		<a href="/support" class="help-btn">Contact Support</a>
	</div>
</aside>

<style>
	.sidebar {
		position: sticky;
		top: 80px;
		display: flex;
		flex-direction: column;
		gap: 20px;
		max-height: calc(100vh - 100px);
		overflow-y: auto;
		padding-bottom: 20px;
		contain: layout style;
		scrollbar-width: thin;
		scrollbar-color: var(--color-border-strong) transparent;
	}

	.sidebar::-webkit-scrollbar {
		width: 6px;
	}

	.sidebar::-webkit-scrollbar-track {
		background: transparent;
	}

	.sidebar::-webkit-scrollbar-thumb {
		background: var(--color-border-strong);
		border-radius: 3px;
	}

	.help-card {
		background: linear-gradient(135deg, var(--color-brand-primary) 0%, var(--color-brand-primary-light) 100%);
		border: 1px solid var(--color-border-default);
		border-top: 3px solid var(--color-watching);
		border-radius: 14px;
		padding: 22px;
		text-align: center;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.04);
		transition: all 0.2s ease-out;
	}

	.help-card:hover {
		box-shadow: 0 4px 16px rgba(20, 62, 89, 0.25);
		transform: translateY(-2px);
	}

	.help-title {
		font-size: 16px;
		font-weight: 700;
		color: var(--color-bg-card);
		margin: 0 0 8px 0;
	}

	.help-text {
		font-size: 14px;
		color: rgba(255, 255, 255, 0.85);
		margin: 0 0 16px 0;
		line-height: 1.5;
	}

	.help-btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		padding: 11px 22px;
		font-size: 14px;
		font-weight: 600;
		color: var(--color-brand-primary);
		background: var(--color-bg-card);
		border: 1.5px solid rgba(255, 255, 255, 0.2);
		border-radius: 10px;
		text-decoration: none;
		transition: all 0.2s ease-out;
	}

	.help-btn:hover {
		background: var(--color-watching);
		color: var(--color-bg-card);
		border-color: var(--color-watching);
		transform: translateY(-2px) scale(1.02);
		box-shadow: 0 4px 12px rgba(245, 158, 11, 0.4);
	}

	.help-btn:focus-visible {
		outline: 2px solid var(--color-bg-card);
		outline-offset: 2px;
	}

	/* Archive Link */
	.archive-link {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 10px;
		padding: 14px 20px;
		background: var(--color-bg-card);
		border: 1px solid var(--color-border-default);
		border-radius: 12px;
		color: var(--color-text-secondary);
		text-decoration: none;
		font-size: 14px;
		font-weight: 600;
		transition: all 0.2s ease-out;
	}

	.archive-link:hover {
		background: var(--color-bg-subtle);
		color: var(--color-brand-primary);
		border-color: var(--color-brand-primary);
	}

	.archive-link:focus-visible {
		outline: 2px solid var(--color-brand-primary);
		outline-offset: 2px;
	}

	/* Large Desktop (1440px+) */
	@media (min-width: 1440px) {
		.sidebar {
			gap: 24px;
		}

		.help-card {
			padding: 26px;
		}
	}

	/* Ultra-wide (1920px+) */
	@media (min-width: 1920px) {
		.sidebar {
			gap: 28px;
		}
	}

	@media (max-width: 1024px) {
		.sidebar {
			position: static;
			max-height: none;
			overflow-y: visible;
		}
	}

	@media (max-width: 640px) {
		.sidebar {
			gap: 16px;
			padding-bottom: 16px;
		}

		.help-card {
			padding: 18px;
			border-radius: 12px;
		}

		.help-title {
			font-size: 15px;
		}

		.help-text {
			font-size: 13px;
		}
	}
</style>

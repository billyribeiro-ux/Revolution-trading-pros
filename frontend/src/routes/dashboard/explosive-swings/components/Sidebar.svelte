<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * Sidebar Component - Dashboard Sidebar Container
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @description Sticky sidebar containing performance, video, and resources
	 * @version 4.0.0 - January 2026 - Nuclear Build Specification
	 * @standards Apple Principal Engineer ICT 7+ Standards
	 */
	import type { ThirtyDayPerformance, WeeklyVideo } from '../types';
	import PerformanceCard from './PerformanceCard.svelte';
	import WeeklyVideoCard from './WeeklyVideoCard.svelte';
	import ResourceLinks from './ResourceLinks.svelte';

	interface Props {
		thirtyDayPerformance: ThirtyDayPerformance;
		weeklyVideo: WeeklyVideo;
		isLoading?: boolean;
		onPlayVideo?: () => void;
	}

	const { thirtyDayPerformance, weeklyVideo, isLoading = false, onPlayVideo }: Props = $props();
</script>

<aside class="sidebar" aria-label="Dashboard sidebar">
	<!-- 30-Day Performance -->
	<PerformanceCard performance={thirtyDayPerformance} {isLoading} />

	<!-- Weekly Video -->
	<WeeklyVideoCard video={weeklyVideo} {isLoading} onPlay={onPlayVideo} />

	<!-- Quick Resources -->
	<ResourceLinks />

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
	}

	.sidebar::-webkit-scrollbar {
		width: 6px;
	}

	.sidebar::-webkit-scrollbar-track {
		background: transparent;
	}

	.sidebar::-webkit-scrollbar-thumb {
		background: #cbd5e1;
		border-radius: 3px;
	}

	.help-card {
		background: linear-gradient(135deg, #f8fafc 0%, #f1f5f9 100%);
		border: 1px solid #e2e8f0;
		border-radius: 12px;
		padding: 20px;
		text-align: center;
	}

	.help-title {
		font-size: 16px;
		font-weight: 600;
		color: #0f172a;
		margin: 0 0 8px 0;
	}

	.help-text {
		font-size: 14px;
		color: #64748b;
		margin: 0 0 16px 0;
		line-height: 1.5;
	}

	.help-btn {
		display: inline-block;
		padding: 10px 20px;
		font-size: 14px;
		font-weight: 600;
		color: #143e59;
		background: #fff;
		border: 1px solid #e2e8f0;
		border-radius: 8px;
		text-decoration: none;
		transition: all 0.2s;
	}

	.help-btn:hover {
		background: #143e59;
		color: #fff;
		border-color: #143e59;
	}

	@media (max-width: 1024px) {
		.sidebar {
			position: static;
			max-height: none;
		}
	}
</style>

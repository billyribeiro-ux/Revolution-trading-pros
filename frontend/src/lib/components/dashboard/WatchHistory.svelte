<!--
/**
 * WatchHistory - Continue Watching Component
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Apple ICT 7 Principal Engineer Grade - February 2026
 *
 * FEATURES:
 * 1. Shows user's recently watched videos with progress
 * 2. Resume from where user left off
 * 3. Progress bar visualization
 * 4. Mobile-first responsive design
 *
 * @version 1.0.0
 */
-->
<script lang="ts">
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';

	// ═══════════════════════════════════════════════════════════════════════
	// PROPS
	// ═══════════════════════════════════════════════════════════════════════

	interface WatchHistoryItem {
		video_id: number;
		title: string;
		thumbnail_url: string | null;
		current_time_seconds: number;
		duration: number | null;
		completion_percent: number;
		completed: boolean;
		last_watched_at: string;
	}

	interface Props {
		/** User ID for fetching history */
		userId: number;
		/** Maximum items to show */
		limit?: number;
		/** Title for the section */
		title?: string;
		/** Show "View All" link */
		showViewAll?: boolean;
		/** Custom class */
		class?: string;
	}

	let props: Props = $props();

	// Derived props with defaults
	let userId = $derived(props.userId);
	let limit = $derived(props.limit ?? 6);
	let title = $derived(props.title ?? 'Continue Watching');
	let showViewAll = $derived(props.showViewAll ?? true);
	let className = $derived(props.class ?? '');

	// ═══════════════════════════════════════════════════════════════════════
	// STATE
	// ═══════════════════════════════════════════════════════════════════════

	let history = $state<WatchHistoryItem[]>([]);
	let isLoading = $state(true);
	let error = $state<string | null>(null);
	void error;

	// ═══════════════════════════════════════════════════════════════════════
	// LIFECYCLE
	// ═══════════════════════════════════════════════════════════════════════

	onMount(async () => {
		if (!browser || !userId) return;
		await fetchHistory();
	});

	// ═══════════════════════════════════════════════════════════════════════
	// METHODS
	// ═══════════════════════════════════════════════════════════════════════

	async function fetchHistory() {
		try {
			isLoading = true;
			const response = await fetch(`/api/videos/history?user_id=${userId}&limit=${limit}`);
			const data = await response.json();

			if (data.success && Array.isArray(data.data)) {
				history = data.data.filter((item: WatchHistoryItem) => !item.completed);
			}
		} catch (e) {
			error = 'Failed to load watch history';
			console.error('Watch history fetch error:', e);
		} finally {
			isLoading = false;
		}
	}

	function formatDuration(seconds: number): string {
		if (!seconds) return '';
		const hours = Math.floor(seconds / 3600);
		const minutes = Math.floor((seconds % 3600) / 60);
		const secs = seconds % 60;

		if (hours > 0) {
			return `${hours}:${String(minutes).padStart(2, '0')}:${String(secs).padStart(2, '0')}`;
		}
		return `${minutes}:${String(secs).padStart(2, '0')}`;
	}

	function formatTimeAgo(dateString: string): string {
		const date = new Date(dateString);
		const now = new Date();
		const diffMs = now.getTime() - date.getTime();
		const diffMins = Math.floor(diffMs / 60000);
		const diffHours = Math.floor(diffMins / 60);
		const diffDays = Math.floor(diffHours / 24);

		if (diffMins < 1) return 'Just now';
		if (diffMins < 60) return `${diffMins}m ago`;
		if (diffHours < 24) return `${diffHours}h ago`;
		if (diffDays < 7) return `${diffDays}d ago`;
		return date.toLocaleDateString();
	}

	function getResumeUrl(item: WatchHistoryItem): string {
		return `/dashboard/videos/${item.video_id}?t=${item.current_time_seconds}`;
	}
</script>

{#if !isLoading && history.length > 0}
	<section class="watch-history {className}">
		<div class="watch-history__header">
			<h2 class="watch-history__title">{title}</h2>
			{#if showViewAll}
				<a href="/dashboard/history" class="watch-history__view-all">View All</a>
			{/if}
		</div>

		<div class="watch-history__grid">
			{#each history as item (item.video_id)}
				<a href={getResumeUrl(item)} class="watch-card">
					<!-- Thumbnail -->
					<div class="watch-card__thumbnail">
						{#if item.thumbnail_url}
							<img src={item.thumbnail_url} alt={item.title} loading="lazy" />
						{:else}
							<div class="watch-card__placeholder">
								<svg viewBox="0 0 24 24" fill="currentColor" width="32" height="32">
									<path d="M8 5v14l11-7z" />
								</svg>
							</div>
						{/if}

						<!-- Progress overlay -->
						<div class="watch-card__progress-bar">
							<div
								class="watch-card__progress-fill"
								style="width: {item.completion_percent}%"
							></div>
						</div>

						<!-- Resume button overlay -->
						<div class="watch-card__resume-overlay">
							<div class="watch-card__resume-btn">
								<svg viewBox="0 0 24 24" fill="currentColor" width="24" height="24">
									<path d="M8 5v14l11-7z" />
								</svg>
							</div>
						</div>

						<!-- Time left badge -->
						{#if item.duration && item.current_time_seconds}
							<div class="watch-card__time-left">
								{formatDuration(item.duration - item.current_time_seconds)} left
							</div>
						{/if}
					</div>

					<!-- Info -->
					<div class="watch-card__info">
						<h3 class="watch-card__title">{item.title}</h3>
						<p class="watch-card__meta">
							<span class="watch-card__progress-text">{item.completion_percent}% watched</span>
							<span class="watch-card__dot"></span>
							<span class="watch-card__time">{formatTimeAgo(item.last_watched_at)}</span>
						</p>
					</div>
				</a>
			{/each}
		</div>
	</section>
{:else if isLoading}
	<section class="watch-history {className}">
		<div class="watch-history__header">
			<h2 class="watch-history__title">{title}</h2>
		</div>
		<div class="watch-history__loading">
			<div class="watch-history__spinner"></div>
		</div>
	</section>
{/if}

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   WATCH HISTORY - 2026 Mobile-First Responsive Design
	   ═══════════════════════════════════════════════════════════════════════════ */

	.watch-history {
		margin-bottom: 2rem;
	}

	.watch-history__header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1rem;
		padding: 0 0.5rem;
	}

	.watch-history__title {
		font-size: 1.125rem;
		font-weight: 600;
		color: var(--color-text, #1f2937);
		margin: 0;
	}

	.watch-history__view-all {
		font-size: 0.875rem;
		color: var(--color-primary, #2563eb);
		text-decoration: none;
		font-weight: 500;
	}

	.watch-history__view-all:hover {
		text-decoration: underline;
	}

	/* Grid - Mobile first */
	.watch-history__grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
		gap: 1rem;
	}

	/* Card */
	.watch-card {
		display: block;
		text-decoration: none;
		background: var(--color-surface, #fff);
		border-radius: 8px;
		overflow: hidden;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
		transition:
			transform 0.2s ease,
			box-shadow 0.2s ease;
	}

	.watch-card:hover {
		transform: translateY(-2px);
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
	}

	/* Thumbnail */
	.watch-card__thumbnail {
		position: relative;
		aspect-ratio: 16 / 9;
		background: #1f2937;
		overflow: hidden;
	}

	.watch-card__thumbnail img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.watch-card__placeholder {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 100%;
		height: 100%;
		color: rgba(255, 255, 255, 0.5);
	}

	/* Progress bar */
	.watch-card__progress-bar {
		position: absolute;
		bottom: 0;
		left: 0;
		right: 0;
		height: 4px;
		background: rgba(255, 255, 255, 0.3);
	}

	.watch-card__progress-fill {
		height: 100%;
		background: var(--color-primary, #ef4444);
		transition: width 0.3s ease;
	}

	/* Resume overlay */
	.watch-card__resume-overlay {
		position: absolute;
		inset: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(0, 0, 0, 0.4);
		opacity: 0;
		transition: opacity 0.2s ease;
	}

	.watch-card:hover .watch-card__resume-overlay {
		opacity: 1;
	}

	.watch-card__resume-btn {
		width: 48px;
		height: 48px;
		border-radius: 50%;
		background: rgba(255, 255, 255, 0.95);
		display: flex;
		align-items: center;
		justify-content: center;
		color: #1f2937;
	}

	.watch-card__resume-btn svg {
		margin-left: 3px;
	}

	/* Time left badge */
	.watch-card__time-left {
		position: absolute;
		bottom: 8px;
		right: 8px;
		padding: 2px 6px;
		background: rgba(0, 0, 0, 0.8);
		color: #fff;
		font-size: 0.75rem;
		border-radius: 4px;
	}

	/* Info */
	.watch-card__info {
		padding: 0.75rem;
	}

	.watch-card__title {
		font-size: 0.9375rem;
		font-weight: 500;
		color: var(--color-text, #1f2937);
		margin: 0 0 0.25rem;
		line-height: 1.3;
		display: -webkit-box;
		-webkit-line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	.watch-card__meta {
		display: flex;
		align-items: center;
		font-size: 0.8125rem;
		color: var(--color-text-muted, #6b7280);
		margin: 0;
	}

	.watch-card__dot {
		width: 3px;
		height: 3px;
		border-radius: 50%;
		background: currentColor;
		margin: 0 0.5rem;
	}

	/* Loading */
	.watch-history__loading {
		display: flex;
		justify-content: center;
		padding: 2rem;
	}

	.watch-history__spinner {
		width: 32px;
		height: 32px;
		border: 3px solid rgba(0, 0, 0, 0.1);
		border-top-color: var(--color-primary, #2563eb);
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE BREAKPOINTS
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (min-width: 640px) {
		.watch-history__title {
			font-size: 1.25rem;
		}

		.watch-history__grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	@media (min-width: 1024px) {
		.watch-history__grid {
			grid-template-columns: repeat(3, 1fr);
		}
	}

	@media (min-width: 1280px) {
		.watch-history__grid {
			grid-template-columns: repeat(4, 1fr);
		}
	}

	/* Dark mode support */
	@media (prefers-color-scheme: dark) {
		.watch-card {
			background: #1f2937;
		}

		.watch-card__title {
			color: #f9fafb;
		}

		.watch-card__meta {
			color: #9ca3af;
		}

		.watch-card__resume-btn {
			background: rgba(255, 255, 255, 0.9);
			color: #1f2937;
		}
	}
</style>

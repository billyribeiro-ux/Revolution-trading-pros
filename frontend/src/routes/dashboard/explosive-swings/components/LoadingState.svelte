<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * LoadingState Component - Skeleton Loaders
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * Provides skeleton loading states for all data sections:
	 * - Stats cards
	 * - Alert cards
	 * - Position cards
	 * - Video cards
	 * - Tables
	 *
	 * @version 1.0.0
	 * @standards Apple Principal Engineer ICT 7+ | WCAG 2.1 AA
	 */

	type LoadingVariant = 'stats' | 'alert' | 'position' | 'video' | 'table' | 'hero' | 'card';

	interface Props {
		variant?: LoadingVariant;
		count?: number;
		columns?: number;
	}

	const { variant = 'card', count = 1, columns = 1 }: Props = $props();
</script>

<div
	class="loading-state"
	class:grid={columns > 1}
	style="--columns: {columns}"
	role="status"
	aria-label="Loading content"
	aria-busy="true"
>
	{#each Array(count) as _, i}
		<div class="skeleton-wrapper" style="animation-delay: {i * 0.1}s">
			{#if variant === 'stats'}
				<!-- Stats Card Skeleton -->
				<div class="skeleton-stats">
					<div class="skeleton-stats-header">
						<div class="skeleton-line w-24 h-4"></div>
						<div class="skeleton-line w-16 h-5"></div>
					</div>
					<div class="skeleton-line w-32 h-8 mt-2"></div>
					<div class="skeleton-line w-20 h-4 mt-3"></div>
				</div>
			{:else if variant === 'alert'}
				<!-- Alert Card Skeleton -->
				<div class="skeleton-alert">
					<div class="skeleton-alert-header">
						<div class="skeleton-circle w-10 h-10"></div>
						<div class="skeleton-alert-title">
							<div class="skeleton-line w-20 h-5"></div>
							<div class="skeleton-line w-32 h-4 mt-1"></div>
						</div>
						<div class="skeleton-line w-16 h-6 badge"></div>
					</div>
					<div class="skeleton-line w-full h-4 mt-4"></div>
					<div class="skeleton-line w-3/4 h-4 mt-2"></div>
					<div class="skeleton-alert-footer">
						<div class="skeleton-line w-24 h-3"></div>
						<div class="skeleton-line w-20 h-8 rounded-lg"></div>
					</div>
				</div>
			{:else if variant === 'position'}
				<!-- Position Card Skeleton -->
				<div class="skeleton-position">
					<div class="skeleton-position-header">
						<div class="skeleton-circle w-12 h-12"></div>
						<div class="flex-1">
							<div class="skeleton-line w-16 h-5"></div>
							<div class="skeleton-line w-24 h-4 mt-1"></div>
						</div>
						<div class="skeleton-line w-20 h-6 badge"></div>
					</div>
					<div class="skeleton-position-stats">
						<div class="skeleton-stat-item">
							<div class="skeleton-line w-12 h-3"></div>
							<div class="skeleton-line w-16 h-5 mt-1"></div>
						</div>
						<div class="skeleton-stat-item">
							<div class="skeleton-line w-12 h-3"></div>
							<div class="skeleton-line w-16 h-5 mt-1"></div>
						</div>
						<div class="skeleton-stat-item">
							<div class="skeleton-line w-12 h-3"></div>
							<div class="skeleton-line w-16 h-5 mt-1"></div>
						</div>
					</div>
					<div class="skeleton-position-actions">
						<div class="skeleton-line w-full h-9 rounded-lg"></div>
					</div>
				</div>
			{:else if variant === 'video'}
				<!-- Video Card Skeleton -->
				<div class="skeleton-video">
					<div class="skeleton-rect aspect-video rounded-lg"></div>
					<div class="skeleton-video-info">
						<div class="skeleton-line w-3/4 h-5 mt-3"></div>
						<div class="skeleton-line w-1/2 h-4 mt-2"></div>
						<div class="skeleton-video-meta">
							<div class="skeleton-line w-16 h-3"></div>
							<div class="skeleton-line w-20 h-3"></div>
						</div>
					</div>
				</div>
			{:else if variant === 'table'}
				<!-- Table Row Skeleton -->
				<div class="skeleton-table-row">
					<div class="skeleton-line w-16 h-5"></div>
					<div class="skeleton-line w-24 h-4"></div>
					<div class="skeleton-line w-24 h-4"></div>
					<div class="skeleton-line w-16 h-4"></div>
					<div class="skeleton-line w-16 h-4"></div>
					<div class="skeleton-line w-20 h-6 badge"></div>
				</div>
			{:else if variant === 'hero'}
				<!-- Hero Section Skeleton -->
				<div class="skeleton-hero">
					<div class="skeleton-hero-content">
						<div class="skeleton-line w-48 h-8"></div>
						<div class="skeleton-line w-64 h-5 mt-3"></div>
						<div class="skeleton-line w-32 h-10 mt-5 rounded-lg"></div>
					</div>
					<div class="skeleton-rect w-80 h-48 rounded-xl"></div>
				</div>
			{:else}
				<!-- Generic Card Skeleton -->
				<div class="skeleton-card">
					<div class="skeleton-rect w-full h-32 rounded-lg"></div>
					<div class="skeleton-line w-3/4 h-5 mt-4"></div>
					<div class="skeleton-line w-full h-4 mt-2"></div>
					<div class="skeleton-line w-2/3 h-4 mt-2"></div>
				</div>
			{/if}
		</div>
	{/each}
	<span class="sr-only">Loading...</span>
</div>

<style>
	.loading-state {
		width: 100%;
	}

	.loading-state.grid {
		display: grid;
		grid-template-columns: repeat(var(--columns), 1fr);
		gap: var(--space-4);
	}

	.skeleton-wrapper {
		animation: fadeIn 0.3s ease-out forwards;
		opacity: 0;
	}

	@keyframes fadeIn {
		to {
			opacity: 1;
		}
	}

	/* Base skeleton elements */
	.skeleton-line,
	.skeleton-rect,
	.skeleton-circle {
		background: linear-gradient(
			90deg,
			var(--color-bg-subtle) 25%,
			var(--color-bg-muted) 50%,
			var(--color-bg-subtle) 75%
		);
		background-size: 200% 100%;
		animation: shimmer 1.5s ease-in-out infinite;
		border-radius: var(--radius-sm);
	}

	.skeleton-circle {
		border-radius: 50%;
	}

	.skeleton-line.badge {
		border-radius: var(--radius-sm);
	}

	@keyframes shimmer {
		0% {
			background-position: 200% 0;
		}
		100% {
			background-position: -200% 0;
		}
	}

	/* Width utilities */
	.w-full {
		width: 100%;
	}
	.w-3\/4 {
		width: 75%;
	}
	.w-2\/3 {
		width: 66.666%;
	}
	.w-1\/2 {
		width: 50%;
	}
	.w-80 {
		width: 320px;
	}
	.w-64 {
		width: 256px;
	}
	.w-48 {
		width: 192px;
	}
	.w-32 {
		width: 128px;
	}
	.w-24 {
		width: 96px;
	}
	.w-20 {
		width: 80px;
	}
	.w-16 {
		width: 64px;
	}
	.w-12 {
		width: 48px;
	}
	.w-10 {
		width: 40px;
	}

	/* Height utilities */
	.h-48 {
		height: 192px;
	}
	.h-32 {
		height: 128px;
	}
	.h-12 {
		height: 48px;
	}
	.h-10 {
		height: 40px;
	}
	.h-9 {
		height: 36px;
	}
	.h-8 {
		height: 32px;
	}
	.h-6 {
		height: 24px;
	}
	.h-5 {
		height: 20px;
	}
	.h-4 {
		height: 16px;
	}
	.h-3 {
		height: 12px;
	}

	/* Spacing utilities */
	.mt-1 {
		margin-top: var(--space-1);
	}
	.mt-2 {
		margin-top: var(--space-2);
	}
	.mt-3 {
		margin-top: var(--space-3);
	}
	.mt-4 {
		margin-top: var(--space-4);
	}
	.mt-5 {
		margin-top: var(--space-5);
	}

	/* Aspect ratio */
	.aspect-video {
		aspect-ratio: 16 / 9;
	}

	/* Border radius */
	.rounded-lg {
		border-radius: var(--radius-lg);
	}
	.rounded-xl {
		border-radius: var(--radius-xl);
	}

	/* Flexbox utilities */
	.flex-1 {
		flex: 1;
	}

	/* Stats Card */
	.skeleton-stats {
		background: var(--color-bg-card);
		border: 1px solid var(--color-border-subtle);
		border-radius: var(--radius-xl);
		padding: var(--space-5);
	}

	.skeleton-stats-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	/* Alert Card */
	.skeleton-alert {
		background: var(--color-bg-card);
		border: 1px solid var(--color-border-subtle);
		border-radius: var(--radius-xl);
		padding: var(--space-5);
	}

	.skeleton-alert-header {
		display: flex;
		align-items: flex-start;
		gap: var(--space-3);
	}

	.skeleton-alert-title {
		flex: 1;
	}

	.skeleton-alert-footer {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-top: var(--space-4);
		padding-top: var(--space-4);
		border-top: 1px solid var(--color-border-subtle);
	}

	/* Position Card */
	.skeleton-position {
		background: var(--color-bg-card);
		border: 1px solid var(--color-border-subtle);
		border-radius: var(--radius-xl);
		padding: var(--space-5);
	}

	.skeleton-position-header {
		display: flex;
		align-items: center;
		gap: var(--space-3);
	}

	.skeleton-position-stats {
		display: flex;
		gap: var(--space-4);
		margin-top: var(--space-4);
		padding-top: var(--space-4);
		border-top: 1px solid var(--color-border-subtle);
	}

	.skeleton-stat-item {
		flex: 1;
	}

	.skeleton-position-actions {
		margin-top: var(--space-4);
	}

	/* Video Card */
	.skeleton-video {
		background: var(--color-bg-card);
		border: 1px solid var(--color-border-subtle);
		border-radius: var(--radius-xl);
		overflow: hidden;
	}

	.skeleton-video-info {
		padding: var(--space-4);
	}

	.skeleton-video-meta {
		display: flex;
		gap: var(--space-4);
		margin-top: var(--space-3);
	}

	/* Table Row */
	.skeleton-table-row {
		display: flex;
		align-items: center;
		gap: var(--space-4);
		padding: var(--space-4);
		background: var(--color-bg-card);
		border-bottom: 1px solid var(--color-border-subtle);
	}

	/* Hero Section */
	.skeleton-hero {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: var(--space-8);
		background: var(--color-bg-card);
		border: 1px solid var(--color-border-subtle);
		border-radius: var(--radius-xl);
		padding: var(--space-8);
	}

	.skeleton-hero-content {
		flex: 1;
	}

	/* Generic Card */
	.skeleton-card {
		background: var(--color-bg-card);
		border: 1px solid var(--color-border-subtle);
		border-radius: var(--radius-xl);
		padding: var(--space-5);
	}

	/* Screen reader only */
	.sr-only {
		position: absolute;
		width: 1px;
		height: 1px;
		padding: 0;
		margin: -1px;
		overflow: hidden;
		clip: rect(0, 0, 0, 0);
		white-space: nowrap;
		border-width: 0;
	}

	/* Responsive */
	@media (max-width: 768px) {
		.loading-state.grid {
			grid-template-columns: 1fr;
		}

		.skeleton-hero {
			flex-direction: column;
		}

		.skeleton-hero .skeleton-rect {
			width: 100%;
		}
	}
</style>

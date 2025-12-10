<!--
/**
 * LessonCard Component - Learning Center
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Displays a lesson preview card with thumbnail, progress, and metadata.
 * Used in learning center grids and module accordions.
 *
 * @version 1.0.0 (December 2025)
 */
-->

<script lang="ts">
	import type { LessonWithRelations } from '$lib/types/learning-center';
	import {
		IconPlayerPlay,
		IconClock,
		IconCheck,
		IconBookmark,
		IconLock
	} from '$lib/icons';

	interface Props {
		lesson: LessonWithRelations;
		showProgress?: boolean;
		showTrainer?: boolean;
		showCategory?: boolean;
		compact?: boolean;
		href?: string;
		onBookmark?: (lessonId: string) => void;
	}

	let {
		lesson,
		showProgress = true,
		showTrainer = true,
		showCategory = false,
		compact = false,
		href,
		onBookmark
	}: Props = $props();

	// Computed values
	let isCompleted = $derived(lesson.userProgress?.isCompleted ?? false);
	let progressPercent = $derived(lesson.userProgress?.progressPercent ?? 0);
	let isBookmarked = $derived(lesson.userProgress?.isBookmarked ?? false);
	let isPremium = $derived(lesson.accessLevel === 'premium');
	let isMemberOnly = $derived(lesson.accessLevel === 'member');

	// Type icons
	const typeIcons: Record<string, string> = {
		video: 'play',
		article: 'file-text',
		pdf: 'file',
		quiz: 'help-circle',
		'webinar-replay': 'video'
	};

	function handleBookmarkClick(e: MouseEvent) {
		e.preventDefault();
		e.stopPropagation();
		onBookmark?.(lesson.id);
	}
</script>

<a
	{href}
	class="lesson-card"
	class:compact
	class:completed={isCompleted}
	class:premium={isPremium}
>
	<!-- Thumbnail -->
	<div class="card-thumbnail">
		{#if lesson.thumbnailUrl || lesson.posterUrl}
			<img
				src={lesson.thumbnailUrl || lesson.posterUrl}
				alt={lesson.title}
				loading="lazy"
			/>
		{:else}
			<div class="thumbnail-placeholder">
				<IconPlayerPlay size={compact ? 24 : 32} />
			</div>
		{/if}

		<!-- Play overlay -->
		<div class="play-overlay">
			<div class="play-button">
				<IconPlayerPlay size={compact ? 20 : 28} />
			</div>
		</div>

		<!-- Duration badge -->
		{#if lesson.duration}
			<span class="duration-badge">
				<IconClock size={12} />
				{lesson.duration}
			</span>
		{/if}

		<!-- Completion badge -->
		{#if isCompleted}
			<span class="completion-badge">
				<IconCheck size={14} />
			</span>
		{/if}

		<!-- Premium/Member badge -->
		{#if isPremium || isMemberOnly}
			<span class="access-badge" class:member={isMemberOnly}>
				<IconLock size={12} />
				{isPremium ? 'Premium' : 'Members'}
			</span>
		{/if}

		<!-- Progress bar -->
		{#if showProgress && progressPercent > 0 && !isCompleted}
			<div class="progress-bar">
				<div class="progress-fill" style="width: {progressPercent}%"></div>
			</div>
		{/if}
	</div>

	<!-- Content -->
	<div class="card-content">
		<!-- Category tag -->
		{#if showCategory && lesson.category}
			<span class="category-tag">{lesson.category.name}</span>
		{/if}

		<!-- Title -->
		<h3 class="card-title">{lesson.title}</h3>

		<!-- Description (not in compact mode) -->
		{#if !compact}
			<p class="card-description">{lesson.description}</p>
		{/if}

		<!-- Footer -->
		<div class="card-footer">
			{#if showTrainer && lesson.trainer}
				<div class="trainer-info">
					{#if lesson.trainer.thumbnailUrl}
						<img
							src={lesson.trainer.thumbnailUrl}
							alt={lesson.trainer.name}
							class="trainer-avatar"
						/>
					{/if}
					<span class="trainer-name">{lesson.trainer.name}</span>
				</div>
			{/if}

			<!-- Bookmark button -->
			{#if onBookmark}
				<button
					type="button"
					class="bookmark-btn"
					class:bookmarked={isBookmarked}
					onclick={handleBookmarkClick}
					aria-label={isBookmarked ? 'Remove bookmark' : 'Add bookmark'}
				>
					<IconBookmark size={18} />
				</button>
			{/if}
		</div>
	</div>
</a>

<style>
	.lesson-card {
		display: flex;
		flex-direction: column;
		background: #1e293b;
		border-radius: 12px;
		overflow: hidden;
		text-decoration: none;
		color: inherit;
		transition: all 0.3s ease;
		border: 1px solid transparent;
	}

	.lesson-card:hover {
		transform: translateY(-4px);
		border-color: rgba(249, 115, 22, 0.3);
		box-shadow: 0 10px 40px rgba(0, 0, 0, 0.3);
	}

	.lesson-card.completed {
		border-color: rgba(34, 197, 94, 0.3);
	}

	.lesson-card.compact {
		flex-direction: row;
		border-radius: 8px;
	}

	.lesson-card.compact:hover {
		transform: translateY(-2px);
	}

	/* Thumbnail */
	.card-thumbnail {
		position: relative;
		aspect-ratio: 16/9;
		background: #0f172a;
		overflow: hidden;
	}

	.lesson-card.compact .card-thumbnail {
		width: 160px;
		flex-shrink: 0;
		aspect-ratio: auto;
		height: 90px;
	}

	.card-thumbnail img {
		width: 100%;
		height: 100%;
		object-fit: cover;
		transition: transform 0.3s ease;
	}

	.lesson-card:hover .card-thumbnail img {
		transform: scale(1.05);
	}

	.thumbnail-placeholder {
		width: 100%;
		height: 100%;
		display: flex;
		align-items: center;
		justify-content: center;
		background: linear-gradient(135deg, #1e293b 0%, #334155 100%);
		color: #64748b;
	}

	/* Play overlay */
	.play-overlay {
		position: absolute;
		inset: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(0, 0, 0, 0.4);
		opacity: 0;
		transition: opacity 0.3s ease;
	}

	.lesson-card:hover .play-overlay {
		opacity: 1;
	}

	.play-button {
		width: 56px;
		height: 56px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(249, 115, 22, 0.9);
		border-radius: 50%;
		color: white;
		transition: transform 0.2s ease;
	}

	.lesson-card.compact .play-button {
		width: 40px;
		height: 40px;
	}

	.lesson-card:hover .play-button {
		transform: scale(1.1);
	}

	/* Badges */
	.duration-badge {
		position: absolute;
		bottom: 8px;
		right: 8px;
		display: flex;
		align-items: center;
		gap: 4px;
		padding: 4px 8px;
		background: rgba(0, 0, 0, 0.8);
		border-radius: 4px;
		font-size: 0.75rem;
		color: white;
	}

	.lesson-card.compact .duration-badge {
		bottom: 4px;
		right: 4px;
		padding: 2px 6px;
		font-size: 0.7rem;
	}

	.completion-badge {
		position: absolute;
		top: 8px;
		right: 8px;
		width: 28px;
		height: 28px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: #22c55e;
		border-radius: 50%;
		color: white;
	}

	.lesson-card.compact .completion-badge {
		width: 24px;
		height: 24px;
		top: 4px;
		right: 4px;
	}

	.access-badge {
		position: absolute;
		top: 8px;
		left: 8px;
		display: flex;
		align-items: center;
		gap: 4px;
		padding: 4px 8px;
		background: linear-gradient(135deg, #f97316, #ea580c);
		border-radius: 4px;
		font-size: 0.7rem;
		font-weight: 600;
		color: white;
		text-transform: uppercase;
	}

	.access-badge.member {
		background: linear-gradient(135deg, #3b82f6, #1d4ed8);
	}

	/* Progress bar */
	.progress-bar {
		position: absolute;
		bottom: 0;
		left: 0;
		right: 0;
		height: 3px;
		background: rgba(255, 255, 255, 0.2);
	}

	.progress-fill {
		height: 100%;
		background: linear-gradient(90deg, #f97316, #ea580c);
		transition: width 0.3s ease;
	}

	/* Content */
	.card-content {
		flex: 1;
		display: flex;
		flex-direction: column;
		padding: 1rem;
	}

	.lesson-card.compact .card-content {
		padding: 0.75rem;
		justify-content: center;
	}

	.category-tag {
		display: inline-block;
		width: fit-content;
		padding: 4px 8px;
		margin-bottom: 0.5rem;
		background: rgba(249, 115, 22, 0.1);
		border-radius: 4px;
		font-size: 0.7rem;
		font-weight: 600;
		color: #f97316;
		text-transform: uppercase;
	}

	.card-title {
		margin: 0;
		font-size: 1rem;
		font-weight: 600;
		color: white;
		line-height: 1.4;
		display: -webkit-box;
		-webkit-line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	.lesson-card.compact .card-title {
		font-size: 0.875rem;
		-webkit-line-clamp: 2;
	}

	.card-description {
		margin: 0.5rem 0 0;
		font-size: 0.875rem;
		color: #94a3b8;
		line-height: 1.5;
		display: -webkit-box;
		-webkit-line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	/* Footer */
	.card-footer {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-top: auto;
		padding-top: 0.75rem;
	}

	.lesson-card.compact .card-footer {
		padding-top: 0.5rem;
	}

	.trainer-info {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.trainer-avatar {
		width: 24px;
		height: 24px;
		border-radius: 50%;
		object-fit: cover;
	}

	.lesson-card.compact .trainer-avatar {
		width: 20px;
		height: 20px;
	}

	.trainer-name {
		font-size: 0.75rem;
		color: #94a3b8;
	}

	/* Bookmark button */
	.bookmark-btn {
		background: none;
		border: none;
		padding: 4px;
		color: #64748b;
		cursor: pointer;
		transition: color 0.2s ease;
	}

	.bookmark-btn:hover {
		color: #f97316;
	}

	.bookmark-btn.bookmarked {
		color: #f97316;
	}

	/* Responsive */
	@media (max-width: 640px) {
		.lesson-card.compact .card-thumbnail {
			width: 120px;
			height: 68px;
		}

		.lesson-card.compact .card-content {
			padding: 0.5rem;
		}

		.lesson-card.compact .card-title {
			font-size: 0.8rem;
		}
	}
</style>

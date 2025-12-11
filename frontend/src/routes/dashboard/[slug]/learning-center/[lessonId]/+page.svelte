<script lang="ts">
	/**
	 * Lesson Detail Page - Individual Lesson View
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * URL: /dashboard/[slug]/learning-center/[lessonId]
	 * Shows individual lesson with video player, description, resources,
	 * and navigation to related lessons.
	 *
	 * @version 1.0.0 (December 2025)
	 */

	import { page } from '$app/stores';
	import { browser } from '$app/environment';
	import { learningCenterStore } from '$lib/stores/learningCenter';
	import type { LessonWithRelations, TradingRoom, Trainer, LessonResource } from '$lib/types/learning-center';
	import { get } from 'svelte/store';
	import VideoEmbed from '$lib/components/VideoEmbed.svelte';
	import { LessonCard, TrainerCard } from '$lib/components/learning-center';
	import {
		IconArrowLeft,
		IconArrowRight,
		IconCheck,
		IconBookmark,
		IconDownload,
		IconShare,
		IconClock,
		IconCalendar,
		IconChevronRight
	} from '$lib/icons';
	import '$lib/styles/st-icons.css';

	// ═══════════════════════════════════════════════════════════════════════════
	// ROUTE PARAMS
	// ═══════════════════════════════════════════════════════════════════════════

	const roomSlug = $derived($page.params.slug);
	const lessonId = $derived($page.params.lessonId);

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let isMarkingComplete = $state(false);
	let showShareModal = $state(false);

	// ═══════════════════════════════════════════════════════════════════════════
	// STORE DATA
	// ═══════════════════════════════════════════════════════════════════════════

	let storeData = $derived(get(learningCenterStore));

	// Find the trading room
	let tradingRoom = $derived.by((): TradingRoom | undefined => {
		return storeData.tradingRooms.find(r => r.slug === roomSlug);
	});

	let roomId = $derived(tradingRoom?.id || '');

	// Get lessons for this room
	let roomLessons = $derived.by((): LessonWithRelations[] => {
		if (!roomId) return [];
		return storeData.lessons.filter(l => l.tradingRoomId === roomId);
	});

	// Find the current lesson
	let lesson = $derived.by((): LessonWithRelations | undefined => {
		return roomLessons.find(l => l.slug === lessonId);
	});

	// Get trainer info
	let trainer = $derived.by((): Trainer | undefined => {
		if (!lesson?.trainerId) return undefined;
		return storeData.trainers.find(t => t.id === lesson.trainerId);
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// NAVIGATION
	// ═══════════════════════════════════════════════════════════════════════════

	// Sort lessons by module and order
	let sortedLessons = $derived.by(() => {
		return [...roomLessons].sort((a, b) => {
			// First by module
			if (a.moduleId !== b.moduleId) {
				return (a.moduleOrder || 0) - (b.moduleOrder || 0);
			}
			// Then by sort order within module
			return a.sortOrder - b.sortOrder;
		});
	});

	// Find current lesson index
	let currentIndex = $derived(sortedLessons.findIndex(l => l.slug === lessonId));

	// Previous and next lessons
	let prevLesson = $derived(currentIndex > 0 ? sortedLessons[currentIndex - 1] : null);
	let nextLesson = $derived(currentIndex < sortedLessons.length - 1 ? sortedLessons[currentIndex + 1] : null);

	// Related lessons (same category or trainer, excluding current)
	let relatedLessons = $derived.by(() => {
		if (!lesson) return [];
		return roomLessons
			.filter(l =>
				l.id !== lesson.id &&
				(l.categoryId === lesson.categoryId || l.trainerId === lesson.trainerId)
			)
			.slice(0, 3);
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// COMPUTED VALUES
	// ═══════════════════════════════════════════════════════════════════════════

	let isCompleted = $derived(lesson?.userProgress?.isCompleted ?? false);
	let isBookmarked = $derived(lesson?.userProgress?.isBookmarked ?? false);
	let progressPercent = $derived(lesson?.userProgress?.progressPercent ?? 0);

	let publishedDate = $derived.by(() => {
		if (!lesson?.publishedAt) return '';
		return new Date(lesson.publishedAt).toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'long',
			day: 'numeric'
		});
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// EVENT HANDLERS
	// ═══════════════════════════════════════════════════════════════════════════

	async function handleMarkComplete() {
		if (!lesson || !roomId || isMarkingComplete) return;

		isMarkingComplete = true;
		try {
			// TODO: Implement mark complete API call
			console.log('Mark lesson complete:', lesson.id, roomId);
		} catch (err) {
			console.error('Failed to mark lesson complete:', err);
		} finally {
			isMarkingComplete = false;
		}
	}

	async function handleToggleBookmark() {
		if (!lesson || !roomId) return;
		try {
			// TODO: Implement toggle bookmark API call
			console.log('Toggle bookmark:', lesson.id, roomId);
		} catch (err) {
			console.error('Failed to toggle bookmark:', err);
		}
	}

	function handleVideoProgress(percent: number) {
		// Update progress as user watches
		if (lesson && roomId && percent > progressPercent) {
			// TODO: Implement progress update API call
			console.log('Update progress:', lesson.id, roomId, percent);
		}
	}

	function handleVideoEnded() {
		// Auto-complete when video finishes
		if (lesson && roomId && !isCompleted) {
			handleMarkComplete();
		}
	}

	function handleShare() {
		if (browser && navigator.share) {
			navigator.share({
				title: lesson?.title,
				text: lesson?.description,
				url: window.location.href
			}).catch(() => {});
		} else {
			showShareModal = true;
		}
	}

	function copyLink() {
		if (browser) {
			navigator.clipboard.writeText(window.location.href);
			showShareModal = false;
		}
	}
</script>

<!-- ═══════════════════════════════════════════════════════════════════════════
     HEAD
     ═══════════════════════════════════════════════════════════════════════════ -->

<svelte:head>
	<title>{lesson?.title || 'Lesson'} | Learning Center | Revolution Trading Pros</title>
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<!-- ═══════════════════════════════════════════════════════════════════════════
     CONTENT
     ═══════════════════════════════════════════════════════════════════════════ -->

{#if !lesson}
	<div class="error-state">
		<h3>Lesson Not Found</h3>
		<p>The lesson "{lessonId}" doesn't exist.</p>
		<a href="/dashboard/{roomSlug}/learning-center" class="btn-back">
			<IconArrowLeft size={18} />
			Back to Learning Center
		</a>
	</div>
{:else}
	<div class="lesson-page">
		<!-- Header -->
		<header class="lesson-header">
			<nav class="breadcrumb">
				<a href="/dashboard">Dashboard</a>
				<IconChevronRight size={14} />
				<a href="/dashboard/{roomSlug}">{tradingRoom?.shortName || roomSlug}</a>
				<IconChevronRight size={14} />
				<a href="/dashboard/{roomSlug}/learning-center">Learning Center</a>
				<IconChevronRight size={14} />
				<span class="current">{lesson.title}</span>
			</nav>

			<div class="header-actions">
				<button
					type="button"
					class="action-btn"
					class:active={isBookmarked}
					onclick={handleToggleBookmark}
					aria-label={isBookmarked ? 'Remove bookmark' : 'Add bookmark'}
				>
					<IconBookmark size={18} />
				</button>
				<button
					type="button"
					class="action-btn"
					onclick={handleShare}
					aria-label="Share lesson"
				>
					<IconShare size={18} />
				</button>
			</div>
		</header>

		<div class="lesson-layout">
			<!-- Main content -->
			<main class="lesson-main">
				<!-- Video Player -->
				{#if lesson.videoUrl}
					<div class="video-container">
						<VideoEmbed
							url={lesson.videoUrl}
							poster={lesson.posterUrl}
							title={lesson.title}
							customControls
							showProgressBar
							showSpeedControl
							showQualitySelector
							showFullscreen
							showPictureInPicture
							startTime={lesson.userProgress?.lastPosition || 0}
							onProgress={handleVideoProgress}
							onEnded={handleVideoEnded}
						/>
					</div>
				{/if}

				<!-- Lesson Info -->
				<div class="lesson-info">
					<div class="lesson-meta">
						{#if lesson.category}
							<span class="category-tag">{lesson.category.name}</span>
						{/if}
						{#if lesson.duration}
							<span class="meta-item">
								<IconClock size={14} />
								{lesson.duration}
							</span>
						{/if}
						{#if publishedDate}
							<span class="meta-item">
								<IconCalendar size={14} />
								{publishedDate}
							</span>
						{/if}
					</div>

					<h1 class="lesson-title">{lesson.title}</h1>

					<!-- Trainer inline -->
					{#if trainer}
						<div class="trainer-inline">
							{#if trainer.thumbnailUrl || trainer.imageUrl}
								<img
									src={trainer.thumbnailUrl || trainer.imageUrl}
									alt={trainer.name}
									class="trainer-avatar"
								/>
							{/if}
							<div class="trainer-info">
								<span class="trainer-name">{trainer.name}</span>
								<span class="trainer-title">{trainer.title}</span>
							</div>
						</div>
					{/if}

					<!-- Progress bar -->
					{#if progressPercent > 0 || isCompleted}
						<div class="progress-indicator">
							<div class="progress-bar">
								<div class="progress-fill" style="width: {isCompleted ? 100 : progressPercent}%"></div>
							</div>
							<span class="progress-text">
								{#if isCompleted}
									<IconCheck size={14} />
									Completed
								{:else}
									{progressPercent}% complete
								{/if}
							</span>
						</div>
					{/if}

					<!-- Description -->
					{#if lesson.fullDescription || lesson.description}
						<div class="lesson-description">
							<h2>About this lesson</h2>
							<p>{lesson.fullDescription || lesson.description}</p>
						</div>
					{/if}

					<!-- Resources -->
					{#if lesson.resources && lesson.resources.length > 0}
						<div class="lesson-resources">
							<h2>Resources</h2>
							<div class="resources-list">
								{#each lesson.resources as resource}
									<a href={resource.fileUrl} class="resource-item" download>
										<div class="resource-icon">
											<IconDownload size={18} />
										</div>
										<div class="resource-info">
											<span class="resource-name">{resource.name}</span>
											{#if resource.description}
												<span class="resource-desc">{resource.description}</span>
											{/if}
										</div>
									</a>
								{/each}
							</div>
						</div>
					{/if}

					<!-- Complete button -->
					{#if !isCompleted}
						<button
							type="button"
							class="btn-complete"
							onclick={handleMarkComplete}
							disabled={isMarkingComplete}
						>
							<IconCheck size={18} />
							{isMarkingComplete ? 'Marking...' : 'Mark as Complete'}
						</button>
					{:else}
						<div class="completion-badge">
							<IconCheck size={20} />
							<span>You've completed this lesson!</span>
						</div>
					{/if}
				</div>

				<!-- Navigation -->
				<nav class="lesson-nav">
					{#if prevLesson}
						<a href="/dashboard/{roomSlug}/learning-center/{prevLesson.slug}" class="nav-link prev">
							<IconArrowLeft size={18} />
							<div class="nav-info">
								<span class="nav-label">Previous</span>
								<span class="nav-title">{prevLesson.title}</span>
							</div>
						</a>
					{:else}
						<div></div>
					{/if}

					{#if nextLesson}
						<a href="/dashboard/{roomSlug}/learning-center/{nextLesson.slug}" class="nav-link next">
							<div class="nav-info">
								<span class="nav-label">Next</span>
								<span class="nav-title">{nextLesson.title}</span>
							</div>
							<IconArrowRight size={18} />
						</a>
					{:else}
						<div></div>
					{/if}
				</nav>
			</main>

			<!-- Sidebar -->
			<aside class="lesson-sidebar">
				<!-- Trainer card -->
				{#if trainer}
					<TrainerCard
						{trainer}
						variant="full"
						showBio
						showSpecialties
						showSocialLinks
					/>
				{/if}

				<!-- Related lessons -->
				{#if relatedLessons.length > 0}
					<div class="sidebar-section">
						<h3>Related Lessons</h3>
						<div class="related-lessons">
							{#each relatedLessons as relatedLesson}
								<LessonCard
									lesson={relatedLesson}
									compact
									showTrainer={false}
									href="/dashboard/{roomSlug}/learning-center/{relatedLesson.slug}"
								/>
							{/each}
						</div>
					</div>
				{/if}

				<!-- Back to learning center -->
				<a href="/dashboard/{roomSlug}/learning-center" class="back-link">
					<IconArrowLeft size={16} />
					Back to all lessons
				</a>
			</aside>
		</div>
	</div>

	<!-- Share Modal -->
	{#if showShareModal}
		<div class="modal-overlay" onclick={() => showShareModal = false}>
			<div class="modal" onclick={(e) => e.stopPropagation()}>
				<h3>Share this lesson</h3>
				<div class="share-url">
					<input type="text" readonly value={browser ? window.location.href : ''} />
					<button type="button" onclick={copyLink}>Copy</button>
				</div>
				<button type="button" class="modal-close" onclick={() => showShareModal = false}>
					Close
				</button>
			</div>
		</div>
	{/if}
{/if}

<!-- ═══════════════════════════════════════════════════════════════════════════
     STYLES
     ═══════════════════════════════════════════════════════════════════════════ -->

<style>
	.lesson-page {
		background: #0f172a;
		min-height: 100vh;
	}

	/* Header */
	.lesson-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 16px 32px;
		background: #1e293b;
		border-bottom: 1px solid #334155;
	}

	.breadcrumb {
		display: flex;
		align-items: center;
		gap: 8px;
		font-size: 13px;
		color: #64748b;
	}

	.breadcrumb a {
		color: #94a3b8;
		text-decoration: none;
		transition: color 0.2s;
	}

	.breadcrumb a:hover {
		color: #f97316;
	}

	.breadcrumb .current {
		color: white;
		max-width: 200px;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.header-actions {
		display: flex;
		gap: 8px;
	}

	.action-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 40px;
		height: 40px;
		background: #334155;
		border: none;
		border-radius: 8px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.action-btn:hover {
		background: #475569;
		color: white;
	}

	.action-btn.active {
		background: #f97316;
		color: white;
	}

	/* Layout */
	.lesson-layout {
		display: grid;
		grid-template-columns: 1fr 340px;
		gap: 32px;
		max-width: 1400px;
		margin: 0 auto;
		padding: 32px;
	}

	/* Main content */
	.lesson-main {
		display: flex;
		flex-direction: column;
		gap: 24px;
	}

	/* Video container */
	.video-container {
		border-radius: 12px;
		overflow: hidden;
		background: #000;
	}

	/* Lesson info */
	.lesson-info {
		background: #1e293b;
		border-radius: 12px;
		padding: 24px;
		border: 1px solid #334155;
	}

	.lesson-meta {
		display: flex;
		flex-wrap: wrap;
		gap: 12px;
		margin-bottom: 16px;
	}

	.category-tag {
		padding: 4px 12px;
		background: rgba(249, 115, 22, 0.1);
		border-radius: 9999px;
		font-size: 0.75rem;
		font-weight: 600;
		color: #f97316;
		text-transform: uppercase;
	}

	.meta-item {
		display: flex;
		align-items: center;
		gap: 4px;
		font-size: 0.875rem;
		color: #64748b;
	}

	.lesson-title {
		margin: 0 0 16px;
		font-size: 1.75rem;
		font-weight: 700;
		color: white;
		line-height: 1.3;
	}

	/* Trainer inline */
	.trainer-inline {
		display: flex;
		align-items: center;
		gap: 12px;
		margin-bottom: 16px;
		padding-bottom: 16px;
		border-bottom: 1px solid #334155;
	}

	.trainer-avatar {
		width: 48px;
		height: 48px;
		border-radius: 50%;
		object-fit: cover;
		border: 2px solid #f97316;
	}

	.trainer-info {
		display: flex;
		flex-direction: column;
	}

	.trainer-name {
		font-size: 0.9rem;
		font-weight: 600;
		color: white;
	}

	.trainer-title {
		font-size: 0.8rem;
		color: #94a3b8;
	}

	/* Progress indicator */
	.progress-indicator {
		display: flex;
		align-items: center;
		gap: 12px;
		margin-bottom: 16px;
	}

	.progress-bar {
		flex: 1;
		height: 6px;
		background: #334155;
		border-radius: 3px;
		overflow: hidden;
	}

	.progress-fill {
		height: 100%;
		background: linear-gradient(90deg, #f97316, #ea580c);
		transition: width 0.3s ease;
	}

	.progress-text {
		display: flex;
		align-items: center;
		gap: 4px;
		font-size: 0.8rem;
		color: #22c55e;
		font-weight: 500;
	}

	/* Description */
	.lesson-description h2 {
		margin: 0 0 12px;
		font-size: 1rem;
		font-weight: 600;
		color: white;
	}

	.lesson-description p {
		margin: 0;
		font-size: 0.9rem;
		color: #cbd5e1;
		line-height: 1.7;
	}

	/* Resources */
	.lesson-resources {
		margin-top: 24px;
		padding-top: 24px;
		border-top: 1px solid #334155;
	}

	.lesson-resources h2 {
		margin: 0 0 16px;
		font-size: 1rem;
		font-weight: 600;
		color: white;
	}

	.resources-list {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.resource-item {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 12px;
		background: #0f172a;
		border-radius: 8px;
		text-decoration: none;
		transition: background 0.2s ease;
	}

	.resource-item:hover {
		background: #1e293b;
	}

	.resource-icon {
		width: 40px;
		height: 40px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(249, 115, 22, 0.1);
		border-radius: 8px;
		color: #f97316;
	}

	.resource-info {
		display: flex;
		flex-direction: column;
	}

	.resource-name {
		font-size: 0.875rem;
		font-weight: 500;
		color: white;
	}

	.resource-desc {
		font-size: 0.75rem;
		color: #64748b;
	}

	/* Complete button */
	.btn-complete {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		width: 100%;
		margin-top: 24px;
		padding: 14px;
		background: linear-gradient(135deg, #f97316, #ea580c);
		border: none;
		border-radius: 8px;
		color: white;
		font-size: 1rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.btn-complete:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow: 0 10px 20px rgba(249, 115, 22, 0.3);
	}

	.btn-complete:disabled {
		opacity: 0.7;
		cursor: not-allowed;
	}

	.completion-badge {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		margin-top: 24px;
		padding: 14px;
		background: rgba(34, 197, 94, 0.1);
		border: 1px solid rgba(34, 197, 94, 0.3);
		border-radius: 8px;
		color: #22c55e;
		font-weight: 600;
	}

	/* Navigation */
	.lesson-nav {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 16px;
	}

	.nav-link {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 16px;
		background: #1e293b;
		border-radius: 12px;
		border: 1px solid #334155;
		text-decoration: none;
		transition: all 0.2s ease;
	}

	.nav-link:hover {
		border-color: #f97316;
		background: #253346;
	}

	.nav-link.next {
		justify-content: flex-end;
		text-align: right;
	}

	.nav-link.next .nav-info {
		align-items: flex-end;
	}

	.nav-info {
		display: flex;
		flex-direction: column;
		gap: 2px;
	}

	.nav-label {
		font-size: 0.75rem;
		color: #64748b;
		text-transform: uppercase;
	}

	.nav-title {
		font-size: 0.9rem;
		font-weight: 500;
		color: white;
		display: -webkit-box;
		-webkit-line-clamp: 1;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	/* Sidebar */
	.lesson-sidebar {
		display: flex;
		flex-direction: column;
		gap: 24px;
	}

	.sidebar-section {
		background: #1e293b;
		border-radius: 12px;
		padding: 20px;
		border: 1px solid #334155;
	}

	.sidebar-section h3 {
		margin: 0 0 16px;
		font-size: 0.9rem;
		font-weight: 600;
		color: white;
	}

	.related-lessons {
		display: flex;
		flex-direction: column;
		gap: 12px;
	}

	.back-link {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 12px 16px;
		background: #1e293b;
		border-radius: 8px;
		border: 1px solid #334155;
		color: #94a3b8;
		text-decoration: none;
		font-size: 0.875rem;
		transition: all 0.2s ease;
	}

	.back-link:hover {
		border-color: #f97316;
		color: white;
	}

	/* Error state */
	.error-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		min-height: 400px;
		text-align: center;
		padding: 40px;
		color: #64748b;
	}

	.error-state h3 {
		margin: 16px 0 8px;
		color: white;
		font-size: 1.25rem;
	}

	.error-state p {
		margin: 0 0 24px;
	}

	.btn-back {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		padding: 10px 20px;
		background: #f97316;
		color: white;
		text-decoration: none;
		border-radius: 8px;
		font-weight: 500;
		transition: background 0.2s ease;
	}

	.btn-back:hover {
		background: #ea580c;
	}

	/* Modal */
	.modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.7);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
	}

	.modal {
		background: #1e293b;
		border-radius: 12px;
		padding: 24px;
		max-width: 400px;
		width: 90%;
	}

	.modal h3 {
		margin: 0 0 16px;
		font-size: 1.125rem;
		color: white;
	}

	.share-url {
		display: flex;
		gap: 8px;
		margin-bottom: 16px;
	}

	.share-url input {
		flex: 1;
		padding: 10px 12px;
		background: #0f172a;
		border: 1px solid #334155;
		border-radius: 6px;
		color: white;
		font-size: 0.875rem;
	}

	.share-url button {
		padding: 10px 16px;
		background: #f97316;
		border: none;
		border-radius: 6px;
		color: white;
		font-weight: 500;
		cursor: pointer;
	}

	.modal-close {
		width: 100%;
		padding: 10px;
		background: #334155;
		border: none;
		border-radius: 6px;
		color: white;
		cursor: pointer;
	}

	/* Responsive */
	@media (max-width: 1024px) {
		.lesson-layout {
			grid-template-columns: 1fr;
		}

		.lesson-sidebar {
			order: -1;
		}
	}

	@media (max-width: 768px) {
		.lesson-header {
			padding: 12px 16px;
		}

		.breadcrumb {
			display: none;
		}

		.lesson-layout {
			padding: 16px;
		}

		.lesson-title {
			font-size: 1.5rem;
		}

		.lesson-nav {
			grid-template-columns: 1fr;
		}
	}
</style>

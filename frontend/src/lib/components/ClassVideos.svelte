<!--
/**
 * ClassVideos Component - Course Video Player with Lesson List
 * ═══════════════════════════════════════════════════════════════════════════
 * Apple ICT 11+ Principal Engineer Grade - January 2026
 * 
 * Features:
 * - SSR-safe: Uses $effect for client-side only operations
 * - Hydration-safe: Proper browser checks and state management
 * - SEO-ready: Accepts pre-loaded data from server
 * - Bunny.net integration with BunnyVideoPlayer
 * - Module-organized lesson list with progress tracking
 * - Mobile-first responsive design
 * 
 * @version 1.0.0
 */
-->
<script lang="ts">
	import { browser } from '$app/environment';
	import BunnyVideoPlayer from '$lib/components/video/BunnyVideoPlayer.svelte';

	interface Lesson {
		id: string;
		title: string;
		slug?: string;
		description?: string;
		duration_minutes?: number;
		position?: number;
		sort_order?: number;
		is_free?: boolean;
		is_preview?: boolean;
		is_published?: boolean;
		bunny_video_guid?: string;
		thumbnail_url?: string;
		module_id?: number;
	}

	interface Module {
		id: number;
		title: string;
		description?: string;
		sort_order: number;
		lesson_count?: number;
		total_duration_minutes?: number;
	}

	interface ModuleWithLessons {
		module: Module;
		lessons: Lesson[];
	}

	interface CourseData {
		course: {
			id: string;
			title: string;
			slug: string;
			bunny_library_id?: number;
		};
		modules: ModuleWithLessons[];
		lessons: Lesson[];
		downloads?: unknown[];
		enrollment?: unknown;
		progress?: unknown[];
	}

	interface Props {
		courseSlug: string;
		title?: string;
		bunnyLibraryId?: string | number;
		maxHeight?: string;
		initialData?: CourseData | null;
	}

	let {
		courseSlug,
		title = 'Course Videos',
		bunnyLibraryId = '',
		maxHeight = '600px',
		initialData = null
	}: Props = $props();

	let courseData = $state<CourseData | null>(null);
	let loading = $state(true);
	let error = $state('');
	let activeLesson = $state<Lesson | null>(null);
	let viewportWidth = $state(0);
	let expandedModules = $state<Set<number>>(new Set());
	let mounted = $state(false);

	const isMobile = $derived(mounted && viewportWidth > 0 && viewportWidth < 640);

	const effectiveLibraryId = $derived(
		bunnyLibraryId || courseData?.course?.bunny_library_id || '390057'
	);

	const allLessons = $derived.by(() => {
		if (!courseData) return [];
		if (courseData.modules && courseData.modules.length > 0) {
			return courseData.modules.flatMap((m) => m.lessons || []);
		}
		return courseData.lessons || [];
	});

	const hasModules = $derived(courseData?.modules && courseData.modules.length > 0);

	$effect(() => {
		if (initialData && !courseData) {
			courseData = initialData;
			loading = false;
			initializeFromData();
		}
	});

	$effect(() => {
		if (!browser) return;

		mounted = true;
		viewportWidth = window.innerWidth;

		const handleResize = () => {
			viewportWidth = window.innerWidth;
		};
		window.addEventListener('resize', handleResize, { passive: true });

		if (!initialData) {
			fetchCourseData();
		}

		return () => window.removeEventListener('resize', handleResize);
	});

	function initializeFromData() {
		if (!courseData) return;

		const lessons = allLessons;
		if (lessons.length > 0 && !activeLesson) {
			activeLesson = lessons[0];
		}

		if (courseData.modules) {
			const newSet = new Set<number>();
			courseData.modules.forEach((m) => newSet.add(m.module.id));
			expandedModules = newSet;
		}
	}

	async function fetchCourseData() {
		loading = true;
		error = '';

		try {
			const res = await fetch(`/api/courses/slug/${courseSlug}/player`);
			const data = await res.json();

			if (data.success && data.data) {
				courseData = data.data;
				initializeFromData();
			} else {
				error = data.error || 'Failed to load course data';
			}
		} catch (err) {
			console.error('[ClassVideos] Error fetching course:', err);
			error = 'Unable to load course videos';
		} finally {
			loading = false;
		}
	}

	function selectLesson(lesson: Lesson) {
		activeLesson = lesson;
	}

	function toggleModule(moduleId: number) {
		const newSet = new Set(expandedModules);
		if (newSet.has(moduleId)) {
			newSet.delete(moduleId);
		} else {
			newSet.add(moduleId);
		}
		expandedModules = newSet;
	}

	function formatDuration(minutes?: number): string {
		if (!minutes) return '';
		const hrs = Math.floor(minutes / 60);
		const mins = minutes % 60;
		return hrs > 0 ? `${hrs}h ${mins}m` : `${mins} min`;
	}

	function getLessonIndex(lesson: Lesson): number {
		return allLessons.findIndex((l) => l.id === lesson.id) + 1;
	}
</script>

<div class="class-videos-wrapper">
	<div class="videos-header">
		<h2>{title}</h2>
		{#if courseData?.course?.title}
			<span class="course-title">{courseData.course.title}</span>
		{/if}
	</div>

	<div class="videos-container" style="max-height: {maxHeight}">
		{#if loading}
			<div class="loading-state">
				<div class="spinner"></div>
				<span>Loading videos...</span>
			</div>
		{:else if error}
			<div class="error-state">
				<span>⚠️ {error}</span>
			</div>
		{:else}
			<div class="videos-layout" class:mobile={isMobile}>
				<!-- Video Player Section -->
				<div class="video-player-section">
					{#if activeLesson}
						<div class="active-video">
							{#if activeLesson.bunny_video_guid && effectiveLibraryId && mounted}
								<BunnyVideoPlayer
									videoGuid={activeLesson.bunny_video_guid}
									libraryId={String(effectiveLibraryId)}
									title={activeLesson.title}
									thumbnailUrl={activeLesson.thumbnail_url}
									controls={true}
									preload={true}
								/>
							{:else if !activeLesson.bunny_video_guid}
								<div class="no-video-placeholder">
									<svg
										xmlns="http://www.w3.org/2000/svg"
										width="64"
										height="64"
										viewBox="0 0 24 24"
										fill="none"
										stroke="currentColor"
										stroke-width="1.5"
									>
										<polygon points="23 7 16 12 23 17 23 7" />
										<rect x="1" y="5" width="15" height="14" rx="2" ry="2" />
									</svg>
									<span>Video coming soon</span>
								</div>
							{:else}
								<div class="video-loading">
									<div class="spinner"></div>
								</div>
							{/if}
						</div>
						<div class="active-video-info">
							<h3>{activeLesson.title}</h3>
							{#if activeLesson.description}
								<p class="lesson-description">{activeLesson.description}</p>
							{/if}
							{#if activeLesson.duration_minutes}
								<span class="lesson-duration">{formatDuration(activeLesson.duration_minutes)}</span>
							{/if}
						</div>
					{:else}
						<div class="empty-player">
							<svg
								xmlns="http://www.w3.org/2000/svg"
								width="64"
								height="64"
								viewBox="0 0 24 24"
								fill="none"
								stroke="currentColor"
								stroke-width="1.5"
							>
								<polygon points="23 7 16 12 23 17 23 7" />
								<rect x="1" y="5" width="15" height="14" rx="2" ry="2" />
							</svg>
							<span class="empty-message">No videos available</span>
							<span class="empty-submessage">This course doesn't have any video lessons yet</span>
						</div>
					{/if}
				</div>

				<!-- Lesson List Section -->
				<div class="lesson-list-section">
					<div class="lesson-list-header">
						<span class="lesson-count">{allLessons.length} lessons</span>
					</div>

					<div class="lesson-list">
						{#if hasModules}
							{#each courseData?.modules || [] as moduleData (moduleData.module.id)}
								<div class="module-group">
									<button
										class="module-header"
										onclick={() => toggleModule(moduleData.module.id)}
										aria-expanded={expandedModules.has(moduleData.module.id)}
									>
										<div class="module-info">
											<span class="module-title">{moduleData.module.title}</span>
											<span class="module-meta">
												{moduleData.lessons?.length || 0} lessons
												{#if moduleData.module.total_duration_minutes}
													• {formatDuration(moduleData.module.total_duration_minutes)}
												{/if}
											</span>
										</div>
										<svg
											class="chevron"
											class:expanded={expandedModules.has(moduleData.module.id)}
											xmlns="http://www.w3.org/2000/svg"
											width="20"
											height="20"
											viewBox="0 0 24 24"
											fill="none"
											stroke="currentColor"
											stroke-width="2"
										>
											<polyline points="6 9 12 15 18 9" />
										</svg>
									</button>

									{#if expandedModules.has(moduleData.module.id)}
										<div class="module-lessons">
											{#each moduleData.lessons || [] as lesson (lesson.id)}
												<button
													class="lesson-item"
													class:active={activeLesson?.id === lesson.id}
													class:has-video={!!lesson.bunny_video_guid}
													onclick={() => selectLesson(lesson)}
												>
													<span class="lesson-number">{getLessonIndex(lesson)}</span>
													<div class="lesson-info">
														<span class="lesson-title">{lesson.title}</span>
														{#if lesson.duration_minutes}
															<span class="lesson-meta"
																>{formatDuration(lesson.duration_minutes)}</span
															>
														{/if}
													</div>
													{#if lesson.is_preview}
														<span class="preview-badge">Preview</span>
													{/if}
													<svg
														class="play-icon"
														xmlns="http://www.w3.org/2000/svg"
														width="16"
														height="16"
														viewBox="0 0 24 24"
														fill="currentColor"
													>
														<polygon points="5 3 19 12 5 21 5 3" />
													</svg>
												</button>
											{/each}
										</div>
									{/if}
								</div>
							{/each}
						{:else}
							{#each allLessons as lesson, index (lesson.id)}
								<button
									class="lesson-item"
									class:active={activeLesson?.id === lesson.id}
									class:has-video={!!lesson.bunny_video_guid}
									onclick={() => selectLesson(lesson)}
								>
									<span class="lesson-number">{index + 1}</span>
									<div class="lesson-info">
										<span class="lesson-title">{lesson.title}</span>
										{#if lesson.duration_minutes}
											<span class="lesson-meta">{formatDuration(lesson.duration_minutes)}</span>
										{/if}
									</div>
									{#if lesson.is_preview}
										<span class="preview-badge">Preview</span>
									{/if}
									<svg
										class="play-icon"
										xmlns="http://www.w3.org/2000/svg"
										width="16"
										height="16"
										viewBox="0 0 24 24"
										fill="currentColor"
									>
										<polygon points="5 3 19 12 5 21 5 3" />
									</svg>
								</button>
							{/each}
						{/if}
					</div>
				</div>
			</div>
		{/if}
	</div>
</div>

<style>
	.class-videos-wrapper {
		background-color: #ffffff;
		border-radius: 4px;
		border: 1px solid #e0e0e0;
		overflow: hidden;
		font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
	}

	.videos-header {
		padding: 16px 20px;
		border-bottom: 1px solid #e0e0e0;
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 12px;
	}

	.videos-header h2 {
		font-size: 1.1rem;
		font-weight: 600;
		color: #333333;
		margin: 0;
	}

	.course-title {
		font-size: 0.875rem;
		color: #666666;
	}

	.videos-container {
		overflow: hidden;
	}

	.videos-layout {
		display: grid;
		grid-template-columns: 1fr 320px;
		min-height: 400px;
	}

	.videos-layout.mobile {
		grid-template-columns: 1fr;
	}

	/* Video Player Section */
	.video-player-section {
		background: #000;
		display: flex;
		flex-direction: column;
	}

	.active-video {
		width: 100%;
		aspect-ratio: 16 / 9;
		background: #000;
	}

	.active-video-info {
		padding: 16px 20px;
		background: #1a1a1a;
		color: #fff;
	}

	.active-video-info h3 {
		font-size: 1rem;
		font-weight: 600;
		margin: 0 0 8px 0;
	}

	.lesson-description {
		font-size: 0.875rem;
		color: #ccc;
		margin: 0 0 8px 0;
		line-height: 1.5;
	}

	.lesson-duration {
		font-size: 0.75rem;
		color: #999;
	}

	.no-video-placeholder,
	.empty-player,
	.video-loading {
		width: 100%;
		height: 100%;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 16px;
		color: #666;
		background: #1a1a1a;
		min-height: 300px;
	}

	.empty-player svg,
	.no-video-placeholder svg {
		color: #444;
	}

	.empty-message {
		font-size: 1rem;
		font-weight: 600;
		color: #888;
	}

	.empty-submessage {
		font-size: 0.875rem;
		color: #666;
	}

	/* Lesson List Section */
	.lesson-list-section {
		background: #fafafa;
		border-left: 1px solid #e0e0e0;
		display: flex;
		flex-direction: column;
		max-height: 600px;
	}

	.videos-layout.mobile .lesson-list-section {
		border-left: none;
		border-top: 1px solid #e0e0e0;
		max-height: 300px;
	}

	.lesson-list-header {
		padding: 12px 16px;
		border-bottom: 1px solid #e0e0e0;
		background: #fff;
	}

	.lesson-count {
		font-size: 0.75rem;
		font-weight: 600;
		color: #666;
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.lesson-list {
		overflow-y: auto;
		flex: 1;
	}

	/* Module Styles */
	.module-group {
		border-bottom: 1px solid #e0e0e0;
	}

	.module-header {
		width: 100%;
		padding: 12px 16px;
		display: flex;
		align-items: center;
		justify-content: space-between;
		background: #fff;
		border: none;
		cursor: pointer;
		text-align: left;
		transition: background 0.15s ease;
	}

	.module-header:hover {
		background: #f5f5f5;
	}

	.module-info {
		display: flex;
		flex-direction: column;
		gap: 2px;
	}

	.module-title {
		font-size: 0.875rem;
		font-weight: 600;
		color: #333;
	}

	.module-meta {
		font-size: 0.75rem;
		color: #888;
	}

	.chevron {
		color: #666;
		transition: transform 0.2s ease;
	}

	.chevron.expanded {
		transform: rotate(180deg);
	}

	.module-lessons {
		background: #fafafa;
	}

	/* Lesson Item */
	.lesson-item {
		width: 100%;
		padding: 12px 16px 12px 24px;
		display: flex;
		align-items: center;
		gap: 12px;
		background: transparent;
		border: none;
		border-bottom: 1px solid #eee;
		cursor: pointer;
		text-align: left;
		transition: background 0.15s ease;
	}

	.lesson-item:hover {
		background: #f0f0f0;
	}

	.lesson-item.active {
		background: #e3f2fd;
		border-left: 3px solid #1976d2;
		padding-left: 21px;
	}

	.lesson-item:not(.has-video) {
		opacity: 0.6;
	}

	.lesson-number {
		width: 24px;
		height: 24px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: #e0e0e0;
		border-radius: 50%;
		font-size: 0.75rem;
		font-weight: 600;
		color: #666;
		flex-shrink: 0;
	}

	.lesson-item.active .lesson-number {
		background: #1976d2;
		color: #fff;
	}

	.lesson-info {
		flex: 1;
		min-width: 0;
	}

	.lesson-title {
		display: block;
		font-size: 0.875rem;
		color: #333;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.lesson-meta {
		font-size: 0.75rem;
		color: #888;
	}

	.preview-badge {
		font-size: 0.625rem;
		font-weight: 600;
		color: #1976d2;
		background: #e3f2fd;
		padding: 2px 6px;
		border-radius: 4px;
		text-transform: uppercase;
	}

	.play-icon {
		color: #999;
		flex-shrink: 0;
	}

	.lesson-item.active .play-icon {
		color: #1976d2;
	}

	/* States */
	.loading-state,
	.error-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 60px 24px;
		color: #666666;
		gap: 12px;
		min-height: 400px;
	}

	.spinner {
		width: 32px;
		height: 32px;
		border: 3px solid #e0e0e0;
		border-top-color: #1976d2;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   2026 MOBILE-FIRST RESPONSIVE DESIGN
	   ═══════════════════════════════════════════════════════════════════════════
	   Breakpoints: xs(360px), sm(640px), md(768px), lg(1024px), xl(1280px)
	   Touch targets: 44x44px minimum for all interactive elements
	   ═══════════════════════════════════════════════════════════════════════════ */

	/* Mobile base styles (default) - Single column layout */
	.videos-layout {
		grid-template-columns: 1fr;
	}

	.videos-header {
		padding: 12px 14px;
		flex-direction: column;
		align-items: flex-start;
		gap: 4px;
	}

	.videos-header h2 {
		font-size: 1rem;
	}

	.course-title {
		font-size: 0.8rem;
	}

	/* Lesson list section on mobile - below video */
	.lesson-list-section {
		border-left: none;
		border-top: 1px solid #e0e0e0;
		max-height: 280px;
	}

	/* Module header - 44px touch target */
	.module-header {
		min-height: 44px;
		padding: 10px 14px;
		-webkit-tap-highlight-color: transparent;
		touch-action: manipulation;
	}

	.module-title {
		font-size: 0.8rem;
	}

	.module-meta {
		font-size: 0.7rem;
	}

	/* Lesson items - 44px min height touch target */
	.lesson-item {
		min-height: 44px;
		padding: 10px 14px 10px 20px;
		gap: 10px;
		-webkit-tap-highlight-color: transparent;
		touch-action: manipulation;
	}

	.lesson-item.active {
		padding-left: 17px;
	}

	.lesson-number {
		width: 22px;
		height: 22px;
		font-size: 0.7rem;
	}

	.lesson-title {
		font-size: 0.8rem;
	}

	.lesson-meta {
		font-size: 0.7rem;
	}

	.play-icon {
		width: 14px;
		height: 14px;
	}

	.active-video-info {
		padding: 12px 14px;
	}

	.active-video-info h3 {
		font-size: 0.9rem;
	}

	.lesson-description {
		font-size: 0.8rem;
	}

	/* xs: 360px+ */
	@media (min-width: 360px) {
		.videos-header {
			padding: 14px 16px;
			gap: 6px;
		}

		.module-header {
			padding: 11px 16px;
		}

		.module-title {
			font-size: 0.85rem;
		}

		.lesson-item {
			padding: 11px 16px 11px 22px;
			gap: 11px;
		}

		.lesson-item.active {
			padding-left: 19px;
		}

		.lesson-title {
			font-size: 0.85rem;
		}

		.active-video-info {
			padding: 14px 16px;
		}

		.active-video-info h3 {
			font-size: 0.95rem;
		}
	}

	/* sm: 640px+ */
	@media (min-width: 640px) {
		.videos-header {
			padding: 16px 20px;
			flex-direction: row;
			align-items: center;
			justify-content: space-between;
			gap: 12px;
		}

		.videos-header h2 {
			font-size: 1.1rem;
		}

		.course-title {
			font-size: 0.875rem;
		}

		.lesson-list-section {
			max-height: 300px;
		}

		.module-header {
			padding: 12px 16px;
		}

		.module-header:hover {
			background: #f5f5f5;
		}

		.module-title {
			font-size: 0.875rem;
		}

		.module-meta {
			font-size: 0.75rem;
		}

		.lesson-item {
			padding: 12px 16px 12px 24px;
			gap: 12px;
		}

		.lesson-item:hover {
			background: #f0f0f0;
		}

		.lesson-item.active {
			padding-left: 21px;
		}

		.lesson-number {
			width: 24px;
			height: 24px;
			font-size: 0.75rem;
		}

		.lesson-title {
			font-size: 0.875rem;
		}

		.lesson-meta {
			font-size: 0.75rem;
		}

		.play-icon {
			width: 16px;
			height: 16px;
		}

		.active-video-info {
			padding: 16px 20px;
		}

		.active-video-info h3 {
			font-size: 1rem;
		}

		.lesson-description {
			font-size: 0.875rem;
		}
	}

	/* md: 768px+ - Two column layout */
	@media (min-width: 768px) {
		.videos-layout {
			grid-template-columns: 1fr 280px;
		}

		.lesson-list-section {
			border-left: 1px solid #e0e0e0;
			border-top: none;
			max-height: 500px;
		}
	}

	/* lg: 1024px+ */
	@media (min-width: 1024px) {
		.videos-layout {
			grid-template-columns: 1fr 320px;
		}

		.lesson-list-section {
			max-height: 600px;
		}
	}

	/* xl: 1280px+ */
	@media (min-width: 1280px) {
		.videos-layout {
			grid-template-columns: 1fr 360px;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ACCESSIBILITY
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (prefers-reduced-motion: reduce) {
		.spinner {
			animation: none;
		}

		.module-header,
		.lesson-item,
		.chevron {
			transition: none;
		}
	}

	.module-header:focus-visible,
	.lesson-item:focus-visible {
		outline: 2px solid #1976d2;
		outline-offset: -2px;
	}

	/* Scrollbar styling for lesson list */
	.lesson-list::-webkit-scrollbar {
		width: 6px;
	}

	.lesson-list::-webkit-scrollbar-track {
		background: #f0f0f0;
	}

	.lesson-list::-webkit-scrollbar-thumb {
		background: #ccc;
		border-radius: 3px;
	}

	.lesson-list::-webkit-scrollbar-thumb:hover {
		background: #bbb;
	}
</style>

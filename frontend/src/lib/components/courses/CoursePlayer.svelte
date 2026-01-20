<script lang="ts">
	/**
	 * CoursePlayer Component
	 * Apple Principal Engineer ICT 7 Grade - January 2026
	 * Full course player with video, lesson navigation, and downloads
	 */

	import { BunnyVideoPlayer } from '$lib/components/video';

	interface Lesson {
		id: string;
		title: string;
		slug: string;
		description?: string;
		duration_minutes?: number;
		bunny_video_guid?: string;
		thumbnail_url?: string;
		is_free?: boolean;
		is_preview?: boolean;
	}

	interface Module {
		id: number;
		title: string;
		description?: string;
		lessons: Lesson[];
	}

	interface Download {
		id: number;
		title: string;
		file_name: string;
		file_size_bytes?: number;
		download_url?: string;
		category?: string;
	}

	interface Course {
		id: string;
		title: string;
		bunny_library_id?: number;
	}

	interface Progress {
		lesson_id: string;
		is_completed: boolean;
		video_position_seconds?: number;
	}

	interface Props {
		course: Course;
		modules: Module[];
		downloads: Download[];
		currentLesson: Lesson;
		progress: Progress[];
		isEnrolled?: boolean;
		onLessonChange?: (lesson: Lesson) => void;
		onProgress?: (lessonId: string, position: number, completed: boolean) => void;
	}

	let {
		course,
		modules,
		downloads,
		currentLesson,
		progress = [],
		isEnrolled = false,
		onLessonChange,
		onProgress
	}: Props = $props();

	let sidebarOpen = $state(true);
	let activeTab = $state<'lessons' | 'downloads'>('lessons');

	const isLessonCompleted = (lessonId: string): boolean => {
		return progress.find((p) => p.lesson_id === lessonId)?.is_completed ?? false;
	};

	const getLessonProgress = (lessonId: string): number => {
		const p = progress.find((p) => p.lesson_id === lessonId);
		if (!p?.video_position_seconds) return 0;
		return p.video_position_seconds;
	};

	const formatDuration = (minutes?: number): string => {
		if (!minutes) return '';
		const h = Math.floor(minutes / 60);
		const m = minutes % 60;
		return h > 0 ? `${h}h ${m}m` : `${m}m`;
	};

	const formatFileSize = (bytes?: number): string => {
		if (!bytes) return '';
		if (bytes < 1024) return `${bytes} B`;
		if (bytes < 1048576) return `${(bytes / 1024).toFixed(1)} KB`;
		return `${(bytes / 1048576).toFixed(1)} MB`;
	};

	const handleLessonClick = (lesson: Lesson) => {
		if (!isEnrolled && !lesson.is_free && !lesson.is_preview) return;
		onLessonChange?.(lesson);
	};

	const handleVideoProgress = (position: number, duration: number) => {
		const completed = duration > 0 && position / duration >= 0.9;
		onProgress?.(currentLesson.id, position, completed);
	};

	const completedCount = $derived(progress.filter((p) => p.is_completed).length);
	const totalLessons = $derived(modules.reduce((acc, m) => acc + m.lessons.length, 0));
	const overallProgress = $derived(
		totalLessons > 0 ? Math.round((completedCount / totalLessons) * 100) : 0
	);
</script>

<div class="player" class:sidebar-closed={!sidebarOpen}>
	<div class="main">
		<div class="video-container">
			{#if currentLesson.bunny_video_guid && course.bunny_library_id}
				<BunnyVideoPlayer
					videoGuid={currentLesson.bunny_video_guid}
					libraryId={course.bunny_library_id}
					title={currentLesson.title}
				/>
			{:else}
				<div class="no-video">
					<svg
						xmlns="http://www.w3.org/2000/svg"
						width="48"
						height="48"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
						><path
							d="m16 13 5.223 3.482a.5.5 0 0 0 .777-.416V7.87a.5.5 0 0 0-.752-.432L16 10.5"
						/><rect x="2" y="6" width="14" height="12" rx="2" /></svg
					>
					<p>No video available for this lesson</p>
				</div>
			{/if}
		</div>
		<div class="lesson-info">
			<h1>{currentLesson.title}</h1>
			{#if currentLesson.description}
				<p>{currentLesson.description}</p>
			{/if}
		</div>
	</div>

	<aside class="sidebar">
		<button class="toggle" onclick={() => (sidebarOpen = !sidebarOpen)} aria-label="Toggle sidebar">
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="20"
				height="20"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"><path d="m15 18-6-6 6-6" /></svg
			>
		</button>

		<div class="sidebar-header">
			<h2>{course.title}</h2>
			<div class="progress-bar">
				<div class="fill" style="width: {overallProgress}%"></div>
			</div>
			<span class="progress-text"
				>{overallProgress}% complete ({completedCount}/{totalLessons})</span
			>
		</div>

		<div class="tabs">
			<button class:active={activeTab === 'lessons'} onclick={() => (activeTab = 'lessons')}
				>Lessons</button
			>
			<button class:active={activeTab === 'downloads'} onclick={() => (activeTab = 'downloads')}
				>Downloads</button
			>
		</div>

		{#if activeTab === 'lessons'}
			<div class="lesson-list">
				{#each modules as module}
					<div class="module">
						<h3>{module.title}</h3>
						<ul>
							{#each module.lessons as lesson}
								{@const completed = isLessonCompleted(lesson.id)}
								{@const locked = !isEnrolled && !lesson.is_free && !lesson.is_preview}
								{@const isCurrent = lesson.id === currentLesson.id}
								<li>
									<button
										class="lesson-btn"
										class:active={isCurrent}
										class:completed
										class:locked
										disabled={locked}
										onclick={() => handleLessonClick(lesson)}
									>
										<span class="status">
											{#if completed}
												<svg
													xmlns="http://www.w3.org/2000/svg"
													width="16"
													height="16"
													viewBox="0 0 24 24"
													fill="none"
													stroke="currentColor"
													stroke-width="2"
													><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14" /><polyline
														points="22 4 12 14.01 9 11.01"
													/></svg
												>
											{:else if locked}
												<svg
													xmlns="http://www.w3.org/2000/svg"
													width="16"
													height="16"
													viewBox="0 0 24 24"
													fill="none"
													stroke="currentColor"
													stroke-width="2"
													><rect width="18" height="11" x="3" y="11" rx="2" ry="2" /><path
														d="M7 11V7a5 5 0 0 1 10 0v4"
													/></svg
												>
											{:else}
												<svg
													xmlns="http://www.w3.org/2000/svg"
													width="16"
													height="16"
													viewBox="0 0 24 24"
													fill="none"
													stroke="currentColor"
													stroke-width="2"><polygon points="5 3 19 12 5 21 5 3" /></svg
												>
											{/if}
										</span>
										<span class="lesson-title">{lesson.title}</span>
										{#if lesson.duration_minutes}
											<span class="duration">{formatDuration(lesson.duration_minutes)}</span>
										{/if}
									</button>
								</li>
							{/each}
						</ul>
					</div>
				{/each}
			</div>
		{:else}
			<div class="download-list">
				{#if downloads.length === 0}
					<p class="empty">No downloads available</p>
				{:else}
					{#each downloads as dl}
						<a href={dl.download_url} class="download-item" download={dl.file_name}>
							<svg
								xmlns="http://www.w3.org/2000/svg"
								width="20"
								height="20"
								viewBox="0 0 24 24"
								fill="none"
								stroke="currentColor"
								stroke-width="2"
								><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" /><polyline
									points="7 10 12 15 17 10"
								/><line x1="12" x2="12" y1="15" y2="3" /></svg
							>
							<div class="dl-info">
								<span class="dl-title">{dl.title}</span>
								<span class="dl-meta">{dl.file_name} • {formatFileSize(dl.file_size_bytes)}</span>
							</div>
						</a>
					{/each}
				{/if}
			</div>
		{/if}
	</aside>
</div>

<style>
	.player {
		display: flex;
		/* ICT11+ Fix: Removed min-height: 100vh - let parent flex container handle height */
		flex: 1;
		background: #0f172a;
	}
	.main {
		flex: 1;
		display: flex;
		flex-direction: column;
	}
	.video-container {
		aspect-ratio: 16/9;
		background: #000;
	}
	.no-video {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		height: 100%;
		color: #64748b;
		gap: 16px;
	}
	.lesson-info {
		padding: 24px;
		color: #fff;
	}
	.lesson-info h1 {
		font-size: 24px;
		margin: 0 0 12px;
	}
	.lesson-info p {
		color: #94a3b8;
		line-height: 1.6;
		margin: 0;
	}

	.sidebar {
		width: 360px;
		background: #1e293b;
		display: flex;
		flex-direction: column;
		position: relative;
		transition:
			width 0.3s,
			margin 0.3s;
	}
	.sidebar-closed .sidebar {
		width: 0;
		margin-left: -360px;
	}
	.toggle {
		position: absolute;
		left: -40px;
		top: 20px;
		width: 32px;
		height: 32px;
		background: #1e293b;
		border: none;
		border-radius: 6px 0 0 6px;
		color: #fff;
		cursor: pointer;
		display: flex;
		align-items: center;
		justify-content: center;
	}
	.sidebar-closed .toggle svg {
		transform: rotate(180deg);
	}

	.sidebar-header {
		padding: 20px;
		border-bottom: 1px solid #334155;
	}
	.sidebar-header h2 {
		font-size: 16px;
		color: #fff;
		margin: 0 0 12px;
	}
	.progress-bar {
		height: 6px;
		background: #334155;
		border-radius: 3px;
		overflow: hidden;
	}
	.fill {
		height: 100%;
		background: #10b981;
		transition: width 0.3s;
	}
	.progress-text {
		font-size: 12px;
		color: #94a3b8;
		margin-top: 8px;
		display: block;
	}

	.tabs {
		display: flex;
		border-bottom: 1px solid #334155;
	}
	.tabs button {
		flex: 1;
		padding: 12px;
		background: none;
		border: none;
		color: #94a3b8;
		font-size: 14px;
		cursor: pointer;
		border-bottom: 2px solid transparent;
	}
	.tabs button.active {
		color: #fff;
		border-bottom-color: #10b981;
	}

	.lesson-list {
		flex: 1;
		overflow-y: auto;
	}
	.module {
		border-bottom: 1px solid #334155;
	}
	.module h3 {
		font-size: 13px;
		color: #94a3b8;
		padding: 12px 16px 8px;
		margin: 0;
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}
	.module ul {
		list-style: none;
		margin: 0;
		padding: 0;
	}
	.lesson-btn {
		width: 100%;
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 12px 16px;
		background: none;
		border: none;
		color: #e2e8f0;
		font-size: 14px;
		cursor: pointer;
		text-align: left;
		transition: background 0.2s;
	}
	.lesson-btn:hover:not(:disabled) {
		background: #334155;
	}
	.lesson-btn.active {
		background: #334155;
		border-left: 3px solid #10b981;
	}
	.lesson-btn.completed .status {
		color: #10b981;
	}
	.lesson-btn.locked {
		color: #64748b;
		cursor: not-allowed;
	}
	.status {
		flex-shrink: 0;
	}
	.lesson-title {
		flex: 1;
	}
	.duration {
		font-size: 12px;
		color: #64748b;
	}

	.download-list {
		flex: 1;
		overflow-y: auto;
		padding: 12px;
	}
	.empty {
		text-align: center;
		color: #64748b;
		padding: 24px;
	}
	.download-item {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 12px;
		background: #334155;
		border-radius: 8px;
		margin-bottom: 8px;
		text-decoration: none;
		color: #fff;
		transition: background 0.2s;
	}
	.download-item:hover {
		background: #475569;
	}
	.dl-info {
		display: flex;
		flex-direction: column;
		gap: 2px;
	}
	.dl-title {
		font-size: 14px;
	}
	.dl-meta {
		font-size: 12px;
		color: #94a3b8;
	}

	@media (max-width: 1024px) {
		.sidebar {
			position: fixed;
			right: 0;
			top: 0;
			bottom: 0;
			z-index: 50;
		}
		.sidebar-closed .sidebar {
			margin-left: 0;
			transform: translateX(100%);
		}
		.toggle {
			left: auto;
			right: 100%;
			border-radius: 6px 0 0 6px;
		}
	}
</style>

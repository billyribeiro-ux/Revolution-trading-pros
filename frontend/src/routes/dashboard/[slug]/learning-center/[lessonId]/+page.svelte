<script lang="ts">
	/**
	 * Individual Lesson Page - WordPress Simpler Trading Exact
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * URL: /dashboard/mastering-the-trade/learning-center/5
	 * Shows video player, lesson content, and navigation between lessons.
	 *
	 * Features:
	 * - Video player with progress tracking
	 * - Lesson notes and resources
	 * - Navigation to previous/next lessons
	 * - Mark as complete functionality
	 * - Related lessons sidebar
	 *
	 * @version 1.0.0 (December 2025)
	 */

	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import { browser } from '$app/environment';
	import IconPlayerPlay from '@tabler/icons-svelte/icons/player-play';
	import IconPlayerPause from '@tabler/icons-svelte/icons/player-pause';
	import IconCheck from '@tabler/icons-svelte/icons/check';
	import IconChevronLeft from '@tabler/icons-svelte/icons/chevron-left';
	import IconChevronRight from '@tabler/icons-svelte/icons/chevron-right';
	import IconClock from '@tabler/icons-svelte/icons/clock';
	import IconDownload from '@tabler/icons-svelte/icons/download';
	import IconNotes from '@tabler/icons-svelte/icons/notes';
	import IconList from '@tabler/icons-svelte/icons/list';
	import IconMaximize from '@tabler/icons-svelte/icons/maximize';
	import IconVolume from '@tabler/icons-svelte/icons/volume';
	import IconSettings from '@tabler/icons-svelte/icons/settings';
	import IconBookmark from '@tabler/icons-svelte/icons/bookmark';
	import IconShare from '@tabler/icons-svelte/icons/share';
	import '$lib/styles/st-icons.css';

	// ═══════════════════════════════════════════════════════════════════════════
	// ROUTE PARAMS
	// ═══════════════════════════════════════════════════════════════════════════

	const slug = $derived($page.params.slug);
	const lessonId = $derived(parseInt($page.params['lessonId'] || '1'));

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let isPlaying = $state(false);
	let currentTime = $state(0);
	let duration = $state(0);
	let volume = $state(1);
	let isMuted = $state(false);
	let playbackRate = $state(1);
	let isFullscreen = $state(false);
	let showNotes = $state(false);
	let showPlaylist = $state(true);
	let isCompleted = $state(false);
	let videoElement: HTMLVideoElement;

	// ═══════════════════════════════════════════════════════════════════════════
	// LESSON DATA (Mock - will be fetched from API)
	// ═══════════════════════════════════════════════════════════════════════════

	interface Lesson {
		id: number;
		title: string;
		description: string;
		duration: string;
		durationSeconds: number;
		videoUrl: string;
		thumbnailUrl: string;
		completed: boolean;
		moduleId: number;
		moduleName: string;
		order: number;
		notes?: string;
		resources?: { name: string; url: string; type: string }[];
	}

	// All lessons in this membership's learning center
	const allLessons: Lesson[] = [
		{
			id: 1,
			title: 'Welcome to Revolution Trading',
			description: 'Get an overview of what you\'ll learn and how to make the most of your membership.',
			duration: '10:00',
			durationSeconds: 600,
			videoUrl: 'https://sample-videos.com/video321/mp4/720/big_buck_bunny_720p_1mb.mp4',
			thumbnailUrl: '/images/lessons/welcome.jpg',
			completed: true,
			moduleId: 1,
			moduleName: 'Welcome & Getting Started',
			order: 1,
			notes: 'Welcome to Revolution Trading! In this lesson, we cover the fundamentals of what you\'ll learn throughout this course.',
			resources: [
				{ name: 'Welcome Guide PDF', url: '/resources/welcome-guide.pdf', type: 'pdf' },
				{ name: 'Platform Quick Reference', url: '/resources/platform-guide.pdf', type: 'pdf' }
			]
		},
		{
			id: 2,
			title: 'Platform Setup Guide',
			description: 'Learn how to set up your trading platform for optimal performance.',
			duration: '15:00',
			durationSeconds: 900,
			videoUrl: 'https://sample-videos.com/video321/mp4/720/big_buck_bunny_720p_1mb.mp4',
			thumbnailUrl: '/images/lessons/platform.jpg',
			completed: true,
			moduleId: 1,
			moduleName: 'Welcome & Getting Started',
			order: 2
		},
		{
			id: 3,
			title: 'Trading Room Navigation',
			description: 'Navigate the trading room like a pro with this comprehensive walkthrough.',
			duration: '12:00',
			durationSeconds: 720,
			videoUrl: 'https://sample-videos.com/video321/mp4/720/big_buck_bunny_720p_1mb.mp4',
			thumbnailUrl: '/images/lessons/navigation.jpg',
			completed: true,
			moduleId: 1,
			moduleName: 'Welcome & Getting Started',
			order: 3
		},
		{
			id: 4,
			title: 'Setting Up Your Workspace',
			description: 'Customize your workspace for maximum efficiency and comfort.',
			duration: '8:00',
			durationSeconds: 480,
			videoUrl: 'https://sample-videos.com/video321/mp4/720/big_buck_bunny_720p_1mb.mp4',
			thumbnailUrl: '/images/lessons/workspace.jpg',
			completed: true,
			moduleId: 1,
			moduleName: 'Welcome & Getting Started',
			order: 4
		},
		{
			id: 5,
			title: 'Understanding Market Structure',
			description: 'Master the fundamentals of market structure and price action.',
			duration: '25:00',
			durationSeconds: 1500,
			videoUrl: 'https://sample-videos.com/video321/mp4/720/big_buck_bunny_720p_1mb.mp4',
			thumbnailUrl: '/images/lessons/market-structure.jpg',
			completed: true,
			moduleId: 2,
			moduleName: 'Core Trading Strategies',
			order: 5,
			notes: 'Market structure is the foundation of technical analysis. Understanding how markets move will help you identify high-probability setups.',
			resources: [
				{ name: 'Market Structure Cheat Sheet', url: '/resources/market-structure.pdf', type: 'pdf' }
			]
		},
		{
			id: 6,
			title: 'Entry & Exit Techniques',
			description: 'Learn precise entry and exit techniques used by professional traders.',
			duration: '30:00',
			durationSeconds: 1800,
			videoUrl: 'https://sample-videos.com/video321/mp4/720/big_buck_bunny_720p_1mb.mp4',
			thumbnailUrl: '/images/lessons/entries.jpg',
			completed: true,
			moduleId: 2,
			moduleName: 'Core Trading Strategies',
			order: 6
		},
		{
			id: 7,
			title: 'Position Sizing Fundamentals',
			description: 'Protect your capital with proper position sizing strategies.',
			duration: '20:00',
			durationSeconds: 1200,
			videoUrl: 'https://sample-videos.com/video321/mp4/720/big_buck_bunny_720p_1mb.mp4',
			thumbnailUrl: '/images/lessons/position-sizing.jpg',
			completed: true,
			moduleId: 2,
			moduleName: 'Core Trading Strategies',
			order: 7
		},
		{
			id: 8,
			title: 'Advanced Entry Patterns',
			description: 'Discover advanced entry patterns for higher probability trades.',
			duration: '35:00',
			durationSeconds: 2100,
			videoUrl: 'https://sample-videos.com/video321/mp4/720/big_buck_bunny_720p_1mb.mp4',
			thumbnailUrl: '/images/lessons/patterns.jpg',
			completed: false,
			moduleId: 2,
			moduleName: 'Core Trading Strategies',
			order: 8
		},
		{
			id: 9,
			title: 'Multi-Timeframe Analysis',
			description: 'Use multiple timeframes to confirm trades and improve accuracy.',
			duration: '40:00',
			durationSeconds: 2400,
			videoUrl: 'https://sample-videos.com/video321/mp4/720/big_buck_bunny_720p_1mb.mp4',
			thumbnailUrl: '/images/lessons/timeframes.jpg',
			completed: false,
			moduleId: 2,
			moduleName: 'Core Trading Strategies',
			order: 9
		},
		{
			id: 10,
			title: 'Candlestick Patterns Mastery',
			description: 'Master the most important candlestick patterns for trading.',
			duration: '45:00',
			durationSeconds: 2700,
			videoUrl: 'https://sample-videos.com/video321/mp4/720/big_buck_bunny_720p_1mb.mp4',
			thumbnailUrl: '/images/lessons/candlesticks.jpg',
			completed: false,
			moduleId: 3,
			moduleName: 'Technical Analysis Deep Dive',
			order: 10
		}
	];

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED STATE
	// ═══════════════════════════════════════════════════════════════════════════

	const currentLesson = $derived(allLessons.find(l => l.id === lessonId) || allLessons[0]);
	const currentIndex = $derived(allLessons.findIndex(l => l.id === lessonId));
	const previousLesson = $derived(currentIndex > 0 ? allLessons[currentIndex - 1] : null);
	const nextLesson = $derived(currentIndex < allLessons.length - 1 ? allLessons[currentIndex + 1] : null);
	const moduleProgress = $derived.by(() => {
		const moduleLessons = allLessons.filter(l => l.moduleId === currentLesson.moduleId);
		const completed = moduleLessons.filter(l => l.completed).length;
		return Math.round((completed / moduleLessons.length) * 100);
	});
	const formattedCurrentTime = $derived(formatTime(currentTime));
	const formattedDuration = $derived(formatTime(duration));
	const progressPercent = $derived(duration > 0 ? (currentTime / duration) * 100 : 0);

	// ═══════════════════════════════════════════════════════════════════════════
	// VIDEO CONTROLS
	// ═══════════════════════════════════════════════════════════════════════════

	function formatTime(seconds: number): string {
		const mins = Math.floor(seconds / 60);
		const secs = Math.floor(seconds % 60);
		return `${mins}:${secs.toString().padStart(2, '0')}`;
	}

	function togglePlay() {
		if (!videoElement) return;
		if (isPlaying) {
			videoElement.pause();
		} else {
			videoElement.play();
		}
		isPlaying = !isPlaying;
	}

	function handleTimeUpdate() {
		if (videoElement) {
			currentTime = videoElement.currentTime;
		}
	}

	function handleLoadedMetadata() {
		if (videoElement) {
			duration = videoElement.duration;
		}
	}

	function handleSeek(event: MouseEvent) {
		const target = event.currentTarget as HTMLElement;
		const rect = target.getBoundingClientRect();
		const percent = (event.clientX - rect.left) / rect.width;
		if (videoElement) {
			videoElement.currentTime = percent * duration;
		}
	}

	function handleVolumeChange(event: Event) {
		const target = event.target as HTMLInputElement;
		volume = parseFloat(target.value);
		if (videoElement) {
			videoElement.volume = volume;
		}
		isMuted = volume === 0;
	}

	function toggleMute() {
		if (videoElement) {
			isMuted = !isMuted;
			videoElement.muted = isMuted;
		}
	}

	function changePlaybackRate(rate: number) {
		playbackRate = rate;
		if (videoElement) {
			videoElement.playbackRate = rate;
		}
	}

	function toggleFullscreen() {
		if (!browser) return;
		const container = document.querySelector('.video-container');
		if (!container) return;

		if (!document.fullscreenElement) {
			container.requestFullscreen();
			isFullscreen = true;
		} else {
			document.exitFullscreen();
			isFullscreen = false;
		}
	}

	function markAsComplete() {
		isCompleted = true;
		// In real app, this would call API to save progress
		console.log(`Lesson ${lessonId} marked as complete`);
	}

	function goToLesson(id: number) {
		goto(`/dashboard/${slug}/learning-center/${id}`);
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// KEYBOARD SHORTCUTS
	// ═══════════════════════════════════════════════════════════════════════════

	function handleKeydown(event: KeyboardEvent) {
		if (event.target instanceof HTMLInputElement || event.target instanceof HTMLTextAreaElement) return;

		switch (event.key) {
			case ' ':
			case 'k':
				event.preventDefault();
				togglePlay();
				break;
			case 'ArrowLeft':
				if (videoElement) videoElement.currentTime -= 10;
				break;
			case 'ArrowRight':
				if (videoElement) videoElement.currentTime += 10;
				break;
			case 'ArrowUp':
				volume = Math.min(1, volume + 0.1);
				if (videoElement) videoElement.volume = volume;
				break;
			case 'ArrowDown':
				volume = Math.max(0, volume - 0.1);
				if (videoElement) videoElement.volume = volume;
				break;
			case 'f':
				toggleFullscreen();
				break;
			case 'm':
				toggleMute();
				break;
		}
	}
</script>

<svelte:head>
	<title>{currentLesson.title} | Learning Center | Revolution Trading Pros</title>
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<svelte:window onkeydown={handleKeydown} />

<!-- ═══════════════════════════════════════════════════════════════════════════
     LESSON HEADER
     ═══════════════════════════════════════════════════════════════════════════ -->

<header class="lesson-header">
	<div class="lesson-header__left">
		<nav class="breadcrumb" aria-label="Breadcrumb">
			<a href="/dashboard">Dashboard</a>
			<span class="separator">/</span>
			<a href="/dashboard/{slug}">{slug.split('-').map(w => w.charAt(0).toUpperCase() + w.slice(1)).join(' ')}</a>
			<span class="separator">/</span>
			<a href="/dashboard/{slug}/learning-center">Learning Center</a>
			<span class="separator">/</span>
			<span class="current">{currentLesson.moduleName}</span>
		</nav>
		<h1 class="lesson-title">{currentLesson.title}</h1>
	</div>
	<div class="lesson-header__right">
		<button class="btn-icon" title="Bookmark">
			<IconBookmark size={20} />
		</button>
		<button class="btn-icon" title="Share">
			<IconShare size={20} />
		</button>
		{#if !isCompleted && !currentLesson.completed}
			<button class="btn-complete" onclick={markAsComplete}>
				<IconCheck size={18} />
				Mark Complete
			</button>
		{:else}
			<span class="completed-badge">
				<IconCheck size={18} />
				Completed
			</span>
		{/if}
	</div>
</header>

<!-- ═══════════════════════════════════════════════════════════════════════════
     MAIN CONTENT
     ═══════════════════════════════════════════════════════════════════════════ -->

<div class="lesson-layout" class:playlist-open={showPlaylist}>
	<!-- Video Player -->
	<main class="lesson-main">
		<div class="video-container">
			<video
				bind:this={videoElement}
				src={currentLesson.videoUrl}
				poster={currentLesson.thumbnailUrl}
				ontimeupdate={handleTimeUpdate}
				onloadedmetadata={handleLoadedMetadata}
				onended={() => { isPlaying = false; markAsComplete(); }}
				onplay={() => isPlaying = true}
				onpause={() => isPlaying = false}
			>
				<track kind="captions" src="" label="English" />
			</video>

			<!-- Video Overlay (click to play/pause) -->
			<button class="video-overlay" onclick={togglePlay}>
				{#if !isPlaying}
					<div class="play-button">
						<IconPlayerPlay size={48} />
					</div>
				{/if}
			</button>

			<!-- Video Controls -->
			<div class="video-controls">
				<!-- Progress Bar -->
				<button class="progress-bar" onclick={handleSeek}>
					<div class="progress-bar__buffered" style="width: 100%"></div>
					<div class="progress-bar__played" style="width: {progressPercent}%"></div>
					<div class="progress-bar__thumb" style="left: {progressPercent}%"></div>
				</button>

				<!-- Control Buttons -->
				<div class="controls-row">
					<div class="controls-left">
						<button class="control-btn" onclick={togglePlay} title={isPlaying ? 'Pause' : 'Play'}>
							{#if isPlaying}
								<IconPlayerPause size={24} />
							{:else}
								<IconPlayerPlay size={24} />
							{/if}
						</button>

						<!-- Volume -->
						<div class="volume-control">
							<button class="control-btn" onclick={toggleMute} title={isMuted ? 'Unmute' : 'Mute'}>
								<IconVolume size={20} />
							</button>
							<input
								type="range"
								class="volume-slider"
								min="0"
								max="1"
								step="0.1"
								value={volume}
								oninput={handleVolumeChange}
							/>
						</div>

						<!-- Time Display -->
						<span class="time-display">{formattedCurrentTime} / {formattedDuration}</span>
					</div>

					<div class="controls-right">
						<!-- Playback Speed -->
						<div class="speed-control">
							<button class="control-btn" title="Playback Speed">
								<IconSettings size={20} />
								<span>{playbackRate}x</span>
							</button>
							<div class="speed-menu">
								{#each [0.5, 0.75, 1, 1.25, 1.5, 2] as rate}
									<button
										class:active={playbackRate === rate}
										onclick={() => changePlaybackRate(rate)}
									>
										{rate}x
									</button>
								{/each}
							</div>
						</div>

						<!-- Playlist Toggle -->
						<button
							class="control-btn"
							onclick={() => showPlaylist = !showPlaylist}
							title="Toggle Playlist"
						>
							<IconList size={20} />
						</button>

						<!-- Fullscreen -->
						<button class="control-btn" onclick={toggleFullscreen} title="Fullscreen">
							<IconMaximize size={20} />
						</button>
					</div>
				</div>
			</div>
		</div>

		<!-- Lesson Info -->
		<div class="lesson-info">
			<div class="lesson-meta">
				<span class="meta-item">
					<IconClock size={16} />
					{currentLesson.duration}
				</span>
				<span class="meta-item module">
					{currentLesson.moduleName}
				</span>
			</div>

			<p class="lesson-description">{currentLesson.description}</p>

			<!-- Tabs -->
			<div class="lesson-tabs">
				<button
					class="tab-btn"
					class:active={!showNotes}
					onclick={() => showNotes = false}
				>
					About
				</button>
				<button
					class="tab-btn"
					class:active={showNotes}
					onclick={() => showNotes = true}
				>
					<IconNotes size={16} />
					Notes & Resources
				</button>
			</div>

			{#if showNotes}
				<div class="lesson-notes">
					{#if currentLesson.notes}
						<div class="notes-content">
							<h3>Lesson Notes</h3>
							<p>{currentLesson.notes}</p>
						</div>
					{/if}

					{#if currentLesson.resources && currentLesson.resources.length > 0}
						<div class="resources-list">
							<h3>Resources</h3>
							{#each currentLesson.resources as resource}
								<a href={resource.url} class="resource-item" download>
									<IconDownload size={18} />
									<span>{resource.name}</span>
								</a>
							{/each}
						</div>
					{/if}

					{#if !currentLesson.notes && (!currentLesson.resources || currentLesson.resources.length === 0)}
						<p class="no-content">No notes or resources available for this lesson.</p>
					{/if}
				</div>
			{:else}
				<div class="lesson-about">
					<h3>What you'll learn</h3>
					<p>{currentLesson.description}</p>
				</div>
			{/if}

			<!-- Navigation -->
			<div class="lesson-navigation">
				{#if previousLesson}
					<button class="nav-btn prev" onclick={() => goToLesson(previousLesson.id)}>
						<IconChevronLeft size={20} />
						<div class="nav-info">
							<span class="nav-label">Previous</span>
							<span class="nav-title">{previousLesson.title}</span>
						</div>
					</button>
				{:else}
					<div></div>
				{/if}

				{#if nextLesson}
					<button class="nav-btn next" onclick={() => goToLesson(nextLesson.id)}>
						<div class="nav-info">
							<span class="nav-label">Next</span>
							<span class="nav-title">{nextLesson.title}</span>
						</div>
						<IconChevronRight size={20} />
					</button>
				{:else}
					<div></div>
				{/if}
			</div>
		</div>
	</main>

	<!-- Playlist Sidebar -->
	{#if showPlaylist}
		<aside class="playlist-sidebar">
			<div class="playlist-header">
				<h2>Course Content</h2>
				<span class="playlist-progress">{moduleProgress}% complete</span>
			</div>

			<div class="playlist-content">
				{#each allLessons as lesson (lesson.id)}
					<button
						class="playlist-item"
						class:active={lesson.id === lessonId}
						class:completed={lesson.completed}
						onclick={() => goToLesson(lesson.id)}
					>
						<div class="playlist-status">
							{#if lesson.completed}
								<IconCheck size={16} />
							{:else if lesson.id === lessonId}
								<IconPlayerPlay size={16} />
							{:else}
								<span class="lesson-number">{lesson.order}</span>
							{/if}
						</div>
						<div class="playlist-info">
							<span class="playlist-title">{lesson.title}</span>
							<span class="playlist-duration">{lesson.duration}</span>
						</div>
					</button>
				{/each}
			</div>
		</aside>
	{/if}
</div>

<!-- ═══════════════════════════════════════════════════════════════════════════
     STYLES
     ═══════════════════════════════════════════════════════════════════════════ -->

<style>
	/* Header */
	.lesson-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		padding: 20px 24px;
		background: #fff;
		border-bottom: 1px solid #e5e7eb;
	}

	.breadcrumb {
		font-size: 13px;
		color: #64748b;
		margin-bottom: 8px;
	}

	.breadcrumb a {
		color: #0984ae;
		text-decoration: none;
	}

	.breadcrumb a:hover {
		text-decoration: underline;
	}

	.breadcrumb .separator {
		margin: 0 8px;
		color: #cbd5e1;
	}

	.lesson-title {
		font-size: 24px;
		font-weight: 700;
		color: #1e293b;
		margin: 0;
	}

	.lesson-header__right {
		display: flex;
		align-items: center;
		gap: 12px;
	}

	.btn-icon {
		width: 40px;
		height: 40px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: #f1f5f9;
		border: none;
		border-radius: 8px;
		color: #64748b;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.btn-icon:hover {
		background: #e2e8f0;
		color: #334155;
	}

	.btn-complete {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 10px 20px;
		background: #0984ae;
		color: #fff;
		border: none;
		border-radius: 8px;
		font-size: 14px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.btn-complete:hover {
		background: #0369a1;
	}

	.completed-badge {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 10px 20px;
		background: #10b981;
		color: #fff;
		border-radius: 8px;
		font-size: 14px;
		font-weight: 600;
	}

	/* Layout */
	.lesson-layout {
		display: flex;
		min-height: calc(100vh - 140px);
		background: #f8fafc;
	}

	.lesson-main {
		flex: 1;
		max-width: 100%;
	}

	.lesson-layout.playlist-open .lesson-main {
		max-width: calc(100% - 360px);
	}

	/* Video Container */
	.video-container {
		position: relative;
		background: #000;
		aspect-ratio: 16 / 9;
	}

	.video-container video {
		width: 100%;
		height: 100%;
		object-fit: contain;
	}

	.video-overlay {
		position: absolute;
		inset: 0;
		background: transparent;
		border: none;
		cursor: pointer;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.play-button {
		width: 80px;
		height: 80px;
		background: rgba(0, 0, 0, 0.7);
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		color: #fff;
		transition: transform 0.15s ease;
	}

	.video-overlay:hover .play-button {
		transform: scale(1.1);
	}

	/* Video Controls */
	.video-controls {
		position: absolute;
		bottom: 0;
		left: 0;
		right: 0;
		background: linear-gradient(transparent, rgba(0, 0, 0, 0.8));
		padding: 40px 16px 16px;
		opacity: 0;
		transition: opacity 0.2s ease;
	}

	.video-container:hover .video-controls {
		opacity: 1;
	}

	/* Progress Bar */
	.progress-bar {
		width: 100%;
		height: 4px;
		background: rgba(255, 255, 255, 0.3);
		border-radius: 2px;
		margin-bottom: 12px;
		position: relative;
		cursor: pointer;
		border: none;
		padding: 0;
	}

	.progress-bar:hover {
		height: 6px;
	}

	.progress-bar__buffered {
		position: absolute;
		height: 100%;
		background: rgba(255, 255, 255, 0.4);
		border-radius: 2px;
	}

	.progress-bar__played {
		position: absolute;
		height: 100%;
		background: #0984ae;
		border-radius: 2px;
	}

	.progress-bar__thumb {
		position: absolute;
		top: 50%;
		width: 12px;
		height: 12px;
		background: #fff;
		border-radius: 50%;
		transform: translate(-50%, -50%);
		opacity: 0;
		transition: opacity 0.15s ease;
	}

	.progress-bar:hover .progress-bar__thumb {
		opacity: 1;
	}

	/* Controls Row */
	.controls-row {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.controls-left,
	.controls-right {
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.control-btn {
		display: flex;
		align-items: center;
		gap: 4px;
		background: transparent;
		border: none;
		color: #fff;
		padding: 8px;
		border-radius: 4px;
		cursor: pointer;
		transition: background 0.15s ease;
	}

	.control-btn:hover {
		background: rgba(255, 255, 255, 0.1);
	}

	.time-display {
		color: #fff;
		font-size: 13px;
		font-family: monospace;
		margin-left: 8px;
	}

	/* Volume Control */
	.volume-control {
		display: flex;
		align-items: center;
	}

	.volume-slider {
		width: 0;
		opacity: 0;
		transition: all 0.2s ease;
	}

	.volume-control:hover .volume-slider {
		width: 80px;
		opacity: 1;
		margin-left: 8px;
	}

	/* Speed Control */
	.speed-control {
		position: relative;
	}

	.speed-menu {
		display: none;
		position: absolute;
		bottom: 100%;
		right: 0;
		background: rgba(0, 0, 0, 0.9);
		border-radius: 8px;
		padding: 8px;
		margin-bottom: 8px;
	}

	.speed-control:hover .speed-menu {
		display: block;
	}

	.speed-menu button {
		display: block;
		width: 100%;
		padding: 8px 16px;
		background: transparent;
		border: none;
		color: #fff;
		font-size: 13px;
		cursor: pointer;
		text-align: left;
		border-radius: 4px;
	}

	.speed-menu button:hover,
	.speed-menu button.active {
		background: rgba(255, 255, 255, 0.1);
	}

	/* Lesson Info */
	.lesson-info {
		padding: 24px;
		background: #fff;
	}

	.lesson-meta {
		display: flex;
		gap: 16px;
		margin-bottom: 16px;
	}

	.meta-item {
		display: flex;
		align-items: center;
		gap: 6px;
		font-size: 14px;
		color: #64748b;
	}

	.meta-item.module {
		background: #f1f5f9;
		padding: 4px 12px;
		border-radius: 16px;
	}

	.lesson-description {
		font-size: 15px;
		color: #475569;
		line-height: 1.6;
		margin-bottom: 24px;
	}

	/* Tabs */
	.lesson-tabs {
		display: flex;
		gap: 8px;
		border-bottom: 1px solid #e5e7eb;
		margin-bottom: 20px;
	}

	.tab-btn {
		display: flex;
		align-items: center;
		gap: 6px;
		padding: 12px 16px;
		background: none;
		border: none;
		border-bottom: 2px solid transparent;
		color: #64748b;
		font-size: 14px;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.tab-btn:hover {
		color: #0984ae;
	}

	.tab-btn.active {
		color: #0984ae;
		border-bottom-color: #0984ae;
	}

	/* Notes & Resources */
	.lesson-notes,
	.lesson-about {
		min-height: 150px;
	}

	.notes-content,
	.resources-list {
		margin-bottom: 24px;
	}

	.notes-content h3,
	.resources-list h3,
	.lesson-about h3 {
		font-size: 16px;
		font-weight: 600;
		color: #1e293b;
		margin: 0 0 12px;
	}

	.notes-content p,
	.lesson-about p {
		font-size: 14px;
		color: #475569;
		line-height: 1.6;
	}

	.resource-item {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 12px 16px;
		background: #f8fafc;
		border-radius: 8px;
		text-decoration: none;
		color: #334155;
		font-size: 14px;
		margin-bottom: 8px;
		transition: background 0.15s ease;
	}

	.resource-item:hover {
		background: #e2e8f0;
	}

	.no-content {
		color: #94a3b8;
		font-size: 14px;
		text-align: center;
		padding: 40px;
	}

	/* Navigation */
	.lesson-navigation {
		display: flex;
		justify-content: space-between;
		margin-top: 32px;
		padding-top: 24px;
		border-top: 1px solid #e5e7eb;
	}

	.nav-btn {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 16px;
		background: #f8fafc;
		border: 1px solid #e5e7eb;
		border-radius: 12px;
		cursor: pointer;
		transition: all 0.15s ease;
		max-width: 45%;
	}

	.nav-btn:hover {
		background: #f1f5f9;
		border-color: #0984ae;
	}

	.nav-btn.next {
		text-align: right;
	}

	.nav-info {
		display: flex;
		flex-direction: column;
		gap: 4px;
	}

	.nav-label {
		font-size: 12px;
		color: #64748b;
		text-transform: uppercase;
	}

	.nav-title {
		font-size: 14px;
		font-weight: 600;
		color: #1e293b;
	}

	/* Playlist Sidebar */
	.playlist-sidebar {
		width: 360px;
		background: #fff;
		border-left: 1px solid #e5e7eb;
		display: flex;
		flex-direction: column;
	}

	.playlist-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 20px;
		border-bottom: 1px solid #e5e7eb;
	}

	.playlist-header h2 {
		font-size: 16px;
		font-weight: 600;
		color: #1e293b;
		margin: 0;
	}

	.playlist-progress {
		font-size: 13px;
		color: #0984ae;
		font-weight: 500;
	}

	.playlist-content {
		flex: 1;
		overflow-y: auto;
	}

	.playlist-item {
		display: flex;
		align-items: center;
		gap: 12px;
		width: 100%;
		padding: 16px 20px;
		background: transparent;
		border: none;
		border-bottom: 1px solid #f1f5f9;
		cursor: pointer;
		transition: background 0.15s ease;
		text-align: left;
	}

	.playlist-item:hover {
		background: #f8fafc;
	}

	.playlist-item.active {
		background: #eff6ff;
		border-left: 3px solid #0984ae;
	}

	.playlist-item.completed {
		opacity: 0.7;
	}

	.playlist-status {
		width: 28px;
		height: 28px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: #f1f5f9;
		border-radius: 50%;
		color: #64748b;
		font-size: 12px;
		font-weight: 600;
		flex-shrink: 0;
	}

	.playlist-item.completed .playlist-status {
		background: #10b981;
		color: #fff;
	}

	.playlist-item.active .playlist-status {
		background: #0984ae;
		color: #fff;
	}

	.playlist-info {
		flex: 1;
		min-width: 0;
	}

	.playlist-title {
		display: block;
		font-size: 14px;
		font-weight: 500;
		color: #334155;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.playlist-duration {
		font-size: 12px;
		color: #94a3b8;
	}

	/* Responsive */
	@media (max-width: 1024px) {
		.lesson-layout.playlist-open .lesson-main {
			max-width: 100%;
		}

		.playlist-sidebar {
			display: none;
		}
	}

	@media (max-width: 768px) {
		.lesson-header {
			flex-direction: column;
			gap: 16px;
		}

		.lesson-header__right {
			width: 100%;
			justify-content: flex-end;
		}

		.lesson-navigation {
			flex-direction: column;
			gap: 12px;
		}

		.nav-btn {
			max-width: 100%;
		}
	}
</style>

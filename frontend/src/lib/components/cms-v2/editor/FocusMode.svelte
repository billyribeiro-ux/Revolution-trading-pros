<!--
/**
 * Focus Mode Component - CMS Editor Enhancement
 * ═══════════════════════════════════════════════════════════════════════════
 * Distraction-free writing environment with writing goals and stats tracking
 *
 * FEATURES:
 * - Toggle focus mode with button and keyboard shortcut (Ctrl/Cmd + Shift + F)
 * - Dims sidebar and toolbar (opacity 0.1, full on hover)
 * - Dims non-selected blocks (opacity 0.3)
 * - Fullscreen mode using Fullscreen API
 * - Stats display (word count, reading time, character count)
 * - Writing goals with progress tracking and celebration animation
 * - LocalStorage persistence for goals
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 */
-->

<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { fade, fly, scale } from 'svelte/transition';
	import { quintOut, elasticOut } from 'svelte/easing';
	import { browser } from '$app/environment';

	// ==========================================================================
	// Props
	// ==========================================================================

	interface Props {
		/** Whether focus mode is currently active */
		isActive?: boolean;
		/** Callback when focus mode is toggled */
		onToggle?: () => void;
		/** Content string for word count calculations */
		content?: string;
		/** Current word goal (null if no goal set) */
		wordGoal?: number | null;
		/** Callback when word goal changes */
		onGoalChange?: (goal: number | null) => void;
		/** Content item ID for localStorage persistence */
		contentId?: string;
	}

	let {
		isActive = $bindable(false),
		onToggle,
		content = '',
		wordGoal = $bindable(null),
		onGoalChange,
		contentId = 'default'
	}: Props = $props();

	// ==========================================================================
	// State
	// ==========================================================================

	let isFullscreen = $state(false);
	let showGoalMenu = $state(false);
	let customGoalInput = $state('');
	let showCelebration = $state(false);
	let goalReachedOnce = $state(false);
	let containerRef = $state<HTMLElement | null>(null);

	// Preset goal options
	const GOAL_PRESETS = [500, 1000, 2000, 5000] as const;

	// ==========================================================================
	// Computed Values
	// ==========================================================================

	/** Calculate word count from content */
	let wordCount = $derived.by(() => {
		if (!content) return 0;
		// Strip HTML tags and count words
		const plainText = content.replace(/<[^>]*>/g, '').trim();
		if (!plainText) return 0;
		return plainText.split(/\s+/).filter((word) => word.length > 0).length;
	});

	/** Calculate character count (excluding HTML tags) */
	let characterCount = $derived.by(() => {
		if (!content) return 0;
		const plainText = content.replace(/<[^>]*>/g, '');
		return plainText.length;
	});

	/** Calculate reading time in minutes (200 words per minute) */
	let readingTime = $derived(Math.ceil(wordCount / 200) || 1);

	/** Calculate goal progress percentage */
	let goalProgress = $derived.by(() => {
		if (!wordGoal || wordGoal <= 0) return 0;
		return Math.min((wordCount / wordGoal) * 100, 100);
	});

	/** Check if goal is reached */
	let goalReached = $derived(wordGoal !== null && wordGoal > 0 && wordCount >= wordGoal);

	/** Words remaining to reach goal */
	let wordsRemaining = $derived.by(() => {
		if (!wordGoal || wordGoal <= 0) return 0;
		return Math.max(wordGoal - wordCount, 0);
	});

	// ==========================================================================
	// Effects
	// ==========================================================================

	// Load goal from localStorage on mount
	$effect(() => {
		if (browser && contentId) {
			const storageKey = `cms-focus-goal-${contentId}`;
			const savedGoal = localStorage.getItem(storageKey);
			if (savedGoal !== null) {
				const parsed = parseInt(savedGoal, 10);
				if (!isNaN(parsed) && parsed > 0) {
					wordGoal = parsed;
					onGoalChange?.(parsed);
				}
			}
		}
	});

	// Save goal to localStorage when it changes
	$effect(() => {
		if (browser && contentId) {
			const storageKey = `cms-focus-goal-${contentId}`;
			if (wordGoal !== null && wordGoal > 0) {
				localStorage.setItem(storageKey, wordGoal.toString());
			} else {
				localStorage.removeItem(storageKey);
			}
		}
	});

	// Trigger celebration when goal is reached
	$effect(() => {
		if (goalReached && !goalReachedOnce && isActive) {
			goalReachedOnce = true;
			showCelebration = true;
			setTimeout(() => {
				showCelebration = false;
			}, 3000);
		}
	});

	// Reset goalReachedOnce when goal changes
	$effect(() => {
		if (wordGoal) {
			goalReachedOnce = wordCount >= wordGoal;
		}
	});

	// Listen for fullscreen changes
	$effect(() => {
		if (browser) {
			const handleFullscreenChange = () => {
				isFullscreen = !!document.fullscreenElement;
			};

			document.addEventListener('fullscreenchange', handleFullscreenChange);
			return () => {
				document.removeEventListener('fullscreenchange', handleFullscreenChange);
			};
		}
	});

	// ==========================================================================
	// Keyboard Shortcuts
	// ==========================================================================

	function handleKeydown(e: KeyboardEvent) {
		const isMeta = e.metaKey || e.ctrlKey;

		// Toggle focus mode: Ctrl/Cmd + Shift + F
		if (isMeta && e.shiftKey && e.key.toLowerCase() === 'f') {
			e.preventDefault();
			toggleFocusMode();
		}

		// Toggle fullscreen: F11 or Ctrl/Cmd + Shift + Enter
		if (e.key === 'F11' || (isMeta && e.shiftKey && e.key === 'Enter')) {
			e.preventDefault();
			toggleFullscreen();
		}

		// Close goal menu on Escape
		if (e.key === 'Escape' && showGoalMenu) {
			e.preventDefault();
			showGoalMenu = false;
		}
	}

	onMount(() => {
		if (browser) {
			window.addEventListener('keydown', handleKeydown);
		}
	});

	onDestroy(() => {
		if (browser) {
			window.removeEventListener('keydown', handleKeydown);
		}
	});

	// ==========================================================================
	// Actions
	// ==========================================================================

	function toggleFocusMode() {
		isActive = !isActive;
		onToggle?.();
	}

	async function toggleFullscreen() {
		if (!browser) return;

		try {
			if (!document.fullscreenElement) {
				await document.documentElement.requestFullscreen();
				isFullscreen = true;
			} else {
				await document.exitFullscreen();
				isFullscreen = false;
			}
		} catch (err) {
			console.warn('Fullscreen not supported:', err);
		}
	}

	function setGoal(goal: number | null) {
		wordGoal = goal;
		onGoalChange?.(goal);
		showGoalMenu = false;
		goalReachedOnce = goal !== null && wordCount >= goal;
	}

	function handleCustomGoal() {
		const parsed = parseInt(customGoalInput, 10);
		if (!isNaN(parsed) && parsed > 0) {
			setGoal(parsed);
			customGoalInput = '';
		}
	}

	function clearGoal() {
		setGoal(null);
		goalReachedOnce = false;
	}

	function handleGoalMenuKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter') {
			handleCustomGoal();
		}
	}

	// Format number with commas
	function formatNumber(num: number): string {
		return num.toLocaleString();
	}
</script>

<!-- Focus Mode Overlay Effects (CSS classes injected into parent) -->
{#if isActive}
	<style>
		/* Dim sidebar when focus mode is active */
		.editor-sidebar,
		[data-focus-dim='sidebar'] {
			opacity: 0.1 !important;
			transition: opacity 0.3s ease !important;
		}

		.editor-sidebar:hover,
		[data-focus-dim='sidebar']:hover {
			opacity: 1 !important;
		}

		/* Dim toolbar when focus mode is active */
		.editor-header,
		[data-focus-dim='toolbar'] {
			opacity: 0.1 !important;
			transition: opacity 0.3s ease !important;
		}

		.editor-header:hover,
		[data-focus-dim='toolbar']:hover {
			opacity: 1 !important;
		}

		/* Dim non-selected blocks */
		.block-wrapper:not(.selected),
		[data-focus-dim='block']:not(.selected) {
			opacity: 0.3 !important;
			transition: opacity 0.3s ease !important;
		}

		.block-wrapper:not(.selected):hover,
		[data-focus-dim='block']:not(.selected):hover {
			opacity: 0.6 !important;
		}

		/* Focus mode background */
		.editor-canvas,
		[data-focus-canvas] {
			background: linear-gradient(
				180deg,
				rgba(15, 23, 42, 0.02) 0%,
				rgba(15, 23, 42, 0.05) 100%
			) !important;
		}
	</style>
{/if}

<div bind:this={containerRef} class="focus-mode-container" class:active={isActive}>
	<!-- Focus Mode Toggle Button -->
	<button
		type="button"
		class="focus-toggle-btn"
		class:active={isActive}
		onclick={toggleFocusMode}
		aria-label={isActive ? 'Exit focus mode' : 'Enter focus mode'}
		aria-pressed={isActive}
		title="Toggle Focus Mode (Ctrl+Shift+F)"
	>
		{#if isActive}
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="18"
				height="18"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
				stroke-linecap="round"
				stroke-linejoin="round"
			>
				<path d="M8 3H5a2 2 0 0 0-2 2v3" />
				<path d="M21 8V5a2 2 0 0 0-2-2h-3" />
				<path d="M3 16v3a2 2 0 0 0 2 2h3" />
				<path d="M16 21h3a2 2 0 0 0 2-2v-3" />
				<circle cx="12" cy="12" r="3" />
			</svg>
		{:else}
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="18"
				height="18"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
				stroke-linecap="round"
				stroke-linejoin="round"
			>
				<circle cx="12" cy="12" r="3" />
				<path d="M3 12h3" />
				<path d="M18 12h3" />
				<path d="M12 3v3" />
				<path d="M12 18v3" />
			</svg>
		{/if}
		<span class="btn-label">Focus</span>
	</button>

	<!-- Stats Panel (visible when focus mode is active) -->
	{#if isActive}
		<div
			class="stats-panel"
			transition:fly={{ y: -10, duration: 200, easing: quintOut }}
			role="status"
			aria-live="polite"
			aria-label="Writing statistics"
		>
			<div class="stats-row">
				<div class="stat-item">
					<span class="stat-value">{formatNumber(wordCount)}</span>
					<span class="stat-label">words</span>
				</div>
				<div class="stat-divider" aria-hidden="true"></div>
				<div class="stat-item">
					<span class="stat-value">{readingTime}</span>
					<span class="stat-label">min read</span>
				</div>
				<div class="stat-divider" aria-hidden="true"></div>
				<div class="stat-item">
					<span class="stat-value">{formatNumber(characterCount)}</span>
					<span class="stat-label">chars</span>
				</div>
			</div>

			<!-- Writing Goal Section -->
			<div class="goal-section">
				{#if wordGoal !== null && wordGoal > 0}
					<div class="goal-progress-container">
						<div class="goal-header">
							<span class="goal-text">
								{#if goalReached}
									Goal reached!
								{:else}
									{formatNumber(wordsRemaining)} words to go
								{/if}
							</span>
							<button
								type="button"
								class="goal-edit-btn"
								onclick={() => (showGoalMenu = !showGoalMenu)}
								aria-label="Edit goal"
								aria-expanded={showGoalMenu}
							>
								<svg
									xmlns="http://www.w3.org/2000/svg"
									width="14"
									height="14"
									viewBox="0 0 24 24"
									fill="none"
									stroke="currentColor"
									stroke-width="2"
									stroke-linecap="round"
									stroke-linejoin="round"
								>
									<path d="M12 20h9" />
									<path d="M16.5 3.5a2.121 2.121 0 0 1 3 3L7 19l-4 1 1-4L16.5 3.5z" />
								</svg>
							</button>
						</div>
						<div class="progress-bar" role="progressbar" aria-valuenow={Math.round(goalProgress)} aria-valuemin={0} aria-valuemax={100}>
							<div
								class="progress-fill"
								class:goal-reached={goalReached}
								style:width="{goalProgress}%"
							></div>
						</div>
						<div class="goal-stats">
							<span>{formatNumber(wordCount)} / {formatNumber(wordGoal)}</span>
							<span>{Math.round(goalProgress)}%</span>
						</div>
					</div>
				{:else}
					<button
						type="button"
						class="set-goal-btn"
						onclick={() => (showGoalMenu = !showGoalMenu)}
						aria-label="Set writing goal"
						aria-expanded={showGoalMenu}
					>
						<svg
							xmlns="http://www.w3.org/2000/svg"
							width="16"
							height="16"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
							stroke-linecap="round"
							stroke-linejoin="round"
						>
							<circle cx="12" cy="12" r="10" />
							<circle cx="12" cy="12" r="6" />
							<circle cx="12" cy="12" r="2" />
						</svg>
						Set daily goal
					</button>
				{/if}

				<!-- Goal Menu Dropdown -->
				{#if showGoalMenu}
					<div
						class="goal-menu"
						transition:scale={{ duration: 150, start: 0.95, easing: quintOut }}
						role="menu"
						aria-label="Writing goal options"
					>
						<div class="goal-menu-header">Set Word Goal</div>
						<div class="goal-presets" role="group" aria-label="Preset goals">
							{#each GOAL_PRESETS as preset}
								<button
									type="button"
									class="goal-preset-btn"
									class:active={wordGoal === preset}
									onclick={() => setGoal(preset)}
									role="menuitem"
								>
									{formatNumber(preset)}
								</button>
							{/each}
						</div>
						<div class="goal-custom">
							<input
								type="number"
								bind:value={customGoalInput}
								onkeydown={handleGoalMenuKeydown}
								placeholder="Custom goal..."
								min="1"
								class="custom-goal-input"
								aria-label="Custom word goal"
							/>
							<button
								type="button"
								class="custom-goal-submit"
								onclick={handleCustomGoal}
								disabled={!customGoalInput || parseInt(customGoalInput) <= 0}
								aria-label="Set custom goal"
							>
								<svg
									xmlns="http://www.w3.org/2000/svg"
									width="16"
									height="16"
									viewBox="0 0 24 24"
									fill="none"
									stroke="currentColor"
									stroke-width="2"
									stroke-linecap="round"
									stroke-linejoin="round"
								>
									<polyline points="20 6 9 17 4 12" />
								</svg>
							</button>
						</div>
						{#if wordGoal !== null}
							<button
								type="button"
								class="clear-goal-btn"
								onclick={clearGoal}
								role="menuitem"
							>
								<svg
									xmlns="http://www.w3.org/2000/svg"
									width="14"
									height="14"
									viewBox="0 0 24 24"
									fill="none"
									stroke="currentColor"
									stroke-width="2"
									stroke-linecap="round"
									stroke-linejoin="round"
								>
									<line x1="18" y1="6" x2="6" y2="18" />
									<line x1="6" y1="6" x2="18" y2="18" />
								</svg>
								Clear goal
							</button>
						{/if}
					</div>
				{/if}
			</div>

			<!-- Fullscreen Toggle -->
			<button
				type="button"
				class="fullscreen-btn"
				onclick={toggleFullscreen}
				aria-label={isFullscreen ? 'Exit fullscreen' : 'Enter fullscreen'}
				title={isFullscreen ? 'Exit Fullscreen (F11)' : 'Enter Fullscreen (F11)'}
			>
				{#if isFullscreen}
					<svg
						xmlns="http://www.w3.org/2000/svg"
						width="16"
						height="16"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
						stroke-linecap="round"
						stroke-linejoin="round"
					>
						<path d="M8 3v3a2 2 0 0 1-2 2H3" />
						<path d="M21 8h-3a2 2 0 0 1-2-2V3" />
						<path d="M3 16h3a2 2 0 0 1 2 2v3" />
						<path d="M16 21v-3a2 2 0 0 1 2-2h3" />
					</svg>
				{:else}
					<svg
						xmlns="http://www.w3.org/2000/svg"
						width="16"
						height="16"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
						stroke-linecap="round"
						stroke-linejoin="round"
					>
						<path d="M8 3H5a2 2 0 0 0-2 2v3" />
						<path d="M21 8V5a2 2 0 0 0-2-2h-3" />
						<path d="M3 16v3a2 2 0 0 0 2 2h3" />
						<path d="M16 21h3a2 2 0 0 0 2-2v-3" />
					</svg>
				{/if}
			</button>
		</div>
	{/if}

	<!-- Goal Reached Celebration Animation -->
	{#if showCelebration}
		<div
			class="celebration-overlay"
			transition:fade={{ duration: 300 }}
			role="alert"
			aria-live="assertive"
		>
			<div
				class="celebration-content"
				transition:scale={{ duration: 500, start: 0.5, easing: elasticOut }}
			>
				<div class="celebration-icon">
					<svg
						xmlns="http://www.w3.org/2000/svg"
						width="48"
						height="48"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
						stroke-linecap="round"
						stroke-linejoin="round"
					>
						<path d="M6 9H4.5a2.5 2.5 0 0 1 0-5H6" />
						<path d="M18 9h1.5a2.5 2.5 0 0 0 0-5H18" />
						<path d="M4 22h16" />
						<path d="M10 14.66V17c0 .55-.47.98-.97 1.21C7.85 18.75 7 20.24 7 22" />
						<path d="M14 14.66V17c0 .55.47.98.97 1.21C16.15 18.75 17 20.24 17 22" />
						<path d="M18 2H6v7a6 6 0 0 0 12 0V2Z" />
					</svg>
				</div>
				<h3 class="celebration-title">Goal Reached!</h3>
				<p class="celebration-text">
					You've written {formatNumber(wordCount)} words. Great work!
				</p>
				<div class="confetti" aria-hidden="true">
					{#each Array(20) as _, i}
						<span
							class="confetti-piece"
							style:--delay="{Math.random() * 0.5}s"
							style:--x="{Math.random() * 200 - 100}px"
							style:--rotation="{Math.random() * 360}deg"
							style:--color="hsl({Math.random() * 360}, 80%, 60%)"
						></span>
					{/each}
				</div>
			</div>
		</div>
	{/if}
</div>

<style>
	.focus-mode-container {
		position: relative;
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	/* Focus Toggle Button */
	.focus-toggle-btn {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.5rem 0.75rem;
		background: transparent;
		border: 1px solid #e5e7eb;
		border-radius: 8px;
		color: #6b7280;
		font-size: 0.8125rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.focus-toggle-btn:hover {
		background: #f3f4f6;
		border-color: #d1d5db;
		color: #374151;
	}

	.focus-toggle-btn.active {
		background: linear-gradient(135deg, #6366f1 0%, #8b5cf6 100%);
		border-color: transparent;
		color: white;
		box-shadow:
			0 4px 12px rgba(99, 102, 241, 0.3),
			0 0 0 3px rgba(99, 102, 241, 0.1);
	}

	.focus-toggle-btn:focus-visible {
		outline: 2px solid #6366f1;
		outline-offset: 2px;
	}

	.btn-label {
		display: none;
	}

	@media (min-width: 640px) {
		.btn-label {
			display: inline;
		}
	}

	/* Stats Panel */
	.stats-panel {
		position: fixed;
		top: 1rem;
		right: 1rem;
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
		padding: 1rem;
		background: linear-gradient(135deg, rgba(255, 255, 255, 0.95) 0%, rgba(249, 250, 251, 0.95) 100%);
		border: 1px solid rgba(99, 102, 241, 0.15);
		border-radius: 12px;
		box-shadow:
			0 10px 40px rgba(0, 0, 0, 0.1),
			0 0 0 1px rgba(255, 255, 255, 0.8) inset;
		backdrop-filter: blur(8px);
		z-index: 1000;
		min-width: 200px;
	}

	.stats-row {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 0.75rem;
	}

	.stat-item {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.125rem;
	}

	.stat-value {
		font-size: 1.125rem;
		font-weight: 700;
		color: #1f2937;
		font-variant-numeric: tabular-nums;
	}

	.stat-label {
		font-size: 0.6875rem;
		font-weight: 500;
		color: #9ca3af;
		text-transform: uppercase;
		letter-spacing: 0.025em;
	}

	.stat-divider {
		width: 1px;
		height: 24px;
		background: #e5e7eb;
	}

	/* Goal Section */
	.goal-section {
		position: relative;
	}

	.set-goal-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		width: 100%;
		padding: 0.5rem 0.75rem;
		background: rgba(99, 102, 241, 0.08);
		border: 1px dashed rgba(99, 102, 241, 0.3);
		border-radius: 8px;
		color: #6366f1;
		font-size: 0.8125rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.set-goal-btn:hover {
		background: rgba(99, 102, 241, 0.12);
		border-style: solid;
	}

	/* Goal Progress */
	.goal-progress-container {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.goal-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	.goal-text {
		font-size: 0.8125rem;
		font-weight: 500;
		color: #4b5563;
	}

	.goal-edit-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 24px;
		height: 24px;
		background: transparent;
		border: none;
		border-radius: 4px;
		color: #9ca3af;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.goal-edit-btn:hover {
		background: #f3f4f6;
		color: #6b7280;
	}

	.progress-bar {
		height: 8px;
		background: #e5e7eb;
		border-radius: 4px;
		overflow: hidden;
	}

	.progress-fill {
		height: 100%;
		background: linear-gradient(90deg, #6366f1 0%, #8b5cf6 100%);
		border-radius: 4px;
		transition: width 0.3s ease;
	}

	.progress-fill.goal-reached {
		background: linear-gradient(90deg, #10b981 0%, #34d399 100%);
		animation: pulse-glow 1.5s ease-in-out infinite;
	}

	@keyframes pulse-glow {
		0%,
		100% {
			box-shadow: 0 0 0 0 rgba(16, 185, 129, 0.4);
		}
		50% {
			box-shadow: 0 0 8px 2px rgba(16, 185, 129, 0.6);
		}
	}

	.goal-stats {
		display: flex;
		justify-content: space-between;
		font-size: 0.75rem;
		color: #9ca3af;
		font-variant-numeric: tabular-nums;
	}

	/* Goal Menu */
	.goal-menu {
		position: absolute;
		top: calc(100% + 0.5rem);
		left: 0;
		right: 0;
		padding: 0.75rem;
		background: white;
		border: 1px solid #e5e7eb;
		border-radius: 10px;
		box-shadow: 0 10px 30px rgba(0, 0, 0, 0.15);
		z-index: 10;
	}

	.goal-menu-header {
		font-size: 0.75rem;
		font-weight: 600;
		color: #6b7280;
		text-transform: uppercase;
		letter-spacing: 0.025em;
		margin-bottom: 0.5rem;
	}

	.goal-presets {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 0.375rem;
		margin-bottom: 0.5rem;
	}

	.goal-preset-btn {
		padding: 0.5rem 0.25rem;
		background: #f3f4f6;
		border: 1px solid transparent;
		border-radius: 6px;
		color: #4b5563;
		font-size: 0.8125rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.goal-preset-btn:hover {
		background: #e5e7eb;
	}

	.goal-preset-btn.active {
		background: #6366f1;
		color: white;
	}

	.goal-custom {
		display: flex;
		gap: 0.375rem;
	}

	.custom-goal-input {
		flex: 1;
		padding: 0.5rem 0.75rem;
		background: #f9fafb;
		border: 1px solid #e5e7eb;
		border-radius: 6px;
		font-size: 0.875rem;
		color: #1f2937;
		outline: none;
		transition: all 0.15s ease;
	}

	.custom-goal-input:focus {
		border-color: #6366f1;
		box-shadow: 0 0 0 3px rgba(99, 102, 241, 0.1);
	}

	.custom-goal-input::placeholder {
		color: #9ca3af;
	}

	/* Hide spinner on number input */
	.custom-goal-input::-webkit-outer-spin-button,
	.custom-goal-input::-webkit-inner-spin-button {
		-webkit-appearance: none;
		margin: 0;
	}

	.custom-goal-input[type='number'] {
		-moz-appearance: textfield;
	}

	.custom-goal-submit {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 36px;
		height: 36px;
		background: #6366f1;
		border: none;
		border-radius: 6px;
		color: white;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.custom-goal-submit:hover:not(:disabled) {
		background: #4f46e5;
	}

	.custom-goal-submit:disabled {
		background: #d1d5db;
		cursor: not-allowed;
	}

	.clear-goal-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.375rem;
		width: 100%;
		margin-top: 0.5rem;
		padding: 0.5rem;
		background: transparent;
		border: none;
		border-radius: 6px;
		color: #ef4444;
		font-size: 0.8125rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.clear-goal-btn:hover {
		background: #fef2f2;
	}

	/* Fullscreen Button */
	.fullscreen-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 100%;
		padding: 0.5rem;
		background: #f3f4f6;
		border: 1px solid #e5e7eb;
		border-radius: 8px;
		color: #6b7280;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.fullscreen-btn:hover {
		background: #e5e7eb;
		color: #374151;
	}

	/* Celebration Overlay */
	.celebration-overlay {
		position: fixed;
		inset: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(0, 0, 0, 0.6);
		backdrop-filter: blur(4px);
		z-index: 9999;
	}

	.celebration-content {
		position: relative;
		display: flex;
		flex-direction: column;
		align-items: center;
		padding: 2.5rem 3rem;
		background: linear-gradient(135deg, #ffffff 0%, #f9fafb 100%);
		border-radius: 20px;
		box-shadow:
			0 25px 60px rgba(0, 0, 0, 0.25),
			0 0 0 1px rgba(255, 255, 255, 0.8) inset;
		text-align: center;
		overflow: hidden;
	}

	.celebration-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 80px;
		height: 80px;
		background: linear-gradient(135deg, #fbbf24 0%, #f59e0b 100%);
		border-radius: 50%;
		color: white;
		margin-bottom: 1.25rem;
		animation: bounce 0.6s ease infinite;
	}

	@keyframes bounce {
		0%,
		100% {
			transform: translateY(0);
		}
		50% {
			transform: translateY(-8px);
		}
	}

	.celebration-title {
		font-size: 1.5rem;
		font-weight: 700;
		color: #1f2937;
		margin: 0 0 0.5rem;
	}

	.celebration-text {
		font-size: 1rem;
		color: #6b7280;
		margin: 0;
	}

	/* Confetti */
	.confetti {
		position: absolute;
		top: 0;
		left: 50%;
		transform: translateX(-50%);
		width: 100%;
		height: 100%;
		pointer-events: none;
		overflow: hidden;
	}

	.confetti-piece {
		position: absolute;
		top: 0;
		left: 50%;
		width: 10px;
		height: 10px;
		background: var(--color);
		border-radius: 2px;
		animation: confetti-fall 2s var(--delay) ease-out forwards;
		transform: translateX(var(--x)) rotate(var(--rotation));
	}

	@keyframes confetti-fall {
		0% {
			top: 0;
			opacity: 1;
			transform: translateX(var(--x)) rotate(var(--rotation)) scale(1);
		}
		100% {
			top: 100%;
			opacity: 0;
			transform: translateX(calc(var(--x) * 1.5)) rotate(calc(var(--rotation) + 360deg)) scale(0.5);
		}
	}

	/* Dark mode support */
	@media (prefers-color-scheme: dark) {
		.stats-panel {
			background: linear-gradient(135deg, rgba(30, 41, 59, 0.95) 0%, rgba(15, 23, 42, 0.95) 100%);
			border-color: rgba(99, 102, 241, 0.2);
		}

		.stat-value {
			color: #f1f5f9;
		}

		.stat-label {
			color: #64748b;
		}

		.stat-divider {
			background: #334155;
		}

		.goal-text {
			color: #cbd5e1;
		}

		.goal-edit-btn:hover {
			background: #334155;
			color: #94a3b8;
		}

		.progress-bar {
			background: #334155;
		}

		.goal-menu {
			background: #1e293b;
			border-color: #334155;
		}

		.goal-menu-header {
			color: #94a3b8;
		}

		.goal-preset-btn {
			background: #334155;
			color: #cbd5e1;
		}

		.goal-preset-btn:hover {
			background: #475569;
		}

		.custom-goal-input {
			background: #0f172a;
			border-color: #334155;
			color: #f1f5f9;
		}

		.custom-goal-input:focus {
			border-color: #6366f1;
		}

		.fullscreen-btn {
			background: #334155;
			border-color: #475569;
			color: #94a3b8;
		}

		.fullscreen-btn:hover {
			background: #475569;
			color: #cbd5e1;
		}

		.focus-toggle-btn {
			border-color: #334155;
			color: #94a3b8;
		}

		.focus-toggle-btn:hover {
			background: #334155;
			border-color: #475569;
			color: #cbd5e1;
		}
	}

	/* Responsive adjustments */
	@media (max-width: 640px) {
		.stats-panel {
			top: auto;
			bottom: 1rem;
			right: 1rem;
			left: 1rem;
			min-width: auto;
		}

		.celebration-content {
			margin: 1rem;
			padding: 2rem;
		}
	}
</style>

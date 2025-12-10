<!--
/**
 * ModuleAccordion Component - Learning Center
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Expandable accordion for organizing lessons into modules/sections.
 * Shows progress tracking and lesson counts per module.
 *
 * @version 1.0.0 (December 2025)
 */
-->

<script lang="ts">
	import type { LessonModule, LessonWithRelations, UserModuleProgress } from '$lib/types/learning-center';
	import { slide } from 'svelte/transition';
	import { quintOut } from 'svelte/easing';
	import LessonCard from './LessonCard.svelte';
	import { IconChevronDown, IconCheck } from '$lib/icons';

	interface Props {
		module: LessonModule;
		lessons: LessonWithRelations[];
		progress?: UserModuleProgress;
		isExpanded?: boolean;
		onToggle?: (moduleId: string) => void;
		onLessonClick?: (lesson: LessonWithRelations) => void;
		onBookmark?: (lessonId: string) => void;
		basePath?: string;
	}

	let {
		module,
		lessons,
		progress,
		isExpanded = $bindable(false),
		onToggle,
		onLessonClick,
		onBookmark,
		basePath = ''
	}: Props = $props();

	// Computed values
	let completedCount = $derived(progress?.completedLessons ?? 0);
	let totalCount = $derived(lessons.length);
	let progressPercent = $derived(totalCount > 0 ? (completedCount / totalCount) * 100 : 0);
	let isComplete = $derived(progressPercent === 100);

	function handleToggle() {
		isExpanded = !isExpanded;
		onToggle?.(module.id);
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter' || e.key === ' ') {
			e.preventDefault();
			handleToggle();
		}
	}
</script>

<div class="module-accordion" class:expanded={isExpanded} class:complete={isComplete}>
	<!-- Module Header -->
	<button
		type="button"
		class="module-header"
		onclick={handleToggle}
		onkeydown={handleKeydown}
		aria-expanded={isExpanded}
		aria-controls="module-content-{module.id}"
	>
		<div class="module-info">
			<!-- Completion indicator -->
			<div class="completion-indicator" class:complete={isComplete}>
				{#if isComplete}
					<IconCheck size={16} />
				{:else}
					<span class="progress-ring">
						<svg viewBox="0 0 36 36">
							<path
								class="ring-bg"
								d="M18 2.0845
									a 15.9155 15.9155 0 0 1 0 31.831
									a 15.9155 15.9155 0 0 1 0 -31.831"
							/>
							<path
								class="ring-fill"
								stroke-dasharray="{progressPercent}, 100"
								d="M18 2.0845
									a 15.9155 15.9155 0 0 1 0 31.831
									a 15.9155 15.9155 0 0 1 0 -31.831"
							/>
						</svg>
					</span>
				{/if}
			</div>

			<!-- Module title and description -->
			<div class="module-text">
				<h3 class="module-title">{module.name}</h3>
				{#if module.description}
					<p class="module-description">{module.description}</p>
				{/if}
			</div>
		</div>

		<!-- Module stats -->
		<div class="module-stats">
			<span class="lesson-count">
				{completedCount} / {totalCount} lessons
			</span>
			<span class="chevron" class:rotated={isExpanded}>
				<IconChevronDown size={20} />
			</span>
		</div>
	</button>

	<!-- Progress bar -->
	{#if totalCount > 0}
		<div class="progress-bar-container">
			<div
				class="progress-bar"
				style="width: {progressPercent}%"
				role="progressbar"
				aria-valuenow={progressPercent}
				aria-valuemin={0}
				aria-valuemax={100}
			></div>
		</div>
	{/if}

	<!-- Module Content (Lessons) -->
	{#if isExpanded}
		<div
			id="module-content-{module.id}"
			class="module-content"
			transition:slide={{ duration: 300, easing: quintOut }}
		>
			<div class="lessons-list">
				{#each lessons as lesson, index (lesson.id)}
					<div class="lesson-item" style="animation-delay: {index * 50}ms">
						<LessonCard
							{lesson}
							compact
							showProgress
							showTrainer={false}
							href="{basePath}/{lesson.slug}"
							{onBookmark}
						/>
					</div>
				{/each}

				{#if lessons.length === 0}
					<div class="empty-state">
						<p>No lessons in this module yet.</p>
					</div>
				{/if}
			</div>
		</div>
	{/if}
</div>

<style>
	.module-accordion {
		background: #1e293b;
		border-radius: 12px;
		border: 1px solid #334155;
		overflow: hidden;
		transition: all 0.3s ease;
	}

	.module-accordion:hover {
		border-color: rgba(249, 115, 22, 0.2);
	}

	.module-accordion.expanded {
		border-color: rgba(249, 115, 22, 0.3);
	}

	.module-accordion.complete {
		border-color: rgba(34, 197, 94, 0.3);
	}

	/* Module Header */
	.module-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		width: 100%;
		padding: 1rem 1.25rem;
		background: none;
		border: none;
		cursor: pointer;
		text-align: left;
		transition: background 0.2s ease;
	}

	.module-header:hover {
		background: rgba(255, 255, 255, 0.02);
	}

	.module-info {
		display: flex;
		align-items: center;
		gap: 1rem;
		flex: 1;
		min-width: 0;
	}

	/* Completion indicator */
	.completion-indicator {
		flex-shrink: 0;
		width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		border-radius: 50%;
		background: #334155;
		color: #64748b;
	}

	.completion-indicator.complete {
		background: #22c55e;
		color: white;
	}

	.progress-ring {
		width: 28px;
		height: 28px;
	}

	.progress-ring svg {
		width: 100%;
		height: 100%;
		transform: rotate(-90deg);
	}

	.ring-bg {
		fill: none;
		stroke: #334155;
		stroke-width: 3;
	}

	.ring-fill {
		fill: none;
		stroke: #f97316;
		stroke-width: 3;
		stroke-linecap: round;
		transition: stroke-dasharray 0.3s ease;
	}

	/* Module text */
	.module-text {
		flex: 1;
		min-width: 0;
	}

	.module-title {
		margin: 0;
		font-size: 1rem;
		font-weight: 600;
		color: white;
		line-height: 1.3;
	}

	.module-description {
		margin: 0.25rem 0 0;
		font-size: 0.8rem;
		color: #94a3b8;
		line-height: 1.4;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	/* Module stats */
	.module-stats {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		flex-shrink: 0;
	}

	.lesson-count {
		font-size: 0.75rem;
		color: #64748b;
		font-weight: 500;
	}

	.chevron {
		display: flex;
		align-items: center;
		justify-content: center;
		color: #64748b;
		transition: transform 0.3s ease;
	}

	.chevron.rotated {
		transform: rotate(180deg);
	}

	/* Progress bar */
	.progress-bar-container {
		height: 2px;
		background: #334155;
	}

	.progress-bar {
		height: 100%;
		background: linear-gradient(90deg, #f97316, #ea580c);
		transition: width 0.3s ease;
	}

	.module-accordion.complete .progress-bar {
		background: #22c55e;
	}

	/* Module Content */
	.module-content {
		padding: 0 1rem 1rem;
		background: rgba(0, 0, 0, 0.1);
	}

	.lessons-list {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		padding-top: 0.5rem;
	}

	.lesson-item {
		opacity: 0;
		animation: fadeInUp 0.3s ease forwards;
	}

	@keyframes fadeInUp {
		from {
			opacity: 0;
			transform: translateY(10px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	.empty-state {
		padding: 2rem;
		text-align: center;
	}

	.empty-state p {
		margin: 0;
		font-size: 0.875rem;
		color: #64748b;
	}

	/* Responsive */
	@media (max-width: 640px) {
		.module-header {
			padding: 0.875rem 1rem;
		}

		.module-info {
			gap: 0.75rem;
		}

		.completion-indicator {
			width: 32px;
			height: 32px;
		}

		.module-title {
			font-size: 0.9rem;
		}

		.module-description {
			display: none;
		}

		.lesson-count {
			font-size: 0.7rem;
		}
	}
</style>

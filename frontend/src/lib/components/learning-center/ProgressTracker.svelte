<!--
/**
 * ProgressTracker Component - Learning Center
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Shows user's learning progress with visual indicators.
 * Displays overall completion, recent activity, and achievements.
 *
 * @version 1.0.0 (December 2025)
 */
-->

<script lang="ts">
	import type { UserRoomProgress, LessonWithRelations } from '$lib/types/learning-center';
	import { IconCheck, IconClock, IconTrendingUp, IconAward } from '$lib/icons';

	interface Props {
		progress?: UserRoomProgress;
		recentLessons?: LessonWithRelations[];
		showStats?: boolean;
		showRecentActivity?: boolean;
		compact?: boolean;
	}

	let {
		progress,
		recentLessons = [],
		showStats = true,
		showRecentActivity = true,
		compact = false
	}: Props = $props();

	// Computed values
	let completedLessons = $derived(progress?.completedLessons ?? 0);
	let totalLessons = $derived(progress?.totalLessons ?? 0);
	let progressPercent = $derived(progress?.progressPercent ?? 0);
	let circumference = 2 * Math.PI * 45; // Circle radius is 45

	// Stats calculation
	let stats = $derived.by(() => {
		const totalModules = progress?.moduleProgress?.length ?? 0;
		const completedModules = progress?.moduleProgress?.filter(
			m => m.progressPercent === 100
		).length ?? 0;

		return {
			totalModules,
			completedModules,
			inProgress: totalModules - completedModules
		};
	});
</script>

<div class="progress-tracker" class:compact>
	<!-- Main progress circle -->
	<div class="progress-main">
		<div class="progress-circle-container">
			<svg class="progress-circle" viewBox="0 0 100 100">
				<!-- Background circle -->
				<circle
					cx="50"
					cy="50"
					r="45"
					fill="none"
					stroke="#334155"
					stroke-width="8"
				/>
				<!-- Progress circle -->
				<circle
					cx="50"
					cy="50"
					r="45"
					fill="none"
					stroke="url(#progressGradient)"
					stroke-width="8"
					stroke-linecap="round"
					stroke-dasharray={circumference}
					stroke-dashoffset={circumference - (circumference * progressPercent) / 100}
					transform="rotate(-90 50 50)"
					class="progress-ring"
				/>
				<!-- Gradient definition -->
				<defs>
					<linearGradient id="progressGradient" x1="0%" y1="0%" x2="100%" y2="0%">
						<stop offset="0%" stop-color="#f97316" />
						<stop offset="100%" stop-color="#ea580c" />
					</linearGradient>
				</defs>
			</svg>
			<div class="progress-text">
				<span class="progress-percent">{Math.round(progressPercent)}%</span>
				<span class="progress-label">Complete</span>
			</div>
		</div>

		{#if !compact}
			<div class="progress-details">
				<h3 class="progress-title">Your Progress</h3>
				<p class="progress-subtitle">
					{completedLessons} of {totalLessons} lessons completed
				</p>

				{#if progressPercent === 100}
					<div class="completion-badge">
						<IconAward size={20} />
						<span>Course Complete!</span>
					</div>
				{:else if progressPercent > 0}
					<p class="progress-encouragement">
						{#if progressPercent < 25}
							Great start! Keep going!
						{:else if progressPercent < 50}
							You're making progress!
						{:else if progressPercent < 75}
							More than halfway there!
						{:else}
							Almost done! Final stretch!
						{/if}
					</p>
				{/if}
			</div>
		{/if}
	</div>

	<!-- Stats grid -->
	{#if showStats && !compact}
		<div class="stats-grid">
			<div class="stat-card">
				<div class="stat-icon completed">
					<IconCheck size={20} />
				</div>
				<div class="stat-content">
					<span class="stat-value">{completedLessons}</span>
					<span class="stat-label">Completed</span>
				</div>
			</div>

			<div class="stat-card">
				<div class="stat-icon remaining">
					<IconClock size={20} />
				</div>
				<div class="stat-content">
					<span class="stat-value">{totalLessons - completedLessons}</span>
					<span class="stat-label">Remaining</span>
				</div>
			</div>

			<div class="stat-card">
				<div class="stat-icon modules">
					<IconTrendingUp size={20} />
				</div>
				<div class="stat-content">
					<span class="stat-value">{stats.completedModules}/{stats.totalModules}</span>
					<span class="stat-label">Modules</span>
				</div>
			</div>
		</div>
	{/if}

	<!-- Recent activity -->
	{#if showRecentActivity && recentLessons.length > 0 && !compact}
		<div class="recent-activity">
			<h4 class="section-title">Continue Learning</h4>
			<div class="recent-lessons">
				{#each recentLessons.slice(0, 3) as lesson}
					<a href="/learning-center/{lesson.slug}" class="recent-lesson">
						<div class="recent-thumbnail">
							{#if lesson.thumbnailUrl}
								<img src={lesson.thumbnailUrl} alt={lesson.title} />
							{/if}
							{#if lesson.userProgress?.progressPercent}
								<div class="mini-progress">
									<div
										class="mini-progress-fill"
										style="width: {lesson.userProgress.progressPercent}%"
									></div>
								</div>
							{/if}
						</div>
						<div class="recent-info">
							<span class="recent-title">{lesson.title}</span>
							<span class="recent-meta">
								{lesson.userProgress?.progressPercent ?? 0}% complete
							</span>
						</div>
					</a>
				{/each}
			</div>
		</div>
	{/if}
</div>

<style>
	.progress-tracker {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
		padding: 1.5rem;
		background: #1e293b;
		border-radius: 12px;
		border: 1px solid #334155;
	}

	.progress-tracker.compact {
		padding: 1rem;
		gap: 0;
	}

	/* Main progress */
	.progress-main {
		display: flex;
		align-items: center;
		gap: 1.5rem;
	}

	.progress-tracker.compact .progress-main {
		justify-content: center;
	}

	.progress-circle-container {
		position: relative;
		width: 120px;
		height: 120px;
		flex-shrink: 0;
	}

	.progress-tracker.compact .progress-circle-container {
		width: 80px;
		height: 80px;
	}

	.progress-circle {
		width: 100%;
		height: 100%;
	}

	.progress-ring {
		transition: stroke-dashoffset 0.5s ease;
	}

	.progress-text {
		position: absolute;
		inset: 0;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
	}

	.progress-percent {
		font-size: 1.75rem;
		font-weight: 700;
		color: white;
		line-height: 1;
	}

	.progress-tracker.compact .progress-percent {
		font-size: 1.25rem;
	}

	.progress-label {
		font-size: 0.7rem;
		color: #94a3b8;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.progress-tracker.compact .progress-label {
		font-size: 0.6rem;
	}

	.progress-details {
		flex: 1;
	}

	.progress-title {
		margin: 0;
		font-size: 1.125rem;
		font-weight: 600;
		color: white;
	}

	.progress-subtitle {
		margin: 0.25rem 0 0;
		font-size: 0.875rem;
		color: #94a3b8;
	}

	.completion-badge {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		margin-top: 0.75rem;
		padding: 0.5rem 1rem;
		background: linear-gradient(135deg, #22c55e, #16a34a);
		border-radius: 9999px;
		color: white;
		font-size: 0.875rem;
		font-weight: 600;
	}

	.progress-encouragement {
		margin: 0.75rem 0 0;
		font-size: 0.875rem;
		color: #f97316;
		font-weight: 500;
	}

	/* Stats grid */
	.stats-grid {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 0.75rem;
	}

	.stat-card {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.875rem;
		background: #0f172a;
		border-radius: 8px;
	}

	.stat-icon {
		width: 40px;
		height: 40px;
		display: flex;
		align-items: center;
		justify-content: center;
		border-radius: 8px;
	}

	.stat-icon.completed {
		background: rgba(34, 197, 94, 0.1);
		color: #22c55e;
	}

	.stat-icon.remaining {
		background: rgba(59, 130, 246, 0.1);
		color: #3b82f6;
	}

	.stat-icon.modules {
		background: rgba(249, 115, 22, 0.1);
		color: #f97316;
	}

	.stat-content {
		display: flex;
		flex-direction: column;
	}

	.stat-value {
		font-size: 1.25rem;
		font-weight: 700;
		color: white;
		line-height: 1.2;
	}

	.stat-label {
		font-size: 0.7rem;
		color: #64748b;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	/* Recent activity */
	.recent-activity {
		border-top: 1px solid #334155;
		padding-top: 1.5rem;
	}

	.section-title {
		margin: 0 0 1rem;
		font-size: 0.875rem;
		font-weight: 600;
		color: white;
	}

	.recent-lessons {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.recent-lesson {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.5rem;
		background: #0f172a;
		border-radius: 8px;
		text-decoration: none;
		transition: all 0.2s ease;
	}

	.recent-lesson:hover {
		background: #1e293b;
		transform: translateX(4px);
	}

	.recent-thumbnail {
		position: relative;
		width: 64px;
		height: 36px;
		border-radius: 4px;
		overflow: hidden;
		background: #334155;
		flex-shrink: 0;
	}

	.recent-thumbnail img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.mini-progress {
		position: absolute;
		bottom: 0;
		left: 0;
		right: 0;
		height: 3px;
		background: rgba(0, 0, 0, 0.5);
	}

	.mini-progress-fill {
		height: 100%;
		background: #f97316;
	}

	.recent-info {
		display: flex;
		flex-direction: column;
		min-width: 0;
	}

	.recent-title {
		font-size: 0.8rem;
		font-weight: 500;
		color: white;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.recent-meta {
		font-size: 0.7rem;
		color: #64748b;
	}

	/* Responsive */
	@media (max-width: 640px) {
		.progress-main {
			flex-direction: column;
			text-align: center;
		}

		.stats-grid {
			grid-template-columns: 1fr;
		}

		.stat-card {
			justify-content: center;
		}
	}
</style>

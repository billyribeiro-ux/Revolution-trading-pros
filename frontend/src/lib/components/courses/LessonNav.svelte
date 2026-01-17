<script lang="ts">
	/**
	 * LessonNav Component
	 * Apple Principal Engineer ICT 7 Grade - January 2026
	 * Navigation between lessons with prev/next controls
	 */

	interface Lesson {
		id: string;
		title: string;
		slug: string;
	}

	interface Props {
		currentLesson: Lesson;
		prevLesson?: Lesson | null;
		nextLesson?: Lesson | null;
		courseSlug: string;
		onNavigate?: (lesson: Lesson) => void;
	}

	let { currentLesson, prevLesson, nextLesson, courseSlug, onNavigate }: Props = $props();

	const handleNav = (lesson: Lesson | null | undefined) => {
		if (!lesson) return;
		if (onNavigate) {
			onNavigate(lesson);
		} else {
			window.location.href = `/classes/${courseSlug}/${lesson.slug}`;
		}
	};
</script>

<nav class="lesson-nav">
	<button class="nav-btn prev" disabled={!prevLesson} onclick={() => handleNav(prevLesson)}>
		<svg
			xmlns="http://www.w3.org/2000/svg"
			width="20"
			height="20"
			viewBox="0 0 24 24"
			fill="none"
			stroke="currentColor"
			stroke-width="2"><path d="m15 18-6-6 6-6" /></svg
		>
		<div class="nav-text">
			<span class="label">Previous</span>
			{#if prevLesson}
				<span class="title">{prevLesson.title}</span>
			{/if}
		</div>
	</button>

	<div class="current">
		<span class="current-label">Current Lesson</span>
		<span class="current-title">{currentLesson.title}</span>
	</div>

	<button class="nav-btn next" disabled={!nextLesson} onclick={() => handleNav(nextLesson)}>
		<div class="nav-text">
			<span class="label">Next</span>
			{#if nextLesson}
				<span class="title">{nextLesson.title}</span>
			{/if}
		</div>
		<svg
			xmlns="http://www.w3.org/2000/svg"
			width="20"
			height="20"
			viewBox="0 0 24 24"
			fill="none"
			stroke="currentColor"
			stroke-width="2"><path d="m9 18 6-6-6-6" /></svg
		>
	</button>
</nav>

<style>
	.lesson-nav {
		display: flex;
		align-items: stretch;
		justify-content: space-between;
		gap: 16px;
		padding: 16px 24px;
		background: #1e293b;
		border-top: 1px solid #334155;
	}

	.nav-btn {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 12px 16px;
		background: #334155;
		border: none;
		border-radius: 8px;
		color: #fff;
		cursor: pointer;
		transition: all 0.2s;
		min-width: 180px;
	}

	.nav-btn:hover:not(:disabled) {
		background: #475569;
	}

	.nav-btn:disabled {
		opacity: 0.4;
		cursor: not-allowed;
	}

	.nav-btn.next {
		text-align: right;
	}

	.nav-btn.next .nav-text {
		align-items: flex-end;
	}

	.nav-text {
		display: flex;
		flex-direction: column;
		gap: 2px;
		flex: 1;
	}

	.label {
		font-size: 11px;
		color: #94a3b8;
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.title {
		font-size: 14px;
		font-weight: 500;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
		max-width: 150px;
	}

	.current {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 4px;
		text-align: center;
	}

	.current-label {
		font-size: 11px;
		color: #64748b;
		text-transform: uppercase;
	}

	.current-title {
		font-size: 14px;
		color: #e2e8f0;
		font-weight: 500;
	}

	@media (max-width: 768px) {
		.lesson-nav {
			flex-wrap: wrap;
			padding: 12px;
		}

		.nav-btn {
			min-width: 120px;
			flex: 1;
		}

		.current {
			order: -1;
			width: 100%;
			padding-bottom: 12px;
			border-bottom: 1px solid #334155;
			margin-bottom: 4px;
		}

		.title {
			max-width: 100px;
		}
	}
</style>

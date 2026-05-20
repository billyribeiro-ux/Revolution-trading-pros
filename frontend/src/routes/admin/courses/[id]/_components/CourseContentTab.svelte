<script lang="ts">
	/**
	 * R19-C extraction (2026-05-20): Content tab — module/lesson tree with
	 * add/delete via discriminated callbacks. Parent owns mutation logic.
	 */
	import IconPlus from '@tabler/icons-svelte-runes/icons/plus';
	import IconTrash from '@tabler/icons-svelte-runes/icons/trash';
	import IconPlayerPlay from '@tabler/icons-svelte-runes/icons/player-play';
	import IconX from '@tabler/icons-svelte-runes/icons/x';

	interface Lesson {
		id: string;
		title: string;
		slug: string;
		description?: string;
		duration_minutes?: number;
		bunny_video_guid?: string;
		thumbnail_url?: string;
		is_free: boolean;
		is_preview?: boolean;
		is_published?: boolean;
		sort_order?: number;
		module_id?: number;
	}

	interface Module {
		id: number;
		title: string;
		description?: string;
		sort_order: number;
		is_published: boolean;
		lessons: Lesson[];
	}

	interface Props {
		courseId: string;
		modules: Module[];
		unassignedLessons: Lesson[];
		onAddModule: () => void;
		onDeleteModule: (moduleId: number) => void;
		onAddLesson: (moduleId?: number) => void;
		onDeleteLesson: (lessonId: string, moduleId?: number) => void;
	}

	let {
		courseId,
		modules,
		unassignedLessons,
		onAddModule,
		onDeleteModule,
		onAddLesson,
		onDeleteLesson
	}: Props = $props();
</script>

<div class="content-section">
	<div class="content-header">
		<h2>Modules & Lessons</h2>
		<button class="btn-secondary" onclick={onAddModule}>
			<IconPlus size={16} aria-hidden="true" />
			Add Module
		</button>
	</div>

	{#each modules as mod (mod.id)}
		<div class="module-card">
			<div class="module-header">
				<h3>{mod.title}</h3>
				<div class="module-actions">
					<button onclick={() => onAddLesson(mod.id)} aria-label="Add lesson">
						<IconPlus size={16} aria-hidden="true" />
					</button>
					<button onclick={() => onDeleteModule(mod.id)} aria-label="Delete module">
						<IconTrash size={16} aria-hidden="true" />
					</button>
				</div>
			</div>
			<ul class="lesson-list">
				{#each mod.lessons as lesson (lesson.id)}
					<li>
						<a href="/admin/courses/{courseId}/lessons/{lesson.id}">
							<IconPlayerPlay size={16} aria-hidden="true" />
							<span>{lesson.title}</span>
							{#if lesson.duration_minutes}
								<span class="duration">{lesson.duration_minutes}m</span>
							{/if}
						</a>
						<button
							onclick={() => onDeleteLesson(lesson.id, mod.id)}
							aria-label="Delete lesson"
						>
							<IconX size={14} aria-hidden="true" />
						</button>
					</li>
				{/each}
				{#if mod.lessons.length === 0}
					<li class="empty">No lessons yet</li>
				{/if}
			</ul>
		</div>
	{/each}

	{#if unassignedLessons.length > 0}
		<div class="module-card unassigned">
			<div class="module-header">
				<h3>Unassigned Lessons</h3>
			</div>
			<ul class="lesson-list">
				{#each unassignedLessons as lesson (lesson.id)}
					<li>
						<a href="/admin/courses/{courseId}/lessons/{lesson.id}">
							<IconPlayerPlay size={16} aria-hidden="true" />
							<span>{lesson.title}</span>
						</a>
						<button onclick={() => onDeleteLesson(lesson.id)} aria-label="Delete lesson">
							<IconX size={14} aria-hidden="true" />
						</button>
					</li>
				{/each}
			</ul>
		</div>
	{/if}

	<button class="btn-add-lesson" onclick={() => onAddLesson()}>
		<IconPlus size={20} aria-hidden="true" />
		Add Lesson (No Module)
	</button>
</div>

<style>
	.content-section {
		background: #fff;
		border: 1px solid #e5e7eb;
		border-radius: 12px;
		padding: 24px;
	}
	.content-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 20px;
	}
	.content-header h2 {
		font-size: 16px;
		font-weight: 600;
		color: #1f2937;
		margin: 0;
	}

	.btn-secondary {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		padding: 10px 16px;
		border-radius: 8px;
		font-size: 14px;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
		text-decoration: none;
		border: none;
		background: #f3f4f6;
		color: #1f2937;
	}
	.btn-secondary:hover {
		background: #e5e7eb;
	}

	.module-card {
		background: #f9fafb;
		border-radius: 8px;
		margin-bottom: 16px;
		overflow: hidden;
	}
	.module-card.unassigned {
		background: #fef3c7;
	}
	.module-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 12px 16px;
		background: #e5e7eb;
	}
	.module-card.unassigned .module-header {
		background: #fcd34d;
	}
	.module-header h3 {
		font-size: 14px;
		font-weight: 600;
		margin: 0;
	}
	.module-actions {
		display: flex;
		gap: 8px;
	}
	.module-actions button {
		background: none;
		border: none;
		color: #6b7280;
		cursor: pointer;
		padding: 4px;
	}
	.module-actions button:hover {
		color: #1f2937;
	}

	.lesson-list {
		list-style: none;
		margin: 0;
		padding: 0;
	}
	.lesson-list li {
		display: flex;
		align-items: center;
		padding: 12px 16px;
		border-bottom: 1px solid #e5e7eb;
	}
	.lesson-list li:last-child {
		border-bottom: none;
	}
	.lesson-list li.empty {
		color: #9ca3af;
		font-style: italic;
		font-size: 13px;
	}
	.lesson-list li a {
		display: flex;
		align-items: center;
		gap: 10px;
		flex: 1;
		text-decoration: none;
		color: #1f2937;
		font-size: 14px;
	}
	.lesson-list li a:hover {
		color: #143e59;
	}
	.lesson-list .duration {
		color: #9ca3af;
		font-size: 12px;
		margin-left: auto;
	}
	.lesson-list li > button {
		background: none;
		border: none;
		color: #9ca3af;
		cursor: pointer;
		padding: 4px;
	}
	.lesson-list li > button:hover {
		color: #dc2626;
	}

	.btn-add-lesson {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		width: 100%;
		padding: 12px;
		background: #f3f4f6;
		border: 2px dashed #d1d5db;
		border-radius: 8px;
		color: #6b7280;
		font-size: 14px;
		cursor: pointer;
		margin-top: 16px;
	}
	.btn-add-lesson:hover {
		background: #e5e7eb;
		border-color: #9ca3af;
	}

	@media (prefers-reduced-motion: reduce) {
		.btn-secondary {
			transition: none;
		}
	}
</style>

<script lang="ts">
	/**
	 * R19-C extraction (2026-05-20): Settings tab — SEO meta + Bunny video
	 * library binding. Uses `$bindable` on `course` so the existing
	 * `bind:value` chains keep working.
	 *
	 * Course interface duplicated here (full shape) — R18-C precedent:
	 * narrowed shapes get rejected by svelte-check.
	 */

	interface Course {
		id: string;
		title: string;
		slug: string;
		description?: string;
		card_description?: string;
		card_image_url?: string;
		card_badge?: string;
		card_badge_color?: string;
		price_cents: number;
		is_free?: boolean;
		is_published: boolean;
		status?: string;
		level?: string;
		instructor_name?: string;
		instructor_title?: string;
		instructor_avatar_url?: string;
		instructor_bio?: string;
		what_you_learn?: string[];
		requirements?: string[];
		target_audience?: string[];
		meta_title?: string;
		meta_description?: string;
		bunny_library_id?: number;
	}

	interface Props {
		course: Course;
	}

	let { course = $bindable() }: Props = $props();
</script>

<div class="form-section">
	<h2>SEO Settings</h2>
	<div class="form-grid">
		<div class="form-group full">
			<label for="meta_title">Meta Title</label>
			<input id="meta_title" name="meta_title" type="text" bind:value={course.meta_title} />
		</div>
		<div class="form-group full">
			<label for="meta_desc">Meta Description</label>
			<textarea id="meta_desc" rows="3" bind:value={course.meta_description}></textarea>
		</div>
	</div>
</div>

<div class="form-section">
	<h2>Video Settings</h2>
	<div class="form-grid">
		<div class="form-group">
			<label for="bunny_lib">Bunny.net Library ID</label>
			<input id="bunny_lib" name="bunny_lib" type="number" bind:value={course.bunny_library_id} />
		</div>
	</div>
</div>

<style>
	.form-section {
		background: #fff;
		border: 1px solid #e5e7eb;
		border-radius: 12px;
		padding: 24px;
		margin-bottom: 24px;
	}
	.form-section h2 {
		font-size: 16px;
		font-weight: 600;
		color: #1f2937;
		margin: 0 0 20px;
	}
	.form-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 16px;
	}
	.form-group {
		display: flex;
		flex-direction: column;
		gap: 6px;
	}
	.form-group.full {
		grid-column: 1 / -1;
	}
	.form-group label {
		font-size: 13px;
		font-weight: 500;
		color: #374151;
	}
	.form-group input,
	.form-group textarea {
		padding: 10px 12px;
		border: 1px solid #e5e7eb;
		border-radius: 6px;
		font-size: 14px;
	}
	.form-group input:focus,
	.form-group textarea:focus {
		outline: none;
		border-color: #143e59;
	}

	@media (max-width: 767.98px) {
		.form-grid {
			grid-template-columns: 1fr;
		}
	}
</style>

<script lang="ts">
	/**
	 * R19-C extraction (2026-05-20): Course Details tab — Basic Info,
	 * Instructor, Card Display. Uses `$bindable` on the parent's `course`
	 * rune so the existing `bind:value` chains keep working unchanged.
	 *
	 * The Course interface is duplicated here (full shape) rather than
	 * narrowed to an index-signature; R18-C precedent: svelte-check rejected
	 * the narrowed shape with "Two different types with this name exist".
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
	<h2>Basic Information</h2>
	<div class="form-grid">
		<div class="form-group full">
			<label for="title">Title</label>
			<input id="title" name="title" type="text" bind:value={course.title} />
		</div>
		<div class="form-group full">
			<label for="slug">Slug</label>
			<input id="slug" name="slug" type="text" bind:value={course.slug} />
		</div>
		<div class="form-group full">
			<label for="description">Description</label>
			<textarea id="description" rows="4" bind:value={course.description}></textarea>
		</div>
		<div class="form-group full">
			<label for="card_description">Card Description (short)</label>
			<input
				id="card_description"
				name="card_description"
				type="text"
				bind:value={course.card_description}
			/>
		</div>
		<div class="form-group">
			<label for="level">Level</label>
			<select id="level" bind:value={course.level}>
				<option value="">Select level</option>
				<option value="Beginner">Beginner</option>
				<option value="Intermediate">Intermediate</option>
				<option value="Advanced">Advanced</option>
			</select>
		</div>
		<div class="form-group">
			<label for="price">Price (cents)</label>
			<input id="price" name="price" type="number" bind:value={course.price_cents} />
		</div>
		<div class="form-group">
			<label class="checkbox-label">
				<input
					id="page-course-is-free"
					name="page-course-is-free"
					type="checkbox"
					bind:checked={course.is_free}
				/>
				Free Course
			</label>
		</div>
	</div>
</div>

<div class="form-section">
	<h2>Instructor</h2>
	<div class="form-grid">
		<div class="form-group">
			<label for="instructor_name">Name</label>
			<input
				id="instructor_name"
				name="instructor_name"
				type="text"
				bind:value={course.instructor_name}
			/>
		</div>
		<div class="form-group">
			<label for="instructor_title">Title</label>
			<input
				id="instructor_title"
				name="instructor_title"
				type="text"
				bind:value={course.instructor_title}
			/>
		</div>
		<div class="form-group full">
			<label for="instructor_bio">Bio</label>
			<textarea id="instructor_bio" rows="3" bind:value={course.instructor_bio}></textarea>
		</div>
	</div>
</div>

<div class="form-section">
	<h2>Card Display</h2>
	<div class="form-grid">
		<div class="form-group full">
			<label for="card_image">Card Image URL</label>
			<input id="card_image" name="card_image" type="text" bind:value={course.card_image_url} />
		</div>
		<div class="form-group">
			<label for="badge">Badge Text</label>
			<input
				id="badge"
				name="badge"
				type="text"
				bind:value={course.card_badge}
				placeholder="e.g., NEW"
			/>
		</div>
		<div class="form-group">
			<label for="badge_color">Badge Color</label>
			<input
				id="badge_color"
				name="badge_color"
				type="color"
				bind:value={course.card_badge_color}
			/>
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
	.form-group select,
	.form-group textarea {
		padding: 10px 12px;
		border: 1px solid #e5e7eb;
		border-radius: 6px;
		font-size: 14px;
	}
	.form-group input:focus,
	.form-group select:focus,
	.form-group textarea:focus {
		outline: none;
		border-color: #143e59;
	}
	.checkbox-label {
		display: flex;
		align-items: center;
		gap: 8px;
		cursor: pointer;
	}
	.checkbox-label input {
		width: 16px;
		height: 16px;
	}

	@media (max-width: 767.98px) {
		.form-grid {
			grid-template-columns: 1fr;
		}
	}
</style>

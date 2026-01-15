<script lang="ts">
	/**
	 * Admin Course Editor Page
	 * Apple Principal Engineer ICT 7 Grade - January 2026
	 */

	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { adminFetch } from '$lib/utils/adminFetch';

	interface Module {
		id: number;
		title: string;
		description?: string;
		sort_order: number;
		is_published: boolean;
		lessons: Lesson[];
	}

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

	let courseId = $state('');

	onMount(() => {
		const pathParts = window.location.pathname.split('/');
		courseId = pathParts[pathParts.length - 1];
		fetchCourse();
	});
	let course = $state<Course | null>(null);
	let modules = $state<Module[]>([]);
	let unassignedLessons = $state<Lesson[]>([]);
	let downloads = $state<Download[]>([]);
	let loading = $state(true);
	let saving = $state(false);
	let uploading = $state(false);
	let fileInput = $state<HTMLInputElement | null>(null);
	let activeTab = $state<'details' | 'content' | 'downloads' | 'settings'>('details');

	const fetchCourse = async () => {
		loading = true;
		try {
			// ICT 7 FIX: Use adminFetch for absolute URL on Pages.dev
			const data = await adminFetch(`/api/admin/courses/${courseId}`);
			if (data.success) {
				course = data.data.course;
				modules = data.data.modules || [];
				unassignedLessons = data.data.unassigned_lessons || [];
				downloads = data.data.downloads || [];
			}
		} catch (e) {
			console.error('Failed to fetch course:', e);
		} finally {
			loading = false;
		}
	};

	const saveCourse = async () => {
		if (!course) return;
		saving = true;
		try {
			// ICT 7 FIX: Use adminFetch for absolute URL on Pages.dev
			const data = await adminFetch(`/api/admin/courses/${courseId}`, {
				method: 'PUT',
				body: JSON.stringify(course)
			});
			if (data.success) {
				course = data.data;
			} else {
				alert(data.error || 'Failed to save course');
			}
		} catch {
			alert('Failed to save course');
		} finally {
			saving = false;
		}
	};

	const publishCourse = async () => {
		if (!course) return;
		try {
			const endpoint = course.is_published ? 'unpublish' : 'publish';
			// ICT 7 FIX: Use adminFetch for absolute URL on Pages.dev
			const data = await adminFetch(`/api/admin/courses/${courseId}/${endpoint}`, { method: 'POST' });
			if (data.success) {
				course = data.data;
			}
		} catch {
			alert('Failed to update publish status');
		}
	};

	const addModule = async () => {
		const title = prompt('Enter module title:');
		if (!title) return;
		try {
			// ICT 7 FIX: Use adminFetch for absolute URL on Pages.dev
			const data = await adminFetch(`/api/admin/courses/${courseId}/modules`, {
				method: 'POST',
				body: JSON.stringify({ title })
			});
			if (data.success) {
				modules = [...modules, { ...data.data, lessons: [] }];
			}
		} catch {
			alert('Failed to create module');
		}
	};

	const deleteModule = async (moduleId: number) => {
		if (!confirm('Delete this module? Lessons will be unassigned.')) return;
		try {
			// ICT 7 FIX: Use adminFetch for absolute URL on Pages.dev
			await adminFetch(`/api/admin/courses/${courseId}/modules/${moduleId}`, { method: 'DELETE' });
			modules = modules.filter(m => m.id !== moduleId);
		} catch {
			alert('Failed to delete module');
		}
	};

	const addLesson = async (moduleId?: number) => {
		const title = prompt('Enter lesson title:');
		if (!title) return;
		try {
			// ICT 7 FIX: Use adminFetch for absolute URL on Pages.dev
			const data = await adminFetch(`/api/admin/courses/${courseId}/lessons`, {
				method: 'POST',
				body: JSON.stringify({ title, module_id: moduleId })
			});
			if (data.success) {
				if (moduleId) {
					const mod = modules.find(m => m.id === moduleId);
					if (mod) mod.lessons = [...mod.lessons, data.data];
					modules = [...modules];
				} else {
					unassignedLessons = [...unassignedLessons, data.data];
				}
			}
		} catch {
			alert('Failed to create lesson');
		}
	};

	// File upload handler
	const handleFileUpload = async (event: Event) => {
		const input = event.target as HTMLInputElement;
		const file = input.files?.[0];
		if (!file) return;

		uploading = true;
		try {
			// ICT 7 FIX: Use adminFetch for absolute URL on Pages.dev
			const urlData = await adminFetch(`/api/admin/courses/${courseId}/upload-url`, {
				method: 'POST',
				body: JSON.stringify({ file_name: file.name, content_type: file.type })
			});
			
			if (!urlData.success) {
				alert(urlData.error || 'Failed to get upload URL');
				return;
			}

			// Upload file to Bunny Storage
			const uploadRes = await fetch(urlData.data.upload_url, {
				method: 'PUT',
				headers: {
					'Content-Type': file.type,
					'AccessKey': urlData.data.access_key
				},
				body: file
			});

			if (!uploadRes.ok) {
				alert('Failed to upload file to storage');
				return;
			}

			// ICT 7 FIX: Use adminFetch for absolute URL on Pages.dev
			const createData = await adminFetch(`/api/admin/courses/${courseId}/downloads`, {
				method: 'POST',
				body: JSON.stringify({
					title: file.name.replace(/\.[^/.]+$/, '').replace(/[-_]/g, ' '),
					file_name: file.name,
					file_path: urlData.data.file_path,
					file_size_bytes: file.size,
					file_type: file.name.split('.').pop()?.toLowerCase(),
					mime_type: file.type,
					download_url: urlData.data.cdn_url,
					category: 'resource'
				})
			});
			if (createData.success) {
				downloads = [...downloads, createData.data];
			} else {
				alert(createData.error || 'Failed to create download record');
			}
		} catch (e) {
			console.error('Upload error:', e);
			alert('Failed to upload file');
		} finally {
			uploading = false;
			input.value = '';
		}
	};

	// Delete download handler
	const deleteDownload = async (downloadId: number) => {
		if (!confirm('Delete this download?')) return;
		try {
			// ICT 7 FIX: Use adminFetch for absolute URL on Pages.dev
			await adminFetch(`/api/admin/courses/${courseId}/downloads/${downloadId}`, { method: 'DELETE' });
			downloads = downloads.filter(d => d.id !== downloadId);
		} catch {
			alert('Failed to delete download');
		}
	};

	// Format file size
	const formatFileSize = (bytes?: number) => {
		if (!bytes) return '-';
		if (bytes < 1024) return `${bytes} B`;
		if (bytes < 1048576) return `${(bytes / 1024).toFixed(1)} KB`;
		return `${(bytes / 1048576).toFixed(1)} MB`;
	};

	const deleteLesson = async (lessonId: string, moduleId?: number) => {
		if (!confirm('Delete this lesson?')) return;
		try {
			// ICT 7 FIX: Use adminFetch for absolute URL on Pages.dev
			await adminFetch(`/api/admin/courses/${courseId}/lessons/${lessonId}`, { method: 'DELETE' });
			if (moduleId) {
				const mod = modules.find(m => m.id === moduleId);
				if (mod) mod.lessons = mod.lessons.filter(l => l.id !== lessonId);
				modules = [...modules];
			} else {
				unassignedLessons = unassignedLessons.filter(l => l.id !== lessonId);
			}
		} catch {
			alert('Failed to delete lesson');
		}
	};
</script>

<svelte:head>
	<title>{course?.title || 'Edit Course'} | Admin</title>
</svelte:head>

<div class="course-editor">
	{#if loading}
		<div class="loading">
			<div class="spinner"></div>
			<p>Loading course...</p>
		</div>
	{:else if course}
		<header class="editor-header">
			<div class="header-left">
				<a href="/admin/courses" class="back-link">
					<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="m15 18-6-6 6-6"/></svg>
					Back to Courses
				</a>
				<h1>{course.title}</h1>
				<span class="status" class:published={course.is_published}>
					{course.is_published ? 'Published' : 'Draft'}
				</span>
			</div>
			<div class="header-actions">
				<a href="/admin/page-builder?course={course.id}" class="btn-builder">
					<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect width="7" height="7" x="3" y="3" rx="1"/><rect width="7" height="7" x="14" y="3" rx="1"/><rect width="7" height="7" x="14" y="14" rx="1"/><rect width="7" height="7" x="3" y="14" rx="1"/></svg>
					Page Builder
				</a>
				<a href="/classes/{course.slug}" target="_blank" class="btn-secondary">
					<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"/><polyline points="15 3 21 3 21 9"/><line x1="10" x2="21" y1="14" y2="3"/></svg>
					Preview
				</a>
				<button class="btn-secondary" onclick={publishCourse}>
					{course.is_published ? 'Unpublish' : 'Publish'}
				</button>
				<button class="btn-primary" onclick={saveCourse} disabled={saving}>
					{saving ? 'Saving...' : 'Save Changes'}
				</button>
			</div>
		</header>

		<nav class="tabs">
			<button class:active={activeTab === 'details'} onclick={() => activeTab = 'details'}>Details</button>
			<button class:active={activeTab === 'content'} onclick={() => activeTab = 'content'}>Content</button>
			<button class:active={activeTab === 'downloads'} onclick={() => activeTab = 'downloads'}>Downloads</button>
			<button class:active={activeTab === 'settings'} onclick={() => activeTab = 'settings'}>Settings</button>
		</nav>

		<div class="tab-content">
			{#if activeTab === 'details'}
				<div class="form-section">
					<h2>Basic Information</h2>
					<div class="form-grid">
						<div class="form-group full">
							<label for="title">Title</label>
							<input id="title" type="text" bind:value={course.title} />
						</div>
						<div class="form-group full">
							<label for="slug">Slug</label>
							<input id="slug" type="text" bind:value={course.slug} />
						</div>
						<div class="form-group full">
							<label for="description">Description</label>
							<textarea id="description" rows="4" bind:value={course.description}></textarea>
						</div>
						<div class="form-group full">
							<label for="card_description">Card Description (short)</label>
							<input id="card_description" type="text" bind:value={course.card_description} />
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
							<input id="price" type="number" bind:value={course.price_cents} />
						</div>
						<div class="form-group">
							<label class="checkbox-label">
								<input type="checkbox" bind:checked={course.is_free} />
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
							<input id="instructor_name" type="text" bind:value={course.instructor_name} />
						</div>
						<div class="form-group">
							<label for="instructor_title">Title</label>
							<input id="instructor_title" type="text" bind:value={course.instructor_title} />
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
							<input id="card_image" type="text" bind:value={course.card_image_url} />
						</div>
						<div class="form-group">
							<label for="badge">Badge Text</label>
							<input id="badge" type="text" bind:value={course.card_badge} placeholder="e.g., NEW" />
						</div>
						<div class="form-group">
							<label for="badge_color">Badge Color</label>
							<input id="badge_color" type="color" bind:value={course.card_badge_color} />
						</div>
					</div>
				</div>
			{:else if activeTab === 'content'}
				<div class="content-section">
					<div class="content-header">
						<h2>Modules & Lessons</h2>
						<button class="btn-secondary" onclick={addModule}>
							<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="12" x2="12" y1="5" y2="19"/><line x1="5" x2="19" y1="12" y2="12"/></svg>
							Add Module
						</button>
					</div>

					{#each modules as mod}
						<div class="module-card">
							<div class="module-header">
								<h3>{mod.title}</h3>
								<div class="module-actions">
									<button onclick={() => addLesson(mod.id)} aria-label="Add lesson">
										<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="12" x2="12" y1="5" y2="19"/><line x1="5" x2="19" y1="12" y2="12"/></svg>
									</button>
									<button onclick={() => deleteModule(mod.id)} aria-label="Delete module">
										<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M3 6h18"/><path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6"/></svg>
									</button>
								</div>
							</div>
							<ul class="lesson-list">
								{#each mod.lessons as lesson}
									<li>
										<a href="/admin/courses/{courseId}/lessons/{lesson.id}">
											<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polygon points="5 3 19 12 5 21 5 3"/></svg>
											<span>{lesson.title}</span>
											{#if lesson.duration_minutes}
												<span class="duration">{lesson.duration_minutes}m</span>
											{/if}
										</a>
										<button onclick={() => deleteLesson(lesson.id, mod.id)} aria-label="Delete lesson">
											<svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="18" x2="6" y1="6" y2="18"/><line x1="6" x2="18" y1="6" y2="18"/></svg>
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
								{#each unassignedLessons as lesson}
									<li>
										<a href="/admin/courses/{courseId}/lessons/{lesson.id}">
											<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polygon points="5 3 19 12 5 21 5 3"/></svg>
											<span>{lesson.title}</span>
										</a>
										<button onclick={() => deleteLesson(lesson.id)} aria-label="Delete lesson">
											<svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="18" x2="6" y1="6" y2="18"/><line x1="6" x2="18" y1="6" y2="18"/></svg>
										</button>
									</li>
								{/each}
							</ul>
						</div>
					{/if}

					<button class="btn-add-lesson" onclick={() => addLesson()}>
						<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="12" x2="12" y1="5" y2="19"/><line x1="5" x2="19" y1="12" y2="12"/></svg>
						Add Lesson (No Module)
					</button>
				</div>
			{:else if activeTab === 'downloads'}
				<div class="downloads-section">
					<div class="content-header">
						<h2>Course Downloads</h2>
						<input type="file" bind:this={fileInput} onchange={handleFileUpload} style="display: none;" />
						<button class="btn-secondary" onclick={() => fileInput?.click()} disabled={uploading}>
							{#if uploading}
								<span class="spinner-small"></span>
								Uploading...
							{:else}
								<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="17 8 12 3 7 8"/><line x1="12" x2="12" y1="3" y2="15"/></svg>
								Upload File
							{/if}
						</button>
					</div>

					{#if downloads.length === 0}
						<div class="empty-downloads">
							<svg xmlns="http://www.w3.org/2000/svg" width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="7 10 12 15 17 10"/><line x1="12" x2="12" y1="15" y2="3"/></svg>
							<p>No downloads yet. Upload files to make them available to enrolled students.</p>
						</div>
					{:else}
						<ul class="downloads-list">
							{#each downloads as dl}
								<li>
									<span class="dl-title">{dl.title}</span>
									<span class="dl-meta">
										<span class="dl-file">{dl.file_name}</span>
										<span class="dl-size">{formatFileSize(dl.file_size_bytes)}</span>
									</span>
									<button onclick={() => deleteDownload(dl.id)} aria-label="Delete download">
										<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M3 6h18"/><path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6"/></svg>
									</button>
								</li>
							{/each}
						</ul>
					{/if}
				</div>
			{:else if activeTab === 'settings'}
				<div class="form-section">
					<h2>SEO Settings</h2>
					<div class="form-grid">
						<div class="form-group full">
							<label for="meta_title">Meta Title</label>
							<input id="meta_title" type="text" bind:value={course.meta_title} />
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
							<input id="bunny_lib" type="number" bind:value={course.bunny_library_id} />
						</div>
					</div>
				</div>
			{/if}
		</div>
	{/if}
</div>

<style>
	.course-editor { padding: 24px; max-width: 1200px; margin: 0 auto; }

	.loading { text-align: center; padding: 64px; }
	.spinner { width: 40px; height: 40px; border: 3px solid #e5e7eb; border-top-color: #143e59; border-radius: 50%; animation: spin 1s linear infinite; margin: 0 auto 16px; }
	@keyframes spin { to { transform: rotate(360deg); } }

	.editor-header { display: flex; justify-content: space-between; align-items: flex-start; margin-bottom: 24px; padding-bottom: 24px; border-bottom: 1px solid #e5e7eb; }
	.header-left { display: flex; flex-direction: column; gap: 8px; }
	.back-link { display: inline-flex; align-items: center; gap: 4px; color: #6b7280; text-decoration: none; font-size: 14px; }
	.back-link:hover { color: #143e59; }
	.header-left h1 { font-size: 24px; margin: 0; color: #1f2937; }
	.status { display: inline-block; padding: 4px 10px; background: #fef3c7; color: #92400e; border-radius: 4px; font-size: 12px; font-weight: 500; }
	.status.published { background: #d1fae5; color: #065f46; }

	.header-actions { display: flex; gap: 8px; }
	.btn-primary, .btn-secondary { display: inline-flex; align-items: center; gap: 6px; padding: 10px 16px; border-radius: 8px; font-size: 14px; font-weight: 500; cursor: pointer; transition: all 0.2s; text-decoration: none; border: none; }
	.btn-primary { background: #143e59; color: #fff; }
	.btn-primary:hover { background: #0f2d42; }
	.btn-primary:disabled { opacity: 0.6; cursor: not-allowed; }
	.btn-secondary { background: #f3f4f6; color: #1f2937; }
	.btn-secondary:hover { background: #e5e7eb; }
	.btn-builder { background: linear-gradient(135deg, #143E59 0%, #1E73BE 100%); color: #fff; }
	.btn-builder:hover { background: linear-gradient(135deg, #0F2D42 0%, #143E59 100%); transform: translateY(-1px); box-shadow: 0 4px 12px rgba(20, 62, 89, 0.3); }

	.tabs { display: flex; gap: 4px; margin-bottom: 24px; border-bottom: 1px solid #e5e7eb; }
	.tabs button { padding: 12px 20px; background: none; border: none; color: #6b7280; font-size: 14px; font-weight: 500; cursor: pointer; border-bottom: 2px solid transparent; margin-bottom: -1px; }
	.tabs button:hover { color: #1f2937; }
	.tabs button.active { color: #143e59; border-bottom-color: #143e59; }

	.form-section { background: #fff; border: 1px solid #e5e7eb; border-radius: 12px; padding: 24px; margin-bottom: 24px; }
	.form-section h2 { font-size: 16px; font-weight: 600; color: #1f2937; margin: 0 0 20px; }
	.form-grid { display: grid; grid-template-columns: repeat(2, 1fr); gap: 16px; }
	.form-group { display: flex; flex-direction: column; gap: 6px; }
	.form-group.full { grid-column: 1 / -1; }
	.form-group label { font-size: 13px; font-weight: 500; color: #374151; }
	.form-group input, .form-group select, .form-group textarea { padding: 10px 12px; border: 1px solid #e5e7eb; border-radius: 6px; font-size: 14px; }
	.form-group input:focus, .form-group select:focus, .form-group textarea:focus { outline: none; border-color: #143e59; }
	.checkbox-label { display: flex; align-items: center; gap: 8px; cursor: pointer; }
	.checkbox-label input { width: 16px; height: 16px; }

	.content-section, .downloads-section { background: #fff; border: 1px solid #e5e7eb; border-radius: 12px; padding: 24px; }
	.content-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px; }
	.content-header h2 { font-size: 16px; font-weight: 600; color: #1f2937; margin: 0; }

	.module-card { background: #f9fafb; border-radius: 8px; margin-bottom: 16px; overflow: hidden; }
	.module-card.unassigned { background: #fef3c7; }
	.module-header { display: flex; justify-content: space-between; align-items: center; padding: 12px 16px; background: #e5e7eb; }
	.module-card.unassigned .module-header { background: #fcd34d; }
	.module-header h3 { font-size: 14px; font-weight: 600; margin: 0; }
	.module-actions { display: flex; gap: 8px; }
	.module-actions button { background: none; border: none; color: #6b7280; cursor: pointer; padding: 4px; }
	.module-actions button:hover { color: #1f2937; }

	.lesson-list { list-style: none; margin: 0; padding: 0; }
	.lesson-list li { display: flex; align-items: center; padding: 12px 16px; border-bottom: 1px solid #e5e7eb; }
	.lesson-list li:last-child { border-bottom: none; }
	.lesson-list li.empty { color: #9ca3af; font-style: italic; font-size: 13px; }
	.lesson-list li a { display: flex; align-items: center; gap: 10px; flex: 1; text-decoration: none; color: #1f2937; font-size: 14px; }
	.lesson-list li a:hover { color: #143e59; }
	.lesson-list .duration { color: #9ca3af; font-size: 12px; margin-left: auto; }
	.lesson-list li > button { background: none; border: none; color: #9ca3af; cursor: pointer; padding: 4px; }
	.lesson-list li > button:hover { color: #dc2626; }

	.btn-add-lesson { display: flex; align-items: center; justify-content: center; gap: 8px; width: 100%; padding: 12px; background: #f3f4f6; border: 2px dashed #d1d5db; border-radius: 8px; color: #6b7280; font-size: 14px; cursor: pointer; margin-top: 16px; }
	.btn-add-lesson:hover { background: #e5e7eb; border-color: #9ca3af; }

	.empty-downloads { text-align: center; padding: 48px; color: #6b7280; }
	.empty-downloads svg { margin-bottom: 16px; opacity: 0.5; }
	.downloads-list { list-style: none; margin: 0; padding: 0; }
	.downloads-list li { display: flex; align-items: center; gap: 12px; padding: 12px; background: #f9fafb; border-radius: 6px; margin-bottom: 8px; }
	.dl-title { font-weight: 500; flex: 1; }
	.dl-meta { display: flex; gap: 16px; align-items: center; }
	.dl-file { color: #6b7280; font-size: 13px; }
	.dl-size { color: #9ca3af; font-size: 12px; }
	.downloads-list button { background: none; border: none; color: #9ca3af; cursor: pointer; }
	.downloads-list button:hover { color: #dc2626; }
	.spinner-small { width: 16px; height: 16px; border: 2px solid #e5e7eb; border-top-color: #143e59; border-radius: 50%; animation: spin 0.8s linear infinite; display: inline-block; }
	@keyframes spin { to { transform: rotate(360deg); } }

	@media (max-width: 768px) {
		.editor-header { flex-direction: column; gap: 16px; }
		.header-actions { width: 100%; justify-content: flex-end; }
		.form-grid { grid-template-columns: 1fr; }
		.tabs { overflow-x: auto; }
	}
</style>

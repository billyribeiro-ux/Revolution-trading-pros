<script lang="ts">
	/**
	 * Admin Lesson Editor Page
	 * Apple Principal Engineer ICT 7 Grade - January 2026
	 */

	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';

	interface Lesson {
		id: string;
		course_id: string;
		module_id?: number;
		title: string;
		slug: string;
		description?: string;
		video_url?: string;
		bunny_video_guid?: string;
		thumbnail_url?: string;
		duration_minutes?: number;
		content_html?: string;
		is_free: boolean;
		is_preview?: boolean;
		is_published?: boolean;
		sort_order?: number;
		drip_days?: number;
	}

	interface Download {
		id: number;
		title: string;
		file_name: string;
		download_url?: string;
	}

	interface Module {
		id: number;
		title: string;
	}

	let courseId = $state('');
	let lessonId = $state('');

	onMount(() => {
		const pathParts = window.location.pathname.split('/');
		// URL: /admin/courses/[id]/lessons/[lessonId]
		lessonId = pathParts[pathParts.length - 1];
		courseId = pathParts[pathParts.length - 3];
		fetchLesson();
	});
	let lesson = $state<Lesson | null>(null);
	let downloads = $state<Download[]>([]);
	let modules = $state<Module[]>([]);
	let loading = $state(true);
	let saving = $state(false);
	let uploading = $state(false);

	const fetchLesson = async () => {
		loading = true;
		try {
			const [lessonRes, modulesRes] = await Promise.all([
				fetch(`/api/admin/courses/${courseId}/lessons/${lessonId}`),
				fetch(`/api/admin/courses/${courseId}/modules`)
			]);
			
			const lessonData = await lessonRes.json();
			const modulesData = await modulesRes.json();

			if (lessonData.success) {
				lesson = lessonData.data.lesson;
				downloads = lessonData.data.downloads || [];
			}
			if (modulesData.success) {
				modules = modulesData.data || [];
			}
		} catch (e) {
			console.error('Failed to fetch lesson:', e);
		} finally {
			loading = false;
		}
	};

	const saveLesson = async () => {
		if (!lesson) return;
		saving = true;
		try {
			const res = await fetch(`/api/admin/courses/${courseId}/lessons/${lessonId}`, {
				method: 'PUT',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify(lesson)
			});
			const data = await res.json();
			if (data.success) {
				lesson = data.data;
			} else {
				alert(data.error || 'Failed to save lesson');
			}
		} catch {
			alert('Failed to save lesson');
		} finally {
			saving = false;
		}
	};

	const deleteLesson = async () => {
		if (!confirm('Are you sure you want to delete this lesson?')) return;
		try {
			await fetch(`/api/admin/courses/${courseId}/lessons/${lessonId}`, { method: 'DELETE' });
			goto(`/admin/courses/${courseId}`);
		} catch {
			alert('Failed to delete lesson');
		}
	};

	const handleVideoUpload = async (event: Event) => {
		const input = event.target as HTMLInputElement;
		const file = input.files?.[0];
		if (!file) return;

		uploading = true;
		try {
			// Get upload URL from Bunny
			const urlRes = await fetch(`/api/admin/unified-videos/upload-url`, {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ title: lesson?.title || file.name })
			});
			const urlData = await urlRes.json();

			if (urlData.success) {
				// Update lesson with video GUID
				if (lesson) {
					lesson.bunny_video_guid = urlData.data.video_guid;
					lesson.video_url = urlData.data.play_url;
				}
				alert('Video created. Upload the file using the TUS URL provided.');
			}
		} catch (e) {
			alert('Failed to create video upload');
		} finally {
			uploading = false;
		}
	};
</script>

<svelte:head>
	<title>{lesson?.title || 'Edit Lesson'} | Admin</title>
</svelte:head>

<div class="lesson-editor">
	{#if loading}
		<div class="loading">
			<div class="spinner"></div>
			<p>Loading lesson...</p>
		</div>
	{:else if lesson}
		<header class="editor-header">
			<div class="header-left">
				<a href="/admin/courses/{courseId}" class="back-link">
					<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="m15 18-6-6 6-6"/></svg>
					Back to Course
				</a>
				<h1>{lesson.title}</h1>
			</div>
			<div class="header-actions">
				<button class="btn-danger" onclick={deleteLesson}>Delete</button>
				<button class="btn-primary" onclick={saveLesson} disabled={saving}>
					{saving ? 'Saving...' : 'Save Lesson'}
				</button>
			</div>
		</header>

		<div class="editor-content">
			<div class="main-panel">
				<section class="form-section">
					<h2>Lesson Details</h2>
					<div class="form-grid">
						<div class="form-group full">
							<label for="title">Title</label>
							<input id="title" type="text" bind:value={lesson.title} />
						</div>
						<div class="form-group full">
							<label for="slug">Slug</label>
							<input id="slug" type="text" bind:value={lesson.slug} />
						</div>
						<div class="form-group full">
							<label for="description">Description</label>
							<textarea id="description" rows="3" bind:value={lesson.description}></textarea>
						</div>
						<div class="form-group">
							<label for="module">Module</label>
							<select id="module" bind:value={lesson.module_id}>
								<option value={null}>No Module</option>
								{#each modules as mod}
									<option value={mod.id}>{mod.title}</option>
								{/each}
							</select>
						</div>
						<div class="form-group">
							<label for="duration">Duration (minutes)</label>
							<input id="duration" type="number" bind:value={lesson.duration_minutes} />
						</div>
						<div class="form-group">
							<label for="sort">Sort Order</label>
							<input id="sort" type="number" bind:value={lesson.sort_order} />
						</div>
						<div class="form-group">
							<label for="drip">Drip Days</label>
							<input id="drip" type="number" bind:value={lesson.drip_days} placeholder="0 = immediate" />
						</div>
					</div>
				</section>

				<section class="form-section">
					<h2>Video</h2>
					<div class="video-section">
						{#if lesson.bunny_video_guid}
							<div class="video-preview">
								<iframe
									src="https://iframe.mediadelivery.net/embed/{lesson.bunny_video_guid}"
									title={lesson.title}
									allow="accelerometer; autoplay; encrypted-media; gyroscope; picture-in-picture"
									allowfullscreen
								></iframe>
							</div>
							<div class="video-info">
								<p><strong>Video GUID:</strong> {lesson.bunny_video_guid}</p>
								<button class="btn-secondary" onclick={() => { if (lesson) lesson.bunny_video_guid = undefined; }}>
									Remove Video
								</button>
							</div>
						{:else}
							<div class="upload-area">
								<svg xmlns="http://www.w3.org/2000/svg" width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="m16 13 5.223 3.482a.5.5 0 0 0 .777-.416V7.87a.5.5 0 0 0-.752-.432L16 10.5"/><rect x="2" y="6" width="14" height="12" rx="2"/></svg>
								<p>Upload a video or enter a Bunny.net Video GUID</p>
								<input
									type="file"
									accept="video/*"
									onchange={handleVideoUpload}
									disabled={uploading}
								/>
								<span class="divider">or</span>
								<input
									type="text"
									placeholder="Enter Bunny Video GUID"
									bind:value={lesson.bunny_video_guid}
								/>
							</div>
						{/if}
					</div>
				</section>

				<section class="form-section">
					<h2>Content</h2>
					<div class="form-group full">
						<label for="content">Lesson Content (HTML)</label>
						<textarea id="content" rows="10" bind:value={lesson.content_html} class="code-input"></textarea>
					</div>
				</section>
			</div>

			<aside class="side-panel">
				<section class="panel-section">
					<h3>Access Settings</h3>
					<label class="toggle">
						<input type="checkbox" bind:checked={lesson.is_published} />
						<span>Published</span>
					</label>
					<label class="toggle">
						<input type="checkbox" bind:checked={lesson.is_free} />
						<span>Free Preview</span>
					</label>
					<label class="toggle">
						<input type="checkbox" bind:checked={lesson.is_preview} />
						<span>Show in Preview</span>
					</label>
				</section>

				<section class="panel-section">
					<h3>Thumbnail</h3>
					<div class="thumbnail-preview">
						{#if lesson.thumbnail_url}
							<img src={lesson.thumbnail_url} alt="Thumbnail" />
						{:else}
							<div class="no-thumb">No thumbnail</div>
						{/if}
					</div>
					<input type="text" placeholder="Thumbnail URL" bind:value={lesson.thumbnail_url} />
				</section>

				<section class="panel-section">
					<h3>Downloads ({downloads.length})</h3>
					{#if downloads.length === 0}
						<p class="empty-text">No downloads attached</p>
					{:else}
						<ul class="downloads-list">
							{#each downloads as dl}
								<li>
									<span>{dl.title}</span>
								</li>
							{/each}
						</ul>
					{/if}
					<button class="btn-add">+ Add Download</button>
				</section>
			</aside>
		</div>
	{/if}
</div>

<style>
	.lesson-editor { padding: 24px; max-width: 1400px; margin: 0 auto; }

	.loading { text-align: center; padding: 64px; }
	.spinner { width: 40px; height: 40px; border: 3px solid #e5e7eb; border-top-color: #143e59; border-radius: 50%; animation: spin 1s linear infinite; margin: 0 auto 16px; }
	@keyframes spin { to { transform: rotate(360deg); } }

	.editor-header { display: flex; justify-content: space-between; align-items: flex-start; margin-bottom: 24px; padding-bottom: 24px; border-bottom: 1px solid #e5e7eb; }
	.header-left { display: flex; flex-direction: column; gap: 8px; }
	.back-link { display: inline-flex; align-items: center; gap: 4px; color: #6b7280; text-decoration: none; font-size: 14px; }
	.back-link:hover { color: #143e59; }
	.header-left h1 { font-size: 24px; margin: 0; color: #1f2937; }

	.header-actions { display: flex; gap: 8px; }
	.btn-primary, .btn-secondary, .btn-danger { padding: 10px 16px; border-radius: 8px; font-size: 14px; font-weight: 500; cursor: pointer; transition: all 0.2s; border: none; }
	.btn-primary { background: #143e59; color: #fff; }
	.btn-primary:hover { background: #0f2d42; }
	.btn-primary:disabled { opacity: 0.6; cursor: not-allowed; }
	.btn-secondary { background: #f3f4f6; color: #1f2937; }
	.btn-secondary:hover { background: #e5e7eb; }
	.btn-danger { background: #fee2e2; color: #dc2626; }
	.btn-danger:hover { background: #fecaca; }

	.editor-content { display: grid; grid-template-columns: 1fr 320px; gap: 24px; }

	.main-panel { display: flex; flex-direction: column; gap: 24px; }
	.form-section { background: #fff; border: 1px solid #e5e7eb; border-radius: 12px; padding: 24px; }
	.form-section h2 { font-size: 16px; font-weight: 600; color: #1f2937; margin: 0 0 20px; }

	.form-grid { display: grid; grid-template-columns: repeat(2, 1fr); gap: 16px; }
	.form-group { display: flex; flex-direction: column; gap: 6px; }
	.form-group.full { grid-column: 1 / -1; }
	.form-group label { font-size: 13px; font-weight: 500; color: #374151; }
	.form-group input, .form-group select, .form-group textarea { padding: 10px 12px; border: 1px solid #e5e7eb; border-radius: 6px; font-size: 14px; }
	.form-group input:focus, .form-group select:focus, .form-group textarea:focus { outline: none; border-color: #143e59; }
	.code-input { font-family: monospace; font-size: 13px; }

	.video-section { display: flex; flex-direction: column; gap: 16px; }
	.video-preview { aspect-ratio: 16/9; background: #000; border-radius: 8px; overflow: hidden; }
	.video-preview iframe { width: 100%; height: 100%; border: none; }
	.video-info { display: flex; align-items: center; justify-content: space-between; padding: 12px; background: #f9fafb; border-radius: 6px; }
	.video-info p { margin: 0; font-size: 13px; color: #6b7280; }

	.upload-area { display: flex; flex-direction: column; align-items: center; gap: 12px; padding: 32px; background: #f9fafb; border: 2px dashed #d1d5db; border-radius: 8px; text-align: center; }
	.upload-area svg { color: #9ca3af; }
	.upload-area p { color: #6b7280; margin: 0; }
	.upload-area input[type="file"] { cursor: pointer; }
	.upload-area input[type="text"] { width: 100%; max-width: 300px; }
	.divider { color: #9ca3af; font-size: 12px; }

	.side-panel { display: flex; flex-direction: column; gap: 16px; }
	.panel-section { background: #fff; border: 1px solid #e5e7eb; border-radius: 12px; padding: 16px; }
	.panel-section h3 { font-size: 14px; font-weight: 600; color: #1f2937; margin: 0 0 12px; }

	.toggle { display: flex; align-items: center; gap: 8px; padding: 8px 0; cursor: pointer; font-size: 14px; }
	.toggle input { width: 18px; height: 18px; }

	.thumbnail-preview { aspect-ratio: 16/9; background: #f3f4f6; border-radius: 6px; overflow: hidden; margin-bottom: 8px; }
	.thumbnail-preview img { width: 100%; height: 100%; object-fit: cover; }
	.no-thumb { display: flex; align-items: center; justify-content: center; height: 100%; color: #9ca3af; font-size: 13px; }

	.empty-text { color: #9ca3af; font-size: 13px; margin: 0; }
	.downloads-list { list-style: none; margin: 0 0 12px; padding: 0; }
	.downloads-list li { padding: 8px; background: #f9fafb; border-radius: 4px; margin-bottom: 4px; font-size: 13px; }
	.btn-add { width: 100%; padding: 8px; background: #f3f4f6; border: 1px dashed #d1d5db; border-radius: 6px; color: #6b7280; font-size: 13px; cursor: pointer; }
	.btn-add:hover { background: #e5e7eb; }

	@media (max-width: 1024px) {
		.editor-content { grid-template-columns: 1fr; }
		.side-panel { order: -1; }
	}

	@media (max-width: 768px) {
		.editor-header { flex-direction: column; gap: 16px; }
		.form-grid { grid-template-columns: 1fr; }
	}
</style>

<script lang="ts">
	/**
	 * Admin Course Editor Page
	 * Apple Principal Engineer ICT 7 Grade - January 2026
	 *
	 * R19-C extraction (2026-05-20): split 1264 LOC tabbed-detail page into
	 * 6 leaf components under `_components/`. Parent is now the orchestrator
	 * only: state, fetch, mutation handlers, ConfirmationModal wiring.
	 * Mirrors R18-C indicators/[id] pattern exactly.
	 */

	import { onMount } from 'svelte';
	// FIX-2026-04-26 (P1-5): use SvelteKit router state for params instead of
	// parsing window.location.pathname (which doesn't react to client-side nav).
	import { page } from '$app/state';
	import { beforeNavigate } from '$app/navigation';
	import { adminFetch } from '$lib/utils/adminFetch';
	import { logger } from '$lib/utils/logger';
	import ConfirmationModal from '$lib/components/admin/ConfirmationModal.svelte';
	// FIX-2026-04-26: replaced native alert() calls with toastStore for non-blocking UX.
	import { toastStore } from '$lib/stores/toast.svelte';
	import CourseHeader from './_components/CourseHeader.svelte';
	import CourseTabs from './_components/CourseTabs.svelte';
	import CourseDetailsTab from './_components/CourseDetailsTab.svelte';
	import CourseContentTab from './_components/CourseContentTab.svelte';
	import CourseDownloadsTab from './_components/CourseDownloadsTab.svelte';
	import CourseSettingsTab from './_components/CourseSettingsTab.svelte';

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

	// FIX-2026-04-26 (P1-5): derive from page.params (reactive) rather than
	// parsing window.location.pathname (mount-only).
	let courseId = $derived(page.params.id ?? '');

	let course = $state<Course | null>(null);
	let modules = $state<Module[]>([]);
	let unassignedLessons = $state<Lesson[]>([]);
	let downloads = $state<Download[]>([]);
	let loading = $state(true);
	let saving = $state(false);
	let uploading = $state(false);
	let activeTab = $state<'details' | 'content' | 'downloads' | 'settings'>('details');

	// Delete confirmation modal state
	let showDeleteModuleModal = $state(false);
	let showDeleteDownloadModal = $state(false);
	let showDeleteLessonModal = $state(false);
	let pendingDeleteModuleId = $state<number | null>(null);
	let pendingDeleteDownloadId = $state<number | null>(null);
	let pendingDeleteLesson = $state<{ id: string; moduleId?: number } | null>(null);

	// FIX-2026-04-26 (P0-3): replace native prompt() flows with ConfirmationModal
	// (input variant). prompt() is blocked in modern browser embed contexts and
	// has zero validation; this surfaces a real modal with trim/length checks.
	let showAddModuleModal = $state(false);
	let addModuleTitle = $state('');
	let showAddLessonModal = $state(false);
	let addLessonTitle = $state('');
	let pendingAddLessonModuleId = $state<number | undefined>(undefined);

	// FIX-2026-04-26 (P1-6): unsaved-changes guard. Snapshot course on load /
	// after each save; compare against current state to know if dirty.
	let lastSavedSnapshot = $state<string>('');
	let hasUnsavedChanges = $derived(course ? JSON.stringify(course) !== lastSavedSnapshot : false);

	beforeNavigate((nav) => {
		// TODO(modal-confirm): SvelteKit's beforeNavigate is synchronous —
		// nav.cancel() must be called inline, so native confirm() is the only
		// portable option today.
		if (
			hasUnsavedChanges &&
			!nav.willUnload &&
			!confirm('You have unsaved changes. Discard and leave?')
		) {
			nav.cancel();
		}
	});

	onMount(() => {
		fetchCourse();
		const handler = (e: BeforeUnloadEvent) => {
			if (hasUnsavedChanges) {
				e.preventDefault();
			}
		};
		window.addEventListener('beforeunload', handler);
		return () => window.removeEventListener('beforeunload', handler);
	});

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
				// FIX-2026-04-26 (P1-6): snapshot for dirty tracking.
				lastSavedSnapshot = JSON.stringify(course);
			}
		} catch (e) {
			logger.error('Failed to fetch course:', e);
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
				// FIX-2026-04-26 (P2-10): merge instead of overwrite. The publish
				// endpoint sometimes returns just the course header, blanking the
				// rich state (modules/downloads) built up by fetchCourse.
				const returned = data.data?.course ?? data.data;
				if (returned && typeof returned === 'object') {
					course = { ...course, ...returned };
				}
				lastSavedSnapshot = JSON.stringify(course);
				toastStore.success('Course saved');
			} else {
				// FIX-2026-04-26: replaced native alert() with toastStore.error.
				// Old: alert(data.error || 'Failed to save course');
				toastStore.error(data.error || 'Failed to save course');
			}
		} catch {
			// FIX-2026-04-26: replaced native alert() with toastStore.error.
			// Old: alert('Failed to save course');
			toastStore.error('Failed to save course');
		} finally {
			saving = false;
		}
	};

	const publishCourse = async () => {
		if (!course) return;
		try {
			const endpoint = course.is_published ? 'unpublish' : 'publish';
			// ICT 7 FIX: Use adminFetch for absolute URL on Pages.dev
			const data = await adminFetch(`/api/admin/courses/${courseId}/${endpoint}`, {
				method: 'POST'
			});
			if (data.success) {
				// FIX-2026-04-26 (P2-10): merge response into course rather than
				// overwriting whole rune (which blanked sub-tabs). Refetch full
				// tree to be safe so modules/downloads stay populated.
				const returned = data.data?.course ?? data.data;
				if (returned && typeof returned === 'object') {
					course = { ...course, ...returned };
				}
				lastSavedSnapshot = JSON.stringify(course);
				await fetchCourse();
			} else {
				// FIX-2026-04-26 (P3-7): show error when publish fails non-throwing.
				toastStore.error(data.error || data.message || 'Failed to update publish status');
			}
		} catch {
			// FIX-2026-04-26: replaced native alert() with toastStore.error.
			// Old: alert('Failed to update publish status');
			toastStore.error('Failed to update publish status');
		}
	};

	// FIX-2026-04-26 (P0-3): open input modal instead of native prompt().
	// Old: const title = prompt('Enter module title:');
	const addModule = () => {
		addModuleTitle = '';
		showAddModuleModal = true;
	};

	const confirmAddModule = async (value?: string) => {
		const title = (value ?? '').trim();
		if (!title) {
			toastStore.error('Module title is required');
			return;
		}
		showAddModuleModal = false;
		try {
			const data = await adminFetch(`/api/admin/courses/${courseId}/modules`, {
				method: 'POST',
				body: JSON.stringify({ title })
			});
			// FIX-2026-04-26 (P0-2): tolerate envelope variations.
			const created = data?.data ?? data;
			if (created && (data.success === undefined || data.success)) {
				modules = [...modules, { ...created, lessons: [] }];
			} else {
				toastStore.error(data?.error || data?.message || 'Failed to create module');
			}
		} catch {
			toastStore.error('Failed to create module');
		}
	};

	const deleteModule = (moduleId: number) => {
		pendingDeleteModuleId = moduleId;
		showDeleteModuleModal = true;
	};

	const confirmDeleteModule = async () => {
		if (pendingDeleteModuleId === null) return;
		showDeleteModuleModal = false;
		const moduleId = pendingDeleteModuleId;
		pendingDeleteModuleId = null;
		try {
			// ICT 7 FIX: Use adminFetch for absolute URL on Pages.dev
			await adminFetch(`/api/admin/courses/${courseId}/modules/${moduleId}`, { method: 'DELETE' });
			modules = modules.filter((m) => m.id !== moduleId);
		} catch {
			logger.error('Failed to delete module');
		}
	};

	// FIX-2026-04-26 (P0-3): open input modal instead of native prompt() —
	// prompt() is blocked in modern browser embed contexts and accepts
	// empty/garbage input with no validation.
	const addLesson = (moduleId?: number) => {
		pendingAddLessonModuleId = moduleId;
		addLessonTitle = '';
		showAddLessonModal = true;
	};

	const confirmAddLesson = async (value?: string) => {
		const title = (value ?? '').trim();
		if (!title) {
			toastStore.error('Lesson title is required');
			return;
		}
		const moduleId = pendingAddLessonModuleId;
		showAddLessonModal = false;
		pendingAddLessonModuleId = undefined;
		try {
			// ICT 7 FIX: Use adminFetch for absolute URL on Pages.dev
			const data = await adminFetch(`/api/admin/courses/${courseId}/lessons`, {
				method: 'POST',
				body: JSON.stringify({ title, module_id: moduleId })
			});
			// FIX-2026-04-26 (P0-2): tolerate envelope variations.
			const created = data?.data ?? data;
			if (created && (data.success === undefined || data.success)) {
				if (moduleId) {
					const mod = modules.find((m) => m.id === moduleId);
					if (mod) mod.lessons = [...mod.lessons, created];
					modules = [...modules];
				} else {
					unassignedLessons = [...unassignedLessons, created];
				}
			} else {
				toastStore.error(data?.error || data?.message || 'Failed to create lesson');
			}
		} catch {
			// FIX-2026-04-26: replaced native alert() with toastStore.error.
			// Old: alert('Failed to create lesson');
			toastStore.error('Failed to create lesson');
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
				// FIX-2026-04-26: replaced native alert() with toastStore.error.
				// Old: alert(urlData.error || 'Failed to get upload URL');
				toastStore.error(urlData.error || 'Failed to get upload URL');
				return;
			}

			// Upload file to Bunny Storage
			const uploadRes = await fetch(urlData.data.upload_url, {
				method: 'PUT',
				headers: {
					'Content-Type': file.type,
					AccessKey: urlData.data.access_key
				},
				body: file
			});

			if (!uploadRes.ok) {
				// FIX-2026-04-26: replaced native alert() with toastStore.error.
				// Old: alert('Failed to upload file to storage');
				toastStore.error('Failed to upload file to storage');
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
				// FIX-2026-04-26: replaced native alert() with toastStore.error.
				// Old: alert(createData.error || 'Failed to create download record');
				toastStore.error(createData.error || 'Failed to create download record');
			}
		} catch (e) {
			logger.error('Upload error:', e);
			// FIX-2026-04-26: replaced native alert() with toastStore.error.
			// Old: alert('Failed to upload file');
			toastStore.error('Failed to upload file');
		} finally {
			uploading = false;
			input.value = '';
		}
	};

	// Delete download handler
	const deleteDownload = (downloadId: number) => {
		pendingDeleteDownloadId = downloadId;
		showDeleteDownloadModal = true;
	};

	const confirmDeleteDownload = async () => {
		if (pendingDeleteDownloadId === null) return;
		showDeleteDownloadModal = false;
		const downloadId = pendingDeleteDownloadId;
		pendingDeleteDownloadId = null;
		try {
			// ICT 7 FIX: Use adminFetch for absolute URL on Pages.dev
			await adminFetch(`/api/admin/courses/${courseId}/downloads/${downloadId}`, {
				method: 'DELETE'
			});
			downloads = downloads.filter((d) => d.id !== downloadId);
		} catch {
			logger.error('Failed to delete download');
		}
	};

	const deleteLesson = (lessonId: string, moduleId?: number) => {
		pendingDeleteLesson = { id: lessonId, moduleId };
		showDeleteLessonModal = true;
	};

	const confirmDeleteLesson = async () => {
		if (!pendingDeleteLesson) return;
		showDeleteLessonModal = false;
		const { id: lessonId, moduleId } = pendingDeleteLesson;
		pendingDeleteLesson = null;
		try {
			// ICT 7 FIX: Use adminFetch for absolute URL on Pages.dev
			await adminFetch(`/api/admin/courses/${courseId}/lessons/${lessonId}`, { method: 'DELETE' });
			if (moduleId) {
				const mod = modules.find((m) => m.id === moduleId);
				if (mod) mod.lessons = mod.lessons.filter((l) => l.id !== lessonId);
				modules = [...modules];
			} else {
				unassignedLessons = unassignedLessons.filter((l) => l.id !== lessonId);
			}
		} catch {
			logger.error('Failed to delete lesson');
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
		<CourseHeader
			courseId={course.id}
			courseTitle={course.title}
			courseSlug={course.slug}
			isPublished={course.is_published}
			{saving}
			onPublishToggle={publishCourse}
			onSave={saveCourse}
		/>

		<CourseTabs bind:activeTab />

		<div class="tab-content">
			{#if activeTab === 'details'}
				<CourseDetailsTab bind:course />
			{:else if activeTab === 'content'}
				<CourseContentTab
					{courseId}
					{modules}
					{unassignedLessons}
					onAddModule={addModule}
					onDeleteModule={deleteModule}
					onAddLesson={addLesson}
					onDeleteLesson={deleteLesson}
				/>
			{:else if activeTab === 'downloads'}
				<CourseDownloadsTab
					{downloads}
					{uploading}
					onFileSelect={handleFileUpload}
					onDeleteDownload={deleteDownload}
				/>
			{:else if activeTab === 'settings'}
				<CourseSettingsTab bind:course />
			{/if}
		</div>
	{/if}
</div>

<ConfirmationModal
	isOpen={showDeleteModuleModal}
	title="Delete Module"
	message="Delete this module? Lessons will be unassigned."
	confirmText="Delete"
	variant="danger"
	onConfirm={confirmDeleteModule}
	onCancel={() => {
		showDeleteModuleModal = false;
		pendingDeleteModuleId = null;
	}}
/>

<ConfirmationModal
	isOpen={showDeleteDownloadModal}
	title="Delete Download"
	message="Delete this download?"
	confirmText="Delete"
	variant="danger"
	onConfirm={confirmDeleteDownload}
	onCancel={() => {
		showDeleteDownloadModal = false;
		pendingDeleteDownloadId = null;
	}}
/>

<ConfirmationModal
	isOpen={showDeleteLessonModal}
	title="Delete Lesson"
	message="Delete this lesson?"
	confirmText="Delete"
	variant="danger"
	onConfirm={confirmDeleteLesson}
	onCancel={() => {
		showDeleteLessonModal = false;
		pendingDeleteLesson = null;
	}}
/>

<!-- FIX-2026-04-26 (P0-3): input modal replacements for native prompt() flows. -->
<ConfirmationModal
	isOpen={showAddModuleModal}
	title="Add Module"
	message="Enter a title for the new module."
	confirmText="Create"
	variant="info"
	showInput={true}
	inputLabel="Module title"
	inputPlaceholder="e.g. Introduction"
	bind:inputValue={addModuleTitle}
	onConfirm={confirmAddModule}
	onCancel={() => {
		showAddModuleModal = false;
		addModuleTitle = '';
	}}
/>

<ConfirmationModal
	isOpen={showAddLessonModal}
	title="Add Lesson"
	message="Enter a title for the new lesson."
	confirmText="Create"
	variant="info"
	showInput={true}
	inputLabel="Lesson title"
	inputPlaceholder="e.g. Getting Started"
	bind:inputValue={addLessonTitle}
	onConfirm={confirmAddLesson}
	onCancel={() => {
		showAddLessonModal = false;
		addLessonTitle = '';
		pendingAddLessonModuleId = undefined;
	}}
/>

<style>
	.course-editor {
		padding: 24px;
		max-width: 1200px;
		margin: 0 auto;
	}

	.loading {
		text-align: center;
		padding: 64px;
	}
	.spinner {
		width: 40px;
		height: 40px;
		border: 3px solid #e5e7eb;
		border-top-color: #143e59;
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin: 0 auto 16px;
	}
	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	@media (prefers-reduced-motion: reduce) {
		.spinner {
			animation: none;
		}
	}
</style>

<script lang="ts">
	/**
	 * Page Builder Admin Route
	 * Apple Principal Engineer ICT 11 Grade - January 2026
	 * 
	 * Main page builder interface for creating and editing course pages.
	 * Fully integrated with backend API for save/load functionality.
	 */

	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import { createBuilderStore } from '$lib/page-builder';
	import ComponentPalette from '$lib/page-builder/components/ComponentPalette.svelte';
	import BuilderCanvas from '$lib/page-builder/components/BuilderCanvas.svelte';
	import ConfigPanel from '$lib/page-builder/components/ConfigPanel.svelte';

	// Create the builder store
	const store = createBuilderStore();

	// State
	let saving = $state(false);
	let loading = $state(true);
	let showSaveSuccess = $state(false);
	let layoutId = $state<string | null>(null);
	let courseId = $state<string | null>(null);
	let courseName = $state<string>('');
	let errorMessage = $state<string | null>(null);

	// Load course and layout data on mount
	onMount(async () => {
		const params = new URLSearchParams(window.location.search);
		courseId = params.get('course');

		if (courseId) {
			await loadCourseData(courseId);
		} else {
			// New layout without course
			loading = false;
		}
	});

	// Load course data and existing layout
	async function loadCourseData(id: string) {
		loading = true;
		errorMessage = null;

		try {
			// Fetch course details
			const courseRes = await fetch(`/api/admin/courses/${id}`);
			const courseData = await courseRes.json();

			if (courseData.success && courseData.data?.course) {
				const course = courseData.data.course;
				courseName = course.title;
				store.updateLayoutMeta({ 
					title: course.title,
					courseId: course.id
				});
			}

			// Try to load existing layout for this course
			const layoutRes = await fetch(`/api/admin/page-layouts/course/${id}`);
			const layoutData = await layoutRes.json();

			if (layoutData.success && layoutData.data) {
				// Load existing layout
				layoutId = layoutData.data.id;
				store.importLayout(layoutData.data);
			}
		} catch (e) {
			console.error('Failed to load course data:', e);
			errorMessage = 'Failed to load course data';
		} finally {
			loading = false;
		}
	}

	// Handle save
	async function handleSave() {
		saving = true;
		errorMessage = null;

		try {
			const layout = store.exportLayout();
			
			// Add course ID to layout
			if (courseId) {
				layout.courseId = courseId;
			}

			let res: Response;
			
			if (layoutId) {
				// Update existing layout
				res = await fetch(`/api/admin/page-layouts/${layoutId}`, {
					method: 'PUT',
					headers: { 'Content-Type': 'application/json' },
					body: JSON.stringify({
						title: layout.title,
						blocks: layout.blocks,
						status: layout.status
					})
				});
			} else {
				// Create new layout
				res = await fetch('/api/admin/page-layouts', {
					method: 'POST',
					headers: { 'Content-Type': 'application/json' },
					body: JSON.stringify({
						course_id: courseId,
						title: layout.title || 'Untitled Layout',
						blocks: layout.blocks
					})
				});
			}

			const data = await res.json();
			
			if (data.success) {
				if (!layoutId && data.data?.id) {
					layoutId = data.data.id;
				}
				store.markSaved();
				showSaveSuccess = true;
				setTimeout(() => { showSaveSuccess = false; }, 3000);
			} else {
				errorMessage = data.error || 'Failed to save layout';
			}
		} catch (e) {
			console.error('Save error:', e);
			errorMessage = 'Failed to save layout. Please try again.';
		} finally {
			saving = false;
		}
	}

	// Handle publish
	async function handlePublish() {
		if (!layoutId) {
			// Save first if no layout ID
			await handleSave();
		}

		if (layoutId) {
			saving = true;
			try {
				const res = await fetch(`/api/admin/page-layouts/${layoutId}/publish`, {
					method: 'POST'
				});
				const data = await res.json();
				
				if (data.success) {
					store.updateLayoutMeta({ status: 'published' });
					store.markSaved();
					showSaveSuccess = true;
					setTimeout(() => { showSaveSuccess = false; }, 3000);
				} else {
					errorMessage = data.error || 'Failed to publish layout';
				}
			} catch (e) {
				errorMessage = 'Failed to publish layout';
			} finally {
				saving = false;
			}
		}
	}

	// Navigate back to course
	function goBack() {
		if (courseId) {
			goto(`/admin/courses/${courseId}`);
		} else {
			goto('/admin/courses');
		}
	}

	// Warn before leaving with unsaved changes
	$effect(() => {
		if (typeof window === 'undefined') return;
		
		const handleBeforeUnload = (e: BeforeUnloadEvent) => {
			if (store.hasUnsavedChanges) {
				e.preventDefault();
				e.returnValue = '';
			}
		};
		window.addEventListener('beforeunload', handleBeforeUnload);
		return () => window.removeEventListener('beforeunload', handleBeforeUnload);
	});
</script>

<svelte:head>
	<title>{courseName ? `${courseName} - Page Builder` : 'Page Builder'} | Admin</title>
</svelte:head>

{#if loading}
	<div class="loading-screen">
		<div class="loading-content">
			<div class="spinner-large"></div>
			<p>Loading page builder...</p>
		</div>
	</div>
{:else}
	<div class="page-builder">
		<!-- Error Banner -->
		{#if errorMessage}
			<div class="error-banner">
				<svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><line x1="12" x2="12" y1="8" y2="12"/><line x1="12" x2="12.01" y1="16" y2="16"/></svg>
				{errorMessage}
				<button onclick={() => errorMessage = null} aria-label="Dismiss">
					<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M18 6 6 18"/><path d="m6 6 12 12"/></svg>
				</button>
			</div>
		{/if}

		<!-- Top Bar -->
		<header class="builder-header">
			<div class="header-left">
				<button onclick={goBack} class="back-link">
					<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="m15 18-6-6 6-6"/></svg>
					{courseId ? 'Back to Course' : 'Back to Courses'}
				</button>
				{#if courseName}
					<span class="course-name">{courseName}</span>
				{/if}
			</div>
			<div class="header-center">
				<span class="builder-badge">Page Builder</span>
				{#if layoutId}
					<span class="layout-badge">Editing</span>
				{:else}
					<span class="layout-badge new">New Layout</span>
				{/if}
				{#if store.hasUnsavedChanges}
					<span class="unsaved-badge">Unsaved changes</span>
				{/if}
			</div>
			<div class="header-right">
				{#if showSaveSuccess}
					<span class="save-success">
						<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M20 6 9 17l-5-5"/></svg>
						Saved
					</span>
				{/if}
				<button class="btn-secondary" onclick={handleSave} disabled={saving}>
					{#if saving}
						<span class="spinner"></span>
						Saving...
					{:else}
						<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"/><polyline points="17 21 17 13 7 13 7 21"/><polyline points="7 3 7 8 15 8"/></svg>
						Save Draft
					{/if}
				</button>
				<button class="btn-primary" onclick={handlePublish} disabled={saving}>
					<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="17 8 12 3 7 8"/><line x1="12" x2="12" y1="3" y2="15"/></svg>
					Publish
				</button>
			</div>
		</header>

		<!-- Main Builder Area -->
		<main class="builder-main">
			<ComponentPalette {store} />
			<BuilderCanvas {store} />
			<ConfigPanel {store} />
		</main>
	</div>
{/if}

<style>
	:global(body) {
		margin: 0;
		overflow: hidden;
	}

	/* Loading Screen */
	.loading-screen {
		display: flex;
		align-items: center;
		justify-content: center;
		height: 100vh;
		background: #F3F4F6;
	}

	.loading-content {
		text-align: center;
	}

	.spinner-large {
		width: 48px;
		height: 48px;
		border: 4px solid #E5E7EB;
		border-top-color: #143E59;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
		margin: 0 auto 16px;
	}

	.loading-content p {
		color: #6B7280;
		font-size: 14px;
		margin: 0;
	}

	/* Error Banner */
	.error-banner {
		display: flex;
		align-items: center;
		gap: 10px;
		padding: 12px 20px;
		background: #FEF2F2;
		border-bottom: 1px solid #FECACA;
		color: #DC2626;
		font-size: 14px;
	}

	.error-banner button {
		margin-left: auto;
		background: none;
		border: none;
		color: #DC2626;
		cursor: pointer;
		padding: 4px;
		border-radius: 4px;
	}

	.error-banner button:hover {
		background: #FEE2E2;
	}

	/* Course Name */
	.course-name {
		font-size: 14px;
		font-weight: 600;
		color: #1F2937;
		padding-left: 12px;
		border-left: 2px solid #E5E7EB;
	}

	/* Layout Badge */
	.layout-badge {
		padding: 4px 10px;
		background: #E5E7EB;
		color: #374151;
		font-size: 11px;
		font-weight: 500;
		border-radius: 12px;
	}

	.layout-badge.new {
		background: #DBEAFE;
		color: #1D4ED8;
	}

	.page-builder {
		display: flex;
		flex-direction: column;
		height: 100vh;
		background: #F3F4F6;
		font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
	}

	/* Header */
	.builder-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 12px 20px;
		background: #FFFFFF;
		border-bottom: 1px solid #E5E7EB;
		gap: 16px;
		flex-shrink: 0;
	}

	.header-left,
	.header-right {
		display: flex;
		align-items: center;
		gap: 12px;
	}

	.header-center {
		display: flex;
		align-items: center;
		gap: 12px;
	}

	.back-link {
		display: flex;
		align-items: center;
		gap: 6px;
		color: #6B7280;
		text-decoration: none;
		font-size: 14px;
		font-weight: 500;
		transition: color 0.15s ease;
	}

	.back-link:hover {
		color: #143E59;
	}

	.builder-badge {
		display: inline-flex;
		align-items: center;
		padding: 4px 12px;
		background: linear-gradient(135deg, #143E59 0%, #1E73BE 100%);
		color: white;
		font-size: 12px;
		font-weight: 600;
		border-radius: 16px;
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.unsaved-badge {
		display: inline-flex;
		align-items: center;
		padding: 4px 10px;
		background: #FEF3C7;
		color: #92400E;
		font-size: 12px;
		font-weight: 500;
		border-radius: 12px;
	}

	.save-success {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		padding: 6px 12px;
		background: #D1FAE5;
		color: #065F46;
		font-size: 13px;
		font-weight: 500;
		border-radius: 6px;
		animation: fadeIn 0.2s ease;
	}

	@keyframes fadeIn {
		from { opacity: 0; transform: translateY(-4px); }
		to { opacity: 1; transform: translateY(0); }
	}

	.btn-secondary,
	.btn-primary {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 10px 16px;
		border: none;
		border-radius: 8px;
		font-size: 14px;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.btn-secondary {
		background: #F3F4F6;
		color: #374151;
		border: 1px solid #E5E7EB;
	}

	.btn-secondary:hover:not(:disabled) {
		background: #E5E7EB;
	}

	.btn-secondary:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.btn-primary {
		background: linear-gradient(135deg, #143E59 0%, #1E5A8A 100%);
		color: white;
	}

	.btn-primary:hover:not(:disabled) {
		background: linear-gradient(135deg, #0F2D42 0%, #143E59 100%);
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(20, 62, 89, 0.3);
	}

	.btn-primary:disabled {
		opacity: 0.5;
		cursor: not-allowed;
		transform: none;
		box-shadow: none;
	}

	.spinner {
		width: 14px;
		height: 14px;
		border: 2px solid #D1D5DB;
		border-top-color: #374151;
		border-radius: 50%;
		animation: spin 0.6s linear infinite;
	}

	@keyframes spin {
		to { transform: rotate(360deg); }
	}

	/* Main Area */
	.builder-main {
		flex: 1;
		display: flex;
		overflow: hidden;
	}

	/* Responsive */
	@media (max-width: 1024px) {
		.header-center {
			display: none;
		}
	}

	@media (max-width: 768px) {
		.builder-header {
			flex-wrap: wrap;
			gap: 12px;
		}

		.header-left,
		.header-right {
			width: 100%;
			justify-content: center;
		}

		.builder-main {
			flex-direction: column;
		}
	}
</style>

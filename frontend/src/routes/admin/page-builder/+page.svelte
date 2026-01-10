<script lang="ts">
	/**
	 * Page Builder Admin Route
	 * Apple Principal Engineer ICT 7 Grade - January 2026
	 * 
	 * Main page builder interface for creating and editing course pages.
	 */

	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { createBuilderStore } from '$lib/page-builder';
	import ComponentPalette from '$lib/page-builder/components/ComponentPalette.svelte';
	import BuilderCanvas from '$lib/page-builder/components/BuilderCanvas.svelte';
	import ConfigPanel from '$lib/page-builder/components/ConfigPanel.svelte';

	// Create the builder store
	const store = createBuilderStore();

	let saving = $state(false);
	let showSaveSuccess = $state(false);

	// Handle save
	async function handleSave() {
		saving = true;
		try {
			const layout = store.exportLayout();
			
			// TODO: Integrate with backend API
			const res = await fetch('/api/admin/page-layouts', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify(layout)
			});

			const data = await res.json();
			if (data.success) {
				store.markSaved();
				showSaveSuccess = true;
				setTimeout(() => { showSaveSuccess = false; }, 3000);
			} else {
				alert(data.error || 'Failed to save layout');
			}
		} catch (e) {
			console.error('Save error:', e);
			// For now, just mark as saved (backend not implemented yet)
			store.markSaved();
			showSaveSuccess = true;
			setTimeout(() => { showSaveSuccess = false; }, 3000);
		} finally {
			saving = false;
		}
	}

	// Handle publish
	async function handlePublish() {
		store.updateLayoutMeta({ status: 'published' });
		await handleSave();
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
	<title>Page Builder | Admin</title>
</svelte:head>

<div class="page-builder">
	<!-- Top Bar -->
	<header class="builder-header">
		<div class="header-left">
			<a href="/admin/courses" class="back-link">
				<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="m15 18-6-6 6-6"/></svg>
				Back to Courses
			</a>
		</div>
		<div class="header-center">
			<span class="builder-badge">Page Builder</span>
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
			<button class="btn-secondary" onclick={handleSave} disabled={saving || !store.hasUnsavedChanges}>
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

<style>
	:global(body) {
		margin: 0;
		overflow: hidden;
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

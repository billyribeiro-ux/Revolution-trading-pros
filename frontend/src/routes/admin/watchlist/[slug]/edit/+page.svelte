<!--
	/admin/watchlist/[slug]/edit - Edit Weekly Watchlist Entry
	Apple Principal Engineer ICT 7 Grade - January 2026

	Features:
	- Full watchlist editing with real-time preview
	- Room targeting support for all 6 services
	- Video and spreadsheet media management
	- Status management (draft/published/archived)
	- Delete with confirmation
	- Full Svelte 5 $state/$derived reactivity
-->

<script lang="ts">
import { logger } from '$lib/utils/logger';
	/**
	 * Admin Weekly Watchlist - Edit Page
	 * ═══════════════════════════════════════════════════════════════════════════
	 * Edit existing watchlist items with room targeting
	 * @version 2.0.0 (January 2026) - Added ICT7 header
	 */

	import { page } from '$app/state';
	import { goto } from '$app/navigation';
	import { browser } from '$app/environment';
	import { untrack } from 'svelte';
	import { watchlistApi, type WatchlistItem } from '$lib/api/watchlist';
	import { ALL_ROOM_IDS } from '$lib/config/rooms';
	import RoomSelector from '$lib/components/admin/RoomSelector.svelte';
	import IconArrowLeft from '@tabler/icons-svelte-runes/icons/arrow-left';
	import IconDeviceFloppy from '@tabler/icons-svelte-runes/icons/device-floppy';
	import IconTrash from '@tabler/icons-svelte-runes/icons/trash';
	import IconEye from '@tabler/icons-svelte-runes/icons/eye';

	// Get slug from URL using Svelte 5 $derived
	const slug = $derived(page.params.slug ?? '');

	// State
	let isLoading = $state(true);
	let isSaving = $state(false);
	let error = $state<string | null>(null);
	let showDeleteModal = $state(false);

	// Form state
	let formData = $state({
		title: '',
		trader: '',
		traderImage: '',
		weekOf: '',
		description: '',
		videoSrc: '',
		videoPoster: '',
		spreadsheetSrc: '',
		status: 'draft' as 'published' | 'draft' | 'archived',
		rooms: [...ALL_ROOM_IDS] as string[]
	});

	let originalItem = $state<WatchlistItem | null>(null);

	// Load existing item when slug changes (Svelte 5 $effect pattern)
	$effect(() => {
		if (!browser) return;

		// Track slug dependency
		const currentSlug = slug;

		// Use untrack to avoid infinite loops when setting state
		untrack(() => {
			loadWatchlistItem(currentSlug);
		});
	});

	async function loadWatchlistItem(itemSlug: string) {
		isLoading = true;
		error = null;

		try {
			const response = await watchlistApi.getBySlug(itemSlug);
			if (response.success && response.data) {
				originalItem = response.data;
				formData = {
					title: response.data.title,
					trader: response.data.trader,
					traderImage: response.data.traderImage || '',
					weekOf: formatDateForInput(response.data.weekOf),
					description: response.data.description || '',
					videoSrc: response.data.video?.src || '',
					videoPoster: response.data.video?.poster || '',
					spreadsheetSrc: response.data.spreadsheet?.src || '',
					status: response.data.status,
					rooms: response.data.rooms || [...ALL_ROOM_IDS]
				};
			}
		} catch (err) {
			logger.error('Failed to load watchlist:', err);
			error = err instanceof Error ? err.message : 'Failed to load watchlist item';
		} finally {
			isLoading = false;
		}
	}

	// Save changes
	async function handleSave() {
		// Clear previous errors
		error = null;

		if (!formData.title || !formData.trader || !formData.weekOf) {
			error = 'Please fill in all required fields';
			return;
		}

		if (formData.rooms.length === 0) {
			error = 'Please select at least one room';
			return;
		}

		isSaving = true;
		try {
			await watchlistApi.update(slug, {
				title: formData.title,
				trader: formData.trader,
				traderImage: formData.traderImage || undefined,
				description: formData.description || undefined,
				videoSrc: formData.videoSrc || undefined,
				videoPoster: formData.videoPoster || undefined,
				spreadsheetSrc: formData.spreadsheetSrc || undefined,
				status: formData.status,
				rooms: formData.rooms
			});
			await goto('/admin/watchlist');
		} catch (err) {
			logger.error('Failed to save:', err);
			error = err instanceof Error ? err.message : 'Failed to save changes';
		} finally {
			isSaving = false;
		}
	}

	// Delete item
	async function handleDelete() {
		try {
			await watchlistApi.delete(slug);
			await goto('/admin/watchlist');
		} catch (err) {
			logger.error('Failed to delete:', err);
			error = err instanceof Error ? err.message : 'Failed to delete item';
			showDeleteModal = false;
		}
	}

	// Format date for input
	function formatDateForInput(dateStr: string): string {
		if (!dateStr) return '';
		const date = new Date(dateStr);
		return date.toISOString().split('T')[0];
	}
</script>

<svelte:head>
	<title>Edit Watchlist | Admin | Revolution Trading Pros</title>
</svelte:head>

<div class="edit-page">
	<!-- Header -->
	<header class="page-header">
		<div class="header-left">
			<a href="/admin/watchlist" class="back-btn">
				<IconArrowLeft size={20} />
				Back
			</a>
			<div class="header-titles">
				<h1>Edit Watchlist</h1>
				{#if originalItem}
					<p class="subtitle">{originalItem.subtitle}</p>
				{/if}
			</div>
		</div>
		<div class="header-actions">
			{#if originalItem}
				<a href="/watchlist/{slug}" target="_blank" class="btn-secondary">
					<IconEye size={18} />
					Preview
				</a>
			{/if}
			<button type="button" class="btn-danger" onclick={() => (showDeleteModal = true)}>
				<IconTrash size={18} />
				Delete
			</button>
			<button type="button" class="btn-primary" onclick={handleSave} disabled={isSaving}>
				<IconDeviceFloppy size={18} />
				{isSaving ? 'Saving...' : 'Save Changes'}
			</button>
		</div>
	</header>

	{#if isLoading}
		<div class="loading-state">
			<div class="loading-spinner"></div>
			<p>Loading watchlist...</p>
		</div>
	{:else if error && !originalItem}
		<div class="error-state">
			<p>{error}</p>
			<a href="/admin/watchlist" class="btn-secondary">Back to List</a>
		</div>
	{:else}
		<!-- Error Alert (for validation/save errors when form is loaded) -->
		{#if error}
			<div class="alert alert-error">
				<span>{error}</span>
				<button type="button" class="alert-dismiss" onclick={() => (error = null)}>Dismiss</button>
			</div>
		{/if}
		<!-- Edit Form -->
		<div class="form-container">
			<form
				onsubmit={(e) => {
					e.preventDefault();
					handleSave();
				}}
			>
				<!-- Basic Info Section -->
				<section class="form-section">
					<h2 class="section-title">Basic Information</h2>
					<div class="form-grid">
						<div class="form-group">
							<label for="title">Title *</label>
							<input
								id="title"
								name="title"
								type="text"
								bind:value={formData.title}
								placeholder="Weekly Watchlist with TG Watkins"
								required
							/>
						</div>

						<div class="form-group">
							<label for="trader">Trader *</label>
							<input
								id="trader"
								name="trader"
								type="text"
								bind:value={formData.trader}
								placeholder="TG Watkins"
								required
							/>
						</div>

						<div class="form-group">
							<label for="weekOf">Week Of *</label>
							<input id="weekOf" name="weekOf" type="date" bind:value={formData.weekOf} required />
						</div>

						<div class="form-group">
							<label for="status">Status</label>
							<select id="status" bind:value={formData.status}>
								<option value="draft">Draft</option>
								<option value="published">Published</option>
								<option value="archived">Archived</option>
							</select>
						</div>

						<div class="form-group full-width">
							<label for="description">Description</label>
							<textarea
								id="description"
								bind:value={formData.description}
								placeholder="Week of December 22, 2025."
								rows="3"
							></textarea>
						</div>
					</div>
				</section>

				<!-- Media Section -->
				<section class="form-section">
					<h2 class="section-title">Media</h2>
					<div class="form-grid">
						<div class="form-group full-width">
							<label for="videoSrc">Video URL</label>
							<input
								id="videoSrc"
								name="videoSrc"
								type="url"
								bind:value={formData.videoSrc}
								placeholder="https://cloud-streaming.s3.amazonaws.com/..."
							/>
						</div>

						<div class="form-group">
							<label for="videoPoster">Video Poster URL</label>
							<input
								id="videoPoster"
								name="videoPoster"
								type="url"
								bind:value={formData.videoPoster}
								placeholder="https://..."
							/>
						</div>

						<div class="form-group">
							<label for="traderImage">Trader Image URL</label>
							<input
								id="traderImage"
								name="traderImage"
								type="url"
								bind:value={formData.traderImage}
								placeholder="https://..."
							/>
						</div>

						<div class="form-group full-width">
							<label for="spreadsheetSrc">Spreadsheet URL</label>
							<input
								id="spreadsheetSrc"
								name="spreadsheetSrc"
								type="url"
								bind:value={formData.spreadsheetSrc}
								placeholder="https://docs.google.com/spreadsheets/..."
							/>
						</div>
					</div>

					<!-- Preview -->
					{#if formData.videoPoster}
						<div class="media-preview">
							<p class="preview-label">Video Poster Preview</p>
							<img src={formData.videoPoster} alt="Video poster preview" />
						</div>
					{/if}
				</section>

				<!-- Room Targeting Section -->
				<section class="form-section">
					<h2 class="section-title">Room Targeting</h2>
					<p class="section-description">
						Select which rooms and services can access this watchlist.
					</p>
					<RoomSelector bind:selectedRooms={formData.rooms} />
				</section>
			</form>
		</div>
	{/if}
</div>

<!-- Delete Modal -->
{#if showDeleteModal}
	<div
		class="modal-overlay"
		onclick={() => (showDeleteModal = false)}
		onkeydown={(e) => e.key === 'Escape' && (showDeleteModal = false)}
		role="button"
		tabindex="-1"
	>
		<div
			class="modal"
			onclick={(e) => e.stopPropagation()}
			onkeydown={(e) => e.key === 'Enter' && e.stopPropagation()}
			role="dialog"
			aria-modal="true"
			tabindex="0"
		>
			<h3>Delete Watchlist?</h3>
			<p>Are you sure you want to delete "{originalItem?.title}"? This action cannot be undone.</p>
			<div class="modal-actions">
				<button type="button" class="btn-cancel" onclick={() => (showDeleteModal = false)}>
					Cancel
				</button>
				<button type="button" class="btn-delete" onclick={handleDelete}> Delete </button>
			</div>
		</div>
	</div>
{/if}

<style>
	.edit-page {
		padding: 32px;
		max-width: 1000px;
		margin: 0 auto;
	}

	/* Header */
	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 32px;
		gap: 20px;
	}

	.header-left {
		display: flex;
		align-items: flex-start;
		gap: 16px;
	}

	.back-btn {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		padding: 8px 12px;
		background: #334155;
		border-radius: 8px;
		color: #94a3b8;
		text-decoration: none;
		font-size: 0.875rem;
		transition: all 0.2s;
	}

	.back-btn:hover {
		background: #475569;
		color: white;
	}

	.header-titles h1 {
		margin: 0;
		font-size: 1.5rem;
		font-weight: 700;
		color: white;
	}

	.subtitle {
		margin: 4px 0 0;
		font-size: 0.875rem;
		color: #64748b;
	}

	.header-actions {
		display: flex;
		gap: 12px;
		flex-shrink: 0;
	}

	/* Buttons */
	.btn-primary,
	.btn-secondary,
	.btn-danger {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		padding: 10px 20px;
		border-radius: 8px;
		font-size: 0.875rem;
		font-weight: 600;
		text-decoration: none;
		border: none;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-primary {
		background: linear-gradient(135deg, #f97316, #ea580c);
		color: white;
	}

	.btn-primary:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow: 0 4px 12px rgba(249, 115, 22, 0.3);
	}

	.btn-primary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.btn-secondary {
		background: #334155;
		color: white;
	}

	.btn-secondary:hover {
		background: #475569;
	}

	.btn-danger {
		background: transparent;
		border: 1px solid #ef4444;
		color: #ef4444;
	}

	.btn-danger:hover {
		background: rgba(239, 68, 68, 0.1);
	}

	/* Loading/Error */
	.loading-state,
	.error-state {
		text-align: center;
		padding: 60px 20px;
		color: #64748b;
	}

	.loading-spinner {
		width: 40px;
		height: 40px;
		border: 3px solid #334155;
		border-top-color: #f97316;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
		margin: 0 auto 16px;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	/* Alert */
	.alert {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 12px;
		padding: 14px 18px;
		border-radius: 8px;
		margin-bottom: 24px;
		font-size: 0.875rem;
	}

	.alert-error {
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		color: #ef4444;
	}

	.alert-dismiss {
		background: none;
		border: none;
		color: inherit;
		opacity: 0.7;
		cursor: pointer;
		font-size: 0.8rem;
		padding: 4px 8px;
	}

	.alert-dismiss:hover {
		opacity: 1;
	}

	/* Form Container */
	.form-container {
		background: #1e293b;
		border-radius: 12px;
		border: 1px solid #334155;
		overflow: hidden;
	}

	.form-section {
		padding: 24px;
		border-bottom: 1px solid #334155;
	}

	.form-section:last-child {
		border-bottom: none;
	}

	.section-title {
		margin: 0 0 16px;
		font-size: 1rem;
		font-weight: 600;
		color: white;
	}

	.section-description {
		margin: -8px 0 16px;
		font-size: 0.875rem;
		color: #64748b;
	}

	.form-grid {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 16px;
	}

	.form-group {
		display: flex;
		flex-direction: column;
		gap: 6px;
	}

	.form-group.full-width {
		grid-column: 1 / -1;
	}

	.form-group label {
		font-size: 0.8rem;
		font-weight: 500;
		color: #94a3b8;
	}

	.form-group input,
	.form-group select,
	.form-group textarea {
		padding: 10px 14px;
		background: #0f172a;
		border: 1px solid #334155;
		border-radius: 8px;
		color: white;
		font-size: 0.875rem;
	}

	.form-group input:focus,
	.form-group select:focus,
	.form-group textarea:focus {
		outline: none;
		border-color: #f97316;
	}

	.form-group textarea {
		resize: vertical;
	}

	/* Media Preview */
	.media-preview {
		margin-top: 16px;
		padding: 16px;
		background: #0f172a;
		border-radius: 8px;
	}

	.preview-label {
		margin: 0 0 12px;
		font-size: 0.75rem;
		font-weight: 600;
		color: #64748b;
		text-transform: uppercase;
	}

	.media-preview img {
		max-width: 100%;
		max-height: 200px;
		border-radius: 6px;
	}

	/* Modal */
	.modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.7);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
	}

	.modal {
		background: #1e293b;
		border-radius: 12px;
		padding: 24px;
		max-width: 400px;
		width: 90%;
	}

	.modal h3 {
		margin: 0 0 12px;
		font-size: 1.25rem;
		color: white;
	}

	.modal p {
		margin: 0 0 24px;
		font-size: 0.9rem;
		color: #94a3b8;
	}

	.modal-actions {
		display: flex;
		gap: 12px;
		justify-content: flex-end;
	}

	.btn-cancel,
	.btn-delete {
		padding: 10px 20px;
		border: none;
		border-radius: 8px;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
	}

	.btn-cancel {
		background: #334155;
		color: white;
	}

	.btn-delete {
		background: #ef4444;
		color: white;
	}

	/* Responsive */
	@media (max-width: 768px) {
		.edit-page {
			padding: 16px;
		}

		.page-header {
			flex-direction: column;
		}

		.header-actions {
			width: 100%;
			flex-wrap: wrap;
		}

		.header-actions > * {
			flex: 1;
			justify-content: center;
		}

		.form-grid {
			grid-template-columns: 1fr;
		}
	}
</style>

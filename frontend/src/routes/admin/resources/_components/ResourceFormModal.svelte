<script lang="ts">
	import { IconCheck } from '$lib/icons';
	import type {
		CreateResourceRequest,
		ResourceType,
		ContentType,
		VideoPlatform,
		AccessLevel
	} from '$lib/api/room-resources';
	import type { Trader } from '$lib/api/trading-rooms';

	type ResourceTypeOption = { id: ResourceType; name: string; icon: unknown };
	type ContentTypeOption = { id: ContentType; name: string; resourceTypes: ResourceType[] };
	type SectionOption = { id: string; name: string; icon?: unknown; description?: string };
	type AccessLevelOption = { id: AccessLevel; name: string; color: string };
	type VideoPlatformOption = { id: VideoPlatform; name: string };
	type CategoryOption = { id: string; name: string; color: string };

	type Props = {
		isEdit: boolean;
		formData: CreateResourceRequest;
		resourceTypes: ResourceTypeOption[];
		availableContentTypes: ContentTypeOption[];
		availableSections: SectionOption[];
		accessLevels: AccessLevelOption[];
		videoPlatforms: VideoPlatformOption[];
		categories: CategoryOption[];
		traders: Trader[];
		isSaving: boolean;
		onClose: () => void;
		onSave: () => void;
		onResourceTypeChange: (resourceType: ResourceType) => void;
		onFileUrlInput: (fileUrl: string) => void;
		onToggleTag: (id: string) => void;
	};

	let {
		isEdit,
		formData = $bindable(),
		resourceTypes,
		availableContentTypes,
		availableSections,
		accessLevels,
		videoPlatforms,
		categories,
		traders,
		isSaving,
		onClose,
		onSave,
		onResourceTypeChange,
		onFileUrlInput,
		onToggleTag
	}: Props = $props();
</script>

<div
	class="modal-overlay"
	onclick={(e: MouseEvent) => {
		if (e.target === e.currentTarget) onClose();
	}}
	onkeydown={(e: KeyboardEvent) => {
		if (e.key === 'Escape') onClose();
	}}
	role="dialog"
	aria-modal="true"
	tabindex="-1"
>
	<div class="modal modal-large" role="document">
		<div class="modal-header">
			<h2>{isEdit ? 'Edit Resource' : 'Add New Resource'}</h2>
			<button class="modal-close" onclick={onClose}>&times;</button>
		</div>
		<div class="modal-body">
			<!-- Title -->
			<div class="form-group">
				<label for="title">Title *</label>
				<input
					type="text"
					id="title"
					name="title"
					bind:value={formData.title}
					placeholder="Resource title"
					required
				/>
			</div>

			<!-- Description -->
			<div class="form-group">
				<label for="description">Description</label>
				<textarea
					id="description"
					bind:value={formData.description}
					placeholder="Brief description..."
					rows="3"
				></textarea>
			</div>

			<!-- Type Selection -->
			<div class="form-row">
				<div class="form-group">
					<label for="resource-type">Resource Type *</label>
					<select
						id="resource-type"
						value={formData.resource_type}
						onchange={(event) => onResourceTypeChange(event.currentTarget.value as ResourceType)}
					>
						{#each resourceTypes as type (type.id)}
							<option value={type.id}>{type.name}</option>
						{/each}
					</select>
				</div>
				<div class="form-group">
					<label for="content-type">Content Type *</label>
					<select id="content-type" bind:value={formData.content_type}>
						{#each availableContentTypes as type (type.id)}
							<option value={type.id}>{type.name}</option>
						{/each}
					</select>
				</div>
			</div>

			<!-- ICT 7: Section Selection -->
			<div class="form-row">
				<div class="form-group">
					<label for="section">Dashboard Section</label>
					<select id="section" bind:value={formData.section}>
						{#each availableSections as section (section.id)}
							<option value={section.id}>{section.name}</option>
						{/each}
					</select>
					<small class="form-hint">Where this resource appears</small>
				</div>
				<!-- ICT 7: Access Level -->
				<div class="form-group">
					<label for="access-level">Access Level</label>
					<select id="access-level" bind:value={formData.access_level}>
						{#each accessLevels as level (level.id)}
							<option value={level.id}>{level.name}</option>
						{/each}
					</select>
					<small class="form-hint">Who can access this resource</small>
				</div>
			</div>

			<!-- File URL -->
			<div class="form-group">
				<label for="file-url">File/Video URL *</label>
				<input
					type="url"
					id="file-url"
					name="file-url"
					value={formData.file_url}
					oninput={(event) => onFileUrlInput(event.currentTarget.value)}
					placeholder="https://..."
					required
				/>
				{#if formData.resource_type === 'video' && formData.video_platform}
					<small class="form-hint"
						>Detected platform: <strong>{formData.video_platform}</strong></small
					>
				{/if}
			</div>

			<!-- Video Platform (for videos only) -->
			{#if formData.resource_type === 'video'}
				<div class="form-row">
					<div class="form-group">
						<label for="video-platform">Video Platform</label>
						<select id="video-platform" bind:value={formData.video_platform}>
							{#each videoPlatforms as platform (platform.id)}
								<option value={platform.id}>{platform.name}</option>
							{/each}
						</select>
					</div>
					<div class="form-group">
						<label for="trader">Trader</label>
						<select id="trader" bind:value={formData.trader_id}>
							<option value={undefined}>Select trader...</option>
							{#each traders as trader (trader.id)}
								<option value={trader.id}>{trader.name}</option>
							{/each}
						</select>
					</div>
				</div>
			{/if}

			<!-- Thumbnail & Date -->
			<div class="form-row">
				<div class="form-group">
					<label for="thumbnail-url">Thumbnail URL</label>
					<input
						type="url"
						id="thumbnail-url"
						name="thumbnail-url"
						bind:value={formData.thumbnail_url}
						placeholder="https://..."
					/>
				</div>
				<div class="form-group">
					<label for="resource-date">Date</label>
					<input
						type="date"
						id="resource-date"
						name="resource-date"
						bind:value={formData.resource_date}
					/>
				</div>
			</div>

			<!-- Tags -->
			<div class="form-group">
				<span id="resource-tags-label" class="group-label">Categories/Tags</span>
				<div class="tags-grid" role="group" aria-labelledby="resource-tags-label">
					{#each categories as category (category.id)}
						<button
							type="button"
							class={['tag-btn', { selected: formData.tags?.includes(category.id) }]}
							style:--tag-color={category.color}
							onclick={() => onToggleTag(category.id)}
						>
							{#if formData.tags?.includes(category.id)}
								<IconCheck size={14} />
							{/if}
							{category.name}
						</button>
					{/each}
				</div>
			</div>

			<!-- Options -->
			<div class="form-options">
				<label class="checkbox-label">
					<input
						id="page-formdata-is-published"
						name="page-formdata-is-published"
						type="checkbox"
						bind:checked={formData.is_published}
					/>
					<span>Published</span>
				</label>
				<label class="checkbox-label">
					<input
						id="page-formdata-is-featured"
						name="page-formdata-is-featured"
						type="checkbox"
						bind:checked={formData.is_featured}
					/>
					<span>Featured (Main Resource)</span>
				</label>
				<label class="checkbox-label">
					<input
						id="page-formdata-is-pinned"
						name="page-formdata-is-pinned"
						type="checkbox"
						bind:checked={formData.is_pinned}
					/>
					<span>Pinned (Always on top)</span>
				</label>
			</div>
		</div>
		<div class="modal-footer">
			<button class="btn-secondary" onclick={onClose}>Cancel</button>
			<button
				class="btn-primary"
				onclick={onSave}
				disabled={isSaving || !formData.title || !formData.file_url}
			>
				{#if isSaving}
					<span class="spinner-small"></span>
					Saving...
				{:else}
					{isEdit ? 'Update' : 'Create'} Resource
				{/if}
			</button>
		</div>
	</div>
</div>

<style>
	/* Modal shell */
	.modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.75);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
		padding: 1rem;
	}

	.modal {
		background: #1e293b;
		border-radius: 16px;
		width: 100%;
		max-width: 500px;
		max-height: 90vh;
		overflow: hidden;
		display: flex;
		flex-direction: column;
	}

	.modal.modal-large {
		max-width: 700px;
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1.25rem 1.5rem;
		border-bottom: 1px solid rgba(230, 184, 0, 0.2);
	}

	.modal-header h2 {
		margin: 0;
		font-size: 1.25rem;
		color: #f1f5f9;
	}

	.modal-close {
		background: transparent;
		border: none;
		font-size: 1.5rem;
		color: #64748b;
		cursor: pointer;
		line-height: 1;
	}

	.modal-close:hover {
		color: #f1f5f9;
	}

	.modal-body {
		padding: 1.5rem;
		overflow-y: auto;
	}

	.modal-footer {
		display: flex;
		justify-content: flex-end;
		gap: 0.75rem;
		padding: 1rem 1.5rem;
		border-top: 1px solid rgba(230, 184, 0, 0.2);
	}

	/* Form Elements */
	.form-group {
		margin-bottom: 1.25rem;
	}

	.form-group label,
	.form-group .group-label {
		display: block;
		margin-bottom: 0.5rem;
		color: #94a3b8;
		font-size: 0.875rem;
		font-weight: 500;
	}

	.form-group input,
	.form-group textarea,
	.form-group select {
		width: 100%;
		padding: 0.75rem 1rem;
		background: rgba(15, 23, 42, 0.8);
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 8px;
		color: #f1f5f9;
		font-size: 0.9rem;
	}

	.form-group input:focus,
	.form-group textarea:focus,
	.form-group select:focus {
		outline: none;
		border-color: var(--primary-500);
	}

	.form-group textarea {
		resize: vertical;
		min-height: 80px;
	}

	.form-row {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1rem;
	}

	.form-hint {
		display: block;
		margin-top: 0.5rem;
		color: #64748b;
		font-size: 0.8rem;
	}

	.form-hint strong {
		color: #4ade80;
		text-transform: uppercase;
	}

	/* Tags Grid */
	.tags-grid {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
	}

	.tag-btn {
		display: flex;
		align-items: center;
		gap: 0.35rem;
		padding: 0.5rem 0.75rem;
		background: rgba(15, 23, 42, 0.8);
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 6px;
		color: #94a3b8;
		font-size: 0.8rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.tag-btn:hover {
		border-color: var(--tag-color, var(--primary-500));
		color: var(--tag-color, var(--primary-400));
	}

	.tag-btn.selected {
		background: color-mix(in srgb, var(--tag-color, var(--primary-500)) 20%, transparent);
		border-color: var(--tag-color, var(--primary-500));
		color: var(--tag-color, var(--primary-400));
	}

	/* Form Options */
	.form-options {
		display: flex;
		flex-wrap: wrap;
		gap: 1.5rem;
	}

	.checkbox-label {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		cursor: pointer;
		color: #94a3b8;
		font-size: 0.9rem;
	}

	.checkbox-label input[type='checkbox'] {
		width: 18px;
		height: 18px;
		accent-color: var(--primary-500);
	}

	/* Buttons */
	.btn-primary {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background: linear-gradient(135deg, var(--primary-500) 0%, var(--primary-600) 100%);
		color: var(--bg-base);
		border: none;
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-primary:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow: 0 4px 15px rgba(230, 184, 0, 0.4);
	}

	.btn-primary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.btn-secondary {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background: rgba(230, 184, 0, 0.1);
		color: #94a3b8;
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-secondary:hover {
		background: rgba(230, 184, 0, 0.2);
		color: #e2e8f0;
	}

	.spinner-small {
		width: 16px;
		height: 16px;
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-top-color: white;
		border-radius: 50%;
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	/* Mobile */
	@media (max-width: 639px) {
		.modal-overlay {
			padding: 0.5rem;
			padding-top: calc(0.5rem + env(safe-area-inset-top, 0px));
			padding-bottom: calc(0.5rem + env(safe-area-inset-bottom, 0px));
		}

		.modal {
			max-width: 100%;
			max-height: calc(
				100vh - 1rem - env(safe-area-inset-top, 0px) - env(safe-area-inset-bottom, 0px)
			);
		}

		.modal.modal-large {
			max-width: 100%;
		}

		.modal-header {
			padding: 1rem;
		}

		.modal-header h2 {
			font-size: 1.125rem;
		}

		.modal-body {
			padding: 1rem;
		}

		.modal-footer {
			padding: 0.75rem 1rem;
			flex-direction: column;
			gap: 0.5rem;
		}

		.modal-footer button {
			width: 100%;
			min-height: 44px;
		}

		.form-row {
			grid-template-columns: 1fr;
			gap: 0.875rem;
		}

		.form-group label,
		.form-group .group-label {
			font-size: 0.8125rem;
		}

		.form-group input,
		.form-group textarea,
		.form-group select {
			min-height: 44px;
			font-size: 0.9375rem;
			padding: 0.625rem 0.875rem;
		}

		.form-group textarea {
			min-height: 80px;
		}

		.tags-grid {
			gap: 0.375rem;
		}

		.tag-btn {
			min-height: 44px;
			padding: 0.5rem 0.75rem;
			font-size: 0.75rem;
		}

		.form-options {
			flex-direction: column;
			gap: 1rem;
		}

		.checkbox-label {
			min-height: 44px;
			padding: 0.5rem 0;
		}

		.checkbox-label input[type='checkbox'] {
			width: 22px;
			height: 22px;
		}

		.btn-primary,
		.btn-secondary {
			min-height: 44px;
			padding: 0.625rem 1rem;
			font-size: 0.875rem;
		}
	}

	@media (min-width: 640px) {
		.modal-footer {
			flex-direction: row;
			justify-content: flex-end;
		}

		.modal-footer button {
			width: auto;
		}

		.form-options {
			flex-direction: row;
			flex-wrap: wrap;
		}
	}

	@media (min-width: 768px) {
		.modal {
			max-width: 500px;
		}

		.modal.modal-large {
			max-width: 700px;
		}

		.form-row {
			grid-template-columns: 1fr 1fr;
		}
	}
</style>

<script lang="ts">
	import IconX from '@tabler/icons-svelte-runes/icons/x';

	interface NewVideoForm {
		title: string;
		description: string;
		video_url: string;
		embed_url: string;
		thumbnail_url: string;
	}

	interface Props {
		newVideo: NewVideoForm;
		adding: boolean;
		onClose: () => void;
		onAdd: () => void;
	}

	let { newVideo = $bindable(), adding, onClose, onAdd }: Props = $props();
</script>

<div
	class="modal-overlay"
	onclick={onClose}
	onkeydown={(e) => e.key === 'Escape' && onClose()}
	role="button"
	tabindex="-1"
	aria-label="Close modal"
>
	<div
		class="modal"
		onclick={(e) => e.stopPropagation()}
		onkeydown={(e) => e.stopPropagation()}
		role="dialog"
		aria-modal="true"
		tabindex="0"
	>
		<div class="modal-header">
			<h2>Add Tutorial Video</h2>
			<button class="btn-close" onclick={onClose} aria-label="Close modal">
				<IconX size={20} aria-hidden="true" />
			</button>
		</div>
		<div class="modal-body">
			<div class="form-group">
				<label for="video-title">Title *</label>
				<input
					type="text"
					id="video-title"
					bind:value={newVideo.title}
					placeholder="e.g., Getting Started with Squeeze Pro"
				/>
			</div>
			<div class="form-group">
				<label for="video-description">Description</label>
				<textarea
					id="video-description"
					rows="3"
					bind:value={newVideo.description}
					placeholder="Brief description of the video"
				></textarea>
			</div>
			<div class="form-group">
				<label for="video-url">Video URL (Direct)</label>
				<input
					type="url"
					id="video-url"
					bind:value={newVideo.video_url}
					placeholder="https://..."
				/>
			</div>
			<div class="form-group">
				<label for="embed-url">Embed URL (YouTube/Vimeo)</label>
				<input
					type="url"
					id="embed-url"
					bind:value={newVideo.embed_url}
					placeholder="https://www.youtube.com/embed/..."
				/>
			</div>
			<div class="form-group">
				<label for="thumbnail-url">Thumbnail URL</label>
				<input
					type="url"
					id="thumbnail-url"
					bind:value={newVideo.thumbnail_url}
					placeholder="https://..."
				/>
			</div>
		</div>
		<div class="modal-footer">
			<button class="btn-secondary" onclick={onClose}>Cancel</button>
			<button class="btn-primary" onclick={onAdd} disabled={adding}>
				{adding ? 'Adding...' : 'Add Video'}
			</button>
		</div>
	</div>
</div>

<style>
	.modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.5);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
		padding: 20px;
	}
	.modal {
		background: #fff;
		border-radius: 12px;
		max-width: 500px;
		width: 100%;
		max-height: 90vh;
		overflow-y: auto;
	}
	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 20px 24px;
		border-bottom: 1px solid #e5e7eb;
	}
	.modal-header h2 {
		margin: 0;
		font-size: 18px;
		font-weight: 600;
	}
	.modal-body {
		padding: 24px;
		display: flex;
		flex-direction: column;
		gap: 16px;
	}
	.modal-footer {
		display: flex;
		justify-content: flex-end;
		gap: 12px;
		padding: 16px 24px;
		border-top: 1px solid #e5e7eb;
	}

	.form-group {
		display: flex;
		flex-direction: column;
		gap: 6px;
	}
	label {
		font-size: 13px;
		font-weight: 500;
		color: #374151;
	}
	input,
	textarea {
		padding: 10px 12px;
		border: 1px solid #e5e7eb;
		border-radius: 6px;
		font-size: 14px;
	}
	input:focus,
	textarea:focus {
		outline: none;
		border-color: #143e59;
	}
	textarea {
		resize: vertical;
	}

	.btn-primary,
	.btn-secondary {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		padding: 10px 20px;
		border: none;
		border-radius: 8px;
		font-size: 14px;
		font-weight: 500;
		cursor: pointer;
	}
	.btn-primary {
		background: #143e59;
		color: #fff;
	}
	.btn-primary:hover {
		background: #0f2d42;
	}
	.btn-primary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}
	.btn-secondary {
		background: #f3f4f6;
		color: #374151;
		border: 1px solid #e5e7eb;
	}
	.btn-secondary:hover {
		background: #e5e7eb;
	}
	.btn-close {
		background: none;
		border: none;
		font-size: 20px;
		cursor: pointer;
		color: #6b7280;
		padding: 4px 8px;
	}
	.btn-close:hover {
		color: #1f2937;
	}

	@media (max-width: 639px) {
		.modal {
			margin: 10px;
			max-width: calc(100% - 20px);
		}
	}

	@media (hover: none) and (pointer: coarse) {
		.btn-primary,
		.btn-secondary {
			padding: 12px 24px;
			min-height: 48px;
		}

		input,
		textarea {
			font-size: 16px;
			min-height: 44px;
		}
	}
</style>

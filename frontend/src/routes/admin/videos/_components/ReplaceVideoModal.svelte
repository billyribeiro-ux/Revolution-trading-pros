<script lang="ts">
	import { IconLink } from '$lib/icons';

	interface ReplacingVideo {
		title: string;
	}

	interface Props {
		open: boolean;
		video: ReplacingVideo | null;
		newVideoUrl: string;
		isSaving: boolean;
		detectPlatform: (url: string) => string;
		onClose: () => void;
		onReplace: () => void;
	}

	let {
		open,
		video,
		newVideoUrl = $bindable(),
		isSaving,
		detectPlatform,
		onClose,
		onReplace
	}: Props = $props();
</script>

{#if open && video}
	<div
		class="modal-overlay"
		role="button"
		tabindex="0"
		onclick={onClose}
		onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && onClose()}
	>
		<div
			class="modal"
			onclick={(e: MouseEvent) => e.stopPropagation()}
			onkeydown={(e: KeyboardEvent) => e.stopPropagation()}
			role="dialog"
			aria-modal="true"
			tabindex="-1"
		>
			<div class="modal-header">
				<h2>Replace Video</h2>
				<button class="modal-close" onclick={onClose} type="button" aria-label="Close"
					>&times;</button
				>
			</div>
			<div class="modal-body">
				<div class="replace-info">
					<p><strong>Current video:</strong> {video.title}</p>
					<p class="text-muted">
						All metadata (title, description, categories, featured status) will be kept. Only the
						video URL will be replaced.
					</p>
				</div>

				<div class="form-group">
					<label for="new-video-url">New Video URL</label>
					<input
						type="url"
						id="new-video-url"
						name="new-video-url"
						placeholder="https://your-video-url.com/video.mp4"
						bind:value={newVideoUrl}
					/>
					<small class="form-hint">Platform will be auto-detected from URL</small>
				</div>

				{#if newVideoUrl}
					<div class="platform-preview">
						<span>Detected platform: <strong>{detectPlatform(newVideoUrl)}</strong></span>
					</div>
				{/if}
			</div>
			<div class="modal-footer">
				<button class="btn-secondary" onclick={onClose} type="button">Cancel</button>
				<button class="btn-primary" onclick={onReplace} disabled={isSaving || !newVideoUrl}>
					{#if isSaving}
						<span class="btn-spinner"></span>
						Replacing...
					{:else}
						<IconLink size={16} />
						Replace Video
					{/if}
				</button>
			</div>
		</div>
	</div>
{/if}

<style>
	/* Modal scaffold (duplicated from parent — still used by other modals there) */
	.modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.7);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 100;
		padding: 1rem;
	}

	.modal {
		background: var(--bg-elevated);
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 16px;
		width: 100%;
		max-width: 600px;
		max-height: 90vh;
		overflow: auto;
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1.25rem;
		border-bottom: 1px solid rgba(230, 184, 0, 0.1);
	}

	.modal-header h2 {
		font-size: 1.25rem;
		font-weight: 600;
		color: var(--text-primary);
		margin: 0;
	}

	.modal-close {
		width: 32px;
		height: 32px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: transparent;
		border: none;
		color: var(--text-tertiary);
		font-size: 1.5rem;
		cursor: pointer;
		border-radius: 8px;
		transition: all 0.2s;
	}

	.modal-close:hover {
		background: rgba(230, 184, 0, 0.1);
		color: var(--text-primary);
	}

	.modal-body {
		padding: 1.25rem;
	}

	.modal-footer {
		display: flex;
		justify-content: flex-end;
		gap: 0.75rem;
		padding: 1.25rem;
		border-top: 1px solid rgba(230, 184, 0, 0.1);
	}

	/* Form */
	.form-group {
		margin-bottom: 1rem;
	}

	.form-group label {
		display: block;
		font-size: 0.85rem;
		font-weight: 500;
		color: var(--text-secondary);
		margin-bottom: 0.5rem;
	}

	.form-group input {
		width: 100%;
		padding: 0.75rem 1rem;
		background: rgba(13, 17, 23, 0.6);
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 10px;
		color: var(--text-primary);
		font-size: 0.9rem;
	}

	.form-group input:focus {
		outline: none;
		border-color: rgba(230, 184, 0, 0.5);
	}

	.form-hint {
		display: block;
		font-size: 0.75rem;
		color: var(--text-tertiary);
		margin-top: 0.375rem;
	}

	/* Buttons */
	.btn-primary {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background: linear-gradient(135deg, #e6b800 0%, #b38f00 100%);
		border: none;
		border-radius: 10px;
		color: #0d1117;
		font-size: 0.875rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-primary:hover:not(:disabled) {
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(230, 184, 0, 0.3);
	}

	.btn-primary:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.btn-secondary {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background: rgba(100, 116, 139, 0.1);
		border: 1px solid rgba(100, 116, 139, 0.2);
		border-radius: 10px;
		color: var(--text-secondary);
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-secondary:hover {
		background: rgba(100, 116, 139, 0.2);
		color: var(--text-primary);
	}

	.btn-spinner {
		width: 16px;
		height: 16px;
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-top-color: white;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	/* Replace-specific */
	.replace-info {
		background: rgba(230, 184, 0, 0.1);
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 8px;
		padding: 1rem;
		margin-bottom: 1.5rem;
	}

	.replace-info p {
		margin: 0.5rem 0;
		color: var(--text-primary);
	}

	.replace-info strong {
		color: var(--text-primary);
	}

	.text-muted {
		color: var(--text-tertiary);
		font-size: 0.875rem;
	}

	.platform-preview {
		background: rgba(34, 197, 94, 0.1);
		border: 1px solid rgba(34, 197, 94, 0.2);
		border-radius: 6px;
		padding: 0.75rem;
		margin-top: 1rem;
		color: var(--success-emphasis);
		font-size: 0.875rem;
	}

	.platform-preview strong {
		text-transform: uppercase;
	}
</style>

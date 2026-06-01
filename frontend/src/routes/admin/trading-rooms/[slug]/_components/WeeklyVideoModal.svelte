<!--
	WeeklyVideoModal — Publish new weekly video modal.
	Extracted from +page.svelte (R8-C) — 1 $bindable form prop, 2 callback props.
-->
<script lang="ts">
	import IconX from '@tabler/icons-svelte-runes/icons/x';

	export interface WeeklyVideoFormData {
		week_of: string;
		week_title: string;
		video_title: string;
		video_url: string;
		video_platform: string;
		thumbnail_url: string;
		duration: string;
		description: string;
	}

	interface Props {
		open: boolean;
		form: WeeklyVideoFormData;
		isSaving: boolean;
		isValid: boolean;
		onSave: () => void;
		onClose: () => void;
	}

	let { open, form = $bindable(), isSaving, isValid, onSave, onClose }: Props = $props();
</script>

{#if open}
	<div
		class="modal-overlay"
		onclick={onClose}
		onkeydown={(e) => e.key === 'Escape' && onClose()}
		aria-hidden="true"
	>
		<div
			class="modal"
			onclick={(e) => e.stopPropagation()}
			onkeydown={(e: KeyboardEvent) => e.stopPropagation()}
			role="dialog"
			aria-modal="true"
			aria-labelledby="video-modal-title"
			tabindex="-1"
		>
			<div class="modal-header">
				<h2 id="video-modal-title">Publish Weekly Video</h2>
				<button class="close-btn" onclick={onClose}>
					<IconX size={24} />
				</button>
			</div>
			<form
				class="modal-body"
				onsubmit={(e) => {
					e.preventDefault();
					onSave();
				}}
			>
				<div class="form-row">
					<div class="form-group">
						<label for="week_of">Week Of *</label>
						<input id="week_of" name="week_of" type="date" bind:value={form.week_of} />
					</div>
					<div class="form-group">
						<label for="week_title">Week Title *</label>
						<input
							id="week_title"
							name="week_title"
							type="text"
							bind:value={form.week_title}
							placeholder="Week of January 13, 2026"
						/>
					</div>
				</div>

				<div class="form-group full-width">
					<label for="video_title">Video Title *</label>
					<input
						id="video_title"
						name="video_title"
						type="text"
						bind:value={form.video_title}
						placeholder="Weekly Breakdown: Top Swing Setups"
					/>
				</div>

				<div class="form-row">
					<div class="form-group flex-2">
						<label for="video_url">Video URL *</label>
						<input
							id="video_url"
							name="video_url"
							type="url"
							bind:value={form.video_url}
							placeholder="https://player.vimeo.com/video/..."
						/>
					</div>
					<div class="form-group">
						<label for="duration">Duration</label>
						<input
							id="duration"
							name="duration"
							type="text"
							bind:value={form.duration}
							placeholder="24:35"
						/>
					</div>
				</div>

				<div class="form-group full-width">
					<label for="thumbnail_url">Thumbnail URL</label>
					<input
						id="thumbnail_url"
						name="thumbnail_url"
						type="url"
						bind:value={form.thumbnail_url}
						placeholder="https://..."
					/>
				</div>

				<div class="form-group full-width">
					<label for="description">Description</label>
					<textarea
						id="description"
						bind:value={form.description}
						placeholder="This week's breakdown covers..."
						rows="3"
					></textarea>
				</div>

				<div class="info-box">
					<strong>Note:</strong> Publishing a new video will automatically archive the current video.
				</div>

				<div class="modal-actions">
					<button type="button" class="btn-secondary" onclick={onClose}>Cancel</button>
					<button type="submit" class="btn-primary" disabled={isSaving || !isValid}>
						{isSaving ? 'Publishing...' : 'Publish Video'}
					</button>
				</div>
			</form>
		</div>
	</div>
{/if}

<style>
	/* Modal */
	.modal-overlay {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background: rgba(0, 0, 0, 0.5);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: var(--z-modal, 400);
		padding: var(--space-2, 20px);
		isolation: isolate;
	}

	.modal {
		background: #fff;
		border-radius: 16px;
		width: 100%;
		max-width: 640px;
		max-height: 90vh;
		overflow-y: auto;
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 24px;
		border-bottom: 1px solid #e2e8f0;
	}

	.modal-header h2 {
		font-size: 20px;
		font-weight: 700;
		color: #1e293b;
		margin: 0;
	}

	.close-btn {
		background: none;
		border: none;
		color: #64748b;
		cursor: pointer;
		padding: 4px;
	}

	.close-btn:hover {
		color: #1e293b;
	}

	.modal-body {
		padding: 24px;
	}

	.form-row {
		display: flex;
		gap: 16px;
		margin-bottom: 20px;
	}

	.form-group {
		flex: 1;
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.form-group.full-width {
		margin-bottom: 20px;
	}

	.form-group.flex-2 {
		flex: 2;
	}

	.form-group label {
		font-size: 13px;
		font-weight: 600;
		color: #475569;
	}

	.form-group input,
	.form-group textarea {
		padding: 12px 14px;
		border: 1.5px solid #e2e8f0;
		border-radius: 10px;
		font-size: 14px;
		transition: border-color 0.2s;
	}

	.form-group input:focus,
	.form-group textarea:focus {
		outline: none;
		border-color: #143e59;
	}

	.form-group textarea {
		resize: vertical;
	}

	.info-box {
		background: #f0f9ff;
		border: 1px solid #bae6fd;
		border-radius: 10px;
		padding: 14px 18px;
		font-size: 14px;
		color: #0369a1;
		margin-bottom: 20px;
	}

	.modal-actions {
		display: flex;
		justify-content: flex-end;
		gap: 12px;
		padding-top: 20px;
		border-top: 1px solid #e2e8f0;
	}

	/* Buttons */
	.btn-primary {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		padding: 12px 24px;
		background: #143e59;
		color: #fff;
		border: none;
		border-radius: 10px;
		font-size: 14px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-primary:hover {
		background: #0f2d42;
	}

	.btn-primary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.btn-secondary {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		padding: 12px 24px;
		background: #f1f5f9;
		color: #475569;
		border: none;
		border-radius: 10px;
		font-size: 14px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-secondary:hover {
		background: #e2e8f0;
	}

	@media (max-width: 767.98px) {
		.form-row {
			flex-direction: column;
		}
	}
</style>

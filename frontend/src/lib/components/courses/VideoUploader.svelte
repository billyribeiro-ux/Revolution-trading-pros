<script lang="ts">
	/**
	 * TUS Video Uploader for Bunny.net Stream
	 * Apple Principal Engineer ICT 7 Grade - January 2026
	 * Resumable uploads with progress tracking
	 */

	import { onMount } from 'svelte';
	import { browser } from '$app/environment';

	// Dynamic import for tus-js-client to avoid SSR issues
	// Type definition for tus module
	type TusModule = typeof import('tus-js-client');
	type TusUpload = import('tus-js-client').Upload;

	let tus = $state<TusModule | null>(null);

	onMount(async () => {
		if (browser) {
			tus = await import('tus-js-client');
		}
	});

	interface Props {
		courseId: string;
		lessonTitle?: string;
		onUploadComplete?: (data: VideoUploadResult) => void;
		onError?: (error: string) => void;
	}

	interface VideoUploadResult {
		videoGuid: string;
		libraryId: string;
		embedUrl: string;
		playUrl: string;
		thumbnailUrl: string;
	}

	let props: Props = $props();

	// Props with defaults
	const lessonTitle = $derived(props.lessonTitle ?? 'Video');

	let file = $state<File | null>(null);
	// @ts-ignore write-only state
	let uploading = $state(false);
	let progress = $state(0);
	let status = $state<'idle' | 'preparing' | 'uploading' | 'processing' | 'complete' | 'error'>(
		'idle'
	);
	let errorMessage = $state('');
	let uploadInstance = $state<TusUpload | null>(null);

	const handleFileSelect = (event: Event) => {
		const input = event.target as HTMLInputElement;
		const selected = input.files?.[0];

		if (!selected) return;

		if (!selected.type.startsWith('video/')) {
			errorMessage = 'Please select a video file';
			status = 'error';
			return;
		}

		// Max 10GB
		if (selected.size > 10 * 1024 * 1024 * 1024) {
			errorMessage = 'File size must be less than 10GB';
			status = 'error';
			return;
		}

		file = selected;
		errorMessage = '';
		status = 'idle';
	};

	const startUpload = async () => {
		if (!file) return;

		status = 'preparing';
		uploading = true;
		progress = 0;

		try {
			// Get TUS upload URL from backend
			const response = await fetch(`/api/admin/courses/${props.courseId}/video-upload`, {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ title: lessonTitle || file.name })
			});

			const data = await response.json();

			if (!data.success) {
				throw new Error(data.error || 'Failed to create video');
			}

			const {
				tus_upload_url,
				auth_signature,
				auth_expiry,
				video_guid,
				library_id,
				embed_url,
				play_url,
				thumbnail_url
			} = data.data;

			status = 'uploading';

			// Ensure tus module is loaded
			if (!tus) {
				throw new Error('Upload library not initialized. Please wait and try again.');
			}

			// Create TUS upload
			const upload = new tus.Upload(file, {
				endpoint: tus_upload_url,
				retryDelays: [0, 1000, 3000, 5000, 10000],
				chunkSize: 5 * 1024 * 1024, // 5MB chunks
				metadata: {
					filename: file.name,
					filetype: file.type,
					title: lessonTitle || file.name
				},
				headers: {
					AuthorizationSignature: auth_signature,
					AuthorizationExpire: auth_expiry.toString(),
					VideoId: video_guid,
					LibraryId: library_id
				},
				onError: (error: Error) => {
					console.error('TUS upload error:', error);
					errorMessage = error.message || 'Upload failed';
					status = 'error';
					uploading = false;
					props.onError?.(errorMessage);
				},
				onProgress: (bytesUploaded: number, bytesTotal: number) => {
					progress = Math.round((bytesUploaded / bytesTotal) * 100);
				},
				onSuccess: () => {
					status = 'processing';
					progress = 100;

					// Poll for encoding status
					pollEncodingStatus(video_guid, library_id, {
						videoGuid: video_guid,
						libraryId: library_id,
						embedUrl: embed_url,
						playUrl: play_url,
						thumbnailUrl: thumbnail_url
					});
				}
			});

			uploadInstance = upload;
			upload.start();
		} catch (e) {
			const msg = e instanceof Error ? e.message : 'Upload failed';
			errorMessage = msg;
			status = 'error';
			uploading = false;
			props.onError?.(msg);
		}
	};

	const pollEncodingStatus = async (
		_videoGuid: string,
		_libraryId: string,
		result: VideoUploadResult
	) => {
		// For now, just mark as complete after upload
		// In production, poll Bunny API for encoding status
		setTimeout(() => {
			status = 'complete';
			uploading = false;
			props.onUploadComplete?.(result);
		}, 2000);
	};

	const cancelUpload = () => {
		if (uploadInstance) {
			uploadInstance.abort();
			uploadInstance = null;
		}
		uploading = false;
		progress = 0;
		status = 'idle';
		file = null;
	};

	const formatSize = (bytes: number): string => {
		if (bytes < 1024) return `${bytes} B`;
		if (bytes < 1048576) return `${(bytes / 1024).toFixed(1)} KB`;
		if (bytes < 1073741824) return `${(bytes / 1048576).toFixed(1)} MB`;
		return `${(bytes / 1073741824).toFixed(2)} GB`;
	};
</script>

<div class="video-uploader">
	{#if status === 'idle' || status === 'error'}
		<div class="upload-zone" class:has-file={file} class:error={status === 'error'}>
			{#if !file}
				<input type="file" accept="video/*" onchange={handleFileSelect} id="video-input" />
				<label for="video-input">
					<svg
						xmlns="http://www.w3.org/2000/svg"
						width="48"
						height="48"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="1.5"
					>
						<path d="m16 13 5.223 3.482a.5.5 0 0 0 .777-.416V7.87a.5.5 0 0 0-.752-.432L16 10.5" />
						<rect x="2" y="6" width="14" height="12" rx="2" />
					</svg>
					<span class="label">Drop video here or click to browse</span>
					<span class="hint">MP4, MOV, WebM up to 10GB</span>
				</label>
			{:else}
				<div class="file-info">
					<svg
						xmlns="http://www.w3.org/2000/svg"
						width="32"
						height="32"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
					>
						<path d="m16 13 5.223 3.482a.5.5 0 0 0 .777-.416V7.87a.5.5 0 0 0-.752-.432L16 10.5" />
						<rect x="2" y="6" width="14" height="12" rx="2" />
					</svg>
					<div class="file-details">
						<span class="file-name">{file.name}</span>
						<span class="file-size">{formatSize(file.size)}</span>
					</div>
					<button
						class="btn-remove"
						onclick={() => {
							file = null;
							status = 'idle';
						}}
						aria-label="Remove file"
					>
						<svg
							xmlns="http://www.w3.org/2000/svg"
							width="20"
							height="20"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
						>
							<line x1="18" x2="6" y1="6" y2="18" /><line x1="6" x2="18" y1="6" y2="18" />
						</svg>
					</button>
				</div>
				<button class="btn-upload" onclick={startUpload} disabled={!tus}>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						width="20"
						height="20"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
					>
						<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" /><polyline
							points="17 8 12 3 7 8"
						/><line x1="12" x2="12" y1="3" y2="15" />
					</svg>
					{tus ? 'Upload Video' : 'Loading...'}
				</button>
			{/if}
		</div>
		{#if status === 'error'}
			<p class="error-message">{errorMessage}</p>
		{/if}
	{:else if status === 'preparing'}
		<div class="status-panel">
			<div class="spinner"></div>
			<p>Preparing upload...</p>
		</div>
	{:else if status === 'uploading'}
		<div class="progress-panel">
			<div class="progress-header">
				<span>Uploading {file?.name}</span>
				<span>{progress}%</span>
			</div>
			<div class="progress-bar">
				<div class="progress-fill" style="width: {progress}%"></div>
			</div>
			<div class="progress-footer">
				<span
					>{formatSize(((file?.size || 0) * progress) / 100)} / {formatSize(file?.size || 0)}</span
				>
				<button class="btn-cancel" onclick={cancelUpload}>Cancel</button>
			</div>
		</div>
	{:else if status === 'processing'}
		<div class="status-panel processing">
			<div class="spinner"></div>
			<p>Processing video...</p>
			<span class="hint">This may take a few minutes</span>
		</div>
	{:else if status === 'complete'}
		<div class="status-panel complete">
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="48"
				height="48"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
			>
				<path d="M22 11.08V12a10 10 0 1 1-5.93-9.14" /><polyline points="22 4 12 14.01 9 11.01" />
			</svg>
			<p>Video uploaded successfully!</p>
			<button
				class="btn-new"
				onclick={() => {
					file = null;
					status = 'idle';
				}}>Upload Another</button
			>
		</div>
	{/if}
</div>

<style>
	.video-uploader {
		width: 100%;
	}

	.upload-zone {
		border: 2px dashed #d1d5db;
		border-radius: 12px;
		padding: 32px;
		text-align: center;
		transition: all 0.2s;
		background: #f9fafb;
	}

	.upload-zone:hover {
		border-color: #143e59;
		background: #f3f4f6;
	}
	.upload-zone.has-file {
		border-style: solid;
		border-color: #143e59;
	}
	.upload-zone.error {
		border-color: #dc2626;
		background: #fef2f2;
	}

	.upload-zone input[type='file'] {
		display: none;
	}

	.upload-zone label {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 12px;
		cursor: pointer;
		color: #6b7280;
	}

	.upload-zone label svg {
		color: #9ca3af;
	}
	.upload-zone label .label {
		font-size: 16px;
		font-weight: 500;
		color: #374151;
	}
	.upload-zone label .hint {
		font-size: 13px;
		color: #9ca3af;
	}

	.file-info {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 16px;
		background: #fff;
		border-radius: 8px;
		margin-bottom: 16px;
	}

	.file-info svg {
		color: #143e59;
		flex-shrink: 0;
	}
	.file-details {
		flex: 1;
		text-align: left;
	}
	.file-name {
		display: block;
		font-weight: 500;
		color: #1f2937;
		word-break: break-all;
	}
	.file-size {
		font-size: 13px;
		color: #6b7280;
	}

	.btn-remove {
		background: none;
		border: none;
		color: #9ca3af;
		cursor: pointer;
		padding: 4px;
	}
	.btn-remove:hover {
		color: #dc2626;
	}

	.btn-upload {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		padding: 12px 24px;
		background: #143e59;
		color: #fff;
		border: none;
		border-radius: 8px;
		font-size: 14px;
		font-weight: 500;
		cursor: pointer;
		transition: background 0.2s;
	}
	.btn-upload:hover {
		background: #0f2d42;
	}
	.btn-upload:disabled {
		background: #9ca3af;
		cursor: not-allowed;
		opacity: 0.6;
	}

	.error-message {
		color: #dc2626;
		font-size: 14px;
		margin-top: 12px;
		text-align: center;
	}

	.status-panel,
	.progress-panel {
		padding: 32px;
		background: #f9fafb;
		border-radius: 12px;
		text-align: center;
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

	.status-panel p {
		font-size: 16px;
		font-weight: 500;
		color: #1f2937;
		margin: 0 0 8px;
	}
	.status-panel .hint {
		font-size: 13px;
		color: #9ca3af;
	}
	.status-panel.complete svg {
		color: #10b981;
		margin-bottom: 16px;
	}

	.progress-header,
	.progress-footer {
		display: flex;
		justify-content: space-between;
		font-size: 14px;
		color: #6b7280;
	}

	.progress-header {
		margin-bottom: 8px;
	}
	.progress-footer {
		margin-top: 12px;
	}

	.progress-bar {
		height: 8px;
		background: #e5e7eb;
		border-radius: 4px;
		overflow: hidden;
	}

	.progress-fill {
		height: 100%;
		background: #143e59;
		border-radius: 4px;
		transition: width 0.3s;
	}

	.btn-cancel {
		background: none;
		border: none;
		color: #dc2626;
		font-size: 14px;
		cursor: pointer;
	}
	.btn-cancel:hover {
		text-decoration: underline;
	}

	.btn-new {
		margin-top: 16px;
		padding: 10px 20px;
		background: #f3f4f6;
		border: none;
		border-radius: 6px;
		font-size: 14px;
		cursor: pointer;
	}
	.btn-new:hover {
		background: #e5e7eb;
	}
</style>

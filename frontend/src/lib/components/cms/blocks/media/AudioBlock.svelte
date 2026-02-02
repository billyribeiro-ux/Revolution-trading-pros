<!--
/**
 * Audio Block Component
 * ═══════════════════════════════════════════════════════════════════════════
 * Audio player with full controls - Apple Principal Engineer ICT 7 Standard
 */
-->

<script lang="ts">
	import {
		IconPlayerPlay,
		IconPlayerPause,
		IconVolume,
		IconVolumeOff,
		IconWaveSine,
		IconFile,
		IconAlertCircle
	} from '$lib/icons';
	import { useMediaControls } from '../hooks/useMediaControls.svelte';
	import { sanitizeURL } from '$lib/utils/sanitization';
	import type { Block, BlockContent } from '../types';
	import type { BlockId } from '$lib/stores/blockState.svelte';

	// ==========================================================================
	// Props
	// ==========================================================================

	interface Props {
		block: Block;
		blockId: BlockId;
		isSelected: boolean;
		isEditing: boolean;
		onUpdate: (updates: Partial<Block>) => void;
		onError?: (error: Error) => void;
	}

	const props: Props = $props();

	// ==========================================================================
	// Media Controls Hook
	// ==========================================================================

	const controls = useMediaControls({
		blockId: props.blockId,
		type: 'audio',
		onError: props.onError,
		onEnded: () => {
			console.log('Audio playback ended');
		}
	});

	// ==========================================================================
	// Content Updates
	// ==========================================================================

	function updateContent(updates: Partial<BlockContent>): void {
		props.onUpdate({
			content: { ...props.block.content, ...updates }
		});
	}

	function handleCaptionInput(e: Event): void {
		const target = e.target as HTMLElement;
		updateContent({ mediaCaption: target.textContent || '' });
	}

	function handlePaste(e: ClipboardEvent): void {
		e.preventDefault();
		const text = e.clipboardData?.getData('text/plain') || '';
		document.execCommand('insertText', false, text);
	}

	function handleURLChange(e: Event): void {
		const value = (e.target as HTMLInputElement).value;
		const sanitized = sanitizeURL(value);
		if (sanitized) {
			updateContent({ mediaUrl: sanitized });
		}
	}

	function handleFileUpload(e: Event): void {
		const file = (e.target as HTMLInputElement).files?.[0];
		if (!file) return;

		// Validate file type
		if (!file.type.startsWith('audio/')) {
			props.onError?.(new Error('Invalid file type. Please upload an audio file.'));
			return;
		}

		// Validate file size (10MB max)
		const maxSize = 10 * 1024 * 1024;
		if (file.size > maxSize) {
			props.onError?.(new Error('File too large. Maximum size: 10MB'));
			return;
		}

		const url = URL.createObjectURL(file);
		updateContent({ mediaUrl: url });
	}

	// ==========================================================================
	// Derived State
	// ==========================================================================

	const hasAudio = $derived(!!props.block.content.mediaUrl);
	const sanitizedURL = $derived(
		props.block.content.mediaUrl ? sanitizeURL(props.block.content.mediaUrl) : ''
	);
</script>

<div class="audio-block" role="region" aria-label="Audio player">
	{#if hasAudio && sanitizedURL}
		<!-- Audio Element (hidden) -->
		<audio
			bind:this={controls.elementRef}
			src={sanitizedURL}
			ontimeupdate={controls.handleTimeUpdate}
			onended={controls.handleEnded}
			onerror={controls.handleError}
			onloadedmetadata={controls.handleLoadedMetadata}
			preload="metadata"
			aria-hidden="true"
		>
			<track kind="captions" label="Captions" />
		</audio>

		<!-- Player UI -->
		<div class="audio-player">
			<!-- Play/Pause Button -->
			<button
				type="button"
				class="audio-play-btn"
				onclick={controls.togglePlay}
				aria-label={controls.playing ? 'Pause audio' : 'Play audio'}
				aria-pressed={controls.playing}
			>
				{#if controls.playing}
					<IconPlayerPause size={24} aria-hidden="true" />
				{:else}
					<IconPlayerPlay size={24} aria-hidden="true" />
				{/if}
			</button>

			<!-- Waveform Icon (decorative) -->
			<div class="audio-waveform" aria-hidden="true">
				<IconWaveSine size={20} />
			</div>

			<!-- Progress Bar -->
			<div
				class="audio-progress-container"
				onclick={controls.handleSeekClick}
				onkeydown={(e) => {
					if (e.key === 'ArrowLeft') {
						controls.seek(Math.max(0, controls.progress - 5));
					} else if (e.key === 'ArrowRight') {
						controls.seek(Math.min(100, controls.progress + 5));
					}
				}}
				role="slider"
				aria-label="Audio progress"
				aria-valuemin={0}
				aria-valuemax={100}
				aria-valuenow={Math.round(controls.progress)}
				aria-valuetext="{controls.formatTime(controls.currentTime)} of {controls.formatTime(controls.duration)}"
				tabindex="0"
			>
				<div class="audio-progress-bar" style="width: {controls.progress}%"></div>
			</div>

			<!-- Time Display -->
			<div class="audio-time" aria-live="off">
				<time datetime="PT{Math.floor(controls.currentTime)}S">
					{controls.formatTime(controls.currentTime)}
				</time>
				<span aria-hidden="true">/</span>
				<time datetime="PT{Math.floor(controls.duration)}S">
					{controls.formatTime(controls.duration)}
				</time>
			</div>

			<!-- Volume Controls -->
			<div class="audio-volume-container">
				<button
					type="button"
					class="audio-mute-btn"
					onclick={controls.toggleMute}
					aria-label={controls.muted ? 'Unmute audio' : 'Mute audio'}
					aria-pressed={controls.muted}
				>
					{#if controls.muted}
						<IconVolumeOff size={20} aria-hidden="true" />
					{:else}
						<IconVolume size={20} aria-hidden="true" />
					{/if}
				</button>
				<input
					type="range"
					min="0"
					max="1"
					step="0.1"
					value={controls.volume}
					oninput={controls.handleVolumeChange}
					class="audio-volume-slider"
					aria-label="Volume"
				/>
			</div>
		</div>

		<!-- Caption -->
		{#if props.block.content.mediaCaption || props.isEditing}
			<p
				contenteditable={props.isEditing}
				class="audio-caption editable-content"
				class:placeholder={!props.block.content.mediaCaption}
				oninput={handleCaptionInput}
				onpaste={handlePaste}
				data-placeholder="Add a caption..."
				role="textbox"
				aria-label="Audio caption"
				aria-multiline="false"
			>
				{props.block.content.mediaCaption || ''}
			</p>
		{/if}
	{:else if props.isEditing}
		<!-- Placeholder (Edit Mode) -->
		<div class="audio-placeholder">
			<IconWaveSine size={48} aria-hidden="true" />
			<span>Add audio URL</span>

			<input
				type="url"
				placeholder="https://example.com/audio.mp3"
				onchange={handleURLChange}
				aria-label="Audio URL"
			/>

			<span class="audio-upload-divider">or</span>

			<label class="audio-file-upload">
				<IconFile size={20} aria-hidden="true" />
				<span>Upload audio file</span>
				<input
					type="file"
					accept="audio/*"
					onchange={handleFileUpload}
					aria-label="Upload audio file"
				/>
			</label>
		</div>
	{:else}
		<!-- No audio in view mode -->
		<div class="audio-empty" role="status">
			<IconAlertCircle size={24} aria-hidden="true" />
			<span>No audio available</span>
		</div>
	{/if}
</div>

<style>
	.audio-block {
		width: 100%;
	}

	/* Player */
	.audio-player {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem;
		background: linear-gradient(135deg, #1e293b 0%, #0f172a 100%);
		border-radius: 12px;
	}

	.audio-play-btn {
		width: 44px;
		height: 44px;
		min-width: 44px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: #3b82f6;
		border: none;
		border-radius: 50%;
		color: white;
		cursor: pointer;
		flex-shrink: 0;
		transition: all 0.15s;
	}

	.audio-play-btn:hover {
		background: #2563eb;
		transform: scale(1.05);
	}

	.audio-play-btn:active {
		transform: scale(0.98);
	}

	.audio-play-btn:focus-visible {
		outline: 2px solid #60a5fa;
		outline-offset: 2px;
	}

	.audio-waveform {
		color: #64748b;
		flex-shrink: 0;
	}

	.audio-progress-container {
		flex: 1;
		height: 8px;
		background: rgba(255, 255, 255, 0.1);
		border-radius: 4px;
		cursor: pointer;
		overflow: hidden;
		position: relative;
	}

	.audio-progress-container:focus-visible {
		outline: 2px solid #60a5fa;
		outline-offset: 2px;
	}

	.audio-progress-bar {
		height: 100%;
		background: linear-gradient(90deg, #3b82f6, #60a5fa);
		border-radius: 4px;
		transition: width 0.1s linear;
		pointer-events: none;
	}

	.audio-time {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		font-size: 0.75rem;
		color: #94a3b8;
		font-variant-numeric: tabular-nums;
		min-width: 70px;
		flex-shrink: 0;
	}

	.audio-volume-container {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		flex-shrink: 0;
	}

	.audio-mute-btn {
		width: 36px;
		height: 36px;
		min-width: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: transparent;
		border: none;
		color: #94a3b8;
		cursor: pointer;
		border-radius: 6px;
		transition: all 0.15s;
	}

	.audio-mute-btn:hover {
		color: white;
		background: rgba(255, 255, 255, 0.1);
	}

	.audio-mute-btn:focus-visible {
		outline: 2px solid #60a5fa;
		outline-offset: 2px;
	}

	.audio-volume-slider {
		width: 80px;
		height: 4px;
		appearance: none;
		background: rgba(255, 255, 255, 0.1);
		border-radius: 2px;
		cursor: pointer;
		outline: none;
	}

	.audio-volume-slider::-webkit-slider-thumb {
		appearance: none;
		width: 14px;
		height: 14px;
		background: white;
		border-radius: 50%;
		cursor: pointer;
		transition: transform 0.15s;
	}

	.audio-volume-slider::-webkit-slider-thumb:hover {
		transform: scale(1.2);
	}

	.audio-volume-slider::-moz-range-thumb {
		width: 14px;
		height: 14px;
		background: white;
		border: none;
		border-radius: 50%;
		cursor: pointer;
		transition: transform 0.15s;
	}

	.audio-volume-slider::-moz-range-thumb:hover {
		transform: scale(1.2);
	}

	/* Caption */
	.audio-caption {
		margin: 0.75rem 0 0;
		font-size: 0.875rem;
		color: #6b7280;
		line-height: 1.5;
		outline: none;
	}

	.audio-caption:focus {
		color: #374151;
	}

	.audio-caption.placeholder:empty::before {
		content: attr(data-placeholder);
		color: #9ca3af;
	}

	/* Placeholder */
	.audio-placeholder {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 0.75rem;
		padding: 3rem;
		background: #f9fafb;
		border: 2px dashed #d1d5db;
		border-radius: 12px;
		color: #9ca3af;
	}

	.audio-placeholder input[type='url'] {
		width: 100%;
		max-width: 400px;
		padding: 0.75rem 1rem;
		border: 1px solid #d1d5db;
		border-radius: 8px;
		font-size: 0.875rem;
		transition: border-color 0.15s;
	}

	.audio-placeholder input[type='url']:focus {
		outline: none;
		border-color: #3b82f6;
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
	}

	.audio-upload-divider {
		color: #9ca3af;
		font-size: 0.875rem;
	}

	.audio-file-upload {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background: white;
		border: 1px solid #d1d5db;
		border-radius: 8px;
		color: #374151;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.15s;
		position: relative;
		overflow: hidden;
	}

	.audio-file-upload:hover {
		background: #f9fafb;
		border-color: #9ca3af;
	}

	.audio-file-upload input[type='file'] {
		position: absolute;
		inset: 0;
		opacity: 0;
		cursor: pointer;
	}

	/* Empty State */
	.audio-empty {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.5rem;
		padding: 2rem;
		background: #f9fafb;
		border: 1px solid #e5e7eb;
		border-radius: 12px;
		color: #6b7280;
	}

	/* Responsive */
	@media (max-width: 768px) {
		.audio-player {
			flex-wrap: wrap;
		}

		.audio-progress-container {
			order: 5;
			width: 100%;
			flex: none;
			margin-top: 0.5rem;
		}

		.audio-volume-container {
			display: none;
		}
	}

	/* Dark Mode */
	:global(.dark) .audio-caption {
		color: #94a3b8;
	}

	:global(.dark) .audio-caption:focus {
		color: #e2e8f0;
	}

	:global(.dark) .audio-placeholder,
	:global(.dark) .audio-empty {
		background: #1e293b;
		border-color: #334155;
		color: #94a3b8;
	}

	:global(.dark) .audio-placeholder input[type='url'] {
		background: #0f172a;
		border-color: #334155;
		color: #e2e8f0;
	}

	:global(.dark) .audio-file-upload {
		background: #1e293b;
		border-color: #334155;
		color: #e2e8f0;
	}

	:global(.dark) .audio-file-upload:hover {
		background: #334155;
	}
</style>

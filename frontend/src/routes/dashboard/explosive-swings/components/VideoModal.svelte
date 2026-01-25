<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * VideoModal Component - Popup Video Player
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @description Full-screen modal for video playback with backdrop click to close
	 * @version 1.0.0
	 * @standards Apple Principal Engineer ICT 7+ Standards
	 */
	import type { Video } from '../types';

	interface Props {
		video: Video | null;
		isOpen: boolean;
		onClose: () => void;
	}

	const { video, isOpen, onClose }: Props = $props();

	function handleBackdropClick(e: MouseEvent) {
		if (e.target === e.currentTarget) {
			onClose();
		}
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') {
			onClose();
		}
	}

	// Bunny.net embed URL only
	const embedUrl = $derived(() => {
		if (!video?.videoUrl) return '';
		return video.videoUrl;
	});
</script>

<svelte:window onkeydown={handleKeydown} />

{#if isOpen && video}
	<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
	<div 
		class="modal-backdrop" 
		onclick={handleBackdropClick}
		onkeydown={(e) => e.key === 'Escape' && onClose()}
		role="dialog"
		aria-modal="true"
		aria-labelledby="video-modal-title"
		tabindex="-1"
	>
		<div class="modal-container">
			<!-- Close Button -->
			<button class="close-btn" onclick={onClose} aria-label="Close video">
				<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
				</svg>
			</button>

			<!-- Video Content -->
			<div class="video-wrapper">
				<div class="video-frame">
					<iframe
						src={embedUrl()}
						title={video.title}
						frameborder="0"
						allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
						allowfullscreen
					></iframe>
				</div>
			</div>

			<!-- Video Info -->
			<div class="video-info">
				<div class="video-meta">
					{#if video.ticker}
						<span class="ticker-badge">{video.ticker}</span>
					{/if}
					{#if video.type}
						<span class="type-badge type-{video.type.toLowerCase()}">{video.type}</span>
					{/if}
				</div>
				<h2 id="video-modal-title" class="video-title">{video.title}</h2>
				<div class="video-details">
					<span class="duration">
						<svg viewBox="0 0 20 20" fill="currentColor">
							<path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm.75-13a.75.75 0 00-1.5 0v5c0 .414.336.75.75.75h4a.75.75 0 000-1.5h-3.25V5z" clip-rule="evenodd" />
						</svg>
						{video.duration}
					</span>
					<span class="date">
						{video.publishedAt.toLocaleDateString('en-US', { 
							month: 'short', 
							day: 'numeric', 
							year: 'numeric' 
						})}
					</span>
				</div>
			</div>
		</div>
	</div>
{/if}

<style>
	/* ═══════════════════════════════════════════════════════════════════════
	   BACKDROP
	   ═══════════════════════════════════════════════════════════════════════ */
	.modal-backdrop {
		position: fixed;
		inset: 0;
		z-index: 9999;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 24px;
		background: rgba(0, 0, 0, 0.85);
		backdrop-filter: blur(8px);
		animation: fadeIn 0.2s ease-out;
	}

	@keyframes fadeIn {
		from { opacity: 0; }
		to { opacity: 1; }
	}

	/* ═══════════════════════════════════════════════════════════════════════
	   CONTAINER
	   ═══════════════════════════════════════════════════════════════════════ */
	.modal-container {
		position: relative;
		width: 100%;
		max-width: 1100px;
		background: #0f172a;
		border-radius: 16px;
		overflow: hidden;
		box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5);
		animation: scaleIn 0.25s ease-out;
	}

	@keyframes scaleIn {
		from { 
			opacity: 0;
			transform: scale(0.95);
		}
		to { 
			opacity: 1;
			transform: scale(1);
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════
	   CLOSE BUTTON
	   ═══════════════════════════════════════════════════════════════════════ */
	.close-btn {
		position: absolute;
		top: 16px;
		right: 16px;
		z-index: 10;
		width: 40px;
		height: 40px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(255, 255, 255, 0.1);
		border: none;
		border-radius: 50%;
		color: #fff;
		cursor: pointer;
		transition: all 0.2s ease-out;
	}

	.close-btn:hover {
		background: rgba(255, 255, 255, 0.2);
		transform: scale(1.1);
	}

	.close-btn svg {
		width: 20px;
		height: 20px;
	}

	/* ═══════════════════════════════════════════════════════════════════════
	   VIDEO WRAPPER
	   ═══════════════════════════════════════════════════════════════════════ */
	.video-wrapper {
		width: 100%;
		background: #000;
	}

	.video-frame {
		position: relative;
		width: 100%;
		padding-bottom: 56.25%; /* 16:9 aspect ratio */
	}

	.video-frame iframe {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		border: none;
	}

	/* ═══════════════════════════════════════════════════════════════════════
	   VIDEO INFO
	   ═══════════════════════════════════════════════════════════════════════ */
	.video-info {
		padding: 20px 24px;
		background: linear-gradient(to bottom, #1e293b, #0f172a);
	}

	.video-meta {
		display: flex;
		align-items: center;
		gap: 8px;
		margin-bottom: 12px;
	}

	.ticker-badge {
		padding: 4px 10px;
		font-size: 12px;
		font-weight: 700;
		color: #fff;
		background: #143e59;
		border-radius: 6px;
		letter-spacing: 0.5px;
	}

	.type-badge {
		padding: 4px 10px;
		font-size: 11px;
		font-weight: 600;
		color: #fff;
		border-radius: 6px;
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.type-badge.type-entry { background: #0d9488; }
	.type-badge.type-exit { background: #059669; }
	.type-badge.type-update { background: #d97706; }
	.type-badge.type-breakdown { background: #475569; }

	.video-title {
		font-size: 18px;
		font-weight: 600;
		color: #fff;
		margin: 0 0 12px 0;
		line-height: 1.4;
	}

	.video-details {
		display: flex;
		align-items: center;
		gap: 16px;
		color: #94a3b8;
		font-size: 14px;
	}

	.duration {
		display: flex;
		align-items: center;
		gap: 6px;
	}

	.duration svg {
		width: 16px;
		height: 16px;
	}

	/* ═══════════════════════════════════════════════════════════════════════
	   RESPONSIVE
	   ═══════════════════════════════════════════════════════════════════════ */
	@media (max-width: 768px) {
		.modal-backdrop {
			padding: 16px;
		}

		.modal-container {
			border-radius: 12px;
		}

		.video-info {
			padding: 16px;
		}

		.video-title {
			font-size: 16px;
		}

		.close-btn {
			top: 12px;
			right: 12px;
			width: 36px;
			height: 36px;
		}
	}

	@media (max-width: 480px) {
		.modal-backdrop {
			padding: 8px;
		}

		.video-info {
			padding: 12px;
		}

		.video-details {
			flex-direction: column;
			align-items: flex-start;
			gap: 4px;
		}
	}
</style>

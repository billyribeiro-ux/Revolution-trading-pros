<script lang="ts">
	/**
	 * VideoInfo Component - Apple Principal Engineer ICT Level 7
	 * Extracted from WeeklyHero.svelte for better maintainability
	 * 
	 * @description Video information panel with title, date, and action buttons
	 * @version 1.0.0 - ICT 7 compliance
	 */

	interface Props {
		videoTitle: string;
		publishedDate: string;
		watchFullUrl: string;
		isAdmin?: boolean;
		onUploadVideo?: () => void;
	}

	const {
		videoTitle,
		publishedDate,
		watchFullUrl,
		isAdmin = false,
		onUploadVideo
	}: Props = $props();
</script>

<div class="video-info">
	<h2>{videoTitle}</h2>
	<p class="published-date">{publishedDate}</p>
	<div class="video-actions">
		<a href={watchFullUrl} class="watch-btn">
			<svg viewBox="0 0 24 24" fill="currentColor" width="18" height="18" aria-hidden="true">
				<path d="M8 5v14l11-7z" />
			</svg>
			Watch Full Video
		</a>
		{#if isAdmin && onUploadVideo}
			<button class="admin-upload-btn" onclick={onUploadVideo} type="button">
				<svg
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
					width="18"
					height="18"
					aria-hidden="true"
				>
					<path d="M21 15v4a2 2 0 01-2 2H5a2 2 0 01-2-2v-4M17 8l-5-5-5 5M12 3v12" />
				</svg>
				Upload Video
			</button>
		{/if}
	</div>
</div>

<style>
	.video-info {
		flex: 1;
		min-width: 280px;
		color: var(--color-bg-card);
		text-align: center;
	}

	.video-info h2 {
		font-family:
			'Montserrat',
			-apple-system,
			BlinkMacSystemFont,
			sans-serif;
		font-size: 28px;
		font-weight: 700;
		line-height: 1.2;
		color: var(--color-bg-card);
		margin: 0 0 8px 0;
		text-wrap: balance; /* stylelint-disable-line property-no-unknown */
	}

	.video-info .published-date {
		font-size: 14px;
		font-weight: 500;
		color: rgba(255, 255, 255, 0.8);
		margin: 0 0 24px 0;
	}

	.video-actions {
		display: flex;
		gap: 12px;
		flex-wrap: wrap;
		justify-content: center;
	}

	/* Primary Button - Watch Full Video */
	.watch-btn {
		display: inline-flex;
		align-items: center;
		gap: 10px;
		padding: 14px 28px;
		background: var(--color-brand-primary);
		color: var(--color-bg-card);
		border: 2px solid var(--color-brand-primary);
		border-radius: 10px;
		font-size: 15px;
		font-weight: 700;
		text-decoration: none;
		transition: all 0.2s ease;
	}

	.watch-btn:hover {
		background: var(--color-brand-primary-hover);
		border-color: var(--color-brand-primary-hover);
		transform: translateY(-2px);
		box-shadow: 0 8px 24px rgba(20, 62, 89, 0.4);
	}

	.watch-btn:focus-visible {
		outline: 3px solid var(--color-bg-card);
		outline-offset: 2px;
	}

	/* Admin Upload Button */
	.admin-upload-btn {
		display: inline-flex;
		align-items: center;
		gap: 10px;
		padding: 14px 28px;
		background: rgba(255, 255, 255, 0.2);
		color: var(--color-bg-card);
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-radius: 10px;
		font-size: 15px;
		font-weight: 700;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.admin-upload-btn:hover {
		background: rgba(255, 255, 255, 0.3);
		border-color: rgba(255, 255, 255, 0.5);
		transform: translateY(-2px);
	}

	.admin-upload-btn:focus-visible {
		outline: 3px solid var(--color-bg-card);
		outline-offset: 2px;
	}

	/* Responsive */
	@media (min-width: 1440px) {
		.video-info h2 {
			font-size: 32px;
		}
	}

	@media (min-width: 1920px) {
		.video-info h2 {
			font-size: 36px;
		}

		.video-info .published-date {
			font-size: 16px;
		}

		.watch-btn {
			padding: 16px 32px;
			font-size: 16px;
		}
	}

	@media (min-width: 2560px) {
		.video-info h2 {
			font-size: 42px;
		}
	}

	/* Reduced Motion */
	@media (prefers-reduced-motion: reduce) {
		.watch-btn,
		.admin-upload-btn {
			transition: none;
		}
	}
</style>


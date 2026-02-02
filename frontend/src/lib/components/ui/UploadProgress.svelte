<!--
	UploadProgress.svelte
	═══════════════════════════════════════════════════════════════════════════════
	Apple Principal Engineer ICT Level 7 - January 2026
	
	Upload progress indicator with determinate and indeterminate states.
	Shows progress bar with status text.
	
	@version 1.0.0
-->
<script lang="ts">
	type UploadStatus = 'idle' | 'preparing' | 'uploading' | 'processing' | 'complete';

	interface Props {
		progress: number;
		status: UploadStatus;
		statusText?: string;
	}

	let props: Props = $props();
	let progress = $derived(props.progress);
	let status = $derived(props.status);
	let statusText = $derived(props.statusText);

	// Derived
	let isIndeterminate = $derived(status === 'preparing' || status === 'processing');
	let displayProgress = $derived(status === 'uploading' ? progress : 100);
	let defaultStatusText = $derived(getDefaultStatusText(status, progress));
	let finalStatusText = $derived(statusText || defaultStatusText);

	function getDefaultStatusText(s: UploadStatus, p: number): string {
		switch (s) {
			case 'preparing':
				return 'Preparing upload...';
			case 'uploading':
				return `Uploading... ${p}%`;
			case 'processing':
				return 'Processing...';
			case 'complete':
				return 'Complete!';
			default:
				return '';
		}
	}
</script>

{#if status !== 'idle'}
	<div class="upload-progress" role="status" aria-live="polite">
		<div
			class="progress-bar"
			role="progressbar"
			aria-valuenow={isIndeterminate ? undefined : displayProgress}
			aria-valuemin={0}
			aria-valuemax={100}
			aria-valuetext={finalStatusText}
			aria-label="Upload progress"
		>
			<div
				class="progress-fill"
				class:indeterminate={isIndeterminate}
				style="width: {displayProgress}%"
			></div>
		</div>
		{#if finalStatusText}
			<span class="progress-text" aria-hidden="true">{finalStatusText}</span>
		{/if}
	</div>
{/if}

<style>
	.upload-progress {
		margin-top: 1rem;
	}

	.progress-bar {
		height: 8px;
		background: rgba(0, 0, 0, 0.1);
		border-radius: 4px;
		overflow: hidden;
	}

	.progress-fill {
		height: 100%;
		background: linear-gradient(90deg, #143e59, #1a5a7e);
		transition: width 0.3s;
		border-radius: 4px;
	}

	.progress-fill.indeterminate {
		width: 100% !important;
		animation: indeterminate 1.5s infinite ease-in-out;
		background: linear-gradient(
			90deg,
			rgba(20, 62, 89, 0.3) 0%,
			#143e59 50%,
			rgba(20, 62, 89, 0.3) 100%
		);
		background-size: 200% 100%;
	}

	@keyframes indeterminate {
		0% {
			background-position: 200% 0;
		}
		100% {
			background-position: -200% 0;
		}
	}

	.progress-text {
		display: block;
		text-align: center;
		margin-top: 0.5rem;
		font-size: 0.8125rem;
		color: #64748b;
	}
</style>

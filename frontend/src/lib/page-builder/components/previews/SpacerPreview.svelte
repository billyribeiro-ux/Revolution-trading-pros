<script lang="ts">
	/**
	 * Spacer Preview Component
	 * Apple Principal Engineer ICT 7 Grade - January 2026
	 */

	import type { SpacerConfig } from '../../types';

	interface Props {
		config: SpacerConfig;
		isPreview: boolean;
	}

	let { config, isPreview }: Props = $props();
</script>

<div 
	class="spacer"
	class:preview={isPreview}
	style="height: {config.height ?? 40}px;"
>
	{#if !isPreview}
		<div class="spacer-indicator">
			<span class="spacer-label">{config.height ?? 40}px</span>
		</div>
	{/if}
</div>

<style>
	.spacer {
		position: relative;
		width: 100%;
	}

	.spacer:not(.preview) {
		background: repeating-linear-gradient(
			45deg,
			transparent,
			transparent 10px,
			rgba(20, 62, 89, 0.03) 10px,
			rgba(20, 62, 89, 0.03) 20px
		);
		border: 1px dashed #D1D5DB;
		border-radius: 4px;
	}

	.spacer-indicator {
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
	}

	.spacer-label {
		display: inline-block;
		padding: 4px 8px;
		background: #F3F4F6;
		border: 1px solid #E5E7EB;
		border-radius: 4px;
		font-size: 11px;
		font-weight: 500;
		color: #6B7280;
	}
</style>

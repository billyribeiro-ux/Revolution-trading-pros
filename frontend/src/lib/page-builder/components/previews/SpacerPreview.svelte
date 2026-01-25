<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * SpacerPreview Component
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @description Configurable vertical spacer with visual height indicator for
	 *              the page builder. Shows striped pattern in edit mode, invisible
	 *              in preview mode.
	 * @version 1.1.0 - ICT 7 compliance: accessibility, validation, transitions
	 * @requires Svelte 5.0+ (January 2026 syntax)
	 * @standards Apple Principal Engineer ICT 7+ Standards
	 *
	 * @example
	 * <SpacerPreview
	 *   config={{ height: 48 }}
	 *   isPreview={false}
	 * />
	 */

	import type { SpacerConfig } from '../../types';

	interface Props {
		config: SpacerConfig;
		isPreview?: boolean;
		minHeight?: number;
	}

	const { config, isPreview = false, minHeight = 8 }: Props = $props();

	// Computed height with validation and minimum constraint
	const computedHeight = $derived.by(() => {
		const rawHeight = config.height;

		if (rawHeight === undefined || rawHeight === null) {
			return 40; // Default
		}

		if (typeof rawHeight === 'number') {
			return Math.max(rawHeight, minHeight);
		}

		// Handle string values (e.g., "40px", "2rem")
		const numericValue = parseInt(String(rawHeight), 10);
		return isNaN(numericValue) ? 40 : Math.max(numericValue, minHeight);
	});

	const spacerStyle = $derived(`height: ${computedHeight}px`);
</script>

<div
	class="spacer"
	class:preview={isPreview}
	style={spacerStyle}
	role="presentation"
	aria-hidden="true"
>
	{#if !isPreview}
		<div class="spacer-indicator">
			<span class="spacer-label">{computedHeight}px</span>
		</div>
	{/if}
</div>

<style>
	.spacer {
		position: relative;
		width: 100%;
		transition: height 0.2s cubic-bezier(0.16, 1, 0.3, 1);
	}

	.spacer:not(.preview) {
		background: repeating-linear-gradient(
			45deg,
			transparent,
			transparent 10px,
			rgba(20, 62, 89, 0.03) 10px,
			rgba(20, 62, 89, 0.03) 20px
		);
		border: 1px dashed #d1d5db;
		border-radius: 4px;
	}

	.spacer-indicator {
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		pointer-events: none;
	}

	.spacer-label {
		display: inline-block;
		padding: 4px 8px;
		background: #f3f4f6;
		border: 1px solid #e5e7eb;
		border-radius: 4px;
		font-size: 11px;
		font-weight: 500;
		color: #6b7280;
		font-variant-numeric: tabular-nums;
		pointer-events: none;
		user-select: none;
	}
</style>

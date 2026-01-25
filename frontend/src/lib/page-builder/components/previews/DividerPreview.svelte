<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * DividerPreview Component
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @description Configurable horizontal divider with customizable style, color,
	 *              thickness, and margins for the page builder.
	 * @version 1.1.0 - ICT 7 compliance: accessibility, 8pt grid, Apple standards
	 * @requires Svelte 5.0+ (January 2026 syntax)
	 * @standards Apple Principal Engineer ICT 7+ Standards
	 *
	 * @example
	 * <DividerPreview
	 *   config={{ style: 'dashed', color: '#143E59', thickness: 2 }}
	 *   isPreview={false}
	 * />
	 */

	import type { DividerConfig } from '../../types';

	interface Props {
		config: DividerConfig;
		isPreview?: boolean;
	}

	const { config, isPreview = false }: Props = $props();

	// Computed style with proper formatting (no whitespace in output)
	const dividerStyle = $derived.by(() => {
		const formatPx = (val: number | undefined, fallback: number): string => {
			const num = val ?? fallback;
			return `${num}px`;
		};

		return [
			`border-top-style: ${config.style ?? 'solid'}`,
			`border-top-color: ${config.color ?? '#E0E0E0'}`,
			`border-top-width: ${formatPx(config.thickness, 1)}`,
			`margin-top: ${formatPx(config.marginTop, 20)}`,
			`margin-bottom: ${formatPx(config.marginBottom, 20)}`
		].join('; ');
	});
</script>

<div class="divider-wrapper" class:preview={isPreview}>
	<hr class="divider" style={dividerStyle} aria-hidden="true" />
	{#if !isPreview}
		<span class="divider-label">Divider</span>
	{/if}
</div>

<style>
	.divider-wrapper {
		position: relative;
	}

	.divider {
		border: none;
		border-top-style: solid;
	}

	.divider-label {
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		padding: 4px 8px;
		background: #f9fafb;
		border: 1px solid #e5e7eb;
		border-radius: 4px;
		font-size: 11px;
		font-weight: 500;
		color: #9ca3af;
		text-transform: uppercase;
		letter-spacing: 0.5px;
		pointer-events: none;
		user-select: none;
	}
</style>

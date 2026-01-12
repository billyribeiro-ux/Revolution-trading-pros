<script lang="ts">
	/**
	 * Divider Preview Component
	 * Apple Principal Engineer ICT 7 Grade - January 2026
	 */

	import type { DividerConfig } from '../../types';

	interface Props {
		config: DividerConfig;
		isPreview: boolean;
	}

	let { config, isPreview }: Props = $props();

	const dividerStyle = $derived(`
		border-top-style: ${config.style ?? 'solid'};
		border-top-color: ${config.color ?? '#E0E0E0'};
		border-top-width: ${config.thickness ?? 1}px;
		margin-top: ${config.marginTop ?? 20}px;
		margin-bottom: ${config.marginBottom ?? 20}px;
	`);
</script>

<div class="divider-wrapper" class:preview={isPreview}>
	<hr class="divider" style={dividerStyle} />
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
		padding: 2px 8px;
		background: #F9FAFB;
		border: 1px solid #E5E7EB;
		border-radius: 4px;
		font-size: 10px;
		font-weight: 500;
		color: #9CA3AF;
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}
</style>

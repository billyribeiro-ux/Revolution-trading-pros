<!--
/**
 * DropZone - Upload-tab drop zone with Browse Files button
 * Extracted from AssetManager.svelte (R6-C). Pure leaf — no binds.
 */
-->
<script lang="ts">
	import Icon from '$lib/components/Icon.svelte';

	interface Props {
		isDragging: boolean;
		acceptTypes: ('image' | 'video' | 'audio' | 'document')[];
		onFileInput: (e: Event) => void;
	}

	const { isDragging, acceptTypes, onFileInput }: Props = $props();
</script>

<div class={['drop-zone', { dragging: isDragging }]} role="region" aria-label="Upload drop zone">
	<Icon name="IconUpload" size={48} stroke={1.5} />
	<h4>Drag and drop files here</h4>
	<p>or</p>
	<label class="browse-btn">
		Browse Files
		<input
			type="file"
			multiple
			accept={acceptTypes.map((t) => (t === 'document' ? 'application/*' : `${t}/*`)).join(',')}
			onchange={onFileInput}
			hidden
		/>
	</label>
	<p class="upload-hint">
		Supports: {acceptTypes.map((t) => t.charAt(0).toUpperCase() + t.slice(1) + 's').join(', ')}
	</p>
</div>

<style>
	.drop-zone {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		min-height: 200px;
		padding: 3rem;
		border: 2px dashed rgba(255, 255, 255, 0.2);
		border-radius: 12px;
		transition: all 0.2s;
	}

	.drop-zone.dragging {
		border-color: #3b82f6;
		background: rgba(59, 130, 246, 0.1);
	}

	.drop-zone :global(svg) {
		color: #64748b;
		margin-bottom: 1rem;
	}

	.drop-zone h4 {
		margin: 0;
		font-size: 1rem;
		font-weight: 600;
		color: #e2e8f0;
	}

	.drop-zone p {
		margin: 0.5rem 0;
		color: #64748b;
	}

	.browse-btn {
		padding: 0.625rem 1.25rem;
		background: #3b82f6;
		color: white;
		border-radius: 8px;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
	}

	.upload-hint {
		font-size: 0.75rem;
		margin-top: 1rem;
	}
</style>

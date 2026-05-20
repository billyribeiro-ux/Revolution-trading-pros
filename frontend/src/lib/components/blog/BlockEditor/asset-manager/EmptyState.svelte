<!--
/**
 * EmptyState - Shown when no assets match current filters
 * Extracted from AssetManager.svelte (R6-C). Pure leaf — no binds.
 */
-->
<script lang="ts">
	import Icon from '$lib/components/Icon.svelte';

	interface Props {
		acceptTypes: ('image' | 'video' | 'audio' | 'document')[];
		onFileInput: (e: Event) => void;
	}

	const { acceptTypes, onFileInput }: Props = $props();
</script>

<div class="empty-state">
	<Icon name="IconPhoto" size={48} stroke={1.5} />
	<h4>No assets found</h4>
	<p>Upload files or adjust your filters</p>
	<label class="upload-cta">
		Upload Files
		<input
			type="file"
			multiple
			accept={acceptTypes
				.map((t) => (t === 'document' ? 'application/*' : `${t}/*`))
				.join(',')}
			onchange={onFileInput}
			hidden
		/>
	</label>
</div>

<style>
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		height: 300px;
		color: #64748b;
		gap: 1rem;
	}

	.empty-state :global(svg) {
		opacity: 0.5;
	}

	.empty-state h4 {
		margin: 0;
		font-size: 1rem;
		color: #94a3b8;
	}

	.empty-state p {
		margin: 0;
		font-size: 0.875rem;
	}

	.upload-cta {
		padding: 0.625rem 1.25rem;
		background: #3b82f6;
		color: white;
		border-radius: 8px;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		margin-top: 0.5rem;
	}
</style>

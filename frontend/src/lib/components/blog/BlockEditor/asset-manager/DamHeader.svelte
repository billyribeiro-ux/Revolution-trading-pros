<!--
/**
 * DamHeader - AssetManager modal header bar
 * Extracted from AssetManager.svelte (R6-C). Pure leaf — no binds.
 */
-->
<script lang="ts">
	import Icon from '$lib/components/Icon.svelte';

	interface Props {
		totalAssets: number;
		acceptTypes: ('image' | 'video' | 'audio' | 'document')[];
		onClose: () => void;
		onFileInput: (e: Event) => void;
	}

	const { totalAssets, acceptTypes, onClose, onFileInput }: Props = $props();
</script>

<header class="dam-header">
	<div class="header-left">
		<h2>Asset Manager</h2>
		<span class="asset-count">{totalAssets.toLocaleString()} assets</span>
	</div>
	<div class="header-actions">
		<label class="upload-btn">
			<Icon name="IconUpload" size={16} />
			Upload
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
		<button class="close-btn" onclick={onClose} aria-label="Close">
			<Icon name="IconX" size={20} />
		</button>
	</div>
</header>

<style>
	.dam-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1rem 1.5rem;
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
		background: rgba(255, 255, 255, 0.02);
	}

	.header-left {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.dam-header h2 {
		margin: 0;
		font-size: 1.25rem;
		font-weight: 600;
		color: #f1f5f9;
	}

	.asset-count {
		font-size: 0.8125rem;
		color: #64748b;
	}

	.header-actions {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.upload-btn {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 1rem;
		background: #3b82f6;
		color: white;
		border: none;
		border-radius: 8px;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.upload-btn:hover {
		background: #2563eb;
	}

	.close-btn {
		width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(255, 255, 255, 0.05);
		border: none;
		border-radius: 8px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
	}

	.close-btn:hover {
		background: rgba(255, 255, 255, 0.1);
		color: #f1f5f9;
	}
</style>

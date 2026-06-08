<!--
	ViewModeToggle — edit/preview mode switcher.
	Extracted from BlockEditor.svelte (R7-C).
-->
<script lang="ts">
	import { IconEdit, IconEye } from '$lib/icons';

	type ViewMode = 'edit' | 'preview';

	interface Props {
		mode: ViewMode;
		onChange: (mode: ViewMode) => void;
	}

	let { mode, onChange }: Props = $props();
</script>

<div class="view-toggle" role="group" aria-label="View mode">
	<button
		type="button"
		class={['view-btn', { active: mode === 'edit' }]}
		onclick={() => onChange('edit')}
		aria-label="Edit mode"
		aria-pressed={mode === 'edit'}
	>
		<IconEdit size={16} aria-hidden="true" />
		Edit
	</button>
	<button
		type="button"
		class={['view-btn', { active: mode === 'preview' }]}
		onclick={() => onChange('preview')}
		aria-label="Preview mode"
		aria-pressed={mode === 'preview'}
	>
		<IconEye size={16} aria-hidden="true" />
		Preview
	</button>
</div>

<style>
	.view-toggle {
		display: flex;
		background: #f0f0f0;
		border-radius: 8px;
		padding: 0.25rem;
	}

	.view-btn {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.375rem 0.75rem;
		border: none;
		background: transparent;
		border-radius: 6px;
		cursor: pointer;
		color: #666;
		font-size: 0.8125rem;
		font-weight: 500;
		transition: all 0.15s;
	}

	.view-btn.active {
		background: white;
		color: #1a1a1a;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
	}

	.view-btn:focus-visible {
		outline: 2px solid #3b82f6;
		outline-offset: 2px;
	}
</style>

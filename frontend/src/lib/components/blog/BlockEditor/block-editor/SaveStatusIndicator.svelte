<!--
	SaveStatusIndicator — header save-status text (saving / unsaved / saved / error).
	Extracted from BlockEditor.svelte (R7-C).
-->
<script lang="ts">
	import { IconCloudUpload } from '$lib/icons';

	interface Props {
		isSaving: boolean;
		saveError: string | null;
		hasUnsavedChanges: boolean;
		lastSavedLabel: string;
	}

	let { isSaving, saveError, hasUnsavedChanges, lastSavedLabel }: Props = $props();
</script>

<div class="save-status">
	{#if saveError}
		<span class="error">{saveError}</span>
	{:else if isSaving}
		<span class="saving"><IconCloudUpload size={16} class="spin" /> Saving...</span>
	{:else if hasUnsavedChanges}
		<span class="unsaved">Unsaved changes</span>
	{:else}
		<span class="saved">Saved {lastSavedLabel}</span>
	{/if}
</div>

<style>
	.save-status {
		font-size: 0.8125rem;
	}

	.save-status .saving {
		color: #3b82f6;
		display: flex;
		align-items: center;
		gap: 0.375rem;
	}

	.save-status .unsaved {
		color: #f59e0b;
	}

	.save-status .saved {
		color: #10b981;
	}

	.save-status .error {
		color: #ef4444;
	}
</style>

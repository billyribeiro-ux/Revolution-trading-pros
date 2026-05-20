<!--
	AnnouncementsLiveRegion — sr-only live region for drag/drop screen-reader announcements.
	Extracted from BlockEditor.svelte (R7-C).
-->
<script lang="ts">
	interface Props {
		announcements: string[];
	}

	let { announcements }: Props = $props();
</script>

<div class="sr-only" role="status" aria-live="polite" aria-atomic="true">
	{#each announcements as announcement, i (i)}
		<p>{announcement}</p>
	{/each}
</div>

<style>
	.sr-only {
		position: absolute;
		width: 1px;
		height: 1px;
		padding: 0;
		margin: -1px;
		overflow: hidden;
		clip: rect(0, 0, 0, 0);
		white-space: nowrap;
		border: 0;
	}
</style>

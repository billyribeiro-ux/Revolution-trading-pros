<script lang="ts">
	import type { Snippet } from 'svelte';
	import type { Attachment } from 'svelte/attachments';
	let {
		title,
		busy = false,
		onClose,
		children
	}: { title: string; busy?: boolean; onClose: () => void; children: Snippet } = $props();
	const id = $props.id();
	const openDialog: Attachment<HTMLDialogElement> = (element) => {
		element.showModal();
		return () => element.close();
	};
</script>

<dialog
	aria-labelledby={id}
	{@attach openDialog}
	oncancel={(event) => {
		event.preventDefault();
		if (!busy) onClose();
	}}
>
	<header>
		<h2 {id}>{title}</h2>
		<button type="button" aria-label={`Close ${title}`} disabled={busy} onclick={onClose}>×</button>
	</header>
	<div class="dialog-body">{@render children()}</div>
</dialog>

<style>
	dialog {
		background: var(--bg-elevated, #161b22);
		color: var(--text-primary, #f0f6fc);
		border: 1px solid var(--border-default, #30363d);
		border-radius: 16px;
		width: min(620px, calc(100vw - 2rem));
		max-height: calc(100dvh - 2rem);
		padding: 0;
		overflow: auto;
		box-shadow: 0 20px 80px #0008;
	}
	dialog::backdrop {
		background: #000b;
	}
	header {
		position: sticky;
		top: 0;
		background: var(--bg-elevated, #161b22);
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		padding: 1rem 1.5rem;
		border-bottom: 1px solid var(--border-default, #30363d);
		z-index: 1;
	}
	h2 {
		font-size: 1.2rem;
		margin: 0;
	}
	button {
		background: transparent;
		color: inherit;
		border: 1px solid var(--border-default, #30363d);
		border-radius: 8px;
		font-size: 1.5rem;
		cursor: pointer;
	}
	.dialog-body {
		padding: 1.5rem;
	}
</style>

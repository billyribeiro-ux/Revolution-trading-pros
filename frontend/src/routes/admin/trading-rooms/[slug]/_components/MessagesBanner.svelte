<!--
	MessagesBanner — success/error toast banner pair.
	Extracted from +page.svelte (R8-C) — 1 callback prop, no binds.
-->
<script lang="ts">
	import IconCheck from '@tabler/icons-svelte-runes/icons/check';
	import IconX from '@tabler/icons-svelte-runes/icons/x';

	interface Props {
		successMessage: string;
		errorMessage: string;
		onDismissError: () => void;
	}

	const { successMessage, errorMessage, onDismissError }: Props = $props();
</script>

{#if successMessage}
	<div class="message success">
		<IconCheck size={20} />
		{successMessage}
	</div>
{/if}
{#if errorMessage}
	<div class="message error">
		<IconX size={20} />
		{errorMessage}
		<button onclick={onDismissError}>×</button>
	</div>
{/if}

<style>
	/* Messages */
	.message {
		display: flex;
		align-items: center;
		gap: 10px;
		padding: 14px 18px;
		border-radius: 10px;
		margin-bottom: 20px;
		font-weight: 500;
	}

	.message.success {
		background: #dcfce7;
		color: #166534;
	}

	.message.error {
		background: #fef2f2;
		color: #991b1b;
	}

	.message button {
		margin-left: auto;
		background: none;
		border: none;
		font-size: 20px;
		cursor: pointer;
		opacity: 0.7;
	}
</style>

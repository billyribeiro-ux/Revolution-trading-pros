<script lang="ts">
	import { IconInbox, IconPlus } from '@tabler/icons-svelte';

	interface Props {
		title: string;
		description: string;
		actionLabel?: string;
		onaction?: () => void;
	}

	let { title, description, actionLabel, onaction }: Props = $props();
</script>

<div class="flex flex-col items-center justify-center py-16 px-6 text-center">
	<div class="mb-4 flex h-16 w-16 items-center justify-center rounded-full bg-slate-800/50">
		<IconInbox size={32} class="text-slate-500" />
	</div>

	<h3 class="mb-2 text-lg font-semibold text-slate-200">{title}</h3>
	<p class="mb-6 max-w-md text-sm text-slate-400">{description}</p>

	{#if actionLabel && onaction}
		<button
			type="button"
			class="inline-flex items-center gap-2 rounded-xl bg-indigo-500 px-4 py-2 text-sm font-medium text-white shadow-sm hover:bg-indigo-400 focus:outline-none focus:ring-2 focus:ring-indigo-500/60"
			onclick={onaction}
		>
			<IconPlus size={18} />
			{actionLabel}
		</button>
	{/if}
</div>

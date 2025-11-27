<script lang="ts">
	import { IconCheck, IconX, IconAlertCircle, IconInfoCircle } from '@tabler/icons-svelte';
	import { toasts, removeToast, type Toast } from '$lib/utils/toast';

	const icons = {
		success: IconCheck,
		error: IconX,
		warning: IconAlertCircle,
		info: IconInfoCircle
	};

	const colors = {
		success: 'bg-green-50 border-green-500 text-green-800',
		error: 'bg-red-50 border-red-500 text-red-800',
		warning: 'bg-yellow-50 border-yellow-500 text-yellow-800',
		info: 'bg-blue-50 border-blue-500 text-blue-800'
	};
</script>

<div class="fixed top-4 right-4 z-50 space-y-2">
	{#each $toasts as toast (toast.id)}
		<div
			class="flex items-center gap-3 px-4 py-3 rounded-lg border-l-4 shadow-lg min-w-[300px] animate-slide-in {colors[
				toast.type
			]}"
		>
			<svelte:component this={icons[toast.type]} size={20} />
			<p class="flex-1 text-sm font-medium">{toast.message}</p>
			<button onclick={() => removeToast(toast.id)} class="text-gray-500 hover:text-gray-700">
				<IconX size={16} />
			</button>
		</div>
	{/each}
</div>

<style>
	@keyframes slide-in {
		from {
			transform: translateX(100%);
			opacity: 0;
		}
		to {
			transform: translateX(0);
			opacity: 1;
		}
	}

	.animate-slide-in {
		animation: slide-in 0.3s ease-out;
	}
</style>

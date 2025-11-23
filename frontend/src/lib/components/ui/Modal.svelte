<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import { IconX } from '@tabler/icons-svelte';

	export let open: boolean = false;
	export let title: string = '';
	export let size: 'sm' | 'md' | 'lg' | 'xl' = 'md';

	const dispatch = createEventDispatcher();

	const sizes = {
		sm: 'max-w-md',
		md: 'max-w-lg',
		lg: 'max-w-2xl',
		xl: 'max-w-4xl'
	};

	function close() {
		open = false;
		dispatch('close');
	}

	function handleBackdropClick(e: MouseEvent) {
		if (e.target === e.currentTarget) {
			close();
		}
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') {
			close();
		}
	}
</script>

{#if open}
	<div
		class="fixed inset-0 z-50 overflow-y-auto"
		on:click={handleBackdropClick}
		on:keydown={handleKeydown}
		role="dialog"
		aria-modal="true"
		tabindex="-1"
	>
		<div class="flex min-h-screen items-center justify-center p-4">
			<!-- Backdrop -->
			<div class="fixed inset-0 bg-black bg-opacity-50 transition-opacity"></div>

			<!-- Modal -->
			<div class="relative bg-white rounded-lg shadow-xl {sizes[size]} w-full">
				<!-- Header -->
				{#if title}
					<div class="flex items-center justify-between px-6 py-4 border-b border-gray-200">
						<h3 class="text-lg font-semibold text-gray-900">{title}</h3>
						<button
							on:click={close}
							class="text-gray-400 hover:text-gray-600 transition-colors"
							aria-label="Close modal"
						>
							<IconX size={20} />
						</button>
					</div>
				{/if}

				<!-- Body -->
				<div class="px-6 py-4">
					<slot />
				</div>

				<!-- Footer -->
				{#if $$slots.footer}
					<div class="px-6 py-4 border-t border-gray-200 bg-gray-50 rounded-b-lg">
						<slot name="footer" />
					</div>
				{/if}
			</div>
		</div>
	</div>
{/if}

<script lang="ts">
	import { AlertTriangle } from '@lucide/svelte';
	import gsap from 'gsap';

	interface Props {
		open: boolean;
		title: string;
		message: string;
		confirmText?: string;
		cancelText?: string;
		variant?: 'default' | 'danger';
		onConfirm: () => void;
		onCancel: () => void;
	}

	let {
		open,
		title,
		message,
		confirmText = 'Confirm',
		cancelText = 'Cancel',
		variant = 'default',
		onConfirm,
		onCancel
	}: Props = $props();

	let dialogEl: HTMLDivElement | undefined = $state();

	$effect(() => {
		if (open && dialogEl) {
			gsap.fromTo(
				dialogEl,
				{ scale: 0.9, opacity: 0 },
				{ scale: 1, opacity: 1, duration: 0.25, ease: 'back.out(1.7)' }
			);
		}
	});

	function handleKeydown(e: KeyboardEvent) {
		if (!open) return;
		if (e.key === 'Escape') {
			e.preventDefault();
			onCancel();
		} else if (e.key === 'Enter') {
			e.preventDefault();
			onConfirm();
		}
	}

	let confirmBg = $derived(variant === 'danger' ? 'var(--calc-put)' : 'var(--calc-accent)');
</script>

<svelte:window onkeydown={handleKeydown} />

{#if open}
	<div
		class="fixed inset-0 z-[10002] flex items-center justify-center"
		role="dialog"
		aria-modal="true"
		aria-labelledby="confirm-title"
		aria-describedby="confirm-message"
	>
		<!-- Backdrop -->
		<button
			class="absolute inset-0 cursor-default"
			style="background: rgba(0,0,0,0.6); backdrop-filter: blur(4px);"
			onclick={onCancel}
			aria-label="Close"
			tabindex={-1}
		></button>

		<!-- Dialog -->
		<div
			bind:this={dialogEl}
			class="relative z-10 w-full max-w-sm rounded-2xl p-5 flex flex-col gap-4"
			style="background: var(--calc-surface); border: 1px solid var(--calc-border); box-shadow: 0 24px 80px rgba(0,0,0,0.5);"
		>
			<!-- Icon + Title -->
			<div class="flex items-start gap-3">
				{#if variant === 'danger'}
					<div
						class="flex-shrink-0 w-9 h-9 rounded-full flex items-center justify-center"
						style="background: rgba(239,68,68,0.1);"
					>
						<AlertTriangle size={18} style="color: #ef4444;" />
					</div>
				{/if}
				<div class="flex flex-col gap-1">
					<h3
						id="confirm-title"
						class="text-sm font-semibold"
						style="color: var(--calc-text); font-family: var(--calc-font-display);"
					>
						{title}
					</h3>
					<p
						id="confirm-message"
						class="text-xs leading-relaxed"
						style="color: var(--calc-text-muted);"
					>
						{message}
					</p>
				</div>
			</div>

			<!-- Actions -->
			<div class="flex items-center gap-2 justify-end">
				<button
					onclick={onCancel}
					class="px-4 py-2 text-xs font-medium rounded-lg cursor-pointer transition-colors"
					style="color: var(--calc-text-muted); border: 1px solid var(--calc-border);"
				>
					{cancelText}
				</button>
				<button
					onclick={onConfirm}
					class="px-4 py-2 text-xs font-semibold rounded-lg cursor-pointer transition-colors"
					style="background: {confirmBg}; color: white;"
				>
					{confirmText}
				</button>
			</div>
		</div>
	</div>
{/if}

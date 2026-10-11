<script lang="ts">
	import { trapFocus } from '../../utils/focus-trap.js';
	import IconAlertTriangle from '@tabler/icons-svelte-runes/icons/alert-triangle';
	import { backOut } from 'svelte/easing';
	import { prefersReducedMotion } from 'svelte/motion';
	import { scale } from 'svelte/transition';
	import type { Attachment } from 'svelte/attachments';

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

	const uid = $props.id();

	function handleKeydown(event: KeyboardEvent) {
		if (!open) return;
		if (event.key === 'Escape') {
			event.preventDefault();
			onCancel();
		} else if (event.key === 'Enter' && !(event.target instanceof HTMLButtonElement)) {
			// A focused button activates itself, so Enter on "Cancel" cancels.
			event.preventDefault();
			onConfirm();
		}
	}

	/** Moves focus into the dialog when it opens and hands it back when it closes. */
	const takeFocus: Attachment<HTMLButtonElement> = (button) => {
		const previous = document.activeElement;
		button.focus();
		return () => {
			if (previous instanceof HTMLElement && previous.isConnected) previous.focus();
		};
	};
</script>

<svelte:window onkeydown={handleKeydown} />

{#if open}
	<div class="fixed inset-0 z-[10002] flex items-center justify-center">
		<!-- Backdrop -->
		<button
			type="button"
			class="absolute inset-0 cursor-default"
			style="background: rgba(0,0,0,0.6); backdrop-filter: blur(4px);"
			onclick={onCancel}
			aria-label="Close"
			tabindex={-1}
		></button>

		<!-- Dialog -->
		<div
			{@attach trapFocus}
			in:scale|global={{
				start: 0.9,
				duration: prefersReducedMotion.current ? 0 : 250,
				easing: backOut
			}}
			class="relative z-10 w-full max-w-sm rounded-2xl p-5 flex flex-col gap-4"
			style="background: var(--calc-surface); border: 1px solid var(--calc-border); box-shadow: 0 24px 80px rgba(0,0,0,0.5);"
			role="dialog"
			aria-modal="true"
			aria-labelledby="{uid}-title"
			aria-describedby="{uid}-message"
		>
			<!-- Icon + Title -->
			<div class="flex items-start gap-3">
				{#if variant === 'danger'}
					<div
						class="flex-shrink-0 w-9 h-9 rounded-full flex items-center justify-center"
						style="background: rgba(239,68,68,0.1);"
					>
						<IconAlertTriangle size={18} color="#ef4444" aria-hidden="true" />
					</div>
				{/if}
				<div class="flex flex-col gap-1">
					<h3
						id="{uid}-title"
						class="text-sm font-semibold"
						style="color: var(--calc-text); font-family: var(--calc-font-display);"
					>
						{title}
					</h3>
					<p
						id="{uid}-message"
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
					type="button"
					onclick={onCancel}
					class="px-4 py-2 text-xs font-medium rounded-lg cursor-pointer transition-colors motion-reduce:transition-none"
					style="color: var(--calc-text-muted); border: 1px solid var(--calc-border);"
				>
					{cancelText}
				</button>
				<button
					{@attach takeFocus}
					type="button"
					onclick={onConfirm}
					class="px-4 py-2 text-xs font-semibold rounded-lg cursor-pointer transition-colors motion-reduce:transition-none"
					style:background={variant === 'danger' ? 'var(--calc-put)' : 'var(--calc-accent)'}
					style:color="white"
				>
					{confirmText}
				</button>
			</div>
		</div>
	</div>
{/if}

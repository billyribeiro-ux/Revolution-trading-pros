<script lang="ts">
	import { trapFocus } from '../../utils/focus-trap.js';
	import IconLink from '@tabler/icons-svelte-runes/icons/link';
	import IconCopy from '@tabler/icons-svelte-runes/icons/copy';
	import IconExternalLink from '@tabler/icons-svelte-runes/icons/external-link';
	import IconX from '@tabler/icons-svelte-runes/icons/x';
	import { page } from '$app/state';
	import { backOut } from 'svelte/easing';
	import { prefersReducedMotion } from 'svelte/motion';
	import { scale } from 'svelte/transition';
	import type { Attachment } from 'svelte/attachments';
	import { generateShareURL, copyToClipboard } from '../../utils/share-utils.js';
	import { getCalculator } from '../../state/context.js';

	const calc = getCalculator();
	const uid = $props.id();

	/** How long the copy button reads "Copied!". */
	const COPIED_FEEDBACK_MS = 2500;

	let copied = $state(false);
	let copiedTimer: ReturnType<typeof setTimeout> | undefined;

	// Built from `page.url` rather than `window.location`, so it is safe to evaluate anywhere.
	const shareURL = $derived(
		generateShareURL(
			{
				inputs: calc.inputs,
				optionType: calc.optionType,
				mode: calc.calculatorMode,
				activeTab: calc.activeTab,
				ticker: calc.activeTicker || undefined,
				strategyLegs: calc.strategyLegs.length > 0 ? calc.strategyLegs : undefined
			},
			`${page.url.origin}/tools/options-calculator`
		)
	);

	const displayURL = $derived(shareURL.length > 80 ? `${shareURL.slice(0, 77)}…` : shareURL);

	async function handleCopy(): Promise<void> {
		const success = await copyToClipboard(shareURL);
		if (success) {
			copied = true;
			clearTimeout(copiedTimer);
			copiedTimer = setTimeout(() => (copied = false), COPIED_FEEDBACK_MS);
			calc.addToast('success', 'Link copied to clipboard!');
		} else {
			calc.addToast('error', 'Failed to copy link.');
		}
	}

	/**
	 * The only way the dialog closes (backdrop, close button, Escape), so every opening starts
	 * with the plain "Copy Link" label.
	 */
	function handleClose(): void {
		calc.showShareLink = false;
		clearTimeout(copiedTimer);
		copied = false;
	}

	function handleKeydown(event: KeyboardEvent): void {
		if (calc.showShareLink && event.key === 'Escape') handleClose();
	}

	/** Moves focus into the dialog when it opens and hands it back to the opener when it closes. */
	const focusDialog: Attachment<HTMLElement> = (dialog) => {
		const opener = document.activeElement;
		dialog.focus();
		return () => {
			if (opener instanceof HTMLElement && opener.isConnected) opener.focus();
		};
	};
</script>

<svelte:window onkeydown={handleKeydown} />

{#if calc.showShareLink}
	<div
		{@attach trapFocus}
		{@attach focusDialog}
		class="fixed inset-0 z-[9995] flex items-center justify-center p-4 outline-none"
		role="dialog"
		aria-modal="true"
		aria-labelledby="{uid}-title"
		tabindex="-1"
	>
		<button
			type="button"
			class="absolute inset-0 cursor-default"
			style="background: rgba(0,0,0,0.6); backdrop-filter: blur(4px);"
			onclick={handleClose}
			aria-label="Close"
			tabindex={-1}
		></button>

		<div
			in:scale={{
				start: 0.92,
				duration: prefersReducedMotion.current ? 0 : 250,
				easing: backOut
			}}
			class="relative z-10 w-full max-w-md rounded-2xl p-5 flex flex-col gap-4"
			style="
				background: var(--calc-surface);
				border: 1px solid var(--calc-border);
				box-shadow: 0 24px 80px rgba(0,0,0,0.5);
			"
		>
			<!-- Header -->
			<div class="flex items-center justify-between">
				<div class="flex items-center gap-2">
					<IconLink size={16} style="color: var(--calc-accent);" aria-hidden="true" />
					<h3
						id="{uid}-title"
						class="text-sm font-semibold"
						style="color: var(--calc-text); font-family: var(--calc-font-display);"
					>
						Share Calculator
					</h3>
				</div>
				<button
					type="button"
					onclick={handleClose}
					class="cursor-pointer p-1 rounded-md"
					style="color: var(--calc-text-muted);"
					aria-label="Close"
				>
					<IconX size={16} aria-hidden="true" />
				</button>
			</div>

			<p class="text-xs leading-relaxed" style="color: var(--calc-text-muted);">
				Anyone with this link will see the calculator pre-loaded with your current settings.
			</p>

			<!-- URL Preview -->
			<div
				class="flex items-center gap-2 rounded-lg px-3 py-2.5"
				style="background: var(--calc-surface-hover); border: 1px solid var(--calc-border);"
			>
				<span
					class="flex-1 text-[10px] truncate select-all"
					style="color: var(--calc-text-secondary); font-family: var(--calc-font-mono);"
				>
					{displayURL}
				</span>
			</div>

			<!-- Actions -->
			<div class="flex gap-2">
				<button
					type="button"
					onclick={handleCopy}
					class="flex-1 flex items-center justify-center gap-1.5 py-2.5 rounded-lg text-xs font-semibold cursor-pointer transition-all motion-reduce:transition-none duration-200"
					style="background: var(--calc-accent); color: white;"
				>
					<IconCopy size={13} aria-hidden="true" />
					{copied ? 'Copied!' : 'Copy Link'}
				</button>
				<!-- A real link: it navigates, so it also supports middle-click, copy-link, etc. -->
				<a
					href={shareURL}
					target="_blank"
					rel="noopener"
					class="flex items-center justify-center gap-1.5 px-4 py-2.5 rounded-lg text-xs font-medium cursor-pointer"
					style="background: var(--calc-surface-hover); color: var(--calc-text-secondary); border: 1px solid var(--calc-border);"
				>
					<IconExternalLink size={13} aria-hidden="true" />
					Open
				</a>
			</div>
		</div>
	</div>
{/if}

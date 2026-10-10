<script lang="ts">
	import IconCode from '@tabler/icons-svelte-runes/icons/code';
	import IconCopy from '@tabler/icons-svelte-runes/icons/copy';
	import IconX from '@tabler/icons-svelte-runes/icons/x';
	import { page } from '$app/state';
	import { backOut } from 'svelte/easing';
	import { prefersReducedMotion } from 'svelte/motion';
	import { scale } from 'svelte/transition';
	import type { Attachment } from 'svelte/attachments';
	import { copyToClipboard } from '../../utils/share-utils.js';
	import { getCalculator } from '../../state/context.js';

	const calc = getCalculator();

	/** How long the copy button reads "Copied!". */
	const COPIED_FEEDBACK_MS = 2500;

	let copied = $state(false);
	let copiedTimer: ReturnType<typeof setTimeout> | undefined;

	// Embed settings persist between openings of the dialog.
	let embedWidth = $state('100%');
	let embedHeight = $state('700');
	let embedTheme = $state<'dark' | 'light'>('dark');

	// Built from `page.url` rather than `window.location`, so it is safe to evaluate anywhere.
	const embedURL = $derived.by(() => {
		const params = new URLSearchParams();
		params.set('theme', embedTheme);
		if (calc.activeTicker) params.set('ticker', calc.activeTicker);
		return `${page.url.origin}/tools/options-calculator/embed?${params.toString()}`;
	});

	const embedCode = $derived(
		`<iframe src="${embedURL}" width="${embedWidth}" height="${embedHeight}px" frameborder="0" style="border-radius:12px;border:1px solid #2a2a3a;" title="Black-Scholes Options Calculator" loading="lazy"></iframe>\n<p style="font-size:11px;color:#888;margin-top:4px;">Powered by <a href="https://revolutiontradingpros.com" target="_blank" rel="noopener">Revolution Trading Pros</a></p>`
	);

	async function handleCopy(): Promise<void> {
		const success = await copyToClipboard(embedCode);
		if (success) {
			copied = true;
			clearTimeout(copiedTimer);
			copiedTimer = setTimeout(() => (copied = false), COPIED_FEEDBACK_MS);
			calc.addToast('success', 'Embed code copied!');
		} else {
			calc.addToast('error', 'Failed to copy embed code.');
		}
	}

	/**
	 * The only way the dialog closes (backdrop, close button, Escape), so every opening starts
	 * with the plain "Copy Embed Code" label.
	 */
	function handleClose(): void {
		calc.showEmbedCode = false;
		clearTimeout(copiedTimer);
		copied = false;
	}

	function handleKeydown(event: KeyboardEvent): void {
		if (calc.showEmbedCode && event.key === 'Escape') handleClose();
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

{#if calc.showEmbedCode}
	<div
		{@attach focusDialog}
		class="fixed inset-0 z-9995 flex items-center justify-center p-4 outline-none"
		role="dialog"
		aria-modal="true"
		aria-label="Embed Code Generator"
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
			class="relative z-10 w-full max-w-lg rounded-2xl p-5 flex flex-col gap-4"
			style="
				background: var(--calc-surface);
				border: 1px solid var(--calc-border);
				box-shadow: 0 24px 80px rgba(0,0,0,0.5);
			"
		>
			<!-- Header -->
			<div class="flex items-center justify-between">
				<div class="flex items-center gap-2">
					<IconCode size={16} style="color: var(--calc-accent);" aria-hidden="true" />
					<h3
						class="text-sm font-semibold"
						style="color: var(--calc-text); font-family: var(--calc-font-display);"
					>
						Embed Calculator
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

			<!-- Customization Options -->
			<div class="grid grid-cols-3 gap-3">
				<label class="flex flex-col gap-1">
					<span
						class="text-[10px] uppercase tracking-wider font-medium"
						style="color: var(--calc-text-muted);">Width</span
					>
					<input
						type="text"
						bind:value={embedWidth}
						class="text-xs px-2 py-1.5 rounded-lg outline-none"
						style="background: var(--calc-surface-hover); color: var(--calc-text); border: 1px solid var(--calc-border); font-family: var(--calc-font-mono);"
					/>
				</label>
				<label class="flex flex-col gap-1">
					<span
						class="text-[10px] uppercase tracking-wider font-medium"
						style="color: var(--calc-text-muted);">Height (px)</span
					>
					<input
						type="text"
						inputmode="numeric"
						bind:value={embedHeight}
						class="text-xs px-2 py-1.5 rounded-lg outline-none"
						style="background: var(--calc-surface-hover); color: var(--calc-text); border: 1px solid var(--calc-border); font-family: var(--calc-font-mono);"
					/>
				</label>
				<label class="flex flex-col gap-1">
					<span
						class="text-[10px] uppercase tracking-wider font-medium"
						style="color: var(--calc-text-muted);">Theme</span
					>
					<select
						bind:value={embedTheme}
						class="text-xs px-2 py-1.5 rounded-lg outline-none cursor-pointer"
						style="background: var(--calc-surface-hover); color: var(--calc-text); border: 1px solid var(--calc-border);"
					>
						<option value="dark">Dark</option>
						<option value="light">Light</option>
					</select>
				</label>
			</div>

			<!-- Code Preview -->
			<div
				class="rounded-lg p-3 overflow-x-auto"
				style="background: var(--calc-surface-hover); border: 1px solid var(--calc-border);"
			>
				<pre
					class="text-[10px] whitespace-pre-wrap break-all leading-relaxed"
					style="color: var(--calc-text-secondary); font-family: var(--calc-font-mono);">{embedCode}</pre>
			</div>

			<!-- Copy Button -->
			<button
				type="button"
				onclick={handleCopy}
				class="flex items-center justify-center gap-1.5 w-full py-2.5 rounded-xl text-xs font-semibold cursor-pointer transition-all duration-200"
				style="background: var(--calc-accent); color: white;"
			>
				<IconCopy size={13} aria-hidden="true" />
				{copied ? 'Copied!' : 'Copy Embed Code'}
			</button>
		</div>
	</div>
{/if}

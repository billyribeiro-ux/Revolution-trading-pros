<script lang="ts">
	import { Link2, Copy, ExternalLink, X } from '@lucide/svelte';
	import gsap from 'gsap';
	import { generateShareURL, copyToClipboard } from '../../utils/share-utils.js';
	import type { CalculatorState } from '../../state/calculator.svelte.js';

	interface Props {
		calc: CalculatorState;
	}

	let { calc }: Props = $props();

	let modalEl: HTMLDivElement | undefined = $state();
	let copied = $state(false);

	let shareURL = $derived(
		generateShareURL({
			inputs: calc.inputs,
			optionType: calc.optionType,
			mode: calc.calculatorMode,
			activeTab: calc.activeTab,
			ticker: calc.activeTicker || undefined,
			strategyLegs: calc.strategyLegs.length > 0 ? calc.strategyLegs : undefined,
		}),
	);

	let displayURL = $derived(
		shareURL.length > 80 ? shareURL.slice(0, 77) + '\u2026' : shareURL,
	);

	$effect(() => {
		if (calc.showShareLink && modalEl) {
			copied = false;
			gsap.fromTo(
				modalEl,
				{ scale: 0.92, opacity: 0 },
				{ scale: 1, opacity: 1, duration: 0.25, ease: 'back.out(1.5)' },
			);
		}
	});

	async function handleCopy(): Promise<void> {
		const success = await copyToClipboard(shareURL);
		if (success) {
			copied = true;
			calc.addToast('success', 'Link copied to clipboard!');
			setTimeout(() => (copied = false), 2500);
		} else {
			calc.addToast('error', 'Failed to copy link.');
		}
	}

	function openInNewTab(): void {
		window.open(shareURL, '_blank', 'noopener');
	}

	function handleClose(): void {
		calc.showShareLink = false;
	}

	function handleKeydown(e: KeyboardEvent): void {
		if (!calc.showShareLink) return;
		if (e.key === 'Escape') handleClose();
	}
</script>

<svelte:window onkeydown={handleKeydown} />

{#if calc.showShareLink}
	<div
		class="fixed inset-0 z-[9995] flex items-center justify-center p-4"
		role="dialog"
		aria-modal="true"
		aria-label="Share Calculator Link"
	>
		<button
			class="absolute inset-0 cursor-default"
			style="background: rgba(0,0,0,0.6); backdrop-filter: blur(4px);"
			onclick={handleClose}
			aria-label="Close"
			tabindex={-1}
		></button>

		<div
			bind:this={modalEl}
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
					<Link2 size={16} style="color: var(--calc-accent);" />
					<h3
						class="text-sm font-semibold"
						style="color: var(--calc-text); font-family: var(--calc-font-display);"
					>Share Calculator</h3>
				</div>
				<button
					onclick={handleClose}
					class="cursor-pointer p-1 rounded-md"
					style="color: var(--calc-text-muted);"
					aria-label="Close"
				>
					<X size={16} />
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
					onclick={handleCopy}
					class="flex-1 flex items-center justify-center gap-1.5 py-2.5 rounded-lg text-xs font-semibold cursor-pointer transition-all duration-200"
					style="background: var(--calc-accent); color: white;"
				>
					<Copy size={13} />
					{copied ? 'Copied!' : 'Copy Link'}
				</button>
				<button
					onclick={openInNewTab}
					class="flex items-center justify-center gap-1.5 px-4 py-2.5 rounded-lg text-xs font-medium cursor-pointer"
					style="background: var(--calc-surface-hover); color: var(--calc-text-secondary); border: 1px solid var(--calc-border);"
				>
					<ExternalLink size={13} />
					Open
				</button>
			</div>
		</div>
	</div>
{/if}

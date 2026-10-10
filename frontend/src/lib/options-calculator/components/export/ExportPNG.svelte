<script lang="ts">
	import IconCamera from '@tabler/icons-svelte-runes/icons/camera';
	import IconX from '@tabler/icons-svelte-runes/icons/x';
	import IconCopy from '@tabler/icons-svelte-runes/icons/copy';
	import IconDownload from '@tabler/icons-svelte-runes/icons/download';
	import IconChartBar from '@tabler/icons-svelte-runes/icons/chart-bar';
	import IconChartLine from '@tabler/icons-svelte-runes/icons/chart-line';
	import IconMaximize from '@tabler/icons-svelte-runes/icons/maximize';
	import { backOut } from 'svelte/easing';
	import { prefersReducedMotion } from 'svelte/motion';
	import { scale } from 'svelte/transition';
	import type { Attachment } from 'svelte/attachments';
	import { captureScreenshot, downloadBlob } from '../../utils/export-utils.js';
	import { getCalculator } from '../../state/context.js';
	import type { CaptureZone, AspectRatio } from '../../engine/types.js';

	interface Props {
		/** The element to screenshot; absent until the calculator layout has mounted. */
		captureElement?: HTMLElement | null;
	}

	let { captureElement = null }: Props = $props();

	const calc = getCalculator();

	const DEFAULT_ZONE: CaptureZone = 'results-chart';
	const DEFAULT_RATIO: AspectRatio = 'auto';

	let zone = $state<CaptureZone>(DEFAULT_ZONE);
	let aspectRatio = $state<AspectRatio>(DEFAULT_RATIO);
	let showLogo = $state(true);
	let showInfoBar = $state(true);
	let showFrame = $state(true);
	let isCapturing = $state(false);
	let capturedBlob = $state.raw<Blob | null>(null);

	/**
	 * Bumped every time the dialog closes, so a capture still running when its dialog was
	 * closed cannot land in (or toast over) the next one.
	 */
	let session = 0;

	interface ZoneOption {
		id: CaptureZone;
		label: string;
		icon: typeof IconChartBar;
	}

	const ZONES: ZoneOption[] = [
		{ id: 'results-only', label: 'Results', icon: IconChartBar },
		{ id: 'results-chart', label: 'Results + Chart', icon: IconChartLine },
		{ id: 'chart-only', label: 'Chart Only', icon: IconChartLine },
		{ id: 'full-calculator', label: 'Full', icon: IconMaximize }
	];

	interface RatioOption {
		id: AspectRatio;
		label: string;
	}

	const RATIOS: RatioOption[] = [
		{ id: 'auto', label: 'Auto' },
		{ id: '16:9', label: '16:9' },
		{ id: '4:5', label: '4:5' },
		{ id: '1:1', label: '1:1' }
	];

	function buildSummaryText(): string {
		const type = calc.optionType === 'call' ? 'Call' : 'Put';
		const price = calc.currentPrice.toFixed(2);
		const delta = calc.currentGreeks.first.delta.toFixed(2);
		const theta = calc.currentGreeks.first.theta.toFixed(4);
		return `${type} $${price} | Δ ${delta} | Θ ${theta}`;
	}

	async function handleCapture(): Promise<void> {
		if (!captureElement) {
			calc.addToast('error', 'No element to capture. Try again.');
			return;
		}
		const current = session;
		isCapturing = true;

		const blob = await captureScreenshot(captureElement, {
			zone,
			aspectRatio,
			showLogo,
			showInfoBar,
			showFrame,
			ticker: calc.activeTicker || undefined,
			summaryText: buildSummaryText(),
			theme: calc.theme
		});

		// The dialog was closed while capturing: that session is over.
		if (current !== session) return;
		isCapturing = false;

		if (blob) {
			capturedBlob = blob;
			calc.addToast('success', 'Screenshot captured!');
		} else {
			calc.addToast('error', 'Screenshot failed. Please try again.');
		}
	}

	async function copyToClipboard(): Promise<void> {
		if (!capturedBlob) return;
		try {
			await navigator.clipboard.write([new ClipboardItem({ 'image/png': capturedBlob })]);
			calc.addToast('success', 'Copied to clipboard!');
		} catch {
			calc.addToast('error', 'Copy failed — try saving instead.');
		}
	}

	function saveToDevice(): void {
		if (!capturedBlob) return;
		downloadBlob(
			capturedBlob,
			`options-analysis-${calc.activeTicker || 'custom'}-${Date.now()}.png`
		);
		calc.addToast('success', 'Saved to device!');
	}

	/**
	 * The only way the dialog closes (backdrop, close button, Escape). Ending the session here
	 * means every opening starts from the default settings.
	 */
	function handleClose(): void {
		session++;
		calc.showExportPNG = false;
		capturedBlob = null;
		isCapturing = false;
		zone = DEFAULT_ZONE;
		aspectRatio = DEFAULT_RATIO;
		showLogo = true;
		showInfoBar = true;
		showFrame = true;
	}

	function handleKeydown(event: KeyboardEvent): void {
		if (calc.showExportPNG && event.key === 'Escape') handleClose();
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

{#if calc.showExportPNG}
	<div
		{@attach focusDialog}
		class="fixed inset-0 z-[9995] flex items-center justify-center p-4 outline-none"
		role="dialog"
		aria-modal="true"
		aria-label="Export PNG Screenshot"
		tabindex="-1"
	>
		<!-- Backdrop -->
		<button
			type="button"
			class="absolute inset-0 cursor-default"
			style="background: rgba(0,0,0,0.6); backdrop-filter: blur(4px);"
			onclick={handleClose}
			aria-label="Close"
			tabindex={-1}
		></button>

		<!-- Modal -->
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
					<IconCamera size={16} style="color: var(--calc-accent);" aria-hidden="true" />
					<h3
						class="text-sm font-semibold"
						style="color: var(--calc-text); font-family: var(--calc-font-display);"
					>
						Export Screenshot
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

			{#if !capturedBlob}
				<!-- Capture Zone Selector -->
				<div class="flex flex-col gap-1.5">
					<span
						class="text-[10px] uppercase tracking-wider font-medium"
						style="color: var(--calc-text-muted);">Capture Zone</span
					>
					<div class="grid grid-cols-4 gap-1.5">
						{#each ZONES as z (z.id)}
							{const Icon = z.icon}
							{const selected = $derived(zone === z.id)}
							<button
								type="button"
								onclick={() => (zone = z.id)}
								class="flex flex-col items-center gap-1 py-2.5 rounded-lg text-[10px] font-medium cursor-pointer transition-all duration-150"
								style:background={selected
									? 'var(--calc-accent-glow)'
									: 'var(--calc-surface-hover)'}
								style:color={selected ? 'var(--calc-accent)' : 'var(--calc-text-muted)'}
								style:border="1px solid {selected ? 'var(--calc-accent)' : 'var(--calc-border)'}"
								aria-pressed={selected}
							>
								<Icon size={14} aria-hidden="true" />
								{z.label}
							</button>
						{/each}
					</div>
				</div>

				<!-- Aspect Ratio Selector -->
				<div class="flex flex-col gap-1.5">
					<span
						class="text-[10px] uppercase tracking-wider font-medium"
						style="color: var(--calc-text-muted);">Aspect Ratio</span
					>
					<div class="grid grid-cols-4 gap-1.5">
						{#each RATIOS as r (r.id)}
							{const selected = $derived(aspectRatio === r.id)}
							<button
								type="button"
								onclick={() => (aspectRatio = r.id)}
								class="py-2 rounded-lg text-[10px] font-medium cursor-pointer transition-all duration-150"
								style:background={selected
									? 'var(--calc-accent-glow)'
									: 'var(--calc-surface-hover)'}
								style:color={selected ? 'var(--calc-accent)' : 'var(--calc-text-muted)'}
								style:border="1px solid {selected ? 'var(--calc-accent)' : 'var(--calc-border)'}"
								aria-pressed={selected}
							>
								{r.label}
							</button>
						{/each}
					</div>
				</div>

				<!-- Branding Toggles -->
				<div class="flex flex-col gap-2">
					<span
						class="text-[10px] uppercase tracking-wider font-medium"
						style="color: var(--calc-text-muted);">Branding</span
					>
					<label
						class="flex items-center gap-2 text-xs cursor-pointer"
						style="color: var(--calc-text-secondary);"
					>
						<input type="checkbox" bind:checked={showLogo} class="accent-[var(--calc-accent)]" />
						Show RTP Logo
					</label>
					<label
						class="flex items-center gap-2 text-xs cursor-pointer"
						style="color: var(--calc-text-secondary);"
					>
						<input type="checkbox" bind:checked={showInfoBar} class="accent-[var(--calc-accent)]" />
						Show Info Bar (ticker, price, timestamp)
					</label>
					<label
						class="flex items-center gap-2 text-xs cursor-pointer"
						style="color: var(--calc-text-secondary);"
					>
						<input type="checkbox" bind:checked={showFrame} class="accent-[var(--calc-accent)]" />
						Show Gradient Frame
					</label>
				</div>

				<!-- Capture Button -->
				<button
					type="button"
					onclick={handleCapture}
					disabled={isCapturing}
					aria-busy={isCapturing}
					class="flex items-center justify-center gap-2 w-full py-3 rounded-xl text-sm font-semibold cursor-pointer transition-all duration-200"
					style:background="var(--calc-accent)"
					style:color="white"
					style:opacity={isCapturing ? 0.6 : 1}
				>
					{#if isCapturing}
						<div
							class="w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin"
							aria-hidden="true"
						></div>
						Capturing...
					{:else}
						<IconCamera size={16} aria-hidden="true" />
						Capture Screenshot
					{/if}
				</button>
			{:else}
				<!-- Success State -->
				<div class="flex flex-col items-center gap-3 py-6">
					<div
						class="w-14 h-14 rounded-full flex items-center justify-center"
						style="background: rgba(16,185,129,0.1);"
					>
						<IconCamera size={24} style="color: #10b981;" aria-hidden="true" />
					</div>
					<p class="text-sm font-semibold" style="color: var(--calc-text);">Screenshot Ready!</p>
				</div>

				<div class="grid grid-cols-2 gap-2">
					<button
						type="button"
						onclick={copyToClipboard}
						class="flex items-center justify-center gap-1.5 py-2.5 rounded-lg text-xs font-medium cursor-pointer"
						style="background: var(--calc-surface-hover); color: var(--calc-text-secondary); border: 1px solid var(--calc-border);"
					>
						<IconCopy size={13} aria-hidden="true" />
						Copy to Clipboard
					</button>
					<button
						type="button"
						onclick={saveToDevice}
						class="flex items-center justify-center gap-1.5 py-2.5 rounded-lg text-xs font-medium cursor-pointer"
						style="background: var(--calc-accent); color: white;"
					>
						<IconDownload size={13} aria-hidden="true" />
						Save to Device
					</button>
				</div>

				<button
					type="button"
					onclick={() => (capturedBlob = null)}
					class="text-[10px] cursor-pointer self-center mt-1"
					style="color: var(--calc-text-muted);"
				>
					Recapture with different settings
				</button>
			{/if}
		</div>
	</div>
{/if}

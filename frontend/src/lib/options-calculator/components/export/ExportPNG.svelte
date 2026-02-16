<script lang="ts">
	import {
		Camera,
		X,
		Copy,
		Download,
		Monitor,
		BarChart3,
		LineChart,
		Maximize
	} from '@lucide/svelte';
	import gsap from 'gsap';
	import { captureScreenshot } from '../../utils/export-utils.js';
	import type { CaptureZone, AspectRatio, ScreenshotConfig } from '../../engine/types.js';
	import type { CalculatorState } from '../../state/calculator.svelte.js';

	interface Props {
		calc: CalculatorState;
		captureElement?: HTMLElement | null;
	}

	let { calc, captureElement = null }: Props = $props();

	let modalEl: HTMLDivElement | undefined = $state();
	let isCapturing = $state(false);
	let capturedBlob = $state<Blob | null>(null);

	let zone = $state<CaptureZone>('results-chart');
	let aspectRatio = $state<AspectRatio>('auto');
	let showLogo = $state(true);
	let showInfoBar = $state(true);
	let showFrame = $state(true);

	$effect(() => {
		if (calc.showExportPNG && modalEl) {
			capturedBlob = null;
			isCapturing = false;
			zone = 'results-chart';
			aspectRatio = 'auto';
			showLogo = true;
			showInfoBar = true;
			showFrame = true;
			gsap.fromTo(
				modalEl,
				{ scale: 0.92, opacity: 0 },
				{ scale: 1, opacity: 1, duration: 0.25, ease: 'back.out(1.5)' }
			);
		}
	});

	interface ZoneOption {
		id: CaptureZone;
		label: string;
		icon: typeof Monitor;
	}

	const ZONES: ZoneOption[] = [
		{ id: 'results-only', label: 'Results', icon: BarChart3 },
		{ id: 'results-chart', label: 'Results + Chart', icon: LineChart },
		{ id: 'chart-only', label: 'Chart Only', icon: LineChart },
		{ id: 'full-calculator', label: 'Full', icon: Maximize }
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
		return `${type} $${price} | \u0394 ${delta} | \u0398 ${theta}`;
	}

	async function handleCapture(): Promise<void> {
		if (!captureElement) {
			calc.addToast('error', 'No element to capture. Try again.');
			return;
		}
		isCapturing = true;

		const config: Partial<ScreenshotConfig> = {
			zone,
			aspectRatio,
			showLogo,
			showInfoBar,
			showFrame,
			ticker: calc.activeTicker || undefined,
			summaryText: buildSummaryText(),
			theme: calc.theme
		};

		const blob = await captureScreenshot(captureElement, config);
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
			calc.addToast('error', 'Copy failed \u2014 try saving instead.');
		}
	}

	function saveToDevice(): void {
		if (!capturedBlob) return;
		const url = URL.createObjectURL(capturedBlob);
		const a = document.createElement('a');
		a.href = url;
		a.download = `options-analysis-${calc.activeTicker || 'custom'}-${Date.now()}.png`;
		a.click();
		URL.revokeObjectURL(url);
		calc.addToast('success', 'Saved to device!');
	}

	function handleClose(): void {
		capturedBlob = null;
		calc.showExportPNG = false;
	}
</script>

{#if calc.showExportPNG}
	<div
		class="fixed inset-0 z-[9995] flex items-center justify-center p-4"
		role="dialog"
		aria-modal="true"
		aria-label="Export PNG Screenshot"
	>
		<!-- Backdrop -->
		<button
			class="absolute inset-0 cursor-default"
			style="background: rgba(0,0,0,0.6); backdrop-filter: blur(4px);"
			onclick={handleClose}
			aria-label="Close"
			tabindex={-1}
		></button>

		<!-- Modal -->
		<div
			bind:this={modalEl}
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
					<Camera size={16} style="color: var(--calc-accent);" />
					<h3
						class="text-sm font-semibold"
						style="color: var(--calc-text); font-family: var(--calc-font-display);"
					>
						Export Screenshot
					</h3>
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

			{#if !capturedBlob}
				<!-- Capture Zone Selector -->
				<div class="flex flex-col gap-1.5">
					<span
						class="text-[10px] uppercase tracking-wider font-medium"
						style="color: var(--calc-text-muted);">Capture Zone</span
					>
					<div class="grid grid-cols-4 gap-1.5">
						{#each ZONES as z (z.id)}
							{@const Icon = z.icon}
							<button
								onclick={() => (zone = z.id)}
								class="flex flex-col items-center gap-1 py-2.5 rounded-lg text-[10px] font-medium cursor-pointer transition-all duration-150"
								style={zone === z.id
									? 'background: var(--calc-accent-glow); color: var(--calc-accent); border: 1px solid var(--calc-accent);'
									: 'background: var(--calc-surface-hover); color: var(--calc-text-muted); border: 1px solid var(--calc-border);'}
							>
								<Icon size={14} />
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
							<button
								onclick={() => (aspectRatio = r.id)}
								class="py-2 rounded-lg text-[10px] font-medium cursor-pointer transition-all duration-150"
								style={aspectRatio === r.id
									? 'background: var(--calc-accent-glow); color: var(--calc-accent); border: 1px solid var(--calc-accent);'
									: 'background: var(--calc-surface-hover); color: var(--calc-text-muted); border: 1px solid var(--calc-border);'}
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
					onclick={handleCapture}
					disabled={isCapturing}
					class="flex items-center justify-center gap-2 w-full py-3 rounded-xl text-sm font-semibold cursor-pointer transition-all duration-200"
					style="background: var(--calc-accent); color: white; opacity: {isCapturing ? 0.6 : 1};"
				>
					{#if isCapturing}
						<div
							class="w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin"
						></div>
						Capturing...
					{:else}
						<Camera size={16} />
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
						<Camera size={24} style="color: var(--color-success);" />
					</div>
					<p class="text-sm font-semibold" style="color: var(--calc-text);">Screenshot Ready!</p>
				</div>

				<div class="grid grid-cols-2 gap-2">
					<button
						onclick={copyToClipboard}
						class="flex items-center justify-center gap-1.5 py-2.5 rounded-lg text-xs font-medium cursor-pointer"
						style="background: var(--calc-surface-hover); color: var(--calc-text-secondary); border: 1px solid var(--calc-border);"
					>
						<Copy size={13} />
						Copy to Clipboard
					</button>
					<button
						onclick={saveToDevice}
						class="flex items-center justify-center gap-1.5 py-2.5 rounded-lg text-xs font-medium cursor-pointer"
						style="background: var(--calc-accent); color: white;"
					>
						<Download size={13} />
						Save to Device
					</button>
				</div>

				<button
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

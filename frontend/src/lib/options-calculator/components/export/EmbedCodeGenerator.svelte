<script lang="ts">
	import { Code2, Copy, X } from '@lucide/svelte';
	import gsap from 'gsap';
	import { copyToClipboard } from '../../utils/share-utils.js';
	import type { CalculatorState } from '../../state/calculator.svelte.js';

	interface Props {
		calc: CalculatorState;
	}

	let { calc }: Props = $props();

	let modalEl: HTMLDivElement | undefined = $state();
	let copied = $state(false);
	let embedWidth = $state('100%');
	let embedHeight = $state('700');
	let embedTheme = $state<'dark' | 'light'>('dark');

	$effect(() => {
		if (calc.showEmbedCode && modalEl) {
			copied = false;
			gsap.fromTo(
				modalEl,
				{ scale: 0.92, opacity: 0 },
				{ scale: 1, opacity: 1, duration: 0.25, ease: 'back.out(1.5)' }
			);
		}
	});

	let embedURL = $derived.by(() => {
		const base =
			typeof window !== 'undefined'
				? `${window.location.origin}/tools/options-calculator/embed`
				: '/tools/options-calculator/embed';
		const params = new URLSearchParams();
		params.set('theme', embedTheme);
		if (calc.activeTicker) params.set('ticker', calc.activeTicker);
		return `${base}?${params.toString()}`;
	});

	let embedCode = $derived(
		`<iframe src="${embedURL}" width="${embedWidth}" height="${embedHeight}px" frameborder="0" style="border-radius:12px;border:1px solid var(--color-border-subtle);" title="Black-Scholes Options Calculator" loading="lazy"></iframe>\n<p style="font-size:11px;color:var(--color-text-muted);margin-top:4px;">Powered by <a href="https://revolutiontradingpros.com" target="_blank" rel="noopener">Revolution Trading Pros</a></p>`
	);

	async function handleCopy(): Promise<void> {
		const success = await copyToClipboard(embedCode);
		if (success) {
			copied = true;
			calc.addToast('success', 'Embed code copied!');
			setTimeout(() => (copied = false), 2500);
		} else {
			calc.addToast('error', 'Failed to copy embed code.');
		}
	}

	function handleClose(): void {
		calc.showEmbedCode = false;
	}

	function handleKeydown(e: KeyboardEvent): void {
		if (!calc.showEmbedCode) return;
		if (e.key === 'Escape') handleClose();
	}
</script>

<svelte:window onkeydown={handleKeydown} />

{#if calc.showEmbedCode}
	<div
		class="fixed inset-0 z-[9995] flex items-center justify-center p-4"
		role="dialog"
		aria-modal="true"
		aria-label="Embed Code Generator"
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
					<Code2 size={16} style="color: var(--calc-accent);" />
					<h3
						class="text-sm font-semibold"
						style="color: var(--calc-text); font-family: var(--calc-font-display);"
					>
						Embed Calculator
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
				onclick={handleCopy}
				class="flex items-center justify-center gap-1.5 w-full py-2.5 rounded-xl text-xs font-semibold cursor-pointer transition-all duration-200"
				style="background: var(--calc-accent); color: white;"
			>
				<Copy size={13} />
				{copied ? 'Copied!' : 'Copy Embed Code'}
			</button>
		</div>
	</div>
{/if}

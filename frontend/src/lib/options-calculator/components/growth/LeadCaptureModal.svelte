<script lang="ts">
	import IconMail from '@tabler/icons-svelte-runes/icons/mail';
	import IconX from '@tabler/icons-svelte-runes/icons/x';
	import IconTrendingUp from '@tabler/icons-svelte-runes/icons/trending-up';
	import { backOut, quadIn } from 'svelte/easing';
	import { prefersReducedMotion } from 'svelte/motion';
	import type { Attachment } from 'svelte/attachments';
	import type { EasingFunction, TransitionConfig } from 'svelte/transition';

	const DISMISS_KEY = 'rtp:calc:lead-dismissed';
	const SHOWN_KEY = 'rtp:calc:lead-shown-session';
	const DISMISS_DAYS = 30;
	const DAY_MS = 24 * 60 * 60 * 1000;
	const TRIGGER_DELAY_MS = 120_000; // 2 minutes
	/** How long the "You're In!" confirmation stays up before the modal closes itself. */
	const AUTO_CLOSE_MS = 2000;

	type StorageArea = 'localStorage' | 'sessionStorage';

	let email = $state('');
	let isSubmitting = $state(false);
	let submitted = $state(false);
	let showModal = $state(false);

	const isValidEmail = $derived(email.trim().length > 3 && email.includes('@'));

	/** Storage may be blocked (private mode, sandboxed embeds); even touching it can throw. */
	function readStorage(area: StorageArea, key: string): string | null {
		try {
			return window[area].getItem(key);
		} catch {
			return null;
		}
	}

	function writeStorage(area: StorageArea, key: string, value: string): void {
		try {
			window[area].setItem(key, value);
		} catch {
			// Not persisted: the prompt may simply show again later.
		}
	}

	/** True when the visitor dismissed (or subscribed) within the last `DISMISS_DAYS` days. */
	function dismissedRecently(): boolean {
		const dismissedAt = Number(readStorage('localStorage', DISMISS_KEY));
		return dismissedAt > 0 && Date.now() - dismissedAt < DISMISS_DAYS * DAY_MS;
	}

	function rememberDismissal(): void {
		writeStorage('localStorage', DISMISS_KEY, Date.now().toString());
	}

	/** Shows the offer once per session, unless it was dismissed (or used) recently. */
	function offerNewsletter(): void {
		if (readStorage('sessionStorage', SHOWN_KEY) || dismissedRecently()) return;
		showModal = true;
		writeStorage('sessionStorage', SHOWN_KEY, '1');
	}

	// A timer is an external side effect: start it on mount, cancel it on unmount.
	$effect(() => {
		const timer = setTimeout(offerNewsletter, TRIGGER_DELAY_MS);
		return () => clearTimeout(timer);
	});

	/** Closes the modal a moment after the confirmation appears; cancelled if it goes first. */
	const closeAfterConfirmation: Attachment = () => {
		const timer = setTimeout(() => (showModal = false), AUTO_CLOSE_MS);
		return () => clearTimeout(timer);
	};

	/** Scale, rise and fade together; instant under reduced motion. */
	function pop(
		_node: Element,
		{ duration = 300, easing = backOut }: { duration?: number; easing?: EasingFunction } = {}
	): TransitionConfig {
		return {
			duration: prefersReducedMotion.current ? 0 : duration,
			easing,
			css: (t, u) => `opacity: ${t}; transform: translateY(${u * 20}px) scale(${0.9 + 0.1 * t});`
		};
	}

	function handleDismiss(): void {
		showModal = false;
		rememberDismissal();
	}

	async function subscribe(address: string): Promise<void> {
		isSubmitting = true;
		try {
			// Goes through the generic /api/* proxy to the newsletter service.
			await fetch('/api/newsletter/subscribe', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ email: address, source: 'options-calculator' })
			});
		} catch {
			// Silently fail — the newsletter must never block the calculator.
		}
		isSubmitting = false;
		submitted = true;
		rememberDismissal();
	}

	function handleSubmit(event: SubmitEvent): void {
		event.preventDefault();
		const address = email.trim();
		if (!address.includes('@') || isSubmitting) return;
		void subscribe(address);
	}

	function handleKeydown(event: KeyboardEvent): void {
		if (showModal && event.key === 'Escape') handleDismiss();
	}

	/** Moves focus into the dialog when it opens and hands it back to the page on close. */
	const focusDialog: Attachment<HTMLElement> = (dialog) => {
		const opener = document.activeElement;
		dialog.focus();
		return () => {
			if (opener instanceof HTMLElement && opener.isConnected) opener.focus();
		};
	};
</script>

<svelte:window onkeydown={handleKeydown} />

{#if showModal}
	<div
		{@attach focusDialog}
		class="fixed inset-0 z-[9997] flex items-center justify-center p-4 outline-none"
		role="dialog"
		aria-modal="true"
		aria-label="Subscribe to trading insights"
		tabindex="-1"
	>
		<!-- Backdrop -->
		<button
			type="button"
			class="absolute inset-0 cursor-default"
			style="background: rgba(0,0,0,0.5); backdrop-filter: blur(4px);"
			onclick={handleDismiss}
			aria-label="Close"
			tabindex={-1}
		></button>

		<!-- Modal -->
		<div
			in:pop
			out:pop={{ duration: 200, easing: quadIn }}
			class="relative z-10 w-full max-w-sm rounded-2xl p-6 flex flex-col items-center gap-4"
			style="
				background: var(--calc-surface);
				border: 1px solid var(--calc-border);
				box-shadow: 0 24px 80px rgba(0,0,0,0.5);
			"
		>
			<!-- Close button -->
			<button
				type="button"
				onclick={handleDismiss}
				class="absolute top-3 right-3 cursor-pointer p-1 rounded-md"
				style="color: var(--calc-text-muted);"
				aria-label="Close"
			>
				<IconX size={16} aria-hidden="true" />
			</button>

			{#if !submitted}
				<!-- Icon -->
				<div
					class="w-14 h-14 rounded-full flex items-center justify-center"
					style="background: linear-gradient(135deg, rgba(99,102,241,0.15), rgba(0,212,170,0.15));"
				>
					<IconTrendingUp size={24} style="color: var(--calc-accent);" aria-hidden="true" />
				</div>

				<!-- Heading -->
				<h3
					class="text-base font-bold text-center"
					style="color: var(--calc-text); font-family: var(--calc-font-display);"
				>
					Get Weekly Options Analysis
				</h3>

				<p class="text-xs text-center leading-relaxed" style="color: var(--calc-text-muted);">
					Join 18,000+ traders receiving actionable options insights, strategy breakdowns, and
					market analysis every week.
				</p>

				<!-- Email form: Enter in the field submits (the browser skips it while disabled). -->
				<form class="w-full flex flex-col gap-1.5" onsubmit={handleSubmit}>
					<div class="flex gap-2">
						<label class="relative flex-1">
							<span class="sr-only">Email address</span>
							<IconMail
								size={14}
								class="absolute left-3 top-1/2 -translate-y-1/2"
								style="color: var(--calc-text-muted);"
								aria-hidden="true"
							/>
							<input
								type="email"
								bind:value={email}
								required
								autocomplete="email"
								placeholder="your@email.com"
								class="w-full text-xs pl-9 pr-3 py-2.5 rounded-lg outline-none"
								style="background: var(--calc-surface-hover); color: var(--calc-text); border: 1px solid var(--calc-border);"
							/>
						</label>
						<button
							type="submit"
							disabled={!isValidEmail || isSubmitting}
							class={[
								'text-xs font-semibold px-4 py-2.5 rounded-lg transition-all motion-reduce:transition-none duration-200 flex-shrink-0',
								isValidEmail && !isSubmitting ? 'cursor-pointer' : 'cursor-not-allowed'
							]}
							style:background="var(--calc-accent)"
							style:color="white"
							style:opacity={isValidEmail && !isSubmitting ? 1 : 0.5}
						>
							{isSubmitting ? '...' : 'Subscribe'}
						</button>
					</div>
				</form>

				<!-- No thanks -->
				<button
					type="button"
					onclick={handleDismiss}
					class="text-[10px] cursor-pointer"
					style="color: var(--calc-text-muted);"
				>
					No thanks, maybe later
				</button>
			{:else}
				<!-- Success State -->
				<div
					{@attach closeAfterConfirmation}
					class="w-14 h-14 rounded-full flex items-center justify-center"
					style="background: rgba(16,185,129,0.1);"
				>
					<IconMail size={24} style="color: #10b981;" aria-hidden="true" />
				</div>
				<h3
					class="text-base font-bold text-center"
					style="color: var(--calc-text); font-family: var(--calc-font-display);"
				>
					You're In!
				</h3>
				<p class="text-xs text-center" style="color: var(--calc-text-muted);">
					Check your inbox for a welcome email.
				</p>
			{/if}
		</div>
	</div>
{/if}

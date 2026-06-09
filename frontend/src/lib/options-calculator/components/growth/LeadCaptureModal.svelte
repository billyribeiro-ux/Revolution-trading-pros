<script lang="ts">
	import IconMail from '@tabler/icons-svelte-runes/icons/mail';
	import IconX from '@tabler/icons-svelte-runes/icons/x';
	import IconTrendingUp from '@tabler/icons-svelte-runes/icons/trending-up';
	import gsap from 'gsap';
	import { onDestroy, onMount } from 'svelte';
	import type { Attachment } from 'svelte/attachments';

	let modalEl: HTMLDivElement | undefined = $state();
	let email = $state('');
	let isSubmitting = $state(false);
	let submitted = $state(false);
	let showModal = $state(false);
	let closeTimer: ReturnType<typeof setTimeout> | null = null;

	const DISMISS_KEY = 'rtp:calc:lead-dismissed';
	const SHOWN_KEY = 'rtp:calc:lead-shown-session';
	const DISMISS_DAYS = 30;
	const TRIGGER_DELAY_MS = 120_000; // 2 minutes

	onMount(() => {
		// Already shown this session
		if (sessionStorage.getItem(SHOWN_KEY)) return;

		// Dismissed within DISMISS_DAYS
		const raw = localStorage.getItem(DISMISS_KEY);
		if (raw) {
			const dismissedAt = parseInt(raw, 10);
			const daysSince = (Date.now() - dismissedAt) / (1000 * 60 * 60 * 24);
			if (daysSince < DISMISS_DAYS) return;
		}

		const timer = setTimeout(() => {
			showModal = true;
			sessionStorage.setItem(SHOWN_KEY, '1');
		}, TRIGGER_DELAY_MS);

		return () => clearTimeout(timer);
	});

	onDestroy(() => {
		if (closeTimer) {
			clearTimeout(closeTimer);
		}
	});

	const animateModal: Attachment<HTMLDivElement> = (element) => {
		modalEl = element;
		const tween = gsap.fromTo(
			element,
			{ scale: 0.9, opacity: 0, y: 20 },
			{ scale: 1, opacity: 1, y: 0, duration: 0.3, ease: 'back.out(1.7)' }
		);

		return () => {
			tween.kill();
			if (modalEl === element) modalEl = undefined;
		};
	};

	function handleDismiss(): void {
		if (modalEl) {
			gsap.to(modalEl, {
				scale: 0.9,
				opacity: 0,
				y: 20,
				duration: 0.2,
				ease: 'power2.in',
				onComplete: () => {
					showModal = false;
					localStorage.setItem(DISMISS_KEY, Date.now().toString());
				}
			});
		} else {
			showModal = false;
			localStorage.setItem(DISMISS_KEY, Date.now().toString());
		}
	}

	async function handleSubmit(): Promise<void> {
		const trimmed = email.trim();
		if (!trimmed || !trimmed.includes('@')) return;

		isSubmitting = true;

		try {
			// POST to newsletter/CRM endpoint
			// Replace with your actual endpoint
			await fetch('/api/newsletter/subscribe', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ email: trimmed, source: 'options-calculator' })
			});
		} catch {
			// Silently fail — don't block UX for newsletter
		}

		isSubmitting = false;
		submitted = true;
		localStorage.setItem(DISMISS_KEY, Date.now().toString());

		// Auto-close after 2 seconds
		if (closeTimer) clearTimeout(closeTimer);
		closeTimer = setTimeout(() => {
			showModal = false;
			closeTimer = null;
		}, 2000);
	}

	function handleKeydown(e: KeyboardEvent): void {
		if (!showModal) return;
		if (e.key === 'Escape') handleDismiss();
		if (e.key === 'Enter' && !isSubmitting) {
			e.preventDefault();
			handleSubmit();
		}
	}

	let isValidEmail = $derived(email.trim().length > 3 && email.includes('@'));
</script>

<svelte:window onkeydown={handleKeydown} />

{#if showModal}
	<div
		class="fixed inset-0 z-[9997] flex items-center justify-center p-4"
		role="dialog"
		aria-modal="true"
		aria-label="Subscribe to trading insights"
	>
		<!-- Backdrop -->
		<button
			class="absolute inset-0 cursor-default"
			style="background: rgba(0,0,0,0.5); backdrop-filter: blur(4px);"
			onclick={handleDismiss}
			aria-label="Close"
			tabindex={-1}
		></button>

		<!-- Modal -->
		<div
			{@attach animateModal}
			class="relative z-10 w-full max-w-sm rounded-2xl p-6 flex flex-col items-center gap-4"
			style="
				background: var(--calc-surface);
				border: 1px solid var(--calc-border);
				box-shadow: 0 24px 80px rgba(0,0,0,0.5);
			"
		>
			<!-- Close button -->
			<button
				onclick={handleDismiss}
				class="absolute top-3 right-3 cursor-pointer p-1 rounded-md"
				style="color: var(--calc-text-muted);"
				aria-label="Close"
			>
				<IconX size={16} />
			</button>

			{#if !submitted}
				<!-- Icon -->
				<div
					class="w-14 h-14 rounded-full flex items-center justify-center"
					style="background: linear-gradient(135deg, rgba(99,102,241,0.15), rgba(0,212,170,0.15));"
				>
					<IconTrendingUp size={24} style="color: var(--calc-accent);" />
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

				<!-- Email Input -->
				<label class="w-full flex flex-col gap-1.5">
					<span class="sr-only">Email address</span>
					<div class="flex gap-2">
						<div class="relative flex-1">
							<IconMail
								size={14}
								class="absolute left-3 top-1/2 -translate-y-1/2"
								style="color: var(--calc-text-muted);"
							/>
							<input
								type="email"
								bind:value={email}
								placeholder="your@email.com"
								class="w-full text-xs pl-9 pr-3 py-2.5 rounded-lg outline-none"
								style="background: var(--calc-surface-hover); color: var(--calc-text); border: 1px solid var(--calc-border);"
							/>
						</div>
						<button
							onclick={handleSubmit}
							disabled={!isValidEmail || isSubmitting}
							class="text-xs font-semibold px-4 py-2.5 rounded-lg cursor-pointer transition-all duration-200 flex-shrink-0"
							style:background="var(--calc-accent)"
							style:color="white"
							style:opacity={isValidEmail && !isSubmitting ? 1 : 0.5}
						>
							{isSubmitting ? '...' : 'Subscribe'}
						</button>
					</div>
				</label>

				<!-- No thanks -->
				<button
					onclick={handleDismiss}
					class="text-[10px] cursor-pointer"
					style="color: var(--calc-text-muted);"
				>
					No thanks, maybe later
				</button>
			{:else}
				<!-- Success State -->
				<div
					class="w-14 h-14 rounded-full flex items-center justify-center"
					style="background: rgba(16,185,129,0.1);"
				>
					<IconMail size={24} style="color: #10b981;" />
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

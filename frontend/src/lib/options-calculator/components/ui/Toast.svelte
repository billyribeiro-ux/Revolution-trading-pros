<script lang="ts">
	import { X, CheckCircle, AlertCircle, Info, AlertTriangle } from '@lucide/svelte';
	import gsap from 'gsap';
	import type { ToastType } from '../../engine/types.js';
	import type { CalculatorState } from '../../state/calculator.svelte.js';

	interface Props {
		calc: CalculatorState;
	}

	let { calc }: Props = $props();

	const DEFAULT_DURATION = 3000;
	const MAX_VISIBLE = 3;

	let visibleToasts = $derived(calc.toasts.slice(0, MAX_VISIBLE));

	$effect(() => {
		const timers: ReturnType<typeof setTimeout>[] = [];

		for (const toast of calc.toasts) {
			const dur = toast.duration ?? DEFAULT_DURATION;
			if (dur > 0) {
				const timer = setTimeout(() => calc.removeToast(toast.id), dur);
				timers.push(timer);
			}
		}

		return () => {
			for (const timer of timers) clearTimeout(timer);
		};
	});

	function animateIn(el: HTMLElement) {
		gsap.fromTo(
			el,
			{ x: 80, opacity: 0, scale: 0.9 },
			{ x: 0, opacity: 1, scale: 1, duration: 0.35, ease: 'back.out(1.5)' },
		);
	}

	const ICONS: Record<ToastType, typeof CheckCircle> = {
		success: CheckCircle,
		error: AlertCircle,
		info: Info,
		warning: AlertTriangle,
	};

	const COLORS: Record<ToastType, { bg: string; border: string; icon: string }> = {
		success: { bg: 'rgba(16,185,129,0.08)', border: 'rgba(16,185,129,0.25)', icon: '#10b981' },
		error: { bg: 'rgba(239,68,68,0.08)', border: 'rgba(239,68,68,0.25)', icon: '#ef4444' },
		info: { bg: 'rgba(99,102,241,0.08)', border: 'rgba(99,102,241,0.25)', icon: '#6366f1' },
		warning: { bg: 'rgba(245,158,11,0.08)', border: 'rgba(245,158,11,0.25)', icon: '#f59e0b' },
	};

	function handleDismiss(id: string, el: HTMLElement) {
		gsap.to(el, {
			x: 80,
			opacity: 0,
			scale: 0.9,
			duration: 0.2,
			ease: 'power2.in',
			onComplete: () => calc.removeToast(id),
		});
	}
</script>

{#if visibleToasts.length > 0}
	<div
		class="fixed bottom-4 right-4 z-[10001] flex flex-col gap-2 pointer-events-none"
		aria-live="polite"
		aria-label="Notifications"
	>
		{#each visibleToasts as toast (toast.id)}
			{@const colors = COLORS[toast.type]}
			{@const Icon = ICONS[toast.type]}
			<div
				class="pointer-events-auto flex items-center gap-2.5 rounded-xl px-4 py-3 min-w-[280px] max-w-[400px] shadow-lg"
				style="background: {colors.bg}; border: 1px solid {colors.border}; backdrop-filter: blur(16px);"
				role="alert"
				use:animateIn
			>
				<Icon size={16} style="color: {colors.icon}; flex-shrink: 0;" />
				<span class="flex-1 text-xs font-medium" style="color: var(--calc-text);">
					{toast.message}
				</span>
				<button
					onclick={(e: MouseEvent) => {
						const el = (e.currentTarget as HTMLElement).closest('[role="alert"]') as HTMLElement;
						handleDismiss(toast.id, el);
					}}
					class="flex-shrink-0 cursor-pointer rounded-md p-0.5 transition-colors"
					style="color: var(--calc-text-muted);"
					aria-label="Dismiss notification"
				>
					<X size={12} />
				</button>
			</div>
		{/each}
	</div>
{/if}

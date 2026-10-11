<script lang="ts">
	import IconX from '@tabler/icons-svelte-runes/icons/x';
	import IconCircleCheck from '@tabler/icons-svelte-runes/icons/circle-check';
	import IconAlertCircle from '@tabler/icons-svelte-runes/icons/alert-circle';
	import IconInfoCircle from '@tabler/icons-svelte-runes/icons/info-circle';
	import IconAlertTriangle from '@tabler/icons-svelte-runes/icons/alert-triangle';
	import { backOut, cubicIn } from 'svelte/easing';
	import { prefersReducedMotion } from 'svelte/motion';
	import type { Attachment } from 'svelte/attachments';
	import type { TransitionConfig } from 'svelte/transition';
	import type { ToastItem, ToastType } from '../../engine/types.js';
	import { getCalculator } from '../../state/context.js';

	const calc = getCalculator();

	const DEFAULT_DURATION = 3000;
	const MAX_VISIBLE = 3;

	const visibleToasts = $derived(calc.toasts.slice(0, MAX_VISIBLE));

	const ICONS: Record<ToastType, typeof IconCircleCheck> = {
		success: IconCircleCheck,
		error: IconAlertCircle,
		info: IconInfoCircle,
		warning: IconAlertTriangle
	};

	const COLORS: Record<ToastType, { bg: string; border: string; icon: string }> = {
		success: { bg: 'rgba(16,185,129,0.08)', border: 'rgba(16,185,129,0.25)', icon: '#10b981' },
		error: { bg: 'rgba(239,68,68,0.08)', border: 'rgba(239,68,68,0.25)', icon: '#ef4444' },
		info: { bg: 'rgba(99,102,241,0.08)', border: 'rgba(99,102,241,0.25)', icon: '#6366f1' },
		warning: { bg: 'rgba(245,158,11,0.08)', border: 'rgba(245,158,11,0.25)', icon: '#f59e0b' }
	};

	interface MotionParams {
		duration: number;
		easing: (t: number) => number;
	}

	/** Slides in from the right while growing from 90%; reverses when the toast leaves. */
	function toastMotion(_node: Element, { duration, easing }: MotionParams): TransitionConfig {
		return {
			duration: prefersReducedMotion.current ? 0 : duration,
			easing,
			css: (t, u) => `transform: translateX(${80 * u}px) scale(${0.9 + 0.1 * t}); opacity: ${t};`
		};
	}

	/** Dismisses the toast after its duration; the countdown starts once it is on screen. */
	function autoDismiss(toast: ToastItem): Attachment {
		return () => {
			const duration = toast.duration ?? DEFAULT_DURATION;
			if (duration <= 0) return;
			const timer = setTimeout(() => calc.removeToast(toast.id), duration);
			return () => clearTimeout(timer);
		};
	}
</script>

<!-- Always mounted, so the last toast can play its exit transition. -->
<section
	class="fixed bottom-4 right-4 z-[10001] flex flex-col gap-2 pointer-events-none"
	aria-label="Notifications"
>
	{#each visibleToasts as toast (toast.id)}
		{const colors = $derived(COLORS[toast.type])}
		{const Icon = $derived(ICONS[toast.type])}
		<div
			{@attach autoDismiss(toast)}
			in:toastMotion={{ duration: 350, easing: backOut }}
			out:toastMotion={{ duration: 200, easing: cubicIn }}
			class="pointer-events-auto flex items-center gap-2.5 rounded-xl px-4 py-3 min-w-[280px] max-w-[400px] shadow-lg"
			style:background={colors.bg}
			style:border="1px solid {colors.border}"
			style:backdrop-filter="blur(16px)"
			role="alert"
		>
			<Icon size={16} color={colors.icon} class="shrink-0" aria-hidden="true" />
			<span class="flex-1 text-xs font-medium" style="color: var(--calc-text);">
				{toast.message}
			</span>
			<button
				type="button"
				onclick={() => calc.removeToast(toast.id)}
				class="flex-shrink-0 cursor-pointer rounded-md p-0.5 transition-colors motion-reduce:transition-none"
				style="color: var(--calc-text-muted);"
				aria-label="Dismiss notification"
			>
				<IconX size={12} aria-hidden="true" />
			</button>
		</div>
	{/each}
</section>

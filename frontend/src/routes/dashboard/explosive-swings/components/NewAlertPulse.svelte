<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * NewAlertPulse Component - Visual Pulse Indicator for New Alerts
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @description Animated pulse indicator to draw attention to new alerts
	 * @version 1.0.0 - Initial Implementation
	 * @standards Apple Principal Engineer ICT 7+ | WCAG 2.1 AA
	 */

	interface Props {
		/** Size variant of the pulse indicator */
		size?: 'sm' | 'md' | 'lg';
		/** Color variant based on alert type */
		variant?: 'entry' | 'update' | 'exit' | 'default';
		/** Whether to show the pulse animation */
		active?: boolean;
		/** Optional label to display */
		label?: string;
	}

	const { size = 'md', variant = 'default', active = true, label }: Props = $props();

	const sizeConfig = $derived.by(() => {
		switch (size) {
			case 'sm':
				return { dot: 8, ring: 16, fontSize: 10 };
			case 'lg':
				return { dot: 16, ring: 32, fontSize: 14 };
			case 'md':
			default:
				return { dot: 12, ring: 24, fontSize: 12 };
		}
	});

	const variantConfig = $derived.by(() => {
		switch (variant) {
			case 'entry':
				return {
					color: 'var(--color-entry, #14b8a6)',
					bgColor: 'var(--color-entry-bg, #ccfbf1)'
				};
			case 'update':
				return {
					color: 'var(--color-watching, #f59e0b)',
					bgColor: 'var(--color-watching-bg, #fef3c7)'
				};
			case 'exit':
				return {
					color: 'var(--color-profit, #10b981)',
					bgColor: 'var(--color-profit-bg, #d1fae5)'
				};
			case 'default':
			default:
				return {
					color: 'var(--color-brand-secondary, #f59e0b)',
					bgColor: 'var(--color-brand-secondary-bg, #fef3c7)'
				};
		}
	});
</script>

<div
	class="pulse-container"
	class:has-label={!!label}
	style="
		--pulse-color: {variantConfig.color};
		--pulse-bg: {variantConfig.bgColor};
		--dot-size: {sizeConfig.dot}px;
		--ring-size: {sizeConfig.ring}px;
		--label-font-size: {sizeConfig.fontSize}px;
	"
	role="status"
	aria-label={label || 'New alert indicator'}
>
	<div class="pulse-wrapper">
		{#if active}
			<span class="pulse-ring" aria-hidden="true"></span>
			<span class="pulse-ring pulse-ring-delayed" aria-hidden="true"></span>
		{/if}
		<span class="pulse-dot" class:active aria-hidden="true"></span>
	</div>
	{#if label}
		<span class="pulse-label">{label}</span>
	{/if}
</div>

<style>
	.pulse-container {
		display: inline-flex;
		align-items: center;
		gap: var(--space-2, 8px);
	}

	.pulse-wrapper {
		position: relative;
		display: flex;
		align-items: center;
		justify-content: center;
		width: var(--ring-size);
		height: var(--ring-size);
	}

	/* Dot */
	.pulse-dot {
		position: absolute;
		width: var(--dot-size);
		height: var(--dot-size);
		border-radius: 50%;
		background: var(--pulse-color);
		z-index: 1;
	}

	.pulse-dot.active {
		animation: dot-glow 2s ease-in-out infinite;
	}

	@keyframes dot-glow {
		0%,
		100% {
			box-shadow: 0 0 0 0 var(--pulse-color);
		}
		50% {
			box-shadow: 0 0 8px 2px var(--pulse-color);
		}
	}

	/* Rings */
	.pulse-ring {
		position: absolute;
		width: var(--dot-size);
		height: var(--dot-size);
		border-radius: 50%;
		background: var(--pulse-color);
		opacity: 0.6;
		animation: pulse-expand 2s cubic-bezier(0.4, 0, 0.2, 1) infinite;
	}

	.pulse-ring-delayed {
		animation-delay: 0.5s;
	}

	@keyframes pulse-expand {
		0% {
			transform: scale(1);
			opacity: 0.6;
		}
		100% {
			transform: scale(2.5);
			opacity: 0;
		}
	}

	/* Label */
	.pulse-label {
		font-size: var(--label-font-size);
		font-weight: var(--font-bold, 700);
		color: var(--pulse-color);
		text-transform: uppercase;
		letter-spacing: 0.05em;
		white-space: nowrap;
	}

	/* Container with label styling */
	.pulse-container.has-label {
		padding: var(--space-1, 4px) var(--space-2, 8px) var(--space-1, 4px) var(--space-1, 4px);
		background: var(--pulse-bg);
		border-radius: var(--radius-full, 9999px);
	}

	/* Reduced motion */
	@media (prefers-reduced-motion: reduce) {
		.pulse-ring {
			animation: none;
			opacity: 0.3;
			transform: scale(1.5);
		}

		.pulse-dot.active {
			animation: none;
		}
	}
</style>

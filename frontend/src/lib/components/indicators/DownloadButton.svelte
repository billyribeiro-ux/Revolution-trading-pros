<!--
	Download Button Component
	═══════════════════════════════════════════════════════════════════════════
	Reusable orange button for downloads and links
	Used in indicator pages for download and documentation links
-->
<script lang="ts">
	interface Props {
		href: string;
		text?: string;
		target?: string;
		rel?: string;
	}

	let { href, text = 'Click to Download', target, rel }: Props = $props();
</script>

<a class="orng_btn" {href} {target} {rel}>
	{text}
</a>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * Download Button - 2026 Mobile-First Responsive Design
	 * Touch targets: min 44x44px
	 * Breakpoints: xs(360px), sm(640px), md(768px), lg(1024px), xl(1280px)
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* Mobile-first base styles with touch-friendly targets */
	.orng_btn {
		background-color: #f8ac00;
		border-radius: 80px;
		font-weight: 700;
		color: #fff;
		/* Flexbox for better alignment */
		display: inline-flex;
		align-items: center;
		justify-content: center;
		text-align: center;
		/* Touch-friendly sizing - minimum 44px height */
		padding: 14px 24px;
		min-height: 44px;
		min-width: 140px;
		/* Responsive font size */
		font-size: clamp(14px, 3.5vw, 16px);
		text-decoration: none;
		transition: all 0.2s ease;
		/* Touch optimizations */
		-webkit-tap-highlight-color: transparent;
		user-select: none;
		-webkit-user-select: none;
		cursor: pointer;
		/* Prevent text wrapping on small screens */
		white-space: nowrap;
	}

	.orng_btn:hover {
		background-color: #df9c00;
		color: #fff;
		text-decoration: none;
	}

	/* Active state for touch feedback */
	.orng_btn:active {
		transform: scale(0.98);
		background-color: #c88a00;
	}

	/* Focus state for accessibility */
	.orng_btn:focus-visible {
		outline: 2px solid #f8ac00;
		outline-offset: 3px;
	}

	/* xs: Extra small devices (≥ 360px) */
	@media (min-width: 360px) {
		.orng_btn {
			min-width: 160px;
		}
	}

	/* sm: Small devices (≥ 640px) */
	@media (min-width: 640px) {
		.orng_btn {
			min-width: 200px;
			font-size: 17px;
			padding: 14px 28px;
		}
	}

	/* md: Medium devices (≥ 768px) */
	@media (min-width: 768px) {
		.orng_btn {
			min-width: 250px;
			font-size: 18px;
			padding: 14px 35px;
		}

		.orng_btn:hover {
			transform: translateY(-1px);
			box-shadow: 0 4px 12px rgba(248, 172, 0, 0.3);
		}
	}

	/* lg: Large devices (≥ 1024px) */
	@media (min-width: 1024px) {
		.orng_btn {
			font-size: 20px;
			padding: 16px 40px;
		}

		.orng_btn:hover {
			transform: translateY(-2px);
			box-shadow: 0 6px 16px rgba(248, 172, 0, 0.35);
		}
	}

	/* Touch device optimizations */
	@media (hover: none) and (pointer: coarse) {
		.orng_btn {
			/* Larger tap targets on touch devices */
			padding: 16px 28px;
			min-height: 48px;
		}

		.orng_btn:hover {
			transform: none;
			box-shadow: none;
		}
	}

	/* Reduced motion preference */
	@media (prefers-reduced-motion: reduce) {
		.orng_btn {
			transition: none;
		}

		.orng_btn:hover,
		.orng_btn:active {
			transform: none;
		}
	}

	/* High contrast mode support */
	@media (prefers-contrast: high) {
		.orng_btn {
			border: 2px solid #fff;
		}
	}
</style>

<!--
	Indicator Header Component
	═══════════════════════════════════════════════════════════════════════════
	Displays indicator name and available platforms
	Used across all indicator detail pages
-->
<script lang="ts">
	interface Props {
		name: string;
		platforms: string[];
	}

	let { name, platforms }: Props = $props();
</script>

<h1>{name}</h1>

<section>
	<p class="platforms">
		<strong>Available Platforms:</strong>
		{platforms.join(', ')}
	</p>
	<hr />
</section>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * Indicator Header - 2026 Mobile-First Responsive Design
	 * Breakpoints: xs(360px), sm(640px), md(768px), lg(1024px), xl(1280px)
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* Mobile-first base styles */
	h1 {
		color: #0c2434;
		font-weight: 700;
		text-align: center;
		margin-bottom: 8px;
		/* Responsive typography using clamp */
		font-size: clamp(24px, 6vw, 54px);
		font-family: 'Open Sans Condensed', sans-serif;
		line-height: 1.15;
		padding: 0 max(12px, env(safe-area-inset-left));
		word-wrap: break-word;
		hyphens: auto;
	}

	.platforms {
		text-align: center;
		margin-bottom: 24px;
		color: #666666;
		font-family: 'Open Sans', sans-serif;
		/* Responsive font size */
		font-size: clamp(14px, 3.5vw, 16px);
		line-height: 1.5;
		padding: 0 max(12px, env(safe-area-inset-left));
	}

	.platforms strong {
		color: #666666;
		font-weight: 700;
	}

	hr {
		border: none;
		border-top: 1px solid #dddddd;
		margin: 16px auto 24px;
		max-width: calc(100% - max(24px, env(safe-area-inset-left) + env(safe-area-inset-right)));
	}

	/* sm: Small devices (≥ 640px) */
	@media (min-width: 640px) {
		h1 {
			margin-bottom: 10px;
		}

		.platforms {
			margin-bottom: 32px;
		}

		hr {
			margin: 20px auto 32px;
		}
	}

	/* md: Medium devices (≥ 768px) */
	@media (min-width: 768px) {
		h1 {
			font-size: clamp(36px, 5vw, 54px);
			padding: 0;
		}

		.platforms {
			margin-bottom: 40px;
			font-size: 16px;
			padding: 0;
		}

		hr {
			margin: 20px 0 40px;
			max-width: 100%;
		}
	}

	/* lg: Large devices (≥ 1024px) */
	@media (min-width: 1024px) {
		h1 {
			font-size: 54px;
		}
	}

	/* Reduced motion preference */
	@media (prefers-reduced-motion: reduce) {
		h1,
		.platforms {
			transition: none;
		}
	}
</style>

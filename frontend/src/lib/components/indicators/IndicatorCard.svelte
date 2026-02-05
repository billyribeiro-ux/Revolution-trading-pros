<!--
	Indicator Card Component
	═══════════════════════════════════════════════════════════════════════════
	Reusable card component for displaying indicators
	Follows Svelte 5 best practices with $props() rune
	
	Props:
	- id: number
	- name: string
	- description?: string
	- platform: string
	- platformUrl: string
	- icon?: string
	- status: 'active' | 'inactive'
-->
<script lang="ts">
	interface Props {
		id: number;
		name: string;
		description?: string;
		platform: string;
		platformUrl: string;
		icon?: string;
		status: 'active' | 'inactive';
	}

	let { id, name, description, platform, platformUrl, icon, status }: Props = $props();
</script>

<article class="card-grid-spacer flex-grid-item col-xs-12 col-sm-6 col-md-6 col-lg-4">
	<div class="card flex-grid-panel">
		<section class="card-body u--squash">
			<h4 class="h5 card-title pb-1">
				<a href={platformUrl}>
					{name}
				</a>
			</h4>
			{#if description}
				<p class="article-card__meta">
					<small>{description}</small>
				</p>
			{/if}
		</section>
		<footer class="card-footer">
			<a class="btn btn-tiny btn-default" href={platformUrl}>Watch Now</a>
		</footer>
	</div>
</article>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * Indicator Card Styles - 2026 Mobile-First Responsive Design
	 * Breakpoints: xs(360px), sm(640px), md(768px), lg(1024px), xl(1280px)
	 * Touch targets: min 44x44px
	 * Safe areas: env(safe-area-inset-*)
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* Mobile-first base styles (< 640px) */
	.card-grid-spacer {
		width: 100%;
		padding-left: max(12px, env(safe-area-inset-left));
		padding-right: max(12px, env(safe-area-inset-right));
		margin-top: 16px;
		margin-bottom: 16px;
		box-sizing: border-box;
	}

	.flex-grid-item {
		display: flex;
		width: 100%;
	}

	.card {
		position: relative;
		background: #fff;
		border-radius: 8px;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.12);
		display: flex;
		flex-direction: column;
		width: 100%;
		/* ICT 7: Apple Standard - max 525px width, min 156px height */
		max-width: 525px;
		min-height: 156px;
		height: 100%;
		margin: 0 auto; /* Center card within column */
		transition: all 0.2s ease-in-out;
		padding-bottom: 72px; /* Increased for touch-friendly button */
	}

	.card:hover {
		box-shadow: 0 8px 24px rgba(0, 0, 0, 0.12);
		transform: translateY(-2px);
	}

	/* Active state for touch devices */
	.card:active {
		transform: translateY(0);
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
	}

	.flex-grid-panel {
		display: flex;
		flex-direction: column;
		height: 100%;
		width: 100%;
	}

	.card-body {
		padding: 16px;
		flex-grow: 0;
		display: flex;
		flex-direction: column;
		text-align: center;
		align-items: center;
		justify-content: center;
	}

	.card-body:last-of-type {
		flex-grow: 1;
	}

	.u--squash {
		padding-bottom: 8px;
	}

	/* Responsive typography - mobile first */
	.card-title {
		font-size: clamp(15px, 4vw, 18px);
		font-weight: 700;
		color: #333;
		margin: 0;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
		line-height: 1.4;
		text-align: center;
	}

	.card-title a {
		color: #333;
		text-decoration: none;
		transition: color 0.2s;
		display: block;
		text-align: center;
		/* Touch-friendly link area */
		padding: 4px 0;
		min-height: 44px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.card-title a:hover {
		color: #1d73be;
	}

	.h5 {
		font-size: clamp(15px, 4vw, 18px);
		font-weight: 600;
	}

	.pb-1 {
		padding-bottom: 0.5rem;
	}

	.article-card__meta {
		color: #999;
		font-size: clamp(12px, 3vw, 13px);
		margin: 8px 0 0;
		text-align: center;
		line-height: 1.5;
	}

	.article-card__meta small {
		font-size: inherit;
	}

	.card-footer {
		position: absolute;
		bottom: 0;
		left: 0;
		right: 0;
		padding: 0 16px 16px;
		padding-bottom: max(16px, env(safe-area-inset-bottom));
		text-align: center;
		display: flex;
		justify-content: center;
		align-items: center;
	}

	/* Touch-friendly button - minimum 44x44px */
	.btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		text-decoration: none;
		border-radius: 6px;
		font-weight: 700;
		font-family: 'Open Sans', sans-serif;
		transition: all 0.2s ease-in-out;
		text-align: center;
		cursor: pointer;
		border: none;
		/* Touch-friendly sizing */
		min-height: 44px;
		min-width: 44px;
		-webkit-tap-highlight-color: transparent;
	}

	.btn-tiny {
		font-size: 14px;
		line-height: 1.2;
		padding: 12px 16px;
		font-weight: 600;
	}

	.btn-default {
		background: #f4f4f4;
		color: #0984ae;
		border-color: transparent;
		box-shadow: none;
		transition: all 0.15s ease-in-out;
	}

	.btn-default:visited {
		color: #0984ae;
	}

	.btn-default:hover {
		color: #0984ae;
		background: #e7e7e7;
		border-color: transparent;
		box-shadow: none;
		text-decoration: none;
	}

	.btn-default:focus,
	.btn-default:active {
		color: #0984ae;
		background: #e7e7e7;
		border-color: transparent;
		outline: 0;
		box-shadow: 0 0 0 0.25rem rgba(13, 110, 253, 0.25);
		text-decoration: none;
	}

	/* Grid System - 2026 Mobile-First with Modern Breakpoints */
	:global(.col-xs-12),
	:global(.col-sm-6),
	:global(.col-md-6),
	:global(.col-lg-4) {
		width: 100%;
		padding-left: max(12px, env(safe-area-inset-left));
		padding-right: max(12px, env(safe-area-inset-right));
		box-sizing: border-box;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * Responsive Breakpoints - Mobile First (min-width)
	 * xs: 360px, sm: 640px, md: 768px, lg: 1024px, xl: 1280px
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* sm: Small devices (≥ 640px) - 2 columns */
	@media (min-width: 640px) {
		.card-grid-spacer {
			padding-left: 15px;
			padding-right: 15px;
			margin-top: 24px;
			margin-bottom: 24px;
		}

		:global(.col-sm-6) {
			width: 50%;
			max-width: 50%;
		}

		.card-body {
			padding: 20px;
		}

		.card-footer {
			padding: 0 20px 20px;
		}
	}

	/* md: Medium devices (≥ 768px) - 2 columns with more spacing */
	@media (min-width: 768px) {
		.card-grid-spacer {
			margin-top: 30px;
			margin-bottom: 30px;
		}

		:global(.col-md-6) {
			width: 50%;
			max-width: 50%;
		}

		.card-title {
			font-size: 18px;
		}

		.card {
			border-radius: 8px;
		}
	}

	/* lg: Large devices (≥ 1024px) - 3 columns */
	@media (min-width: 1024px) {
		:global(.col-lg-4) {
			width: 33.333333% !important;
			max-width: 33.333333% !important;
		}

		.card:hover {
			box-shadow: 0 12px 32px rgba(0, 0, 0, 0.15);
			transform: translateY(-4px);
		}
	}

	/* xl: Extra large devices (≥ 1280px) */
	@media (min-width: 1280px) {
		.card-grid-spacer {
			padding-left: 20px;
			padding-right: 20px;
		}
	}

	/* High DPI / Retina displays */
	@media (-webkit-min-device-pixel-ratio: 2), (min-resolution: 192dpi) {
		.card {
			box-shadow: 0 0.5px 2px rgba(0, 0, 0, 0.12);
		}
	}

	/* Reduced motion preference */
	@media (prefers-reduced-motion: reduce) {
		.card {
			transition: none;
		}
		.card:hover {
			transform: none;
		}
	}

	/* Touch device optimizations */
	@media (hover: none) and (pointer: coarse) {
		.card:hover {
			transform: none;
			box-shadow: 0 1px 3px rgba(0, 0, 0, 0.12);
		}

		.btn-tiny {
			padding: 14px 18px;
		}
	}
</style>

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
	 * Indicator Card Styles - WordPress Match with Modern Hover Effects
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.card-grid-spacer {
		padding-left: 15px;
		padding-right: 15px;
		margin-top: 30px;
		margin-bottom: 30px;
	}

	.flex-grid-item {
		display: flex;
	}

	.card {
		position: relative;
		background: #fff;
		border-radius: 5px;
		box-shadow: 0 1px 2px rgba(0, 0, 0, 0.15);
		display: flex;
		flex-direction: column;
		width: 100%;
		height: 100%;
		transition: all 0.15s ease-in-out;
		padding-bottom: 60px;
	}

	.card:hover {
		box-shadow: 0 5px 20px rgba(0, 0, 0, 0.1);
		transform: translateY(-2px);
	}

	.flex-grid-panel {
		display: flex;
		flex-direction: column;
		height: 100%;
		width: 100%;
	}

	.card-body {
		padding: 20px;
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
		padding-bottom: 10px;
	}

	.card-title {
		font-size: 18px;
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
	}

	.card-title a:hover {
		color: #143E59;
	}

	.h5 {
		font-size: 18px;
		font-weight: 600;
	}

	.pb-1 {
		padding-bottom: 0.5rem;
	}

	.article-card__meta {
		color: #999;
		font-size: 13px;
		margin: 8px 0 0;
		text-align: center;
	}

	.article-card__meta small {
		font-size: 13px;
	}

	.card-footer {
		position: absolute;
		bottom: 0;
		left: 0;
		right: 0;
		padding: 0 20px 20px;
		text-align: center;
		display: flex;
		justify-content: center;
		align-items: center;
	}

	.btn {
		display: inline-block;
		text-decoration: none;
		border-radius: 5px;
		font-weight: 700;
		font-family: 'Open Sans', sans-serif;
		transition: all 0.2s ease-in-out;
		text-align: center;
		cursor: pointer;
		border: none;
	}

	.btn-tiny {
		font-size: 14px;
		line-height: 16px;
		padding: 6px 8px;
		font-weight: 600;
	}

	.btn-default {
		background: #143E59;
		color: #ffffff;
		border: 1px solid #143E59;
		box-shadow: none;
		transition: all 0.2s ease;
		min-width: 120px;
	}

	.btn-default:visited {
		color: #ffffff;
	}

	.btn-default:hover {
		color: #ffffff;
		background: #0f2d42;
		border-color: #0f2d42;
		box-shadow: 0 2px 8px rgba(20, 62, 89, 0.3);
		text-decoration: none;
		transform: translateY(-1px);
	}

	.btn-default:focus,
	.btn-default:active {
		color: #ffffff;
		background: #0f2d42;
		border-color: #0f2d42;
		outline: 0;
		box-shadow: 0 0 0 3px rgba(20, 62, 89, 0.2);
		text-decoration: none;
	}

	/* Grid System - WordPress/Bootstrap Standard Breakpoints */
	:global(.col-xs-12),
	:global(.col-sm-6),
	:global(.col-md-6),
	:global(.col-lg-4) {
		width: 100%;
		padding-left: 15px;
		padding-right: 15px;
		box-sizing: border-box;
	}

	/* Small Tablet: 2 columns (≥ 576px) */
	@media (min-width: 576px) {
		:global(.col-sm-6) {
			width: 50%;
		}
	}

	/* Tablet: 2 columns (≥ 768px) */
	@media (min-width: 768px) {
		:global(.col-md-6) {
			width: 50%;
		}
	}

	/* Desktop: 3 columns (≥ 992px) - Standard Bootstrap lg breakpoint */
	@media (min-width: 992px) {
		:global(.col-lg-4) {
			width: 33.333333% !important;
		}
	}

	/* Responsive */
	@media (max-width: 768px) {
		.card-grid-spacer {
			margin-bottom: 20px;
		}
	}
</style>

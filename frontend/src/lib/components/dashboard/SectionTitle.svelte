<script lang="ts">
	/**
	 * SectionTitle - Simpler Trading Style
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Svelte 5 component for section headings:
	 * - Standard: Large dark text
	 * - Alt: Small blue uppercase with underline
	 *
	 * @version 1.0.0 - Svelte 5 with $props()
	 */

	interface Props {
		/** Title text */
		title: string;
		/** Variant: 'default' or 'alt' (blue uppercase) */
		variant?: 'default' | 'alt';
		/** Whether to show underline (alt variant only) */
		underline?: boolean;
		/** HTML tag to use */
		tag?: 'h1' | 'h2' | 'h3' | 'h4' | 'h5' | 'h6';
	}

	let {
		title,
		variant = 'default',
		underline = false,
		tag = 'h2'
	}: Props = $props();
</script>

{#if variant === 'alt'}
	<svelte:element
		this={tag}
		class="section-title-alt"
		class:section-title-alt--underline={underline}
	>
		{title}
	</svelte:element>
{:else}
	<svelte:element this={tag} class="section-title">
		{title}
	</svelte:element>
{/if}

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   SECTION TITLE - Default Style
	   ═══════════════════════════════════════════════════════════════════════════ */
	.section-title {
		color: #333 !important;
		font-size: 32px;
		font-weight: 700;
		font-family: 'Open Sans', sans-serif;
		line-height: 1.2;
		margin: 0 0 30px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   SECTION TITLE ALT - Blue Uppercase Style
	   ═══════════════════════════════════════════════════════════════════════════ */
	.section-title-alt {
		color: #0984ae;
		font-size: 11px;
		font-weight: 700;
		font-family: 'Open Sans', sans-serif;
		text-transform: uppercase;
		letter-spacing: 1px;
		margin: 0 0 15px;
	}

	.section-title-alt--underline {
		padding-bottom: 30px;
		position: relative;
	}

	.section-title-alt--underline::after {
		content: "";
		position: absolute;
		bottom: 2px;
		left: 0;
		width: 50px;
		height: 2px;
		background-color: #e8e8e8;
	}

	/* Responsive */
	@media (max-width: 767px) {
		.section-title {
			font-size: 24px;
			margin-bottom: 20px;
		}
	}
</style>

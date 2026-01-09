<!--
	Class Header Section Component - SSOT
	═══════════════════════════════════════════════════════════════════════════
	Standardized class header section for all class pages.
	This is the Single Source of Truth (SSOT) for class header sections.
	
	@version 1.0.0 - ICT 7 Standards (Nov 2025)
	@author Revolution Trading Pros
-->
<script lang="ts">
	// ICT 7 FIX: Svelte 5 $props() syntax (Nov 2025 best practice)
	interface Props {
		title: string;
		subtitle?: string;
		description?: string;
	}
	
	let { title, subtitle = '', description = '' }: Props = $props();
</script>

<section class="class-section cpost-section" id="class-info">
	<div class="section-inner">
		<div id="non-member-class-upsell">
			<div id="non-member-class-title">
				<h1>{title}</h1>
				{#if subtitle}
					<h3>{subtitle}</h3>
				{/if}
			</div>
			{#if description}
				<div id="non-member-class-description">
					<p>{description}</p>
				</div>
			{/if}
		</div>
	</div>
</section>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * Class Header Section - Apple ICT 7 Standards
	 * SSOT for all class header sections across the platform
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.class-section {
		margin-bottom: 40px;
	}

	.section-inner {
		width: 100%;
	}

	/* Title Section - Centered */
	#non-member-class-title h1 {
		font-size: 2.5rem;
		font-weight: 700;
		color: #333333;
		margin-bottom: 30px;
		line-height: 1.2;
		text-align: center;
	}

	#non-member-class-title h3 {
		font-size: 1.5rem;
		font-weight: 500;
		color: #666666;
		margin-bottom: 20px;
		text-align: center;
	}

	#non-member-class-description {
		text-align: center;
		margin-top: 20px;
	}

	#non-member-class-description p {
		font-size: 1.1rem;
		line-height: 1.6;
		color: #555555;
		max-width: 800px;
		margin: 0 auto;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * Responsive Design - Mobile First
	 * ═══════════════════════════════════════════════════════════════════════════ */

	@media (max-width: 768px) {
		#non-member-class-title h1 {
			font-size: 2rem;
		}

		#non-member-class-title h3 {
			font-size: 1.3rem;
		}

		#non-member-class-description p {
			font-size: 1rem;
		}
	}

	@media (max-width: 480px) {
		#non-member-class-title h1 {
			font-size: 1.75rem;
		}

		#non-member-class-title h3 {
			font-size: 1.1rem;
		}

		#non-member-class-description p {
			font-size: 0.95rem;
		}
	}
</style>

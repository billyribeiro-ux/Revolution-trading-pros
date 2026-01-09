<!--
	Class Downloads Section Component - SSOT
	═══════════════════════════════════════════════════════════════════════════
	Standardized Box.com iframe downloads section for all class pages.
	This is the Single Source of Truth (SSOT) for class download sections.
	
	@version 1.0.0 - ICT 7 Standards (Nov 2025)
	@author Revolution Trading Pros
-->
<script lang="ts">
	// ICT 7 FIX: Svelte 5 $props() syntax (Nov 2025 best practice)
	interface Props {
		boxUrl: string;
		title?: string;
	}
	
	let { boxUrl, title = 'Class Downloads' }: Props = $props();
</script>

<section class="class-section cpost-section" id="dl-rp-row">
	<div class="section-inner">
		<section class="class-subsection" id="class-downloads">
			<h2>{title}</h2>
			<div class="class-downloads-container">
				<iframe
					src={boxUrl}
					width="500"
					height="400"
					frameborder="0"
					allowfullscreen
					title={title}
				></iframe>
			</div>
		</section>
	</div>
</section>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * Class Downloads Section - Apple ICT 7 Standards
	 * SSOT for all class download sections across the platform
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.class-section {
		margin-bottom: 40px;
	}

	.section-inner {
		width: 100%;
	}

	/* Downloads Section - Centered */
	#class-downloads {
		text-align: center;
	}

	#class-downloads h2 {
		font-size: 1.8rem;
		font-weight: 600;
		color: #333333;
		margin-bottom: 25px;
	}

	.class-downloads-container {
		width: 100%;
		max-width: 500px;
		margin: 0 auto;
	}

	.class-downloads-container iframe {
		width: 100%;
		border: 1px solid #dddddd;
		border-radius: 4px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * Responsive Design - Mobile First
	 * ═══════════════════════════════════════════════════════════════════════════ */

	@media (max-width: 768px) {
		#class-downloads h2 {
			font-size: 1.5rem;
		}

		.class-downloads-container {
			max-width: 100%;
		}

		.class-downloads-container iframe {
			height: 300px;
		}
	}

	@media (max-width: 480px) {
		#class-downloads h2 {
			font-size: 1.3rem;
		}

		.class-downloads-container iframe {
			height: 250px;
		}
	}
</style>

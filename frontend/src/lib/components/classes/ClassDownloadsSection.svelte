<!--
	Class Downloads Section Component - SSOT
	═══════════════════════════════════════════════════════════════════════════
	Standardized Box.com iframe downloads section for all class pages.
	This is the Single Source of Truth (SSOT) for class download sections.
	
	EXACT SPECIFICATIONS:
	- Desktop Container: 1080px × 512px
	- Desktop Iframe: 518px height
	- Padding: 25px all sides
	- Background: #FFFFFF (white)
	- Heading: 1.75rem (28px), weight 400, #4a4a4a
	
	@version 2.0.0 - Apple ICT 7 Principal Engineer Grade
	@author Revolution Trading Pros
-->
<script lang="ts">
	// Svelte 5 $props() syntax - Apple ICT 7 best practice
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
					allowfullscreen
					{title}
					loading="lazy"
					referrerpolicy="strict-origin-when-cross-origin"
					sandbox="allow-scripts allow-same-origin allow-forms allow-popups allow-popups-to-escape-sandbox"
				></iframe>
				<noscript>
					<p class="iframe-fallback">
						Please enable JavaScript to view class downloads, or 
						<a href={boxUrl.replace('/embed/', '/')} target="_blank" rel="noopener noreferrer">
							click here to open in Box.com
						</a>.
					</p>
				</noscript>
			</div>
		</section>
	</div>
</section>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * CLASS DOWNLOADS - Apple ICT 7 Principal Engineer - EXACT SPECIFICATIONS
	 * Desktop: 1080px × 512px container, 518px iframe
	 * ═══════════════════════════════════════════════════════════════════════════ */

	#dl-rp-row {
		padding: 40px 0;
		background-color: #EFEFEF;
	}

	#dl-rp-row .section-inner {
		display: flex;
		flex-direction: column;
		align-items: center;
		max-width: 1100px;
		margin: 0 auto;
		padding: 0 20px;
	}

	.class-subsection#class-downloads {
		background-color: #FFFFFF !important;
		padding: 25px;
		width: 100%;
		max-width: 1080px;
		height: 512px;
		box-sizing: border-box;
		margin: 0 auto;
		box-shadow: none;
	}

	.class-subsection#class-downloads h2 {
		font-size: 1.5rem !important;
		font-weight: 400 !important;
		color: #333333 !important;
		margin: 0 0 20px 0 !important;
		text-align: left;
		line-height: 1.2 !important;
		font-family: "Open Sans", -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif !important;
	}

	.class-downloads-container {
		width: 100%;
		height: 518px;
		box-sizing: border-box;
		background: #FFFFFF;
		overflow: hidden;
	}

	.class-downloads-container iframe {
		width: 100%;
		height: 518px;
		border: none;
		display: block;
	}

	.iframe-fallback {
		padding: 20px;
		text-align: center;
		background-color: #FFFFFF;
		border: 1px solid #dddddd;
		border-radius: 4px;
		color: #666666;
	}

	.iframe-fallback a {
		color: #1e73be;
		text-decoration: underline;
		display: inline-block;
		min-height: 44px;
		line-height: 44px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * RESPONSIVE BREAKPOINTS - Apple HIG Aligned
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* Mobile Small (< 428px) */
	@media (max-width: 427px) {
		#dl-rp-row .section-inner {
			padding: 0 16px;
		}

		.class-subsection#class-downloads {
			width: 100%;
			height: auto;
			min-height: 350px;
			padding: 16px;
		}

		.class-downloads-container {
			height: 300px;
		}

		.class-downloads-container iframe {
			height: 300px;
		}
	}

	/* Mobile (428px - 743px) */
	@media (min-width: 428px) and (max-width: 743px) {
		.class-subsection#class-downloads {
			width: 100%;
			height: auto;
			min-height: 400px;
		}

		.class-downloads-container {
			height: 350px;
		}

		.class-downloads-container iframe {
			height: 350px;
		}
	}

	/* Tablet (744px - 1023px) */
	@media (min-width: 744px) and (max-width: 1023px) {
		#dl-rp-row .section-inner {
			padding: 0 32px;
		}

		.class-subsection#class-downloads {
			width: 100%;
			max-width: 720px;
			height: auto;
			min-height: 450px;
		}

		.class-downloads-container {
			height: 400px;
		}

		.class-downloads-container iframe {
			height: 400px;
		}
	}

	/* Desktop (1024px - 1365px) */
	@media (min-width: 1024px) and (max-width: 1365px) {
		#dl-rp-row .section-inner {
			padding: 0 40px;
		}

		.class-subsection#class-downloads {
			width: 100%;
			max-width: 1080px;
			height: 512px;
		}

		.class-downloads-container {
			height: 518px;
		}

		.class-downloads-container iframe {
			height: 518px;
		}
	}

	/* Desktop Large (1366px+) */
	@media (min-width: 1366px) {
		#dl-rp-row .section-inner {
			padding: 0;
		}

		.class-subsection#class-downloads {
			width: 1080px;
			max-width: 1080px;
			height: 512px;
		}

		.class-downloads-container {
			height: 518px;
		}

		.class-downloads-container iframe {
			height: 518px;
		}
	}
</style>

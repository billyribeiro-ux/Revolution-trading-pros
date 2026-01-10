<!--
	Class Downloads Section Component - SSOT
	═══════════════════════════════════════════════════════════════════════════
	Standardized Box.com iframe downloads section for all class pages.
	
	@version 2.0.0 - Apple ICT 7 Principal Engineer
	@author Revolution Trading Pros
-->
<script lang="ts">
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

	#class-downloads {
		background-color: #FFFFFF;
		padding: 25px;
		width: 100%;
		max-width: 1080px;
		box-sizing: border-box;
		margin: 0 auto;
	}

	#class-downloads h2 {
		font-size: 1.5rem;
		font-weight: 400;
		color: #333333;
		margin: 0 0 20px 0;
		text-align: left;
		line-height: 1.2;
		font-family: "Open Sans", -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
	}

	.class-downloads-container {
		width: 100%;
		height: 400px;
		box-sizing: border-box;
		background: #FFFFFF;
		overflow: hidden;
	}

	.class-downloads-container iframe {
		width: 100%;
		height: 100%;
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
	}

	@media (max-width: 768px) {
		.class-downloads-container {
			height: 350px;
		}
	}

	@media (max-width: 480px) {
		#class-downloads {
			padding: 16px;
		}
		
		.class-downloads-container {
			height: 300px;
		}
	}
</style>

<!--
	Trader Header Component
	═══════════════════════════════════════════════════════════════════════════
	Reusable header component for all trader pages.
	
	@version 1.0.0
-->
<script lang="ts">
	import type { Trader } from '$lib/data/traders';

	// ICT 7 FIX: Svelte 5 $props() syntax (Nov 2025 best practice)
	interface Props {
		trader: Trader;
	}

	let { trader }: Props = $props();
</script>

<div class="fl-row fl-row-full-width fl-row-bg-color">
	<div class="fl-row-content-wrap">
		<div class="fl-row-content fl-row-fixed-width fl-node-content">
			<div class="fl-col-group fl-col-group-equal-height fl-col-group-align-bottom">
				<!-- Left Column - Photo -->
				<div class="fl-col fl-col-small">
					<div class="fl-col-content fl-node-content">
						<div class="fl-module fl-module-photo fl-visible-desktop-medium">
							<div class="fl-module-content fl-node-content">
								<div class="fl-photo fl-photo-align-center">
									<div class="fl-photo-content fl-photo-img-png">
										<img
											class="fl-photo-img"
											src={trader.image}
											alt={trader.name}
											width="2000"
											height="2000"
										/>
									</div>
								</div>
							</div>
						</div>
					</div>
				</div>
				<!-- Right Column - Info -->
				<div class="fl-col">
					<div class="fl-col-content fl-node-content">
						<div class="fl-module fl-module-heading">
							<div class="fl-module-content fl-node-content">
								<h2 class="fl-heading">
									<span class="fl-heading-text trader-name">{trader.name}</span>
								</h2>
							</div>
						</div>
						<div class="fl-module fl-module-heading">
							<div class="fl-module-content fl-node-content">
								<h2 class="fl-heading">
									<span class="fl-heading-text trader-title">{trader.title}</span>
								</h2>
							</div>
						</div>
						<div class="fl-module fl-module-separator">
							<div class="fl-module-content fl-node-content">
								<div class="fl-separator"></div>
							</div>
						</div>
						<div class="fl-module fl-module-rich-text">
							<div class="fl-module-content fl-node-content">
								<div class="fl-rich-text">
									<p><em>{trader.quote}</em></p>
								</div>
							</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	</div>
</div>

<style>
	.fl-row {
		margin: 0;
		padding: 0;
		width: 100%;
	}

	.fl-row-full-width {
		width: 100%;
	}

	.fl-row-bg-color {
		background-color: #e5e9f4;
	}

	.fl-row-content-wrap {
		max-width: 1200px;
		margin: 0 auto;
		padding: 40px 20px;
	}

	.fl-row-content {
		width: 100%;
	}

	.fl-row-fixed-width {
		max-width: 1100px;
		margin: 0 auto;
	}

	.fl-col-group {
		display: flex;
		flex-wrap: wrap;
		margin: 0 -15px;
	}

	.fl-col-group-equal-height {
		align-items: stretch;
	}

	.fl-col-group-align-bottom {
		align-items: flex-end;
	}

	.fl-col {
		flex: 1;
		padding: 0 15px;
		min-width: 0;
	}

	.fl-col-small {
		flex: 0 0 300px;
		max-width: 300px;
	}

	.fl-col-content {
		height: 100%;
	}

	.fl-node-content {
		position: relative;
	}

	.fl-module {
		margin-bottom: 20px;
	}

	.fl-module:last-child {
		margin-bottom: 0;
	}

	.fl-module-content {
		position: relative;
	}

	.fl-module-photo {
		margin-bottom: 0;
	}

	.fl-photo {
		display: block;
	}

	.fl-photo-align-center {
		text-align: center;
	}

	.fl-photo-content {
		display: inline-block;
		line-height: 0;
	}

	.fl-photo-img-png {
		background: transparent;
	}

	.fl-photo-img {
		max-width: 100%;
		height: auto;
		width: 250px;
		border-radius: 50%;
	}

	.fl-heading {
		margin: 0;
		padding: 0;
		line-height: 1.2;
	}

	.fl-heading-text {
		display: block;
	}

	.fl-heading-text.trader-name {
		font-size: 36px;
		font-weight: 700;
		color: #333;
	}

	.fl-heading-text.trader-title {
		font-size: 18px;
		font-weight: 400;
		color: #666;
		margin-top: 5px;
	}

	.fl-module-separator {
		margin: 20px 0;
	}

	.fl-separator {
		border-top: 2px solid #f69532;
		width: 60px;
	}

	.fl-rich-text {
		font-size: 16px;
		line-height: 1.7;
		color: #444;
	}

	.fl-rich-text p {
		margin: 0 0 15px;
	}

	.fl-rich-text p:last-child {
		margin-bottom: 0;
	}

	.fl-rich-text em {
		font-style: italic;
		color: #555;
	}

	@media (max-width: 991px) {
		.fl-col-small {
			flex: 0 0 200px;
			max-width: 200px;
		}

		.fl-photo-img {
			width: 180px;
		}

		.fl-heading-text.trader-name {
			font-size: 28px;
		}
	}

	@media (max-width: 767px) {
		.fl-col-group {
			flex-direction: column;
		}

		.fl-col-small {
			flex: 0 0 100%;
			max-width: 100%;
			text-align: center;
			margin-bottom: 20px;
		}

		.fl-col {
			flex: 0 0 100%;
			max-width: 100%;
		}

		.fl-photo-img {
			width: 150px;
		}

		.fl-heading-text.trader-name {
			font-size: 24px;
			text-align: center;
		}

		.fl-heading-text.trader-title {
			text-align: center;
		}

		.fl-separator {
			margin: 15px auto;
		}

		.fl-rich-text {
			text-align: center;
		}
	}
</style>

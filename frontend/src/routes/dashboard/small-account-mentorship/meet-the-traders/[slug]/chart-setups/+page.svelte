<!--
	Chart Setups Sub-Page
	═══════════════════════════════════════════════════════════════════════════
	Trader-specific chart setups content. Uses shared layout.
	Matches John Carter reference implementation exactly.
	
	@version 2.0.0
-->
<script lang="ts">
	import { page } from '$app/state';
	import { getTraderBySlug } from '$lib/data/traders';

	let trader = $derived(getTraderBySlug(page.params.slug ?? ''));
</script>

<!-- Page Content -->
{#if trader}
	<div class="content-block">
		<h2 class="page-heading">{trader.name.split(' ')[0]}'s Indicators</h2>
	</div>

	<div class="content-block">
		<div class="content-text">
			<p class="no-margin">
				You may have heard, {trader.name.split(' ')[0]} uses a combination of indicators and market internals
				across his 6 monitors that he looks at daily. As you know each trader should decide what is most
				important to them to have on their own charts, and what works best for them.
			</p>

			<p class="no-margin">His core setup includes the following <b>free indicators</b>:</p>
			<ul>
				<li>
					<p class="no-margin">
						<a
							href="https://intercom.help/simpler-trading/en/articles/3264121"
							target="_blank"
							rel="noopener">The Exponential Moving Averages</a
						> (8, 21, 34, 55, and 200)
					</p>
				</li>
				<li>
					<p class="no-margin">
						<a
							href="https://intercom.help/simpler-trading/en/articles/3293713"
							target="_blank"
							rel="noopener">VolumeAvg</a
						>
					</p>
				</li>
				<li>
					<p class="no-margin">
						<a
							href="https://intercom.help/simpler-trading/en/articles/3347719"
							target="_blank"
							rel="noopener">Custom Time Frames</a
						> (39, 78, 195, and Daily charts)
					</p>
				</li>
			</ul>

			<div class="intercom-container">
				<img
					src="https://downloads.intercomcdn.com/i/o/258328594/bff0055e0282c1edd00ced72/image.png"
					alt="Chart Setup Example"
					loading="lazy"
				/>
			</div>

			<p class="no-margin">
				Additionally, {trader.name.split(' ')[0]} uses the following <b>premium indicators</b> that aren't
				included in the below link.
			</p>
			<ul>
				<li>
					<p class="no-margin">
						<a
							href="https://intercom.help/simpler-trading/en/articles/3189130"
							target="_blank"
							rel="noopener">Three ATR / Keltner Channels Setup</a
						> (free in most platforms)
					</p>
				</li>
				<li>
					<p class="no-margin">
						<a
							href="https://intercom.help/simpler-trading/en/articles/3186315"
							target="_blank"
							rel="noopener">The Squeeze Pro</a
						>
						(You can use the free version of
						<a
							href="https://intercom.help/simpler-trading/en/articles/3263889"
							target="_blank"
							rel="noopener">The Squeeze</a
						> as well)
					</p>
				</li>
				<li>
					<p class="no-margin">
						<a
							href="https://intercom.help/simpler-trading/en/articles/3230954"
							target="_blank"
							rel="noopener">Early In-N-Out Pro</a
						>
					</p>
				</li>
				<li>
					<p class="no-margin">
						<a
							href="https://intercom.help/simpler-trading/en/articles/3999973"
							target="_blank"
							rel="noopener">Multi-Squeeze Pro</a
						>
					</p>
				</li>
				<li>
					<p class="no-margin">
						<a
							href="https://intercom.help/simpler-trading/en/articles/3287902"
							target="_blank"
							rel="noopener">The Voodoo lines</a
						>
					</p>
				</li>
				<li>
					<p class="no-margin">
						<a
							href="https://intercom.help/simpler-trading/en/articles/3188127"
							target="_blank"
							rel="noopener">Reversal Arrows</a
						>
					</p>
				</li>
				<li>
					<p class="no-margin">
						<a
							href="https://intercom.help/simpler-trading/en/articles/4426649"
							target="_blank"
							rel="noopener">Ready Aim Fire Pro</a
						>
					</p>
				</li>
				<li>
					<a
						href="https://intercom.help/simpler-trading/en/articles/7208076-about-micro-voodoo-lines"
						target="_blank"
						rel="noopener">Micro Voodoo Lines</a
					>
				</li>
				<li>
					<a
						href="https://intercom.help/simpler-trading/en/articles/6003945-about-quant-pivots"
						target="_blank"
						rel="noopener">Quant Pivots</a
					>
				</li>
			</ul>

			<div class="intercom-container">
				<img
					src="https://cdn.simplertrading.com/2023/06/20151402/image-57.png"
					alt="Advanced Chart Setup"
					width="2546"
					height="1433"
					srcset="https://cdn.simplertrading.com/2023/06/20151402/image-57.png 2546w, https://cdn.simplertrading.com/2023/06/20151402/image-57-300x169.png 300w, https://cdn.simplertrading.com/2023/06/20151402/image-57-1024x576.png 1024w, https://cdn.simplertrading.com/2023/06/20151402/image-57-150x84.png 150w, https://cdn.simplertrading.com/2023/06/20151402/image-57-768x432.png 768w, https://cdn.simplertrading.com/2023/06/20151402/image-57-1536x865.png 1536w, https://cdn.simplertrading.com/2023/06/20151402/image-57-2048x1153.png 2048w, https://cdn.simplertrading.com/2023/06/20151402/image-57-400x225.png 400w"
					sizes="(max-width: 2546px) 100vw, 2546px"
					loading="lazy"
				/>
			</div>

			<p class="no-margin"><b>Basic chart layout:</b></p>
			<p class="no-margin">
				Below is a link to the shared chart which will get you set up with the basic chart setup,
				look and feel which you will see on {trader.name.split(' ')[0]}'s charts.
			</p>
			<p class="no-margin">
				Below is a shared link TOS, copy and paste this shared link into Thinkorswim (TOS):
			</p>
			<p class="no-margin">
				<a href="https://tos.mx/F5XE5sa" target="_blank" rel="nofollow noopener"
					>https://tos.mx/F5XE5sa</a
				>
			</p>
			<p class="no-margin">
				Please go to: Setup &gt; Open Shared item &gt; paste link into Shared item URL: &gt; Open
			</p>
		</div>
	</div>
{/if}

<style>
	.content-block {
		margin-bottom: 20px;
	}

	.content-block:last-child {
		margin-bottom: 0;
	}

	.page-heading {
		font-size: 52px;
		font-weight: 700;
		color: #0c2434;
		font-family: 'Open Sans Condensed', sans-serif;
		margin: 0;
		line-height: 1.2;
	}

	.content-text {
		font-size: 23px;
		line-height: 1.5;
		color: #333;
	}

	.content-text p {
		margin: 0 0 20px;
	}

	.content-text p.no-margin {
		margin-bottom: 10px;
	}

	.content-text ul {
		margin: 15px 0 20px;
		padding-left: 40px;
	}

	.content-text li {
		margin-bottom: 8px;
	}

	.content-text li p {
		margin: 0;
	}

	.content-text a {
		color: #0984ae;
		text-decoration: none;
		transition: color 0.2s;
	}

	.content-text a:hover {
		color: #076787;
		text-decoration: underline;
	}

	.content-text b {
		font-weight: 700;
	}

	.intercom-container {
		margin: 20px 0;
		text-align: center;
	}

	.intercom-container img {
		max-width: 100%;
		height: auto;
		display: block;
		margin: 0 auto;
		border-radius: 4px;
	}

	@media (max-width: 768px) {
		.page-heading {
			font-size: 32px;
			text-align: center;
		}

		.content-text {
			font-size: 18px;
		}
	}
</style>

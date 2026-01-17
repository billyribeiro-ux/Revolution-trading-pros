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
	h1 {
		color: #0c2434;
		font-weight: 700;
		text-align: center;
		margin-bottom: 10px;
		font-size: 30px;
		font-family: 'Open Sans Condensed', sans-serif;
		line-height: 1.1em;
	}

	.platforms {
		text-align: center;
		margin-bottom: 40px;
		color: #666666;
		font-family: 'Open Sans', sans-serif;
	}

	.platforms strong {
		color: #666666;
		font-weight: 700;
	}

	hr {
		border: none;
		border-top: 1px solid #dddddd;
		margin: 20px 0 40px;
	}

	@media (min-width: 768px) {
		h1 {
			font-size: 54px;
		}
	}
</style>

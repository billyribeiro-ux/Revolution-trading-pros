<!--
	Download Button Component
	═══════════════════════════════════════════════════════════════════════════
	Reusable orange button for downloads and links
	Used in indicator pages for download and documentation links
-->
<script lang="ts">
	interface Props {
		href: string;
		text?: string;
		target?: string;
		rel?: string;
	}

	let { 
		href, 
		text = 'Click to Download',
		target,
		rel
	}: Props = $props();
</script>

<a 
	class="orng_btn" 
	{href}
	{target}
	{rel}
>
	{text}
</a>

<style>
	.orng_btn {
		background-color: #f8ac00;
		border-radius: 80px;
		font-weight: 700;
		color: #fff;
		display: inline-block;
		text-align: center;
		padding: 8px 20px;
		min-width: 150px;
		font-size: 16px;
		text-decoration: none;
		transition: background-color 0.2s ease;
	}

	.orng_btn:hover {
		background-color: #df9c00;
		color: #fff;
		text-decoration: none;
	}

	@media (min-width: 416px) {
		.orng_btn {
			min-width: 200px;
			font-size: 18px;
			padding: 10px 25px;
		}
	}

	@media (min-width: 768px) {
		.orng_btn {
			min-width: 250px;
			font-size: 20px;
			padding: 10px 35px;
		}
	}
</style>

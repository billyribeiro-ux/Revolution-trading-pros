<script lang="ts">
	/**
	 * EmptyState - Shared empty state component
	 * Svelte 5 with $props()
	 */
	interface Props {
		title: string;
		buttonText?: string;
		buttonHref?: string;
	}

	let { title, buttonText, buttonHref }: Props = $props();
</script>

<div class="empty-state">
	<h4 class="empty-state__title">{title}</h4>
	{#if buttonText && buttonHref}
		<a class="empty-state__button" href={buttonHref}>{buttonText}</a>
	{/if}
</div>

<style>
	.empty-state {
		text-align: left;
	}

	.empty-state__title {
		font-size: 16px;
		font-weight: 600;
		color: #333;
		margin: 0 0 20px;
		font-family: 'Open Sans', sans-serif;
	}

	.empty-state__button {
		display: inline-block;
		padding: 12px 24px;
		font-size: 14px;
		font-weight: 800;
		font-family: 'Open Sans', sans-serif;
		text-transform: uppercase;
		letter-spacing: 1.125px;
		background: #F69532;
		color: #fff;
		border: none;
		border-radius: 4px;
		text-decoration: none;
		box-shadow: 0 2px 5px rgba(0, 0, 0, 0.16);
		transition: all 0.2s ease-in-out;
	}

	.empty-state__button:hover {
		background-color: #dc7309;
	}
</style>

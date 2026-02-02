<script lang="ts">
	interface Props {
		size?: number;
		class?: string;
	}

	let { size = 24, class: className = '' }: Props = $props();
</script>

<svg
	xmlns="http://www.w3.org/2000/svg"
	width={size}
	height={size}
	viewBox="0 0 24 24"
	fill="none"
	stroke="currentColor"
	stroke-width="2"
	stroke-linecap="round"
	stroke-linejoin="round"
	class={className}
>
	<polyline points="15 3 21 3 21 9" />
	<polyline points="9 21 3 21 3 15" />
	<line x1="21" x2="14" y1="3" y2="10" />
	<line x1="3" x2="10" y1="21" y2="14" />
</svg>

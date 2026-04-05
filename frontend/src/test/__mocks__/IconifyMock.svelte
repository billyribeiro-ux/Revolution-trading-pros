<script lang="ts">
	interface Props {
		icon?: string;
		width?: number;
		height?: number;
		color?: string;
		class?: string;
		style?: string;
		'aria-hidden'?: boolean | 'true' | 'false';
		'aria-label'?: string;
		[key: string]: unknown;
	}

	let { icon = '', width = 24, height = 24, class: className = '', ...rest }: Props = $props();
</script>

<svg
	data-icon={icon}
	{width}
	{height}
	class={className}
	{...rest}
	xmlns="http://www.w3.org/2000/svg"
	viewBox="0 0 24 24"
>
	<rect {width} {height} fill="currentColor" />
</svg>

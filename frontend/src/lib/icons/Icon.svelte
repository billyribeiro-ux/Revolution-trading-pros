<script lang="ts">
	import IconifyIcon from '@iconify/svelte';

	interface Props {
		icon: string;
		size?: number;
		color?: string;
		stroke?: number;
		class?: string;
		'aria-hidden'?: boolean | 'true' | 'false';
		'aria-label'?: string;
		style?: string;
	}

	let {
		icon,
		size = 24,
		color = 'currentColor',
		stroke: _stroke,
		class: className = '',
		style,
		...rest
	}: Props = $props();
</script>

<!-- @ts-expect-error @iconify/svelte ships Svelte 4 class-component types; our Props.icon is string -->
<IconifyIcon
	icon={icon as any}
	width={size}
	height={size}
	{color}
	class={className}
	{style}
	{...rest}
/>

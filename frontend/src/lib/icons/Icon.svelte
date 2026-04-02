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
		stroke,
		class: className = '',
		style,
		...rest
	}: Props = $props();
</script>

<IconifyIcon {icon} width={size} height={size} {color} class={className} {style} {...rest} />

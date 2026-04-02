<script lang="ts">
	import IconifyIcon from '@iconify/svelte';

	interface Props {
		icon: string;
		size?: number;
		color?: string;
		stroke?: number;
		class?: string;
	}

	let { icon, size = 24, color = 'currentColor', stroke, class: className = '' }: Props = $props();
</script>

<IconifyIcon {icon} width={size} height={size} {color} class={className} />

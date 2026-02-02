// Type augmentations for third-party libraries

/**
 * Type declarations for @tabler/icons-svelte-runes subpath imports
 * Required for TypeScript to understand individual icon imports
 */
declare module '@tabler/icons-svelte-runes/icons/*' {
	import type { Component } from 'svelte';

	interface IconProps {
		size?: number | string;
		stroke?: number | string;
		color?: string;
		class?: string;
		style?: string;
		[key: string]: unknown;
	}

	const Icon: Component<IconProps>;
	export default Icon;
}

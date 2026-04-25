<script lang="ts">
	/**
	 * Icon - Centralized Tabler icon wrapper.
	 *
	 * Selects an icon from the `$lib/icons` barrel by name and renders it with
	 * a consistent size token system. Use this in place of inline `<svg>` markup
	 * so analytics dashboards (and the rest of the app) share one source of
	 * truth for iconography, sizing, and accessibility semantics.
	 *
	 * @example
	 *   <Icon name="IconUsers" size="lg" label="Visitors" />
	 *   <Icon name="IconSearch" size={18} />
	 */
	import * as Icons from '$lib/icons';

	type IconName = keyof typeof Icons;
	type SizeToken = 'xs' | 'sm' | 'md' | 'lg' | 'xl';

	interface Props {
		name: IconName;
		/** Numeric pixel size, or one of the named tokens. */
		size?: number | SizeToken;
		/** Provide an accessible label. If omitted, the icon is decorative (aria-hidden). */
		label?: string;
	}

	const SIZES = { xs: 14, sm: 16, md: 20, lg: 24, xl: 32 } as const;

	let { name, size = 'md', label }: Props = $props();

	const px = $derived(typeof size === 'number' ? size : SIZES[size]);
	const Component = $derived(Icons[name]);
</script>

{#if Component}
	{#if label}
		<Component size={px} aria-label={label} role="img" />
	{:else}
		<Component size={px} aria-hidden="true" />
	{/if}
{/if}

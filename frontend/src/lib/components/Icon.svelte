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
	 *   <Icon name="IconCheck" class="w-4 h-4 text-emerald-400" />
	 */
	import * as Icons from '$lib/icons';

	type IconName = keyof typeof Icons;
	type SizeToken = 'xs' | 'sm' | 'md' | 'lg' | 'xl';

	interface Props {
		name: IconName;
		/** Numeric pixel size, or one of the named tokens. Ignored if `class` sets sizing. */
		size?: number | SizeToken;
		/** Stroke width (default: 2). */
		stroke?: number | string;
		/** Color override; defaults to currentColor (inherits from parent). */
		color?: string;
		/** Tailwind / utility classes. Lets the icon size & color via classes the same way the legacy inline SVGs did. */
		class?: string;
		/** Provide an accessible label. If omitted, the icon is decorative (aria-hidden). */
		label?: string;
	}

	const SIZES = { xs: 14, sm: 16, md: 20, lg: 24, xl: 32 } as const;

	let { name, size = 'md', stroke = 2, color, class: className = '', label }: Props = $props();

	const px = $derived(typeof size === 'number' ? size : SIZES[size]);
	const Component = $derived(Icons[name]);
</script>

{#if Component}
	{#if label}
		<Component size={px} {stroke} {color} class={className} aria-label={label} role="img" />
	{:else}
		<Component size={px} {stroke} {color} class={className} aria-hidden="true" />
	{/if}
{/if}

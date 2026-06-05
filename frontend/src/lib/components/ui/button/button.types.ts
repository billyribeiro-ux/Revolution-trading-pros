/**
 * Button Component Types
 */
import { type WithElementRef } from '$lib/utils.js';
import type { HTMLAnchorAttributes, HTMLButtonAttributes } from 'svelte/elements';

export type ButtonVariant = 'default' | 'destructive' | 'outline' | 'secondary' | 'ghost' | 'link';
export type ButtonSize = 'default' | 'sm' | 'lg' | 'icon' | 'icon-sm' | 'icon-lg';

interface ButtonVariantOptions {
	variant?: ButtonVariant | null;
	size?: ButtonSize | null;
}

export function buttonVariants({
	variant = 'default',
	size = 'default'
}: ButtonVariantOptions = {}): string {
	return [
		'ui-button',
		`ui-button--${variant ?? 'default'}`,
		`ui-button--${size ?? 'default'}`
	].join(' ');
}

export type ButtonProps = WithElementRef<HTMLButtonAttributes> &
	WithElementRef<HTMLAnchorAttributes> & {
		variant?: ButtonVariant;
		size?: ButtonSize;
	};

/**
 * Button Component Types
 */
import { cn, type WithElementRef } from '$lib/utils.js';
import type { HTMLAnchorAttributes, HTMLButtonAttributes } from 'svelte/elements';

const base =
	"focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive inline-flex shrink-0 items-center justify-center gap-2 rounded-md text-sm font-medium whitespace-nowrap transition-all outline-none focus-visible:ring-[3px] disabled:pointer-events-none disabled:opacity-50 aria-disabled:pointer-events-none aria-disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4";

const variantClasses: Record<string, string> = {
	default: 'bg-primary text-primary-foreground hover:bg-primary/90 shadow-xs',
	destructive:
		'bg-destructive hover:bg-destructive/90 focus-visible:ring-destructive/20 dark:focus-visible:ring-destructive/40 dark:bg-destructive/60 text-white shadow-xs',
	outline:
		'bg-background hover:bg-accent hover:text-accent-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 border shadow-xs',
	secondary: 'bg-secondary text-secondary-foreground hover:bg-secondary/80 shadow-xs',
	ghost: 'hover:bg-accent hover:text-accent-foreground dark:hover:bg-accent/50',
	link: 'text-primary underline-offset-4 hover:underline'
};

const sizeClasses: Record<string, string> = {
	default: 'h-9 px-4 py-2 has-[>svg]:px-3',
	sm: 'h-8 gap-1.5 rounded-md px-3 has-[>svg]:px-2.5',
	lg: 'h-10 rounded-md px-6 has-[>svg]:px-4',
	icon: 'size-11',
	'icon-sm': 'size-9',
	'icon-lg': 'size-12'
};

export function buttonVariants(opts?: {
	variant?: ButtonVariant;
	size?: ButtonSize;
	class?: string;
}): string {
	const v = opts?.variant ?? 'default';
	const s = opts?.size ?? 'default';
	return cn(base, variantClasses[v ?? 'default'], sizeClasses[s ?? 'default'], opts?.class);
}

export type ButtonVariant = 'default' | 'destructive' | 'outline' | 'secondary' | 'ghost' | 'link';
export type ButtonSize = 'default' | 'sm' | 'lg' | 'icon' | 'icon-sm' | 'icon-lg';

export type ButtonProps = WithElementRef<HTMLButtonAttributes> &
	WithElementRef<HTMLAnchorAttributes> & {
		variant?: ButtonVariant;
		size?: ButtonSize;
	};

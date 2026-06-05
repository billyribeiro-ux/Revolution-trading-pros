/**
 * Badge Component Types
 */

export type BadgeVariant = 'default' | 'secondary' | 'destructive' | 'outline';

interface BadgeVariantOptions {
	variant?: BadgeVariant | null;
}

export function badgeVariants({ variant = 'default' }: BadgeVariantOptions = {}): string {
	return ['ui-badge', `ui-badge--${variant ?? 'default'}`].join(' ');
}

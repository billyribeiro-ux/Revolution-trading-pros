/**
 * Navigation Types
 * Shared type definitions for nav components
 */

export interface SubMenuItem {
	href: string;
	label: string;
	description?: string;
}

export interface NavItem {
	id: string;
	label: string;
	href?: string;
	submenu?: SubMenuItem[];
}

export interface NavConfig {
	items: NavItem[];
	logo: {
		src: string;
		alt: string;
		width: number;
		height: number;
	};
	cta?: {
		href: string;
		label: string;
	};
}

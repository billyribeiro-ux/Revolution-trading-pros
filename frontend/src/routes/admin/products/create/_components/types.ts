/**
 * R24-C extraction (2026-05-20): hoisted types for the product create page so
 * children can share the same shapes without re-importing from the parent.
 */

import type { Component } from 'svelte';

export type ProductType = 'course' | 'indicator' | 'membership' | 'bundle';

export interface ProductFormData {
	name: string;
	slug: string;
	type: ProductType;
	description: string;
	long_description: string;
	price: string;
	sale_price: string;
	currency: string;
	features: string[];
	is_active: boolean;
	thumbnail: string;
	meta_title: string;
	meta_description: string;
	indexable: boolean;
	canonical_url: string;
}

export interface ProductTypeOption {
	value: ProductType;
	label: string;
	icon: Component<{ size?: number }>;
	color: string;
}

export interface ProductPricePreview {
	original: string;
	sale: string | null;
}

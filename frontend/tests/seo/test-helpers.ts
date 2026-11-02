import * as fs from 'node:fs';
import * as path from 'node:path';

export interface InventoryEntry {
	app: string;
	route: string;
	pathPattern: string;
	url: string | null;
	indexable: boolean;
	template: string;
}

const INVENTORY_PATH = path.resolve(
	import.meta.dirname,
	'../../../docs/seo/audit/02-inventory.json'
);

export function loadInventory(): InventoryEntry[] {
	return JSON.parse(fs.readFileSync(INVENTORY_PATH, 'utf-8')) as InventoryEntry[];
}

export const REPRESENTATIVE_URLS: Array<{ path: string; template: string }> = [
	{ path: '/', template: 'home' },
	{ path: '/about', template: 'static' },
	{ path: '/pricing', template: 'product' },
	{ path: '/alerts/explosive-swings', template: 'product' },
	{ path: '/live-trading-rooms/day-trading', template: 'product' },
	{ path: '/courses/day-trading-masterclass', template: 'course' },
	{ path: '/indicators/rsi', template: 'software' },
	{ path: '/authors/billy-ribeiro', template: 'profile' },
	{ path: '/guides/swing-entry', template: 'article' },
	{ path: '/resources/etf-stocks-list', template: 'article' },
	{ path: '/blog', template: 'listing' },
	{ path: '/classes', template: 'listing' },
	{ path: '/cookie-policy', template: 'static' }
];

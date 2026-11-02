/**
 * Classes Page Configuration
 *
 * SSG Configuration:
 * - prerender: true (static page for SEO)
 * - ssr: true (server-side rendering)
 */

import type { SEOInput } from '#lib/seo/types.js';
import { buildBreadcrumb, buildItemList } from '#lib/seo/schemas.js';
import { PRODUCTION_ORIGIN } from '#lib/seo/config.js';

const SITE = PRODUCTION_ORIGIN;

export const prerender = true;
export const ssr = true;

export interface PublicClassItem {
	id: string;
	title: string;
	slug: string;
	card_image_url?: string;
	card_description?: string;
	card_badge?: string;
	card_badge_color?: string;
	instructor_name?: string;
	level?: string;
	price_cents: number;
	is_free?: boolean;
	lesson_count?: number;
	total_duration_minutes?: number;
}

const PUBLIC_CLASSES: PublicClassItem[] = [
	{
		id: 'quickstart-precision-trading-c',
		title: 'Quickstart to Precision Trading',
		slug: 'quickstart-precision-trading-c',
		card_image_url: '/revolution-trading-pros.png',
		card_description:
			'Master institutional market structure, multi-timeframe alignment, and high-precision order execution in this foundational trading class.',
		card_badge: 'Foundational',
		card_badge_color: '#047857',
		instructor_name: 'Billy Ribeiro',
		level: 'Beginner',
		price_cents: 29700,
		lesson_count: 12,
		total_duration_minutes: 360
	},
	{
		id: 'tax-loss-harvest-c',
		title: 'Tax-Loss Harvesting Mastery for Traders',
		slug: 'tax-loss-harvest-c',
		card_image_url: '/revolution-trading-pros.png',
		card_description:
			'Learn year-end tax-loss harvesting strategies, wash-sale rules, and capital-efficiency techniques for active equity and options traders.',
		card_badge: 'Strategy',
		card_badge_color: '#1d4ed8',
		instructor_name: 'Billy Ribeiro',
		level: 'Intermediate',
		price_cents: 19700,
		lesson_count: 8,
		total_duration_minutes: 240
	}
];

export const load = () => ({
	courses: PUBLIC_CLASSES,
	seo: {
		title: 'Trading Classes & Courses',
		description:
			'Browse our trading courses and classes. Learn from professional traders and improve your trading skills with beginner to advanced programs.',
		jsonld: [
			buildBreadcrumb([
				{ name: 'Home', url: `${SITE}/` },
				{ name: 'Trading Classes', url: `${SITE}/classes` }
			]),
			buildItemList(
				'Revolution Trading Pros Classes',
				PUBLIC_CLASSES.map((c) => ({
					name: c.title,
					url: `${SITE}/classes/${c.slug}`,
					description: c.card_description
				}))
			)
		]
	} satisfies SEOInput
});

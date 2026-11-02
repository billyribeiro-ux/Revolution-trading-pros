import type { PageLoad, EntryGenerator } from './$types';
import { error } from '@sveltejs/kit';
import type { SEOInput } from '#lib/seo/types.js';
import { buildBreadcrumb, buildSoftwareApplication } from '#lib/seo/schemas.js';
import { indicators } from '../data';
import { PRODUCTION_ORIGIN } from '#lib/seo/config.js';

const SITE = PRODUCTION_ORIGIN;

export const prerender = true;

export const entries: EntryGenerator = () =>
	indicators.filter((i) => i.slug !== 'rsi').map((i) => ({ id: i.slug }));

export const load: PageLoad = ({ params }) => {
	const id = params.id ?? '';
	const indicator = indicators.find((i) => i.slug === id);

	if (!indicator) {
		error(404, 'Indicator not found');
	}

	const name = indicator.name;
	const description = indicator.description;
	const url = `${SITE}/indicators/${id}`;

	const seo: SEOInput = {
		title: name,
		description,
		jsonld: [
			buildBreadcrumb([
				{ name: 'Home', url: `${SITE}/` },
				{ name: 'Trading Indicators', url: `${SITE}/indicators` },
				{ name, url }
			]),
			buildSoftwareApplication({
				name,
				description,
				url,
				applicationCategory: 'FinanceApplication',
				operatingSystem: 'Web, ThinkorSwim, TradingView',
				image: `${SITE}/revolution-trading-pros.png`,
				offers: {
					price: 0,
					priceCurrency: 'USD',
					url
				}
			})
		]
	};

	return { indicator, seo };
};

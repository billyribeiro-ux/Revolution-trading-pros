import type { PageLoad } from './$types';
import type { SEOInput } from '$lib/seo/types';
import { buildBreadcrumb, buildSoftwareApplication } from '$lib/seo/schemas';
import { indicators } from '../data';

const SITE = 'https://revolutiontradingpros.com';

export const prerender = false;

export const load: PageLoad = ({ params }) => {
	const id = params.id ?? '';
	const indicator = indicators.find((i) => i.slug === id);

	const name = indicator?.name ?? id.replace(/-/g, ' ');
	const description =
		indicator?.description ??
		`Learn about the ${name} indicator and how Revolution Trading Pros traders use it.`;
	const url = `${SITE}/indicators/${id}`;

	const seo: SEOInput = {
		title: `${name} — Indicator Details`,
		description,
		jsonld: [
			buildBreadcrumb([
				{ name: 'Home', url: `${SITE}/` },
				{ name: 'Indicators', url: `${SITE}/indicators` },
				{ name, url }
			]),
			buildSoftwareApplication({
				name,
				description,
				url,
				applicationCategory: 'FinanceApplication'
			})
		]
	};

	return { seo };
};

import type { PageServerLoad } from './$types';

/**
 * @type {import('./$types').PageServerLoad}
 */
export const load = (async () => {
	// Mock archive data - will be replaced with API call
	const archiveEntries = [
		{
			slug: '12082025-taylor-horton',
			title: 'Weekly Watchlist for December 8, 2025',
			trader: 'Taylor Horton',
			date: 'December 8, 2025'
		},
		{
			slug: '12012025-raghee-horner',
			title: 'Weekly Watchlist for December 1, 2025',
			trader: 'Raghee Horner',
			date: 'December 1, 2025'
		},
		{
			slug: '11242025-david-starr',
			title: 'Weekly Watchlist for November 24, 2025',
			trader: 'David Starr',
			date: 'November 24, 2025'
		},
		{
			slug: '11172025-mike-teeto',
			title: 'Weekly Watchlist for November 17, 2025',
			trader: 'Mike Teeto',
			date: 'November 17, 2025'
		},
		{
			slug: '11102025-taylor-horton',
			title: 'Weekly Watchlist for November 10, 2025',
			trader: 'Taylor Horton',
			date: 'November 10, 2025'
		},
		{
			slug: '11032025-allison-ostrander',
			title: 'Weekly Watchlist for November 3, 2025',
			trader: 'Allison Ostrander',
			date: 'November 3, 2025'
		},
		{
			slug: '10272025-bruce-marshall',
			title: 'Weekly Watchlist for October 27, 2025',
			trader: 'Bruce Marshall',
			date: 'October 27, 2025'
		},
		{
			slug: '10202025-melissa-beegle',
			title: 'Weekly Watchlist for October 20, 2025',
			trader: 'Melissa Beegle',
			date: 'October 20, 2025'
		},
		{
			slug: '10132025-tg-watkins',
			title: 'Weekly Watchlist for October 13, 2025',
			trader: 'TG Watkins',
			date: 'October 13, 2025'
		}
	];

	return {
		archiveEntries,
		totalPages: 22,
		currentPage: 1
	};
}) satisfies PageServerLoad;

/**
 * @type {import('./$types').PageServerLoad}
 */
export const load = (async () => {
	// Mock archive data matching WordPress reference - will be replaced with API call
	const archiveEntries = [
		{
			slug: '01122026-tg-watkins',
			title: 'Weekly Watchlist for January 12, 2026',
			trader: 'TG Watkins',
			date: 'January 12, 2026'
		},
		{
			slug: '01032026-melissa-beegle',
			title: 'Weekly Watchlist for January 3, 2026',
			trader: 'Melissa Beegle',
			date: 'January 3, 2026'
		},
		{
			slug: '12292025-david-starr',
			title: 'Weekly Watchlist for December 29, 2025',
			trader: 'David Starr',
			date: 'December 29, 2025'
		},
		{
			slug: '12222025-tg-watkins',
			title: 'Weekly Watchlist for December 22, 2025',
			trader: 'TG Watkins',
			date: 'December 22, 2025'
		},
		{
			slug: '12152025-allison-ostrander',
			title: 'Weekly Watchlist for December 15, 2025',
			trader: 'Allison Ostrander',
			date: 'December 15, 2025'
		},
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
		}
	];

	// Latest watchlist for featured section
	const latestWatchlist = {
		slug: '01122026-tg-watkins',
		trader: 'TG Watkins',
		weekText: 'Week of January 12, 2026.',
		image: 'https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/TG-Watchlist-Rundown.jpg'
	};

	return {
		archiveEntries,
		latestWatchlist,
		totalPages: 22,
		currentPage: 1
	};
});

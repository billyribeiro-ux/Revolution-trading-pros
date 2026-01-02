export const load = async ({ params }: { params: { slug: string } }) => {
	const { slug } = params;
	
	// TODO: Replace with actual API call to fetch class data
	// const response = await fetch(`/api/classes/${slug}`);
	// const classData = await response.json();
	
	// Helper function to format slug to title
	const formatTitle = (str: string) => 
		str.split('-').map((word: string) => word.charAt(0).toUpperCase() + word.slice(1)).join(' ');
	
	// Mock data for now
	const classData = {
		id: 2173470,
		title: formatTitle(slug),
		slug: slug,
		description: `Learn ${slug.split('-').join(' ')} strategies`,
		videoUrl: '',
		sections: [
			{
				title: `${formatTitle(slug)} 2025`,
				lessons: [
					{ title: 'Introduction', duration: '10:30', videoId: 'abc123' },
					{ title: 'Strategy Overview', duration: '15:45', videoId: 'def456' },
					{ title: 'Implementation', duration: '20:15', videoId: 'ghi789' }
				]
			}
		],
		metadata: {
			pageType: 'article',
			contentTitle: formatTitle(slug)
		}
	};
	
	return {
		classData
	};
};

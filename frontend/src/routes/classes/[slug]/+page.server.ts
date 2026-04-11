/**
 * Class Detail Page - SSR Load Function
 * ═══════════════════════════════════════════════════════════════════════════
 * Apple ICT 11+ Principal Engineer Grade - January 2026
 *
 * Features:
 * - Server-side rendering for SEO
 * - Pre-fetches course data from API
 * - Generates meta tags for social sharing
 * - Hydration-ready data structure
 *
 * @version 1.0.0
 */

import { env } from '$env/dynamic/private';
import type { PageServerLoad } from './$types';
import type { SEOInput } from '$lib/seo/types';
import { breadcrumbSchema } from '$lib/seo/jsonld';
import { logger } from '$lib/utils/logger';

const API_URL = env.API_URL || 'https://revolution-trading-pros-api.fly.dev';
const SITE_URL =
	import.meta.env.VITE_SITE_URL || 'https://revolution-trading-pros.pages.dev';

interface Lesson {
	id: string;
	title: string;
	slug?: string;
	description?: string;
	duration_minutes?: number;
	bunny_video_guid?: string;
	thumbnail_url?: string;
	is_preview?: boolean;
	module_id?: number;
}

interface Module {
	id: number;
	title: string;
	description?: string;
	sort_order: number;
}

interface ModuleWithLessons {
	module: Module;
	lessons: Lesson[];
}

interface CourseData {
	course: {
		id: string;
		title: string;
		slug: string;
		description?: string;
		card_image_url?: string;
		meta_title?: string;
		meta_description?: string;
		og_image_url?: string;
		bunny_library_id?: number;
		instructor_name?: string;
	};
	modules: ModuleWithLessons[];
	lessons: Lesson[];
	downloads?: unknown[];
	enrollment?: unknown;
}

export const load: PageServerLoad = async ({ params, cookies, fetch }) => {
	const { slug } = params;

	// Get auth token for API request
	const accessToken = cookies.get('access_token');

	const headers: Record<string, string> = {
		'Content-Type': 'application/json'
	};

	if (accessToken) {
		headers['Authorization'] = `Bearer ${accessToken}`;
	}

	let courseData: CourseData | null = null;
	let error: string | null = null;

	try {
		// Fetch course player data from API
		const response = await fetch(`${API_URL}/api/my/courses/${slug}/player`, {
			method: 'GET',
			headers
		});

		if (response.ok) {
			const data = await response.json();
			if (data.success && data.data) {
				courseData = data.data;
			}
		}
	} catch (err) {
		logger.error('[SSR] Failed to fetch course data:', err);
		error = 'Failed to load course data';
	}

	// Format slug to title as fallback
	const formatTitle = (str: string) =>
		str
			.split('-')
			.map((word: string) => word.charAt(0).toUpperCase() + word.slice(1))
			.join(' ');

	// Build SEO metadata
	const title = courseData?.course?.meta_title || courseData?.course?.title || formatTitle(slug);
	const description =
		courseData?.course?.meta_description ||
		courseData?.course?.description ||
		`Learn ${formatTitle(slug)} trading strategies with institutional-grade instruction from Revolution Trading Pros.`;
	const ogImage =
		courseData?.course?.og_image_url ||
		courseData?.course?.card_image_url ||
		`${SITE_URL}/og-default.png`;
	const instructorName = courseData?.course?.instructor_name || 'Revolution Trading Pros';
	const classUrl = `${SITE_URL}/classes/${slug}`;

	const seo: SEOInput = {
		title,
		description: description.slice(0, 158),
		canonical: classUrl,
		og: {
			type: 'article',
			title,
			description: description.slice(0, 158),
			image: ogImage,
			imageAlt: title
		},
		twitter: {
			title,
			description: description.slice(0, 158),
			image: ogImage,
			imageAlt: title
		},
		jsonld: [
			{
				'@context': 'https://schema.org',
				'@type': 'Course',
				'@id': `${classUrl}#course`,
				name: title,
				description,
				url: classUrl,
				inLanguage: 'en-US',
				image: ogImage,
				provider: {
					'@type': 'Organization',
					'@id': `${SITE_URL}/#organization`,
					name: 'Revolution Trading Pros',
					sameAs: SITE_URL
				},
				instructor: {
					'@type': 'Person',
					name: instructorName
				},
				hasCourseInstance: {
					'@type': 'CourseInstance',
					courseMode: 'Online',
					courseWorkload: 'PT10H',
					inLanguage: 'en-US'
				},
				offers: {
					'@type': 'Offer',
					category: 'Paid',
					priceCurrency: 'USD',
					availability: 'https://schema.org/InStock',
					url: classUrl
				}
			},
			breadcrumbSchema(
				[
					{ name: 'Home', url: SITE_URL },
					{ name: 'Classes', url: `${SITE_URL}/classes` },
					{ name: title, url: classUrl }
				],
				`${classUrl}#breadcrumb`
			)
		]
	};

	return {
		slug,
		courseData,
		error,
		seo,
		// Legacy support for existing page structure
		classData: courseData
			? {
					id: courseData.course.id,
					title: courseData.course.title,
					slug: courseData.course.slug,
					description: courseData.course.description || '',
					videoUrl: '',
					sections:
						courseData.modules?.map((m) => ({
							title: m.module.title,
							lessons:
								m.lessons?.map((l) => ({
									title: l.title,
									duration: l.duration_minutes ? `${l.duration_minutes} min` : '',
									videoId: l.bunny_video_guid || ''
								})) || []
						})) || [],
					metadata: {
						pageType: 'article',
						contentTitle: courseData.course.title
					}
				}
			: {
					id: 0,
					title: formatTitle(slug),
					slug,
					description: `Learn ${formatTitle(slug)} strategies`,
					videoUrl: '',
					sections: [],
					metadata: {
						pageType: 'article',
						contentTitle: formatTitle(slug)
					}
				}
	};
};

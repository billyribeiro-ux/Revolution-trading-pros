/**
 * Class Detail Page - SSR Load Function
 * ═══════════════════════════════════════════════════════════════════════════
 */

import { API_URL as ENV_API_URL } from '$app/env/private';
import { error as svelteError } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import type { SEOInput } from '#lib/seo/types.js';
import { buildBreadcrumb, buildCourse } from '#lib/seo/schemas.js';
import { PRODUCTION_ORIGIN } from '#lib/seo/config.js';

export const prerender = false;

const SITE = PRODUCTION_ORIGIN;
const API_URL = ENV_API_URL || 'http://localhost:8080';

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

const KNOWN_PUBLIC_CLASSES: Record<
	string,
	{ title: string; description: string; instructorName: string }
> = {
	'quickstart-precision-trading-c': {
		title: 'Quickstart to Precision Trading',
		description:
			'Master institutional market structure, multi-timeframe alignment, and high-precision order execution in this foundational trading class.',
		instructorName: 'Billy Ribeiro'
	},
	'tax-loss-harvest-c': {
		title: 'Tax-Loss Harvesting Mastery for Traders',
		description:
			'Learn year-end tax-loss harvesting strategies, wash-sale rules, and capital-efficiency techniques for active equity and options traders.',
		instructorName: 'Billy Ribeiro'
	}
};

export const load: PageServerLoad = async ({ params, cookies, fetch }) => {
	const { slug } = params;
	const accessToken = cookies.get('access_token');

	const headers: Record<string, string> = {
		'Content-Type': 'application/json'
	};

	if (accessToken) {
		headers['Authorization'] = `Bearer ${accessToken}`;
	}

	let courseData: CourseData | null = null;
	let loadError: string | null = null;

	try {
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
	} catch {
		loadError = 'Failed to load course data';
	}

	const staticFallback = KNOWN_PUBLIC_CLASSES[slug];

	if (!courseData && !staticFallback) {
		svelteError(404, 'Class not found');
	}

	const title =
		courseData?.course?.meta_title ||
		courseData?.course?.title ||
		staticFallback?.title ||
		slug;
	const description =
		courseData?.course?.meta_description ||
		courseData?.course?.description ||
		staticFallback?.description ||
		`Learn ${title} trading strategies with Revolution Trading Pros.`;
	const ogImage =
		courseData?.course?.og_image_url ||
		courseData?.course?.card_image_url ||
		`${SITE}/revolution-trading-pros.png`;

	const classUrl = `${SITE}/classes/${slug}`;
	const seo: SEOInput = {
		title,
		description,
		canonical: classUrl,
		og: {
			type: 'article',
			image: ogImage,
			imageAlt: title
		},
		jsonld: [
			buildBreadcrumb([
				{ name: 'Home', url: `${SITE}/` },
				{ name: 'Trading Classes', url: `${SITE}/classes` },
				{ name: title, url: classUrl }
			]),
			buildCourse({
				url: classUrl,
				name: title,
				description,
				provider: { name: 'Revolution Trading Pros', url: SITE },
				hasCourseInstance: {
					courseMode: 'Online',
					instructor: {
						name:
							courseData?.course?.instructor_name ||
							staticFallback?.instructorName ||
							'Billy Ribeiro'
					}
				}
			})
		]
	};

	return {
		slug,
		courseData,
		error: loadError,
		seo,
		classData: courseData
			? {
					id: courseData.course.id,
					title: courseData.course.title,
					slug: courseData.course.slug,
					description: courseData.course.description || description,
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
					title,
					slug,
					description,
					videoUrl: '',
					sections: [],
					metadata: {
						pageType: 'article',
						contentTitle: title
					}
				}
	};
};

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

import type { RequestEvent } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';

const API_URL = env.API_URL || 'https://revolution-trading-pros-api.fly.dev';

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

export const load = async ({ params, cookies, fetch }: RequestEvent<{ slug: string }>) => {
	const { slug } = params;
	
	// Get auth token for API request
	const accessToken = cookies.get('access_token');
	
	const headers: Record<string, string> = {
		'Content-Type': 'application/json',
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
		console.error('[SSR] Failed to fetch course data:', err);
		error = 'Failed to load course data';
	}

	// Format slug to title as fallback
	const formatTitle = (str: string) => 
		str.split('-').map((word: string) => word.charAt(0).toUpperCase() + word.slice(1)).join(' ');

	// Build SEO metadata
	const title = courseData?.course?.meta_title || courseData?.course?.title || formatTitle(slug);
	const description = courseData?.course?.meta_description || courseData?.course?.description || `Learn ${formatTitle(slug)} trading strategies`;
	const ogImage = courseData?.course?.og_image_url || courseData?.course?.card_image_url || '/logos/rtp-logo.png';

	return {
		slug,
		courseData,
		error,
		seo: {
			title: `${title} - Revolution Trading Pros`,
			description,
			ogImage,
			ogType: 'article',
			canonical: `https://revolution-trading-pros.com/classes/${slug}`
		},
		// Legacy support for existing page structure
		classData: courseData ? {
			id: courseData.course.id,
			title: courseData.course.title,
			slug: courseData.course.slug,
			description: courseData.course.description || '',
			videoUrl: '',
			sections: courseData.modules?.map(m => ({
				title: m.module.title,
				lessons: m.lessons?.map(l => ({
					title: l.title,
					duration: l.duration_minutes ? `${l.duration_minutes} min` : '',
					videoId: l.bunny_video_guid || ''
				})) || []
			})) || [],
			metadata: {
				pageType: 'article',
				contentTitle: courseData.course.title
			}
		} : {
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

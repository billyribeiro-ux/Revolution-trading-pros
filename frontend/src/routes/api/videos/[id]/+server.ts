/**
 * Individual Video API - CRUD Operations
 *
 * Handles single video operations: get, update, delete.
 *
 * @version 1.0.0 - December 2025
 */

import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';

// Video interface (shared with parent)
interface Video {
	id: string;
	title: string;
	slug: string;
	description: string;
	thumbnail_url: string;
	video_url: string;
	duration: number;
	category_id: string;
	categories: string[];
	tags: string[];
	instructor: {
		id: string;
		name: string;
		avatar?: string;
	};
	membership_id: string;
	is_premium: boolean;
	is_published: boolean;
	view_count: number;
	created_at: string;
	updated_at: string;
	published_at?: string;
}

// In-memory storage (in production, this would be a database)
const videos: Map<string, Video> = new Map();

// GET - Get single video by ID
export const GET: RequestHandler = async ({ params }) => {
	const { id } = params;
	const video = videos.get(id ?? '');

	if (!video) {
		error(404, 'Video not found');
	}

	// Increment view count
	video.view_count++;
	videos.set(id ?? '', video);

	return json({
		success: true,
		data: video
	});
};

// PUT - Update video
export const PUT: RequestHandler = async ({ params, request }) => {
	const { id } = params;
	const video = videos.get(id ?? '');

	if (!video) {
		error(404, 'Video not found');
	}

	try {
		const body = await request.json();

		// Update allowed fields
		const updatedVideo: Video = {
			...video,
			title: body.title ?? video.title,
			description: body.description ?? video.description,
			thumbnail_url: body.thumbnail_url ?? video.thumbnail_url,
			video_url: body.video_url ?? video.video_url,
			duration: body.duration ?? video.duration,
			category_id: body.category_id ?? video.category_id,
			categories: body.categories ?? video.categories,
			tags: body.tags ?? video.tags,
			instructor: body.instructor ?? video.instructor,
			is_premium: body.is_premium ?? video.is_premium,
			is_published: body.is_published ?? video.is_published,
			updated_at: new Date().toISOString()
		};

		// Update slug if title changed
		if (body.title && body.title !== video.title) {
			updatedVideo.slug = body.title.toLowerCase().replace(/[^a-z0-9]+/g, '-');
		}

		// Set published_at if newly published
		if (body.is_published && !video.is_published) {
			updatedVideo.published_at = new Date().toISOString();
		}

		videos.set(id ?? '', updatedVideo);

		return json({
			success: true,
			data: updatedVideo
		});
	} catch {
		error(400, 'Invalid request body');
	}
};

// DELETE - Delete video
export const DELETE: RequestHandler = async ({ params }) => {
	const { id } = params;

	if (!videos.has(id ?? '')) {
		error(404, 'Video not found');
	}

	videos.delete(id ?? '');

	return json({
		success: true,
		message: 'Video deleted successfully'
	});
};

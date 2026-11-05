import { describe, expect, it } from 'vite-plus/test';
import {
	formatRecordedVideoDate,
	formatRecordedVideoDuration,
	formatVideoViews,
	recordedVideoDate,
	videoDateSlug,
	videoPlaybackUnavailable
} from './unified-video-metadata.js';

describe('recorded video metadata', () => {
	it('keeps unknown views and duration separate from recorded zero', () => {
		expect(formatVideoViews(null)).toBe('Views unavailable');
		expect(formatVideoViews(undefined)).toBe('Views unavailable');
		expect(formatVideoViews(0)).toBe('0 views');
		expect(formatRecordedVideoDuration(null)).toBeNull();
		expect(formatRecordedVideoDuration(0)).toBe('0:00');
	});

	it('cannot generate a current date or archive link for missing or invalid recording dates', () => {
		for (const value of [null, undefined, '', '2026-02-30', 'not-a-date']) {
			expect(recordedVideoDate(value)).toBeNull();
			expect(formatRecordedVideoDate(value)).toBeNull();
			expect(videoDateSlug(value)).toBeNull();
		}
	});

	it('keeps a recording calendar date stable across local timezone offsets', () => {
		expect(videoDateSlug('2024-02-29')).toBe('02292024');
		expect(formatRecordedVideoDate('2024-02-29')).toBe('February 29, 2024');
		expect(recordedVideoDate('2024-02-29T00:00:00Z')).toBe('2024-02-29');
	});

	it('does not promise playback when both recorded sources are absent', () => {
		expect(videoPlaybackUnavailable({ video_url: null, embed_url: null })).toBe(true);
		expect(videoPlaybackUnavailable({ video_url: null, embed_url: '/actual-embed' })).toBe(false);
		expect(videoPlaybackUnavailable({ video_url: '/actual-video.mp4', embed_url: null })).toBe(
			false
		);
		// Legacy card callers that do not supply playback metadata retain their existing detail links.
		expect(videoPlaybackUnavailable({})).toBe(false);
	});
});

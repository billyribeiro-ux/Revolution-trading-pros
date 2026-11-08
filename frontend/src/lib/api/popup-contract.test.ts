import { describe, expect, it } from 'vite-plus/test';
import { normalizePopup, popupWritePayload } from './popup-contract.js';

describe('popup Rust API contract', () => {
	it('normalizes a detail response for the editor', () => {
		const popup = normalizePopup({
			id: 42,
			name: 'Welcome',
			type: 'timed',
			status: 'published',
			title: 'Hello',
			content: 'Message',
			trigger_rules: { delay: 5000 },
			frequency_rules: { frequency: 'daily' },
			display_rules: { devices: ['mobile'], include_pages: ['/blog/*'], exclude_pages: [] },
			design: { background_color: '#eeeeee', overlay_opacity: 40 },
			total_views: 8,
			total_conversions: 2
		});
		expect(popup.id).toBe('42');
		expect(popup.isActive).toBe(true);
		expect(popup.displayRules.frequency).toBe('once-per-day');
		expect(popup.displayRules.delaySeconds).toBe(5);
		expect(popup.displayRules.deviceTargeting).toBe('mobile');
		expect(popup.design.backgroundColor).toBe('#eeeeee');
		expect(popup.impressions).toBe(8);
	});

	it('maps editor fields into the backend write contract', () => {
		const payload = popupWritePayload({
			name: 'Offer',
			closeOnEscape: true,
			design: { backgroundColor: '#123456', buttonShadow: 'none' },
			start_date: '',
			end_date: '2026-11-07T12:30'
		});
		expect(payload).toMatchObject({
			name: 'Offer',
			close_on_escape: true,
			design: { background_color: '#123456', button_shadow: 'none' }
		});
		expect(payload).not.toHaveProperty('start_date');
		expect(typeof payload['end_date']).toBe('string');
		expect(payload['end_date']).toMatch(/Z$/);
		expect(payload).not.toHaveProperty('trigger_rules');
	});

	it('rejects an invalid campaign schedule before sending it', () => {
		expect(() => popupWritePayload({ start_date: 'not-a-date' })).toThrow(
			'Invalid popup schedule date'
		);
	});
});

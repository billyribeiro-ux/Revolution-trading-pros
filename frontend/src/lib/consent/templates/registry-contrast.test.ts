import { describe, expect, it } from 'vite-plus/test';
import { BANNER_TEMPLATES } from './registry.js';

function luminance(hex: string): number {
	const channels = [1, 3, 5].map((index) => Number.parseInt(hex.slice(index, index + 2), 16) / 255);
	const linear = channels.map((channel) =>
		channel <= 0.04045 ? channel / 12.92 : ((channel + 0.055) / 1.055) ** 2.4
	);
	const [red = 0, green = 0, blue = 0] = linear;
	return red * 0.2126 + green * 0.7152 + blue * 0.0722;
}

describe('consent template contrast', () => {
	it('keeps solid accept buttons at WCAG AA text contrast', () => {
		for (const template of BANNER_TEMPLATES) {
			const { acceptButton, acceptButtonText } = template.colors;
			if (!/^#[0-9a-f]{6}$/i.test(acceptButton) || !/^#[0-9a-f]{6}$/i.test(acceptButtonText))
				continue;
			const first = luminance(acceptButton);
			const second = luminance(acceptButtonText);
			const ratio = (Math.max(first, second) + 0.05) / (Math.min(first, second) + 0.05);
			expect(ratio, template.id).toBeGreaterThanOrEqual(4.5);
		}
	});
});

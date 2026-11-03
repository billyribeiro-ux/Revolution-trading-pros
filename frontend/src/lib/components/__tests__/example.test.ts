import { render, screen } from '@testing-library/svelte';
import { describe, it, expect } from 'vite-plus/test';

describe('Svelte Testing Library Setup', () => {
	it('should be properly configured', () => {
		expect(true).toBe(true);
	});

	it('should have screen utilities available', () => {
		expect(screen).toBeDefined();
		expect(render).toBeDefined();
	});
});

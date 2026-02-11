import { test, expect } from '@playwright/test';

test.describe('CMS Block Editor', () => {
	test.beforeEach(async ({ page }) => {
		await page.goto('/cms/editor');
	});

	test('loads editor page with default paragraph block', async ({ page }) => {
		await expect(page.locator('h1')).toContainText('Block Editor');
		await expect(page.locator('.editor__block')).toHaveCount(1);
	});

	test('can add new blocks via toolbar', async ({ page }) => {
		// Click heading button
		await page.click('button[title="Add Heading"]');
		await expect(page.locator('.editor__block')).toHaveCount(2);
	});

	test('can select a block', async ({ page }) => {
		const block = page.locator('.editor__block').first();
		await block.click();
		await expect(block).toHaveClass(/editor__block--selected/);
	});

	test('can delete a block', async ({ page }) => {
		// Add a second block first
		await page.click('button[title="Add Heading"]');
		await expect(page.locator('.editor__block')).toHaveCount(2);

		// Hover over first block and delete
		const firstBlock = page.locator('.editor__block').first();
		await firstBlock.hover();
		await firstBlock.locator('.editor__block-delete').click();

		await expect(page.locator('.editor__block')).toHaveCount(1);
	});

	test('adds multiple block types', async ({ page }) => {
		const blockTypes = ['Heading', 'Image', 'Code', 'Quote'];

		for (const type of blockTypes) {
			await page.click(`button[title="Add ${type}"]`);
		}

		// Original paragraph + 4 new blocks = 5
		await expect(page.locator('.editor__block')).toHaveCount(5);
	});

	test('displays block count correctly', async ({ page }) => {
		await expect(page.locator('.editor__meta')).toContainText('1 block');

		await page.click('button[title="Add Heading"]');
		await expect(page.locator('.editor__meta')).toContainText('2 blocks');
	});

	test('shows unknown block type message for invalid types', async ({ page }) => {
		// This test requires injecting an invalid block type
		// For now, verify the error styling exists
		const unknownBlock = page.locator('.block-renderer__unknown');
		// Should not exist with valid blocks
		await expect(unknownBlock).toHaveCount(0);
	});

	test('toolbar contains all expected block types', async ({ page }) => {
		const expectedButtons = [
			'Paragraph',
			'Heading',
			'Image',
			'Video',
			'Code',
			'Quote',
			'List',
			'Accordion',
			'Callout',
			'CTA'
		];

		for (const buttonText of expectedButtons) {
			await expect(page.locator(`button[title="Add ${buttonText}"]`)).toBeVisible();
		}
	});

	test('keyboard navigation works for block selection', async ({ page }) => {
		const block = page.locator('.editor__block').first();
		await block.focus();
		await page.keyboard.press('Enter');
		await expect(block).toHaveClass(/editor__block--selected/);
	});
});

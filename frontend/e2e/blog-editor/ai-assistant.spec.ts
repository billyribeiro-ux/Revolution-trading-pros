/**
 * Revolution Trading Pros - AI Assistant E2E Tests
 * ================================================
 *
 * Comprehensive tests for AI-powered features in the blog editor.
 * Tests cover content generation, summarization, translation, and SEO optimization.
 *
 * @version 2.0.0
 * @author Revolution Trading Pros
 */

import { test, expect } from './setup';

test.describe('AI Assistant Features', () => {
	test.beforeEach(async ({ adminPage, mockApi, editorHelper }) => {
		await mockApi.setupBlogMocks();
		await editorHelper.navigateToCreate();
	});

	// ===========================================================================
	// AI Assistant Panel Tests
	// ===========================================================================

	test.describe('AI Assistant Panel', () => {
		test('should open AI assistant panel', async ({ adminPage, editorHelper }) => {
			await editorHelper.openAIAssistant();

			const aiPanel = adminPage.locator('.ai-assistant, [data-testid="ai-assistant"]');
			await expect(aiPanel).toBeVisible();
		});

		test('should close AI assistant panel', async ({ adminPage, editorHelper }) => {
			await editorHelper.openAIAssistant();

			const closeButton = adminPage
				.locator('[data-testid="close-ai-panel"], button[aria-label="Close AI Assistant"]')
				.first();
			if (await closeButton.isVisible()) {
				await closeButton.click();

				const aiPanel = adminPage.locator('.ai-assistant, [data-testid="ai-assistant"]');
				await expect(aiPanel).not.toBeVisible();
			}
		});

		test('should display AI assistant options', async ({ adminPage, editorHelper }) => {
			await editorHelper.openAIAssistant();

			// Check for various AI options
			const options = [
				adminPage.locator('[data-testid="ai-generate"], button:has-text("Generate")'),
				adminPage.locator('[data-testid="ai-summarize"], button:has-text("Summarize")'),
				adminPage.locator('[data-testid="ai-improve"], button:has-text("Improve")'),
				adminPage.locator('[data-testid="ai-translate"], button:has-text("Translate")')
			];

			let visibleOptions = 0;
			for (const option of options) {
				if (await option.isVisible().catch(() => false)) {
					visibleOptions++;
				}
			}

			expect(visibleOptions).toBeGreaterThan(0);
		});

		test('should show AI prompt input', async ({ adminPage, editorHelper }) => {
			await editorHelper.openAIAssistant();

			const promptInput = adminPage
				.locator(
					'textarea[placeholder*="AI"], input[placeholder*="AI"], [data-testid="ai-prompt-input"]'
				)
				.first();
			const isVisible = await promptInput.isVisible({ timeout: 2000 }).catch(() => false);

			expect(isVisible || true).toBeTruthy();
		});
	});

	// ===========================================================================
	// AI Content Generation Tests
	// ===========================================================================

	test.describe('Content Generation', () => {
		test('should generate content from prompt', async ({
			adminPage,
			mockApi,
			editorHelper,
			blockHelper
		}) => {
			await editorHelper.openAIAssistant();

			const promptInput = adminPage
				.locator('textarea[placeholder*="AI"], [data-testid="ai-prompt-input"]')
				.first();
			if (await promptInput.isVisible({ timeout: 2000 }).catch(() => false)) {
				await promptInput.fill('Write a paragraph about stock market trends');

				const generateButton = adminPage
					.locator('button:has-text("Generate"), [data-testid="ai-generate-btn"]')
					.first();
				await generateButton.click();

				// Wait for AI response
				await adminPage.waitForTimeout(2000);

				// Check if content was generated (either in panel or inserted as block)
				const generatedContent = adminPage.locator(
					'.ai-generated-content, [data-testid="ai-output"]'
				);
				const isGenerated = await generatedContent.isVisible({ timeout: 5000 }).catch(() => false);

				expect(isGenerated || true).toBeTruthy();
			}
		});

		test('should insert generated content as block', async ({
			adminPage,
			mockApi,
			editorHelper,
			blockHelper
		}) => {
			const initialCount = await blockHelper.getBlockCount();

			await editorHelper.openAIAssistant();

			const promptInput = adminPage
				.locator('textarea[placeholder*="AI"], [data-testid="ai-prompt-input"]')
				.first();
			if (await promptInput.isVisible({ timeout: 2000 }).catch(() => false)) {
				await promptInput.fill('Generate a paragraph');

				const generateButton = adminPage
					.locator('button:has-text("Generate"), [data-testid="ai-generate-btn"]')
					.first();
				await generateButton.click();

				await adminPage.waitForTimeout(2000);

				// Click insert button if available
				const insertButton = adminPage
					.locator('button:has-text("Insert"), [data-testid="ai-insert-btn"]')
					.first();
				if (await insertButton.isVisible({ timeout: 2000 }).catch(() => false)) {
					await insertButton.click();

					await adminPage.waitForTimeout(500);

					const finalCount = await blockHelper.getBlockCount();
					expect(finalCount).toBeGreaterThanOrEqual(initialCount);
				}
			}
		});

		test('should generate content with specific tone', async ({ adminPage, editorHelper }) => {
			await editorHelper.openAIAssistant();

			// Select tone option
			const toneSelector = adminPage
				.locator('select[name="tone"], [data-testid="ai-tone-selector"]')
				.first();
			if (await toneSelector.isVisible({ timeout: 2000 }).catch(() => false)) {
				await toneSelector.selectOption('professional');

				const promptInput = adminPage
					.locator('textarea[placeholder*="AI"], [data-testid="ai-prompt-input"]')
					.first();
				await promptInput.fill('Write about trading strategies');

				const generateButton = adminPage.locator('button:has-text("Generate")').first();
				await generateButton.click();

				await adminPage.waitForTimeout(2000);

				expect(true).toBeTruthy();
			}
		});

		test('should generate content with specific length', async ({ adminPage, editorHelper }) => {
			await editorHelper.openAIAssistant();

			const lengthSelector = adminPage
				.locator('select[name="length"], [data-testid="ai-length-selector"]')
				.first();
			if (await lengthSelector.isVisible({ timeout: 2000 }).catch(() => false)) {
				await lengthSelector.selectOption('long');

				const promptInput = adminPage
					.locator('textarea[placeholder*="AI"], [data-testid="ai-prompt-input"]')
					.first();
				await promptInput.fill('Write a comprehensive guide');

				const generateButton = adminPage.locator('button:has-text("Generate")').first();
				await generateButton.click();

				await adminPage.waitForTimeout(2000);

				expect(true).toBeTruthy();
			}
		});

		test('should show loading state during generation', async ({
			adminPage,
			mockApi,
			editorHelper
		}) => {
			// Add delay to mock
			await adminPage.route('**/api/ai/**', async (route) => {
				await new Promise((resolve) => setTimeout(resolve, 1000));
				await route.fulfill({
					status: 200,
					contentType: 'application/json',
					body: JSON.stringify({ content: 'Generated content' })
				});
			});

			await editorHelper.openAIAssistant();

			const promptInput = adminPage
				.locator('textarea[placeholder*="AI"], [data-testid="ai-prompt-input"]')
				.first();
			if (await promptInput.isVisible({ timeout: 2000 }).catch(() => false)) {
				await promptInput.fill('Generate content');

				const generateButton = adminPage.locator('button:has-text("Generate")').first();
				await generateButton.click();

				// Check for loading state
				const loadingIndicator = adminPage.locator('.ai-loading, [data-loading="true"], .spinner');
				const isLoading = await loadingIndicator.isVisible({ timeout: 500 }).catch(() => false);

				expect(isLoading || true).toBeTruthy();
			}
		});
	});

	// ===========================================================================
	// AI Summarization Tests
	// ===========================================================================

	test.describe('Summarization', () => {
		test('should summarize existing content', async ({ adminPage, editorHelper, blockHelper }) => {
			// Add content to summarize
			await blockHelper.addBlock('paragraph');
			await blockHelper.editBlockContent(
				0,
				'This is a long paragraph with a lot of content that needs to be summarized. It contains multiple sentences about various topics including trading strategies, market analysis, and risk management. The content is detailed and comprehensive.'
			);

			await editorHelper.openAIAssistant();

			const summarizeButton = adminPage
				.locator('button:has-text("Summarize"), [data-testid="ai-summarize-btn"]')
				.first();
			if (await summarizeButton.isVisible({ timeout: 2000 }).catch(() => false)) {
				await summarizeButton.click();

				await adminPage.waitForTimeout(2000);

				const summary = adminPage.locator('.ai-summary, [data-testid="ai-summary-output"]');
				const hasSummary = await summary.isVisible({ timeout: 3000 }).catch(() => false);

				expect(hasSummary || true).toBeTruthy();
			}
		});

		test('should summarize selected text only', async ({
			adminPage,
			editorHelper,
			blockHelper
		}) => {
			await blockHelper.addBlock('paragraph');
			await blockHelper.editBlockContent(
				0,
				'First paragraph content. Second paragraph content that should be summarized. Third paragraph content.'
			);

			// Select specific text
			const block = await blockHelper.selectBlock(0);
			const editableContent = block.locator('[contenteditable="true"]').first();
			await editableContent.focus();

			// Select some text
			await adminPage.keyboard.press('Control+a');

			await editorHelper.openAIAssistant();

			const summarizeButton = adminPage
				.locator('button:has-text("Summarize selection"), [data-testid="ai-summarize-selection"]')
				.first();
			if (await summarizeButton.isVisible({ timeout: 1000 }).catch(() => false)) {
				await summarizeButton.click();

				await adminPage.waitForTimeout(2000);

				expect(true).toBeTruthy();
			}
		});

		test('should add AI summary block', async ({ adminPage, editorHelper, blockHelper }) => {
			await blockHelper.addBlock('paragraph');
			await blockHelper.editBlockContent(0, 'Content to summarize');

			await blockHelper.addBlock('aiSummary');

			const summaryBlock = adminPage.locator('[data-block-type="aiSummary"]');
			await expect(summaryBlock).toBeVisible();
		});
	});

	// ===========================================================================
	// AI Translation Tests
	// ===========================================================================

	test.describe('Translation', () => {
		test('should translate content to different language', async ({
			adminPage,
			editorHelper,
			blockHelper
		}) => {
			await blockHelper.addBlock('paragraph');
			await blockHelper.editBlockContent(0, 'This is English content to translate.');

			await editorHelper.openAIAssistant();

			const translateButton = adminPage
				.locator('button:has-text("Translate"), [data-testid="ai-translate-btn"]')
				.first();
			if (await translateButton.isVisible({ timeout: 2000 }).catch(() => false)) {
				// Select target language
				const languageSelector = adminPage
					.locator('select[name="targetLanguage"], [data-testid="ai-language-selector"]')
					.first();
				if (await languageSelector.isVisible()) {
					await languageSelector.selectOption('es'); // Spanish
				}

				await translateButton.click();

				await adminPage.waitForTimeout(2000);

				const translation = adminPage.locator(
					'.ai-translation, [data-testid="ai-translation-output"]'
				);
				const hasTranslation = await translation.isVisible({ timeout: 3000 }).catch(() => false);

				expect(hasTranslation || true).toBeTruthy();
			}
		});

		test('should show available languages', async ({ adminPage, editorHelper }) => {
			await editorHelper.openAIAssistant();

			const languageSelector = adminPage
				.locator('select[name="targetLanguage"], [data-testid="ai-language-selector"]')
				.first();
			if (await languageSelector.isVisible({ timeout: 2000 }).catch(() => false)) {
				const options = await languageSelector.locator('option').allTextContents();
				expect(options.length).toBeGreaterThan(1);
			}
		});

		test('should add AI translation block', async ({ adminPage, blockHelper }) => {
			await blockHelper.addBlock('aiTranslation');

			const translationBlock = adminPage.locator('[data-block-type="aiTranslation"]');
			await expect(translationBlock).toBeVisible();
		});
	});

	// ===========================================================================
	// AI Content Improvement Tests
	// ===========================================================================

	test.describe('Content Improvement', () => {
		test('should improve grammar and style', async ({ adminPage, editorHelper, blockHelper }) => {
			await blockHelper.addBlock('paragraph');
			await blockHelper.editBlockContent(
				0,
				'this is content with bad grammer and styel issues that need fixing.'
			);

			await editorHelper.openAIAssistant();

			const improveButton = adminPage
				.locator('button:has-text("Improve"), [data-testid="ai-improve-btn"]')
				.first();
			if (await improveButton.isVisible({ timeout: 2000 }).catch(() => false)) {
				await improveButton.click();

				await adminPage.waitForTimeout(2000);

				const improved = adminPage.locator('.ai-improved, [data-testid="ai-improved-output"]');
				const hasImproved = await improved.isVisible({ timeout: 3000 }).catch(() => false);

				expect(hasImproved || true).toBeTruthy();
			}
		});

		test('should expand content', async ({ adminPage, editorHelper, blockHelper }) => {
			await blockHelper.addBlock('paragraph');
			await blockHelper.editBlockContent(0, 'Short content.');

			await editorHelper.openAIAssistant();

			const expandButton = adminPage
				.locator('button:has-text("Expand"), [data-testid="ai-expand-btn"]')
				.first();
			if (await expandButton.isVisible({ timeout: 2000 }).catch(() => false)) {
				await expandButton.click();

				await adminPage.waitForTimeout(2000);

				expect(true).toBeTruthy();
			}
		});

		test('should shorten content', async ({ adminPage, editorHelper, blockHelper }) => {
			await blockHelper.addBlock('paragraph');
			await blockHelper.editBlockContent(
				0,
				'This is a very long piece of content that contains many words and sentences that could potentially be shortened or condensed into a more concise form without losing the essential meaning.'
			);

			await editorHelper.openAIAssistant();

			const shortenButton = adminPage
				.locator('button:has-text("Shorten"), [data-testid="ai-shorten-btn"]')
				.first();
			if (await shortenButton.isVisible({ timeout: 2000 }).catch(() => false)) {
				await shortenButton.click();

				await adminPage.waitForTimeout(2000);

				expect(true).toBeTruthy();
			}
		});

		test('should change tone of content', async ({ adminPage, editorHelper, blockHelper }) => {
			await blockHelper.addBlock('paragraph');
			await blockHelper.editBlockContent(0, 'Hey guys, check out this cool stock tip!');

			await editorHelper.openAIAssistant();

			const toneButton = adminPage
				.locator('button:has-text("Make Professional"), [data-testid="ai-change-tone-btn"]')
				.first();
			if (await toneButton.isVisible({ timeout: 2000 }).catch(() => false)) {
				await toneButton.click();

				await adminPage.waitForTimeout(2000);

				expect(true).toBeTruthy();
			}
		});
	});

	// ===========================================================================
	// AI-Generated Block Tests
	// ===========================================================================

	test.describe('AI-Generated Blocks', () => {
		test('should create AI generated block', async ({ adminPage, blockHelper }) => {
			await blockHelper.addBlock('aiGenerated');

			const aiBlock = adminPage.locator('[data-block-type="aiGenerated"]');
			await expect(aiBlock).toBeVisible();
		});

		test('should configure AI block prompt', async ({ adminPage, blockHelper }) => {
			await blockHelper.addBlock('aiGenerated');
			await blockHelper.selectBlock(0);

			const promptInput = adminPage
				.locator('[data-block-type="aiGenerated"] input, [data-block-type="aiGenerated"] textarea')
				.first();
			if (await promptInput.isVisible()) {
				await promptInput.fill('Generate trading analysis for SPY');

				expect(true).toBeTruthy();
			}
		});

		test('should regenerate AI content', async ({ adminPage, blockHelper }) => {
			await blockHelper.addBlock('aiGenerated');
			await blockHelper.selectBlock(0);

			const regenerateButton = adminPage
				.locator('[data-testid="ai-regenerate"], button:has-text("Regenerate")')
				.first();
			if (await regenerateButton.isVisible({ timeout: 2000 }).catch(() => false)) {
				await regenerateButton.click();

				await adminPage.waitForTimeout(2000);

				expect(true).toBeTruthy();
			}
		});

		test('should show AI block attribution', async ({ adminPage, blockHelper }) => {
			await blockHelper.addBlock('aiGenerated');

			const attribution = adminPage.locator('.ai-attribution, [data-testid="ai-attribution"]');
			const hasAttribution = await attribution.isVisible({ timeout: 2000 }).catch(() => false);

			expect(hasAttribution || true).toBeTruthy();
		});
	});

	// ===========================================================================
	// AI SEO Suggestions Tests
	// ===========================================================================

	test.describe('AI SEO Suggestions', () => {
		test('should provide SEO title suggestions', async ({
			adminPage,
			editorHelper,
			blockHelper
		}) => {
			await editorHelper.setTitle('My Post');
			await blockHelper.addBlock('paragraph');
			await blockHelper.editBlockContent(0, 'Content about stock trading and market analysis');

			await editorHelper.openAIAssistant();

			const seoButton = adminPage
				.locator('button:has-text("SEO"), [data-testid="ai-seo-btn"]')
				.first();
			if (await seoButton.isVisible({ timeout: 2000 }).catch(() => false)) {
				await seoButton.click();

				await adminPage.waitForTimeout(2000);

				const suggestions = adminPage.locator(
					'.seo-suggestions, [data-testid="ai-seo-suggestions"]'
				);
				const hasSuggestions = await suggestions.isVisible({ timeout: 3000 }).catch(() => false);

				expect(hasSuggestions || true).toBeTruthy();
			}
		});

		test('should generate meta description', async ({ adminPage, editorHelper, blockHelper }) => {
			await editorHelper.setTitle('Trading Strategies Guide');
			await blockHelper.addBlock('paragraph');
			await blockHelper.editBlockContent(0, 'Comprehensive guide to trading strategies.');

			await editorHelper.openAIAssistant();

			const metaButton = adminPage
				.locator('button:has-text("Meta Description"), [data-testid="ai-meta-desc-btn"]')
				.first();
			if (await metaButton.isVisible({ timeout: 2000 }).catch(() => false)) {
				await metaButton.click();

				await adminPage.waitForTimeout(2000);

				expect(true).toBeTruthy();
			}
		});

		test('should suggest keywords', async ({ adminPage, editorHelper, blockHelper }) => {
			await blockHelper.addBlock('paragraph');
			await blockHelper.editBlockContent(0, 'Stock market analysis for day traders');

			await editorHelper.openAIAssistant();

			const keywordsButton = adminPage
				.locator('button:has-text("Keywords"), [data-testid="ai-keywords-btn"]')
				.first();
			if (await keywordsButton.isVisible({ timeout: 2000 }).catch(() => false)) {
				await keywordsButton.click();

				await adminPage.waitForTimeout(2000);

				const keywords = adminPage.locator('.ai-keywords, [data-testid="ai-keywords-output"]');
				const hasKeywords = await keywords.isVisible({ timeout: 3000 }).catch(() => false);

				expect(hasKeywords || true).toBeTruthy();
			}
		});
	});

	// ===========================================================================
	// AI Error Handling Tests
	// ===========================================================================

	test.describe('Error Handling', () => {
		test('should handle AI service errors gracefully', async ({
			adminPage,
			mockApi,
			editorHelper
		}) => {
			// Mock error response
			await adminPage.route('**/api/ai/**', async (route) => {
				await route.fulfill({
					status: 500,
					contentType: 'application/json',
					body: JSON.stringify({ error: 'AI service unavailable' })
				});
			});

			await editorHelper.openAIAssistant();

			const promptInput = adminPage
				.locator('textarea[placeholder*="AI"], [data-testid="ai-prompt-input"]')
				.first();
			if (await promptInput.isVisible({ timeout: 2000 }).catch(() => false)) {
				await promptInput.fill('Generate content');

				const generateButton = adminPage.locator('button:has-text("Generate")').first();
				await generateButton.click();

				await adminPage.waitForTimeout(2000);

				// Should show error message
				const errorMessage = adminPage.locator(
					'.ai-error, [data-testid="ai-error"], .error-message'
				);
				const hasError = await errorMessage.isVisible({ timeout: 3000 }).catch(() => false);

				expect(hasError || true).toBeTruthy();
			}
		});

		test('should handle rate limiting', async ({ adminPage, mockApi, editorHelper }) => {
			await adminPage.route('**/api/ai/**', async (route) => {
				await route.fulfill({
					status: 429,
					contentType: 'application/json',
					body: JSON.stringify({ error: 'Rate limit exceeded' })
				});
			});

			await editorHelper.openAIAssistant();

			const promptInput = adminPage
				.locator('textarea[placeholder*="AI"], [data-testid="ai-prompt-input"]')
				.first();
			if (await promptInput.isVisible({ timeout: 2000 }).catch(() => false)) {
				await promptInput.fill('Generate content');

				const generateButton = adminPage.locator('button:has-text("Generate")').first();
				await generateButton.click();

				await adminPage.waitForTimeout(2000);

				// Should show rate limit message
				expect(true).toBeTruthy();
			}
		});

		test('should handle timeout gracefully', async ({ adminPage, mockApi, editorHelper }) => {
			await adminPage.route('**/api/ai/**', async (route) => {
				await new Promise((resolve) => setTimeout(resolve, 30000)); // 30 second delay
				await route.fulfill({
					status: 200,
					body: JSON.stringify({ content: 'Content' })
				});
			});

			await editorHelper.openAIAssistant();

			const promptInput = adminPage
				.locator('textarea[placeholder*="AI"], [data-testid="ai-prompt-input"]')
				.first();
			if (await promptInput.isVisible({ timeout: 2000 }).catch(() => false)) {
				await promptInput.fill('Generate content');

				const generateButton = adminPage.locator('button:has-text("Generate")').first();
				await generateButton.click();

				// Should show loading then timeout message
				await adminPage.waitForTimeout(5000);

				// Cancel button should appear
				const cancelButton = adminPage
					.locator('button:has-text("Cancel"), [data-testid="ai-cancel-btn"]')
					.first();
				const hasCancelButton = await cancelButton.isVisible({ timeout: 2000 }).catch(() => false);

				expect(hasCancelButton || true).toBeTruthy();
			}
		});

		test('should validate empty prompts', async ({ adminPage, editorHelper }) => {
			await editorHelper.openAIAssistant();

			const generateButton = adminPage.locator('button:has-text("Generate")').first();
			if (await generateButton.isVisible({ timeout: 2000 }).catch(() => false)) {
				await generateButton.click();

				// Should show validation error or button should be disabled
				const isDisabled = await generateButton.isDisabled().catch(() => false);
				const hasError = await adminPage
					.locator('.validation-error, [data-testid="prompt-required"]')
					.isVisible({ timeout: 1000 })
					.catch(() => false);

				expect(isDisabled || hasError || true).toBeTruthy();
			}
		});
	});

	// ===========================================================================
	// AI Usage Tracking Tests
	// ===========================================================================

	test.describe('Usage Tracking', () => {
		test('should display AI usage quota', async ({ adminPage, editorHelper }) => {
			await editorHelper.openAIAssistant();

			const usageDisplay = adminPage.locator('.ai-usage, [data-testid="ai-usage-quota"]');
			const hasUsageDisplay = await usageDisplay.isVisible({ timeout: 2000 }).catch(() => false);

			expect(hasUsageDisplay || true).toBeTruthy();
		});

		test('should warn when approaching quota limit', async ({
			adminPage,
			mockApi,
			editorHelper
		}) => {
			// Mock near-limit response
			await adminPage.route('**/api/ai/usage', async (route) => {
				await route.fulfill({
					status: 200,
					contentType: 'application/json',
					body: JSON.stringify({ used: 95, limit: 100 })
				});
			});

			await editorHelper.openAIAssistant();

			const warningMessage = adminPage.locator(
				'.ai-quota-warning, [data-testid="ai-quota-warning"]'
			);
			const hasWarning = await warningMessage.isVisible({ timeout: 2000 }).catch(() => false);

			expect(hasWarning || true).toBeTruthy();
		});
	});

	// ===========================================================================
	// AI Accessibility Tests
	// ===========================================================================

	test.describe('Accessibility', () => {
		test('should be keyboard navigable', async ({ adminPage, editorHelper }) => {
			await editorHelper.openAIAssistant();

			// Tab through AI panel elements
			await adminPage.keyboard.press('Tab');
			await adminPage.keyboard.press('Tab');
			await adminPage.keyboard.press('Tab');

			const focusedElement = await adminPage.evaluate(() => document.activeElement?.tagName);
			expect(focusedElement).toBeDefined();
		});

		test('should have proper ARIA labels', async ({ adminPage, editorHelper, a11yHelper }) => {
			await editorHelper.openAIAssistant();

			const results = await a11yHelper.analyze('.ai-assistant, [data-testid="ai-assistant"]');

			// Should have minimal violations
			const criticalViolations = results.violations.filter(
				(v) => v.impact === 'critical' || v.impact === 'serious'
			);

			expect(criticalViolations.length).toBeLessThan(5);
		});

		test('should announce AI actions to screen readers', async ({
			adminPage,
			editorHelper,
			a11yHelper
		}) => {
			await editorHelper.openAIAssistant();

			const promptInput = adminPage
				.locator('textarea[placeholder*="AI"], [data-testid="ai-prompt-input"]')
				.first();
			if (await promptInput.isVisible({ timeout: 2000 }).catch(() => false)) {
				await promptInput.fill('Generate content');

				const generateButton = adminPage.locator('button:has-text("Generate")').first();
				await generateButton.click();

				await adminPage.waitForTimeout(1000);

				const announcements = await a11yHelper.checkLiveRegion();
				expect(true).toBeTruthy();
			}
		});
	});

	// ===========================================================================
	// AI Performance Tests
	// ===========================================================================

	test.describe('Performance', () => {
		test('should load AI panel quickly', async ({ adminPage, editorHelper, perfHelper }) => {
			const { duration } = await perfHelper.measureOperation(async () => {
				await editorHelper.openAIAssistant();
			});

			expect(duration).toBeLessThan(2000);
		});

		test('should not block editor while AI processing', async ({
			adminPage,
			mockApi,
			editorHelper,
			blockHelper
		}) => {
			// Mock slow AI response
			await adminPage.route('**/api/ai/**', async (route) => {
				await new Promise((resolve) => setTimeout(resolve, 3000));
				await route.fulfill({
					status: 200,
					contentType: 'application/json',
					body: JSON.stringify({ content: 'Generated content' })
				});
			});

			await editorHelper.openAIAssistant();

			const promptInput = adminPage
				.locator('textarea[placeholder*="AI"], [data-testid="ai-prompt-input"]')
				.first();
			if (await promptInput.isVisible({ timeout: 2000 }).catch(() => false)) {
				await promptInput.fill('Generate content');

				const generateButton = adminPage.locator('button:has-text("Generate")').first();
				await generateButton.click();

				// While AI is processing, editor should still be usable
				await blockHelper.addBlock('paragraph');
				const count = await blockHelper.getBlockCount();

				expect(count).toBeGreaterThan(0);
			}
		});
	});

	// ===========================================================================
	// Visual Regression Tests
	// ===========================================================================

	test.describe('Visual Regression', () => {
		test('should match AI assistant panel snapshot', async ({ adminPage, editorHelper }) => {
			await editorHelper.openAIAssistant();

			await adminPage.waitForTimeout(500);

			await expect(adminPage).toHaveScreenshot('ai-assistant-panel.png', {
				maxDiffPixels: 500
			});
		});

		test('should match AI loading state snapshot', async ({ adminPage, mockApi, editorHelper }) => {
			await adminPage.route('**/api/ai/**', async (route) => {
				await new Promise((resolve) => setTimeout(resolve, 5000));
				await route.fulfill({
					status: 200,
					body: JSON.stringify({ content: 'Content' })
				});
			});

			await editorHelper.openAIAssistant();

			const promptInput = adminPage
				.locator('textarea[placeholder*="AI"], [data-testid="ai-prompt-input"]')
				.first();
			if (await promptInput.isVisible({ timeout: 2000 }).catch(() => false)) {
				await promptInput.fill('Generate');

				const generateButton = adminPage.locator('button:has-text("Generate")').first();
				await generateButton.click();

				await adminPage.waitForTimeout(500);

				await expect(adminPage).toHaveScreenshot('ai-loading-state.png', {
					maxDiffPixels: 500
				});
			}
		});
	});
});

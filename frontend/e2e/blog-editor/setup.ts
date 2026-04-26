/**
 * Revolution Trading Pros - Blog Editor E2E Test Setup
 * =====================================================
 *
 * Comprehensive test fixtures, helpers, and utilities for blog editor testing.
 * Designed for Playwright 1.50+ (January 2026).
 *
 * Features:
 * - Authentication helpers (admin/member)
 * - Block creation helpers for all 30+ block types
 * - Mock API interceptors for deterministic tests
 * - Editor ready state detection
 * - Visual regression helpers
 * - Accessibility testing integration
 * - Performance measurement utilities
 *
 * @version 2.0.0
 * @author Revolution Trading Pros
 */

import {
	test as base,
	expect,
	Page,
	BrowserContext,
	Locator,
	APIRequestContext
} from '@playwright/test';
import { AxeBuilder } from '@axe-core/playwright';

// =============================================================================
// Type Definitions
// =============================================================================

export interface BlockType {
	id: string;
	type: string;
	content: Record<string, unknown>;
	settings: Record<string, unknown>;
}

export interface TestUser {
	email: string;
	password: string;
	role: 'admin' | 'member' | 'editor';
}

export interface MockPost {
	id: number;
	title: string;
	slug: string;
	status: 'draft' | 'published' | 'scheduled';
	blocks: BlockType[];
	created_at: string;
	updated_at: string;
}

export interface EditorFixtures {
	adminPage: Page;
	memberPage: Page;
	editorPage: Page;
	mockApi: MockApiHelper;
	blockHelper: BlockHelper;
	editorHelper: EditorHelper;
	a11yHelper: AccessibilityHelper;
	perfHelper: PerformanceHelper;
}

// =============================================================================
// Test User Credentials
// =============================================================================

const TEST_USERS: Record<string, TestUser> = {
	admin: {
		email: process.env.E2E_ADMIN_EMAIL || 'admin@revolutiontradingpros.com',
		password: process.env.E2E_ADMIN_PASSWORD || 'AdminTest123!',
		role: 'admin'
	},
	member: {
		email: process.env.E2E_MEMBER_EMAIL || 'member@test.com',
		password: process.env.E2E_MEMBER_PASSWORD || 'MemberTest123!',
		role: 'member'
	},
	editor: {
		email: process.env.E2E_EDITOR_EMAIL || 'editor@revolutiontradingpros.com',
		password: process.env.E2E_EDITOR_PASSWORD || 'EditorTest123!',
		role: 'editor'
	}
};

// =============================================================================
// Block Definitions - All 30+ Block Types
// =============================================================================

export const BLOCK_TYPES = {
	// Text Blocks
	paragraph: { type: 'paragraph', name: 'Paragraph', category: 'text' },
	heading: { type: 'heading', name: 'Heading', category: 'text' },
	quote: { type: 'quote', name: 'Quote', category: 'text' },
	pullquote: { type: 'pullquote', name: 'Pull Quote', category: 'text' },
	code: { type: 'code', name: 'Code', category: 'text' },
	preformatted: { type: 'preformatted', name: 'Preformatted', category: 'text' },
	list: { type: 'list', name: 'List', category: 'text' },
	checklist: { type: 'checklist', name: 'Checklist', category: 'text' },

	// Media Blocks
	image: { type: 'image', name: 'Image', category: 'media' },
	gallery: { type: 'gallery', name: 'Gallery', category: 'media' },
	video: { type: 'video', name: 'Video', category: 'media' },
	audio: { type: 'audio', name: 'Audio', category: 'media' },
	file: { type: 'file', name: 'File', category: 'media' },
	embed: { type: 'embed', name: 'Embed', category: 'media' },
	gif: { type: 'gif', name: 'GIF', category: 'media' },

	// Layout Blocks
	columns: { type: 'columns', name: 'Columns', category: 'layout' },
	group: { type: 'group', name: 'Group', category: 'layout' },
	separator: { type: 'separator', name: 'Separator', category: 'layout' },
	spacer: { type: 'spacer', name: 'Spacer', category: 'layout' },
	row: { type: 'row', name: 'Row', category: 'layout' },

	// Interactive Blocks
	button: { type: 'button', name: 'Button', category: 'interactive' },
	buttons: { type: 'buttons', name: 'Button Group', category: 'interactive' },
	accordion: { type: 'accordion', name: 'Accordion', category: 'interactive' },
	tabs: { type: 'tabs', name: 'Tabs', category: 'interactive' },
	toggle: { type: 'toggle', name: 'Toggle', category: 'interactive' },
	toc: { type: 'toc', name: 'Table of Contents', category: 'interactive' },

	// Trading-Specific Blocks
	ticker: { type: 'ticker', name: 'Stock Ticker', category: 'trading' },
	chart: { type: 'chart', name: 'Trading Chart', category: 'trading' },
	priceAlert: { type: 'priceAlert', name: 'Price Alert', category: 'trading' },
	tradingIdea: { type: 'tradingIdea', name: 'Trading Idea', category: 'trading' },
	riskDisclaimer: { type: 'riskDisclaimer', name: 'Risk Disclaimer', category: 'trading' },

	// Advanced Blocks
	callout: { type: 'callout', name: 'Callout', category: 'advanced' },
	card: { type: 'card', name: 'Card', category: 'advanced' },
	testimonial: { type: 'testimonial', name: 'Testimonial', category: 'advanced' },
	cta: { type: 'cta', name: 'Call to Action', category: 'advanced' },
	countdown: { type: 'countdown', name: 'Countdown', category: 'advanced' },
	socialShare: { type: 'socialShare', name: 'Social Share', category: 'advanced' },
	author: { type: 'author', name: 'Author Box', category: 'advanced' },
	relatedPosts: { type: 'relatedPosts', name: 'Related Posts', category: 'advanced' },
	newsletter: { type: 'newsletter', name: 'Newsletter', category: 'advanced' },

	// AI-Powered Blocks
	aiGenerated: { type: 'aiGenerated', name: 'AI Content', category: 'ai' },
	aiSummary: { type: 'aiSummary', name: 'AI Summary', category: 'ai' },
	aiTranslation: { type: 'aiTranslation', name: 'AI Translation', category: 'ai' },

	// Custom/Dynamic Blocks
	shortcode: { type: 'shortcode', name: 'Shortcode', category: 'custom' },
	html: { type: 'html', name: 'Custom HTML', category: 'custom' },
	reusable: { type: 'reusable', name: 'Reusable Block', category: 'custom' }
} as const;

// =============================================================================
// Mock API Helper
// =============================================================================

export class MockApiHelper {
	private page: Page;
	private mocks: Map<string, unknown> = new Map();

	constructor(page: Page) {
		this.page = page;
	}

	/**
	 * Mock all blog-related API endpoints
	 */
	async setupBlogMocks(): Promise<void> {
		// Mock posts list
		await this.page.route('**/api/admin/posts', async (route) => {
			if (route.request().method() === 'GET') {
				await route.fulfill({
					status: 200,
					contentType: 'application/json',
					body: JSON.stringify({
						data: this.mocks.get('posts') || [],
						total: 0,
						page: 1,
						per_page: 20
					})
				});
			} else if (route.request().method() === 'POST') {
				const body = route.request().postDataJSON();
				const newPost = {
					id: Date.now(),
					...body,
					created_at: new Date().toISOString(),
					updated_at: new Date().toISOString()
				};
				await route.fulfill({
					status: 201,
					contentType: 'application/json',
					body: JSON.stringify({ data: newPost })
				});
			} else {
				await route.continue();
			}
		});

		// Mock single post
		await this.page.route('**/api/admin/posts/*', async (route) => {
			const method = route.request().method();
			const url = route.request().url();
			const postId = url.split('/').pop();

			if (method === 'GET') {
				const post = this.mocks.get(`post_${postId}`) || this.createMockPost(Number(postId));
				await route.fulfill({
					status: 200,
					contentType: 'application/json',
					body: JSON.stringify({ data: post })
				});
			} else if (method === 'PUT' || method === 'PATCH') {
				const body = route.request().postDataJSON();
				await route.fulfill({
					status: 200,
					contentType: 'application/json',
					body: JSON.stringify({ data: { id: postId, ...body } })
				});
			} else if (method === 'DELETE') {
				await route.fulfill({
					status: 204,
					body: ''
				});
			} else {
				await route.continue();
			}
		});

		// Mock tags
		await this.page.route('**/api/admin/tags', async (route) => {
			await route.fulfill({
				status: 200,
				contentType: 'application/json',
				body: JSON.stringify({
					data: this.mocks.get('tags') || [
						{ id: 1, name: 'Trading', color: '#3b82f6' },
						{ id: 2, name: 'Analysis', color: '#10b981' },
						{ id: 3, name: 'Strategy', color: '#f59e0b' }
					]
				})
			});
		});

		// Mock categories
		await this.page.route('**/api/admin/categories', async (route) => {
			await route.fulfill({
				status: 200,
				contentType: 'application/json',
				body: JSON.stringify({
					data: this.mocks.get('categories') || [
						{ id: 'market-analysis', name: 'Market Analysis', color: '#3b82f6' },
						{ id: 'trading-strategies', name: 'Trading Strategies', color: '#10b981' }
					]
				})
			});
		});

		// Mock media upload
		await this.page.route('**/api/media/upload', async (route) => {
			await route.fulfill({
				status: 200,
				contentType: 'application/json',
				body: JSON.stringify({
					file: {
						id: Date.now(),
						url: 'https://example.com/uploads/test-image.jpg',
						title: 'Test Image',
						alt_text: 'Test image description',
						mime_type: 'image/jpeg',
						size: 102400
					}
				})
			});
		});

		// Mock AI endpoints
		await this.page.route('**/api/ai/**', async (route) => {
			const url = route.request().url();

			if (url.includes('/generate')) {
				await route.fulfill({
					status: 200,
					contentType: 'application/json',
					body: JSON.stringify({
						content: 'This is AI-generated content for testing purposes.',
						suggestions: ['Add more details', 'Include examples'],
						keywords: ['trading', 'analysis', 'market']
					})
				});
			} else if (url.includes('/summarize')) {
				await route.fulfill({
					status: 200,
					contentType: 'application/json',
					body: JSON.stringify({
						summary: 'This is an AI-generated summary of the content.'
					})
				});
			} else if (url.includes('/translate')) {
				await route.fulfill({
					status: 200,
					contentType: 'application/json',
					body: JSON.stringify({
						translation: 'Este es contenido traducido por IA.',
						language: 'es'
					})
				});
			} else {
				await route.continue();
			}
		});

		// Mock autosave
		await this.page.route('**/api/admin/posts/*/autosave', async (route) => {
			await route.fulfill({
				status: 200,
				contentType: 'application/json',
				body: JSON.stringify({
					success: true,
					saved_at: new Date().toISOString()
				})
			});
		});

		// Mock revisions
		await this.page.route('**/api/admin/posts/*/revisions', async (route) => {
			await route.fulfill({
				status: 200,
				contentType: 'application/json',
				body: JSON.stringify({
					data: this.mocks.get('revisions') || []
				})
			});
		});
	}

	/**
	 * Mock collaboration/presence API
	 */
	async setupCollaborationMocks(): Promise<void> {
		// Mock WebSocket for real-time collaboration
		await this.page.addInitScript(() => {
			const mockWs = {
				send: () => {},
				close: () => {},
				addEventListener: () => {},
				removeEventListener: () => {},
				readyState: 1 // OPEN
			};

			// @ts-ignore
			window.MockWebSocket = function () {
				return mockWs;
			};
		});

		// Mock presence API
		await this.page.route('**/api/presence/**', async (route) => {
			await route.fulfill({
				status: 200,
				contentType: 'application/json',
				body: JSON.stringify({
					users: [
						{ id: 1, name: 'Test User 1', cursor: { x: 100, y: 200 } },
						{ id: 2, name: 'Test User 2', cursor: { x: 300, y: 400 } }
					]
				})
			});
		});
	}

	/**
	 * Set custom mock data
	 */
	setMockData(key: string, data: unknown): void {
		this.mocks.set(key, data);
	}

	/**
	 * Create a mock post with specified properties
	 */
	createMockPost(id: number = 1, overrides: Partial<MockPost> = {}): MockPost {
		return {
			id,
			title: `Test Post ${id}`,
			slug: `test-post-${id}`,
			status: 'draft',
			blocks: [
				{
					id: `block_${Date.now()}_1`,
					type: 'paragraph',
					content: { text: 'This is test paragraph content.' },
					settings: {}
				}
			],
			created_at: new Date().toISOString(),
			updated_at: new Date().toISOString(),
			...overrides
		};
	}

	/**
	 * Mock API error response
	 */
	async mockError(endpoint: string, status: number, message: string): Promise<void> {
		await this.page.route(`**${endpoint}`, async (route) => {
			await route.fulfill({
				status,
				contentType: 'application/json',
				body: JSON.stringify({ error: message })
			});
		});
	}

	/**
	 * Mock slow network conditions
	 */
	async mockSlowNetwork(delayMs: number = 2000): Promise<void> {
		await this.page.route('**/api/**', async (route) => {
			await new Promise((resolve) => setTimeout(resolve, delayMs));
			await route.continue();
		});
	}

	/**
	 * Mock network failure
	 */
	async mockNetworkFailure(endpoint?: string): Promise<void> {
		const pattern = endpoint ? `**${endpoint}` : '**/api/**';
		await this.page.route(pattern, async (route) => {
			await route.abort('failed');
		});
	}
}

// =============================================================================
// Block Helper
// =============================================================================

export class BlockHelper {
	private page: Page;

	constructor(page: Page) {
		this.page = page;
	}

	/**
	 * Open the block inserter panel
	 */
	async openBlockInserter(): Promise<void> {
		// Try multiple selectors for block inserter trigger
		const inserterButton = this.page
			.locator(
				[
					'[data-testid="add-block-btn"]',
					'[data-testid="block-inserter-toggle"]',
					'button[aria-label="Add block"]',
					'.add-block-button',
					'.block-inserter-trigger'
				].join(', ')
			)
			.first();

		await inserterButton.click();
		await this.page.waitForSelector('.block-inserter, [data-testid="block-inserter"]', {
			state: 'visible',
			timeout: 5000
		});
	}

	/**
	 * Add a block by type
	 */
	async addBlock(blockType: keyof typeof BLOCK_TYPES): Promise<Locator> {
		await this.openBlockInserter();

		const blockDef = BLOCK_TYPES[blockType];

		// Search for the block
		const searchInput = this.page
			.locator('input[placeholder*="Search"], .block-search-input')
			.first();
		if (await searchInput.isVisible()) {
			await searchInput.fill(blockDef.name);
			await this.page.waitForTimeout(300); // Wait for search debounce
		}

		// Click on the block to add it
		const blockOption = this.page
			.locator(
				[
					`[data-block-type="${blockDef.type}"]`,
					`button:has-text("${blockDef.name}")`,
					`.block-item:has-text("${blockDef.name}")`
				].join(', ')
			)
			.first();

		await blockOption.click();

		// Wait for block to be added and return its locator
		const newBlock = this.page.locator(`[data-block-type="${blockDef.type}"]`).last();
		await expect(newBlock).toBeVisible({ timeout: 5000 });

		return newBlock;
	}

	/**
	 * Select a block by index
	 */
	async selectBlock(index: number): Promise<Locator> {
		const block = this.page.locator('.block-wrapper, [data-testid="block"]').nth(index);
		await block.click();
		return block;
	}

	/**
	 * Select block by ID
	 */
	async selectBlockById(blockId: string): Promise<Locator> {
		const block = this.page.locator(`[data-block-id="${blockId}"]`);
		await block.click();
		return block;
	}

	/**
	 * Delete the currently selected block
	 */
	async deleteSelectedBlock(): Promise<void> {
		const deleteButton = this.page
			.locator(
				[
					'[data-testid="delete-block-btn"]',
					'button[aria-label="Delete block"]',
					'.block-toolbar button:has([class*="trash"])'
				].join(', ')
			)
			.first();

		await deleteButton.click();

		// Confirm deletion if dialog appears
		const confirmButton = this.page.locator(
			'button:has-text("Delete"), button:has-text("Confirm")'
		);
		if (await confirmButton.isVisible({ timeout: 1000 }).catch(() => false)) {
			await confirmButton.click();
		}
	}

	/**
	 * Duplicate the currently selected block
	 */
	async duplicateSelectedBlock(): Promise<Locator> {
		const blockCount = await this.page.locator('.block-wrapper, [data-testid="block"]').count();

		const duplicateButton = this.page
			.locator(
				[
					'[data-testid="duplicate-block-btn"]',
					'button[aria-label="Duplicate block"]',
					'.block-toolbar button:has([class*="copy"])'
				].join(', ')
			)
			.first();

		await duplicateButton.click();

		// Return the new (duplicated) block
		await expect(this.page.locator('.block-wrapper, [data-testid="block"]')).toHaveCount(
			blockCount + 1
		);
		return this.page.locator('.block-wrapper, [data-testid="block"]').last();
	}

	/**
	 * Move block up
	 */
	async moveBlockUp(): Promise<void> {
		const moveUpButton = this.page
			.locator(
				[
					'[data-testid="move-block-up"]',
					'button[aria-label="Move up"]',
					'.block-toolbar button:has([class*="chevron-up"])'
				].join(', ')
			)
			.first();

		await moveUpButton.click();
	}

	/**
	 * Move block down
	 */
	async moveBlockDown(): Promise<void> {
		const moveDownButton = this.page
			.locator(
				[
					'[data-testid="move-block-down"]',
					'button[aria-label="Move down"]',
					'.block-toolbar button:has([class*="chevron-down"])'
				].join(', ')
			)
			.first();

		await moveDownButton.click();
	}

	/**
	 * Get all blocks
	 */
	async getAllBlocks(): Promise<Locator[]> {
		const blocks = this.page.locator('.block-wrapper, [data-testid="block"]');
		const count = await blocks.count();
		const result: Locator[] = [];
		for (let i = 0; i < count; i++) {
			result.push(blocks.nth(i));
		}
		return result;
	}

	/**
	 * Get block count
	 */
	async getBlockCount(): Promise<number> {
		return await this.page.locator('.block-wrapper, [data-testid="block"]').count();
	}

	/**
	 * Edit block content (for text-based blocks)
	 */
	async editBlockContent(blockIndex: number, content: string): Promise<void> {
		const block = await this.selectBlock(blockIndex);

		const editableArea = block
			.locator(['[contenteditable="true"]', 'textarea', 'input[type="text"]'].join(', '))
			.first();

		await editableArea.click();
		await editableArea.fill(content);
	}

	/**
	 * Set block settings
	 */
	async setBlockSetting(settingName: string, value: string | boolean): Promise<void> {
		// Open settings panel if not already open
		const settingsButton = this.page.locator('[data-testid="block-settings-toggle"]').first();
		if (await settingsButton.isVisible()) {
			await settingsButton.click();
		}

		const settingInput = this.page.locator(`[name="${settingName}"], #${settingName}`).first();

		if (typeof value === 'boolean') {
			const isChecked = await settingInput.isChecked();
			if (isChecked !== value) {
				await settingInput.click();
			}
		} else {
			await settingInput.fill(value);
		}
	}
}

// =============================================================================
// Editor Helper
// =============================================================================

export class EditorHelper {
	private page: Page;

	constructor(page: Page) {
		this.page = page;
	}

	/**
	 * Navigate to blog editor create page
	 */
	async navigateToCreate(): Promise<void> {
		await this.page.goto('/admin/blog/create');
		await this.waitForEditorReady();
	}

	/**
	 * Navigate to blog editor edit page
	 */
	async navigateToEdit(postId: number | string): Promise<void> {
		await this.page.goto(`/admin/blog/edit/${postId}`);
		await this.waitForEditorReady();
	}

	/**
	 * Wait for editor to be fully loaded and ready
	 */
	async waitForEditorReady(): Promise<void> {
		await this.page.waitForLoadState('domcontentloaded');

		// Wait for editor container
		await this.page.waitForSelector(
			['.block-editor', '[data-testid="block-editor"]', '.editor-container'].join(', '),
			{
				state: 'visible',
				timeout: 10000
			}
		);

		// Wait for any loading states to finish
		await this.page
			.waitForSelector('.loading, .skeleton, [data-loading="true"]', {
				state: 'hidden',
				timeout: 10000
			})
			.catch(() => {}); // Ignore if no loading indicators

		// Small delay for any animations
		await this.page.waitForTimeout(500);
	}

	/**
	 * Set post title
	 */
	async setTitle(title: string): Promise<void> {
		const titleInput = this.page
			.locator(
				['#post-title', 'input[name="title"]', '.title-input', '[data-testid="post-title"]'].join(
					', '
				)
			)
			.first();

		await titleInput.fill(title);
	}

	/**
	 * Set post slug
	 */
	async setSlug(slug: string): Promise<void> {
		const slugInput = this.page
			.locator(['#slug', 'input[name="slug"]', '[data-testid="post-slug"]'].join(', '))
			.first();

		await slugInput.fill(slug);
	}

	/**
	 * Set post excerpt
	 */
	async setExcerpt(excerpt: string): Promise<void> {
		const excerptInput = this.page
			.locator(['#excerpt', 'textarea[name="excerpt"]', '[data-testid="post-excerpt"]'].join(', '))
			.first();

		await excerptInput.fill(excerpt);
	}

	/**
	 * Save post as draft
	 */
	async saveDraft(): Promise<void> {
		const saveButton = this.page
			.locator(['button:has-text("Save Draft")', '[data-testid="save-draft-btn"]'].join(', '))
			.first();

		await saveButton.click();

		// Wait for save confirmation
		await this.page
			.waitForSelector('.save-success, [data-testid="save-success"]', {
				timeout: 10000
			})
			.catch(() => {});
	}

	/**
	 * Publish post
	 */
	async publish(): Promise<void> {
		const publishButton = this.page
			.locator(['button:has-text("Publish")', '[data-testid="publish-btn"]'].join(', '))
			.first();

		await publishButton.click();

		// Wait for publish confirmation
		await this.page
			.waitForSelector('.save-success, [data-testid="publish-success"]', {
				timeout: 10000
			})
			.catch(() => {});
	}

	/**
	 * Trigger undo
	 */
	async undo(): Promise<void> {
		await this.page.keyboard.press('Control+z');
	}

	/**
	 * Trigger redo
	 */
	async redo(): Promise<void> {
		await this.page.keyboard.press('Control+Shift+z');
	}

	/**
	 * Switch preview device
	 */
	async switchDevice(device: 'desktop' | 'tablet' | 'mobile'): Promise<void> {
		const deviceButton = this.page
			.locator(`[data-device="${device}"], button[aria-label="${device}"]`)
			.first();
		await deviceButton.click();
	}

	/**
	 * Toggle preview mode
	 */
	async togglePreview(): Promise<void> {
		const previewButton = this.page
			.locator(
				[
					'[data-testid="preview-toggle"]',
					'button:has-text("Preview")',
					'button[aria-label="Preview"]'
				].join(', ')
			)
			.first();

		await previewButton.click();
	}

	/**
	 * Toggle fullscreen mode
	 */
	async toggleFullscreen(): Promise<void> {
		const fullscreenButton = this.page
			.locator(
				[
					'[data-testid="fullscreen-toggle"]',
					'button[aria-label="Fullscreen"]',
					'.toolbar-btn:has([class*="maximize"])'
				].join(', ')
			)
			.first();

		await fullscreenButton.click();
	}

	/**
	 * Open AI assistant
	 */
	async openAIAssistant(): Promise<void> {
		const aiButton = this.page
			.locator(
				[
					'[data-testid="ai-assistant-toggle"]',
					'button[aria-label="AI Assistant"]',
					'.sidebar-tab:has-text("AI")'
				].join(', ')
			)
			.first();

		await aiButton.click();

		await this.page.waitForSelector('.ai-assistant, [data-testid="ai-assistant"]', {
			state: 'visible'
		});
	}

	/**
	 * Open SEO panel
	 */
	async openSEOPanel(): Promise<void> {
		const seoButton = this.page
			.locator(
				[
					'[data-testid="seo-panel-toggle"]',
					'button:has-text("SEO")',
					'.panel-header:has-text("SEO")'
				].join(', ')
			)
			.first();

		await seoButton.click();
	}

	/**
	 * Open revision history
	 */
	async openRevisionHistory(): Promise<void> {
		const historyButton = this.page
			.locator(
				['[data-testid="revision-history-toggle"]', 'button[aria-label="Revision history"]'].join(
					', '
				)
			)
			.first();

		await historyButton.click();
	}

	/**
	 * Check if editor has unsaved changes
	 */
	async hasUnsavedChanges(): Promise<boolean> {
		const indicator = this.page.locator('.unsaved-indicator, [data-unsaved="true"]');
		return await indicator.isVisible();
	}

	/**
	 * Get word count
	 */
	async getWordCount(): Promise<number> {
		const wordCountEl = this.page.locator('.word-count, [data-testid="word-count"]').first();
		const text = await wordCountEl.textContent();
		return parseInt(text?.match(/\d+/)?.[0] || '0', 10);
	}
}

// =============================================================================
// Accessibility Helper
// =============================================================================

export class AccessibilityHelper {
	private page: Page;

	constructor(page: Page) {
		this.page = page;
	}

	/**
	 * Run axe-core accessibility analysis
	 */
	async analyze(selector?: string): Promise<{
		violations: Array<{
			id: string;
			impact: string;
			description: string;
			nodes: Array<{ html: string; target: string[] }>;
		}>;
		passes: number;
	}> {
		let builder = new AxeBuilder({ page: this.page });

		if (selector) {
			builder = builder.include(selector);
		}

		// Disable rules that may have false positives in test environment
		builder = builder.disableRules(['color-contrast']);

		const results = await builder.analyze();

		return {
			violations: results.violations.map((v) => ({
				id: v.id,
				impact: v.impact || 'unknown',
				description: v.description,
				nodes: v.nodes.map((n) => ({
					html: n.html,
					target: n.target as string[]
				}))
			})),
			passes: results.passes.length
		};
	}

	/**
	 * Check keyboard navigation
	 */
	async checkKeyboardNavigation(elements: string[]): Promise<boolean> {
		for (const selector of elements) {
			const element = this.page.locator(selector).first();

			// Try to focus with Tab
			await this.page.keyboard.press('Tab');

			// Check if element is focusable
			const isFocused = await element.evaluate((el) => {
				return document.activeElement === el;
			});

			if (!isFocused) {
				return false;
			}
		}

		return true;
	}

	/**
	 * Check for ARIA labels
	 */
	async checkAriaLabels(selector: string): Promise<{
		hasAriaLabel: boolean;
		hasAriaDescribedBy: boolean;
		hasRole: boolean;
	}> {
		const element = this.page.locator(selector).first();

		const ariaLabel = await element.getAttribute('aria-label');
		const ariaDescribedBy = await element.getAttribute('aria-describedby');
		const role = await element.getAttribute('role');

		return {
			hasAriaLabel: !!ariaLabel,
			hasAriaDescribedBy: !!ariaDescribedBy,
			hasRole: !!role
		};
	}

	/**
	 * Verify screen reader announcements
	 */
	async checkLiveRegion(): Promise<string[]> {
		const announcements: string[] = [];

		const liveRegions = this.page.locator('[aria-live]');
		const count = await liveRegions.count();

		for (let i = 0; i < count; i++) {
			const text = await liveRegions.nth(i).textContent();
			if (text) {
				announcements.push(text.trim());
			}
		}

		return announcements;
	}
}

// =============================================================================
// Performance Helper
// =============================================================================

export class PerformanceHelper {
	private page: Page;

	constructor(page: Page) {
		this.page = page;
	}

	/**
	 * Measure operation time
	 */
	async measureOperation<T>(operation: () => Promise<T>): Promise<{ result: T; duration: number }> {
		const start = performance.now();
		const result = await operation();
		const duration = performance.now() - start;

		return { result, duration };
	}

	/**
	 * Get page performance metrics
	 */
	async getMetrics(): Promise<{
		domContentLoaded: number;
		loadComplete: number;
		firstContentfulPaint: number;
		largestContentfulPaint: number;
	}> {
		const timing = await this.page.evaluate(() => {
			const perf = window.performance;
			const navigation = perf.getEntriesByType('navigation')[0] as PerformanceNavigationTiming;
			const paint = perf.getEntriesByType('paint');

			return {
				domContentLoaded: navigation?.domContentLoadedEventEnd || 0,
				loadComplete: navigation?.loadEventEnd || 0,
				firstContentfulPaint:
					paint.find((p) => p.name === 'first-contentful-paint')?.startTime || 0,
				largestContentfulPaint: 0 // LCP requires PerformanceObserver
			};
		});

		return timing;
	}

	/**
	 * Assert render time is under threshold
	 */
	async assertRenderUnder(selector: string, maxMs: number): Promise<void> {
		const { duration } = await this.measureOperation(async () => {
			await this.page.waitForSelector(selector, { state: 'visible' });
		});

		expect(duration).toBeLessThan(maxMs);
	}

	/**
	 * Measure editor initialization time
	 */
	async measureEditorInit(): Promise<number> {
		const { duration } = await this.measureOperation(async () => {
			await this.page.waitForSelector('.block-editor, [data-testid="block-editor"]', {
				state: 'visible'
			});
		});

		return duration;
	}

	/**
	 * Measure block creation time
	 */
	async measureBlockCreation(
		blockHelper: BlockHelper,
		blockType: keyof typeof BLOCK_TYPES
	): Promise<number> {
		const { duration } = await this.measureOperation(async () => {
			await blockHelper.addBlock(blockType);
		});

		return duration;
	}
}

// =============================================================================
// Login Helper
// =============================================================================

export async function loginAsAdmin(page: Page): Promise<void> {
	await loginUser(page, TEST_USERS.admin);
}

export async function loginAsMember(page: Page): Promise<void> {
	await loginUser(page, TEST_USERS.member);
}

export async function loginAsEditor(page: Page): Promise<void> {
	await loginUser(page, TEST_USERS.editor);
}

async function loginUser(page: Page, user: TestUser): Promise<void> {
	await page.goto('/login');
	await page.waitForLoadState('domcontentloaded');

	// Fill email
	const emailInput = page
		.locator(
			['input[type="email"]', 'input[name="email"]', 'input[placeholder*="email" i]'].join(', ')
		)
		.first();

	await emailInput.fill(user.email);

	// Fill password
	const passwordInput = page.locator('input[type="password"]').first();
	await passwordInput.fill(user.password);

	// Submit
	const submitBtn = page
		.locator(
			['button[type="submit"]', 'button:has-text("Login")', 'button:has-text("Sign in")'].join(', ')
		)
		.first();

	await submitBtn.click();

	// Wait for navigation
	await page.waitForURL(/\/(dashboard|admin|home)/, { timeout: 15000 }).catch(() => {});
	await page.waitForLoadState('networkidle').catch(() => {});
}

// =============================================================================
// Create Post Helper
// =============================================================================

export async function createTestPost(
	page: Page,
	options: {
		title: string;
		slug?: string;
		status?: 'draft' | 'published';
		blocks?: BlockType[];
	}
): Promise<number> {
	const editorHelper = new EditorHelper(page);
	const blockHelper = new BlockHelper(page);

	await editorHelper.navigateToCreate();
	await editorHelper.setTitle(options.title);

	if (options.slug) {
		await editorHelper.setSlug(options.slug);
	}

	// Add blocks if specified
	if (options.blocks) {
		for (const block of options.blocks) {
			await blockHelper.addBlock(block.type as keyof typeof BLOCK_TYPES);
		}
	}

	if (options.status === 'published') {
		await editorHelper.publish();
	} else {
		await editorHelper.saveDraft();
	}

	// Extract post ID from URL or response
	const url = page.url();
	const match = url.match(/\/edit\/(\d+)/);
	return match ? parseInt(match[1], 10) : Date.now();
}

// =============================================================================
// Extended Test Fixture
// =============================================================================

export const test = base.extend<EditorFixtures>({
	adminPage: async ({ page }, use) => {
		await loginAsAdmin(page);
		await use(page);
	},

	memberPage: async ({ page }, use) => {
		await loginAsMember(page);
		await use(page);
	},

	editorPage: async ({ page }, use) => {
		await loginAsEditor(page);
		await use(page);
	},

	mockApi: async ({ page }, use) => {
		const mockApi = new MockApiHelper(page);
		await mockApi.setupBlogMocks();
		await use(mockApi);
	},

	blockHelper: async ({ page }, use) => {
		const blockHelper = new BlockHelper(page);
		await use(blockHelper);
	},

	editorHelper: async ({ page }, use) => {
		const editorHelper = new EditorHelper(page);
		await use(editorHelper);
	},

	a11yHelper: async ({ page }, use) => {
		const a11yHelper = new AccessibilityHelper(page);
		await use(a11yHelper);
	},

	perfHelper: async ({ page }, use) => {
		const perfHelper = new PerformanceHelper(page);
		await use(perfHelper);
	}
});

export { expect } from '@playwright/test';

// =============================================================================
// Test Data Generators
// =============================================================================

export function generateTestPost(overrides: Partial<MockPost> = {}): MockPost {
	const id = Date.now();
	return {
		id,
		title: `Test Post ${id}`,
		slug: `test-post-${id}`,
		status: 'draft',
		blocks: [
			{
				id: `block_${id}_1`,
				type: 'paragraph',
				content: { text: 'Test content paragraph.' },
				settings: {}
			}
		],
		created_at: new Date().toISOString(),
		updated_at: new Date().toISOString(),
		...overrides
	};
}

export function generateTestBlocks(count: number): BlockType[] {
	return Array.from({ length: count }, (_, i) => ({
		id: `block_${Date.now()}_${i}`,
		type: 'paragraph',
		content: { text: `Paragraph ${i + 1} content.` },
		settings: {}
	}));
}

// =============================================================================
// Screenshot Helpers
// =============================================================================

export async function takeScreenshot(page: Page, name: string): Promise<Buffer> {
	return await page.screenshot({
		path: `test-results/screenshots/${name}.png`,
		fullPage: true
	});
}

export async function compareScreenshot(page: Page, name: string): Promise<void> {
	await expect(page).toHaveScreenshot(`${name}.png`, {
		maxDiffPixels: 100,
		threshold: 0.2
	});
}

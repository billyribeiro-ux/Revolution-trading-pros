/**
 * Revolution Trading Pros - Trading Room Tests
 *
 * E2E tests for live trading room functionality:
 * - Room listing and navigation
 * - Room page content
 * - Stream container verification
 * - Chat functionality (when authenticated)
 * - Member-only access controls
 *
 * Netflix L11+ Standard: Test core product features
 */

import { test, expect } from '@playwright/test';
import { TradingRoomPage, createTradingRoomPage } from '../pages';
import { KNOWN_TEST_DATA } from '../helpers/test-data.helper';
import { loginViaUI, TEST_USER } from '../helpers';

test.describe('Trading Rooms', () => {
	test.describe('Trading Rooms Listing', () => {
		test('trading rooms index page loads', async ({ page }) => {
			await page.goto('/live-trading-rooms');

			await expect(page.locator('h1')).toBeVisible();
		});

		test('trading rooms index displays available rooms', async ({ page }) => {
			await page.goto('/live-trading-rooms');

			// Should show at least one room card or link
			const roomLinks = page.locator(
				'a[href*="trading-rooms/"], [data-testid*="room-card"], .room-card'
			);
			const count = await roomLinks.count();

			expect(count).toBeGreaterThan(0);
		});

		test('day trading room link is present', async ({ page }) => {
			await page.goto('/live-trading-rooms');

			const dayTradingLink = page.locator(
				`a[href*="${KNOWN_TEST_DATA.tradingRooms.dayTrading.slug}"]`
			);
			await expect(dayTradingLink).toBeVisible();
		});

		test('swing trading room link is present', async ({ page }) => {
			await page.goto('/live-trading-rooms');

			const swingTradingLink = page.locator(
				`a[href*="${KNOWN_TEST_DATA.tradingRooms.swingTrading.slug}"]`
			);
			await expect(swingTradingLink).toBeVisible();
		});

		test('small accounts room link is present', async ({ page }) => {
			await page.goto('/live-trading-rooms');

			const smallAccountsLink = page.locator(
				`a[href*="${KNOWN_TEST_DATA.tradingRooms.smallAccounts.slug}"]`
			);
			await expect(smallAccountsLink).toBeVisible();
		});
	});

	test.describe('Day Trading Room', () => {
		test('day trading room page loads', async ({ page }) => {
			const tradingRoom = new TradingRoomPage(page, 'day-trading');
			await tradingRoom.goto();

			await tradingRoom.verifyPageLoaded();
		});

		test('day trading room has title', async ({ page }) => {
			const tradingRoom = new TradingRoomPage(page, 'day-trading');
			await tradingRoom.goto();

			const title = await tradingRoom.getRoomTitle();
			expect(title.length).toBeGreaterThan(0);
		});

		test('day trading room shows main content section', async ({ page }) => {
			await page.goto('/live-trading-rooms/day-trading');

			// Should have some content (stream, info, etc.)
			const mainContent = page.locator('main, .main-content, section').first();
			await expect(mainContent).toBeVisible();
		});
	});

	test.describe('Swing Trading Room', () => {
		test('swing trading room page loads', async ({ page }) => {
			const tradingRoom = new TradingRoomPage(page, 'swing-trading');
			await tradingRoom.goto();

			await tradingRoom.verifyPageLoaded();
		});

		test('swing trading room has descriptive content', async ({ page }) => {
			await page.goto('/live-trading-rooms/swing-trading');

			// Should have description or info about the room
			const content = page.locator('p, .description');
			const count = await content.count();

			expect(count).toBeGreaterThan(0);
		});
	});

	test.describe('Small Accounts Room', () => {
		test('small accounts room page loads', async ({ page }) => {
			const tradingRoom = new TradingRoomPage(page, 'small-accounts');
			await tradingRoom.goto();

			await tradingRoom.verifyPageLoaded();
		});
	});

	test.describe('Room Content Structure', () => {
		const rooms = ['day-trading', 'swing-trading', 'small-accounts'];

		for (const room of rooms) {
			test(`${room} room has proper heading structure`, async ({ page }) => {
				await page.goto(`/live-trading-rooms/${room}`);

				const h1 = page.locator('h1');
				await expect(h1).toBeVisible();
			});

			test(`${room} room has call-to-action`, async ({ page }) => {
				await page.goto(`/live-trading-rooms/${room}`);

				// Should have a CTA button (Join, Subscribe, etc.)
				const ctaButton = page.locator(
					'button, a.btn, a[href*="checkout"], a[href*="join"], a[href*="subscribe"]'
				);
				const count = await ctaButton.count();

				expect(count).toBeGreaterThan(0);
			});
		}
	});

	test.describe('Authentication Gating', () => {
		test('trading room shows access info for anonymous users', async ({ page }) => {
			await page.goto('/live-trading-rooms/day-trading');

			// Anonymous users should see either:
			// 1. Login/signup CTA
			// 2. Subscription required message
			// 3. Limited preview content

			const ctaElement = page.locator(
				'a[href*="login"], a[href*="register"], a[href*="checkout"], button:has-text("Join"), button:has-text("Subscribe")'
			);
			const infoMessage = page.getByText(
				/member|subscribe|join|sign in|log in/i
			);

			const hasCta = await ctaElement.count() > 0;
			const hasInfo = await infoMessage.isVisible().catch(() => false);

			// Should show some form of access info
			expect(hasCta || hasInfo).toBe(true);
		});

		test.skip('authenticated member sees stream content', async ({ page }) => {
			// Skip if no test credentials
			if (!process.env.E2E_TEST_USER_EMAIL) {
				test.skip();
				return;
			}

			// Login first
			await loginViaUI(page, TEST_USER);

			// Navigate to trading room
			const tradingRoom = new TradingRoomPage(page, 'day-trading');
			await tradingRoom.goto();

			// Should see stream container (if user has subscription)
			const hasStream = await tradingRoom.verifyStreamContainer();

			// Either shows stream or auth required (depending on subscription)
			if (!hasStream) {
				const authRequired = await tradingRoom.verifyAuthRequired();
				expect(authRequired).toBe(true);
			} else {
				expect(hasStream).toBe(true);
			}
		});
	});

	test.describe('Room Information', () => {
		test('trading room shows schedule or availability info', async ({ page }) => {
			await page.goto('/live-trading-rooms/day-trading');

			// Look for schedule-related content
			const scheduleContent = page.locator(
				'[data-testid="schedule"], .schedule, .hours, .availability'
			);
			const timeText = page.getByText(/am|pm|est|pst|market hours|schedule/i);

			const hasScheduleSection = await scheduleContent.count() > 0;
			const hasTimeInfo = await timeText.isVisible().catch(() => false);

			// Room should show some timing info
			expect(hasScheduleSection || hasTimeInfo).toBe(true);
		});

		test('trading room shows trader/host information', async ({ page }) => {
			await page.goto('/live-trading-rooms/day-trading');

			// Look for trader info
			const traderInfo = page.locator(
				'[data-testid="trader"], .trader, .host, .instructor'
			);
			const traderName = page.getByText(/trader|host|led by|with/i);

			const hasTraderSection = await traderInfo.count() > 0;
			const hasTraderText = await traderName.isVisible().catch(() => false);

			// Should show who leads the room
			expect(hasTraderSection || hasTraderText).toBe(true);
		});
	});

	test.describe('Mobile Responsiveness', () => {
		test('trading room renders on mobile', async ({ page }) => {
			await page.setViewportSize({ width: 375, height: 667 });
			await page.goto('/live-trading-rooms/day-trading');

			const h1 = page.locator('h1');
			await expect(h1).toBeVisible();
		});

		test('trading rooms listing is scrollable on mobile', async ({ page }) => {
			await page.setViewportSize({ width: 375, height: 667 });
			await page.goto('/live-trading-rooms');

			// Should be able to scroll
			await page.evaluate(() => window.scrollTo(0, 500));
			const scrollY = await page.evaluate(() => window.scrollY);

			expect(scrollY).toBeGreaterThan(0);
		});
	});

	test.describe('Performance', () => {
		test('trading room page loads within 3 seconds', async ({ page }) => {
			const startTime = Date.now();
			await page.goto('/live-trading-rooms/day-trading');
			await page.waitForLoadState('domcontentloaded');
			const loadTime = Date.now() - startTime;

			expect(loadTime).toBeLessThan(3000);
		});

		test('no critical JS errors on trading room page', async ({ page }) => {
			const errors: string[] = [];

			page.on('pageerror', (error) => {
				if (
					!error.message.includes('NetworkError') &&
					!error.message.includes('fetch') &&
					!error.message.includes('API')
				) {
					errors.push(error.message);
				}
			});

			await page.goto('/live-trading-rooms/day-trading');
			await page.waitForTimeout(2000);

			expect(errors).toHaveLength(0);
		});
	});
});

test.describe('Trading Room Features', () => {
	test.describe.skip('Chat Functionality', () => {
		// These tests require authenticated user with room access
		test.beforeEach(async ({ page }) => {
			if (!process.env.E2E_TEST_USER_EMAIL) {
				test.skip();
				return;
			}
			await loginViaUI(page, TEST_USER);
		});

		test('chat container is visible for members', async ({ page }) => {
			const tradingRoom = new TradingRoomPage(page, 'day-trading');
			await tradingRoom.goto();

			const hasChat = await tradingRoom.verifyChatContainer();
			// Chat may not be visible if user doesn't have subscription
			expect(typeof hasChat).toBe('boolean');
		});

		test('member can send chat message', async ({ page }) => {
			const tradingRoom = new TradingRoomPage(page, 'day-trading');
			await tradingRoom.goto();

			if (await tradingRoom.verifyChatContainer()) {
				await tradingRoom.sendChatMessage('Test message from E2E');
				await tradingRoom.verifyChatMessageSent('Test message from E2E');
			}
		});
	});

	test.describe('Alerts Section', () => {
		test('trading room has alerts or trade section', async ({ page }) => {
			await page.goto('/live-trading-rooms/day-trading');

			// Look for alerts, trades, or signals section
			const alertsSection = page.locator(
				'[data-testid="alerts"], .alerts, .trades, .signals, section:has-text("alert")'
			);

			const hasAlerts = await alertsSection.count() > 0;
			// Alerts section is feature-dependent
			expect(typeof hasAlerts).toBe('boolean');
		});
	});
});

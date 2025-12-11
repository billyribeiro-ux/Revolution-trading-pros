/**
 * Revolution Trading Pros - Trading Room Page Object
 *
 * Page object for live trading room pages with:
 * - Stream container verification
 * - Chat interactions
 * - Alert section checks
 * - Member list verification
 */

import { type Page, type Locator, expect } from '@playwright/test';
import { BasePage } from './base.page';

export class TradingRoomPage extends BasePage {
	// Main sections
	readonly pageTitle: Locator;
	readonly streamContainer: Locator;
	readonly chatContainer: Locator;
	readonly alertsSection: Locator;
	readonly membersList: Locator;

	// Chat elements
	readonly chatInput: Locator;
	readonly chatSendButton: Locator;
	readonly chatMessages: Locator;

	// Controls
	readonly joinButton: Locator;
	readonly leaveButton: Locator;
	readonly muteButton: Locator;
	readonly fullscreenButton: Locator;

	// Info sections
	readonly traderInfo: Locator;
	readonly scheduleInfo: Locator;

	// Dynamic room slug
	private roomSlug: string;

	constructor(page: Page, roomSlug: string = 'day-trading') {
		super(page);
		this.roomSlug = roomSlug;

		// Main sections
		this.pageTitle = page.locator('h1').first();
		this.streamContainer = page.locator(
			'[data-testid="stream-container"], .stream-container, .video-container, iframe, video'
		).first();
		this.chatContainer = page.locator(
			'[data-testid="chat-container"], .chat-container, .chat-panel'
		);
		this.alertsSection = page.locator(
			'[data-testid="alerts"], .alerts-section, .trade-alerts'
		);
		this.membersList = page.locator(
			'[data-testid="members-list"], .members-list, .participants'
		);

		// Chat elements
		this.chatInput = page.locator(
			'[data-testid="chat-input"], .chat-input input, .chat-input textarea'
		);
		this.chatSendButton = page.locator(
			'[data-testid="chat-send"], .chat-send, button:has-text("Send")'
		);
		this.chatMessages = page.locator(
			'[data-testid="chat-message"], .chat-message, .message'
		);

		// Controls
		this.joinButton = page.locator(
			'button:has-text("Join"), [data-testid="join-room"]'
		);
		this.leaveButton = page.locator(
			'button:has-text("Leave"), [data-testid="leave-room"]'
		);
		this.muteButton = page.locator(
			'button[aria-label*="mute"], [data-testid="mute"]'
		);
		this.fullscreenButton = page.locator(
			'button[aria-label*="fullscreen"], [data-testid="fullscreen"]'
		);

		// Info
		this.traderInfo = page.locator(
			'[data-testid="trader-info"], .trader-info, .host-info'
		);
		this.scheduleInfo = page.locator(
			'[data-testid="schedule"], .schedule-info, .room-schedule'
		);
	}

	get path(): string {
		return `/live-trading-rooms/${this.roomSlug}`;
	}

	/**
	 * Sets the room slug and updates path
	 */
	setRoom(slug: string): void {
		this.roomSlug = slug;
	}

	/**
	 * Navigates to a specific trading room
	 */
	async gotoRoom(slug: string): Promise<void> {
		this.roomSlug = slug;
		await this.goto();
	}

	/**
	 * Verifies the page loaded with expected content
	 */
	async verifyPageLoaded(): Promise<void> {
		await expect(this.pageTitle).toBeVisible();
		// Check that main content area exists
		await expect(this.mainContent).toBeVisible();
	}

	/**
	 * Verifies the stream/video container is present
	 */
	async verifyStreamContainer(): Promise<boolean> {
		try {
			await this.streamContainer.waitFor({ state: 'visible', timeout: 10000 });
			return true;
		} catch {
			return false;
		}
	}

	/**
	 * Verifies the chat container is present
	 */
	async verifyChatContainer(): Promise<boolean> {
		try {
			await this.chatContainer.waitFor({ state: 'visible', timeout: 5000 });
			return true;
		} catch {
			return false;
		}
	}

	/**
	 * Sends a chat message
	 */
	async sendChatMessage(message: string): Promise<void> {
		await this.chatInput.fill(message);
		await this.chatSendButton.click();
	}

	/**
	 * Gets the last chat message
	 */
	async getLastChatMessage(): Promise<string | null> {
		try {
			const lastMessage = this.chatMessages.last();
			await lastMessage.waitFor({ state: 'visible', timeout: 5000 });
			return await lastMessage.textContent();
		} catch {
			return null;
		}
	}

	/**
	 * Verifies a message appears in chat
	 */
	async verifyChatMessageSent(message: string): Promise<void> {
		await expect(this.chatMessages.filter({ hasText: message })).toBeVisible({
			timeout: 10000
		});
	}

	/**
	 * Gets the count of chat messages
	 */
	async getChatMessageCount(): Promise<number> {
		return await this.chatMessages.count();
	}

	/**
	 * Clicks the join room button
	 */
	async joinRoom(): Promise<void> {
		if (await this.joinButton.isVisible()) {
			await this.joinButton.click();
		}
	}

	/**
	 * Clicks the leave room button
	 */
	async leaveRoom(): Promise<void> {
		if (await this.leaveButton.isVisible()) {
			await this.leaveButton.click();
		}
	}

	/**
	 * Verifies alerts section shows content
	 */
	async verifyAlertsSection(): Promise<boolean> {
		try {
			await this.alertsSection.waitFor({ state: 'visible', timeout: 5000 });
			return true;
		} catch {
			return false;
		}
	}

	/**
	 * Gets the room title
	 */
	async getRoomTitle(): Promise<string> {
		return (await this.pageTitle.textContent()) || '';
	}

	/**
	 * Verifies trader/host information is displayed
	 */
	async verifyTraderInfo(): Promise<boolean> {
		try {
			await this.traderInfo.waitFor({ state: 'visible', timeout: 5000 });
			return true;
		} catch {
			return false;
		}
	}

	/**
	 * Toggles fullscreen mode
	 */
	async toggleFullscreen(): Promise<void> {
		if (await this.fullscreenButton.isVisible()) {
			await this.fullscreenButton.click();
		}
	}

	/**
	 * Checks if the stream is playing (has video content)
	 */
	async isStreamPlaying(): Promise<boolean> {
		try {
			// Check for video element with valid duration
			const video = this.page.locator('video');
			if (await video.isVisible()) {
				const duration = await video.evaluate((el: HTMLVideoElement) => el.duration);
				return duration > 0;
			}

			// Or check for iframe (embedded stream)
			const iframe = this.page.locator('iframe[src*="vimeo"], iframe[src*="youtube"]');
			return await iframe.isVisible();
		} catch {
			return false;
		}
	}

	/**
	 * Verifies the room requires authentication
	 */
	async verifyAuthRequired(): Promise<boolean> {
		// Check if redirected to login or shows auth message
		const url = this.page.url();
		const hasAuthMessage = await this.page
			.getByText(/sign in|log in|members only|subscribe/i)
			.isVisible()
			.catch(() => false);

		return url.includes('/login') || hasAuthMessage;
	}
}

/**
 * Factory for creating TradingRoomPage instances
 */
export function createTradingRoomPage(page: Page, room: 'day-trading' | 'swing-trading' | 'small-accounts'): TradingRoomPage {
	return new TradingRoomPage(page, room);
}

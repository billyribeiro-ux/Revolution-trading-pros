/**
 * Email Marketing Store (Svelte 5 Runes)
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Centralized state management for email campaigns, sequences, and automations.
 *
 * @version 2.0.0 - Svelte 5 Runes Migration
 */

import { emailApi } from '$lib/api/email';
import type {
	EmailCampaign,
	EmailTemplate,
	EmailSequence,
	EmailAutomation,
	EmailSegment,
	EmailSubscriber,
	EmailAnalytics
} from '$lib/api/email';

// ═══════════════════════════════════════════════════════════════════════════
// Store State
// ═══════════════════════════════════════════════════════════════════════════

interface EmailState {
	campaigns: EmailCampaign[];
	templates: EmailTemplate[];
	sequences: EmailSequence[];
	automations: EmailAutomation[];
	segments: EmailSegment[];
	subscribers: EmailSubscriber[];
	analytics: EmailAnalytics | null;
	currentCampaign: EmailCampaign | null;
	currentTemplate: EmailTemplate | null;
	currentSequence: EmailSequence | null;
	currentAutomation: EmailAutomation | null;
	isLoading: boolean;
	error: string | null;
	pagination: {
		page: number;
		per_page: number;
		total: number;
	};
}

const initialState: EmailState = {
	campaigns: [],
	templates: [],
	sequences: [],
	automations: [],
	segments: [],
	subscribers: [],
	analytics: null,
	currentCampaign: null,
	currentTemplate: null,
	currentSequence: null,
	currentAutomation: null,
	isLoading: false,
	error: null,
	pagination: {
		page: 1,
		per_page: 20,
		total: 0
	}
};

// ═══════════════════════════════════════════════════════════════════════════
// State (Svelte 5 Runes)
// ═══════════════════════════════════════════════════════════════════════════

let emailState = $state<EmailState>({ ...initialState });

// ═══════════════════════════════════════════════════════════════════════════
// Email Store API
// ═══════════════════════════════════════════════════════════════════════════

export const emailStore = {
	get state() {
		return emailState;
	},

	get campaigns() {
		return emailState.campaigns;
	},

	get templates() {
		return emailState.templates;
	},

	get sequences() {
		return emailState.sequences;
	},

	get automations() {
		return emailState.automations;
	},

	get isLoading() {
		return emailState.isLoading;
	},

	get error() {
		return emailState.error;
	},

	// ═══════════════════════════════════════════════════════════════════════
	// Campaign Methods
	// ═══════════════════════════════════════════════════════════════════════

	async loadCampaigns(params?: { status?: string; type?: string; page?: number }) {
		emailState = { ...emailState, isLoading: true, error: null };

		try {
			const response = await emailApi.getCampaigns({
				...params,
				per_page: emailState.pagination.per_page
			});

			emailState = {
				...emailState,
				campaigns: response.campaigns,
				pagination: {
					page: response.page,
					per_page: response.per_page,
					total: response.total
				},
				isLoading: false
			};
		} catch (error: any) {
			emailState = {
				...emailState,
				error: error.message,
				isLoading: false
			};
		}
	},

	async loadCampaign(id: string) {
		emailState = { ...emailState, isLoading: true, error: null };

		try {
			const { campaign } = await emailApi.getCampaign(id);

			emailState = {
				...emailState,
				currentCampaign: campaign,
				isLoading: false
			};
		} catch (error: any) {
			emailState = {
				...emailState,
				error: error.message,
				isLoading: false
			};
		}
	},

	async createCampaign(data: Partial<EmailCampaign>) {
		emailState = { ...emailState, isLoading: true, error: null };

		try {
			const { campaign } = await emailApi.createCampaign(data);

			emailState = {
				...emailState,
				campaigns: [campaign, ...emailState.campaigns],
				currentCampaign: campaign,
				isLoading: false
			};

			return campaign;
		} catch (error: any) {
			emailState = {
				...emailState,
				error: error.message,
				isLoading: false
			};
			throw error;
		}
	},

	async updateCampaign(id: string, data: Partial<EmailCampaign>) {
		emailState = { ...emailState, isLoading: true, error: null };

		try {
			const { campaign } = await emailApi.updateCampaign(id, data);

			emailState = {
				...emailState,
				campaigns: emailState.campaigns.map((c) => (c.id === id ? campaign : c)),
				currentCampaign: campaign,
				isLoading: false
			};

			return campaign;
		} catch (error: any) {
			emailState = {
				...emailState,
				error: error.message,
				isLoading: false
			};
			throw error;
		}
	},

	async deleteCampaign(id: string) {
		emailState = { ...emailState, isLoading: true, error: null };

		try {
			await emailApi.deleteCampaign(id);

			emailState = {
				...emailState,
				campaigns: emailState.campaigns.filter((c) => c.id !== id),
				currentCampaign: emailState.currentCampaign?.id === id ? null : emailState.currentCampaign,
				isLoading: false
			};
		} catch (error: any) {
			emailState = {
				...emailState,
				error: error.message,
				isLoading: false
			};
			throw error;
		}
	},

	async sendCampaign(id: string, sendAt?: string) {
		emailState = { ...emailState, isLoading: true, error: null };

		try {
			await emailApi.sendCampaign(id, sendAt ? { send_at: sendAt } : undefined);

			// Reload campaign to get updated status
			const { campaign } = await emailApi.getCampaign(id);

			emailState = {
				...emailState,
				campaigns: emailState.campaigns.map((c) => (c.id === id ? campaign : c)),
				currentCampaign: campaign,
				isLoading: false
			};
		} catch (error: any) {
			emailState = {
				...emailState,
				error: error.message,
				isLoading: false
			};
			throw error;
		}
	},

	async sendTestEmail(id: string, emails: string[]) {
		emailState = { ...emailState, isLoading: true, error: null };

		try {
			await emailApi.sendTestEmail(id, emails);
			emailState = { ...emailState, isLoading: false };
		} catch (error: any) {
			emailState = {
				...emailState,
				error: error.message,
				isLoading: false
			};
			throw error;
		}
	},

	// ═══════════════════════════════════════════════════════════════════════
	// Template Methods
	// ═══════════════════════════════════════════════════════════════════════

	async loadTemplates(params?: { category?: string; page?: number }) {
		emailState = { ...emailState, isLoading: true, error: null };

		try {
			const response = await emailApi.getTemplates({
				...params,
				per_page: emailState.pagination.per_page
			});

			emailState = {
				...emailState,
				templates: response.templates,
				isLoading: false
			};
		} catch (error: any) {
			emailState = {
				...emailState,
				error: error.message,
				isLoading: false
			};
		}
	},

	async loadTemplate(id: string) {
		emailState = { ...emailState, isLoading: true, error: null };

		try {
			const { template } = await emailApi.getTemplate(id);

			emailState = {
				...emailState,
				currentTemplate: template,
				isLoading: false
			};
		} catch (error: any) {
			emailState = {
				...emailState,
				error: error.message,
				isLoading: false
			};
		}
	},

	async createTemplate(data: Partial<EmailTemplate>) {
		emailState = { ...emailState, isLoading: true, error: null };

		try {
			const { template } = await emailApi.createTemplate(data);

			emailState = {
				...emailState,
				templates: [template, ...emailState.templates],
				currentTemplate: template,
				isLoading: false
			};

			return template;
		} catch (error: any) {
			emailState = {
				...emailState,
				error: error.message,
				isLoading: false
			};
			throw error;
		}
	},

	async updateTemplate(id: string, data: Partial<EmailTemplate>) {
		emailState = { ...emailState, isLoading: true, error: null };

		try {
			const { template } = await emailApi.updateTemplate(id, data);

			emailState = {
				...emailState,
				templates: emailState.templates.map((t) => (t.id === id ? template : t)),
				currentTemplate: template,
				isLoading: false
			};

			return template;
		} catch (error: any) {
			emailState = {
				...emailState,
				error: error.message,
				isLoading: false
			};
			throw error;
		}
	},

	async deleteTemplate(id: string) {
		emailState = { ...emailState, isLoading: true, error: null };

		try {
			await emailApi.deleteTemplate(id);

			emailState = {
				...emailState,
				templates: emailState.templates.filter((t) => t.id !== id),
				currentTemplate: emailState.currentTemplate?.id === id ? null : emailState.currentTemplate,
				isLoading: false
			};
		} catch (error: any) {
			emailState = {
				...emailState,
				error: error.message,
				isLoading: false
			};
			throw error;
		}
	},

	// ═══════════════════════════════════════════════════════════════════════
	// Sequence Methods
	// ═══════════════════════════════════════════════════════════════════════

	async loadSequences(params?: { status?: string; page?: number }) {
		emailState = { ...emailState, isLoading: true, error: null };

		try {
			const response = await emailApi.getSequences({
				...params,
				per_page: emailState.pagination.per_page
			});

			emailState = {
				...emailState,
				sequences: response.sequences,
				isLoading: false
			};
		} catch (error: any) {
			emailState = {
				...emailState,
				error: error.message,
				isLoading: false
			};
		}
	},

	async loadSequence(id: string) {
		emailState = { ...emailState, isLoading: true, error: null };

		try {
			const { sequence } = await emailApi.getSequence(id);

			emailState = {
				...emailState,
				currentSequence: sequence,
				isLoading: false
			};
		} catch (error: any) {
			emailState = {
				...emailState,
				error: error.message,
				isLoading: false
			};
		}
	},

	async createSequence(data: Partial<EmailSequence>) {
		emailState = { ...emailState, isLoading: true, error: null };

		try {
			const { sequence } = await emailApi.createSequence(data);

			emailState = {
				...emailState,
				sequences: [sequence, ...emailState.sequences],
				currentSequence: sequence,
				isLoading: false
			};

			return sequence;
		} catch (error: any) {
			emailState = {
				...emailState,
				error: error.message,
				isLoading: false
			};
			throw error;
		}
	},

	async updateSequence(id: string, data: Partial<EmailSequence>) {
		emailState = { ...emailState, isLoading: true, error: null };

		try {
			const { sequence } = await emailApi.updateSequence(id, data);

			emailState = {
				...emailState,
				sequences: emailState.sequences.map((s) => (s.id === id ? sequence : s)),
				currentSequence: sequence,
				isLoading: false
			};

			return sequence;
		} catch (error: any) {
			emailState = {
				...emailState,
				error: error.message,
				isLoading: false
			};
			throw error;
		}
	},

	async deleteSequence(id: string) {
		emailState = { ...emailState, isLoading: true, error: null };

		try {
			await emailApi.deleteSequence(id);

			emailState = {
				...emailState,
				sequences: emailState.sequences.filter((s) => s.id !== id),
				currentSequence: emailState.currentSequence?.id === id ? null : emailState.currentSequence,
				isLoading: false
			};
		} catch (error: any) {
			emailState = {
				...emailState,
				error: error.message,
				isLoading: false
			};
			throw error;
		}
	},

	// ═══════════════════════════════════════════════════════════════════════
	// Automation Methods
	// ═══════════════════════════════════════════════════════════════════════

	async loadAutomations(params?: { status?: string; page?: number }) {
		emailState = { ...emailState, isLoading: true, error: null };

		try {
			const response = await emailApi.getAutomations({
				...params,
				per_page: emailState.pagination.per_page
			});

			emailState = {
				...emailState,
				automations: response.automations,
				isLoading: false
			};
		} catch (error: any) {
			emailState = {
				...emailState,
				error: error.message,
				isLoading: false
			};
		}
	},

	async loadAutomation(id: string) {
		emailState = { ...emailState, isLoading: true, error: null };

		try {
			const { automation } = await emailApi.getAutomation(id);

			emailState = {
				...emailState,
				currentAutomation: automation,
				isLoading: false
			};
		} catch (error: any) {
			emailState = {
				...emailState,
				error: error.message,
				isLoading: false
			};
		}
	},

	async createAutomation(data: Partial<EmailAutomation>) {
		emailState = { ...emailState, isLoading: true, error: null };

		try {
			const { automation } = await emailApi.createAutomation(data);

			emailState = {
				...emailState,
				automations: [automation, ...emailState.automations],
				currentAutomation: automation,
				isLoading: false
			};

			return automation;
		} catch (error: any) {
			emailState = {
				...emailState,
				error: error.message,
				isLoading: false
			};
			throw error;
		}
	},

	async updateAutomation(id: string, data: Partial<EmailAutomation>) {
		emailState = { ...emailState, isLoading: true, error: null };

		try {
			const { automation } = await emailApi.updateAutomation(id, data);

			emailState = {
				...emailState,
				automations: emailState.automations.map((a) => (a.id === id ? automation : a)),
				currentAutomation: automation,
				isLoading: false
			};

			return automation;
		} catch (error: any) {
			emailState = {
				...emailState,
				error: error.message,
				isLoading: false
			};
			throw error;
		}
	},

	async deleteAutomation(id: string) {
		emailState = { ...emailState, isLoading: true, error: null };

		try {
			await emailApi.deleteAutomation(id);

			emailState = {
				...emailState,
				automations: emailState.automations.filter((a) => a.id !== id),
				currentAutomation: emailState.currentAutomation?.id === id ? null : emailState.currentAutomation,
				isLoading: false
			};
		} catch (error: any) {
			emailState = {
				...emailState,
				error: error.message,
				isLoading: false
			};
			throw error;
		}
	},

	// ═══════════════════════════════════════════════════════════════════════
	// Segment Methods
	// ═══════════════════════════════════════════════════════════════════════

	async loadSegments() {
		emailState = { ...emailState, isLoading: true, error: null };

		try {
			const response = await emailApi.getSegments();

			emailState = {
				...emailState,
				segments: response.segments,
				isLoading: false
			};
		} catch (error: any) {
			emailState = {
				...emailState,
				error: error.message,
				isLoading: false
			};
		}
	},

	async createSegment(data: Partial<EmailSegment>) {
		emailState = { ...emailState, isLoading: true, error: null };

		try {
			const { segment } = await emailApi.createSegment(data);

			emailState = {
				...emailState,
				segments: [segment, ...emailState.segments],
				isLoading: false
			};

			return segment;
		} catch (error: any) {
			emailState = {
				...emailState,
				error: error.message,
				isLoading: false
			};
			throw error;
		}
	},

	// ═══════════════════════════════════════════════════════════════════════
	// Subscriber Methods
	// ═══════════════════════════════════════════════════════════════════════

	async loadSubscribers(params?: { status?: string; search?: string; page?: number }) {
		emailState = { ...emailState, isLoading: true, error: null };

		try {
			const response = await emailApi.getSubscribers({
				...params,
				per_page: emailState.pagination.per_page
			});

			emailState = {
				...emailState,
				subscribers: response.subscribers,
				pagination: {
					...emailState.pagination,
					total: response.total
				},
				isLoading: false
			};
		} catch (error: any) {
			emailState = {
				...emailState,
				error: error.message,
				isLoading: false
			};
		}
	},

	// ═══════════════════════════════════════════════════════════════════════
	// Analytics Methods
	// ═══════════════════════════════════════════════════════════════════════

	async loadAnalytics(period: string = '30d') {
		emailState = { ...emailState, isLoading: true, error: null };

		try {
			const { analytics } = await emailApi.getAnalytics(period);

			emailState = {
				...emailState,
				analytics,
				isLoading: false
			};
		} catch (error: any) {
			emailState = {
				...emailState,
				error: error.message,
				isLoading: false
			};
		}
	},

	// ═══════════════════════════════════════════════════════════════════════
	// Utility Methods
	// ═══════════════════════════════════════════════════════════════════════

	clearError() {
		emailState = { ...emailState, error: null };
	},

	reset() {
		emailState = { ...initialState };
	}
};

// ═══════════════════════════════════════════════════════════════════════════
// Derived Values (Svelte 5 Runes)
// ═══════════════════════════════════════════════════════════════════════════

export const activeCampaigns = $derived(
	emailState.campaigns.filter((c) => c.status === 'sending' || c.status === 'sent')
);

export const draftCampaigns = $derived(
	emailState.campaigns.filter((c) => c.status === 'draft')
);

export const activeSequences = $derived(
	emailState.sequences.filter((s) => s.status === 'active')
);

export const activeAutomations = $derived(
	emailState.automations.filter((a) => a.status === 'active')
);

export const isEmailLoading = $derived(emailState.isLoading);

export const hasEmailError = $derived(emailState.error !== null);

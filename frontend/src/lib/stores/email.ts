/**
 * Email Marketing Store
 * ═══════════════════════════════════════════════════════════════════════════
 * 
 * Centralized state management for email campaigns, sequences, and automations.
 */

import { writable, derived, get } from 'svelte/store';
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
// Main Store
// ═══════════════════════════════════════════════════════════════════════════

function createEmailStore() {
	const { subscribe, set, update } = writable<EmailState>(initialState);

	return {
		subscribe,
		
		// ═══════════════════════════════════════════════════════════════════════
		// Campaign Methods
		// ═══════════════════════════════════════════════════════════════════════

		async loadCampaigns(params?: { status?: string; type?: string; page?: number }) {
			update(state => ({ ...state, isLoading: true, error: null }));
			
			try {
				const response = await emailApi.getCampaigns({
					...params,
					per_page: get({ subscribe }).pagination.per_page
				});
				
				update(state => ({
					...state,
					campaigns: response.campaigns,
					pagination: {
						page: response.page,
						per_page: response.per_page,
						total: response.total
					},
					isLoading: false
				}));
			} catch (error: any) {
				update(state => ({
					...state,
					error: error.message,
					isLoading: false
				}));
			}
		},

		async loadCampaign(id: string) {
			update(state => ({ ...state, isLoading: true, error: null }));
			
			try {
				const { campaign } = await emailApi.getCampaign(id);
				
				update(state => ({
					...state,
					currentCampaign: campaign,
					isLoading: false
				}));
			} catch (error: any) {
				update(state => ({
					...state,
					error: error.message,
					isLoading: false
				}));
			}
		},

		async createCampaign(data: Partial<EmailCampaign>) {
			update(state => ({ ...state, isLoading: true, error: null }));
			
			try {
				const { campaign } = await emailApi.createCampaign(data);
				
				update(state => ({
					...state,
					campaigns: [campaign, ...state.campaigns],
					currentCampaign: campaign,
					isLoading: false
				}));
				
				return campaign;
			} catch (error: any) {
				update(state => ({
					...state,
					error: error.message,
					isLoading: false
				}));
				throw error;
			}
		},

		async updateCampaign(id: string, data: Partial<EmailCampaign>) {
			update(state => ({ ...state, isLoading: true, error: null }));
			
			try {
				const { campaign } = await emailApi.updateCampaign(id, data);
				
				update(state => ({
					...state,
					campaigns: state.campaigns.map(c => c.id === id ? campaign : c),
					currentCampaign: campaign,
					isLoading: false
				}));
				
				return campaign;
			} catch (error: any) {
				update(state => ({
					...state,
					error: error.message,
					isLoading: false
				}));
				throw error;
			}
		},

		async deleteCampaign(id: string) {
			update(state => ({ ...state, isLoading: true, error: null }));
			
			try {
				await emailApi.deleteCampaign(id);
				
				update(state => ({
					...state,
					campaigns: state.campaigns.filter(c => c.id !== id),
					currentCampaign: state.currentCampaign?.id === id ? null : state.currentCampaign,
					isLoading: false
				}));
			} catch (error: any) {
				update(state => ({
					...state,
					error: error.message,
					isLoading: false
				}));
				throw error;
			}
		},

		async sendCampaign(id: string, sendAt?: string) {
			update(state => ({ ...state, isLoading: true, error: null }));
			
			try {
				await emailApi.sendCampaign(id, sendAt ? { send_at: sendAt } : undefined);
				
				// Reload campaign to get updated status
				const { campaign } = await emailApi.getCampaign(id);
				
				update(state => ({
					...state,
					campaigns: state.campaigns.map(c => c.id === id ? campaign : c),
					currentCampaign: campaign,
					isLoading: false
				}));
			} catch (error: any) {
				update(state => ({
					...state,
					error: error.message,
					isLoading: false
				}));
				throw error;
			}
		},

		async sendTestEmail(id: string, emails: string[]) {
			update(state => ({ ...state, isLoading: true, error: null }));
			
			try {
				await emailApi.sendTestEmail(id, emails);
				update(state => ({ ...state, isLoading: false }));
			} catch (error: any) {
				update(state => ({
					...state,
					error: error.message,
					isLoading: false
				}));
				throw error;
			}
		},

		// ═══════════════════════════════════════════════════════════════════════
		// Template Methods
		// ═══════════════════════════════════════════════════════════════════════

		async loadTemplates(params?: { category?: string; page?: number }) {
			update(state => ({ ...state, isLoading: true, error: null }));
			
			try {
				const response = await emailApi.getTemplates({
					...params,
					per_page: get({ subscribe }).pagination.per_page
				});
				
				update(state => ({
					...state,
					templates: response.templates,
					isLoading: false
				}));
			} catch (error: any) {
				update(state => ({
					...state,
					error: error.message,
					isLoading: false
				}));
			}
		},

		async loadTemplate(id: string) {
			update(state => ({ ...state, isLoading: true, error: null }));
			
			try {
				const { template } = await emailApi.getTemplate(id);
				
				update(state => ({
					...state,
					currentTemplate: template,
					isLoading: false
				}));
			} catch (error: any) {
				update(state => ({
					...state,
					error: error.message,
					isLoading: false
				}));
			}
		},

		async createTemplate(data: Partial<EmailTemplate>) {
			update(state => ({ ...state, isLoading: true, error: null }));
			
			try {
				const { template } = await emailApi.createTemplate(data);
				
				update(state => ({
					...state,
					templates: [template, ...state.templates],
					currentTemplate: template,
					isLoading: false
				}));
				
				return template;
			} catch (error: any) {
				update(state => ({
					...state,
					error: error.message,
					isLoading: false
				}));
				throw error;
			}
		},

		async updateTemplate(id: string, data: Partial<EmailTemplate>) {
			update(state => ({ ...state, isLoading: true, error: null }));
			
			try {
				const { template } = await emailApi.updateTemplate(id, data);
				
				update(state => ({
					...state,
					templates: state.templates.map(t => t.id === id ? template : t),
					currentTemplate: template,
					isLoading: false
				}));
				
				return template;
			} catch (error: any) {
				update(state => ({
					...state,
					error: error.message,
					isLoading: false
				}));
				throw error;
			}
		},

		async deleteTemplate(id: string) {
			update(state => ({ ...state, isLoading: true, error: null }));
			
			try {
				await emailApi.deleteTemplate(id);
				
				update(state => ({
					...state,
					templates: state.templates.filter(t => t.id !== id),
					currentTemplate: state.currentTemplate?.id === id ? null : state.currentTemplate,
					isLoading: false
				}));
			} catch (error: any) {
				update(state => ({
					...state,
					error: error.message,
					isLoading: false
				}));
				throw error;
			}
		},

		// ═══════════════════════════════════════════════════════════════════════
		// Sequence Methods
		// ═══════════════════════════════════════════════════════════════════════

		async loadSequences(params?: { status?: string; page?: number }) {
			update(state => ({ ...state, isLoading: true, error: null }));
			
			try {
				const response = await emailApi.getSequences({
					...params,
					per_page: get({ subscribe }).pagination.per_page
				});
				
				update(state => ({
					...state,
					sequences: response.sequences,
					isLoading: false
				}));
			} catch (error: any) {
				update(state => ({
					...state,
					error: error.message,
					isLoading: false
				}));
			}
		},

		async loadSequence(id: string) {
			update(state => ({ ...state, isLoading: true, error: null }));
			
			try {
				const { sequence } = await emailApi.getSequence(id);
				
				update(state => ({
					...state,
					currentSequence: sequence,
					isLoading: false
				}));
			} catch (error: any) {
				update(state => ({
					...state,
					error: error.message,
					isLoading: false
				}));
			}
		},

		async createSequence(data: Partial<EmailSequence>) {
			update(state => ({ ...state, isLoading: true, error: null }));
			
			try {
				const { sequence } = await emailApi.createSequence(data);
				
				update(state => ({
					...state,
					sequences: [sequence, ...state.sequences],
					currentSequence: sequence,
					isLoading: false
				}));
				
				return sequence;
			} catch (error: any) {
				update(state => ({
					...state,
					error: error.message,
					isLoading: false
				}));
				throw error;
			}
		},

		async updateSequence(id: string, data: Partial<EmailSequence>) {
			update(state => ({ ...state, isLoading: true, error: null }));
			
			try {
				const { sequence } = await emailApi.updateSequence(id, data);
				
				update(state => ({
					...state,
					sequences: state.sequences.map(s => s.id === id ? sequence : s),
					currentSequence: sequence,
					isLoading: false
				}));
				
				return sequence;
			} catch (error: any) {
				update(state => ({
					...state,
					error: error.message,
					isLoading: false
				}));
				throw error;
			}
		},

		async deleteSequence(id: string) {
			update(state => ({ ...state, isLoading: true, error: null }));
			
			try {
				await emailApi.deleteSequence(id);
				
				update(state => ({
					...state,
					sequences: state.sequences.filter(s => s.id !== id),
					currentSequence: state.currentSequence?.id === id ? null : state.currentSequence,
					isLoading: false
				}));
			} catch (error: any) {
				update(state => ({
					...state,
					error: error.message,
					isLoading: false
				}));
				throw error;
			}
		},

		// ═══════════════════════════════════════════════════════════════════════
		// Automation Methods
		// ═══════════════════════════════════════════════════════════════════════

		async loadAutomations(params?: { status?: string; page?: number }) {
			update(state => ({ ...state, isLoading: true, error: null }));
			
			try {
				const response = await emailApi.getAutomations({
					...params,
					per_page: get({ subscribe }).pagination.per_page
				}));
				
				update(state => ({
					...state,
					automations: response.automations,
					isLoading: false
				}));
			} catch (error: any) {
				update(state => ({
					...state,
					error: error.message,
					isLoading: false
				}));
			}
		},

		async loadAutomation(id: string) {
			update(state => ({ ...state, isLoading: true, error: null }));
			
			try {
				const { automation } = await emailApi.getAutomation(id);
				
				update(state => ({
					...state,
					currentAutomation: automation,
					isLoading: false
				}));
			} catch (error: any) {
				update(state => ({
					...state,
					error: error.message,
					isLoading: false
				}));
			}
		},

		async createAutomation(data: Partial<EmailAutomation>) {
			update(state => ({ ...state, isLoading: true, error: null }));
			
			try {
				const { automation } = await emailApi.createAutomation(data);
				
				update(state => ({
					...state,
					automations: [automation, ...state.automations],
					currentAutomation: automation,
					isLoading: false
				}));
				
				return automation;
			} catch (error: any) {
				update(state => ({
					...state,
					error: error.message,
					isLoading: false
				}));
				throw error;
			}
		},

		async updateAutomation(id: string, data: Partial<EmailAutomation>) {
			update(state => ({ ...state, isLoading: true, error: null }));
			
			try {
				const { automation } = await emailApi.updateAutomation(id, data);
				
				update(state => ({
					...state,
					automations: state.automations.map(a => a.id === id ? automation : a),
					currentAutomation: automation,
					isLoading: false
				}));
				
				return automation;
			} catch (error: any) {
				update(state => ({
					...state,
					error: error.message,
					isLoading: false
				}));
				throw error;
			}
		},

		async deleteAutomation(id: string) {
			update(state => ({ ...state, isLoading: true, error: null }));
			
			try {
				await emailApi.deleteAutomation(id);
				
				update(state => ({
					...state,
					automations: state.automations.filter(a => a.id !== id),
					currentAutomation: state.currentAutomation?.id === id ? null : state.currentAutomation,
					isLoading: false
				}));
			} catch (error: any) {
				update(state => ({
					...state,
					error: error.message,
					isLoading: false
				}));
				throw error;
			}
		},

		// ═══════════════════════════════════════════════════════════════════════
		// Segment Methods
		// ═══════════════════════════════════════════════════════════════════════

		async loadSegments() {
			update(state => ({ ...state, isLoading: true, error: null }));
			
			try {
				const response = await emailApi.getSegments();
				
				update(state => ({
					...state,
					segments: response.segments,
					isLoading: false
				}));
			} catch (error: any) {
				update(state => ({
					...state,
					error: error.message,
					isLoading: false
				}));
			}
		},

		async createSegment(data: Partial<EmailSegment>) {
			update(state => ({ ...state, isLoading: true, error: null }));
			
			try {
				const { segment } = await emailApi.createSegment(data);
				
				update(state => ({
					...state,
					segments: [segment, ...state.segments],
					isLoading: false
				}));
				
				return segment;
			} catch (error: any) {
				update(state => ({
					...state,
					error: error.message,
					isLoading: false
				}));
				throw error;
			}
		},

		// ═══════════════════════════════════════════════════════════════════════
		// Subscriber Methods
		// ═══════════════════════════════════════════════════════════════════════

		async loadSubscribers(params?: { status?: string; search?: string; page?: number }) {
			update(state => ({ ...state, isLoading: true, error: null }));
			
			try {
				const response = await emailApi.getSubscribers({
					...params,
					per_page: get({ subscribe }).pagination.per_page
				});
				
				update(state => ({
					...state,
					subscribers: response.subscribers,
					pagination: {
						...state.pagination,
						total: response.total
					},
					isLoading: false
				}));
			} catch (error: any) {
				update(state => ({
					...state,
					error: error.message,
					isLoading: false
				}));
			}
		},

		// ═══════════════════════════════════════════════════════════════════════
		// Analytics Methods
		// ═══════════════════════════════════════════════════════════════════════

		async loadAnalytics(period: string = '30d') {
			update(state => ({ ...state, isLoading: true, error: null }));
			
			try {
				const { analytics } = await emailApi.getAnalytics(period);
				
				update(state => ({
					...state,
					analytics,
					isLoading: false
				}));
			} catch (error: any) {
				update(state => ({
					...state,
					error: error.message,
					isLoading: false
				}));
			}
		},

		// ═══════════════════════════════════════════════════════════════════════
		// Utility Methods
		// ═══════════════════════════════════════════════════════════════════════

		clearError() {
			update(state => ({ ...state, error: null }));
		},

		reset() {
			set(initialState);
		}
	};
}

export const emailStore = createEmailStore();

// ═══════════════════════════════════════════════════════════════════════════
// Derived Stores
// ═══════════════════════════════════════════════════════════════════════════

export const activeCampaigns = derived(
	emailStore,
	$store => $store.campaigns.filter(c => c.status === 'sending' || c.status === 'sent')
);

export const draftCampaigns = derived(
	emailStore,
	$store => $store.campaigns.filter(c => c.status === 'draft')
);

export const activeSequences = derived(
	emailStore,
	$store => $store.sequences.filter(s => s.status === 'active')
);

export const activeAutomations = derived(
	emailStore,
	$store => $store.automations.filter(a => a.status === 'active')
);

export const isLoading = derived(
	emailStore,
	$store => $store.isLoading
);

export const hasError = derived(
	emailStore,
	$store => $store.error !== null
);

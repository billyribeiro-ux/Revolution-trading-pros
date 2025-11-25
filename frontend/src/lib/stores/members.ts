/**
 * Enterprise Members Store
 * Revolution Trading Pros - L8+ Google Enterprise Grade
 */

import { writable, derived, get } from 'svelte/store';
import {
	membersApi,
	type Member,
	type MemberStats,
	type Service,
	type ChurnedStats,
	type MemberFilters,
	type ChurnedFilters,
	type PaginationInfo,
	type EmailTemplate
} from '$lib/api/members';

// State interfaces
interface MembersState {
	members: Member[];
	stats: MemberStats | null;
	services: Service[];
	pagination: PaginationInfo | null;
	filters: MemberFilters;
	loading: boolean;
	error: string | null;
	selectedMember: Member | null;
	selectedMemberEngagement: number;
	selectedMemberTimeline: Array<{
		type: string;
		title: string;
		date: string;
		icon: string;
		meta?: Record<string, unknown>;
	}>;
}

interface ChurnedState {
	members: Member[];
	stats: ChurnedStats | null;
	pagination: PaginationInfo | null;
	filters: ChurnedFilters;
	loading: boolean;
	error: string | null;
}

interface ServiceMembersState {
	service: { id: number; name: string; type: string } | null;
	stats: {
		total_members: number;
		active_members: number;
		trial_members: number;
		churned_members: number;
		total_revenue: number;
	} | null;
	members: Member[];
	pagination: PaginationInfo | null;
	loading: boolean;
	error: string | null;
}

interface EmailState {
	templates: EmailTemplate[];
	presetTemplates: EmailTemplate[];
	loading: boolean;
	sending: boolean;
	error: string | null;
}

// Initial states
const initialMembersState: MembersState = {
	members: [],
	stats: null,
	services: [],
	pagination: null,
	filters: {
		per_page: 25,
		page: 1,
		sort_by: 'created_at',
		sort_dir: 'desc'
	},
	loading: false,
	error: null,
	selectedMember: null,
	selectedMemberEngagement: 0,
	selectedMemberTimeline: []
};

const initialChurnedState: ChurnedState = {
	members: [],
	stats: null,
	pagination: null,
	filters: {
		per_page: 25,
		page: 1
	},
	loading: false,
	error: null
};

const initialServiceMembersState: ServiceMembersState = {
	service: null,
	stats: null,
	members: [],
	pagination: null,
	loading: false,
	error: null
};

const initialEmailState: EmailState = {
	templates: [],
	presetTemplates: [],
	loading: false,
	sending: false,
	error: null
};

// Create stores
function createMembersStore() {
	const { subscribe, set, update } = writable<MembersState>(initialMembersState);

	return {
		subscribe,

		/**
		 * Load members with current filters
		 */
		async loadMembers(filters?: MemberFilters) {
			update((state) => ({
				...state,
				loading: true,
				error: null,
				filters: filters ? { ...state.filters, ...filters } : state.filters
			}));

			try {
				const currentState = get({ subscribe });
				const response = await membersApi.getMembers(currentState.filters);
				update((state) => ({
					...state,
					members: response.members,
					pagination: response.pagination,
					loading: false
				}));
			} catch (error) {
				update((state) => ({
					...state,
					error: error instanceof Error ? error.message : 'Failed to load members',
					loading: false
				}));
			}
		},

		/**
		 * Load member statistics
		 */
		async loadStats() {
			try {
				const stats = await membersApi.getStats();
				update((state) => ({ ...state, stats }));
			} catch (error) {
				console.error('Failed to load member stats:', error);
			}
		},

		/**
		 * Load available services
		 */
		async loadServices() {
			try {
				const response = await membersApi.getServices();
				update((state) => ({ ...state, services: response.services }));
			} catch (error) {
				console.error('Failed to load services:', error);
			}
		},

		/**
		 * Load single member details
		 */
		async loadMember(id: number) {
			update((state) => ({ ...state, loading: true, error: null }));

			try {
				const response = await membersApi.getMember(id);
				update((state) => ({
					...state,
					selectedMember: response.member,
					selectedMemberEngagement: response.engagement_score,
					selectedMemberTimeline: response.timeline,
					loading: false
				}));
			} catch (error) {
				update((state) => ({
					...state,
					error: error instanceof Error ? error.message : 'Failed to load member',
					loading: false
				}));
			}
		},

		/**
		 * Update filters and reload
		 */
		async setFilters(filters: Partial<MemberFilters>) {
			const currentState = get({ subscribe });
			const newFilters = { ...currentState.filters, ...filters, page: 1 };
			await this.loadMembers(newFilters);
		},

		/**
		 * Go to page
		 */
		async goToPage(page: number) {
			await this.loadMembers({ page });
		},

		/**
		 * Clear selected member
		 */
		clearSelectedMember() {
			update((state) => ({
				...state,
				selectedMember: null,
				selectedMemberEngagement: 0,
				selectedMemberTimeline: []
			}));
		},

		/**
		 * Reset store
		 */
		reset() {
			set(initialMembersState);
		}
	};
}

function createChurnedStore() {
	const { subscribe, set, update } = writable<ChurnedState>(initialChurnedState);

	return {
		subscribe,

		/**
		 * Load churned members
		 */
		async loadChurnedMembers(filters?: ChurnedFilters) {
			update((state) => ({
				...state,
				loading: true,
				error: null,
				filters: filters ? { ...state.filters, ...filters } : state.filters
			}));

			try {
				const currentState = get({ subscribe });
				const response = await membersApi.getChurnedMembers(currentState.filters);
				update((state) => ({
					...state,
					members: response.members,
					stats: response.stats,
					pagination: response.pagination,
					loading: false
				}));
			} catch (error) {
				update((state) => ({
					...state,
					error: error instanceof Error ? error.message : 'Failed to load churned members',
					loading: false
				}));
			}
		},

		/**
		 * Update filters and reload
		 */
		async setFilters(filters: Partial<ChurnedFilters>) {
			const currentState = get({ subscribe });
			const newFilters = { ...currentState.filters, ...filters, page: 1 };
			await this.loadChurnedMembers(newFilters);
		},

		/**
		 * Go to page
		 */
		async goToPage(page: number) {
			await this.loadChurnedMembers({ page });
		},

		/**
		 * Reset store
		 */
		reset() {
			set(initialChurnedState);
		}
	};
}

function createServiceMembersStore() {
	const { subscribe, set, update } = writable<ServiceMembersState>(initialServiceMembersState);

	return {
		subscribe,

		/**
		 * Load members for a specific service
		 */
		async loadServiceMembers(
			serviceId: number,
			filters?: { status?: string; search?: string; per_page?: number; page?: number }
		) {
			update((state) => ({ ...state, loading: true, error: null }));

			try {
				const response = await membersApi.getMembersByService(serviceId, filters);
				update((state) => ({
					...state,
					service: response.service,
					stats: response.stats,
					members: response.members,
					pagination: response.pagination,
					loading: false
				}));
			} catch (error) {
				update((state) => ({
					...state,
					error: error instanceof Error ? error.message : 'Failed to load service members',
					loading: false
				}));
			}
		},

		/**
		 * Reset store
		 */
		reset() {
			set(initialServiceMembersState);
		}
	};
}

function createEmailStore() {
	const { subscribe, set, update } = writable<EmailState>(initialEmailState);

	return {
		subscribe,

		/**
		 * Load email templates
		 */
		async loadTemplates() {
			update((state) => ({ ...state, loading: true, error: null }));

			try {
				const response = await membersApi.getEmailTemplates();
				update((state) => ({
					...state,
					templates: response.templates,
					presetTemplates: response.preset_templates,
					loading: false
				}));
			} catch (error) {
				update((state) => ({
					...state,
					error: error instanceof Error ? error.message : 'Failed to load templates',
					loading: false
				}));
			}
		},

		/**
		 * Send email to single member
		 */
		async sendEmail(
			memberId: number,
			data: {
				template_id?: string | number;
				subject: string;
				body: string;
				campaign_type?: 'winback' | 'promo' | 'general' | 'reminder';
			}
		) {
			update((state) => ({ ...state, sending: true, error: null }));

			try {
				const response = await membersApi.sendEmail(memberId, data);
				update((state) => ({ ...state, sending: false }));
				return response;
			} catch (error) {
				update((state) => ({
					...state,
					error: error instanceof Error ? error.message : 'Failed to send email',
					sending: false
				}));
				throw error;
			}
		},

		/**
		 * Send bulk email
		 */
		async sendBulkEmail(data: {
			member_ids: number[];
			template_id?: string | number;
			subject: string;
			body: string;
			campaign_type?: 'winback' | 'promo' | 'general' | 'reminder' | 'free_trial';
			personalize?: boolean;
		}) {
			update((state) => ({ ...state, sending: true, error: null }));

			try {
				const response = await membersApi.sendBulkEmail(data);
				update((state) => ({ ...state, sending: false }));
				return response;
			} catch (error) {
				update((state) => ({
					...state,
					error: error instanceof Error ? error.message : 'Failed to send emails',
					sending: false
				}));
				throw error;
			}
		},

		/**
		 * Reset store
		 */
		reset() {
			set(initialEmailState);
		}
	};
}

// Export stores
export const membersStore = createMembersStore();
export const churnedStore = createChurnedStore();
export const serviceMembersStore = createServiceMembersStore();
export const emailStore = createEmailStore();

// Derived stores
export const activeMembersCount = derived(membersStore, ($store) =>
	$store.stats ? $store.stats.subscriptions.active : 0
);

export const totalMembersCount = derived(membersStore, ($store) =>
	$store.stats ? $store.stats.overview.total_members : 0
);

export const monthlyRecurringRevenue = derived(membersStore, ($store) =>
	$store.stats ? $store.stats.revenue.mrr : 0
);

export const churnRate = derived(membersStore, ($store) =>
	$store.stats ? $store.stats.subscriptions.churn_rate : 0
);

/**
 * Enterprise Members Store (Svelte 5 Runes)
 * Revolution Trading Pros - L8+ Google Enterprise Grade
 *
 * @version 2.0.0 - Svelte 5 Runes Migration
 */

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
import { logger } from '$lib/utils/logger';

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

interface MemberEmailState {
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

const initialEmailState: MemberEmailState = {
	templates: [],
	presetTemplates: [],
	loading: false,
	sending: false,
	error: null
};

// ═══════════════════════════════════════════════════════════════════════════
// State (Svelte 5 Runes)
// ═══════════════════════════════════════════════════════════════════════════

let membersState = $state<MembersState>({ ...initialMembersState });
let churnedState = $state<ChurnedState>({ ...initialChurnedState });
let serviceMembersState = $state<ServiceMembersState>({ ...initialServiceMembersState });
let memberEmailState = $state<MemberEmailState>({ ...initialEmailState });

// ═══════════════════════════════════════════════════════════════════════════
// Members Store
// ═══════════════════════════════════════════════════════════════════════════

export const membersStore = {
	get state() {
		return membersState;
	},

	get members() {
		return membersState.members;
	},

	get stats() {
		return membersState.stats;
	},

	get loading() {
		return membersState.loading;
	},

	get selectedMember() {
		return membersState.selectedMember;
	},

	get services() {
		return membersState.services;
	},

	get pagination() {
		return membersState.pagination;
	},

	/**
	 * Load members with current filters
	 * ICT 7 Grade: API client handles request deduplication automatically
	 */
	async loadMembers(filters?: MemberFilters) {
		const newFilters = filters ? { ...membersState.filters, ...filters } : membersState.filters;

		membersState = {
			...membersState,
			loading: true,
			error: null,
			filters: newFilters
		};

		try {
			const response = await membersApi.getMembers(newFilters);
			membersState = {
				...membersState,
				members: response.members,
				pagination: response.pagination,
				loading: false
			};
		} catch (error) {
			membersState = {
				...membersState,
				error: error instanceof Error ? error.message : 'Failed to load members',
				loading: false
			};
		}
	},

	/**
	 * Load member statistics
	 */
	async loadStats() {
		try {
			const stats = await membersApi.getStats();
			membersState = { ...membersState, stats };
		} catch (error) {
			logger.error('Failed to load member stats:', error);
		}
	},

	/**
	 * Load available services
	 */
	async loadServices() {
		try {
			const response = await membersApi.getServices();
			membersState = { ...membersState, services: response.services };
		} catch (error) {
			logger.error('Failed to load services:', error);
		}
	},

	/**
	 * Load single member details
	 */
	async loadMember(id: number) {
		membersState = { ...membersState, loading: true, error: null };

		try {
			const response = await membersApi.getMember(id);
			membersState = {
				...membersState,
				selectedMember: response.member,
				selectedMemberEngagement: response.engagement_score,
				selectedMemberTimeline: response.timeline,
				loading: false
			};
		} catch (error) {
			membersState = {
				...membersState,
				error: error instanceof Error ? error.message : 'Failed to load member',
				loading: false
			};
		}
	},

	/**
	 * Update filters and reload
	 */
	async setFilters(filters: Partial<MemberFilters>) {
		const newFilters = { ...membersState.filters, ...filters, page: 1 };
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
		membersState = {
			...membersState,
			selectedMember: null,
			selectedMemberEngagement: 0,
			selectedMemberTimeline: []
		};
	},

	/**
	 * Reset store
	 */
	reset() {
		membersState = { ...initialMembersState };
	}
};

// ═══════════════════════════════════════════════════════════════════════════
// Churned Store
// ═══════════════════════════════════════════════════════════════════════════

export const churnedStore = {
	get state() {
		return churnedState;
	},

	get members() {
		return churnedState.members;
	},

	get stats() {
		return churnedState.stats;
	},

	get loading() {
		return churnedState.loading;
	},

	get pagination() {
		return churnedState.pagination;
	},

	/**
	 * Load churned members
	 */
	async loadChurnedMembers(filters?: ChurnedFilters) {
		const newFilters = filters ? { ...churnedState.filters, ...filters } : churnedState.filters;

		churnedState = {
			...churnedState,
			loading: true,
			error: null,
			filters: newFilters
		};

		try {
			const response = await membersApi.getChurnedMembers(newFilters);
			churnedState = {
				...churnedState,
				members: response.members,
				stats: response.stats,
				pagination: response.pagination,
				loading: false
			};
		} catch (error) {
			churnedState = {
				...churnedState,
				error: error instanceof Error ? error.message : 'Failed to load churned members',
				loading: false
			};
		}
	},

	/**
	 * Update filters and reload
	 */
	async setFilters(filters: Partial<ChurnedFilters>) {
		const newFilters = { ...churnedState.filters, ...filters, page: 1 };
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
		churnedState = { ...initialChurnedState };
	}
};

// ═══════════════════════════════════════════════════════════════════════════
// Service Members Store
// ═══════════════════════════════════════════════════════════════════════════

export const serviceMembersStore = {
	get state() {
		return serviceMembersState;
	},

	get members() {
		return serviceMembersState.members;
	},

	get service() {
		return serviceMembersState.service;
	},

	get loading() {
		return serviceMembersState.loading;
	},

	get stats() {
		return serviceMembersState.stats;
	},

	get pagination() {
		return serviceMembersState.pagination;
	},

	/**
	 * Load members for a specific service
	 */
	async loadServiceMembers(
		serviceId: number,
		filters?: { status?: string; search?: string; per_page?: number; page?: number }
	) {
		serviceMembersState = { ...serviceMembersState, loading: true, error: null };

		try {
			const response = await membersApi.getMembersByService(serviceId, filters);
			serviceMembersState = {
				...serviceMembersState,
				service: response.service,
				stats: response.stats,
				members: response.members,
				pagination: response.pagination,
				loading: false
			};
		} catch (error) {
			serviceMembersState = {
				...serviceMembersState,
				error: error instanceof Error ? error.message : 'Failed to load service members',
				loading: false
			};
		}
	},

	/**
	 * Reset store
	 */
	reset() {
		serviceMembersState = { ...initialServiceMembersState };
	}
};

// ═══════════════════════════════════════════════════════════════════════════
// Email Store (for members)
// ═══════════════════════════════════════════════════════════════════════════

export const emailStore = {
	get state() {
		return memberEmailState;
	},

	get templates() {
		return memberEmailState.templates;
	},

	get sending() {
		return memberEmailState.sending;
	},

	get presetTemplates() {
		return memberEmailState.presetTemplates;
	},

	/**
	 * Load email templates
	 */
	async loadTemplates() {
		memberEmailState = { ...memberEmailState, loading: true, error: null };

		try {
			const response = await membersApi.getEmailTemplates();
			memberEmailState = {
				...memberEmailState,
				templates: response.templates,
				presetTemplates: response.preset_templates,
				loading: false
			};
		} catch (error) {
			memberEmailState = {
				...memberEmailState,
				error: error instanceof Error ? error.message : 'Failed to load templates',
				loading: false
			};
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
		memberEmailState = { ...memberEmailState, sending: true, error: null };

		try {
			const response = await membersApi.sendEmail(memberId, data);
			memberEmailState = { ...memberEmailState, sending: false };
			return response;
		} catch (error) {
			memberEmailState = {
				...memberEmailState,
				error: error instanceof Error ? error.message : 'Failed to send email',
				sending: false
			};
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
		memberEmailState = { ...memberEmailState, sending: true, error: null };

		try {
			const response = await membersApi.sendBulkEmail(data);
			memberEmailState = { ...memberEmailState, sending: false };
			return response;
		} catch (error) {
			memberEmailState = {
				...memberEmailState,
				error: error instanceof Error ? error.message : 'Failed to send emails',
				sending: false
			};
			throw error;
		}
	},

	/**
	 * Reset store
	 */
	reset() {
		memberEmailState = { ...initialEmailState };
	}
};

// ═══════════════════════════════════════════════════════════════════════════
// Getter Functions (Svelte 5 - cannot export $derived from modules)
// ═══════════════════════════════════════════════════════════════════════════

export function getActiveMembersCount(): number {
	return membersState.stats?.subscriptions?.active ?? 0;
}

export function getTotalMembersCount(): number {
	return membersState.stats?.overview?.total_members ?? 0;
}

export function getMonthlyRecurringRevenue(): number {
	return membersState.stats?.revenue?.mrr ?? 0;
}

export function getChurnRate(): number {
	return membersState.stats?.subscriptions?.churn_rate ?? 0;
}

import type { MemberProfile } from '#lib/api/member-profile.js';
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
} from '#lib/api/members.js';

import type {
	ServiceCatalogFilters,
	ServiceMemberFilters,
	ServiceMembers
} from '#lib/api/member-statistics.js';

// State interfaces
interface MembersState {
	members: Member[];
	stats: MemberStats | null;
	services: Service[];
	statsLoading: boolean;
	statsError: string | null;
	servicesLoading: boolean;
	servicesError: string | null;
	servicesPagination: PaginationInfo | null;
	servicesFilters: ServiceCatalogFilters;
	pagination: PaginationInfo | null;
	filters: MemberFilters;
	loading: boolean;
	error: string | null;
	selectedMember: MemberProfile['member'] | null;
	selectedMemberEngagement: null;
	selectedMemberTimeline: MemberProfile['timeline'] | null;
	selectedMemberLoading: boolean;
	selectedMemberError: string | null;
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
	service: ServiceMembers['service'] | null;
	stats: ServiceMembers['stats'] | null;
	members: ServiceMembers['members'];
	pagination: PaginationInfo | null;
	serviceId: number | null;
	filters: ServiceMemberFilters;
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
	statsLoading: false,
	statsError: null,
	servicesLoading: false,
	servicesError: null,
	servicesPagination: null,
	servicesFilters: {},
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
	selectedMemberEngagement: null,
	selectedMemberTimeline: null,
	selectedMemberLoading: false,
	selectedMemberError: null
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
	serviceId: null,
	filters: { page: 1, per_page: 25 },
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

let memberLoadGeneration = 0;
let statsLoadGeneration = 0;
let catalogLoadGeneration = 0;
let serviceLoadGeneration = 0;
let membersState = $state<MembersState>({ ...initialMembersState });
let churnedState = $state<ChurnedState>({ ...initialChurnedState });
let serviceMembersState = $state<ServiceMembersState>({ ...initialServiceMembersState });
let memberEmailState = $state<MemberEmailState>({ ...initialEmailState });

// ═══════════════════════════════════════════════════════════════════════════
// Members Store
// ═══════════════════════════════════════════════════════════════════════════

let memberDetailGeneration = 0;

export const membersStore = {
	get error() {
		return membersState.error;
	},
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

	get selectedMemberLoading() {
		return membersState.selectedMemberLoading;
	},
	get selectedMemberError() {
		return membersState.selectedMemberError;
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
		const generation = ++memberLoadGeneration;
		const newFilters = filters ? { ...membersState.filters, ...filters } : membersState.filters;

		membersState = {
			...membersState,
			loading: true,
			members: [],
			error: null,
			filters: newFilters
		};

		try {
			const response = await membersApi.getMembers(newFilters);
			if (generation !== memberLoadGeneration) return;
			membersState = {
				...membersState,
				members: response.members,
				pagination: response.pagination,
				loading: false
			};
		} catch (error) {
			if (generation !== memberLoadGeneration) return;
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
		const generation = ++statsLoadGeneration;
		membersState = { ...membersState, stats: null, statsLoading: true, statsError: null };
		try {
			const stats = await membersApi.getStats();
			if (generation !== statsLoadGeneration) return;
			membersState = { ...membersState, stats, statsLoading: false };
		} catch (error) {
			if (generation !== statsLoadGeneration) return;
			membersState = {
				...membersState,
				statsLoading: false,
				statsError: error instanceof Error ? error.message : 'Failed to load member statistics'
			};
		}
	},

	async loadServices(filters?: ServiceCatalogFilters) {
		const generation = ++catalogLoadGeneration;
		const selected = { ...membersState.servicesFilters, ...filters };
		membersState = {
			...membersState,
			servicesLoading: true,
			servicesError: null,
			servicesFilters: selected
		};
		try {
			const response = await membersApi.getServices(selected);
			if (generation !== catalogLoadGeneration) return;
			const services =
				response.selected &&
				!response.services.some((service) => service.id === response.selected?.id)
					? [response.selected, ...response.services]
					: response.services;
			membersState = {
				...membersState,
				services,
				servicesPagination: response.pagination,
				servicesLoading: false
			};
		} catch (error) {
			if (generation !== catalogLoadGeneration) return;
			membersState = {
				...membersState,
				services: [],
				servicesPagination: null,
				servicesLoading: false,
				servicesError: error instanceof Error ? error.message : 'Failed to load membership plans'
			};
		}
	},

	/**
	 * Load single member details
	 */
	async loadMember(id: string | number) {
		const generation = ++memberDetailGeneration;
		membersState = {
			...membersState,
			selectedMember: null,
			selectedMemberTimeline: null,
			selectedMemberEngagement: null,
			selectedMemberLoading: true,
			selectedMemberError: null
		};
		try {
			const response = await membersApi.getMember(id);
			if (generation !== memberDetailGeneration) return;
			membersState = {
				...membersState,
				selectedMember: response.member,
				selectedMemberEngagement: response.engagement.score,
				selectedMemberTimeline: response.timeline,
				selectedMemberLoading: false
			};
		} catch (error) {
			if (generation !== memberDetailGeneration) return;
			membersState = {
				...membersState,
				selectedMemberError:
					error instanceof Error ? error.message : 'Failed to load recorded member profile',
				selectedMemberLoading: false
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
		++memberDetailGeneration;
		membersState = {
			...membersState,
			selectedMember: null,
			selectedMemberEngagement: null,
			selectedMemberTimeline: null,
			selectedMemberLoading: false,
			selectedMemberError: null
		};
	},

	/**
	 * Reset store
	 */
	reset() {
		++memberDetailGeneration;
		++memberLoadGeneration;
		++statsLoadGeneration;
		++catalogLoadGeneration;
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
	async loadServiceMembers(serviceId: number, filters?: ServiceMemberFilters) {
		const generation = ++serviceLoadGeneration;
		const nextFilters = {
			...(serviceMembersState.serviceId === serviceId
				? serviceMembersState.filters
				: initialServiceMembersState.filters),
			...filters
		};
		serviceMembersState = {
			...serviceMembersState,
			service: null,
			stats: null,
			members: [],
			pagination: null,
			serviceId,
			filters: nextFilters,
			loading: true,
			error: null
		};
		try {
			const response = await membersApi.getMembersByService(serviceId, nextFilters);
			if (generation !== serviceLoadGeneration) return;
			serviceMembersState = {
				...serviceMembersState,
				service: response.service,
				stats: response.stats,
				members: response.members,
				pagination: response.pagination,
				loading: false
			};
		} catch (error) {
			if (generation !== serviceLoadGeneration) return;
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
		++serviceLoadGeneration;
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

export function getActiveMembersCount(): number | null {
	return membersState.stats?.subscriptions.active ?? null;
}

export function getTotalMembersCount(): number | null {
	return membersState.stats?.overview.total_members ?? null;
}

export function getMonthlyRecurringRevenue(): number | null {
	const cents = membersState.stats?.revenue.mrr_cents ?? null;
	return cents === null ? null : cents / 100;
}

export function getChurnRate(): number | null {
	return membersState.stats?.subscriptions.churn_rate ?? null;
}

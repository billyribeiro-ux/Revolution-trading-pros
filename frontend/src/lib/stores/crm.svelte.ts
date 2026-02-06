/**
 * RevolutionCRM-L8-System - Svelte 5 Runes Store
 * Apple Principal Engineer ICT 7 Grade - January 2026
 *
 * Migrated from Svelte 4 writable/derived to Svelte 5 $state/$derived runes
 * for improved reactivity and type safety.
 */

import type { Contact, Deal, Pipeline, ContactFilters, DealFilters } from '$lib/crm/types';

// ═══════════════════════════════════════════════════════════════════════════
// STATE - Svelte 5 Runes
// ═══════════════════════════════════════════════════════════════════════════

class CrmStore {
	// Contacts state
	contacts = $state<Contact[]>([]);
	selectedContact = $state<Contact | null>(null);
	contactFilters = $state<ContactFilters>({});

	// Deals state
	deals = $state<Deal[]>([]);
	selectedDeal = $state<Deal | null>(null);
	dealFilters = $state<DealFilters>({});

	// Pipelines state
	pipelines = $state<Pipeline[]>([]);
	selectedPipeline = $state<Pipeline | null>(null);

	// UI state
	isLoading = $state(false);
	error = $state<string | null>(null);

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED STATE
	// ═══════════════════════════════════════════════════════════════════════════

	get dealsByStage(): Record<string, Deal[]> {
		if (!this.selectedPipeline) return {};

		const grouped: Record<string, Deal[]> = {};

		this.selectedPipeline.stages?.forEach((stage) => {
			grouped[stage.id] = this.deals.filter((deal) => deal.stage_id === stage.id);
		});

		return grouped;
	}

	get contactsByStatus(): Record<string, Contact[]> {
		return this.contacts.reduce(
			(acc, contact) => {
				if (!acc[contact.status]) acc[contact.status] = [];
				acc[contact.status]!.push(contact);
				return acc;
			},
			{} as Record<string, Contact[]>
		);
	}

	get highValueContacts(): Contact[] {
		return this.contacts
			.filter((c) => c.lead_score >= 75 || c.lifetime_value > 1000)
			.sort((a, b) => b.lead_score - a.lead_score);
	}

	get atRiskContacts(): Contact[] {
		return this.contacts
			.filter((c) => c.status === 'customer' && c.health_score < 50)
			.sort((a, b) => a.health_score - b.health_score);
	}

	get totalContactCount(): number {
		return this.contacts.length;
	}

	get totalDealValue(): number {
		return this.deals.reduce((sum, deal) => sum + deal.amount, 0);
	}

	get openDealsCount(): number {
		return this.deals.filter((d) => d.status === 'open').length;
	}

	get wonDealsCount(): number {
		return this.deals.filter((d) => d.status === 'won').length;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// ACTIONS
	// ═══════════════════════════════════════════════════════════════════════════

	setContacts(contacts: Contact[]) {
		this.contacts = contacts;
	}

	addContact(contact: Contact) {
		this.contacts = [...this.contacts, contact];
	}

	updateContact(id: string, updates: Partial<Contact>) {
		this.contacts = this.contacts.map((c) =>
			c.id === id ? { ...c, ...updates } : c
		);
	}

	removeContact(id: string) {
		this.contacts = this.contacts.filter((c) => c.id !== id);
	}

	selectContact(contact: Contact | null) {
		this.selectedContact = contact;
	}

	setDeals(deals: Deal[]) {
		this.deals = deals;
	}

	addDeal(deal: Deal) {
		this.deals = [...this.deals, deal];
	}

	updateDeal(id: string, updates: Partial<Deal>) {
		this.deals = this.deals.map((d) =>
			d.id === id ? { ...d, ...updates } : d
		);
	}

	removeDeal(id: string) {
		this.deals = this.deals.filter((d) => d.id !== id);
	}

	selectDeal(deal: Deal | null) {
		this.selectedDeal = deal;
	}

	setPipelines(pipelines: Pipeline[]) {
		this.pipelines = pipelines;
	}

	selectPipeline(pipeline: Pipeline | null) {
		this.selectedPipeline = pipeline;
	}

	setLoading(loading: boolean) {
		this.isLoading = loading;
	}

	setError(error: string | null) {
		this.error = error;
	}

	clearError() {
		this.error = null;
	}

	reset() {
		this.contacts = [];
		this.selectedContact = null;
		this.contactFilters = {};
		this.deals = [];
		this.selectedDeal = null;
		this.dealFilters = {};
		this.pipelines = [];
		this.selectedPipeline = null;
		this.isLoading = false;
		this.error = null;
	}
}

// Export singleton instance
export const crmStore = new CrmStore();

// Also export individual reactive getters for component use
export const getContacts = () => crmStore.contacts;
export const getDeals = () => crmStore.deals;
export const getPipelines = () => crmStore.pipelines;
export const getIsLoading = () => crmStore.isLoading;
export const getError = () => crmStore.error;

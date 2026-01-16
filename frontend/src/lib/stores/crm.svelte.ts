/**
 * RevolutionCRM-L8-System - Svelte Stores
 */

import { writable, derived } from 'svelte/store';
import type { Contact, Deal, Pipeline, ContactFilters, DealFilters } from '$lib/crm/types';

export const contacts = writable<Contact[]>([]);
export const selectedContact = writable<Contact | null>(null);
export const contactFilters = writable<ContactFilters>({});

export const deals = writable<Deal[]>([]);
export const selectedDeal = writable<Deal | null>(null);
export const dealFilters = writable<DealFilters>({});

export const pipelines = writable<Pipeline[]>([]);
export const selectedPipeline = writable<Pipeline | null>(null);

export const isLoading = writable(false);
export const error = writable<string | null>(null);

// Derived stores
export const dealsByStage = derived([deals, selectedPipeline], ([$deals, $selectedPipeline]) => {
	if (!$selectedPipeline) return {};

	const grouped: Record<string, Deal[]> = {};

	$selectedPipeline.stages?.forEach((stage) => {
		grouped[stage.id] = $deals.filter((deal) => deal.stage_id === stage.id);
	});

	return grouped;
});

export const contactsByStatus = derived(contacts, ($contacts) => {
	return $contacts.reduce(
		(acc, contact) => {
			if (!acc[contact.status]) acc[contact.status] = [];
			acc[contact.status]!.push(contact);
			return acc;
		},
		{} as Record<string, Contact[]>
	);
});

export const highValueContacts = derived(contacts, ($contacts) => {
	return $contacts
		.filter((c) => c.lead_score >= 75 || c.lifetime_value > 1000)
		.sort((a, b) => b.lead_score - a.lead_score);
});

export const atRiskContacts = derived(contacts, ($contacts) => {
	return $contacts
		.filter((c) => c.status === 'customer' && c.health_score < 50)
		.sort((a, b) => a.health_score - b.health_score);
});

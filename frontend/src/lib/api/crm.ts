/**
 * RevolutionCRM-L8-System - API Client
 * FluentCRM Pro Feature Parity - December 2025
 */

import type {
	Contact,
	Deal,
	Pipeline,
	Stage,
	Activity,
	Note,
	ContactSegment,
	TimelineEvent,
	DealForecast,
	ContactFilters,
	DealFilters,
	// FluentCRM Pro - Email Sequences
	EmailSequence,
	SequenceMail,
	SequenceTracker,
	SequenceFilters,
	SequenceStats,
	// FluentCRM Pro - Recurring Campaigns
	RecurringCampaign,
	RecurringMail,
	RecurringCampaignFilters,
	RecurringCampaignStats,
	// FluentCRM Pro - Smart Links
	SmartLink,
	SmartLinkClick,
	SmartLinkFilters,
	SmartLinkStats,
	SmartLinkAnalytics,
	// FluentCRM Pro - Automation Funnels
	AutomationFunnel,
	FunnelAction,
	FunnelSubscriber,
	FunnelFilters,
	FunnelStats,
	// FluentCRM Pro - Contact Lists
	ContactList,
	// FluentCRM Pro - Contact Tags
	ContactTag,
	// FluentCRM Pro - CRM Companies
	CrmCompany,
	CompanyFilters,
	CompanyStats
} from '$lib/crm/types';
import { apiClient } from './client';

export class CrmAPI {
	// Contacts
	async getContacts(filters?: ContactFilters): Promise<{ data: Contact[]; meta: any }> {
		return apiClient.get('/admin/crm/contacts', { params: filters });
	}

	async getContact(id: string): Promise<Contact> {
		return apiClient.get(`/admin/crm/contacts/${id}`);
	}

	async createContact(data: Partial<Contact>): Promise<Contact> {
		return apiClient.post('/admin/crm/contacts', data);
	}

	async updateContact(id: string, data: Partial<Contact>): Promise<Contact> {
		return apiClient.put(`/admin/crm/contacts/${id}`, data);
	}

	async deleteContact(id: string): Promise<void> {
		return apiClient.delete(`/admin/crm/contacts/${id}`);
	}

	async getContactTimeline(id: string): Promise<TimelineEvent[]> {
		return apiClient.get(`/admin/crm/contacts/${id}/timeline`);
	}

	async recalculateContactScore(id: string): Promise<Contact> {
		return apiClient.post(`/admin/crm/contacts/${id}/recalculate-score`);
	}

	// Deals
	async getDeals(filters?: DealFilters): Promise<{ data: Deal[]; meta: any }> {
		return apiClient.get('/admin/crm/deals', { params: filters });
	}

	async getDeal(id: string): Promise<Deal> {
		return apiClient.get(`/admin/crm/deals/${id}`);
	}

	async createDeal(data: Partial<Deal>): Promise<Deal> {
		return apiClient.post('/admin/crm/deals', data);
	}

	async updateDeal(id: string, data: Partial<Deal>): Promise<Deal> {
		return apiClient.put(`/admin/crm/deals/${id}`, data);
	}

	async updateDealStage(id: string, stageId: string, reason?: string): Promise<Deal> {
		return apiClient.patch(`/admin/crm/deals/${id}/stage`, { stage_id: stageId, reason });
	}

	async winDeal(id: string, wonDetails?: string, closeDate?: string): Promise<Deal> {
		return apiClient.post(`/admin/crm/deals/${id}/win`, {
			won_details: wonDetails,
			close_date: closeDate
		});
	}

	async loseDeal(id: string, lostReason: string): Promise<Deal> {
		return apiClient.post(`/admin/crm/deals/${id}/lose`, { lost_reason: lostReason });
	}

	async getDealForecast(period: string = 'this_month'): Promise<DealForecast> {
		return apiClient.get('/admin/crm/deals/forecast', { params: { period } });
	}

	// Pipelines
	async getPipelines(): Promise<Pipeline[]> {
		return apiClient.get('/admin/crm/pipelines');
	}

	async getPipeline(id: string): Promise<Pipeline> {
		return apiClient.get(`/admin/crm/pipelines/${id}`);
	}

	async createPipeline(data: Partial<Pipeline>): Promise<Pipeline> {
		return apiClient.post('/admin/crm/pipelines', data);
	}

	async updatePipeline(id: string, data: Partial<Pipeline>): Promise<Pipeline> {
		return apiClient.put(`/admin/crm/pipelines/${id}`, data);
	}

	async deletePipeline(id: string): Promise<void> {
		return apiClient.delete(`/admin/crm/pipelines/${id}`);
	}

	async addStage(pipelineId: string, data: Partial<Stage>): Promise<Stage> {
		return apiClient.post(`/admin/crm/pipelines/${pipelineId}/stages`, data);
	}

	// =====================================================
	// FLUENTCRM PRO - EMAIL SEQUENCES (Drip Campaigns)
	// =====================================================

	async getSequences(filters?: SequenceFilters): Promise<{ data: EmailSequence[]; meta: any }> {
		return apiClient.get('/admin/crm/sequences', { params: filters });
	}

	async getSequence(id: string): Promise<{ sequence: EmailSequence; stats: SequenceStats }> {
		return apiClient.get(`/admin/crm/sequences/${id}`);
	}

	async createSequence(data: Partial<EmailSequence>): Promise<EmailSequence> {
		return apiClient.post('/admin/crm/sequences', data);
	}

	async updateSequence(id: string, data: Partial<EmailSequence>): Promise<EmailSequence> {
		return apiClient.put(`/admin/crm/sequences/${id}`, data);
	}

	async deleteSequence(id: string): Promise<void> {
		return apiClient.delete(`/admin/crm/sequences/${id}`);
	}

	async duplicateSequence(id: string): Promise<EmailSequence> {
		return apiClient.post(`/admin/crm/sequences/${id}/duplicate`);
	}

	async getSequenceEmails(sequenceId: string): Promise<SequenceMail[]> {
		return apiClient.get(`/admin/crm/sequences/${sequenceId}/emails`);
	}

	async createSequenceEmail(sequenceId: string, data: Partial<SequenceMail>): Promise<SequenceMail> {
		return apiClient.post(`/admin/crm/sequences/${sequenceId}/emails`, data);
	}

	async updateSequenceEmail(sequenceId: string, emailId: string, data: Partial<SequenceMail>): Promise<SequenceMail> {
		return apiClient.put(`/admin/crm/sequences/${sequenceId}/emails/${emailId}`, data);
	}

	async deleteSequenceEmail(sequenceId: string, emailId: string): Promise<void> {
		return apiClient.delete(`/admin/crm/sequences/${sequenceId}/emails/${emailId}`);
	}

	async reorderSequenceEmails(sequenceId: string, emailIds: string[]): Promise<SequenceMail[]> {
		return apiClient.post(`/admin/crm/sequences/${sequenceId}/emails/reorder`, { email_ids: emailIds });
	}

	async getSequenceSubscribers(id: string, filters?: { status?: string; per_page?: number }): Promise<{ data: SequenceTracker[]; meta: any }> {
		return apiClient.get(`/admin/crm/sequences/${id}/subscribers`, { params: filters });
	}

	async subscribeToSequence(id: string, contactIds: string[], startPosition?: number): Promise<{ subscribed_count: number }> {
		return apiClient.post(`/admin/crm/sequences/${id}/subscribe`, {
			contact_ids: contactIds,
			start_position: startPosition
		});
	}

	async unsubscribeFromSequence(id: string, contactIds: string[], reason?: string): Promise<{ unsubscribed_count: number }> {
		return apiClient.post(`/admin/crm/sequences/${id}/unsubscribe`, {
			contact_ids: contactIds,
			reason
		});
	}

	// =====================================================
	// FLUENTCRM PRO - RECURRING CAMPAIGNS
	// =====================================================

	async getRecurringCampaigns(filters?: RecurringCampaignFilters): Promise<{ data: RecurringCampaign[]; meta: any }> {
		return apiClient.get('/admin/crm/recurring-campaigns', { params: filters });
	}

	async getRecurringCampaign(id: string): Promise<{ campaign: RecurringCampaign; stats: RecurringCampaignStats }> {
		return apiClient.get(`/admin/crm/recurring-campaigns/${id}`);
	}

	async createRecurringCampaign(data: Partial<RecurringCampaign>): Promise<RecurringCampaign> {
		return apiClient.post('/admin/crm/recurring-campaigns', data);
	}

	async updateRecurringCampaign(id: string, data: Partial<RecurringCampaign>): Promise<RecurringCampaign> {
		return apiClient.put(`/admin/crm/recurring-campaigns/${id}`, data);
	}

	async deleteRecurringCampaign(id: string): Promise<void> {
		return apiClient.delete(`/admin/crm/recurring-campaigns/${id}`);
	}

	async duplicateRecurringCampaign(id: string): Promise<RecurringCampaign> {
		return apiClient.post(`/admin/crm/recurring-campaigns/${id}/duplicate`);
	}

	async getRecurringCampaignEmails(campaignId: string, filters?: { status?: string; per_page?: number }): Promise<{ data: RecurringMail[]; meta: any }> {
		return apiClient.get(`/admin/crm/recurring-campaigns/${campaignId}/emails`, { params: filters });
	}

	async triggerRecurringCampaign(id: string): Promise<RecurringMail> {
		return apiClient.post(`/admin/crm/recurring-campaigns/${id}/trigger`);
	}

	// =====================================================
	// FLUENTCRM PRO - SMART LINKS
	// =====================================================

	async getSmartLinks(filters?: SmartLinkFilters): Promise<{ data: SmartLink[]; meta: any }> {
		return apiClient.get('/admin/crm/smart-links', { params: filters });
	}

	async getSmartLink(id: string): Promise<{ link: SmartLink; stats: SmartLinkStats }> {
		return apiClient.get(`/admin/crm/smart-links/${id}`);
	}

	async createSmartLink(data: Partial<SmartLink>): Promise<SmartLink> {
		return apiClient.post('/admin/crm/smart-links', data);
	}

	async updateSmartLink(id: string, data: Partial<SmartLink>): Promise<SmartLink> {
		return apiClient.put(`/admin/crm/smart-links/${id}`, data);
	}

	async deleteSmartLink(id: string): Promise<void> {
		return apiClient.delete(`/admin/crm/smart-links/${id}`);
	}

	async getSmartLinkClicks(id: string, filters?: { per_page?: number }): Promise<{ data: SmartLinkClick[]; meta: any }> {
		return apiClient.get(`/admin/crm/smart-links/${id}/clicks`, { params: filters });
	}

	async getSmartLinkAnalytics(id: string): Promise<SmartLinkAnalytics> {
		return apiClient.get(`/admin/crm/smart-links/${id}/analytics`);
	}

	// =====================================================
	// FLUENTCRM PRO - AUTOMATION FUNNELS
	// =====================================================

	async getAutomationFunnels(filters?: FunnelFilters): Promise<{ data: AutomationFunnel[]; meta: any }> {
		return apiClient.get('/admin/crm/automations', { params: filters });
	}

	async getAutomationFunnel(id: string): Promise<{ funnel: AutomationFunnel; stats: FunnelStats; trigger_types: Record<string, string>; action_types: Record<string, string> }> {
		return apiClient.get(`/admin/crm/automations/${id}`);
	}

	async createAutomationFunnel(data: Partial<AutomationFunnel>): Promise<AutomationFunnel> {
		return apiClient.post('/admin/crm/automations', data);
	}

	async updateAutomationFunnel(id: string, data: Partial<AutomationFunnel>): Promise<AutomationFunnel> {
		return apiClient.put(`/admin/crm/automations/${id}`, data);
	}

	async deleteAutomationFunnel(id: string): Promise<void> {
		return apiClient.delete(`/admin/crm/automations/${id}`);
	}

	async duplicateAutomationFunnel(id: string): Promise<AutomationFunnel> {
		return apiClient.post(`/admin/crm/automations/${id}/duplicate`);
	}

	async importAutomationFunnel(data: Record<string, any>): Promise<AutomationFunnel> {
		return apiClient.post('/admin/crm/automations/import', data);
	}

	async exportAutomationFunnel(id: string): Promise<Record<string, any>> {
		return apiClient.get(`/admin/crm/automations/${id}/export`);
	}

	async getTriggerTypes(): Promise<Record<string, string>> {
		return apiClient.get('/admin/crm/automations/trigger-types');
	}

	async getActionTypes(): Promise<Record<string, string>> {
		return apiClient.get('/admin/crm/automations/action-types');
	}

	async getFunnelActions(funnelId: string): Promise<FunnelAction[]> {
		return apiClient.get(`/admin/crm/automations/${funnelId}/actions`);
	}

	async createFunnelAction(funnelId: string, data: Partial<FunnelAction>): Promise<FunnelAction> {
		return apiClient.post(`/admin/crm/automations/${funnelId}/actions`, data);
	}

	async updateFunnelAction(funnelId: string, actionId: string, data: Partial<FunnelAction>): Promise<FunnelAction> {
		return apiClient.put(`/admin/crm/automations/${funnelId}/actions/${actionId}`, data);
	}

	async deleteFunnelAction(funnelId: string, actionId: string): Promise<void> {
		return apiClient.delete(`/admin/crm/automations/${funnelId}/actions/${actionId}`);
	}

	async reorderFunnelActions(funnelId: string, actionIds: string[]): Promise<FunnelAction[]> {
		return apiClient.post(`/admin/crm/automations/${funnelId}/actions/reorder`, { action_ids: actionIds });
	}

	async getFunnelSubscribers(funnelId: string, filters?: { status?: string; per_page?: number }): Promise<{ data: FunnelSubscriber[]; meta: any }> {
		return apiClient.get(`/admin/crm/automations/${funnelId}/subscribers`, { params: filters });
	}

	async addToFunnel(funnelId: string, contactIds: string[]): Promise<{ added_count: number }> {
		return apiClient.post(`/admin/crm/automations/${funnelId}/add-contacts`, { contact_ids: contactIds });
	}

	async removeFromFunnel(funnelId: string, contactIds: string[]): Promise<{ removed_count: number }> {
		return apiClient.post(`/admin/crm/automations/${funnelId}/remove-contacts`, { contact_ids: contactIds });
	}

	// =====================================================
	// FLUENTCRM PRO - CONTACT LISTS
	// =====================================================

	async getContactLists(filters?: { search?: string; is_public?: boolean; per_page?: number }): Promise<{ data: ContactList[]; meta: any }> {
		return apiClient.get('/admin/crm/lists', { params: filters });
	}

	async getContactList(id: string): Promise<{ list: ContactList; contacts_count: number }> {
		return apiClient.get(`/admin/crm/lists/${id}`);
	}

	async createContactList(data: Partial<ContactList>): Promise<ContactList> {
		return apiClient.post('/admin/crm/lists', data);
	}

	async updateContactList(id: string, data: Partial<ContactList>): Promise<ContactList> {
		return apiClient.put(`/admin/crm/lists/${id}`, data);
	}

	async deleteContactList(id: string): Promise<void> {
		return apiClient.delete(`/admin/crm/lists/${id}`);
	}

	async getListContacts(id: string, filters?: { per_page?: number }): Promise<{ data: Contact[]; meta: any }> {
		return apiClient.get(`/admin/crm/lists/${id}/contacts`, { params: filters });
	}

	async addContactsToList(id: string, contactIds: string[]): Promise<{ added_count: number }> {
		return apiClient.post(`/admin/crm/lists/${id}/contacts`, { contact_ids: contactIds });
	}

	async removeContactsFromList(id: string, contactIds: string[]): Promise<{ removed_count: number }> {
		return apiClient.delete(`/admin/crm/lists/${id}/contacts`, { data: { contact_ids: contactIds } });
	}

	// =====================================================
	// FLUENTCRM PRO - CONTACT TAGS
	// =====================================================

	async getContactTags(filters?: { search?: string; per_page?: number }): Promise<{ data: ContactTag[]; meta: any }> {
		return apiClient.get('/admin/crm/contact-tags', { params: filters });
	}

	async getContactTag(id: string): Promise<{ tag: ContactTag; contacts_count: number }> {
		return apiClient.get(`/admin/crm/contact-tags/${id}`);
	}

	async createContactTag(data: Partial<ContactTag>): Promise<ContactTag> {
		return apiClient.post('/admin/crm/contact-tags', data);
	}

	async updateContactTag(id: string, data: Partial<ContactTag>): Promise<ContactTag> {
		return apiClient.put(`/admin/crm/contact-tags/${id}`, data);
	}

	async deleteContactTag(id: string): Promise<void> {
		return apiClient.delete(`/admin/crm/contact-tags/${id}`);
	}

	async getTagContacts(id: string, filters?: { per_page?: number }): Promise<{ data: Contact[]; meta: any }> {
		return apiClient.get(`/admin/crm/contact-tags/${id}/contacts`, { params: filters });
	}

	async applyTagToContacts(id: string, contactIds: string[]): Promise<{ applied_count: number }> {
		return apiClient.post(`/admin/crm/contact-tags/${id}/apply`, { contact_ids: contactIds });
	}

	async removeTagFromContacts(id: string, contactIds: string[]): Promise<{ removed_count: number }> {
		return apiClient.post(`/admin/crm/contact-tags/${id}/remove`, { contact_ids: contactIds });
	}

	async bulkApplyTags(tagIds: string[], contactIds: string[]): Promise<{ applied_count: number }> {
		return apiClient.post('/admin/crm/contact-tags/bulk-apply', { tag_ids: tagIds, contact_ids: contactIds });
	}

	// =====================================================
	// FLUENTCRM PRO - CRM COMPANIES (B2B)
	// =====================================================

	async getCompanies(filters?: CompanyFilters): Promise<{ data: CrmCompany[]; meta: any }> {
		return apiClient.get('/admin/crm/companies', { params: filters });
	}

	async getCompany(id: string): Promise<{ company: CrmCompany; stats: CompanyStats; industries: Record<string, string>; sizes: Record<string, string> }> {
		return apiClient.get(`/admin/crm/companies/${id}`);
	}

	async createCompany(data: Partial<CrmCompany>): Promise<CrmCompany> {
		return apiClient.post('/admin/crm/companies', data);
	}

	async updateCompany(id: string, data: Partial<CrmCompany>): Promise<CrmCompany> {
		return apiClient.put(`/admin/crm/companies/${id}`, data);
	}

	async deleteCompany(id: string): Promise<void> {
		return apiClient.delete(`/admin/crm/companies/${id}`);
	}

	async getCompanyContacts(id: string, filters?: { per_page?: number }): Promise<{ data: Contact[]; meta: any }> {
		return apiClient.get(`/admin/crm/companies/${id}/contacts`, { params: filters });
	}

	async getCompanyDeals(id: string, filters?: { status?: string; per_page?: number }): Promise<{ data: Deal[]; meta: any }> {
		return apiClient.get(`/admin/crm/companies/${id}/deals`, { params: filters });
	}

	async getIndustries(): Promise<Record<string, string>> {
		return apiClient.get('/admin/crm/companies/industries');
	}

	async getCompanySizes(): Promise<Record<string, string>> {
		return apiClient.get('/admin/crm/companies/sizes');
	}
}

export const crmAPI = new CrmAPI();

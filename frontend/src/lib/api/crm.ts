/**
 * RevolutionCRM-L8-System - API Client
 */

import type { 
  Contact, Deal, Pipeline, Stage, Activity, Note, ContactSegment, 
  TimelineEvent, DealForecast, ContactFilters, DealFilters 
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
    return apiClient.post(`/admin/crm/deals/${id}/win`, { won_details: wonDetails, close_date: closeDate });
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
}

export const crmAPI = new CrmAPI();

import { apiClient } from './client';
import type {
	Workflow,
	WorkflowNode,
	WorkflowEdge,
	WorkflowRun,
	WorkflowAnalytics
} from '$lib/types/workflow';

export const workflowApi = {
	// Workflows
	async getWorkflows(): Promise<Workflow[]> {
		const response = (await apiClient.get('/workflows')) as { data: Workflow[] };
		return response.data;
	},

	async getWorkflow(id: number): Promise<Workflow> {
		const response = (await apiClient.get(`/workflows/${id}`)) as { data: Workflow };
		return response.data;
	},

	async createWorkflow(data: Partial<Workflow>): Promise<Workflow> {
		const response = (await apiClient.post('/workflows', data)) as { data: Workflow };
		return response.data;
	},

	async updateWorkflow(id: number, data: Partial<Workflow>): Promise<Workflow> {
		const response = (await apiClient.put(`/workflows/${id}`, data)) as { data: Workflow };
		return response.data;
	},

	async deleteWorkflow(id: number): Promise<void> {
		await apiClient.delete(`/workflows/${id}`);
	},

	async toggleWorkflowStatus(id: number, status: 'active' | 'paused'): Promise<Workflow> {
		const response = (await apiClient.put(`/workflows/${id}/status`, { status })) as {
			data: Workflow;
		};
		return response.data;
	},

	// Nodes
	async getWorkflowNodes(workflowId: number): Promise<WorkflowNode[]> {
		const response = (await apiClient.get(`/workflows/${workflowId}/nodes`)) as {
			data: WorkflowNode[];
		};
		return response.data;
	},

	async createNode(workflowId: number, data: Partial<WorkflowNode>): Promise<WorkflowNode> {
		const response = (await apiClient.post(`/workflows/${workflowId}/nodes`, data)) as {
			data: WorkflowNode;
		};
		return response.data;
	},

	async updateNode(
		workflowId: number,
		nodeId: number,
		data: Partial<WorkflowNode>
	): Promise<WorkflowNode> {
		const response = (await apiClient.put(`/workflows/${workflowId}/nodes/${nodeId}`, data)) as {
			data: WorkflowNode;
		};
		return response.data;
	},

	async deleteNode(workflowId: number, nodeId: number): Promise<void> {
		await apiClient.delete(`/workflows/${workflowId}/nodes/${nodeId}`);
	},

	// Edges
	async getWorkflowEdges(workflowId: number): Promise<WorkflowEdge[]> {
		const response = (await apiClient.get(`/workflows/${workflowId}/edges`)) as {
			data: WorkflowEdge[];
		};
		return response.data;
	},

	async createEdge(workflowId: number, data: Partial<WorkflowEdge>): Promise<WorkflowEdge> {
		const response = (await apiClient.post(`/workflows/${workflowId}/edges`, data)) as {
			data: WorkflowEdge;
		};
		return response.data;
	},

	async deleteEdge(workflowId: number, edgeId: number): Promise<void> {
		await apiClient.delete(`/workflows/${workflowId}/edges/${edgeId}`);
	},

	// Execution
	async executeWorkflow(id: number, triggerData?: Record<string, any>): Promise<WorkflowRun> {
		const response = (await apiClient.post(`/workflows/${id}/execute`, {
			trigger_data: triggerData
		})) as { data: WorkflowRun };
		return response.data;
	},

	async getWorkflowRuns(workflowId: number, limit = 50): Promise<WorkflowRun[]> {
		const response = (await apiClient.get(`/workflows/${workflowId}/runs?limit=${limit}`)) as {
			data: WorkflowRun[];
		};
		return response.data;
	},

	async getWorkflowRun(workflowId: number, runId: number): Promise<WorkflowRun> {
		const response = (await apiClient.get(`/workflows/${workflowId}/runs/${runId}`)) as {
			data: WorkflowRun;
		};
		return response.data;
	},

	// Analytics
	async getWorkflowAnalytics(workflowId: number): Promise<WorkflowAnalytics> {
		const response = (await apiClient.get(`/workflows/${workflowId}/analytics`)) as {
			data: WorkflowAnalytics;
		};
		return response.data;
	},

	// Templates
	async getWorkflowTemplates(): Promise<Workflow[]> {
		const response = (await apiClient.get('/workflows/templates')) as { data: Workflow[] };
		return response.data;
	},

	async createFromTemplate(templateId: number, name: string): Promise<Workflow> {
		const response = (await apiClient.post('/workflows/from-template', {
			template_id: templateId,
			name
		})) as { data: Workflow };
		return response.data;
	}
};

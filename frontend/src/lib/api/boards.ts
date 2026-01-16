/**
 * Fluent Boards Pro - API Client
 * Complete project management API
 */

import type {
	Board,
	BoardFilters,
	BoardSettings,
	BoardMember,
	BoardRole,
	BoardTemplate,
	BoardWebhook,
	BoardReport,
	BoardsSettings,
	Stage,
	Task,
	TaskFilters,
	Subtask,
	Checklist,
	ChecklistItem,
	Label,
	CustomFieldDefinition,
	CustomFieldValue,
	Attachment,
	Comment,
	TimeEntry,
	TimeEntryFilters,
	TimeTrackingStats,
	Folder,
	Activity,
	ActivityFilters,
	ImportJob,
	ImportMapping,
	ReportPeriod,
	StorageConfig,
	WebhookEvent
} from '$lib/boards/types';
import { apiClient } from './client.svelte';

export class BoardsAPI {
	// =====================================================
	// BOARDS
	// =====================================================

	async getBoards(filters?: BoardFilters): Promise<{ data: Board[]; meta: any }> {
		return apiClient.get('/admin/boards', { params: filters });
	}

	async getBoard(id: string): Promise<{ board: Board; members: BoardMember[]; stats: any }> {
		return apiClient.get(`/admin/boards/${id}`);
	}

	async createBoard(data: Partial<Board>): Promise<Board> {
		return apiClient.post('/admin/boards', data);
	}

	async updateBoard(id: string, data: Partial<Board>): Promise<Board> {
		return apiClient.put(`/admin/boards/${id}`, data);
	}

	async deleteBoard(id: string): Promise<void> {
		return apiClient.delete(`/admin/boards/${id}`);
	}

	async duplicateBoard(id: string, title?: string): Promise<Board> {
		return apiClient.post(`/admin/boards/${id}/duplicate`, { title });
	}

	async archiveBoard(id: string): Promise<Board> {
		return apiClient.post(`/admin/boards/${id}/archive`);
	}

	async restoreBoard(id: string): Promise<Board> {
		return apiClient.post(`/admin/boards/${id}/restore`);
	}

	async toggleFavorite(id: string): Promise<Board> {
		return apiClient.post(`/admin/boards/${id}/favorite`);
	}

	async updateBoardSettings(id: string, settings: Partial<BoardSettings>): Promise<Board> {
		return apiClient.put(`/admin/boards/${id}/settings`, settings);
	}

	async getBoardActivity(id: string, filters?: ActivityFilters): Promise<{ data: Activity[]; meta: any }> {
		return apiClient.get(`/admin/boards/${id}/activity`, { params: filters });
	}

	// =====================================================
	// BOARD MEMBERS
	// =====================================================

	async getBoardMembers(boardId: string): Promise<BoardMember[]> {
		return apiClient.get(`/admin/boards/${boardId}/members`);
	}

	async addBoardMember(boardId: string, userId: string, role: BoardRole): Promise<BoardMember> {
		return apiClient.post(`/admin/boards/${boardId}/members`, { user_id: userId, role });
	}

	async updateBoardMemberRole(boardId: string, memberId: string, role: BoardRole): Promise<BoardMember> {
		return apiClient.put(`/admin/boards/${boardId}/members/${memberId}`, { role });
	}

	async removeBoardMember(boardId: string, memberId: string): Promise<void> {
		return apiClient.delete(`/admin/boards/${boardId}/members/${memberId}`);
	}

	async inviteMemberByEmail(boardId: string, email: string, role: BoardRole): Promise<{ invitation_sent: boolean }> {
		return apiClient.post(`/admin/boards/${boardId}/invite`, { email, role });
	}

	// =====================================================
	// STAGES
	// =====================================================

	async getStages(boardId: string): Promise<Stage[]> {
		return apiClient.get(`/admin/boards/${boardId}/stages`);
	}

	async createStage(boardId: string, data: Partial<Stage>): Promise<Stage> {
		return apiClient.post(`/admin/boards/${boardId}/stages`, data);
	}

	async updateStage(boardId: string, stageId: string, data: Partial<Stage>): Promise<Stage> {
		return apiClient.put(`/admin/boards/${boardId}/stages/${stageId}`, data);
	}

	async deleteStage(boardId: string, stageId: string, moveTasksTo?: string): Promise<void> {
		return apiClient.delete(`/admin/boards/${boardId}/stages/${stageId}`, {
			params: { move_tasks_to: moveTasksTo }
		});
	}

	async reorderStages(boardId: string, stageIds: string[]): Promise<Stage[]> {
		return apiClient.post(`/admin/boards/${boardId}/stages/reorder`, { stage_ids: stageIds });
	}

	async toggleStageCollapse(boardId: string, stageId: string): Promise<Stage> {
		return apiClient.post(`/admin/boards/${boardId}/stages/${stageId}/toggle-collapse`);
	}

	// =====================================================
	// TASKS
	// =====================================================

	async getTasks(boardId: string, filters?: TaskFilters): Promise<{ data: Task[]; meta: any }> {
		return apiClient.get(`/admin/boards/${boardId}/tasks`, { params: filters });
	}

	async getTask(boardId: string, taskId: string): Promise<Task> {
		return apiClient.get(`/admin/boards/${boardId}/tasks/${taskId}`);
	}

	async createTask(boardId: string, data: Partial<Task>): Promise<Task> {
		return apiClient.post(`/admin/boards/${boardId}/tasks`, data);
	}

	async updateTask(boardId: string, taskId: string, data: Partial<Task>): Promise<Task> {
		return apiClient.put(`/admin/boards/${boardId}/tasks/${taskId}`, data);
	}

	async deleteTask(boardId: string, taskId: string): Promise<void> {
		return apiClient.delete(`/admin/boards/${boardId}/tasks/${taskId}`);
	}

	async moveTask(boardId: string, taskId: string, stageId: string, position: number): Promise<Task> {
		return apiClient.post(`/admin/boards/${boardId}/tasks/${taskId}/move`, {
			stage_id: stageId,
			position
		});
	}

	async duplicateTask(boardId: string, taskId: string): Promise<Task> {
		return apiClient.post(`/admin/boards/${boardId}/tasks/${taskId}/duplicate`);
	}

	async archiveTask(boardId: string, taskId: string): Promise<Task> {
		return apiClient.post(`/admin/boards/${boardId}/tasks/${taskId}/archive`);
	}

	async restoreTask(boardId: string, taskId: string): Promise<Task> {
		return apiClient.post(`/admin/boards/${boardId}/tasks/${taskId}/restore`);
	}

	async completeTask(boardId: string, taskId: string): Promise<Task> {
		return apiClient.post(`/admin/boards/${boardId}/tasks/${taskId}/complete`);
	}

	async reopenTask(boardId: string, taskId: string): Promise<Task> {
		return apiClient.post(`/admin/boards/${boardId}/tasks/${taskId}/reopen`);
	}

	async bulkUpdateTasks(boardId: string, taskIds: string[], data: Partial<Task>): Promise<{ updated_count: number }> {
		return apiClient.post(`/admin/boards/${boardId}/tasks/bulk-update`, {
			task_ids: taskIds,
			...data
		});
	}

	async bulkDeleteTasks(boardId: string, taskIds: string[]): Promise<{ deleted_count: number }> {
		return apiClient.post(`/admin/boards/${boardId}/tasks/bulk-delete`, { task_ids: taskIds });
	}

	async bulkMoveTasks(boardId: string, taskIds: string[], stageId: string): Promise<{ moved_count: number }> {
		return apiClient.post(`/admin/boards/${boardId}/tasks/bulk-move`, {
			task_ids: taskIds,
			stage_id: stageId
		});
	}

	// Task Assignees
	async assignTask(boardId: string, taskId: string, userIds: string[]): Promise<Task> {
		return apiClient.post(`/admin/boards/${boardId}/tasks/${taskId}/assign`, { user_ids: userIds });
	}

	async unassignTask(boardId: string, taskId: string, userIds: string[]): Promise<Task> {
		return apiClient.post(`/admin/boards/${boardId}/tasks/${taskId}/unassign`, { user_ids: userIds });
	}

	// Task Labels
	async addLabelToTask(boardId: string, taskId: string, labelId: string): Promise<Task> {
		return apiClient.post(`/admin/boards/${boardId}/tasks/${taskId}/labels/${labelId}`);
	}

	async removeLabelFromTask(boardId: string, taskId: string, labelId: string): Promise<Task> {
		return apiClient.delete(`/admin/boards/${boardId}/tasks/${taskId}/labels/${labelId}`);
	}

	// Task Watchers
	async watchTask(boardId: string, taskId: string): Promise<Task> {
		return apiClient.post(`/admin/boards/${boardId}/tasks/${taskId}/watch`);
	}

	async unwatchTask(boardId: string, taskId: string): Promise<Task> {
		return apiClient.delete(`/admin/boards/${boardId}/tasks/${taskId}/watch`);
	}

	// =====================================================
	// SUBTASKS
	// =====================================================

	async getSubtasks(boardId: string, taskId: string): Promise<Subtask[]> {
		return apiClient.get(`/admin/boards/${boardId}/tasks/${taskId}/subtasks`);
	}

	async createSubtask(boardId: string, taskId: string, data: Partial<Subtask>): Promise<Subtask> {
		return apiClient.post(`/admin/boards/${boardId}/tasks/${taskId}/subtasks`, data);
	}

	async updateSubtask(boardId: string, taskId: string, subtaskId: string, data: Partial<Subtask>): Promise<Subtask> {
		return apiClient.put(`/admin/boards/${boardId}/tasks/${taskId}/subtasks/${subtaskId}`, data);
	}

	async deleteSubtask(boardId: string, taskId: string, subtaskId: string): Promise<void> {
		return apiClient.delete(`/admin/boards/${boardId}/tasks/${taskId}/subtasks/${subtaskId}`);
	}

	async toggleSubtaskComplete(boardId: string, taskId: string, subtaskId: string): Promise<Subtask> {
		return apiClient.post(`/admin/boards/${boardId}/tasks/${taskId}/subtasks/${subtaskId}/toggle`);
	}

	async reorderSubtasks(boardId: string, taskId: string, subtaskIds: string[]): Promise<Subtask[]> {
		return apiClient.post(`/admin/boards/${boardId}/tasks/${taskId}/subtasks/reorder`, {
			subtask_ids: subtaskIds
		});
	}

	async convertSubtaskToTask(boardId: string, taskId: string, subtaskId: string): Promise<Task> {
		return apiClient.post(`/admin/boards/${boardId}/tasks/${taskId}/subtasks/${subtaskId}/convert`);
	}

	// =====================================================
	// CHECKLISTS
	// =====================================================

	async getChecklists(boardId: string, taskId: string): Promise<Checklist[]> {
		return apiClient.get(`/admin/boards/${boardId}/tasks/${taskId}/checklists`);
	}

	async createChecklist(boardId: string, taskId: string, title: string): Promise<Checklist> {
		return apiClient.post(`/admin/boards/${boardId}/tasks/${taskId}/checklists`, { title });
	}

	async updateChecklist(boardId: string, taskId: string, checklistId: string, title: string): Promise<Checklist> {
		return apiClient.put(`/admin/boards/${boardId}/tasks/${taskId}/checklists/${checklistId}`, { title });
	}

	async deleteChecklist(boardId: string, taskId: string, checklistId: string): Promise<void> {
		return apiClient.delete(`/admin/boards/${boardId}/tasks/${taskId}/checklists/${checklistId}`);
	}

	async addChecklistItem(boardId: string, taskId: string, checklistId: string, title: string): Promise<ChecklistItem> {
		return apiClient.post(`/admin/boards/${boardId}/tasks/${taskId}/checklists/${checklistId}/items`, { title });
	}

	async updateChecklistItem(boardId: string, taskId: string, checklistId: string, itemId: string, data: Partial<ChecklistItem>): Promise<ChecklistItem> {
		return apiClient.put(`/admin/boards/${boardId}/tasks/${taskId}/checklists/${checklistId}/items/${itemId}`, data);
	}

	async deleteChecklistItem(boardId: string, taskId: string, checklistId: string, itemId: string): Promise<void> {
		return apiClient.delete(`/admin/boards/${boardId}/tasks/${taskId}/checklists/${checklistId}/items/${itemId}`);
	}

	async toggleChecklistItem(boardId: string, taskId: string, checklistId: string, itemId: string): Promise<ChecklistItem> {
		return apiClient.post(`/admin/boards/${boardId}/tasks/${taskId}/checklists/${checklistId}/items/${itemId}/toggle`);
	}

	// =====================================================
	// LABELS
	// =====================================================

	async getLabels(boardId: string): Promise<Label[]> {
		return apiClient.get(`/admin/boards/${boardId}/labels`);
	}

	async createLabel(boardId: string, data: Partial<Label>): Promise<Label> {
		return apiClient.post(`/admin/boards/${boardId}/labels`, data);
	}

	async updateLabel(boardId: string, labelId: string, data: Partial<Label>): Promise<Label> {
		return apiClient.put(`/admin/boards/${boardId}/labels/${labelId}`, data);
	}

	async deleteLabel(boardId: string, labelId: string): Promise<void> {
		return apiClient.delete(`/admin/boards/${boardId}/labels/${labelId}`);
	}

	// =====================================================
	// CUSTOM FIELDS
	// =====================================================

	async getCustomFields(boardId: string): Promise<CustomFieldDefinition[]> {
		return apiClient.get(`/admin/boards/${boardId}/custom-fields`);
	}

	async createCustomField(boardId: string, data: Partial<CustomFieldDefinition>): Promise<CustomFieldDefinition> {
		return apiClient.post(`/admin/boards/${boardId}/custom-fields`, data);
	}

	async updateCustomField(boardId: string, fieldId: string, data: Partial<CustomFieldDefinition>): Promise<CustomFieldDefinition> {
		return apiClient.put(`/admin/boards/${boardId}/custom-fields/${fieldId}`, data);
	}

	async deleteCustomField(boardId: string, fieldId: string): Promise<void> {
		return apiClient.delete(`/admin/boards/${boardId}/custom-fields/${fieldId}`);
	}

	async reorderCustomFields(boardId: string, fieldIds: string[]): Promise<CustomFieldDefinition[]> {
		return apiClient.post(`/admin/boards/${boardId}/custom-fields/reorder`, { field_ids: fieldIds });
	}

	async setCustomFieldValue(boardId: string, taskId: string, fieldId: string, value: any): Promise<CustomFieldValue> {
		return apiClient.post(`/admin/boards/${boardId}/tasks/${taskId}/custom-fields/${fieldId}`, { value });
	}

	// =====================================================
	// ATTACHMENTS
	// =====================================================

	async getAttachments(boardId: string, taskId: string): Promise<Attachment[]> {
		return apiClient.get(`/admin/boards/${boardId}/tasks/${taskId}/attachments`);
	}

	async uploadAttachment(boardId: string, taskId: string, file: File): Promise<Attachment> {
		const formData = new FormData();
		formData.append('file', file);
		return apiClient.post(`/admin/boards/${boardId}/tasks/${taskId}/attachments`, formData, {
			headers: { 'Content-Type': 'multipart/form-data' }
		});
	}

	async deleteAttachment(boardId: string, taskId: string, attachmentId: string): Promise<void> {
		return apiClient.delete(`/admin/boards/${boardId}/tasks/${taskId}/attachments/${attachmentId}`);
	}

	async setCoverImage(boardId: string, taskId: string, attachmentId: string): Promise<Task> {
		return apiClient.post(`/admin/boards/${boardId}/tasks/${taskId}/cover`, { attachment_id: attachmentId });
	}

	async removeCoverImage(boardId: string, taskId: string): Promise<Task> {
		return apiClient.delete(`/admin/boards/${boardId}/tasks/${taskId}/cover`);
	}

	// =====================================================
	// COMMENTS
	// =====================================================

	async getComments(boardId: string, taskId: string): Promise<Comment[]> {
		return apiClient.get(`/admin/boards/${boardId}/tasks/${taskId}/comments`);
	}

	async createComment(boardId: string, taskId: string, content: string, parentId?: string): Promise<Comment> {
		return apiClient.post(`/admin/boards/${boardId}/tasks/${taskId}/comments`, {
			content,
			parent_id: parentId
		});
	}

	async updateComment(boardId: string, taskId: string, commentId: string, content: string): Promise<Comment> {
		return apiClient.put(`/admin/boards/${boardId}/tasks/${taskId}/comments/${commentId}`, { content });
	}

	async deleteComment(boardId: string, taskId: string, commentId: string): Promise<void> {
		return apiClient.delete(`/admin/boards/${boardId}/tasks/${taskId}/comments/${commentId}`);
	}

	async addReaction(boardId: string, taskId: string, commentId: string, emoji: string): Promise<Comment> {
		return apiClient.post(`/admin/boards/${boardId}/tasks/${taskId}/comments/${commentId}/reactions`, { emoji });
	}

	async removeReaction(boardId: string, taskId: string, commentId: string, emoji: string): Promise<Comment> {
		return apiClient.delete(`/admin/boards/${boardId}/tasks/${taskId}/comments/${commentId}/reactions/${emoji}`);
	}

	// =====================================================
	// TIME TRACKING
	// =====================================================

	async getTimeEntries(filters?: TimeEntryFilters): Promise<{ data: TimeEntry[]; meta: any }> {
		return apiClient.get('/admin/boards/time-entries', { params: filters });
	}

	async getTaskTimeEntries(boardId: string, taskId: string): Promise<TimeEntry[]> {
		return apiClient.get(`/admin/boards/${boardId}/tasks/${taskId}/time-entries`);
	}

	async createTimeEntry(boardId: string, taskId: string, data: Partial<TimeEntry>): Promise<TimeEntry> {
		return apiClient.post(`/admin/boards/${boardId}/tasks/${taskId}/time-entries`, data);
	}

	async updateTimeEntry(boardId: string, taskId: string, entryId: string, data: Partial<TimeEntry>): Promise<TimeEntry> {
		return apiClient.put(`/admin/boards/${boardId}/tasks/${taskId}/time-entries/${entryId}`, data);
	}

	async deleteTimeEntry(boardId: string, taskId: string, entryId: string): Promise<void> {
		return apiClient.delete(`/admin/boards/${boardId}/tasks/${taskId}/time-entries/${entryId}`);
	}

	async startTimer(boardId: string, taskId: string): Promise<TimeEntry> {
		return apiClient.post(`/admin/boards/${boardId}/tasks/${taskId}/timer/start`);
	}

	async stopTimer(boardId: string, taskId: string): Promise<TimeEntry> {
		return apiClient.post(`/admin/boards/${boardId}/tasks/${taskId}/timer/stop`);
	}

	async getActiveTimer(): Promise<TimeEntry | null> {
		return apiClient.get('/admin/boards/timer/active');
	}

	async getTimeTrackingStats(boardId: string, dateFrom?: string, dateTo?: string): Promise<TimeTrackingStats> {
		return apiClient.get(`/admin/boards/${boardId}/time-tracking/stats`, {
			params: { date_from: dateFrom, date_to: dateTo }
		});
	}

	// =====================================================
	// FOLDERS
	// =====================================================

	async getFolders(): Promise<Folder[]> {
		return apiClient.get('/admin/boards/folders');
	}

	async getFolder(id: string): Promise<{ folder: Folder; boards: Board[] }> {
		return apiClient.get(`/admin/boards/folders/${id}`);
	}

	async createFolder(data: Partial<Folder>): Promise<Folder> {
		return apiClient.post('/admin/boards/folders', data);
	}

	async updateFolder(id: string, data: Partial<Folder>): Promise<Folder> {
		return apiClient.put(`/admin/boards/folders/${id}`, data);
	}

	async deleteFolder(id: string, deleteBoardsToo?: boolean): Promise<void> {
		return apiClient.delete(`/admin/boards/folders/${id}`, {
			params: { delete_boards: deleteBoardsToo }
		});
	}

	async moveBoardToFolder(boardId: string, folderId: string | null): Promise<Board> {
		return apiClient.post(`/admin/boards/${boardId}/move-to-folder`, { folder_id: folderId });
	}

	async reorderFolders(folderIds: string[]): Promise<Folder[]> {
		return apiClient.post('/admin/boards/folders/reorder', { folder_ids: folderIds });
	}

	// =====================================================
	// IMPORT/EXPORT
	// =====================================================

	async importFromTrello(file: File): Promise<ImportJob> {
		const formData = new FormData();
		formData.append('file', file);
		formData.append('source', 'trello');
		return apiClient.post('/admin/boards/import', formData, {
			headers: { 'Content-Type': 'multipart/form-data' }
		});
	}

	async importFromAsana(file: File): Promise<ImportJob> {
		const formData = new FormData();
		formData.append('file', file);
		formData.append('source', 'asana');
		return apiClient.post('/admin/boards/import', formData, {
			headers: { 'Content-Type': 'multipart/form-data' }
		});
	}

	async importFromCsv(file: File, boardId?: string): Promise<ImportJob> {
		const formData = new FormData();
		formData.append('file', file);
		formData.append('source', 'csv');
		if (boardId) formData.append('board_id', boardId);
		return apiClient.post('/admin/boards/import', formData, {
			headers: { 'Content-Type': 'multipart/form-data' }
		});
	}

	async getImportJob(id: string): Promise<ImportJob> {
		return apiClient.get(`/admin/boards/import/${id}`);
	}

	async getImportPreview(id: string): Promise<{ headers: string[]; rows: any[]; total_rows: number }> {
		return apiClient.get(`/admin/boards/import/${id}/preview`);
	}

	async startImport(id: string, mapping: ImportMapping): Promise<ImportJob> {
		return apiClient.post(`/admin/boards/import/${id}/start`, { mapping });
	}

	async cancelImport(id: string): Promise<void> {
		return apiClient.post(`/admin/boards/import/${id}/cancel`);
	}

	async exportBoard(boardId: string, format: 'json' | 'csv'): Promise<Blob> {
		return apiClient.get(`/admin/boards/${boardId}/export`, {
			params: { format },
			responseType: 'blob'
		});
	}

	// =====================================================
	// WEBHOOKS
	// =====================================================

	async getWebhooks(boardId: string): Promise<BoardWebhook[]> {
		return apiClient.get(`/admin/boards/${boardId}/webhooks`);
	}

	async createWebhook(boardId: string, data: Partial<BoardWebhook>): Promise<BoardWebhook> {
		return apiClient.post(`/admin/boards/${boardId}/webhooks`, data);
	}

	async updateWebhook(boardId: string, webhookId: string, data: Partial<BoardWebhook>): Promise<BoardWebhook> {
		return apiClient.put(`/admin/boards/${boardId}/webhooks/${webhookId}`, data);
	}

	async deleteWebhook(boardId: string, webhookId: string): Promise<void> {
		return apiClient.delete(`/admin/boards/${boardId}/webhooks/${webhookId}`);
	}

	async testWebhook(boardId: string, webhookId: string): Promise<{ success: boolean; response_code: number }> {
		return apiClient.post(`/admin/boards/${boardId}/webhooks/${webhookId}/test`);
	}

	async getWebhookEvents(): Promise<Record<WebhookEvent, string>> {
		return apiClient.get('/admin/boards/webhooks/events');
	}

	// =====================================================
	// REPORTS
	// =====================================================

	async getBoardReport(boardId: string, period: ReportPeriod, dateFrom?: string, dateTo?: string): Promise<BoardReport> {
		return apiClient.get(`/admin/boards/${boardId}/reports`, {
			params: { period, date_from: dateFrom, date_to: dateTo }
		});
	}

	async getOverallReport(period: ReportPeriod, dateFrom?: string, dateTo?: string): Promise<BoardReport> {
		return apiClient.get('/admin/boards/reports', {
			params: { period, date_from: dateFrom, date_to: dateTo }
		});
	}

	async exportReport(boardId: string, period: ReportPeriod, format: 'pdf' | 'csv'): Promise<Blob> {
		return apiClient.get(`/admin/boards/${boardId}/reports/export`, {
			params: { period, format },
			responseType: 'blob'
		});
	}

	// =====================================================
	// TEMPLATES
	// =====================================================

	async getTemplates(): Promise<BoardTemplate[]> {
		return apiClient.get('/admin/boards/templates');
	}

	async getTemplate(id: string): Promise<BoardTemplate> {
		return apiClient.get(`/admin/boards/templates/${id}`);
	}

	async createFromTemplate(templateId: string, title: string, folderId?: string): Promise<Board> {
		return apiClient.post(`/admin/boards/templates/${templateId}/use`, {
			title,
			folder_id: folderId
		});
	}

	async saveAsTemplate(boardId: string, title: string, description: string, category: string): Promise<BoardTemplate> {
		return apiClient.post(`/admin/boards/${boardId}/save-as-template`, {
			title,
			description,
			category
		});
	}

	async deleteTemplate(id: string): Promise<void> {
		return apiClient.delete(`/admin/boards/templates/${id}`);
	}

	// =====================================================
	// SETTINGS
	// =====================================================

	async getSettings(): Promise<BoardsSettings> {
		return apiClient.get('/admin/boards/settings');
	}

	async updateSettings(settings: Partial<BoardsSettings>): Promise<BoardsSettings> {
		return apiClient.put('/admin/boards/settings', settings);
	}

	async getStorageConfig(): Promise<StorageConfig | null> {
		return apiClient.get('/admin/boards/settings/storage');
	}

	async updateStorageConfig(config: StorageConfig): Promise<StorageConfig> {
		return apiClient.put('/admin/boards/settings/storage', config);
	}

	async testStorageConfig(config: StorageConfig): Promise<{ success: boolean; message: string }> {
		return apiClient.post('/admin/boards/settings/storage/test', config);
	}

	// =====================================================
	// SEARCH
	// =====================================================

	async searchTasks(query: string, boardId?: string): Promise<Task[]> {
		return apiClient.get('/admin/boards/search/tasks', {
			params: { q: query, board_id: boardId }
		});
	}

	async searchBoards(query: string): Promise<Board[]> {
		return apiClient.get('/admin/boards/search', { params: { q: query } });
	}

	// =====================================================
	// ACTIVITY
	// =====================================================

	async getGlobalActivity(filters?: ActivityFilters): Promise<{ data: Activity[]; meta: any }> {
		return apiClient.get('/admin/boards/activity', { params: filters });
	}

	async getTaskActivity(boardId: string, taskId: string): Promise<Activity[]> {
		return apiClient.get(`/admin/boards/${boardId}/tasks/${taskId}/activity`);
	}

	// =====================================================
	// DASHBOARD STATS
	// =====================================================

	async getDashboardStats(): Promise<{
		total_boards: number;
		total_tasks: number;
		completed_tasks: number;
		overdue_tasks: number;
		my_tasks: number;
		tasks_due_today: number;
		tasks_due_this_week: number;
		recent_activity: Activity[];
	}> {
		return apiClient.get('/admin/boards/dashboard');
	}

	async getMyTasks(filters?: TaskFilters): Promise<{ data: Task[]; meta: any }> {
		return apiClient.get('/admin/boards/my-tasks', { params: filters });
	}

	async getOverdueTasks(): Promise<Task[]> {
		return apiClient.get('/admin/boards/overdue-tasks');
	}

	async getTasksDueToday(): Promise<Task[]> {
		return apiClient.get('/admin/boards/tasks-due-today');
	}

	async getTasksDueThisWeek(): Promise<Task[]> {
		return apiClient.get('/admin/boards/tasks-due-this-week');
	}
}

export const boardsAPI = new BoardsAPI();

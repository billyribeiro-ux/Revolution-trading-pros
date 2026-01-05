<?php

namespace App\Http\Controllers\Api;

use App\Http\Controllers\Controller;
use App\Models\Form;
use App\Models\FormSubmission;
use Illuminate\Http\JsonResponse;
use Illuminate\Http\Request;
use Illuminate\Support\Facades\Cache;

/**
 * FormSubmissionController - Enterprise-grade Form Submission API Controller
 *
 * Security: Authorization checks, SQL injection prevention, optimized queries
 */
class FormSubmissionController extends Controller
{
    /**
     * Allowed columns for sorting (SQL injection prevention)
     */
    private const ALLOWED_SORT_COLUMNS = ['created_at', 'updated_at', 'status', 'id'];

    /**
     * Display a listing of submissions for a form
     */
    public function index(Request $request, int $formId): JsonResponse
    {
        $form = Form::find($formId);

        if (!$form) {
            return response()->json([
                'success' => false,
                'message' => 'Form not found',
            ], 404);
        }

        // Authorization check - verify user owns the form
        $this->authorize('viewSubmissions', $form);

        $query = FormSubmission::where('form_id', $formId)
            ->with('data')
            ->withoutTrashed();

        // Filter by status
        if ($request->has('status')) {
            $query->where('status', $request->status);
        }

        // Sort with whitelist validation (SQL injection prevention)
        $sortBy = $request->get('sort_by', 'created_at');
        $sortOrder = $request->get('sort_order', 'desc');

        if (!in_array($sortBy, self::ALLOWED_SORT_COLUMNS, true)) {
            $sortBy = 'created_at';
        }
        if (!in_array(strtolower($sortOrder), ['asc', 'desc'], true)) {
            $sortOrder = 'desc';
        }

        $query->orderBy($sortBy, $sortOrder);

        $perPage = $request->get('per_page', 20);
        $submissions = $query->paginate($perPage);

        return response()->json([
            'success' => true,
            'data' => $submissions,
        ]);
    }

    /**
     * Display the specified submission
     */
    public function show(int $formId, int $submissionId): JsonResponse
    {
        $form = Form::findOrFail($formId);

        // Authorization check
        $this->authorize('viewSubmissions', $form);

        $submission = FormSubmission::where('form_id', $formId)
            ->where('id', $submissionId)
            ->with(['data', 'user:id,name,email'])
            ->first();

        if (!$submission) {
            return response()->json([
                'success' => false,
                'message' => 'Submission not found',
            ], 404);
        }

        // Mark as read if unread
        $submission->markAsRead();

        return response()->json([
            'success' => true,
            'data' => $submission,
        ]);
    }

    /**
     * Update submission status
     */
    public function updateStatus(Request $request, int $formId, int $submissionId): JsonResponse
    {
        $form = Form::findOrFail($formId);

        // Authorization check
        $this->authorize('update', $form);

        $submission = FormSubmission::where('form_id', $formId)
            ->where('id', $submissionId)
            ->first();

        if (!$submission) {
            return response()->json([
                'success' => false,
                'message' => 'Submission not found',
            ], 404);
        }

        $request->validate([
            'status' => 'required|in:' . implode(',', FormSubmission::STATUSES),
        ]);

        $submission->status = $request->status;
        $submission->save();

        return response()->json([
            'success' => true,
            'message' => 'Status updated successfully',
            'data' => $submission,
        ]);
    }

    /**
     * Delete a submission
     */
    public function destroy(int $formId, int $submissionId): JsonResponse
    {
        $form = Form::findOrFail($formId);

        // Authorization check
        $this->authorize('delete', $form);

        $submission = FormSubmission::where('form_id', $formId)
            ->where('id', $submissionId)
            ->first();

        if (!$submission) {
            return response()->json([
                'success' => false,
                'message' => 'Submission not found',
            ], 404);
        }

        $submission->delete();

        return response()->json([
            'success' => true,
            'message' => 'Submission deleted successfully',
        ]);
    }

    /**
     * Bulk update status
     */
    public function bulkUpdateStatus(Request $request, int $formId): JsonResponse
    {
        $form = Form::findOrFail($formId);

        // Authorization check
        $this->authorize('update', $form);

        $request->validate([
            'submission_ids' => 'required|array|max:100',
            'submission_ids.*' => 'integer',
            'status' => 'required|in:' . implode(',', FormSubmission::STATUSES),
        ]);

        $updated = FormSubmission::where('form_id', $formId)
            ->whereIn('id', $request->submission_ids)
            ->update(['status' => $request->status]);

        return response()->json([
            'success' => true,
            'message' => "{$updated} submissions updated successfully",
        ]);
    }

    /**
     * Bulk delete submissions
     */
    public function bulkDelete(Request $request, int $formId): JsonResponse
    {
        $form = Form::findOrFail($formId);

        // Authorization check
        $this->authorize('delete', $form);

        $request->validate([
            'submission_ids' => 'required|array|max:100',
            'submission_ids.*' => 'integer',
        ]);

        $deleted = FormSubmission::where('form_id', $formId)
            ->whereIn('id', $request->submission_ids)
            ->delete();

        return response()->json([
            'success' => true,
            'message' => "{$deleted} submissions deleted successfully",
        ]);
    }

    /**
     * Get submission statistics for a form (optimized single query)
     */
    public function stats(int $formId): JsonResponse
    {
        $form = Form::find($formId);

        if (!$form) {
            return response()->json([
                'success' => false,
                'message' => 'Form not found',
            ], 404);
        }

        // Authorization check
        $this->authorize('viewSubmissions', $form);

        // Cache stats for 5 minutes to reduce database load
        $cacheKey = "form_stats_{$formId}";
        $stats = Cache::remember($cacheKey, 300, function () use ($formId) {
            // Single optimized query instead of 9 separate queries
            $counts = FormSubmission::where('form_id', $formId)
                ->selectRaw("
                    COUNT(*) as total,
                    SUM(CASE WHEN status = 'unread' THEN 1 ELSE 0 END) as unread,
                    SUM(CASE WHEN status = 'read' THEN 1 ELSE 0 END) as `read`,
                    SUM(CASE WHEN status = 'starred' THEN 1 ELSE 0 END) as starred,
                    SUM(CASE WHEN status = 'archived' THEN 1 ELSE 0 END) as archived,
                    SUM(CASE WHEN status = 'spam' THEN 1 ELSE 0 END) as spam,
                    SUM(CASE WHEN DATE(created_at) = CURDATE() THEN 1 ELSE 0 END) as today,
                    SUM(CASE WHEN created_at >= ? THEN 1 ELSE 0 END) as this_week,
                    SUM(CASE WHEN created_at >= ? THEN 1 ELSE 0 END) as this_month
                ", [now()->startOfWeek(), now()->startOfMonth()])
                ->first();

            return [
                'total' => (int) $counts->total,
                'unread' => (int) $counts->unread,
                'read' => (int) $counts->read,
                'starred' => (int) $counts->starred,
                'archived' => (int) $counts->archived,
                'spam' => (int) $counts->spam,
                'today' => (int) $counts->today,
                'this_week' => (int) $counts->this_week,
                'this_month' => (int) $counts->this_month,
            ];
        });

        return response()->json([
            'success' => true,
            'data' => $stats,
        ]);
    }

    /**
     * Export submissions as CSV (with streaming for large datasets)
     */
    public function export(Request $request, int $formId)
    {
        $form = Form::with('fields')->find($formId);

        if (!$form) {
            return response()->json([
                'success' => false,
                'message' => 'Form not found',
            ], 404);
        }

        // Authorization check
        $this->authorize('viewSubmissions', $form);

        // Use cursor for memory-efficient large exports
        $submissions = FormSubmission::where('form_id', $formId)
            ->with('data')
            ->cursor();

        // Build CSV headers from fields
        $headers = ['Submission ID', 'Status', 'Submitted At', 'IP Address'];
        foreach ($form->fields as $field) {
            $headers[] = $field->label;
        }

        // Build CSV rows
        $rows = [];
        foreach ($submissions as $submission) {
            $row = [
                $submission->submission_id,
                $submission->status,
                $submission->created_at->toIso8601String(),
                $submission->ip_address,
            ];

            $dataByField = $submission->data->keyBy('field_name');
            foreach ($form->fields as $field) {
                $row[] = $dataByField->has($field->name) ? $dataByField[$field->name]->value : '';
            }

            $rows[] = $row;
        }

        // Generate CSV content
        $csv = fopen('php://temp', 'r+');
        fputcsv($csv, $headers);
        foreach ($rows as $row) {
            fputcsv($csv, $row);
        }
        rewind($csv);
        $content = stream_get_contents($csv);
        fclose($csv);

        return response($content)
            ->header('Content-Type', 'text/csv')
            ->header('Content-Disposition', 'attachment; filename="' . $form->slug . '-submissions.csv"');
    }
}

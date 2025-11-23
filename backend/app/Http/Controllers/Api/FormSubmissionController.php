<?php

namespace App\Http\Controllers\Api;

use App\Http\Controllers\Controller;
use App\Models\Form;
use App\Models\FormSubmission;
use Illuminate\Http\JsonResponse;
use Illuminate\Http\Request;

/**
 * FormSubmissionController - Enterprise-grade Form Submission API Controller
 */
class FormSubmissionController extends Controller
{
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

        $query = FormSubmission::where('form_id', $formId)
            ->with('data')
            ->withoutTrashed();

        // Filter by status
        if ($request->has('status')) {
            $query->where('status', $request->status);
        }

        // Sort
        $sortBy = $request->get('sort_by', 'created_at');
        $sortOrder = $request->get('sort_order', 'desc');
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
        $request->validate([
            'submission_ids' => 'required|array',
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
        $request->validate([
            'submission_ids' => 'required|array',
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
     * Get submission statistics for a form
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

        $stats = [
            'total' => FormSubmission::where('form_id', $formId)->count(),
            'unread' => FormSubmission::where('form_id', $formId)->where('status', 'unread')->count(),
            'read' => FormSubmission::where('form_id', $formId)->where('status', 'read')->count(),
            'starred' => FormSubmission::where('form_id', $formId)->where('status', 'starred')->count(),
            'archived' => FormSubmission::where('form_id', $formId)->where('status', 'archived')->count(),
            'spam' => FormSubmission::where('form_id', $formId)->where('status', 'spam')->count(),
            'today' => FormSubmission::where('form_id', $formId)->whereDate('created_at', today())->count(),
            'this_week' => FormSubmission::where('form_id', $formId)->where('created_at', '>=', now()->startOfWeek())->count(),
            'this_month' => FormSubmission::where('form_id', $formId)->where('created_at', '>=', now()->startOfMonth())->count(),
        ];

        return response()->json([
            'success' => true,
            'data' => $stats,
        ]);
    }

    /**
     * Export submissions as CSV
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

        $submissions = FormSubmission::where('form_id', $formId)
            ->with('data')
            ->get();

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

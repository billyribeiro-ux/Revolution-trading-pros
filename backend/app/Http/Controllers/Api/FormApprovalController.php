<?php

declare(strict_types=1);

namespace App\Http\Controllers\Api;

use App\Http\Controllers\Controller;
use App\Models\FormSubmission;
use App\Models\FormApprovalSettings;
use App\Services\Forms\FormSubmissionApprovalService;
use Illuminate\Http\JsonResponse;
use Illuminate\Http\Request;
use Illuminate\Support\Facades\Auth;

/**
 * FormApprovalController - FluentForms 6.1.8 (December 2025)
 *
 * API endpoints for admin approval workflow.
 */
class FormApprovalController extends Controller
{
    public function __construct(
        private readonly FormSubmissionApprovalService $approvalService
    ) {}

    /**
     * Get submissions pending approval
     */
    public function pending(Request $request, int $formId): JsonResponse
    {
        $submissions = $this->approvalService->getPendingSubmissions($formId);

        return response()->json([
            'success' => true,
            'data' => $submissions,
            'count' => $submissions->count(),
        ]);
    }

    /**
     * Get submissions by approval status
     */
    public function byStatus(Request $request, int $formId, string $status): JsonResponse
    {
        $submissions = $this->approvalService->getSubmissionsByStatus($formId, $status);

        return response()->json([
            'success' => true,
            'data' => $submissions,
            'count' => $submissions->count(),
        ]);
    }

    /**
     * Approve a submission
     */
    public function approve(Request $request, int $submissionId): JsonResponse
    {
        $submission = FormSubmission::findOrFail($submissionId);
        $note = $request->input('note');

        $result = $this->approvalService->approve($submission, Auth::user(), $note);

        return response()->json($result, $result['success'] ? 200 : 403);
    }

    /**
     * Reject a submission
     */
    public function reject(Request $request, int $submissionId): JsonResponse
    {
        $submission = FormSubmission::findOrFail($submissionId);
        $note = $request->input('note');

        $result = $this->approvalService->reject($submission, Auth::user(), $note);

        return response()->json($result, $result['success'] ? 200 : 403);
    }

    /**
     * Request revision
     */
    public function requestRevision(Request $request, int $submissionId): JsonResponse
    {
        $submission = FormSubmission::findOrFail($submissionId);
        $note = $request->input('note');

        $result = $this->approvalService->requestRevision($submission, Auth::user(), $note);

        return response()->json($result, $result['success'] ? 200 : 403);
    }

    /**
     * Put on hold
     */
    public function hold(Request $request, int $submissionId): JsonResponse
    {
        $submission = FormSubmission::findOrFail($submissionId);
        $note = $request->input('note');

        $result = $this->approvalService->putOnHold($submission, Auth::user(), $note);

        return response()->json($result, $result['success'] ? 200 : 403);
    }

    /**
     * Reset to pending
     */
    public function reset(Request $request, int $submissionId): JsonResponse
    {
        $submission = FormSubmission::findOrFail($submissionId);
        $note = $request->input('note');

        $result = $this->approvalService->resetToPending($submission, Auth::user(), $note);

        return response()->json($result, $result['success'] ? 200 : 403);
    }

    /**
     * Get approval history
     */
    public function history(int $submissionId): JsonResponse
    {
        $submission = FormSubmission::findOrFail($submissionId);
        $history = $this->approvalService->getApprovalHistory($submission);

        return response()->json([
            'success' => true,
            'data' => $history,
        ]);
    }

    /**
     * Get approval statistics
     */
    public function stats(int $formId): JsonResponse
    {
        $stats = $this->approvalService->getApprovalStats($formId);

        return response()->json([
            'success' => true,
            'data' => $stats,
        ]);
    }

    /**
     * Bulk approve submissions
     */
    public function bulkApprove(Request $request): JsonResponse
    {
        $ids = $request->input('submission_ids', []);
        $note = $request->input('note');

        $result = $this->approvalService->bulkApprove($ids, Auth::user(), $note);

        return response()->json([
            'success' => $result['failed'] === 0,
            'data' => $result,
        ]);
    }

    /**
     * Bulk reject submissions
     */
    public function bulkReject(Request $request): JsonResponse
    {
        $ids = $request->input('submission_ids', []);
        $note = $request->input('note');

        $result = $this->approvalService->bulkReject($ids, Auth::user(), $note);

        return response()->json([
            'success' => $result['failed'] === 0,
            'data' => $result,
        ]);
    }

    /**
     * Get/Update approval settings for a form
     */
    public function settings(Request $request, int $formId): JsonResponse
    {
        if ($request->isMethod('GET')) {
            $settings = FormApprovalSettings::getOrCreate($formId);

            return response()->json([
                'success' => true,
                'data' => $settings,
            ]);
        }

        // Update settings
        $settings = FormApprovalSettings::getOrCreate($formId);
        $settings->update($request->only([
            'approval_required',
            'approver_ids',
            'approval_rules',
            'notify_approvers',
            'notify_submitter',
            'approval_email_template',
            'rejection_email_template',
            'revision_email_template',
            'auto_approve_after_hours',
        ]));

        return response()->json([
            'success' => true,
            'message' => 'Settings updated successfully.',
            'data' => $settings->fresh(),
        ]);
    }
}

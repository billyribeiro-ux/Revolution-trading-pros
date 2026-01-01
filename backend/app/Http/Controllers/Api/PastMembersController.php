<?php

namespace App\Http\Controllers\Api;

use App\Http\Controllers\Controller;
use App\Services\MembershipService;
use Illuminate\Http\Request;
use Illuminate\Http\JsonResponse;

/**
 * PastMembersController - Manages past members list and win-back campaigns.
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 */
class PastMembersController extends Controller
{
    public function __construct(
        protected MembershipService $membershipService
    ) {}

    /**
     * Get paginated list of past members.
     */
    public function index(Request $request): JsonResponse
    {
        $filters = $request->only([
            'plan_id',
            'expired_after',
            'expired_before',
            'days_since_expired',
            'search',
        ]);

        $perPage = $request->input('per_page', 20);
        $pastMembers = $this->membershipService->getPastMembers($filters, $perPage);

        // Transform the data
        $data = $pastMembers->through(function ($user) {
            $lastMembership = $user->memberships->first();

            return [
                'id' => $user->id,
                'name' => $user->name,
                'email' => $user->email,
                'last_membership' => $lastMembership ? [
                    'plan_name' => $lastMembership->plan?->name ?? 'Unknown',
                    'status' => $lastMembership->status,
                    'started_at' => $lastMembership->starts_at?->toISOString(),
                    'expired_at' => $lastMembership->expires_at?->toISOString(),
                    'days_since_expired' => $lastMembership->expires_at?->diffInDays(now()),
                    'cancellation_reason' => $lastMembership->cancellation_reason,
                ] : null,
                'total_memberships' => $user->memberships->count(),
                'created_at' => $user->created_at->toISOString(),
            ];
        });

        return response()->json($data);
    }

    /**
     * Get past members statistics.
     */
    public function stats(): JsonResponse
    {
        $stats = $this->membershipService->getPastMembersStats();
        return response()->json($stats);
    }

    /**
     * Get lifecycle analytics.
     */
    public function analytics(): JsonResponse
    {
        $analytics = $this->membershipService->getLifecycleAnalytics();
        return response()->json($analytics);
    }

    /**
     * Send win-back email to a single past member.
     */
    public function sendWinBack(Request $request, int $userId): JsonResponse
    {
        $request->validate([
            'offer_code' => 'nullable|string|max:20',
            'discount_percent' => 'nullable|integer|min:1|max:100',
            'discount_months' => 'nullable|integer|min:1|max:12',
            'expires_in_days' => 'nullable|integer|min:1|max:30',
        ]);

        $user = \App\Models\User::findOrFail($userId);

        $success = $this->membershipService->sendWinBackEmail(
            $user,
            $request->input('offer_code'),
            $request->input('discount_percent'),
            $request->input('discount_months'),
            $request->input('expires_in_days', 7)
        );

        if ($success) {
            return response()->json([
                'message' => 'Win-back email sent successfully',
                'user_id' => $userId,
            ]);
        }

        return response()->json([
            'message' => 'Failed to send win-back email',
            'user_id' => $userId,
        ], 500);
    }

    /**
     * Send bulk win-back emails.
     */
    public function sendBulkWinBack(Request $request): JsonResponse
    {
        $request->validate([
            'user_ids' => 'required|array|min:1|max:100',
            'user_ids.*' => 'integer|exists:users,id',
            'offer_code' => 'nullable|string|max:20',
            'discount_percent' => 'nullable|integer|min:1|max:100',
            'discount_months' => 'nullable|integer|min:1|max:12',
            'expires_in_days' => 'nullable|integer|min:1|max:30',
        ]);

        $results = $this->membershipService->sendBulkWinBackEmails(
            $request->input('user_ids'),
            $request->input('offer_code'),
            $request->input('discount_percent'),
            $request->input('discount_months'),
            $request->input('expires_in_days', 7)
        );

        return response()->json([
            'message' => "Sent {$results['sent']} win-back emails, {$results['failed']} failed",
            'results' => $results,
        ]);
    }

    /**
     * Send feedback survey to a single past member.
     */
    public function sendSurvey(Request $request, int $userId): JsonResponse
    {
        $request->validate([
            'incentive_description' => 'nullable|string|max:255',
        ]);

        $user = \App\Models\User::findOrFail($userId);

        $incentive = null;
        if ($request->has('incentive_description')) {
            $incentive = (object) [
                'description' => $request->input('incentive_description'),
            ];
        }

        $success = $this->membershipService->sendFeedbackSurvey($user, $incentive);

        if ($success) {
            return response()->json([
                'message' => 'Feedback survey sent successfully',
                'user_id' => $userId,
            ]);
        }

        return response()->json([
            'message' => 'Failed to send feedback survey',
            'user_id' => $userId,
        ], 500);
    }

    /**
     * Send bulk feedback surveys.
     */
    public function sendBulkSurvey(Request $request): JsonResponse
    {
        $request->validate([
            'user_ids' => 'required|array|min:1|max:100',
            'user_ids.*' => 'integer|exists:users,id',
            'incentive_description' => 'nullable|string|max:255',
        ]);

        $incentive = null;
        if ($request->has('incentive_description')) {
            $incentive = (object) [
                'description' => $request->input('incentive_description'),
            ];
        }

        $results = $this->membershipService->sendBulkFeedbackSurveys(
            $request->input('user_ids'),
            $incentive
        );

        return response()->json([
            'message' => "Sent {$results['sent']} feedback surveys, {$results['failed']} failed",
            'results' => $results,
        ]);
    }
}

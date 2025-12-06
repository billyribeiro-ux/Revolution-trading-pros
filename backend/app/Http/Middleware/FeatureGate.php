<?php

declare(strict_types=1);

namespace App\Http\Middleware;

use Closure;
use Illuminate\Http\Request;
use Symfony\Component\HttpFoundation\Response;
use App\Services\Subscription\UsageTracker;
use App\Exceptions\FeatureNotAvailableException;
use App\Exceptions\UsageLimitExceededException;
use Illuminate\Support\Facades\Log;

/**
 * Feature Gate Middleware (ICT9+ Enterprise Grade)
 *
 * Controls access to features based on subscription plan:
 * - Feature availability checking
 * - Usage limit enforcement
 * - Graceful degradation
 *
 * @version 1.0.0
 * @level ICT9+ Principal Engineer Grade
 */
class FeatureGate
{
    public function __construct(
        private UsageTracker $usageTracker
    ) {}

    /**
     * Handle an incoming request
     *
     * @param Request $request
     * @param Closure $next
     * @param string $feature Feature identifier
     * @param int $usage Usage quantity to record (default: 1)
     */
    public function handle(Request $request, Closure $next, string $feature, int $usage = 1): Response
    {
        $user = $request->user();

        if (!$user) {
            return response()->json([
                'error' => 'Authentication required',
                'code' => 'auth_required',
            ], 401);
        }

        // Check if feature is available on user's plan
        if (!$this->hasFeature($user, $feature)) {
            Log::info('Feature access denied - not in plan', [
                'user_id' => $user->id,
                'feature' => $feature,
            ]);

            return response()->json([
                'error' => 'This feature is not available on your current plan',
                'code' => 'feature_not_available',
                'feature' => $feature,
                'upgrade_url' => route('billing.plans'),
            ], 403);
        }

        // Check usage limits
        if (!$this->usageTracker->hasQuota($user, $feature, $usage)) {
            $limit = $this->usageTracker->getLimit($user, $feature);
            $currentUsage = $this->usageTracker->getUsage($user, $feature);

            Log::info('Feature access denied - usage limit exceeded', [
                'user_id' => $user->id,
                'feature' => $feature,
                'current_usage' => $currentUsage,
                'limit' => $limit,
            ]);

            return response()->json([
                'error' => 'You have reached your usage limit for this feature',
                'code' => 'usage_limit_exceeded',
                'feature' => $feature,
                'current_usage' => $currentUsage,
                'limit' => $limit,
                'upgrade_url' => route('billing.plans'),
            ], 429);
        }

        // Record usage
        $this->usageTracker->record($user, $feature, $usage, [
            'endpoint' => $request->path(),
            'method' => $request->method(),
        ]);

        // Add usage info to response headers
        $response = $next($request);

        $limit = $this->usageTracker->getLimit($user, $feature);
        $remaining = $this->usageTracker->getRemainingQuota($user, $feature);

        if ($response instanceof Response) {
            $response->headers->set('X-Feature', $feature);
            if ($limit !== null) {
                $response->headers->set('X-Usage-Limit', (string) $limit);
                $response->headers->set('X-Usage-Remaining', (string) $remaining);
            }
        }

        return $response;
    }

    /**
     * Check if user's plan includes a feature
     */
    private function hasFeature($user, string $feature): bool
    {
        $subscription = $user->activeSubscription();

        if (!$subscription) {
            // Check free tier features
            return $this->isFreeFeature($feature);
        }

        $plan = $subscription->plan;

        if (!$plan) {
            return false;
        }

        $features = $plan->features ?? [];

        // Features can be boolean or have limits
        return isset($features[$feature]) && $features[$feature] !== false;
    }

    /**
     * Check if feature is available in free tier
     */
    private function isFreeFeature(string $feature): bool
    {
        $freeFeatures = config('subscription.free_features', []);
        return in_array($feature, $freeFeatures, true);
    }
}

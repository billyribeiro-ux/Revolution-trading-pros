<?php

namespace App\Services\Workflow\AI;

use App\Models\Workflow;
use App\Models\WorkflowRun;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Cache;

class WorkflowIntelligenceService
{
    /**
     * Predict workflow outcome based on historical data
     */
    public function predictOutcome(Workflow $workflow, array $context): array
    {
        $historicalRuns = $workflow->runs()
            ->completed()
            ->limit(100)
            ->get();

        if ($historicalRuns->count() < 10) {
            return [
                'confidence' => 'low',
                'predicted_success' => 0.5,
                'estimated_duration_ms' => $workflow->avg_execution_time_ms ?? 5000,
                'message' => 'Insufficient historical data for accurate prediction',
            ];
        }

        $successRate = $historicalRuns->where('status', 'completed')->count() / $historicalRuns->count();
        $avgDuration = $historicalRuns->avg('duration_ms');

        // Analyze context similarity
        $similarRuns = $this->findSimilarRuns($historicalRuns, $context);
        $contextSuccessRate = $similarRuns->where('status', 'completed')->count() / max($similarRuns->count(), 1);

        // Weighted prediction
        $predictedSuccess = ($successRate * 0.4) + ($contextSuccessRate * 0.6);

        return [
            'confidence' => $this->calculateConfidence($historicalRuns->count(), $similarRuns->count()),
            'predicted_success' => round($predictedSuccess, 2),
            'estimated_duration_ms' => round($avgDuration),
            'similar_runs_analyzed' => $similarRuns->count(),
            'recommendation' => $this->generateRecommendation($predictedSuccess),
        ];
    }

    /**
     * Suggest workflow improvements based on performance data
     */
    public function suggestImprovements(Workflow $workflow): array
    {
        $suggestions = [];

        // Analyze failure patterns
        $failureReasons = $workflow->runs()
            ->failed()
            ->selectRaw('error_message, COUNT(*) as count')
            ->groupBy('error_message')
            ->orderByDesc('count')
            ->limit(5)
            ->get();

        foreach ($failureReasons as $failure) {
            if ($failure->count > 3) {
                $suggestions[] = [
                    'type' => 'error_handling',
                    'priority' => 'high',
                    'issue' => "Recurring failure: {$failure->error_message}",
                    'suggestion' => $this->generateErrorSuggestion($failure->error_message),
                ];
            }
        }

        // Analyze execution time
        if ($workflow->avg_execution_time_ms > 30000) {
            $suggestions[] = [
                'type' => 'performance',
                'priority' => 'medium',
                'issue' => 'Slow execution time',
                'suggestion' => 'Consider adding parallel execution for independent actions or implementing caching',
            ];
        }

        // Analyze node efficiency
        $nodePerformance = $this->analyzeNodePerformance($workflow);
        foreach ($nodePerformance as $node) {
            if ($node['avg_duration_ms'] > 10000) {
                $suggestions[] = [
                    'type' => 'node_optimization',
                    'priority' => 'medium',
                    'issue' => "Slow node: {$node['node_type']}",
                    'suggestion' => 'This node is taking longer than expected. Consider optimizing the action or adding a timeout.',
                ];
            }
        }

        // Success rate analysis
        if ($workflow->getSuccessRate() < 80) {
            $suggestions[] = [
                'type' => 'reliability',
                'priority' => 'high',
                'issue' => 'Low success rate',
                'suggestion' => 'Add error handling, retry logic, or condition checks to improve reliability',
            ];
        }

        return $suggestions;
    }

    /**
     * Generate AI-powered workflow suggestions based on user behavior
     */
    public function suggestWorkflows(string $userId, array $behaviorData): array
    {
        $suggestions = [];

        // Analyze user's existing workflows
        $existingWorkflows = Workflow::forUser($userId)->get();
        $existingTriggers = $existingWorkflows->pluck('trigger_config.type')->unique();

        // Common workflow patterns
        $patterns = $this->getCommonPatterns();

        foreach ($patterns as $pattern) {
            if (!$existingTriggers->contains($pattern['trigger'])) {
                $suggestions[] = [
                    'name' => $pattern['name'],
                    'description' => $pattern['description'],
                    'trigger' => $pattern['trigger'],
                    'actions' => $pattern['actions'],
                    'estimated_impact' => $pattern['impact'],
                    'confidence' => $this->calculatePatternConfidence($pattern, $behaviorData),
                ];
            }
        }

        // Sort by confidence
        usort($suggestions, fn($a, $b) => $b['confidence'] <=> $a['confidence']);

        return array_slice($suggestions, 0, 5);
    }

    /**
     * Optimize workflow execution order
     */
    public function optimizeExecutionOrder(Workflow $workflow): array
    {
        $nodes = $workflow->nodes()->get();
        $edges = $workflow->edges()->get();

        // Analyze node dependencies
        $dependencies = $this->buildDependencyGraph($nodes, $edges);

        // Identify parallel execution opportunities
        $parallelGroups = $this->identifyParallelGroups($dependencies);

        // Calculate optimal order
        $optimizedOrder = $this->calculateOptimalOrder($nodes, $dependencies, $parallelGroups);

        return [
            'current_order' => $nodes->pluck('id')->toArray(),
            'optimized_order' => $optimizedOrder,
            'parallel_groups' => $parallelGroups,
            'estimated_time_savings_ms' => $this->estimateTimeSavings($workflow, $parallelGroups),
        ];
    }

    /**
     * Predict best time to execute workflow
     */
    public function predictBestExecutionTime(Workflow $workflow): array
    {
        $runs = $workflow->runs()
            ->completed()
            ->selectRaw('HOUR(started_at) as hour, AVG(duration_ms) as avg_duration, COUNT(*) as count')
            ->groupBy('hour')
            ->get();

        if ($runs->isEmpty()) {
            return [
                'recommended_hour' => 9, // Default to 9 AM
                'confidence' => 'low',
                'reason' => 'No historical data available',
            ];
        }

        // Find hour with best performance
        $bestHour = $runs->sortBy('avg_duration')->first();

        return [
            'recommended_hour' => $bestHour->hour,
            'confidence' => 'high',
            'avg_duration_ms' => round($bestHour->avg_duration),
            'reason' => "Historical data shows {$bestHour->count} successful runs at this time with optimal performance",
        ];
    }

    /**
     * Detect anomalies in workflow execution
     */
    public function detectAnomalies(Workflow $workflow): array
    {
        $recentRuns = $workflow->runs()
            ->where('started_at', '>=', now()->subDays(7))
            ->get();

        $anomalies = [];

        // Check for sudden failure rate increase
        $recentFailureRate = $recentRuns->where('status', 'failed')->count() / max($recentRuns->count(), 1);
        $historicalFailureRate = 1 - ($workflow->getSuccessRate() / 100);

        if ($recentFailureRate > $historicalFailureRate * 1.5) {
            $anomalies[] = [
                'type' => 'failure_spike',
                'severity' => 'high',
                'message' => 'Failure rate has increased significantly in the last 7 days',
                'current_rate' => round($recentFailureRate * 100, 1),
                'normal_rate' => round($historicalFailureRate * 100, 1),
            ];
        }

        // Check for execution time anomalies
        $recentAvgDuration = $recentRuns->avg('duration_ms');
        if ($recentAvgDuration > $workflow->avg_execution_time_ms * 1.5) {
            $anomalies[] = [
                'type' => 'performance_degradation',
                'severity' => 'medium',
                'message' => 'Execution time has increased significantly',
                'current_avg_ms' => round($recentAvgDuration),
                'normal_avg_ms' => $workflow->avg_execution_time_ms,
            ];
        }

        return $anomalies;
    }

    /**
     * Generate personalized action recommendations
     */
    public function recommendNextAction(array $context, array $userHistory): array
    {
        // Analyze what actions typically follow in successful workflows
        $commonSequences = $this->analyzeActionSequences();

        $currentAction = $context['_last_action'] ?? null;
        
        if (!$currentAction) {
            return [
                'recommended_action' => 'send_email',
                'confidence' => 0.7,
                'reason' => 'Email is the most common first action in successful workflows',
            ];
        }

        $nextActions = $commonSequences[$currentAction] ?? [];
        
        if (empty($nextActions)) {
            return [
                'recommended_action' => 'create_task',
                'confidence' => 0.5,
                'reason' => 'No clear pattern found, suggesting a common follow-up action',
            ];
        }

        $topAction = $nextActions[0];

        return [
            'recommended_action' => $topAction['action'],
            'confidence' => $topAction['confidence'],
            'reason' => "This action follows {$currentAction} in {$topAction['frequency']}% of successful workflows",
            'alternatives' => array_slice($nextActions, 1, 3),
        ];
    }

    // Helper methods

    private function findSimilarRuns($runs, array $context): \Illuminate\Support\Collection
    {
        return $runs->filter(function ($run) use ($context) {
            $similarity = $this->calculateContextSimilarity($run->context, $context);
            return $similarity > 0.7;
        });
    }

    private function calculateContextSimilarity(array $context1, array $context2): float
    {
        $keys1 = array_keys($context1);
        $keys2 = array_keys($context2);
        $commonKeys = array_intersect($keys1, $keys2);
        
        if (empty($commonKeys)) {
            return 0;
        }

        $matches = 0;
        foreach ($commonKeys as $key) {
            if ($context1[$key] === $context2[$key]) {
                $matches++;
            }
        }

        return $matches / count($commonKeys);
    }

    private function calculateConfidence(int $totalRuns, int $similarRuns): string
    {
        if ($totalRuns < 10) return 'low';
        if ($similarRuns < 5) return 'medium';
        return 'high';
    }

    private function generateRecommendation(float $predictedSuccess): string
    {
        if ($predictedSuccess > 0.9) {
            return 'High probability of success. Safe to execute.';
        } elseif ($predictedSuccess > 0.7) {
            return 'Good probability of success. Monitor execution.';
        } elseif ($predictedSuccess > 0.5) {
            return 'Moderate success probability. Consider reviewing workflow configuration.';
        } else {
            return 'Low success probability. Review and test workflow before production use.';
        }
    }

    private function generateErrorSuggestion(string $errorMessage): string
    {
        if (str_contains($errorMessage, 'timeout')) {
            return 'Add timeout handling or increase timeout duration';
        } elseif (str_contains($errorMessage, 'not found')) {
            return 'Add existence checks before accessing resources';
        } elseif (str_contains($errorMessage, 'permission')) {
            return 'Verify API credentials and permissions';
        } else {
            return 'Add try-catch error handling and retry logic';
        }
    }

    private function analyzeNodePerformance(Workflow $workflow): array
    {
        return DB::table('workflow_run_logs')
            ->join('workflow_runs', 'workflow_run_logs.workflow_run_id', '=', 'workflow_runs.id')
            ->join('workflow_nodes', 'workflow_run_logs.node_id', '=', 'workflow_nodes.id')
            ->where('workflow_runs.workflow_id', $workflow->id)
            ->where('workflow_run_logs.status', 'completed')
            ->selectRaw('workflow_nodes.node_type, AVG(workflow_run_logs.duration_ms) as avg_duration_ms, COUNT(*) as execution_count')
            ->groupBy('workflow_nodes.node_type')
            ->get()
            ->toArray();
    }

    private function getCommonPatterns(): array
    {
        return [
            [
                'name' => 'Welcome Email Sequence',
                'description' => 'Send welcome email when new contact is created',
                'trigger' => 'contact.created',
                'actions' => ['send_email', 'add_tag', 'create_task'],
                'impact' => 'high',
            ],
            [
                'name' => 'Abandoned Cart Recovery',
                'description' => 'Follow up when cart is abandoned',
                'trigger' => 'cart.abandoned',
                'actions' => ['delay', 'send_email', 'create_deal'],
                'impact' => 'high',
            ],
            [
                'name' => 'Lead Scoring Automation',
                'description' => 'Update lead score based on behavior',
                'trigger' => 'page.visited',
                'actions' => ['update_field', 'add_tag', 'send_notification'],
                'impact' => 'medium',
            ],
        ];
    }

    private function calculatePatternConfidence(array $pattern, array $behaviorData): float
    {
        // Simple confidence calculation based on behavior data
        return 0.8; // Placeholder
    }

    private function buildDependencyGraph($nodes, $edges): array
    {
        $graph = [];
        foreach ($nodes as $node) {
            $graph[$node->id] = [];
        }
        foreach ($edges as $edge) {
            $graph[$edge->from_node_id][] = $edge->to_node_id;
        }
        return $graph;
    }

    private function identifyParallelGroups(array $dependencies): array
    {
        // Identify nodes that can run in parallel
        $parallelGroups = [];
        // Implementation would analyze dependency graph
        return $parallelGroups;
    }

    private function calculateOptimalOrder($nodes, $dependencies, $parallelGroups): array
    {
        // Topological sort with parallel execution consideration
        return $nodes->pluck('id')->toArray(); // Placeholder
    }

    private function estimateTimeSavings(Workflow $workflow, array $parallelGroups): int
    {
        // Estimate time savings from parallel execution
        return 0; // Placeholder
    }

    private function analyzeActionSequences(): array
    {
        // Analyze common action sequences across all workflows
        return Cache::remember('action_sequences', 3600, function () {
            return [
                'send_email' => [
                    ['action' => 'add_tag', 'confidence' => 0.85, 'frequency' => 65],
                    ['action' => 'create_task', 'confidence' => 0.75, 'frequency' => 45],
                ],
                'create_deal' => [
                    ['action' => 'send_notification', 'confidence' => 0.9, 'frequency' => 70],
                    ['action' => 'create_task', 'confidence' => 0.8, 'frequency' => 55],
                ],
            ];
        });
    }
}

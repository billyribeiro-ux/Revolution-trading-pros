<?php

namespace App\Services\Workflow\AI;

use App\Models\Workflow;
use Illuminate\Support\Facades\DB;

class WorkflowOptimizer
{
    /**
     * Auto-optimize workflow based on performance data
     */
    public function autoOptimize(Workflow $workflow): array
    {
        $optimizations = [];

        // 1. Identify bottlenecks
        $bottlenecks = $this->identifyBottlenecks($workflow);
        foreach ($bottlenecks as $bottleneck) {
            $optimizations[] = [
                'type' => 'bottleneck_removal',
                'node_id' => $bottleneck['node_id'],
                'current_duration_ms' => $bottleneck['avg_duration_ms'],
                'action' => 'Consider caching, parallel execution, or timeout optimization',
            ];
        }

        // 2. Optimize condition placement
        $conditionOptimizations = $this->optimizeConditions($workflow);
        $optimizations = array_merge($optimizations, $conditionOptimizations);

        // 3. Suggest parallel execution
        $parallelOpportunities = $this->findParallelOpportunities($workflow);
        $optimizations = array_merge($optimizations, $parallelOpportunities);

        // 4. Remove redundant nodes
        $redundantNodes = $this->findRedundantNodes($workflow);
        $optimizations = array_merge($optimizations, $redundantNodes);

        return [
            'workflow_id' => $workflow->id,
            'current_performance' => [
                'avg_duration_ms' => $workflow->avg_execution_time_ms,
                'success_rate' => $workflow->getSuccessRate(),
            ],
            'optimizations' => $optimizations,
            'estimated_improvement' => $this->estimateImprovement($optimizations),
        ];
    }

    /**
     * Generate optimal workflow structure for a goal
     */
    public function generateOptimalWorkflow(string $goal, array $constraints = []): array
    {
        // AI-driven workflow generation based on goal
        $templates = $this->getGoalTemplates();
        
        $matchedTemplate = null;
        foreach ($templates as $template) {
            if (str_contains(strtolower($goal), strtolower($template['keyword']))) {
                $matchedTemplate = $template;
                break;
            }
        }

        if (!$matchedTemplate) {
            $matchedTemplate = $templates['default'];
        }

        return [
            'name' => $matchedTemplate['name'],
            'description' => $matchedTemplate['description'],
            'nodes' => $this->generateNodes($matchedTemplate['structure']),
            'edges' => $this->generateEdges($matchedTemplate['structure']),
            'estimated_success_rate' => $matchedTemplate['success_rate'],
        ];
    }

    /**
     * Suggest workflow variations for A/B testing
     */
    public function suggestVariations(Workflow $workflow): array
    {
        $variations = [];

        // Variation 1: Different action order
        $variations[] = [
            'name' => 'Optimized Action Order',
            'description' => 'Reorder actions based on success patterns',
            'changes' => $this->generateActionOrderVariation($workflow),
            'expected_impact' => '+5-10% success rate',
        ];

        // Variation 2: Add delay optimization
        $variations[] = [
            'name' => 'Smart Delays',
            'description' => 'Add strategic delays for better engagement',
            'changes' => $this->generateDelayVariation($workflow),
            'expected_impact' => '+10-15% engagement',
        ];

        // Variation 3: Condition refinement
        $variations[] = [
            'name' => 'Refined Conditions',
            'description' => 'Optimize condition logic for better targeting',
            'changes' => $this->generateConditionVariation($workflow),
            'expected_impact' => '+8-12% conversion',
        ];

        return $variations;
    }

    // Helper methods

    private function identifyBottlenecks(Workflow $workflow): array
    {
        return DB::table('workflow_run_logs')
            ->join('workflow_runs', 'workflow_run_logs.workflow_run_id', '=', 'workflow_runs.id')
            ->where('workflow_runs.workflow_id', $workflow->id)
            ->where('workflow_run_logs.status', 'completed')
            ->selectRaw('node_id, AVG(duration_ms) as avg_duration_ms')
            ->groupBy('node_id')
            ->having('avg_duration_ms', '>', 5000)
            ->get()
            ->toArray();
    }

    private function optimizeConditions(Workflow $workflow): array
    {
        // Analyze condition nodes and suggest optimizations
        $conditionNodes = $workflow->nodes()->where('node_type', 'condition')->get();
        
        $optimizations = [];
        foreach ($conditionNodes as $node) {
            // Check if condition is too complex
            $conditionCount = count($node->config['conditions'] ?? []);
            if ($conditionCount > 5) {
                $optimizations[] = [
                    'type' => 'condition_simplification',
                    'node_id' => $node->id,
                    'action' => 'Simplify condition logic or split into multiple nodes',
                ];
            }
        }

        return $optimizations;
    }

    private function findParallelOpportunities(Workflow $workflow): array
    {
        $opportunities = [];
        $nodes = $workflow->nodes()->get();
        $edges = $workflow->edges()->get();

        // Find action nodes that don't depend on each other
        $actionNodes = $nodes->where('node_type', 'action');
        
        foreach ($actionNodes as $node1) {
            foreach ($actionNodes as $node2) {
                if ($node1->id !== $node2->id && !$this->hasPath($node1->id, $node2->id, $edges)) {
                    $opportunities[] = [
                        'type' => 'parallel_execution',
                        'nodes' => [$node1->id, $node2->id],
                        'action' => 'Execute these actions in parallel to reduce total time',
                    ];
                }
            }
        }

        return $opportunities;
    }

    private function findRedundantNodes(Workflow $workflow): array
    {
        // Identify nodes that don't contribute to workflow success
        $redundant = [];
        
        // Check for nodes that always pass through without effect
        $logs = DB::table('workflow_run_logs')
            ->join('workflow_runs', 'workflow_run_logs.workflow_run_id', '=', 'workflow_runs.id')
            ->where('workflow_runs.workflow_id', $workflow->id)
            ->where('workflow_run_logs.status', 'skipped')
            ->selectRaw('node_id, COUNT(*) as skip_count')
            ->groupBy('node_id')
            ->having('skip_count', '>', 10)
            ->get();

        foreach ($logs as $log) {
            $redundant[] = [
                'type' => 'redundant_node',
                'node_id' => $log->node_id,
                'action' => 'This node is frequently skipped and may be unnecessary',
            ];
        }

        return $redundant;
    }

    private function estimateImprovement(array $optimizations): array
    {
        $timeReduction = 0;
        $successIncrease = 0;

        foreach ($optimizations as $opt) {
            switch ($opt['type']) {
                case 'bottleneck_removal':
                    $timeReduction += 20; // 20% time reduction
                    break;
                case 'parallel_execution':
                    $timeReduction += 30; // 30% time reduction
                    break;
                case 'redundant_node':
                    $timeReduction += 5; // 5% time reduction
                    $successIncrease += 2; // 2% success increase
                    break;
            }
        }

        return [
            'estimated_time_reduction_percent' => min($timeReduction, 50),
            'estimated_success_increase_percent' => min($successIncrease, 15),
        ];
    }

    private function hasPath(int $from, int $to, $edges): bool
    {
        // Check if there's a path from one node to another
        $visited = [];
        $queue = [$from];

        while (!empty($queue)) {
            $current = array_shift($queue);
            if ($current === $to) {
                return true;
            }

            if (in_array($current, $visited)) {
                continue;
            }

            $visited[] = $current;

            $nextNodes = $edges->where('from_node_id', $current)->pluck('to_node_id');
            foreach ($nextNodes as $next) {
                $queue[] = $next;
            }
        }

        return false;
    }

    private function getGoalTemplates(): array
    {
        return [
            'lead_nurture' => [
                'keyword' => 'lead',
                'name' => 'Lead Nurturing Workflow',
                'description' => 'Automated lead nurturing sequence',
                'structure' => ['trigger' => 'contact.created', 'actions' => ['send_email', 'delay', 'add_tag']],
                'success_rate' => 0.85,
            ],
            'onboarding' => [
                'keyword' => 'onboard',
                'name' => 'User Onboarding Workflow',
                'description' => 'Welcome and onboard new users',
                'structure' => ['trigger' => 'user.created', 'actions' => ['send_email', 'create_task', 'send_notification']],
                'success_rate' => 0.9,
            ],
            'default' => [
                'keyword' => '',
                'name' => 'Custom Workflow',
                'description' => 'Custom workflow template',
                'structure' => ['trigger' => 'custom', 'actions' => ['send_email']],
                'success_rate' => 0.75,
            ],
        ];
    }

    private function generateNodes(array $structure): array
    {
        // Generate node configuration from structure
        return [];
    }

    private function generateEdges(array $structure): array
    {
        // Generate edge configuration from structure
        return [];
    }

    private function generateActionOrderVariation(Workflow $workflow): array
    {
        return ['reorder' => 'actions based on success patterns'];
    }

    private function generateDelayVariation(Workflow $workflow): array
    {
        return ['add_delays' => 'strategic delays between actions'];
    }

    private function generateConditionVariation(Workflow $workflow): array
    {
        return ['refine_conditions' => 'optimize targeting logic'];
    }
}

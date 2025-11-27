<?php

namespace App\Services\Workflow;

use App\Models\Workflow;
use App\Models\WorkflowRun;
use App\Models\WorkflowNode;
use App\Models\WorkflowRunLog;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Log;

class WorkflowExecutor
{
    public function __construct(
        private TriggerEvaluator $triggerEvaluator,
        private ConditionEvaluator $conditionEvaluator,
        private ActionRunner $actionRunner
    ) {}

    /**
     * Execute a workflow from a trigger event
     */
    public function execute(Workflow $workflow, array $triggerData, ?string $userId = null): WorkflowRun
    {
        if (!$workflow->isActive()) {
            throw new \Exception("Workflow {$workflow->id} is not active");
        }

        return DB::transaction(function () use ($workflow, $triggerData, $userId) {
            // Create workflow run
            $run = WorkflowRun::create([
                'workflow_id' => $workflow->id,
                'trigger_event_id' => $triggerData['event_id'] ?? null,
                'status' => 'running',
                'started_at' => now(),
                'context' => $this->buildInitialContext($triggerData),
                'triggered_by_user_id' => $userId,
            ]);

            try {
                // Get trigger node
                $triggerNode = $workflow->nodes()->where('node_type', 'trigger')->first();
                
                if (!$triggerNode) {
                    throw new \Exception('No trigger node found');
                }

                // Execute workflow from trigger node
                $this->executeNode($run, $triggerNode, $run->context);

                // Mark as completed
                $run->markAsCompleted();
                $workflow->incrementSuccessCount();
                $workflow->updateAverageExecutionTime($run->duration_ms);

            } catch (\Exception $e) {
                Log::error('Workflow execution failed', [
                    'workflow_id' => $workflow->id,
                    'run_id' => $run->id,
                    'error' => $e->getMessage(),
                ]);

                $run->markAsFailed($e->getMessage());
                $workflow->incrementFailureCount();
            }

            $workflow->incrementExecutionCount();
            $workflow->update(['last_executed_at' => now()]);

            return $run;
        });
    }

    /**
     * Execute a single node
     */
    private function executeNode(WorkflowRun $run, WorkflowNode $node, array $context): array
    {
        $startTime = microtime(true);

        // Create log entry
        $log = WorkflowRunLog::create([
            'workflow_run_id' => $run->id,
            'node_id' => $node->id,
            'status' => 'running',
            'executed_at' => now(),
            'input_data' => $context,
        ]);

        try {
            $output = match ($node->node_type) {
                'trigger' => $this->executeTrigger($node, $context),
                'condition' => $this->executeCondition($node, $context),
                'action' => $this->executeAction($node, $context),
                'delay' => $this->executeDelay($node, $context),
                'branch' => $this->executeBranch($run, $node, $context),
                'parallel' => $this->executeParallel($run, $node, $context),
                default => $context,
            };

            // Update log
            $duration = (microtime(true) - $startTime) * 1000;
            $log->update([
                'status' => 'completed',
                'output_data' => $output,
                'duration_ms' => round($duration),
            ]);

            // Execute next nodes
            if ($node->node_type !== 'branch' && $node->node_type !== 'parallel') {
                $this->executeNextNodes($run, $node, $output);
            }

            return $output;

        } catch (\Exception $e) {
            $duration = (microtime(true) - $startTime) * 1000;
            $log->update([
                'status' => 'failed',
                'error' => $e->getMessage(),
                'duration_ms' => round($duration),
            ]);

            throw $e;
        }
    }

    /**
     * Execute trigger node
     */
    private function executeTrigger(WorkflowNode $node, array $context): array
    {
        // Trigger nodes just pass through context
        return $context;
    }

    /**
     * Execute condition node
     */
    private function executeCondition(WorkflowNode $node, array $context): array
    {
        $result = $this->conditionEvaluator->evaluate($node->config, $context);
        return array_merge($context, ['_condition_result' => $result]);
    }

    /**
     * Execute action node
     */
    private function executeAction(WorkflowNode $node, array $context): array
    {
        $result = $this->actionRunner->run($node->config['action_type'], $node->config, $context);
        return array_merge($context, ['_action_result' => $result]);
    }

    /**
     * Execute delay node
     */
    private function executeDelay(WorkflowNode $node, array $context): array
    {
        $delaySeconds = $node->config['delay_seconds'] ?? 0;
        
        if ($delaySeconds > 0) {
            // Queue delayed execution
            \App\Jobs\ResumeWorkflowJob::dispatch($context)
                ->delay(now()->addSeconds($delaySeconds));
            
            throw new \Exception('Workflow paused for delay');
        }

        return $context;
    }

    /**
     * Execute branch node (if/else)
     */
    private function executeBranch(WorkflowRun $run, WorkflowNode $node, array $context): array
    {
        $conditionResult = $context['_condition_result'] ?? false;

        // Get edges based on condition - eager load toNode to prevent N+1
        $edges = $node->outgoingEdges()
            ->where('condition_type', $conditionResult ? 'if_true' : 'if_false')
            ->with(['toNode'])
            ->get();

        foreach ($edges as $edge) {
            $nextNode = $edge->toNode;
            $this->executeNode($run, $nextNode, $context);
        }

        return $context;
    }

    /**
     * Execute parallel node
     */
    private function executeParallel(WorkflowRun $run, WorkflowNode $node, array $context): array
    {
        $edges = $node->outgoingEdges()->where('condition_type', 'parallel')->with(['toNode'])->get();

        $results = [];
        foreach ($edges as $edge) {
            $nextNode = $edge->toNode;
            $results[] = $this->executeNode($run, $nextNode, $context);
        }

        // Merge all results
        return array_merge($context, ['_parallel_results' => $results]);
    }

    /**
     * Execute next nodes
     */
    private function executeNextNodes(WorkflowRun $run, WorkflowNode $node, array $context): void
    {
        $edges = $node->outgoingEdges()->where('condition_type', 'always')->with(['toNode'])->get();

        foreach ($edges as $edge) {
            $nextNode = $edge->toNode;
            $this->executeNode($run, $nextNode, $context);
        }
    }

    /**
     * Build initial context from trigger data
     */
    private function buildInitialContext(array $triggerData): array
    {
        return [
            'trigger' => $triggerData,
            'timestamp' => now()->toIso8601String(),
            'variables' => [],
        ];
    }
}

<?php

namespace App\Http\Controllers;

use App\Models\Workflow;
use App\Models\WorkflowNode;
use App\Models\WorkflowEdge;
use App\Services\Workflow\WorkflowExecutor;
use Illuminate\Http\Request;
use Illuminate\Support\Facades\Auth;

class WorkflowController extends Controller
{
    public function __construct(
        private WorkflowExecutor $executor
    ) {}

    /**
     * Get all workflows for user
     */
    public function index()
    {
        $workflows = Workflow::forUser(Auth::id())
            ->with(['triggers', 'schedules'])
            ->orderBy('created_at', 'desc')
            ->get();

        return response()->json(['data' => $workflows]);
    }

    /**
     * Get single workflow
     */
    public function show(Workflow $workflow)
    {
        $this->authorize('view', $workflow);

        return response()->json(['data' => $workflow->load(['nodes', 'edges', 'triggers'])]);
    }

    /**
     * Create workflow
     */
    public function store(Request $request)
    {
        $validated = $request->validate([
            'name' => 'required|string|max:255',
            'description' => 'nullable|string',
            'trigger_config' => 'required|array',
            'status' => 'in:active,paused,archived',
        ]);

        $workflow = Workflow::create([
            ...$validated,
            'user_id' => Auth::id(),
            'version' => 1,
        ]);

        return response()->json(['data' => $workflow], 201);
    }

    /**
     * Update workflow
     */
    public function update(Request $request, Workflow $workflow)
    {
        $this->authorize('update', $workflow);

        $validated = $request->validate([
            'name' => 'string|max:255',
            'description' => 'nullable|string',
            'trigger_config' => 'array',
            'status' => 'in:active,paused,archived',
        ]);

        $workflow->update($validated);

        return response()->json(['data' => $workflow]);
    }

    /**
     * Delete workflow
     */
    public function destroy(Workflow $workflow)
    {
        $this->authorize('delete', $workflow);

        $workflow->delete();

        return response()->json(['message' => 'Workflow deleted']);
    }

    /**
     * Toggle workflow status
     */
    public function toggleStatus(Request $request, Workflow $workflow)
    {
        $this->authorize('update', $workflow);

        $validated = $request->validate([
            'status' => 'required|in:active,paused',
        ]);

        $workflow->update(['status' => $validated['status']]);

        return response()->json(['data' => $workflow]);
    }

    /**
     * Get workflow nodes
     */
    public function nodes(Workflow $workflow)
    {
        $this->authorize('view', $workflow);

        $nodes = $workflow->nodes()->orderBy('order')->get();

        return response()->json(['data' => $nodes]);
    }

    /**
     * Create node
     */
    public function createNode(Request $request, Workflow $workflow)
    {
        $this->authorize('update', $workflow);

        $validated = $request->validate([
            'node_type' => 'required|in:trigger,condition,action,delay,branch,parallel,merge,end',
            'config' => 'required|array',
            'position_x' => 'required|integer',
            'position_y' => 'required|integer',
            'parent_node_id' => 'nullable|exists:workflow_nodes,id',
            'order' => 'integer',
        ]);

        $node = $workflow->nodes()->create($validated);

        return response()->json(['data' => $node], 201);
    }

    /**
     * Update node
     */
    public function updateNode(Request $request, Workflow $workflow, WorkflowNode $node)
    {
        $this->authorize('update', $workflow);

        $validated = $request->validate([
            'config' => 'array',
            'position_x' => 'integer',
            'position_y' => 'integer',
            'order' => 'integer',
        ]);

        $node->update($validated);

        return response()->json(['data' => $node]);
    }

    /**
     * Delete node
     */
    public function deleteNode(Workflow $workflow, WorkflowNode $node)
    {
        $this->authorize('update', $workflow);

        $node->delete();

        return response()->json(['message' => 'Node deleted']);
    }

    /**
     * Get workflow edges
     */
    public function edges(Workflow $workflow)
    {
        $this->authorize('view', $workflow);

        $edges = $workflow->edges()->get();

        return response()->json(['data' => $edges]);
    }

    /**
     * Create edge
     */
    public function createEdge(Request $request, Workflow $workflow)
    {
        $this->authorize('update', $workflow);

        $validated = $request->validate([
            'from_node_id' => 'required|exists:workflow_nodes,id',
            'to_node_id' => 'required|exists:workflow_nodes,id',
            'condition_type' => 'required|in:always,if_true,if_false,parallel',
            'label' => 'nullable|string',
        ]);

        $edge = $workflow->edges()->create($validated);

        return response()->json(['data' => $edge], 201);
    }

    /**
     * Delete edge
     */
    public function deleteEdge(Workflow $workflow, WorkflowEdge $edge)
    {
        $this->authorize('update', $workflow);

        $edge->delete();

        return response()->json(['message' => 'Edge deleted']);
    }

    /**
     * Execute workflow
     */
    public function execute(Request $request, Workflow $workflow)
    {
        $this->authorize('execute', $workflow);

        $triggerData = $request->input('trigger_data', []);
        $triggerData['event_id'] = uniqid('test_');

        $run = $this->executor->execute($workflow, $triggerData, Auth::id());

        return response()->json(['data' => $run]);
    }

    /**
     * Get workflow runs
     */
    public function runs(Workflow $workflow)
    {
        $this->authorize('view', $workflow);

        $runs = $workflow->runs()
            ->with('logs')
            ->orderBy('started_at', 'desc')
            ->limit(50)
            ->get();

        return response()->json(['data' => $runs]);
    }

    /**
     * Get workflow analytics
     */
    public function analytics(Workflow $workflow)
    {
        $this->authorize('view', $workflow);

        $analytics = [
            'workflow_id' => $workflow->id,
            'total_runs' => $workflow->execution_count,
            'successful_runs' => $workflow->success_count,
            'failed_runs' => $workflow->failure_count,
            'success_rate' => $workflow->getSuccessRate(),
            'avg_duration_ms' => $workflow->avg_execution_time_ms,
            'runs_by_day' => $this->getRunsByDay($workflow),
            'failure_reasons' => $this->getFailureReasons($workflow),
        ];

        return response()->json(['data' => $analytics]);
    }

    private function getRunsByDay(Workflow $workflow)
    {
        return $workflow->runs()
            ->selectRaw('DATE(started_at) as date, COUNT(*) as count')
            ->where('started_at', '>=', now()->subDays(30))
            ->groupBy('date')
            ->orderBy('date')
            ->get();
    }

    private function getFailureReasons(Workflow $workflow)
    {
        return $workflow->runs()
            ->failed()
            ->selectRaw('error_message as reason, COUNT(*) as count')
            ->groupBy('error_message')
            ->orderByDesc('count')
            ->limit(10)
            ->get();
    }
}

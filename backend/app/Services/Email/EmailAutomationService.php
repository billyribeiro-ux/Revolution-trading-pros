<?php

declare(strict_types=1);

namespace App\Services\Email;

use App\Models\EmailAutomationWorkflow;
use App\Models\EmailTemplate;
use App\Models\User;
use App\Services\EmailService;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Facades\DB;

/**
 * Email Automation Service
 * RevolutionEmailTemplates-L8-System
 * 
 * Manages automated email workflows and triggers
 */
class EmailAutomationService
{
    public function __construct(
        private EmailService $emailService,
        private EmailTemplateRenderService $renderService
    ) {}

    /**
     * Trigger automation workflow
     */
    public function triggerWorkflow(string $event, array $context): void
    {
        $workflows = EmailAutomationWorkflow::where('trigger_event', $event)
            ->where('is_active', true)
            ->with(['template'])
            ->orderBy('priority', 'desc')
            ->get();

        foreach ($workflows as $workflow) {
            if ($this->shouldTrigger($workflow, $context)) {
                $this->executeWorkflow($workflow, $context);
            }
        }
    }

    /**
     * Check if workflow should be triggered
     */
    private function shouldTrigger(EmailAutomationWorkflow $workflow, array $context): bool
    {
        // Check trigger conditions
        if ($workflow->trigger_conditions) {
            foreach ($workflow->trigger_conditions as $condition) {
                if (!$this->evaluateCondition($condition, $context)) {
                    return false;
                }
            }
        }

        // Check send limits
        if ($workflow->send_once_per_user && isset($context['user_id'])) {
            $alreadySent = DB::table('email_logs')
                ->where('workflow_id', $workflow->id)
                ->where('recipient_email', $context['user']->email ?? null)
                ->exists();

            if ($alreadySent) {
                return false;
            }
        }

        if ($workflow->max_sends_per_user && isset($context['user_id'])) {
            $sendCount = DB::table('email_logs')
                ->where('workflow_id', $workflow->id)
                ->where('recipient_email', $context['user']->email ?? null)
                ->count();

            if ($sendCount >= $workflow->max_sends_per_user) {
                return false;
            }
        }

        return true;
    }

    /**
     * Execute workflow
     */
    private function executeWorkflow(EmailAutomationWorkflow $workflow, array $context): void
    {
        try {
            $template = $workflow->template;

            // Apply delay if specified
            if ($workflow->delay_minutes > 0) {
                // Queue for later
                dispatch(function () use ($workflow, $template, $context) {
                    $this->sendAutomationEmail($workflow, $template, $context);
                })->delay(now()->addMinutes($workflow->delay_minutes));
            } else {
                // Send immediately
                $this->sendAutomationEmail($workflow, $template, $context);
            }

            // Update statistics
            $this->updateWorkflowStatistics($workflow, 'triggered');

            Log::info('[EmailAutomationService] Workflow executed', [
                'workflow_id' => $workflow->id,
                'event' => $workflow->trigger_event,
            ]);

        } catch (\Exception $e) {
            Log::error('[EmailAutomationService] Workflow execution failed', [
                'workflow_id' => $workflow->id,
                'error' => $e->getMessage(),
            ]);

            $this->updateWorkflowStatistics($workflow, 'failed');
        }
    }

    /**
     * Send automation email
     */
    private function sendAutomationEmail(EmailAutomationWorkflow $workflow, EmailTemplate $template, array $context): void
    {
        $user = $context['user'] ?? null;

        if (!$user || !$user->email) {
            Log::warning('[EmailAutomationService] No user email in context', [
                'workflow_id' => $workflow->id,
            ]);
            return;
        }

        // Render template
        $rendered = $this->renderService->render($template, $context);

        // Send email
        $this->emailService->send(
            $user->email,
            $rendered['subject'],
            $rendered['html'],
            [
                'type' => 'automation',
                'workflow_id' => $workflow->id,
                'template_id' => $template->id,
            ]
        );

        $this->updateWorkflowStatistics($workflow, 'sent');
    }

    /**
     * Evaluate condition
     */
    private function evaluateCondition(array $condition, array $context): bool
    {
        $field = $condition['field'] ?? '';
        $operator = $condition['operator'] ?? '==';
        $value = $condition['value'] ?? '';

        $actualValue = data_get($context, $field);

        return match($operator) {
            '==' => $actualValue == $value,
            '!=' => $actualValue != $value,
            '>' => $actualValue > $value,
            '<' => $actualValue < $value,
            '>=' => $actualValue >= $value,
            '<=' => $actualValue <= $value,
            'contains' => str_contains((string)$actualValue, (string)$value),
            'not_contains' => !str_contains((string)$actualValue, (string)$value),
            'in' => in_array($actualValue, (array)$value),
            'not_in' => !in_array($actualValue, (array)$value),
            default => false,
        };
    }

    /**
     * Update workflow statistics
     */
    private function updateWorkflowStatistics(EmailAutomationWorkflow $workflow, string $event): void
    {
        $statistics = $workflow->statistics ?? [];
        $statistics[$event] = ($statistics[$event] ?? 0) + 1;
        $statistics['last_' . $event . '_at'] = now()->toDateTimeString();

        $workflow->update(['statistics' => $statistics]);
    }

    /**
     * Create automation workflow
     */
    public function createWorkflow(array $data): EmailAutomationWorkflow
    {
        return EmailAutomationWorkflow::create([
            'name' => $data['name'],
            'trigger_event' => $data['trigger_event'],
            'trigger_conditions' => $data['trigger_conditions'] ?? null,
            'template_id' => $data['template_id'],
            'delay_minutes' => $data['delay_minutes'] ?? 0,
            'is_active' => $data['is_active'] ?? true,
            'priority' => $data['priority'] ?? 0,
            'send_once_per_user' => $data['send_once_per_user'] ?? true,
            'max_sends_per_user' => $data['max_sends_per_user'] ?? null,
        ]);
    }

    /**
     * Update workflow
     */
    public function updateWorkflow(EmailAutomationWorkflow $workflow, array $data): EmailAutomationWorkflow
    {
        $workflow->update($data);
        return $workflow->fresh();
    }

    /**
     * Activate workflow
     */
    public function activateWorkflow(EmailAutomationWorkflow $workflow): EmailAutomationWorkflow
    {
        $workflow->update(['is_active' => true]);
        
        Log::info('[EmailAutomationService] Workflow activated', [
            'workflow_id' => $workflow->id,
        ]);

        return $workflow;
    }

    /**
     * Deactivate workflow
     */
    public function deactivateWorkflow(EmailAutomationWorkflow $workflow): EmailAutomationWorkflow
    {
        $workflow->update(['is_active' => false]);
        
        Log::info('[EmailAutomationService] Workflow deactivated', [
            'workflow_id' => $workflow->id,
        ]);

        return $workflow;
    }

    /**
     * Get workflow analytics
     */
    public function getWorkflowAnalytics(EmailAutomationWorkflow $workflow): array
    {
        $statistics = $workflow->statistics ?? [];

        return [
            'triggered_count' => $statistics['triggered'] ?? 0,
            'sent_count' => $statistics['sent'] ?? 0,
            'failed_count' => $statistics['failed'] ?? 0,
            'last_triggered_at' => $statistics['last_triggered_at'] ?? null,
            'last_sent_at' => $statistics['last_sent_at'] ?? null,
            'success_rate' => $this->calculateSuccessRate($statistics),
        ];
    }

    /**
     * Calculate success rate
     */
    private function calculateSuccessRate(array $statistics): float
    {
        $triggered = $statistics['triggered'] ?? 0;
        $sent = $statistics['sent'] ?? 0;

        if ($triggered === 0) {
            return 0;
        }

        return ($sent / $triggered) * 100;
    }
}

<?php

namespace App\Services\Workflow;

use App\Services\Workflow\Actions\ActionInterface;
use Illuminate\Support\Facades\Log;

class ActionRunner
{
    private array $actions = [];

    public function __construct()
    {
        $this->registerActions();
    }

    /**
     * Register all action handlers
     */
    private function registerActions(): void
    {
        // Contact actions
        $this->register('add_tag', \App\Services\Workflow\Actions\AddTagAction::class);
        $this->register('remove_tag', \App\Services\Workflow\Actions\RemoveTagAction::class);
        $this->register('update_field', \App\Services\Workflow\Actions\UpdateFieldAction::class);
        
        // Email actions
        $this->register('send_email', \App\Services\Workflow\Actions\SendEmailAction::class);
        $this->register('send_transactional', \App\Services\Workflow\Actions\SendTransactionalAction::class);
        
        // CRM actions
        $this->register('create_deal', \App\Services\Workflow\Actions\CreateDealAction::class);
        $this->register('update_deal', \App\Services\Workflow\Actions\UpdateDealAction::class);
        $this->register('create_task', \App\Services\Workflow\Actions\CreateTaskAction::class);
        
        // Notification actions
        $this->register('send_notification', \App\Services\Workflow\Actions\SendNotificationAction::class);
        $this->register('send_webhook', \App\Services\Workflow\Actions\SendWebhookAction::class);
        
        // Integration actions
        $this->register('http_request', \App\Services\Workflow\Actions\HttpRequestAction::class);
    }

    /**
     * Register an action handler
     */
    public function register(string $type, string $class): void
    {
        $this->actions[$type] = $class;
    }

    /**
     * Run an action
     */
    public function run(string $actionType, array $config, array $context): array
    {
        if (!isset($this->actions[$actionType])) {
            throw new \Exception("Action type '{$actionType}' not found");
        }

        $actionClass = $this->actions[$actionType];
        $action = app($actionClass);

        if (!$action instanceof ActionInterface) {
            throw new \Exception("Action must implement ActionInterface");
        }

        try {
            return $action->execute($config, $context);
        } catch (\Exception $e) {
            Log::error("Action execution failed", [
                'action_type' => $actionType,
                'error' => $e->getMessage(),
                'config' => $config,
            ]);

            throw $e;
        }
    }

    /**
     * Check if action type exists
     */
    public function has(string $actionType): bool
    {
        return isset($this->actions[$actionType]);
    }

    /**
     * Get all registered action types
     */
    public function getRegisteredActions(): array
    {
        return array_keys($this->actions);
    }
}
